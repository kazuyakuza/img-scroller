# Wave-4 Task 2 — Zoom In/Out: Implementation Plan (Critical Workflow step 4.1b)

- Date: 2026-11-02
- Source TODO: `.agent/todos/20261029/20261029-todo-6.md` → **Task 2: Zoom in/out** (ONLY Task 2 — do not touch Task 3/4/5 sub-items)
- Caller-approved global plan: `.kilo/plans/20261102-wave-4-fullscreen-zoom.md` (this 4.1b is its Task-2 second cycle step)
- Verified-API source (read-only, do NOT edit): `.kilo/plans/20261102-wave4-task2-zoom-frontend-spec.md` ("the spec")
- Target implementer: JUNIOR developer at 50% restriction. Every decision below is already made. Do not re-decide anything. If a detail here conflicts with something you observe, STOP and return the question to the caller.
- Branch: `feat/wave-4-fullscreen-zoom` — **confirmed HEAD `7276fe9` at plan time** (`git log`). Recent history: Task-1 fullscreen landed and marked done (`9253d4a` review fixes, `77a8a1b` spec+simplification docs, `4486b81` module docs, `24e0615` verification docs, `7276fe9` task-1 mark). Version is already **0.4.0** (bump commit `4985c6f`). Uncommitted at plan time: `.agent/project-info/context.md` (modified) and the untracked `.kilo/plans/20261102-wave4-task2-zoom-frontend-spec.md` (spec file) — these stay untouched by the implementer.
- Do NOT edit any existing `.kilo/plans/*` file; this file is the new Task-2 plan.

---

## 0. Binding user-approved decisions (restated verbatim = §0 restatement)

The user-approved binding block this plan encodes:

1. **Zoom in hotkeys**: `I` (egui `Key::I` aliasing) AND Shift+Plus (`Shift` held + `=` producing '+'); **zoom out hotkeys**: `O` AND Shift+Minus — FOUR hotkeys total; each edge-triggered via `key_pressed`-style per-frame intake; NO collision with arrows/F11/resize keys.
2. **UI controls INSIDE the existing bottom-center pill**: zoom-in [+], zoom-out [−], AND restore-zoom-to-fit buttons (painted-shape icon family, no font glyphs) — placement order after the fullscreen square, accessible Button info like the others.
3. **Restore-zoom-to-fit action**: hotkey `0` (egui `Key::Num0` — exact 0.37 enum member, verified in §1) + that UI button; resets zoom to pure fit (factor 1.0).
4. **Step**: ×1.25 fixed per zoom-in/out step (multiplicative); min = fit (1.0 — never below fit); max = ×8.0; clamped at bounds with pill message-line feedback (`Zoom max` / `Zoom min` in the pill's message line UI).
5. **Reset policy**: zoom resets to fit on EVERY successful image open ("When load a folder, always start with image fit") — i.e., on every successful `open_image` outcome; no reset on fullscreen toggle or empty-folder mode transitions (state untouched there).
6. **Session-only; no persistence.**
7. **Fullscreen interplay**: zoom UI buttons are chrome → NOT rendered while fullscreen ON; zoom/restore hotkeys remain active in every mode (consistent with F11's always-intake pattern); the zoomed image surface replaces the fitted display the same way in both modes.

### Decision ledger (the plan's own binding resolutions — implementer applies w/o judgment)

| # | Decision | Encoded choice |
|---|----------|----------------|
| D1 | Reset policy | **Every successful open resets zoom to fit** (the verbatim user sentence wins over the spec §4 "Recommendation B"; the spec explicitly deferred this to the architector). Failure paths never touch `zoom_factor`. |
| D2 | At-bound message scheme | **Rejected-step only**: the message is set exactly when a zoom step is attempted while ALREADY at its corresponding bound (the step's factor does not change). A step that *lands* on a bound silently clamps, no message. The message goes through the existing `message_text` channel → clears automatically on the next successful open (existing policy). No message-precedence computation is added. |
| D3 | Button enablement (consistent scheme) | Like the arrows (disabled exactly at the dead bound): zoom-in enabled iff NOT at ×8.0 max; zoom-out enabled iff NOT at fit (1.0); restore-fit enabled iff NOT at fit (disabled at fit, per the user's settled wording). Disabled buttons paint dimmed and stay inert (same pattern as the arrows today). |
| D4 | Keyboard alias encoding | `Shift+Plus` is accepted through BOTH logical resolutions egui 0.37 can deliver: `Key::Plus` pressed, OR `Key::Equals` pressed while `modifiers.shift` is held. `Shift+Minus`: `Key::Minus` pressed while `modifiers.shift` is held. Plain (unshifted) `Equals`/`Minus`/`Plus` are NOT hotkeys on their own — only `I`, `O`, `0`, and the two shift chords. |
| D5 | Pure-math home | NEW module `src/zoom.rs` — sibling of `src/scaling.rs`, no egui *painting* (uses `egui::Vec2` math only, same stance as `scaling.rs`). NOT inside `src/scaling.rs` (0.4 zoom/fit-width/height waves would inflate that file toward its cap; a sibling keeps the wave split clean). |
| D6 | App seam split | State field + seam in `src/app/zoom.rs`; buttons in `src/app/zoom_button.rs`; keyboard intake extracted into `src/app/keyboard_intake.rs` (app.rs is at 199/200 lines and CANNOT take zoom code — see §1). |
| D7 | Paint-call args | Project rule "≤2 args" forbids the spec's 3-param `paint`. Encoded: a 2-field parameter object `ImagePainting { decoded_image, zoom_factor }` in `src/app_display.rs` (fields `pub(crate)` — necessary cross-module, justified by the max-args rule). |
| D8 | Icon designs | Plus = two crossed filled bars; Minus = one horizontal filled bar; Restore-fit = stroked (unfilled) square outline (distinct from the fullscreen button's filled square; `Shape::rect_stroke` with an explicit `StrokeKind` — REQUIRED since egui 0.31, verified). All painted via `Shape` primitives + the existing `widgets.{inactive,hovered,noninteractive}.fg_stroke` palette. |
| D9 | Fullscreen dispatch | NO change to `full_ui_chrome_is_visible()` semantics or the dispatch order; zoom buttons inherit the pill-gate (they live inside the pill's render path). |

---

## 1. Verified landed-state facts (researched 2026-11-02, HEAD `7276fe9`)

### 1.1 Code state (line counts verified)

| File | Lines | Note |
|---|---|---|
| `src/app.rs` | 199 | 1 line of headroom — the intake block inside `impl eframe::App` spans lines 133–159 and MUST be extracted (D6) |
| `src/app/floating_controls.rs` | 176 | pill: message line + `[←][→] [↑][↓]` + fullscreen square in `show_button_row` |
| `src/app/fullscreen.rs` | 47 | `full_ui_chrome_is_visible()`, `toggle_fullscreen_and_send_command()`, `show_fullscreen_layout()` |
| `src/app/fullscreen_button.rs` | 47 | painted-square precedent (filled `egui::Shape::rect_filled`, 36×32 button, `WidgetInfo::labeled`) |
| `src/app/empty_state_view.rs` / `empty_folder_view.rs` / `folder_navigation.rs` / `arrow_direction.rs` | 41 / 16 / 156 / 59 | untouched |
| `src/app_display.rs` | 107 | `CurrentImageDisplay::paint(panel_ui, decoded_image)` — the display path to extend |
| `src/scaling.rs` | 95 | `fitted_size(...)` pure math + 5 inline tests — untouched |
| Unit tests | **57** | verified by `#[test]` census: app/tests 9, floating_controls_tests 1, folder_navigation_tests 5, folder_navigation_continuation_tests 4, fullscreen_tests 7, last_viewed_tests 5, image_loading 4, app_display 2, name_collation 2, scaling 5, folder_listing/tests 5, sibling_folder_listing tests 6 + neighbor 2 = 57 |
| `src/main.rs` | 28 | declares all top-level `mod`s — gains 1 line |
| `Cargo.toml` / `Cargo.lock` | — | NO dependency change → both untouched; version already 0.4.0 |

Open-success wiring point (the D1 landing point): **`src/app.rs` → `accept_open_outcome` → the `Ok(decoded_image)` match arm** — today lines 72–82 (sets listing, last-viewed, sibling folders, clears `browsed_folder_context`, stores path+payload, `message_text = None`, flags the texture upload). The zoom reset line lands there.

### 1.2 Verified egui 0.37 API facts (Context7 lookups, 2026-11-02; consistent with the spec §3)

1. `egui::Key` **includes** `Plus`, `Minus`, `Equals`, `Num0`…`Num9`, letters `A`…`Z` (hence `I`, `O`), `F1`…`F35`, the four arrows. `Num0` is the exact digit-zero member name.
2. `InputState::key_pressed(Key) -> bool` — edge-triggered (true for one frame per physical press; **includes key-repeat** — holding `I` zooms repeatedly; accepted, same class as the arrows/F11 pattern).
3. `InputState.modifiers` is a public field: `Modifiers { alt, ctrl, shift, mac_cmd, command }` — read as `ctx.input(|i| i.modifiers.shift)`.
4. `egui::Image::new(texture).fit_to_exact_size(Vec2)` — builder; the current display primitive (kept).
5. **`egui::Context::input(|i| ...)` is read-only intake** (the established pattern in `app.rs` today).
6. `Ui::allocate_exact_size(desired_size, sense)` returns "a Rect with exactly what you asked for… never smaller than requested" — so a zoomed Image widget requesting a size larger than the panel keeps its requested rect (centered by `Layout::centered_and_justified`) and its **paint is clipped by the panel's Ui clip rect** → the visible region is the CENTER of the zoomed image; the overflow is simply not drawn. No scrollbar, no pan, no manual clip manipulation this wave.
7. `egui::Shape::rect_stroke(rect, corner_radius, stroke, stroke_kind: StrokeKind)` — the `StrokeKind` argument is **required** (mandatory since 0.31, PR #5648; still true in 0.37). Encode `egui::StrokeKind::Inside`. `egui::Shape::rect_filled(rect, corner_radius, fill_color)` — 3 args (landed precedent in `fullscreen_button.rs`).
8. `egui::WidgetInfo::labeled(egui::Role::Button, enabled: bool, label)` — the accessible-button-info precedent (landed).
9. `egui::Shape::rotated_triangle(rect, rotation, color)` — landed arrows precedent; NOT used for zoom (zoom icons are rects/outline per D8).

### 1.3 Pitfall to know BEFORE typing (two module namings)

Two zoom modules exist with the same leaf name `zoom`:
- `crate::zoom` = the pure-math module `src/zoom.rs` (declared in `src/main.rs`).
- `super::zoom` = the app-seam module `src/app/zoom.rs` (declared as `mod zoom;` in `src/app.rs`).

Therefore:
- **In `src/app.rs`: never write `use crate::zoom;`** — the child `mod zoom;` name collides (error E0255). Use the fully-qualified paths `crate::zoom::RESTORE_TO_FIT_FACTOR` at its 2 use sites (S5).
- `src/app_display.rs` (a top-level module) imports the PURE math: `use crate::zoom;`.
- `src/app/zoom.rs` (pure-math customer) imports `use crate::zoom;`.
- `src/app/keyboard_intake.rs` and `src/app/zoom_tests.rs` use the APP seam: `use super::zoom;` and `use super::zoom_button;` (NOT `crate::zoom` — the app seam owns `RESTORE_FIT_KEY` and the chord functions).
- `src/app/zoom_button.rs` imports neither zoom module (it drives the app seam through methods on `ImagesScrollerApp`).

---

## 2. File set (exact paths, expected final line counts)

**New files (5):**

| File | Expected lines ≤ | Contents |
|---|---|---|
| `src/zoom.rs` | 150 | pure zoom math + its inline `#[cfg(test)] mod tests` (8 tests) |
| `src/app/zoom.rs` | 110 | `zoom_factor` seam: state actions, bound-predicates, pill-message assignment, keyboard chord helpers, key/message constants |
| `src/app/zoom_button.rs` | 170 | the three pill buttons (paint + click + enablement + accessible labels) |
| `src/app/keyboard_intake.rs` | 60 | the extracted per-frame keyboard intake (arrows + F11 + zoom chords, copied-behavior) |
| `src/app/zoom_tests.rs` | 130 | app-state zoom tests (11 tests) |

**Edited files (5):**

| File | Expected result ≤ | Changes |
|---|---|---|
| `src/app.rs` | 190 | +`zoom_factor` field(+init), +reset-on-open, intake moved out (22 lines removed, 1 line call added), +zoomed paint call, +3 `mod` + 1 test-`mod` declarations, +1 import change |
| `src/main.rs` | 30 | +`mod zoom;` |
| `src/app/floating_controls.rs` | 178 | +zoom group appended after the fullscreen button (1 fn call + module-doc phrase) |
| `src/app_display.rs` | 125 | +`ImagePainting` param object, `paint` computes `fitted * zoom`, +`use crate::zoom;` |
| `src/app/fullscreen.rs` | 50 | fullscreen layout's paint call gains the zoom factor |

Untouched: `src/scaling.rs`, `src/app/arrow_direction.rs`, `src/app/empty_state_view.rs`, `src/app/empty_folder_view.rs`, `src/app/folder_navigation.rs`, `src/platform/*`, `src/image_loading.rs`, `Cargo.toml`, `Cargo.lock`, ALL `.md` docs (Task 5 territory), `CHANGELOG.md` (never).

Every file ≤200 lines; every method ≤50 lines; depth ≤2; ≤2 args; private-by-default (`pub(crate)`/`pub(super)` only where encoded); no commented-out code; single-section boolean conditions (all predicates extracted into named functions per §3).

---

## 3. Ordered implementation steps (tiny; follow exactly)

### S1 — Create `src/zoom.rs` (pure math)

Module doc + API exactly:

```rust
//! Zoom math (Wave-4 Task 2): the multiplicative step scaling within fixed
//! bounds, the restore-to-fit reset, the at-bound detection, and the
//! fit-relative draw size. Pure calculations only: no egui painting, no
//! texture or pixel code. Sibling of `src/scaling.rs`.

use eframe::egui;

/// Fixed multiplicative factor of one zoom-in/zoom-out step.
pub(crate) const ZOOM_STEP_MULTIPLIER: f32 = 1.25;
/// Lower zoom bound: plain whole-image fit (zooming out below fit is rejected).
pub(crate) const ZOOM_FACTOR_MINIMUM: f32 = 1.0;
/// Upper zoom bound: the largest supported zoom-in.
pub(crate) const ZOOM_FACTOR_MAXIMUM: f32 = 8.0;
/// The restore-to-fit target captured as a named value (family of the bounds).
pub(crate) const RESTORE_TO_FIT_FACTOR: f32 = ZOOM_FACTOR_MINIMUM;
/// Absorption bound for the at-bound detections (fits-factor drift guard).
const ZOOM_BOUND_TOLERANCE: f32 = 1.0e-4; // 0.0001 f32 epsilon-class guard
```

Functions (all `pub(crate)`, each ≤6 lines, depth 1):

```rust
/// The zoom factor after one zoom-in step from `factor`, clamped to the bounds.
pub(crate) fn zoom_in_factor(factor: f32) -> f32 {
    clamped_zoom_factor(factor * ZOOM_STEP_MULTIPLIER)
}

/// The zoom factor after one zoom-out step from `factor`, never below fit.
pub(crate) fn zoom_out_factor(factor: f32) -> f32 {
    clamped_zoom_factor(factor / ZOOM_STEP_MULTIPLIER)
}

fn clamped_zoom_factor(factor: f32) -> f32 {
    factor.clamp(ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM)
}

/// Whether `factor` has reached the fit floor (zoom-out/restore are inert there).
pub(crate) fn zoom_factor_is_at_minimum(factor: f32) -> bool {
    factor <= ZOOM_FACTOR_MINIMUM + ZOOM_BOUND_TOLERANCE
}

/// Whether `factor` has reached the ceiling (zoom-in is inert there).
pub(crate) fn zoom_factor_is_at_maximum(factor: f32) -> bool {
    factor >= ZOOM_FACTOR_MAXIMUM - ZOOM_BOUND_TOLERANCE
}
```

Draw-size seed for Wave-5 pan composition:

```rust
/// The per-frame draw size of the image at `fit_size` scaled by the zoom factor
/// (Wave-5 pan composes its offsets on this size; no scroll/pan this wave).
pub(crate) fn zoomed_draw_size(
    fit_size: egui::Vec2,
    zoom_factor: f32,
) -> egui::Vec2 {
    fit_size * zoom_factor
}
```

Inline tests live in `src/zoom.rs` (S9 zp1–zp8). Register `mod zoom;` in `src/main.rs` (one line, placed with the other top-level `mod`s, alphabetical after `mod name_collation;`).

### S2 — Create `src/app/zoom.rs` (state seam + keyboard chords + pill feedback)

```rust
//! Manual zoom (Wave-4 Task 2): the session zoom-factor seam, its keyboard
//! chords (I / Shift+Plus, O / Shift+Minus, 0 = restore fit), and the
//! at-bound pill-message assignment. Buttons live in `zoom_button.rs`; the
//! pure math lives in `crate::zoom`. Session state only: never persisted.

use eframe::egui;

use super::ImagesScrollerApp;
use crate::zoom;

/// Zoom-in hotkey `I` (alias family: the Shift+'+' chord below).
const ZOOM_IN_KEY: egui::Key = egui::Key::I;
/// Zoom-out hotkey `O` (alias family: the Shift+'-' chord below).
const ZOOM_OUT_KEY: egui::Key = egui::Key::O;
/// Restore-to-fit hotkey: the digit zero (egui's exact digit member).
pub(super) const RESTORE_FIT_KEY: egui::Key = egui::Key::Num0;

/// Pill message shown when a zoom-out step is rejected at the fit bound
/// (pinned by the zoom tests; reaches the pill through `message_text`).
pub(super) const ZOOM_AT_MINIMUM_MESSAGE: &str = "Zoom min";
/// Pill message shown when a zoom-in step is rejected at the x8 bound.
pub(super) const ZOOM_AT_MAXIMUM_MESSAGE: &str = "Zoom max";
```

Impl block on `ImagesScrollerApp` (all `pub(super)` except the two trap helpers):

```rust
impl ImagesScrollerApp {
    /// One zoom-in step; a step rejected at the ceiling reports "Zoom max".
    pub(super) fn zoom_in_once(&mut self) {
        if self.trap_zoom_maximum() {
            return;
        }
        self.zoom_factor = zoom::zoom_in_factor(self.zoom_factor);
    }

    /// One zoom-out step; a step rejected at the fit floor reports "Zoom min".
    pub(super) fn zoom_out_once(&mut self) {
        if self.trap_zoom_minimum() {
            return;
        }
        self.zoom_factor = zoom::zoom_out_factor(self.zoom_factor);
    }

    /// Hotkey `0` and the restore button: returns the zoom to whole-image fit.
    /// Silent by design everywhere (the fit state is its own feedback).
    pub(super) fn restore_zoom_to_fit(&mut self) {
        self.zoom_factor = zoom::RESTORE_TO_FIT_FACTOR;
    }

    /// Zoom-in-button enablement: enabled until the x8 ceiling is reached.
    pub(super) fn zoom_can_grow(&self) -> bool {
        !zoom::zoom_factor_is_at_maximum(self.zoom_factor)
    }

    /// Enablement of BOTH the zoom-out and the restore-fit buttons:
    /// enabled everywhere except exactly at the fit floor.
    pub(super) fn zoomed_beyond_fit(&self) -> bool {
        !zoom::zoom_factor_is_at_minimum(self.zoom_factor)
    }

    /// Rejected-step feedback (D2): a step attempted while already at the
    /// bound leaves the factor unchanged; the message clears on the next
    /// successful open (the standing `message_text` policy).
    fn trap_zoom_maximum(&mut self) -> bool {
        if !zoom::zoom_factor_is_at_maximum(self.zoom_factor) {
            return false;
        }
        self.message_text = Some(ZOOM_AT_MAXIMUM_MESSAGE);
        true
    }

    /// The fit-floor twin of `trap_zoom_maximum` (D2).
    fn trap_zoom_minimum(&mut self) -> bool {
        if !zoom::zoom_factor_is_at_minimum(self.zoom_factor) {
            return false;
        }
        self.message_text = Some(ZOOM_AT_MINIMUM_MESSAGE);
        true
    }
}
```

Chord helpers (free functions, bottom of `src/app/zoom.rs` — consumed by `keyboard_intake`; each passes a `&egui::Context`; every `if` is a single call, single-section):

```rust
/// The zoom-in chord set: hotkey `I`, or the Shift+'+' alias (Shift held
/// while `=`/`+` is pressed; BOTH egui logical resolutions of Shift+Plus are
/// accepted -- D4).
pub(super) fn zoom_in_chord_was_pressed(chord_context: &egui::Context) -> bool {
    if chord_context.input(|input| input.key_pressed(ZOOM_IN_KEY)) {
        return true;
    }
    zoom_in_shift_alias_was_pressed(chord_context)
}

fn zoom_in_shift_alias_was_pressed(chord_context: &egui::Context) -> bool {
    if chord_context.input(|input| input.key_pressed(egui::Key::Plus)) {
        return true;
    }
    if !chord_context.input(|input| input.key_pressed(egui::Key::Equals)) {
        return false;
    }
    chord_context.input(|input| input.modifiers.shift)
}

/// The zoom-out chord set: hotkey `O`, or the Shift+'-' alias (Shift held
/// while `-` resolves to `Key::Minus`).
pub(super) fn zoom_out_chord_was_pressed(chord_context: &egui::Context) -> bool {
    if chord_context.input(|input| input.key_pressed(ZOOM_OUT_KEY)) {
        return true;
    }
    if !chord_context.input(|input| input.key_pressed(egui::Key::Minus)) {
        return false;
    }
    chord_context.input(|input| input.modifiers.shift)
}
```

`zoom_in_chord_was_pressed` etc. are `pub(super)` so `src/app/keyboard_intake.rs` can call them.

### S3 — Create `src/app/zoom_button.rs` (three pill buttons)

Design: one tiny spec struct + one enum, mirroring the landed `floating_controls.rs` button pattern (allocate 36×32 → `WidgetInfo::labeled` → pointing-hand cursor → paint glyph → click dispatch). Order in the pill row: `[+] [−] [□-fit]` immediately after the fullscreen square.

```rust
//! The floating pill's zoom buttons (Wave-4 Task 2, user-approved placement
//! INSIDE the pill, after the fullscreen square): zoom-in [+], zoom-out [-],
//! and restore-zoom-to-fit -- painted rectangle-shape glyphs, no font glyphs.
//! Enablement follows the at-bound scheme (D3); the restore-fit marker is a
//! stroked (unfilled) square to stay distinct from the fullscreen filled one.

use eframe::egui;

use super::ImagesScrollerApp;
```

(This module imports NEITHER zoom seam — the buttons drive the actions purely through methods on `ImagesScrollerApp`; see §1.3.)

Types (two tiny items mirroring the landed pill-button pattern):

```rust
enum ZoomButtonAction {
    ZoomIn,
    ZoomOut,
    RestoreFit,
}

struct ZoomButtonSpec {
    action: ZoomButtonAction,
    enabled: bool,
}
```

Constants (the three labels are pinned by `src/app/zoom_tests.rs`, hence `pub(super)`; everything else stays private to the module):

```rust
const ZOOM_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const ZOOM_GROUP_GAP: f32 = 16.0;
/// One zoom-glyph bar length/thickness (the + and - icon geometry).
const GLYPH_BAR_LENGTH: f32 = 12.0;
const GLYPH_BAR_THICKNESS: f32 = 2.5;
/// The restore-fit marker: a stroked square outline.
const FIT_MARKER_SIDE: f32 = 11.0;
const FIT_MARKER_STROKE_WIDTH: f32 = 2.0;

pub(super) const ZOOM_IN_BUTTON_LABEL: &str = "Zoom in";
pub(super) const ZOOM_OUT_BUTTON_LABEL: &str = "Zoom out";
pub(super) const RESTORE_FIT_BUTTON_LABEL: &str = "Restore zoom to fit";
```

Entry point called from `floating_controls.rs`:

```rust
/// Appends the zoom button group to the pill row, separated by the standard
/// group gap after the fullscreen square (user-approved placement).
pub(super) fn show_zoom_pill_buttons(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    row_ui.add_space(ZOOM_GROUP_GAP);
    let button_specs = zoom_button_specs(app);
    for button_spec in &button_specs {
        button_spec.show_and_apply_action(app, row_ui);
    }
}

fn zoom_button_specs(app: &ImagesScrollerApp) -> [ZoomButtonSpec; 3] {
    [
        ZoomButtonSpec::row_button(ZoomButtonAction::ZoomIn, app.zoom_can_grow()),
        ZoomButtonSpec::row_button(ZoomButtonAction::ZoomOut, app.zoomed_beyond_fit()),
        ZoomButtonSpec::row_button(ZoomButtonAction::RestoreFit, app.zoomed_beyond_fit()),
    ]
}
```

Types + behavior (mirror `floating_controls.rs::ArrowButtonSpec` — full method list of the `impl ZoomButtonSpec` block, nothing more):

1. `fn row_button(action: ZoomButtonAction, enabled: bool) -> Self` — constructor.
2. `fn show_and_apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui)` — the per-button per-frame path:

```rust
    /// Paints the glyph and, when clicked while enabled, applies its action.
    fn show_and_apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let (painted_rect, button_response) =
            row_ui.allocate_exact_size(ZOOM_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, self.accessible_label())
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_glyph(row_ui, &painted_rect, &button_response);
        if self.accepts_click(&button_response) {
            self.apply_action(app);
        }
    }
```

3. `fn accepts_click(&self, button_response: &egui::Response) -> bool` — `self.enabled && button_response.clicked()` (a disabled zoom button never fires — the disabled-arrow pattern).
4. `fn apply_action(&self, app: &mut ImagesScrollerApp)` — dispatch:

```rust
    fn apply_action(&self, app: &mut ImagesScrollerApp) {
        match self.action {
            ZoomButtonAction::ZoomIn => app.zoom_in_once(),
            ZoomButtonAction::ZoomOut => app.zoom_out_once(),
            ZoomButtonAction::RestoreFit => app.restore_zoom_to_fit(),
        }
    }
```

5. `fn accessible_label(&self) -> &'static str` — a `match self.action` returning the three `pub(super)` label constants.
6. `fn zoom_glyph_color(&self, row_ui: &egui::Ui, button_response: &egui::Response) -> egui::Color32` — the exact color ladder of `ArrowButtonSpec::arrow_color` (disabled → `widgets.noninteractive.fg_stroke.color`; hovered → `widgets.hovered.fg_stroke.color`; otherwise → `widgets.inactive.fg_stroke.color`).

The glyph painting (painted-shape family, no font glyphs — prescriptive; the shape builders take the ≤2-arg design and return the shapes; `paint_glyph` adds them to the painter):

```rust
    /// Paints the button's glyph shapes at the disabled/idle/hovered palette.
    fn paint_glyph(
        &self,
        row_ui: &egui::Ui,
        painted_rect: &egui::Rect,
        button_response: &egui::Response,
    ) {
        let glyph_center = painted_rect.center();
        let glyph_color = self.zoom_glyph_color(row_ui, button_response);
        for glyph_shape in self.zoom_glyph_shapes(glyph_center, glyph_color) {
            row_ui.painter().add(glyph_shape);
        }
    }

    /// Per-action glyph geometry (zoom icons are rects/outline only, D8).
    fn zoom_glyph_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        match self.action {
            ZoomButtonAction::ZoomIn => self.crossed_bar_shapes(glyph_center, glyph_color),
            ZoomButtonAction::ZoomOut => self.single_bar_shapes(glyph_center, glyph_color),
            ZoomButtonAction::RestoreFit => self.stroked_square_shapes(glyph_center, glyph_color),
        }
    }

    /// Zoom-in [+]: a horizontal bar crossed by a vertical one, both filled.
    fn crossed_bar_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let horizontal = egui::Shape::rect_filled(
            egui::Rect::from_center_size(
                glyph_center,
                egui::Vec2::new(GLYPH_BAR_LENGTH, GLYPH_BAR_THICKNESS),
            ),
            egui::CornerRadius::ZERO,
            glyph_color,
        );
        let vertical = egui::Shape::rect_filled(
            egui::Rect::from_center_size(
                glyph_center,
                egui::Vec2::new(GLYPH_BAR_THICKNESS, GLYPH_BAR_LENGTH),
            ),
            egui::CornerRadius::ZERO,
            glyph_color,
        );
        vec![horizontal, vertical]
    }

    /// Zoom-out [−]: one horizontal filled bar.
    fn single_bar_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let bar = egui::Shape::rect_filled(
            egui::Rect::from_center_size(
                glyph_center,
                egui::Vec2::new(GLYPH_BAR_LENGTH, GLYPH_BAR_THICKNESS),
            ),
            egui::CornerRadius::ZERO,
            glyph_color,
        );
        vec![bar]
    }

    /// Restore-fit icon: a stroked square outline (fit marker; distinct from
    /// the fullscreen button's FILLED square).
    fn stroked_square_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let marker_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(FIT_MARKER_SIDE, FIT_MARKER_SIDE),
        );
        let marker = egui::Shape::rect_stroke(
            marker_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(FIT_MARKER_STROKE_WIDTH, glyph_color),
            egui::StrokeKind::Inside,
        );
        vec![marker]
    }
```

(If any method ends up exceeding 2 non-self arguments, re-shape it like the builders above did; do NOT relax the project rules.)

### S4 — Create `src/app/keyboard_intake.rs` (extract the intake block)

One new impl method taking over the entire arrow/F11 intake of `src/app.rs` lines 133–159, PLUS the three zoom chords (all edge-triggered, evaluated in EVERY mode, before the layout dispatch — binds the fullscreen interplay clauses of §0 items 1+7):

```rust
//! Per-frame keyboard intake: every edge-triggered chord of the app (the
//! fullscreen toggle, the four navigation arrows, the zoom in/out/restore
//! chords) is read once per frame and dispatched; the intake runs in EVERY
//! mode (fullscreen included) and always BEFORE the layout dispatch.

use eframe::egui;

use super::fullscreen;
use super::ImagesScrollerApp;
use super::NavigationDirection;
use super::zoom;

impl ImagesScrollerApp {
    /// Reads every chord for this frame and dispatches its shared action.
    /// One `key_pressed` read per chord key (no hidden chained collisions).
    pub(super) fn read_and_dispatch_keystrokes(&mut self, screen_context: &egui::Context) {
        let fullscreen_pressed =
            was_pressed(screen_context, fullscreen::FULLSCREEN_TOGGLE_KEY);
        let zoom_in_pressed = zoom::zoom_in_chord_was_pressed(screen_context);
        let zoom_out_pressed = zoom::zoom_out_chord_was_pressed(screen_context);
        let restore_fit_pressed = was_pressed(screen_context, zoom::RESTORE_FIT_KEY);
        let left_pressed  = was_pressed(screen_context, egui::Key::ArrowLeft);
        let right_pressed = was_pressed(screen_context, egui::Key::ArrowRight);
        let up_pressed    = was_pressed(screen_context, egui::Key::ArrowUp);
        let down_pressed  = was_pressed(screen_context, egui::Key::ArrowDown);
        if fullscreen_pressed {
            self.toggle_fullscreen_and_send_command(screen_context);
        }
        if zoom_in_pressed {
            self.zoom_in_once();
        }
        if zoom_out_pressed {
            self.zoom_out_once();
        }
        if restore_fit_pressed {
            self.restore_zoom_to_fit();
        }
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
    }
}

/// The read entry every plain chord funnels through (module-local helper).
fn was_pressed(screen_context: &egui::Context, pressed_key: egui::Key) -> bool {
    screen_context.input(|input| input.key_pressed(pressed_key))
}
```

Notes: reads happen FIRST, dispatches AFTER (F11/arrows behavior preserved verbatim); the fullscreen remains reachable through the shared `toggle_fullscreen_and_send_command` (NO second viewport command is created); `RESTORE_FIT_KEY` is read via the same `was_pressed` seam (single-section condition per keystroke).

### S5 — Edit `src/app.rs` (state field + reset + mod decls; intake moved out)

Edits, in order:

1. Import change: `use crate::app_display::CurrentImageDisplay;` → `use crate::app_display::{CurrentImageDisplay, ImagePainting};` — do NOT add `use crate::zoom;` (E0255 pitfall §1.3).
2. Struct field, appended after `fullscreen_on: bool,`:

```rust
    /// Session zoom factor (Wave-4 Task 2): 1.0 = whole-image fit; read by the
    /// display path; reset on every successful open; never persisted.
    zoom_factor: f32,
```

3. `empty_state()` gains the init line (verbatim; full path per §1.3):

```rust
            zoom_factor: crate::zoom::RESTORE_TO_FIT_FACTOR,
```

4. `accept_open_outcome` — the `Ok(decoded_image)` arm gains the reset line BETWEEN the message-clearing and the pending-upload flag (exact insertion point, today's line 80–81):

```rust
                self.message_text = None;
                self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;
                self.current_display.accept_decoded_image();
```

5. The `impl eframe::App` block shrinks to:

```rust
impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.read_and_dispatch_keystrokes(ui.ctx());
        self.show_chrome_or_fullscreen_layout(frame, ui);
    }
}
```

(the 22-line per-frame bool/intake block moves to `src/app/keyboard_intake.rs` in S4 — the last preserved line ordering: intake first, layout second, exactly as today).

6. `show_navigation_layout` — the paint call gains the zoom factor:

```rust
        egui::CentralPanel::default().show(ui, |central_ui| {
            if let Some(decoded_image) = self.decoded_image.as_ref() {
                self.current_display.paint(
                    central_ui,
                    ImagePainting { decoded_image, zoom_factor: self.zoom_factor },
                );
            } else {
                empty_folder_view::show_empty_folder_message(central_ui);
            }
        });
```

7. Child-module declarations appended in the existing order (after `mod fullscreen_button;`), and the test module after the others:

```rust
mod zoom;

mod zoom_button;

mod keyboard_intake;
```

```rust
#[cfg(test)]
mod zoom_tests;
```

### S6 — Edit `src/app_display.rs` (zoomed paint + param object)

1. Import: add `use crate::zoom;` beside the existing `use crate::scaling;`.
2. Parameter object right above `CurrentImageDisplay` (D7; brief doc justifying the 2-arg shape):

```rust
/// The 2-argument paint request (project max-args rule): WHICH decoded image
/// is on screen and at WHAT session zoom factor its fitted size is drawn.
pub(crate) struct ImagePainting<'a> {
    pub(crate) decoded_image: &'a DecodedImage,
    pub(crate) zoom_factor: f32,
}
```

3. `CurrentImageDisplay::paint` becomes (the exact display primitive of the spec §9; overflow behavior = egui-verified §1.2 item 6):

```rust
    /// Paints the decoded image at `fit * zoom`, centered; size overflow is
    /// clipped by the panel's default clip rect (center-region visible).
    pub(crate) fn paint(&mut self, panel_ui: &mut egui::Ui, painting: ImagePainting) {
        self.ensure_current_image_texture(panel_ui.ctx(), painting.decoded_image);
        let Some(image_texture) = self.image_texture.as_ref() else {
            return;
        };
        let Some(fit_size) = scaling::fitted_size(
            panel_ui.available_size(),
            image_dimensions(painting.decoded_image),
        ) else {
            return;
        };
        let display_size = zoom::zoomed_draw_size(fit_size, painting.zoom_factor);
        panel_ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |image_ui| {
                image_ui.add(egui::Image::new(image_texture).fit_to_exact_size(display_size));
            },
        );
    }
```

No change to `ensure_current_image_texture` / the one-upload-per-decode discipline / the module's inline tests.

### S7 — Edit `src/app/fullscreen.rs` (fullscreen paint gains the factor)

`show_fullscreen_layout` keeps `full_ui_chrome_is_visible()` and the central surface EXACTLY as-is; only the paint call gains the parameter object. Add ONE import to the file (it currently has none for the display): `use crate::app_display::ImagePainting;`

```rust
    pub(super) fn show_fullscreen_layout(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |central_ui| {
            if let Some(decoded_image) = self.decoded_image.as_ref() {
                self.current_display.paint(
                    central_ui,
                    ImagePainting { decoded_image, zoom_factor: self.zoom_factor },
                );
            }
        });
    }
```

NO other fullscreen change: NO zoom reset on toggle, NO chrome-gate change, the pill (with the zoom buttons) renders again identical when OFF.

### S8 — Edit `src/app/floating_controls.rs` (append the zoom group)

In `show_button_row`, after the fullscreen button line, insert:

```rust
        fullscreen_button::show_fullscreen_pill_button(app, row_ui);
        zoom_button::show_zoom_pill_buttons(app, row_ui);
```

And extend the `use super::` tuple to add `zoom_button` (the local `use super::fullscreen_button;` pattern). Final pill order: `[←][→] [↑][↓] | [□ fullscreen] | [+] [−] [□-fit restore]`. Update the module doc's first paragraph phrase "…four painted vector-triangle arrow buttons and the fifth fullscreen toggle button" → "…four painted vector-triangle arrow buttons, the fifth fullscreen toggle button, and the three zoom buttons (zoom in, zoom out, restore-to-fit)". Nothing else in the file changes (message line untouched; the zoom click's message set happens the same frame — the message label is already committed that frame, so it renders NEXT frame; same immediate-mode behavior as every other pill message).

### S9 — Tests (EXACTLY these 19; brings the suite 57 → 76)

**In `src/zoom.rs` (inline `#[cfg(test)] mod tests`, 8 tests, all pure, container-runnable). Open the test module with `use super::*;` + `use eframe::egui;` (the `src/scaling.rs` test-module style):**

```rust
    #[test]
    fn zoom_in_step_multiplies_by_the_step_multiplier() {
        assert_eq!(zoom_in_factor(1.0), 1.25);
        assert_eq!(zoom_in_factor(4.0), 5.0); // both values exact in f32
    }

    #[test]
    fn zoom_in_clamps_at_the_maximum() {
        assert_eq!(zoom_in_factor(7.0), ZOOM_FACTOR_MAXIMUM); // 8.75 requested
        assert_eq!(zoom_in_factor(8.0), ZOOM_FACTOR_MAXIMUM);
    }

    #[test]
    fn zoom_out_step_divides_by_the_step_multiplier() {
        assert_eq!(zoom_out_factor(10.0), 8.0);
        assert_eq!(zoom_out_factor(5.0), 4.0);
    }

    #[test]
    fn zoom_out_never_goes_below_fit() {
        assert_eq!(zoom_out_factor(1.0), ZOOM_FACTOR_MINIMUM);
        let below_fit_step = 1.05;
        assert_eq!(zoom_out_factor(below_fit_step), ZOOM_FACTOR_MINIMUM);
    }

    #[test]
    fn zoom_factor_is_at_minimum_is_exact_at_the_fit_bound() {
        assert!(zoom_factor_is_at_minimum(1.0));
        assert!(!zoom_factor_is_at_minimum(1.25));
    }

    #[test]
    fn zoom_factor_is_at_maximum_is_exact_at_the_ceiling() {
        assert!(zoom_factor_is_at_maximum(8.0));
        assert!(!zoom_factor_is_at_maximum(5.0));
    }

    #[test]
    fn zoomed_draw_size_multiplies_the_fit_size_by_the_zoom_factor() {
        let fit_size = egui::Vec2::new(200.0, 100.0);
        assert_eq!(zoomed_draw_size(fit_size, 2.0), egui::Vec2::new(400.0, 200.0));
        let small_fit = egui::Vec2::new(320.0, 240.0);
        assert_eq!(zoomed_draw_size(small_fit, 1.0), small_fit); // fit = no-op zoom
    }

    #[test]
    fn zoom_bounds_and_restore_constant_are_pinned() {
        assert_eq!(ZOOM_STEP_MULTIPLIER, 1.25);
        assert_eq!(ZOOM_FACTOR_MINIMUM, 1.0);
        assert_eq!(ZOOM_FACTOR_MAXIMUM, 8.0);
        assert_eq!(RESTORE_TO_FIT_FACTOR, 1.0);
    }
```

Note on test vectors: pick values that are exact or identically-rounded in binary floating point (1.25 powers, 10/5 fractions) so `assert_eq!` comparisons are exact — the same discipline as the existing `src/scaling.rs` tests.

**In `src/app/zoom_tests.rs` (11 tests; reuse the fixtures exactly like `fullscreen_tests.rs` does: `use super::tests::{TempImageFolder, sample_decoded_image};` — private-field access to `zoom_factor`/`message_text` is fine from a child module). Write the 11 test functions with EXACTLY these names and behaviors (the `t#` below is only a listing marker):

```text
t1  fresh_app_starts_zoomed_to_fit          — empty_state(): zoom_factor == 1.0 (crate::zoom::RESTORE_TO_FIT_FACTOR), zoomed_beyond_fit() == false
t2  zoom_in_once_zooms_beyond_fit           — zoom_in_once(): zoom_factor == 1.25; zoomed_beyond_fit() == true
t3  restore_zoom_to_fit_returns_to_fit      — zoom_in_once(); restore_zoom_to_fit(): zoom_factor == 1.0
t4  zoom_out_at_fit_is_rejected_and_reports_zoom_min — fresh app, zoom_out_once(): zoom_factor stays 1.0; message_text == Some(super::zoom::ZOOM_AT_MINIMUM_MESSAGE)
t5  repeated_zoom_in_reaches_the_ceiling_then_reports_zoom_max — loop `for _ in 0..10` zoom_in_once(); zoom_can_grow() == false; one more zoom_in_once() → message_text == Some(super::zoom::ZOOM_AT_MAXIMUM_MESSAGE) and zoom_factor == 8.0
t6  restore_fit_hotkey_is_num0              — assert_eq!(super::zoom::RESTORE_FIT_KEY, egui::Key::Num0)
t7  fullscreen_toggles_do_not_touch_the_zoom_state — zoom_in_once(); toggle_fullscreen(); toggle_fullscreen(); zoom_factor still 1.25; both chrome-gate states cycled
t8  every_successful_open_resets_the_zoom_to_fit — TempImageFolder with "a.png"/"b.png"; accept_open_outcome(b) ok; zoom_in_once() ×2; accept_open_outcome(a) ok → zoom_factor == 1.0 AND message_text is None
t9  failed_open_keeps_the_zoom_factor       — first accept_open_outcome ok; zoom_in_once(); app.open_image(PathBuf::from("unregistered-in-container.bin")) fails → zoom_factor UNCHANGED (still 1.25), message == UNSUPPORTED_FILE_MESSAGE, decoded_image still Some
t10 zoom_bound_feedback_texts_are_pinned     — assert_eq!(super::zoom::ZOOM_AT_MINIMUM_MESSAGE, "Zoom min"); assert_eq!(super::zoom::ZOOM_AT_MAXIMUM_MESSAGE, "Zoom max")
t11 zoom_button_accessible_labels_are_pinned — assert_eq!(super::zoom_button::ZOOM_IN_BUTTON_LABEL, "Zoom in"); + the out/restore label consts (three assertions in one test)
```

The file's imports: `use super::{ImagesScrollerApp, zoom, zoom_button};` (or spelled-out `super::zoom::...`/`super::zoom_button::...` paths), `use super::tests::{TempImageFolder, sample_decoded_image};`, `use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;`, `use std::path::PathBuf;`, `use eframe::egui;`.

**No changes** to existing test files; no new fixture types (the TempImageFolder/sample fixtures of `src/app/tests.rs` suffice).

### S10 — Register modules + run the harness (§5)

1. Confirm `src/main.rs` gained exactly `mod zoom;` beside the other top-level module declarations (S1) — nothing else registered there.
2. Run the harness of §5; fix fmt via the sanctioned manual `cargo fmt` run if the fmt-check alone fails.
3. Commit per §6.

---

## 4. Expected final unit-test census (57 → 76)

8 pure tests in `src/zoom.rs` + 11 app-state tests in `src/app/zoom_tests.rs` = **19 new**, so `cargo test --locked` must end with **76 passed; 0 failed** (57 pre-existing + 19). The named test functions (S9 lists their bodies/behaviors):

| # | Test fn name | Module |
|---|------|--------|
| zp1 | `zoom_in_step_multiplies_by_the_step_multiplier` | src/zoom.rs |
| zp2 | `zoom_in_clamps_at_the_maximum` | src/zoom.rs |
| zp3 | `zoom_out_step_divides_by_the_step_multiplier` | src/zoom.rs |
| zp4 | `zoom_out_never_goes_below_fit` | src/zoom.rs |
| zp5 | `zoom_factor_is_at_minimum_is_exact_at_the_fit_bound` | src/zoom.rs |
| zp6 | `zoom_factor_is_at_maximum_is_exact_at_the_ceiling` | src/zoom.rs |
| zp7 | `zoomed_draw_size_multiplies_the_fit_size_by_the_zoom_factor` | src/zoom.rs |
| zp8 | `zoom_bounds_and_restore_constant_are_pinned` | src/zoom.rs |
| zt1 | `fresh_app_starts_zoomed_to_fit` | src/app/zoom_tests.rs |
| zt2 | `zoom_in_once_zooms_beyond_fit` | src/app/zoom_tests.rs |
| zt3 | `restore_zoom_to_fit_returns_to_fit` | src/app/zoom_tests.rs |
| zt4 | `zoom_out_at_fit_is_rejected_and_reports_zoom_min` | src/app/zoom_tests.rs |
| zt5 | `repeated_zoom_in_reaches_the_ceiling_then_reports_zoom_max` | src/app/zoom_tests.rs |
| zt6 | `restore_fit_hotkey_is_num0` | src/app/zoom_tests.rs |
| zt7 | `fullscreen_toggles_do_not_touch_the_zoom_state` | src/app/zoom_tests.rs |
| zt8 | `every_successful_open_resets_the_zoom_to_fit` | src/app/zoom_tests.rs |
| zt9 | `failed_open_keeps_the_zoom_factor` | src/app/zoom_tests.rs |
| zt10 | `zoom_bound_feedback_texts_are_pinned` | src/app/zoom_tests.rs |
| zt11 | `zoom_button_accessible_labels_are_pinned` | src/app/zoom_tests.rs |

Key expectations per row (behavioral, all at S9): zp1 1.0→1.25 / 4.0→5.0; zp2 7.0→8.0 / 8.0→8.0; zp3 10.0→8.0 / 5.0→4.0; zp4 1.0→1.0 / 1.05→1.0; zp5–zp6 predicate exact on/off; zp7 200×100@2.0→400×200 and 1.0 = no-op; zp8 pins 1.25/1.0/8.0/restore-1.0; zt1 fresh = fit; zt2 one step = 1.25; zt3 restore = 1.0; zt4 rejected step keeps factor + "Zoom min"; zt5 10 steps reach 8.0 and the 11th reports "Zoom max"; zt6 `Key::Num0` pinned; zt7 factor survives two fullscreen toggles; zt8 re-open resets to fit and clears the message; zt9 open failure keeps factor and reports the standard unsupported message; zt10–zt11 pin the message/label literals.

---

## 5. Harness commands (Alpine VM; exit code is the only success evidence)

1. `vm_status` first (call the `alpine-vm_vm_status` MCP tool; only proceed when running + SSH-reachable).
2. Dev checks through docker compose (`docker` is an allowlisted MCP prefix):

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
```

Expected final line: `ALL CHECKS PASSED` (exit 0). Expected intermediate lines: PASS fmt-check, PASS check, PASS test (**76 passed; 0 failed**), PASS clippy.

3. If ONLY fmt-check fails: run `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` (the sanctioned manual fix — the harness never auto-fixes), then repeat step 2.

4. NO `cargo build`/`cargo clippy --fix`/dependency commands; NO release cross-build, NO `dist/` assembly in this step (the Windows exe rebuild is Wave-4 Task 4's step).

---

## 6. Git actions (Critical Workflow 4.2 hygiene)

1. BEFORE staging: read `.gitignore` (verified: `dist/`, `target/`, `logs/` ignored — nothing else new matches) and run `git status`.
2. Stage EXACTLY these 10 files (nothing else — no `.agent/`, no `.kilo/`, no `dist/`, no `logs/` files):

```
src/zoom.rs
src/app/zoom.rs
src/app/zoom_button.rs
src/app/keyboard_intake.rs
src/app/zoom_tests.rs
src/app.rs
src/app/floating_controls.rs
src/app_display.rs
src/app/fullscreen.rs
src/main.rs
```

3. Single commit, made after dev-checks are green:

```
feat: zoom in out via i o shift keys and pill buttons
```

4. Do NOT push (push is workflow-Step-5-only, and only `origin`). Do NOT create branches (already on `feat/wave-4-fullscreen-zoom`). Do NOT touch `CHANGELOG.md`, `.agent/todos/*` (the `[DONE]` mark is step 4.6, not 4.2), or `.agent/project-info/*`.

---

## 7. Conformance checklist for code review (step 4.3)

Map each item below to the spec §12 numbering; every line must pass before 4.3 signs off:

- [ ] `ImagesScrollerApp` contains EXACTLY ONE new zoom field: `zoom_factor: f32` (spec §12.1) — no pan offset, no zoom mode enum this wave.
- [ ] Field initializes to 1.0 via `crate::zoom::RESTORE_TO_FIT_FACTOR`; no disk/persistence touch (spec §12.2).
- [ ] Reset-on-open: the reset line lives ONLY in `accept_open_outcome`'s `Ok` arm; NO reset on navigate-into-empty-folder, NO reset on fullscreen toggle (D1; spec §4's option A verbatim).
- [ ] `zoom_in_once`/`zoom_out_once` are the single shared action seams for BOTH keyboard and UI triggers (spec §12.4) — no second accumulative implementation exists anywhere.
- [ ] Bounds enforced at the pure seam: 1.0 min, 8.0 max; the ×1.25 multiplicative step is at the UI-keyboard/button envelope (via `zoom_in_once`, spec §12.5).
- [ ] Enablement: zoom-in off EXACTLY at 8.0; zoom-out and restore off EXACTLY at 1.0 (D3; spec §12.6 enablement equivalent).
- [ ] Feedback: `"Zoom max"`/`"Zoom min"` ONLY as a rejected step's message through `message_text` (D2; spec §12.6); NO other message writes, NO display-precedence machinery.
- [ ] Chords: `I`, Shift+'+', `O`, Shift+'-', `0` all through `InputState::key_pressed` reads collected BEFORE the layout dispatch (spec §12.7); the fullscreen/arrows read order in `read_and_dispatch_keystrokes` matches today's behavior; NO collision with them and no arrow/F11 key touched.
- [ ] Zoom controls are INSIDE the pill's render tree (spec §12.8) — hidden during fullscreen together with the pill by the `full_ui_chrome_is_visible()` gate; fullscreen state untouched by zoom actions (D9).
- [ ] Display size per frame = `scaling::fitted_size(...) * zoom_factor` via `zoom::zoomed_draw_size`, overflow clipped by the panel's own clip rect (spec §12.10; §1.2 item 6 proof).
- [ ] No per-frame texture upload (spec §12.11) — only the display size changes inside `paint`; the `ensure_current_image_texture` pending-upload guard is UNTOUCHED.
- [ ] Project rules: files ≤200 lines; methods ≤50; ≤2 args (ImagePainting param object in force); depth ≤2; NO commented-out code; single-section boolean conditions; self-documenting names; all code inside `src/`.

---

## 8. Docs touchpoints (owned by later steps — record only, do NOT edit here)

- Task 4 (verification): dev-check evidence + the 76-test census; OPERATOR-RUN checklist items for zoom (`I`/`0`/shift chords + the three pill buttons; pill clamp feedback; the zoomed image stays centered with its overflow clipped). NOT part of 4.2.
- Task 5 (documentation): update `README.md` (controls section gains `I`, Shift+=, `O`, Shift+-, `0` + the pill button descriptions), `.agent/project-info/tech.md` (a zoom-mechanism section), `.agent/project-info/architecture.md` (mark manual zoom landed inside the Pan/Zoom Controller component), `.agent/project-info/context.md` (Current Work Focus / Recent Changes), and `.agent/project-structure.md` (the file map gains `src/zoom.rs` and the four new `src/app/*` modules; the `floating_controls.rs` line notes the zoom button group). NO `CHANGELOG.md`.
- Also Task-5-owned: the wave-level session context entries (branch state, commit hashes).

---

## 9. Explicitly out of scope (do NOT implement)

- Fit width / fit height modes, any `FitMode` enum or `scaling.rs` change (Task 3; the ONLY zero-cost hook this plan leaves is `zoomed_draw_size` composing with `scaling::fitted_size`).
- Pan, scrollbars, drag-to-pan, edge-then-navigate (Wave 5).
- Persistence of the zoom state (never — session-only).
- ctrl-scroll / pinch (`InputState::zoom_delta`) continuous zoom (parked as future possible).
- Release exe rebuild, DLL import audit (Task 4), version bump (already 0.4.0), `CHANGELOG.md`, any `.md` edit.
- Any `.kilo/plans/*` file edit (ALL are read-only for the implementer).

---

## 10. Success criteria matching this plan to TODO Task 2

- [ ] Zoom in / zoom out work via keyboard (`I`, Shift+Plus, `O`, Shift+Minus) AND via the three pill buttons (+/−/restore-fit) with the approved placement, D3 enablement, D2 feedback.
- [ ] Restore-to-fit resets zoom to 1.0 through BOTH the `0` hotkey and its pill button.
- [ ] ×1.25 multiplicative step, [1.0, 8.0] clamp envelope, session-only state (no persistence), reset-on-every-successful-open.
- [ ] No regressions: arrows, F11, pill layout, message line, texture discipline, all pre-existing 57 tests still green.
- [ ] Dev checks in the container end `ALL CHECKS PASSED` with 76 unit tests.
- [ ] The 10-file diff (§6.2) is committed as a single `feat:` commit on `feat/wave-4-fullscreen-zoom` (no push, no branch work).

(End of plan.)
