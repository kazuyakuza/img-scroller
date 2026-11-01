# Wave 2 UI fixes — centered floating controls + empty-state center Open

**Date:** 2026-11-01  
**Workflow step:** 4.1a (frontend-specialist)  
**Source TODO:** [`.agent/todos/20261101/20261101-todo-1.md`](../../.agent/todos/20261101/20261101-todo-1.md)  
**Branch:** `fix/centered-floating-controls`  
**Status:** Front-end technical specification — user directives only; no implementation plan.

---

## 1. Directives (verbatim from the TODO / operator-run report)

1. **Icons visible-guaranteed.** The arrow-button icons (← → ↑ ↓) render as empty boxes on Windows — buttons must show guaranteed-visible icons in a plain egui/eframe default-font build.
2. **Centered bottom floating buttons replacing rail/toolbar surfaces.** Instead of toolbars/rails, use centered bottom floating buttons.
3. **Open visible ONLY in the empty state, big + center.** The Open… option must be visible only when no image is loaded, shown BIG at the center of the app.

The user’s directives are the specification; no UI-layout proposal gate is needed this time.

---

## 2. Front-end analysis

### 2.1 Window shell and app-state boundaries

The window shell does **not** change:

- `src/app.rs` `window_options()` keeps `ViewportBuilder::default().with_title(APP_WINDOW_TITLE).with_resizable(true).with_maximized(false)`.
- `ImagesScrollerApp` state fields stay exactly as landed: `opened_image_path`, `current_folder_listing`, `decoded_image`, `message_text`, `current_display`.
- The open-flow seam (`open_image`, `accept_open_outcome`) and the navigation seam (`navigate`, `neighbor_image_path`, `NavigationDirection`) are reused, not reimplemented.
- The texture lifecycle in `src/app_display.rs` (`CurrentImageDisplay`) is untouched.

### 2.2 Component-structure change

Current structure (to retire):

- `egui::Panel::top("open_toolbar")` with the always-visible `Open…` button.
- `src/app/rails.rs`: `Panel::right("folder_rail")` and `Panel::bottom("image_rail")` shown conditionally on `image_is_open()`.

Target structure:

- **No top toolbar.** Remove the `open_toolbar` panel and the `open_toolbar_button` helper entirely.
- **No rails.** Delete `src/app/rails.rs` and retire `mod rails;`.
- **Floating overlay** (only while an image is open): a single centered-bottom row of icon buttons laid on top of the image area, not as a space-reserving panel.
- **Empty-state center Open** (only while no image is open): one large centered button inside the `CentralPanel`.
- `CentralPanel` remains the image surface; the overlay is painted above it.

### 2.3 Image texture pipeline

Unchanged from Wave 1/Wave 2:

- Decoded pixels → `egui::ColorImage::from_rgba_unmultiplied` → `ctx.load_texture` → `egui::Image::new(texture_handle).fit_to_exact_size(...)`.
- Upload is gated by `texture_upload_pending` and happens exactly once per successful decode.
- The new UI surface must not trigger per-frame texture re-uploads or allocations.

### 2.4 Keyboard / input interplay

- Arrow-key intake stays **frame-first** in `ImagesScrollerApp::ui()` using `ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowLeft/Right))`.
- Left/Right semantics stay direct (Wave-5 pan deferral is binding).
- The floating overlay must not consume or capture arrow-key events.

### 2.5 Theming / tokens

- Use default egui button visuals for enabled/disabled/hover states.
- Optional pill appearance can be achieved with an `egui::Frame` (fill + corner radius + inner margin) around the button row, but no custom theme is required by the directives.
- High-contrast default visuals are already provided by the standard egui style; do not override colors in a way that would reduce contrast.

### 2.6 Responsive / DPI behavior

- The overlay is anchored relative to the window, not sized in physical pixels.
- Use a logical-point bottom offset (e.g. 24.0 points) and let egui scale it via `pixels_per_point`.
- `Area::constrain(true)` (default) keeps the pill inside the content rect on small windows.
- The image fit-scaling in `app_display.rs` already handles window resizing.

### 2.7 Accessibility

- Icon-only buttons must expose accessible names (e.g. `Response::accessible_name` or `widget_info` with `Role::Button`).
- Disabled buttons remain focusable/exposed by default via `add_enabled`.

---

## 3. Icon-mechanism analysis for the 4.1b architector

The architector must **pick one** mechanism decisively. Below are the three candidates with egui 0.37 API facts, feasibility, and risks. The only binding requirement is that all four arrow buttons show visible icons on Windows 10/11 in a default-font build.

### 3a. egui-painted vector arrow triangles (no font dependency)

**Approach:** Draw triangles (or other shapes) using `egui::Painter` inside a custom clickable widget. egui 0.37 provides shape helpers such as `egui::Shape::convex_polygon` and `egui::Shape::rotated_triangle` ([`egui::Shape` docs](https://docs.rs/egui/0.37.0/egui/enum.Shape.html)). A minimal custom widget can allocate interaction space with `ui.allocate_exact_size(size, Sense::click())` or `ui.allocate_response(size, Sense::click())`, then paint with `ui.painter()` ([`Ui::painter`](https://docs.rs/egui/0.37.0/egui/struct.Ui.html), [`Ui::allocate_exact_size`](https://docs.rs/egui/0.37.0/egui/struct.Ui.html), [`Sense`](https://docs.rs/egui/0.37.0/egui/struct.Sense.html)). `Response::clicked()`, `Response::hovered()`, and `Response::enabled` report state ([`Response` docs](https://docs.rs/egui/0.37.0/egui/response/struct.Response.html)). Accessibility can be supplied via `response.widget_info(|| WidgetInfo::labeled(Role::Button, ui.is_enabled(), "Previous image"))` (egui 0.37 accessibility docs).

**Feasibility:** High — pure Rust, no assets, guaranteed shape rendering.
**Risk / cost:** The implementer must manually implement the enabled/disabled/hover visuals (color choice, opacity, maybe background frame) and keep hit-testing correct. Slightly more code than a text button; easy to get visually inconsistent with default buttons.
**Visual-state requirements:** Must visibly distinguish normal, hovered, and disabled on Windows. Use `ui.visuals().widgets` colors or explicitly dim/tint when `!response.enabled()` or `!ui.is_enabled()`.

### 3b. ASCII labels `"<"`, `">"`, `"^"`, `"v"`

**Approach:** Replace the Unicode glyph labels with plain ASCII strings inside standard `egui::Button`s: `egui::Button::new("<")`, etc. `Button` implements `Widget`; disabled state is produced with `ui.add_enabled(false, button)` ([`egui::Button` docs](https://docs.rs/egui/0.37.0/egui/widgets/struct.Button.html), [`Ui::add_enabled`](https://docs.rs/egui/0.37.0/egui/struct.Ui.html)). Hover/click/disabled visuals are handled automatically.

**Feasibility:** Very high — trivial one-line change.
**Risk / cost:** Visually less polished; users may not read `<`/`>` as previous/next arrows, and `^`/`v` as up/down. Does not match the classic arrow icon expectation.
**Visual-state requirements:** Fully satisfied by the built-in `Button` widget.

### 3c. Bundled font file with arrow glyphs

**Approach:** Embed a font (e.g. via `include_bytes!`) and register it in `FontDefinitions` through `Context::set_fonts` during `App::new`. After that, `Button::new("←")` etc. render using the bundled font. The binary stays a single portable `.exe` but gains a font asset.

**Feasibility:** Medium — well supported by egui, but introduces an external asset.
**Risk / cost:** Font file licensing must be compatible with distribution; increases binary size; adds build-time embedding and runtime font-loading code; unnecessary if vector shapes or ASCII are acceptable. If the bundled font is omitted or fails to load, the tofu boxes return.
**Visual-state requirements:** Fully satisfied by the built-in `Button` widget once the font is loaded.

### 3d. Note on image-based icons (not a candidate, but API-relevant)

If the architector considers small arrow images, `Button::image(image: impl Into<Image<'_>>)` exists in egui 0.37 ([`Button::image`](https://docs.rs/egui/0.37.0/egui/widgets/struct.Button.html)), but it requires a texture/image source and does not solve the visible-icon problem more simply than vector shapes. The three candidates above are sufficient.

---

## 4. Floating centered bottom controls anatomy

### 4.1 Container choice

**Primary candidate: `egui::Area` + `egui::Frame`.**

- `egui::Area::new(egui::Id::new("floating_navigation_pill"))` creates a screen-space area that does not reserve layout space in `CentralPanel` ([`Area` docs](https://docs.rs/egui/0.37.0/egui/containers/struct.Area.html)).
- `Area::anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -24.0))` anchors the area to the bottom center of the window with a 24-point bottom offset. Anchoring also makes the area immovable.
- `Area::order(egui::Order::Foreground)` can raise it above the central panel if needed (egui’s default area layer already renders above standard panels, but explicit `Order::Foreground` is safer).
- `Area::interactable(true)` (default) keeps the buttons clickable.
- Inside the `Area`, wrap the button row in an `egui::Frame` for the pill background / rounded corners / inner margin.
- `Area::constrain(true)` (default) keeps the pill inside the window content rect.

**Alternative candidate: widgets inside `CentralPanel` after the image.**

- Possible in principle, but not recommended: laying out a bottom-centered row inside the same `CentralPanel` closure risks competing with the fit-scaled image for space and complicates the paint order. The `Area` candidate is the idiomatic egui overlay.

### 4.2 Button row contents

When `image_is_open()` is true, show one horizontal row:

- **Previous image button** — icon only, wired exactly to `app.navigate(NavigationDirection::Previous)`.
- **Next image button** — icon only, wired exactly to `app.navigate(NavigationDirection::Next)`.
- **Up folder button** — visible but disabled (`ui.add_enabled(false, …)`), reserved for Wave 3.
- **Down folder button** — visible but disabled (`ui.add_enabled(false, …)`), reserved for Wave 3.

Suggested order (left-to-right): `[←] [→]  [↑] [↓]`. Spacing between groups is optional; a small gap (e.g. 16 points) between image arrows and folder arrows is acceptable.

### 4.3 Button enablement

- Previous/Next enablement is derived from the existing seam:

```rust
let previous_enabled = app.neighbor_image_path(NavigationDirection::Previous).is_some();
let next_enabled = app.neighbor_image_path(NavigationDirection::Next).is_some();
```

This preserves STAY parity with the keyboard path: buttons are disabled when there is no image, no listing, an unresolvable index, or the list edge is reached.

- Up/Down are always `add_enabled(false, …)` in Wave 2.

### 4.4 Button sizing

- Recommended minimum button size: `egui::Vec2::new(36.0, 32.0)` (larger than the retired 28×24 rail buttons because the overlay sits over the image and needs an easy click target).
- Spacing between buttons: `8.0` points.
- Pill frame inner margin: `8.0` points horizontally, `6.0` vertically.

### 4.5 Message placement

The literal-K3 message semantics are binding: **the unsupported-file / failure message is visible only while an image is open.** The empty-state failure shows **no message** (user confirmed).

Two placement options for 4.1b to choose between:

1. **Inside the pill row** — add the message as a label after the four buttons inside the same `Frame`. This keeps the message attached to the controls but may widen the pill significantly.
2. **Small floating line above the pill** — a separate `Area` or label anchored just above the button row. This keeps the pill compact and makes the message read like a status line.

Both options are acceptable as long as the message is hidden when `!image_is_open()` (including empty-state failures).

---

## 5. Empty-state center Open

When **no image is open** (`!self.image_is_open()`):

- Show **exactly one** large centered `Open…` button.
- The button calls `self.open_via_dialog(frame)` (reuse the existing platform dialog seam).
- There must be **no other Open… control** in any state: the top toolbar is gone, and no Open button appears while an image is open.

### 5.1 Concrete size recommendation

- Use `egui::Button::new("Open…").min_size(egui::Vec2::new(180.0, 48.0))`.
- Render the label at a larger size so it reads as the primary empty-state action, e.g. `egui::Button::new(egui::RichText::new("Open…").size(18.0)).min_size(egui::Vec2::new(180.0, 48.0))`.
- Center the button by laying it out inside `CentralPanel` with `egui::Layout::centered_and_justified(egui::Direction::TopDown)` or by using an `Area` anchored `Align2::CENTER_CENTER`.
- These dimensions are in logical points; egui scales them automatically via `pixels_per_point`.

---

## 6. Input interplay

- Keyboard arrow-key handling remains **before** any UI panels/areas are shown, so it is frame-first.
- `egui::Area` defaults to `Sense::click()` when `interactable(true)` and is **not** focusable by default (no `Sense::FOCUSABLE` bit). Therefore the floating overlay will not capture arrow-key events for internal focus navigation.
- The overlay buttons sense clicks only; they do not intercept `ArrowLeft`/`ArrowRight`.
- The final interaction order in `ui()` should be: read arrow keys → branch on `image_is_open()` to draw either the empty-state Open button or the floating overlay + central image.

---

## 7. Conformance checklist for 4.5a verification

- [ ] Window shell unchanged: title `Images Scroller`, resizable, not maximized.
- [ ] `ImagesScrollerApp` state fields are not expanded solely for UI surface needs.
- [ ] Top `open_toolbar` panel and `open_toolbar_button` helper are removed entirely.
- [ ] `src/app/rails.rs` is deleted; `mod rails;` is removed from `src/app.rs`; no commented-out rail code remains.
- [ ] Floating controls are shown only when `image_is_open()`.
- [ ] The floating row is centered at the bottom of the window and is not a space-reserving panel (uses `Area` or equivalent overlay pattern).
- [ ] Previous button calls `app.navigate(NavigationDirection::Previous)`.
- [ ] Next button calls `app.navigate(NavigationDirection::Next)`.
- [ ] Previous/Next enablement equals `neighbor_image_path(direction).is_some()`.
- [ ] Up/Down buttons are visible but disabled in Wave 2 (`add_enabled(false, …)`) and call no navigation handler.
- [ ] No second button handler, no inline decode logic, and no direct mutation of `opened_image_path`/`decoded_image`/`message_text` from the UI code.
- [ ] The failure message is displayed only while an image is open; empty-state failures show no message.
- [ ] Empty state shows one big centered `Open…` button that calls `open_via_dialog(frame)`.
- [ ] No `Open…` control appears while an image is open.
- [ ] Arrow-key intake is checked before the overlay is drawn and remains unchanged.
- [ ] The overlay does not capture or consume arrow-key events.
- [ ] Texture upload remains gated by `texture_upload_pending`; no per-frame re-upload or allocation churn.
- [ ] All new/modified source files respect the project rules (≤200 lines, ≤50-line methods, ≤2 param preference, private-by-default, single-section booleans, self-documenting names, no commented-out code).
- [ ] Unit tests are updated to reflect the retired rails module and final enablement helper location; test count is recorded.

---

## 8. Explicitly out of scope

- Menu-area placeholder / "Open with…" — deferred to later v1 waves.
- Wave 3 up/down folder navigation wiring — buttons are visible-disabled placeholders only.
- Wave 5 pan-then-navigate interplay — arrow keys navigate directly, pan deferred.
- Fullscreen / zoom / fit-mode controls — not in this fix.
- New keyboard shortcuts beyond the existing Left/Right navigation.
- Window-title changes, tooltips, or advanced accessibility announcements.
- Theming overhaul beyond default egui visuals and an optional pill frame.

---

## Sources / API references

- `egui::Area` (anchor, interactable, order, constrain): <https://docs.rs/egui/0.37.0/egui/containers/struct.Area.html>
- `egui::Shape::convex_polygon`, `Shape::rotated_triangle`: <https://docs.rs/egui/0.37.0/egui/enum.Shape.html>
- `egui::Button`, `Button::image`, `Button::min_size`, `Button::sense`: <https://docs.rs/egui/0.37.0/egui/widgets/struct.Button.html>
- `egui::Ui::add_enabled`, `Ui::allocate_exact_size`, `Ui::painter`: <https://docs.rs/egui/0.37.0/egui/struct.Ui.html>
- `egui::Sense`: <https://docs.rs/egui/0.37.0/egui/struct.Sense.html>
- `egui::Response` (clicked, hovered, enabled, widget_info): <https://docs.rs/egui/0.37.0/egui/response/struct.Response.html>
- egui 0.37 accessibility custom-widget example (Context7): `allocate_exact_size` + `WidgetInfo::labeled(Role::Button, …)`.
