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
2. **Horizontal Scroll (Sibling Images)** — left/right arrow keys and left/right UI buttons navigate to the previous/next sibling image in the same folder; the scope is strictly the current folder (not recursive) and the order is alphanumeric.
3. **Vertical Scroll (Sibling Folders)** — up/down arrow keys and up/down UI buttons navigate to the previous/next sibling folder under the same parent folder; when moving to a sibling folder, the target is (a) the folder's session last-viewed image if the folder was already visited this session (RAM-only), otherwise (b) ALWAYS the target folder's first image in alphanumeric order — no other fallback; the app also remembers the last viewed image per visited folder for the current session (RAM-only, destroyed on app close — no persistence to disk in v1).
4. **Fullscreen Mode** — default OFF at launch: the window opens normal or maximized at the user's own placement/size; when OFF, the UI controls are visible and the image takes almost all of the central space; when ON, the window header and all UI controls are hidden and only the image is shown; toggles via a keyboard shortcut and a UI control (exact bindings deferred).
5. **Zoom In/Out and Fit Modes** — zoom in/out via keyboard shortcut and UI controls, plus "fit width" and "fit height" modes.
6. **Pan and Navigate at Image Edges While Zoomed** — when the zoomed image overflows the window, the arrow keys pan the image first, and navigation (images horizontally, folders vertically) triggers only when panning reaches the image's edge; when the image fits entirely, the arrows navigate directly.
7. **Fast "Open with..." Menu** — a menu entry lists the programs the Windows system has registered for the currently viewed file's type, and selecting one opens the current file with it; the list is built from system registration data at runtime.

Additional scope notes:

- v1 sorting is alphanumeric for both folders and files; a future settings feature will let the user change it.
- Features explicitly deferred from v1 (for example rotation, slideshow, image editing, printing, persistence of the last-viewed memory across sessions, and a sort-order settings UI) are listed in the brief's out-of-scope section ([.agent/project-info/brief.md](.agent/project-info/brief.md)).

## Technology Stack (Planned)

The choices below are confirmed decisions but not yet implemented — the project is greenfield, with no code.

| Component | Choice (planned) | Notes |
| --- | --- | --- |
| Language | Rust | Confirmed decision in the brief; not yet implemented. |
| GUI | egui/eframe | Pure Rust, lightweight; suitable for zoom/pan/fullscreen/keyboard shortcuts. |
| Image decoding | Windows Imaging Component (WIC) system codecs | The app accepts any image format registered in the system instead of bundling its own decoders. |
| Target platform | Windows 10 and Windows 11 | Planned (not yet implemented). |
| Target architectures | x64 and x86 builds | Planned (not yet implemented). |

- Formats are supported when a system codec exists; formats not registered in the system are not supported in v1 — the accepted constraint of relying on system codecs.

## Distribution (Planned)

- Zip-style portable installer: the user picks a destination folder; the package unzips the application binary AND its config file into that destination.
- The package includes a helper and/or guidance for registering the app as the default image viewer (file associations) in Windows.
- The helper's implementation details (script vs. executable, exact mechanism) are deferred to implementation.

## Project Status

This project is greenfield and in the planning phase: the intended v1 product is fully defined, but no application code exists yet (as of 2026-10-29) — there is no `Cargo.toml` and no source code. The full requirements, decisions, and scope boundaries live in the project brief: [.agent/project-info/brief.md](.agent/project-info/brief.md).

## Build and Run (Not Yet Implemented)

This section is a placeholder: nothing is implemented yet and no build system exists, so there is nothing to build or run at the moment. Once the first implementation wave creates the Rust crate, a standard Cargo workflow will apply once `Cargo.toml` exists, and the steps will be documented here and in [.agent/project-info/tech.md](.agent/project-info/tech.md). No runnable commands are provided until then.

## Note for AI Agents

- AI agents working on this project must read and adhere to [AGENTS.md](AGENTS.md) and follow the procedures defined in [.agent/WORKFLOWS.md](.agent/WORKFLOWS.md).
- The source of truth for requirements and scope is [.agent/project-info/brief.md](.agent/project-info/brief.md); agents must read it before starting any task.

## License

Images Scroller is released under the MIT License; the full text is in the [LICENSE](LICENSE) file, and its copyright notice embeds the GitHub project URL.

- The app can be used 100% free of charge by anyone.
- Forking the project, changing the code, and using a modified version are allowed.
- Anyone sharing the app outside the GitHub **Releases** section of the GitHub project (https://github.com/kazuyakuza/img-scroller) must add a reference to that GitHub project (attribution).
