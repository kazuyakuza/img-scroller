# Wave 4 Task 2 — Zoom In/Out: Front-end Technical Specification

Date: 2026-11-02
Source TODO: `.agent/todos/20261029/20261029-todo-6.md`
Base: `feat/wave-4-fullscreen-zoom` after Task 1 fullscreen implementation
Spec file: `.kilo/plans/20261102-wave4-task2-zoom-frontend-spec.md`

## 1. Binding context (source of truth from Task 1)

- The app already carries one session flag `fullscreen_on: bool` in `ImagesScrollerApp`.
- Fullscreen ON hides **all** UI chrome (floating pill, empty-state Open button, empty-folder message) via the gate `full_ui_chrome_is_visible()`.
- Fullscreen OFF restores the exact 0.3.0-derived layout: bottom-center floating pill with message line + four painted arrow buttons + the fifth painted-square fullscreen button.
- The fullscreen toggle is `F11`, evaluated per-frame with `ui.ctx().input(|i| i.key_pressed(egui::Key::F11))`.
- Zoom must NOT break those contracts.

## 2. Window-shell / state / texture contract

- **Window shell**: no change to `src/main.rs` / `window_options()`. Title stays `Images Scroller`, `resizable(true)`, `with_maximized(false)`, never a modal.
- **App state**: add exactly **one** new field to `ImagesScrollerApp` in `src/app.rs`:
  ```rust
  zoom_factor: f32,
  ```
  No other zoom/pan state is added in Task 2.
- **Component structure**: the existing chrome is reused. Zoom UI controls are gated by `full_ui_chrome_is_visible()` like every other chrome element.
- **Texture pipeline**: unchanged one-upload-per-decode discipline in `CurrentImageDisplay`. Zoom only changes the **widget size** passed to `egui::Image::fit_to_exact_size`; it never touches the `TextureHandle` or triggers a re-upload.
- **Theming**: zoom buttons use the same painted-shape family as the arrow triangles and fullscreen square, and the same `widgets.{inactive,hovered}.fg_stroke` palette. No new fonts or custom colors.
- **Responsive behavior**: `fitted_size` is recomputed per-frame from `panel_ui.available_size()`; `zoom_factor` is multiplied in. Resizing the window changes the fit baseline but keeps the zoom factor.
- **Accessibility**: keyboard shortcuts are always active when the window is focused; zoom buttons carry accessible labels `Zoom in` / `Zoom out`; the message line gives at-bounds feedback.
- **Performance**: no per-frame texture upload, no per-frame heap allocation for the zoom path beyond a few `Vec2`/`f32` stack values.

## 3. Verified egui 0.37 API facts

- `egui::Key` includes `Plus`, `Minus`, `Equals`, `Num0`..`Num9`, etc. It does **not** include `NumpadAdd` or `NumpadSubtract`; numpad-specific `+`/`-` keys cannot be bound directly.
- `InputState::key_pressed(Key) -> bool` is edge-triggered (true for one frame per physical press, includes key-repeat).
- `InputState::zoom_delta() -> f32` and `smooth_scroll_delta() -> Vec2` exist for continuous input (ctrl-scroll / pinch), but the classic viewer target is keyboard-first.
- `egui::Image::fit_to_exact_size(Vec2)` is a builder method that makes the widget request an exact display size.
- `egui::Image::paint_at(&Ui, Rect)` is available for explicit rectangle painting.
- `egui::Painter::clip_rect()` / `with_clip_rect(Rect)` exist, but the `Ui` created by `CentralPanel` already has a clip rectangle equal to the panel content area, so shapes/widgets painted inside it are clipped to the viewport by default.

## 4. Zoom state model

Add one field to `ImagesScrollerApp`:

```rust
/// Session zoom factor (Wave-4 Task 2): 1.0 = whole-image fit.
/// Applied to the per-frame fit size in the display path.
zoom_factor: f32,
```

Initialization:

```rust
zoom_factor: 1.0,
```

Rules:

- Session-only RAM state; **no persistence** to disk in v1.
- The factor survives arrow-key navigation and fullscreen toggles.
- The factor is applied to the fit size of the **currently displayed image**; it is dimensionless and independent of image resolution.

### Reset-on-open policy (propose-and-ask)

| Option | Behavior | Pros | Cons |
|---|---|---|---|
| **A — Reset on every successful open** | Any successful open (arrows, dialog, CLI) sets `zoom_factor = 1.0`. | Always starts a new image at fit; no surprise huge images. | Breaks "survives arrows navigation"; forces re-zoom after each arrow. |
| **B — Never reset during the session** | `zoom_factor` persists across all opens, navigation, and fullscreen toggles. | Simplest model; fully satisfies "survives arrows navigation". | Opening a very different image while zoomed may surprise the user. |
| **C — Reset only on explicit opens** | Dialog/CLI open resets to 1.0; arrow navigation preserves zoom. | Keeps browsing consistent; resets on deliberate new file. | Slightly more state to distinguish open sources. |

**Recommendation: Option B (pure session zoom, no reset).** It is the smallest state change and directly matches the "session state" requirement. The architector encodes one of these.

## 5. Action model

Two edge-triggered actions share one pure state seam:

```rust
pub(super) fn zoom_in(&mut self);
pub(super) fn zoom_out(&mut self);
```

Zoom step:

| Aspect | Proposed value | Notes |
|---|---|---|
| Step multiplier | `1.25` | ~25% larger/smaller per step. |
| Minimum | `1.0` (fit) | Zoom-out below fit is rejected. |
| Maximum | `8.0` | 8× fit, generous upper bound. |
| Initial | `1.0` | Whole-image fit. |

Constants (suggested home: `src/scaling.rs` next to `fitted_size`):

```rust
const ZOOM_FACTOR_MINIMUM: f32 = 1.0;
const ZOOM_FACTOR_MAXIMUM: f32 = 8.0;
const ZOOM_STEP_MULTIPLIER: f32 = 1.25;
```

Pure helpers:

```rust
pub(crate) fn clamped_zoom_factor(factor: f32) -> f32 { factor.clamp(ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM) }
pub(crate) fn next_zoom_in(factor: f32) -> f32 { clamped_zoom_factor(factor * ZOOM_STEP_MULTIPLIER) }
pub(crate) fn next_zoom_out(factor: f32) -> f32 { clamped_zoom_factor(factor / ZOOM_STEP_MULTIPLIER) }
```

App-level action seam:

```rust
impl ImagesScrollerApp {
    pub(super) fn zoom_in(&mut self) {
        self.zoom_factor = scaling::next_zoom_in(self.zoom_factor);
    }

    pub(super) fn zoom_out(&mut self) {
        self.zoom_factor = scaling::next_zoom_out(self.zoom_factor);
    }

    pub(super) fn zoom_is_at_minimum(&self) -> bool {
        self.zoom_factor <= ZOOM_FACTOR_MINIMUM + f32::EPSILON
    }

    pub(super) fn zoom_is_at_maximum(&self) -> bool {
        self.zoom_factor >= ZOOM_FACTOR_MAXIMUM - f32::EPSILON
    }
}
```

### At-bounds feedback

The message line already surfaces `message_text` (unsupported-file feedback). Add a zoom-bound idle message with lower precedence:

```rust
const ZOOM_AT_MINIMUM_MESSAGE: &str = "Zoom min";
const ZOOM_AT_MAXIMUM_MESSAGE: &str = "Zoom max";
```

Precedence:

1. `message_text` (unsupported file / open failure) — highest.
2. Zoom bound text if at the corresponding bound and `message_text` is `None`.
3. Empty string otherwise.

UI button enablement:

- Zoom-in button enabled iff `!zoom_is_at_maximum()`.
- Zoom-out button enabled iff `!zoom_is_at_minimum()`.

## 6. Keyboard proposals

All candidates avoid `ArrowLeft`/`ArrowRight`/`ArrowUp`/`ArrowDown` (reserved for navigation / Wave-5 pan) and `F11` (fullscreen).

| Candidate | Zoom in | Zoom out | Pros | Cons |
|---|---|---|---|---|
| **K1 — Equals / Minus (RECOMMENDED)** | `egui::Key::Equals` | `egui::Key::Minus` | One-key, no modifier, labels match the printed `=`/`-` symbols on most keyboards. | On some layouts the `+` symbol requires Shift and is a separate logical key (`Key::Plus`), so Shift+= would not trigger zoom in. |
| **K2 — Plus / Minus alias** | `Key::Equals` **and** `Key::Plus` | `Key::Minus` | Covers both unshifted `=` and shifted `+`; most intuitive symbol set. | Two logical keys bound to one action; slightly more intake code. |
| **K3 — Ctrl+Equals / Ctrl+Minus** | `Ctrl` + `Equals` | `Ctrl` + `Minus` | Browser-like, hard to trigger accidentally. | Two-handed, less immediate for a fast viewer. |

**Recommendation: K1 (Equals / Minus).** It is one-handed, requires no modifier, and matches the existing arrow-key/F11 pattern of simple per-frame `key_pressed` checks. If the planner/user wants to be extra forgiving, K2 is the fallback.

Numpad-dedicated `+`/`-` bindings are **not possible** because egui 0.37's `Key` enum has no `NumpadAdd`/`NumpadSubtract` variants.

Optional middle-ground: `InputState::zoom_delta()` from ctrl-scroll or pinch can be wired later as a continuous supplement, but Task 2's required input is keyboard + UI buttons.

## 7. UI-control placement proposals (propose-and-ask)

The pill currently shows: message line, then `[←] [→] | [↑] [↓] | [□]` (five buttons). All zoom buttons are painted shapes, not font glyphs.

| Option | Placement | Pros | Cons |
|---|---|---|---|
| **A — Sixth/seventh pill buttons** | Append `[+] [−]` after the fullscreen square: `[←][→] | [↑][↓] | [□] | [+] [−]`. | Single control surface; discoverable alongside existing buttons. | Pill becomes ~7 buttons wide (~300+ px); may clip on narrow windows; mixes navigation and view controls. |
| **B — Separate bottom-right overlay (RECOMMENDED)** | Add a second small `egui::Area` anchored bottom-right, holding `[+] [−]`, rendered only when chrome is visible. | Keeps the navigation pill stable; easy to extend with Task 3 fit-width/fit-height buttons; no crowding. | Second overlay to maintain; may overlap the image corner; slightly less consolidated UI. |
| **C — Top-right overlay** | Small `[+] [−]` pair anchored top-right of the window. | Matches conventional "window chrome" location for view controls; far from the navigation habit. | Third overlay area; visually distant from the bottom pill; inconsistent with centered navigation. |

**Recommendation for Planner presentation: Option B.** The pill is already at five buttons; adding two more risks clipping and reduces the visual distinction of the navigation controls. A dedicated small zoom overlay in the bottom-right is the cleanest evolution.

## 8. Fullscreen interplay contract

- Zoom UI controls are chrome: they render **only** while `full_ui_chrome_is_visible()` is `true` (fullscreen OFF).
- Zoom keyboard shortcuts are evaluated in the same per-frame intake block as `F11` and the arrows, **before** the fullscreen/chrome layout dispatch. They remain active while fullscreen ON.
- Toggling fullscreen ON or OFF does **not** modify `zoom_factor`.
- In fullscreen ON, the image is drawn via the same display path using the current `zoom_factor`.

## 9. Display contract

- The drawn image size at zoom factor `z` is:
  ```
  display_size = fitted_size(available_size, image_dimensions) * z
  ```
- The image stays centered in the `CentralPanel` using the existing `Layout::centered_and_justified` + `egui::Image::fit_to_exact_size(display_size)` pattern.
- If `display_size` exceeds the available viewport, the excess is **hidden** by the default clip rectangle of the `CentralPanel` `Ui`. No manual `ClipRect` manipulation is required.
- **Wave-4 Task 2 behavior**: no auto-pan, no scrollbars, no drag-to-pan. The user sees the center region of the zoomed image; the rest is clipped.
- Texture upload discipline remains one upload per successful decode.

### Signature change in `src/app_display.rs`

`CurrentImageDisplay::paint` should receive the current zoom factor so the display size can be scaled:

```rust
pub(crate) fn paint(
    &mut self,
    panel_ui: &mut egui::Ui,
    decoded_image: &DecodedImage,
    zoom_factor: f32,
) {
    self.ensure_current_image_texture(panel_ui.ctx(), decoded_image);
    let Some(image_texture) = self.image_texture.as_ref() else { return; };
    let Some(fit_size) = scaling::fitted_size(panel_ui.available_size(), image_dimensions(decoded_image)) else { return; };
    let display_size = scaling::zoomed_size(fit_size, zoom_factor);
    panel_ui.with_layout(
        egui::Layout::centered_and_justified(egui::Direction::TopDown),
        |image_ui| {
            image_ui.add(egui::Image::new(image_texture).fit_to_exact_size(display_size));
        },
    );
}
```

Pure helper in `src/scaling.rs`:

```rust
pub(crate) fn zoomed_size(fit_size: egui::Vec2, zoom_factor: f32) -> egui::Vec2 {
    fit_size * zoom_factor
}
```

## 10. Wave-5 forward-compat note

- The only state field added now is `zoom_factor: f32`.
- Wave 5 will add a `pan_offset: egui::Vec2` and edge-navigation logic.
- The anchor for that logic is a pure helper that returns the current fitted-and-zoomed draw size:
  ```rust
  fn current_fitted_draw_size(&self, available_size: egui::Vec2) -> Option<egui::Vec2> { ... }
  ```
  This size, compared against the viewport size, tells Wave 5 whether horizontal/vertical panning is possible and when the pan offset has reached an image edge.
- Keep the zoom action seam (`zoom_in` / `zoom_out`) separate from pan logic; Wave 5 will compose on top without rewriting the zoom state.

## 11. Unit-test surface

Container-runnable tests for the pure math (no OS/GUI runtime):

- `next_zoom_in` multiplies by the step multiplier and clamps at the maximum.
- `next_zoom_out` divides by the step multiplier and clamps at the minimum (fit).
- `clamped_zoom_factor` clamps values below minimum and above maximum.
- `zoomed_size` scales a fit size by a zoom factor.
- `zoom_is_at_minimum` / `zoom_is_at_maximum` predicates are exact at the bounds and false inside the range.
- Reset-on-open policy: if Option B is encoded, assert that `zoom_factor` is unchanged after a simulated successful `accept_open_outcome`; if a different policy is encoded, assert that policy.

Suggested location: extend `src/scaling.rs` with the zoom math helpers and their inline `#[cfg(test)] mod tests` block. This keeps the pure logic co-located with `fitted_size` and avoids a new module for a small surface.

## 12. 4.5a conformance checklist

For later front-end verification against this spec:

1. `ImagesScrollerApp` contains exactly one new zoom-related field: `zoom_factor: f32`.
2. `zoom_factor` initializes to `1.0` and is never persisted.
3. The reset-on-open policy matches the architector-encoded decision.
4. `zoom_in` and `zoom_out` are pure state seams used by both keyboard and UI triggers.
5. Clamping bounds are enforced: `ZOOM_FACTOR_MINIMUM = 1.0`, `ZOOM_FACTOR_MAXIMUM = 8.0` (or the architector-encoded values).
6. At-bounds predicates drive button enablement and message-line feedback (`Zoom min` / `Zoom max`).
7. Keyboard zoom shortcuts are evaluated with `ui.ctx().input(|i| i.key_pressed(...))`, edge-triggered, before layout dispatch, and do not collide with arrows or F11.
8. Zoom UI controls are gated by `full_ui_chrome_is_visible()` and are hidden while fullscreen ON.
9. Zoom keyboard shortcuts remain active while fullscreen ON; fullscreen toggles do not reset `zoom_factor`.
10. Display size equals `fitted_size * zoom_factor`; overflow is clipped by the default panel clip rect; no pan/scroll affordance is added this wave.
11. No per-frame texture re-upload: `CurrentImageDisplay` upload discipline remains one upload per successful decode.
12. UI-control placement matches the user-approved option.

## 13. Explicitly out of scope

- Fit width / fit height modes (Task 3). Task 2 only provides the hooks (`zoom_factor`, `fitted_size` helper) so Task 3 can compose later.
- Pan/scroll behavior and edge-then-navigate logic (Wave 5).
- The parked future idea (subfolder picker on image-less root folder).
- `CHANGELOG.md` edits.
- Any persistence of zoom state.
