# Project Brief

## Overview

- Windows desktop application for viewing images, built in Rust.
- Recreates the behavior of the classic Windows "gallery"-era image viewer: open one image and easily navigate between the sibling images in the same folder and the sibling folders under the same parent folder.
- Navigation model: horizontal navigation moves across images within the current folder; vertical navigation moves across sibling folders under the same parent folder.
- Project name: **Images Scroller** (final). The repository directory name `img-scroller` remains unchanged — it is the repository name only, not the app name.
- Project status: greenfield — planning phase; no application code exists yet. This brief defines the intended v1 product.

## Problem Statement

Modern Windows image viewers do not support the classic gallery-style scrolling behavior: after opening an image, the user cannot move to the next/previous image or jump to sibling folders with simple arrow keys in the intended way.

Existing alternative viewers that attempt this behavior require the user to pre-register or pre-add the folder to the application before browsing, which adds friction.

Consequence: browsing a library of image folders (e.g., a photo archive organized as `parent\folder1`, `parent\folder2`, ...) requires manual navigation or a file manager instead of direct in-viewer scrolling.

Therefore: a viewer is needed that treats any opened image as the entry point into its folder tree, without any pre-registration of folders.

## Purpose and Goals

- Purpose: restore the classic Windows "gallery" viewing behavior as a modern, lightweight Rust application.
- Goal: opening ANY image file (from Explorer, a file manager, or the in-app open dialog) immediately enables full image/folder scrolling in its location — zero folder setup required.
- Goal: simple, discoverable navigation — keyboard arrows AND on-screen UI buttons for both horizontal (images) and vertical (folders) movement.
- Goal: support zooming (including fit width / fit height) and fullscreen viewing, with clean pan-then-navigate behavior while zoomed (see `## v1 Feature Scope`).
- Goal: leverage the operating system instead of duplicating it — decode images with system-registered codecs and offer system-registered programs in an "Open with..." menu (see `## v1 Feature Scope`).
- Goal: remain easy to install and distribute as a portable, zip-style package (see `## Distribution and Default App Registration`).

## v1 Feature Scope

### Open Image with System-Registered Formats

- The app opens any image whose format is registered as a codec in the Windows system (via WIC — see `## Technology Stack`), e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed.
- Formats not registered in the system are not supported in v1; this is the accepted constraint of relying on system codecs.
- The image opens in a regular resizable viewer window consistent with the classic viewer experience (not maximized, not a modal dialog).

### Horizontal Scroll (Sibling Images)

- Left arrow key and a left UI button navigate to the previous sibling image in the same folder.
- Right arrow key and a right UI button navigate to the next sibling image in the same folder.
- Scope is strictly the current folder's images (not recursive).
- Image order follows the sort order defined in `## Sorting Order` (alphanumeric).
- While the image is zoomed beyond the window width, left/right keys pan first — see `### Pan and Navigate at Image Edges While Zoomed`.

### Vertical Scroll (Sibling Folders)

- Up arrow key and an up UI button navigate to the previous sibling folder; down arrow key and a down UI button navigate to the next sibling folder.
- Navigation is restricted to sibling folders under the SAME parent folder: viewing `parent\folder2\img1` and pressing up/down moves to an image in `parent\folder1` or `parent\folder3` — never to folders of `parent` (no grandparent-level jumping), and never recursively deeper.
- Target image priority: (a) if the target folder was already visited this session, the app reopens its session last-viewed image (RAM-only); (b) otherwise, the app ALWAYS opens the target folder's FIRST image in alphanumeric order (per `## Sorting Order`). No other fallback.
- Last-viewed memory rule (per session): while navigating folders during one app session, the app remembers the last viewed image of each visited folder; revisiting a folder in the SAME session reopens that last viewed image (this is priority (a) above); the memory lives only in RAM for the current session and is destroyed when the application closes — there is NO persistence to disk in v1.

### Fullscreen Mode

- Default fullscreen state at launch: OFF — the window opens normal or maximized at the user's own placement/size.
- Fullscreen OFF: the UI controls are visible (UI layout details are deferred to a later definition by the user) and the image takes almost all of the central space.
- Fullscreen ON: the window header and ALL UI controls are hidden; only the image is shown.
- Both a keyboard shortcut and a UI control toggle it; exact bindings to be decided during implementation.

### Zoom In/Out and Fit Modes

- Zoom in / zoom out via keyboard shortcut and UI controls.
- "Fit width" mode: scale the image so its width fits the window.
- "Fit height" mode: scale the image so its height fits the window.
- Specific zoom step sizes are implementation details, not brief content.

### Pan and Navigate at Image Edges While Zoomed

- When the zoomed image is WIDER than the window, the left/right arrow keys pan the image horizontally; horizontal image navigation (previous/next sibling image) triggers only when panning reaches the image's left or right edge.
- When the zoomed image is TALLER than the window, the up/down arrow keys pan the image vertically; folder navigation (previous/next sibling folder) triggers only when panning reaches the image's top or bottom edge.
- When the image fits entirely (no overflow), arrows navigate directly.

### Fast "Open with..." Menu

- A menu entry opens a "fast open-with" list of programs that the Windows system has registered for the currently viewed file's type/extension.
- Selecting a program opens the current file with it.
- The list is built from system registration data at runtime — the app does not maintain its own program database.

## Sorting Order

- v1 sorting: alphanumeric for BOTH folders and files. All navigation (horizontal image order, vertical folder order, "first image" rule, last-viewed revisit) follows this single order.
- A future settings feature will let the user change the sort order; v1 ships with alphanumeric fixed and no setting UI.

## Distribution and Default App Registration

- Distribution model: zip-style portable installer.
- Installer flow: the user picks a destination folder; the package unzips the application binary AND its config file into that destination.
- Default image viewer registration: the package includes a helper and/or guidance that helps the user register the app as the default image viewer (file associations) in Windows.
- Implementation details of the helper (script vs. executable, exact mechanism) are deferred to implementation and are not decided in this brief.
- License: MIT (the full text is in the repository root `LICENSE` file). Anyone can use the app 100% free; forks and modified versions are allowed; anyone sharing the app outside the future GitHub "Releases" section must add a reference to the GitHub project — the project URL is embedded in the LICENSE copyright notice.

## Technology Stack

These choices are confirmed decisions but not yet implemented (planned decisions — greenfield, no code yet).

- Language: Rust.
- GUI: egui/eframe (pure Rust, lightweight; suitable for zoom/pan/fullscreen/keyboard shortcuts).
- Image decoding: Windows Imaging Component (WIC) system codecs — the app accepts any image format registered in the system instead of bundling its own decoders.
- Target platform: Windows 10 and Windows 11.
- Target architectures: x64 and x86 builds.
- Build environment: builds and checks run inside a Linux container on the Alpine VM via Docker (Docker exists only on the VM; the host needs no Rust/Cargo); the release Windows `.exe` is cross-compiled from that container (x86_64-pc-windows-gnu); Windows targets confirmed x64 AND x86 (x86 realized in Wave 7 packaging) — decision date 2026-10-30.

## Out of Scope for v1 (Deferred)

Features below are explicitly NOT in v1 and may be added later:

- Rotation.
- Slideshow.
- Image editing.
- Printing.
- Any further functionality inherited from the old gallery app beyond the 7 v1 features.
- Settings UI for changing the sort order (v1 is fixed alphanumeric — see `## Sorting Order`).
- Persistence of the per-folder last-viewed memory across sessions (v1 memory is session-only and destroyed on close — see `### Vertical Scroll (Sibling Folders)`).

## Addendum — Operator Corrections (2026-11-02, BINDING)

The 2026-11-02 operator chat corrections supersede the contradicting sentences of this brief wherever they appear (this addendum governs); everything not contradicted below stays byte-identical. Source: [`.agent/todos/20261102/20261102-todo-1.md`](../todos/20261102/20261102-todo-1.md) (Wave 4.1).

1. **Launch window state** (supersedes the `### Open Image with System-Registered Formats` sentence "The image opens in a regular resizable viewer window consistent with the classic viewer experience (not maximized, not a modal dialog)" AND the `### Fullscreen Mode` sentence "Default fullscreen state at launch: OFF — the window opens normal or maximized at the user's own placement/size"): the application window opens **ALWAYS MAXIMIZED at EVERY launch** (never auto-fullscreened; the fullscreen default stays OFF), and while the release build runs, **no console window is visible**. The viewing window remains a regular RESIZABLE window — the maximized state is just its launch size.
2. **Zoom range is symmetric** (supersedes any reading of `### Zoom In/Out and Fit Modes` that blocks zoom-out at whole-image fit): zoom-out is NOT blocked at the neutral fit level; the zoom-out lower limit is SYMMETRIC to the zoom-in upper limit (same magnitude reversed — when the zoom-in max is ×8, the zoom-out floor is ÷8 of the fit size).
3. **Zoom/fit actions are silent** (supersedes the Wave-4 message scheme): the bound-step and fit-activation messages are removed — zoom/fit actions do not announce anything (the only app message remains the unsupported-file failure text).
4. **Fit width/height are ONE-SHOT size sets** (supersedes fit-mode persistence in `### Zoom In/Out and Fit Modes`): pressing `W`/`H` or their buttons ONLY sets the current zoom level so the current image's width/height fills the window — nothing else; normal zoom in/out works from that level afterward (no persistent mode may block zoom-out).
5. **ESC exits fullscreen** (additional to F11; the exact interplay was decided in the Wave-4.1 architector step): ESC exits fullscreen ON; otherwise ESC does nothing.
6. **Affordance add-on** (new requirement from the 2026-11-02 operator chat): each keyboard-distinct UI button displays a small reference of its related hotkey beneath it, and hovering any button shows a short tooltip of what it does.

## Addendum — Operator Additions (2026-11-07, BINDING)

The 2026-11-07 operator chat additions supersede any contradicting sentence of this brief wherever it appears; everything not contradicted below stays byte-identical. Source: [`.agent/todos/20261107/20261107-todo-1.md`](../todos/20261107/20261107-todo-1.md) (Wave 4.2).

7. **ESC double-press close while fullscreen OFF** (additional to item 5; the exact popup wording, lifetime, and fullscreen interplay were decided in the Wave-4.2 architector step): while fullscreen is OFF, the FIRST ESC press shows a CENTERED confirmation popup with the message "Press ESC again to close" for 2 seconds; the application closes ONLY when a SECOND DISTINCT ESC press arrives while the popup is visible — two presses, NEVER held/autorepeated; an expired popup re-arms on the next press; while fullscreen is ON, ESC exits fullscreen only (item 5's rule — no popup, no close).

## Addendum — Operator Corrections (2026-11-12, BINDING)

The 2026-11-12 operator chat corrections supersede any contradicting sentence of this brief wherever it appears; everything not contradicted below stays byte-identical. Source: [`.agent/todos/20261112/20261112-todo-1.md`](../todos/20261112/20261112-todo-1.md) (Wave 5.1).

8. **Navigation order follows the system name order** (refines `## Sorting Order`'s alphanumeric rule for non-ASCII names; the exact mechanism was decided in the Wave-5.1 architector step): BOTH navigation axes (sibling images in a folder; sibling folders under the same parent) order entry names by the names' expected Unicode/system order — the order the Windows system (e.g. the file manager) shows for those names, including non-ASCII/Japanese names; the first-image rule and the last-viewed revisit priority read from the same order.

<!-- DO NOT DELETE NEXT SECTION -->

## Important Note for AI Agents

All agents working on this project MUST adhere to the workflows and rules outlined in [AI Agent Onboarding document](../../AGENTS.md).

Before starting any task:

1. **Review `AGENTS.md`**: is the primary source of instructions for agents.
2. **Follow Workflows**: follow the procedures defined in `.agent/WORKFLOWS.md`, especially the `.kilo/commands/critical-workflow.md`.

<!-- END DO NOT DELETE -->
