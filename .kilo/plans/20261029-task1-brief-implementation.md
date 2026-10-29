# Task 1 Implementation Plan — Rewrite `.agent/project-info/brief.md` (Project Brief for img-scroller)

- **Workflow step**: Task 1, step 4.1 (Implementation Plan) of the Critical Workflow.
- **Source of truth**: `.kilo/plans/20261029-project-brief.md` → "Global Pre-analysis" section (all user-approved Q&A decisions).
- **Assignment**: Task 1 in `.agent/todos/20261029/20261029-todo-1.md`.
- **Implementer of the plan below**: docs-specialist (Markdown-generation rule restricts doc files to Planner/Docs Specialist). Critical Workflow steps 4.3 (review), 4.5b (verification), 4.6 (TODO mark + commit) are separate steps — NOT covered by this plan.

---

## 1. Scope Guard (read first)

The implementer performs EXACTLY these actions and nothing else:

- Rewrite ONE file: `.agent/project-info/brief.md`.
- NO other file may be created, edited, or deleted in this task (not `README.md`, not `product.md`/`context.md`/`architecture.md`/`tech.md` — those belong to Tasks 2 and 3).
- NO git commands (branch already exists: `feat/project-brief-and-info`; commit happens in Step 4.6 by implementer, not here).
- NO build/test/tooling steps (Markdown-only change; no build exists yet — greenfield).
- Do NOT touch the `.agent/todos/20261029/20261029-todo-1.md` file (Overwrite-TODO-File-Prevention rule; TODO marking is Step 4.6).

---

## 2. Preservation Requirement (CRITICAL)

The current file ends with a template-protected block (current lines 26–37):

```
<!-- DO NOT DELETE NEXT SECTION -->

## Important Note for AI Agents

All agents working on this project MUST adhere to the workflows and rules outlined in [AI Agent Onboarding document](../../AGENTS.md).

Before starting any task:

1. **Review `AGENTS.md`**: is the primary source of instructions for agents.
2. **Follow Workflows**: follow the procedures defined in `.agent/WORKFLOWS.md`, especially the `.kilo/commands/critical-workflow.md`.

<!-- END DO NOT DELETE -->
```

Rules for the rewrite:

- This block MUST be copied into the new file **verbatim**: same heading text (`## Important Note for AI Agents`), same relative link path `../../AGENTS.md`, same two numbered items, same `<!-- DO NOT DELETE NEXT SECTION -->` and `<!-- END DO NOT DELETE -->` marker comments, in the same order.
- The markers must remain HTML comments on their own lines (visibility of the protected boundaries).
- This is the LAST content in the file (nothing may be appended after `<!-- END DO NOT DELETE -->`).
- Everything ABOVE (`# Project Brief` title may stay; all of Overview / Core Requirements / Goals / Project Scope boilerplate) is REPLACED per Section 3 below.
- Use REAL newline characters in the file content — never literal `\n` escape sequences (Newline-Prevention rule).

---

## 3. Required File Structure — exact section list and order

The rewritten `.agent/project-info/brief.md` MUST contain exactly these sections, in this order (top-level `##` headings; do not add, remove, rename, or reorder):

1. `# Project Brief` (document title — keep this heading text)
2. `## Overview`
3. `## Problem Statement`
4. `## Purpose and Goals`
5. `## v1 Feature Scope`
6. `## Sorting Order`
7. `## Distribution and Default App Registration`
8. `## Technology Stack`
9. `## Out of Scope for v1 (Deferred)`
10. `<!-- DO NOT DELETE NEXT SECTION -->` + `## Important Note for AI Agents` + `<!-- END DO NOT DELETE -->` block (verbatim, as per Section 2)

Format constraints:

- Use Markdown headings hierarchy consistently: `#` for the document title, `##` for the top-level sections listed above, `###` only for the per-feature subsections inside `## v1 Feature Scope` (see 3.5).
- Content tone: factual, concise, declarative statements (this file is the source of truth for project scope per `.agent/project-info/instructions.md`). Tutorial prose or filler sentences are not wanted.
- Target length of the whole file (including the preserved block): roughly 110–170 lines. The old boilerplate from the current file (lines 3–24: "base project", "AI-Readiness", "Kilo Code", "scaffolding tool" content) must NOT survive anywhere in the new content. The word "Kilo Code" may appear ONLY inside the preserved block area — in practice it does not appear there, so the new written content must avoid it entirely.
- Do not add comments other than the two required `<!-- ... -->` marker lines.

### 3.1 Section: `## Overview`

Content points (every bullet is an explicit decision — include all):

- The project is a Windows desktop application for viewing images, built in Rust.
- It recreates the behavior of the classic Windows "gallery"-era image viewer: open one image and easily navigate between the sibling images in the same folder and the sibling folders under the same parent folder.
- Working project name: `img-scroller` (the repository name — provisional, used until a product name is chosen; state this explicitly so the file doesn't imply it's final).
- Project status: greenfield — planning phase; no application code exists yet. The brief defines the intended v1 product.
- One-line summary of the navigation model: horizontal navigation moves across images within the current folder; vertical navigation moves across sibling folders under the same parent folder.

### 3.2 Section: `## Problem Statement`

Content points:

- Modern Windows image viewers do not support the classic gallery-style scrolling behavior: after opening an image, the user cannot move to the next/previous image or jump to sibling folders with simple arrow keys in the intended way.
- Existing alternative viewers that attempt this require the user to pre-register or pre-add the folder to the application before browsing, which adds friction.
- Consequence: browsing a library of image folders (e.g., a photo archive organized in `parent\folder1`, `parent\folder2`, ...) requires manual navigation or a file manager instead of direct in-viewer scrolling.
- Therefore: a viewer is needed that treats any opened image as the entry point into its folder tree, without any pre-registration of folders.

### 3.3 Section: `## Purpose and Goals`

Content points:

- Purpose: restore the classic Windows "gallery" viewing behavior as a modern, lightweight Rust application.
- Goal: opening ANY image file (from Explorer, file manager, or in-app open dialog) immediately enables full image/folder scrolling in its location — zero folder setup required.
- Goal: simple, discoverable navigation — keyboard arrows AND on-screen UI buttons for both horizontal (images) and vertical (folders) movement.
- Goal: support zooming (including fit width / fit height) and fullscreen viewing with clean pan-then-navigate behavior while zoomed (see feature 5 and feature 6 content points in 3.5).
- Goal: leverage the operating system instead of duplicating it — decode images with system-registered codecs and offer system-registered programs in an "Open with..." menu (see features 1 and 7).
- Goal: remain easy to install and distribute as a portable, zip-style package (see `## Distribution and Default App Registration`).

### 3.4 Section: `## Purpose and Goals` — NOT a separate scope section

No separate "Core Requirements" section is created. The old template headings (`Core Requirements`, `Goals`, `Project Scope`) are replaced by the sections in this plan; do not keep the old heading names.

### 3.5 Section: `## v1 Feature Scope`

MUST contain exactly 7 subsections (`### 5.1` numbering of the headings is NOT added to the text; use plain `### Feature name` headings), in this exact order and with these exact content points. These 7 features are the user's approved v1 list (from the TODO Q&A — section 6, items of the global plan pre-analysis) and must remain 7 distinct features in this order.

#### Subsection: `### Open Image with System-Registered Formats`

- The app opens any image whose format is registered as a codec in the Windows system (via WIC — see `## Technology Stack`), e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed.
- Formats not registered in the system are not supported in v1 (this is the accepted constraint of relying on system codecs — state it here).
- Classic-view window behavior on open: the image opens in a regular resizable viewer window consistent with the classic viewer experience (not maximized, not a modal dialog).

#### Subsection: `### Horizontal Scroll (Sibling Images)`

- Left arrow key and a left UI button navigate to the previous sibling image in the same folder.
- Right arrow key and a right UI button navigate to the next sibling image in the same folder.
- Scope is strictly the current folder's images (not recursive).
- Image order follows the sort order defined in `## Sorting Order` (alphanumeric).
- While the image is zoomed beyond the window width, left/right keys pan first — see `### Pan and Navigate at Image Edges While Zoomed` below.

#### Subsection: `### Vertical Scroll (Sibling Folders)`

- Up arrow key and an up UI button navigate to the previous sibling folder; down arrow key and a down UI button navigate to the next sibling folder.
- Navigation is restricted to sibling folders under the SAME parent folder. Example semantics (include this example in the section): viewing `parent\folder2\img1` and pressing up/down moves to an image in `parent\folder1` or `parent\folder3` — never to folders of `parent` (no grandparent-level jumping), and never recursively deeper.
- `### Same-Filename Rule` (include as a bullet or short sub-list in this subsection — wording decision: present as a named rule bullet "Same-filename rule:"):
  - When moving to a sibling folder, the app tries to show the image with the SAME filename as the currently viewed one.
  - If that filename does not exist in the target folder, the app opens the target folder's FIRST image (per the alphanumeric sort order in `## Sorting Order`).
- `### Per-Session Last-Viewed Memory` (include as a second named rule bullet "Last-viewed memory rule (per session):"):
  - While navigating folders during one app session, the app remembers the last viewed image of each visited folder.
  - Revisiting a folder in the SAME session reopens that last viewed image (this takes priority over the same-filename rule and the first-image rule when the folder was already visited in the session).
  - The memory lives only in RAM for the current session and is destroyed when the application closes — there is NO persistence to disk in v1.

#### Subsection: `### Fullscreen Mode`

- Toggle fullscreen on/off.
- Both a keyboard shortcut and a UI control toggle it (the specific binding is decided during implementation; the brief only requires that both exist — phrase it as "via keyboard shortcut and UI control, exact bindings to be decided during implementation"). This phrasing is the pre-agreed way to avoid inventing bindings now.

#### Subsection: `### Zoom In/Out and Fit Modes`

- Zoom in / zoom out via keyboard shortcut and UI controls.
- Include explicit "fit width" mode: scale the image so its width fits the window.
- Include explicit "fit height" mode: scale the image so its height fits the window.
- Specific zoom step sizes are implementation details, not brief content.

#### Subsection: `### Pan and Navigate at Image Edges While Zoomed`

- Overflow behavior while zoomed in (the approved Q&A behavior, stated for both axes):
  - When the zoomed image is WIDER than the window, the left/right arrow keys pan the image horizontally; horizontal image navigation (previous/next sibling image) triggers only when panning reaches the image's left or right edge.
  - When the zoomed image is TALLER than the window, the up/down arrow keys pan the image vertically; folder navigation (previous/next sibling folder) triggers only when panning reaches the image's top or bottom edge.
- When the image fits entirely (no overflow), arrows navigate directly.

#### Subsection: `### Fast "Open with..." Menu`

- A menu entry opens a "fast open-with" list of programs that the WINDOWS SYSTEM has registered for the currently viewed file's type/extension.
- Selecting a program opens the current file with it.
- The list is built from system registration data at runtime — the app does not maintain its own program database.

### 3.6 Section: `## Sorting Order`

Content points:

- v1 sorting: alphanumeric for BOTH folders and files. All navigation (horizontal image order, vertical folder order, "first image" fallback, last-viewed revisit) follows this single order.
- Explicit forward-compatibility note (approved Q&A decision): a future settings feature will let the user change the sort order; v1 ships with alphanumeric fixed and no setting UI.

### 3.7 Section: `## Distribution and Default App Registration`

Content points:

- Distribution model: zip-style portable installer.
- Installer flow: the user picks a destination folder; the package unzips the application binary AND its config file into that destination.
- Default image viewer registration: the package includes a helper and/or guidance that helps the user register the app as the default image viewer (file associations) in Windows.
- Implementation details of the helper (script vs. executable, exact mechanism) are deferred to implementation and are not decided in this brief.

### 3.8 Section: `## Technology Stack`

Content points — mark the section content as "planned decisions (greenfield — no code yet)" with one explicit sentence at the top of the section stating that these choices are confirmed decisions but not yet implemented:

- Language: Rust.
- GUI: egui/eframe (pure Rust, lightweight; suitable for zoom/pan/fullscreen/keyboard shortcuts).
- Image decoding: Windows Imaging Component (WIC) system codecs — the app accepts any image format registered in the system instead of bundling its own decoders.
- Target platform: Windows 10 and Windows 11.
- Target architectures: x64 and x86 builds.

### 3.9 Section: `## Out of Scope for v1 (Deferred)`

Content points — a bulleted list; framing sentence: features below are explicitly NOT in v1 and may be added later:

- Rotation.
- Slideshow.
- Image editing.
- Printing.
- Any further functionality inherited from the old gallery app beyond the 7 v1 features.
- Settings UI for changing the sort order (v1 is fixed alphanumeric — see `## Sorting Order`).
- Persistence of the per-folder last-viewed memory across sessions (v1 memory is session-only and destroyed on close — see the vertical scroll feature rules).

---

## 4. Assembly Instructions (how to write the file)

1. Read the current `.agent/project-info/brief.md` and copy the protected block (from line `<!-- DO NOT DELETE NEXT SECTION -->` through the final `<!-- END DO NOT DELETE -->`) exactly as-is; it will be pasted unchanged at the end of the new document.
2. Write the new document top-down per Section 3 structure, using the content points of 3.1–3.9 as the required substance for each section. Bullet lists are the primary formatting device; short declarative paragraphs are allowed for the Overview and Problem Statement.
3. Paste the preserved block at the very end. Nothing after it.
4. Verify with the checklist in Section 5 before reporting completion.

Anti-patterns to avoid (explicit):

- Do NOT copy-paste any template boilerplate text from the current file other than the protected block.
- Do NOT invent unapproved features (thumbnail bar, drag & drop between apps, dark/light theme selection, multi-monitor support, EXIF panel, etc.) — anything not listed in the global plan pre-analysis or this plan is out of scope.
- Do NOT invent keyboard bindings (e.g., do not write "F11 for fullscreen") — if a binding must be mentioned, use the deferred phrasing from 3.5 Fullscreen.
- Do NOT mention panels/screenshots/components of the UI that are not in the plan.
- Do NOT write implementation instructions or code. No code fences. No tables.
- Do NOT use literal `\n` in content; real line breaks only.
- Do NOT rename the section headings from Section 3's list (the heading texts are the decided structure, not suggestions).

---

## 5. Verification Checklist (the implementer runs this; list every check)

After writing the file, verify ALL of the following. Each check must pass; failure of any check requires fixing the file before completion.

### 5.1 Structure checks

- [ ] File contains exactly the 9 content sections from 3.3.1–3.9 order: Overview, Problem Statement, Purpose and Goals, v1 Feature Scope, Sorting Order, Distribution and Default App Registration, Technology Stack, Out of Scope for v1 (Deferred), plus the `# Project Brief` title and the preserved trailing block. No extra, missing, or renamed top-level sections.
- [ ] The old template sections (`Core Requirements`, `Goals`, `Project Scope`) are absent.
- [ ] The only HTML comments in the file are the two preserve markers (`<!-- DO NOT DELETE NEXT SECTION -->` and `<!-- END DO NOT DELETE -->`).
- [ ] The preserved block sits at the very end of the file (EOF right after `<!-- END DO NOT DELETE -->`).
- [ ] File uses real newlines (open the file and confirm multi-line rendering; no literal `\n` text anywhere).

### 5.2 Preservation checks

- [ ] Inside the preserved block: heading `## Important Note for AI Agents` present with the exact same text.
- [ ] The link target `../../AGENTS.md` is present and unchanged.
- [ ] The two numbered steps and their exact wording are unchanged.
- [ ] Both marker comment lines are present and unmodified.

### 5.3 Content coverage checks — all 7 v1 features (each must be present with the required semantics)

- [ ] Feature 1 — Open image: system-registered formats (WIC-dependent), unsupported-formats constraint, classic-view window behavior.
- [ ] Feature 2 — Horizontal scroll: left/right arrow keys AND UI buttons; sibling images in the same folder only; order per `## Sorting Order`.
- [ ] Feature 3 — Vertical scroll: up/down arrow keys AND UI buttons; sibling folders under the same parent folder only, with the `parent\folder2\img1` → `parent\folder1`/`parent\folder3` example OR an equally explicit same-parent-only statement.
- [ ] Feature 3a — Same-filename rule: same filename preferred; fallback to the target folder's FIRST image per sort order when missing.
- [ ] Feature 3b — Per-session last-viewed rule: last viewed image per folder is remembered and wins on revisit within the same session; memory destroyed on app close; explicitly no persistence.
- [ ] Feature 4 — Fullscreen: on/off toggle, via shortcut and UI control, with the "bindings decided during implementation" phrasing.
- [ ] Feature 5 — Zoom: in/out plus BOTH "fit width" and "fit height" modes named explicitly.
- [ ] Feature 6 — Overflow-zoom pan-then-navigate: width overflow → left/right pan first, navigate only at edges; height overflow → up/down pan first, folder navigation only at edges; direct navigation when image fits.
- [ ] Feature 7 — "Open with..." menu: system-registered programs for the current file type; app has no own program database.

### 5.4 Content coverage checks — remaining Q&A decisions

- [ ] Provisional project name `img-scroller` stated as provisional.
- [ ] Greenfield status (no code yet) stated.
- [ ] Sort order: alphanumeric for folders AND files, fixed in v1, future settings option noted.
- [ ] Distribution: zip-style installer with destination-folder choice, unzips app + config.
- [ ] Default app registration: helper/guidance for registering as default image viewer.
- [ ] Tech stack: Rust + egui/eframe + WIC + Windows 10/11 + x64/x86, all present, marked planned/unimplemented.
- [ ] Deferred list includes rotation, slideshow, editing, printing, other old-gallery extras, sort-order settings UI, cross-session memory persistence.

### 5.5 Boilerplate-removal checks

- [ ] No occurrence of: "base project", "scaffolding tool", "AI-Readiness", "Kilo Code", "kilocodeignore", "Reduce setup time", "base template" anywhere in the NEW written content.
- [ ] No mention of `docs/`, `.opencode/`, or unrelated tooling in the brief content.

### 5.6 Confinement checks

- [ ] `git status` shows ONLY `.agent/project-info/brief.md` modified (no other file touched).
- [ ] No TODO file edits; no new files created; no commit made in this step.
- [ ] No build/test commands invoked (none exist for Markdown).

---

## 6. Out-of-Plan Notes (for the caller / reviewer, not the implementer)

- Steps 4.3 (code-reviewer vs. user requirements), 4.5b (architector verification vs. Q&A), and 4.6 (mark TODO `[DONE]`, commit) execute after 4.2 and are governed by the global plan `.kilo/plans/20261029-project-brief.md`.
- The rewritten brief intentionally does not duplicate content that will live in Task 2 files (`product.md`, `context.md`, `architecture.md`, `tech.md`); the brief stays the scope/requirements source of truth.
