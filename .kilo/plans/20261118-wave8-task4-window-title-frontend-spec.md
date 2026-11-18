# Wave 8 Task 4 — Window Title Path Feature: Front-end Technical Specification

## 1. Scope and binding source

This spec covers the egui/eframe front-end changes for Wave 8 Task 4:

> The window title shows the path of the VISIBLE image and updates on every navigation. If the full path fits, show it. Otherwise show the image NAME + extension completely; if the name is too long, show only its last characters. With remaining space, add first the folder containing the file, then the drive; between the drive and the visible folder chain show `...`; with still-available space keep adding parent folders (walking up). While the window is NOT maximized: show ONLY the file name (+ extension), last-chars-truncated if too long. While fullscreen is ON: on every visible-image change (navigation), show a popup at the TOP CENTER containing the file name + extension for 3 seconds.

Binding source: `.agent/todos/20261117/20261117-todo-1.md` Task 4 verbatim.

This is step **4.1a** (front-end technical specification). Application code is NOT written here. Exact measurement/truncation thresholds, ellipsis rendering, and popup visuals are left for the architector step 4.1b to settle; this document provides the front-end analysis, the API evidence, and the pinned structural contracts the implementer must follow.

## 2. Front-end analysis

### 2.1 Window-shell / state boundaries

The existing window shell (`src/main.rs`):

```rust
fn window_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(app::APP_WINDOW_TITLE)
            .with_resizable(true)
            .with_maximized(true),
        ..eframe::NativeOptions::default()
    }
}
```

- Title source is currently the static constant `APP_WINDOW_TITLE = "Images Scroller"` (`src/app.rs:16`).
- The window is launched **maximized** and resizable; the task's "not-maximized" rule applies when the user restores the window from maximized state.
- Fullscreen is the existing session flag `fullscreen_on` (`src/app.rs:33`), toggled via `ViewportCommand::Fullscreen` in `src/app/fullscreen.rs`.

The title is now dynamic. The following new fields are required on `ImagesScrollerApp` (`src/app.rs`):

| Field | Type | Purpose |
|-------|------|---------|
| `last_title_image_path` | `Option<PathBuf>` | Detects a navigation change so the title/popup update runs only when the visible image changes. |
| `last_maximized_state` | `Option<bool>` | Detects maximize/restore changes so the title can switch between full-path and name-only modes. |
| `last_sent_window_title` | `String` | Prevents sending identical `ViewportCommand::Title` every frame. |
| `fullscreen_name_popup_expires_at` | `Option<f64>` | 3-second popup deadline; `None` = hidden. |
| `fullscreen_name_popup_text` | `String` | Text shown in the fullscreen popup (cached so it stays stable for the full 3 s even if another navigation occurs). |

These fields are RAM-only, session-local, and never persisted, matching the existing fullscreen/zoom/pan/open-with state.

### 2.2 Title update contract

A new per-frame method on `ImagesScrollerApp` is added (tentatively `update_window_title_and_popups`) and called once per frame from `eframe::App::ui`, **after** `read_and_dispatch_keystrokes` and **before** the layout dispatch. It:

1. Reads `ctx.input(|i| i.viewport())` to obtain `ViewportInfo` (maximized/fullscreen state).
2. Compares the current `opened_image_path` with `last_title_image_path` and the current maximized state with `last_maximized_state`.
3. If either changed, computes the new title string and sends `egui::ViewportCommand::Title(new_title)` **only if** it differs from `last_sent_window_title`.
4. If `fullscreen_on` is true and `opened_image_path` changed, arms the top-center name popup (sets text and deadline = now + 3.0 s) and schedules an expiry wake.
5. Expires the fullscreen popup if `input.time >= deadline`.

Empty-state behavior (pinned):

- When no image is open (`opened_image_path == None`), the title is the base app name `APP_WINDOW_TITLE` (`"Images Scroller"`).
- This also applies while an empty sibling folder is being browsed (the browsed-folder context does not carry an opened image).

Update frequency (pinned):

- Title commands are sent only on open/navigation and on maximize/restore transitions.
- The implementation must NOT send `ViewportCommand::Title` every frame when nothing changed.

### 2.3 Composition algorithm front-end hooks

A new **pure, container-testable** module `src/window_title.rs` is proposed. It exposes a function with approximately this signature:

```rust
pub(crate) fn compose_window_title(
    path: Option<&std::path::Path>,
    available_width: f32,
    is_maximized: bool,
    measure: &mut dyn FnMut(&str) -> f32,
) -> String
```

- If `path` is `None`, returns `"Images Scroller"`.
- If `is_maximized` is `false`, returns only the file name + extension, truncated from the left (last characters kept) if it does not fit.
- If `is_maximized` is `true`, runs the user's path-truncation algorithm and uses `measure` to decide what fits.

The `measure` closure is supplied by the caller and encapsulates the text-width API (see §3.3). This keeps the composition logic independent of egui context lifetime/lock rules and makes it unit-testable with a stub measure.

Helper seams (recommended for the implementer):

- `file_name_with_extension(path: &Path) -> String` — extracts the file name + extension (`image.png`).
- `drive_prefix(path: &Path) -> Option<String>` — on Windows returns `C:` or `\\server\share`; on other targets returns the path prefix or `None`.
- `fit_or_truncate_last(text: &str, max_width: f32, measure: &mut dyn FnMut(&str) -> f32) -> String` — keeps the last characters that fit, prefixing with an ellipsis if truncation occurred.

The exact separator between path segments (e.g. `\`, `/`, ` - `, or a narrow space) and the exact width thresholds are deferred to §6 (architector decisions).

### 2.4 Available-width measurement

Two options are presented for the architector.

**Option A — pixel-accurate measurement via egui fonts (recommended)**

Use the actual egui font layout API to measure candidate strings:

```rust
let galley = ctx.fonts(|fonts| {
    fonts.layout_no_wrap(
        text.to_string(),
        egui::FontId::default(),
        egui::Color32::WHITE,
    )
});
let width_in_points = galley.size().x;
```

Evidence:

- `Context::fonts(|f| ...)` returns `FontsView` (docs.rs egui 0.37.0).
- `FontsView::layout_no_wrap(text, font_id, color) -> Arc<Galley>` (epaint 0.37.0).
- `Galley::size() -> Vec2`; the width is `galley.size().x` (docs.rs egui 0.37.0).

Available width is read from the viewport:

```rust
let viewport = ctx.input(|i| i.viewport().clone());
let available = viewport
    .outer_rect
    .map(|r| r.width())
    .unwrap_or_else(|| ctx.viewport_rect().width());
```

Caveat: the OS caption font is not the same as egui's default font, so a safety margin is required (e.g. `available * 0.85` or subtract a fixed number of points for the minimize/maximize/close buttons). The architector decides the margin.

**Option B — character-count heuristic**

Estimate an average character width (e.g. 7 points for the default body font) and truncate by character count. Simpler but much less accurate on paths with mixed ASCII/non-ASCII characters or varying glyph widths.

This spec recommends **Option A** because the task explicitly asks for pixel-based fitting and the egui 0.37 API provides it directly.

### 2.5 Not-maximized rule hook

The maximized state is read from egui 0.37 `ViewportInfo`:

```rust
let is_maximized = ctx.input(|i| i.viewport().maximized).unwrap_or(false);
```

Evidence: `ViewportInfo` has a public `maximized: Option<bool>` field (docs.rs egui 0.37.0). `None` means unknown; this spec pins the fallback to `false` (name-only mode) when unknown.

Behavior:

- While `is_maximized == true`: use the full path-fitting algorithm.
- While `is_maximized == false` or `None`: show ONLY the file name + extension, last-chars-truncated if too long.
- The transition from maximized to restored (or vice versa) re-triggers title computation even if the image path did not change.

### 2.6 Fullscreen popup

When `fullscreen_on` is true and the visible image changes, a non-interactive overlay `Area` appears at the top center of the screen for 3 seconds, showing the file name + extension.

Render shape (matches the existing ESC-close popup precedent in `src/app/esc_close_confirm.rs`):

```rust
egui::Area::new(egui::Id::unique("fullscreen_image_name_popup"))
    .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 16.0))
    .order(egui::Order::Foreground)
    .interactable(false)
    .show(ctx, |popup_ui| {
        egui::Frame::new()
            .fill(POPUP_FILL_COLOR)
            .corner_radius(POPUP_CORNER_RADIUS)
            .inner_margin(POPUP_INNER_MARGIN_POINTS)
            .show(popup_ui, |frame_ui| {
                frame_ui.label(
                    egui::RichText::new(&self.fullscreen_name_popup_text)
                        .size(POPUP_TEXT_SIZE)
                        .color(POPUP_TEXT_COLOR),
                );
            });
    });
```

Lifetime contract (pinned):

- Armed on every visible-image change while `fullscreen_on == true`.
- Deadline = `input.time + 3.0` seconds.
- Expires automatically when `input.time >= deadline`.
- Toggling fullscreen OFF disarms the popup immediately.
- Because eframe sleeps with `ControlFlow::Wait`, the popup uses the same `request_repaint_after` expiry-wake pattern as `esc_close_confirm.rs` (see §3.4).

The popup is rendered in the fullscreen layout path (`src/app/fullscreen.rs`) so it appears only while fullscreen is ON and above the central image surface (`Order::Foreground`).

## 3. Verified egui 0.37 API evidence

The following APIs were verified against the actual docs.rs egui 0.37.0 / epaint 0.37.0 bindings.

### 3.1 Setting the window title

```rust
ctx.send_viewport_cmd(egui::ViewportCommand::Title("new title".to_string()));
```

Evidence:

- `ViewportCommand::Title(String)` exists in `egui::viewport::ViewportCommand` and is documented as "Set the window title".
- `Context::send_viewport_cmd(cmd: ViewportCommand)` is the public method to send viewport commands.

### 3.2 Reading viewport state

```rust
let maximized = ctx.input(|i| i.viewport().maximized);
let fullscreen = ctx.input(|i| i.viewport().fullscreen);
```

Evidence:

- `InputState::viewport(&self) -> &ViewportInfo`.
- `ViewportInfo` has public fields `maximized: Option<bool>` and `fullscreen: Option<bool>`.

### 3.3 Measuring text width

```rust
let width = ctx.fonts(|fonts| {
    fonts.layout_no_wrap(text.to_string(), egui::FontId::default(), egui::Color32::WHITE)
}).size().x;
```

Evidence:

- `Context::fonts<R>(&self, reader: impl FnOnce(&FontsView) -> R) -> R`.
- `FontsView::layout_no_wrap(&mut self, text: String, font_id: FontId, color: Color32) -> Arc<Galley>`.
- `Galley::size(&self) -> Vec2`.

### 3.4 Popup / Area primitives and expiry wake

Existing project precedent (`src/app/esc_close_confirm.rs`) already uses:

- `egui::Area::new(egui::Id::unique(...))` with `.anchor(...)`, `.order(egui::Order::Foreground)`, `.interactable(false)`.
- `egui::Frame::new()` with `.fill(...)`, `.corner_radius(...)`, `.inner_margin(...)`.
- `ctx.request_repaint_after(Duration::from_secs_f64(remaining + margin))` to wake the eframe event loop from `ControlFlow::Wait`.

The fullscreen name popup reuses the same primitives. No new egui widget types are required.

## 4. Module and test-seam proposal

### 4.1 New modules

1. **`src/window_title.rs`** — pure composition logic.
   - `compose_window_title`
   - Helper functions for drive prefix, file-name extraction, and last-chars truncation.
   - Inline unit tests or a child `src/window_title/tests.rs`.
   - Container-testable; no egui Context required.

2. **`src/app/window_title.rs`** (child module) — app wiring.
   - Reads viewport state and computes available width.
   - Calls `window_title::compose_window_title` with an egui-font measure closure.
   - Manages `last_title_image_path`, `last_maximized_state`, `last_sent_window_title`, and the fullscreen popup deadline/text.
   - Renders the fullscreen popup in the fullscreen layout.

`src/app.rs` is already at the 200-line project cap, so the wiring must live in a child module with a small public seam consumed by `app.rs`.

### 4.2 State changes on `ImagesScrollerApp`

Add the five fields listed in §2.1 and initialize them in `empty_state()`.

### 4.3 Call sites

- In `ImagesScrollerApp::ui` (`src/app.rs`), call the new wiring method once per frame after `read_and_dispatch_keystrokes` and before `show_chrome_or_fullscreen_layout`.
- In `src/app/fullscreen.rs`, render the popup inside `show_fullscreen_layout` when armed (delegated to the new child module).
- Disarm the popup inside `toggle_fullscreen_and_send_command` when toggling OFF (similar to how ESC-close popup is disarmed).

### 4.4 Expected test census delta

Current census (per `tech.md`): **28 test-bearing files** = 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules.

This task adds pure logic that should be unit-tested. Expected delta:

- If inline tests are used in `src/window_title.rs`: 6 in-file test modules → **29 test-bearing files**.
- If a child `src/window_title/tests.rs` is used: 5 in-file + 5 child `tests.rs` → **29 test-bearing files**.

The front-end wiring in `src/app/window_title.rs` can be covered by existing integration-style tests or left to manual verification; no new test module is required for the wiring if the pure core is well-tested.

## 5. Interaction risks

| Risk | Mitigation |
|------|------------|
| **ViewportCommand title spam** | Only send when the computed string differs from `last_sent_window_title`. |
| **Per-frame allocations** | Reuse `String` buffers where practical; the composition runs only when path or maximized state changes, not every frame. |
| **Maximized-state detection reliability** | `ViewportInfo::maximized` may be `None` on some platforms or early frames. Fallback to `false` (name-only). Document this as acceptable because launch is always-maximized, so the first known state will be `Some(true)`. |
| **Available-width accuracy** | OS caption font, DPI, and title-bar button widths differ from egui's measurement. Apply a conservative safety margin (architector decision). |
| **Fullscreen popup z-order** | Use `Order::Foreground` and render inside the fullscreen layout path so it paints above the central image. |
| **Popup overlap with pill/menu** | The popup is at the top center; the pill is hidden while fullscreen is ON, so no overlap. If future chrome is added to fullscreen, this must be revisited. |
| **Popup expiry under `ControlFlow::Wait`** | Use `request_repaint_after` with a small margin, exactly like `esc_close_confirm.rs`. |
| **IME / focus** | The popup is non-interactable; the title change does not move focus. |
| **Drive detection on Windows vs UNC** | Use `std::path::Prefix` to extract disk or UNC prefix; stub builds can return `None`. |

## 6. Items left for the architector (4.1b)

The following are explicitly deferred to the architector step:

1. **Measurement method**: Option A (pixel-accurate via `layout_no_wrap`) or Option B (char-count heuristic). This spec recommends Option A.
2. **Safety margin**: The fraction of available width to use (e.g. 85 %, 90 %) or fixed subtraction for window buttons.
3. **Segment separator**: The character(s) between drive, `...`, folder chain, and file name (e.g. `\`, `/`, ` - `).
4. **Ellipsis rendering**: Use the literal `...` string and measure its width, or use a Unicode ellipsis character (`…`).
5. **Name-only truncation threshold**: How many points of safety margin before truncating the file name when not maximized.
6. **Fullscreen popup visuals**: Fill color, corner radius, text size, text color, top offset, and whether to add a subtle shadow/outline. Recommend matching the existing ESC-close popup style.
7. **Popup disarm on fullscreen OFF**: Confirm whether toggling fullscreen OFF should immediately hide the popup (recommended: yes, mirror ESC-close disarm behavior).

## 7. 4.5a conformance checklist

When the implementation is complete, the front-end verification step must confirm:

- [ ] `src/main.rs` still launches with title from `APP_WINDOW_TITLE` and `with_maximized(true)` unchanged.
- [ ] `ImagesScrollerApp` carries the five new fields and initializes them in `empty_state()`.
- [ ] A new pure module `src/window_title.rs` exists and is unit-tested.
- [ ] The title update method is called exactly once per frame and only sends `ViewportCommand::Title` when the computed title changed.
- [ ] When no image is open, the title is `"Images Scroller"`.
- [ ] When an image is open and the window is maximized, the title follows the full-path → name → folder → drive → parent-folders fitting algorithm.
- [ ] When the window is not maximized, the title shows only the file name + extension, truncated from the left if too long.
- [ ] Maximizing/restoring the window re-triggers title composition.
- [ ] Text measurement uses the actual egui 0.37 `layout_no_wrap` + `Galley::size().x` API if Option A was chosen.
- [ ] Fullscreen popup uses an `Area` anchored `CENTER_TOP` with `Order::Foreground` and `interactable(false)`.
- [ ] Fullscreen popup arms for 3.0 s on every visible-image change while `fullscreen_on == true`.
- [ ] Fullscreen popup expiry uses `request_repaint_after` (mirrors `esc_close_confirm.rs`).
- [ ] Toggling fullscreen OFF disarms the popup.
- [ ] No per-frame texture re-upload or allocation churn is introduced.
- [ ] Test census matches the expected delta (29 test-bearing files) or the architector's recorded deviation.
- [ ] The implementer did not introduce architectural decisions beyond the 50 % restriction (no unapproved new dependencies, no changes to unrelated modules, no scope expansion).

## 8. Spec file metadata

- **Task**: Wave 8, Task 4 — Window title shows the visible image's path.
- **Step**: 4.1a (Front-end Technical Specification).
- **Pinned contracts**: title update timing, empty-state title, state fields, pure composition module, not-maximized detection via `ViewportInfo::maximized`, fullscreen popup shape/lifetime/wake pattern.
- **Open items**: measurement margin, segment separator, ellipsis form, popup visuals — deferred to 4.1b.
