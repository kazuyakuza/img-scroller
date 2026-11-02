# Wave 4 Task 1 — Fullscreen Toggle: 4.1b Implementation Plan

Date: 2026-11-02 · Critical Workflow step 4.1b (architector) for Wave-4 Task 1
Branch: `feat/wave-4-fullscreen-zoom` · Base: HEAD `730235d` · Version: 0.4.0
Source TODO: `.agent/todos/20261029/20261029-todo-6.md` (Task 1 sub-items)
Front-end spec (binding): `.kilo/plans/20261102-wave4-task1-fullscreen-frontend-spec.md`
Global plan (parent): `.kilo/plans/20261102-wave-4-fullscreen-zoom.md`
Plan file (this): `.kilo/plans/20261102-wave-4-task1-fullscreen-plan.md` (NEW file)

## 0. Binding Decisions (source of truth for this plan — do not re-litigate)

| Topic | Decision |
|---|---|
| Launch state | Fullscreen **OFF**; no `NativeOptions`/`ViewportBuilder` launch fullscreen/size/maximize change (`with_maximized(false)` stays) |
| Runtime mechanism | `ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(bool))` — egui 0.37.0 verified (spec §2) |
| State field | Exactly ONE new field: `fullscreen_on: bool` in `ImagesScrollerApp`, initialized `false` in `empty_state()`, session-only, never persisted |
| Keyboard shortcut | **F11** (`egui::Key::F11` — verified present in the egui 0.37 `Key` enum), same per-frame intake block as the arrow keys, edge-triggered via `key_pressed` |
| UI toggle control | **Fifth button appended to the existing bottom-center pill row** (user-approved Option A), after `[←][→] | [↑][↓]` with a group gap |
| Icon technique | **Painted filled square** via `egui::Shape::rect_filled(rect, egui::CornerRadius::ZERO, color)` (verified egui 0.37.0); consistent with the painted vector-triangle arrows; NO font glyphs |
| Button enablement | ALWAYS enabled in the normal chrome state (the only viewport-local control); `WidgetInfo::labeled(egui::Role::Button, true, "Toggle fullscreen")` |
| Hiding contract | Fullscreen ON: pill NOT rendered, empty-state Open NOT rendered, empty-folder message NOT rendered; ONLY the central image surface runs. OFF: exact 0.3.0 layout returns |
| Gate encoding | ONE dedicated pure helper `full_ui_chrome_is_visible(); `navigation_context_is_active()` remains unchanged and still used (extended-by-composition, NOT replaced) |
| Command timing | The viewport command is sent from the SAME place the flag flips (one shared toggle action used by BOTH trigger sites), so the layout that renders in the same frame is already the post-toggle one |
| Keyboard in ON | Arrow keys keep navigating; F11 is the ONLY exit from ON (all controls hidden); successful opens via arrow navigation keep fullscreen ON (no reset) |

## 1. File-Change Map

| File | Action | Line-budget note |
|---|---|---|
| `src/app.rs` | EDIT | Currently 199 lines (cap 200). The empty-state Open block (~33 lines) MOVES OUT (see row below); fullscreen additions must keep the file ≤ 200 lines |
| `src/app/empty_state_view.rs` | NEW | Verbatim move of `show_empty_state_layout` + `show_centered_open_button` + the 4 open-button consts from `app.rs` as an inline impl block. ~45 lines. Pure extraction: DELETE the originals from `app.rs` (no-commented-out-code rule) |
| `src/app/fullscreen.rs` | NEW | Flag toggle + one visibility gate + fullscreen layout + `FULLSCREEN_TOGGLE_KEY`. ~55 lines |
| `src/app/fullscreen_button.rs` | NEW | The pill's fifth button (always enabled, painted square, click → shared toggle action). ~60 lines |
| `src/app/floating_controls.rs` | EDIT | Append fifth button to the pill row (+ group-gap const + `use`). ~175 lines after edit |
| `src/app/fullscreen_tests.rs` | NEW | 7 unit tests (see §7). ~85 lines |
| `src/main.rs` | NO CHANGE | `mod app;` tree is declared inside `app.rs`; `window_options()` untouched |
| `Cargo.toml` / `Cargo.lock` | NO CHANGE | egui/eframe `=0.37.0` already pinned |

Extraction rationale: `app.rs` is parked at 199 lines; the mandated additions (field + intake + 2 dispatch methods + 4 mod lines) cannot fit under the 200-line cap, so the empty-state view moves to its own sibling module (`empty_folder_view.rs` naming precedent). This extraction is behavior-preserving — 0.3.0 layout must be byte-for-byte same behavior after the task.

## 2. Atomic Steps (ordered; each verifiable)

### Step 1 — `src/app/app.rs`: new field + init (state model, spec §4)

1. In the `ImagesScrollerApp` struct (after `current_display`, keep field order stable), add exactly one field:
```rust
    /// Session fullscreen flag (Wave-4 Task 1): ON hides the native chrome AND
    /// all UI controls; never persisted; read by the fullscreen modules.
    fullscreen_on: bool,
```
2. In `empty_state()`, add the init line to the struct literal:
```rust
            fullscreen_on: false,
```
3. Do NOT touch `window_options()` (`with_title`, `with_resizable(true)`, `with_maximized(false)` stay; nothing added).

Verification (pure, container tests land in Step 6): fresh app ⇒ flag `false`.

### Step 2 — `src/app/empty_state_view.rs`: NEW file (pure extraction)

Create `src/app/empty_state_view.rs`; MOVE (copy then remove from `app.rs` — originals must not remain commented) the following from `app.rs` completely verbatim as an inline impl block: `show_empty_state_layout` (lines 109–117), `show_centered_open_button` (lines 121–136), and the consts `EMPTY_STATE_OPEN_AREA_ID`, `OPEN_LABEL_TEXT`, `OPEN_LABEL_SIZE`, `OPEN_BUTTON_SIZE` (lines 165–168). Visibility of both methods becomes `pub(super)` (callers: `app.rs` chrome dispatch). The moved methods must not gain or lose behavior; imports needed: `use eframe::egui; use super::ImagesScrollerApp;`.

### Step 3 — `src/app/fullscreen.rs`: NEW file (toggle action + gate + layout + key)

```rust
//! Fullscreen mode (Wave-4 Task 1): the session flag state, its edge-triggered
//! viewport-command toggle, the ONE control-visibility gate, and the
//! fullscreen-only central surface. OFF restores the exact 0.3.0 layout.

use eframe::egui;

use super::ImagesScrollerApp;

/// F11: the de-facto Windows fullscreen toggle (spec §7 user-approved choice;
/// the only exit while ON, since ALL controls are hidden in ON).
pub(super) const FULLSCREEN_TOGGLE_KEY: egui::Key = egui::Key::F11;

impl ImagesScrollerApp {
    /// Toggle action shared by the F11 intake and the pill button: flips the
    /// flag and sends the matching borderless-fullscreen command exactly once,
    /// in the frame the flag flips. Returns the post-toggle flag.
    pub(super) fn toggle_fullscreen_and_send_command(
        &mut self,
        screen_context: &egui::Context,
    ) -> bool {
        let fullscreen_after_toggle = self.toggle_fullscreen();
        screen_context.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
            fullscreen_after_toggle,
        ));
        fullscreen_after_toggle
    }

    /// The ONE control-visibility gate: EVERY overlay control (pill, empty-state
    /// Open button, empty-folder message, every future body control) renders
    /// only while fullscreen is OFF. Fullscreen keeps the central image surface.
    pub(super) fn full_ui_chrome_is_visible(&self) -> bool {
        !self.fullscreen_on
    }

    /// Fullscreen layout: the central surface only; the decoded image when one
    /// is open, otherwise the BLANK surface (no pill, no Open button, no
    /// "Empty folder" message).
    pub(super) fn show_fullscreen_layout(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |central_ui| {
            if let Some(decoded_image) = self.decoded_image.as_ref() {
                self.current_display.paint(central_ui, decoded_image);
            }
        });
    }

    /// Pure flip seam: exactly one flag inversion per trigger, the post-toggle
    /// flag equals the command value sent by the shared toggle action.
    pub(super) fn toggle_fullscreen(&mut self) -> bool {
        self.fullscreen_on = !self.fullscreen_on;
        self.fullscreen_on
    }
}
```

Notes:
- `decoded_image`/`current_display`/`fullscreen_on` are struct fields of `ImagesScrollerApp` (private in `app` but visible to descendant modules — same access pattern as `tests.rs`, `floating_controls.rs`, `folder_navigation.rs`). Child module impl blocks on parent types are the established pattern (`folder_navigation.rs`).
- `if let ... else`-less variant above compiles under edition 2024 disjoint closure capture (identical borrow pattern as `show_navigation_layout`'s `Ok` arm).
- `send_viewport_cmd` — verified 0.37 method name (spec §2). The command must NOT be sent per frame; it fires only inside `toggle_fullscreen_and_send_command`, which both trigger sites call exactly once per trigger.

### Step 4 — `src/app/app.rs`: intake + dispatch + mod registrations

4a. Replace the whole body of `impl eframe::App::ui` with (arrow-key intake verbatim unchanged, F11 added FIRST so the toggle frame renders the post-toggle layout):

```rust
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let fullscreen_toggle_pressed = ui
            .ctx()
            .input(|i| i.key_pressed(fullscreen::FULLSCREEN_TOGGLE_KEY));
        if fullscreen_toggle_pressed {
            self.toggle_fullscreen_and_send_command(ui.ctx());
        }
        let left_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowLeft));
        let right_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowRight));
        let up_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowUp));
        let down_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowDown));
        if left_pressed {
            self.navigate(NavigationDirection::Previous);
        }
        if right_pressed {
            self.navigate(NavigationDirection::Next);
        }
        if up_pressed {
            self.navigate(NavigationDirection::PreviousSiblingFolder);
        }
        if down_pressed {
            self.navigate(NavigationDirection::NextSiblingFolder);
        }
        self.show_chrome_or_fullscreen_layout(frame, ui);
    }
```

Intake contract: the F11 check is in the SAME per-frame intake block as the arrows and evaluates on every frame in ALL modes (fresh empty state, image mode, empty-folder mode) — it is BEFORE the layout dispatch, so no mode can gate it out. `key_pressed` is inherently edge-triggered (true one frame per physical press — no re-toggle while held). Arrow navigation keeps working while ON.

4b. Add inside the primary `impl ImagesScrollerApp`:

```rust
    /// Frame layout dispatch: fullscreen ON runs ONLY the central surface; OFF
    /// runs the exact 0.3.0 layouts (pill + central surface, or the centered
    /// Open button in the fresh empty state).
    fn show_chrome_or_fullscreen_layout(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        if self.full_ui_chrome_is_visible() {
            self.show_chrome_layout(frame, ui);
        } else {
            self.show_fullscreen_layout(ui);
        }
    }

    /// The 0.3.0 chrome layouts, unchanged (navigation-context gate preserved).
    fn show_chrome_layout(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        if self.navigation_context_is_active() {
            self.show_navigation_layout(ui);
        } else {
            self.show_empty_state_layout(frame, ui);
        }
    }
```

Both conditions are single-section method calls (single-section-boolean rule; nesting depth 1 — max-depth rule satisfied). `navigation_context_is_active()` itself is UNTOUCHED in `folder_navigation.rs` — the fullscreen gate is a separate dedicated pure helper (`full_ui_chrome_is_visible`) composed above it.

4c. Remove the moved empty-state code + consts (Step 2 originals) and register modules near the existing child-module block at the bottom of `app.rs`:

```rust
mod empty_state_view;

mod fullscreen;

mod fullscreen_button;

#[cfg(test)]
mod fullscreen_tests;
```

Line-budget check after Steps 1–4: `app.rs` ≈ 185 lines (≤ 200; if fmt pushes past, do NOT delete behavior — run the harness fmt and re-count; any overflow problem is reported to the caller, never silently reshaped).

### Step 5 — `src/app/fullscreen_button.rs`: NEW file (fifth pill button)

```rust
//! The floating pill's FIFTH button: the fullscreen toggle (user-approved
//! Option A placement). Painted filled-square icon (the painted-shape family
//! of the vector-triangle arrows; font glyphs are avoided). The button is
//! ALWAYS enabled in the normal chrome state -- it is the only viewport-local
//! control, and it is not rendered while fullscreen is ON.

use eframe::egui;

use super::ImagesScrollerApp;

const FULLSCREEN_BUTTON_LABEL: &str = "Toggle fullscreen";
const FULLSCREEN_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const FULLSCREEN_SQUARE_SIZE: egui::Vec2 = egui::Vec2::new(10.0, 10.0);

/// Appends the ALWAYS-ENABLED fullscreen toggle button to the pill row.
pub(super) fn show_fullscreen_pill_button(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    let (_, button_response) =
        row_ui.allocate_exact_size(FULLSCREEN_BUTTON_SIZE, egui::Sense::click());
    button_response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::Role::Button, true, FULLSCREEN_BUTTON_LABEL)
    });
    let painted_button = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
    paint_fullscreen_square(row_ui, &painted_button);
    if painted_button.clicked() {
        app.toggle_fullscreen_and_send_command(row_ui.ctx());
    }
}

fn paint_fullscreen_square(row_ui: &egui::Ui, button_response: &egui::Response) {
    let square_rect =
        egui::Rect::from_center_size(button_response.rect.center(), FULLSCREEN_SQUARE_SIZE);
    let square = egui::Shape::rect_filled(
        square_rect,
        egui::CornerRadius::ZERO,
        fullscreen_square_color(row_ui, button_response),
    );
    row_ui.painter().add(square);
}

/// Default egui button-text palette: inactive idle, brightened on hover
/// (the enabled-arrow palette minus the disabled branch -- always enabled).
fn fullscreen_square_color(row_ui: &egui::Ui, button_response: &egui::Response) -> egui::Color32 {
    if button_response.hovered() {
        return row_ui.visuals().widgets.hovered.fg_stroke.color;
    }
    row_ui.visuals().widgets.inactive.fg_stroke.color
}
```

Facts baked in (all verified against egui 0.37.0): `Rect::from_center_size`, `Response::widget_info(closure)` + `WidgetInfo::labeled(Role::Button, enabled: bool, &'static str)` (same call shape as the arrows in `floating_controls.rs`), `Shape::rect_filled(Rect, impl Into<CornerRadius>, impl Into<Color32>)`, `CornerRadius::ZERO`, `CursorIcon::PointingHand`. The click calls the SAME shared toggle action as F11 (flag flip + one command send in the same frame — `row_ui.ctx()` is the same `Context` as the app's `ui`).

### Step 6 — `src/app/floating_controls.rs`: append the fifth button to the row

1. Add import: `use super::fullscreen_button;`
2. Add const (next to `FOLDER_GROUP_GAP`): `const FULLSCREEN_GROUP_GAP: f32 = 16.0;` (visually separates the fullscreen button from the `[↑][↓]` folder group, mirroring the two folder-subgroup gap).
3. Rewrite `show_button_row`:

```rust
fn show_button_row(app: &mut ImagesScrollerApp, pill_ui: &mut egui::Ui) {
    let button_specs = arrow_button_specs(app);
    pill_ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |row_ui| {
        for button_spec in &button_specs {
            add_folder_group_gap(row_ui, button_spec.control);
            button_spec.show_and_navigate(app, row_ui);
        }
        row_ui.add_space(FULLSCREEN_GROUP_GAP);
        fullscreen_button::show_fullscreen_pill_button(app, row_ui);
    });
}
```

Row order becomes: `[←] [→] | [↑] [↓] | [□]` (square = painted fullscreen icon). Nothing else in this file changes; the module doc's first line gains `+ the fifth fullscreen toggle button` wording (self-documenting-code rule; do not leave the stale "four painted arrow buttons" text).

### Step 7 — `src/app/fullscreen_tests.rs`: NEW test module (7 tests)

Style: match the existing repo test style exactly — plain descriptive `#[test]` fn names with NO `test_` prefix (see `src/app/tests.rs` / `floating_controls_tests.rs`), placed in a sibling child module of `app`. Fixtures/imports to use: `use super::tests::{TempImageFolder, sample_decoded_image};`, `use super::fullscreen::FULLSCREEN_TOGGLE_KEY;`, `use super::ImagesScrollerApp;`, `use eframe::egui;`.

Expected tests (7 — container suite should report exactly +7, i.e. 57 passing):

1. `fresh_app_starts_with_fullscreen_off` — `ImagesScrollerApp::empty_state()`; assert `!app.fullscreen_on`; assert `app.full_ui_chrome_is_visible()`.
2. `fullscreen_shortcut_is_f11` — pins the user-approved decision: `assert_eq!(FULLSCREEN_TOGGLE_KEY, egui::Key::F11)`.
3. `each_toggle_trigger_flips_the_flag_exactly_once` — `app.toggle_fullscreen()` returns `true` and the field is `true`; a second trigger returns `false` and the field is `false`; a third returns `true` again. (The exact-once-per-trigger contract: each trigger performs exactly one inversion and the returned value is the command's `Fullscreen(bool)` payload.)
4. `fullscreen_on_hides_the_chrome_with_an_image_open` — open via `accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()))`; `full_ui_chrome_is_visible()` true; toggle → false (pill/Open/status hidden even though context is active).
5. `fullscreen_off_restores_the_chrome_in_every_reachable_mode` — two reachable modes asserted in one test: fresh empty state (gate `true` via `empty_state()`), and navigation context (open an image via `accept_open_outcome`; gate `true`); plus one double-toggle pass starting fresh: toggle ON (gate `false`) then OFF (gate `true` again) with no image open, proving OFF fully restores the empty-state chrome.
6. `navigation_context_stays_active_across_fullscreen_toggles` — with an image open, `navigation_context_is_active()` stays `true` after toggle ON and after toggle OFF (the 0.3.0 gate is untouched by the flag).
7. `successful_open_during_fullscreen_keeps_fullscreen_on` — toggle to ON, then a successful `accept_open_outcome`; assert chrome gate still `false` and `decoded_image.is_some()` and `fullscreen_on` still `true` (no reset on open/navigation success).

Rule completions to hand-check before finishing the file: each `fn` body ≤ 50 lines, whole file ≤ 200 lines, no methods > 2 params (none needed), `pub(super)` nothing (all tests private fns), no commented-out code.

### Step 8 — Pre-commit conformance pass (read-over list)

Read-over list for the implementer BEFORE the harness run (all rules):
- `*.rs` files touched/created: `app.rs`, `floating_controls.rs` ≤ 200 lines; new modules ≤ 200 and ideally ≤ 125 non-blank/non-comment lines.
- No commented-out code anywhere (moved code REMOVED, not commented).
- Private-by-default: only `pub(super)` items named above; nothing `pub`.
- Manual `pnpm`-style spot check not needed — the harness runs fmt/clippy; run the harness EARLY (§5) to surface drift.

## 3. What Each binding TODO sub-item maps to

| TODO Task 1 sub-item | Plan location |
|---|---|
| Default OFF at launch, no forced size, never auto-maximized/fullscreen | Step 1.3 (`window_options` untouched) + Step 3 field init |
| ON hides native title bar/borders AND all controls | Step 3 (`ViewportCommand::Fullscreen`) + gate `full_ui_chrome_is_visible` + dispatch (Step 4b) |
| OFF restores all controls; image central, almost all space | Step 4b `show_chrome_layout` (0.3.0 paths preserved); central image unchanged |
| Toggle via keyboard AND UI control | Step 4a (F11 intake) + Steps 5–6 (fifth pill button) — same shared action (Step 3) |

## 4. Build / Verify (harness — run by the 4.2 implementer; Task 4 formally owns the full matrix)

Environment rules (binding): all Rust/Cargo runs INSIDE the container on the Alpine VM via `alpine-vm` MCP; the host has no Rust/Cargo.

1. `vm_status` — ALWAYS first; if the VM or `/img-scroller` share is unavailable, STOP and return the question to the caller.
2. Dev checks (fmt → check → test → clippy; authoritative success = exit code + literal `ALL CHECKS PASSED`):
```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
```
Expected: `ALL CHECKS PASSED`, 57/57 unit tests (50 existing + 7 new).
3. If fmt drifts (known Windows CRLF working tree): rerun the check; do NOT hand-fix formatting locally.
4. Exe rebuild + DLL audit + OPERATOR-RUN are Task 4's own sub-items (`scripts/build-windows.sh` → `dist/windows-x64/img-scroller.exe`; the 22-DLL import audit constant must be unchanged) — OUT of this plan's steps; do not run them in Task 1's cycle.
5. GUI/runtime facts (borderless fullscreen on glow/Windows, F11 one-key, odd Fn-key laptops, minimized-restore) are OPERATOR-RUN only — agent-side GUI execution impossible per docs.

## 5. Git actions (owner: 4.2 implementer + caller as noted)

- BEFORE any commit: `git status` + read `.gitignore`; stage ONLY source/plan files touched by this task; never force-add; `target/` stays ignored; no `dist/`/`logs/` staging.
- Commit 1 (caller, immediately after this plan is approved): `docs: add wave-4 task1 fullscreen implementation plan` — `--only .kilo/plans/20261102-wave-4-task1-fullscreen-plan.md` (NEVER edits any existing `.kilo/plans/*` file).
- Commit 2 (4.2 implementer, after dev-checks `ALL CHECKS PASSED`):
```
feat: fullscreen toggle via f11 and fifth pill button
```
covering all files in §1 (one feature commit; do not fragment across steps unless asked).
- Review-fix commits (only if 4.3 raises items): `fix: fullscreen review fixes (<short list>)`.
- TODO `[DONE]` mark commit at 4.6, `task: mark wave-4 task1 fullscreen done`, preserving the rest of the TODO file byte-for-byte (overwrite-todo rule).
- Push: NEVER in these steps (push restricted to workflow Step 5, `origin` only).

## 6. Docs touchpoints (Task 5 (4.1b: list only; Task 5 owns every edit — do NOT touch in Task 1's code steps))

- `README.md` → Controls/Usage section: F11 toggle + fifth pill button; fullscreen hides all UI; only exit via F11 while ON.
- `.agent/project-info/tech.md` → user-facing behavior/technology notes: `ViewportCommand::Fullscreen` mechanism on the glow backend (Windows 10/11), session-only state, no persistence.
- `.agent/project-info/architecture.md` → app-shell description: new modules `empty_state_view.rs`, `fullscreen.rs`, `fullscreen_button.rs`, `fullscreen_tests.rs`; the one visibility gate; dispatch shape.
- `.agent/project-info/context.md` → Recent Changes + Current Project State + Immediate Next Steps (Task 1 landed, counts).
- `.agent/project-structure.md` → file-set change (4 new `src/app/*` files + `app.rs` description update: navigation gate, empty-state module moved out, fullscreen dispatch) per the Project Structure workflow.
- Explicitly NOT touched: `CHANGELOG.md`, any existing `.kilo/plans/*` file.

## 7. Out of scope guard (hard)

- Zoom in/out (Wave-4 Task 2), fit width/fit height (Wave-4 Task 3), zoom↔fit interplay — no fields, no math, no buttons.
- Wave-5 pan-then-navigate behavior.
- Parked future idea (subfolder picker on image-less root folder, `future-ideas.md`) — no code, no TODO change.
- Multi-monitor `ViewportCommand::SetMonitor` (spec §12).
- `CHANGELOG.md`; existing `.kilo/plans/*` edits; `window_options()`/`NativeOptions` negotiation (launch-time options stay frozen).
- No persistence anywhere (no config/registry/file state).

## 8. 4.5a conformance cross-check (this plan → spec checklist, all pre-satisfied)

Spec §11 items 1–10 map to: §0 table (items 1–3), Steps 3–4 (item 4–5: one send per toggle frame, both directions), Steps 4b/3 gate + Step 6 hiding (item 6–7), Step 4a (`ui.ctx().input` edge-triggered intake, no arrow interplay — item 8), Step 6 (user-approved Option A — item 9); item 10 (texture upload discipline) — no display code touched (`app_display.rs` untouched).

## 9. Risks / deviations (explicit)

- NO deviations from the approved spec/global plan/user decisions were made. All differences are implementation-side only: (a) files split (`empty_state_view.rs`, `fullscreen_button.rs`, `fullscreen.rs`) purely to honor the 200-line cap — behavior unchanged; (b) the spec's recommendation F11 + Option A pill button + `rect_filled` square icon are encoded with verified egui 0.37.0 API names (`Key::F11`, `Shape::rect_filled` + `CornerRadius::ZERO`, `WidgetInfo::labeled`, `send_viewport_cmd`). If `Shape::rect_filled`'s `impl Into<CornerRadius>` inference trips clippy's pedantic suggestions during the harness run, the fallback is `egui::Shape::convex_polygon(four corner points, color, egui::Stroke::NONE)` — same painted solid square; encode NO other variant.

## 10. Pre-commit checklist (execute before EVERY commit)

1. Read `.gitignore` (in-container change does not alter it) + `git status` — no ignored files staged (watch `target/`, `dist/`, `logs/`).
2. Harness run green: `vm_status` first, then the dev-checks command of §4 step 2 → literal `ALL CHECKS PASSED` recorded in the report.
3. `git diff --stat` reviewed — only §1 files changed (+ the plan file for commit 1).
4. Commit messages verbatim as §5; then `git status` clean again.