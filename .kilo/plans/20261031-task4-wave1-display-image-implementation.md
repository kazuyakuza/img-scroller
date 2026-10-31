# Implementation Plan — Wave 1, Task 4: Display the Decoded Image (fit-scaled, centered, one texture per decode)

- Date: 2026-10-31
- Workflow step: Critical Workflow §4.1b (Analysis & Planning) — Task 4 of TODO
  [`.agent/todos/20261029/20261029-todo-3.md`](../../.agent/todos/20261029/20261029-todo-3.md) ("### Task 4: Display the image")
- Role: Architector. Plan ONLY — this file is the step deliverable. No application code written in this step; no cargo/docker runs (research reads only).
- Executing branch (already created in Step 2): `feat/wave-1-skeleton-open-image` at commit `0eacf61` (per caller context, HEAD `1498049`) — NO branch operations, NO push in this task.
- Plan approval: AUTO-APPROVED ("Approve Global and Tasks Plans", 2026-10-30). Save + return the path; do NOT ask for approval.
- Front-end input (workflow 4.1 requirement; Task 4 IS front-end related): the 4.1a Front-end Technical Specification is IN FORCE: [`.kilo/plans/20261031-task4-wave1-display-image-frontend-spec.md`](20261031-task4-wave1-display-image-frontend-spec.md) — §1 surface, §2 fit rule, §3 resize, §4 texture lifecycle, §5 state seams, §6 performance guard, §7 deferrals settled here, §9 static checklist for 4.5a. Encoded adaptations are listed in §6 and do NOT require re-opening the spec.
- Global plan in force (do NOT modify): [`.kilo/plans/20261030-wave1-skeleton-open-image.md`](20261030-wave1-skeleton-open-image.md) — §0.2 build environment, §2 Task 4 ("egui upload of decoded pixels; scaling math as pure helper (container-testable) — architector decides integration point"), §3 risks.
- Task-2 plan bindings consumed verbatim (do NOT re-decide): [`.kilo/plans/20261031-task2-wave1-scaffold-window-shell.md`](20261031-task2-wave1-scaffold-window-shell.md) §1.1 (pins: `eframe = "=0.37.0"`, `windows = "=0.62.2"`, edition 2024, decision b module-layout expansion point "Task 4 adds a pure fit-scale helper module (candidate name `src/scaling.rs`, finalized by Task 4's 4.1b)", decision g test strategy) + §8 Task-4 binding ("Image pixels … uploaded as an egui texture … fit-scale math is a pure helper module … with container-runnable tests … no zoom modes (Wave 4)").
- Task-3 plan bindings consumed verbatim (do NOT re-decide): [`.kilo/plans/20261031-task3-wave1-open-image-implementation.md`](20261031-task3-wave1-open-image-implementation.md) §8 (T3-fix binding: `DecodedImage` consumed AS-IS — no payload-shape change; REMOVE the two app.rs scoped `#[allow(dead_code)]` "when adding reads") + amendments fix2 §4 (`egui::Panel::top/bottom` in `ui()` — do not regress), fix3 §2 (canonical `image_loading.rs` scoped-allow set), fix4 (final windows.rs). All four T3 amendments are already IN THE TREE; this task does not re-open any of their content.
- Target implementer: JUNIOR developer, 50% restriction — every structural/naming/scope decision is encoded here; no judgment calls left behind.

## 0. Verified facts (research 2026-10-31; every API shape read first-hand from the resolved egui/epaint 0.37.0 sources or docs.rs 0.37.0)

Verification method note (per Task-2 §2.3 discipline + T3 fix2 lesson): verify the ACTUAL 0.37 shapes before drafting; cite below. egui 0.37.0 published 2026-10-15.

| # | Fact | Source |
| --- | --- | --- |
| V1 | `egui::Context::load_texture(&self, name: impl Into<String>, image: impl Into<ImageData>, options: TextureOptions) -> TextureHandle` — verified signature. Doc note verbatim: "⚠️ Make sure to only call this ONCE for each image, i.e. NOT in your main GUI code." The canonical example shown is a `texture: Option<egui::TextureHandle>` field with a guarded creation step. | docs.rs egui 0.37.0 `Context` page (`load_texture`/`load_texture_cached` section); egui-0.37.0 `crates/egui/src/context.rs` |
| V2 | `egui::ColorImage::from_rgba_unmultiplied(size: [usize; 2], rgba: &[u8]) -> ColorImage` — STRAIGHT (un-multiplied) RGBA8, "Create an ColorImage from flat un-multiplied RGBA data"; PANICS (`assert_eq!`) if `size[0] * size[1] * 4 != rgba.len()`. `DecodedImage.pixels` is exactly a straight RGBA8 row-major buffer (`pixel_buffer_size` invariant ⇒ `len() == w*h*4`), so the call cannot panic on any value produced by the decode seam or the app tests (all drafts below construct consistent payloads). `From<ColorImage> for ImageData` exists ⇒ `ColorImage` passes DIRECTLY into `load_texture`'s `impl Into<ImageData>`. | egui-0.37.0 `crates/epaint/src/image.rs` |
| V3 | `TextureOptions::default()` = `TextureOptions::LINEAR` (linear magnification + minification, `ClampToEdge`, no mipmaps): `impl Default for TextureOptions { fn default() -> Self { Self::LINEAR } }`. Spec §4.5 ("default egui linear sampling") is satisfied literally by `egui::TextureOptions::default()`. | egui-0.37.0 `crates/epaint/src/textures.rs` (`TextureOptions` + `Default` impl) |
| V4 | `egui::TextureHandle`: cheap `Clone`, `Drop` (texture freed when last handle dropped), derived `PartialEq`/`Eq` (equality = same `TextureManager` Arc + `TextureId`, so "upload happened once" IS test-observable: a second `load_texture` yields a NEW id → NOT equal), `id()`, `size_vec2()`, `name()`. `impl From<&TextureHandle> for SizedTexture` exists. | docs.rs egui 0.37.0 `TextureHandle` page |
| V5 | `egui::Image` (root re-export verified: lib.rs `pub use self::widgets::{… Image, ImageFit, ImageOptions, ImageSize, ImageSource, …}`): `Image::new(source: impl Into<ImageSource<'a>>) -> Image<'a>` accepts `&TextureHandle` (via `From<&'a TextureHandle> for SizedTexture` → `From<T: Into<SizedTexture>> for ImageSource<'static>`); sizing builder `.fit_to_exact_size(size: Vec2)` sets `ImageFit::Exact(size)`, and `ImageSize::calc_size` resolves `Exact(size)` through `scale_to_fit(image_source_size, size.min(max_size), maintain_aspect_ratio: true)` — i.e. UNIFORM min-factor scaling (same formula as our fit helper; for a `Texture` source the source size IS the decoded w×h). Widget-impl paint path: `Image::ui` → `ui.allocate_exact_size(ui_size)` → paints the texture across the returned rect. `Layout::centered()` DOES NOT exist in 0.37; the verified constructors are `left_to_right(Align)`, `right_to_left(Align)`, `top_down(Align)`, `top_down_justified(Align)`, `bottom_up(Align)`, `from_main_dir_and_cross_align`, and `centered_and_justified(main_dir: Direction)` ("single widget, uses up all available space", `Align::Center` on both axes). | egui-0.37.0 `crates/egui/src/widgets/image.rs`, `crates/egui/src/lib.rs` (re-exports), `crates/egui/src/layout.rs` |
| V6 | Centering trace (decisive for §1.2-e): with `ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), …)` around a single `ui.add(image)`: (1) `Layout::next_frame_ignore_wrap` sees a vertical layout with `cross_align == Center` AND `cross_justify == true` ⇒ the justified frame expands to the FULL available rect; (2) `Ui::allocate_exact_size(ui_size, …)` returns `placer.align_size_within_rect(ui_size, response.rect)`; (3) `Layout::align_size_within_rect` applies `self.align2()` — for TopDown: horizontal placement = `cross_align` = Center, vertical = `main_align` = Center (TopDown constructor), i.e. `Align2(CENTER, CENTER)` ⇒ the fitted-size rect is CENTERED on both axes inside the full panel interior; (4) `Image::ui` paints the texture across exactly that centered rect. Verified by walking `layout.rs` `next_frame_ignore_wrap` / `justify_and_align` / `align_size_within_rect`, `ui.rs` `allocate_exact_size`, `widgets/image.rs` `impl Widget for Image`. | egui-0.37.0 `crates/egui/src/{layout.rs, ui.rs, widgets/image.rs}` |
| V7 | `Ui` methods used, all verified public in 0.37: `ctx(&self) -> &Context`, `available_size(&self) -> Vec2`, `painter()`; `Ui` derefs to `&Context`. Additionally verified public (NOT used — design rejected it, see §1.2-e): `available_rect_before_wrap() -> Rect`, `max_rect()`. | egui-0.37.0 `crates/egui/src/ui.rs` |
| V8 | Current tree state: `src/app.rs` (150 lines: the two scoped `#[allow(dead_code)]` on `opened_image_path` + `decoded_image`; `egui::Panel::top("open_toolbar")` / `egui::Panel::bottom("message_bar")` per fix2 §4; `CentralPanel::default().show(ui, |_ui| {})`; 4 tests using `super::APP_WINDOW_TITLE`, `app.opened_image_path`, `app.decoded_image`, `app.message_text` field accesses); `src/image_loading.rs` (87 lines; the fix3 §2 scoped-allow set: `pub(crate) struct DecodedImage`, `Unsupported` variant, `user_message`, `pixel_buffer_size`); `src/main.rs` (26 lines; mods: `app`, `image_loading`, `platform`); `src/platform/{mod,windows,stub}.rs`; NO `src/scaling.rs`, NO `src/app_display.rs` (globbed 2026-10-31). | repo files read 2026-10-31 |
| V9 | `egui::Direction` is root re-exported (`pub use epaint::{… Direction …}`); `egui::Layout`, `egui::Context`, `egui::ColorImage`, `egui::TextureHandle`, `egui::TextureOptions`, `egui::Vec2`, `egui::Rect` likewise. All reached via `eframe::egui` (no new dependency, no manifest change in Task 4). | egui-0.37.0 `crates/egui/src/lib.rs` |
| V10 | `egui::vec2(x, y)` and `egui::Vec2::new(x, y)`: `Vec2` derives `PartialEq` + `Debug` (testable with `assert_eq!`). `Vec2 * f32` scale-multiplication exists. | egui-0.37.0 re-exports of `emath` |

## 1. Scope and decisions

### 1.1 In scope (TODO Task 4 sub-items, verbatim mapping)

| # | TODO sub-item | Delivered by |
| --- | --- | --- |
| 1 | Render the decoded image inside the window, scaled to fit the initial view (simple whole-image fit; zoom modes are Wave 4) | §3.2 (`scaling.rs` pure fit math), §3.3+§3.4 (texture + paint wiring into the existing `CentralPanel`) |
| 2 | Resizing the window keeps the display sane (no cropped/clipped render beyond acceptable fit scaling) | Per-frame recompute: fit is recomputed from the CURRENT `panel_ui.available_size()` inside every paint; NO cached scale/rect/transform state exists anywhere in the struct (spec §3.1–3.3) |
| + | Dev-checks incl. new container-runnable tests; cross-target compile re-check (decided here: RUN) | §4 S4/S5 |

### 1.2 Decided in this plan (mandate §2 a–f — settled; zero judgment calls left)

| # | Item | Decision | Value / rationale |
| --- | --- | --- | --- |
| a | Scaling module | **Name finalized: `src/scaling.rs`** (Task-2 §1.1-b candidate name → binding). ONE pure helper: `pub(crate) fn fitted_size(available_size: egui::Vec2, image_size: egui::Vec2) -> Option<egui::Vec2>` with formula `scale = min(available.x / image.x, available.y / image.y)`; `Some(image_size * scale)`. Guard pattern mirrors `pixel_buffer_size` exactly (named `Option` guard, sequential single-section positive-parameter checks via a private `is_positive_size(Vec2) -> bool` helper; BOTH the image dims and the available dims are guarded → zero/negative axes ⇒ `None` ⇒ app paints nothing). NO second pure helper: **CENTERED PLACEMENT IS DECIDED BY EGUI LAYOUT** — `egui::Layout::centered_and_justified(egui::Direction::TopDown)` (the 0.37-verified, public, two-axis-centered constructor; `Layout::centered()` does NOT exist in 0.37 — V5/V6). The spec's `offset = (available − scaled) / 2` centering is realized by egui's internal `Align2(CENTER, CENTER)` resolution of the fitted rect (trace V6), keeping the pure module at exactly one function (spec §2.1 satisfied: no paint calls, no texture code in `scaling.rs`). `scale_factor` and `is_positive_size` stay PRIVATE to scaling.rs (they are not part of the tested contract; `fitted_size` covers them). | |
| b | Texture lifecycle in the app | **KNOWN-PATTERN LAZY UPLOAD, decided firmly (supersedes spec §4.4/§5.2/§5.5 letter — encoded adaptations, see §6 A4/A5):** the app does NOT thread a `Context` into the open flow and does NOT store a `Context` handle (both mandated rejections at 4.1: the CLI-open path runs BEFORE the event loop, and `accept_open_outcome` has no `ctx`). Instead: app field `current_display: CurrentImageDisplay` (small wrapper — explicitly allowed by spec §4 "or a small wrapper around it" and §5.1 "exact field name is a local detail") living in NEW module `src/app_display.rs`. Wrapper state: `image_texture: Option<egui::TextureHandle>` + **`texture_upload_pending: bool`** (the mandate's sanctioned "texture-matches bool" pattern — chosen over the `Option<(u32, u32)>` dims-fingerprint because a dimensions-only fingerprint would show a STALE texture when two DIFFERENT images with the SAME dimensions are opened in sequence, a realistic user flow; the pending bool makes `decode → upload` 1:1 with no fingerprint weakness). Flow: `accept_open_outcome` Ok arm calls `current_display.accept_decoded_image()` (sets `texture_upload_pending = true`); EVERY paint frame calls `CurrentImageDisplay::paint(panel_ui, decoded_image)` whose FIRST step `ensure_current_image_texture(ctx, decoded)` uploads **iff pending**, then clears the flag: `egui::Context::load_texture(CURRENT_IMAGE_TEXTURE_NAME, egui::ColorImage::from_rgba_unmultiplied([w_usize, h_usize], &decoded.pixels), egui::TextureOptions::default())` (V1/V2/V3; per-frame cost after the first frame = one bool read/branch — spec §6.1/6.3 upheld). Texture replaced by plain assignment on each upload; the old `TextureHandle` is dropped by assignment (V4 Drop ⇒ old texture freed; spec §4.3, no manual cache management). Static texture name: `const CURRENT_IMAGE_TEXTURE_NAME: &str = "current_image";` (spec §4.6's suggested name verbatim; `load_texture` allocates a NEW managed id per call, the name is debug-only, so re-use of the static name across decodes is correct). Failure/error semantics: `Err` arm of `accept_open_outcome` stays EXACTLY as Task 3 wrote it — it touches NEITHER `decoded_image` NOR `current_display` ⇒ previous image stays visible behind the message bar (spec §4.7, acceptance 3); empty state (`decoded_image == None`): the CentralPanel closure's `if let Some` simply never calls paint ⇒ blank center (spec §1.2). | |
| c | Paint wiring (exact) | In `ui()` the CentralPanel changes from `show(ui, |_ui| {})` to `show(ui, |panel_ui| { if let Some(decoded_image) = self.decoded_image.as_ref() { self.current_display.paint(panel_ui, decoded_image); } })` (depth 2, within rules). Inside `CurrentImageDisplay::paint(&mut self, panel_ui: &mut egui::Ui, decoded_image: &DecodedImage)`: (1) ensure texture (above); (2) `let Some(image_texture) = self.image_texture.as_ref() else { return; };`; (3) `let Some(display_size) = scaling::fitted_size(panel_ui.available_size(), image_dimensions(decoded_image)) else { return; };` (per-frame recompute — §1.1 item 2); (4) placement: `panel_ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |image_ui| { image_ui.add(egui::Image::new(image_texture).fit_to_exact_size(display_size)); });`. **The exact 0.37 `Image` sizing call is `.fit_to_exact_size(Vec2)`** (V5: `ImageFit::Exact(size)` → uniform `min-factor` scaling via `scale_to_fit(image_source_size, size, maintain_aspect_ratio=true)`; the source size for our `Texture` source IS the decoded w×h ⇒ the widget paints at exactly the fitted size). `.resized(...)` does NOT exist in 0.37, `.fit_to_fraction`/`shrink_to_fit` operate on the AVAILABLE space rather than passing our precomputed fitted size — rejected in favor of the spec-deferred "exact call" being settled as `fit_to_exact_size` with OUR pure-math size. The widget route (ui.add) is chosen over BOTH the low-level painter route (`Image::paint_at(ui, rect)` — would require our own centered-rect helper + a second Ui geometry seam) and `Ui::image((id, size))` (screen-position determined by the default layout cursor → NOT centered). Cited trace V6 proves `centered_and_justified` + `add` centers BOTH axes. | |
| d | `#[allow(dead_code)]` deltas (exact) | **app.rs: exactly ONE allow removed.** `decoded_image`'s `#[allow(dead_code)]` REMOVED — Task 4 display reads `width`/`height`/`pixels` (T3 §8 binding: remove when adding reads ✓). `opened_image_path` KEEPS its scoped `#[allow(dead_code)]` WITH AN UPDATED inline reason ("read again by Wave 2's navigation code") — the mandate's "remove the two" is CONDITIONAL in the in-force spec: frontend-spec §5.4 states verbatim "The `#[allow(dead_code)]` on `opened_image_path` may be removed if the implementation reads it in this task; otherwise it may stay until later navigation code reads it." Decision (b)'s pending-bool pattern gives nothing to read it against, and inventing a read (e.g. title change) is explicitly OUT of scope (spec §7). Removing the allow with no read would fail `clippy -D warnings`. This delta decision is flagged in the caller summary. **image_loading.rs: exactly ONE allow removed, ONE kept-set unchanged.** The four fix3 §2 scoped allows are re-analyzed against Task-4's tree: `pub(crate) struct DecodedImage` + its three pub fields become READ on BOTH targets (display code runs on Linux in dev-checks: `decoded_image.width`/`.height`/`.pixels` are read in `ensure_current_image_texture`) ⇒ struct allow REMOVED. The remaining THREE (`Unsupported` variant, `user_message`, `pixel_buffer_size`) stay BYTE-UNTOUCHED — their only non-test constructions/calls live in `src/platform/windows.rs` (cfg'd out on the container target), Task 4 adds no new callers, so they are still dead under `cargo check`/clippy on the container target (byte-freeze policy: leave exactly as the tree holds them). | |
| e | State that MUST NOT change | `opened_image_path`, `decoded_image`, `message_text` keep their field names and types (existing 4 tests read them verbatim — §3.4 preserves the tests byte-for-byte). `DecodedImage` payload shape unchanged (T3 binding). `UNSUPPORTED_FILE_MESSAGE`, button text `Open…`, window title, `Panel::top/bottom` ids unchanged. | |
| f | Fullscreen/zoom/nav | NOT in this task — no keyboard handling, no UI toggles, no fit-width/fit-height, no title-change (frontend-spec §7 list respected; no such symbol exists in any draft). | |
| + | New module count | Exactly ONE new egui-touching module `src/app_display.rs` (sanctioned by the mandate's "may add ONE module if size demands; encode now" — demanded: full single-file app.rs with tests measured ~253 lines > 200 limit; the split keeps every file well under the Max-Lines rule) plus the pure module `src/scaling.rs`. `src/main.rs` gains `mod app_display;` + `mod scaling;`. No Cargo.toml/Cargo.lock change (no new dependency; egui access via the existing `eframe::egui` re-export, V9). | |

## 2. Line-budget verification (Max-Lines rule, evaluated on the §3 drafts as written)

| File | Substantive content | Measured draft lines (approx., pre-fmt) | Limit |
| --- | --- | --- | --- |
| `src/app.rs` | shell + open flow, display delegated | ~160 (incl. the 51-line preserved test module) | 200 ✓ (margin ≈ 40 for fmt reflow) |
| `src/app_display.rs` | texture state + paint + 2 tests | ~115 | 200 ✓ |
| `src/scaling.rs` | 1 pub + 2 private helpers + 5 tests | ~85 | 200 ✓ |
| `src/main.rs` | mods + entry | 28 | 200 ✓ |

Method bodies: largest is `ensure_current_image_texture` (~14 lines) and `paint` (~15) — ≤50 ✓. Max nesting = 2 (closure inside closure in `paint`; `if let` inside the CentralPanel closure in `ui()`) ✓. Max 2 non-self params everywhere (`ensure_current_image_texture(&mut self, ctx, decoded)` = 2 non-self, same counting as T3 note C6) ✓. All new members private or `pub(crate)`; only `initial`/`accept_decoded_image`/`paint` leave `app_display` ✓ prefer-private.

## 3. Deliverables — full drafts (implementer materializes verbatim; LF endings, trailing newline, UTF-8 no BOM)

### 3.1 `src/main.rs` (REPLACE full file — adds two mod lines)

```rust
//! Entry point of the Images Scroller viewer.

mod app;
mod app_display;
mod image_loading;
mod platform;
mod scaling;

use std::path::PathBuf;

fn main() -> eframe::Result {
    let cli_image_path = cli_image_path();
    eframe::run_native(
        app::APP_WINDOW_TITLE,
        app::window_options(),
        Box::new(move |creation_context| {
            let mut application = app::ImagesScrollerApp::new(creation_context);
            if let Some(image_path) = cli_image_path {
                application.open_image(image_path);
            }
            Ok(Box::new(application))
        }),
    )
}

fn cli_image_path() -> Option<PathBuf> {
    std::env::args_os().nth(1).map(PathBuf::from)
}
```

Semantics: identical to Task 3 except the two new module declarations (alphabetical order). The CLI-open-before-event-loop order is unchanged — display of the CLI-opened image happens on the first `ui()` frame via the pending flag (decision b). NO other change.

### 3.2 `src/scaling.rs` (NEW, full file)

```rust
//! Fit-scaling math for displaying a decoded image inside the viewer window.
//! Pure calculations only: no egui painting, no texture or pixel code.

use eframe::egui;

/// The largest uniform scale that keeps the image inside the available space,
/// applied to both image axes (same factor on both axes, aspect preserved).
/// Returns None when either size has a zero or negative dimension, the same
/// guard style as `pixel_buffer_size` in the decode seam.
pub(crate) fn fitted_size(available_size: egui::Vec2, image_size: egui::Vec2) -> Option<egui::Vec2> {
    if !is_positive_size(image_size) {
        return None;
    }
    if !is_positive_size(available_size) {
        return None;
    }
    Some(image_size * scale_factor(available_size, image_size))
}

fn scale_factor(available_size: egui::Vec2, image_size: egui::Vec2) -> f32 {
    let width_ratio = available_size.x / image_size.x;
    let height_ratio = available_size.y / image_size.y;
    width_ratio.min(height_ratio)
}

fn is_positive_size(size: egui::Vec2) -> bool {
    if size.x <= 0.0 {
        return false;
    }
    if size.y <= 0.0 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::fitted_size;
    use eframe::egui;

    // Test vectors are chosen with scale factors that are exact in binary
    // floating point (0.5, 2.0, 3.0), so `assert_eq!` comparisons are exact.

    #[test]
    fn wide_image_fits_the_available_height() {
        let available_size = egui::Vec2::new(800.0, 300.0);
        let image_size = egui::Vec2::new(400.0, 100.0);
        assert_eq!(
            fitted_size(available_size, image_size),
            Some(egui::Vec2::new(800.0, 200.0))
        );
    }

    #[test]
    fn tall_image_fits_the_available_width() {
        let available_size = egui::Vec2::new(300.0, 800.0);
        let image_size = egui::Vec2::new(100.0, 400.0);
        assert_eq!(
            fitted_size(available_size, image_size),
            Some(egui::Vec2::new(200.0, 800.0))
        );
    }

    #[test]
    fn oversized_image_shrinks_preserving_the_aspect_ratio() {
        let available_size = egui::Vec2::new(400.0, 400.0);
        let image_size = egui::Vec2::new(800.0, 400.0);
        assert_eq!(
            fitted_size(available_size, image_size),
            Some(egui::Vec2::new(400.0, 200.0))
        );
    }

    #[test]
    fn small_image_scales_up_to_fill_one_axis() {
        let available_size = egui::Vec2::new(400.0, 400.0);
        let image_size = egui::Vec2::new(100.0, 200.0);
        assert_eq!(
            fitted_size(available_size, image_size),
            Some(egui::Vec2::new(200.0, 400.0))
        );
    }

    #[test]
    fn empty_dimensions_reject_the_fit() {
        let usable = egui::Vec2::new(100.0, 100.0);
        assert_eq!(fitted_size(usable, egui::Vec2::new(0.0, 10.0)), None);
        assert_eq!(fitted_size(usable, egui::Vec2::new(10.0, 0.0)), None);
        assert_eq!(fitted_size(egui::Vec2::new(0.0, 100.0), usable), None);
        assert_eq!(fitted_size(egui::Vec2::new(100.0, 0.0), usable), None);
    }
}
```

Semantics the implementer must be able to state back:
- `fitted_size` = frontend-spec §2.2 formula literally (min of the two ratios, no downward clamp needed — positivity guards make both ratios well-defined and `≥ 0`; `scale < 0` is impossible since both operands are positive). Upscaling IS allowed (spec §2.2 has no upper clamp; test 4 pins that behavior).
- None-returns: image-dims guard AND available-dims guard (operand order in calls: available FIRST, image SECOND — callers in §3.3 pass exactly that order).
- The spec's §2.5 positive-dimension assumption (decode seam rejects empty buffers) is why this helper ENCOUNTERS None only defensively; the app treats `None` as "paint nothing this frame" (§3.3).

### 3.3 `src/app_display.rs` (NEW, full file)

```rust
//! Display state and paint step of the central image surface: texture lifecycle
//! (one upload per successful decode) and the fit-scaled centered render.

use eframe::egui;

use crate::image_loading::DecodedImage;
use crate::scaling;

const CURRENT_IMAGE_TEXTURE_NAME: &str = "current_image";

/// Texture state of the image shown in the central surface. A successful open
/// flags a pending upload; the next paint uploads exactly once and clears it.
pub(crate) struct CurrentImageDisplay {
    image_texture: Option<egui::TextureHandle>,
    texture_upload_pending: bool,
}

impl CurrentImageDisplay {
    pub(crate) fn initial() -> Self {
        Self {
            image_texture: None,
            texture_upload_pending: false,
        }
    }

    /// A successful decode replaced the decoded payload: the next paint uploads it.
    pub(crate) fn accept_decoded_image(&mut self) {
        self.texture_upload_pending = true;
    }

    /// Paints the decoded image fit-scaled and centered on the given surface.
    pub(crate) fn paint(&mut self, panel_ui: &mut egui::Ui, decoded_image: &DecodedImage) {
        self.ensure_current_image_texture(panel_ui.ctx(), decoded_image);

        let Some(image_texture) = self.image_texture.as_ref() else {
            return;
        };
        let Some(display_size) =
            scaling::fitted_size(panel_ui.available_size(), image_dimensions(decoded_image))
        else {
            return;
        };
        panel_ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |image_ui| {
                image_ui.add(egui::Image::new(image_texture).fit_to_exact_size(display_size));
            },
        );
    }

    /// Uploads the decoded pixels as a texture only while a new decode is pending,
    /// replacing the previous handle by assignment (the dropped handle frees the
    /// old texture). Never runs more than once per successful decode.
    fn ensure_current_image_texture(
        &mut self,
        ctx: &egui::Context,
        decoded_image: &DecodedImage,
    ) {
        if !self.texture_upload_pending {
            return;
        }
        let image_size = [decoded_image.width as usize, decoded_image.height as usize];
        let color_image =
            egui::ColorImage::from_rgba_unmultiplied(image_size, &decoded_image.pixels);
        self.image_texture = Some(ctx.load_texture(
            CURRENT_IMAGE_TEXTURE_NAME,
            color_image,
            egui::TextureOptions::default(),
        ));
        self.texture_upload_pending = false;
    }

    fn image_dimensions(decoded_image: &DecodedImage) -> egui::Vec2 {
        egui::Vec2::new(decoded_image.width as f32, decoded_image.height as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::CurrentImageDisplay;
    use crate::image_loading::DecodedImage;
    use eframe::egui;

    #[test]
    fn initial_display_has_no_texture_and_no_pending_upload() {
        let display = CurrentImageDisplay::initial();
        assert!(display.image_texture.is_none());
        assert!(!display.texture_upload_pending);
    }

    #[test]
    fn successful_upload_happens_exactly_once_per_decoded_image() {
        let mut display = CurrentImageDisplay::initial();
        display.accept_decoded_image();
        assert!(display.texture_upload_pending);

        let decoded_image = DecodedImage {
            width: 2,
            height: 2,
            pixels: vec![0; 16],
        };
        let context = egui::Context::default();
        display.ensure_current_image_texture(&context, &decoded_image);
        assert!(display.image_texture.is_some());
        assert!(!display.texture_upload_pending);
        let uploaded_texture = display.image_texture.clone().expect("uploaded texture");

        display.ensure_current_image_texture(&context, &decoded_image);
        assert_eq!(display.image_texture, Some(uploaded_texture));
    }
}
```

Semantics/draft notes the implementer must be able to state back:
- `egui::Context::default()` in the test creates a REAL egui context without a backend; `load_texture` succeeds (texture delta accumulates in the manager, no renderer needed) — pure-logic container test (decision-g compliant). `TextureHandle` is `PartialEq` + cheap-`Clone` (V4): equality holds iff the SAME allocation (same manager Arc + same `TextureId`) — so the final `assert_eq!` fails if a second `ensure_current_image_texture` call had re-uploaded (a re-upload yields a new `TextureId`) ⇒ the test PINS "upload once per decode".
- The upload call shape (V1/V2/V3): `ctx.load_texture(name, color_image, options)` with `ColorImage::from_rgba_unmultiplied([usize; 2], &pixels)` and `TextureOptions::default()` (= LINEAR). `[w as usize, h as usize]` from the payload's u32 dims; the `pixel_buffer_size` invariant guarantees `pixels.len() == w*h*4` ⇒ the `from_rgba_unmultiplied` size-assert CAN NOT fire (spec §6.4 invariant).
- `ensure_current_image_texture` is private; it is reachable by tests because the test module is a CHILD of this module. Only `initial`, `accept_decoded_image`, `paint` are `pub(crate)`.
- `paint` early-returns WITHOUT painting when the texture is missing or the fit math returns None — the central surface stays default-background (spec §1.2, §2.6).
- `image_dimensions` is a private free fn (1 arg) — keeps `fitted_size` calls uniform and avoids duplicating the u32→f32 conversion.

### 3.4 `src/app.rs` (REPLACE full file — the ONLY edits vs the tree: struct fields block, ONE allow removal, the new `current_display` field, `empty_state`, the `accept_open_outcome` Ok arm ONE new line, the CentralPanel closure; ALL 4 EXISTING TESTS PRESERVED VERBATIM)

```rust
//! Application shell: the regular, resizable viewer window of Images Scroller.

use std::path::PathBuf;

use eframe::egui;

use crate::app_display::CurrentImageDisplay;
use crate::image_loading::{self, DecodedImage, ImageLoadFailure};
use crate::platform;

/// Title shown on the viewer window title bar (final app name, user-decided 2026-10-29).
pub const APP_WINDOW_TITLE: &str = "Images Scroller";

pub struct ImagesScrollerApp {
    /// Path of the last successfully decoded image (read again by Wave 2's navigation code).
    #[allow(dead_code)]
    opened_image_path: Option<PathBuf>,
    /// Decoded pixels of the last successfully opened image (read by the display surface).
    decoded_image: Option<DecodedImage>,
    message_text: Option<&'static str>,
    /// Texture state of the central image display (spec §4 "small wrapper" clause).
    current_display: CurrentImageDisplay,
}

impl ImagesScrollerApp {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self::empty_state()
    }

    fn empty_state() -> Self {
        Self {
            opened_image_path: None,
            decoded_image: None,
            message_text: None,
            current_display: CurrentImageDisplay::initial(),
        }
    }

    /// Opens an image synchronously: decodes now, then applies the outcome to the open-flow state.
    pub fn open_image(&mut self, path: PathBuf) {
        let outcome = image_loading::load_image(&path);
        self.accept_open_outcome(path, outcome);
    }

    /// Failure keeps any previously loaded image and state untouched except the message.
    fn accept_open_outcome(
        &mut self,
        path: PathBuf,
        outcome: Result<DecodedImage, ImageLoadFailure>,
    ) {
        match outcome {
            Ok(decoded_image) => {
                self.opened_image_path = Some(path);
                self.decoded_image = Some(decoded_image);
                self.message_text = None;
                self.current_display.accept_decoded_image();
            }
            Err(_failure) => {
                self.message_text = Some(image_loading::UNSUPPORTED_FILE_MESSAGE);
            }
        }
    }

    fn open_toolbar_button(&mut self, ui: &mut egui::Ui, frame: &eframe::Frame) {
        ui.horizontal(|ui| {
            if ui.button("Open…").clicked() {
                self.open_via_dialog(frame);
            }
        });
    }

    fn open_via_dialog(&mut self, frame: &eframe::Frame) {
        if let Some(image_path) = platform::show_open_image_dialog(frame) {
            self.open_image(image_path);
        }
    }
}

impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::Panel::top("open_toolbar").show(ui, |ui| {
            self.open_toolbar_button(ui, frame);
        });

        if let Some(message) = self.message_text {
            egui::Panel::bottom("message_bar").show(ui, |ui| {
                ui.label(message);
            });
        }

        egui::CentralPanel::default().show(ui, |panel_ui| {
            if let Some(decoded_image) = self.decoded_image.as_ref() {
                self.current_display.paint(panel_ui, decoded_image);
            }
        });
    }
}

pub fn window_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_WINDOW_TITLE)
            .with_resizable(true)
            .with_maximized(false),
        ..eframe::NativeOptions::default()
    }
}

#[cfg(test)]
mod tests {
    use super::ImagesScrollerApp;
    use crate::image_loading::{DecodedImage, UNSUPPORTED_FILE_MESSAGE};
    use std::path::PathBuf;

    #[test]
    fn window_title_is_the_final_app_name() {
        assert_eq!(super::APP_WINDOW_TITLE, "Images Scroller");
    }

    #[test]
    fn failed_open_via_cli_reports_the_message_and_keeps_no_path() {
        let mut app = ImagesScrollerApp::empty_state();
        app.open_image(PathBuf::from("unregistered-in-container.bin"));
        assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
        assert!(app.opened_image_path.is_none());
        assert!(app.decoded_image.is_none());
    }

    #[test]
    fn successful_open_stores_path_and_payload_and_clears_message() {
        let mut app = ImagesScrollerApp::empty_state();
        let image_path = PathBuf::from("photo.png");
        let decoded = DecodedImage {
            width: 1,
            height: 1,
            pixels: vec![0, 0, 0, 255],
        };
        app.accept_open_outcome(image_path.clone(), Ok(decoded));
        assert_eq!(app.opened_image_path, Some(image_path));
        assert!(app.decoded_image.is_some());
        assert!(app.message_text.is_none());
    }

    #[test]
    fn failure_after_success_keeps_the_previous_image_visible() {
        let mut app = ImagesScrollerApp::empty_state();
        let decoded = DecodedImage {
            width: 2,
            height: 2,
            pixels: vec![0; 16],
        };
        app.accept_open_outcome(PathBuf::from("photo.png"), Ok(decoded));
        app.open_image(PathBuf::from("unregistered-in-container.bin"));
        assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
        assert_eq!(app.opened_image_path, Some(PathBuf::from("photo.png")));
        assert!(app.decoded_image.is_some());
    }
}
```

Draft notes the implementer must be able to state back:
- Lines unchanged from the tree: module doc comment, `use` block shape (`crate::image_loading::{self, …}` still used by the Err arm), `APP_WINDOW_TITLE` + doc, `new`, `open_image`, the Err arm, `open_toolbar_button`, `open_via_dialog`, the `Panel::top/bottom` blocks (fix2 §4 binding — do NOT regress to `TopBottomPanel`), `window_options`, and the ENTIRE test module (all 4 tests VERBATIM — no moved field, no renamed assertion; they compile as-is because `opened_image_path`/`decoded_image`/`message_text` remain struct fields).
- Lines changed vs the tree: (1) struct gains ONE new field `current_display: CurrentImageDisplay` with doc; (2) `decoded_image` loses its `#[allow(dead_code)]` and its doc comment's parenthetical becomes "(read by the display surface)"; (3) `opened_image_path` keeps its allow with the updated parenthetical "(read again by Wave 2's navigation code)"; (4) `empty_state` gains `current_display: CurrentImageDisplay::initial()`; (5) `accept_open_outcome` Ok arm gains ONE line `self.current_display.accept_decoded_image();` (Err arm UNTOUCHED — spec §4.7); (6) CentralPanel closure as decision (c). Nothing else.
- Borrow checker facts: the CentralPanel closure captures `self.decoded_image` (shared) and `self.current_display` (mutated through `paint`) — disjoint-field closure captures, edition-2024-legal; inside `CurrentImageDisplay::paint`, `decoded_image` is the CALLER's reference (never aliases `self.image_texture`), so `ensure_current_image_texture(&mut self, …)` mutates while `decoded_image` is only read.
- 4.6 workflow note: the implementer flips the two Task 4 checkboxes `[ ]`→`[x]` in `20261029-todo-3.md` at step 4.6 ONLY (not an S-step here; rest of the TODO file preserved verbatim).

### 3.5 `src/image_loading.rs` (EDIT — exactly ONE deleted line)

Delete the line `#[allow(dead_code)]` (line 11 of the tree file) that sits directly above `pub(crate) struct DecodedImage {`. Resulting region:

```rust
pub(crate) const UNSUPPORTED_FILE_MESSAGE: &str =
    "This file's format is not supported: no Windows image codec is registered for it.";

pub(crate) struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}
```

The remaining three scoped allows (`Unsupported` variant line 19, `user_message` line 26 incl. its `///` doc comment, `pixel_buffer_size` line 38 incl. its `///` doc comment) stay BYTE-AS-IS (still dead on the container target — only non-test call sites are in the cfg-Windows `windows.rs`; no new callers added by Task 4). Everything else in the file untouched (tests included).

### 3.6 `.agent/project-structure.md` (EDIT)

Replace the `# Files in src/` list with EXACTLY (alphabetical; adds the two new files, app.rs/app_display.rs wording updated):

```markdown
# Files in src/

- app.rs - eframe application shell: viewer window options, open-flow state, top Open… button, bottom message bar, window-title constant
- app_display.rs - central image display: texture lifecycle wrapper (one upload per decode), fit-scaled paint, pending-upload flag
- image_loading.rs - open-image seam: load_image entry, DecodedImage payload shape, failure classification, unsupported-file message constant
- main.rs - entry point: CLI image-path intake, starts the eframe native app
- scaling.rs - pure fit-scaling math: largest uniform factor for the available area (no egui painting)
```

Keep the existing `# Folders in src/` block and everything else (including `platform/` and `# Other folders`) untouched.

## 4. Implementation steps (for 4.2 — ordered, atomic, verifiable)

Rule: call `alpine-vm_vm_status` BEFORE EVERY VM command; use the stated `timeoutMs`; the command EXIT CODE is authoritative; absent cargo stderr is NOT a failure signal (binding).

- S1. Preconditions:
  1. `alpine-vm_vm_status` → running + SSH reachable (else STOP + user question).
  2. Host `git status` → working tree clean of TRACKED changes on `feat/wave-1-skeleton-open-image`; untracked files allowed ONLY of the workflow-artifact kinds (`.kilo/plans/20261031-task4-*.md`, `.agent/project-info/context.md` if updated). Any other drift → STOP + report state.
  3. Host `ls src` → expect exactly `app.rs, app_display-placeholder-none, image_loading.rs, main.rs, platform/`; i.e. NO `src/scaling.rs`, NO `src/app_display.rs`. If either exists → STOP: Task 4 (partially) implemented; report.
  4. Host read of `src/app.rs` → confirm the tree state of V8 (the two scoped allows present, `Panel::top/bottom` in `ui()`, `_ui` empty closure). Any mismatch → STOP + report.
- S2. Write the drafts (host file tools, LF, trailing newline, UTF-8 no BOM): `src/app_display.rs` (§3.3), `src/scaling.rs` (§3.2), REPLACED `src/app.rs` (§3.4), REPLACED `src/main.rs` (§3.1), §3.5 image_loading.rs one-line edit, §3.6 project-structure edit.
- S3. Rustfmt normalize (VM; dev-checks never applies fmt): `vm_status`, then `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` — `timeoutMs: 120000`. EBUSY-class failure (vboxsf on the share) → retry the IDENTICAL command ONCE; still failing → proceed: rely on S4's `fmt-check` output (it prints the expected formatting) and apply host-side whitespace-only fixes per that guidance (sanctioned in-draft fix category). Formatting-only reflow by fmt is expected, NOT a defect (fix2/F3 precedent) — never "fix fmt back"; only semantic content of §3 drafts is binding.
- S4. Dev-checks (VM): `vm_status`, then `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — `timeoutMs: 1200000`. EXPECTED: exit 0; per-check `PASS fmt-check exit=0`, `PASS check exit=0`, `PASS test exit=0`, `PASS clippy exit=0`; the test section shows `test result: ok. 15 passed`; final line `ALL CHECKS PASSED`.
  - Test-count arithmetic (finalized here, verify against output): 4 existing app.rs tests + 4 image_loading tests (UNCHANGED byte-set) + 2 app_display tests (§3.3) + 5 scaling tests (§3.2) = **15**. All live in the single `img-scroller` bin target ⇒ one `test result: ok. 15 passed` line. If the output shows a different count → do NOT use FORCE_FAIL to mask; record the actual count in the summary and continue ONLY if all pass.
  - First-run failure handling (same as T3 S5): (i) sanctioned in-draft micro-fixes ONLY — formatting minutiae, missing underscore-prefixed parameters, unused-import removal, `#[allow(dead_code)]` scope corrections (if clippy disputes any of the EXACT allow deltas in §1.2-d: restore the tree's byte-precise allow set for `image_loading.rs`'s three kept items and record the correction; opening_image_path's allow must stay); (ii) structural failure (dependency resolution, toolchain, eframe drift, any failure requiring redesign) → STOP + report with full captured output.
- S5. CROSS-TARGET re-check (VM; DECISION: RUN ONCE — see below command + justification). FIRST the DF-GATE: `vm_status`, then `df -h` via `alpine-vm_vm_run_command` (single command, prefix `df`); if the filesystem hosting the cargo-target volume shows LESS than 3 GiB available → STOP + report (do NOT free space yourself; threshold per latest T3 precedent fix4). Then run:
  `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu` — `timeoutMs: 1800000`; EBUSY → retry identical ONCE.
  - Justification for running although no `platform/windows.rs` change occurred: (1) `src/app.rs`, `src/main.rs`, `src/app_display.rs`, `src/scaling.rs` are target-NEUTRAL surfaces that were only compiled for the Linux target in S4 — the GNU check proves they compile for the second target inclusion too; (2) Task 5's release cross-build (`scripts/build-windows.sh`) requires a green GNU compile anyway, so an error would surface now at check-speed instead of at release-build speed; (3) the GNU target dir is warm since T3 → single incremental-check command, cheap.
  - Expected: exit 0. If errors appear ONLY in code written by this task's drafts (app.rs/app_display.rs/scaling.rs/main.rs/image_loading.rs) apply at most ONE fix round from the sanctioned in-draft list of S4(i); anything else (any windows.rs error — no windows.rs content changed, so an error there is T3-regression territory — dependency/feature issues, lockfile mismatch) → STOP + report with full captured output.
- S6. Git hygiene + commit per §5.
- S7. Return structured summary: files created/modified (full-verbatim drafts landed), VM commands with exit codes, dev-checks per-check lines + `test result: ok. 15 passed` + `ALL CHECKS PASSED`, cross-target check result + df reading, (record if the fallback fmt path or any in-draft fix category was used, with output lines), commit hash. Evidence lives in MCP captured outputs + the summary — do NOT create evidence files in the repo. Operator-run manual UX items (see §6, Task-5 additions) stay OPERATOR-PENDING — recorded, never executed by agents.

## 5. Commit points and messages (gitignore compliance BEFORE the commit)

Committer: implementer (4.2), after S5 succeeds.

Commit 1 — Task 4 implementation (single commit):
- Files EXACTLY (6): `src/main.rs`, `src/app.rs`, `src/app_display.rs`, `src/scaling.rs`, `src/image_loading.rs`, `.agent/project-structure.md`
- Message EXACT: `Display decoded image fit-scaled in the viewer window`

Procedure (Gitignore Compliance Rule):
1. Read `.gitignore` — confirm `dist/`, `logs/`, `*.log`, `target/` entries exist (do NOT modify the file).
2. `git status` — only the six listed modified/new paths appear (+ the expected untracked workflow artifacts); NO `logs/checks/*.log` (gitignored) listed.
3. Stage ONLY: `git add src/main.rs src/app.rs src/app_display.rs src/scaling.rs src/image_loading.rs .agent/project-structure.md` (NO `git add .`).
4. Re-run `git status` — staged set matches exactly; NO gitignored path, NO TODO file, NO `.kilo/plans/*`, NO `.agent/project-info/*` staged.
5. Commit with the exact message above.
6. `git status` clean afterward. NO push (restricted to Step 5, `origin` only). NO branch ops. NO `CHANGELOG.md`. NO edits to existing `.kilo/plans/*` files (this plan is a NEW file per the naming rule; no Cargo.toml/Cargo.lock change happens in Task 4 — nothing else to stage).

## 6. Frontend-spec and TODO verification mapping (reading list for 4.5a / 4.5b)

### 6.1 Frontend-spec §9 static checklist → draft location

| Frontend-spec item | Location | Status |
| --- | --- | --- |
| Private texture-handle field (or wrapper) + keeps `decoded_image` | §3.4 `current_display: CurrentImageDisplay` wrapper field (spec §4 explicitly allows "a small wrapper around it"); `decoded_image` kept | Direct pass |
| Upload exactly once on successful decode | §3.3 `accept_decoded_image` (flag set ONLY in `accept_open_outcome` Ok arm) + `ensure_current_image_texture` (guarded, self-clearing) + §3.3 test `successful_upload_happens_exactly_once_per_decoded_image` | Direct pass (adaptation-A4 semantics below) |
| `ui` reads the texture and paints inside `CentralPanel` | §3.4 CentralPanel closure → `paint` → `egui::Image::new(...).fit_to_exact_size(...)` | Direct pass |
| `decoded_image == None` ⇒ empty CentralPanel | §3.4 `if let Some` guard — no widgets added | Direct pass |
| Fit math in a pure helper module using the current available rect | §3.2 `scaling::fitted_size` called with `panel_ui.available_size()` INSIDE the paint (per-frame) | Direct pass |
| Uniform scale, centered, whole image fits | §3.2 formula + §3.2 tests 1–4; centering by `Layout::centered_and_justified` (§1.2-a/c, adaptation A6) | Direct pass |
| `ColorImage::from_rgba_unmultiplied` used | §3.3 ensure step (straight RGBA, V2) | Direct pass |
| No texture upload / `ColorImage` construction / pixel copy inside the un-guarded `ui` path | Single upload guarded by the pending flag; no pixel clones/copies anywhere (pixels passed by reference into `from_rgba_unmultiplied`, which moves data ONCE per decode into the texture) | Adaptation A4 (read-through) |
| No zoom/pan/fit-width/fit-height/fullscreen/navigation code | Absent from all drafts (mandate f) | Direct pass |
| `Open…` text + `UNSUPPORTED_FILE_MESSAGE` unchanged | §3.4 (bytes unchanged) | Direct pass |
| `#[allow(dead_code)]` on `decoded_image` removed; `opened_image_path` handled per §5.4 | §3.4/§1.2-d (removed / kept-with-reason per the spec's own §5.4 conditional) | Direct pass (§5.4 branch taken) |
| File/method size, depth, privates, self-documenting rules | §2 table + §3 drafts | Direct pass |

Encoded adaptations (do NOT "fix" back; each carries the corresponding spec tolerance/deferral):
- A4 (spec §4.4 "ui never calls load_texture" + §5.5 "texture field read-only inside ui" + §6.1): the literal no-upload-in-ui statement is satisfied through the CALLEE (`ensure_current_image_texture` inside `CurrentImageDisplay`), guarded by the pending flag so the upload COUNT equals the decode COUNT — the spec's own testable INTENTS (§4.2 "only when a new successful decode", §6.1 "no per-frame re-upload", §6.2/6.3 no pixel copies / no image-proportional per-frame allocations) all hold; §4.2's "(or equivalent)" covers the mechanism. Verified-masked failure mode of the mandate's alternative (`Option<(u32,u32)>` dims fingerprint) was the reason for the bool pattern (§1.2-b) — the bool IS the "texture-matches" pattern the mandate sanctioned.
- A5 (spec §5.2 "open_image … creates the ColorImage and TextureHandle"): the open flow now FLAGS instead of creating; creation happens once on the next paint (the ctx-plumbing problem stated in the caller mandate; decided at 4.1b, spec §7 defers exact construction surfaces to this step).
- A6 (spec §2.4 centering formula): centering is performed by `egui::Layout::centered_and_justified(egui::Direction::TopDown)` (verified 0.37 constructor; `Layout::centered()` does not exist — V5/V6); the offset math is egui's `Align2(CENTER, CENTER)` within the justified full-area frame (trace V6). No second scaling helper; no egui paint calls in scaling.rs (spec §2.1 upheld).
- A7 (field naming): wrapper field `current_display` with inner `image_texture` — the concrete `egui::Image` call uses wrapper-internal `&TextureHandle`. Spec §5.1 "exact field name is a local detail" + §4 "small wrapper" clause cover this.

### 6.2 TODO Task 4 sub-items + Task-5 additions → 4.5b

| TODO item | Verify in 4.5b by | Pass state |
| --- | --- | --- |
| Render decoded image scaled to fit initial view | §3.2/§3.3/§3.4 read-through + operator item below | Static content match |
| Resize keeps display sane | No cached scale/rect/transform state exists; fit recomputed from `panel_ui.available_size()` every paint (§1.1 item 2 evidence) | Static content match |
| Dev-checks pass via harness | S4 captured output: `ALL CHECKS PASSED` + `test result: ok. 15 passed` | Evidence lines exist |
| Unit tests architector-defined, container-runnable | S4: 15 tests ran on the container target; OS-bound code remains untested (decision g) | Evidence |
| Windows-target compile | S5 exit 0 on `x86_64-pc-windows-gnu` (+ df ≥ 3 GiB gate recorded) | Evidence |
| Frontend-spec §9 checklist | §6.1 table read-through (A4–A7) | All rows pass |
| Operator-run manual UX (Task-5 additions from Task 4) | Recorded as operator-pending, never agent-executed: (i) opened jpg/png DISPLAYS in the window, whole image visible, fit-scaled and centered (CLI arg open and dialog open); (ii) resizing the window keeps the image visible and centered (no crop, no offset); (iii) after an unsupported-file message, the previous image remains visible behind the message bar | Recorded |
| Workflow 4.6 marker duty | Implementer flips both Task 4 checkboxes `[ ]`→`[x]` in `20261029-todo-3.md` (content preserved), then commits | Explicit follow-up; not an S-step |

## 7. Risks and handling

| # | Risk | Mitigation / handling |
| --- | --- | --- |
| R1 | egui 0.37 `Image`/`TextureHandle`/`Layout` API shape drift | Mitigated by design: every named call (V1–V7) verified from the resolved 0.37.0 sources before drafting, with the sizing call (`fit_to_exact_size`) and the centering mechanism (`centered_and_justified`) explicitly cited; S4 compiles the drafts in-container; S5 re-checks the GNU target. Any compile error NOT fixable within the S4(i)/S5 sanctioned categories → STOP + full output (never re-design ad hoc). |
| R2 | vboxsf EBUSY on container fmt | S3 decision tree: identical retry ONCE → then fmt-check-output-guided host whitespace fixes (known pattern from T3 fix2 R3/F3). Never invent additional file-writing powers. |
| R3 | Test-count drift | Encoded arithmetic = 15 (8 pre-existing + 2 + 5). Actual count from S4 output recorded in the summary; masked failures via FORCE_FAIL forbidden. |
| R4 | f32 precision loss for pixel dims > 2^24 (u32→f32) | Accepted v1: same conversion happens inside egui's `ColorImage`/texture sizing path; affects only gigantic images, display-only. `pixel_buffer_size` (usize math) still guards the decode-side invariant. |
| R5 | app.rs Max-Lines violation after fmt | Measured ~160/200 with margin (§2). Ifunexpected fmt growth pushes a file over 200 lines: remove ONLY redundant `///` doc comments (the per-field/per-method docs marked optional in §3.3/§3.4) — one line at a time, re-run S3; if still over → STOP + report (do NOT invent further modules; the mandate's one-module allowance is consumed by §3.3). |
| R6 | GUI runtime not validatable agent-side | Container is compile/test/lint only; display behavior validated OPERATOR-RUN (§6.2 items). Recorded as operator-pending, never failed. |
| R7 | Pending-flag invariant corruption | Impossible by construction: `accept_decoded_image` (the only `true` setter) is called exclusively from the `accept_open_outcome` Ok arm (decoded_image always `Some` when pending); the Err arm touches nothing; `ensure_current_image_texture` (the only `false` setter) clears only after a successful upload. Tests pin both transitions. |
| R8 | MCP capture may omit cargo stderr / VM unavailability | Exit code authoritative (binding); `vm_status` before every VM command; VM/share unavailable → STOP + user question. |

## 8. Out of scope (explicit)

- Zoom in/out, pan, fit-width, fit-height, nearest/linear sampling toggles, fit-mode switching (Wave 4).
- Fullscreen (later wave); folder/image navigation keys and buttons (Wave 2+); window-title change to the opened filename (deferred; also the reason `opened_image_path` keeps its allow).
- Any "Open with..." menu; any per-frame caching of scaled pixels; any bundled-image-decode path.
- README/`tech.md`/`architecture.md`/`context.md` updates (Task 6); CHANGELOG; TODO-file checkbox flips beyond §3.4 note; edits to existing `.kilo/plans/*`; git push; branch operations.
- No `Cargo.toml`/`Cargo.lock` change (no new dependency — egui reached via `eframe::egui`, V9); no `src/platform/**` change of any byte.

DECISION SUMMARY: replace `src/app.rs` per §3.4, add `src/app_display.rs` (§3.3, wrapper + lazy upload via pending bool) and `src/scaling.rs` (§3.2, one pure fit helper), two mod-line additions in `src/main.rs` (`app_display`, `scaling` — §3.1), remove exactly one `#[allow(dead_code)]` in `src/image_loading.rs` (§3.5), update `project-structure.md` (§3.6); dev-checks expect `test result: ok. 15 passed` / `ALL CHECKS PASSED`; ONE cross-target GNU re-check behind a 3 GiB df-gate; single 6-file commit `Display decoded image fit-scaled in the viewer window`; STOP rules per §4/§7.
