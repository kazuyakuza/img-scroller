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
