# Wave 4 Task 3 — Fit Width / Fit Height Modes: Implementation Plan (architector 4.1b)

Date: 2026-11-02 · Source TODO: `.agent/todos/20261029/20261029-todo-6.md` (Task 3)
Base: branch `feat/wave-4-fullscreen-zoom` @ `dbdf0c7` (verified: `task: mark wave-4 task2 zoom done`)
Spec: `.kilo/plans/20261102-wave4-task3-fit-modes-frontend-spec.md` (4.1a, binding for UI/API facts)
Nature: implementation plan ONLY — the 4.2 implementer executes it; no code was written here.

---

## §0 Binding decisions (user-approved block, restated verbatim)

* Hotkeys: `W` = fit width, `H` = fit height (`egui::Key::W` / `egui::Key::H` — verified against
  egui 0.37 by the 4.1a spec §3; both are plain unbound letter keys), edge-triggered, same
  per-frame always-intake (arrows/F11/I/O/0 family), NO mode-gating of the intake.
* UI: fit-width + fit-height buttons INSIDE the pill, as a NEW gap-separated group after the
  zoom buttons (10 buttons total; keep painted-shape families, no font glyphs; accessible labels
  `Fit width`/`Fit height`; still only 2 extra buttons total).
* Zoom↔mode interplay (Option A adapted verbatim): single draw composition
  `draw_size = fit_mode_size(mode, available, image_aspect) * zoom_factor`; zoom_in/zoom_out KEEP
  the current mode; the fit-wide/fit-tall buttons SET the mode AND reset factor to 1.0;
  restore-to-fit button AND `0` both SET the mode to `FitVisible` and reset factor 1.0;
  active non-default mode shown in the pill message line (`Fit width` / `Fit height`);
  `FitVisible` silent.
* Reset policy coherence: EVERY successful image open still resets zoom AND returns the mode to
  `FitVisible` (user: "When load a folder, always start with image fit") — one reset point at
  accept_open_outcome Ok arm.
* Session-only, no persistence. Fullscreen: fit buttons are chrome (hidden while ON); W/H hotkeys
  active in every mode; mode state survives fullscreen toggles; empty-folder browsing mode: fit
  buttons DISABLED (no image — same acceptance stance as zoom buttons there; verified what Task 2
  landed — see §1d — and kept consistent per that verification) ; fresh empty state: pill absent →
  buttons absent.

### §0a Adjudications encoded by this plan (deviations the reviewer must see)

1. **`fit_mode_size(available, image, mode)` is a 3-arg call in the spec** — the project rule
   allows ≤2 non-self args (param objects allowed). The spec §4 explicitly permits "the equivalent
   `fit_width_size`/`fit_height_size` split". ENCODED: the split + a `FitMode::base_size` METHOD
   (`self` + 2 args) as the single dispatch point. 4.5a checklist item 3 crosswalks:
   `FitVisible ⇔ scaling::fitted_size` (existing), `FitWidth ⇔ scaling::fit_width_size`,
   `FitHeight ⇔ scaling::fit_height_size`.
2. **Feedback timing**: spec §7 F1 says "show the label while that mode is active", but F1's own
   precedence tuple (`unsupported file > zoom bound > fit mode`) plus the single `message_text`
   channel make per-frame relabeling impossible without clobbering failure/zoom messages (which
   breaks the Task-2 standing-message policy). ENCODED (per §0: "same as Task 2 zoom feedback:
   set at activation"): the label is assigned ONCE at activation and stands under the existing
   clear policy (cleared by the next successful open, overwritten by the next event). Precedence
   is by event chronology; the fit dispatches sit AFTER the zoom dispatches in the intake chain so
   a same-frame fit activation overwrites a same-frame `Zoom min`/`Zoom max` message
   (the §0-mandated "activation message overwrites" precedence — this intentionally departs from
   spec F1's `zoom bound > fit mode` ordering).
3. **Fit-button enablement in empty-folder browsing mode**: §0 binds "fit buttons DISABLED (no
   image)". VERIFIED HEAD: the Task-2 zoom buttons are gated ONLY by zoom-bound factors
   (`zoom_can_grow` / `zoomed_beyond_fit`), with NO open-image gate. Therefore the fit buttons
   introduce ONE new explicit predicate, `image_is_open`, encoded once and reused; zoom buttons
   stay untouched. Flagged here so 4.5a consistency review judges the *stated* stance, not an
   invented one.
4. **Spec §11's "pressing `W` dispatches set_fit_mode(FitWidth)" test**: frame-based key
   simulation is a GUI-bound test (no precedent in Tasks 1–2; fullscreen/zoom tests only pin the
   `egui::Key` members, e.g. `restore_fit_hotkey_is_num0`). ENCODED the same precedent: pin tests
   assert the `Key::W`/`Key::H` constants and the seams they drive; NO OS/frame-bound intake tests
   (also the §0 note "No OS-bound tests").

---

## §1 Verified landed state at HEAD `dbdf0c7` (line census, seams)

Re-verified by direct read this session (counts from HEAD, NOT the caller's estimates where they differ):

| File | Lines | Tests at HEAD |
|---|---|---|
| `src/app.rs` | **198** (essentially AT CAP) | — |
| `src/app/floating_controls.rs` | **179** | — |
| `src/app/keyboard_intake.rs` | **59** | — |
| `src/app/zoom_button.rs` | **189** | — |
| `src/app/zoom.rs` (app seam) | **118** | — |
| `src/zoom.rs` (pure math) | **112** (caller said ~118 — corrected) | 8 |
| `src/app/fullscreen.rs` | **54** | — |
| `src/app/fullscreen_button.rs` | **47** | — |
| `src/app_display.rs` | **120** | 2 |
| `src/scaling.rs` | **95** (not in caller list — added) | 5 |
| `src/app/arrow_direction.rs` | **66** | — |
| `\u200b`Total `#[test]` census | — | **76** (grep-verified) |

Other verified seams:
- `ImagesScrollerApp` fields (app.rs): `fullscreen_on: bool`, `zoom_factor: f32`; init at
  `empty_state()` lines 47–60; reset line `self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;`
  at `accept_open_outcome` Ok arm (line 85).
- `ImagePainting { decoded_image, zoom_factor }` constructed at TWO sites: `app.rs`
  `show_navigation_layout` (lines 109–116) and `app/fullscreen.rs` `show_fullscreen_layout`
  (lines 36–43).
- Intake: `keyboard_intake.rs::read_and_dispatch_keystrokes` reads (in order) F11,
  `zoom::zoom_in_chord_was_pressed`, `zoom::zoom_out_chord_was_pressed`, `zoom::RESTORE_FIT_KEY`
  (`Key::Num0`), then the four arrows; dispatch order mirrors the read order. No mode gating
  anywhere. `zoom::RESTORE_FIT_KEY` is `pub(super)` — the pin pattern to copy.
- Pill row (`floating_controls::show_button_row`): arrows → 16.0 gap → fullscreen button →
  `zoom_button::show_zoom_pill_buttons` (which itself adds `ZOOM_GROUP_GAP = 16.0`).
- `arrow_direction.rs`: `pub(super) enum ArrowDirection { Up, Down, Left, Right }` with
  `pub(super) fn paint_rotation(self) -> f32` (0.0 = down, TAU/2 = up, TAU/4 = left,
  −TAU/4 = right) and `egui::Shape::rotated_triangle(rect, rotation, color)` — REUSE this for the
  fit glyphs (same file-family as the arrows).
- `empty_folder_view::show_empty_folder_message` is `pub(super)` — reusable from a NEW app child
  module.
- `CurrentImageDisplay::paint(panel_ui, ImagePainting)` keeps 2 args; it alone computes
  `scaling::fitted_size → zoom::zoomed_draw_size`.
- `zoom_button.rs` enablement: ONLY `zoom_can_grow()` / `zoomed_beyond_fit()` (factor predicates,
  no image gate) — the §1d verification backing adjudication 3.
- `restore_zoom_to_fit` (app seam) currently only resets the factor; `zoom_button`'s restore
  button calls exactly this seam (no other touchpoint).
- Message channel: single `message_text: Option<&'static str>`; zoom assigns
  `Zoom min`/`Zoom max` at rejected steps; cleared to `None` only at the Ok arm.

### §1d Line-budget consequence (mandated extraction)

`app.rs` needs ≥5 added lines (field + doc, init, 2 ImagePainting-literal growths) →
over the 200 cap if appended. Task-2 precedent (keyboard_intake extraction) applies. The TWO
paint sites shall be extracted into ONE new app child module that owns the ZERO-duplicated
`ImagePainting` literal (`src/app/central_surface.rs`), keeping app.rs at ~195 and single-sourcing
the display composition. This is the ONE new structural module of Task 3 (besides the button
sibling prepared for the glyph suite, §2.6).

---

## §2 Design decisions (all structural choices pre-made)

1. **State**: exactly one new app field — `fit_mode: crate::scaling::FitMode`.
   - Enum home: `src/scaling.rs` EXTENSION (justification vs caps: scaling.rs = 95 lines; with the
     enum + 2 pure fns + the `base_size` method it lands ≈ 144, huge headroom; the `FitVisible`
     variant reuses the PRIVATE `fitted_size`/`is_positive_size` internals without widening
     visibility — a separate `src/fit_mode.rs` sibling would have to re-expose them). The Task-2
     precedent (`src/zoom.rs` as a math sibling) is NOT copied because co-location with
     `fitted_size` is exactly what the Whitelist of reuse needs.
   - App seam home: `src/app/zoom.rs` EXTENSION (118 → ≈160). Justification: the mode functions are
     defined BY their zoom interplay (Option A), share `RESTORE_TO_FIT_FACTOR`, and the
     restore-and-open-reset seams must be rewired in the same file; a separate `src/app/fit_mode.rs`
     would duplicate the reset policy. No new seam module.
2. **Pure math** (`src/scaling.rs`):
   - `pub(crate) enum FitMode { FitVisible, FitWidth, FitHeight }` (`Clone, Copy, Debug, PartialEq`).
   - `pub(crate) fn fit_width_size(available_size, image_size) -> Option<egui::Vec2>` —
     guards identical to `fitted_size`, then `Some(image_size * (available_size.x / image_size.x))`.
   - `pub(crate) fn fit_height_size(...)` — axis-y twin: `image_size * (available_size.y / image_size.y)`.
   - `impl FitMode { pub(crate) fn base_size(self, available_size, image_size) -> Option<egui::Vec2> }`
     dispatching FitVisible→`fitted_size`, FitWidth→`fit_width_size`, FitHeight→`fit_height_size`.
   - Composition: `display_size = fit_mode::base_size(available, image) → zoom::zoomed_draw_size(fit, factor)`.
     `zoom::zoomed_draw_size` is UNCHANGED (one composition helper; both paint sites reach it
     through the single `CurrentImageDisplay::paint`).
3. **Intake** (`src/app/keyboard_intake.rs`): 2 reads (`zoom::FIT_WIDTH_KEY` / `zoom::FIT_HEIGHT_KEY`
   via the module-local `was_pressed`) + 2 dispatch ifs calling `set_fit_mode(..)`, placed after the
   restore-fit read/dispatch and BEFORE the arrow reads/dispatches. All modes (fullscreen included).
   No gating, no condition on `image_is_open`.
4. **Mode-set-and-reset seam** (`src/app/zoom.rs`): `set_fit_mode(FitMode)` = set mode +
   `zoom_factor = RESTORE_TO_FIT_FACTOR` + announce non-default modes once (`Fit width`/`Fit height`),
   `FitVisible` silent. Used by: both hotkeys (intake), both buttons (click), the restore-fit
   button + `0` (via `restore_zoom_to_fit` → `set_fit_mode(FitVisible)`), and the open-success
   reset (Ok arm). Non-default freq: none.
5. **Buttons** (`src/app/fit_mode_button.rs`, NEW sibling — zoom_button.rs at 189 cannot absorb a
   second glyph suite): mirrors the zoom_button structure exactly (spec/name/conventions).
   - Enablement: `image_is_open()` — rendered in empty-folder mode DISABLED, enabled with an image
     open, absent with the fresh empty state (pill gate, unchanged, no edit).
   - Glyphs (painted shapes, no font glyphs; spec §7 design adopted):
     - `Fit width` [W]: ONE horizontal filled bar centered (12.0 × 2.5) + TWO painted triangles,
       `ArrowDirection::Left` at `center − (7.5, 0)` and `ArrowDirection::Right` at
       `center + (7.5, 0)`, triangle size `(6.0, 6.0)`; rotations from `arrow_direction::ArrowDirection::paint_rotation()`.
     - `Fit height` [H]: ONE vertical filled bar (2.5 × 12.0) + `ArrowDirection::Up` at
       `center − (0, 7.5)` and `ArrowDirection::Down` at `center + (0, 7.5)`; same triangle size.
     - Palette + pointer cursor + `widget_info` — byte-for-byte the `zoom_button.rs` pattern
       (`noninteractive` disabled, `hovered` hovered, `inactive` idle).
6. **Pill wiring** (`src/app/floating_controls.rs`): after `zoom_button::show_zoom_pill_buttons`
   add the fit group (its own `show_fit_mode_pill_buttons` adds the `16.0` gap) → row =
   `[←][→] | [↑][↓] | [□] | [+][−][fit □] | [W][H]` = 10 buttons.
7. **Feedback**: single channel; the label is the standing `message_text` under adjudication 2
   (§0a). Fit constants: `FIT_WIDTH_MESSAGE = "Fit width"`, `FIT_HEIGHT_MESSAGE = "Fit height"`.
8. **Display** (`src/app_display.rs`): `ImagePainting` gains `fit_mode: crate::scaling::FitMode`
   (3-field shape per spec §9 recommendation — `paint` stays 2-arg). `paint` computes
   `painting.fit_mode.base_size(panel_ui.available_size(), image_dimensions(...))` then
   `zoom::zoomed_draw_size`. Both current paint sites route through this single fn via the new
   `central_surface` extraction. No texture change (one upload per decode stands; resizing
   recomputes per frame).
9. **Fullscreen**: NO dispatch change (chrome gate already hides the pill); the fullscreen paint
   delegates to the shared central-surface seam; `fit_mode`/`zoom_factor` untouched by toggles.
10. **Tests**: pure logic only — math in a NEW `src/scaling/fit_mode_tests.rs` child module
    (deferred from inline growth so `scaling.rs` cannot hit the cap), app policy in a NEW
    `src/app/fit_mode_tests.rs`. Expected suite: **76 → 93 (i.e., 93 = 76 + 6 + 11)**.

---

## §3 Ordered implementation steps (execute 3.1 → 3.10 exactly)

Step numbering is execution order. New FILES first where an edit declares them, to keep
`cargo check` green at each fold (the container check runs only at the end; ordering still
minimizes drift).

### 3.1 NEW FILE `src/app/central_surface.rs` (~38 lines)

Purpose: the single owner of the `ImagePainting` literal; the two central-surface bodies shared by
chrome and fullscreen. Extracted so `app.rs` can hold the fit field.

```rust
//! The central-surface bodies shared by the chrome and fullscreen layouts:
//! the ONE ImagePainting literal of the application (which decoded image at
//! WHICH session fit mode and zoom factor paints) and the empty-folder
//! fallback of the chrome navigation layout.

use eframe::egui;

use super::{ImagesScrollerApp, empty_folder_view};
use crate::app_display::ImagePainting;

impl ImagesScrollerApp {
    /// The chrome navigation surface: the image paint when open, otherwise the
    /// centered empty-folder message.
    pub(super) fn show_navigation_central_surface(&mut self, central_ui: &mut egui::Ui) {
        if !self.image_is_open() {
            empty_folder_view::show_empty_folder_message(central_ui);
            return;
        }
        self.paint_open_image(central_ui);
    }

    /// The fullscreen surface: ONLY the image paint while open (no fallback).
    pub(super) fn show_central_image_while_open(&mut self, central_ui: &mut egui::Ui) {
        if self.image_is_open() {
            self.paint_open_image(central_ui);
        }
    }

    /// The single paint request: the decoded image at the session fit mode's
    /// base size, scaled by the zoom factor (inside `CurrentImageDisplay`).
    fn paint_open_image(&mut self, central_ui: &mut egui::Ui) {
        let Some(decoded_image) = self.decoded_image.as_ref() else {
            return;
        };
        self.current_display.paint(
            central_ui,
            ImagePainting {
                decoded_image,
                zoom_factor: self.zoom_factor,
                fit_mode: self.fit_mode,
            },
        );
    }
}
```

Notes: disjoint-field borrows through `self` compile exactly as at app.rs today;
`image_is_open` becomes `pub(super)` in 3.2; `ImagePainting.fit_mode` field is added in 3.4 —
declare and keep this file's `fit_mode: self.fit_mode,` untouched through later steps.

### 3.2 EDIT `src/app.rs`

1. Struct field — add after `zoom_factor: f32,` (line 37):
   ```rust
       /// Session fit mode (Wave-4 Task 3): `FitVisible` = whole-image fit;
       /// set by the fit seam; reset on every successful open; never persisted.
       fit_mode: crate::scaling::FitMode,
   ```
2. `empty_state()` — add `fit_mode: crate::scaling::FitMode::FitVisible,` as the last field of the
   literal (after the `zoom_factor` line).
3. `accept_open_outcome` Ok arm — REPLACE the line
   `self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;` (line 85) with the ONE seam call:
   ```rust
                   self.set_fit_mode(crate::scaling::FitMode::FitVisible);
   ```
   (This is the single reset point: mode + factor in one call; announcement is silent for
   `FitVisible`; the preceding `self.message_text = None;` line stays.)
4. `image_is_open` (line 101) — visibility `fn` → `pub(super) fn` (same line, no body change).
5. `show_navigation_layout` (lines 105–121) — REPLACE the whole CentralPanel block with:
   ```rust
       /// Pill layout: the floating pill plus the central surface -- the image paint when one is open, else the empty-folder message.
       fn show_navigation_layout(&mut self, ui: &mut egui::Ui) {
           floating_controls::show_floating_pill(self, ui);
           egui::CentralPanel::default().show(ui, |central_ui| {
               self.show_navigation_central_surface(central_ui);
           });
       }
   ```
   (doc line above unchanged in wording; the extraction removes ~10 lines).
6. Import trim — line 7 `use crate::app_display::{CurrentImageDisplay, ImagePainting};` becomes
   `use crate::app_display::CurrentImageDisplay;` (ImagePainting moved out with the extraction).
7. Module declarations — add beside the existing child mods (alphabetical-neutral placement, after
   `mod arrow_direction;` and before `mod folder_navigation;`, mirroring current one-per-line style):
   ```rust
   mod central_surface;
   ```
   and (near the test mods section, grouped with the other `#[cfg(test)]` mods):
   ```rust
   mod fit_mode_button;
   ...
   #[cfg(test)]
   mod fit_mode_tests;
   ```
   Final tally: app.rs = 198 +3 (field) +1 (init) −10 (extraction) +3 (mods) = **195** ✓.

### 3.3 EDIT `src/fullscreen…` = `src/app/fullscreen.rs`

1. `show_fullscreen_layout` — REPLACE the closure body (the `if let Some(decoded_image) …`
   block) with UP-CASED single call:
   ```rust
           egui::CentralPanel::default().show(ui, |central_ui| {
               self.show_central_image_while_open(central_ui);
           });
   ```
2. DELETE the now-unused `use crate::app_display::ImagePainting;` line.
   Result ≈ 43 lines.

### 3.4 EDIT `src/app_display.rs`

1. `ImagePainting` (lines 12–17) becomes EXACTLY:
   ```rust
   /// The 2-argument paint request (project max-args rule): WHICH decoded image
   /// is on screen, at WHAT session fit mode its available-area base size is
   /// taken from, and WHAT zoom factor scales that base into the draw size.
   pub(crate) struct ImagePainting<'a> {
       pub(crate) decoded_image: &'a DecodedImage,
       pub(crate) fit_mode: crate::scaling::FitMode,
       pub(crate) zoom_factor: f32,
   }
   ```
   (field order: mode between image and factor; field-order is presentation only but FIX it as listed).
2. In `paint`, REPLACE the `let Some(fit_size) = scaling::fitted_size(...) else { return; };` block
   with the mode-parameterized base:
   ```rust
           let Some(fit_size) = painting.fit_mode.base_size(
               panel_ui.available_size(),
               image_dimensions(painting.decoded_image),
           ) else {
               return;
           };
   ```
   `let display_size = zoom::zoomed_draw_size(fit_size, painting.zoom_factor);` and everything below
   stay byte-identical. `use crate::scaling;` already present.
3. Module doc line 1 ("fit-scaled paint") unchanged; helper doc line 82 wording stays
   ("shared by the one `fitted_size` call") — REPLACE with
   "Image dimensions as `egui::Vec2`, shared by the one fit-base call in `paint`."
   Result ≈ 121-122 lines.

### 3.5 EDIT `src/scaling.rs` + NEW FILE `src/scaling/fit_mode_tests.rs`

Extend `src/scaling.rs` AFTER `fitted_size` (before the `#[cfg(test)] mod tests` block). Add
(≈49 lines with docs) EXACTLY:

```rust
/// The three session fit modes of the viewer (Wave-4 Task 3). `FitVisible` is
/// the whole-image fit carried since Wave 1; `FitWidth`/`FitHeight` are the
/// Windows-user expressions of "fit width"/"fit height". Session-only means:
/// no persistence lives here (the app seam owns the reset policy).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FitMode {
    /// Whole-image fit: fully visible and centered.
    FitVisible,
    /// Width fit: the width spans the available width; vertical overflow allowed.
    FitWidth,
    /// Height fit: the height spans the available height; horizontal overflow allowed.
    FitHeight,
}

impl FitMode {
    /// The mode's fit-base size of `image_size` inside `available_size`.
    /// Returns None on any zero/negative dimension (the `fitted_size` guards,
    /// shared by every variant).
    pub(crate) fn base_size(
        self,
        available_size: egui::Vec2,
        image_size: egui::Vec2,
    ) -> Option<egui::Vec2> {
        match self {
            Self::FitVisible => fitted_size(available_size, image_size),
            Self::FitWidth => fit_width_size(available_size, image_size),
            Self::FitHeight => fit_height_size(available_size, image_size),
        }
    }
}

/// The width-fit draw size: the image WIDTH spans the available width; the
/// height follows the aspect ratio and may overflow vertically. Same
/// zero/negative-dimension rejection style as `fitted_size`.
pub(crate) fn fit_width_size(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<egui::Vec2> {
    if !is_positive_size(image_size) {
        return None;
    }
    if !is_positive_size(available_size) {
        return None;
    }
    Some(image_size * (available_size.x / image_size.x))
}

/// The height-fit twin of `fit_width_size`: the image HEIGHT spans the
/// available height; the width may overflow horizontally.
pub(crate) fn fit_height_size(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<egui::Vec2> {
    if !is_positive_size(image_size) {
        return None;
    }
    if !is_positive_size(available_size) {
        return None;
    }
    Some(image_size * (available_size.y / image_size.y))
}
```

Append to the AUTO-EXPLICIT test block placement — at `src/scaling.rs` file END add only:
```rust
#[cfg(test)]
mod fit_mode_tests;
```
(all 6 new tests live in the child file — the existing inline 5 tests are untouched).
`src/scaling.rs` → ≈ 145 lines ✓; `src/scaling/fit_mode_tests.rs` NEW file (§4a).

### 3.6 EDIT `src/app/zoom.rs`

1. Header doc — after the current first paragraph line (the "Manual zoom (Wave-4 Task 2)…" opening),
   ADD the sentence block:
   ```
   //! Fit modes (Wave-4 Task 3) share this seam: `set_fit_mode` is the ONE
   //! mode-set-and-reset call (hotkeys `W`/`H`, both pill buttons, the restore
   //! path, and the open-success reset all funnel through it).
   ```
2. After `pub(super) const RESTORE_FIT_KEY…` add:
   ```rust
   /// Fit-width hotkey `W` (Wave-4 Task 3, user-approved K1 binding).
   pub(super) const FIT_WIDTH_KEY: egui::Key = egui::Key::W;
   /// Fit-height hotkey `H` (the user-approved twin of `W`).
   pub(super) const FIT_HEIGHT_KEY: egui::Key = egui::Key::H;

   /// Pill label announced once, at fit-width activation (singular channel).
   pub(super) const FIT_WIDTH_MESSAGE: &str = "Fit width";
   /// Pill label announced once, at fit-height activation.
   pub(super) const FIT_HEIGHT_MESSAGE: &str = "Fit height";
   ```
3. Inside the `impl ImagesScrollerApp` block add:
   ```rust
       /// Sets the session fit mode and anchors the zoom factor to that mode's
       /// 1× base (Option A interplay). Non-default modes announce themselves
       /// once, at activation; `FitVisible` is silent (it is also the restore /
       /// open-success reset target).
       pub(super) fn set_fit_mode(&mut self, fit_mode: crate::scaling::FitMode) {
           self.fit_mode = fit_mode;
           self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;
           self.announce_fit_mode(fit_mode);
       }

       fn announce_fit_mode(&mut self, fit_mode: crate::scaling::FitMode) {
           if let Some(mode_label) = fit_mode_activation_label(fit_mode) {
               self.message_text = Some(mode_label);
           }
       }
   ```
4. REPLACE `restore_zoom_to_fit`'s body with the one-seam call (keeps the pub(super) signature):
   ```rust
       /// Hotkey `0` and the restore button: back to whole-image fit (Wave-4
       /// Task 3: the mode returns to `FitVisible` too). Silent: the fit state
       /// is its own feedback.
       pub(super) fn restore_zoom_to_fit(&mut self) {
           self.set_fit_mode(crate::scaling::FitMode::FitVisible);
       }
   ```
   (The old doc line "Silent by design everywhere…" wording is REPLACED by the above doc; zoom-IN
   / zoom-OUT methods untouched.)
5. File-end free function (outside the impl), the ONLY message mapper:
   ```rust
   /// The once-at-activation pill label of a fit mode; the default `FitVisible`
   /// is silent by design.
   fn fit_mode_activation_label(fit_mode: crate::scaling::FitMode) -> Option<&'static str> {
       match fit_mode {
           crate::scaling::FitMode::FitVisible => None,
           crate::scaling::FitMode::FitWidth => Some(FIT_WIDTH_MESSAGE),
           crate::scaling::FitMode::FitHeight => Some(FIT_HEIGHT_MESSAGE),
       }
   }
   ```
   Expected app/zoom.rs ≈ 160 lines ✓.

### 3.7 EDIT `src/app/keyboard_intake.rs` (59 → ≈ 68)

1. Header doc — in the module doc's chord list (line ~2: "the fullscreen toggle, the four
   navigation arrows, the zoom in/out/restore chords"), INSERT "the fit-mode `W`/`H` cards" phase:
   the doc becomes: "the fullscreen toggle, the four navigation arrows, the zoom in/out/restore
   chords, and the fit-mode `W`/`H` keys".
2. In `read_and_dispatch_keystrokes`, AFTER `let restore_fit_pressed = …`, ADD:
   ```rust
           let fit_width_pressed = was_pressed(screen_context, zoom::FIT_WIDTH_KEY);
           let fit_height_pressed = was_pressed(screen_context, zoom::FIT_HEIGHT_KEY);
   ```
3. AFTER the `if restore_fit_pressed { self.restore_zoom_to_fit(); }` block ADD (BEFORE the arrows):
   ```rust
           if fit_width_pressed {
               self.set_fit_mode(crate::scaling::FitMode::FitWidth);
           }
           if fit_height_pressed {
               self.set_fit_mode(crate::scaling::FitMode::FitHeight);
           }
   ```
   (Order guarantees the §0a adjudication-2 precedence: a fit activation in the SAME frame as a
   rejected zoom step overwrites the `Zoom min`/`Zoom max` message.)

### 3.8 NEW FILE `src/app/fit_mode_button.rs` (~118 lines; mirror of zoom_button.rs)

```rust
//! The floating pill's fit-mode buttons (Wave-4 Task 3, user-approved INSIDE
//! the pill, after the zoom group): fit-width [W] and fit-height [H]. Painted
//! line-and-triangle glyphs of the SAME vector family as the arrows -- no font
//! glyphs. Enabled exactly while an image is open; rendered DISABLED while an
//! empty folder is browsed (fresh empty state: the pill is absent anyway).

use eframe::egui;

use super::arrow_direction::ArrowDirection;
use super::ImagesScrollerApp;
use crate::scaling::FitMode;

pub(super) const FIT_WIDTH_BUTTON_LABEL: &str = "Fit width";
pub(super) const FIT_HEIGHT_BUTTON_LABEL: &str = "Fit height";

const FIT_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const FIT_GROUP_GAP: f32 = 16.0;
const FIT_AXIS_LINE_LENGTH: f32 = 12.0;
const FIT_AXIS_LINE_THICKNESS: f32 = 2.5;
const FIT_ARROW_TRIANGLE_SIZE: egui::Vec2 = egui::Vec2::new(6.0, 6.0);
const FIT_ARROW_SPACING: f32 = 7.5;

enum FitModeButtonAction {
    FitWidth,
    FitHeight,
}

struct FitModeButtonSpec {
    action: FitModeButtonAction,
    enabled: bool,
}

/// Appends the fit-mode group to the pill row, separated by the standard
/// group gap after the zoom buttons (user-approved placement).
pub(super) fn show_fit_mode_pill_buttons(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    row_ui.add_space(FIT_GROUP_GAP);
    let button_specs = fit_mode_button_specs(app);
    for button_spec in &button_specs {
        button_spec.show_and_apply_action(app, row_ui);
    }
}

fn fit_mode_button_specs(app: &ImagesScrollerApp) -> [FitModeButtonSpec; 2] {
    let image_displayed = app.image_is_open();
    [
        FitModeButtonSpec::row_button(FitModeButtonAction::FitWidth, image_displayed),
        FitModeButtonSpec::row_button(FitModeButtonAction::FitHeight, image_displayed),
    ]
}

impl FitModeButtonSpec {
    fn row_button(action: FitModeButtonAction, enabled: bool) -> Self {
        Self { action, enabled }
    }

    /// Paints the glyph and, when clicked while enabled, sets the fit mode
    /// (the app seam resets the factor and gives the feedback).
    fn show_and_apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let (_, button_response) =
            row_ui.allocate_exact_size(FIT_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, self.accessible_label())
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_glyph(row_ui, &button_response);
        if self.accepts_click(&button_response) {
            self.apply_action(app);
        }
    }

    /// A disabled fit button never fires (the disabled-arrow pattern).
    fn accepts_click(&self, button_response: &egui::Response) -> bool {
        self.enabled && button_response.clicked()
    }

    fn apply_action(&self, app: &mut ImagesScrollerApp) {
        match self.action {
            FitModeButtonAction::FitWidth => app.set_fit_mode(FitMode::FitWidth),
            FitModeButtonAction::FitHeight => app.set_fit_mode(FitMode::FitHeight),
        }
    }

    fn accessible_label(&self) -> &'static str {
        match self.action {
            FitModeButtonAction::FitWidth => FIT_WIDTH_BUTTON_LABEL,
            FitModeButtonAction::FitHeight => FIT_HEIGHT_BUTTON_LABEL,
        }
    }

    /// Default egui button-text palette: idle in `inactive`, brightened on
    /// hover, dimmed in `noninteractive` while disabled (the arrow palette).
    fn fit_glyph_color(&self, row_ui: &egui::Ui, button_response: &egui::Response) -> egui::Color32 {
        if !self.enabled {
            return row_ui.visuals().widgets.noninteractive.fg_stroke.color;
        }
        if button_response.hovered() {
            return row_ui.visuals().widgets.hovered.fg_stroke.color;
        }
        row_ui.visuals().widgets.inactive.fg_stroke.color
    }

    fn paint_glyph(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let glyph_center = button_response.rect.center();
        let glyph_color = self.fit_glyph_color(row_ui, button_response);
        for glyph_shape in self.fit_glyph_shapes(glyph_center, glyph_color) {
            row_ui.painter().add(glyph_shape);
        }
    }

    /// Per-action glyph geometry: one axis line plus the two opposing
    /// triangles of the painted-arrow family (`Shape::rotated_triangle`).
    fn fit_glyph_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        match self.action {
            FitModeButtonAction::FitWidth => {
                fit_width_glyph_shapes(glyph_center, glyph_color)
            }
            FitModeButtonAction::FitHeight => {
                fit_height_glyph_shapes(glyph_center, glyph_color)
            }
        }
    }
}

/// Fit-width [W]: a horizontal bar capped by left/right arrow triangles.
fn fit_width_glyph_shapes(
    glyph_center: egui::Pos2,
    glyph_color: egui::Color32,
) -> Vec<egui::Shape> {
    let axis_line = axis_line_shape(glyph_center, glyph_color, true);
    let left = fit_arrow_triangle_shape(
        glyph_center - egui::vec2(FIT_ARROW_SPACING, 0.0),
        ArrowDirection::Left,
        glyph_color,
    );
    let right = fit_arrow_triangle_shape(
        glyph_center + egui::vec2(FIT_ARROW_SPACING, 0.0),
        ArrowDirection::Right,
        glyph_color,
    );
    vec![axis_line, left, right]
}

/// Fit-height [H]: a vertical bar capped by up/down arrow triangles.
fn fit_height_glyph_shapes(
    glyph_center: egui::Pos2,
    glyph_color: egui::Color32,
) -> Vec<egui::Shape> {
    let axis_line = axis_line_shape(glyph_center, glyph_color, false);
    let up = fit_arrow_triangle_shape(
        glyph_center - egui::vec2(0.0, FIT_ARROW_SPACING),
        ArrowDirection::Up,
        glyph_color,
    );
    let down = fit_arrow_triangle_shape(
        glyph_center + egui::vec2(0.0, FIT_ARROW_SPACING),
        ArrowDirection::Down,
        glyph_color,
    );
    vec![axis_line, up, down]
}

/// The axis line: horizontal when `horizontal`, else vertical.
fn axis_line_shape(glyph_center: egui::Pos2, glyph_color: egui::Color32, horizontal: bool) -> egui::Shape {
    let line_size = if horizontal {
        egui::Vec2::new(FIT_AXIS_LINE_LENGTH, FIT_AXIS_LINE_THICKNESS)
    } else {
        egui::Vec2::new(FIT_AXIS_LINE_THICKNESS, FIT_AXIS_LINE_LENGTH)
    };
    egui::Shape::rect_filled(
        egui::Rect::from_center_size(glyph_center, line_size),
        egui::CornerRadius::ZERO,
        glyph_color,
    )
}

/// One painted triangle at `arrow_center` facing `x`'s rotation (arrow family).
fn fit_arrow_triangle_shape(
    arrow_center: egui::Pos2,
    arrow_facing: ArrowDirection,
    glyph_color: egui::Color32,
) -> egui::Shape {
    let arrow_rect =
        egui::Rect::from_center_size(arrow_center, FIT_ARROW_TRIANGLE_SIZE);
    egui::Shape::rotated_triangle(
        arrow_rect,
        arrow_facing.paint_rotation(),
        glyph_color,
    )
}
```

Verification facts embedded: `ArrowDirection`/`paint_rotation` are `pub(super)` (reads from
sibling child modules — same visibility floating_controls relies on), `Shape::rotated_triangle`
ordering matches floating_controls line 160–164. Method-body lengths ≤ 50 lines ✓ (longest =
`show_and_apply_action` ≈ 14).

### 3.9 EDIT `src/app/floating_controls.rs` (179 → 181-182)

1. Header doc line 1 — the phrase "and the three zoom buttons (zoom in, zoom out, restore-to-fit)"
   becomes "and the three zoom buttons, plus the fit-width/fit-height buttons (Wave-4 Task 3)".
2. Imports — after `use super::fullscreen_button;` ADD:
   ```rust
   use super::fit_mode_button;
   ```
3. `show_button_row` (line 70–81) — AFTER
   `zoom_button::show_zoom_pill_buttons(app, row_ui);` ADD:
   ```rust
           fit_mode_button::show_fit_mode_pill_buttons(app, row_ui);
   ```

### 3.10 NEW FILE `src/app/fit_mode_tests.rs` (~11 policy tests)

Pattern base: `src/app/zoom_tests.rs` imports (`use super::tests::{TempImageFolder, sample_decoded_image};`,
`use super::{ImagesScrollerApp, zoom, fit_mode_button};`). Exact tests (11):

```rust
#[test]
fn fresh_app_starts_in_the_whole_image_fit_mode() {
    let app = ImagesScrollerApp::empty_state();
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);
    assert_eq!(app.zoom_factor, crate::zoom::RESTORE_TO_FIT_FACTOR);
}
```
```rust
#[test]
fn fit_width_activation_resets_the_factor_and_announces_fit_width() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitWidth);
    assert_eq!(app.zoom_factor, 1.0);
    assert_eq!(app.message_text, Some(zoom::FIT_WIDTH_MESSAGE));
}

#[test]
fn fit_height_activation_resets_the_factor_and_announces_fit_height() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitHeight);
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitHeight);
    assert_eq!(app.zoom_factor, 1.0);
    assert_eq!(app.message_text, Some(zoom::FIT_HEIGHT_MESSAGE));
}

#[test]
fn fit_visible_activation_is_silent_and_resets_the_mode_and_factor() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    assert!(app.message_text.is_some());
    app.set_fit_mode(crate::scaling::FitMode::FitVisible);
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
}

#[test]
fn zoom_steps_keep_the_active_fit_mode() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    app.zoom_in_once();
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitWidth);
    assert_eq!(app.zoom_factor, 1.25);
}

#[test]
fn restore_fit_returns_to_the_whole_image_mode_at_the_fit_factor() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitHeight);
    app.zoom_in_once();
    app.restore_zoom_to_fit();
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);
    assert_eq!(app.zoom_factor, 1.0);
}

#[test]
fn fit_activation_message_overwrites_the_zoom_bound_message() {
    let mut app = ImagesScrollerApp::empty_state();
    app.zoom_out_once(); // rejected at the fit floor
    assert_eq!(app.message_text, Some(zoom::ZOOM_AT_MINIMUM_MESSAGE));
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    assert_eq!(app.message_text, Some(zoom::FIT_WIDTH_MESSAGE));
}

#[test]
fn every_successful_open_returns_to_the_whole_image_fit() {
    let folder = TempImageFolder::create("fit_reset_on_open");
    folder.create_file("a.png");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    app.zoom_in_once();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
}

#[test]
fn fit_mode_hotkeys_are_pinned_to_the_w_and_h_keys() {
    assert_eq!(zoom::FIT_WIDTH_KEY, egui::Key::W);
    assert_eq!(zoom::FIT_HEIGHT_KEY, egui::Key::H);
}

#[test]
fn fit_mode_button_labels_are_pinned() {
    assert_eq!(fit_mode_button::FIT_WIDTH_BUTTON_LABEL, "Fit width");
    assert_eq!(fit_mode_button::FIT_HEIGHT_BUTTON_LABEL, "Fit height");
}

#[test]
fn fit_mode_button_enablement_gate_is_the_open_image_state() {
    let mut app = ImagesScrollerApp::empty_state();
    assert!(!app.image_is_open());
    let folder = TempImageFolder::create("fit_button_gate");
    folder.create_file("a.png");
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert!(app.image_is_open());
}
```
(`message_text`/`fit_mode` field reads are legal from a child-of-`app` test module — precedent:
zoom_tests.rs line 41. Test-count total: 76 + 11 = 87 via this file.)

### 3.11 The 6 math tests in `src/scaling/fit_mode_tests.rs`

```rust
use super::{FitMode, fit_height_size, fit_width_size, fitted_size};
use eframe::egui;

/// Exact-in-f32 test vectors only (powers of two divisors), matching the
/// `fitted_size` test conventions.

#[test]
fn fit_width_spans_the_available_width_and_overflows_vertically() {
    let available_size = egui::Vec2::new(400.0, 400.0);
    let image_size = egui::Vec2::new(100.0, 200.0);
    assert_eq!(
        fit_width_size(available_size, image_size),
        Some(egui::Vec2::new(400.0, 800.0))
    );
}

#[test]
fn fit_height_spans_the_available_height_and_overflows_horizontally() {
    let available_size = egui::Vec2::new(400.0, 400.0);
    let image_size = egui::Vec2::new(200.0, 100.0);
    assert_eq!(
        fit_height_size(available_size, image_size),
        Some(egui::Vec2::new(800.0, 400.0))
    );
}

#[test]
fn square_image_width_height_and_visible_fits_agree() {
    let available_size = egui::Vec2::new(400.0, 400.0);
    let image_size = egui::Vec2::new(100.0, 100.0);
    assert_eq!(
        FitMode::FitVisible.base_size(available_size, image_size),
        Some(egui::Vec2::new(400.0, 400.0))
    );
    assert_eq!(
        FitMode::FitWidth.base_size(available_size, image_size),
        Some(egui::Vec2::new(400.0, 400.0))
    );
    assert_eq!(
        FitMode::FitHeight.base_size(available_size, image_size),
        Some(egui::Vec2::new(400.0, 400.0))
    );
}

#[test]
fn fit_mode_base_sizes_difference_is_exact_on_a_wide_image() {
    let available_size = egui::Vec2::new(800.0, 300.0);
    let image_size = egui::Vec2::new(400.0, 100.0);
    assert_eq!(
        FitMode::FitVisible.base_size(available_size, image_size),
        Some(egui::Vec2::new(800.0, 200.0))
    );
    assert_eq!(
        FitMode::FitWidth.base_size(available_size, image_size),
        Some(egui::Vec2::new(800.0, 100.0))
    );
    assert_eq!(
        FitMode::FitHeight.base_size(available_size, image_size),
        Some(egui::Vec2::new(1200.0, 300.0))
    );
}

#[test]
fn fit_visible_base_size_matches_the_whole_image_fitted_size() {
    let available_size = egui::Vec2::new(300.0, 800.0);
    let image_size = egui::Vec2::new(100.0, 400.0);
    assert_eq!(
        FitMode::FitVisible.base_size(available_size, image_size),
        fitted_size(available_size, image_size)
    );
}

#[test]
fn non_positive_dimensions_reject_every_fit_mode() {
    let usable = egui::Vec2::new(100.0, 100.0);
    for fit_mode in [FitMode::FitVisible, FitMode::FitWidth, FitMode::FitHeight] {
        assert_eq!(fit_mode.base_size(usable, egui::Vec2::new(0.0, 10.0)), None);
        assert_eq!(fit_mode.base_size(usable, egui::Vec2::new(-4.0, 10.0)), None);
        assert_eq!(fit_mode.base_size(egui::Vec2::new(0.0, 100.0), usable), None);
        assert_eq!(fit_mode.base_size(egui::Vec2::new(100.0, -1.0), usable), None);
    }
}
```
(6 tests; visibility: the child module path is `super::{FitMode, …}` inside `src/scaling.rs` —
`fit_mode_tests` IS a child of `scaling`, so `fitted_size`/`is_positive_size`-level private items
are visible via `super::`. The `for` loop test violates no depth rule: body depth = 1.)

---

## §4 Unit-test census (expected)

| Subject | File | Count |
|---|---|---|
| Fit-mode math (width/height span, overflow direction, square agreement, inter-mode difference, visible ⇔ fitted, non-positive rejection) | `src/scaling/fit_mode_tests.rs` (NEW) | 6 |
| App policy (init, activation ×2, silent-visible, zoom-keeps-mode, restore, overwrite vs zoom-bound messages, open-reset, key pins, button labels, enablement gate) | `src/app/fit_mode_tests.rs` (NEW) | 11 |
| Existing suites | unchanged | 76 |
| **Suite total expected at 4.2** | | **93** |

NO os/egui-frame tests (§0a adjudication 4). NO tests for button glyph geometry (paint-only).

---

## §5 Harness steps (all Rust/Cargo inside the Alpine-VM container)

1. `vm_status` FIRST (alpine-vm MCP). If the VM or `/img-scroller` share is unavailable → STOP and
   return a user question (never proceed host-side).
2. Dev checks (the ONLY evidence gate; exit code is authoritative):
   ```
   docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
   ```
   Expected: `ALL CHECKS PASSED` (exit 0) with fmt, check, test × 93, clippy `-D warnings` green.
3. If `cargo fmt --check` fails: run once
   `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt`
   then re-run the dev-checks command. If clippy names a concrete lint, apply the smallest fix
   inside the touched files only (no other files), re-run checks.
4. NO release rebuild in 4.1b (the exe rebuild is Task 4's step, NOT this one). Do NOT bump
   Cargo.toml (0.4.0 already committed in Step 3). No DLL audit here.
5. Record the observed final counts (93 tests, `ALL CHECKS PASSED`) in the 4.5a/4.6 evidence trail.

---

## §6 Git steps (4.2-only scope)

1. Read `.gitignore` and run `git status` BEFORE staging (Gitignore Compliance Rule). Expected
   dirty set: the 8 source files of §3 + the 2 new plan/spec markdown files + `context.md`
   (context.md belongs to Task-5 docs flow — leave it UNSTAGED; do not stage `logs/`, `dist/`,
   `target/`, `*.log` — they must not exist in `git status` staged sets; if any appear, unstage).
2. Stage EXACTLY:
   - `src/app.rs`, `src/app_display.rs`, `src/scaling.rs`, `src/app/floating_controls.rs`,
     `src/app/keyboard_intake.rs`, `src/app/zoom.rs`, `src/app/fullscreen.rs`
     (modified),
   - `src/app/central_surface.rs`, `src/app/fit_mode_button.rs`, `src/app/fit_mode_tests.rs`,
     `src/scaling/fit_mode_tests.rs` (new),
   - `.kilo/plans/20261102-wave4-task3-fit-modes-frontend-spec.md`,
     `.kilo/plans/20261102-wave-4-task3-fit-modes-plan.md` (plans/spec, already live at HEAD-side).
3. Commit message (caller-suggested literal):
   ```
   feat: fit width and height modes via w h keys and pill buttons
   ```
   One commit for the Task-3 4.2 slice. Amend/follow-up commit only if the code-review round (4.3)
   creates fixes — those get their own review-fix commits per Workflow.
4. NO push (Step-5 restriction), NO branch change, NO merge, NO `-DONE` rename yet (that is 4.6).

---

## §7 4.5a conformance crosswalk (for the frontend reviewer)

Spec §12 checklist → where it is satisfied by THIS plan:
1. exactly one new field `fit_mode: crate::scaling::FitMode` → §3.2.1;
2. init `FitVisible` + reset on every successful open via ONE seam call at the Ok arm; never
   persisted → §3.2.3 + test `every_successful_open_returns_to_the_whole_image_fit`;
3. tri-state enum + exact sizes → §3.5 (adjudication 1 crosswalk: `base_size` method = the
   spec's `fit_mode_size` equivalent);
4. Option A interplay → §3.6.3/3.6.4 + tests `zoom_steps_keep_the_active_fit_mode`,
   `restore_fit_returns_to_the_whole_image_mode_at_the_fit_factor`;
5. `ImagePainting` carries `fit_mode`, display math = base × factor → §3.4 + §3.1;
6. `W`/`H` edge-triggered, pre-layout, no collision (arrows/F11/I/O/0 untouched; W/H are plain
   letter keys unbound in the app) → §3.7;
7. fit buttons chrome-gated by the pill's existing visibility path (`navigation_context_is_active`
   + `full_ui_chrome_is_visible`) with NO change → §3.9 (pill gate untouched);
8. intake unconditional (fullscreen ON included); toggles don't touch mode/zoom
   (verified pattern: fullscreen seams read nothing of them) → §3.7 + landed fullscreen.rs;
9. active-mode feedback = message-line labels per §0a adjudication 2 (once-at-activation) → §3.6;
10. one upload per decode discipline intact (paint touches size only) → §3.4;
11. placement = user-approved §0 inside-pill option → §3.9 (10 buttons, 16.0 gaps);
12. painted-shape glyphs + labels `Fit width`/`Fit height` + same palette → §3.8.

## §8 Out-of-scope guard (withheld from this step)

- Wave-5 pan (`pan_offset`, edge-then-navigate) — the draw-size composition stays the Wave-5
  anchor; do NOT add any pan/clip logic now.
- The parked future idea (subfolder picker) stays parked in `.agent/project-info/future-ideas.md`.
- Persistence of fit/zoom state: NONE (session-only; also means NO `docs`, NO config file edits).
- `CHANGELOG.md`: untouched (wave standing ban).
- Existing `.kilo/plans/*` edits: FORBIDDEN — this plan is a NEW file; so are §3-new sources.
- Other waves/tasks' TODO sub-items: none touched; the Task-3 `[ ]` boxes flip only at 4.6.
- `.agent/project-structure.md` description refresh belongs to the Task-5 docs cycle (it will
  list `central_surface.rs`/`fit_mode_button.rs`/`fit_mode_tests.rs` (app + scaling) and the
  W/H bindings in tech/architecture docs).

## §9 Line-budget table (planned, per the ≤200 rule; verify after fmt)

| File | HEAD | Planned | Δ reason |
|---|---|---|---|
| `src/app.rs` | 198 | ≈195 | +field/init/mods (8) −paint extraction (10) |
| `src/app/central_surface.rs` | — | ≈38 NEW | ImagePainting literal single-sourced |
| `src/app/fit_mode_button.rs` | — | ≈118 NEW | 2-button glyph suite |
| `src/app/fit_mode_tests.rs` | — | ≈110 NEW | 11 app-policy tests |
| `src/scaling/fit_mode_tests.rs` | — | ≈80 NEW | 6 math tests |
| `src/app/zoom.rs` | 118 | ≈160 | keys+msgs+`set_fit_mode`+label fn |
| `src/app/keyboard_intake.rs` | 59 | ≈68 | W/H reads + dispatch + doc line |
| `src/app/floating_controls.rs` | 179 | ≈182 | import + row call + doc phrase |
| `src/app/fullscreen.rs` | 54 | ≈43 | literal → shared seam |
| `src/app_display.rs` | 120 | ≈122 | 3-field painting + mode base dispatch |
| `src/scaling.rs` | 95 | ≈145 | enum + 2 fns + method + mod decl |
| `src/zoom.rs`, `zoom_button.rs`, `fullscreen_button.rs`, `folder_navigation.rs` | 112/189/47/170 | unchanged | — |

All ≤ 200 ✓; the two >180 grow by ≤3 lines each (only import/call/doc additions).
