# Grouped Implementation Plan — Tasks 2, 3, 4, 5: App Name, CHANGELOG Structure, MIT License, Requirement Corrections

- Date: 2026-10-29
- Source TODO: `.agent/todos/20261029/20261029-todo-2.md` (Tasks 2, 3, 4, 5 ONLY; Task 1 is DONE and out of scope)
- Global plan: `.kilo/plans/20261029-planning-waves.md` — binding: "Addendum v2 — User Decisions" items 2–7
- Branch: `feat/planning-waves` (created at workflow Step 2 — the executor must NOT create/switch branches)
- Front-end related: **NO** for all four tasks → sub-steps 4.1a / 4.5a are NOT executed
- Documentation-only cycle: no application code, no `Cargo.toml`, no dependency installs, no version entries in CHANGELOG.md

## Plan Amendment (2026-10-29)

- **GAP-20261029-AA RESOLVED — option (a) adopted by the caller (Planner).** `.agent/project-info/architecture.md` is added to the authorized edit set for Task 5. Authorization: caller decision + the NEW Task 5 sub-bullet in `.agent/todos/20261029/20261029-todo-2.md` (line 62): "Sync the same corrections in `.agent/project-info/architecture.md` (authorized GAP-20261029-AA fix, user-approved planning wave): its session-memory component line and its vertical-navigation critical-path line still reference the removed same-filename rule — replace them with the corrected priority; and reflect the corrected fullscreen behavior (default OFF; OFF shows controls with the image central; ON hides header and all controls) in its fullscreen/zoom critical-path line."
- Consequences applied in this revision: §1 item 4 and §4 now include the architecture.md sync (new §4.4 with ACTIVE edits L–O); §7/§8 add the ACTIVE architecture commit; §9 verification set grows to 7 docs; architecture.md is removed from the §8 file-exclusion list and §12 bans.
- Former §11 "FLAGGED GAP — Requires Caller Decision" is CLOSED and kept as a resolution record.

---

## 1. Overview of the Grouped Cycle

This plan covers ONE grouped Critical-Workflow cycle (workflow allows grouping extremely short/related tasks) executing TODO Tasks 2–5 of `.agent/todos/20261029/20261029-todo-2.md`:

1. **Task 2** — App name finalized as **"Images Scroller"**: update all live documentation references that currently describe the name as provisional (`img-scroller`). The repository directory name `img-scroller` remains unchanged (stated in each renamed doc).
2. **Task 3** — Create `CHANGELOG.md` NOW (user decision, planning-waves addendum item 6): Keep a Changelog 1.1 structure + SemVer + explicit "entries begin only after MVP (full v1)" banner. NO version entries.
3. **Task 4** — License = **MIT** (addendum item 5): create `LICENSE` at repo root with the GitHub project URL auto-detected at execution time; rewrite the README License section in plain language; record the decision in `brief.md`.
4. **Task 5** — Propagate the 2026-10-29 requirement corrections into the source-of-truth docs: (a) vertical-scroll target priority — same-filename rule REMOVED; (b) full fullscreen-mode behavior. Synced in `brief.md`, `product.md`, `README.md`, and `architecture.md` (architecture.md authorized per the Plan Amendment at the top).

### Executor Identity & Workflow Step Mapping

- The **4.2 executor is the docs-specialist** sub-agent, acting as a **JUNIOR developer under 50% restriction**: hard-blocked from modifying unrelated files, expanding scope, making architectural decisions, skipping steps, or choosing unspecified approaches.
- Front-end flags: all four tasks = NO → 4.1a and 4.5a are skipped (recorded as N/A).
- Step roles:

| Phase | Step | Role | Output |
| --- | --- | --- | --- |
| Planning | 4.1b | architector (this plan) | `.kilo/plans/20261029-tasks2-5-name-changelog-license.md` |
| Execution | 4.2 | docs-specialist / (junior) | All edits + commits (list in §8) |
| Review | 4.3 | code-reviewer (+ code-simplifier recorded N/A — no source code) | adherence fix plan only if deviations found |
| Documentation | 4.4 | recorded as merged into 4.2/4.3 (the deliverable IS documentation) | — |
| Verification | 4.5b | architector | adherence report `.kilo/plans/20261029-tasks2-5-docs-adherence.md` |
| Completion | 4.6 | implementer | marks Tasks 2, 3, 4, 5 `[DONE]` individually in the TODO file (each sub-item `[]` → `[x]`), preserving all other content; commit per the workflow |

- Historical `.kilo/plans/*.md` files (including planning-waves, project-brief, task1-wave-files-spec, this file) are NEVER edited by the executor.

---

## 2. Executor Pre-Steps (4.2 start) — Deterministic

Run these read-only commands first, ONE at a time (single commands, no chaining):

1. `git branch --show-current` → MUST output `feat/planning-waves`. If not: STOP and return a question to the caller (branch ops are restricted to workflow Step 2).
2. `git status` → workspace must be clean apart from expected untracked artifacts; follow `.kilo/rules/gitignore-compliance.md` (read `.gitignore` before every commit in this cycle; never stage gitignore-matching files).
3. **URL auto-detection (Task 4 input)**: `git remote get-url origin`
   - On success: the output is the raw remote URL. Normalize it mechanically to the canonical GitHub HTTPS URL:
     - If it starts with `git@github.com:` → replace that prefix with `https://github.com/`.
     - Strip a trailing `.git` and any trailing `/`.
     - If, after normalization, the host is not `github.com` → **STOP** and return a question to the caller (do not guess or substitute another host).
   - Result is called `<URL>` for the rest of this plan. ALL `<URL>` occurrences in new file text resolve to this ONE detected URL, everywhere.
   - Derive `<OWNER-CAPS>`: the first path segment after `https://github.com/` in `<URL>`, with every character uppercased verbatim (e.g. `https://github.com/ibej/img-scroller` → owner `IBEJ`; hyphens/digits are kept and uppercased). Defined mechanically: `owner-in-URL.to_uppercase()`.
   - If `git remote get-url origin` fails or `origin` does not exist → **STOP** and return a question to the caller. NEVER invent a URL.
   - The URL/owner detection result must be stated back in the executor's completion summary.

---

## 3. Task 2 — Exact Edits: App Name → "Images Scroller"

Edits are applied per file in the order below; each change is a pure string/line replacement. Do NOT touch any other line. Historical `.kilo/plans/*.md` are NOT modified. Every updated location also states that the repository directory name `img-scroller` remains unchanged.

### 3.1 README.md

- **Edit A — line 1**
  - BEFORE: `# img-scroller`
  - AFTER: `# Images Scroller`
- **Edit B — line 2 (italic note, replace entire line)**
  - BEFORE: `*The repository name \`img-scroller\` is a provisional working name, used until a final product name is chosen; it is not the final product name.*`
  - AFTER: `*The app name is **Images Scroller** (final). The repository directory name \`img-scroller\` remains unchanged — it is the repository name only, not the app name.*`
- **Edit C — line 6, first sentence only (replace only the first two words `img-scroller is` → `Images Scroller is`)**
  - BEFORE: `` `img-scroller` is a Windows desktop image viewer built in Rust. ``
  - AFTER: `` Images Scroller is a Windows desktop image viewer built in Rust. ``
  - The rest of line 6 is preserved verbatim (less leading indentation/space preservation: keep the surrounding line exactly except this replacement).

### 3.2 .agent/project-info/brief.md (line 8)

- BEFORE: `` - Working project name: `img-scroller` (the repository name — provisional, used until a product name is chosen; it is not the final product name). ``
- AFTER: `` - Project name: **Images Scroller** (final). The repository directory name `img-scroller` remains unchanged — it is the repository name only, not the app name. ``

### 3.3 .agent/project-info/product.md (line 5)

- BEFORE: `` - Provisional working name: `img-scroller` — this is the repository name, used until a product name is chosen; it is **not** the final product name. ``
- AFTER: `` - Product name: **Images Scroller** (final). The repository directory name `img-scroller` remains unchanged — it is the repository name only, not the app name. ``

### 3.4 .agent/project-info/context.md (line 7, example phrase only)

- BEFORE: `"[Project Info: Active] I understand we're building \`img-scroller\` (provisional name). Currently working on <current focus>."`
- AFTER: `"[Project Info: Active] I understand we're building **Images Scroller** (repository directory name \`img-scroller\` unchanged). Currently working on <current focus>."`
- Only the example-phrase substring inside line 7 changes; the rest of the bullet is preserved verbatim.

---

## 4. Task 5 — Exact Edits: Requirement Corrections (vertical scroll priority + fullscreen behavior)

Applied AFTER Task 2 edits (per-file order: Task 2 lines first, then these). Files: README, brief, product, architecture (§4.4, ACTIVE per Plan Amendment); context.md gets NO Task 5 edit.

### 4.1 .agent/project-info/brief.md

**Edit D — `### Vertical Scroll (Sibling Folders)`: replace the same-filename-rule bullet (line 50) and rewrite the last-viewed bullet (line 51).** Line 48 (up/down navigation) and line 49 (same-parent restriction) are UNCHANGED.

- BEFORE (line 50, entire line): `` - Same-filename rule: when moving to a sibling folder, the app tries to show the image with the SAME filename as the currently viewed one; if that filename does not exist in the target folder, the app opens the target folder's FIRST image (per the alphanumeric sort order in `## Sorting Order`). ``
  - AFTER (line 50, entire line): `` - Target image priority: (a) if the target folder was already visited this session, the app reopens its session last-viewed image (RAM-only); (b) otherwise, the app ALWAYS opens the target folder's FIRST image in alphanumeric order (per `## Sorting Order`). No other fallback. ``
- BEFORE (line 51, entire line): `` - Last-viewed memory rule (per session): while navigating folders during one app session, the app remembers the last viewed image of each visited folder; revisiting a folder in the SAME session reopens that last viewed image, and this takes priority over the same-filename rule and the first-image rule when the folder was already visited in the session; the memory lives only in RAM for the current session and is destroyed when the application closes — there is NO persistence to disk in v1. ``
  - AFTER (line 51, entire line): `` - Last-viewed memory rule (per session): while navigating folders during one app session, the app remembers the last viewed image of each visited folder; revisiting a folder in the SAME session reopens that last viewed image (this is priority (a) above); the memory lives only in RAM for the current session and is destroyed when the application closes — there is NO persistence to disk in v1. ``

**Edit E — `### Fullscreen Mode`: replace both bullets (lines 55–56) with four bullets.**

- BEFORE (lines 55–56, both lines):
  - `- Toggle fullscreen on/off.`
  - `- Both a keyboard shortcut and a UI control toggle it; exact bindings to be decided during implementation.`
  - AFTER (replacement text):
    - `- Default fullscreen state at launch: OFF — the window opens normal or maximized at the user's own placement/size.`
    - `- Fullscreen OFF: the UI controls are visible (UI layout details are deferred to a later definition by the user) and the image takes almost all of the central space.`
    - `- Fullscreen ON: the window header and ALL UI controls are hidden; only the image is shown.`
    - `- Both a keyboard shortcut and a UI control toggle it; exact bindings to be decided during implementation.`

**Edit F — `## Sorting Order` (line 79), cross-reference consistency only.**

- BEFORE: `` - v1 sorting: alphanumeric for BOTH folders and files. All navigation (horizontal image order, vertical folder order, "first image" fallback, last-viewed revisit) follows this single order. ``
- AFTER: `` - v1 sorting: alphanumeric for BOTH folders and files. All navigation (horizontal image order, vertical folder order, "first image" rule, last-viewed revisit) follows this single order. ``
- Rationale (recorded in plan only): with the same-filename rule removed, the first-image rule is not a "fallback" anymore — it is the (b) priority rule. Deterministic wording alignment; no other line in `## Sorting Order` changes.

- All other brief lines verified NOT to need edits: `### Horizontal Scroll` (no mention of the corrected rules); `## Out of Scope for v1 (Deferred)` line 109 (last-viewed persistence) remains valid; `### Zoom In/Out`, `### Pan and Navigate...`, `### Fast "Open with..."` unaffected.

### 4.2 .agent/project-info/product.md

**Edit G — v1 capability 3 (Vertical Scroll): replace the two sub-bullets (lines 38–39) under item 3.** The item-3 first line (line 37) is UNCHANGED.

- BEFORE (line 38): ``    - Same-filename rule: when moving to a sibling folder, the app tries to show the image with the SAME filename as the currently viewed one; if that filename does not exist in the target folder, the app opens the target folder's FIRST image (per the alphanumeric sort order). ``
  - AFTER (line 38): ``    - Target image priority: (a) if the target folder was already visited this session, the app reopens its session last-viewed image (RAM-only); (b) otherwise, the app ALWAYS opens the target folder's FIRST image in alphanumeric order. No other fallback. ``
- BEFORE (line 39): ``    - Last-viewed memory rule (per session): the app remembers the last viewed image of each visited folder; revisiting a folder in the SAME session reopens that last viewed image, and this takes **priority** over the same-filename rule and the first-image rule when the folder was already visited in the session; the memory lives only in RAM for the current session and is destroyed when the application closes — there is NO persistence to disk in v1. ``
  - AFTER (line 39): ``    - Last-viewed memory rule (per session): the app remembers the last viewed image of each visited folder; revisiting a folder in the SAME session reopens that last viewed image (this is priority (a) above); the memory lives only in RAM for the current session and is destroyed when the application closes — there is NO persistence to disk in v1. ``

**Edit H — v1 capability 4 (Fullscreen Mode, line 40): replace the entire line.**

- BEFORE: `` 4. **Fullscreen Mode** (brief: `### Fullscreen Mode`) — toggle fullscreen on/off via a keyboard shortcut AND a UI control; exact bindings decided during implementation. ``
- AFTER: `` 4. **Fullscreen Mode** (brief: `### Fullscreen Mode`) — default OFF at launch: the window opens normal or maximized at the user's own placement/size; fullscreen OFF shows the UI controls (UI layout details deferred to a later definition by the user) with the image taking almost all of the central space; fullscreen ON hides the window header and ALL UI controls, showing only the image; toggled via a keyboard shortcut AND a UI control, exact bindings decided during implementation. ``

**Edit I — sorting note (line 45), cross-reference consistency only.**

- BEFORE: `` Sorting note: v1 sorting is alphanumeric for BOTH folders and files. All navigation (horizontal image order, vertical folder order, "first image" fallback, last-viewed revisit) follows this single order. ``
- AFTER: `` Sorting note: v1 sorting is alphanumeric for BOTH folders and files. All navigation (horizontal image order, vertical folder order, "first image" rule, last-viewed revisit) follows this single order. ``
- Rest of line 45 is preserved verbatim. Product `## Explicit Non-Goals` line 72 (last-viewed persistence) verified unchanged-valid.

### 4.3 README.md

**Edit J — v1 Features item 3 (line 22): replace the entire line.**

- BEFORE: `` 3. **Vertical Scroll (Sibling Folders)** — up/down arrow keys and up/down UI buttons navigate to the previous/next sibling folder under the same parent folder; when moving to a sibling folder, the file matching the currently viewed image's name is preferred (same-filename rule), otherwise the target folder's first image is opened; the app also remembers the last viewed image per visited folder for the current session (RAM-only, destroyed on app close — no persistence to disk in v1). ``
- AFTER: `` 3. **Vertical Scroll (Sibling Folders)** — up/down arrow keys and up/down UI buttons navigate to the previous/next sibling folder under the same parent folder; when moving to a sibling folder, the target is (a) the folder's session last-viewed image if the folder was already visited this session (RAM-only), otherwise (b) ALWAYS the target folder's first image in alphanumeric order — no other fallback; the app also remembers the last viewed image per visited folder for the current session (RAM-only, destroyed on app close — no persistence to disk in v1). ``

**Edit K — v1 Features item 4 (line 23): replace the entire line.**

- BEFORE: `` 4. **Fullscreen Mode** — toggles fullscreen on/off via a keyboard shortcut and a UI control. ``
- AFTER: `` 4. **Fullscreen Mode** — default OFF at launch: the window opens normal or maximized at the user's own placement/size; when OFF, the UI controls are visible and the image takes almost all of the central space; when ON, the window header and all UI controls are hidden and only the image is shown; toggles via a keyboard shortcut and a UI control (exact bindings deferred). ``

- README lines verified NOT to need Task 5 edits: item 2 (horizontal), item 6 (pan/navigate), the additional-scope notes (lines 28–31).

### 4.4 .agent/project-info/architecture.md (ACTIVE per Plan Amendment — GAP-20261029-AA)

**Edit L — `## Logical Components (Planned)`, Navigation Engine bullet (line 18): replace the entire line.**

- BEFORE: `` - **Navigation Engine (planned)**: horizontal — previous/next sibling image in the current folder (alphanumeric order); vertical — previous/next sibling folder under the SAME parent (same-filename rule, first-image fallback in alphanumeric order). ``
- AFTER: `` - **Navigation Engine (planned)**: horizontal — previous/next sibling image in the current folder (alphanumeric order); vertical — previous/next sibling folder under the SAME parent (target image priority: (a) session last-viewed image for folders already visited this session, (b) otherwise ALWAYS the target folder's first image in alphanumeric order). ``

**Edit M — `## Logical Components (Planned)`, Session Last-Viewed Memory bullet (line 19): replace the entire line.**

- BEFORE: `` - **Session Last-Viewed Memory (planned)**: RAM-only map of the last viewed image per visited folder; last-viewed takes priority over the same-filename and first-image rules when the folder was already visited this session; destroyed on app close; no disk persistence in v1. ``
- AFTER: `` - **Session Last-Viewed Memory (planned)**: RAM-only map of the last viewed image per visited folder; last-viewed is priority (a) when the folder was already visited this session; destroyed on app close; no disk persistence in v1. ``

**Edit N — `## Navigation Critical Paths (Planned)`, item 3 Vertical navigation (line 28): replace the entire line.**

- BEFORE: `` 3. **Vertical navigation**: arrow key or UI button → previous/next sibling folder under the same parent → target image chosen by priority: (a) last viewed this session (if the folder was already visited), else (b) the same filename if it exists in the target folder, else (c) the target folder's first image (alphanumeric order) → decode → display; while the image is taller than the window, pan first; navigation only at the image's top/bottom edge. ``
- AFTER: `` 3. **Vertical navigation**: arrow key or UI button → previous/next sibling folder under the same parent → target image chosen by priority: (a) the folder's session last-viewed image if the folder was already visited this session (RAM-only), else (b) ALWAYS the target folder's first image (alphanumeric order) → decode → display; while the image is taller than the window, pan first; navigation only at the image's top/bottom edge. ``

**Edit O — `## Navigation Critical Paths (Planned)`, item 4 Fullscreen / zoom toggles (line 29): replace the entire line (fullscreen behavior sync).**

- BEFORE: `` 4. **Fullscreen / zoom toggles**: keyboard shortcut and UI control toggle fullscreen; zoom via shortcut + controls; fit width / fit height modes. ``
- AFTER: `` 4. **Fullscreen / zoom toggles**: fullscreen is OFF by default at launch (the window opens normal or maximized at the user's own placement/size); fullscreen OFF shows the UI controls with the image taking almost all of the central space; fullscreen ON hides the window header and ALL UI controls, showing only the image; keyboard shortcut and UI control toggle it; zoom via shortcut + controls; fit width / fit height modes. ``

- All other architecture.md lines verified NOT to need Task 5 edits: line 20 (Pan/Zoom Controller), `## Data and State` (last-viewed persistence statements remain valid), `## Technical Decisions Summary` line 60 (section pointer only), `## Open Architectural Decisions` (keybindings still deferred).

---

## 5. Task 3 — Exact New File: CHANGELOG.md

Create `CHANGELOG.md` at the repository root with EXACTLY this final content (Keep a Changelog 1.1, badge-free, real line breaks, single trailing newline, `---` not present; empty stub sections have no body lines):

```markdown
# Changelog

All notable changes to **Images Scroller** are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project uses Semantic Versioning (SemVer).

> No entries yet — changelog updates begin only after the MVP version (full v1) exists.

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security
```

Hard constraints for this file:

- NO version entries of any kind; the ONLY version-like heading is `## [Unreleased]`.
- The six section stubs under `[Unreleased]` stay empty (this is the Keep a Changelog convention for an empty Unreleased block).
- The MVP checkpoint is end of Wave 7 (full v1) — addendum item 2; Waves 1–6 must never touch this file in future waves.
- Banner wording is EXACT as above (supersedes the wording variant in planning-waves addendum item 6; caller's latest directive uses "begin only after").

---

## 6. Task 4 — Exact New File: LICENSE + README License Section + brief.md Decision Record

### 6.1 LICENSE (new file, repository root)

The executor uses the detected `<OWNER-CAPS>` and `<URL>` from §2. Final content template (standard MIT text; the copyright line embeds the project URL exactly in the pattern `Copyright (c) 2026 <OWNER-CAPS> — Images Scroller: <URL>`):

```markdown
MIT License

Copyright (c) 2026 <OWNER-CAPS> — Images Scroller: <URL>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

Rules: the MIT permission paragraphs are EXACTLY the standard text above (one paragraph per line, single blank line between blocks: heading / copyright / two paragraphs); `<OWNER-CAPS>` and `<URL>` are the only substituted tokens; no other placeholder, no trailing commentary. If the URL could not be detected per §2 → STOP (do not create a file with an invented URL).

### 6.2 README.md — `## License` section replacement

- BEFORE (line 68, the single paragraph under `## License`): `` No LICENSE file exists in this repository, and the project's license has not been determined yet. Until a license is added, treat the project as all-rights-implicitly-reserved by the author and do not assume redistribution rights. ``
- AFTER (`## License` heading unchanged; body replaced by exactly):

```markdown
Images Scroller is released under the MIT License; the full text is in the [LICENSE](LICENSE) file, and its copyright notice embeds the GitHub project URL.

- The app can be used 100% free of charge by anyone.
- Forking the project, changing the code, and using a modified version are allowed.
- Anyone sharing the app outside the GitHub **Releases** section of the GitHub project (<URL>) must add a reference to that GitHub project (attribution).
```

`<URL>` resolves to the SAME detected URL as §6.1 (applied in plain form inside the parentheses, e.g. `(https://github.com/owner/img-scroller)` — no markdown link syntax around it).

### 6.3 .agent/project-info/brief.md — license decision record (location DECIDED here)

Recorded in `## Distribution and Default App Registration` (license rules are distribution/sharing constraints; NOT in `## Technology Stack`). Add ONE new bullet as the LAST bullet of that section (after line 87, before the next `##` heading):

- AFTER (new bullet, exact text): `` - License: MIT (the full text is in the repository root `LICENSE` file). Anyone can use the app 100% free; forks and modified versions are allowed; anyone sharing the app outside the future GitHub "Releases" section must add a reference to the GitHub project — the project URL is embedded in the LICENSE copyright notice. ``
- No other bullet in that section changes.

---

## 7. Per-File Edit Sequencing (deterministic, so Task 2 + 4 + 5 compose cleanly)

Apply in this exact order; groups map 1-to-1 onto the commit list (§8) — all add/commit boundaries happen BETWEEN groups:

| Group | Files touched | Edits (in order within the group) |
| --- | --- | --- |
| G1 (Task 2) | README, brief, product, context | README A → B → C; brief 3.2; product 3.3; context 3.4 |
| G2 (Task 5 — 2 commits, see §8 steps 2–3) | brief, product, README, architecture | brief D → E → F; product G → H → I; README J → K; architecture L → M → N → O |
| G3 (Task 3) | CHANGELOG.md (new) | whole file per §5 |
| G4 (Task 4) | LICENSE (new), README, brief | LICENSE whole file (§6.1); README §6.2; brief §6.3 |

No two groups touch the same line of the same file (README lines 1, 2, 6 → G1; lines 22, 23 → G2; `## License` body → G4; brief line 8 → G1; lines 50–51, 55–56, 79 → G2; distribution bullet → G4; product line 5 → G1; lines 38–39, 40, 45 → G2; architecture lines 18, 19, 28, 29 → G2) — group order is therefore safe regardless of file-level interleaving. Within G2, the architecture.md edits (L–O) are applied AFTER the brief/product/README edits (D–K), in that order.

---

## 8. Execution Order + Exact Commit List (FINAL — architecture commit ACTIVE)

Before EACH commit: read `.gitignore`, run `git status` (gitignore-compliance rule), stage only the named files, then commit. Single commands only (no `&`/`&&` chaining; `git add` and `git commit` as separate tool calls).

1. `git add README.md .agent/project-info/brief.md .agent/project-info/product.md .agent/project-info/context.md`
2. `git commit -m "docs: rename app to Images Scroller in live documentation (task 2)"`
3. `git add README.md .agent/project-info/brief.md .agent/project-info/product.md`
4. `git commit -m "docs: record vertical scroll and fullscreen rule corrections (task 5)"`
5. `git add .agent/project-info/architecture.md`
6. `git commit -m "docs: sync architecture.md with vertical scroll and fullscreen corrections (task 5)"`
7. `git add CHANGELOG.md`
8. `git commit -m "docs: add keep-a-changelog structure template (task 3)"`
9. `git add LICENSE`
10. `git commit -m "docs: add MIT license with project attribution (task 4)"`
11. `git add README.md .agent/project-info/brief.md`
12. `git commit -m "docs: add license summary and attribution note to README and brief (task 4)"`

Commit list summary: exactly **6 commits** — G1 (name-rename), G2a (corrections), G2b (architecture sync, immediately after the corrections commit), G3 (changelog), G4a (LICENSE only), G4b (README license + brief decision). The final commit list is FIXED (no merging of README commits: the README License edit is task-4-specific and committed separately from the LICENSE file creation for deterministic review).

- Git operations FORBIDDEN for the executor: branch create/switch, merge, push (branch ops = workflow Step 2 only; push = Step 5 only).
- Files NOT to be created/modified: AGENTS.md, `.kilo/plans/*.md` (historical), `.agent/todos/**` (all TODO/wave files, including the source TODO `20261029-todo-2.md` — the `[DONE]` marking belongs to workflow 4.6, a separate sub-task), `tech.md`, `docs/`, `src/`, `.opencode/`, `.kilo/rules/*`, `.agent/WORKFLOWS.md`. `architecture.md` IS authorized ONLY for the four line edits in §4.4 and nothing else.

---

## 9. Verification Checklist (for 4.2 completion signal and 4.3/4.5b review)

1. `git status` clean after the final commit; `git log --oneline -7` shows the expected 6 doc commits on `feat/planning-waves`.
2. `git diff main...feat/planning-waves` shows changes ONLY in: `README.md`, `.agent/project-info/brief.md`, `.agent/project-info/product.md`, `.agent/project-info/context.md`, `.agent/project-info/architecture.md`, and additions of `CHANGELOG.md` and `LICENSE` (**7 files** — the task prompt's "8 touched" parenthetical reconciles with these 7 docs plus the TODO-file `[DONE]` marking (4.6) and this plan/adherence artifacts; the enumerated list is authoritative).
3. Provisional-name scan — `grep -ni "provisional"` in README.md, brief.md, product.md, context.md, architecture.md → **0 matches**.
4. Repository-name scan — `grep -n "img-scroller"` (without file glob restriction) in README.md, brief.md, product.md, context.md, architecture.md → EXACTLY one match per file, and each match is inside the "repository directory name ... remains unchanged" wording:
   - README line 2; brief line 8; product line 5; context line 7. (architecture.md contains no app-name references — expected 0 matches in this file.)
5. Removed-rule scan — `grep -in "same-filename"` and `grep -in "same filename"` in README.md, brief.md, product.md, architecture.md → **0 matches**.
6. Priority wording present — `grep -n "Target image priority"` → brief.md line 50, product.md line 38; `grep -n "no other fallback"` → brief.md line 50, product.md line 38, README.md line 22; `grep -n "target image priority"` → architecture.md line 18; `grep -n "ALWAYS the target folder's first image"` → architecture.md line 28 (and brief.md line 50 / product.md line 38 / README.md line 22 among them).
7. Fullscreen wording present — `grep -n "Default fullscreen state at launch"` → brief line 55; `grep -in "default off at launch"` → README line 23; `grep -n "default OFF at launch"` → product line 40; `grep -n "fullscreen is OFF by default at launch"` → architecture.md line 29; `grep -n "ALL UI controls are hidden"` → architecture.md line 29.
8. CHANGELOG scan — `grep -n "^## \[" CHANGELOG.md` → ONLY `## [Unreleased]`; no numeric version headings; the SemVer line and the banner line are present (per §5).
9. LICENSE scan — copyright line matches, e.g. `Copyright (c) 2026 IBEJ — Images Scroller: https://github.com/ibej/img-scroller` shape; contains the three standard MIT paragraphs; no placeholders remaining (`grep -n "<URL>\|<OWNER-CAPS>"` → 0 matches in LICENSE and README).
10. README License section no longer contains the sentence "No LICENSE file exists"; brief.md `## Distribution and Default App Registration` ends with the MIT decision bullet.
11. Historical `.kilo/plans/*.md` and all `.agent/todos/**` untouched by these 6 commits (diff shows no changes outside the 7-file set).

---

## 10. Deviations / STOP Conditions (executor: follow verbatim)

- STOP and return a question to the caller (do not proceed, do not guess) if ANY of:
  1. Branch ≠ `feat/planning-waves` at 4.2 start.
  2. `git remote get-url origin` fails, missing, or does not resolve to `https://github.com/...` after normalization.
  3. Any BEFORE text in §§3–6 is not found exactly (file content drifted) — do not approximate with fuzzy matching.
  4. Workspace dirty at start, or any commit is rejected by hooks.

---

## 11. GAP-20261029-AA — RESOLVED (record only; no caller decision pending)

**CLOSED 2026-10-29 by the caller (Planner) decision: option (a) adopted** — `.agent/project-info/architecture.md` is authorized for the Task 5 sync per the Plan Amendment at the top and the NEW Task 5 sub-bullet in the source TODO (line 62).

- The formerly gated ("ready but not instructed") architecture.md edits are **ACTIVE** as §4.4 (edits L, M, N, O).
- The formerly optional architecture commit is **ACTIVE** in §8 as commit 3 (`docs: sync architecture.md with vertical scroll and fullscreen corrections (task 5)`), positioned immediately after the brief/product/README corrections commit.
- The fullscreen sync (edit O, line 29) replaces the former "optional secondary line 29 addition" note.
- This section remains ONLY as a historical record of the flagged gap; no `architecture.md` drift remains expected after execution.

---

## 12. Banned Actions (verbatim recap)

- NO changelog version entries (only empty `[Unreleased]` stubs).
- NO invented GitHub URL or owner (STOP per §10.2 instead).
- NO code, no `Cargo.toml`, no dependency installs.
- NO edits to `.kilo/plans/*.md` historical files (including this plan file — plans are not edited by the executor).
- NO edits to AGENTS.md.
- NO edits to the wave TODO files (`.agent/todos/**`, including the source TODO `20261029-todo-2.md` and the Wave 1–7 files) — the `[DONE]` marking is performed at workflow 4.6 by the implementer sub-agent, not by the 4.2 executor.
- NO branch/push operations by the executor (branch = Step 2; push = Step 5, per addendum item 7 — after Step-4/6 completion by the implementer, merge to `main` then push `main` to `origin` ONLY).
