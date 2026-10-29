# Project Context Log

## How to Use This File

Project-info rules for AI agents (full rules: [instructions.md](instructions.md)):

- **Automatic Read**: at the start of EVERY task, agents MUST automatically read this file to understand the current project state. The first response MUST open with `[Project Info: Active]` plus a confirmation phrase aligned to the current focus, e.g. "[Project Info: Active] I understand we're building `img-scroller` (provisional name). Currently working on <current focus>."
- **Critical Closing Step**: before emitting the final completion signal or stopping execution, agents MUST update this file: recent changes, then current state, then immediate next steps.
- **Context window overflow**: if the context window fills during an extended session, write the current session state into this file, then recommend the user start a fresh chat — the new session resumes from this file.
- **On-demand reads**: `architecture.md`, `tech.md`, and `product.md` are read when a task directly impacts them.
- All project-info rules live in [instructions.md](instructions.md).

## Current Work Focus

Dated 2026-10-29. Executing the wave defined in [`.agent/todos/20261029/20261029-todo-1.md`](../todos/20261029/20261029-todo-1.md) and the global plan [`.kilo/plans/20261029-project-brief.md`](../../.kilo/plans/20261029-project-brief.md): **Project Info initialization** — a documentation-only wave, no application code.

Active item: **Task 2** — creating the 4 remaining core project-info files (`product.md`, `context.md`, `architecture.md`, `tech.md`) derived from the approved brief. Implementation plan: `.kilo/plans/20261029-task2-info-implementation.md`.

## Recent Changes

1. 2026-10-29: Feature branch `feat/project-brief-and-info` created from `main` per Critical Workflow step 2.
2. 2026-10-29: Version-update step (workflow step 3) skipped: no version manifest exists (no `Cargo.toml` / `package.json`) — recorded as N/A in the global plan.
3. 2026-10-29: Task 1 [DONE]: [`.agent/project-info/brief.md`](brief.md) rewritten per the approved plan (overview, problem statement, v1 feature scope of 7 features, Q&A decisions, deferred scope). Commit `fa83030` — "docs: write project brief per approved plan (task 1)".
4. 2026-10-29: Task 2: the 4 remaining core project-info files created from the approved brief (this wave; plan `.kilo/plans/20261029-task2-info-implementation.md`).

## Current Project State

- Greenfield: **no source code, no `Cargo.toml`.** `src/` exists only as an empty placeholder (`.gitkeep`); no application code is present yet. The repository is docs/config scaffolding only: `.agent/`, `.kilo/`, `.opencode/`, `docs/` (per [`.agent/project-structure.md`](../project-structure.md)), plus a template `README.md` (still template until the Task 3 rewrite).
- Project Info: the complete core set is present after this task — `brief.md`, `product.md`, `context.md`, `architecture.md`, `tech.md` — plus `instructions.md` and the `.initialized` marker file (still present; its removal is handled in the Task 2 completion step).
- AGENTS.md integration is complete: all core project-info files are linked from the root [`AGENTS.md`](../../AGENTS.md) (links already added by the Planner).
- Git: branch `feat/project-brief-and-info` active; `main` not yet merged with this branch (the merge happens at workflow Step 5).

## Immediate Next Steps

1. Finish this wave: Task 3 — rewrite the root `README.md` (overview, v1 features, tech choices, greenfield status, placeholder build/run notes).
2. Workflow Step 5 (per the global plan): mark the TODO done and rename it, merge `feat/project-brief-and-info` into `main`, push to `origin` only if it is set.
3. Handle removal of the `.agent/project-info/.initialized` marker in the Task 2 completion step (flagged in the implementation plan, Section 8).
4. After this wave: **start implementation of the app skeleton** — the first coding wave of the greenfield app, per the planned stack (Rust + egui/eframe + WIC decoding; see the "planned" notes in [architecture.md](architecture.md) and [tech.md](tech.md)). Decisions marked "planned/not yet implemented" there feed this work.
