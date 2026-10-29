# Implementation Plan — Task 2: Initialize Project Info Core Files (20261029-task2-info-implementation)

## 1. Task Metadata

- **TODO**: `.agent/todos/20261029/20261029-todo-1.md` → **Task 2** only.
- **Workflow step**: 4.1b (Implementation Plan) — produced by architector. Executed next by **docs-specialist** in step 4.2 (per global plan `.kilo/plans/20261029-project-brief.md`, Task 2 section).
- **Wave date**: 2026-10-29 (single date used everywhere).
- **Branch**: `feat/project-brief-and-info` (already active; no branch operations in this task).
- **Deliverable of 4.2**: exactly 4 new files in `.agent/project-info/`:
  1. `product.md`
  2. `context.md`
  3. `architecture.md`
  4. `tech.md`
- **Source of truth**: `.agent/project-info/brief.md` (approved) + Global Plan pre-analysis (11 Q&A decisions). If any conflict is detected while writing, `brief.md` wins (per `instructions.md` "Important Notes") — stop and ask the caller instead of deviating.
- **Language**: all files in English, Markdown, UTF-8, **real newline characters** (never literal `\n`).

## 2. Constraints and No-Touch List

- Do NOT modify: `brief.md`, `instructions.md`, `.agent/project-info/.initialized`, `AGENTS.md`, `README.md` (Task 3 territory), `.agent/project-structure.md` (no folder is added — new files live in an already-listed folder), any `.agent/todos/` file (4.6 handles `[DONE]`), any `.kilo/plans/` file other than reading.
- Do NOT create folders. The 4 files go directly into the existing `.agent/project-info/` folder.
- Do NOT create additional project-info files (no specs, no summaries). Exactly 4 files.
- Do NOT commit in step 4.2. Commit is done by implementer in step 4.6 per workflow.
- Do NOT run build/test commands — none exist (greenfield; no `Cargo.toml`).
- Facts must come from `brief.md` + global plan pre-analysis. **No invented features, no invented technology names** (e.g., do not name a WIC-access crate; it is undecided). Room for "own prose" is limited to phrasing/ordering within the fixed facts: where the caller mandates a rule wording, apply it **verbatim** (marked `verbatim below`).

## 3. Pre-Execution Checks (docs-specialist, step 4.2)

1. `git branch --show-current` → must output `feat/project-brief-and-info`. If not, stop and ask caller.
2. `git status` → working tree clean (only untracked plan/temp differences reported to caller).
3. Full read of: `AGENTS.md`, `.agent/project-info/instructions.md`, `.agent/project-info/brief.md`, `.agent/todos/20261029/20261029-todo-1.md`, `.kilo/plans/20261029-project-brief.md`, `.agent/project-structure.md`, and this plan file.
4. Response opening must include `[Project Info: Active]` + confirmation phrase (instructions.md "Initial Instruction" and "Visual Anchor").

## 4. File Definitions

Heading levels below are exact (`#` / `##`). "Content points" are the facts each section must contain; implementer writes clean prose/bullets from them.

### 4.1 `.agent/project-info/product.md`

Sections in this exact order:

1. `# Product Overview`
2. `## Product Summary`
   - Provisional working name: `img-scroller` (repository name — **provisional**, not the final product name; used until a product name is chosen).
   - Windows desktop application for viewing images, built in Rust.
   - Recreates the behavior of the classic Windows "gallery"-era image viewer: open one image and easily navigate between sibling **images** in the same folder and sibling **folders** under the same parent folder.
   - Status: greenfield — planning phase; **no application code exists yet** (as of 2026-10-29).
   - Source of truth: `brief.md` in the same folder (relative link).
3. `## Problem Definition`
   - Modern Windows image viewers do not support the classic gallery-style scrolling behavior: after opening an image, the user cannot move to the next/previous image or jump to sibling folders with simple arrow keys in the intended way.
   - Existing alternative viewers that attempt this behavior require the user to pre-register or pre-add the folder to the application before browsing — friction.
   - Consequence: browsing a library of image folders (e.g., `parent\folder1`, `parent\folder2`, ...) requires manual navigation or a file manager instead of direct in-viewer scrolling.
   - Therefore: a viewer is needed that treats **any opened image as the entry point into its folder tree**, with **zero folder setup**.
4. `## Target Users and Platform`
   - Users browsing image-folder libraries organized as `parent\folder1`, `parent\folder2`, ... without pre-setup.
   - Platform: Windows 10 / Windows 11 (planned); x64 and x86 builds (planned).
5. `## Navigation Model`
   - Horizontal axis = images **within the current folder only** (not recursive): Left/Right arrow keys AND on-screen left/right UI buttons → previous/next sibling image.
   - Vertical axis = sibling folders **under the SAME parent folder** only: Up/Down arrow keys AND on-screen up/down UI buttons → previous/next sibling folder.
   - `verbatim`: "Navigation is restricted to sibling folders under the SAME parent folder ... never to folders of `parent` (no grandparent-level jumping), and never recursively deeper."
   - Opening ANY image file (from Explorer, a file manager, or the in-app open dialog) immediately enables full image/folder scrolling in its location — zero folder setup required.
6. `## v1 User-Facing Capabilities`
   - Exactly the **7 v1 features** (numbered list, each tied back to its brief subsection via relative link or section title):
     1. **Open Image with System-Registered Formats** — opens any image whose format is registered as a Windows codec (via WIC), e.g. jpg, png, bmp, gif, webp, heic (when a system codec is installed); opens in a regular **resizable** viewer window (not maximized, not a modal dialog).
     2. **Horizontal Scroll (Sibling Images)** — Left/Right keys + left/right UI buttons navigate previous/next sibling image in the same folder; strict current-folder scope; order = alphanumeric (per `## Sorting Order`).
     3. **Vertical Scroll (Sibling Folders)** — Up/Down keys + up/down UI buttons navigate sibling folders under the same parent.
        - Same-filename rule, `verbatim`: "when moving to a sibling folder, the app tries to show the image with the SAME filename as the currently viewed one; if that filename does not exist in the target folder, the app opens the target folder's FIRST image (per the alphanumeric sort order)".
        - Last-viewed memory rule (per session): the app remembers the last viewed image of each visited folder; revisiting a folder in the SAME session reopens that last viewed image — takes **priority** over the same-filename rule and the first-image rule when the folder was already visited in the session; `verbatim`: "the memory lives only in RAM for the current session and is destroyed when the application closes — there is NO persistence to disk in v1".
     4. **Fullscreen Mode** — toggle on/off via keyboard shortcut AND UI control; exact bindings decided during implementation.
     5. **Zoom In/Out and Fit Modes** — zoom in/out via keyboard shortcut and UI controls; "fit width" mode (image width fits window); "fit height" mode (image height fits window); zoom step sizes are implementation details, not brief content.
     6. **Pan and Navigate at Image Edges While Zoomed** — `verbatim` width rule: "When the zoomed image is WIDER than the window, the left/right arrow keys pan the image horizontally; horizontal image navigation (previous/next sibling image) triggers only when panning reaches the image's left or right edge."; `verbatim` height rule: "When the zoomed image is TALLER than the window, the up/down arrow keys pan the image vertically; folder navigation (previous/next sibling folder) triggers only when panning reaches the image's top or bottom edge."; `verbatim` no-overflow case: "When the image fits entirely (no overflow), arrows navigate directly."
     7. **Fast "Open with..." Menu** — menu entry lists programs the Windows system has registered for the currently viewed file's type/extension; selecting one opens the current file with it; list built from system registration data at runtime — no internal program database.
   - Sorting note (single paragraph at the end of this section): v1 provisioning is `verbatim` "alphanumeric for BOTH folders and files. All navigation (horizontal image order, vertical folder order, 'first image' fallback, last-viewed revisit) follows this single order." A future settings feature will let the user change the sort order; **v1 ships fixed alphanumeric with no settings UI**.
7. `## Product Goals`
   - Purpose: restore the classic Windows "gallery" viewing behavior as a modern, lightweight Rust application.
   - Zero-setup entry: opening ANY image immediately enables full image/folder scrolling in its location.
   - Simple, discoverable navigation — keyboard arrows AND on-screen UI buttons for both axes.
   - Zooming (including fit width / fit height) and fullscreen viewing with clean pan-then-navigate behavior while zoomed.
   - Leverage the operating system instead of duplicating it — system-registered codecs for decoding; system-registered programs in "Open with...".
   - Easy to install/distribute as a portable, zip-style package.
8. `## Distribution Experience`
   - Zip-style portable installer; user picks a destination folder; the package unzips the application binary AND its config file into that destination.
   - A helper and/or guidance helps the user register the app as the default image viewer (file associations) in Windows.
   - Helper implementation details (script vs. executable, exact mechanism) are **deferred to implementation**.
9. `## Explicit Non-Goals (Deferred)`
   - List verbatim set from brief `## Out of Scope for v1 (Deferred)`: rotation; slideshow; image editing; printing; any further functionality inherited from the old gallery app beyond the 7 v1 features; settings UI for changing the sort order; persistence of the per-folder last-viewed memory across sessions.
   - Frame as "explicitly NOT in v1; may be added later".

### 4.2 `.agent/project-info/context.md`

Sections in this exact order:

1. `# Project Context Log`
2. `## How to Use This File`
   - Required note (mandated by `instructions.md`, include as bullets):
     - Agents MUST **automatically read** this file at the start of EVERY task ("Automatic Read") and must include `[Project Info: Active]` plus the confirmation phrase (building `img-scroller`, current focus) in the first response.
     - Agents MUST update this file (recent changes → current state → next steps) **before emitting the final completion signal** ("Critical Closing Step").
     - If the context window fills during a long session: write the session state here, then recommend starting a new chat (which resumes from this file).
     - `architecture.md`, `tech.md`, `product.md` are read on demand — when a task directly impacts them.
     - All rules live in `.agent/project-info/instructions.md` (link).
3. `## Current Work Focus`
   - Dated 2026-10-29. Executing the wave defined in `.agent/todos/20261029/20261029-todo-1.md` and global plan `.kilo/plans/20261029-project-brief.md`: **Project Info initialization** (documentation-only wave, no code).
   - Active item during this wave: **Task 2** — creating the 4 remaining core project-info files (`product.md`, `context.md`, `architecture.md`, `tech.md`) derived from the approved brief; plan: `.kilo/plans/20261029-task2-info-implementation.md`.
4. `## Recent Changes`
   - Numbered/chronological factual log, each line prefixed `2026-10-29:`:
     1. Feature branch `feat/project-brief-and-info` created from `main` per Critical Workflow step 2.
     2. Version-update step (workflow step 3) skipped: no version manifest exists (no `Cargo.toml` / `package.json`) — recorded as N/A in the global plan.
     3. Task 1 [DONE]: `.agent/project-info/brief.md` rewritten per approved plan (overview, problem statement, v1 feature scope of 7 features, Q&A decisions, deferred scope); commit `fa83030` "docs: write project brief per approved plan (task 1)".
     4. Task 2: the 4 remaining core project-info files created from the approved brief (this wave; plan `20261029-task2-info-implementation.md`).
   - List only what actually happened in this wave — no outside-work items.
5. `## Current Project State`
   - Greenfield: **no source code, no `Cargo.toml`, no `src/` folder yet.** Repository is docs/config scaffolding only: `.agent/`, `.kilo/`, `.opencode/`, `docs/` (per `.agent/project-structure.md`), plus a template `README.md` (still template until Task 3 rewrite).
   - Project Info: complete core set present after this task — `brief.md`, `product.md`, `context.md`, `architecture.md`, `tech.md`, plus `instructions.md` (and the `.initialized` marker file).
   - Git: branch `feat/project-brief-and-info` active; `main` not yet merged with this branch (merge happens at workflow Step 5).
6. `## Immediate Next Steps`
   1. Finish this wave: Task 3 — rewrite root `README.md` (overview, v1 features, tech choices, greenfield status, placeholder build/run notes).
   2. Workflow Step 5 (per global plan): mark TODO done/rename, merge `feat/project-brief-and-info` into `main`, push to `origin` only if set.
   3. After the wave: **start implementation of the app skeleton** — first coding wave of the greenfield app, per the planned stack (Rust + egui/eframe + WIC decoding; see `architecture.md` / `tech.md` "planned" notes). Decisions marked "planned/not yet implemented" there feed this work.
   4. Open follow-ups pending caller decision (do not self-resolve): linking the new project-info files in `AGENTS.md` (required by `instructions.md` "Integration in AGENTS.md" but outside the 4-file scope of Task 2) and handling of the `.agent/project-info/.initialized` marker (see plan Section 8).

### 4.3 `.agent/project-info/architecture.md`

Every architectural/technical statement in this file must carry a **planned/not-yet-implemented** marker (or a leading "Status" section stating it once, applying to the whole file). Sections in this exact order:

1. `# System Architecture`
2. `## Status`
   - All content below describes the **planned** architecture for a greenfield project; **nothing is implemented yet** (no code, no `Cargo.toml`, as of 2026-10-29). Facts sourced from `brief.md` (source of truth).
3. `## Overview`
   - Planned shape: single-purpose Windows desktop GUI application — an image viewer with two-axis folder navigation; no server/back-end; ships as a single application binary + config file (distribution flow in `## Data and State` / `tech.md`).
   - Three pillars chosen in the brief: egui/eframe GUI (planned), WIC system-codec decoding (planned), and reliance on Windows system registration data (codecs, "Open with..." program list).
4. `## Logical Components (Planned)`
   - Bulleted component list — responsibilities only, sourced from the v1 features; **no invented modules**. State explicitly that this is a logical decomposition and that concrete module/file layout is undecided (see `## Open Architectural Decisions`):
     - **Viewer UI (egui/eframe, immediate-mode, planned)**: regular resizable window; renders the image; hosts the on-screen left/right/up/down navigation buttons, fullscreen toggle, zoom controls (incl. fit width/fit height), and the fast "Open with..." menu entry.
     - **Image Decoding Layer (WIC, planned)**: decodes any image format registered as a Windows system codec; no bundled decoders; formats without a system codec are unsupported in v1 (accepted constraint).
     - **Navigation Engine (planned)**: horizontal = previous/next sibling image in the current folder (alphanumeric order); vertical = previous/next sibling folder under the SAME parent (same-filename rule → first-image fallback in alphanumeric order).
     - **Session Last-Viewed Memory (planned)**: RAM-only map of last viewed image per visited folder; last-viewed has priority over same-filename and first-image rules when the folder was already visited this session; destroyed on app close; no disk persistence in v1.
     - **Pan/Zoom Controller (planned)**: zoom in/out, fit width/fit height; pan-then-navigate behavior — width-overflowed image: left/right keys pan until the image's horizontal edge, then trigger horizontal navigation; height-overflowed image: up/down keys pan until the vertical edge, then trigger folder navigation; no overflow: arrows navigate directly.
     - **Fast "Open with..." Service (planned)**: at runtime queries the Windows system for programs registered for the current file's type/extension; no internal program database.
     - **Distribution Packaging (planned)**: zip-style installer (destination-folder selection; unzips app binary + config) plus default-viewer registration helper/guidance — helper mechanism deferred.
5. `## Navigation Critical Paths (Planned)`
   - Numbered end-to-end flows (this is the file's "critical paths" content):
     1. **Entry**: open an image (Explorer association, file manager, or in-app open dialog) → decode via WIC system codec → display in regular resizable window → navigation enabled immediately; no folder pre-registration step exists.
     2. **Horizontal navigation**: arrow key or UI button → previous/next sibling image by alphanumeric order in the current folder → decode → display; while the image is wider than the window, pan first; navigation only at the image's left/right edge.
     3. **Vertical navigation**: arrow key or UI button → previous/next sibling folder under the same parent → target image chosen by priority: (a) last viewed this session (if folder visited) → (b) same filename if it exists → (c) target folder's first image (alphanumeric); decode → display; while the image is taller than the window, pan first; navigation only at the image's top/bottom edge.
     4. **Fullscreen / zoom toggles**: keyboard shortcut and UI control toggle fullscreen; zoom via shortcut + controls; fit width/fit height modes.
     5. **Open with**: menu entry → enumerate system-registered programs for the current file type → selected program opens the current file.
6. `## Data and State (Planned)`
   - In-memory (per session): currently viewed image path / folder context; per-folder last-viewed map (RAM-only, destroyed on close — no persistence in v1); window/zoom state of the session.
   - On disk: an application **config file** shipped by the installer alongside the binary (brief distribution flow). Config contents/format are **not defined anywhere yet** — record existence only; contents are an implementation detail to be decided.
   - No other persistent state in v1; explicitly no persistence of last-viewed memory.
7. `## Project Paths`
   - All source code must live in `src/` (project structure rule; folder map in `.agent/project-structure.md`).
   - Planned: Cargo project at the repository root with `Cargo.toml` (does **not** exist yet; created in the first implementation wave).
   - Current repository layout is docs/config scaffolding only: `.agent/`, `.kilo/`, `.opencode/`, `docs/`. `src/` does not exist yet.
   - **Internal module layout inside `src/`: NOT YET DECIDED** — to be defined during implementation; do not pre-invent module names in this file.
8. `## Design Patterns`
   - GUI paradigm follows egui/eframe's **immediate-mode** model (consequence of the confirmed stack choice).
   - Everything else (state management, internal abstractions) is **not yet decided**; keep an open-decisions pointer to the list below.
9. `## Technical Decisions Summary`
   - Table with columns: Decision | Status | Source (brief section). Rows:
     - Language: Rust | planned | `## Technology Stack`
     - GUI: egui/eframe | planned | `## Technology Stack`
     - Image decoding: WIC system codecs | planned | `## Technology Stack`
     - Target OS: Windows 10 / 11 | planned | `## Technology Stack`
     - Target architectures: x64 + x86 | planned | `## Technology Stack`
     - Sorting: alphanumeric fixed in v1 | v1 constraint | `## Sorting Order`
     - Last-viewed memory: session-only, RAM | v1 constraint | `### Vertical Scroll (Sibling Folders)`
     - Distribution: zip-style installer | planned | `## Distribution and Default App Registration`
10. `## Open Architectural Decisions`
    - Explicit undecided list (each to be decided during implementation, NOT now): internal `src/` module structure and component boundaries; fullscreen/zoom keybindings; zoom step sizes; WIC access mechanism / binding approach; distribution-helper mechanism (script vs. executable); config file contents/format; design patterns beyond immediate-mode.

### 4.4 `.agent/project-info/tech.md`

Every stack/setup statement must be marked planned (or covered by a leading Status section). Sections in this exact order:

1. `# Technical Specifications`
2. `## Status`
   - Planned stack for a greenfield project; **nothing implemented yet** (no code, no `Cargo.toml`, as of 2026-10-29). Facts sourced from `brief.md` (source of truth) + global plan decisions.
3. `## Technology Stack (Planned)`
   - Table: Component | Choice (planned) | Rationale/notes:
     - Language | Rust | confirmed decision in brief; not yet implemented.
     - GUI | egui/eframe | pure Rust, lightweight; suitable for zoom/pan/fullscreen/keyboard shortcuts.
     - Image decoding | Windows Imaging Component (WIC) system codecs | the app accepts any image format registered in the system instead of bundling its own decoders.
     - Target platform | Windows 10 and Windows 11.
     - Target architectures | x64 and x86 builds.
4. `## Image Format Support`
   - Opens any image whose format is registered as a codec in the Windows system (via WIC), e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed.
   - Formats not registered in the system are **not supported in v1** — accepted constraint of relying on system codecs.
5. `## Dependencies (Planned/To Be Defined)`
   - Confirmed planned: egui/eframe crates (GUI) — become Cargo dependencies once the crate exists.
   - Open: the crate/approach used to invoke WIC system codecs is **not yet decided** — record as undecided, do not name a crate.
   - No other dependencies decided yet.
6. `## Development Environment and Setup (Placeholder)`
   - Planned toolchain: Rust via the standard Rust toolchain installer (`rustup`) and `cargo` — typical commands; nothing installed/verified yet (no toolchain requirements recorded anywhere; expand during the first implementation wave).
   - Never install tools globally — in-project installs only (rule `.kilo/rules/never-global-installs.md`).
7. `## Build and Run (Placeholder)`
   - No build system exists yet. Once `Cargo.toml` exists, the standard workflow applies (marked planned): `cargo build` (debug), `cargo build --release` (release), `cargo run` (dev launch).
   - x64/x86 build specifics (e.g., exact target triples) are **to be defined during toolchain setup** — do not invent them here.
8. `## Testing (Placeholder)`
   - No test suite and no testing framework exist; nothing decided yet. This section expands during implementation. (Unit-test quests per code-guidelines apply once code exists.)
9. `## Technical Constraints`
   - Windows-only application (Windows 10/11); x64 + x86 builds.
   - Format support depends on the user's system codecs (unsupported formats exist by design).
   - No bundled image decoders.
   - Last-viewed per-folder memory is session-only, RAM-held, destroyed on close — **no persistence to disk in v1**.
   - v1 sort order fixed alphanumeric — no settings UI.
   - Fullscreen/zoom keybindings and zoom step sizes are implementation details, not fixed here.
10. `## Distribution and Packaging (Planned)`
    - Zip-style portable installer: user picks the destination folder; package unzips the application binary AND its config file there.
    - Default image-viewer registration: the package includes a helper and/or guidance for registering the app as the default image viewer (file associations); mechanism deferred to implementation.
11. `## Tool Usage Patterns for AI Agents`
    - Required reads per task start: `AGENTS.md`, `.agent/project-info/instructions.md`, `.agent/project-info/context.md`; workflows list: `.agent/WORKFLOWS.md` (Critical Workflow governs task execution).
    - Tool preference: follow `.kilo/rules/tool-selection-priority.md` (semantic/code-aware tools first; built-in glob/grep/read/edit/write next; shell only for CLI-native ops; single unchained commands; PowerShell last resort).
    - Markdown/docs: created by Planner Agent or Docs Specialist only (`.kilo/rules/markdown-generation-rule.md`); plans in `.kilo/plans/`, TODOs in `.agent/todos/<YYYYMMDD>/`.
    - Code location and style rules to apply once code exists: all source in `src/` (`.kilo/rules/project-structure.md`), self-documenting code, method/file size limits, min 2-depth nesting, private-by-default members (links to the individual rule files in `.kilo/rules/`).
12. `## References`
    - `brief.md` (source of truth — requirements/scope), `.agent/project-info/instructions.md` (project-info rules), `.agent/project-structure.md` (structure map), `.kilo/plan/20261029-project-brief.md` — typo? No: use correct path `.kilo/plans/20261029-project-brief.md` (Q&A decisions log).

## 5. Cross-File Consistency Requirements (verification targets)

1. **Facts matrix** — these must be stated **without contradiction** in all files where they appear (product/context/architecture/tech), each traceable to `brief.md`:
   - Provisional name `img-scroller`, not final product name.
   - Greenfield status; no code/Cargo.toml/src yet; date 2026-10-29.
   - Stack: Rust / egui/eframe / WIC system codecs / Windows 10-11 / x64+x86 — always "planned".
   - Exactly 7 v1 features (no 8th feature anywhere; no renamed features).
   - Same-filename rule → first-image fallback (alphanumeric).
   - Last-viewed per-folder session memory with priority, RAM-only, no persistence.
   - Pan-then-navigate at edges (width → horizontal nav; height → folder nav; no overflow → direct).
   - Sorting alphanumeric for folders AND files in v1; future settings feature lets user change it.
   - Fast "Open with..." from system registration, no internal program database.
   - Distribution: zip installer (destination folder; unzips binary + config) + default-viewer registration helper/guidance.
   - Out-of-scope list identical to brief (7 items: rotation, slideshow, editing, printing, further old-gallery functionality, sort-settings UI, cross-session persistence).
2. **Dates**: only 2026-10-29 for this wave's events. Recent-changes entries in `context.md` must not include invented dates or future events.
3. **Rules wording**: the rules marked `verbatim in Section 4` must appear word-for-word (or near-verbatim, same facts and emphasis) — not paraphrased loosely.
4. **Markers**: no "is/has/does" phrasing implying implemented behavior in architecture/tech; use "(planned)" / "not yet implemented" / "placeholder".
5. **No invented decisions**: no crate names for WIC access, no keybindings, no zoom steps, no target triples, no config file contents, no module names in `src/`.
6. **Links**: all cross-project-info links are relative (same folder); external paths written exactly (`brief.md`, `instructions.md`, `.kilo/plans/20261029-project-brief.md`).
7. `context.md` must not contradict `brief.md`/`architecture.md`/`tech.md`; any inconsistency resolution defers to `brief.md` and is flagged to the caller.

## 6. Execution Steps (docs-specialist, step 4.2 — atomic, in order)

1. Run Section 3 pre-execution checks; stop on any failure.
2. Create `.agent/project-info/product.md` per Section 4.1 (write tool; real newlines; UTF-8; no BOM concerns noted — plain Markdown).
3. Create `.agent/project-info/context.md` per Section 4.2.
4. Create `.agent/project-info/architecture.md` per Section 4.3.
5. Create `.agent/project-info/tech.md` per Section 4.4.
6. Apply Section 5 consistency matrix while writing (fix within the 4 files if a bullet was written inconsistently).
7. Run the per-file checklists (Section 7.1) and the global checklist (Section 7.2).
8. Run confinement check (Section 7.3). Do NOT stage or commit.
9. Report completion summary (done vs. not done) to the caller; do NOT proceed to 4.3/4.6.

## 7. Verification Checklists

### 7.1 Per-file

**`product.md`**
- [ ] All 9 headings exist, exactly ordered as Section 4.1; no extra/renamed top-level sections.
- [ ] Contains: provisional-name note; greenfield status + 2026-10-29; relative link to `brief.md`.
- [ ] All 7 v1 features listed, numbered, matching brief subsection names; verbatim/near-verbatim rules present (same-filename/first-image, last-viewed priority + RAM-only/no-persistence, pan-then-navigate width/height/no-overflow).
- [ ] Non-goals list matches brief exactly (7 items).
- [ ] No invented features, no settings UI, no roadmap additions.

**`context.md`**
- [ ] All 6 headings exist in order (Section 4.2).
- [ ] "How to Use This File" includes: auto-read at task start, `[Project Info: Active]` + confirmation phrase, update-before-closing mandate, context-window overflow protocol, on-demand reads, link to `instructions.md`.
- [ ] Current Work Focus states: the 2026-10-29 wave (todo + global plan paths) and Task 2 active.
- [ ] Recent Changes lists only this wave's steps (branch, version-step N/A, Task 1 [DONE] + commit `fa83030`, Task 2 files created), each dated 2026-10-29; no outside events.
- [ ] Current Project State includes: no code/Cargo.toml/src; scaffolding-only repo; template README until Task 3; branch name; core project-info set now complete.
- [ ] Immediate Next Steps includes Task 3, workflow Step 5, then "start implementation of the app skeleton" (planned stack pointer), plus the 2 open follow-ups (AGENTS.md links; `.initialized` marker).

**`architecture.md`**
- [ ] All 10 headings exist in order (Section 4.3).
- [ ] Status section declares planned/not-implemented with date.
- [ ] Logical Components listed with plausibly-traceable responsibilities; internal module layout explicitly "NOT YET DECIDED".
- [ ] Navigation Critical Paths: entry, horizontal, vertical (priority order a→b→c correct), fullscreen/zoom, open-with.
- [ ] Data and State: config file existence stated; contents marked undecided; memory RAM-only/no persistence.
- [ ] Technical Decisions Summary table matches Section 4.3 rows exactly.
- [ ] Open Architectural Decisions list matches Section 4.3 exactly (6 items).

**`tech.md`**
- [ ] All 12 headings exist in order (Section 4.4).
- [ ] Stack table rows exactly as Section 4.4; all marked planned.
- [ ] Format support: registered-codec model with the named examples, heic conditional, non-registered unsupported in v1.
- [ ] No invented crate names for WIC access; no target triples; no invented commands beyond the standard `cargo build`/`--release`/`run` placeholders, all marked planned.
- [ ] Constraints section matches brief; distribution section matches brief (binary + config unzip; registration helper deferred).
- [ ] References section has correct relative paths (no typo like `.kilo/plan/`).

### 7.2 Global

- [ ] `.agent/project-info/` contains `product.md`, `context.md`, `architecture.md`, `tech.md` — and **no other new files** (existing items `brief.md`, `instructions.md`, `.initialized` untouched).
- [ ] Only the 4 files modified across the repo; no further edits.
- [ ] All dates in the wave = 2026-10-29; no contradictions between files and vs `brief.md`.
- [ ] Files use real newlines/UTF-8 (visually verify one multi-line file renders plainly; no literal `\n` inside lines).

### 7.3 Git Confinement Check

- Run: `git status --porcelain` — expected output exactly:
  ```
  ?? .agent/project-info/architecture.md
  ?? .agent/project-info/context.md
  ?? .agent/project-info/product.md
  ?? .agent/project-info/tech.md
  ```
  (order may vary; content must not). Also run `git diff` — must be empty.
- If anything else appears (modified/staged/untracked beyond the 4): stop, report the diff to the caller; do not revert on your own and do not commit.

## 8. Open Questions for Caller (not blockers for 4.2; flagged, resolved elsewhere)

1. `instructions.md` ("Integration in AGENTS.md") requires all project-info files to be linked in `AGENTS.md`, but the approved Task 2 scope is only the 4 files (verification demands "only these modified"). Recommend a follow-up TODO item (Planner's decision) to add links in `AGENTS.md`.
2. The Project Info Initialization workflow (`instructions.md` + `.kilo/commands/project-info-init.md`) mentions removing `.agent/project-info/.initialized` after initialization; Task 2 does not mention it. Left untouched by this plan — confirm whether/how to remove it (likely as part of Task 2 completion or Step 5).

## 9. Handoff Notes

- Consumed in 4.2 by **docs-specialist** (Markdown-generation rule: docs restricted to Planner/Docs Specialist). Caller's prompt must include: TODO path, global plan path, this plan path, and the "SUB-AGENT TASK — SINGLE DISCRETE STEP" prefix per Critical Workflow.
- Commit decision belongs to implementer in 4.6 (`docs: ...` style message, e.g. `docs: initialize project info core files (task 2)`), after 4.3 review fixes if any.
- Max 3 review cycles in 4.3; escalate to user after that.
