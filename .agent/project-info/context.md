# Project Context Log

## How to Use This File

Project-info rules for AI agents (full rules: [instructions.md](instructions.md)):

- **Automatic Read**: at the start of EVERY task, agents MUST automatically read this file to understand the current project state. The first response MUST open with `[Project Info: Active]` plus a confirmation phrase aligned to the current focus, e.g. "[Project Info: Active] I understand we're building `Images Scroller` (repository directory `img-scroller`). Currently working on <current focus>."
- **Critical Closing Step**: before emitting the final completion signal or stopping execution, agents MUST update this file: recent changes, then current state, then immediate next steps.
- **Context window overflow**: if the context window fills during an extended session, write the current session state into this file, then recommend the user start a fresh chat — the new session resumes from this file.
- **On-demand reads**: `architecture.md`, `tech.md`, and `product.md` are read when a task directly impacts them.
- All project-info rules live in [instructions.md](instructions.md).

## Current Work Focus

Dated 2026-10-29. The **planning wave** defined in [`.agent/todos/20261029/20261029-todo-2.md`](../todos/20261029/20261029-todo-2.md) (completed on `feat/planning-waves`; renamed `-DONE` at workflow Step 5) and its global plan [`.kilo/plans/20261029-planning-waves.md`](../../.kilo/plans/20261029-planning-waves.md) (all 5 tasks `[DONE]`) is **complete**: 7 phased implementation TODO files defined, app named, changelog structured, license added, requirement corrections propagated. No application code written (doc-only session, per user instruction).

## Recent Changes

1. 2026-10-29: Branch `feat/planning-waves` created from `main` (planning-wave TODO + global plan committed as `d4dbf93`); Step 3 (version update) = N/A, no version manifest.
2. 2026-10-29: **Task 1 done** (`1f1069a`, `a91fd79`): 7 wave TODO files created ([`20261029-todo-3.md`](../todos/20261029/20261029-todo-3.md) … [`20261029-todo-9.md`](../todos/20261029/20261029-todo-9.md)) per the approved spec `.kilo/plans/20261029-task1-wave-files-spec.md` — Wave 1 skeleton/open image, Wave 2 horizontal scroll, Wave 3 vertical scroll, Wave 4 fullscreen+zoom, Wave 5 pan-then-navigate, Wave 6 "Open with...", Wave 7 distribution + MVP checkpoint; code-reviewer: no fixes; architector adherence: CONFORMS.
3. 2026-10-29: **Tasks 2–5 done** (commits `5e0af25`, `c7d6a07`, `e7a49f1`, `a67b47a`, `0c8b402`, `cca0fc5`, `9a96578`): live docs renamed to **Images Scroller** (repository directory `img-scroller` unchanged; `provisional` wording removed from live docs); requirement corrections propagated to brief/product/README/architecture — vertical navigation: **NO same-filename rule** (visited-this-session → session last-viewed image, RAM-only; otherwise ALWAYS the target folder's first image); fullscreen: default OFF at launch, window at the user's placement, OFF shows UI controls with the image almost all central space, ON hides the window header and ALL controls (shows only the image); [`CHANGELOG.md`](../../../CHANGELOG.md) created with the Keep a Changelog + SemVer structure and the banner "entries begin only after the MVP version (full v1) exists"; MIT [`LICENSE`](../../../LICENSE) added — `Copyright (c) 2026 KAZUYAKUZA — Images Scroller: https://github.com/kazuyakuza/img-scroller` (URL auto-detected from `origin`, credential userinfo stripped by caller approval); README License section rewritten; license decision recorded in the brief's Distribution section. Bound decisions: `.kilo/plans/20261029-planning-waves.md` `## Addendum v2`; cycle plan `.kilo/plans/20261029-tasks2-5-name-changelog-license.md`; adherence: CONFORMS.
4. 2026-10-29: **MVP checkpoint = end of Wave 7 (full v1)** per user decision — waves 1–6 must NOT touch `CHANGELOG.md`; only Wave 7's final task writes the first entry.

## Current Project State

- Greenfield: **no source code, no `Cargo.toml`.** `src/` is still only the `.gitkeep` placeholder. The repository holds docs/config scaffolding, plus the new `CHANGELOG.md` (empty structure) and MIT `LICENSE`.
- App name: **Images Scroller**; repository directory remains `img-scroller`.
- Requirement corrections (2026-10-29) are now the source of truth in brief/product/architecture/README — the same-filename rule is REMOVED from v1 (session last-viewed → first image priority); the fullscreen OFF/OFF-UI/ON-minimal behavior is recorded. UI layout details remain deferred to later user definition.
- The 7 wave TODO files are ready; each is meant to be processed standalone in a future session via the Critical Workflow (tasks one 4.x cycle at a time), on its own `feat/wave-N-*` branch.
- Git: planning wave completed on `feat/planning-waves` (Step 5: rename TODO `-DONE`, merge to `main`, push `main` to `origin` ONLY per user decision).

## Immediate Next Steps

1. Step 5 wrap-up (this session): commit the context-log update, rename the planning-wave TODO with the `-DONE` suffix, merge `feat/planning-waves` into `main`, delete the feature branch, push `main` to `origin` (only remote push target allowed; `base-project` is fetch-only).
2. **Implementation waves begin** in a NEW session: process [`.agent/todos/20261029/20261029-todo-3.md`](../todos/20261029/20261029-todo-3.md) (Wave 1 — project skeleton and open image) via the Critical Workflow. One wave file per session, in order (todo-3 → todo-9).
3. Keep the project-info core files current as implementation lands (architecture, tech, this context log).
