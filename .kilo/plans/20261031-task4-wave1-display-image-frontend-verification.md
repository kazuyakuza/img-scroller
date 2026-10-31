# Task 4 — Front-end Implementation Verification Report (4.5a)

**Task:** Wave 1, Task 4 — Display the decoded image in the central surface  
**Date:** 2026-10-31  
**Commit verified:** `2141e47` (`Display decoded image fit-scaled in the viewer window`)  
**Branch:** `feat/wave-1-skeleton-open-image`  
**Verifier role:** Front-end specialist (egui/eframe), static read-only review

---

## 1. Spec §9 Checklist

| # | Frontend-spec §9 item | Verdict | Evidence (`file:line`) |
|---|-----------------------|---------|------------------------|
| 1 | `ImagesScrollerApp` has a private texture-handle field (or wrapper) and keeps `decoded_image` | **PASS** | `src/app.rs:22` `current_display: CurrentImageDisplay` (wrapper allowed by spec §4/§5.1); `src/app.rs:19` `decoded_image: Option<DecodedImage>` retained |
| 2 | `open_image` (or its helper) uploads the texture exactly once on successful decode | **ADAPTED-PASS** (A4/A5) | `src/app.rs:56` sets pending flag only in Ok arm; `src/app_display.rs:53-66` uploads once when pending and self-clears; test pins identity at `src/app_display.rs:88-106` |
| 3 | `ui` reads the texture handle and uses it to paint the image inside `CentralPanel` when `decoded_image` is `Some` | **PASS** | `src/app.rs:91-95` CentralPanel closure; `src/app_display.rs:32-48` `paint` reads `self.image_texture` and calls `ui.add(Image::new(...))` |
| 4 | When `decoded_image` is `None`, `CentralPanel` remains empty (no placeholder) | **PASS** | `src/app.rs:92-94` `if let Some(...)` guard adds nothing when `None`; closure body otherwise empty |
| 5 | Fit math lives in a pure helper module/file and uses the current available central rect | **PASS** | `src/scaling.rs:10-21` `fitted_size` pure helper, no egui paint/texture code; `src/app_display.rs:37-38` called with `panel_ui.available_size()` |
| 6 | The scale is uniform and centers the image; the whole image fits inside the available rect | **PASS** | `src/scaling.rs:23-27` `scale_factor = min(width_ratio, height_ratio)`; `src/app_display.rs:42-47` centers via `Layout::centered_and_justified(TopDown)` + `fit_to_exact_size(display_size)` |
| 7 | `ColorImage::from_rgba_unmultiplied` (or equivalent documented straight-RGBA path) is used | **PASS** | `src/app_display.rs:59` `egui::ColorImage::from_rgba_unmultiplied(image_size, &decoded_image.pixels)` |
| 8 | No texture upload, `ColorImage` construction, or pixel copy occurs inside `ui` | **ADAPTED-PASS** (A4) | Upload lives in `CurrentImageDisplay::ensure_current_image_texture` (`src/app_display.rs:53-66`), guarded by pending flag; `ui` calls `paint`, which calls the guarded ensure. No pixel clones/copies in `ui` path |
| 9 | No zoom, pan, fit-width/fit-height, fullscreen, or navigation code exists | **PASS** | Grep for `zoom\|pan\|fullscreen\|fit_width\|fit_height\|navigate\|navigation\|arrow\|keyboard\|key_press` in `src/` returned only unrelated matches ("navigation" in doc comments, `paint`/`panel_ui` identifiers). No such feature code |
| 10 | The `Open…` button text and `UNSUPPORTED_FILE_MESSAGE` wording are unchanged | **PASS** | `src/app.rs:66` `"Open…"`; `src/image_loading.rs:8-9` message byte-identical |
| 11 | `#[allow(dead_code)]` removed from `decoded_image`; `opened_image_path` handled per §5.4 | **PASS** | `src/app.rs:19` no allow on `decoded_image`; `src/app.rs:16` allow kept on `opened_image_path` with Wave 2 rationale; `src/image_loading.rs:11` struct allow removed; three remaining allows in `image_loading.rs` (`Unsupported`, `user_message`, `pixel_buffer_size`) byte-kept per §1.2-d |
| 12 | File/method size, depth, private-by-default, and self-documenting rules respected | **PASS** | File lines: `app.rs` 158, `app_display.rs` 107, `scaling.rs` 95, `main.rs` 28, `image_loading.rs` 86 — all ≤200. Largest bodies (`paint` ~17 lines, `ensure_current_image_texture` ~14 lines, `accept_open_outcome` ~12 lines) ≤50. Max nesting = 2. New fields/helpers private or `pub(crate)`. Descriptive names used throughout |

**Adaptation summary:** Items 2 and 8 are passed through adaptations A4–A5 documented in the implementation plan §6.1 and the amendment: the open flow sets a pending flag instead of calling `load_texture` directly (because `Context` is unavailable in the CLI-open path), and the first paint frame performs the single upload. The upload count still equals the successful-decode count; no per-frame re-upload occurs.

---

## 2. Surface Verification

- **Empty state unchanged:** `src/app.rs:91-95` — `CentralPanel` closure adds no widgets when `decoded_image` is `None`. Background remains the default panel background.
- **With image:** `src/app.rs:93` calls `self.current_display.paint(panel_ui, decoded_image)`; `src/app_display.rs:32-48` performs the guarded texture ensure, fit-size compute, and centered image widget add.
- **Toolbar + message bar untouched:** `src/app.rs:81-89` retains the top `Open…` toolbar and conditional bottom `message_bar` exactly as in Task 3; text unchanged.

---

## 3. Fit Verification

- **Formula:** `src/scaling.rs:23-27` computes `width_ratio` and `height_ratio`, returns `min(...)`. `src/scaling.rs:20` applies `image_size * scale_factor`. Uniform scaling by construction.
- **None-guards:** `src/scaling.rs:14-19` rejects non-positive image or available dimensions.
- **Per-frame recompute:** `src/app_display.rs:37-38` calls `scaling::fitted_size(panel_ui.available_size(), ...)` inside `paint` every frame. No cached scale, rect, or transform state exists in any struct.
- **Centering:** `src/app_display.rs:42-47` uses `egui::Layout::centered_and_justified(egui::Direction::TopDown)` around a single `Image::new(...).fit_to_exact_size(display_size)`, matching the 0.37-verified two-axis centering trace (implementation plan V5/V6).

---

## 4. Texture Lifecycle Verification

- **Pending flag set only on success:** `src/app.rs:56` `self.current_display.accept_decoded_image()` is inside the `Ok(decoded_image)` arm only.
- **Upload once per decode:** `src/app_display.rs:53-66` checks `texture_upload_pending`; if true, builds `ColorImage`, calls `ctx.load_texture`, assigns the new handle, and clears the flag. Subsequent frames return immediately.
- **Straight RGBA:** `src/app_display.rs:59` `ColorImage::from_rgba_unmultiplied(...)`.
- **Linear filtering:** `src/app_display.rs:63` `egui::TextureOptions::default()` (equals `LINEAR`).
- **Static texture name:** `src/app_display.rs:9` `const CURRENT_IMAGE_TEXTURE_NAME: &str = "current_image"`.
- **Failure leaves texture untouched:** `src/app.rs:58-60` Err arm only sets `message_text`; `decoded_image`, `opened_image_path`, and `current_display` are not modified, so the previous texture remains.
- **No per-frame upload in `ui`:** The only texture-upload call is inside `ensure_current_image_texture`, which is gated by `texture_upload_pending`. The `ui` path calls `paint` → `ensure`, but ensure is a no-op after the first post-decode frame.

---

## 5. Performance Guard Verification

- **No per-frame re-upload:** Confirmed above; upload gated by pending flag.
- **No per-frame pixel copies:** `src/app_display.rs:59` passes `&decoded_image.pixels` by reference into `from_rgba_unmultiplied`; the pixel `Vec` is owned by `DecodedImage` and never cloned/sliced/transformed in `ui`.
- **No image-proportional allocations in `ui`:** Fit math uses scalar arithmetic only. The only allocation in the paint path is the texture upload itself, which happens once per decode.
- **Size invariant:** `DecodedImage` payload invariant (`pixel_buffer_size`) ensures `pixels.len() == width * height * 4`; the `[width as usize, height as usize]` size passed to `from_rgba_unmultiplied` cannot trigger its assert.

---

## 6. Out-of-Scope Respect

No zoom, pan, fit-width/fit-height, fullscreen, navigation keys, title-change, or "Open with..." code exists anywhere in `src/`. This matches spec §7 and implementation-plan mandate f.

---

## 7. `#[allow(dead_code)]` Deltas

| Location | Before | After | Verdict |
|----------|--------|-------|---------|
| `src/app.rs` `decoded_image` | `#[allow(dead_code)]` | removed | PASS |
| `src/app.rs` `opened_image_path` | `#[allow(dead_code)]` | kept, doc updated to Wave 2 rationale | PASS per spec §5.4 branch |
| `src/image_loading.rs` `DecodedImage` struct | `#[allow(dead_code)]` | removed | PASS |
| `src/image_loading.rs` `Unsupported`, `user_message`, `pixel_buffer_size` | `#[allow(dead_code)]` | kept byte-identical | PASS per implementation plan §1.2-d |

---

## 8. Sanctioned Deviations Read-Through

The implementation follows the approved adaptations A4–A7 and the amendment canonical fixes:

1. **A4 — Pending-flag upload pattern:** Spec §4.4/§5.5 literally says "no upload in `ui`" / "texture field read-only in `ui`". The implementation moves the upload into `CurrentImageDisplay::ensure_current_image_texture`, called from `paint` (which is called from `ui`), but guarded so the upload count equals the decode count. This is the sanctioned "texture-matches bool" pattern from the mandate.
2. **A5 — Open flow flags instead of creating texture:** Spec §5.2 says open creates the texture. Because `Context` is not available during CLI-open (before event loop) and `accept_open_outcome` has no `ctx`, the open flow sets `texture_upload_pending = true`; the next paint uploads once. Sanctioned by implementation plan §1.2-b.
3. **A6 — Centering via egui layout:** Spec §2.4 gives an explicit offset formula. The implementation uses `Layout::centered_and_justified(TopDown)` + `fit_to_exact_size`, which the implementation plan verified resolves to the same two-axis centered placement in egui 0.37.0 (trace V6). Sanctioned by implementation plan §1.2-a.
4. **A7 — Wrapper field naming:** Spec §5.1 says exact field name is a local detail; spec §4 explicitly allows "a small wrapper around it". The implementation uses `current_display: CurrentImageDisplay` with inner `image_texture: Option<egui::TextureHandle>`. Sanctioned by implementation plan §1.2-b.
5. **Amendment — module-scope `image_dimensions`:** The base-plan draft placed `image_dimensions` inside the `impl` block; the amendment moved it to module scope. `src/app_display.rs:70-72` confirms the module-scope free function; call site in `paint` (`src/app_display.rs:38`) uses it without `Self::`.
6. **Amendment — identity equality without `Debug`:** The base-plan test used `assert_eq!`, which requires `Debug`; egui 0.37 `TextureHandle` does not implement `Debug`. The amendment changed it to `assert!(display.image_texture == Some(uploaded_texture));` (`src/app_display.rs:105`), preserving the identity-equality pin via `PartialEq`.
7. **Amendment — `src/platform/windows.rs` fmt-restoration:** Commit diff shows only a 3-line whitespace reflow of the `windows::Win32::Graphics::Imaging` import list (`src/platform/windows.rs:12-16`). No semantic change. Sanctioned formatting-restoration deviation.

All deviations are pre-approved and documented. No additional structural deviations were found.

---

## 9. Diffs: Spec vs Implementation

Expected differences are exactly A4–A7 plus the amendment canonical fixes listed in §8. After reviewing every file in the commit, no other spec deviation was found.

| Aspect | Spec expectation | Implementation | Disposition |
|--------|------------------|----------------|-------------|
| Texture upload trigger | `open_image` creates texture directly | Pending flag set in Ok arm; upload on first paint | ADAPTED-PASS (A4/A5) |
| Centering mechanism | Explicit offset formula | `centered_and_justified` + `fit_to_exact_size` | ADAPTED-PASS (A6) |
| Texture field shape | `Option<TextureHandle>` in app | `CurrentImageDisplay` wrapper with `image_texture` + `texture_upload_pending` | ADAPTED-PASS (A7) |
| `image_dimensions` helper | Not specified | Module-scope free fn | AMENDMENT canonical fix |
| Texture identity test | `assert_eq!` | `assert!(... == Some(...))` | AMENDMENT canonical fix |
| `windows.rs` | Unchanged | 3-line import-list fmt reflow | AMENDMENT sanctioned fmt restoration |

**No fix steps required** for front-end structure or behavior.

---

## 10. Quality Issues for Later Tasks

| # | Issue | Location | Disposition |
|---|-------|----------|-------------|
| 1 | **One-frame stale image on dialog open.** When a new image is opened from the in-app dialog, `accept_decoded_image` sets the pending flag during the same `ui` frame whose `CentralPanel` paint has already completed. The old texture remains assigned until the next frame, so the previous image is visible for one additional frame before the new upload replaces it. | `src/app.rs:56`, `src/app_display.rs:27-29`, `src/app_display.rs:53-66` | **Acceptable for v1.** The pending-flag pattern is the sanctioned adaptation (A4/A5). The first paint after the decode uploads and displays the new image. No fix required now; note for future UX polish if desired. |

No other front-end quality issues identified.

---

## 11. Operator-Run Pending Items

The following require manual runtime validation on Windows and are **not** executed by this agent:

1. Open a supported image via CLI argument — verify it displays fit-scaled and centered in the central panel.
2. Open a supported image via the in-app `Open…` dialog — verify the same.
3. Resize the window — verify the image remains whole, visible, and centered without cropping or offset drift.
4. Open an unsupported file after a successful load — verify the message bar appears and the previous image remains visible behind it.

These items are recorded for Task 5 operator-run verification.

---

## 12. Final Verdict

**VERIFICATION PASSED (4 adapted items: A4, A5, A6, A7; plus 3 amendment canonical fixes).**

The implementation at commit `2141e47` conforms to the Task 4 front-end technical specification with only the pre-approved adaptations and amendment fixes. The checklist, surface behavior, fit math, texture lifecycle, performance guard, out-of-scope boundaries, and project-rule constraints are all satisfied. No front-end code changes are required.

---

**Report path:** `.kilo/plans/20261031-task4-wave1-display-image-frontend-verification.md`
