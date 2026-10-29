# Task 3 Implementation Plan — Rewrite Root README.md

- **Workflow step**: Task 3 (TODO 20261029-todo-1) — step 4.1 (Implementation Plan), produced by Architector.
- **Implementer for 4.2**: docs-specialist (per global plan `.kilo/plans/20261029-project-brief.md`, Task 3).
- **Branch**: `feat/project-brief-and-info` (must already be checked out; verify before editing).
- **Source of truth**: [.agent/project-info/brief.md](../../.agent/project-info/brief.md). Secondary consistent phrasing sources: `product.md`, `architecture.md`, `tech.md`, `context.md`. Global Pre-analysis of `.kilo/plans/20261029-project-brief.md` is authoritative for all app decisions.
- **Deliverable**: `README.md` (repository root) — full rewrite of the base-template README.
- **Result file**: `README.md` only. No other file may be created or modified.

---

## 1. Pre-analysis

- The current `README.md` (171 lines) is the untouched base-template README ("Base Project for AI Agent Driven Development"). It describes the AI-agent template — boosters, Critical Workflow diagram, agent models, setup how-tos — none of which describe this product.
- The project is greenfield: no code, no `Cargo.toml`, `src/` holds only `.gitkeep`. Every technical claim in the new README is a *planned* decision, not a fact about existing code.
- No LICENSE file exists; no product name has been chosen (repo name `img-scroller` is provisional); no build system exists; no hotkeys/keybindings have been decided.
- Therefore: the rewrite must present the *intended v1 product* with explicit "planned / not yet implemented" framing, and must not invent badges, links, commands, shortcuts, or a license.

## 2. Rules the new README must obey

1. Self-documenting prose; descriptive headings; no code-style comments tricks.
2. Real newline characters in the file (never literal `\n` sequences).
3. No invented content: no shields/badges, no build commands (no toolchain exists to run them), no keybindings/hotkeys (undecided per brief), no product-name branding, no license claim.
4. Feature wording must match the brief's seven v1 feature headings verbatim.
5. Do NOT modify any other file: not `AGENTS.md`, not project-info files, not the TODO, not project-structure.md.
6. Keep total file moderate in size: a project README, not a second brief. Point to `brief.md` for full detail instead of duplicating it.

## 3. Exact section list and order

The new `README.md` replaces all current content with exactly these top-level (`##`) sections, in exactly this order (the `#` title line comes first; no Table of Contents):

1. `# img-scroller` (title; provisional name)
2. `## Overview`
3. `## Problem Statement`
4. `## v1 Features`
5. `## Technology Stack (Planned)`
6. `## Distribution (Planned)`
7. `## Project Status`
8. `## Build and Run (Not Yet Implemented)`
9. `## Note for AI Agents`
10. `## License`

## 4. Section-by-section content specification

### 4.1 Title

- Single line: `# img-scroller`.
- Immediately under the title, one italic line stating the naming status: the repository name `img-scroller` is a provisional working name, used until a final product name is chosen.
- Wording basis: brief `## Overview` bullet "Working project name: `img-scroller` (the repository name — provisional, used until a product name is chosen; it is not the final product name)".

### 4.2 `## Overview`

One paragraph only (3–5 sentences, bullet list NOT required, plain prose). It must state:

- A Windows desktop image viewer built in Rust.
- It recreates the behavior of the classic Windows "gallery"-era image viewer.
- Open one image and navigate: horizontally across the sibling images in the same folder, vertically across the sibling folders under the same parent folder.
- Arrow keys and on-screen UI buttons for both axes.
- Opening any image (from Explorer, a file manager, or the in-app open dialog) immediately enables this navigation in its location — no folder pre-registration or setup.

Wording basis: brief `## Overview` + `## Purpose and Goals`; keep phrases aligned with `product.md` sections "Product Summary" and "Navigation Model".

### 4.3 `## Problem Statement`

Plain prose (2–3 short paragraphs or one paragraph + two bullets). Content points:

- Modern Windows image viewers lack the classic gallery-style scrolling: after opening an image, the user cannot move to the next/previous image or jump to sibling folders with simple arrow keys in the intended way.
- Existing alternative viewers that attempt this behavior require the user to pre-register or pre-add the folder to the application before browsing — added friction.
- Consequence: browsing a library of image folders (example structure `parent\folder1`, `parent\folder2`, …) requires manual navigation or a file manager instead of direct in-viewer scrolling.
- Therefore: a viewer is needed that treats any opened image as the entry point into its folder tree, with zero folder setup.

Wording basis: brief `## Problem Statement` (reuse its sentences nearly verbatim).

### 4.4 `## v1 Features`

- Intro sentence: the v1 scope is exactly the seven features below (source: brief `## v1 Feature Scope`), each one sentence to two sentences here — README stays leaf-level; detail lives in the brief.
- Numbered list, using the brief's exact feature names as bold list labels:
  1. **Open Image with System-Registered Formats** — opens any image whose format is registered as a codec in the Windows system (e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed); the image opens in a regular resizable viewer window.
  2. **Horizontal Scroll (Sibling Images)** — left/right arrow keys and left/right UI buttons navigate to the previous/next sibling image in the same folder; scope is strictly the current folder (not recursive); order is alphanumeric.
  3. **Vertical Scroll (Sibling Folders)** — up/down arrow keys and up/down UI buttons navigate to the previous/next sibling folder under the same parent folder; when moving to a sibling folder, the filename matching the currently viewed image is preferred (same-filename rule), otherwise the target folder's first image is opened; the app also remembers the last viewed image per visited folder for the current session (RAM-only, destroyed on app close — no persistence to disk in v1).
  4. **Fullscreen Mode** — toggles fullscreen on/off via a keyboard shortcut and a UI control.
  5. **Zoom In/Out and Fit Modes** — zoom in/out via keyboard shortcut and UI controls, plus "fit width" and "fit height" modes.
  6. **Pan and Navigate at Image Edges While Zoomed** — when the zoomed image overflows the window, the arrow keys pan the image first, and navigation (images horizontally, folders vertically) triggers only when panning reaches the image's edge; when the image fits entirely, arrows navigate directly.
  7. **Fast "Open with..." Menu** — a menu entry lists programs the Windows system has registered for the currently viewed file's type, and selecting one opens the current file with it; the list is built from system registration data at runtime.
- Immediately after the list, one short note (two bullets or one short paragraph):
  - v1 sorting is alphanumeric for both folders and files; a future settings feature will let the user change it.
  - Features explicitly deferred from v1 (for example: rotation, slideshow, image editing, printing; also persistence of the last-viewed memory across sessions and a sort-order settings UI) are listed in the brief's out-of-scope section — link to `brief.md`.
- Do NOT invent any keybinding here (fullscreen/zoom bindings are undecided per brief).

Wording basis: brief `### Open Image with System-Registered Formats` through `### Fast "Open with..." Menu`, `## Sorting Order`, `## Out of Scope for v1 (Deferred)`; mirror phrasing of `product.md` "v1 User-Facing Capabilities".

### 4.5 `## Technology Stack (Planned)`

- Intro sentence: the choices below are confirmed decisions but not yet implemented — the project is greenfield with no code.
- Markdown table with exactly three columns: Component | Choice (planned) | Notes. Rows (identical values to `tech.md` "Technology Stack (Planned)"):
  1. Language | Rust | Confirmed decision; not yet implemented.
  2. GUI | egui/eframe | Pure Rust, lightweight; suitable for zoom/pan/fullscreen/keyboard shortcuts.
  3. Image decoding | Windows Imaging Component (WIC) system codecs | Accepts any image format registered in the system instead of bundling decoders.
  4. Target platform | Windows 10 and Windows 11 | Planned.
  5. Target architectures | x64 and x86 builds | Planned.
- One closing bullet: formats are supported when a system codec exists; formats not registered in the system are not supported in v1 (accepted constraint of relying on system codecs).

### 4.6 `## Distribution (Planned)`

Short bullet list (3 bullets):

- Zip-style portable installer: the user picks a destination folder; the package unzips the application binary AND its config file into that destination.
- The package includes a helper and/or guidance for registering the app as the default image viewer (file associations) in Windows.
- The helper's implementation details (script vs. executable, exact mechanism) are deferred to implementation.

Wording basis: brief `## Distribution and Default App Registration` (section title here is shortened to "Distribution (Planned)" — allowed; content points identical).

### 4.7 `## Project Status`

Short prose (2–3 sentences, no bullet list needed):

- Greenfield, planning phase: the intended v1 product is defined, but no application code exists yet (as of 2026-10-29) — no `Cargo.toml`, no `src/` code.
- Full requirements, decisions, and scope boundaries: link to [.agent/project-info/brief.md](.agent/project-info/brief.md) (relative link, exactly this path).

### 4.8 `## Build and Run (Not Yet Implemented)`

- Heading exactly: `## Build and Run (Not Yet Implemented)`.
- Content: explicitly state that this section is a placeholder: nothing is implemented yet and no build system exists, so there is nothing to build or run at the moment; this section will describe the build/run steps once the first implementation wave creates the Rust crate (planned standard `cargo` workflow documented later in `tech.md`).
- Do NOT list any commands as executable instructions (a mention that a standard Cargo workflow will apply later is acceptable if clearly future tense: "will apply once `Cargo.toml` exists").
- Keep to 2–4 sentences.

### 4.9 `## Note for AI Agents`

Short section (2–3 bullets):

- AI agents working on this project must read and adhere to [AGENTS.md](AGENTS.md) and follow [`.agent/WORKFLOWS.md`](.agent/WORKFLOWS.md) — same requirement as the brief's "Important Note for AI Agents".
- The project's source of truth for requirements and scope is [.agent/project-info/brief.md](.agent/project-info/brief.md); agents must read it before starting any task.

### 4.10 `## License`

- Content points (plain prose, 2 sentences):
  - No LICENSE file exists in the repository and the project license has not been decided/determined yet.
  - Message: until a license is added, treat the project as all-rights-implicitly-reserved by the author; do not assume redistribution rights.
- Do NOT name, suggest, or link any specific license. Do NOT create a LICENSE file.

## 5. Files to touch

| File | Action |
| --- | --- |
| `README.md` (root) | Full rewrite (overwrite all 171 lines) with the sections above |

No other file may be changed. Suggested file length target: ~90–130 lines including blank lines.

## 6. Steps for the implementer (4.2)

1. Verify branch: `git status` — must show clean (or only intended changes), on `feat/project-brief-and-info`, HEAD at/after `dd8204a`. If not, stop and return to caller.
2. Read source-of-truth for tone consistency before writing: `.agent/project-info/brief.md`, `product.md`, `tech.md` (skim), and the current `README.md`.
3. Overwrite `README.md` with the section list from Section 3 and content per Section 4, writing real newlines.
4. Self-check against Section 7 checklist; fix finding before review.
5. Hand off to 4.3 review (code-reviewer; code-simplifier N/A for Markdown, skipped per global plan).

## 7. Verification checklist (4.5b, Architector)

Git confinement:

- [ ] `git status` after 4.2 shows exactly one modified file: `README.md`. No other tracked file staged/modified; no untracked project files created.
- [ ] `git diff --stat` shows changes confined to `README.md`.
- [ ] `.gitignore` compliance: `README.md` is tracked and not ignored; staging it is safe. (Gitignore Compliance Rule applies at commit time in 4.6, not here.)

Structural presence (all mandatory):

- [ ] Title `# img-scroller` is the first line, with the provisional-name note directly under it.
- [ ] Sections exist in exactly this order: `## Overview`, `## Problem Statement`, `## v1 Features`, `## Technology Stack (Planned)`, `## Distribution (Planned)`, `## Project Status`, `## Build and Run (Not Yet Implemented)`, `## Note for AI Agents`, `## License`.
- [ ] No leftover template content (no "Base Project for AI Agent Driven Development", no "The Critical Workflow", no "Agent Models", no "Getting Started", no "Troubleshooting", no template Table of Contents, no mermaid diagram).
- [ ] File contains real newline characters (open in editor: no literal `\n` sequences visible).

Content consistency checks vs brief (.agent/project-info/brief.md):

- [ ] The 7 features are listed with the brief's exact heading names, in the brief's order.
- [ ] Feature details don't contradict the brief: current-folder-only horizontal scope; same-parent-only vertical scope; same-filename rule → first-image fallback; session-only last-viewed memory (RAM, destroyed on close); alphanumeric sort; pan-then-navigate at edges; system-built "Open with..." list.
- [ ] Tech table values identical to brief `## Technology Stack`: Rust, egui/eframe, WIC system codecs, Windows 10/11, x64 + x86, all framed as planned/not-yet-implemented.
- [ ] Distribution bullets match brief: destination-folder choice, binary + config file, default-viewer registration helper/guidance, mechanism deferred.
- [ ] Status text says greenfield / planning phase / no application code, with the as-of date 2026-10-29.
- [ ] Build/run section explicitly says not yet implemented — no code — and lists zero executable commands.
- [ ] `.agent/project-info/brief.md` is linked (relative path) in both `## Project Status` and `## Note for AI Agents` (link exact path `.agent/project-info/brief.md`).
- [ ] `AGENTS.md` link present in the AI-agents note.

Prohibition checks (nothing invented):

- [ ] No badges/shields, no CI references, no screenshots, no release links.
- [ ] No specific keybindings/hotkeys (no "F11", no "Ctrl+...", etc.).
- [ ] No license name (no MIT/Apache/GPL text, no SPDX id, no LICENSE file created).
- [ ] No commands presented as runnable today (no `cargo build` in imperative form).
- [ ] Feature list contains no extra ninth-plus feature and no deferred feature presented as in-scope.

## 8. Git actions (owned by later steps, listed for completeness)

- Step 4.2/4.3/4.4/4.5: no commits; the docs-specialist touches only `README.md`.
- Step 4.6 (implementer): mark Task 3 `[DONE]` in `.agent/todos/20261029/20261029-todo-1.md` (preserving all other file content per Overwrite TODO File Prevention), stage ONLY `README.md` + the TODO update, verify against `.gitignore`, and commit with a message in the repo's style (reference example from Task 1): `docs: rewrite README to describe the project per approved plan (task 3)`.
- Push/merge happen only in workflow Step 5 (implementer), per `.kilo/plans/20261029-project-brief.md` Step 5 — not in this task's steps.

## 9. Out of scope of this step (do not do)

- No edits to `AGENTS.md`, `.agent/project-info/*`, `.agent/todos/*`, `.agent/project-structure.md`, `docs/*`, `.kilo/*` (other than this plan file), or any source folder.
- No commit, no branch ops, no merge, no push.
- No product-name decision, no LICENSE file, no keybinding decisions.
