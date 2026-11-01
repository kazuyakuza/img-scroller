# Implementation Plan — Wave-2 UI Fixes: Centered Floating Navigation Controls + Empty-State Center Open

Date: 2026-11-01
Workflow step: Cycle A → Step 4.1b (architector)
Source TODO: [`.agent/todos/20261101/20261101-todo-1.md`](../../.agent/todos/20261101/20261101-todo-1.md) → Tasks 1–3 (icon mechanism + floating controls + center Open; binding directives from operator-run feedback, 2026-11-01)
Global plan: [`.kilo/plans/20261101-wave2-ui-restyle.md`](../../.kilo/plans/20261101-wave2-ui-restyle.md) (§2.3 decision inventory, §2.2 constraints, Cycle A step map)
Front-end input: [`.kilo/plans/20261101-floating-controls-frontend-spec.md`](../../.kilo/plans/20261101-floating-controls-frontend-spec.md) (4.1a spec FULL-CONSUMED: §2.3 verdicts, §3 icon-mechanism analysis, §4 anatomy options, §5 empty-state shape, §7 conformance checklist)
Predecessor: Wave-2 landed state (`2c9db24`→`de332fc`; branch `fix/centered-floating-controls`, version `0.2.1`): `src/app.rs` 163 L (ui() = key intake → top toolbar → rails gate → CentralPanel), `src/app/rails.rs` 74 L, `src/app/tests.rs` 187 L, `src/app/rails_tests.rs` 40 L; 26 unit tests.
Approval: user AUTO-APPROVED per-task plans (approval mode 2026-11-01) — every decision below is DECISIVE; zero judgment remains for the implementer.
Status: PLAN — implementation owns the code; Cycle B owns dev-checks/exe (NO cargo here).

---

## 0. Scope Guard

This step ships EXACTLY: a guaranteed-visible icon mechanism for the four reserved arrow buttons, one centered bottom floating control pill replacing rails/toolbars, and ONE empty-state centered Open button. NOTHING else:

- NO Wave-3 `↑`/`↓` wiring (up/down buttons are painted DISABLED and call no handler — user-approved reserve pattern).
- NO Wave-5 pan, NO fullscreen/zoom, NO menu placeholder/"Open with…", NO new keyboard shortcuts.
- NO documentation edits (Cycle C owns `README.md`/`tech.md`/`architecture.md`/`context.md`/`.agent/project-structure.md`). NO `CHANGELOG.md` (wave binding). NO edits to existing `.kilo/plans/*` files. NO TODO `[DONE]` marks (step 4.6 owns those).
- **NO cargo commands in this step** (per global plan Cycle A 4.2: "no cargo — Task-4 runs checks; report compile-hazard notes"). The implementer self-verifies statically (§9) and REPORTS hazards (§8.3).
- `Cargo.toml`/`Cargo.lock` need NOTHING (no new dependency; version already `0.2.1` from step 3; `git status`/`git log` verified on branch `fix/centered-floating-controls`, HEAD `10d0253 chore: bump version to 0.2.1`).

## 1. Binding decision — icon mechanism (TODO Task 1, frontend spec §3)

**PICK: egui-painted vector triangles — frontend spec option 3a — implemented with `egui::Shape::rotated_triangle`.**

Rationale (decisive):

1. **No font dependency.** The tofu-box defect is a font-availability failure; vector shapes are painted by epaint geometry and CANNOT fail on Windows 10/11 regardless of system/default font state (spec §3a feasibility: "guaranteed shape rendering"). Options 3b (ASCII `< > ^ v`) and 3c (bundled font: new binary asset, license burden, falls back to tofu when mis-loaded — spec §3b/§3c risks) are REJECTED.
2. **egui 0.37 has a dedicated arrow-icon painter** — verified from docs.rs `egui::Shape` (0.37.0):
   `pub fn rotated_triangle(rect: Rect, rotation: f32, fill: impl Into<Color32>) -> Shape` — "A filled triangle inscribed in rect, pointing down. rotation is in radians… 0.0 points down, TAU / 4.0 left, TAU / 2.0 up, and -TAU / 4.0 right. Useful to paint small arrow icons in the ui, like the on combo boxes or submenu buttons." No stroke (`PathStroke`) parameter is needed; `Shape::convex_polygon` (also verified, stroke = `impl Into<PathStroke>`) is available but NOT used.
3. **The disabled/hover visual handling is encoded explicitly** (the one cost of 3a — spec §3a "visual-state requirements"): exact color trio + hover cursor + accessibility info per §2.5/§5.3 below. This fully discharges the caller's "requires explicit disabled/hover visual handling — encode exactly how".

Rotation mapping (binding, from the verified 0.37 docs text):

| Direction button | `ArrowDirection` | `rotated_triangle` rotation |
| --- | --- | --- |
| ← previous image | `Left` | `std::f32::consts::TAU / 4.0` |
| → next image | `Right` | `-std::f32::consts::TAU / 4.0` |
| ↑ up folder (Wave-3 reserve) | `Up` | `std::f32::consts::TAU / 2.0` |
| ↓ down folder (Wave-3 reserve) | `Down` | `0.0` |

(`std::f32::consts::TAU` — stable const; `egui::TAU` re-export is not relied on.)

## 2. Binding decision — replacement module and pill anatomy (TODO Task 2, frontend spec §4)

### 2.1 Files

- **DELETE** `src/app/rails.rs` (74 L) and `src/app/rails_tests.rs` (40 L) via `git rm` — obsolete, NOT commented out (no-commented-code rule; git history retains them; global-plan §2.3-5).
- **CREATE** `src/app/arrow_direction.rs` — the arrow type child module (caller's "small param enum `ArrowDirection {Up, Down, Left, Right}` in its own child location") hosting `ArrowDirection` + `ArrowControl`.
- **CREATE** `src/app/floating_controls.rs` — the pill module carrying `navigation_button_enabled` (MOVED from rails.rs verbatim semantics) + the pill renderer + the per-button widget.
- **CREATE** `src/app/floating_controls_tests.rs` — the relocated rails enablement test (§6).
- Registration in `src/app.rs` (the `rails` pattern — child modules, NOT `src/main.rs`; `src/main.rs` UNTOUCHED, same privacy reason as Wave-2 Task-3 §3.1: file-private `NavigationDirection`/`navigate`/`neighbor_image_path`/`message_text` stay visible only to `app`'s descendants):

```rust
mod arrow_direction;

mod floating_controls;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod floating_controls_tests;
```

### 2.2 Container anatomy (frontend spec §4.1 — `Area` + `Frame` candidate, DECIDED)

ONE egui `Area` per frame:

- `egui::Area::new(egui::Id::new("floating_navigation_pill"))` (id salt exact).
- `.anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -24.0))` — anchored bottom-center, immovable by definition (verified docs: "Anchoring also makes the window immovable… It is an error to set both an anchor and a position"). Bottom offset −24 logical points = spec §2.6/§4.1 value; the negative y moves the pill UP (egui y grows downward — verified egui coordinate convention).
- `.order(egui::Order::Foreground)` — explicit, per spec §4.1 "safer": renders above the CentralPanel regardless of call order; CentralPanel stays the image surface and the overlay is non-space-reserving.
- Interaction: defaults kept (`constrain`/`interactable` defaults) — verified `Area::anchor` page + spec §4.1/§2.6; the pill stays clickable and does NOT consume arrow keys (Area is not focusable; spec §6).
- Inside: ONE `egui::Frame::new()` (verified 0.37 struct Frame with builder methods):
  - `.fill(FLOATING_PILL_FILL_COLOR)` where `FLOATING_PILL_FILL_COLOR: egui::Color32 = egui::Color32::from_rgba_unmultiplied(20, 20, 20, 180)` — translucent dark fill over the image (decided; readable over any image, non-opaque).
  - `.corner_radius(FLOATING_PILL_CORNER_RADIUS)` where `FLOATING_PILL_CORNER_RADIUS: u8 = 10` (verified `impl Into<CornerRadius>` + `From<u8> for CornerRadius` in epaint 0.37).
  - `.inner_margin(FLOATING_PILL_INNER_MARGIN_POINTS)` where `FLOATING_PILL_INNER_MARGIN_POINTS: f32 = 8.0` — uniform margin, ONE verified `From<f32>` Into-Margin value instead of an unverified `Margin::symmetric`/const-`Margin` (spec's 8/6 split intentionally simplified to 8/8 — recorded decision).
- Buttons remain clickable: the Frame is a pure paint container (no sense of its own), the click responses come from the four per-button allocations inside it.

### 2.3 Row contents AND exact order (DECIDED, stable)

Left-to-right in ONE horizontal row: **`[←] [→] (16-pt gap) [↑] [↓]`** — the frontend spec §4.2 suggested order `[←] [→] … [↑] [↓]` with an optional 16-point group gap before the Wave-3-reserved folder pair. The gap is applied by `row_ui.add_space(FOLDER_GROUP_GAP)` exactly before the `FolderUp` button (single-section condition, extracted helper).

Row layout: `egui::Layout::left_to_right(egui::Align::Center)` (landed rails.rs pattern).

### 2.4 Sizing

- Hit area per button: `egui::Vec2::new(36.0, 32.0)` — caller's "or bigger 32×28" honored via the frontend spec §4.4 recommended minimum 36×32 ("larger than the retired 28×24 rail buttons because the overlay sits over the image and needs an easy click target"). The whole 36×32 rect answers clicks, hover, and the pointer cursor.
- Triangle: `egui::Vec2::new(10.0, 10.0)`, centered in the button rect via `egui::Rect::from_center_size(response.rect.center(), ARROW_TRIANGLE_SIZE)`.
- Spacing between buttons: default egui `item_spacing` (≈8 pt) — no style override; the ONLY explicit space is the 16-pt group gap.

### 2.5 Painted-button widget mechanics (verified 0.37 APIs)

Per button, IN the pill row ui:

1. `let (_, response) = ui.allocate_exact_size(ARROW_BUTTON_SIZE, egui::Sense::click())` — returns `(Rect, Response)` (verified docs.rs Ui 0.37).
2. `response.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, self.control.accessible_label()))` — Accessibility (spec §2.7), pattern verified from egui 0.37 accessibility docs ("give it explicit widget information… `WidgetInfo::labeled(Role::Button, ui.is_enabled(), …)`" — this plan passes the truthful per-button `enabled` instead of `ui.is_enabled()`).
3. `let response = response.on_hover_cursor(egui::CursorIcon::PointingHand)` — verified `CursorIcon::PointingHand` in egui 0.37; NOTE: `Response::on_hover_cursor` consumes `self` and returns `Response` → MUST reassign the binding (hazard §8.3-e).
4. Paint ONE `Shape::rotated_triangle` through `ui.painter().add(...)` with the color from the `arrow_color` method (§5.3).
5. Click: EXACTLY `app.navigate(NavigationDirection::Previous/Next)` (the keys' seam; no second handler, no direct state mutation — frontend spec §7 checklist).

**Visual states (byte-exact rule):** the triangle color is selected by a dedicated method with the default egui button-text palette from `ui.visuals()`:

| State | Color source |
| --- | --- |
| Disabled (no neighbor / Wave-3 reserve) | `ui.visuals().widgets.noninteractive.fg_stroke.color` |
| Enabled + pointer hovering | `ui.visuals().widgets.hovered.fg_stroke.color` |
| Enabled idle | `ui.visuals().widgets.inactive.fg_stroke.color` |

Disabled buttons still allocate the click sense but their click is GATED by enablement (extracted `accepts_click` helper — single-section-boolean rule satisfied: the compound `enabled && clicked()` lives in the named extraction method). No per-button background rect is painted (hover feedback = color change only; avoids any `CornerRadius` painting surface). Theme-consistent, contrast-preserving (default visuals untouched — spec §2.5).

### 2.6 Message placement (frontend spec §4.5 option 2 + caller recommendation, DECIDED)

The message renders as a **small label line INSIDE the same Area/Frame, directly ABOVE the button row** (keeps the pill compact vs. the §4.5 option 1 in-row variant):

- `pill_ui.label(egui::RichText::new(message.unwrap_or("")).size(MESSAGE_TEXT_SIZE))` with `MESSAGE_TEXT_SIZE: f32 = 12.0`, in a `egui::Layout::top_down(egui::Align::Center)` child layout so it centers over the row.
- `message.unwrap_or("")` → shows `message_text` when `Some`, an empty label otherwise (caller's exact wording). `message_text` STATE semantics untouched.
- Literal-K3 visibility is preserved STRUCTURALLY: the whole pill (message included) exists only in the image-open branch of `ui()` (§4). Empty-state failures therefore show no message ✓.

## 3. Binding decision — storage policy (caller item 3)

Buttons store NO state (no struct field, no `ctx.memory` value, no widget state). Enablement is DERIVED EVERY FRAME from the existing seam:

- Prev/Next: `navigation_button_enabled(app, direction)` == `app.neighbor_image_path(direction).is_some()` (STAY parity — no image, no listing, unresolvable index, first/last edge → disabled; identical to keys).
- Up/Down: enablement resolves to `false` structurally (`navigation_direction() == None` → not enabled) — permanent Wave-2 reserve.

No state-field additions anywhere (frontend spec §7 "state fields are not expanded solely for UI surface needs"). `ImagesScrollerApp` fields are byte-identical to the landed struct.

## 4. Binding decision — `ui()` final shape (TODO Task 3 + caller item 6, frontend spec §5/§6)

Final `impl eframe::App` body structure (exact; code in §5.1):

1. **Key intake stays EXACTLY as landed, frame-first** (4 lines of `key_pressed` reads + two `if` blocks, byte-identical: spec §2.4, §6 — "read arrow keys → branch on `image_is_open()`").
2. Branch on `if self.image_is_open()` (single-section):
   - **Image open** → `show_image_layout(ui)`: `floating_controls::show_floating_pill(self, ui)` (the ONE pill) + `egui::CentralPanel::default()` painting the fit-scaled image (existing paint block byte-identical; texture upload stays gated by `texture_upload_pending` — untouched).
   - **Empty state** → `show_empty_state_layout(frame, ui)`: `egui::CentralPanel::default()` (paints NOTHING, kept) containing `show_centered_open_button(frame, central_ui)`.

### 4.1 Empty-state center Open — exact shape (frontend spec §5.1, caller item 4)

- ONE `egui::Button::new(egui::RichText::new("Open…").size(18.0)).min_size(egui::Vec2::new(180.0, 48.0))`.
- **Centering mechanism (DECIDED — the verified one):** an `egui::Area::new(egui::Id::new("empty_state_open_button")).anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)`, shown via the CentralPanel's `central_ui`.
  Recorded deviation note (for 4.5a reading): the caller's "vertical centering with remaining space" was given as an *example* ("e.g."), and the frontend spec §5.1 itself lists the Area-CENTER_CENTER option as the second sanctioned pattern. It is chosen because the alternative, `egui::Layout::centered_and_justified(Direction::TopDown)`, JUSTIFIES widgets along both axes (landing precedent `app_display.rs::paint` deliberately uses it so the IMAGE stretches) and would stretch the Button across the full panel — wrong visual. `Ui::centered` (bare, both-axes-centering) does NOT exist in egui 0.37 (verified full `Ui` method index on docs.rs 0.37.0 — only `centered_and_justified`, `vertical_centered*`, `horizontal_centered`). The Area-anchored button centers on the exact window center and is immune to layout math.
- Click → `self.open_via_dialog(frame)` EXACTLY (the platform dialog seam reused; dialog called from the immediate-mode context each clicked frame — same pattern the retired `open_toolbar_button` used).
- **Open-control uniqueness (spec wording asserted):** the string "Open…" appears exactly ONCE in `src/` — the `OPEN_LABEL_TEXT` constant in `src/app.rs` — and its ONLY render site is inside the `else` (empty-state) branch of `ui()`. While an image is open the branch never runs → NO Open control anywhere (the top `open_toolbar` panel is gone entirely; `open_toolbar_button` is retired fully, not commented).

### 4.2 Obsolete removals in `src/app.rs` (all at once, nothing survives)

1. DELETE the `fn open_toolbar_button` method (orphaned; retire fully — no commented code).
2. DELETE the `egui::Panel::top("open_toolbar")` show block from `ui()`.
3. DELETE the rails gate block (`if self.image_is_open() { rails::show_folder_rail…; rails::show_image_rail…; }`).
4. UPDATE one doc-comment line: `image_is_open`'s doc becomes `/// True while an image remains successfully opened (the floating controls' empty-state gate).` (stale "rails" wording; doc-conformance step 4.4 may still gloss).
5. Add the constants of §5.1 and the three new layout methods; replace the module registrations (§2.1).

## 5. EXACT code

### 5.1 `src/app.rs` — full replacement of the affected regions (everything else byte-identical)

Preserve byte-identical: file doc comment; imports; `APP_WINDOW_TITLE`; `ImagesScrollerApp` struct + `new`/`empty_state`; `open_image`; `accept_open_outcome`; `open_via_dialog`; `navigate`; `neighbor_image_path`; `image_is_open` body; `window_options`.

Delete: `open_toolbar_button` (lines 74–80 incl. adjacent blank), the top-panel block, the rails gate. Keep the key intake INSIDE `ui()` byte-identical at the top.

```rust
impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let left_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowLeft));
        let right_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowRight));
        if left_pressed {
            self.navigate(NavigationDirection::Previous);
        }
        if right_pressed {
            self.navigate(NavigationDirection::Next);
        }
        if self.image_is_open() {
            self.show_image_layout(ui);
        } else {
            self.show_empty_state_layout(frame, ui);
        }
    }
}

/// Image-open layout: the centered floating control pill plus the central image paint.
fn show_image_layout(&mut self, ui: &mut egui::Ui) {
    floating_controls::show_floating_pill(self, ui);
    egui::CentralPanel::default().show(ui, |central_ui| {
        if let Some(decoded_image) = self.decoded_image.as_ref() {
            self.current_display.paint(central_ui, decoded_image);
        }
    });
}

/// Empty-state layout: an empty central paint carrying the ONE center Open button.
fn show_empty_state_layout(&mut self, open_dialog_frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    egui::CentralPanel::default().show(ui, |central_ui| {
        self.show_centered_open_button(open_dialog_frame, central_ui);
    });
}

/// The only Open control: anchored to the window center in the empty state;
/// the click reuses the native open dialog seam (the retired toolbar's path).
fn show_centered_open_button(
    &mut self,
    open_dialog_frame: &mut eframe::Frame,
    central_ui: &mut egui::Ui,
) {
    egui::Area::new(egui::Id::new(EMPTY_STATE_OPEN_AREA_ID))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(central_ui, |area_ui| {
            let open_button = egui::Button::new(
                egui::RichText::new(OPEN_LABEL_TEXT).size(OPEN_LABEL_SIZE),
            )
            .min_size(OPEN_BUTTON_SIZE);
            if area_ui.add(open_button).clicked() {
                self.open_via_dialog(open_dialog_frame);
            }
        });
}
```

New constants (placed directly above `pub fn window_options()`, one blank line before):

```rust
const EMPTY_STATE_OPEN_AREA_ID: &str = "empty_state_open_button";
const OPEN_LABEL_TEXT: &str = "Open…";
const OPEN_LABEL_SIZE: f32 = 18.0;
const OPEN_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(180.0, 48.0);
```

Module registrations at the file bottom (replaces `mod rails;` + `#[cfg(test)] mod rails_tests;`):

```rust
mod arrow_direction;

mod floating_controls;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod floating_controls_tests;
```

### 5.2 CREATE `src/app/arrow_direction.rs` — full file content

```rust
//! Painted-arrow types of the floating navigation pill: the triangle paint
//! direction in the `egui::Shape::rotated_triangle` rotation space, and the
//! navigation control each pill button represents.

use super::NavigationDirection;

/// Painted triangle direction. `Shape::rotated_triangle` paints 0.0 down,
/// a quarter turn left, half a turn up, minus a quarter turn right.
pub(super) enum ArrowDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ArrowDirection {
    pub(super) fn paint_rotation(self) -> f32 {
        match self {
            Self::Up => std::f32::consts::TAU / 2.0,
            Self::Down => 0.0,
            Self::Left => std::f32::consts::TAU / 4.0,
            Self::Right => -std::f32::consts::TAU / 4.0,
        }
    }
}

/// Which navigation control a floating-pill button represents. The image
/// arrows navigate via the keyboard seam; the folder arrows are disabled in
/// Wave 2 and follow Wave 3.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum ArrowControl {
    ImagePrevious,
    ImageNext,
    FolderUp,
    FolderDown,
}

impl ArrowControl {
    pub(super) fn painted_direction(self) -> ArrowDirection {
        match self {
            Self::ImagePrevious => ArrowDirection::Left,
            Self::ImageNext => ArrowDirection::Right,
            Self::FolderUp => ArrowDirection::Up,
            Self::FolderDown => ArrowDirection::Down,
        }
    }

    pub(super) fn accessible_label(self) -> &'static str {
        match self {
            Self::ImagePrevious => "Previous image",
            Self::ImageNext => "Next image",
            Self::FolderUp => "Up folder",
            Self::FolderDown => "Down folder",
        }
    }

    /// None for the Wave-3-reserved folder arrows (no navigation this wave).
    pub(super) fn navigation_direction(self) -> Option<NavigationDirection> {
        match self {
            Self::ImagePrevious => Some(NavigationDirection::Previous),
            Self::ImageNext => Some(NavigationDirection::Next),
            Self::FolderUp | Self::FolderDown => None,
        }
    }
}
```

### 5.3 CREATE `src/app/floating_controls.rs` — full file content

```rust
//! Floating centered bottom navigation controls: one overlay pill above the
//! central image holding the message line and four painted arrow buttons.
//! Previous/next enablement follows the keyboard seam per frame; up/down are
//! the Wave-3-reserved disabled pair. The caller keeps the image-open gate.

use eframe::egui;

use super::arrow_direction::ArrowControl;
use super::{ImagesScrollerApp, NavigationDirection};

const FLOATING_PILL_AREA_ID: &str = "floating_navigation_pill";
const FLOATING_PILL_BOTTOM_OFFSET: f32 = -24.0;
const FLOATING_PILL_FILL_COLOR: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied(20, 20, 20, 180);
const FLOATING_PILL_CORNER_RADIUS: u8 = 10;
const FLOATING_PILL_INNER_MARGIN_POINTS: f32 = 8.0;
const ARROW_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const ARROW_TRIANGLE_SIZE: egui::Vec2 = egui::Vec2::new(10.0, 10.0);
const FOLDER_GROUP_GAP: f32 = 16.0;
const MESSAGE_TEXT_SIZE: f32 = 12.0;

/// A navigation button is enabled exactly when the navigation seam resolves
/// a neighbor: no image, no listing, unresolvable index, and list edges disable.
pub(super) fn navigation_button_enabled(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> bool {
    app.neighbor_image_path(direction).is_some()
}

/// Shows the pill: one centered, anchored overlay Area above the image surface
/// (non-space-reserving; Order::Foreground keeps it painted above CentralPanel).
pub(super) fn show_floating_pill(app: &mut ImagesScrollerApp, host_ui: &mut egui::Ui) {
    egui::Area::new(egui::Id::new(FLOATING_PILL_AREA_ID))
        .anchor(
            egui::Align2::CENTER_BOTTOM,
            egui::vec2(0.0, FLOATING_PILL_BOTTOM_OFFSET),
        )
        .order(egui::Order::Foreground)
        .show(host_ui, |area_ui| {
            show_pill_contents(app, area_ui);
        });
}

fn show_pill_contents(app: &mut ImagesScrollerApp, area_ui: &mut egui::Ui) {
    let message = app.message_text;
    egui::Frame::new()
        .fill(FLOATING_PILL_FILL_COLOR)
        .corner_radius(FLOATING_PILL_CORNER_RADIUS)
        .inner_margin(FLOATING_PILL_INNER_MARGIN_POINTS)
        .show(area_ui, |pill_ui| {
            show_message_line(message, pill_ui);
            show_button_row(app, pill_ui);
        });
}

/// Shows the literal-K3 message: the failure text when Some, empty otherwise.
fn show_message_line(message: Option<&'static str>, pill_ui: &mut egui::Ui) {
    pill_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |line_ui| {
        line_ui.label(egui::RichText::new(message.unwrap_or("")).size(MESSAGE_TEXT_SIZE));
    });
}

fn show_button_row(app: &mut ImagesScrollerApp, pill_ui: &mut egui::Ui) {
    let button_specs = arrow_button_specs(app);
    pill_ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Center),
        |row_ui| {
            for button_spec in &button_specs {
                add_folder_group_gap(row_ui, button_spec.control);
                button_spec.show_and_navigate(app, row_ui);
            }
        },
    );
}

fn add_folder_group_gap(row_ui: &mut egui::Ui, arrow_control: ArrowControl) {
    if arrow_control == ArrowControl::FolderUp {
        row_ui.add_space(FOLDER_GROUP_GAP);
    }
}

fn arrow_button_specs(app: &ImagesScrollerApp) -> [ArrowButtonSpec; 4] {
    [
        ArrowButtonSpec::in_row(ArrowControl::ImagePrevious, app),
        ArrowButtonSpec::in_row(ArrowControl::ImageNext, app),
        ArrowButtonSpec::in_row(ArrowControl::FolderUp, app),
        ArrowButtonSpec::in_row(ArrowControl::FolderDown, app),
    ]
}

/// Per-frame row-button request: the control it draws plus the enablement
/// derived that frame. Buttons store no state between frames.
struct ArrowButtonSpec {
    control: ArrowControl,
    enabled: bool,
}

fn arrow_button_is_enabled(arrow_control: ArrowControl, app: &ImagesScrollerApp) -> bool {
    let Some(direction) = arrow_control.navigation_direction() else {
        return false;
    };
    navigation_button_enabled(app, direction)
}

impl ArrowButtonSpec {
    fn in_row(arrow_control: ArrowControl, app: &ImagesScrollerApp) -> Self {
        let enabled = arrow_button_is_enabled(arrow_control, app);
        Self {
            control: arrow_control,
            enabled,
        }
    }

    /// Paints the arrow and, when clicked while enabled, drives the keyboard
    /// navigation seam. Disabled arrows paint dimmed and stay inert.
    fn show_and_navigate(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let button_response = self.allocate_and_paint(row_ui);
        if let Some(direction) = self.click_direction(&button_response) {
            app.navigate(direction);
        }
    }

    fn allocate_and_paint(&self, row_ui: &mut egui::Ui) -> egui::Response {
        let (_, button_response) =
            row_ui.allocate_exact_size(ARROW_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::Role::Button,
                self.enabled,
                self.control.accessible_label(),
            )
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_triangle(row_ui, &button_response);
        button_response
    }

    fn click_direction(&self, button_response: &egui::Response) -> Option<NavigationDirection> {
        if !self.accepts_click(button_response) {
            return None;
        }
        self.control.navigation_direction()
    }

    /// A disabled arrow never navigates (the Wave-3 reserve stays handler-free).
    fn accepts_click(&self, button_response: &egui::Response) -> bool {
        self.enabled && button_response.clicked()
    }

    fn paint_triangle(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let triangle_rect = egui::Rect::from_center_size(
            button_response.rect.center(),
            ARROW_TRIANGLE_SIZE,
        );
        let triangle = egui::Shape::rotated_triangle(
            triangle_rect,
            self.control.painted_direction().paint_rotation(),
            self.arrow_color(row_ui, button_response),
        );
        row_ui.painter().add(triangle);
    }

    /// Default egui button-text palette: idle in `inactive`, brightened on
    /// hover, dimmed in `noninteractive` while disabled (exact visual states).
    fn arrow_color(&self, row_ui: &egui::Ui, button_response: &egui::Response) -> egui::Color32 {
        if !self.enabled {
            return row_ui.visuals().widgets.noninteractive.fg_stroke.color;
        }
        if button_response.hovered() {
            return row_ui.visuals().widgets.hovered.fg_stroke.color;
        }
        row_ui.visuals().widgets.inactive.fg_stroke.color
    }
}
```

Rule audit baked into this file (verify while typing, per §9):

1. Method lengths: longest is `allocate_and_paint` ≈ 16 lines ≤ 50 ✓. Max nesting: closure (1) → `if`/`for`/`if let` (2) everywhere ✓ (`show_button_row`: closure + for = 2 ✓; `arrow_color`: two sequential early-return `if`s = depth 1 each ✓).
2. Param counts ≤ 2 non-self everywhere (`show_and_navigate(self, app, row_ui)` = 2 ✓; `paint_triangle`/`arrow_color` = 2 ✓; free fns ≤ 2 ✓). Compound booleans ONLY inside extraction methods (`accepts_click`, `arrow_button_is_enabled`) per the single-section rule ✓.
3. All items PRIVATE except the three widened with stated consumers: `show_floating_pill` (called from `app.rs`), `navigation_button_enabled` (test access, moved verbatim from rails.rs), and the `pub(super)` arrow types/methods (sibling-module use). Access labels "Previous image"/"Next image"/"Up folder"/"Down folder" surface in AccessKit.
4. Borrow chain: `message` copied out of `app` BEFORE the Frame closure; `arrow_button_specs(app)` finishes its immutable borrow before `show_and_navigate` reborrows `app` mutably inside the row closure; `paint_triangle` takes `&egui::Ui` + `&Response` while `app` is NOT borrowed — no second simultaneous `&mut app` (rails.rs closure-capture precedent).

## 6. Tests — moves, retirement, and arithmetic (caller item 5)

### 6.1 Verified impact on the landed 26 tests (26 = app/tests 9 + rails_tests 1 + app_display 2 + scaling 5 + image_loading 4 + folder_listing 5; grep-census verified)

- `src/app/tests.rs` (187 L): INSPECTION-VERIFIED untouched. All 9 tests assert STATE and call `navigate`/`neighbor_image_path`/`accept_open_outcome`/`empty_state`/`APP_WINDOW_TITLE` — zero references to `open_toolbar_button`, the `open_toolbar` panel, `rails::`, or any renderer. The empty-state center-Open adds NO state and NO state transition → NO new app test.
- `src/app/rails_tests.rs` (40 L): the single test `arrow_button_enablement_follows_the_navigation_edges` MOVES into the new module test file (same semantics, same fixture sharing; `imports` updated). The old file is `git rm`-deleted.

### 6.2 CREATE `src/app/floating_controls_tests.rs` — full file content

```rust
//! Unit tests for the floating-controls arrow-button enablement
//! (`super::floating_controls::navigation_button_enabled`).

use super::floating_controls::navigation_button_enabled;
use super::tests::{TempImageFolder, sample_decoded_image};
use super::{ImagesScrollerApp, NavigationDirection};

#[test]
fn arrow_button_enablement_follows_the_navigation_edges() {
    let folder = TempImageFolder::create("button_edges");
    folder.create_file("a.png");
    folder.create_file("b.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    assert!(!navigation_button_enabled(&app, NavigationDirection::Previous));
    assert!(!navigation_button_enabled(&app, NavigationDirection::Next));
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert!(!navigation_button_enabled(&app, NavigationDirection::Previous));
    assert!(navigation_button_enabled(&app, NavigationDirection::Next));
    app.accept_open_outcome(folder.image_path("c.png"), Ok(sample_decoded_image()));
    assert!(navigation_button_enabled(&app, NavigationDirection::Previous));
    assert!(!navigation_button_enabled(&app, NavigationDirection::Next));
}
```

- The `pub(super)`/`use super::tests::…` fixture-sharing pattern survives unchanged (`TempImageFolder`, `sample_decoded_image` stay shared).
- **Expected test count: 26 — unchanged** (one relocation, no additions; the moved test itself is unchanged semantically: empty state → both disabled; first sibling → ← disabled/→ enabled; last sibling → ← enabled/→ disabled).

### 6.3 Untouched test files: `src/app_display.rs` (2), `src/scaling.rs` (5), `src/image_loading.rs` (4), `src/folder_listing/tests.rs` (5).

## 7. Line-count arithmetic (≤200 budget, verification)

- `src/app.rs`: 163 − 8 (`open_toolbar_button` + blank) − 4 (top-panel block) − 4 (rails gate) + ~5 (consts) + ~30 (three layout methods + docs) + 1 (`mod arrow_direction;`) ≈ **183 ≤ 200 ✓** (implementer confirms the actual count; ±5 tolerated).
- `src/app/floating_controls.rs`: **≈ 150 ≤ 200 ✓** (as written in §5.3).
- `src/app/arrow_direction.rs`: **≈ 75 ≤ 200 ✓**.
- `src/app/floating_controls_tests.rs`: **≈ 38 ≤ 200 ✓**.
- Deleted: rails.rs (74), rails_tests.rs (40). Untouched: `src/main.rs`, `src/app_display.rs`, `src/folder_listing.rs` + tests, `src/image_loading.rs`, `src/platform/*`, `src/scaling.rs`, `Cargo.toml`, `Cargo.lock`.

## 8. Implementation steps (in order — junior-executable)

1. `git branch --show-current` must print `fix/centered-floating-controls`; mismatch → STOP, ask the caller.
2. Read `.gitignore`; run `git status` (expect: modified `.agent/project-info/context.md`, untracked `.kilo/plans/20261101-floating-controls-frontend-spec.md`, this plan untracked — do NOT stage `context.md` now, §8.2).
3. `git rm src/app/rails.rs src/app/rails_tests.rs`.
4. CREATE `src/app/arrow_direction.rs` EXACTLY per §5.2.
5. CREATE `src/app/floating_controls.rs` EXACTLY per §5.3.
6. CREATE `src/app/floating_controls_tests.rs` EXACTLY per §6.2.
7. EDIT `src/app.rs` per §5.1: delete `open_toolbar_button`, the top-panel block, the rails gate; replace `ui()` and the `impl eframe::App` block with the §5.1 structure; add the four constants; update the `image_is_open` doc line; replace module registrations. Touch NOTHING else.
8. Static self-verification per §9 (NO cargo); fix only self-found drift by re-editing the files.
9. Commit per §8.1/§8.2; then `git status` (clean except the NOT-staged items above) and `git log --oneline -1` (verify hash + message).

### 8.1 Staging list (EXACT — gitignore-checked; nothing else)

| Staged | Why |
| --- | --- |
| `src/app.rs` | §5.1 edits |
| `src/app/arrow_direction.rs` | new arrow types module |
| `src/app/floating_controls.rs` | new pill module |
| `src/app/floating_controls_tests.rs` | relocated enablement test |
| `src/app/rails.rs` (deleted, `git rm`) | obsolete rails |
| `src/app/rails_tests.rs` (deleted, `git rm`) | obsolete rails tests |
| `.kilo/plans/20261101-floating-controls.md` | this plan (4.1b artifact) |
| `.kilo/plans/20261101-floating-controls-frontend-spec.md` | the task's untracked 4.1a spec (committed alongside — Wave-2 Task-3 §7.2 precedent) |

NOT staged: `.agent/project-info/context.md` (upkeep from earlier steps — caller/planner territory), `dist/`, `logs/`, `target/` (gitignored — verify `git check-ignore` if in doubt), any TODO file, any docs file, `.agent/project-structure.md` (stale for rails → Cycle C docs task), `Cargo.toml`/`Cargo.lock` (unchanged). Never stage gitignore-matching paths; never force-add.

### 8.2 Commit — ONE commit, message EXACTLY

```
feat: centered floating navigation controls and empty-state open button (UI fixes)
```

NO amend, NO force, NO push (push restricted to workflow step 5, `main` → `origin` ONLY). No TODO marks (step 4.6). No docs edits (Cycle C).

## 9. Implementer self-verification checklist (static, read/search only)

1. Line counts: app.rs ≈183 (≤200), floating_controls.rs ≈150, arrow_direction.rs ≈75, floating_controls_tests.rs ≈38; `git diff --stat` shows only §8.1 paths.
2. Search `src/`: the strings `open_toolbar`, `rails`, `folder_rail`, `image_rail`, `RAIL_SIZE` appear NOWHERE; `Open…` appears exactly once (`OPEN_LABEL_TEXT` in app.rs) and its only render site is the empty-state branch.
3. `mod arrows`: `src/app.rs` bottom declares exactly `mod arrow_direction;`, `mod floating_controls;`, `#[cfg(test)] mod tests;`, `#[cfg(test)] mod floating_controls_tests;`.
4. Key intake byte-identical at the top of `ui()`; `ui()` = intake → single `image_is_open()` branch.
5. Every method ≤50 lines; nesting ≤2; every `if` condition single-section (compound `&&` only inside `accepts_click`/`arrow_button_is_enabled`); every fn ≤2 non-self params; no `pub`/`pub(crate)` additions except the `pub(super)` items listed in §5; fields stay private.
6. Click arms call ONLY `app.navigate(NavigationDirection::Previous/Next)`; Up/Down have NO handler; enablement = `neighbor_image_path(...).is_some()` via the moved helper only.
7. No commented-out code; no `// TODO`; no CHANGELOG/docs/plan edits.
8. `git status` after commit: clean except `context.md` (unstaged) and the two staged plan files committed.

### 8.3 Hazard report format (REPORT, do not fix — Cycle B runs the container dev-checks)

The implementer reports these verified/pattern hazards with the step report: (a) `Shape::rotated_triangle(rect, rotation, fill)` — verified on docs.rs egui 0.37 (rotation space 0.0 down / TAU÷4 left / TAU÷2 up / −TAU÷4 right); (b) `Frame::new().fill().corner_radius().inner_margin().show()` — verified docs.rs 0.37 (`From<u8>`→CornerRadius, `From<f32>`→Margin/Into chain, `Color32::from_rgba_unmultiplied` const); (c) `Area::new(Id).anchor(Align2, Into<Vec2>)` + `.order(egui::Order::Foreground)` — anchor verified (0.37 area source; ordering long-standing API per spec §4.1); (d) `Ui::allocate_exact_size -> (Rect, Response)`, `Sense::click`, `Response::widget_info(|| WidgetInfo::labeled(Role::Button, enabled, label))` — verified docs.rs 0.37 + accessibility docs; (e) `Response::on_hover_cursor` CONSUMES and returns the Response — the reassignment in §5.3 is mandatory; (f) `CursorIcon::PointingHand` — verified docs.rs 0.37; (g) `Color32::from_rgba_unmultiplied` as `const` initializer — long-standing const fn (verify in container); (h) the `&mut app` capture through nested closures (`Area.show → Frame.show → row closure`) — standard egui sequential pattern (rails.rs precedent; verify borrow); (i) rustfmt wrapping of the longest §5.3 lines — if `cargo fmt --check` fails in Cycle B, remediation is a `cargo fmt` run in the 4.3/B fix cycle, never a manual reflow here; (j) clippy `nursery/pedantic` candidates: none used by design; `match self` arms with `Some/None` are exhaustively covered. ANY other suspected hazard is reported the same way.

## 10. Risk notes

- **Pill-over-image contrast**: the translucent dark fill (alpha 180/255) keeps the icons readable over light images; the icon colors come from default visuals (white-on-default-theme / near-black in light theme — over a translucent DARK fill, a light default theme could invert icon contrast on a light background image. Recorded AS-IS for the OPERATOR-RUN re-test: if the operator reports weak contrast, the fix is a Planner-level token decision, NOT implementer improvisation).
- The painted-button hit area is the full 36×32 rect; disabled reserve buttons swallow clicks over the image but call nothing (no zoom/pan exists in this wave).
- Borrow/const hazards bounded by §8.3; test-count parity resolved authoritatively in Cycle B.
- `.agent/project-structure.md` becomes stale for `rails.rs → floating_controls` — expected, Cycle C owns it (recorded so 4.5b doesn't misread it).

## 11. Out of scope (explicit)

- Wave-3 `↑`/`↓` wiring, sibling-folder navigation, last-viewed memory (`ArrowControl::navigation_direction` returning `None` for folder arrows is the ONLY concession made for Wave 3's reserve).
- Wave-5 pan-then-navigate; menu placeholder/"Open with…"; fullscreen/zoom/fit-mode controls; new keyboard shortcuts; tooltips; beyond-default theming.
- ALL docs incl. `.agent/project-structure.md`, `CHANGELOG.md`, existing plan files, TODO files, project-info files.
- Cargo/fmt/clippy/test runs and the exe rebuild (Cycle B Tasks 4); OPERATOR-RUN re-test item list stays as TODO Task 4 states.

## 12. Verification mapping (TODO Tasks 1–3 → plan elements; 4.5a §7 checklist cross-ref)

| TODO task / sub-item | Covered by |
| --- | --- |
| Task 1: guaranteed-visible icons on Windows 10/11 | §1 (vector triangles, no font dependency) + §2.5/§5.2/§5.3 exact painter code; 4.5a checklist item 1 + OPERATOR-RUN |
| Task 2: rails/toolbars removed; centered floating buttons on the existing navigation path | §2.1 (deletions/registration), §2.2–2.6 anatomy, §4.2 removals, §5 exact code; enablement via `navigation_button_enabled` (§3); message = small label above the row inside the SAME Area/Frame (§2.6); image-open gate kept by the caller (`ui()` branch) |
| Task 2: Wave-3 ↑/↓ visible + disabled, handler-free | §2.3 row order `[←] [→] (gap) [↑] [↓]`, §5.2 `navigation_direction()` = None, §5.3 dimmed `noninteractive` color, `accepts_click` gate |
| Task 2: K3 message semantics | §2.6 (`Some` → label / `None` → empty) + structural image-open gating §4 |
| Task 3: NO Open control while an image is open; big centered Open in the empty state; native dialog reuse | §4/§4.1 branch structure, `OPEN_LABEL_TEXT` uniqueness assertion (§9-2), `open_via_dialog` reuse, Area CENTER_CENTER 18-pt/180×48 |
| Cross-checks: key intake frame-first unchanged; overlay consumes no arrow keys; texture lifecycle untouched; rule compliance; test updates recorded | §4, §2.2 (defaults), §0, §5 audits, §6 |

---

## Addendum — Decisions Summary for the caller

1. **ICONS: egui-painted vector triangles** via verified `egui::Shape::rotated_triangle` (0.37 arrows-by-design API — no font dependency, kills the Windows tofu defect); exact disabled (`widgets.noninteractive.fg_stroke`)/hover (`hovered` + `CursorIcon::PointingHand`)/idle (`inactive`) visuals encoded; 36×32 hit rect (spec §4.4), 10×10 triangle, AccessKit `WidgetInfo::labeled(Role::Button, …)`; compound click gate isolated in `accepts_click`.
2. **PILL ANATOMY:** ONE `Area` (id `"floating_navigation_pill"`) anchored `CENTER_BOTTOM` offset `(0,−24)`, `Order::Foreground`, containing one rounded translucent-dark `Frame` (r10, fill rgba(20,20,20,180), margin 8) with: message label (12 pt, `Some`→text else empty) ABOVE one left-to-right row in order **`[←] [→] (16-pt gap) [↑] [↓]`**; ←/→ drive EXACTLY `app.navigate(Previous/Next)`; ↑/↓ painted-disabled Wave-3 reserves; enablement derived per frame via the MOVED `pub(super) navigation_button_enabled` (`neighbor_image_path(...).is_some()`); buttons stateless.
3. **EMPTY STATE:** `ui()` else-branch = empty CentralPanel + ONE `egui::Button` ("Open…", RichText 18 pt, min 180×48) inside an `Area` anchored `CENTER_CENTER` (the frontend-spec §5.1 second pattern — chosen because `centered_and_justified` would stretch the button; `Ui::centered` doesn't exist in 0.37, verified); click → `open_via_dialog(frame)`; top `open_toolbar` panel + `open_toolbar_button` retired fully; ZERO Open controls while an image is open (branch-structural, asserted in checklist).
4. **MODULE/TEST MAPPING:** DELETE `src/app/rails.rs` + `src/app/rails_tests.rs` (git rm; history retains); CREATE `src/app/arrow_direction.rs` (ArrowDirection + ArrowControl, child of `app`), `src/app/floating_controls.rs` (pill + moved helper), `src/app/floating_controls_tests.rs` (enablement test relocated, same semantics); `src/app/tests.rs` verified untouched (state-only tests). Counts: app.rs ≈183, pill ≈150, arrows ≈75, tests ≈38 — all ≤200; test census stays 26.
5. **ui() FINAL ORDER:** byte-identical arrow-key intake → `if image_is_open { pill Area + CentralPanel image paint } else { CentralPanel(nothing) + center-Open Area }`.
6. **GIT:** one commit, staging exactly the 3 new + 1 edited source files + 2 git-rm deletions + this plan + the 4.1a spec; message `feat: centered floating navigation controls and empty-state open button (UI fixes)`; no cargo (hazards §8.3 reported), no push, no TODO/docs/cargo work.
