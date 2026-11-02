# Wave 4 Task 3 — Fit Width / Fit Height Modes: Front-end Technical Specification

Date: 2026-11-02
Source TODO: `.agent/todos/20261029/20261029-todo-6.md`
Base: `feat/wave-4-fullscreen-zoom` after Task 2 zoom implementation (HEAD `dbdf0c7`)
Spec file: `.kilo/plans/20261102-wave4-task3-fit-modes-frontend-spec.md`

## 1. Binding context (source of truth from Tasks 1–2)

- The app carries two session flags in `ImagesScrollerApp`:
  - `fullscreen_on: bool` — ON hides all UI chrome via `full_ui_chrome_is_visible()`.
  - `zoom_factor: f32` — 1.0 = the current fit base, min 1.0, max 8.0, step 1.25; reset to 1.0 on every successful open.
- Fullscreen OFF shows the bottom-center floating pill: message line, `[←][→] | [↑][↓] | [□] | [+][−][fit]` (8 buttons; group gaps 16.0).
- Keyboard intake (`src/app/keyboard_intake.rs`) is edge-triggered, evaluated once per frame before layout, and active in every mode (fullscreen included). Existing chords: `F11`, `I`/`Shift+Plus`, `O`/`Shift+Minus`, `0`, and the four arrow keys.
- The display path uses `ImagePainting { decoded_image, zoom_factor }` and computes `display_size = fitted_size(available, image) * zoom_factor` via `crate::zoom::zoomed_draw_size`.
- Texture upload remains one upload per successful decode; zoom never triggers a re-upload.

## 2. Window-shell / state / texture contract

- **Window shell**: no change to `src/main.rs` / `window_options()`. Title stays `Images Scroller`, `resizable(true)`, `with_maximized(false)`, never a modal.
- **App state**: add exactly one new field to `ImagesScrollerApp` in `src/app.rs`:
  ```rust
  /// Session fit mode (Wave-4 Task 3). `FitVisible` is the whole-image-fit
  /// behavior carried from Wave 1/2; `FitWidth`/`FitHeight` are the new modes.
  fit_mode: crate::scaling::FitMode,
  ```
  No other fit-mode state is added in Task 3.
- **Component structure**: reuse the existing chrome. Fit-mode UI controls are gated by `full_ui_chrome_is_visible()` like every other chrome element.
- **Texture pipeline**: unchanged one-upload-per-decode discipline in `CurrentImageDisplay`. Fit modes only change the **widget size** passed to `egui::Image::fit_to_exact_size`; they never touch the `TextureHandle` or trigger a re-upload.
- **Theming**: fit-mode buttons use the same painted-shape family and `widgets.{inactive,hovered,noninteractive}.fg_stroke` palette as the arrow triangles, fullscreen square, and zoom glyphs. No new fonts or custom colors.
- **Responsive behavior**: `fit_mode_size` is recomputed per-frame from `panel_ui.available_size()` and the current `fit_mode`. Resizing the window changes the fit baseline while `zoom_factor` and `fit_mode` persist.
- **Accessibility**: keyboard shortcuts are always active when the window is focused; fit-mode buttons carry accessible labels `Fit width` / `Fit height`; the message line gives mode feedback.
- **Performance**: no per-frame texture upload, no per-frame heap allocation for the fit-mode path beyond a few `Vec2`/`f32` stack values.

## 3. Verified egui 0.37 API facts

- `egui::Key` includes `W`, `H`, `Num1`, `Num2`, `Shift`, `Plus`, `Equals`, `Minus`, `ArrowLeft`, `ArrowRight`, `ArrowUp`, `ArrowDown`, `F11`, etc. `Key::W` and `Key::H` are plain letter keys and are unbound in this app.
- `InputState::key_pressed(Key) -> bool` is edge-triggered (true for one frame per physical press, includes key-repeat).
- `egui::Area`, `egui::Frame`, `egui::Image::fit_to_exact_size(Vec2)`, and the default `CentralPanel` clip rectangle behavior are unchanged from Task 2.

## 4. Fit-mode model

Add a tri-state enum and a mode-parameterized pure size function to `src/scaling.rs` (co-located with the existing `fitted_size` math):

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FitMode {
    /// Whole-image fit: the image is fully visible and centered (Wave 1/2 default).
    FitVisible,
    /// Width fit: the image width matches the available width; height follows the aspect ratio and may overflow vertically.
    FitWidth,
    /// Height fit: the image height matches the available height; width follows the aspect ratio and may overflow horizontally.
    FitHeight,
}
```

Recommended pure function (architector may encode the equivalent `fit_width_size`/`fit_height_size` split if preferred):

```rust
/// The draw size for `image_size` under `mode` inside `available_size`.
/// Returns `None` if either size is non-positive, matching `fitted_size` guards.
pub(crate) fn fit_mode_size(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
    mode: FitMode,
) -> Option<egui::Vec2> {
    if !is_positive_size(image_size) || !is_positive_size(available_size) {
        return None;
    }
    let scale = match mode {
        FitMode::FitVisible => scale_factor(available_size, image_size),
        FitMode::FitWidth => available_size.x / image_size.x,
        FitMode::FitHeight => available_size.y / image_size.y,
    };
    Some(image_size * scale)
}
```

Definitions:

| Mode | Scale factor | Resulting size | Overflow intent |
|---|---|---|---|
| `FitVisible` | `min(available.x/image.x, available.y/image.y)` | whole image inside available area | none |
| `FitWidth` | `available.x / image.x` | width == available width, height == image.y * scale | vertical overflow allowed |
| `FitHeight` | `available.y / image.y` | height == available height, width == image.x * scale | horizontal overflow allowed |

- Session-only RAM state; **no persistence** to disk in v1.
- Default at launch and after every successful open: `FitMode::FitVisible` (mirrors the Task-2 reset-to-fit policy).
- `fit_mode` survives arrow-key navigation only until the next successful open resets it; it survives fullscreen toggles.

## 5. Zoom↔fit-mode interplay proposals

### Option A — Mode-relative multiplier (RECOMMENDED)

`display_size = fit_mode_size(available, image, fit_mode) * zoom_factor`

- `zoom_factor` remains a dimensionless 1.0..8.0 multiplier of the **current mode's** base size.
- `zoom_in` / `zoom_out` adjust `zoom_factor` only; `fit_mode` does not change.
- Pressing the fit-width UI button / hotkey sets `fit_mode = FitWidth` **and** `zoom_factor = 1.0`.
- Pressing the fit-height UI button / hotkey sets `fit_mode = FitHeight` **and** `zoom_factor = 1.0`.
- Pressing the restore-fit UI button / `0` hotkey sets `fit_mode = FitVisible` **and** `zoom_factor = 1.0`.
- Fit modes recompute per-frame from the window size; no explicit resize event is required.

**Pros:** mental model is direct (the mode is a scaling policy; zoom scales that policy); active-mode state is stable and easy to feedback; preserves the existing `zoom_in_once` / `zoom_out_once` seams without special-casing modes.

**Cons:** zooming while in fit-width can produce very tall images; the "restore fit" button returns to whole-image fit rather than the current mode's 1× base.

### Option B — Fit mode as a zoom preset; manual zoom exits to `FitVisible`

- Fit-width / fit-height buttons set `fit_mode` and compute an absolute scale relative to the whole-image fit base, storing it as `zoom_factor`.
- Any subsequent `zoom_in` / `zoom_out` transitions `fit_mode` to `FitVisible` and continues scaling relative to the whole-image fit base from that absolute scale.
- Restore-fit always returns to `FitVisible` and `zoom_factor = 1.0`.

**Pros:** manual zoom always behaves like Task 2 after the first step; less chance of accidentally enormous vertical/horizontal images.

**Cons:** mode state is lost as soon as the user zooms, so active-mode feedback disappears; more arithmetic and state transitions; pan-anchor logic in Wave 5 must handle a mode switch mid-zoom.

**Recommendation: Option A.** It keeps `fit_mode` as a genuine, persistent view mode, matches the user's request for selectable fit modes, and minimizes changes to the existing zoom action seams.

## 6. Keyboard candidates for fit-width / fit-height

All candidates avoid the existing chords (`ArrowLeft`/`ArrowRight`/`ArrowUp`/`ArrowDown`, `F11`, `I`/`O`/`0`).

| Candidate | Fit width | Fit height | Pros | Cons |
|---|---|---|---|---|
| **K1 — `W` / `H` (RECOMMENDED)** | `egui::Key::W` | `egui::Key::H` | Mnemonic, one-key family consistent with `I`/`O`/`0`/`F11`; no modifier. | `W` is a common letter, but this app has no text input. |
| **K2 — `Shift+W` / `Shift+H`** | `Shift` + `W` | `Shift` + `H` | Harder to trigger accidentally. | Two-handed, inconsistent with the simple one-key zoom family. |
| **K3 — `1` / `2`** | `egui::Key::Num1` | `egui::Key::Num2` | Digit-family, adjacent keys. | Less mnemonic; conflicts with any future numbered presets. |

**Recommendation: K1 (`W` / `H`).** It is discoverable, single-handed, and follows the established keyboard family.

Intake contract: add `fit_width_pressed` and `fit_height_pressed` to the same per-frame block in `src/app/keyboard_intake.rs`:

```rust
let fit_width_pressed = was_pressed(screen_context, egui::Key::W);
let fit_height_pressed = was_pressed(screen_context, egui::Key::H);
...
if fit_width_pressed {
    self.set_fit_mode(crate::scaling::FitMode::FitWidth);
}
if fit_height_pressed {
    self.set_fit_mode(crate::scaling::FitMode::FitHeight);
}
```

These reads are active in fullscreen and in chrome mode, just like the zoom chords.

## 7. UI-control placement proposals (propose-and-ask)

The pill currently shows: message line, then `[←][→] | [↑][↓] | [□] | [+][−][fit]`.

### Option A — Extend the pill with a new fit-mode group (RECOMMENDED)

Append `[W][H]` after the zoom group with a 16.0 group gap:

```
[←][→] | [↑][↓] | [□] | [+][−][fit] | [W][H]
```

- **Pros:** single control surface; consistent with the user-approved Task-2 pill placement; fit-mode buttons sit next to the zoom group, which is the natural view-control neighborhood.
- **Cons:** 10 buttons wide (~440 px including margins/gaps). On very narrow windows the pill may clip the image edges. If this becomes a problem, Option B is the fallback.

### Option B — Separate bottom-left overlay

Add a second small `egui::Area` anchored bottom-left, holding only `[W][H]`, rendered when chrome is visible.

- **Pros:** avoids pill crowding; easy to reposition independently.
- **Cons:** second overlay to maintain; may visually compete with the centered navigation pill.

### Option C — Replace/augment the message line with textual buttons

Not recommended; it mixes control affordances with status text and breaks the painted-shape button family.

**Recommendation for Planner presentation: Option A.** It is the smallest evolution of the existing UI and keeps all view controls together. The fit-mode group should use the same 16.0 gap convention as the other groups.

### Active-mode feedback options

| Option | Mechanism | Pros | Cons |
|---|---|---|---|
| **F1 — Message-line label (RECOMMENDED)** | Show `Fit width` or `Fit height` in the existing pill message line while that mode is active (precedence: unsupported file > zoom bound > fit mode). | Reuses existing status surface; accessible; no new button-state visual logic. | `FitVisible` is silent by design (it is the default). |
| **F2 — Button highlight | Paint the active fit-mode button in the `hovered` palette (or a subtle accent) while its mode is active. | Immediate visual indicator at the control. | Requires selected-state rendering; may be confused with hover state. |
| **F3 — Both | Combine F1 and a muted glyph fill. | Strongest feedback. | More code for a small surface. |

**Recommendation: F1.** It is the simplest, most accessible, and least surprising. `FitVisible` needs no label because it is the default state.

Fit-mode button glyphs (painted shapes, no font glyphs):
- **Fit width (`W`):** a horizontal line capped by left-pointing and right-pointing arrow triangles.
- **Fit height (`H`):** a vertical line capped by up-pointing and down-pointing arrow triangles.

Use the same `ARROW_TRIANGLE_SIZE`/`ZOOM_BUTTON_SIZE` constants family and the same disabled/idle/hovered color logic.

## 8. Fullscreen interplay

- Fit-mode UI controls are chrome: they render **only** while `full_ui_chrome_is_visible()` is `true` (fullscreen OFF).
- Fit-mode keyboard shortcuts are evaluated in the same per-frame intake block as `F11`, zoom, and the arrows; they remain active while fullscreen ON.
- Toggling fullscreen ON or OFF does **not** modify `fit_mode` or `zoom_factor`.
- In fullscreen ON, the image is drawn via the same display path using the current `fit_mode` and `zoom_factor`.

## 9. Display contract

Under the recommended Option A interplay, the drawn image size is:

```
display_size = fit_mode_size(available_size, image_dimensions, fit_mode) * zoom_factor
```

The image stays centered in the `CentralPanel` using the existing `Layout::centered_and_justified` + `egui::Image::fit_to_exact_size(display_size)` pattern. If `display_size` exceeds the available viewport, the excess is hidden by the default clip rectangle of the `CentralPanel` `Ui`.

### Minimal `ImagePainting` shape change

`src/app_display.rs` must receive the current `fit_mode` so the display size can use the correct base. The minimal change is to add one field:

```rust
pub(crate) struct ImagePainting<'a> {
    pub(crate) decoded_image: &'a DecodedImage,
    pub(crate) zoom_factor: f32,
    pub(crate) fit_mode: crate::scaling::FitMode,
}
```

`CurrentImageDisplay::paint` then becomes:

```rust
let Some(fit_size) = scaling::fit_mode_size(
    panel_ui.available_size(),
    image_dimensions(painting.decoded_image),
    painting.fit_mode,
) else {
    return;
};
let display_size = zoom::zoomed_draw_size(fit_size, painting.zoom_factor);
```

Alternative two-field encoding (architector may choose): keep `ImagePainting` at two top-level fields by passing a precomputed `fit_base_size: egui::Vec2` plus `zoom_factor`. The recommended encoding is the three-field version above because the available size is only known inside `paint`.

`zoom::zoomed_draw_size` remains unchanged:

```rust
pub(crate) fn zoomed_draw_size(fit_size: egui::Vec2, zoom_factor: f32) -> egui::Vec2 {
    fit_size * zoom_factor
}
```

## 10. Wave-5 forward-compat note

- The only new state field added now is `fit_mode: FitMode`.
- Wave 5 will add a `pan_offset: egui::Vec2` and edge-navigation logic.
- The anchor for Wave 5 is the same per-frame draw size computed here:
  ```rust
  fn current_fitted_draw_size(&self, available_size: egui::Vec2) -> Option<egui::Vec2> { ... }
  ```
  This size, compared against the viewport size, tells Wave 5 whether horizontal/vertical panning is possible and when the pan offset has reached an image edge.
- Keep the fit-mode action seam (`set_fit_mode`) separate from pan logic; Wave 5 composes its offsets onto the same draw size without rewriting the fit-mode state.

## 11. Unit-test surface

Container-runnable tests for the pure math and policy (no OS/GUI runtime):

- `fit_mode_size` math:
  - Wide image + `FitWidth` → width matches available, height overflows (taller than available).
  - Tall image + `FitHeight` → height matches available, width overflows (wider than available).
  - Square image + `FitWidth`/`FitHeight`/`FitVisible` correctness.
  - Zero/negative dimensions return `None` for all modes.
- Zoom↔mode interplay policy (under Option A):
  - Setting fit-width resets `zoom_factor` to 1.0 and `fit_mode` to `FitWidth`.
  - `zoom_in_once` while in `FitWidth` multiplies `zoom_factor`; `fit_mode` stays `FitWidth`.
  - `restore_zoom_to_fit` returns `fit_mode` to `FitVisible` and `zoom_factor` to 1.0.
- Reset-on-open policy: after a simulated successful `accept_open_outcome`, `fit_mode` is `FitVisible` and `zoom_factor` is 1.0.
- Hotkey pin test: pressing `W` dispatches `set_fit_mode(FitWidth)`; pressing `H` dispatches `set_fit_mode(FitHeight)`.
- Enablement/active-state rendering predicates: fit-mode buttons are enabled whenever an image is open; the active-mode message is `Some` exactly when `fit_mode != FitVisible`.

Suggested locations: extend `src/scaling.rs` with the fit-mode math tests; add `src/app/fit_mode_tests.rs` (or equivalent) for the app-level policy and hotkey wiring tests.

## 12. 4.5a conformance checklist

For later front-end verification against this spec:

1. `ImagesScrollerApp` contains exactly one new fit-mode field: `fit_mode: crate::scaling::FitMode`.
2. `fit_mode` initializes to `FitVisible` and is reset to `FitVisible` on every successful open; it is never persisted.
3. `scaling::FitMode` is a tri-state enum (`FitVisible`, `FitWidth`, `FitHeight`) and `scaling::fit_mode_size` computes the exact sizes defined in §4.
4. The recommended Option A interplay is encoded: `zoom_in`/`zoom_out` adjust `zoom_factor` only; fit-mode buttons/hotkeys set the mode and reset `zoom_factor` to 1.0; restore-fit returns to `FitVisible` and `zoom_factor = 1.0`.
5. `ImagePainting` carries the current `fit_mode` (or an equivalent precomputed base size) and the display math uses `fit_mode_size * zoom_factor`.
6. Fit-mode keyboard shortcuts are evaluated with `ui.ctx().input(|i| i.key_pressed(...))`, edge-triggered, before layout dispatch, and do not collide with arrows, `F11`, `I`/`O`/`0`.
7. Fit-mode UI controls are gated by `full_ui_chrome_is_visible()` and are hidden while fullscreen ON.
8. Fit-mode keyboard shortcuts remain active while fullscreen ON; fullscreen toggles do not reset `fit_mode` or `zoom_factor`.
9. The active-mode feedback matches the chosen mechanism (message-line labels recommended).
10. No per-frame texture re-upload: `CurrentImageDisplay` upload discipline remains one upload per successful decode.
11. UI-control placement matches the user-approved option.
12. Fit-mode button glyphs are painted shapes with the same palette family and accessible labels.

## 13. Out of scope and Task-2 seam compatibility

### Out of scope

- Wave-5 pan/scroll behavior and edge-then-navigate logic.
- The parked future idea (subfolder picker on an image-less root folder).
- Persistence of fit-mode or zoom state.
- `CHANGELOG.md` edits.

### Task-2 seams touched and how they stay user-compatible

| Task-2 seam | Change | User-visible compatibility |
|---|---|---|
| `zoom_factor` range/clamp/step | None. | Identical behavior. |
| `zoom_in_once` / `zoom_out_once` | Still adjust `zoom_factor`; no mode special-casing under Option A. | Identical behavior. |
| `restore_zoom_to_fit` / `0` hotkey | Now also resets `fit_mode` to `FitVisible`. | When no fit mode is active, this is identical to the old restore-fit; when a fit mode is active, it returns to the familiar whole-image fit. |
| Keyboard intake block | Adds `W`/`H` reads alongside existing chords. | Existing chords unchanged. |
| `ImagePainting` | Gains a `fit_mode` field. | Caller updates the struct literal; the zoom math itself is unchanged. |
| Display math in `app_display.rs` | Uses `fit_mode_size` instead of `fitted_size`. | `FitVisible` produces the same size as the old `fitted_size`. |
| Zoom UI buttons / pill placement | Unchanged; fit-mode buttons are additional. | Existing `[+][−][fit]` behavior unchanged. |

---

**Digest for Planner/user:**
- **Recommended hotkeys:** `W` = fit width, `H` = fit height.
- **Shortlisted placement options:** (A) extend the existing pill with a new `[W][H]` group (recommended); (B) separate bottom-left overlay for the two fit-mode buttons.
- **Recommended zoom↔mode interplay:** Option A — persistent mode-relative multiplier; `zoom_in`/`zoom_out` scale the current mode, fit-mode buttons set the mode and reset zoom, restore-fit returns to `FitVisible`.
- **Recommended active-mode feedback:** message-line label (`Fit width` / `Fit height`) while the mode is active.
