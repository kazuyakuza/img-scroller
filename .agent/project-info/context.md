# Project Context Log

## How to Use This File

Project-info rules for AI agents (full rules: [instructions.md](instructions.md)):

- **Automatic Read**: at the start of EVERY task, agents MUST automatically read this file to understand the current project state. The first response MUST open with `[Project Info: Active]` plus a confirmation phrase aligned to the current focus, e.g. "[Project Info: Active] I understand we're building `Images Scroller` (repository directory `img-scroller`). Currently working on <current focus>."
- **Critical Closing Step**: before emitting the final completion signal or stopping execution, agents MUST update this file: recent changes, then current state, then immediate next steps.
- **Context window overflow**: if the context window fills during an extended session, write the current session state into this file, then recommend the user start a fresh chat — the new session resumes from this file.
- **On-demand reads**: `architecture.md`, `tech.md`, and `product.md` are read when a task directly impacts them.
- All project-info rules live in [instructions.md](instructions.md).

## Current Work Focus

Dated 2026-10-30. The **frontend-specialist agent-definition realignment** (TODO [`.agent/todos/20261030/20261030-todo-2.md`](../todos/20261030/20261030-todo-2.md), one-task mini-plan [`.kilo/plans/20261030-frontend-specialist-realignment.md`](../../.kilo/plans/20261030-frontend-specialist-realignment.md)) is **confirmed and committed on `main`**: the agent definition is now scoped to egui/eframe immediate-mode desktop GUI in Rust in BOTH copies (`.opencode/agents/frontend-specialist.md` and `.kilo/agents/frontend-specialist.md`; frontmatter untouched; identical body edits; min plan approved, changes user-confirmed, committed, TODO renamed `-DONE`).

**SESSION PAUSED — user restarts opencode to load the new definition (config is not hot-reloaded).** The Wave-1 implementation ([`.agent/todos/20261029/20261029-todo-3.md`](../todos/20261029/20261029-todo-3.md)) was NOT started in this session (global plan approved; per-task plans auto-approved; NO 4.x cycle executed yet, NO feature branch created yet). Wave-1 continues in a NEW chat session.

Wave-1 session state snapshot (for the next session's Planner):

- Approved global plan: [`.kilo/plans/20261030-wave1-skeleton-open-image.md`](../../.kilo/plans/20261030-wave1-skeleton-open-image.md) — still in force.
- Front-end marking decision (user-updated): **Tasks 2, 3, 4 ARE front-end related** once the realigned definition is active → 4.1a (frontend-specialist, egui/eframe spec) + 4.5a (frontend-specialist, static verification; agent runs NO cargo/docker) + 4.1b/4.5b (architector) for those tasks. Task 1 (harness) and Task 5/6 non-front-end.
- Workflow start point in the next session: Step 2 (feature branch `feat/wave-1-skeleton-open-image` from `main`) → Step 3 N/A → Task 1 4.1b cycle onward. Git state at pause: on `main`, definition-alignment commit landed; working tree clean; nothing pushed beyond previous `origin` state.

## Recent Changes

1. 2026-10-30: Branch `feat/container-build-harness` created from `main`; `.opencode/opencode.json` committed with the `alpine-vm` MCP registration **as-is** (user explicitly accepted committing the `VM_SSH_PASSWORD` literal — local dev VM default credential — recorded in the global plan Addendum); planning artifacts committed (`32831fa`, `4c986bf`). Step 3 (version update) = N/A.
2. 2026-10-30: **Task 1 done** (`b2d6a5b`): all 7 wave TODO files ([`20261029-todo-3.md`](../todos/20261029/20261029-todo-3.md) … [todo-9](../todos/20261029/20261029-todo-9.md)) updated per spec `.kilo/plans/20261030-task1-wave-files-spec.md` (reviewed: all-checks PASS; adherence: CONFORMS) — Wave 1 gains the container build harness as its FIRST task (Dockerfile, docker-compose.yml, scripts/dev-checks.sh, scripts/build-windows.sh, docs/build.md, docs/build-windows.md, `.gitignore` `target/` addition, Cargo.lock/--locked policy, Alpine-VM/MCP notes; import audit retained with OBSERVED-DLL adaptation; NO FFmpeg; dist/exe + volume names + Rust pin deferred to Wave 1's architector); ALL waves' verification reworded (container dev-checks via `alpine-vm` MCP — `vm_status` first; manual UX = OPERATOR-RUN on Windows); Wave 7 gets the x64 AND x86 matrix (i686 extension of the Wave-1 script + Dockerfile when realized, consume + extend, packaging consumes `dist/` artifacts).
3. 2026-10-30: **Task 2 done** (`069ea4d`, `50b95a5`, `7173890`): `tech.md` gained "## Build Environment (Container on the Alpine VM)" (container model, `/img-scroller` share, Docker-only-on-VM, `alpine-vm` MCP usage + caveats, pin policy, `--locked` policy, gitignored `dist/`/`logs/`, OPERATOR-RUN stance, reference attribution verified 2026-10-30 with `rust:1.82` exemplar); `brief.md` Technology Stack gained the build-environment decision line (x64 + x86; x86 realized Wave 7); `architecture.md` gained "## Build and Release Pipeline (Planned)"; `README.md` Build-and-Run placeholder gained the container context (build docs to be created in Wave 1).
4. Reference pattern used throughout: `kazuyakuza/rust-youtube-streamer` README build sections + docs/build.md + docs/build-windows.md + Dockerfile + docker-compose.yml + scripts — verified consistent 2026-10-30 (user-approved source).

## Current Project State

- Greenfield: **no source code, no `Cargo.toml`.** Docs/config scaffolding + `CHANGELOG.md` (empty structure) + MIT `LICENSE` + planning wave files, all updated for the container build environment.
- The 7 wave TODO files are ready and now carry the full build/verification harness context; Wave 1 (`20261029-todo-3.md`) = 6 tasks starting with the harness, processed via the Critical Workflow in a future session on branch `feat/wave-1-skeleton-open-image`.
- Build environment facts (binding, dated 2026-10-30): all Rust/Cargo inside the Linux container on the Alpine VM (`alpine-vm` MCP; `vm_status` first; exit code authoritative); project shared at `/img-scroller`; Docker + Compose plugin ONLY on the VM; host needs NO Rust/Cargo; pin + names decided by Wave 1's architector; x64 + x86 release `.exe` cross-compiled from the container (x86 in Wave 7); GUI/runtime validation is OPERATOR-RUN on the real Windows machine.
- Git: harness update branch `feat/container-build-harness` (Step 5: merge to `main`, push `origin` only).

## Immediate Next Steps

1. **User restarts opencode** to load the realigned frontend-specialist definition (config-only change; takes effect on startup).
2. **Next session (NEW chat)**: process [`.agent/todos/20261029/20261029-todo-3.md`](../todos/20261029/20261029-todo-3.md) (Wave 1 — container harness + project skeleton + open image) via the Critical Workflow using the approved global plan `.kilo/plans/20261030-wave1-skeleton-open-image.md` (Tasks 2–4 front-end related; start at Step 2 branch creation). One wave file per session, in order (todo-3 → todo-9; MVP checkpoint at the end of Wave 7).
