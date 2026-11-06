# Wave 4.1 Task 2 — Button Hotkey References + Hover Tooltips: Implementation Plan

**Date:** 2026-11-05
**Workflow step:** 4.1b (architector) — referencing the 4.1a spec
**Source TODO:** [`.agent/todos/20261102/20261102-todo-1.md`](../../.agent/todos/20261102/20261102-todo-1.md) §Task 2
**Front-end spec (BINDING input, read in full):** [`.kilo/plans/20261102-wave4-1-task2-references-tooltips-frontend-spec.md`](20261102-wave4-1-task2-references-tooltips-frontend-spec.md)
**Branch:** `fix/wave-4-operator-fixes`
**Verified HEAD:** `86a1c2c` (`task: mark wave4.1 task1 launch state done`) — Task 1 mark landed; version already `0.4.1` (bump commit `53c2e60` at Step 3; NO further version action).

---

## 0. Decisions locked by this plan (all sourced from the approved spec §2/§3 + user-approved set)

1. **Captions (9 pt, below glyph, default text color):** `F11`, `I`, `O`, `0`, `W`, `H` on the six keyboard-distinct buttons. The four arrow buttons are **uncaptioned** (spec §2: the arrows are painted as the arrow keys themselves).
2. **Tooltips (all ten buttons, exact strings):** `Previous image (Left Arrow)`, `Next image (Right Arrow)`, `Previous sibling folder (Up Arrow)`, `Next sibling folder (Down Arrow)`, `Fullscreen (F11)`, `Zoom in (I)`, `Zoom out (O)`, `Restore fit (0)`, `Fit width (W)`, `Fit height (H)`.
3. **Layout:** per-button vertical column of the button's width; pill row changes `Layout::left_to_right(Align::Center)` → `Layout::left_to_right(Align::TOP)`; caption sits `2.0` pt under the glyph block; caption text `RichText::new(ref).size(9.0).color(visuals().text_color())`.
4. **Wiring shape (minimal-change, one shared helper):** a single `button_hint` module owns the caption/tooltip data AND the shared vertical-column renderer. Each pill button module keeps its existing allocate/widget-info/cursor/glyph/click code; it becomes the paint closure passed to the shared helper, and every button gets the same wrap:
   `allocate column (button width) → paint button (unchanged code) → attach tooltip pair on the button Response → paint caption (only when a reference exists)`.
5. **Data layout (keyed BY BUTTON, no label duplication):** `PillButton` enum of all ten buttons; the two pure accessors `hotkey_reference(PillButton) -> Option<&'static str>` and `tooltip_text(PillButton) -> &'static str` match on the enum. Small conversion constructors map the landed button types onto `PillButton`: `PillButton::from_arrow_control(ArrowControl)`, `PillButton::from_zoom_action(ZoomButtonAction)`, `PillButton::from_fit_mode_action(FitModeButtonAction)`. `ZoomButtonAction`/`FitModeButtonAction` need only a visibility+derive upgrade (`enum` → `#[derive(Clone, Copy)] pub(super) enum`) — matching the established `ArrowControl` `pub(super)` pattern. Their bodies/uses are otherwise untouched.
6. **Two-argument rule compliance:** every helper call stays at ≤2 params via two param objects:
   - `PillButtonPlacement::for_button(button: PillButton, button_size: egui::Vec2)` (2 args) — built where the button size consts live (per module).
   - `ButtonColumnSite::new(row_ui, placement)` (2 args) — the row `Ui` plus its placement.
   - `show_button_column(site, paint_closure)` (2 args; the closure is the render callback).
7. **Tooltip attach behavior (explicit decision):** both `on_hover_text` and `on_disabled_hover_text` are chained on the SAME button `Response` (spec §9 checklist, literal conformance). The landings' responses are egui-*enabled* (the modules' "disabled" state is manual paint + guarded clicks — NOT an egui-disabled state), so the enabled-tooltip branch serves hovering on every button, including painted-disabled ones. **No `Ui::disable()` / `set_enabled` changes** — clicks and enablement must not change (out-of-scope guard §18). Clicked/click-guards keep working because `Response` tooltip-chaining preserves flags (`Response` is `Clone`; the methods take `self` and return the modified copy).

### Verified egui 0.37.0 API facts (checked against the pinned `eframe = "=0.37.0"` upstream sources, 2026-11-05)

- `Response::on_hover_text(self, text: impl Into<WidgetText>) -> Self` and `Response::on_disabled_hover_text(self, text: impl Into<WidgetText>) -> Self` exist in `response.rs` and are chainable (`&str: Into<WidgetText>`).
- `Visuals::text_color() -> Color32` exists in `style.rs` (resolves `override_text_color` → noninteractive text color = the default text color).
- `Ui::allocate_ui_with_layout<R>(&mut self, desired_size: Vec2, layout: Layout, add_contents: impl FnOnce(&mut Self) -> R) -> InnerResponse<R>` exists in `ui.rs` (returns `.inner` = the closure's return).
- `Ui::spacing_mut().item_spacing: Vec2` and `Ui::visuals()`, `Ui::allocate_exact_size(size, sense) -> (Rect, Response)`, `Response::widget_info(...)` — all already used by the landed code.
- `egui::Id::unique(salt)`, `Area::anchor(Align2, Vec2)`, `Order::Foreground` — already used by the pill; untouched.

---

## 1. Pre-flight checks (STRICT; STOP on any mismatch)

1. `git branch --show-current` → must print `fix/wave-4-operator-fixes`.
2. `git log --oneline -3` → must show HEAD `86a1c2c task: mark wave4.1 task1 launch state done`, `4052c5a`, `93eb47d`.
3. `git status` → expected working-tree state: modified `.agent/project-info/context.md` ONLY, plus untracked `.kilo/plans/20261102-wave4-1-task2-references-tooltips-frontend-spec.md` and (after this plan is saved) untracked `.kilo/plans/20261102-wave4-1-task2-references-tooltips-plan.md`. If ANY other modified/untracked file appears → STOP and return the question to the caller.
4. Read `.gitignore` and confirm the files to be staged (spec/plan `.md`, `src/*.rs`, `context.md`, `project-structure.md`) match no ignore pattern (`target/`, `dist/`, `logs/` and friends must never be staged).
5. `git log`-verified line counts at HEAD (authoritative today; re-measure before the budget gate in §13):
   `floating_controls.rs` **181**, `fullscreen_button.rs` **47**, `zoom_button.rs` **189**, `fit_mode_button.rs` **172**, `arrow_direction.rs` 66, `keyboard_intake.rs` 70, `app.rs` **198**.

## 2. Git actions for this step

- Branch creation / switching / `git push` are NOT part of this step (branch already exists; push is restricted to workflow Step 5). Work commits happen ON the existing branch.
- **Commit 1 (docs)** — message exactly `docs: add wave4.1 task2 spec and plan`:
  - `git add .kilo/plans/20261102-wave4-1-task2-references-tooltips-frontend-spec.md .kilo/plans/20261102-wave4-1-task2-references-tooltips-plan.md`
  - `git add .agent/project-info/context.md` — FIRST inspect its diff (`git diff .agent/project-info/context.md`): it must contain only the Wave-4.1 Task-2 4.1a context-log entries (documenting the spec event). If any other content is in the diff → STOP, ask the caller.
  - Run the pre-commit checklist (§16), then `git commit -m "docs: add wave4.1 task2 spec and plan"`.
- **Commit 2 (feat)** — message exactly `feat: button hotkey references and hover tooltips`, staged AFTER all code steps pass the harness (§12): `git add src/...` the five touched/created files ONLY (app.rs, button_hint.rs, floating_controls.rs, fullscreen_button.rs, zoom_button.rs, fit_mode_button.rs), then commit.
- **Commit 3 (docs, small)** — message `docs: record wave4.1 task2 button-hint state`: `.agent/project-info/context.md` (closing-step entry per §15) + `.agent/project-structure.md` (per §15). Not part of the user-specified two commits; it is the step's context-log closing duty, following the established wave pattern.
- NO `CHANGELOG.md` edits; NO edits to any existing `.kilo/plans/*` file (new files only); NO TODO-file edits (the `[DONE]` mark belongs to 4.6).

## 3. Step 1 — create `src/app/button_hint.rs` (NEW file)

Create `src/app/button_hint.rs` with EXACTLY the following content (the data layout, strings and signatures are binding; the implementer may only fix rustfmt-level formatting; hard cap ≤200 lines):

```rust
//! Button-hint plumbing for the floating pill (Wave 4.1 Task 2): the per-button
//! hotkey captions and the hover tooltip strings are keyed here, keyed by
//! button, so no pill button module duplicates them; the shared vertical
//! button-column layout wraps every pill button's paint code.

use eframe::egui;

use super::arrow_direction::ArrowControl;
use super::fit_mode_button::FitModeButtonAction;
use super::zoom_button::ZoomButtonAction;

/// Pill-button caption size (9 pt; the spec's `RichText::size(9.0)`).
const CAPTION_TEXT_SIZE: f32 = 9.0;
/// Vertical gap between the painted glyph block and the caption line.
const CAPTION_TEXT_GAP: f32 = 2.0;

/// All ten pill buttons, keyed by button (the caption/tooltip tables below
/// resolve per button; no per-module duplicate string tables).
#[derive(Clone, Copy)]
pub(super) enum PillButton {
    ImagePrevious,
    ImageNext,
    FolderUp,
    FolderDown,
    Fullscreen,
    ZoomIn,
    ZoomOut,
    RestoreFit,
    FitWidth,
    FitHeight,
}

impl PillButton {
    pub(super) fn from_arrow_control(control: ArrowControl) -> Self {
        match control {
            ArrowControl::ImagePrevious => Self::ImagePrevious,
            ArrowControl::ImageNext => Self::ImageNext,
            ArrowControl::FolderUp => Self::FolderUp,
            ArrowControl::FolderDown => Self::FolderDown,
        }
    }

    pub(super) fn from_zoom_action(action: ZoomButtonAction) -> Self {
        match action {
            ZoomButtonAction::ZoomIn => Self::ZoomIn,
            ZoomButtonAction::ZoomOut => Self::ZoomOut,
            ZoomButtonAction::RestoreFit => Self::RestoreFit,
        }
    }

    pub(super) fn from_fit_mode_action(action: FitModeButtonAction) -> Self {
        match action {
            FitModeButtonAction::FitWidth => Self::FitWidth,
            FitModeButtonAction::FitHeight => Self::FitHeight,
        }
    }
}

/// The hotkey reference caption under the glyph (`None` for the four arrows:
/// each arrow is painted as the arrow key itself).
fn hotkey_reference(button: PillButton) -> Option<&'static str> {
    match button {
        PillButton::ImagePrevious
        | PillButton::ImageNext
        | PillButton::FolderUp
        | PillButton::FolderDown => None,
        PillButton::Fullscreen => Some("F11"),
        PillButton::ZoomIn => Some("I"),
        PillButton::ZoomOut => Some("O"),
        PillButton::RestoreFit => Some("0"),
        PillButton::FitWidth => Some("W"),
        PillButton::FitHeight => Some("H"),
    }
}

/// The hover tooltip of every pill button (its primary hotkey included).
fn tooltip_text(button: PillButton) -> &'static str {
    match button {
        PillButton::ImagePrevious => "Previous image (Left Arrow)",
        PillButton::ImageNext => "Next image (Right Arrow)",
        PillButton::FolderUp => "Previous sibling folder (Up Arrow)",
        PillButton::FolderDown => "Next sibling folder (Down Arrow)",
        PillButton::Fullscreen => "Fullscreen (F11)",
        PillButton::ZoomIn => "Zoom in (I)",
        PillButton::ZoomOut => "Zoom out (O)",
        PillButton::RestoreFit => "Restore fit (0)",
        PillButton::FitWidth => "Fit width (W)",
        PillButton::FitHeight => "Fit height (H)",
    }
}

/// What the shared column helper needs about one pill button: which button
/// (caption + tooltip source) and its painted size (the column's width; the
/// size consts stay in the button modules, single-source per module).
pub(super) struct PillButtonPlacement {
    button: PillButton,
    button_size: egui::Vec2,
}

impl PillButtonPlacement {
    pub(super) fn for_button(button: PillButton, button_size: egui::Vec2) -> Self {
        Self {
            button,
            button_size,
        }
    }
}

/// The pill row Ui being appended to, plus its button's placement (parameter
/// object keeping the shared helper at the two-argument limit).
pub(super) struct ButtonColumnSite<'row_ui> {
    row_ui: &'row_ui mut egui::Ui,
    placement: PillButtonPlacement,
}

impl<'row_ui> ButtonColumnSite<'row_ui> {
    pub(super) fn new(row_ui: &'row_ui mut egui::Ui, placement: PillButtonPlacement) -> Self {
        Self { row_ui, placement }
    }
}

/// Wraps one pill button: a top-aligned vertical column as wide as the button;
/// the paint closure owns the unchanged allocate/widget-info/glyph code; the
/// helper attaches the tooltip pair on the button Response and, when the
/// button has a hotkey reference, the 9 pt caption line under the glyph.
pub(super) fn show_button_column(
    ButtonColumnSite { row_ui, placement }: ButtonColumnSite<'_>,
    paint_button: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    let button = placement.button;
    let column_width = placement.button_size.x;
    let column_response = row_ui.allocate_ui_with_layout(
        egui::Vec2::new(column_width, 0.0),
        egui::Layout::top_down(egui::Align::Center),
        |column_ui| {
            column_ui.spacing_mut().item_spacing.y = CAPTION_TEXT_GAP;
            let painted_button = paint_button(column_ui);
            let tooltip = tooltip_text(button);
            let painted_button =
                painted_button.on_hover_text(tooltip).on_disabled_hover_text(tooltip);
            if let Some(reference) = hotkey_reference(button) {
                column_ui.label(
                    egui::RichText::new(reference)
                        .size(CAPTION_TEXT_SIZE)
                        .color(column_ui.visuals().text_color()),
                );
            }
            painted_button
        },
    );
    column_response.inner
}

#[cfg(test)]
mod tests {
    use super::{hotkey_reference, tooltip_text, PillButton};

    #[test]
    fn hotkey_reference_matches_the_spec() {
        use PillButton::*;
        assert_eq!(hotkey_reference(ImagePrevious), None);
        assert_eq!(hotkey_reference(ImageNext), None);
        assert_eq!(hotkey_reference(FolderUp), None);
        assert_eq!(hotkey_reference(FolderDown), None);
        assert_eq!(hotkey_reference(Fullscreen), Some("F11"));
        assert_eq!(hotkey_reference(ZoomIn), Some("I"));
        assert_eq!(hotkey_reference(ZoomOut), Some("O"));
        assert_eq!(hotkey_reference(RestoreFit), Some("0"));
        assert_eq!(hotkey_reference(FitWidth), Some("W"));
        assert_eq!(hotkey_reference(FitHeight), Some("H"));
    }

    #[test]
    fn tooltip_text_matches_the_spec() {
        use PillButton::*;
        assert_eq!(tooltip_text(ImagePrevious), "Previous image (Left Arrow)");
        assert_eq!(tooltip_text(ImageNext), "Next image (Right Arrow)");
        assert_eq!(tooltip_text(FolderUp), "Previous sibling folder (Up Arrow)");
        assert_eq!(tooltip_text(FolderDown), "Next sibling folder (Down Arrow)");
        assert_eq!(tooltip_text(Fullscreen), "Fullscreen (F11)");
        assert_eq!(tooltip_text(ZoomIn), "Zoom in (I)");
        assert_eq!(tooltip_text(ZoomOut), "Zoom out (O)");
        assert_eq!(tooltip_text(RestoreFit), "Restore fit (0)");
        assert_eq!(tooltip_text(FitWidth), "Fit width (W)");
        assert_eq!(tooltip_text(FitHeight), "Fit height (H)");
    }
}
```

Design notes to hold during review:
- `hotkey_reference`/`tooltip_text` are **module-private on purpose** (prefer-private rule: only `button_hint`'s own helper consumes them). The 4.5a checklist's "exports" item is satisfied at module scope (`PillButton`, the three conversion constructors, `PillButtonPlacement`, `ButtonColumnSite`, `show_button_column` are `pub(super)`).
- The cyclic module references are fine within one crate (`button_hint` ↔ `zoom_button` / `fit_mode_button`).
- Expected size ≈150–170 lines incl. the inline test module (hard cap 200; see §13 gate).
- `use PillButton::*;` inside the test fns is a nested-glob import confined to the test scope (existing house pattern uses explicit paths but `use super::...` + variant list would exceed line budget; if the reviewer prefers it, that is a 4.3 concern, not an implementer-side choice).

## 4. Step 2 — register the module in `src/app.rs` (`main.rs` UNTOUCHED — `button_hint` is a child of `app`)

In `src/app.rs`, between line 154 `mod arrow_direction;` and line 155 (the blank line before `mod central_surface;`), insert TWO new lines (blank + declaration), keeping every other row byte-identical:

```rust
mod arrow_direction;

mod button_hint;

```

(The first inserted line is blank; the second is `mod button_hint;` — the existing blank line before `mod central_surface;` remains, so the sequence reads `mod arrow_direction;` / blank / `mod button_hint;` / blank / `mod central_surface;`.)

Alphabetical position preserved; the file grows 198 → exactly 200 lines (≤200 holds; the §13 gate re-verifies). DO NOT touch anything else in `app.rs` — no new fields (out-of-scope guard §18).

## 5. Step 3 — `src/app/floating_controls.rs` (181 → ~189)

Three mechanical edits and one one-line doc addition. NO renames of existing parameter names; NO changes to gap constants, `show_pill_contents`, `show_message_line`, `arrow_button_specs`, `ArrowButtonSpec` fields, or `navigation_button_enabled`.

3.1 Module header: insert ONE new `//!` line after line 5 (`//! the keyboard seam per frame; the caller keeps the pill-visibility gate.`):
```rust
//! Every button paints inside a top-aligned vertical column; the six
//! keyboard-distinct buttons carry their 9 pt hotkey caption (Wave-4.1 Task 2).
```
(This is two `//!` lines; recount freely — must stay ≤200.)

3.2 After line 9 `use super::arrow_direction::ArrowControl;` insert the import (alphabetical among the `super::` row):
```rust
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
```

3.3 Line 73 (inside `show_button_row`): change the row alignment ONLY:
```rust
    pill_ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |row_ui| {
```

3.4 Replace the body of `ArrowButtonSpec::show_and_navigate` (old 6 lines) with the column wrap — click handling keeps its existing literal shape:
```rust
    fn show_and_navigate(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let placement = PillButtonPlacement::for_button(
            PillButton::from_arrow_control(self.control),
            ARROW_BUTTON_SIZE,
        );
        let button_column = ButtonColumnSite::new(row_ui, placement);
        let button_response =
            show_button_column(button_column, |column_ui| self.allocate_and_paint(column_ui));
        if let Some(direction) = self.click_direction(&button_response) {
            app.navigate(direction);
        }
    }
```

3.5 `allocate_and_paint`, `paint_triangle`, `click_direction`, `accepts_click`, `arrow_color` are UNCHANGED (the paint closure passes `column_ui` into the existing `row_ui`-named parameters; no signature edits). The four arrows therefore get the tooltip pair and NO caption (reference is `None`), automatically.

## 6. Step 4 — `src/app/fullscreen_button.rs` (47 → ~54)

4.1 After line 9 `use super::ImagesScrollerApp;` add:
```rust
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
```

4.2 Replace `show_fullscreen_pill_button`'s inline allocate/paint block with the wrap + an extracted module-private `allocate_and_paint` free function (the fn is passed as the closure by path):
```rust
/// Appends the ALWAYS-ENABLED fullscreen toggle button to the pill row.
pub(super) fn show_fullscreen_pill_button(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    let placement = PillButtonPlacement::for_button(PillButton::Fullscreen, FULLSCREEN_BUTTON_SIZE);
    let button_column = ButtonColumnSite::new(row_ui, placement);
    let painted_button = show_button_column(button_column, allocate_and_paint);
    if painted_button.clicked() {
        app.toggle_fullscreen_and_send_command(row_ui.ctx());
    }
}

fn allocate_and_paint(column_ui: &mut egui::Ui) -> egui::Response {
    let (_, button_response) =
        column_ui.allocate_exact_size(FULLSCREEN_BUTTON_SIZE, egui::Sense::click());
    button_response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::Role::Button, true, FULLSCREEN_BUTTON_LABEL)
    });
    let painted_button = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
    paint_fullscreen_square(column_ui, &painted_button);
    painted_button
}
```
(The extracted body is the old inline code byte-for-byte, with `row_ui` → `column_ui` in the allocate/paint calls; `paint_fullscreen_square` and `fullscreen_square_color` unchanged.)

## 7. Step 5 — `src/app/zoom_button.rs` (189 → ~195; tight file — apply ALL three edits exactly)

7.1 After line 9 `use super::ImagesScrollerApp;` add:
```rust
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
```

7.2 Upgrade the action enum (in place, lines 22-26): one derive line + one visibility word, plus `self.action` staying out-of-borrow legal during placement construction:
```rust
#[derive(Clone, Copy)]
pub(super) enum ZoomButtonAction {
    ZoomIn,
    ZoomOut,
    RestoreFit,
}
```

7.3 Replace `show_and_apply_action` body with the wrap, and extract the allocate/paint block into `allocate_and_paint` (extracted = old inline code byte-for-byte, `row_ui` params kept):
```rust
    /// Paints the glyph and, when clicked while enabled, applies its action.
    fn show_and_apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let placement = PillButtonPlacement::for_button(
            PillButton::from_zoom_action(self.action),
            ZOOM_BUTTON_SIZE,
        );
        let button_column = ButtonColumnSite::new(row_ui, placement);
        let button_response =
            show_button_column(button_column, |column_ui| self.allocate_and_paint(column_ui));
        if self.accepts_click(&button_response) {
            self.apply_action(app);
        }
    }

    fn allocate_and_paint(&self, column_ui: &mut egui::Ui) -> egui::Response {
        let (_, button_response) =
            column_ui.allocate_exact_size(ZOOM_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, self.accessible_label())
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_glyph(column_ui, &button_response);
        button_response
    }
```

7.4 **Deterministic line-budget trim — merge `zoom_glyph_shapes` into `paint_glyph`** (mechanical, no behavior change; frees ~7 lines so the file stays ≤200). Replace BOTH functions (`paint_glyph` + `zoom_glyph_shapes`, old 21 lines) with the single merged function:
```rust
    /// Paints the button's per-action glyph (zoom icons are rects/outline
    /// only, D8) at the disabled/idle/hovered palette.
    fn paint_glyph(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let glyph_center = button_response.rect.center();
        let glyph_color = self.zoom_glyph_color(row_ui, button_response);
        let glyph_shapes = match self.action {
            ZoomButtonAction::ZoomIn => self.crossed_bar_shapes(glyph_center, glyph_color),
            ZoomButtonAction::ZoomOut => self.single_bar_shapes(glyph_center, glyph_color),
            ZoomButtonAction::RestoreFit => self.stroked_square_shapes(glyph_center, glyph_color),
        };
        for glyph_shape in glyph_shapes {
            row_ui.painter().add(glyph_shape);
        }
    }
```
Everything else in the module (consts, `show_zoom_pill_buttons`, `zoom_button_specs`, `row_button`, `accepts_click`, `apply_action`, `accessible_label`, `zoom_glyph_color`, the three shape functions and their doc comments) is UNCHANGED, including `pub(super) const ZOOM_{IN,OUT,RESTORE_FIT}_BUTTON_LABEL` and `ZOOM_GROUP_GAP`.

## 8. Step 6 — `src/app/fit_mode_button.rs` (172 → ~185)

8.1 After line 9 `use super::ImagesScrollerApp;` add:
```rust
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
```

8.2 Upgrade the action enum (in place):
```rust
#[derive(Clone, Copy)]
pub(super) enum FitModeButtonAction {
    FitWidth,
    FitHeight,
}
```

8.3 Replace `show_and_apply_action` body with the wrap + extract `allocate_and_paint` (same shape as zoom: `PillButton::from_fit_mode_action(self.action)` with `FIT_BUTTON_SIZE`; same closing `if self.accepts_click(...) { self.apply_action(app); }`; extracted body = old inline code byte-for-byte, including `self.enabled` widget info with `self.accessible_label()`).

8.4 Everything else UNCHANGED: `FIT_GROUP_GAP` + `show_fit_mode_pill_buttons` untouched, `FitModeButtonAction::glyph_shapes` stays a separate function (no merge needed), all six glyph-geometry consts untouched, the two `pub(super) const FIT_{WIDTH,HEIGHT}_BUTTON_LABEL` untouched.

## 9. Step 7 — files that must NOT change in this task

`arrow_direction.rs` (66), `keyboard_intake.rs` (the hotkey truth — captions/tooltips only MIRROR it), `central_surface.rs`, `app_display.rs`, `src/zoom.rs`, `src/scaling.rs`, `src/app*tests.rs`, `floating_controls_tests.rs` (uses only `navigation_button_enabled`), `main.rs`, `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, all sibling `.kilo/plans/*`, all other plans/specs. If any of these seems needed → STOP and ask the caller.

## 10. Step 8 — formatting round (ALPINE VM CONTAINER, never host; `vm_status` FIRST)

All commands run through the `alpine-vm` MCP (allowlisted `docker` prefix; one command per call; exit code is authoritative).

1. `vm_status` — only proceed when running + SSH-reachable; if the VM or `/img-scroller` share is unavailable → STOP and return a user question.
2. Apply rustfmt (the only mechanical auto-write allowed; uses the committed lockfile-safe container):
   `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt`
3. Inspect the format round with `git diff` — formatting-only changes to the files touched in §3–§8; anything formatting OTHER files is a red flag → revert those path hunks and re-test.
4. Verify: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt --check` → exit 0.

## 11. Step 9 — dev-check harness (full, all four checks must pass)

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
```

Expected: names `fmt-check`, `check`, `test`, `clippy` each `PASS ... exit=0`; final line `ALL CHECKS PASSED`; script exit code **0**. `cargo test` line must read `95 passed; 0 failed` (§14). On failure: fix the named check (fmt → repeat §10; check/clippy → fix the reported diagnostics in the touched files ONLY, keeping every rule article here; test → the two new tests must not fail) and re-run the FULL harness. Never move to commits without `ALL CHECKS PASSED`.

## 12. Step 10 — line-budget validation gate (caption/table for the implementer + reviewers)

Re-measure inside the container after the fmt round:

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "wc -l src/app.rs src/app/*.rs"
```

| File | HEAD | Expected after task | Hard cap (project rule) |
|---|---|---|---|
| `src/app/button_hint.rs` | new | ≈150–170 | ≤200 |
| `src/app.rs` | 198 | 200 | ≤200 |
| `src/app/floating_controls.rs` | 181 | ≈189 | ≤200 |
| `src/app/fullscreen_button.rs` | 47 | ≈54 | ≤200 |
| `src/app/zoom_button.rs` | 189 | ≈195 | ≤200 |
| `src/app/fit_mode_button.rs` | 172 | ≈185 | ≤200 |
| `src/app/arrow_direction.rs` | 66 | 66 (untouched) | ≤200 |
| `src/app/keyboard_intake.rs` | 70 | 70 (untouched) | ≤200 |

STOP conditions: any touched file lands >200 after the fmt round → STOP and report the measured counts to the caller (do NOT autonomously restructure). Method-length rule (≤50 body lines) is met by construction: no new/edited method exceeds ~15 body lines. No method/function gains a third argument anywhere.

## 13. Step 11 — commits (order is binding: docs → feat → docs-markdown)

1. Commit 1 (docs, from §2) may land right after Step 1 of the plan (both plan/spec files exist before code work).
2. After §11 passes: stage ONLY the six `src/` files (`git add src/app.rs src/app/button_hint.rs src/app/floating_controls.rs src/app/fullscreen_button.rs src/app/zoom_button.rs src/app/fit_mode_button.rs`), run the pre-commit checklist (§16), then:
   `git commit -m "feat: button hotkey references and hover tooltips"`
3. Then the closing markdown commit (§15): update the two context/structure files, stage them, then:
   `git commit -m "docs: record wave4.1 task2 button-hint state"`

## 14. Test plan

- **New (in `src/app/button_hint.rs` inline `#[cfg(test)] mod tests`, exactly these names):**
  1. `hotkey_reference_matches_the_spec` — 10 assertions pinning the `Option<&'static str>` table.
  2. `tooltip_text_matches_the_spec` — 10 assertions pinning every tooltip string.
- **Suite census: 93 → 95** (+2 `#[test]` items; 20 assertion-level expectations). No test mocks `egui::Response`; both fns take only the enum and return `&'static str` (spec §8: the render/column behavior is GUI-bound and intentionally untested).
- No existing test file is edited; the census check is the harness `cargo test` line in §11.
- Exe rebuild / DLL import audit / OPERATOR-RUN steps are NOT part of this task (they live in Wave-4.1 Task 8's verification step). The GUI/runtime checkbox "button hotkey references + tooltips render" will be operator-validated later.

## 15. Documentation updates inside this step (commits 1 and 3) — content pinned

- **Commit 1** — the two plan/spec markdown files + `context.md` (its ALREADY-present 4.1a diff; inspect before staging).
- **Commit 3** —
  1. `.agent/project-info/context.md`: prepend one `Recent Changes` entry (dated 2026-11-05): Task 2 steps 4.1a+4.1b complete — spec + implementation-plan paths, the user-approved caption/tooltip set, the `src/app/button_hint.rs` module, +2 tests (93 → 95), and the commit hashes of commits 1–2; update `Current Work Focus` to "Task 2 4.1b complete — implementation (4.2) pending"; bump the HEAD note.
  2. `.agent/project-structure.md`: in the `app/` modules bullet, phrase `button_hint.rs (the pill's hotkey-caption/tooltip tables keyed by PillButton plus the shared vertical button-column layout helper)`, and append `button_hint.rs` to the module list in alphabetical position (between arrow_direction and the rest); the top `app.rs` line gains `the pill's button-hint module routing` style note ONLY if the line still fits its existing shape — otherwise leave the top line unchanged.
- 4.4 (docs-specialist) will later own `README.md`/`tech.md`/`architecture.md` keybinding/tooltip tables per the wave's Task-9 plan; nothing else here.

## 16. Pre-commit checklist (run before EVERY commit)

1. Read `.gitignore`; `git status` clean of ignored-pattern files; nothing under `target/`, `dist/`, `logs/` staged.
2. `git diff --cached` matches exactly the named file set for that commit (§2/§13).
3. No `CHANGELOG.md` change, no existing `.kilo/plans/*` modification, no TODO-file change included.
4. Commit message matches §13 verbatim.
5. `git log --oneline -4` afterwards shows the expected chain; `git branch --show-current` still the feature branch (no merge/push actions — those are workflow Step 5).

## 17. Code-review heads-up (for 4.3 reviewers)

- The closure bodies introduce a 3rd nesting level (fn → egui closure → `if let`) in `show_button_column` — the same shape already lands at `floating_controls.rs::show_button_row` (with_layout closure → for-loop). If the simplifier wants the caption `if let` extracted, that is a follow-up simplification plan decision, NOT an implementer-side change here.
- `zoom_button.rs` lands ~195/200 — reviewers should trim only, they already carry the plan's merge trim (§7.4); do not re-open the `row_button` constructor or shape functions.
- `military-mode`: no new doc comments were added beyond the specified ones; module doc headers reflect the column behavior exactly once (floating_controls §5.1 + button_hint module header).

## 18. Out-of-scope guard (hard boundaries for the implementer)

- Clicks, enablement predicates, navigation/zoom/fit seams, keyboard intake, fullscreen gating (`full_ui_chrome_is_visible`): ALL unchanged. The pill's render path is inside the chrome gate, so while fullscreen is ON nothing renders — no extra tooltip gating, no UI-state change (spec §7).
- No `ImagesScrollerApp` field added; no stored tooltip/caption state (tooltips are stateless per-frame popups; captions are string-literal labels — no per-frame texture or allocation churn beyond one Label per captioned button).
- Group gaps (`FOLDER_GROUP_GAP`, `FULLSCREEN_GROUP_GAP`, `ZOOM_GROUP_GAP`, `FIT_GROUP_GAP`) unchanged; the pill frame is allowed to grow taller by the caption height (spec §3/§5.1).
- Nothing in Wave-4.1 Tasks 3–7 territory (F11 immediacy, ESC, stray message, zoom bounds/aliases, one-shot fit) may be touched, even where the files overlap.
- Neither the parked future-ideas (auto-hide chrome, bottom-hover reveal, drag & drop) nor any principle beyond this plan.

## 19. Conformance crosswalk → 4.5a frontend verification

| 4.5a checklist item (spec §9) | Plan step |
|---|---|
|`button_hint.rs` exists with `PillButton`, `hotkey_reference`, `tooltip_text`, shared column helper | §3 (module-scope access via `pub(super)`/private per §3 notes) |
| No new `ImagesScrollerApp` fields | §18 + §4 |
| Pill row `left_to_right(Align::TOP)` | §5.3 |
| Every button in a vertical column of its button width | §5.4, §6.2, §7.3, §8.3 |
| Arrows uncaptioned; six view-control buttons show `F11/I/O/0/W/H` | §3 data table + tests |
| Captions `RichText` 9 pt, current text color, `2.0` gap, top-aligned columns | §3 consts + §5.3 |
| Tooltips via `on_hover_text` AND `on_disabled_hover_text` on the same click Response | §3 helper + §0.7 decision note |
| Tooltip strings exact | §3 + pinned tests |
| Group gaps unchanged | §5–§8 (no gap edits) |
| Fullscreen ON hides references/tooltips (no extra gate) | §18 |
| Inline tests pass | §11 harness (95 passed) |
| ≤200-line / ≤50-line-method conformance | §12 gate |
| No per-frame texture re-upload / allocation churn | §18 |

## 20. Downstream notes for the caller (4.2 → 4.6)

- **4.2 (implementer):** execute §1–§13 in order; the code snippets are authoritative shapes — minor rustfmt reflow is fine, semantic changes are not.
- **4.3 (reviewer/simplifier):** the review envelope = §19 + §17; expect a small simplification plan only if the `Align`/closure shape deserves extraction.
- **4.4 (docs-specialist):** besides §15's two files, update `README.md`, `tech.md`, `architecture.md` with the user-facing consequence: every pill button in chrome shows its hover tooltip; six captioned buttons show `F11`/`I`/`O`/`0`/`W`/`H`; the pill gained a caption row (~10–12 px).
- **4.5a (frontend-specialist):** verify against `.kilo/plans/20261102-wave4-1-task2-references-tooltips-frontend-spec.md` §9 using the crosswalk above; `hotkey_reference`/`tooltip_text` being module-private is the recorded deliberate decision (§3 notes), not a deviation.
- **4.5b (architector):** compare the landed diff to this plan's step list; the budget table (§12) must match the measured values within ±2 lines excluding `button_hint.rs`'s estimated range.
- **4.6 (implementer):** mark TODO Task 2 sub-items `[x]` on `[ ]` only; commit `task: mark wave4.1 task2 button hints done` following the Task-1 pattern.
- **Operator-run relevance (Task 8 note):** hover shows the tooltip; the six captioned buttons show their hotkeys; painted-disabled buttons (zoom at bounds, fit with no image, list-edge arrows) still show their tooltip on hover; arrows carry tooltips with no caption.

---

**Verification against TODO Task 2:** both sub-items are covered — buttons' beneath-glyph hotkey references (six keyboard-distinct buttons, exact user-approved set, 9 pt below glyph, arrows uncaptioned) and hover tooltips with short descriptions on all ten buttons; the wave-architector decision freedom is exercised exactly as the TODO requires, in a single shared helper with two pure pinned-string tests.
