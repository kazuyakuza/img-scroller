# Wave 7 Task 6 — MVP Checkpoint: First `CHANGELOG.md` Entry — Implementation Plan (4.1b)

- Source TODO: `.agent/todos/20261029/20261029-todo-9.md` → `### Task 6: MVP checkpoint — first CHANGELOG.md entry (FINAL task of the wave)`.
- Global plan: `.kilo/plans/20261114-wave7-distribution-mvp.md` → `## Task 6` (this file is the NEW plan the global plan's Task-6 pre-analysis points at; it is created in the 4.1b step and committed by the implementer at Step 0).
- Date of this plan: 2026-11-15. Status: PENDING PLANNER PRESENTATION (the implementer executes plan §4 only after the Planner approves it).
- Front-end related: NO (no 4.1a frontend spec, no 4.5a frontend verification — Wave-level decision #3 of the global plan).

## 1. Verified Environment Facts (architector verification, 2026-11-15)

- Branch: `feat/wave-7-distribution`; HEAD `1c03cb2` (`task: mark wave7 task5 done`); working tree CLEAN (`git status` — nothing to commit). Verified with single git commands on 2026-11-15.
- TODO gate: Tasks 1–5 of `20261029-todo-9.md` are ALL `[DONE]` (headings `### Task 1: Config file (existence + format decision) [DONE]` … `### Task 5: Documentation updates [DONE]`, lines 11–34; the Task-4 manual-UX OPERATOR-RUN sub-item stays `[ ]` — per the recorded execution rule, the gate is the task HEADINGS, not the user-gated sub-item). The implementer RE-VERIFIES this at Step 1 (§4).
- `CHANGELOG.md` (22 lines, read 2026-11-15): `# Changelog` + the 2-sentence intro + `The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).` + `This project uses Semantic Versioning (SemVer).` + the banner line `> No entries yet — changelog updates begin only after the MVP version (full v1) exists.` + `## [Unreleased]` + six EMPTY subsections (`### Added`, `### Changed`, `### Deprecated`, `### Removed`, `### Fixed`, `### Security`). NO link-reference footer exists (the only link is inline in the intro line).
- Line endings: `.gitattributes` pins NO rule for `*.md` (only `*.sh`/`packaging/*.toml`/`*.txt` LF and `packaging/*.cmd` CRLF). The working-tree endings of `CHANGELOG.md` must be OBSERVED by the implementer before editing (Step 2 pre-check in §4) and preserved byte-for-byte in style.
- Version facts: `Cargo.toml` package version is 1.0.0 (bumped at Step 3 of the wave, commit `e7089d0`); the entry headline is `[1.0.0]`; NO version edit is part of this task.
- Alpine VM/container: NOT needed for this task (markdown-only; no build, no tests, no packaging runs).

## 2. Decisions (ALL pinned — no open choices for the implementer)

### D1 — Entry placement and structure

- `## [Unreleased]` STAYS on top, BYTE-UNCHANGED: the heading and all six empty subsection headings (`### Added` through `### Security`) keep their exact text, order, and blank-line spacing as they exist today.
- The new section goes IMMEDIATELY AFTER the last empty Unreleased subsection (`### Security`), separated by one blank line before it (matching the file's existing blank-line style):
  `## [1.0.0] - <date>` (date per D3).
- Under `## [1.0.0]` there is EXACTLY ONE subsection: `### Added`, followed by the bullet list (the draft checklist in §3). NO `### Fixed`, NO `### Changed`, NO other subsections under `[1.0.0]`.
  - Justification (pinned): this is the INITIAL release summarizing the whole product, not a delta from a previous release — there is no prior released version against which anything was "fixed" or "changed". The operator-feedback correction behaviors (Waves 4.1/4.2/5.1/5.2/6.1/6.2) are v1.0.0 content and FOLD INTO the feature bullets of `### Added` (per-bullet mapping in §3); they do NOT surface as standalone Fixed entries. DECIDED: `### Added` only.
- NO link-reference footer at the bottom of the file. The existing file uses a plain inline-link style (one inline link in the intro); the entry keeps that style. DECIDED: no footer, plain inline style only.

### D2 — The banner

- The line `> No entries yet — changelog updates begin only after the MVP version (full v1) exists.` is REMOVED — plain removal, NO replacement text. The file's intro (lines 3–6) already explains the format and scope; the entry itself supersedes the banner. Authorized by the global plan Task-6 section ("the banner is removed/superseded by this entry; that edit is part of THIS task's authorization"). Nothing else in the intro is touched.

### D3 — Version and date

- Headline: `## [1.0.0] - 2026-11-15` — version `1.0.0` (matches the Step-3 bump; NEVER edited by this task), date `2026-11-15` (the milestone execution date — today per the environment).
  - Fallback rule (single, deterministic): if the implementer executes the Step-2 edit on a LATER calendar date than 2026-11-15, the date in the headline is THAT execution date (the entry date is the date the entry lands). Never a future date, never any other date source.

### D4 — Wording discipline (the wave's "written at execution" requirement)

- The bullet sentences are DRAFTED BY THE IMPLEMENTER at execution. This plan does NOT pre-draft the prose. The plan pins instead: the structure (D1), the bullet FACTS each bullet must reflect with their per-bullet source documents (§3), the tone rules (D5), and the fact-discipline rule (D6). The implementer writes the actual sentences from the checklist.
- HARD RULE: the wave file and the spec do NOT contain pre-drafted entry text; do not copy sentence fragments from the plan — derive each sentence from the pinned facts and sources.

### D5 — Tone and style rules (pinned)

- Concise, user-facing release-note style, consistent with Keep a Changelog 1.1.0.
- ONE sentence per bullet (a sentence may contain semicolons/parentheticals, but it must be ONE sentence — no multi-sentence bullets).
- Each bullet starts with what the app does / what the package ships (noun-led or verb-led "Added"-section phrasing, e.g. "Open images in any format registered in the Windows system…", "Ships as a portable zip package…").
- Plain text only: NO markdown bold/italics in bullets, NO inline links in bullets, NO nested sub-bullets. Bullets are flat `- ` list items.
- FORBIDDEN content in the entry text: module/file names from `src/` (e.g. `keyboard_intake.rs`), commit hashes, internal wave numbers ("Wave 4.1"), test censuses, artifact byte sizes, dependency version pins, or any internal development vocabulary. The entry is for USERS of the app.
- The bullet ORDER is exactly the §3 checklist order (features 1–7 in the brief's order, then the close-contract bullet, then the distribution bullets).

### D6 — Fact-discipline rule (pinned)

- EVERY factual claim in every bullet must be traceable to the listed sources in §3 (the landed project-info docs + README). If a fact cannot be sourced from them, it MUST NOT appear in the entry. Never invent, never extrapolate beyond the pinned facts, never add qualifiers the sources do not support.

## 3. DRAFT CHECKLIST — the bullets to write (the implementer writes the sentences)

Exactly TWELVE bullets under `### Added`, in this order. Per bullet: the FACTS it must reflect and the SOURCE documents (read them before drafting; keep sentences within D5's constraints).

### Feature bullets (1–7)

1. **Open image with system-registered formats** — Facts: the app opens any image whose format is registered as a codec in the Windows system (via the Windows Imaging Component), e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed; formats without a system codec are not supported and show the app's unsupported-file message; the image opens from Explorer, a file manager, or the in-app open dialog; the window opens ALWAYS MAXIMIZED at every launch and the release build shows no console window (fold the launch-state correction here). Sources: `brief.md` `## v1 Feature Scope` → `### Open Image with System-Registered Formats` + Addendum item 1; `product.md` `## v1 User-Facing Capabilities` item 1; `README.md` `## v1 Features` items 1 and 4 (launch clause); `tech.md` `## Image Format Support` + the launch-state bullet of `## Fullscreen, Zoom and Fit Modes`.
2. **Horizontal scroll (sibling images)** — Facts: left/right arrow keys and the pill's left/right buttons navigate to the previous/next sibling image in the SAME folder (strictly current-folder scope, not recursive); navigation order follows the SYSTEM's name order — the same order a Windows file manager shows, including non-ASCII/Japanese names, digit-aware natural ordering, case-insensitive (fold the name-order correction here); at the first/last image the viewer stays (no wrap, silent edges). Sources: `brief.md` `### Horizontal Scroll (Sibling Images)` + Addendum item 8; `README.md` `## v1 Features` item 2; `tech.md` `## Horizontal Navigation Mechanism` + `## Entry-Name System Order`.
3. **Vertical scroll (sibling folders)** — Facts: up/down arrow keys and the pill's up/down buttons navigate to the previous/next sibling folder under the SAME parent (no grandparent-level jumps, no recursion, no wrap); the target is the folder's SESSION last-viewed image if visited this session, otherwise ALWAYS the folder's first image in the system name order (no other fallback); the last-viewed memory is RAM-only and destroyed on close (no persistence in v1); an image-less sibling folder is entered in an empty-folder browsing state ("Empty folder" message, not skipped). Sources: `brief.md` `### Vertical Scroll (Sibling Folders)`; `product.md` item 3; `README.md` `## v1 Features` item 3; `tech.md` `## Vertical Navigation and Session Last-Viewed Memory`; `architecture.md` Logical Components (Navigation Engine / Session Last-Viewed Memory).
4. **Fullscreen mode** — Facts: toggled by the F11 keyboard shortcut and the on-screen fullscreen button; when ON the window header/borders and ALL UI controls disappear and only the image shows; the arrow keys keep navigating while ON; F11 — or ESC — exits fullscreen. Sources: `brief.md` `### Fullscreen Mode` + Addendum item 5; `README.md` `## v1 Features` item 4; `tech.md` `## Fullscreen, Zoom and Fit Modes` (fullscreen bullet); `architecture.md` Logical Components (Full-Screen View).
5. **Zoom in/out and fit** — Facts: zoom in/out via the keyboard (`I` / `O`, plus strict Shift aliases) and the pill's [+]/[−] buttons; every step multiplies/divides by 1.25 within SYMMETRIC bounds — up to ×8 of the whole-image fit and down to ÷8 of it (fold the symmetric-correction fact; zoom-out is not blocked at the fit level); steps at a bound clamp silently — zoom actions never show a message; the `0` key and the restore-fit button return to whole-image fit; "fit width" (`W`) and "fit height" (`H`) are ONE-SHOT size sets (the image's width/height fills the window once; zoom continues from that level — not a persistent mode). Sources: `brief.md` `### Zoom In/Out and Fit Modes` + Addendum items 2–4; `README.md` `## v1 Features` item 5; `tech.md` `## Fullscreen, Zoom and Fit Modes`; `architecture.md` Logical Components (Manual Zoom + Fit).
6. **Pan-then-navigate while zoomed** — Facts: while the zoomed image is WIDER than the window, left/right arrow keys pan it (about 1/8 of the window per press) and sibling-image navigation triggers only when panning reaches the image's left/right edge; while TALLER, up/down pan and folder navigation triggers at the top/bottom edge; at the edge a distinct press navigates (holding never navigates); when the image fits entirely the arrows navigate directly. Sources: `brief.md` `### Pan and Navigate at Image Edges While Zoomed`; `README.md` `## v1 Features` item 6; `tech.md` `## Wave-5 Pan-then-Navigate Mechanism`; `architecture.md` Logical Components (Pan-then-Navigate).
7. **Fast "Open with..." menu** — Facts: an on-screen pill button — also the `Q` keyboard shortcut — opens a menu listing the programs the Windows system has registered for the currently viewed file's type, built from system registration data at runtime (no internal program database); selecting a program opens the current file with it; while the menu is open all other app functionalities are off and ESC or Q again closes it; a failed launch shows the app's single open-with failure message. Sources: `brief.md` `### Fast "Open with..." Menu`; `README.md` `## v1 Features` item 7; `tech.md` `## Open-with Enumeration and Launcher Mechanism`; `architecture.md` Logical Components (Fast "Open with..." Service).

### Close-contract bullet (8)

8. **ESC double-press close** — Facts: while fullscreen is OFF, the FIRST ESC press shows a centered "Press ESC again to close" popup for about 2 seconds (it dismisses itself after that); the application closes only on a SECOND distinct press while the popup is visible (holding ESC never counts; an expired popup re-arms); while fullscreen is ON, ESC exits fullscreen only. Sources: `brief.md` Addendum item 7; `README.md` `## v1 Features` item 4 close clause + the controls table's close row; `architecture.md` Logical Components (ESC-Close Confirm Popup).

### Distribution bullets (9–12)

9. **Portable zip distribution** — Facts: distributed as a portable zip package (no installer); ONE zip per architecture — 64-bit (`ImagesScroller-1.0.0-windows-x64.zip`) and 32-bit (`ImagesScroller-1.0.0-windows-x86.zip`); the user picks any destination folder via the standard Windows extract-all flow — that folder IS the install folder. Sources: `brief.md` `## Distribution and Default App Registration`; `README.md` `## Distribution and Installation` (+ the install steps); `tech.md` `## Distribution and Packaging (Implemented, Wave 7)`; `architecture.md` `## Build and Release Pipeline`.
10. **Package contents** — Facts: each zip contains exactly four files at its root: the application executable `img-scroller.exe`, the application config file `img-scroller.toml`, the default-viewer registration helper `register-default-viewer.cmd`, and the user guidance `DEFAULT-VIEWER-GUIDE.txt`. Sources: `README.md` `## Distribution and Installation` (the 4-file paragraph); `architecture.md` `## Build and Release Pipeline` (the packaging bullet); `tech.md` `## Distribution and Packaging (Implemented, Wave 7)`.
11. **Config file** — Facts: the application config file `img-scroller.toml` ships next to the executable; the current version does not read it (no settings exist in v1 — nothing to configure). Sources: `architecture.md` `## Data and State` (the On-disk bullet); `README.md` install step 3 note; `tech.md` `## Distribution and Packaging (Implemented, Wave 7)` (the config record).
12. **Default-viewer registration helper and guidance** — Facts: the package includes the `register-default-viewer.cmd` helper and the `DEFAULT-VIEWER-GUIDE.txt` guidance; running the helper registers Images Scroller for the user's Windows account only (no administrator rights), adds it to the "Open with" lists, makes it visible in the Windows Settings "Default apps" page, and opens that page — the final per-type default choice is made by the user in Settings; it covers the 17 image file types the app supports; re-running it is safe and `register-default-viewer.cmd --unregister` removes exactly the created registration entries. Sources: `brief.md` `## Distribution and Default App Registration`; `README.md` `## Distribution and Installation` → the "Make Images Scroller the default image viewer" section; `tech.md` `## Distribution and Packaging (Implemented, Wave 7)` (the helper record); `architecture.md` Logical Components (Distribution Packaging).

## 4. Implementation Steps for the Implementer (step 4.2)

Execute these steps IN ORDER on branch `feat/wave-7-distribution`. NO push (push is restricted to the wave's Step 5). NO version edits. NO container/VM runs. The ONLY content file this task touches is `CHANGELOG.md` (plus the plan file itself at Step 0). Do NOT edit any existing `.kilo/plans/*` file.

### Step 0 — Commit the plan file

1. Read `.gitignore` and run `git status` (Gitignore Compliance Rule) — confirm nothing gitignore-matching is present and the tree is otherwise clean.
2. Stage ONLY the plan file: `git add .kilo/plans/20261114-wave7-task6-changelog-mvp-plan.md`.
3. Commit with the EXACT message: `docs: add wave7 task6 changelog plan`.

### Step 1 — Gate check (STOP conditions)

1. Read `.agent/todos/20261029/20261029-todo-9.md` and verify the headings of Tasks 1–5 ALL carry `[DONE]`. (The Task-4 manual-UX OPERATOR-RUN sub-item `[ ]` is NOT the gate.)
2. Verify: `git branch --show-current` → `feat/wave-7-distribution`; `git status` → clean.
3. If ANY gate fails → STOP and return a question to the caller. Never proceed on a failed gate.

### Step 2 — Edit `CHANGELOG.md`

1. PRE-CHECK (line endings, BEFORE editing): observe the file's current line endings with a concrete command (e.g. `rg -c "\r$" CHANGELOG.md` — a count equal to the line count ⇒ all-CRLF; count 0 ⇒ all-LF; a middle value ⇒ mixed). Record the observed result; the edited file MUST preserve the observed endings style exactly (the edit tool preserves existing endings; verify again after the edit — if the endings change, redo the edit).
2. Make EXACTLY TWO changes:
   a. REMOVE the banner line `> No entries yet — changelog updates begin only after the MVP version (full v1) exists.` and its surrounding blank line so the intro paragraph flows directly into `## [Unreleased]` with the file's normal blank-line spacing (plain removal — D2).
   b. AFTER the `### Security` line (the last Unreleased subsection), add — separated by one blank line — the new section:
      - `## [1.0.0] - <date per D3>`
      - one blank line, then `### Added`
      - one blank line, then the TWELVE bullets of the §3 checklist, in order, each a single flat `- ` sentence per D5/D6, drafted by you at execution from the pinned facts and sources.
3. TOUCH NOTHING ELSE: `# Changelog`, the intro lines, the `## [Unreleased]` heading, and all six empty Unreleased subsection headings stay byte-identical (D1). Do NOT add any other subsection under `[1.0.0]`, do NOT add a link-reference footer (D1), do NOT reflow/reword the intro.
4. SELF-VERIFY after editing (read the whole file): intro unchanged; banner gone; `## [Unreleased]` + six empty subsections unchanged; `## [1.0.0] - <date>` directly after the Unreleased block; exactly one `### Added` under it with the 12 bullets; no other changes; line endings preserved; file ends with a single trailing newline as before.
5. Scope check: `git status` must show ONLY `CHANGELOG.md` modified (nothing staged yet). Anything else → STOP and return a question.

### Step 3 — Commit

1. Read `.gitignore`, run `git status` (Gitignore Compliance Rule) — confirm only `CHANGELOG.md` is modified and nothing gitignore-matching is present.
2. Stage ONLY `CHANGELOG.md`: `git add CHANGELOG.md`.
3. Commit with the EXACT verbatim message: `docs: add changelog first entry v1.0.0`.
4. Confirm the tree is clean and NO push is performed.

## 5. Guidance for the Remaining Cycle Steps (NOT executed by the 4.2 implementer)

- **4.3 code-reviewer**: verify against the diff — (1) only `CHANGELOG.md` (+ the Step-0 plan-file commit) changed; (2) the banner line is gone; (3) `## [Unreleased]` and its six empty subsections are byte-unchanged; (4) `## [1.0.0] - <date>` sits directly after the Unreleased block with the correct version and date (D3); (5) exactly one `### Added` with 12 bullets in §3 order, each ONE sentence (D5); (6) FACT-DISCIPLINE: every factual claim traceable to a §3 source; no module names/hashes/wave numbers/censuses/byte sizes/dependency pins (D5 forbidden list); (7) no `### Fixed`/other subsections under `[1.0.0]`, no link-reference footer (D1); (8) line endings preserved; (9) both commit messages match §4 verbatim. Max 3 review cycles, then escalate.
- **Code simplifier**: NOT required — prose-only change; nothing to simplify in code. Skip.
- **4.4 docs-specialist / 4.5b architector verify-only sweep**: no doc updates belong to this task (Task 5 landed all docs; the scope guard is `CHANGELOG.md` only). A verify-only check may confirm no other doc references the removed banner sentence — if any does, STOP and return a question (do NOT edit it in this task).
- **Adherence (diff-based)**: the executed diff must match plan §4 exactly — two changes to `CHANGELOG.md`, two verbatim commit messages, no other files.
- **4.6 task mark** (later step, by its own sub-agent): mark Task 6's two sub-items `[x]` in `.agent/todos/20261029/20261029-todo-9.md`, preserving all other file content (overwrite-prevention rule), with its own commit.

## 6. Binding Constraints (restated)

- NO `git push` (push happens ONLY at the wave's Step 5, `main` → `origin` ONLY).
- NO version edits (`Cargo.toml`/`Cargo.lock` untouched; the 1.0.0 headline already matches the Step-3 bump).
- NO edits to existing `.kilo/plans/*` files (this plan file is NEW; its creation is authorized).
- `CHANGELOG.md` is the ONLY content file this task touches.
- NO container/VM/alpine-vm runs (markdown-only task).
- Any ambiguity → STOP and return the question to the caller; never guess.

## 7. Verification of This Plan Against the Source Task

Checked against `.agent/todos/20261029/20261029-todo-9.md` Task 6 verbatim sub-items: (1) the MVP gate is encoded as Step 1 (gate check, STOP on failure) with the recorded "task headings, not the user-gated sub-item" reading; (2) the entry is written in the existing Keep-a-Changelog structure at execution (D4 — no pre-drafted prose; the plan pins facts/sources/structure only), summarizing v1.0.0 content (the 7 features + distribution — §3's 12 bullets cover all seven brief features, the ESC-close correction, and the four distribution facts), with placement/banner/section decisions per the global plan's Task-6 section. Plan file path (NEW, per the important-paths template): `.kilo/plans/20261114-wave7-task6-changelog-mvp-plan.md`.
