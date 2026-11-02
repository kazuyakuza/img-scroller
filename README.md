# Images Scroller
*The app name is **Images Scroller** (final). The repository directory name `img-scroller` remains unchanged — it is the repository name only, not the app name.*

## Overview

Images Scroller is a Windows desktop image viewer built in Rust. It recreates the behavior of the classic Windows "gallery"-era image viewer: open one image and navigate horizontally across the sibling images in the same folder, and vertically across the sibling folders under the same parent folder. Both axes are driven by arrow keys and by on-screen UI buttons. Opening any image — from Explorer, a file manager, or the in-app open dialog — immediately enables this navigation in its location, with no folder pre-registration or setup of any kind.

## Problem Statement

Modern Windows image viewers lack the classic gallery-style scrolling behavior: after opening an image, the user cannot move to the next/previous image or jump to sibling folders with simple arrow keys in the intended way. Existing alternative viewers that attempt this behavior require the user to pre-register or pre-add the folder to the application before browsing — added friction.

The consequence is that browsing a library of image folders (for example, a photo archive organized as `parent\folder1`, `parent\folder2`, ...) requires manual navigation or a file manager instead of direct in-viewer scrolling.

Therefore, this project provides a viewer that treats any opened image as the entry point into its folder tree, with zero folder setup.

## v1 Features

The v1 scope is exactly the seven features below, as defined in the project brief ([.agent/project-info/brief.md](.agent/project-info/brief.md)); the brief holds the full detail for each one.

1. **Open Image with System-Registered Formats** — opens any image whose format is registered as a codec in the Windows system (e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed); the image opens in a regular resizable viewer window.
2. **Horizontal Scroll (Sibling Images)** — left/right arrow keys and the floating bottom pill's ←/→ buttons navigate to the previous/next sibling image in the same folder; the scope is strictly the current folder (not recursive); the order is alphanumeric — names compare case-insensitively, with an exact-character tie-break when names differ only by case; at the first/last image the viewer stays on it (no wrap-around). The floating bottom pill appears while an image is open or while an empty sibling folder is being browsed; its ←/→/↑/↓ arrows are drawn as painted vector triangles (never font glyphs — they render identically on any Windows system regardless of installed fonts), and the up/down folder arrows navigate to the previous/next sibling folder under the same parent (each arrow is enabled exactly when its direction resolves a destination).
3. **Vertical Scroll (Sibling Folders)** — up/down arrow keys and up/down UI buttons navigate to the previous/next sibling folder under the same parent folder; when moving to a sibling folder, the target is (a) the folder's session last-viewed image if the folder was already visited this session (RAM-only), otherwise (b) ALWAYS the target folder's first image in alphanumeric order — no other fallback; the app also remembers the last viewed image per visited folder for the current session (RAM-only, destroyed on app close — no persistence to disk in v1). An image-less sibling folder is NOT skipped: navigation enters it in an empty-folder browsing state — the open image is released, the blank viewer surface shows the centered "Empty folder" message, and the up/down controls keep moving between sibling folders from there (no wrap-around; at the first/last sibling folder the viewer stays).
4. **Fullscreen Mode** — default OFF at launch: the window opens normal or maximized exactly at the user's own placement/size (the feature never changes the launch behavior). When OFF, the floating bottom pill and all other UI controls are visible and the image takes almost all of the central space. Toggling shares ONE action between the F11 keyboard shortcut and the pill's painted fullscreen-square button; the first press turns fullscreen ON — the window header/borders and ALL UI controls (the pill included) disappear and only the image shows, while the arrow keys keep navigating images/folders; F11 is the ONLY exit while ON (the pill is not rendered then). Fullscreen toggling never touches the zoom or fit state.
5. **Zoom In/Out and Fit Modes** — zoom in/out via the keyboard (`I`, alias Shift+`+`; `O`, alias Shift+`−`) and via the pill's [+][−] buttons; every step multiplies the zoom by 1.25 within fixed bounds (1.0–8.0); a step attempted at a bound is rejected with the pill message `Zoom max` / `Zoom min` (a step that merely lands on the bound clamps silently); zooming in enlarges the image's center — the overflow is not drawn (pan is Wave 5). The `0` key and the pill's [restore-fit] button return to whole-image fit, and EVERY successful image open resets to whole-image fit at ×1.0 as well. "Fit width" (`W` key or the pill's fit-width button) scales the image so its width fills the window (it may overflow vertically); "fit height" (`H` key or the pill's fit-height button) scales it so its height fills the window (it may overflow horizontally); choosing a fit mode resets the zoom to ×1.0, zooming keeps the active mode, the pill announces a non-default mode once (`Fit width` / `Fit height` — whole-image fit is silent), and both fit buttons are disabled while an empty folder is browsed. Zoom and fit state are session-only (no persistence in v1).
6. **Pan and Navigate at Image Edges While Zoomed** — when the zoomed image overflows the window, the arrow keys pan the image first, and navigation (images horizontally, folders vertically) triggers only when panning reaches the image's edge; when the image fits entirely, the arrows navigate directly.
7. **Fast "Open with..." Menu** — a menu entry lists the programs the Windows system has registered for the currently viewed file's type, and selecting one opens the current file with it; the list is built from system registration data at runtime.

| Control | Keyboard | On-screen (bottom pill) |
| --- | --- | --- |
| Navigate previous/next image | Left / Right arrow | [←] / [→] arrow buttons |
| Navigate previous/next sibling folder | Up / Down arrow | [↑] / [↓] arrow buttons |
| Toggle fullscreen | F11 (only exit while ON) | fullscreen square button |
| Zoom in / zoom out | I (alias Shift+Plus) / O (alias Shift+Minus) | [+] / [−] buttons |
| Restore whole-image fit | 0 | restore-fit button |
| Fit width / fit height | W / H | fit-width / fit-height buttons |

The pill renders while an image is open or while an empty sibling folder is browsed; every button is enabled exactly when its action resolves a destination (at the zoom bounds and without an open image the respective buttons paint disabled).

Additional scope notes:

- v1 sorting is alphanumeric for both folders and files; a future settings feature will let the user change it.
- Features explicitly deferred from v1 (for example rotation, slideshow, image editing, printing, persistence of the last-viewed memory across sessions, and a sort-order settings UI) are listed in the brief's out-of-scope section ([.agent/project-info/brief.md](.agent/project-info/brief.md)).
- Per the brief, the on-screen UI layout remains subject to later definition by the user; the current floating-bottom-pill realization (including the Wave-4 control groups) is the landed baseline.

## Technology Stack

The choices below are confirmed and implemented for the Wave-1 open-image slice, the Wave-2 horizontal-navigation slice, the Wave-3 vertical-navigation slice, and the Wave-4 fullscreen/zoom/fit slice; the remaining v1 features (pan-then-navigate at the image edges while zoomed, "Open with...") are still open — see [Project Status](#project-status) and [.agent/project-info/architecture.md](.agent/project-info/architecture.md).

| Component | Choice | Notes |
| --- | --- | --- |
| Language | Rust | Implemented: Cargo package `img-scroller` 0.4.0, edition 2024. |
| GUI | egui/eframe | Implemented: `eframe` =0.37.0 (glow renderer); pure Rust, lightweight. |
| Image decoding | Windows Imaging Component (WIC) system codecs | Implemented via the `windows` crate =0.62.2 (Windows-target-scoped); no bundled decoders. |
| Target platform | Windows 10 and Windows 11 | Runtime/GUI validation is operator-run on a real Windows machine. |
| Target architectures | x64 and x86 builds | x64 cross-build landed (Wave 1); x86 belongs to Wave 7. |

- Formats are supported when a system codec exists; formats not registered in the system are not supported in v1 — the accepted constraint of relying on system codecs.

## Distribution (Planned)

- Zip-style portable installer: the user picks a destination folder; the package unzips the application binary AND its config file into that destination.
- The package includes a helper and/or guidance for registering the app as the default image viewer (file associations) in Windows.
- The helper's implementation details (script vs. executable, exact mechanism) are deferred to implementation.

## Project Status

Wave 1 landed (2026-10-31, merged to `main`): the Cargo project exists, and the viewer opens images via a CLI argument or the in-app open dialog, decodes them through the WIC system codecs, and displays them fit-scaled in the regular resizable window. Wave 2 landed (branch `feat/wave-2-horizontal-scroll`, version 0.2.0): horizontal sibling-image navigation — Left/Right arrow keys move to the previous/next image of the opened image's folder in case-insensitive alphanumeric order. A 0.2.1 UI fix (2026-11-01, branch `fix/centered-floating-controls`) rebuilt the on-screen controls per operator feedback: one centered floating bottom pill (translucent dark frame, painted vector-triangle arrows — ←/→ navigate) replaces the Wave-2 corner rails and the Wave-1 top toolbar, and the empty state shows one big centered Open button (no Open control while an image is open); its operator re-test was closed by the later 0.3.0 run below. Wave 3 landed (branch `feat/wave-3-vertical-scroll`, version 0.3.0): vertical sibling-folder navigation — the Up/Down arrow keys and the pill's ↑/↓ buttons move to the previous/next sibling folder under the same parent; a folder visited this session reopens its last-viewed image and a new folder opens its first image in alphanumeric order (NO same-filename attempt, no wrap-around); an image-less sibling folder is entered in an empty-folder browsing state (the blank viewer surface shows "Empty folder", the pill stays, and the folder arrows keep working from it); the per-folder last-viewed memory is RAM-only for the current session (destroyed on app close; no persistence in v1). Wave 4 landed (branch `feat/wave-4-fullscreen-zoom`, version 0.4.0): fullscreen mode (default OFF at launch; F11 and the pill's fullscreen square toggle it; ON hides the header/borders and all controls with only the image, and F11 is the only exit), manual zoom (I / O / 0 keys with Shift+Plus/Shift+Minus aliases, [+][−][restore-fit] pill buttons, ×1.25 steps bounded 1.0–8.0, `Zoom max`/`Zoom min` messages on rejected steps, reset to fit on every successful open), and fit modes (W/H keys and the pill's fit-width/fit-height buttons; fit width/height fill the window's width/height and reset the zoom; the whole-image-fit default returns on restore/`0` and every successful open); zoom and fit state are session-only. Verification so far: container dev checks (`ALL CHECKS PASSED`, 93 unit tests), Windows x64 release cross-build (exit 0 → `dist/windows-x64/img-scroller.exe` = 19,535,645 bytes), import audit; running the `.exe` — including fullscreen, zoom, and fit-mode behavior alongside the earlier navigation checks — is operator-run on a real Windows machine (the Wave-1 and 0.3.0 builds passed the operator's manual checks; the 0.3.0 run was reported 2026-11-02). The remaining v1 features (pan-then-navigate at the image edges while zoomed, "Open with...") are open. The full requirements, decisions, and scope boundaries live in the project brief: [.agent/project-info/brief.md](.agent/project-info/brief.md).

## Build and Run

Images Scroller is a Windows desktop image viewer built in Rust with egui/eframe; image decoding uses the Windows Imaging Component (WIC) system codecs. All Rust/Cargo commands run inside a Linux Docker container on the Alpine VM — the host machine needs no Rust or Cargo installed.

v1 format constraint: the app opens any image whose format is registered as a codec in the Windows system (via WIC — e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed); formats not registered in the system are not supported in v1.

### Container dev checks (no host Rust/Cargo)

Build the container image once, then run the dev-check harness:

    docker compose -f /img-scroller/docker-compose.yml build rust
    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh

The harness runs `cargo fmt --check`, `cargo check --locked`, `cargo test --locked`, and `cargo clippy --locked -- -D warnings`, then prints `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1). Full semantics — per-check fallback commands, the `FORCE_FAIL` fault-injection hook, the UTC log under `logs/checks/`, and the `alpine-vm` MCP usage notes — are documented in [docs/build.md](docs/build.md); this README does not duplicate them.

### Windows x64 release cross-build

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh

A successful run leaves the release executable at `dist/windows-x64/img-scroller.exe` (the `dist/` tree is gitignored). Running the produced `.exe` — GUI behavior and image-format handling — is validated by the operator on a real Windows 10/11 machine; the Linux container cannot run the Windows GUI. Typed failures, overwrite/rebuild behavior, toolchain notes, and the static-linking import audit are documented in [docs/build-windows.md](docs/build-windows.md).

## Note for AI Agents

- AI agents working on this project must read and adhere to [AGENTS.md](AGENTS.md) and follow the procedures defined in [.agent/WORKFLOWS.md](.agent/WORKFLOWS.md).
- The source of truth for requirements and scope is [.agent/project-info/brief.md](.agent/project-info/brief.md); agents must read it before starting any task.

## License

Images Scroller is released under the MIT License; the full text is in the [LICENSE](LICENSE) file, and its copyright notice embeds the GitHub project URL.

- The app can be used 100% free of charge by anyone.
- Forking the project, changing the code, and using a modified version are allowed.
- Anyone sharing the app outside the GitHub **Releases** section of the GitHub project (https://github.com/kazuyakuza/img-scroller) must add a reference to that GitHub project (attribution).
