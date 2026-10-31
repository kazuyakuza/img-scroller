# Amendment 3 — Task 3 Implementation Plan: windows.rs cross-target (S6) compiler findings — byte-exact final replacement

- Date: 2026-10-31
- Amends: base plan [`20261031-task3-wave1-open-image-implementation.md`](20261031-task3-wave1-open-image-implementation.md) **§2.1.5 (`src/platform/windows.rs`) ONLY**. Prior amendments stand: [`fix.md`](20261031-task3-wave1-open-image-fix.md) (target-scoped manifest — in tree), [`fix2.md`](20261031-task3-wave1-open-image-fix2.md) (`mod windows;`/`mod stub;` + `egui::Panel` call sites — in tree).
- Workflow step: Critical Workflow 4.1b MINI AMENDMENT #3 (Task 3), resume 4.2 at step S6.
- Plan approval: AUTO-APPROVED ("Approve Global and Tasks Plans", 2026-10-30). Save + return path; no user approval round.
- Committer (4.2) resumes ONLY at this amendment's resume instructions (§6). No other plan deliverable and NO unrelated file changes.

## 1. Verified evidence summary

Implementer ran S6 (`docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu`): **exit 101, 14 errors — ALL in `src/platform/windows.rs`**; the Linux target builds clean (S5 dev-checks: `ALL CHECKS PASSED`, `test result: ok. 8 passed`). Error classes (3) beyond the base-plan S6 six-category micro-fix list, plus the 3 in-list categories:

| # | Finding | Verified against windows-0.62.2 / raw-window-handle-0.6.2 (2026-10-31) |
| --- | --- | --- |
| V1-a | `WINCODEC_ERR_COMPONENTNOTFOUND` and `RPC_E_CHANGED_MODE` do NOT live in `Graphics::Imaging` / `System::Com` (draft import sites) — both are `pub const …: HRESULT` in **`Win32::Foundation`** | microsoft.github.io/windows-docs-rs constant pages, windows 0.62.2 |
| V1-b | `converter.Initialize` scopes `None::<&IWICPalette>` → type `IWICPalette` must be imported from `Win32::Graphics::Imaging` | `IWICFormatConverter::Initialize<P0, P3>` page: `P3: Param<IWICPalette>` |
| V1-c | `PWSTR::to_string()` yields `Result<String, FromUtf16Error>` (draft's `file_name.map(PathBuf::from)` does not type-check) | windows-strings 0.5.1 `PWSTR` page (windows-core 0.62.2 dep graph) |
| V2 | `HasWindowHandle::window_handle` returns `Result<WindowHandle<'_>, HandleError>` — NOT an `Option`; draft `frame.window_handle()?` in an `Option`-returning fn is invalid | raw-window-handle 0.6.2 `HasWindowHandle` page; `WindowHandle::as_raw() -> RawWindowHandle`; `Win32WindowHandle.hwnd: NonZeroIsize` (`.get()` + `HWND(*mut c_void)` cast valid) |
| V3 | `map_err(classify_failure)` sites (×6) pass `windows::core::Error` **by value**; draft classifier took `&Error` → type mismatch at every site | windows-core 0.62.2 `Error` page (`code(&self) -> HRESULT` works on an owned value) |
| V4 | `CopyPixels` takes **3 args** in 0.62: `(prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8])` — draft's 4-arg form (`None, stride, len, ptr`) is wrong; `byte_length as u32` cast is obsolete (allocation length is owned by `vec![0u8; byte_length]`) | `IWICBitmapSource::CopyPixels` page (surfaced on `IWICBitmapFrameDecode`/`IWICFormatConverter` via `Deref<target = IWICBitmapSource>`) |

Also verified for the fix-3 mapping: `Error::from_hresult(code: HRESULT) -> Error` exists (windows-core 0.62.2) and `E_FAIL` is `pub const E_FAIL: HRESULT` in `Win32::Foundation`.

## 2. S5 sanctioned fixes — CANONICAL, already in the tree, NO action

Two in-draft fixes the implementer applied under base-plan S5 category (i) and now recorded verbatim as canonical state (any future plan reads THESE as the reference content):

1. `src/app.rs` test module: title assertion reads `assert_eq!(super::APP_WINDOW_TITLE, "Images Scroller");` (was `ImagesScrollerApp::APP_WINDOW_TITLE` in base §2.1.3 C3 — same constant, path-valid inside the struct test module).
2. `src/image_loading.rs`: scoped `#[allow(dead_code)]` placed on EXACTLY four items — `pub(crate) struct DecodedImage` (line above the struct), the `Unsupported` enum variant, the `user_message` method (with its existing `///` doc comment), and the `pixel_buffer_size` function (with its existing `///` doc comment) — exactly as the tree currently holds (read 2026-10-31). These remain until Task 4 removes them (base §8 bindings unchanged).

## 3. EXACT final `src/platform/windows.rs` (FULL file — the ONLY source change this amendment authorizes)

Replace the complete file (host file tools; LF endings, trailing newline, UTF-8 no BOM). Incorporates ALL six patch items; otherwise identical to base §2.1.5. Layout is rustfmt-compatible (matching the wrapping conventions `cargo fmt` accepted in the current tree):

```rust
//! Windows implementation of the platform facade: native open dialog and WIC
//! decoding. Compiled only on Windows targets; first-compile validation is the
//! cross-target check step of this task's implementation.

use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::{
    GENERIC_READ, HWND, RPC_E_CHANGED_MODE, S_FALSE, S_OK, WINCODEC_ERR_COMPONENTNOTFOUND,
};
use windows::Win32::Graphics::Imaging::{
    CLSID_WICImagingFactory, GUID_WICPixelFormat32bppRGBA, IWICBitmapDecoder, IWICBitmapFrameDecode,
    IWICFormatConverter, IWICImagingFactory, IWICPalette, WICBitmapDitherTypeNone,
    WICBitmapPaletteTypeCustom, WICDecodeMetadataCacheOnDemand,
};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree,
};
use windows::Win32::UI::Shell::{
    FOS_FILEMUSTEXIST, FOS_FORCEFILESYSTEM, FOS_PATHMUSTEXIST, FileOpenDialog, IFileOpenDialog,
    IShellItem, SIGDN_FILESYSPATH,
};
use windows::core::{Error, PCWSTR};

use crate::image_loading::{DecodedImage, ImageLoadFailure, pixel_buffer_size};

pub(crate) fn show_open_image_dialog(frame: &eframe::Frame) -> Option<PathBuf> {
    if !ensure_com_initialized() {
        return None;
    }
    run_open_dialog(owner_window(frame)).ok()
}

pub(crate) fn decode_image(path: &Path) -> Result<DecodedImage, ImageLoadFailure> {
    if !ensure_com_initialized() {
        return Err(ImageLoadFailure::Unloadable);
    }
    decode_with_factory(path)
}

fn ensure_com_initialized() -> bool {
    let com_state = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    match com_state {
        S_OK | S_FALSE | RPC_E_CHANGED_MODE => true,
        _ => false,
    }
}

fn owner_window(frame: &eframe::Frame) -> Option<HWND> {
    match frame.window_handle().ok()?.as_raw() {
        RawWindowHandle::Win32(handle) => Some(HWND(handle.hwnd.get() as *mut _)),
        _ => None,
    }
}

fn run_open_dialog(owner: Option<HWND>) -> Result<PathBuf, Error> {
    unsafe {
        let dialog: IFileOpenDialog =
            CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER)?;
        let options =
            dialog.GetOptions()? | FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST;
        dialog.SetOptions(options)?;
        dialog.Show(owner)?;
        selected_dialog_path(dialog)
    }
}

unsafe fn selected_dialog_path(dialog: IFileOpenDialog) -> Result<PathBuf, Error> {
    let item: IShellItem = dialog.GetResult()?;
    let wide_path = item.GetDisplayName(SIGDN_FILESYSPATH)?;
    let file_name = wide_path
        .to_string()
        .map_err(|_| Error::from_hresult(windows::Win32::Foundation::E_FAIL))?;
    CoTaskMemFree(Some(wide_path.as_ptr().cast::<core::ffi::c_void>()));
    Ok(PathBuf::from(file_name))
}

fn decode_with_factory(path: &Path) -> Result<DecodedImage, ImageLoadFailure> {
    unsafe {
        let factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)
                .map_err(classify_failure)?;
        let frame = frame_from_path(&factory, path)?;
        decoded_image(&factory, &frame)
    }
}

unsafe fn frame_from_path(
    factory: &IWICImagingFactory,
    path: &Path,
) -> Result<IWICBitmapFrameDecode, Error> {
    let file_path: Vec<u16> = path.as_os_str().encode_wide().chain([0]).collect();
    let decoder: IWICBitmapDecoder = factory
        .CreateDecoderFromFilename(
            PCWSTR(file_path.as_ptr()),
            None,
            GENERIC_READ,
            WICDecodeMetadataCacheOnDemand,
        )
        .map_err(classify_failure)?;
    decoder.GetFrame(0)
}

unsafe fn decoded_image(
    factory: &IWICImagingFactory,
    frame: &IWICBitmapFrameDecode,
) -> Result<DecodedImage, ImageLoadFailure> {
    let mut width = 0u32;
    let mut height = 0u32;
    frame
        .GetSize(&mut width, &mut height)
        .map_err(classify_failure)?;
    let byte_length = pixel_buffer_size(width, height).ok_or(ImageLoadFailure::Unloadable)?;
    let converter: IWICFormatConverter =
        factory.CreateFormatConverter().map_err(classify_failure)?;
    converter
        .Initialize(
            frame,
            &GUID_WICPixelFormat32bppRGBA,
            WICBitmapDitherTypeNone,
            None::<&IWICPalette>,
            0.0,
            WICBitmapPaletteTypeCustom,
        )
        .map_err(classify_failure)?;
    let mut pixels = vec![0u8; byte_length];
    let row_stride = width.checked_mul(4).ok_or(ImageLoadFailure::Unloadable)?;
    frame
        .CopyPixels(std::ptr::null(), row_stride, &mut pixels)
        .map_err(classify_failure)?;
    Ok(DecodedImage {
        width,
        height,
        pixels,
    })
}

fn classify_failure(error: Error) -> ImageLoadFailure {
    match error.code() {
        WINCODEC_ERR_COMPONENTNOTFOUND => ImageLoadFailure::Unsupported,
        _ => ImageLoadFailure::Unloadable,
    }
}
```

Shorthand patch map (all six; applied in §3 above):
1. Foundation import gains `RPC_E_CHANGED_MODE` + `WINCODEC_ERR_COMPONENTNOTFOUND` (S_OK/S_FALSE/GENERIC_READ/HWND were already there); the `System::Com` import DROPS `RPC_E_CHANGED_MODE`.
2. `owner_window`: `frame.window_handle().ok()?.as_raw()` (`?` must land first on the `Result`, `.ok()` demotes `HandleError` to `None`).
3. `selected_dialog_path`: `wide_path.to_string()` mapped via `Error::from_hresult(windows::Win32::Foundation::E_FAIL)` (self-contained, no E_FAIL import added), then `Ok(PathBuf::from(file_name))`; `CoTaskMemFree` stays AFTER the string materialization (borrow ends before the free).
4. `classify_failure(error: Error)` — BY VALUE (all six `map_err(classify_failure)` sites consume an owned `Error`).
5. `decoded_image`: `frame.CopyPixels(std::ptr::null(), row_stride, &mut pixels)` — 3-arg form, `row_stride` stays `u32`, `byte_length` still comes from `pixel_buffer_size` and feeds only `vec![0u8; byte_length]`.
6. `None::<&IWICPalette>` in `Initialize` kept; `IWICPalette` added to the `Graphics::Imaging` import (the ONE picked alternative — do NOT fully qualify instead).

## 4. Unchanged-semantics confirmation (explicit; verify during 4.3/4.5b)

- Facade signatures unchanged: `show_open_image_dialog(frame: &eframe::Frame) -> Option<PathBuf>` and `decode_image(path: &Path) -> Result<DecodedImage, ImageLoadFailure>` — no signature edits, only body/import corrections.
- `ImageLoadFailure` semantics unchanged: `WINCODEC_ERR_COMPONENTNOTFOUND → Unsupported`, everything else → `Unloadable` (§3 classifier).
- `DecodedImage` payload unchanged: straight RGBA8 `{ width: u32, height: u32, pixels: Vec<u8> }`; NO byte-swapping code added (conversion stays inside WIC via `GUID_WICPixelFormat32bppRGBA`).
- Dialog behavior unchanged: no `SetFileTypes` (default "All files"), same `FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST` options, null-owner fallback (`owner_window` → None on non-Win32/failed handle), dialog errors of `Show` → `.ok()` → `None`, `CoTaskMemFree(Some(...))` retained.
- COM init pattern unchanged: `CoInitializeEx(None, COINIT_APARTMENTTHREADED)` tolerating `S_OK | S_FALSE | RPC_E_CHANGED_MODE`.
- `src/platform/mod.rs` (fix2 §3), `src/app.rs` (fix2 §4 + §2 item 1), `src/image_loading.rs` (§2 item 2), `src/main.rs`, `src/platform/stub.rs`, `Cargo.toml` (fix §1), `Cargo.lock`, `.agent/project-structure.md`: stand AS THE TREE HOLDS THEM — this amendment touches NO other file.

## 5. Risks unique to this amendment

| # | Risk | Handling |
| --- | --- | --- |
| G1 | Implementer "helpfully" re-writes windows.rs beyond §3 (e.g. refactors, adds comments, renames `handle`) | FORBIDDEN: §3 is byte-exact content; ANY deviation = amend/review failure. Do not touch any other file. |
| G2 | The re-run fails AGAIN in `windows.rs` | HARD STOP + report with the full captured output — NO additional fix rounds are authorized by this amendment. |
| G3 | `df` output < 3 GiB on `/` | STOP before the compile (df-gate); report; do NOT free disk space yourself. |
| G4 | EBUSY-class error on the compose invocation | Retry the IDENTICAL command ONCE (base R1 rule); second failure → STOP + report. |

## 6. Resume instructions for 4.2 (single verification round, then commit)

- W0. Preconditions (re-run): `alpine-vm_vm_status` → running + SSH reachable (else STOP + user question). Host `git status` → expect the fix2 §8 tree state PLUS `?? .kilo/plans/20261031-task3-wave1-open-image-fix3.md`; any OTHER drift → STOP + report. Confirm `src/platform/windows.rs` still holds the S3 draft (141 lines) — the S6-failing content.
- W1. Replace `src/platform/windows.rs` (host file tools) with the EXACT §3 full file. Read the file back host-side and verify: Foundation import block matches item 1; `owner_window` uses `.ok()?`; `selected_dialog_path` uses the `from_hresult(E_FAIL)` map; `CopyPixels` is the 3-arg form; `classify_failure` takes `Error` by value.
- W2. DF-GATE: `alpine-vm_vm_status`, then `df -h` via `alpine-vm_vm_run_command` (single command, prefix `df`); if `/` available < 3 GiB → STOP + report (G3).
- W3. SINGLE re-run (`timeoutMs: 1800000`; EBUSY → identical retry ONCE per G4): `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu`. Expected: exit 0.
  - Exit 0 → W4. Exit non-0 with errors in `windows.rs` → STOP + report with full captured output (G2). Errors OUTSIDE `windows.rs` → STOP + report.
- W4. Base-plan S7 commit VERBATIM (gitignore-compliance procedure, §4 file list = EXACT 9 files: `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/app.rs`, `src/image_loading.rs`, `src/platform/mod.rs`, `src/platform/windows.rs`, `src/platform/stub.rs`, `.agent/project-structure.md`; message EXACT: `Open images via CLI argument or native dialog with WIC decode`). Read `.gitignore` before staging; NO `git add .`; NO staging of `.gitignore`-matching paths (logs/target/dist), NO context.md/TODO/plan-file staging; `git status` clean after; NO push, NO branch ops, NO `CHANGELOG.md`, NO edits to existing `.kilo/plans/*`.
- W5. Base-plan S8 summary VERBATIM, plus record: amendment 3 applied (windows.rs §3 replacement; §2 canonical fixes stand); NO further plan-file edits; operator-run UX items stay OPERATOR-PENDING (Task 5).

## 7. Verification mapping for 4.5b (delta only)

- S6 evidence now = the W3 re-run showing exit 0 on `x86_64-pc-windows-gnu` (replaces the base-plan S6 first-run evidence line; the failed first run is recorded as amendment-3 evidence).
- `windows.rs` content = §3 byte-exact (read-back check).
- Everything else in base §6 unchanged.

DECISION: replace base §2.1.5 draft content with §3's verified byte-exact file; single S6 re-run under the 3 GiB df-gate; STOP permanently on any residual `windows.rs` failure; proceed to the unchanged S7 commit + S8 summary on success.
