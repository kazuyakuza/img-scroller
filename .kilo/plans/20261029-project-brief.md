# Global Plan — 2026-10-29: Project Brief & Project Info Init (Rust Image Scroller)

## Global Pre-analysis

- **Project type**: Greenfield Rust desktop application for Windows. No code exists yet (only scaffold). This wave is **documentation only** — no application code is written.
- **Developer (junior, 50% restriction) context**: work is pure Markdown under `.agent/project-info/`; no build, no tests, no code reviews of source.
- **Decisions captured from user Q&A (source of truth for the brief)**:
  1. **Purpose**: restore the classic Windows "gallery" behavior — open an image and easily scroll through sibling images in the same folder and sibling folders under the same parent folder. Modern Windows viewers lack this scrolling behavior; existing alternatives require pre-registering the folder in the app.
  2. **Name (provisional)**: repo name `img-scroller` used until user renames.
  3. **GUI stack**: egui/eframe (pure Rust, lightweight, easy zoom/pan/fullscreen/hotkeys).
  4. **Image decoding**: system codecs via **WIC** (Windows Imaging Component) — accepts any image format registered in the system (jpg, png, bmp, gif, webp, heic when codec installed, etc.).
  5. **OS target**: Windows 10/11, x64 + x86 builds.
  6. **v1 features (user's exact list)**:
     - Open an image whose format is registered in the system; classic-view window behavior.
     - Horizontal scroll: left/right keys and UI buttons navigate sibling images in the same folder.
     - Vertical scroll: up/down keys and UI buttons navigate sibling folders under the same parent folder (same-parent-folder only, e.g. `parent\folder2\img1` → up/down → `parent\folder1` / `parent\folder3`).
       - Rule: always open the sibling folder's first image when the same filename does not exist there.
       - Rule: per-session memory of the last viewed image per folder — revisiting a folder in the same session shows that last viewed image; memory is lost when the app closes (no persistence).
     - Fullscreen mode on/off.
     - Zoom in/out, including "fit width" and "fit height".
     - Overflow behavior while zoomed: when width doesn't fit, left/right keys pan the image first and only trigger horizontal navigation at the image edges; same for vertical (height) with up/down keys.
     - Fast "Open with..." menu listing system-registered programs for the current file type.
  7. **Sort order**: alphanumeric for both folders and files; note in brief that a future settings option will let the user change it.
  8. **Distribution**: zip-style installer (user picks destination folder; unzip app + config file there), plus a way to help/explain registering the app as the default image viewer (file associations).
  9. **Deferred (explicitly not v1)**: any extra functionality from the old gallery app (rotation, slideshow, editing, printing, etc.) — stated in brief as "may be added later".
- **Front-end related?**: No GUI code in this wave; tasks are documentation tasks. Front-end 4.1a/4.5a steps are **not executed** in this wave.

## Step 2: Git Feature Branch Setup — implementer

- `git status`; commit unstaged files per Gitignore Compliance Rule; ensure on `main`; create feature branch `feat/project-brief-and-info`; switch to it.

## Step 3: Version Update — implementer

- **N/A — skip**: no version manifest exists yet (no `Cargo.toml`/`package.json`). Nothing to bump.

## Task 1: Write Project Brief — docs-related (not front-end)

Pre-analysis: single Markdown file edit: `.agent/project-info/brief.md` (currently base-template boilerplate; preserve the "Important Note for AI Agents" section as required by template).

- **4.1 Implementation plan** → architector: produce detailed section-by-section plan for the brief (structure, exact content points from pre-analysis, preserving the required trailing agent-note section).
- **4.2 Implementation** → docs-specialist (adapted: Markdown-generation rule restricts doc files to Planner/Docs Specialist, so docs tasks are implemented by docs-specialist instead of implementer). Rewrite `brief.md` per plan.
- **4.3 Review & simplification** → code-reviewer reviews brief content vs user requirements & completeness (code-simplifier: N/A for Markdown — skipped). Fix plan if required → implementer/docs-specialist applies. Max 3 review cycles, then escalate.
- **4.4 Documentation** → merged into 4.2 (the deliverable itself is documentation).
- **4.5 Verification (4.5b only)** → architector: verify brief adherence vs Q&A requirements; report diffs/plan changes.
- **4.6 Completion** → implementer: mark `[DONE]` in TODO file, commit with meaningful message.

## Task 2: Initialize remaining Project Info core files — docs-related (not front-end)

Pre-analysis: create `.agent/project-info/product.md`, `context.md`, `architecture.md`, `tech.md`; content must be consistent with the approved brief (specs files not yet initialized; these are the required core files per `.agent/project-info/instructions.md`). Architecture/tech files must reflect decisions (egui/eframe, WIC, Win10/11 x64+x86, zip installer) marked clearly as "planned/provisional" since no code exists yet.

- **4.1 Implementation plan** → architector.
- **4.2 Implementation** → docs-specialist.
- **4.3 Review & simplification** → code-reviewer (code-simplifier N/A — skipped).
- **4.4 Documentation** → merged into 4.2.
- **4.5 Verification (4.5b only)** → architector.
- **4.6 Completion** → implementer: mark `[DONE]`, commit.

## Task 3: Update root README.md — docs-related (not front-end) [added by user approval 2026-10-29]

Pre-analysis: rewrite the root template `README.md` derived from the approved brief: project name/overview, problem statement, v1 features, tech choices, status (greenfield — planning phase, no code yet), and placeholder build/run notes to be expanded when code exists.

- **4.1 Implementation plan** → architector.
- **4.2 Implementation** → docs-specialist.
- **4.3 Review & simplification** → code-reviewer (code-simplifier N/A — skipped).
- **4.4 Documentation** → merged into 4.2.
- **4.5 Verification (4.5b only)** → architector.
- **4.6 Completion** → implementer: mark `[DONE]`, commit.

## Step 5: TODO File Completion — implementer

- Rename `.agent/todos/20261029/20261029-todo-1.md` → `20261029-todo-1-DONE.md` (content unchanged).
- Remove any tmp files created during the process.
- Ensure all files are committed on the feature branch; checkout `main`; merge feature branch; delete branch after verified merge.
- If `origin` remote is set: push `main` to `origin` ONLY (per Git Remote Safety Rule).

## Step 6: Finish

- Summarize realized work; provide follow-up prompt text for the next TODO file.
