# Task 4 — Front-end Technical Specification: Display the Image in the Central Surface

**Date:** 2026-10-31  
**Wave:** Wave 1 — Project Skeleton and Open Image  
**Scope boundary:** Task 4 only (rendering the decoded image in the existing central surface with whole-image fit). Zoom, pan, fit-width/fit-height, fullscreen, and navigation keys are explicitly out of scope.  
**Target implementer restriction:** Junior developer under 50% authority — no architectural or scope decisions are left to the implementer.

## 1. Display surface

**Decision:** the existing `egui::CentralPanel` in `ImagesScrollerApp::ui` becomes the image area. When no image is loaded the central panel stays exactly as Task 3 left it (blank, filled with the default panel background). When `decoded_image` is `Some`, the decoded image is painted inside the central surface so that the whole image is visible and fit-scaled. The top `Open…` toolbar and the conditional bottom message bar remain unchanged and continue to reduce the central panel's available area.

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 1.1 | Central panel is the image surface | `egui::CentralPanel::default().show(...)` is the only surface that renders the image. |
| 1.2 | Empty state is unchanged | When `decoded_image` is `None`, the `CentralPanel` closure adds no widgets, labels, or placeholders. |
| 1.3 | Image is drawn when available | When `decoded_image` is `Some` and a matching texture handle exists, the image is rendered inside the central panel. |
| 1.4 | Whole image is visible | The painted image rectangle is fully contained in the available central rect; no part of the image is clipped or off-screen. |
| 1.5 | Fit-scaled | The image is scaled uniformly (same factor on both axes) so the entire image fits the available central rect. |
| 1.6 | Toolbar/message bar unchanged | The top `Open…` toolbar and conditional bottom message bar keep their Task-3 behavior, sizing, and text. |
| 1.7 | No central UI chrome | No additional buttons, labels, scroll bars, or frames are added inside the central panel in this task. |

## 2. Initial fit behavior

**Decision:** simple whole-image fit. The image is scaled by the largest uniform factor that keeps the entire image inside the available central rectangle, then centered on both axes. The background is the central panel's default background; no custom letterbox color is introduced.

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 2.1 | Fit math is isolated | The scale/rectangle calculation lives in a pure helper module (e.g., `src/scaling.rs`) and contains no egui paint calls or texture code. |
| 2.2 | Scale factor | `scale = min(available_width / image_width, available_height / image_height)`, clamped to `>= 0`. |
| 2.3 | Uniform scaling | The same scale is applied to width and height; the image aspect ratio is preserved. |
| 2.4 | Centered | The painted rectangle is centered in the available central rect on both axes: `offset = (available_size - scaled_image_size) / 2`. |
| 2.5 | Positive dimensions | The helper may assume `image_width > 0` and `image_height > 0`; zero-dimension images are not expected because the decoding seam already rejects empty buffers. |
| 2.6 | No letterbox color | The area around the fitted image is filled by the default `CentralPanel` background style; no explicit background rectangle or tint is painted for the image. |

## 3. Resize behavior

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 3.1 | Recompute every frame | The fit rectangle is computed from the *current* available central rect each frame; no cached transform, scale, or rectangle from a previous frame is reused without recalculation. |
| 3.2 | No manual clipping | No explicit clipping is added; with correct fit math the image rectangle never exceeds the central rect. |
| 3.3 | Window resize stays sane | Resizing smaller or larger keeps the whole image visible and centered; the display never crops the image or leaves it offset outside the window. |

## 4. Texture lifecycle

**Decision:** the app owns an `Option<egui::TextureHandle>` (or a small wrapper around it) and replaces it on every successful decode. The old handle is dropped by assignment. The texture is uploaded exactly once per `DecodedImage`; the `ui` method only reads the handle.

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 4.1 | Straight RGBA upload | The upload path uses `egui::ColorImage::from_rgba_unmultiplied([width, height], &pixels)` because `DecodedImage::pixels` is straight (non-premultiplied) RGBA8. |
| 4.2 | Upload once per decode | `ctx.load_texture(...)` (or equivalent) is called only when a new successful decode produces a new `DecodedImage`; it is not called every frame. |
| 4.3 | Replace-on-open | On successful open the previous texture handle is replaced by the new one; no manual cache invalidation or texture-atlas management is required. |
| 4.4 | No per-frame upload | The `ui` method never calls `load_texture`, `from_rgba_unmultiplied`, or any other texture-upload function. |
| 4.5 | Linear filtering | Texture filtering uses the default egui linear sampling (`TextureOptions::default()` / `TextureOptions::LINEAR`) unless the implementer documents a different choice. |
| 4.6 | Texture keyed by a static name | A single static texture name (e.g., `"current_image"`) is used; the handle is stored in the app struct, not recreated by name each frame. |
| 4.7 | Failure leaves texture untouched | On `ImageLoadFailure` the existing texture handle and decoded state remain unchanged, so the previous image stays visible behind the message bar. |

## 5. State / struct seams

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 5.1 | Add a texture handle field | `ImagesScrollerApp` gains a private field holding the current image texture handle (e.g., `current_image_texture: Option<egui::TextureHandle>`). The exact field name is a local detail. |
| 5.2 | Upload triggered by `open_image` | `open_image` (or a small private helper it calls) creates the `ColorImage` and `TextureHandle` when `load_image` returns `Ok`, then stores it in the new field. |
| 5.3 | Keep decoded payload | `decoded_image` remains owned by the app struct; its `width`/`height` are used for fit math. |
| 5.4 | Remove the dead-code allow on `decoded_image` | The `#[allow(dead_code)]` attribute on `decoded_image` is removed because Task 4 reads it. The `#[allow(dead_code)]` on `opened_image_path` may be removed if the implementation reads it in this task; otherwise it may stay until later navigation code reads it. |
| 5.5 | No texture upload in `ui` | The texture field is read-only inside `ui`; mutation happens only in the open flow. |

## 6. Performance guard

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 6.1 | No per-frame re-upload | The texture upload call is absent from the per-frame `ui` path. |
| 6.2 | No per-frame pixel copies | The pixel `Vec<u8>` is not cloned, re-sliced, or transformed inside `ui`. |
| 6.3 | No per-frame allocations in `ui` | `ui` performs no allocations proportional to image size; fit math uses only scalar/float arithmetic. |
| 6.4 | Size invariant | Overflow/underflow in fit math is avoided by relying on the existing `DecodedImage` payload invariant and positive-dimension assumption. |

## 7. Out of scope / not specified

The following are explicitly deferred:

- Zoom in/out, pan, fit-width, fit-height (Wave 4).
- Fullscreen mode (later wave).
- Horizontal/vertical image/folder navigation keys and UI buttons (Wave 2+).
- Changing the window title to the opened filename.
- Any "Open with..." menu.
- Exact egui widget vs. low-level painter API used to draw the textured rectangle.
- Exact field name for the texture handle and exact helper function signatures in the scaling module.

## 8. Project-rule constraints that shape the GUI code

| Rule | How it applies to Task 4 |
|------|--------------------------|
| `max-lines-per-file` (≤200 lines; ideally ≤125 substantive) | `app.rs` may grow, but image display logic is delegated to small helpers (`ensure_texture`, `fit_rect`, `paint_image`). |
| `max-lines-per-method` (≤50 lines) | `ui`, `open_image`, and any new helpers each stay ≤50 lines of body. |
| `max-depth` (≤2 nested blocks) | Extract any third-level nesting (e.g., texture creation, fit math) into private helpers. |
| `prefer-private-members` | New texture field and helpers are private; only `new`, `ui`, and `open_image` remain public. |
| `self-documenting-code` | Use names like `current_image_texture`, `fit_scale`, `image_display_rect`. |
| `single-section-boolean-conditions` | Any multi-section `if` is extracted into a named helper returning `bool`. |
| `no-commented-code` | No commented-out code is left in source files. |

## 9. Static verification checklist for 4.5a

The following must be verifiable by reading the code only (no build/run):

- [ ] `ImagesScrollerApp` has a private texture-handle field (or wrapper) and keeps `decoded_image`.
- [ ] `open_image` (or its helper) uploads the texture exactly once on successful decode.
- [ ] `ui` reads the texture handle and uses it to paint the image inside `CentralPanel` when `decoded_image` is `Some`.
- [ ] When `decoded_image` is `None`, `CentralPanel` remains empty (no placeholder).
- [ ] Fit math lives in a pure helper module/file (e.g., `src/scaling.rs`) and uses the current available central rect.
- [ ] The scale is uniform and centers the image; the whole image fits inside the available rect.
- [ ] `ColorImage::from_rgba_unmultiplied` (or an equivalent documented straight-RGBA path) is used.
- [ ] No texture upload, `ColorImage` construction, or pixel copy occurs inside `ui`.
- [ ] No zoom, pan, fit-width/fit-height, fullscreen, or navigation code exists.
- [ ] The `Open…` button text and `UNSUPPORTED_FILE_MESSAGE` wording are unchanged.
- [ ] `#[allow(dead_code)]` is removed from `decoded_image`; `opened_image_path` is handled per §5.4.
- [ ] File/method size, depth, private-by-default, and self-documenting rules are respected.

## 10. Acceptance criteria

1. Opening a supported image (CLI or dialog) renders it in the central panel, scaled to fit and centered.
2. Resizing the window keeps the whole image visible and centered at all times.
3. Opening an unsupported file after a successful load shows the message bar but leaves the previous image visible.
4. The texture is uploaded once per successful open; no per-frame upload/allocation churn is present in `ui`.
5. The top toolbar and bottom message bar keep their Task-3 behavior.
6. Container dev-checks and tests pass (evidence comes from the implementer/architector verification step).
