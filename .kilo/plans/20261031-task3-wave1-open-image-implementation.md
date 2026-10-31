# Implementation Plan — Wave 1, Task 3: Open Image (CLI argument + native open dialog + WIC decode + message surface)

- Date: 2026-10-31
- Workflow step: Critical Workflow §4.1b (Analysis & Planning) — Task 3 of TODO
  [`.agent/todos/20261029/20261029-todo-3.md`](../../.agent/todos/20261029/20261029-todo-3.md) ("### Task 3: Open Image — CLI argument and in-app open dialog")
- Role: Architector. Plan ONLY — this file is the step deliverable. No application code written in this step; no cargo/docker runs that modify the repo (research/verification reads only).
- Executing branch (already created in Step 2): `feat/wave-1-skeleton-open-image` — NO branch operations, NO push in this task.
- Plan approval: AUTO-APPROVED ("Approve Global and Tasks Plans", 2026-10-30). Save and return the path; do NOT ask for approval.
- Front-end input (workflow 4.1 requirement, Task 3 IS front-end related): the 4.1a Front-end Technical Specification is IN FORCE as input: [`.kilo/plans/20261031-task3-wave1-open-image-frontend-spec.md`](20261031-task3-wave1-open-image-frontend-spec.md) — §5 state shape, §6 paint contract, §8 static checklist for 4.5a, §9 deferrals settled here. §5 of this plan maps the spec to the drafts; the three encoded adaptations are listed there and do NOT require re-opening the spec.
- Global plan in force (do NOT modify): [`.kilo/plans/20261030-wave1-skeleton-open-image.md`](20261030-wave1-skeleton-open-image.md) — §0.2 build environment, §2 Task 3, §3 risks.
- Task-2 plan bindings consumed verbatim (do NOT re-decide): [`.kilo/plans/20261031-task2-wave1-scaffold-window-shell.md`](20261031-task2-wave1-scaffold-window-shell.md) §1.1 (windows@=0.62.2 + 6 features; end of §1.1-h "NO third cfg arm"), §8 (single decision chain for Task 3).
- Target implementer: JUNIOR developer, 50% restriction — every structural/naming/scope decision is encoded here; no judgment calls left behind.

## 0. Verified facts (research, 2026-10-31; dates cited per fact)

| # | Fact | Source |
| --- | --- | --- |
| V1 | `windows` crate max stable version = **0.62.2** → the Task-2 pin holds with NO drift | crates.io API `https://crates.io/api/v1/crates/windows` (`"max_stable_version":"0.62.2"`), read 2026-10-31 |
| V2 | The `windows` crate crate-root is `#![cfg(windows)]` → on the Linux container host target it compiles to an EMPTY crate (all items gated out). A plain `[dependencies] windows` declaration therefore does NOT break dev-checks (`cargo check/test/clippy` on `x86_64-unknown-linux-gnu`). Only the cross-target invocation compiles the real API. | windows-rs source `crates/libs/windows/src/lib.rs` (master, read 2026-10-31) |
| V3 | eframe 0.37.0 normal-depends on `raw-window-handle = ^0.6.2` (and `winit = 0.30.13` with `rwh_06`); `eframe::Frame` implements `HasWindowHandle` (non-Wasm) exposing `frame.window_handle()`; `Frame::winit_window()` returns `Option<&Arc<Window>>` | docs.rs eframe 0.37.0 Cargo.toml + `eframe::Frame` API pages, read 2026-10-31 |
| V4 | In `windows` 0.62, the dialog stack lives in `Win32::UI::Shell` (NOT a separate `Shell::Common` requirement): `FileOpenDialog` coclass, `IFileOpenDialog`, `IShellItem`, `SIGDN_FILESYSPATH`, `FOS_*`; `IFileOpenDialog::Show` takes `hwndowner: Option<HWND>`; `IShellItem::GetDisplayName(SIGDN) -> Result<PWSTR>` | windows-rs docs site (`/doc/windows/Win32/UI/Shell/struct.IFileOpenDialog.html`, `struct.IShellItem.html`), read 2026-10-31 |
| V5 | `GENERIC_READ` / its type `GENERIC_ACCESS_RIGHTS` live in `Win32::Foundation` — NO `Win32_Storage_FileSystem` or other extra feature is needed for `CreateDecoderFromFilename` | windows-rs docs (`/doc/windows/Win32/Foundation/struct.GENERIC_ACCESS_RIGHTS.html`), read 2026-10-31 |
| V6 | `CreateDecoderFromFilename` (windows-rs 0.62): `(wzfilename: Param<PCWSTR>, pguidvendor: *const GUID, dwdesiredaccess: GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeMetadataCacheOption) -> Result<IWICBitmapDecoder>`; `CreateFormatConverter() -> Result<IWICFormatConverter>` on the same factory | windows-rs docs (`IWICImagingFactory_Impl` pages), read 2026-10-31 |
| V7 | windows-rs interfaces surface INHERITED methods directly (e.g. `IFileOpenDialog::Show` "inherited from IModalWindow via dereferencing") → `IWICBitmapFrameDecode` reaches `GetSize`/`CopyPixels` of `IWICBitmapSource` without an explicit `.cast()` | windows-rs docs method pages ("inherited … via dereferencing" note), read 2026-10-31 |
| V8 | Current repo state: `Cargo.toml` has exactly ONE dependency `(eframe = "=0.37.0", default-features=false, 6 features)`; `src/` = `main.rs` + `app.rs` matching the Task-2 drafts verbatim; dev-checks harness + compose file unchanged since Task 1/2 | repo files read 2026-10-31 (`Cargo.toml`, `src/main.rs`, `src/app.rs`, `scripts/dev-checks.sh`, `docker-compose.yml`) |

## 1. Scope

### 1.1 In scope (TODO Task 3 sub-items, verbatim mapping)

| # | TODO sub-item (abbreviated) | Delivered by |
| --- | --- | --- |
| 1 | If launched with an image-path CLI argument, load that image; otherwise open via in-app open dialog (native Windows dialog) | §2.1 drafts (`main.rs` CLI intake; `app.rs` button → `platform::show_open_image_dialog`) |
| 2 | Decode via WIC system codecs — no bundled decoders; unregistered formats unsupported; WIC binding + src/ module layout per architector | §1.2 decisions c/d/e, §2.1 drafts (`image_loading.rs` + `platform/` facade) |
| 3 | On unsupported/unloadable file: informational message in the window (wording per architector) | §1.2 decision g (bottom bar) + §2.1 drafts (constant verbatim in §2.1.2 per Task-2 §1.1-f; Err arm in §2.1.3) |
| + | Dev-checks pass with the new unit tests; windows-only code compiles for the Windows target NOW (not deferred to Task 5) | §3 steps S5 (dev-checks), S6 (MANDATORY first cross-target check) |

### 1.2 Decided in this plan (mandate §2 a–h — settled; zero judgment calls left)

| # | Item | Decision | Value / rationale |
| --- | --- | --- | --- |
| a | CLI arg intake | `main` captures `std::env::args_os().nth(1).map(PathBuf::from)` BEFORE app construction (exact one-line form is mandated by frontend-spec §2.1); the `Option<PathBuf>` is moved into the `AppCreator` closure; if `Some`, `app.open_image(path)` runs once after construction, before the event loop. Extras ignored. | `args_os` over `args`: `args()` PANICS on non-UTF-8 (i.e. invalid-UTF-16 on Windows) command lines; Windows paths are WTF-8/UTF-16-capable in `OsStr`, so the lossless OS-string path is kept end-to-end (CLI → `PathBuf` → WIC `encode_wide`). Extras ignored: no v1 feature consumes a second positional or flag, and inventing argument semantics now would break later-wave flag compatibility — a later wave may add flags BEFORE any claiming logic exists (intentional liability). |
| b | Pure logic units + inline `#[cfg(test)]` only | Three test-bearing pure units: (1) `UNSUPPORTED_FILE_MESSAGE` exactness test; (2) `pixel_buffer_size(width: u32, height: u32) -> Option<usize>` (None on zero-dimension OR checked-overflow; used by BOTH the Windows decode path and the tests — this is the shared pure seam useful to the OS-bound side); (3) the app-state open-transition rules tested through a private `accept_open_outcome` helper plus one end-to-end `open_image` call landing on the stub's failure. NO path-normalization helper (spec has none; folder-listing logic is a later wave). NO argument-filter helper: spec §2.1 fixes the intake as a single expression; a wrapper would only duplicate a bound one-liner (decision recorded, not skipped silently). Tests stay pure-logic and runnable in the container (Task-2 decision g); the HRESULT→classification mapping and all WIC/COM code remain OS-bound and untested in the container (Task-2 decision g item 2). | |
| c | Facade design | Error type `ImageLoadFailure` (enum `{ Unsupported, Unloadable }`) is OWNED IN `src/image_loading.rs` (home decision — consumed by the app via `image_loading`, produced by both platform sides; keeps `platform` free of domain semantics). Facade signatures (IDENTICAL both sides, re-exported from `platform/mod.rs`): `pub(crate) fn show_open_image_dialog(frame: &eframe::Frame) -> Option<PathBuf>` (None = cancelled/failed dialog) and `pub(crate) fn decode_image(path: &Path) -> Result<DecodedImage, ImageLoadFailure>`. `stub_impl` (Linux fallback, so container checks compile + app tests can run): dialog → `None`; decode → `Err(ImageLoadFailure::Unloadable)` ALWAYS — this makes the failure transition testable end-to-end on the container target, while the success transition is testable via the private `accept_open_outcome` helper with a synthetic `Ok`. No third cfg arm (Task-2 §1.1-h binding). | |
| d | Payload shape + pixel format | `pub(crate) struct DecodedImage { pub width: u32, pub height: u32, pub pixels: Vec<u8> }` in `image_loading.rs` (exact frontend-spec §5 shape; effective field visibility is crate-scoped by the `pub(crate)` struct). Pixel format for the payload: **straight (non-premultiplied) RGBA8, 4 bytes/pixel** — decided NOW as the fixed interface for Task 4. Realization: the BGRA→RGBA-style swizzle is done INSIDE WIC by requesting the conversion target `GUID_WICPixelFormat32bppRGBA` in `IWICFormatConverter::Initialize` (WIC's built-in converter handles every source format, including premultiplied variants); `windows.rs` NEVER byte-swaps by hand. Rationale vs the mandate's "convert immediately in windows.rs" suggestion: same payload result, one less hand-rolled pixel loop, WIC is the trusted conversion path. Premultiplied-vs-straight question (spec §9): resolved to STRAIGHT by the `32bppRGBA` target (it is the non-premultiplied RGBA WIC format). Task 4 consumes `DecodedImage` as-is (its egui `Color32` mapping call is ITS decision — payload format is fixed here). | |
| e | WIC pipeline (platform/windows.rs) | Exact sequence (see §2.1 draft): threaded-`CoInitializeEx` → `CoCreateInstance` of `CLSID_WICImagingFactory` (plain factory, NOT CoCreateInstanceEx) → wide path via `OsStr::encode_wide()` + `PCWSTR` → `factory.CreateDecoderFromFilename(PCWSTR, None, GENERIC_READ, WICDecodeMetadataCacheOnDemand)` → `decoder.GetFrame(0)` → `frame.GetSize(&mut w, &mut h)` → guard `pixel_buffer_size(w, h)` (None → `Unloadable`) → `factory.CreateFormatConverter()` + `converter.Initialize(frame, &GUID_WICPixelFormat32bppRGBA, WICBitmapDitherTypeNone, None::<&IWICPalette>, 0.0, WICBitmapPaletteTypeCustom)` → `pixels = vec![0u8; byte_len]` + `frame.CopyPixels(None, stride, byte_len, pixels.as_mut_ptr())` → `DecodedImage`. Error mapping: classifier `failure_kind(&windows::core::Error)` → `error.code() == WINCODEC_ERR_COMPONENTNOTFOUND` ⇒ `Unsupported` (surface signalling "no codec registered", i.e. the brief's unregistered-format case) ⇒ otherwise `Unloadable`. **Feature gates used = EXACTLY the Task-2 pinned 6 (`Win32_Foundation`, `Win32_Graphics_Imaging`, `Win32_System_Com`, `Win32_UI_Shell`, `Win32_UI_Shell_Common`, `Win32_UI_WindowsAndMessaging`) — NO additive features required** (V4/V5: dialogs + `GENERIC_READ` resolve under Foundation/Shell; `Win32_UI_Shell_Common` + `Win32_UI_WindowsAndMessaging` are carried per the Task-2 baseline binding — verbatim expansion of the pinned list is forbidden, and dropping/pinning would itself be a plan-gated change of that binding; both are harmless cargo features today and cover dialog-adjacent types for later waves). | |
| f | Dialog (platform/windows.rs) | `IFileOpenDialog` via `CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER)`; options: `dialog.SetOptions(dialog.GetOptions()? | FOS_FORCEFILESYSTEM | FOS_FILEMUSTEXIST | FOS_PATHMUSTEXIST)`; **NO `SetFileTypes` call — the dialog keeps its default "All files" behavior (any file selectable)**: per the brief, ONLY WIC decides format support, and an unregistered selection then yields the message flow instead of a pre-filtered list (spec §3 constant). A WIC registered-formats list is NOT built. Owner: `Some(HWND)` extracted from `frame` via `raw_window_handle` (`RawWindowHandle::Win32(win32) ⇒ HWND(win32.hwnd.get() as *mut _)`), else `None` — null-owner fallback (Task-2 decision e binding). Result: `GetResult()` → `IShellItem::GetDisplayName(SIGDN_FILESYSPATH)` → `PWSTR::to_string()` → `PathBuf`, then `CoTaskMemFree(Some(...))` on the shell-allocated string. Returned `Ok`/`Err`: ANY error result of `Show` (including ERROR_CANCELLED) maps to `None` = cancelled (spec §1.6: no state change, no message on cancel; other dialog failures are surfaced identically — v1 keeps ONE dialog surface). COM init strategy: `CoInitializeEx(None, COINIT_APARTMENTTHREADED)` per facade call on the calling (UI) thread, tolerating `S_OK`, `S_FALSE` (already init), and `RPC_E_CHANGED_MODE` (thread pre-initialized MTA by eframe/winit — proceed anyway; WIC and the common dialog function in either apartment; operator-run validates the real pattern). NO `CoIncrementMTAUsage`; NO `CoUninitialize` (process-lifetime COM, deliberate v1 minimalism; add only if a later wave introduces threads). | |
| g | App integration | `ui()` paint order EXACTLY per spec §6: (1) `egui::TopBottomPanel::top("open_toolbar")` with one `Open…` button in a `ui.horizontal`; (2) conditional `egui::TopBottomPanel::bottom("message_bar")` shown ONLY when `message_text` is `Some`, label = stored `&'static str`; (3) empty `egui::CentralPanel`. Pragmatic synchronous pattern (encoded, per spec sketch + desktop convention): the button handler runs `platform::show_open_image_dialog(frame)` + `self.open_image(path)` SYNCHRONOUSLY inside the click branch — the native modal dialog blocking the UI thread is the standard Windows desktop pattern; state + message land visibly on the next frame; no thread/channel/polling (spec §5.7). `open_image` is the only non-UI mutator (spec §5.1–5.3); decode runs inside `open_image` via `image_loading::load_image`, NEVER in the paint path (spec §5.4); `ui()` calls `open_image` (allowed by spec §5.4 wording) but never `load_image` directly. CLI-open happens at construction time (before the event loop). NO decode/rendering of the image in `ui()`'s central area (Task 4). | |
| h | HTML | None — desktop GUI only; no HTML artifacts of any kind in this task. | |
| + | New dependency (allowed addition, plan-gated) | `raw-window-handle = "=0.6.2"` added via `cargo add` alongside `windows`. Rationale (V3): the facade's dialog-owner plumbing needs the `HasWindowHandle` trait in scope to read the HWND from `&eframe::Frame`; eframe 0.37 requires `^0.6.2`, so exact-pinning `=0.6.2` satisfies eframe's own requirement graph (locks 0.6.2, eframe stays within `^0.6.2`). No winit direct dependency needed (we never name the `winit` type — the trait method goes straight on `Frame`). | |

### 1.3 Consumed Task-1/Task-2 bindings (restated only as used)

- Harness: `docker compose -f /img-scroller/docker-compose.yml run --rm rust …`; `vm_status` BEFORE every VM command; exit code authoritative; absent cargo stderr is NOT a failure signal; VM/share unavailable → STOP + user question.
- Dev-checks semantics: `fmt-check` → `check --locked` → `test --locked` → `clippy --locked -- -D warnings`; final `ALL CHECKS PASSED` (exit 0); log under gitignored `logs/checks/`.
- `Cargo.lock` tracked; `--locked` on every harness cargo invocation (EXCEPT the single fallback lock-materialization run in §3, caller-pre-authorized).
- Package chain unchanged: `img-scroller` 0.1.0, edition 2024, eframe pin `=0.37.0` NEVER regressed.
- Platform facade cfg pattern (Task-2 §1.1-h): `#[cfg(target_os = "windows")] mod windows_impl;` + `#[cfg(not(target_os = "windows"))] mod stub_impl;` — same signatures both sides; NEVER a windows-target-only check mode in dev-checks.
- Message wording constant (Task-2 §1.1-f): `UNSUPPORTED_FILE_MESSAGE` lives in `src/image_loading.rs`; exact string per frontend-spec §3.2.
- Task-2 deferral notes carried: `_creation_context` REMAINS underscore-prefixed (still unused in Task 3); the `#[allow(dead_code)]` on state fields is UPDATED (see §2.1.3 note C) — Task-2's note said "remove when populated", but a written-never-read field still trips `dead_code` under clippy `-D warnings`, so the scoped allows stay on `opened_image_path` AND `decoded_image` until Task 4 reads them (removal becomes a Task-4 binding).

## 2. Deliverables — full drafts (implementer materializes verbatim; LF endings, trailing newline, UTF-8 no BOM)

### 2.1.0 `Cargo.toml` — dependency additions (via `cargo add`, NOT hand-written on the primary path)

After S3 the `[dependencies]` table must read (shape check; eframe entry byte-identical to today):

```toml
[dependencies]
eframe = { version = "=0.37.0", default-features = false, features = [
    "glow",
    "default_fonts",
    "clipboard",
    "links",
    "wayland",
    "x11",
] }
raw-window-handle = "=0.6.2"
windows = { version = "=0.62.2", features = [
    "Win32_Foundation",
    "Win32_Graphics_Imaging",
    "Win32_UI_Shell",
    "Win32_UI_Shell_Common",
    "Win32_UI_WindowsAndMessaging",
    "Win32_System_Com",
] }
```

Post-add checks (each MUST hold, else STOP + report): `eframe` entry unchanged (exact `=0.37.0`, 6 features); `raw-window-handle` present with EXACT `=0.6.2`; `windows` present with EXACT `=0.62.2` and EXACTLY the six features named above; NO other new packages.

### 2.1.1 `src/main.rs` (REPLACE full file)

```rust
//! Entry point of the Images Scroller viewer.

mod app;
mod image_loading;
mod platform;

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

Semantics the implementer must be able to state back:
- `cli_image_path()` body IS frontend-spec §2.1 verbatim (`args_os().nth(1).map(PathBuf::from)`); parsed BEFORE `run_native`, moved into the `AppCreator` (`FnOnce` — legal to capture), applied right after construction (spec §2 decision text; encode this as SUPERSEDING the Task-2 §8 suggestion of passing into `new()` — the 4.1a spec is the binding front-end input).
- One arg = image path; extras ignored (§1.2-a); no dialog on CLI launch (spec §2.2); no extra success UI (spec §2.3).
- `open_image` runs once BEFORE the event loop starts → at first paint the message bar (if the file is unsupported) is already `Some`.

### 2.1.2 `src/image_loading.rs` (NEW, full file)

```rust
//! Open-image seam: decoded payload shape, failure classification, and the
//! unsupported-file message constant.

use std::path::Path;

use crate::platform;

pub(crate) const UNSUPPORTED_FILE_MESSAGE: &str =
    "This file's format is not supported: no Windows image codec is registered for it.";

pub(crate) struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

pub(crate) enum ImageLoadFailure {
    Unsupported,
    Unloadable,
}

impl ImageLoadFailure {
    /// The single informational message shown for every open failure in Wave 1.
    pub(crate) fn user_message(self) -> &'static str {
        UNSUPPORTED_FILE_MESSAGE
    }
}

pub(crate) fn load_image(path: &Path) -> Result<DecodedImage, ImageLoadFailure> {
    platform::decode_image(path)
}

/// Byte length of the RGBA8 payload for width×height pixels.
/// Returns None for empty dimensions or for sizes that overflow `usize`.
pub(crate) fn pixel_buffer_size(width: u32, height: u32) -> Option<usize> {
    if width == 0 {
        return None;
    }
    if height == 0 {
        return None;
    }
    let pixel_count = width.checked_mul(height)?;
    pixel_count.checked_mul(4).map(|byte_count| byte_count as usize)
}

#[cfg(test)]
mod tests {
    use super::{ImageLoadFailure, UNSUPPORTED_FILE_MESSAGE, pixel_buffer_size};

    #[test]
    fn unsupported_message_matches_the_agreed_wording() {
        assert_eq!(
            UNSUPPORTED_FILE_MESSAGE,
            "This file's format is not supported: no Windows image codec is registered for it."
        );
    }

    #[test]
    fn every_failure_reports_the_unsupported_message() {
        assert_eq!(
            ImageLoadFailure::Unsupported.user_message(),
            UNSUPPORTED_FILE_MESSAGE
        );
        assert_eq!(
            ImageLoadFailure::Unloadable.user_message(),
            UNSUPPORTED_FILE_MESSAGE
        );
    }

    #[test]
    fn pixel_buffer_size_matches_width_height_times_four() {
        assert_eq!(pixel_buffer_size(1920, 1080), Some(1920 * 1080 * 4));
    }

    #[test]
    fn pixel_buffer_size_rejects_empty_and_overflowing_dimensions() {
        assert_eq!(pixel_buffer_size(0, 5), None);
        assert_eq!(pixel_buffer_size(5, 0), None);
        assert_eq!(pixel_buffer_size(u32::MAX, u32::MAX), None);
    }
}
```

### 2.1.3 `src/app.rs` (REPLACE full file)

```rust
//! Application shell: the regular, resizable viewer window of Images Scroller.

use std::path::PathBuf;

use eframe::egui;

use crate::image_loading::{self, DecodedImage, ImageLoadFailure};
use crate::platform;

/// Title shown on the viewer window title bar (final app name, user-decided 2026-10-29).
pub const APP_WINDOW_TITLE: &str = "Images Scroller";

pub struct ImagesScrollerApp {
    /// Path of the last successfully decoded image (read again from Task 4's display code).
    #[allow(dead_code)]
    opened_image_path: Option<PathBuf>,
    /// Decoded pixels of the last successfully opened image (read again from Task 4's display code).
    #[allow(dead_code)]
    decoded_image: Option<DecodedImage>,
    message_text: Option<&'static str>,
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
        }
    }

    /// Opens an image synchronously: decodes now, then applies the outcome to the open-flow state.
    pub fn open_image(&mut self, path: PathBuf) {
        let outcome = image_loading::load_image(&path);
        self.accept_open_outcome(path, outcome);
    }

    /// Failure keeps any previously loaded image and state untouched except the message.
    fn accept_open_outcome(&mut self, path: PathBuf, outcome: Result<DecodedImage, ImageLoadFailure>) {
        match outcome {
            Ok(decoded_image) => {
                self.opened_image_path = Some(path);
                self.decoded_image = Some(decoded_image);
                self.message_text = None;
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
        egui::TopBottomPanel::top("open_toolbar").show(ui, |ui| {
            self.open_toolbar_button(ui, frame);
        });

        if let Some(message) = self.message_text {
            egui::TopBottomPanel::bottom("message_bar").show(ui, |ui| {
                ui.label(message);
            });
        }

        egui::CentralPanel::default().show(ui, |_ui| {});
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
        assert_eq!(ImagesScrollerApp::APP_WINDOW_TITLE, "Images Scroller");
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

Notes C (implementer must be able to state back):
- C1: `#[allow(dead_code)]` is scoped to EXACTLY `opened_image_path` + `decoded_image` with inline reasons (write-never-read until Task 4; §1.3 binding deviation from Task-2's "remove when populated" wording — Task 4 removes both).
- C2: `_creation_context` stays underscore-prefixed (still unused; Task-2 deferral note carried).
- C3: Test title assertion switched to `ImagesScrollerApp::APP_WINDOW_TITLE` (same constant, path-valid inside the struct's test module).
- C4: `failed_open…` test goes through real `open_image` → on the container target this reaches `stub_impl` → `Err(Unloadable)` → message flow (this is the reason the stub returns a deterministic failure: container-runnable OS-bound-adjacent coverage of transition 5.6).
- C5: `failure_after_success…` proves spec §5.6 verbatim (message set; `opened_image_path`/`decoded_image` untouched).
- C6: Rule compliance: largest method body = `accept_open_outcome` (12 lines) and `ui` (13 lines) ≤ 50; max nesting = 2 (closures inside `ui`, or `if` inside helper bodies — the third level was eliminated by extracting `open_toolbar_button`/`open_via_dialog` per the max-depth rule); max args = 2 non-self params everywhere; `empty_state` private; only `APP_WINDOW_TITLE`, `ImagesScrollerApp`, `new`, `open_image`, `window_options` are public (spec §7 private-members table).
- C7: The Egui click branch runs dialog + decode synchronously (§1.2-g rationale encoded).

### 2.1.4 `src/platform/mod.rs` (NEW, full file)

```rust
//! OS-bound facade for the native open dialog and image decoding.
//! Both target sides expose the same signatures; see the Task-2 plan §1.1-h.

#[cfg(target_os = "windows")]
pub(crate) use windows_impl::{decode_image, show_open_image_dialog};

#[cfg(not(target_os = "windows"))]
pub(crate) use stub_impl::{decode_image, show_open_image_dialog};

#[cfg(target_os = "windows")]
mod windows_impl;

#[cfg(not(target_os = "windows"))]
mod stub_impl;
```

### 2.1.5 `src/platform/windows.rs` (NEW, full file — Windows target only, compiled by NOT dev-checks but by step S6; first-compile policy see §7)

```rust
//! Windows implementation of the platform facade: native open dialog and WIC
//! decoding. Compiled only on Windows targets; first-compile validation is the
//! cross-target check step of this task's implementation.

use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::core::{Error, PCWSTR};
use windows::Win32::Foundation::{GENERIC_READ, HWND, S_FALSE, S_OK};
use windows::Win32::Graphics::Imaging::{
    CLSID_WICImagingFactory, GUID_WICPixelFormat32bppRGBA, IWICBitmapDecoder, IWICBitmapFrameDecode,
    IWICFormatConverter, IWICImagingFactory, WICBitmapDitherTypeNone, WICBitmapPaletteTypeCustom,
    WICDecodeMetadataCacheOnDemand, WINCODEC_ERR_COMPONENTNOTFOUND,
};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemFree,
    RPC_E_CHANGED_MODE,
};
use windows::Win32::UI::Shell::{
    FOS_FILEMUSTEXIST, FOS_FORCEFILESYSTEM, FOS_PATHMUSTEXIST, FileOpenDialog, IFileOpenDialog,
    IShellItem, SIGDN_FILESYSPATH,
};

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
    match frame.window_handle()?.as_raw() {
        RawWindowHandle::Win32(win32_handle) => Some(HWND(win32_handle.hwnd.get() as *mut _)),
        _ => None,
    }
}

fn run_open_dialog(owner: Option<HWND>) -> Result<PathBuf, Error> {
    unsafe {
        let dialog: IFileOpenDialog = CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER)?;
        let options = dialog.GetOptions()? | FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST;
        dialog.SetOptions(options)?;
        dialog.Show(owner)?;
        selected_dialog_path(dialog)
    }
}

unsafe fn selected_dialog_path(dialog: IFileOpenDialog) -> Result<PathBuf, Error> {
    let item: IShellItem = dialog.GetResult()?;
    let wide_path = item.GetDisplayName(SIGDN_FILESYSPATH)?;
    let file_name = wide_path.to_string();
    CoTaskMemFree(Some(wide_path.as_ptr().cast::<core::ffi::c_void>()));
    file_name.map(PathBuf::from)
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

unsafe fn frame_from_path(factory: &IWICImagingFactory, path: &Path) -> Result<IWICBitmapFrameDecode, Error> {
    let file_path: Vec<u16> = path.as_os_str().encode_wide().chain([0]).collect();
    let decoder: IWICBitmapDecoder = factory
        .CreateDecoderFromFilename(PCWSTR(file_path.as_ptr()), None, GENERIC_READ, WICDecodeMetadataCacheOnDemand)
        .map_err(classify_failure)?;
    decoder.GetFrame(0)
}

unsafe fn decoded_image(factory: &IWICImagingFactory, frame: &IWICBitmapFrameDecode) -> Result<DecodedImage, ImageLoadFailure> {
    let mut width = 0u32;
    let mut height = 0u32;
    frame.GetSize(&mut width, &mut height).map_err(classify_failure)?;
    let byte_length = pixel_buffer_size(width, height).ok_or(ImageLoadFailure::Unloadable)?;
    let converter: IWICFormatConverter = factory
        .CreateFormatConverter()
        .map_err(classify_failure)?;
    converter
        .Initialize(frame, &GUID_WICPixelFormat32bppRGBA, WICBitmapDitherTypeNone, None::<&IWICPalette>, 0.0, WICBitmapPaletteTypeCustom)
        .map_err(classify_failure)?;
    let mut pixels = vec![0u8; byte_length];
    let row_stride = width.checked_mul(4).ok_or(ImageLoadFailure::Unloadable)?;
    frame
        .CopyPixels(None, row_stride, byte_length as u32, pixels.as_mut_ptr())
        .map_err(classify_failure)?;
    Ok(DecodedImage { width, height, pixels })
}

fn classify_failure(error: &Error) -> ImageLoadFailure {
    match error.code() {
        WINCODEC_ERR_COMPONENTNOTFOUND => ImageLoadFailure::Unsupported,
        _ => ImageLoadFailure::Unloadable,
    }
}
```

Draft notes (binding details the implementer must be able to state back):
- `WINCODEC_ERR_COMPONENTNOTFOUND` lives in `Win32::Graphics::Imaging` (imported there, used in `classify_failure`).
- `RPC_E_CHANGED_MODE` lives in `Win32::System::Com` (imported there). `S_OK`/`S_FALSE` are `HRESULT` consts in `Win32::Foundation`; the match arms work against `windows::core::HRESULT` (same `windows_result::HRESULT` via `windows::core` re-export).
- `IWICBitmapDecoder` is imported for the `frame_from_path` type annotation. No `PWSTR` import is needed (inferred from `GetDisplayName`'s return); the `PWSTR`-internals allowance is in the §3-S6 micro-fix list.
- windows-rs `Result<T>` defaults to `windows::core::Result<T>`; the draft spells `Result<…, Error>` explicitly — keep it.
- COM objects (`IWICImagingFactory`, decoder, frame, converter, dialog, `IShellItem`) release on drop (windows-rs references are ref-counted) — no manual Release code anywhere.
- Decision h of the mandate: NO HTML/web artifacts of any kind exist in this task (desktop GUI only) — there is nothing to draft for any web surface.

### 2.1.6 `src/platform/stub.rs` (NEW, full file)

```rust
//! Non-Windows fallback of the platform facade: keeps container checks and
//! container test runs compiling and lets the app-level failure transition
//! stay testable on the Linux host target (decode always fails here).

use std::path::{Path, PathBuf};

use crate::image_loading::{DecodedImage, ImageLoadFailure};

pub(crate) fn show_open_image_dialog(_frame: &eframe::Frame) -> Option<PathBuf> {
    None
}

pub(crate) fn decode_image(_path: &Path) -> Result<DecodedImage, ImageLoadFailure> {
    Err(ImageLoadFailure::Unloadable)
}
```

### 2.1.7 `.agent/project-structure.md` (EDIT)

Current content has:
`- app.rs - eframe application shell: viewer window options, app struct, window-title constant`
`- main.rs - entry point: starts the eframe native app`
(blank)
`(no folders in src/)`

Replace the whole block from the app.rs bullet down through `(no folders in src/)` with EXACTLY:

```markdown
- app.rs - eframe application shell: viewer window options, open-flow state, top Open… button, bottom message bar, window-title constant
- image_loading.rs - open-image seam: load_image entry, DecodedImage payload shape, failure classification, unsupported-file message constant
- main.rs - entry point: CLI image-path intake, starts the eframe native app

# Folders in src/

- platform/ - OS-bound facade for the native open dialog and WIC decode: mod.rs (facade re-exports), windows.rs (Windows implementation), stub.rs (non-Windows fallback)
```

Keep everything else (header line, `# Other folders` block) untouched.

## 3. Implementation steps (for 4.2 — ordered, atomic, verifiable)

Rule: call `alpine-vm_vm_status` BEFORE EVERY VM command; use the stated `timeoutMs`; the command EXIT CODE is authoritative; absent cargo stderr is NOT a failure signal.

- S1. Preconditions: `vm_status` → running + SSH reachable (else STOP + user question). Then `git status` (host) → working tree clean on `feat/wave-1-skeleton-open-image` with NO uncommitted drift (expect NO out-of-place files; `Cargo.toml` shows only eframe). If drift → STOP + report state. `ls` confirms `src/main.rs`, `src/app.rs` present, no `src/platform/` yet, no `src/image_loading.rs` yet; if any Task-3 file already exists → STOP: already (partially) implemented; report.
- S2. Dependency add (VM; the wave's only manifest mutation), two commands, each `timeoutMs: 180000`:
  1. `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo add windows@=0.62.2 --features Win32_Foundation,Win32_Graphics_Imaging,Win32_System_Com,Win32_UI_Shell,Win32_UI_Shell_Common,Win32_UI_WindowsAndMessaging`
  2. `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo add raw-window-handle@=0.6.2`
  Verify with `docker compose -f /img-scroller/docker-compose.yml run --rm rust cat Cargo.toml` against §2.1.0 (all post-add checks). Then `git diff --stat` sees exactly `Cargo.toml` + `Cargo.lock`.
  - EBUSY failure handling (vboxsf on the named share): if the command fails with a "Text file busy"/"EBUSY"-class filesystem error → retry the IDENTICAL command ONCE. If it fails again → apply the CALLER-AUTHORIZED FALLBACK (pre-authorized for this task's flow): write the exact §2.1.0 `[dependencies]` entries by hand via host file tools, then run `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check` ONCE (NO `--locked`) to materialize the `Cargo.lock` entries, record in the summary that the fallback path was used (with output dump lines). Continue with `--locked` from S3 onward — a manifest/lock mismatch after this point is a STOP condition.
  - Any non-EBUSY failure (network, invalid feature name, version unavailable) → STOP (do NOT "correct" features/pins yourself) unless it is a plain transient network failure, in which case retry the identical command once.
- S3. Write source drafts verbatim (host file tools): `src/image_loading.rs` (§2.1.2), `src/platform/mod.rs` (§2.1.4), `src/platform/windows.rs` (§2.1.5, including the binding draft notes), `src/platform/stub.rs` (§2.1.6), REPLACED `src/app.rs` (§2.1.3), REPLACED `src/main.rs` (§2.1.1), §2.1.7 project-structure edit.
- S4. Rustfmt normalize (VM; dev-checks never applies fmt, so normalize FIRST): `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` — `timeoutMs: 120000`. EBUSY → retry identical once; then proceed — if fmt still cannot touch files, rely on the fmt-check output in S5 (it prints the expected formatting) and apply host-side whitespace fixes per that guidance (in-draft formatting fix category). This step only normalizes formatting of the §2 drafts; any semantic change discovered here is a S6-STOP matter.
- S5. Dev-checks (VM): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — `timeoutMs: 1200000`. Expected: exit 0; per-check `PASS fmt-check exit=0`, `PASS check exit=0`, `PASS test exit=0`, `PASS clippy exit=0`; `cargo test` shows `test result: ok. 8 passed` (4 app tests: title + 3 open-transition tests; 4 image_loading tests: message wording + per-variant message + buffer-size math + empty/overflow rejection); final line `ALL CHECKS PASSED`.
  - First-run failure handling: (i) trivial in-draft fixes allowed — formatting minutiae, missing underscore-prefixed parameters, unused-import removal, missing `#[allow(dead_code)]` scope corrections; (ii) `windows.rs`-specific enums/types do NOT block S5 (the file is cfg'd out on the Linux target); (iii) structural failure (dependency resolution, toolchain, eframe drift) → STOP + report with full captured output.
- S6. CROSS-TARGET FIRST COMPILE (VM; MANDATORY, decision recorded — this is where `windows.rs` gets its FIRST compile, so windows-rs errors surface NOW rather than in Task 5): `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu` — `timeoutMs: 1800000` (first compile of the whole eframe+winit+glow tree for the GNU target into `/cargo-target/x86_64-pc-windows-gnu`; several minutes are normal; artifacts land in the named volume, not the repo).
  Expected: exit 0. If errors are reported ONLY in `src/platform/windows.rs` (or files it drives), apply AT MOST ONE fix round limited to this allowed micro-fix list, then re-run S6:
  1. `Show(owner)` param shape (`Option<HWND>` vs `HWND`) — pass `None`/`Some(hwnd)` accordingly.
  2. `PWSTR` internals (`as_ptr()`/field access/`.to_string()` availability).
  3. `CopyPixels` integer widths (u32 ↔ usize casts on `row_stride`/`byte_length`).
  4. `GetSize`/`CopyPixels` inheritance not surfaceed → switch to implicit/explicit parent: `let frame: IWICBitmapSource = frame.cast()?;` (adjust affected call sites in `decoded_image`).
  5. `Param`/`None` type annotations in `converter.Initialize` (e.g. `None as _` instead of `None::<&IWICPalette>`, dropping the explicit generic).
  6. Import module-path corrections (e.g. dialog types under `Win32::UI::Shell::Common` instead of `Shell`).
  Anything beyond the list (missing type/feature, version drift, unused-feature complaints, any error outside windows.rs) → STOP + report with full output. Note: unused windows features NEVER warn; they are binding-pinned, do not remove them.
- S7. Git hygiene + commit per §4.
- S8. Return structured summary: files created/modified (full-verbatim drafts landed), VM commands with exit codes, dev-checks per-check lines + `ALL CHECKS PASSED` + `test result: ok. 8 passed`, cross-target check result, fallback usage (if any), commit hash. Evidence lives in MCP captured outputs + the summary — do NOT create evidence files in the repo. Manuscript/notes: the operator-run manual UX items (CLI open of jpg/png on Windows, dialog open on Windows, unregistered-format message on Windows) stay OPERATOR-PENDING until Task 5 — recorded, never executed by agents.

## 4. Commit points and messages (gitignore compliance BEFORE the commit)

Committer: implementer (4.2), after S6 succeeds.

Commit 1 — Task 3 implementation (single commit):
- Files: `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/app.rs`, `src/image_loading.rs`, `src/platform/mod.rs`, `src/platform/windows.rs`, `src/platform/stub.rs`, `.agent/project-structure.md`
- Message: `Open images via CLI argument or native dialog with WIC decode`

Procedure (Gitignore Compliance Rule):
1. Read `.gitignore` — confirm `dist/`, `logs/`, `*.log`, `target/` entries exist (they do; do not modify).
2. `git status` — only the listed modified/new files appear; NO `logs/checks/*.log` (gitignored) listed.
3. Stage ONLY: `git add Cargo.toml Cargo.lock src/main.rs src/app.rs src/image_loading.rs src/platform/mod.rs src/platform/windows.rs src/platform/stub.rs .agent/project-structure.md` (no `git add .`).
4. Re-run `git status` — staged set matches exactly; NO gitignored paths staged.
5. Commit with the exact message above.
6. `git status` clean afterward. NO push (restricted to Step 5, `origin` only). NO branch ops. NO `CHANGELOG.md`. NO edits to existing `.kilo/plans/*` files (this plan was created as a NEW file per the naming rule).

## 5. Frontend-spec mapping (input → implementation; reading list for 4.5a)

| Frontend-spec item | Location in drafts |
| --- | --- |
| §1.1 button labeled exactly `Open…` (U+2026) | §2.1.3 `open_toolbar_button` |
| §1.2 top toolbar (not Window/Area/modal) | §2.1.3 `ui()` first surface — `egui::TopBottomPanel::top("open_toolbar")` (ADAPTATION A1) |
| §1.3 non-resizable default panel styling | same call (no custom Frame/height; egui panel default) |
| §1.4 horizontal layout | §2.1.3 `ui.horizontal` |
| §1.5 click → dialog → `open_image(path)` | §2.1.3 `open_via_dialog` + §2.1.5 `show_open_image_dialog` (naming ADAPTATION A2) |
| §1.6 cancel → no state change, no message | §2.1.5 dialog `Ok`/`Err` → `None` → nothing happens |
| §1.7 toolbar always visible | §1.2-g paint order (no fullscreen logic in Task 3) |
| §2.1 args source line | §2.1.1 `cli_image_path` |
| §2.2 no dialog on CLI launch | §2.1.1 (no dialog call in the CLI path) |
| §2.3 no extra success UI | §2.1.1 + §2.1.3 `ui()` (central surface stays blank on success) |
| §2.4 empty-state = toolbar + blank center | §2.1.3 |
| §2.5 no central hint | §2.1.3 `CentralPanel` no widgets |
| §3.1 message in bottom panel `message_bar` | §2.1.3 (ADAPTATION A1) |
| §3.2 label text = verbatim constant | §2.1.2 constant + §2.1.3 `accept_open_outcome` uses `image_loading::UNSUPPORTED_FILE_MESSAGE` directly |
| §3.3 default body text style | §2.1.3 plain `ui.label(message)` |
| §3.4 shown only when `Some` | §2.1.3 `if let Some(message)` |
| §3.5 persists until next successful open | §2.1.3 transition (`Ok` → `message_text = None`) |
| §3.6 non-blocking/non-modal/keeps image | bottom `TopBottomPanel` only; no dimming; decode failure never clears state |
| §3.7 no OS popup for the message | constant rendered in-window; no MessageBox in any draft |
| §4.1–4.3 no keys, no Esc/Enter, default focus | no input handling anywhere in the drafts |
| §5.1–5.4 three private fields; `new` = all None; single mutator `open_image`; decode via `image_loading::load_image` outside `ui()` | §2.1.3 struct + `empty_state` + `open_image` (`ui()` calls `open_image`, never `load_image` — spec §5.4 literal) |
| §5.5 success transition | §2.1.3 `accept_open_outcome` Ok arm |
| §5.6 failure transition (message set; image state untouched) | §2.1.3 Err arm + tests (`failure_after_success…`) |
| §5.7 no threads/channels/polls | §1.2-g (synchronous pattern) |
| §5.8 payload owned by app struct | `decoded_image: Option<DecodedImage>` field |
| §6.1–6.4 paint order; empty central panel; no modal surface; no per-frame decode/platform calls | §2.1.3 `ui()` — see ADAPTATION A3 for the per-frame interpretation |
| §7 rules table | §2.1.3 note C6 compliance list |
| §8 static checklist | this table = item-by-item mapping; 4.5a reads the checklist against the drafts via this table |
| §9 deferrals | all resolved in §1.2 (c: signatures/home; d: pixel format; e: WIC sequence; f: dialog+COM; message error type = `ImageLoadFailure`) |

Encoded adaptations (do NOT "fix" back; each covered by the spec's own tolerance):
- A1: spec writes `egui::Panel::top/bottom(...)` — no `egui::Panel` type exists in egui 0.37; the equivalent standard surface is `egui::TopBottomPanel::top/bottom(...)` (identical pattern to Task-2's `update`→`ui` adaptation; both checklists read as "a top/bottom egui panel with that id").
- A2: spec sketch calls `platform::open_image_dialog()` with no args; spec §9 explicitly leaves "Signature and HWND-owner plumbing" TO THIS 4.1b → settled as `platform::show_open_image_dialog(frame: &eframe::Frame) -> Option<PathBuf>` (owner HWND extracted inside the Windows side; UI layer passes the frame it already holds).
- A3: spec's §8 checklist item "`ui()` performs no file I/O, decode, or platform dialog calls" is read with the §1.5/§6 sketch as authority: NO per-frame file I/O, decode, or platform calls on the paint path; the single user-triggered dialog call inside the button branch is the spec's own §1.5 mandated behavior (the modal dialog itself is blocking-by-design on Windows). `load_image` never appears in `ui()` — it is called only by `open_image`, per spec §5.4, which is literally satisfied.

## 6. Verification mapping (for 4.5b — Overall Plan Adherence)

| TODO Task 3 sub-item | Verify in 4.5b by | Pass state |
| --- | --- | --- |
| CLI argument → load image; otherwise in-app dialog | §2.1.1 read + §2.1.3 button handler + §2.1.5 dialog fn | Static content match |
| Decode via WIC, no bundled decoders, unregistered = unsupported | §2.1.5 pipeline reads `CreateDecoderFromFilename`/`GetFrame`/`CopyPixels`; NO `image`-crate decode path in OUR code; §2.1.2 `ImageLoadFailure::Unsupported` + message | Static content match |
| Unsupported/unloadable → informational message, wording decided | §2.1.2 constant verbatim + §2.1.3 bottom bar + `accept_open_outcome` Err arm | Constant + transitions match |
| Dev-checks (Task 5 re-run incl. this wave's tests) | S5 captured output shows `ALL CHECKS PASSED` + `test result: ok. 8 passed` | Evidence lines exist |
| `cargo test --locked` covers architector-defined unit tests | S5: the 8 planned tests ran in-container on the pure-logic side (OS-bound code untested per Task-2 §1.1-g) | Evidence |
| Windows-target compile validated THIS task (first compile policy) | S6 exit 0 on `cargo check --locked --target x86_64-pc-windows-gnu` | Evidence |
| Frontend-spec §8 checklist | §5 mapping table read-through against drafts (with A1–A3) | All rows pass |
| Operator-run manual UX items | Recorded as operator-pending (Task 5), never agent-executed | Recorded |
| Workflow 4.6 marker duty | Implementer flips Task 3 sub-item `[ ]`→`[x]` in `20261029-todo-3.md` (rest of file preserved), then commits | Explicit follow-up; not part of §3 S-steps |

## 7. Risks and handling

| # | Risk | Mitigation / handling |
| --- | --- | --- |
| R1 | vboxsf EBUSY on manifest/source writes from the container | S2 decision tree: retry identical once → then CALLER-AUTHORIZED fallback (host-side write of §2.1.0 entries + one `cargo check` without `--locked` to materialize lock entries). S4 analogous (fmt diff-guided host fixes). Never invent powers beyond these two fallbacks; anything else → STOP. |
| R2 | Lockfile drift when adding `windows` + `raw-window-handle` | Expected: `Cargo.lock` additions ONLY under windows-family crates (`windows`, `windows_core`, `windows_result`, `windows_strings`, `windows_link`, `windows_targets`, per-target import-lib crates like `windows_x86_64_gnu`/`_msvc`) + `raw-window-handle`. At S2 verify via `git diff Cargo.lock` that NO eframe/egui/winit/glow line changed (NEVER regress the `=0.37.0`-family pins; a changed eframe-family entry = STOP). The exact-pin `raw-window-handle = "=0.6.2"` satisfies eframe's `^0.6.2` so resolution keeps both sides at 0.6.2. |
| R3 | `windows.rs` API drift vs windows-rs 0.62 signatures (my drafts derive signature shapes from docs; minor bit-width/option typing can differ) | By design this file's FIRST compile is S6 (cross-target check) — catches typed mistakes NOW; the §3-S6 micro-fix list (6 categories) is the ONLY authorized repair surface; anything beyond → STOP. Dev-checks (Linux) never sees the file (V2: `windows` crate is `#![cfg(windows)]` — empty crate on Linux; the module itself is cfg'd out at `platform/mod.rs`). |
| R4 | COM init behavior differences on real Windows (eframe/winit may pre-init the thread MTA; dialogs prefer STA) | Encoded tolerant pattern (S_OK/S_FALSE/RPC_E_CHANGED_MODE all proceed); the operator-run check in Task 5 validates real behavior; the container cannot run COM at all (stub side keeps checks green). |
| R5 | Clippy `-D warnings` noise on drafts (unused imports, needless wildcards) | S5 in-draft fix class; heavily mitigated by the S4 fmt normalize pass. |
| R6 | `empty_state`/`accept_open_outcome` design smell (test-only private constructor / two-param helper) | Justified: eframe `CreationContext` is not constructible in unit tests, so state-3-field initialization + transition rules must be reachable test-side; both are private members (private-by-default rule) and ≤50-line bodies. Reviewed in 4.3. |
| R7 | MCP capture may omit cargo stderr | Exit code authoritative (binding); never interpret absent stderr as success/failure. |
| R8 | VM/share unavailable mid-task | `vm_status` before every command; STOP + user question (binding). |

## 8. Cross-task bindings (decision chain — consumed by Task 4/5 plans)

Bind to Task 4 (display):
- Consume exactly `image_loading::DecodedImage { width: u32, height: u32, pixels: Vec<u8> }` — STRAIGHT (non-premultiplied) RGBA8 bytes, row-major, `pixels.len() == width * height * 4` (invariant from `pixel_buffer_size`).
- Task 4 REMOVES the two `#[allow(dead_code)]` attributes (+ reasons) from `opened_image_path` and `decoded_image` when it adds actual reads (this supersedes Task-2's "remove when populated" phrasing).
- egui `ColorImage` upload call (e.g. `from_rgba_unmultiplied` vs `from_rgba_premultiplied`) is a TASK-4 decision; the payload is fixed straight-RGBA and must be fed accordingly.
- The Task-4 render surface stays the `CentralPanel` anchor established in `ImagesScrollerApp::ui`; fit-scale math = pure helper module (candidate `src/scaling.rs`, finalized by Task 4's 4.1b).

Bind to Task 5 (verification):
- Dev-checks re-run must stay `ALL CHECKS PASSED` (8 Task-3 tests included).
- Release cross-build (`scripts/build-windows.sh`) builds on the now-warmed `/cargo-target/x86_64-pc-windows-gnu` volume state; windows-gnu link validity (glow + windows import libs) exercised there as pre-planned.
- Operator-run list for Task 5 gains Task-3 items: CLI open jpg/png (supported + unsupported), dialog open (supported + unsupported + cancel), message bar persistence/clearing behavior on the real Windows machine; agent-side is never executed.

## 9. Out of scope (explicit)

- NO image display/rendering (no `TextureHandle`, no paint of pixels in `ui()` — Task 4).
- NO sibling-folder/sibling-image listing or navigation, NO sort order logic (later waves).
- NO arrow/Esc/Enter keyboard handling (spec §4).
- NO README/tech/architecture/context updates — Task 6; NO `CHANGELOG.md`; NO edits to existing `.kilo/plans/*` files (this file is a NEW plan file).
- NO push, NO branch operations, NO merge to `main` (Step 5).
- NO harness changes (Dockerfile/docker-compose/scripts/docs/build* — Task 1 artifacts stay untouched).
- NO tests for OS-bound COM/WIC/HRESULT-classification logic in the container (Task-2 §1.1-g); NO operator-side GUI execution by agents.
