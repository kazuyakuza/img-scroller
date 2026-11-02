# System Architecture

## Status

The planned v1 architecture below is partially **implemented** (Wave 1, branch `feat/wave-1-skeleton-open-image`, 2026-10-31): the window shell, the open-image flow (CLI argument + native dialog + WIC decode), and the fit-scaled display exist in `src/` and are verified through the container harness. Wave 2 (branch `feat/wave-2-horizontal-scroll`, version 0.2.0, 2026-11-01) landed the horizontal sibling-image navigation slice: the current-folder listing module, Left/Right keyboard navigation, and the on-screen corner navigation rails. A follow-up UI-fix slice (branch `fix/centered-floating-controls`, version 0.2.1, 2026-11-01, per operator-run feedback) replaced the corner rails and the top Open… toolbar with one centered floating bottom pill (painted vector-triangle arrows; no font dependency) and an empty-state center Open button. Wave 3 (branch `feat/wave-3-vertical-scroll`, version 0.3.0, 2026-11-01/02) landed the vertical sibling-folder navigation slice: the sibling-folder listing module, the Up/Down keys and pill buttons, the vertical target-image priority, the empty-folder browsing mode, and the session last-viewed memory. Dev checks ended `ALL CHECKS PASSED` (50 unit tests), the exe was rebuilt at `dist/windows-x64/img-scroller.exe` (19,502,691 bytes), and the DLL import audit is unchanged (the OBSERVED 22-DLL list lives only in `scripts/build-windows.sh` + [docs/build-windows.md](../../docs/build-windows.md)). Fullscreen, zoom, and "Open with..." remain planned. Facts are sourced from [brief.md](brief.md) (source of truth), the landed code, and [tech.md](tech.md).

## Contents

1. [Overview](#overview)
2. [Logical Components](#logical-components)
3. [Module Layout (Implemented)](#module-layout-implemented)
4. [Open-Image and Display Flow (Implemented)](#open-image-and-display-flow-implemented)
5. [Navigation Critical Paths](#navigation-critical-paths)
6. [Data and State](#data-and-state)
7. [Build and Release Pipeline](#build-and-release-pipeline)
8. [Project Paths](#project-paths)
9. [Design Patterns](#design-patterns)
10. [Technical Decisions Summary](#technical-decisions-summary)
11. [Open Architectural Decisions](#open-architectural-decisions)

## Overview

- Shape: a single-purpose Windows desktop GUI application — an image viewer with two-axis folder navigation. No server, no back-end. It ships as a single application binary plus a config file (distribution flow: see `## Data and State` below and [tech.md](tech.md)).
- Three pillars chosen in the brief: egui/eframe GUI, WIC system-codec decoding, and reliance on Windows system registration data (codecs and the "Open with..." program list). Wave 1 landed pillars 1–2 as the open-image slice; pillar 3 ("Open with...") is planned.

## Logical Components

The decomposition below is derived from the v1 features; each component carries a landed/planned annotation. The realized module layout is in `## Module Layout (Implemented)`.

- **Viewer UI (egui/eframe, immediate-mode) — Wave-1 slice LANDED; Wave-2 navigation slice LANDED; 0.2.1 floating-controls UI-fix LANDED**: regular resizable window (title `Images Scroller`, not maximized at launch); the empty state shows ONE big centered "Open…" button (18 pt text, large 180×48 button) that opens the native dialog; with an image open there is NO Open control anywhere. While an image is open — or an empty sibling folder is being browsed — ONE centered floating bottom pill renders above the central surface — a non-space-reserving overlay `Area` anchored bottom-center with a translucent dark frame — holding the message line and four arrow buttons with PAINTED vector-triangle arrows (no font dependency; ←/→ navigate previous/next image, ↑/↓ navigate previous/next sibling folder, each enabled exactly when its navigation seam resolves a destination). In empty-folder browsing mode the blank central surface shows the centered `Empty folder` message while the pill's message line stays empty. Both overlay Areas are stateless surfaces (per-call `egui::Id::unique(...)` ids — no remembered window/layout state) and the painted buttons expose AccessKit button info (`WidgetInfo::labeled(Role::Button, …)`). Remaining UI (fullscreen toggle, zoom controls, "Open with..." menu entry): planned.
- **Image Decoding Layer (WIC) — LANDED**: decodes any image format registered as a Windows system codec through the windows-rs COM projection (mechanism: [tech.md](tech.md) → `## WIC Decode and Open-Dialog Mechanism (Implemented)`); no bundled decoders; formats without a system codec are unsupported in v1 (accepted constraint).
- **Navigation Engine — both slices LANDED (horizontal Wave 2; vertical Wave 3)**: horizontal — previous/next sibling image in the current folder (listing: non-recursive parent-folder read, 17-extension whitelist; order: lowercase-folded alphanumeric with exact-name tie-break; list edges stay silently); vertical — the ADJACENT sibling folder under the SAME parent (target image priority: (a) session last-viewed image for folders already visited this session, (b) otherwise ALWAYS the target folder's first image in alphanumeric order — NO same-filename attempt, NO other fallback; no skip, no wrap; an image-less sibling folder is entered in empty-folder browsing mode, not skipped).
- **Session Last-Viewed Memory — LANDED (Wave 3)**: `src/last_viewed_images.rs` holds the RAM-only map of the last viewed image per visited folder — created empty with the app, updated at every successful open (the key is the RESOLVED open path's parent folder — D5 canonicalization fact), read for the visited-folder priority, destroyed on app close; no disk persistence in v1.
- **Pan/Zoom Controller — planned**: zoom in/out, fit width/fit height, and pan-then-navigate — width-overflowed image: left/right keys pan until the image's horizontal edge, then trigger horizontal navigation; height-overflowed image: up/down keys pan until the vertical edge, then trigger folder navigation; no overflow: arrows navigate directly.
- **Fast "Open with..." Service — planned**: at runtime queries the Windows system for programs registered for the current file's type/extension; no internal program database.
- **Distribution Packaging — planned**: zip-style installer (destination-folder selection; unzips the app binary and config file) plus default-viewer registration helper/guidance — the helper mechanism is deferred.

## Module Layout (Implemented)

All source lives in `src/` (folder map: [`.agent/project-structure.md`](../project-structure.md)); files stay under the 200-line rule.

- `src/main.rs` — entry point: takes the first CLI argument as an optional image path (`std::env::args_os().nth(1)`), starts the eframe native app with the `Images Scroller` title/window options, and opens the CLI image before the first frame when present.
- `src/app.rs` — application shell: `ImagesScrollerApp` state (opened path, in-session folder listing, session last-viewed memory, vertical sibling-folder listing, browsed empty-folder context, decoded payload, message text, display), the `open_image` flow (canonicalizes the path once — the D5 single-normalization point — then decode → state transition; failure changes only the message), the four-direction arrow-key intake and the shared `navigate(NavigationDirection)` seam wiring, the empty-state centered "Open…" button wiring, the navigation-context gate that shows the pill (image open OR empty-folder context), the `APP_WINDOW_TITLE` constant, and `NativeOptions` (resizable, not maximized). Child modules: `src/app/arrow_direction.rs`, `src/app/floating_controls.rs`, `src/app/folder_navigation.rs`, `src/app/empty_folder_view.rs`, `src/app/tests.rs`, `src/app/floating_controls_tests.rs`, `src/app/folder_navigation_tests.rs`, `src/app/folder_navigation_continuation_tests.rs`, `src/app/last_viewed_tests.rs`.
- `src/folder_listing.rs` — pure-logic current-folder listing (child module `src/folder_listing/tests.rs`): one non-recursive read of the opened image's parent folder, a fixed 17-extension image whitelist (case-insensitive match), lowercase-folded name sort with exact-name tie-break, and ±1 neighbor resolution that reports the list's beginning/end, plus `first_listed_image` (the vertical navigation's new-folder first-image probe); its sort is the shared collation from `src/name_collation.rs` (mechanism details: [tech.md](tech.md) → the horizontal and Wave-3 mechanism sections).
- `src/sibling_folder_listing.rs` — pure sibling-folder listing for vertical navigation (split test modules `src/sibling_folder_listing/tests.rs` and `src/sibling_folder_listing/neighbor_tests.rs` over one TempScope fixture): one non-recursive read of the current folder's parent, immediate child directories only (current folder excluded; symlinked entries unlisted — the `file_type` policy), the shared fixed collation sort, and ±1 neighbor resolution reporting the list edges; an unreadable/missing scope yields an empty listing, an unanchorable anchor no listing.
- `src/last_viewed_images.rs` — session last-viewed memory: RAM-only folder→last-displayed-image map, updated at every open-success, read for the visited-folder target priority, destroyed with the app; no persistence.
- `src/name_collation.rs` — the fixed v1 alphanumeric entry-name collation (`compare_entry_names`: lowercase form first, exact-character tie-break) shared by the image listing and the sibling-folder listing.
- `src/app/floating_controls.rs` — the floating navigation pill (replaced the deleted `src/app/rails.rs`): one overlay `Area` anchored bottom-center (translucent-dark `Frame`, non-space-reserving, `Order::Foreground`) holding the message line (a 12-pt label above the button row) and one row of four arrow buttons painted via `egui::Shape::rotated_triangle` (no font dependency); a button is enabled exactly when the navigation seam resolves a neighbor (`neighbor_image_path(...).is_some()`), and ALL FOUR arrows drive the `navigate` seam — the folder ones mapped to `PreviousSiblingFolder`/`NextSiblingFolder`; the pill renders while an image is open or an empty-folder context is browsed (gate `navigation_context_is_active()` in `src/app/folder_navigation.rs`, called from `app.rs`).
- `src/app/arrow_direction.rs` — pure child-module types for the pill: `ArrowDirection` (the `rotated_triangle` rotation mapping: left/right/up/down) and `ArrowControl` (which button a widget represents: accessible labels + a concrete `NavigationDirection` for EVERY arrow — the folder arrows map to `PreviousSiblingFolder`/`NextSiblingFolder`).
- `src/image_loading.rs` — open-image seam: `load_image` (delegates to the platform facade), `DecodedImage` payload (width, height, RGBA8 `pixels`), `ImageLoadFailure` (`Unsupported` / `Unloadable`), the `pixel_buffer_size` guard, and the unsupported-file message constant.
- `src/platform/` — OS-bound facade behind cfg-gating: `mod.rs` re-exports `decode_image` and `show_open_image_dialog`; `windows.rs` implements the native `IFileOpenDialog` and the WIC decode (mechanism: [tech.md](tech.md)); `stub.rs` is the non-Windows fallback (dialog → `None`, decode → `Unloadable`) that keeps container checks/tests compiling.
- `src/scaling.rs` — pure fit-scaling math: `fitted_size` returns the largest uniform size that keeps the image inside the available area (aspect preserved; zero/negative dimensions rejected); no egui painting.
- `src/app_display.rs` — central display: `CurrentImageDisplay` texture wrapper — one texture upload per successful decode (pending-upload flag; the dropped old handle frees the previous texture), fit-scaled centered paint via `egui::Image::fit_to_exact_size`.

## Open-Image and Display Flow (Implemented)

1. Launch: CLI argument present → `open_image(path)` runs before the first frame; absent → the empty state's big centered "Open…" button → native `IFileOpenDialog` (owner HWND via `raw-window-handle`).
2. Decode: `platform::decode_image` → WIC factory → decoder from filename → frame → format converter initialized to 32bppRGBA → `CopyPixels` copies from the converter (mechanism detail: [tech.md](tech.md)).
3. Success: state stores path + `DecodedImage`, clears the message, and flags a pending texture upload; the next frame's central-panel paint uploads exactly once and paints the image fit-scaled and centered (per-frame fit recomputation via `scaling::fitted_size`, so window resizes keep the display sane).
4. Failure: `Unsupported`/`Unloadable` → the floating pill's message line shows the unsupported-file message (the pill renders while an image is open or an empty-folder context is browsed); a previously loaded image and the rest of the state stay untouched.

## Navigation Critical Paths

Path 1's open/decode/display slice (Wave 1), Path 2's horizontal sibling-image navigation (Wave 2), and Path 3's vertical sibling-folder navigation (Wave 3) are implemented; the pan/zoom and "Open with..." parts remain planned.

1. **Entry**: open an image (Explorer association, file manager, or in-app open dialog) → decode via a WIC system codec → display in the regular resizable window → navigation enabled immediately; no folder pre-registration step exists.
2. **Horizontal navigation (Wave-2 slice LANDED)**: arrow key or UI button → previous/next sibling image in the current folder (listing: 17-extension whitelist; order: lowercase-folded alphanumeric with exact-name tie-break; list edges stay silently) → decode → display; arrows navigate directly at this wave — pan-then-navigate (navigation only at the zoomed image's left/right edge) lands in Wave 5.
3. **Vertical navigation (Wave-3 slice LANDED)**: arrow key or UI button → the ADJACENT previous/next sibling folder under the same parent (no skip, no wrap; silent list edges) → target image chosen by priority: (a) the folder's session last-viewed image if the folder was already visited this session (RAM-only), else (b) ALWAYS the target folder's first image (alphanumeric order); NO same-filename attempt → decode → display through the normal open flow; an image-less sibling folder is entered in empty-folder browsing mode (the blank surface shows `Empty folder`; the vertical listing re-anchors on the browsed folder). The pan-first interplay — navigation only at the zoomed image's top/bottom edge — remains Wave-5 planned; arrows navigate directly now.
4. **Fullscreen / zoom toggles**: fullscreen is OFF by default at launch (the window opens normal or maximized at the user's own placement/size); fullscreen OFF shows the UI controls with the image taking almost all of the central space; fullscreen ON hides the window header and ALL UI controls, showing only the image; keyboard shortcut and UI control toggle it; zoom via shortcut + controls; fit width / fit height modes.
5. **Open with**: menu entry → enumerate system-registered programs for the current file type → the selected program opens the current file.

## Data and State

- **In memory (Wave-1 landed)**: the open-flow state in `ImagesScrollerApp` — last opened path, decoded RGBA payload, current message text, and the display texture handle — all RAM-only, replaced on each successful open; nothing persisted.
- **In memory (per session; Wave-3 landed)**: the horizontal folder listing, the vertical sibling-folder listing, the browsed empty-folder context, and the per-folder last-viewed map (RAM-only, destroyed on close — no persistence in v1); window/zoom state for the session stays planned.
- **On disk (planned)**: an application **config file** shipped by the installer alongside the binary (brief distribution flow). Its contents/format are **not defined anywhere yet** — only its existence is recorded here; contents are an implementation decision.
- No other persistent state in v1; explicitly no persistence of the last-viewed memory.

## Build and Release Pipeline

Landed in Wave 1 (Task 1) and exercised in Task 5: dev checks ended `ALL CHECKS PASSED`; the Windows x64 cross-build exited 0 → a non-empty `dist/windows-x64/img-scroller.exe`. Guides: [docs/build.md](../../docs/build.md) and [docs/build-windows.md](../../docs/build-windows.md).

- Dev checks: `scripts/dev-checks.sh` runs inside the container — in order `cargo fmt --check`, `cargo check --locked`, `cargo test --locked`, `cargo clippy --locked -- -D warnings` — emitting per-check PASS/FAIL lines and a final `ALL CHECKS PASSED` summary, with a UTC-timestamped log under the gitignored `logs/checks/`.
- Windows release cross-build: `scripts/build-windows.sh` runs `cargo build --release --locked --target x86_64-pc-windows-gnu` inside the container and installs the final exe atomically (`.part` copy → rename) at `dist/windows-x64/img-scroller.exe`; a failed run leaves the previous artifact untouched; the x86 target belongs to Wave 7 (consuming/extending this script and adding the i686 MinGW toolchain when the x86 target is added).
- Container image: pinned official `rust:1.99.1-bookworm` → `img-scroller-rust:latest` (rustfmt, clippy, MinGW-w64 x64 toolchain, `x86_64-pc-windows-gnu` target); named volume `img-scroller-cargo-target` holds intermediate build output via `CARGO_TARGET_DIR`.
- Import-audit evidence: the OBSERVED 22-DLL import list of the produced exe is recorded once in `scripts/build-windows.sh` (`OBSERVED_DLL_IMPORT_LIST`) and in [docs/build-windows.md](../../docs/build-windows.md); not duplicated here.
- Validation stance: container-only — all agent-side build/check validation happens inside the container; Windows runtime/GUI validation is OPERATOR-RUN on the real Windows 10/11 machine (the Linux container cannot execute the Windows GUI application).

## Project Paths

- All source code must live in `src/` (project structure rule; folder map: [`.agent/project-structure.md`](../project-structure.md)).
- Cargo project at the repository root: `Cargo.toml` (package `img-scroller` 0.3.0, edition 2024) plus the tracked `Cargo.lock` — exists since Wave 1.
- Repository layout: application code in `src/` (the `app` shell with its `arrow_direction`/`floating_controls`/`folder_navigation`/`empty_folder_view` child modules, `app_display`, `folder_listing`, `last_viewed_images`, `name_collation`, `sibling_folder_listing`, `image_loading`, `scaling`, plus the `platform/` facade), harness in `Dockerfile`/`docker-compose.yml`/`scripts/`, guides in `docs/`, agent scaffolding in `.agent/`/`.kilo/`/`.opencode/`.
- Module layout: decided and implemented — see `## Module Layout (Implemented)`.

## Design Patterns

- The GUI paradigm follows egui/eframe's **immediate-mode** model (confirmed; in use in the landed shell).
- Landed Wave-1 pattern: OS-bound code isolated behind the cfg-gated `platform` facade, so container-runnable pure logic (`scaling`, `image_loading`, app state transitions) stays testable on the Linux host target.
- Everything else (navigation state management, internal abstractions) is not yet decided — see `## Open Architectural Decisions`.

## Technical Decisions Summary

| Decision | Status | Source (brief section) |
| --- | --- | --- |
| Language: Rust | landed (Wave 1) | `## Technology Stack` |
| GUI: egui/eframe | landed (Wave 1: window + open + display slice) | `## Technology Stack` |
| Image decoding: WIC system codecs | landed (Wave 1) | `## Technology Stack` |
| Target OS: Windows 10 / 11 | implemented for the GUI; runtime check operator-run | `## Technology Stack` |
| Target architectures: x64 + x86 | x64 landed (Wave 1); x86 planned (Wave 7) | `## Technology Stack` |
| Sorting: alphanumeric fixed in v1 | v1 constraint | `## Sorting Order` |
| Last-viewed memory: session-only, RAM | landed (Wave 3) | `### Vertical Scroll (Sibling Folders)` |
| Distribution: zip-style installer | planned | `## Distribution and Default App Registration` |

## Open Architectural Decisions

Each item below is to be decided during implementation, NOT now:

- Fullscreen/zoom keybindings and zoom step sizes.
- Pan-then-navigate details while zoomed (Wave 5).
- Distribution-helper mechanism (script vs. executable).
- Config file contents/format.
- Design patterns beyond immediate-mode.
