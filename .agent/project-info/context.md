# Project Context Log

## How to Use This File

Project-info rules for AI agents (full rules: [instructions.md](instructions.md)):

- **Automatic Read**: at the start of EVERY task, agents MUST automatically read this file to understand the current project state. The first response MUST open with `[Project Info: Active]` plus a confirmation phrase aligned to the current focus, e.g. "[Project Info: Active] I understand we're building `Images Scroller` (repository directory `img-scroller`). Currently working on <current focus>."
- **Critical Closing Step**: before emitting the final completion signal or stopping execution, agents MUST update this file: recent changes, then current state, then immediate next steps.
- **Context window overflow**: if the context window fills during an extended session, write the current session state into this file, then recommend the user start a fresh chat — the new session resumes from this file.
- **On-demand reads**: `architecture.md`, `tech.md`, and `product.md` are read when a task directly impacts them.
- All project-info rules live in [instructions.md](instructions.md).

## Current Work Focus

Dated 2026-10-31. The new opencode session is active with the realigned frontend-specialist definition. Wave 1 implementation is in progress on branch `feat/wave-1-skeleton-open-image` (HEAD `1498049` per caller context). **Task 4 step 4.1a (Front-end Technical Specification) is complete**: the frontend spec for the image display surface has been produced at [`.kilo/plans/20261031-task4-wave1-display-image-frontend-spec.md`](../../.kilo/plans/20261031-task4-wave1-display-image-frontend-spec.md). The spec covers the central panel as the image area, whole-image fit scaling, resize behavior, one-time texture upload, and the static checklist for 4.5a. The next step is Task 4's 4.1b (architector technical specification) followed by implementation.

## Recent Changes

1. 2026-10-31: Wave 1 implementation session on branch `feat/wave-1-skeleton-open-image`. **Task 4 step 4.1a complete**: front-end technical specification for the image display surface produced at [`.kilo/plans/20261031-task4-wave1-display-image-frontend-spec.md`](../../.kilo/plans/20261031-task4-wave1-display-image-frontend-spec.md); spec path returned to caller. Central panel = image surface; fit = whole-image visible and centered; texture = uploaded once per successful decode using straight RGBA; zoom/pan/fullscreen/navigation explicitly out of scope.
2. 2026-10-31: **Task 3 implementation landed** (caller-provided commit `0eacf61`): the open-image flow (CLI argument, in-app `Open…` dialog, WIC decode seam, bottom message bar) is present in `src/app.rs`, `src/image_loading.rs`, and `src/platform/...`. State shape matches the Task 3 frontend spec: `opened_image_path` + `message_text` + `decoded_image`.
3. 2026-10-31: Wave 1 implementation session on branch `feat/wave-1-skeleton-open-image`. **Task 3 step 4.1a complete**: front-end technical specification for the open-image flow and in-window message surface produced at [`.kilo/plans/20261031-task3-wave1-open-image-frontend-spec.md`](../../.kilo/plans/20261031-task3-wave1-open-image-frontend-spec.md); spec path returned to caller.
4. 2026-10-31: Wave 1 implementation session active on branch `feat/wave-1-skeleton-open-image`. **Task 2 step 4.1a complete**: front-end technical specification for the egui/eframe app shell produced at [`.kilo/plans/20261031-task2-wave1-app-shell-frontend-spec.md`](../../.kilo/plans/20261031-task2-wave1-app-shell-frontend-spec.md); spec path returned to caller. No application code written; no Cargo commands run.
5. 2026-10-30: Branch `feat/container-build-harness` created from `main`; `.opencode/opencode.json` committed with the `VM_SSH_PASSWORD` literal — local dev VM default credential — recorded in the global plan Addendum); planning artifacts committed (`32831fa`, `4c986bf`). Step 3 (version update) = N/A.
6. 2026-10-30: **Task 1 done** (`b2d6a5b`): all 7 wave TODO files ([`20261029-todo-3.md`](../todos/20261029/20261029-todo-3.md) … [todo-9](../todos/20261029/20261029-todo-9.md)) updated per spec `.kilo/plans/20261030-task1-wave-files-spec.md` (reviewed: all-checks PASS; adherence: CONFORMS) — Wave 1 gains the container build harness as its FIRST task (Dockerfile, docker-compose.yml, scripts/dev-checks.sh, scripts/build-windows.sh, docs/build.md, docs/build-windows.md, `.gitignore` `target/` addition, Cargo.lock/--locked policy, Alpine-VM/MCP notes; import audit retained with OBSERVED-DLL adaptation; NO FFmpeg; dist/exe + volume names + Rust pin deferred to Wave 1's architector); ALL waves' verification reworded (container dev-checks via `alpine-vm` MCP — `vm_status` first; manual UX = OPERATOR-RUN on Windows); Wave 7 gets the x64 AND x86 matrix (i686 extension of the Wave-1 script + Dockerfile when realized, consume + extend, packaging consumes `dist/` artifacts).
7. 2026-10-30: **Task 2 done** (`069ea4d`, `50b95a5`, `7173890`): `tech.md` gained "## Build Environment (Container on the Alpine VM)" (container model, `/img-scroller` share, Docker-only-on-VM, `alpine-vm` MCP usage + caveats, pin policy, `--locked` policy, gitignored `dist/`/`logs/`, OPERATOR-RUN stance, reference attribution verified 2026-10-30 with `rust:1.82` exemplar); `brief.md` Technology Stack gained the build-environment decision line (x64 + x86; x86 realized Wave 7); `architecture.md` gained "## Build and Release Pipeline (Planned)"; `README.md` Build-and-Run placeholder gained the container context (build docs to be created in Wave 1).
8. 2026-10-30: Reference pattern used throughout: `kazuyakuza/rust-youtube-streamer` README build sections + docs/build.md + docs/build-windows.md + Dockerfile + docker-compose.yml + scripts — verified consistent 2026-10-30 (user-approved source).

## Current Project State

- Wave 1 implementation is in progress on branch `feat/wave-1-skeleton-open-image` (HEAD `1498049` per caller context). The Cargo project scaffold, window shell, and open-image flow are in place (`Cargo.toml`, `src/main.rs`, `src/app.rs`, `src/image_loading.rs`, `src/platform/...`, tracked `Cargo.lock`). No image texture or fit-scaling code exists yet.
- **Task 3 implementation is complete** (caller-provided commit `0eacf61`); **Task 4 step 4.1a (frontend spec) is complete**. Task 4 step 4.1b (architector spec) is the next pending step before implementation.
- The 7 wave TODO files are ready and carry the full build/verification harness context; Wave 1 (`20261029-todo-3.md`) = 6 tasks processed via the Critical Workflow.
- Build environment facts (binding, dated 2026-10-30): all Rust/Cargo inside the Linux container on the Alpine VM (`alpine-vm` MCP; `vm_status` first; exit code authoritative); project shared at `/img-scroller`; Docker + Compose plugin ONLY on the VM; host needs NO Rust/Cargo; pin + names decided by Wave 1's architector; x64 + x86 release `.exe` cross-compiled from the container (x86 in Wave 7); GUI/runtime validation is OPERATOR-RUN on the real Windows machine.
- Git: the definition-realignment work is committed on `main`; the Wave 1 feature branch is `feat/wave-1-skeleton-open-image`.

## Immediate Next Steps

1. **Task 4 step 4.1b**: architector produces the technical specification for the image display surface (texture-handle ownership pattern, fit-scaling helper module signature, exact egui paint call, tests).
2. **Task 4 implementation**: implement central-panel image rendering, whole-image fit scaling, and one-time texture upload per the frontend spec and architector spec.
3. Continue processing [`.agent/todos/20261029/20261029-todo-3.md`](../todos/20261029/20261029-todo-3.md) through the Critical Workflow (Task 5 verification follows Task 4 implementation).
