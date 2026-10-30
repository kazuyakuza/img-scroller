# Global Plan — Planning Wave: Phased TODO Files, App Name, Changelog Structure, License

- Date: 2026-10-29
- Source TODO: [`.agent/todos/20261029/20261029-todo-2.md`](../.agent/todos/20261029/20261029-todo-2.md)
- Requirements source of truth: [`.agent/project-info/brief.md`](../.app/../.agent/project-info/brief.md) (final app name reported by the user: **Images Scroller**).
- Session scope: PLANNING ONLY. No app implementation, no `Cargo.toml`, no code, no dependency installs. The 7 wave TODO files produced here are processed one-per-session in future sessions, each via the Critical Workflow.

## Global Pre-Analysis

- Repository state: `main`, clean, tracks `origin/main`; greenfield — docs/config scaffolding only, no source code, no `Cargo.toml`/`package.json` (no version manifest → workflow Step 3 is N/A).
- Live docs referencing the provisional name `img-scroller` (must be updated to **Images Scroller**): `README.md` (lines 1–6), `.agent/project-info/brief.md` (line 8), `.agent/project-info/product.md` (line 5), `.agent/project-info/context.md` (line 7 example phrase). Historical `.kilo/plans/*.md` are NOT modified.
- Repository directory name `img-scroller` stays unchanged; only the app/product name in documentation changes.
- Changelog: **Keep a Changelog** convention (`CHANGELOG.md`; `### Added / Changed / Deprecated / Removed / Fixed / Security` under `## [x.y.z]` headings) + SemVer. No entries until MVP. File creation timing = pending user decision (asked).
- License target: permissive license satisfying — 100% free use; forks/modified versions allowed; sharing outside GitHub Releases must reference the GitHub project. Practically this is a notice-retention permissive license (MIT / BSD-2-Clause / Apache-2.0); the copyright/attribution notice will embed the GitHub project URL so every redistribution carries the reference. Specific pick = pending user decision (asked).
- All greenfield technical decisions (WIC binding, module layout, keybindings, zoom steps, helper mechanism, config format) remain deferred to each future wave's analysis step — this plan decides none of them.
- Front-end related flags (for 4.1a/4.5a): Task 1 = NO; Tasks 2, 3, 4 = NO. No 4.1a/4.5a sub-steps in this wave.
- Grouping note (Critical Workflow allows grouping extremely short/related tasks): Task 1 gets its own cycle; Tasks 2, 3 and 4 are grouped into ONE cycle (they are tiny, related doc edits governed by the same rules) while the TODO file will still mark each `[DONE]` individually.

## Steps Mapping (Steps 2–6)

### Step 2 — Git Feature Branch Setup => implementer

- Commit the new TODO file (`.agent/todos/20261029/20261029-todo-2.md`) with message `docs: add planning-wave todo (phases, naming, changelog, license)`.
- On `main` already → create feature branch `feat/planning-waves`; all remaining work happens there; merge back at Step 5.

### Step 3 — Version Update => N/A

- No version manifest exists (`Cargo.toml` created only in a future implementation wave). Recorded as skipped; log `N/A — no version manifest (no Cargo.toml/package.json)`.

### Task 1 — Phased TODO files (waves) — cycle 4.1–4.6 (front-end: NO)

- 4.1b architector — produce the wave-files specification plan: `.kilo/plans/20261029-task1-wave-files-spec.md`
  - Exact spec for 7 wave TODO files, to be created as `.agent/todos/20261029/20261029-todo-3.md` … `20261029-todo-9.md`:
    - Wave 1 (todo-3): Project skeleton — Cargo project, egui/eframe app window, in-app open dialog, WIC decode, image display (regular resizable window). Deliverable: app opens and shows an image; build/run documented.
    - Wave 2 (todo-4): Horizontal scroll — current-folder image listing, alphanumeric sort, prev/next image via arrow keys + left/right UI buttons. Pan/zoom interplay deferred to Wave 5.
    - Wave 3 (todo-5): Vertical scroll — sibling folders under SAME parent, same-filename rule, first-image fallback, session last-viewed memory (RAM-only). MVP checkpoint placed here or elsewhere per the user's MVP decision.
    - Wave 4 (todo-6): Fullscreen mode + Zoom in/out + fit width/fit height (related UI toggles grouped; keybindings/zoom steps decided in that wave's analysis).
    - Wave 5 (todo-7): Pan-then-navigate at image edges while zoomed (edge-triggered horizontal/vertical navigation).
    - Wave 6 (todo-8): Fast "Open with..." menu — enumerate system-registered programs for current file type/extension at runtime.
    - Wave 7 (todo-9): Distribution and packaging — zip-style portable installer, config file, default-viewer registration helper/guidance (mechanism decided in that wave's analysis).
  - Every wave file: requirements mapped to brief sections, expected deliverables checklist, verification expectations (build/test/UX), explicit "decision deferred to this wave's analysis" notes, and a note that the implementing session must follow the Critical Workflow on a fresh feature branch.
  - MVP checkpoint labeled per the user's MVP decision (in the wave definition).
- Planner (this agent) creates the 7 wave TODO files per the spec — markdown rule: TODO files are created/modified by the Planner only.
- 4.3 code-reviewer — review TODO files vs spec → fix plan file `.kilo/plans/20261029-task1-wave-files-fixes.md` if needed; code-simplifier: N/A (no source code in this task; recorded).
- 4.4 docs-specialist — N/A (the task output is agent TODO artifacts, not project documentation; recorded).
- 4.5b architector — plan adherence report `.kilo/plans/20261029-task1-wave-files-adherence.md`.
- 4.6 implementer — mark Task 1 `[DONE]` in the TODO file + commit `docs: planning wave task 1 done (7 wave todo files defined)`.

### Tasks 2, 3, 4 — Naming, Changelog structure, License — grouped cycle 4.1–4.6 (front-end: NO)

- 4.1b architector — one combined doc-edit plan: `.kilo/plans/20261029-tasks2-4-name-changelog-license.md`
  - Task 2 exact edits: rename references in `README.md`, `.agent/project-info/brief.md`, `.agent/project-info/product.md`, `.agent/project-info/context.md` → app name **Images Scroller**; keep repo-name note; do not touch `.kilo/plans/*`.
  - Task 3: CHANGELOG.md per user decision — (a) create now with Keep-a-Changelog empty structure + "entries begin at MVP" banner note (recommended), or (b) record the structure spec in template form within the plan only and create the file in the MVP wave.
  - Task 4: `LICENSE` at repo root with the user-chosen license, copyright/attribution line embedding the GitHub project URL (user-provided); `README.md` License section rewritten (license summary + plain-language attribution requirement); license recorded as a decision in `.agent/project-info/brief.md`.
- 4.2 docs-specialist — executes all doc edits per plan (docs may be created/modified by Docs Specialist). Commits per logical group: `docs: rename app to Images Scroller in live documentation (tasks 2)`, `docs: add changelog structure template (task 3)`, `docs: add license and attribution note (task 4)` (exact counts may merge when files overlap, e.g. README).
- 4.3 code-reviewer — adherence review of the doc edits → fix plan `.kilo/plans/20261029-tasks2-4-docs-fixes.md` if needed; code-simplifier: N/A (no source code; recorded).
- 4.4 docs-specialist — covered by 4.2/4.3 for this documentation-only cycle (recorded as merged).
- 4.5b architector — adherence report `.kilo/plans/20261029-tasks2-4-docs-adherence.md`.
- 4.6 implementer — mark Tasks 2, 3, 4 `[DONE]` individually in the TODO file + commit(s).

### Step 5 — TODO File Completion => implementer

- Rename `.agent/todos/20261029/20261029-todo-2.md` → `20261029-todo-2-DONE.md` (content unchanged).
- Update `.agent/project-info/context.md` (critical closing step) with the planning-wave outcome.
- Ensure all work committed on `feat/planning-waves`; delete tmp files if any.
- Merge to `main`; on success delete the feature branch; push `main` to `origin` ONLY per the user's push decision (origin confirmed configured).

### Step 6 — Finish (Planner)

- Short summary + next-session handoff text pointing to the first wave file (`20261029-todo-3.md`), or to the decided MVP wave.

## Pending User Decisions (asked via `question` tool before execution)

1. Global plan approval (Critical Workflow: "Approve Global and Tasks Plans" / "Approve Global Plan").
2. MVP checkpoint wave (default recommendation: end of Wave 3 — core viewer with both axes usable; alternatives: end of Wave 1; end of full v1 = Wave 7).
3. License choice (default recommendation: MIT with GitHub URL in the notice; alternatives: BSD-2-Clause, Apache-2.0, custom text).
4. GitHub project URL to embed in the license/attribution notice.
5. CHANGELOG.md creation timing (recommended: create file now as empty structure; alternative: structure spec only until MVP).
6. Push `main` to `origin` after Step 5 merge (recommended: push; alternative: local only).

---

# Addendum v2 — User Decisions (2026-10-29, chat Q&A)

The user approved the global plan WITH corrections ("approve global and tasks plans" after these changes are applied), and resolved all pending decisions:

1. **Plan approval**: approved — per-task plans (4.1) are auto-approved accordingly.
2. **MVP checkpoint = end of Wave 7 (full v1)**. Consequence: ONLY Wave 7 may write the first changelog entry (v1.0.0 material) at its completion; Waves 1–6 must NOT touch `CHANGELOG.md`.
3. **Wave 3 correction (requirements change)**: Vertical navigation rule — the same-filename rule is REMOVED from v1. Target image priority when moving to a sibling folder: (a) if the folder was already visited this session → its session last-viewed image (RAM-only); (b) otherwise → ALWAYS the target folder's FIRST image (alphanumeric order). No other fallback. This CONTRADICTS the current brief (`### Vertical Scroll (Sibling Folders)`); the brief, `product.md`, and `README.md` must be corrected (handled by the new Task 5 in the TODO file, executed inside the Tasks 2–4 grouped doc cycle → now "Tasks 2–5").
4. **Fullscreen correction (requirements change, Wave 4 / brief `### Fullscreen Mode`)**: default fullscreen state OFF at launch; the window opens normal or maximized at the user's own placement/size; fullscreen OFF → UI controls visible (UI layout details DEFERRED to a later definition by the user) and the image must take almost all of the central space; fullscreen ON → hide the window header and ALL UI controls, showing only the image. Also propagated by Task 5.
5. **License = MIT (chosen)**. Copyright/attribution notice embeds the GitHub project URL auto-detected via `git remote get-url origin` at execution time. `README.md` License section rewritten (summary + plain-language attribution requirement for sharing outside GitHub Releases); choice recorded in the brief.
6. **CHANGELOG.md created NOW** as an empty Keep-a-Changelog structure + SemVer + explicit banner: "No entries yet — changelog updates begin after the MVP version (full v1) exists." No entries in this session.
7. **Step 5 push**: push `main` to `origin` ONLY after the merge.

Grouped doc cycle scope updated: the cycle covers Tasks 2, 3, 4 AND the new Task 5 (name updates + changelog structure + license + requirement corrections). The architector spec for Task 1 (wave files) must encode corrections 2, 3, 4 inside wave TODO files (todo-5 vertical rules, todo-6 fullscreen behavior, todo-9 MVP checkpoint + changelog activation).
