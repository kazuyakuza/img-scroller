# Wave 5 — Pan then Navigate at Image Edges (Front-end Technical Specification)

**Spec path:** `.kilo/plans/20261107-wave5-pan-navigate-frontend-spec.md`  
**Scope:** B-Group Tasks 1+2+3 (combined cycle), Critical Workflow step 4.1a only. No application code.  
**Base:** Wave 4.2 codebase (`fix/wave-4-2-operator-feedback` / post-merge `main`); `src/app_display.rs` currently paints `fitted_size × zoom_factor` centered and clipped.  
**Branch for implementation:** `feat/wave-5-pan-then-navigate` (per global plan).  
**References:** `.agent/todos/20261029/20261029-todo-7.md`, `.agent/project-info/brief.md` §Pan and Navigate at Image Edges While Zoomed, `.kilo/plans/20261107-wave42-then-wave5-global-plan.md` §Part B.

---

## 1. Interaction model — per-axis pan-or-navigate state machine

### 1.1 Axis overflow is the only gate

For every arrow-key press the app decides independently per axis:

- **Horizontal axis (Left / Right):** overflow exists when the zoomed draw width is **strictly greater** than the available central-panel width.
- **Vertical axis (Up / Down):** overflow exists when the zoomed draw height is **strictly greater** than the available central-panel height.

The two axes are completely independent: horizontal overflow affects only Left/Right; vertical overflow affects only Up/Down. If one axis overflows and the other does not, the non-overflowing axis keeps its pre-Wave-5 direct-navigation behavior.

### 1.2 State machine per axis

| Axis overflows? | Key direction | Current pan position | Action |
|-----------------|---------------|----------------------|--------|
| No | any | (irrelevant) | **Navigate** directly (pre-Wave-5 behavior restored for that axis). |
| Yes | Toward the edge that is **not yet reached** | Not at the requested edge | **Pan** one step toward that edge. |
| Yes | Toward the edge that is **already reached** | At the requested edge | **Navigate** on the *next distinct press* only. |

### 1.3 Edge-reached-then-navigate rule (pinned)

**Pinned decision:** navigation on an overflowing axis is **never** triggered in the same key press that moves the pan offset to the edge. The user must press the same arrow key a **second distinct time** while the offset is already clamped at the edge.

- Example (wide image): pressing Right repeatedly pans right until the right edge is visible. The press that lands exactly on the right edge only pans. The **next** distinct Right press navigates to the next sibling image.
- Example (tall image): pressing Down repeatedly pans down until the bottom edge is visible. The press that lands on the bottom edge only pans. The **next** distinct Down press navigates to the next sibling folder (Wave-3 target rule).

### 1.4 Auto-repeat handling at edges (pinned recommendation)

**Pinned recommendation:** panning accepts OS auto-repeat (`key_pressed` fires on repeat, per Wave-4.1 F11 observation), so holding an arrow key smoothly pans. At the edge, navigation must require a **non-repeat** key event, identical in spirit to the Wave-4.2 ESC-close repeat filter. A held key that is already at the edge must not navigate repeatedly.

- Pan step processing: use `input.key_pressed(...)`.
- Edge-trigger navigation: use a scan of `input.events` for `egui::Event::Key { key, pressed: true, repeat: false, .. }` matching the arrow key.

> This item is listed as deferred in the TODO; the above is the recommended contract for 4.1b. The 4.1b architector may adjust only if the "distinct press at edge" invariant is preserved.

### 1.5 Other invariants (pinned)

- **No wrap, no skip, silent edges:** when the listing edge is reached (first/last image or first/last sibling folder), navigation stays silent exactly as in Waves 2–3.
- **Per-axis independence:** a diagonal pan is impossible because the arrow keys are axis-aligned; the horizontal and vertical pan offsets are stored and clamped independently.
- **Mouse / touch pan:** out of scope for v1. The brief specifies keyboard pan; 4.1b should record mouse-wheel/drag as out-of-scope.

---

## 2. Pan offsets — state, clamp math, and reset semantics

### 2.1 State field

Add one session-RAM field to `ImagesScrollerApp` in `src/app.rs`:

```rust
/// Pan displacement of the visible viewport from the centered position,
/// in logical pixels. Positive x = viewport panned right; positive y =
/// viewport panned down. Reset on every successful open and on restore-to-fit.
pan_offset: egui::Vec2,
```

- Initialize to `egui::Vec2::ZERO` in `ImagesScrollerApp::empty_state()`.
- Type `egui::Vec2` is chosen because the display and scaling code already uses `egui::Vec2`; no new abstraction is needed.

### 2.2 Coordinate system and clamp math (pinned)

Let:

- `panel_rect` = the central panel rectangle (`ui.available_rect_before_wrap()`).
- `available_size` = `panel_rect.size()`.
- `image_size` = `decoded_image.size()`.
- `fit_size` = `scaling::fitted_size(available_size, image_size)`.
- `draw_size` = `zoom::zoomed_draw_size(fit_size, zoom_factor)`.
- `overflow` = `(draw_size - available_size).max(egui::vec2(0.0, 0.0))`.
- The centered (no-pan) top-left of the image is `panel_rect.center() - draw_size * 0.5`.
- The actual painted top-left is `centered_min - pan_offset`.

**Clamp range (pinned):**

```text
pan_offset.x ∈ [ -overflow.x / 2 , +overflow.x / 2 ]
pan_offset.y ∈ [ -overflow.y / 2 , +overflow.y / 2 ]
```

- `0` means centered (no pan).
- Negative x means viewport panned left; positive x means viewport panned right.
- Negative y means viewport panned up; positive y means viewport panned down.
- When there is no overflow on an axis, the clamp range collapses to `[0, 0]`.

This range guarantees the image always fully covers the panel on the overflowing axis:

- At the negative limit the panel left/top edge aligns with the image left/top edge (no gap).
- At the positive limit the panel right/bottom edge aligns with the image right/bottom edge (no overscan visible because the panel clip rect hides the rest).

### 2.3 Edge predicates (pinned)

Use a small tolerance (`PAN_EDGE_TOLERANCE = 0.5` logical pixels) to avoid float jitter:

- Left edge reached: `clamped_pan_offset.x <= -overflow.x / 2 + tolerance`.
- Right edge reached: `clamped_pan_offset.x >= +overflow.x / 2 - tolerance`.
- Top edge reached: `clamped_pan_offset.y <= -overflow.y / 2 + tolerance`.
- Bottom edge reached: `clamped_pan_offset.y >= +overflow.y / 2 - tolerance`.

### 2.4 Pan step size (recommended; architector finalizes)

The pure pan module exposes one tunable constant:

```rust
pub(crate) const PAN_STEP_FRACTION: f32 = 0.125;
```

- One arrow press pans by `PAN_STEP_FRACTION * available_axis_size` logical pixels.
- The last step before an edge is clamped to the remaining distance; it never overshoots.

**Recommendation:** keep `0.125` (1/8 of the viewport dimension per press) unless the 4.1b architector decides otherwise. This gives a fast but controllable pan on a maximized window.

> The TODO explicitly defers pan step size to 4.1b; the value above is the spec's recommended default, not a binding architecture decision.

### 2.5 Reset semantics (pinned)

- **Every successful open** resets `pan_offset` to `Vec2::ZERO` (mirrors the existing `zoom_factor = 1.0` reset in `accept_open_outcome`).
- **Restore-to-fit (`0` key / restore button)** resets `pan_offset` to `Vec2::ZERO`. `restore_zoom_to_fit` in `src/app/zoom.rs` must call the new `self.reset_pan_offset()` seam.
- **One-shot `W` / `H` fit keys and buttons** do **not** reset `pan_offset`. They only change `zoom_factor`; the existing offset is re-clamped against the new `draw_size` on the next frame.
- **Zoom-in / zoom-out steps** do not reset `pan_offset`; the offset is re-clamped against the new size.
- **Entering empty-folder mode** does not need to reset `pan_offset` because `decoded_image` is cleared and the pan controller is inactive when no image is open. (Resetting on successful open is sufficient.)

---

## 3. Display integration — composing pan offsets into the existing paint

### 3.1 `ImagePainting` gains the pan offset

Extend the 2-field paint request in `src/app_display.rs` to 3 fields:

```rust
pub(crate) struct ImagePainting<'a> {
    pub(crate) decoded_image: &'a DecodedImage,
    pub(crate) zoom_factor: f32,
    pub(crate) pan_offset: egui::Vec2,
}
```

`src/app/central_surface.rs` passes `self.pan_offset` when constructing `ImagePainting`.

### 3.2 Replace centered layout with explicit positioned rect

In `CurrentImageDisplay::paint`:

1. Compute `fit_size` and `draw_size` exactly as today.
2. Compute `overflow` and `clamped_offset` using the pure pan math.
3. Compute `panel_rect = panel_ui.available_rect_before_wrap()`.
4. Compute `centered_min = panel_rect.center() - draw_size * 0.5`.
5. Compute `image_rect = egui::Rect::from_min_size(centered_min - clamped_offset, draw_size)`.
6. Paint with `panel_ui.put(image_rect, egui::Image::new(image_texture).fit_to_exact_size(draw_size));`.

Remove the existing `panel_ui.with_layout(egui::Layout::centered_and_justified(...), ...)` block.

### 3.3 No-gap / no-overscan guarantee

The clamp range in §2.2 ensures:

- `image_rect.left() >= panel_rect.left()` and `image_rect.right() <= panel_rect.right()` on the horizontal axis when overflow exists (with equality at the respective edges).
- The same vertically.
- When no overflow exists on an axis, `clamped_offset` is `0`, so the image is centered on that axis.

The panel's default clip rect will clip any part of the image that extends outside, but the clamp math guarantees no part of the panel background shows through on an overflowing axis.

### 3.4 Texture discipline unchanged

- One texture upload per successful decode remains the rule.
- Panning must not trigger any texture re-upload or allocation.
- `CurrentImageDisplay::paint` already recomposes `draw_size` every frame; adding the offset is a pure rect-position change.

---

## 4. Keyboard and on-screen buttons

### 4.1 Arrow-key intake routing

`src/app/keyboard_intake.rs` stops calling `self.navigate(...)` directly for the four arrow keys. Instead it routes each arrow through the new pan-or-navigate controller.

Recommended intake shape:

1. Read the four arrow presses with the existing `was_pressed` helper.
2. Read the four arrow **non-repeat** flags by scanning `ctx.input(|i| i.events)` for `egui::Event::Key { key, pressed: true, repeat: false }` matching each arrow key.
3. For each pressed arrow, call a new app seam method such as `self.handle_arrow_key(ctx, ArrowKeyPress { direction, non_repeat })`.

The order of the four arrow calls must remain Left → Right → Up → Down (matching current intake order) so that if two arrows are somehow pressed in the same frame, behavior is deterministic.

### 4.2 Pan-or-navigate controller seam

New app seam module `src/app/pan.rs` exposes at minimum:

```rust
pub(super) struct ArrowKeyPress {
    pub direction: NavigationDirection,
    pub non_repeat: bool,
}

impl ImagesScrollerApp {
    pub(super) fn reset_pan_offset(&mut self);
    pub(super) fn handle_arrow_key(&mut self, ctx: &egui::Context, press: ArrowKeyPress);
}
```

Behavior of `handle_arrow_key`:

- If no image is open (`self.image_is_open() == false`), call `self.navigate(press.direction)` directly.
- If an image is open, compute overflow/clamped offset, map `press.direction` to `(PanAxis, PanSign)`, call the pure `pan::arrow_outcome(...)` function, and:
  - `PanOutcome::Pan(delta)` → update `self.pan_offset` on the requested axis and re-clamp.
  - `PanOutcome::Navigate` → call `self.navigate(press.direction)`.

Mapping:

- `NavigationDirection::Previous` → `PanAxis::Horizontal`, `PanSign::Negative`.
- `NavigationDirection::Next` → `PanAxis::Horizontal`, `PanSign::Positive`.
- `NavigationDirection::PreviousSiblingFolder` → `PanAxis::Vertical`, `PanSign::Negative`.
- `NavigationDirection::NextSiblingFolder` → `PanAxis::Vertical`, `PanSign::Positive`.

### 4.3 On-screen pill arrow buttons (pinned)

**Pinned decision:** the four pill arrow buttons keep their current enablement semantics and always navigate on click. They are navigation affordances, not pan controls.

- Enablement stays `app.neighbor_image_path(direction).is_some()` (computed per frame in `src/app/floating_controls.rs`).
- A clicked enabled arrow still calls `app.navigate(direction)` directly, bypassing the pan controller.
- No button becomes disabled just because the image is zoomed and not at the edge.

Consequences:

- `src/app/floating_controls.rs` requires **no functional change** for this wave (only the existing direct `navigate` call remains).
- The pan controller is keyboard-only.

---

## 5. Fullscreen ON behavior

- The arrow-key intake is ungated: it runs the same `read_and_dispatch_keystrokes` path while fullscreen is ON.
- The pan-or-navigate controller therefore works identically in fullscreen.
- The available size used for overflow/clamp math is `ctx.viewport_rect().size()`, which is the fullscreen viewport when fullscreen is ON.
- Zoom in/out keys (`I`/`O`/aliases/`0`/`W`/`H`) and the fullscreen-exit keys (F11 / ESC) remain untouched and continue to work while fullscreen is ON.
- The Wave-4.2 ESC-close popup only appears while fullscreen is OFF and must not intercept arrow keys.

---

## 6. Wave-3 folder-navigation trigger

- The vertical edge uses the **existing** `NavigationDirection::PreviousSiblingFolder` and `NavigationDirection::NextSiblingFolder` seam.
- The horizontal edge uses the **existing** `NavigationDirection::Previous` and `NavigationDirection::Next` seam.
- Target selection for a folder navigation triggered at the top/bottom edge reuses the Wave-3 rule exactly: session last-viewed image → first whitelisted image in alphanumeric order; no same-filename fallback; no skip/wrap; silent list edges; image-less folder enters empty-folder browsing mode.
- The pan controller must **not** duplicate or restate the Wave-3 target algorithm; it only decides whether to call `self.navigate(direction)`.

---

## 7. Impact list and test plan

### 7.1 New files

| File | Purpose |
|------|---------|
| `src/pan.rs` | Pure math: overflow per axis, clamped offset, edge predicates, step size, `arrow_outcome` decision. |
| `src/pan/pan_tests.rs` (or inline `mod tests;`) | Pure unit tests for the math. |
| `src/app/pan.rs` | App seam: `reset_pan_offset`, `handle_arrow_key`, axis/sign mapping, `ArrowKeyPress` request object. |
| `src/app/pan_tests.rs` | App-state tests for the pan-or-navigate state machine and reset semantics. |

### 7.2 Modified files

| File | Change |
|------|--------|
| `src/app.rs` | Add `pan_offset: egui::Vec2` field; initialize in `empty_state`; reset to zero in `accept_open_outcome` on success; add `mod pan;` and `#[cfg(test)] mod pan_tests;`. |
| `src/app/keyboard_intake.rs` | Replace the four direct `self.navigate(...)` calls with calls to `self.handle_arrow_key(ctx, ArrowKeyPress { direction, non_repeat })`; add non-repeat scan helper for arrow keys. |
| `src/app/zoom.rs` | `restore_zoom_to_fit` calls `self.reset_pan_offset()`. One-shot `W`/`H` seams do **not** reset pan. |
| `src/app_display.rs` | Extend `ImagePainting` with `pan_offset`; replace centered layout with explicit `ui.put(image_rect, ...)` using clamped offset. |
| `src/app/central_surface.rs` | Pass `self.pan_offset` into `ImagePainting`. |

`src/app/floating_controls.rs` is expected to require **no change** because arrow buttons keep direct navigation.

### 7.3 Expected test additions

**`src/pan.rs` / `src/pan/pan_tests.rs` (pure math, ~8–10 tests):**

- Wide image reports horizontal overflow, zero vertical.
- Tall image reports vertical overflow, zero horizontal.
- Small/fit image reports zero overflow on both axes.
- Clamped offset stays inside the symmetric bounds.
- Clamped offset collapses to zero when no overflow.
- Right/Down arrows pan positive; Left/Up arrows pan negative.
- A pan step that would overshoot an edge is clamped to the edge.
- At-edge + non-repeat returns `Navigate`; at-edge + repeat returns `Pan(0.0)` or no navigation.
- Small/fit image arrow returns `Navigate` regardless of repeat.

**`src/app/pan_tests.rs` (app state, ~8–10 tests):**

- Fresh app has zero pan offset.
- Successful open resets pan offset to zero.
- `restore_zoom_to_fit` resets pan offset to zero.
- One-shot `W`/`H` does not reset pan offset (offset stays, then is re-clamped by later paint/controller).
- Zoomed-wide image: Left/Right pan while not at edge.
- Zoomed-wide image at left edge + non-repeat Left navigates to previous image.
- Zoomed-tall image at bottom edge + non-repeat Down navigates to next sibling folder.
- Fits-entirely image: arrow navigates directly (no pan state change).
- No-image / empty-folder mode: arrow still calls `navigate` directly (existing edge behavior preserved).

### 7.4 Census delta estimate

Current test census: **127** tests (Wave 4.2 Task 5). This wave is expected to add **+16 to +20** unit tests across `src/pan/pan_tests.rs` and `src/app/pan_tests.rs`, for a projected total of **143–147** tests. The exact count is left to the 4.1b plan and implementation.

---

## 8. 4.5a conformance checklist

When this specification is handed to the 4.5a front-end verification step, the verifier must confirm:

1. **Window shell unchanged:** title `Images Scroller`, resizable, always maximized at launch, never auto-fullscreened, not modal.
2. **App state:** `ImagesScrollerApp` contains a single `pan_offset: egui::Vec2` session field, initialized to zero, reset on every successful open.
3. **Pure pan module exists:** `src/pan.rs` contains overflow, clamp, edge, step, and `arrow_outcome` logic with unit tests.
4. **App pan seam exists:** `src/app/pan.rs` wires `handle_arrow_key`, `reset_pan_offset`, and axis/sign mapping.
5. **Keyboard intake routing:** `src/app/keyboard_intake.rs` no longer calls `navigate` directly for arrows; arrows route through the pan-or-navigate seam.
6. **Non-repeat edge navigation:** at-edge navigation requires a non-repeat key event; panning accepts repeat.
7. **Display integration:** `src/app_display.rs` paints with explicit `ui.put(image_rect, ...)` using the clamped pan offset; the old centered layout is gone.
8. **No gap / no overscan:** the clamp math guarantees the image rect fully covers the central panel on any overflowing axis.
9. **Button semantics unchanged:** pill arrow buttons remain enabled per `neighbor_image_path(...).is_some()` and always navigate directly on click.
10. **Fullscreen ON:** arrow keys pan/navigate identically while fullscreen is ON; zoom and fullscreen keys remain unaffected.
11. **Wave-3 folder trigger preserved:** vertical edge navigates via `PreviousSiblingFolder`/`NextSiblingFolder`; horizontal edge via `Previous`/`Next`; target selection and empty-folder behavior unchanged.
12. **Reset semantics:** restore/0 and every successful open reset pan to zero; one-shot `W`/`H` does not.
13. **Texture discipline:** no per-frame texture re-upload or allocation; one upload per successful decode only.
14. **No mouse pan:** mouse-wheel / drag pan is absent and recorded as out-of-scope.
15. **No fit-mode suppression:** because `FitMode` was removed in Wave 4.1, no mode suppresses panning.

---

## 9. Deferred decisions flagged for the 4.1b architector

Per the TODO file, the following are explicitly deferred to 4.1b. This spec does not settle them but flags the interplays:

1. **Pan step size:** the recommended default is `PAN_STEP_FRACTION = 0.125`. Adjusting this constant changes how many presses are needed to reach an edge.
2. **Auto-repeat at edges:** the recommended contract is "pan accepts repeat; edge-trigger navigation requires a non-repeat event." If 4.1b changes this, the held-key-at-edge behavior must be specified to avoid accidental rapid navigation.
3. **Mouse / touch pan:** spec assumes out of scope for v1. 4.1b should confirm and record this note.
4. **"Fit modes suppress panning":** moot because Wave 4.1 removed `FitMode`. No action needed.

---

## 10. Implementation boundaries for the junior implementer

- Do **not** change the navigation target-selection logic (Wave 3) or the listing modules.
- Do **not** change pill arrow button behavior beyond what is required to keep direct navigation working.
- Do **not** add mouse-wheel or drag panning.
- Do **not** introduce persistent state or disk writes.
- Do **not** modify the window shell options.
- Keep all new source files under the 200-line rule and methods under the 50-line rule.
- Follow the existing naming style (`snake_case`, descriptive names, self-documenting code).
