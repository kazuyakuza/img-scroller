# Wave 4.1 Task 7 — One-Shot Fit Width/Height: Front-End Technical Specification

**Date:** 2026-11-06  
**Branch:** `fix/wave-4-operator-fixes` (HEAD `aa2175b`)  
**Source TODO:** `.agent/todos/20261102/20261102-todo-1.md` Task 7  
**Landed design being superseded:** Wave 4 Task 3 `FitMode` tri-state composition (`FitVisible`/`FitWidth`/`FitHeight`)  
**Output file:** `.kilo/plans/20261102-wave4-1-task7-one-shot-fit-frontend-spec.md`  

## 1. Final UX behavior contract (binding)

This section is the **source-of-truth UX contract** for the architector step (4.1b) and the 4.5a verification step. It replaces the persistent `FitMode` semantics landed in Wave 4 Task 3.

### 1.1 What `W` and `H` do now

- `W` (keyboard or pill button) with an image open: set the session `zoom_factor` to the ratio that makes the image's display **width span the available window width**, preserving aspect. Height is allowed to overflow/clip vertically.
- `H` with an image open: set `zoom_factor` to the ratio that makes the image's display **height span the available window height**, preserving aspect. Width is allowed to overflow/clip horizontally.
- The set is a **one-shot**: after it, `I`/`O` (or `[+]`/`[−]`) step `zoom_factor` up/down normally from that level. No persistent mode exists that could block zoom-out.
- Pressing `W` twice recomputes the same factor (no state change beyond the factor value). Window resize after a `W`-set does **not** re-fit: the stored factor is multiplied by the **current** whole-image-fit base size each frame (`display = fitted_size(current_available_size) × zoom_factor`). This is an operator-testable decision and must be recorded in the implementation comments.

### 1.2 Exact factor formula

For the current decoded image and the current available area:

```text
fit_width_factor  = available_width  / whole_image_fitted_width
fit_height_factor = available_height / whole_image_fitted_height
```

where `whole_image_fitted_width/height` are the dimensions returned by the existing `scaling::fitted_size(available_size, image_size)` (largest uniform scale that keeps the whole image inside the area). Because `fitted_size` preserves aspect, the ratio is the same on both axes, so either axis can be used.

Proposed Rust signatures in `src/scaling.rs`:

```rust
pub(crate) fn fit_width_factor(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<f32>;

pub(crate) fn fit_height_factor(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<f32>;
```

- Return `None` when either size is non-positive (reuse the existing `fitted_size` guards).
- The caller clamps the returned factor to `[ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM]` (`[1/8, 8]` from `src/zoom.rs`). This is a safety net; mathematically the ratio is always ≥ 1.0, so the upper bound is the only realistic clamp.

**Square-image sanity:** for a square image in a square window, `fitted_size` equals the available size, so both ratios are `1.0`. Therefore `W`≈`H`≈`0`/restore≈ whole-image fit, with no visible jump.

### 1.3 Interaction with restore, open, and zoom bounds

- `0` key and the restore-fit pill button: set `zoom_factor` to `1.0` (the neutral fit baseline). Silent. No mode state is touched because no mode state exists.
- Every successful image open: reset `zoom_factor` to `1.0`. Silent.
- `I`/`O` after a `W`/`H` set operate on the stored `zoom_factor` with the Task-5 symmetric step (`×1.25`/`÷1.25`, floor `1/8`, ceiling `8`).
- No message is shown for `W`, `H`, `0`, or open-reset (see §1.4).

### 1.4 Message behavior

- **User-approved addition (this session): remove the `Fit width` / `Fit height` activation messages entirely.** `W`/`H` are silent one-shots, consistent with the already-removed zoom messages.
- The only messages that may appear in the pill line are the unsupported-file message and, from Task 5, the rejected-step `Zoom max` / `Zoom min` messages.

### 1.5 Guards and no-op cases

- No image open (fresh empty state): pill is absent, buttons unrenderable; `W`/`H` keys are no-op silent.
- Empty-folder browsing mode: pill is visible, fit buttons disabled as landed; `W`/`H` keys are no-op silent.
- Invalid/empty decoded payload: no-op silent.
- Button dimming remains the only feedback for the "no image" state.

## 2. Removal surface enumeration (advisory to the architector)

The following Wave-4-Task-3 seams disappear or are repurposed. Two implementation options are noted where the architector may prefer a different split; a recommendation is given.

### 2.1 State and composition

- `ImagesScrollerApp::fit_mode` field — **remove**.
- `scaling::FitMode` enum and `FitMode::base_size` — **remove**.
- `app_display::ImagePainting::fit_mode` field — **remove**. `ImagePainting` becomes `{ decoded_image, zoom_factor }`.
- `app_display::CurrentImageDisplay::paint` base-size computation — change from `fit_mode.base_size(...)` to `scaling::fitted_size(panel_ui.available_size(), image_size)` (always whole-image fit as the base).
- `app::central_surface::paint_open_image` — stop passing `fit_mode` into `ImagePainting`.

### 2.2 App seam

- `app::zoom::set_fit_mode` — **remove**.
- `app::zoom::announce_fit_mode` — **remove**.
- `app::zoom::fit_mode_activation_label` — **remove**.
- `app::zoom::FIT_WIDTH_MESSAGE` and `FIT_HEIGHT_MESSAGE` — **remove**.
- `app::zoom::restore_zoom_to_fit` — keep but simplify to `self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;` only.
- New app seam: `ImagesScrollerApp::set_zoom_to_fit_width(&mut self, ctx: &egui::Context)` and `set_zoom_to_fit_height(&mut self, ctx: &egui::Context)`.
  - Guard: no-op if `!self.image_is_open()`.
  - Compute factor from `ctx.screen_rect().size()` (see §6 residual choice) and the decoded image size.
  - Clamp with the existing `crate::zoom::clamped_zoom_factor` helper (make it `pub(crate)`).
  - Assign to `self.zoom_factor`. Do not write `message_text`.

### 2.3 Button module

- `app::fit_mode_button::FitModeButtonAction` — **remove/rename**.
- `app::fit_mode_button.rs` — either:
  - **Option A (recommended):** rename the file to `app::fit_button`, rename the enum to `FitButtonAction { FitWidth, FitHeight }`, keep the painted glyphs and `image_is_open` enablement gate, and wire clicks to `set_zoom_to_fit_width`/`set_zoom_to_fit_height`. This preserves the existing pill layout and file boundary.
  - **Option B:** fold the two fit actions into `app::zoom_button::ZoomButtonAction` and render them inside the zoom group. This is architecturally possible but mixes zoom-step glyphs with fit glyphs; not recommended.

**Recommendation:** Option A. The pill row then remains conceptually grouped as `[arrows] [fullscreen] [zoom] [fit]`, matching the landed layout.

### 2.4 Keyboard intake

- `app::keyboard_intake.rs`: `W`/`H` dispatch changes from `self.set_fit_mode(...)` to `self.set_zoom_to_fit_width(screen_context)` / `self.set_zoom_to_fit_height(screen_context)`.
- Module doc comment updates: replace references to "fit-mode `W`/`H` keys" with "one-shot fit-width/fit-height `W`/`H` keys".
- Intake order stays as landed (F11 → ESC → zoom-in → zoom-out → restore `0` → `W` → `H` → arrows). Order is now irrelevant for fit/zoom composition but preserves the existing read order.

### 2.5 Button-hint plumbing

- `app::button_hint::PillButton::from_fit_mode_action` — rename to `from_fit_action` (or equivalent) and map `FitButtonAction::FitWidth`/`FitHeight`.
- `hotkey_reference` and `tooltip_text` for `FitWidth`/`FitHeight` — **unchanged** ("Fit width (W)" / "Fit height (H)") per the user-approved set.

### 2.6 `image_is_open` gate

- After the rework, the seam that provides the gate continues to be `ImagesScrollerApp::image_is_open`.
- In Option A, `fit_button::show_fit_pill_buttons` derives enablement from `app.image_is_open()`, exactly as the current `fit_mode_button.rs` does.

### 2.7 `src/app.rs`

- Remove `fit_mode` field initialization in `empty_state`.
- In `accept_open_outcome`, replace `self.set_fit_mode(crate::scaling::FitMode::FitVisible);` with `self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;`.
- Update module declarations: `mod fit_mode_button;` → `mod fit_button;` (Option A), and `mod fit_mode_tests;` → `mod fit_one_shot_tests;`.

## 3. What survives unchanged

- `scaling::fitted_size`, `scaling::fit_width_size`, `scaling::fit_height_size` — the pure math helpers stay; they become the inputs to the new factor functions.
- `zoom::ZOOM_STEP_MULTIPLIER`, `ZOOM_FACTOR_MAXIMUM`, `ZOOM_FACTOR_MINIMUM`, `RESTORE_TO_FIT_FACTOR`, the step/at-bound predicates, and `zoomed_draw_size` — unchanged from Task 5.
- Pill layout, group gaps, button sizes, painted glyphs, tooltips, captions — unchanged.
- Task-5 zoom range (`[1/8, 8]`) and silent at-bound behavior — unchanged.
- Intake order (`W`/`H` after zoom chords) — unchanged.
- Texture pipeline (one upload per decode, no per-frame re-upload) — unchanged.

## 4. Proposed new pure-math helpers

Add to `src/scaling.rs` (below the existing `fit_width_size`/`fit_height_size`):

```rust
/// The zoom factor that scales the whole-image fit up so the image WIDTH
/// spans `available_size.x`. Returns None on non-positive dimensions.
pub(crate) fn fit_width_factor(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<f32> {
    let fitted = fitted_size(available_size, image_size)?;
    Some(available_size.x / fitted.x)
}

/// The height twin of `fit_width_factor`.
pub(crate) fn fit_height_factor(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<f32> {
    let fitted = fitted_size(available_size, image_size)?;
    Some(available_size.y / fitted.y)
}
```

Add a convenience accessor on `DecodedImage` in `src/image_loading.rs` so both `app_display.rs` and `app::zoom.rs` can reuse it without duplicating the `width as f32`/`height as f32` cast:

```rust
impl DecodedImage {
    pub(crate) fn size(&self) -> egui::Vec2 {
        egui::Vec2::new(self.width as f32, self.height as f32)
    }
}
```

Make `crate::zoom::clamped_zoom_factor` `pub(crate)` so the app seam can clamp the fit factor:

```rust
pub(crate) fn clamped_zoom_factor(factor: f32) -> f32 {
    factor.clamp(ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM)
}
```

## 5. Propose-and-ask assessment

- **No UI placement changes.** The pill remains 10 buttons in the same order; only the internal semantics of the two fit buttons change.
- **No new user question is required** beyond the already-answered fit-message removal.
- **Residual UI-affecting choice flagged:** which rectangle supplies `available_size` for the keyboard-triggered `W`/`H` factor, because the keyboard intake has no `Ui` and runs before `CentralPanel` is built. Recommendation: use `ctx.screen_rect().size()` (the window client area). This matches the operator intent "display width spans the window" to within the existing `CentralPanel` frame margins. The display paint continues to use `panel_ui.available_size()` for the whole-image-fit base; the resulting one-shot display is therefore window-spanning in intent and only marginally smaller in practice. This choice should be recorded in a code comment.

## 6. Unit-test surface

### 6.1 Removed tests

- `src/scaling/fit_mode_tests.rs` (6 tests) — the `FitMode::base_size` matrix is obsolete.
- `src/app/fit_mode_tests.rs` (11 tests) — the `set_fit_mode` / mode-state tests are obsolete.

Total removals: **17 tests**.

### 6.2 New / replacement tests

In a renamed `src/scaling/fit_factor_tests.rs` (+6):

1. `fit_width_factor_is_one_for_a_wide_image` — wide image, width is the constraining axis.
2. `fit_width_factor_exceeds_one_for_a_tall_image` — tall image, width-fit zooms in.
3. `fit_height_factor_is_one_for_a_tall_image` — tall image, height is the constraining axis.
4. `fit_height_factor_exceeds_one_for_a_wide_image` — wide image, height-fit zooms in.
5. `square_image_fit_factors_are_one` — square image in square window.
6. `fit_factors_reject_non_positive_dimensions` — zero/negative width or height returns `None`.

In a renamed `src/app/fit_one_shot_tests.rs` (+7):

1. `fit_width_sets_zoom_factor_to_width_fit_ratio` — verify the stored factor after `W`.
2. `fit_height_sets_zoom_factor_to_height_fit_ratio` — verify the stored factor after `H`.
3. `fit_width_is_silent` — `message_text` stays `None` after `W`.
4. `fit_height_is_silent` — `message_text` stays `None` after `H`.
5. `pressing_fit_width_twice_recomputes_the_same_factor` — no persistent mode; same factor both times.
6. `fit_action_is_no_op_when_no_image_is_open` — empty-state/empty-folder `W`/`H` do nothing.
7. `restore_fit_and_open_reset_zoom_factor_to_one` — `0` and open-success go back to `1.0`.

Existing hotkey/label pin tests can move here or stay grouped; the implementer may group them for cohesion:

- `fit_width_hotkey_is_w`, `fit_height_hotkey_is_h`.
- `fit_width_button_label_is_pinned`, `fit_height_button_label_is_pinned`.
- `fit_buttons_enabled_only_when_image_is_open`.

These are relabels/repurposes of existing tests, not net additions.

### 6.3 Census estimate

- Current census: **102** tests.
- Remove: 6 + 11 = **17**.
- Add: 6 + 7 = **13**.
- **Expected final census: 98 tests** (±1 depending on how the implementer groups the hotkey/label pin tests).

## 7. 4.5a conformance checklist

During front-end verification (Critical Workflow step 4.5a), confirm the implementation matches this spec:

### State and composition

- [ ] `ImagesScrollerApp` contains `zoom_factor: f32` and **no** `fit_mode` field.
- [ ] `app_display::ImagePainting` contains `decoded_image` and `zoom_factor` and **no** `fit_mode` field.
- [ ] `CurrentImageDisplay::paint` computes base size with `scaling::fitted_size(...)` (whole-image fit) every frame, then scales by `zoom_factor` via `zoom::zoomed_draw_size`.
- [ ] `app::central_surface::paint_open_image` does not pass any mode into `ImagePainting`.

### New app seam

- [ ] `app::zoom` exposes `set_zoom_to_fit_width(&mut self, ctx: &egui::Context)` and `set_zoom_to_fit_height(...)`.
- [ ] Those methods guard on `image_is_open()`, compute the factor via `scaling::fit_width_factor`/`fit_height_factor`, clamp with `zoom::clamped_zoom_factor`, assign to `self.zoom_factor`, and do **not** touch `message_text`.
- [ ] `restore_zoom_to_fit` sets `self.zoom_factor = RESTORE_TO_FIT_FACTOR` only.
- [ ] `accept_open_outcome` resets `zoom_factor` to `RESTORE_TO_FIT_FACTOR` directly (no `set_fit_mode` call).

### Keyboard and buttons

- [ ] `app::keyboard_intake` dispatches `W`/`H` to `set_zoom_to_fit_width`/`set_zoom_to_fit_height`.
- [ ] Fit pill buttons call the same two methods.
- [ ] Fit pill buttons are enabled exactly when `image_is_open()` is true.

### Removal verification

- [ ] No `FitMode` enum, `FitMode::base_size`, or `#[allow(clippy::enum_variant_names)]` remains in `src/scaling.rs`.
- [ ] No `set_fit_mode`, `announce_fit_mode`, `fit_mode_activation_label`, `FIT_WIDTH_MESSAGE`, or `FIT_HEIGHT_MESSAGE` remains in `src/app/zoom.rs`.
- [ ] `fit_mode_button.rs` is either renamed/reworked to a fit-actions module (Option A) or folded into `zoom_button.rs` (Option B, only if the architector chooses it).
- [ ] `button_hint.rs` maps the fit actions correctly; tooltip strings remain "Fit width (W)" / "Fit height (H)".

### Behavior and performance

- [ ] After `W`/`H`, `I`/`O` step `zoom_factor` normally; zoom-out is never blocked by any persistent mode.
- [ ] `0`/restore and every successful open reset to `zoom_factor = 1.0` silently.
- [ ] No fit-activation messages appear for `W`/`H`/`0`/open.
- [ ] Texture upload discipline is unchanged: one upload per successful decode, no per-frame re-upload triggered by fit logic.
- [ ] No per-frame allocations are introduced by the fit one-shot logic.
- [ ] Test census matches the expected ~98 tests.

## 8. Out of scope

- Wave 4.1 Task 5 symmetric zoom range (`[1/8, 8]`): already landed; consumed as-is.
- Wave 4.1 Task 6 strict-shift zoom aliases: already landed; consumed as-is.
- Parked future ideas (drag & drop, fullscreen reveal, normal-mode auto-hide chrome, Wave 5 pan-then-navigate refinements, per-folder zoom memory).
- `CHANGELOG.md` updates.
- Edits to any existing `.kilo/plans/*` files (this is a new plan file only).

## 9. Dependencies on parallel Task 4/5/6 landings

This spec assumes the Task 5 symmetric zoom range and Task 6 strict-shift alias work visible at HEAD `aa2175b` remain in place:

- `zoom_factor` range is `[1/8, 8]` with `1.0` the neutral baseline.
- Zoom actions are silent (Task 4 stray-message removal).
- Shift aliases are strict; plain `=`/`+`/`-` do nothing.

The Task 7 implementation must compose cleanly with those decisions and must not regress them.
