# Product Overview

## Product Summary

- Product name: **Images Scroller** (final). The repository directory name `img-scroller` remains unchanged — it is the repository name only, not the app name.
- Windows desktop application for viewing images, built in Rust.
- Recreates the behavior of the classic Windows "gallery"-era image viewer: open one image and easily navigate between sibling **images** in the same folder and sibling **folders** under the same parent folder.
- Status: greenfield — planning phase; **no application code exists yet** (as of 2026-10-29).
- Source of truth: [brief.md](brief.md) in this same folder.

## Problem Definition

- Modern Windows image viewers do not support the classic gallery-style scrolling behavior: after opening an image, the user cannot move to the next/previous image or jump to sibling folders with simple arrow keys in the intended way.
- Existing alternative viewers that attempt this behavior require the user to pre-register or pre-add the folder to the application before browsing — friction.
- Consequence: browsing a library of image folders (e.g., `parent\folder1`, `parent\folder2`, ...) requires manual navigation or a file manager instead of direct in-viewer scrolling.
- Therefore: a viewer is needed that treats **any opened image as the entry point into its folder tree**, with **zero folder setup**.

## Target Users and Platform

- Users browsing image-folder libraries organized as `parent\folder1`, `parent\folder2`, ... without pre-setup.
- Platform: Windows 10 / Windows 11 (planned).
- Builds: x64 and x86 (planned).

## Navigation Model

- Horizontal axis = images **within the current folder only** (not recursive): Left/Right arrow keys AND on-screen left/right UI buttons move to the previous/next sibling image.
- Vertical axis = sibling folders **under the SAME parent folder** only: Up/Down arrow keys AND on-screen up/down UI buttons move to the previous/next sibling folder.
- Navigation is restricted to sibling folders under the SAME parent folder: viewing `parent\folder2\img1` and pressing up/down moves to an image in `parent\folder1` or `parent\folder3` — never to folders of `parent` (no grandparent-level jumping), and never recursively deeper.
- Opening ANY image file (from Explorer, a file manager, or the in-app open dialog) immediately enables full image/folder scrolling in its location — zero folder setup required.

## v1 User-Facing Capabilities

The v1 scope is exactly these 7 features (per [`brief.md` — v1 Feature Scope](brief.md)):

1. **Open Image with System-Registered Formats** (brief: `### Open Image with System-Registered Formats`) — opens any image whose format is registered as a Windows codec (via WIC), e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed; the image opens in a regular **resizable** viewer window (not maximized, not a modal dialog).
2. **Horizontal Scroll (Sibling Images)** (brief: `### Horizontal Scroll (Sibling Images)`) — Left/Right keys and left/right UI buttons navigate the previous/next sibling image in the same folder; strict current-folder scope (not recursive); order is alphanumeric (see the sorting note below).
3. **Vertical Scroll (Sibling Folders)** (brief: `### Vertical Scroll (Sibling Folders)`) — Up/Down keys and up/down UI buttons navigate sibling folders under the same parent folder.
   - Target image priority: (a) if the target folder was already visited this session, the app reopens its session last-viewed image (RAM-only); (b) otherwise, the app ALWAYS opens the target folder's FIRST image in alphanumeric order. No other fallback.
   - Last-viewed memory rule (per session): the app remembers the last viewed image of each visited folder; revisiting a folder in the SAME session reopens that last viewed image (this is priority (a) above); the memory lives only in RAM for the current session and is destroyed when the application closes — there is NO persistence to disk in v1.
4. **Fullscreen Mode** (brief: `### Fullscreen Mode`) — default OFF at launch: the window opens normal or maximized at the user's own placement/size; fullscreen OFF shows the UI controls (UI layout details deferred to a later definition by the user) with the image taking almost all of the central space; fullscreen ON hides the window header and ALL UI controls, showing only the image; toggled via a keyboard shortcut AND a UI control, exact bindings decided during implementation.
5. **Zoom In/Out and Fit Modes** (brief: `### Zoom In/Out and Fit Modes`) — zoom in/out via keyboard shortcut and UI controls; "fit width" mode scales the image so its width fits the window; "fit height" mode scales the image so its height fits the window; specific zoom step sizes are implementation details.
6. **Pan and Navigate at Image Edges While Zoomed** (brief: `### Pan and Navigate at Image Edges While Zoomed`) — when the zoomed image is WIDER than the window, the left/right arrow keys pan the image horizontally; horizontal image navigation (previous/next sibling image) triggers only when panning reaches the image's left or right edge. When the zoomed image is TALLER than the window, the up/down arrow keys pan the image vertically; folder navigation (previous/next sibling folder) triggers only when panning reaches the image's top or bottom edge. When the image fits entirely (no overflow), arrows navigate directly.
7. **Fast "Open with..." Menu** (brief: `### Fast "Open with..." Menu`) — a menu entry lists the programs the Windows system has registered for the currently viewed file's type/extension; selecting one opens the current file with it; the list is built from system registration data at runtime — no internal program database.

Sorting note: v1 sorting is alphanumeric for BOTH folders and files. All navigation (horizontal image order, vertical folder order, "first image" rule, last-viewed revisit) follows this single order. A future settings feature will let the user change the sort order; v1 ships fixed alphanumeric with no settings UI.

## Product Goals

- Purpose: restore the classic Windows "gallery" viewing behavior as a modern, lightweight Rust application.
- Zero-setup entry: opening ANY image immediately enables full image/folder scrolling in its location.
- Simple, discoverable navigation — keyboard arrows AND on-screen UI buttons for both axes.
- Zooming (including fit width / fit height) and fullscreen viewing with clean pan-then-navigate behavior while zoomed.
- Leverage the operating system instead of duplicating it — system-registered codecs for decoding; system-registered programs in "Open with...".
- Easy to install and distribute as a portable, zip-style package.

## Distribution Experience

- Zip-style portable installer: the user picks a destination folder; the package unzips the application binary AND its config file into that destination.
- A helper and/or guidance helps the user register the app as the default image viewer (file associations) in Windows.
- Helper implementation details (script vs. executable, exact mechanism) are deferred to implementation.

## Explicit Non-Goals (Deferred)

These are explicitly NOT in v1; they may be added later (per brief `## Out of Scope for v1 (Deferred)`):

- Rotation.
- Slideshow.
- Image editing.
- Printing.
- Any further functionality inherited from the old gallery app beyond the 7 v1 features.
- Settings UI for changing the sort order (v1 is fixed alphanumeric).
- Persistence of the per-folder last-viewed memory across sessions (v1 memory is session-only, destroyed on close).
