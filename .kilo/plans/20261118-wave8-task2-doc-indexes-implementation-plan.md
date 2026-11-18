# Wave 8, Task 2 — README TOC + Long-Doc Indexes — Implementation Plan (4.1b)

- Wave: 8 (`feat/wave-8-wizard-installer`; branch creation is restricted to workflow Step 2 — this task does NOT create or switch branches)
- Task: 2 — README TOC + long-doc indexes (`.agent/todos/20261117/20261117-todo-1.md` → `### Task 2`)
- Workflow step: 4.1b (Analysis and Planning — architector). Produces THIS plan. The 4.2 implementer executes §4; 4.3/4.4/4.5b/4.6 follow §6.
- Binding sources: the TODO Task 2 verbatim; the Critical Workflow (`.kilo/commands/critical-workflow.md`); the >100-line index rule (workflow step 4.4: "Add TOC/Index when doc file > 100 lines") extended by Task 2 to all long docs.
- Scope fence: ONLY the files in §1's MODIFY list are edited. Tasks 1 (DONE), 3–7 are out of scope.

## 1. Verified inventory (measured at this commit — line counts are authoritative)

Measured with the read tool's exact-total line counts (file: total lines — index status):

| File | Lines | >100? | Index today | Disposition |
| --- | --- | --- | --- | --- |
| `README.md` | 133 | YES | NONE | **MODIFY — add `## Contents`** (§4.1) |
| `docs/build-windows.md` | 193 | YES | NONE (Task 1 already extended it with the NSIS section; recount confirmed 193) | **MODIFY — add `## Contents`** (§4.2) |
| `.agent/project-info/context.md` | 116 | YES | NONE (it has a "How to Use This File" usage block, lines 3–11) | **MODIFY — add `## Contents`** (§4.3; decision D3) |
| `.agent/project-info/tech.md` | 222 | YES | `## Contents` at line 7 (19 items) | VERIFY-ONLY (§4.4) |
| `.agent/project-info/architecture.md` | 150 | YES | `## Contents` at line 7 (11 items) | VERIFY-ONLY (§4.4) |
| `docs/build.md` | 80 | NO | NONE | VERIFY-ONLY — exempt by the >100 rule at this commit (§4.4) |
| `docs/how-to-write-todo-files.md` | 88 | NO | NONE | VERIFY-ONLY — exempt (§4.4) |
| `docs/how-to-set-up-git.md` | 62 | NO | NONE | VERIFY-ONLY — exempt (§4.4) |

Cross-check facts (verified this step): every `##` heading of tech.md (19 content headings, excluding `## Status` and `## Contents`) and architecture.md (11 content headings, excluding `## Status` and `## Contents`) is already covered 1:1 by its existing `## Contents` list — the anchor-format style to match is the tech.md/architecture.md numbered list (`1. [Heading](#heading-slug)`). `README.md` already uses GitHub-flavored anchors elsewhere (`[Project Status](#project-status)` at its Technology-Stack line) — the anchor format below is consistent with that. `CHANGELOG.md` is deliberately NOT in scope: the TODO Task 2 enumerates exactly `README.md`, `tech.md`, `architecture.md`, `context.md`, `docs/*.md` — CHANGELOG is not among them (no scope expansion).

## 2. Settled decisions (no open choices)

- **D1 — TOC format**: every new index is a `## Contents` section whose body is a numbered list `1. [Heading](#anchor)` of GitHub-flavored-markdown anchors, one list item per `##` content heading — matching the existing tech.md/architecture.md style verbatim. Only `README.md` additionally lists its `###` sub-headings as indented unnumbered sub-bullets (`   - [Sub-heading](#sub-anchor)`) beneath their parent `##` item, because the README's subsections (install steps, default-viewer flow, build commands) are its most navigation-worthy content; tech.md/architecture.md/build-windows.md have zero `###` headings so nothing is lost there. Slugs: lowercase; spaces → hyphens; punctuation (`(`, `)`, `/`, `.`) stripped; a removed `/` between two hyphens leaves a DOUBLE hyphen (e.g. `Overwrite / rebuild behavior` → `#overwrite--rebuild-behavior`) — this is standard GitHub slug behavior and is pinned per-item in §4.
- **D2 — TOC position**: each new `## Contents` is inserted immediately BEFORE the file's first content `##` heading, with exactly one blank line between every element (see §4 for the per-file pinned insertion point).
- **D3 — context.md gets an index (NOT exempt)**: the file is 116 lines (>100) and its "How to Use This File" block is USAGE GUIDANCE for agents, not a content index — it does not list the log sections. The >100 rule applies uniformly; the four log sections (`Current Work Focus`, `Recent Changes`, `Current Project State`, `Immediate Next Steps`) are stable headings, so upkeep cost is minimal. The `## Contents` lists ONLY the four log sections (tech.md's Contents excludes `## Status` by the same convention; the How-to preamble is excluded here by the same convention).
- **D4 — README TOC subsections**: INCLUDE the five `###` sub-headings (§4.1 lists them verbatim) as sub-bullets.
- **D5 — update triggers (rolling rule, encoded now)**: any later task of this wave (Tasks 3/4/6) that adds, removes, or retitles a `##`/`###` heading in a file carrying a `## Contents` MUST update that file's `## Contents` in the SAME edit (add the item at the correct list position). Task 6's documentation step performs the FINAL TOC-coverage sweep (its 4.4/4.5b re-checks every `## Contents` against the actual headings); Task 3's hotkeys record (if it adds a heading under `## v1 Features`) and Task 4's window-title docs (Task 6) are the known candidates. The TOCs added here list sections that exist AT THIS COMMIT — nothing speculative.
- **D6 — standing boundary rule**: if any later step's edits push a currently-≤100 doc (`docs/build.md`, `docs/how-to-write-todo-files.md`, `docs/how-to-set-up-git.md`) over 100 lines, that doc gains a `## Contents` at that moment (same format). Per the current wave plan, none of these files is in Tasks 3–7's edit lists, so none is expected to cross the boundary; this rule is recorded as a guard, not an action.
- **D7 — no `alpine-vm` MCP / no harness run**: this is a pure-markdown task; zero `src/` or manifest changes. NO dev-checks run, NO VM/docker commands. Verification is §5 (read-tool line counts + slug/heading cross-check).
- **D8 — commit messages (verbatim, pinned)**: commit A (the plan file only) = `docs: add wave8 task2 doc-indexes implementation plan`; commit B (the three file edits) = `docs: add wave8 task2 readme toc and long-doc indexes` (the caller's example message, pinned verbatim).
- **D9 — git boundaries**: no branch creation/switch (Step 2's job), no `git push` (Step 5's job), no TODO `[DONE]` marks (4.6's job), no edits to existing `.kilo/plans/*` files.

## 3. Prohibitions for the 4.2 implementer

- NO code changes; do NOT touch `src/`, `Cargo.toml`, `Cargo.lock`, `packaging/`, `scripts/`, `.gitattributes`.
- NO edits to `tech.md` or `architecture.md` (verify-only this task), NO edits to `docs/build.md`, `docs/how-to-write-todo-files.md`, `docs/how-to-set-up-git.md`.
- NO edits to `brief.md`, `product.md`, `future-ideas.md`, `CHANGELOG.md`, `AGENTS.md`, `.agent/project-structure.md`, or the TODO file (4.6's job).
- NO reflow/rewriting of any existing prose — the ONLY change per file is the inserted `## Contents` block (plus nothing else).
- Markdown files are not gitignored; still run `git status` before each commit and stage ONLY the intended files (Gitignore Compliance Rule).

## 4. Exact edits

### 4.1 `README.md` — add `## Contents`

- Insertion point: after the title block — line 1 (`# Images Scroller`) and line 2 (the italic app-name note) — INCLUDING the blank line 3, i.e. the new block goes immediately BEFORE the current `## Overview` heading (currently line 4). After the edit the sequence is: italic note, blank, `## Contents`, blank, the 16 TOC lines below, blank, `## Overview`.
- Insert EXACTLY this block (16 lines: the heading + blank + 9 numbered items + 5 sub-items; slug check per item is given in the comment-free form — verify during review that each anchor matches the corresponding heading's GitHub slug):

```markdown
## Contents

1. [Overview](#overview)
2. [Problem Statement](#problem-statement)
3. [v1 Features](#v1-features)
4. [Technology Stack](#technology-stack)
5. [Distribution and Installation](#distribution-and-installation)
   - [Install (the destination folder is your choice)](#install-the-destination-folder-is-your-choice)
   - [Make Images Scroller the default image viewer (optional)](#make-images-scroller-the-default-image-viewer-optional)
6. [Project Status](#project-status)
7. [Build and Run](#build-and-run)
   - [Container dev checks (no host Rust/Cargo)](#container-dev-checks-no-host-rustcargo)
   - [Windows release cross-builds (x64 and x86)](#windows-release-cross-builds-x64-and-x86)
   - [Portable zip packaging](#portable-zip-packaging)
8. [Note for AI Agents](#note-for-ai-agents)
9. [License](#license)
```

- Coverage = ALL content headings of README.md: `## Overview`, `## Problem Statement`, `## v1 Features`, `## Technology Stack`, `## Distribution and Installation` (+ `### Install (the destination folder is your choice)`, `### Make Images Scroller the default image viewer (optional)`), `## Project Status`, `## Build and Run` (+ `### Container dev checks (no host Rust/Cargo)`, `### Windows release cross-builds (x64 and x86)`, `### Portable zip packaging`), `## Note for AI Agents`, `## License`. The `#` title itself is NOT listed (tech.md/architecture.md precedent: the Contents list covers the content sections, not the document title).

### 4.2 `docs/build-windows.md` — add `## Contents`

- Insertion point: after the intro paragraph (lines 3–6), immediately BEFORE the current `## Prerequisites` heading (line 8; line 7 is the separating blank). After the edit: intro, blank, `## Contents`, blank, the 13 TOC lines below, blank, `## Prerequisites`.
- Insert EXACTLY this block (13 lines: heading + blank + 11 numbered items):

```markdown
## Contents

1. [Prerequisites](#prerequisites)
2. [Invocation](#invocation)
3. [Expected output path](#expected-output-path)
4. [Overwrite / rebuild behavior](#overwrite--rebuild-behavior)
5. [Typed failure table](#typed-failure-table)
6. [Toolchain / target notes](#toolchain--target-notes)
7. [Static-linking / import audit (retained scope)](#static-linking--import-audit-retained-scope)
8. [Portable zip packaging](#portable-zip-packaging)
9. [Windows installer packaging (NSIS)](#windows-installer-packaging-nsis)
10. [Validation stance](#validation-stance)
11. [Lockfile policy](#lockfile-policy)
```

- Coverage = ALL `##` headings of the file (it has zero `###` headings). Double-hyphen slugs (items 4, 6, 7) are the pinned GitHub behavior (see D1).

### 4.3 `.agent/project-info/context.md` — add `## Contents` (decision D3)

- Insertion point: after the "How to Use This File" block (lines 3–11), immediately BEFORE the current `## Current Work Focus` heading (line 13; line 12 is the separating blank). After the edit: last How-to bullet, blank, `## Contents`, blank, the 7 TOC lines below, blank, `## Current Work Focus`.
- Insert EXACTLY this block (7 lines: heading + blank + 4 numbered items; the How-to preamble is deliberately NOT listed — same convention as tech.md excluding `## Status`):

```markdown
## Contents

1. [Current Work Focus](#current-work-focus)
2. [Recent Changes](#recent-changes)
3. [Current Project State](#current-project-state)
4. [Immediate Next Steps](#immediate-next-steps)
```

### 4.4 Verify-only files (NO edits)

- `tech.md` (222 lines): confirm `## Contents` (line 7) lists exactly its 19 content `##` headings in document order — CONFIRMED at plan time; record the verdict.
- `architecture.md` (150 lines): confirm `## Contents` (line 7) lists exactly its 11 content `##` headings in document order — CONFIRMED at plan time; record the verdict.
- `docs/build.md` (80), `docs/how-to-write-todo-files.md` (88), `docs/how-to-set-up-git.md` (62): all ≤100 → exempt; record the verdict. (Note: `docs/build.md` DOES contain `###` sub-headings — irrelevant while ≤100.)

## 5. Verification (4.2 executes; all steps are pure read checks — NO VM, NO docker, NO harness)

1. After the three edits, re-read each modified file and record its new total line count: expected README.md = 150 (133 + 17 inserted lines: heading + blank + 16-list block + 1 trailing blank), docs/build-windows.md = 207 (193 + 14), context.md = 123 (116 + 7). The delta must equal EXACTLY the inserted block size; `git diff` must show ONLY inserted lines (no modified/deleted lines elsewhere in these files).
2. Link self-check: for every TOC item, re-derive the anchor from the target heading with the D1 slug rules and confirm the item's anchor matches. Every anchor must resolve to a REAL heading in the SAME file (relative same-file anchors only — no cross-file links in the new TOCs).
3. Coverage self-check: re-grep each modified file for `^#{1,3} ` headings and confirm every content heading appears in its `## Contents` (README: 9 `##` + 5 `###`; build-windows: 11 `##`; context: 4 `##`).
4. Confirm the two pre-existing indexes still cover tech.md/architecture.md (§4.4 verdicts recorded).
5. `git status` clean of unintended files; stage only the three edited files for commit B.

## 6. Workflow sub-step mapping for Task 2

- **4.2 implementer**: commit A (plan file, message D8); apply §4.1–4.3 edits exactly; run §5 verification; commit B (the three files, message D8). Report what was done / NOT done.
- **4.3 code-reviewer + code-simplifier**: review the three edits against THIS plan (insertion points, exact block content, slug correctness); simplification is NOT expected (a TOC list has nothing to simplify — record "not required" if so).
- **4.4 docs-specialist**: VERIFY-ONLY for this task — the task itself IS the docs-index work; the 4.4 agent re-checks that every >100-line doc in the Task-2 enumeration now carries an index and that links resolve; it must NOT re-edit the TOCs (any fix goes through the planner). Expected summary: "verified, no changes".
- **4.5b architector**: adherence check of the implementation against THIS plan; report diffs (if any) and acceptability.
- **4.6 implementer**: mark `### Task 2` `[DONE]` in the TODO file with both sub-items `[]` → `[x]` (preserve all other content), commit `task: mark wave8 task2 done`.
- NO `git push`, NO branch actions, NO version changes (Step 3 already handled the wave bump).

## 7. Plan-vs-task verification (this step's closing check)

Task 2 requires: README TOC near the top (§4.1 — after title/intro, before first section ✓); every >100 doc verified with an index added where missing (§1 inventory: all seven enumerated files dispositioned ✓); format consistency with the existing `## Contents` style (§2 D1 ✓); context.md decision recorded (D3 — ADD ✓); update triggers recorded (D5 ✓); sub-step mapping + commit messages recorded (D8, §6 ✓); prohibitions + verification recorded (§3, §5 ✓). All task requirements are encoded; no open choices remain.
