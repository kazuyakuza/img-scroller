# Wave 6 Task 3 — "Open with..." Menu UI Implementation Plan

**Date:** 2026-11-13
**Branch:** `feat/wave-6-open-with` (current; branch creation happened at Step 2 — NO branch actions here)
**Version target:** 0.6.0 (already bumped at Step 3 — NO version actions here)
**Step:** Critical Workflow 4.1b (architector) for TODO Task 3 of Wave 6, cycle B
**Source TODO:** `.agent/todos/20261029/20261029-todo-8.md` → `### Task 3: Menu entry in the UI`
**Front-end input (4.1a spec):** `.kilo/plans/20261112-wave6-task3-open-with-menu-frontend-spec.md`
**Global plan:** `.kilo/plans/20261112-wave6-open-with.md` → `### Cycle B — TODO Task 3 (menu entry in the UI) — FRONT-END`
**Cycle-A landed sources consumed:** `src/open_with_listing.rs`, `src/platform/mod.rs`, `src/platform/windows_open_with.rs`, `src/platform/stub.rs`
**Landed UI sources modified:** `src/app.rs`, `src/app/floating_controls.rs`, `src/app/button_hint.rs`, `src/app/folder_navigation.rs`, `src/platform/mod.rs`
**Base state:** HEAD `84613cc`, dev-checks `ALL CHECKS PASSED`, census **170 unit tests / 27 test files**, version 0.6.0.
**Implementer:** JUNIOR developer under 50% restriction — every structural/architectural choice below is BINDING; do not re-derive or substitute any of it.

---

## §0 Approved decisions (binding propose-and-ask resolution — DO NOT re-open)

The user approved the following via the Planner (propose-and-ask); this plan encodes them exactly:

| # | Approved decision | Encoded value |
|---|---|---|
| A1 | PLACEMENT = Option A: an **11th pill button at the right end** of the floating controls row, custom painted vector glyph (square with an emerging arrow) | §2 D1, D7 |
| A2 | Popup = **above the pill** via egui 0.37 `egui::Popup::menu(&button_response).align(egui::RectAlign::TOP)` | §2 D2 |
| A3 | EMPTY-RESULT BEHAVIOR = the menu button renders **DISABLED with tooltip `"No programs registered for this file type"`** when the system has no registered programs for the current file's type | §2 D3 |
| A4 | Menu entries show `RegisteredProgram::display_name` **verbatim** | §2 D4 |
| A5 | Long lists scroll with `egui::ScrollArea::vertical().max_height(300.0)` | §2 D4 |
| A6 | Launch failures surface `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` in the pill's message line | §2 D6 |
| A7 | **NO dedicated keyboard hotkey in v1** (no caption, no intake key) | §2 D9 |
| A8 | Enumeration→popup wiring: **WHEN the popup opens, enumerate via the facade**; click → launcher seam with the current canonical `opened_image_path` + `launch_index` | §2 D5, D6 |

TODO Task 3 verbatim check:
- *"Add the "Open with..." menu entry/portal in the viewer UI; placement follows the propose-and-ask instruction …"* → placement was proposed (spec §4 Option A) and approved (A1/A2); §2 encodes it.
- *"Behavior when NO programs are registered for the type is decided in this wave's architector step."* → decided = A3 (user-approved); §2 D3 encodes it.

---

## §1 High-level approach

One new pill button module renders the 11th button and drives an upward `egui::Popup::menu`. The app shell gains ONE private display-state field (`open_with_programs: Vec<RegisteredProgram>`) refreshed at exactly TWO points — every successful open (so the button's disabled state is correct from the first render, approved A3) and the button-click frame that opens the popup (so the menu enumerates WHEN OPENED, approved A8). The popup contents read the stored list only (the closure runs per frame while the popup is open — NO per-frame OS enumeration). Entry clicks hand the CURRENT canonical file to the platform launcher keyed by `launch_index` (Cycle-A contract: the launcher re-enumerates fresh; nothing is cached at the service layer). A launch failure writes the single static failure message into the pill's message line.

No changes to: keyboard intake, the central surface, the display paint path, `Cargo.toml`, `Cargo.lock` (zero new dependencies — the Popup/ScrollArea APIs are core egui 0.37, already in the pinned stack), `main.rs`, `app_display.rs`, `central_surface.rs`, `open_with_listing.rs`.

## §2 Pinned design decisions (architector encodings)

- **D1 — Placement/row position:** rightmost button in the pill row, AFTER the fit-width/fit-height group, preceded by a 16-pt group gap (mirrors `FULLSCREEN_GROUP_GAP`/`FIT_GROUP_GAP`). Row order becomes: `[←][→] | [↑][↓] | [fullscreen] | [+][−][restore-fit] | [W][H] | [Open-with]` (11 buttons).
- **D2 — Popup mechanics (verified against docs.rs egui 0.37.0 `Popup`):** `egui::Popup::menu(&button_response)` (opens when the widget is clicked; layout `top_down_justified(Align::Min)`), `.align(egui::RectAlign::TOP)` (default placement centered ABOVE the button; egui falls back to `RectAlign::symmetries` then `RectAlign::MENU_ALIGNS` automatically when it does not fit), `.accessible_name("Open with...")`, then `.show(|menu_ui| { ... })` — NOTE: in egui 0.37 `Popup::show` takes NO parent-Ui argument (`pub fn show<R>(self, content: impl FnOnce(&mut Ui) -> R) -> Option<InnerResponse<R>>`); it derives everything from the response. The `Option` return is intentionally discarded. Menu closes automatically on item click, outside click, and Esc — no manual state.
- **D2b — Popup styling:** egui's DEFAULT menu frame/style (`Popup::menu` already applies `menu_style`). NO custom frame/stroke overrides — the pill's translucent-dark frame is the pill overlay's identity, not the popup's; customizing it would add surface with no approved requirement.
- **D3 — Empty-result / state gates (all four):**
  - Fresh empty state: the pill itself is absent → nothing renders (no code).
  - Image open + stored list non-empty → button ENABLED, tooltip `"Open with..."`.
  - Image open + stored list EMPTY → button renders DISABLED (dimmed glyph, no popup attached, clicks inert) with hover tooltip `"No programs registered for this file type"` (approved A3).
  - Empty-folder browsed mode (`image_is_open()` false) → button renders DISABLED with the STANDARD tooltip `"Open with..."` (decided: disabled, NOT hidden — spec §4 rule; the no-programs tooltip would be misleading because no file is open).
  - Fullscreen ON → `full_ui_chrome_is_visible()` hides the whole pill → the button never renders (verify-only, §5 V9).
- **D4 — Menu contents:** entries render `scroll_ui.button(display_name)` verbatim (no icons, no re-sort, no truncation); the entry loop is wrapped in `egui::ScrollArea::vertical().max_height(300.0)`. All entries enabled; one click = one launch.
- **D5 — Enumeration/refresh contract (exactly two points, both decided):**
  1. Every successful open (`accept_open_outcome` success arm): `self.open_with_programs = platform::enumerate_open_with_programs(&path);` — seeds the display list so the approved disabled-button behavior is correct from the first render. (Spec §3 wording said "only on successful open"; the approved A8 wiring ADDS point 2 — this is the recorded, deliberate adjustment.)
  2. The button-click frame that opens the popup: re-enumerate via the facade and REPLACE the stored list, so the menu lists a FRESH enumeration WHEN OPENED. The popup contents closure then reads the stored list only.
  - NO per-frame enumeration anywhere; NO service-layer cache (the launcher re-enumerates fresh per hand-off — Cycle-A contract unchanged).
  - Accepted micro-edge 1: a second button click (popup toggle-close) also re-enumerates — harmless (one OS call per explicit user click).
  - Accepted micro-edge 2: if the system lost its registrations between open and popup-open, the fresh list is empty → the popup opens with no entries (dismiss via outside click/Esc) and the button renders disabled from the next frame (self-correcting).
- **D6 — Launch hand-off:** entry click → `app.launch_open_with_program(program_index)` → `platform::open_file_with_selected_program(&self.opened_image_path.clone(), program_launch_index)`; on `Err` → `self.message_text = Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)` (replaces whatever message was showing — the single message surface, same pattern as `ImageLoadFailure`); on success → NO state change (popup auto-closes on item click; the app stays on the current image).
- **D7 — Glyph geometry (painted via `pill_style.rs` outlined-shape builders; constants in the new module):**
  - Button: `36.0 × 32.0` points (same as all pill buttons); glyph center = button rect center.
  - Square: `10.0 × 10.0` points, painted with `pill_style::outlined_rect_shape` (WHITE enabled fill, 1-pt BLACK outside outline; disabled palette via `glyph_state_for`/`glyph_state_color`), center offset from the button center by `(-4.0, +4.0)` so the up-right emergence stays inside the button.
  - Arrow shaft: 7-point line at 45° up-right from the square's top-right corner: rise = `7.0 * std::f32::consts::FRAC_1_SQRT_2`; painted as TWO `egui::Shape::line_segment` calls — first the BLACK under-stroke of width `2.5 + 2.0 * OUTLINE_STROKE_WIDTH` (= 4.5), then the glyph-color stroke of width 2.5 (the outline-along-a-diagonal technique; every painted pill glyph keeps its 1-pt black outline, Wave-4.2 rule).
  - Arrow head: `pill_style::outlined_triangle_shape` with a `6.0 × 6.0` rect centered at the shaft endpoint, rotation `-std::f32::consts::TAU / 8.0` (up-right in the `ArrowDirection` rotation space).
  - Paint order: square → shaft (black underlay, then white) → head.
- **D8 — Tooltip scheme:** `button_hint.rs` gains `PillButton::OpenWith` with tooltip `"Open with..."` and `hotkey_reference(OpenWith) == None`; `PillButtonPlacement` gains an OPTIONAL `disabled_tooltip` override consumed by `show_button_column` with a STATE-AWARE hover-text selection (painted-disabled buttons get the override; enabled buttons keep the table tooltip; all other 10 buttons keep identical behavior because their override is `None`).
- **D9 — No hotkey:** no caption (`hotkey_reference` → None), NO keyboard-intake change, no `keyboard_intake.rs` edit.
- **D10 — Module split:** ONE new production module `src/app/open_with_button.rs` (button paint + popup wiring + the `ImagesScrollerApp` seam methods for this feature — precedent: `src/app/pan.rs` owns controller + state writes) and ONE new test module `src/app/open_with_button_tests.rs`. Exact names — do not rename.
- **D11 — Stale-annotation cleanup:** `src/platform/mod.rs`'s two `#[allow(unused_imports)]` attributes on the open-with re-exports are removed and its comment updated (both re-exports become consumed by this task; the file's own comment anticipated this). `src/open_with_listing.rs` is UNTOUCHED (its `#[allow(dead_code)]` set stays — `programs_from_handler_names` and `OpenWithLaunchFailure::user_message` remain unconsumed on the container target and would warn if the allows were removed).
- **D12 — Test census:** 7 new pinned tests → **177 unit tests / 28 test files** (spec estimated 176 ±1; the delta is the pinned list in §4). Button-hint tests get extended assertions with NO count change.

## §3 Implementation steps (execute in order)

**Step 0 — Pre-flight (STOP conditions).**
1. Run `git status` and `git branch --show-current`. Expected branch: `feat/wave-6-open-with`; expected tree: clean except the Task-3 planning artifacts (`.kilo/plans/20261112-wave6-task3-open-with-menu-frontend-spec.md` and this plan file — both untracked). If the branch differs or OTHER unexpected changes exist → STOP and report to the caller (branch creation is Step-2-restricted; do not fix anything here).
2. Read `.gitignore` (expects `dist/`, `logs/`, `*.log`, `target/` among its entries). No `.gitignore`-matching file may ever be staged.

**Step 1 — Commit the planning artifacts (docs commit).**
- Stage `.kilo/plans/20261112-wave6-task3-open-with-menu-implementation-plan.md` and, if still untracked, `.kilo/plans/20261112-wave6-task3-open-with-menu-frontend-spec.md` (both are this task's planning artifacts; nothing else).
- Verify with `git status` that ONLY those `.md` files are staged (Gitignore Compliance Rule).
- Commit: `docs: add wave6 task3 open-with-menu implementation plan`

**Step 2 — `src/app/button_hint.rs` (exact edits).**
1. In `enum PillButton`, append the variant `OpenWith,` after `FitHeight,`.
2. In `hotkey_reference`, extend the first match arm's pattern list: `PillButton::ImagePrevious | PillButton::ImageNext | PillButton::FolderUp | PillButton::FolderDown | PillButton::OpenWith => None,`.
3. In `tooltip_text`, add the arm `PillButton::OpenWith => "Open with...",` (before the closing brace).
4. In `struct PillButtonPlacement`, add the field `disabled_tooltip: Option<&'static str>,` after `enabled: bool,`; in `for_button`'s struct literal add `disabled_tooltip: None,`; after `with_enabled` add:
```rust
    /// Overrides the disabled-hover tooltip (the no-programs empty-result
    /// case); `None` keeps the button's standard tooltip in both states.
    pub(super) fn with_disabled_tooltip(self, disabled_tooltip: Option<&'static str>) -> Self {
        Self { disabled_tooltip, ..self }
    }
```
5. In `show_button_column`, replace the two lines
```rust
            let tooltip = tooltip_text(button);
            let painted_button = painted_button
                .on_hover_text(tooltip)
                .on_disabled_hover_text(tooltip);
```
with
```rust
            let tooltip = tooltip_text(button);
            let painted_tooltip = if placement.enabled {
                tooltip
            } else {
                placement.disabled_tooltip.unwrap_or(tooltip)
            };
            let painted_button = painted_button
                .on_hover_text(painted_tooltip)
                .on_disabled_hover_text(painted_tooltip);
```
(State-aware because egui's `on_hover_text` fires on hover regardless of the painted-disabled state of these manually-allocated responses; all other buttons are behavior-identical with `None` overrides.) Expected file total ≈ 181 lines (cap 200 ✓).

**Step 3 — `src/app.rs` (state + registration; exact edits).**
1. Add the import `use crate::open_with_listing::RegisteredProgram;` between the `last_viewed_images` and `platform` import lines.
2. After the `pan_offset` field add:
```rust
    /// Open-with display list for the currently opened file (Wave-6 Task 3):
    /// refreshed on every successful open and on the popup-opening click;
    /// display-state only (the launcher re-enumerates fresh per hand-off).
    open_with_programs: Vec<RegisteredProgram>,
```
3. In `empty_state()` after `pan_offset: egui::Vec2::ZERO,` add `open_with_programs: Vec::new(),`.
4. In `accept_open_outcome`'s success arm, insert immediately BEFORE `self.opened_image_path = Some(path);` (the move of `path` must come after the borrow):
```rust
                self.open_with_programs = platform::enumerate_open_with_programs(&path);
```
The failure arm is UNTOUCHED (a failed open keeps the previously displayed image AND its list).
5. Keep `app.rs` ≤ 200 lines by compressing four field doc comments (exact replacements; saves 7 lines against +9 added → 199 total):
   - `fullscreen_on` (3→1): `/// Session fullscreen flag: ON hides the native chrome and ALL UI controls; never persisted.`
   - `zoom_factor` (3→1): `/// Session zoom factor (1.0 = whole-image fit); reset on every successful open; never persisted.`
   - `esc_close_popup_expires_at` (4→2): first line `/// ESC-close popup expiry deadline (Wave 4.2): `None` = popup not visible;`, second line `/// a value = the absolute `InputState::time` (seconds) when it dismisses.`
   - `pan_offset` (3→2): first line `/// Pan offset from the centered position (Wave 5); zeroed on every`, second line `/// successful open and on restore-to-fit; never persisted.`
6. In the module declarations, add `mod open_with_button;` between `mod keyboard_intake;` and `mod pan;` (alphabetical). In the `#[cfg(test)]` block, add between `mod last_viewed_tests;` and `mod pan_tests;`:
```rust
#[cfg(test)]
mod open_with_button_tests;
```

**Step 4 — `src/app/folder_navigation.rs` (clear on empty-folder entry).**
In `enter_empty_folder_mode`, insert immediately after `self.current_folder_listing = None;`:
```rust
        self.open_with_programs.clear();
```
(→ 172 lines.)

**Step 5 — `src/platform/mod.rs` (consume the re-exports).**
Remove BOTH `#[allow(unused_imports)]` attributes from the two open-with re-export `use` statements and replace the comment `// Consumed by the Task-3 menu UI; the allow silences the unused re-export until then.` with `// Consumed by the Task-3 menu UI (src/app/open_with_button.rs).` (→ 35 lines). Touch NOTHING else in `src/platform/`.

**Step 6 — `src/app/floating_controls.rs` (row integration).**
1. Add `use super::open_with_button;` after `use super::fullscreen_button;`.
2. In `show_button_row`, after `fit_button::show_fit_pill_buttons(app, row_ui);` add:
```rust
        open_with_button::show_open_with_pill_button(app, row_ui);
```
3. Reword the module doc's first sentence to include the new button (`…plus the one-shot fit-width/fit-height buttons, and the open-with menu button`) without adding lines. (→ 186 lines.)

**Step 7 — NEW `src/app/open_with_button.rs` (full skeleton; pin names, constants, and doc headers; expected ≈ 180–190 lines, cap 200 — if it lands over 200, shorten ONLY doc-comment text, never the code structure).**
```rust
//! The floating pill's 11th button (Wave-6 Task 3, user-approved Option A):
//! the "Open with..." entry, painted as a square-with-emerging-arrow vector
//! glyph in the shared pill family. The enabled button opens an upward
//! `egui::Popup::menu` listing the programs the system registered for the
//! current file's type; entries show `RegisteredProgram::display_name`
//! verbatim inside a 300-point scroll area; a click hands the CURRENT
//! canonical file to the platform launcher keyed by `launch_index`, and a
//! launch failure surfaces `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` in the pill's
//! message line. Enumeration contract (decided): the display list is
//! refreshed on every successful open and once on the click frame that opens
//! the popup (the menu enumerates WHEN OPENED); the popup contents read the
//! stored list only -- no per-frame OS enumeration. Empty enumeration renders
//! the button DISABLED with the no-programs tooltip; in empty-folder mode the
//! button renders disabled with the standard tooltip; no dedicated hotkey
//! exists in v1 (no caption).

use crate::open_with_listing::OPEN_WITH_LAUNCH_FAILURE_MESSAGE;
use crate::platform;

use eframe::egui;

use super::ImagesScrollerApp;
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
use super::pill_style;

pub(super) const NO_PROGRAMS_TOOLTIP_TEXT: &str = "No programs registered for this file type";
const OPEN_WITH_BUTTON_LABEL: &str = "Open with...";
const OPEN_WITH_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const OPEN_WITH_GROUP_GAP: f32 = 16.0;
const OPEN_WITH_MENU_MAX_HEIGHT: f32 = 300.0;
const OPEN_WITH_SQUARE_SIDE: f32 = 10.0;
const OPEN_WITH_SQUARE_CENTER_OFFSET: egui::Vec2 = egui::Vec2::new(-4.0, 4.0);
const OPEN_WITH_ARROW_LENGTH: f32 = 7.0;
const OPEN_WITH_ARROW_TRIANGLE_SIZE: egui::Vec2 = egui::Vec2::new(6.0, 6.0);
const OPEN_WITH_ARROW_SHAFT_THICKNESS: f32 = 2.5;
const OPEN_WITH_ARROW_SHAFT_OUTLINE_WIDTH: f32 =
    OPEN_WITH_ARROW_SHAFT_THICKNESS + 2.0 * pill_style::OUTLINE_STROKE_WIDTH;
const OPEN_WITH_ARROW_HEAD_ROTATION: f32 = -std::f32::consts::TAU / 8.0;

/// Appends the open-with button to the pill row, separated by the standard
/// group gap after the fit group (user-approved rightmost position).
pub(super) fn show_open_with_pill_button(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    row_ui.add_space(OPEN_WITH_GROUP_GAP);
    let button_spec = OpenWithButtonSpec::for_app(app);
    button_spec.show_and_open_menu(app, row_ui);
}

/// Per-frame button request: the painted enablement plus the disabled-tooltip
/// override. Buttons store no state between frames.
struct OpenWithButtonSpec {
    enabled: bool,
    disabled_tooltip: Option<&'static str>,
}

impl OpenWithButtonSpec {
    fn for_app(app: &ImagesScrollerApp) -> Self {
        Self {
            enabled: app.open_with_menu_is_available(),
            disabled_tooltip: no_programs_tooltip_for(app),
        }
    }

    /// Paints the glyph; the click frame re-enumerates the program list (the
    /// menu enumerates WHEN OPENED) and the enabled button drives the upward
    /// menu popup. A disabled button paints dimmed and never opens one.
    fn show_and_open_menu(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let placement = PillButtonPlacement::for_button(
            PillButton::OpenWith,
            OPEN_WITH_BUTTON_SIZE,
        )
        .with_enabled(self.enabled)
        .with_disabled_tooltip(self.disabled_tooltip);
        let button_response = show_button_column(
            ButtonColumnSite::new(row_ui, placement),
            |column_ui| self.allocate_and_paint(column_ui),
        );
        self.enumerate_when_menu_opens(app, &button_response);
        if self.enabled {
            egui::Popup::menu(&button_response)
                .align(egui::RectAlign::TOP)
                .accessible_name(OPEN_WITH_BUTTON_LABEL)
                .show(|menu_ui| app.show_open_with_menu_entries(menu_ui));
        }
    }

    fn allocate_and_paint(&self, column_ui: &mut egui::Ui) -> egui::Response {
        let (_, button_response) =
            column_ui.allocate_exact_size(OPEN_WITH_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, OPEN_WITH_BUTTON_LABEL)
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_glyph(column_ui, &button_response);
        button_response
    }

    /// A disabled open-with button never fires (the disabled-arrow pattern).
    fn accepts_click(&self, button_response: &egui::Response) -> bool {
        self.enabled && button_response.clicked()
    }

    /// The decided refresh contract: on the click frame that opens the popup,
    /// re-enumerate via the facade and replace the stored list, so the menu
    /// lists a fresh enumeration WHEN OPENED. The popup contents then read the
    /// stored list only (no per-frame enumeration while the menu is open).
    fn enumerate_when_menu_opens(
        &self,
        app: &mut ImagesScrollerApp,
        button_response: &egui::Response,
    ) {
        if !self.accepts_click(button_response) {
            return;
        }
        let Some(opened_path) = app.opened_image_path.clone() else {
            return;
        };
        app.open_with_programs = platform::enumerate_open_with_programs(&opened_path);
    }

    fn paint_glyph(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let glyph_center = button_response.rect.center();
        let glyph_state = pill_style::glyph_state_for(self.enabled, button_response.hovered());
        let glyph_color = pill_style::glyph_state_color(glyph_state, row_ui);
        for glyph_shape in open_with_glyph_shapes(glyph_center, glyph_color) {
            row_ui.painter().add(glyph_shape);
        }
    }
}

/// The glyph: a 10x10 square with a 7-point arrow emerging from its top-right
/// corner pointing up-right (shaft plus a small triangle head). Paint order:
/// square, shaft (BLACK under-stroke then WHITE fill stroke -- the outline
/// along a diagonal line), triangle head. The square shifts down-left so the
/// up-right emergence stays inside the button.
fn open_with_glyph_shapes(
    glyph_center: egui::Pos2,
    glyph_color: egui::Color32,
) -> Vec<egui::Shape> {
    let square_center = glyph_center + OPEN_WITH_SQUARE_CENTER_OFFSET;
    let square_rect =
        egui::Rect::from_center_size(square_center, egui::Vec2::splat(OPEN_WITH_SQUARE_SIDE));
    let shaft_start = square_rect.right_top();
    let shaft_rise = OPEN_WITH_ARROW_LENGTH * std::f32::consts::FRAC_1_SQRT_2;
    let shaft_end = shaft_start + egui::vec2(shaft_rise, -shaft_rise);
    vec![
        pill_style::outlined_rect_shape(square_rect, glyph_color),
        egui::Shape::line_segment(
            [shaft_start, shaft_end],
            egui::Stroke::new(
                OPEN_WITH_ARROW_SHAFT_OUTLINE_WIDTH,
                pill_style::OUTLINE_COLOR,
            ),
        ),
        egui::Shape::line_segment(
            [shaft_start, shaft_end],
            egui::Stroke::new(OPEN_WITH_ARROW_SHAFT_THICKNESS, glyph_color),
        ),
        pill_style::outlined_triangle_shape(pill_style::TriangularGlyphSpec {
            rect: egui::Rect::from_center_size(shaft_end, OPEN_WITH_ARROW_TRIANGLE_SIZE),
            rotation: OPEN_WITH_ARROW_HEAD_ROTATION,
            fill_color: glyph_color,
        }),
    ]
}

/// The no-programs tooltip applies ONLY while a file is open with an empty
/// enumeration; in empty-folder mode (no file at all) the button stays
/// disabled with the standard tooltip instead.
pub(super) fn no_programs_tooltip_for(app: &ImagesScrollerApp) -> Option<&'static str> {
    if app.image_is_open() && app.open_with_programs.is_empty() {
        return Some(NO_PROGRAMS_TOOLTIP_TEXT);
    }
    None
}

impl ImagesScrollerApp {
    /// The menu is available exactly while an image is open AND the stored
    /// enumeration found registered programs (the approved disabled-button
    /// empty-result behavior). Pure over app state; container-testable.
    pub(super) fn open_with_menu_is_available(&self) -> bool {
        self.image_is_open() && !self.open_with_programs.is_empty()
    }

    /// Hands the CURRENT canonical file to the selected program: the launcher
    /// re-enumerates fresh and indexes `program_launch_index` into its own
    /// list (the Cycle-A launch-index contract). A failure surfaces the one
    /// static failure message in the pill's message line; success changes
    /// nothing (the app stays on the current image).
    pub(super) fn launch_open_with_program(&mut self, program_launch_index: usize) {
        let Some(opened_path) = self.opened_image_path.clone() else {
            return;
        };
        if platform::open_file_with_selected_program(&opened_path, program_launch_index).is_err() {
            self.message_text = Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE);
        }
    }

    /// The popup's contents: the stored program list in a vertically scrolling
    /// menu (verbatim display names; one launch per click).
    pub(super) fn show_open_with_menu_entries(&mut self, menu_ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .max_height(OPEN_WITH_MENU_MAX_HEIGHT)
            .show(menu_ui, |scroll_ui| self.show_menu_entry_buttons(scroll_ui));
    }

    fn show_menu_entry_buttons(&mut self, scroll_ui: &mut egui::Ui) {
        for program_index in 0..self.open_with_programs.len() {
            let entry_response = {
                let display_name =
                    self.open_with_programs[program_index].display_name.as_str();
                scroll_ui.button(display_name)
            };
            if entry_response.clicked() {
                self.launch_open_with_program(program_index);
            }
        }
    }
}
```
Notes for the implementer (binding, not judgment):
- Do NOT import `RegisteredProgram` in this file — it is unused there and would fail `clippy -D warnings` (the field type lives in `app.rs`; the test module imports it).
- The scoped `{}` block around `display_name` keeps the immutable borrow of `self.open_with_programs` from crossing the mutable `launch_open_with_program` call (no clones, no per-frame allocation churn).
- `no_programs_tooltip_for` and `NO_PROGRAMS_TOOLTIP_TEXT` are `pub(super)` ONLY for the sibling test module.

**Step 8 — NEW `src/app/open_with_button_tests.rs` (exact content shape; 7 tests).**
Imports:
```rust
use super::folder_navigation_tests::{TempParentScope, app_opened_in};
use super::open_with_button::{NO_PROGRAMS_TOOLTIP_TEXT, no_programs_tooltip_for};
use super::tests::sample_decoded_image;
use super::{ImagesScrollerApp, NavigationDirection};
use crate::open_with_listing::{OPEN_WITH_LAUNCH_FAILURE_MESSAGE, RegisteredProgram};
use std::path::PathBuf;
```
Fixture:
```rust
fn two_programs_fixture() -> Vec<RegisteredProgram> {
    vec![
        RegisteredProgram {
            display_name: String::from("Paint"),
            launch_index: 0,
        },
        RegisteredProgram {
            display_name: String::from("Photos"),
            launch_index: 1,
        },
    ]
}
```
The 7 pinned tests (exact names; the container stub enumeration is ALWAYS empty and the stub launcher ALWAYS fails — that is what makes these assertions container-deterministic):
1. `open_with_menu_available_when_image_open_and_programs_listed` — `empty_state()` → `accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()))` → assign `two_programs_fixture()` to `app.open_with_programs` → assert `app.open_with_menu_is_available()`.
2. `open_with_menu_unavailable_when_no_image_is_open` — `empty_state()` → assign the fixture to the field → assert `!app.open_with_menu_is_available()`.
3. `open_with_menu_unavailable_while_the_list_is_empty` — `empty_state()` → successful `accept_open_outcome` (container enumeration = empty) → assert the path is stored AND `!app.open_with_menu_is_available()`.
4. `no_programs_tooltip_applies_only_while_a_file_is_open` — `empty_state()` → `assert_eq!(no_programs_tooltip_for(&app), None)` → successful `accept_open_outcome` → `assert_eq!(no_programs_tooltip_for(&app), Some(NO_PROGRAMS_TOOLTIP_TEXT))`.
5. `launch_failure_surfaces_the_static_failure_message` — `empty_state()` → successful `accept_open_outcome` → `app.launch_open_with_program(0)` → assert `app.message_text == Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)`.
6. `every_successful_open_reenumerates_and_replaces_the_stored_list` — successful `accept_open_outcome` → assign the fixture to the field → a SECOND successful `accept_open_outcome` → assert `app.open_with_programs.is_empty()` (the successful-open refresh replaced the display list).
7. `entering_empty_folder_mode_clears_the_stored_list` — `TempParentScope::create("open_with_empty_mode")` with `folder1`/`folder2` child folders, `folder1/img.png` only → `app_opened_in(&scope.child_image_path("folder1", "img.png"))` → assign the fixture to the field → `app.navigate(NavigationDirection::NextSiblingFolder)` (folder2 is image-less → empty-folder mode) → assert `app.open_with_programs.is_empty()` AND `!app.open_with_menu_is_available()`.

**Step 9 — Button-hint test extensions (NO count change).**
In `src/app/button_hint_tests.rs`: add `assert_eq!(hotkey_reference(OpenWith), None);` inside `hotkey_reference_matches_the_spec` and `assert_eq!(tooltip_text(OpenWith), "Open with...");` inside `tooltip_text_matches_the_spec` (both closures already `use PillButton::*`).

**Step 10 — Format, harness, feature commit.**
1. VM: call `alpine-vm` MCP `vm_status` FIRST. If the VM or `/img-scroller` is unavailable → STOP, return a user question.
2. Format fix (the dev-check script never applies fmt fixes): `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt`.
3. Harness run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — `vm_status` already done; the command EXIT CODE is the authoritative success evidence; an absent cargo stderr is NOT a failure signal.
4. Expected: per-check `PASS fmt-check`, `PASS check`, `PASS test`, `PASS clippy` and the final `ALL CHECKS PASSED` (exit 0); the test check reports **177 passed; 0 failed**. Corroborate 28 test files (27 previous + `src/app/open_with_button_tests.rs`). The run's log lands in the gitignored `logs/checks/` — nothing to commit.
5. If fmt-check FAILs on a second run after the fix → re-run `cargo fmt`, re-run the harness (max 2 fix cycles, then STOP and report).
6. Commit ALL code changes: `feat: open with menu pill button and program list popup`. Before committing: read `.gitignore`, run `git status`, verify NO `.gitignore`-matching file (`target/`, `dist/`, `logs/`, `*.log`) is staged and no `Cargo.toml`/`Cargo.lock` change is included.

## §4 Test plan summary

- New test file: `src/app/open_with_button_tests.rs` — 7 pinned tests (§3 Step 8).
- Extended: `src/app/button_hint_tests.rs` — 2 existing tests gain `OpenWith` assertions (count unchanged).
- Untestable in the container (by design, OS/egui-bound): the button paint, the popup mechanics, the enumerate-on-click wiring site — covered by the 4.5a checklist (§7) and later by the Task-4 OPERATOR-RUN.
- **PINNED census: 177 unit tests / 28 test files** (170 + 7; the spec estimated 176 ±1 — 177 is the exact pinned value).

## §5 Code-inspection verifications (no code changes — V-list)

- **V1:** The popup is attached ONLY when `self.enabled` — a disabled button never opens a menu.
- **V2:** The popup closure calls ONLY `show_open_with_menu_entries` (reads the stored list) — NO facade call inside the closure (no per-frame enumeration while the menu is open).
- **V3:** The successful-open enumeration line sits BEFORE `self.opened_image_path = Some(path);` (borrow-before-move).
- **V4:** The failure arm of `accept_open_outcome` does NOT touch `open_with_programs` (the previously displayed image keeps its list).
- **V5:** `enter_empty_folder_mode` clears the list (§3 Step 4).
- **V6:** Row order after integration = 11 buttons with the open-with button LAST and a 16-pt gap before it.
- **V7:** No keyboard-intake change exists (no hotkey, approved A7); `hotkey_reference(OpenWith) == None`.
- **V8:** Zero new dependencies: `Cargo.toml`/`Cargo.lock` untouched; `git diff --stat` shows only the six files of §3 + the two new files.
- **V9:** Fullscreen-ON coverage: `show_open_with_pill_button` is reachable only through `show_floating_pill` → `show_navigation_layout` → `show_chrome_layout`'s `full_ui_chrome_is_visible()` branch — the existing chrome gate hides the new button with everything else (verify by reading the call chain; no code change).

## §6 Git plan

- Branch: stay on `feat/wave-6-open-with` (NO branch creation — Step 2 restricted). NO push (Step 5 restricted; and per the Git Remote Safety Rule push goes to `origin` ONLY, never here).
- Commit 1 (after pre-flight): `docs: add wave6 task3 open-with-menu implementation plan` — this plan file + the 4.1a frontend spec file if still untracked.
- Commit 2 (after harness green): `feat: open with menu pill button and program list popup` — all six edited files + the two new files (§3 Steps 2–9).
- Gitignore Compliance Rule before EVERY commit: read `.gitignore`, run `git status`, ensure no `dist/`, `logs/`, `*.log`, `target/`, or other ignored file is staged; never `git add -A` blindly.
- Meaningful messages per commit; no empty commits; no force-add of tracked files matching ignore patterns.

## §7 4.5a conformance checklist (carried from the 4.1a spec §10, adjusted to the approved decisions — 4.5a re-checks every item)

1. `PillButton::OpenWith` exists in `button_hint.rs` with tooltip `"Open with..."` and `hotkey_reference(...) == None`.
2. The new pill button is 36×32 points, uses the shared `ButtonColumnSite`/`show_button_column` wrapper, and paints a custom vector glyph (WHITE enabled fill, 1-pt BLACK outline; square + shaft + triangle head per D7).
3. The button sits at the RIGHT END of the pill row (after the fit group), separated by a 16-pt group gap.
4. The menu is driven by `egui::Popup::menu(&button_response)`; it is attached ONLY while the button is enabled.
5. Menu default placement is ABOVE the button (`egui::RectAlign::TOP`) with egui's automatic fallback alternatives; popup styling = egui's default menu style (no custom frame).
6. Menu entries are `ui.button(&program.display_name)` verbatim.
7. Long lists are wrapped in `egui::ScrollArea::vertical().max_height(300.0)`.
8. Empty enumeration renders a DISABLED button whose hover tooltip is `"No programs registered for this file type"` (approved A3); in empty-folder browsed mode the button is disabled with the standard `"Open with..."` tooltip; in the fresh empty state the pill (and the button) is absent.
9. Selecting an entry calls `platform::open_file_with_selected_program(path, launch_index)` with the CURRENT canonical `opened_image_path`.
10. On launch error, `message_text` is set to `OPEN_WITH_LAUNCH_FAILURE_MESSAGE`; on success nothing changes.
11. Button/menu unavailable when no image is open, in empty-folder mode, and hidden entirely when fullscreen is ON (chrome gate covers the new button — V9).
12. `open_with_programs` is refreshed on every successful open AND once on the popup-opening click (approved enumerate-WHEN-OPENED wiring); the popup closure performs NO enumeration (no per-frame OS enumeration anywhere).
13. Test census matches the pinned **177 / 28**.
14. No per-frame texture re-upload or allocation churn is introduced (no clones of the program list in the render path; the popup path allocates only egui widgets).

## §8 Scope boundaries (HARD)

- NO Task-4 work (no release rebuild, no DLL import audit, no OPERATOR-RUN checklist execution).
- NO Task-5 work (no README/tech/architecture/context edits; no CHANGELOG.md — forbidden this wave).
- NO edits to any `.md` file by the implementer (markdown-generation rule): `.agent/project-structure.md` gains the two new files in the 4.4 docs step — flagged here so 4.4 knows (`src/app/open_with_button.rs`, `src/app/open_with_button_tests.rs`).
- NO `Cargo.toml`/`Cargo.lock` changes (zero new dependencies); no `main.rs`, `keyboard_intake.rs`, `central_surface.rs`, `app_display.rs`, or `open_with_listing.rs` changes.
- No scope beyond TODO Task 3's two sub-items; anything discovered mid-implementation that seems to need more → STOP and report to the caller.

## §9 Notes for 4.3 review and later steps

- Review checkpoints: the exact constants of D7; the two enumeration points of D5 (and their placement in `accept_open_outcome`/`enumerate_when_menu_opens`); the state-aware tooltip selection (Step 2.5 — the subtlest edit); the borrow-scoping block in `show_menu_entry_buttons`; the line-cap management in `app.rs` (199 expected).
- Known accepted behaviors (NOT defects): the popup-open re-enumeration also runs on the popup toggle-close click; an empty fresh enumeration at popup-open yields an entry-less popup (dismissable) with the button disabling from the next frame; a DISABLED button never re-enumerates (a user installing a program mid-session sees the button enable only after the next successful open or — if it was enabled — the next popup-open).
- Task-4 handoff (manual UX, operator-run, NOT this task): for a jpg/png file the menu lists at least the system-associated programs; selecting one opens THIS exact file in that program; the list is not dependent on any app-maintained database.
- Task-5 handoff: record the approved placement (11th pill button + upward popup), the empty-result disabled behavior, and the two-point refresh contract in tech.md/architecture.md.

## §10 Plan self-check against TODO Task 3 verbatim + approved decisions

- Sub-item 1 (menu entry/portal in the viewer UI; placement per approved propose-and-ask): encoded in §2 D1/D2/D7 and §3 Steps 6–7 — Option A, 11th pill button, rightmost, upward `Popup::menu`. ✓
- Sub-item 2 (behavior when NO programs are registered — decided by the architector, user-approved): encoded in §2 D3/D8 and §3 Steps 2/7/8 — disabled button + `"No programs registered for this file type"` tooltip. ✓
- Approved A1–A8: each mapped in §0's table to its encoding section. ✓
- Code rules: all source in `src/` ✓; files ≤ 200 lines (app.rs 199, open_with_button.rs ≤ 200) ✓; method bodies ≤ 50 lines ✓; ≤ 2 explicit params per method (receiver excluded, codebase precedent) ✓; private members by default (only `pub(super)` where tests/consumers require) ✓; self-documenting names ✓; NO commented-out code ✓; single-section boolean conditions via named predicates ✓; max nesting 2 ✓; named constants ✓; unit tests for the new pure logic ✓.

**Plan file:** `.kilo/plans/20261112-wave6-task3-open-with-menu-implementation-plan.md`
