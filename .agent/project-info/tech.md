# Technical Specifications

## Status

Planned stack for a greenfield project; **nothing is implemented yet** — no application code, no `Cargo.toml` (as of 2026-10-29). Facts are sourced from [brief.md](brief.md) (source of truth) and the global-plan decisions log.

## Technology Stack (Planned)

| Component | Choice (planned) | Rationale/notes |
| --- | --- | --- |
| Language | Rust | Confirmed decision in the brief; not yet implemented. |
| GUI | egui/eframe | Pure Rust, lightweight; suitable for zoom/pan/fullscreen/keyboard shortcuts. |
| Image decoding | Windows Imaging Component (WIC) system codecs | The app accepts any image format registered in the system instead of bundling its own decoders. |
| Target platform | Windows 10 and Windows 11 | Planned (not yet implemented). |
| Target architectures | x64 and x86 builds | Planned (not yet implemented). |

## Image Format Support

- The app opens any image whose format is registered as a codec in the Windows system (via WIC), e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed.
- Formats not registered in the system are **not supported in v1** — the accepted constraint of relying on system codecs.

## Dependencies (Planned/To Be Defined)

- Confirmed planned: the egui/eframe crates (GUI) — these become Cargo dependencies once the crate exists.
- Undecided: the crate/approach used to invoke WIC system codecs is **not yet decided** — deliberately no crate name is recorded here.
- No other dependencies are decided yet.

## Development Environment and Setup (Placeholder)

- Planned toolchain: Rust via the standard Rust toolchain installer (`rustup`) and `cargo`. Nothing is installed or verified yet; no toolchain requirements are recorded anywhere — this section expands during the first implementation wave.
- Never install tools globally — in-project installs only (rule: [`.kilo/rules/never-global-installs.md`](../../.kilo/rules/never-global-installs.md)).

## Build Environment (Container on the Alpine VM)

Build-environment decision recorded 2026-10-30 (source: [`.agent/todos/20261030/20261030-todo-1.md`](../todos/20261030/20261030-todo-1.md) → "Build Environment Definition"; user decisions bound in the container-harness plan). Nothing below is implemented yet — all harness files land in Wave 1.

- Container/VM model: ALL Rust/Cargo commands (builds, checks, tests) execute inside a Linux container on the Alpine VM. The host machine that runs the AI agents does NOT need Rust or Cargo installed — the Rust toolchain exists only inside the container image, so the toolchain notes in `## Development Environment and Setup (Placeholder)` above describe the toolchain shipped inside the container image, not a host install.
- Project share: the project is shared on the VM at `/img-scroller`; the container working directory is `/img-scroller`.
- Docker prerequisites: Docker with the Compose plugin is available ONLY on the VM — never on the host.
- Access pattern for AI agents: the VM is reached through the `alpine-vm` MCP — call `vm_status` first, then run commands; the command's exit code is the authoritative success evidence; cargo stderr may be absent in the MCP captured output (an absent stderr is NOT a failure signal). If the VM or the `/img-scroller` share is unavailable, STOP the task and return a question to the caller/user.
- Cargo invocations across all waves run through the container harness (`docker compose -f /img-scroller/docker-compose.yml …` usage is documented by the Wave-1 build docs — see below).
- Base-image pin policy: the exact pinned official Rust base image is decided by Wave 1's architector step — the reference project (`kazuyakuza/rust-youtube-streamer`) pinned `rust:1.82`; this is cited as the reference's exemplar ONLY, and no version is adopted or recorded here.
- Lock/artifact policy: `Cargo.lock` is tracked in git; every harness cargo invocation uses `--locked`; produced artifacts stay in the gitignored `dist/` (release `.exe`) and check logs in the gitignored `logs/`/`logs/checks/` — the repo root `.gitignore` already contains `dist/`, `logs/`, and `*.log` (Wave 1 adds `target/` defensively).
- Validation stance: validation is container-only — agent-side Windows GUI/runtime execution is impossible from the Linux container; runtime/GUI validation is OPERATOR-RUN on the real Windows 10/11 machine.
- Harness landing: all harness files (Dockerfile, `docker-compose.yml`, `scripts/dev-checks.sh`, `scripts/build-windows.sh`) and the build docs (`docs/build.md`, `docs/build-windows.md`) are created in Wave 1 (Task 1 of [`.agent/todos/20261029/20261029-todo-3.md`](../todos/20261029/20261029-todo-3.md)) — until then NO harness files exist.
- Reference pattern attribution: the pattern is adapted from the `kazuyakuza/rust-youtube-streamer` project (README build sections, `docs/build.md`, `docs/build-windows.md`, Dockerfile, `docker-compose.yml`, scripts) — verified mutually consistent on 2026-10-30, with project-specific adaptations (NO FFmpeg; own volume/artifact/naming decisions deferred to Wave 1's architector; never inherited reference names).

## Build and Run (Placeholder)

- No build system exists yet. Once `Cargo.toml` exists, the standard workflow will apply (planned): `cargo build` (debug), `cargo build --release` (release), `cargo run` (dev launch).
- x64/x86 build specifics (e.g., exact target triples) are to be defined during toolchain setup — not invented here.

## Testing (Placeholder)

- No test suite and no testing framework exist; nothing is decided yet. This section expands during implementation (unit-test quests per the code guidelines apply once code exists).

## Technical Constraints

- Windows-only application (Windows 10/11); x64 + x86 builds.
- Format support depends on the user's system codecs — unsupported formats exist by design.
- No bundled image decoders.
- The per-folder last-viewed memory is session-only, RAM-held, destroyed on close — **no persistence to disk in v1**.
- v1 sort order is fixed alphanumeric — no settings UI.
- Fullscreen/zoom keybindings and zoom step sizes are implementation details, not fixed here.

## Distribution and Packaging (Planned)

- Zip-style portable installer: the user picks the destination folder; the package unzips the application binary AND its config file there.
- Default image-viewer registration: the package includes a helper and/or guidance for registering the app as the default image viewer (file associations); the mechanism is deferred to implementation.

## Tool Usage Patterns for AI Agents

- Required reads at task start: root `AGENTS.md`, [`.agent/project-info/instructions.md`](instructions.md), [`.agent/project-info/context.md`](context.md); workflow index: [`.agent/WORKFLOWS.md`](../WORKFLOWS.md) (the Critical Workflow governs task execution).
- Tool preference: follow [`.kilo/rules/tool-selection-priority.md`](../../.kilo/rules/tool-selection-priority.md) — semantic/code-aware tools first, built-in glob/grep/read/edit/write next, shell only for CLI-native operations, single unchained commands, PowerShell last resort.
- Markdown/docs: created by the Planner Agent or Docs Specialist only (`.kilo/rules/markdown-generation-rule.md`); plans live in `.kilo/plans/`, TODOs in `.agent/todos/<YYYYMMDD>/`.
- Code location and style rules apply once code exists: all source in `src/` (`.kilo/rules/project-structure.md`), self-documenting code, method/file size limits, max 2-level nesting, private-by-default members (individual rule files in `.kilo/rules/`).

## References

- [`brief.md`](brief.md) — source of truth (requirements and scope).
- [`instructions.md`](instructions.md) — project-info rules.
- [`.agent/project-structure.md`](../project-structure.md) — structure map.
- [`.kilo/plans/20261029-project-brief.md`](../../.kilo/plans/20261029-project-brief.md) — Q&A decisions log.
