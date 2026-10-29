# System Architecture

## Status

All content below describes the **planned** architecture for a greenfield project. **Nothing is implemented yet** — no application code, no `Cargo.toml` (as of 2026-10-29). Facts are sourced from [brief.md](brief.md) (source of truth).

## Overview

- Planned shape: a single-purpose Windows desktop GUI application — an image viewer with two-axis folder navigation. No server, no back-end. It ships as a single application binary plus a config file (distribution flow: see `## Data and State (Planned)` below and [tech.md](tech.md)).
- Three pillars chosen in the brief (all planned): egui/eframe GUI, WIC system-codec decoding, and reliance on Windows system registration data (codecs and the "Open with..." program list).

## Logical Components (Planned)

This is a logical decomposition derived from the v1 features; it lists responsibilities only and does not describe existing code. The concrete module/file layout inside `src/` is undecided (see `## Open Architectural Decisions`).

- **Viewer UI (egui/eframe, immediate-mode, planned)**: regular resizable window; renders the image; hosts the on-screen left/right/up/down navigation buttons, the fullscreen toggle, the zoom controls (including fit width / fit height), and the fast "Open with..." menu entry.
- **Image Decoding Layer (WIC, planned)**: decodes any image format registered as a Windows system codec; no bundled decoders; formats without a system codec are unsupported in v1 (accepted constraint).
- **Navigation Engine (planned)**: horizontal — previous/next sibling image in the current folder (alphanumeric order); vertical — previous/next sibling folder under the SAME parent (same-filename rule, first-image fallback in alphanumeric order).
- **Session Last-Viewed Memory (planned)**: RAM-only map of the last viewed image per visited folder; last-viewed takes priority over the same-filename and first-image rules when the folder was already visited this session; destroyed on app close; no disk persistence in v1.
- **Pan/Zoom Controller (planned)**: zoom in/out, fit width/fit height, and pan-then-navigate — width-overflowed image: left/right keys pan until the image's horizontal edge, then trigger horizontal navigation; height-overflowed image: up/down keys pan until the vertical edge, then trigger folder navigation; no overflow: arrows navigate directly.
- **Fast "Open with..." Service (planned)**: at runtime queries the Windows system for programs registered for the current file's type/extension; no internal program database.
- **Distribution Packaging (planned)**: zip-style installer (destination-folder selection; unzips the app binary and config file) plus default-viewer registration helper/guidance — the helper mechanism is deferred.

## Navigation Critical Paths (Planned)

1. **Entry**: open an image (Explorer association, file manager, or in-app open dialog) → decode via a WIC system codec → display in the regular resizable window → navigation enabled immediately; no folder pre-registration step exists.
2. **Horizontal navigation**: arrow key or UI button → previous/next sibling image in the current folder (alphanumeric order) → decode → display; while the image is wider than the window, pan first; navigation only at the image's left/right edge.
3. **Vertical navigation**: arrow key or UI button → previous/next sibling folder under the same parent → target image chosen by priority: (a) last viewed this session (if the folder was already visited), else (b) the same filename if it exists in the target folder, else (c) the target folder's first image (alphanumeric order) → decode → display; while the image is taller than the window, pan first; navigation only at the image's top/bottom edge.
4. **Fullscreen / zoom toggles**: keyboard shortcut and UI control toggle fullscreen; zoom via shortcut + controls; fit width / fit height modes.
5. **Open with**: menu entry → enumerate system-registered programs for the current file type → the selected program opens the current file.

## Data and State (Planned)

- **In memory (per session)**: currently viewed image path and folder context; the per-folder last-viewed map (RAM-only, destroyed on close — no persistence in v1); window/zoom state for the session.
- **On disk**: an application **config file** shipped by the installer alongside the binary (brief distribution flow). Its contents/format are **not defined anywhere yet** — only its existence is recorded here; contents are an implementation decision.
- No other persistent state in v1; explicitly no persistence of the last-viewed memory.

## Project Paths

- All source code must live in `src/` (project structure rule; folder map: [`.agent/project-structure.md`](../project-structure.md)).
- Planned: Cargo project at the repository root with `Cargo.toml` — does **not** exist yet; created in the first implementation wave.
- Current repository layout is docs/config scaffolding only: `.agent/`, `.kilo/`, `.opencode/`, `docs/`, plus `src/` containing only a `.gitkeep` placeholder. No application code exists yet.
- Internal module layout inside `src/`: **NOT YET DECIDED** — to be defined during implementation; no module names are pre-invented in this file.

## Design Patterns

- The GUI paradigm follows egui/eframe's **immediate-mode** model (consequence of the confirmed stack choice).
- Everything else (state management, internal abstractions) is not yet decided — see `## Open Architectural Decisions`.

## Technical Decisions Summary

| Decision | Status | Source (brief section) |
| --- | --- | --- |
| Language: Rust | planned | `## Technology Stack` |
| GUI: egui/eframe | planned | `## Technology Stack` |
| Image decoding: WIC system codecs | planned | `## Technology Stack` |
| Target OS: Windows 10 / 11 | planned | `## Technology Stack` |
| Target architectures: x64 + x86 | planned | `## Technology Stack` |
| Sorting: alphanumeric fixed in v1 | v1 constraint | `## Sorting Order` |
| Last-viewed memory: session-only, RAM | v1 constraint | `### Vertical Scroll (Sibling Folders)` |
| Distribution: zip-style installer | planned | `## Distribution and Default App Registration` |

## Open Architectural Decisions

Each item below is to be decided during implementation, NOT now:

- Internal `src/` module structure and component boundaries.
- Fullscreen/zoom keybindings and zoom step sizes.
- WIC access mechanism / binding approach.
- Distribution-helper mechanism (script vs. executable).
- Config file contents/format.
- Design patterns beyond immediate-mode.
