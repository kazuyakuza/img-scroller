# Implementation Plan — Wave 1, Task 6: Documentation updates (Critical Workflow step 4.1b)

- Date: 2026-10-31
- Author: Architector (step 4.1b, Analysis & Planning) — deliverable of THIS step is this plan file only
- TODO source (TOP PRIORITY): [`.agent/todos/20261029/20261029-todo-3.md`](../../.agent/todos/20261029/20261029-todo-3.md) → `### Task 6: Documentation updates` (4 sub-items)
- Global plan: [`.kilo/plans/20261030-wave1-skeleton-open-image.md`](20261030-wave1-skeleton-open-image.md) §2 → "Task 6" row
- Task classification: **NOT front-end related** → no 4.1a / no 4.5a; 4.5b verifies against this plan
- Plan approval: AUTO-APPROVED ("Approve Global and Tasks Plans", 2026-10-30) → save + return path; no user approval question
- Repo state verified 2026-10-31: branch `feat/wave-1-skeleton-open-image` @ `b2645f4`, working tree clean (Tasks 1–5 landed and marked `[DONE]`)
- Executor of this plan: implementer (JUNIOR, 50% restriction) in step 4.2 — zero judgment calls required; every edit below is verbatim
- NOT part of this task: `.agent/project-info/context.md` (global plan Task 6 row: handled by the PLANNER as the session-closing step), `CHANGELOG.md` (wave note: do NOT touch), `.agent/project-structure.md` (accurate already), TODO checkbox marks (step 4.6), any push/branch op (Step 5 only)

## 0. Scope

Files this plan changes (exactly three):

| File | Change kind |
| --- | --- |
| `README.md` | 3 anchored section edits (Edit tool) |
| `.agent/project-info/tech.md` | full-file rewrite (Write tool) |
| `.agent/project-info/architecture.md` | full-file rewrite (Write tool) |

Files that must remain byte-identical: everything else, including `docs/build.md`, `docs/build-windows.md`, `scripts/*.sh`, `src/**`, `Cargo.toml`, `Cargo.lock`, `.gitignore`, `.agent/project-info/brief.md`, `.agent/project-info/product.md`, `.agent/project-info/context.md`, `CHANGELOG.md`, `.agent/todos/**`, `.kilo/plans/*` (existing plans are never edited).

No commands run on the Alpine VM, no cargo, no docker in this task: every fact below was verified by reading the repository files (verification table in §4).

## 1. Decisions (encoded — no implementer judgment)

1. **OBSERVED DLL list placement**: tech.md records the 22-entry OBSERVED DLL import list by POINTER only (to `scripts/build-windows.sh` → `OBSERVED_DLL_IMPORT_LIST` constant, and to `docs/build-windows.md` → "Static-linking / import audit (retained scope)"). The list is NOT duplicated inline in tech.md or in README (docs-split rule; verified: the constant exists at `scripts/build-windows.sh` line 15 and the audited list at `docs/build-windows.md` lines 58–62).
2. **README stale-fact corrections beyond the build/run section (DECIDED — flagged to caller)**: the TODO lists only the build/run placeholder for README, but two more README statements are invalidated by Wave 1 and would directly contradict the new Build and Run section: the "## Technology Stack (Planned)" section (greenfield/"not yet implemented") and the "## Project Status" section ("no Cargo.toml and no source code"). This plan corrects exactly those two sections with the verbatim drafts in §3.1, preserving section order, heading style, table shape, and every other line of README. Section structure is preserved; no renumbering/restyling. If the caller vetoes, the fallback is to drop Edits 1–2 and keep only Edit 3 — but the plan encodes the corrections as mandatory.
3. **TOC rule**: final README = 89 lines (< 100) → NO TOC. Final tech.md = 121 lines (> 100) → a `## Contents` index is included in the verbatim draft. Final architecture.md = 118 lines (> 100) → a `## Contents` index is included. The Contents blocks are unconditional parts of the drafts (no conditional logic for the implementer).
4. **tech.md Build Environment section**: PRESERVED as a section with its existing bullets; only the invalidated statements are adjusted (lead-in line, base-image-pin bullet, harness-landing bullet, docs pointer in the cargo-invocations bullet) and exactly two landed bullets are appended (named volume, import audit). See §3.2.
5. **architecture.md structure**: overall shape preserved (same section order); the four `(Planned)` heading suffixes invalidated by this wave are dropped (`Logical Components`, `Navigation Critical Paths`, `Data and State`, `Build and Release Pipeline`); two new sections are added (`## Module Layout (Implemented, Wave 1)`, `## Open-Image and Display Flow (Implemented)`); `## Open Architectural Decisions` keeps only truly-undecided items and adds the navigation-engine item (the caller's "folder navigation" open item); the decided items "Internal src/ module structure" and "WIC access mechanism" are REMOVED (landed).
6. **WIC mechanism recording**: a dedicated tech.md section (`## WIC Decode and Open-Dialog Mechanism (Implemented)`) carries the full chain; architecture.md and README reference it instead of duplicating it.
7. **Commit**: exactly ONE commit after all three files land: message `Document Wave 1 build and verification state` (matches repo style: imperative, no prefix; caller-suggested). No push, no branch ops.
8. **Testing facts wording**: tech.md states "15 unit tests, all container-runnable", with the per-file breakdown (app.rs 4, image_loading.rs 4, app_display.rs 2, scaling.rs 5 — verified in the source files), the pure-logic-only scope, the OS-bound-untested-by-design rule, and the operator-run record. No other test claims.

## 2. Step S1 — Preconditions (STOP gates)

Run these reads BEFORE any edit. All anchors were verified on 2026-10-31 @ `b2645f4`; if ANY check fails, STOP and report to the caller — do NOT adapt, do NOT edit.

1. `git status` → branch is `feat/wave-1-skeleton-open-image`, working tree clean. If not clean or wrong branch → STOP.
2. Read `README.md` and verify these EXACT strings exist:
   - `## Technology Stack (Planned)` (heading)
   - `The choices below are confirmed decisions but not yet implemented — the project is greenfield, with no code.`
   - `## Project Status`
   - `This project is greenfield and in the planning phase: the intended v1 product is fully defined, but no application code exists yet (as of 2026-10-29) — there is no \`Cargo.toml\` and no source code.`
   - `## Build and Run (Not Yet Implemented)`
   Any drift → STOP.
3. Read `.agent/project-info/tech.md` and verify these EXACT strings exist:
   - `Planned stack for a greenfield project; **nothing is implemented yet**` (Status line)
   - `## Development Environment and Setup (Placeholder)`
   - `## Build and Run (Placeholder)`
   - `## Testing (Placeholder)`
   - `Nothing below is implemented yet — all harness files land in Wave 1.` (Build Environment lead-in)
   Any drift → STOP.
4. Read `.agent/project-info/architecture.md` and verify these EXACT strings exist:
   - `**Nothing is implemented yet** — no application code, no \`Cargo.toml\` (as of 2026-10-29).` (Status)
   - `The concrete module/file layout inside \`src/\` is undecided` (Logical Components intro)
   - `the pipeline exists only as a plan — no pipeline is implemented yet — and its files land in Wave 1` (Build and Release Pipeline lead-in)
   - `**NOT YET DECIDED**` (Project Paths module-layout line)
   Any drift → STOP.
5. Confirm the target plan file does not exist yet (it does not: verified 2026-10-31).

## 3. Step S2 — File changes (verbatim)

Tool notes: use the built-in `edit` tool for README (after a fresh Read) and the built-in `write` tool for tech.md and architecture.md (after a fresh Read — the Write tool requires a prior Read of an existing file). All content uses REAL newline characters (Newline Prevention Rule) — never literal `\n` sequences.

### 3.1 `README.md` — three anchored edits

Everything outside the three oldStrings below stays byte-identical (lines 1–32, 47–52, 63–74 of the current 74-line file). Expected final length: 89 lines (< 100 → no TOC).

**Edit A — Technology Stack section** (replaces the whole block from the `## Technology Stack (Planned)` heading through the `- Formats are supported…` line):

oldString:

```markdown
## Technology Stack (Planned)

The choices below are confirmed decisions but not yet implemented — the project is greenfield, with no code.

| Component | Choice (planned) | Notes |
| --- | --- | --- |
| Language | Rust | Confirmed decision in the brief; not yet implemented. |
| GUI | egui/eframe | Pure Rust, lightweight; suitable for zoom/pan/fullscreen/keyboard shortcuts. |
| Image decoding | Windows Imaging Component (WIC) system codecs | The app accepts any image format registered in the system instead of bundling its own decoders. |
| Target platform | Windows 10 and Windows 11 | Planned (not yet implemented). |
| Target architectures | x64 and x86 builds | Planned (not yet implemented). |

- Formats are supported when a system codec exists; formats not registered in the system are not supported in v1 — the accepted constraint of relying on system codecs.
```

newString:

```markdown
## Technology Stack

The choices below are confirmed and implemented for the Wave-1 open-image slice; the remaining v1 features (navigation, fullscreen, zoom, "Open with...") are still open — see [Project Status](#project-status) and [.agent/project-info/architecture.md](.agent/project-info/architecture.md).

| Component | Choice | Notes |
| --- | --- | --- |
| Language | Rust | Implemented: Cargo package `img-scroller` 0.1.0, edition 2024. |
| GUI | egui/eframe | Implemented: `eframe` =0.37.0 (glow renderer); pure Rust, lightweight. |
| Image decoding | Windows Imaging Component (WIC) system codecs | Implemented via the `windows` crate =0.62.2 (Windows-target-scoped); no bundled decoders. |
| Target platform | Windows 10 and Windows 11 | Runtime/GUI validation is operator-run on a real Windows machine. |
| Target architectures | x64 and x86 builds | x64 cross-build landed (Wave 1); x86 belongs to Wave 7. |

- Formats are supported when a system codec exists; formats not registered in the system are not supported in v1 — the accepted constraint of relying on system codecs.
```

**Edit B — Project Status section**:

oldString:

```markdown
## Project Status

This project is greenfield and in the planning phase: the intended v1 product is fully defined, but no application code exists yet (as of 2026-10-29) — there is no `Cargo.toml` and no source code. The full requirements, decisions, and scope boundaries live in the project brief: [.agent/project-info/brief.md](.agent/project-info/brief.md).
```

newString:

```markdown
## Project Status

Wave 1 landed (2026-10-31, branch `feat/wave-1-skeleton-open-image`): the Cargo project exists, and the viewer opens images via a CLI argument or the in-app open dialog, decodes them through the WIC system codecs, and displays them fit-scaled in the regular resizable window. Verification so far: container dev checks (`ALL CHECKS PASSED`), Windows x64 release cross-build (exit 0), import audit; running the `.exe` on Windows is operator-run. The remaining v1 features (horizontal/vertical navigation, fullscreen, zoom, "Open with...") are open. The full requirements, decisions, and scope boundaries live in the project brief: [.agent/project-info/brief.md](.agent/project-info/brief.md).
```

**Edit C — Build and Run section** (the TODO's core README item: one-liner, constraint, harness commands + pointer, cross-build + pointer):

oldString:

```markdown
## Build and Run (Not Yet Implemented)

This section is a placeholder: nothing is implemented yet and no build system exists, so there is nothing to build or run at the moment. Once the first implementation wave creates the Rust crate, a standard Cargo workflow will apply once `Cargo.toml` exists, and the steps will be documented here and in [.agent/project-info/tech.md](.agent/project-info/tech.md). No runnable commands are provided until then.

One build decision is already recorded (2026-10-30): all cargo/build commands will run INSIDE a Linux container on the Alpine VM (accessed through the `alpine-vm` MCP) — the host machine needs no Rust or Cargo installed. The dedicated build documentation (dev-checks through the container and the Windows `.exe` cross-build) arrives with the first implementation wave: `docs/build.md` and `docs/build-windows.md` (both to be created in Wave 1).
```

newString:

```markdown
## Build and Run

Images Scroller is a Windows desktop image viewer built in Rust with egui/eframe; image decoding uses the Windows Imaging Component (WIC) system codecs. All Rust/Cargo commands run inside a Linux Docker container on the Alpine VM — the host machine needs no Rust or Cargo installed.

v1 format constraint: the app opens any image whose format is registered as a codec in the Windows system (via WIC — e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed); formats not registered in the system are not supported in v1.

### Container dev checks (no host Rust/Cargo)

Build the container image once, then run the dev-check harness:

    docker compose -f /img-scroller/docker-compose.yml build rust
    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh

The harness runs `cargo fmt --check`, `cargo check --locked`, `cargo test --locked`, and `cargo clippy --locked -- -D warnings`, then prints `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1). Full semantics — per-check fallback commands, the `FORCE_FAIL` fault-injection hook, the UTC log under `logs/checks/`, and the `alpine-vm` MCP usage notes — are documented in [docs/build.md](docs/build.md); this README does not duplicate them.

### Windows x64 release cross-build

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh

A successful run leaves the release executable at `dist/windows-x64/img-scroller.exe` (the `dist/` tree is gitignored). Running the produced `.exe` — GUI behavior and image-format handling — is validated by the operator on a real Windows 10/11 machine; the Linux container cannot run the Windows GUI. Typed failures, overwrite/rebuild behavior, toolchain notes, and the static-linking import audit are documented in [docs/build-windows.md](docs/build-windows.md).
```

Final README shape check (S3): section order = Overview → Problem Statement → v1 Features → Technology Stack → Distribution (Planned) → Project Status → Build and Run (with the two `###` subsections) → Note for AI Agents → License; 89 lines; no TOC.

### 3.2 `.agent/project-info/tech.md` — full rewrite (Write tool)

Replace the ENTIRE file with EXACTLY the content below (121 lines; the `## Contents` index is mandatory per Decision 3):

````markdown
# Technical Specifications

## Status

Landed for Wave 1 (branch `feat/wave-1-skeleton-open-image`, 2026-10-31): the Cargo project, the egui/eframe window shell, the open-image flow (CLI argument + native open dialog + WIC decode), and the fit-scaled image display are implemented and verified through the container harness (dev checks: `ALL CHECKS PASSED`; Windows x64 release cross-build: exit 0 → `dist/windows-x64/img-scroller.exe`). The remaining v1 features (navigation, fullscreen, zoom, "Open with...", packaging) are planned. Facts are sourced from [brief.md](brief.md) (source of truth), the landed code, and the build guides ([docs/build.md](../../docs/build.md), [docs/build-windows.md](../../docs/build-windows.md)).

## Contents

1. [Technology Stack](#technology-stack)
2. [Image Format Support](#image-format-support)
3. [Dependencies (Landed, Wave 1)](#dependencies-landed-wave-1)
4. [WIC Decode and Open-Dialog Mechanism (Implemented)](#wic-decode-and-open-dialog-mechanism-implemented)
5. [Development Environment and Setup](#development-environment-and-setup)
6. [Build Environment (Container on the Alpine VM)](#build-environment-container-on-the-alpine-vm)
7. [Build and Run](#build-and-run)
8. [Testing](#testing)
9. [Technical Constraints](#technical-constraints)
10. [Distribution and Packaging (Planned)](#distribution-and-packaging-planned)
11. [Tool Usage Patterns for AI Agents](#tool-usage-patterns-for-ai-agents)
12. [References](#references)

## Technology Stack

| Component | Choice | Rationale/notes |
| --- | --- | --- |
| Language | Rust, edition 2024 | Cargo package `img-scroller` version 0.1.0 (single binary; tracked `Cargo.lock`). |
| GUI | egui/eframe | `eframe` =0.37.0 (`default-features = false`), glow renderer feature; pure Rust, lightweight. |
| Image decoding | Windows Imaging Component (WIC) system codecs | Accessed through the `windows` crate =0.62.2 (mechanism: `## WIC Decode and Open-Dialog Mechanism (Implemented)`); no bundled decoders. |
| Target platform | Windows 10 and Windows 11 | Window shell + open flow implemented; runtime/GUI validation is operator-run. |
| Target architectures | x64 and x86 builds | x64 (`x86_64-pc-windows-gnu`) cross-build landed in Wave 1; x86 belongs to Wave 7. |
| Build environment | Linux Docker container on the Alpine VM | Host needs no Rust/Cargo; details in `## Build Environment (Container on the Alpine VM)`. |

## Image Format Support

- The app opens any image whose format is registered as a codec in the Windows system (via WIC), e.g. jpg, png, bmp, gif, webp, and heic when a system codec is installed.
- Formats not registered in the system are **not supported in v1** — the accepted constraint of relying on system codecs.

## Dependencies (Landed, Wave 1)

Landed in `Cargo.toml` (every pin is exact `=`; every harness cargo invocation runs with `--locked`):

- GUI stack: `eframe` `=0.37.0` with `default-features = false` and exactly six features — `glow`, `default_fonts`, `clipboard`, `links`, `wayland`, `x11`; `glow` is the chosen renderer backend decision (default features are disabled so the feature set stays explicitly pinned).
- Window-handle access: `raw-window-handle` `=0.6.2` (resolves the native HWND that owns the open dialog).
- Windows-target-scoped (under `[target.'cfg(windows)'.dependencies]`): `windows` `=0.62.2` with exactly six features — `Win32_Foundation`, `Win32_Graphics_Imaging`, `Win32_System_Com`, `Win32_UI_Shell`, `Win32_UI_Shell_Common`, `Win32_UI_WindowsAndMessaging`.
- No other dependencies; the `image` crate is deliberately NOT used (no-bundled-decoders constraint).

## WIC Decode and Open-Dialog Mechanism (Implemented)

Implemented in `src/platform/windows.rs` via the windows-rs COM projection (`windows` crate =0.62.2), cfg-gated behind the `platform` facade (non-Windows builds compile `src/platform/stub.rs`):

- COM initialization: `CoInitializeEx(None, COINIT_APARTMENTTHREADED)`; `S_OK`, `S_FALSE`, and `RPC_E_CHANGED_MODE` all count as initialized.
- Open dialog: `CoCreateInstance(&FileOpenDialog)` → `IFileOpenDialog` with options `FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST`; `Show` receives the owner HWND resolved from the eframe frame through `raw-window-handle` (Win32 handle); the result is read via `GetResult` → `IShellItem::GetDisplayName(SIGDN_FILESYSPATH)` and freed with `CoTaskMemFree`.
- WIC decode chain: `CoCreateInstance(&CLSID_WICImagingFactory)` → `IWICImagingFactory::CreateDecoderFromFilename` (`GENERIC_READ`, `WICDecodeMetadataCacheOnDemand`) → `GetFrame(0)` → `GetSize` → `CreateFormatConverter` → `Initialize` to `GUID_WICPixelFormat32bppRGBA` (dither `WICBitmapDitherTypeNone`, palette `WICBitmapPaletteTypeCustom`) → `CopyPixels` into an RGBA8 byte buffer.
- Failure classification: `WINCODEC_ERR_COMPONENTNOTFOUND` → `Unsupported`; every other error → `Unloadable`; both surface the single unsupported-file message (constant in `src/image_loading.rs`) in the window's bottom message bar.

## Development Environment and Setup

- The Rust toolchain (stable, with rustfmt and clippy) exists ONLY inside the container image `img-scroller-rust:latest` (pinned base `rust:1.99.1-bookworm`) — nothing is installed on the host; the host needs no Rust/Cargo. Never install tools globally — in-project/container installs only (rule: [`.kilo/rules/never-global-installs.md`](../../.kilo/rules/never-global-installs.md)).
- The image also carries the MinGW-w64 x86-64 GNU cross toolchain (linker `x86_64-w64-mingw32-gcc`; `x86_64-w64-mingw32-objdump` backs the Windows import audit) and the rustup target `x86_64-pc-windows-gnu`.
- Agents reach the VM through the `alpine-vm` MCP: call `vm_status` first, then run commands; the command's exit code is the authoritative evidence; cargo stderr may be absent in captured MCP output (an absent stderr is NOT a failure signal). Full usage notes: [docs/build.md](../../docs/build.md).

## Build Environment (Container on the Alpine VM)

Build-environment decision recorded 2026-10-30 (source: [`.agent/todos/20261030/20261030-todo-1.md`](../todos/20261030/20261030-todo-1.md) → "Build Environment Definition"; user decisions bound in the container-harness plan). **Landed in Wave 1 Task 1 (2026-10-31)**: every harness file and both build guides exist and are exercised.

- Container/VM model: ALL Rust/Cargo commands (builds, checks, tests) execute inside a Linux container on the Alpine VM. The host machine that runs the AI agents does NOT need Rust or Cargo installed — the Rust toolchain exists only inside the container image, so the toolchain notes in `## Development Environment and Setup` above describe the toolchain shipped inside the container image, not a host install.
- Project share: the project is shared on the VM at `/img-scroller`; the container working directory is `/img-scroller`.
- Docker prerequisites: Docker with the Compose plugin is available ONLY on the VM — never on the host.
- Access pattern for AI agents: the VM is reached through the `alpine-vm` MCP — call `vm_status` first, then run commands; the command's exit code is the authoritative success evidence; cargo stderr may be absent in the MCP captured output (an absent stderr is NOT a failure signal). If the VM or the `/img-scroller` share is unavailable, STOP the task and return a question to the caller/user.
- Cargo invocations across all waves run through the container harness (`docker compose -f /img-scroller/docker-compose.yml …`); usage is documented in [docs/build.md](../../docs/build.md).
- Base-image pin (landed): official `rust:1.99.1-bookworm` → image name `img-scroller-rust:latest`; rebuild only after a Dockerfile change; toolchain upgrades are plan-gated.
- Named volume (landed): `img-scroller-cargo-target` mounted at `/cargo-target` via `CARGO_TARGET_DIR=/cargo-target` — intermediate Cargo output never lands in the repository; the only tracked root-level build artifact is `Cargo.lock`.
- Lock/artifact policy: `Cargo.lock` is tracked in git; every harness cargo invocation uses `--locked`; produced artifacts stay in the gitignored `dist/` (release `.exe`) and check logs in the gitignored `logs/`/`logs/checks/` — the repo root `.gitignore` contains `dist/`, `logs/`, `*.log`, and `target/`.
- Validation stance: validation is container-only — agent-side Windows GUI/runtime execution is impossible from the Linux container; runtime/GUI validation is OPERATOR-RUN on the real Windows 10/11 machine.
- Harness landing (DONE, Wave 1 Task 1): `Dockerfile`, `docker-compose.yml`, `scripts/dev-checks.sh`, `scripts/build-windows.sh`, `docs/build.md`, `docs/build-windows.md` all exist and are exercised (Task 5: `ALL CHECKS PASSED`; cross-build exit 0 → `dist/windows-x64/img-scroller.exe`, 19,428,809 bytes).
- Import audit (DONE, Wave 1 Task 5): the OBSERVED 22-DLL import list of the produced exe is recorded in `scripts/build-windows.sh` (`OBSERVED_DLL_IMPORT_LIST` constant) and in [docs/build-windows.md](../../docs/build-windows.md) → "Static-linking / import audit (retained scope)"; this file deliberately does NOT duplicate the list (docs-split rule). Audited 2026-10-31 with `x86_64-w64-mingw32-objdump`.
- Reference pattern attribution: the pattern is adapted from the `kazuyakuza/rust-youtube-streamer` project (README build sections, `docs/build.md`, `docs/build-windows.md`, Dockerfile, `docker-compose.yml`, scripts) — verified mutually consistent on 2026-10-30, with project-specific adaptations (NO FFmpeg; own volume/artifact/naming decisions; never inherited reference names).

## Build and Run

- Dev checks: build the image once (`docker compose -f /img-scroller/docker-compose.yml build rust`), then run `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — fmt/check/test/clippy in fixed order, final `ALL CHECKS PASSED` (exit 0) / `N CHECK(S) FAILED` (exit 1); full semantics (per-check fallbacks, `FORCE_FAIL`, log path, MCP notes): [docs/build.md](../../docs/build.md).
- Windows x64 release: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` — runs `cargo build --release --locked --target x86_64-pc-windows-gnu` and leaves a non-empty `dist/windows-x64/img-scroller.exe`; full guide (typed failures, overwrite behavior, audit): [docs/build-windows.md](../../docs/build-windows.md).
- Dev launch: `cargo run` works inside the container for the compile path, but the container has no display server — the GUI window only runs on a Windows machine (operator-run validation).
- x86 release: NOT built in Wave 1 — belongs to Wave 7 (extend `scripts/build-windows.sh` + add the i686 MinGW toolchain when realized).

## Testing

- 15 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3): `src/app.rs` — 4 (window-title constant; failed CLI open keeps no path and shows the message; successful open stores path/payload and clears the message; failure after success keeps the previous image); `src/image_loading.rs` — 4 (message wording; per-failure message mapping; pixel-buffer sizing; empty/overflow rejection); `src/app_display.rs` — 2 (initial texture state; upload happens exactly once per decode); `src/scaling.rs` — 5 (fit vectors ×4; zero-dimension rejection).
- Scope: pure logic only. OS-bound WIC/dialog code (`src/platform/windows.rs`) is untested by design — it cannot execute in the Linux container; the facade's non-Windows `stub.rs` (dialog → `None`, decode → `Unloadable`) keeps container builds/tests compiling and keeps the app-level failure transition testable.
- Windows runtime/GUI behavior (native dialog, real decode, real display) is OPERATOR-RUN on a real Windows 10/11 machine — recorded as operator-pending, never agent-failed.

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
- [`docs/build.md`](../../docs/build.md) — container dev-checks guide (Alpine VM).
- [`docs/build-windows.md`](../../docs/build-windows.md) — Windows x64 cross-build guide + OBSERVED DLL import list.
- [`.kilo/plans/20261030-wave1-skeleton-open-image.md`](../../.kilo/plans/20261030-wave1-skeleton-open-image.md) — Wave 1 global plan.
````

Unchanged-section guarantee: `## Image Format Support`, `## Technical Constraints`, `## Distribution and Packaging (Planned)`, and `## Tool Usage Patterns for AI Agents` above reproduce the current file verbatim; the `## References` section keeps its four existing entries and appends exactly three.

### 3.3 `.agent/project-info/architecture.md` — full rewrite (Write tool)

Replace the ENTIRE file with EXACTLY the content below (118 lines; `## Contents` index mandatory):

````markdown
# System Architecture

## Status

The planned v1 architecture below is partially **implemented** (Wave 1, branch `feat/wave-1-skeleton-open-image`, 2026-10-31): the window shell, the open-image flow (CLI argument + native dialog + WIC decode), and the fit-scaled display exist in `src/` and are verified through the container harness. Navigation, fullscreen, zoom, and "Open with..." remain planned. Facts are sourced from [brief.md](brief.md) (source of truth), the landed code, and [tech.md](tech.md).

## Contents

1. [Overview](#overview)
2. [Logical Components](#logical-components)
3. [Module Layout (Implemented, Wave 1)](#module-layout-implemented-wave-1)
4. [Open-Image and Display Flow (Implemented)](#open-image-and-display-flow-implemented)
5. [Navigation Critical Paths](#navigation-critical-paths)
6. [Data and State](#data-and-state)
7. [Build and Release Pipeline](#build-and-release-pipeline)
8. [Project Paths](#project-paths)
9. [Design Patterns](#design-patterns)
10. [Technical Decisions Summary](#technical-decisions-summary)
11. [Open Architectural Decisions](#open-architectural-decisions)

## Overview

- Shape: a single-purpose Windows desktop GUI application — an image viewer with two-axis folder navigation. No server, no back-end. It ships as a single application binary plus a config file (distribution flow: see `## Data and State` below and [tech.md](tech.md)).
- Three pillars chosen in the brief: egui/eframe GUI, WIC system-codec decoding, and reliance on Windows system registration data (codecs and the "Open with..." program list). Wave 1 landed pillars 1–2 as the open-image slice; pillar 3 ("Open with...") is planned.

## Logical Components

The decomposition below is derived from the v1 features; each component carries a landed/planned annotation. The realized module layout is in `## Module Layout (Implemented, Wave 1)`.

- **Viewer UI (egui/eframe, immediate-mode) — Wave-1 slice LANDED**: regular resizable window (title `Images Scroller`, not maximized at launch); top "Open…" toolbar button; bottom message bar; central image surface painted fit-scaled and centered. Remaining UI (navigation buttons, fullscreen toggle, zoom controls, "Open with..." menu entry): planned.
- **Image Decoding Layer (WIC) — LANDED**: decodes any image format registered as a Windows system codec through the windows-rs COM projection (mechanism: [tech.md](tech.md) → `## WIC Decode and Open-Dialog Mechanism (Implemented)`); no bundled decoders; formats without a system codec are unsupported in v1 (accepted constraint).
- **Navigation Engine — planned**: horizontal — previous/next sibling image in the current folder (alphanumeric order); vertical — previous/next sibling folder under the SAME parent (target image priority: (a) session last-viewed image for folders already visited this session, (b) otherwise ALWAYS the target folder's first image in alphanumeric order).
- **Session Last-Viewed Memory — planned**: RAM-only map of the last viewed image per visited folder; destroyed on app close; no disk persistence in v1.
- **Pan/Zoom Controller — planned**: zoom in/out, fit width/fit height, and pan-then-navigate — width-overflowed image: left/right keys pan until the image's horizontal edge, then trigger horizontal navigation; height-overflowed image: up/down keys pan until the vertical edge, then trigger folder navigation; no overflow: arrows navigate directly.
- **Fast "Open with..." Service — planned**: at runtime queries the Windows system for programs registered for the current file's type/extension; no internal program database.
- **Distribution Packaging — planned**: zip-style installer (destination-folder selection; unzips the app binary and config file) plus default-viewer registration helper/guidance — the helper mechanism is deferred.

## Module Layout (Implemented, Wave 1)

All source lives in `src/` (folder map: [`.agent/project-structure.md`](../project-structure.md)); files stay under the 200-line rule.

- `src/main.rs` — entry point: takes the first CLI argument as an optional image path (`std::env::args_os().nth(1)`), starts the eframe native app with the `Images Scroller` title/window options, and opens the CLI image before the first frame when present.
- `src/app.rs` — application shell: `ImagesScrollerApp` state (opened path, decoded payload, message text, display), the `open_image` flow (decode → state transition; failure changes only the message), the top "Open…" button wiring, the `APP_WINDOW_TITLE` constant, and `NativeOptions` (resizable, not maximized).
- `src/image_loading.rs` — open-image seam: `load_image` (delegates to the platform facade), `DecodedImage` payload (width, height, RGBA8 `pixels`), `ImageLoadFailure` (`Unsupported` / `Unloadable`), the `pixel_buffer_size` guard, and the unsupported-file message constant.
- `src/platform/` — OS-bound facade behind cfg-gating: `mod.rs` re-exports `decode_image` and `show_open_image_dialog`; `windows.rs` implements the native `IFileOpenDialog` and the WIC decode (mechanism: [tech.md](tech.md)); `stub.rs` is the non-Windows fallback (dialog → `None`, decode → `Unloadable`) that keeps container checks/tests compiling.
- `src/scaling.rs` — pure fit-scaling math: `fitted_size` returns the largest uniform size that keeps the image inside the available area (aspect preserved; zero/negative dimensions rejected); no egui painting.
- `src/app_display.rs` — central display: `CurrentImageDisplay` texture wrapper — one texture upload per successful decode (pending-upload flag; the dropped old handle frees the previous texture), fit-scaled centered paint via `egui::Image::fit_to_exact_size`.

## Open-Image and Display Flow (Implemented)

1. Launch: CLI argument present → `open_image(path)` runs before the first frame; absent → the user clicks "Open…" → native `IFileOpenDialog` (owner HWND via `raw-window-handle`).
2. Decode: `platform::decode_image` → WIC factory → decoder from filename → frame → format-convert to 32bppRGBA → `CopyPixels` (mechanism detail: [tech.md](tech.md)).
3. Success: state stores path + `DecodedImage`, clears the message, and flags a pending texture upload; the next frame's central-panel paint uploads exactly once and paints the image fit-scaled and centered (per-frame fit recomputation via `scaling::fitted_size`, so window resizes keep the display sane).
4. Failure: `Unsupported`/`Unloadable` → the bottom message bar shows the unsupported-file message; a previously loaded image and the rest of the state stay untouched.

## Navigation Critical Paths

Path 1's open/decode/display slice is implemented (Wave 1); the navigation, pan/zoom, and "Open with..." parts of all paths remain planned.

1. **Entry**: open an image (Explorer association, file manager, or in-app open dialog) → decode via a WIC system codec → display in the regular resizable window → navigation enabled immediately; no folder pre-registration step exists.
2. **Horizontal navigation**: arrow key or UI button → previous/next sibling image in the current folder (alphanumeric order) → decode → display; while the image is wider than the window, pan first; navigation only at the image's left/right edge.
3. **Vertical navigation**: arrow key or UI button → previous/next sibling folder under the same parent → target image chosen by priority: (a) the folder's session last-viewed image if the folder was already visited this session (RAM-only), else (b) ALWAYS the target folder's first image (alphanumeric order) → decode → display; while the image is taller than the window, pan first; navigation only at the image's top/bottom edge.
4. **Fullscreen / zoom toggles**: fullscreen is OFF by default at launch (the window opens normal or maximized at the user's own placement/size); fullscreen OFF shows the UI controls with the image taking almost all of the central space; fullscreen ON hides the window header and ALL UI controls, showing only the image; keyboard shortcut and UI control toggle it; zoom via shortcut + controls; fit width / fit height modes.
5. **Open with**: menu entry → enumerate system-registered programs for the current file type → the selected program opens the current file.

## Data and State

- **In memory (Wave-1 landed)**: the open-flow state in `ImagesScrollerApp` — last opened path, decoded RGBA payload, current message text, and the display texture handle — all RAM-only, replaced on each successful open; nothing persisted.
- **In memory (per session, planned)**: folder context while navigating; the per-folder last-viewed map (RAM-only, destroyed on close — no persistence in v1); window/zoom state for the session.
- **On disk (planned)**: an application **config file** shipped by the installer alongside the binary (brief distribution flow). Its contents/format are **not defined anywhere yet** — only its existence is recorded here; contents are an implementation decision.
- No other persistent state in v1; explicitly no persistence of the last-viewed memory.

## Build and Release Pipeline

Landed in Wave 1 (Task 1) and exercised in Task 5: dev checks ended `ALL CHECKS PASSED`; the Windows x64 cross-build exited 0 → `dist/windows-x64/img-scroller.exe` (19,428,809 bytes). Guides: [docs/build.md](../../docs/build.md) and [docs/build-windows.md](../../docs/build-windows.md).

- Dev checks: `scripts/dev-checks.sh` runs inside the container — in order `cargo fmt --check`, `cargo check --locked`, `cargo test --locked`, `cargo clippy --locked -- -D warnings` — emitting per-check PASS/FAIL lines and a final `ALL CHECKS PASSED` summary, with a UTC-timestamped log under the gitignored `logs/checks/`.
- Windows release cross-build: `scripts/build-windows.sh` runs `cargo build --release --locked --target x86_64-pc-windows-gnu` inside the container and installs the final exe atomically (`.part` copy → rename) at `dist/windows-x64/img-scroller.exe`; a failed run leaves the previous artifact untouched; the x86 target belongs to Wave 7 (consuming/extending this script and adding the i686 MinGW toolchain when the x86 target is added).
- Container image: pinned official `rust:1.99.1-bookworm` → `img-scroller-rust:latest` (rustfmt, clippy, MinGW-w64 x64 toolchain, `x86_64-pc-windows-gnu` target); named volume `img-scroller-cargo-target` holds intermediate build output via `CARGO_TARGET_DIR`.
- Import-audit evidence: the OBSERVED 22-DLL import list of the produced exe is recorded once in `scripts/build-windows.sh` (`OBSERVED_DLL_IMPORT_LIST`) and in [docs/build-windows.md](../../docs/build-windows.md); not duplicated here.
- Validation stance: container-only — all agent-side build/check validation happens inside the container; Windows runtime/GUI validation is OPERATOR-RUN on the real Windows 10/11 machine (the Linux container cannot execute the Windows GUI application).

## Project Paths

- All source code must live in `src/` (project structure rule; folder map: [`.agent/project-structure.md`](../project-structure.md)).
- Cargo project at the repository root: `Cargo.toml` (package `img-scroller` 0.1.0, edition 2024) plus the tracked `Cargo.lock` — exists since Wave 1.
- Repository layout: application code in `src/` (five modules plus the `platform/` facade), harness in `Dockerfile`/`docker-compose.yml`/`scripts/`, guides in `docs/`, agent scaffolding in `.agent/`/`.kilo/`/`.opencode/`.
- Module layout: decided and implemented — see `## Module Layout (Implemented, Wave 1)`.

## Design Patterns

- The GUI paradigm follows egui/eframe's **immediate-mode** model (confirmed; in use in the landed shell).
- Landed Wave-1 pattern: OS-bound code isolated behind the cfg-gated `platform` facade, so container-runnable pure logic (`scaling`, `image_loading`, app state transitions) stays testable on the Linux host target.
- Everything else (navigation state management, internal abstractions) is not yet decided — see `## Open Architectural Decisions`.

## Technical Decisions Summary

| Decision | Status | Source (brief section) |
| --- | --- | --- |
| Language: Rust | landed (Wave 1) | `## Technology Stack` |
| GUI: egui/eframe | landed (Wave 1: window + open + display slice) | `## Technology Stack` |
| Image decoding: WIC system codecs | landed (Wave 1) | `## Technology Stack` |
| Target OS: Windows 10 / 11 | implemented for the GUI; runtime check operator-run | `## Technology Stack` |
| Target architectures: x64 + x86 | x64 landed (Wave 1); x86 planned (Wave 7) | `## Technology Stack` |
| Sorting: alphanumeric fixed in v1 | v1 constraint | `## Sorting Order` |
| Last-viewed memory: session-only, RAM | v1 constraint | `### Vertical Scroll (Sibling Folders)` |
| Distribution: zip-style installer | planned | `## Distribution and Default App Registration` |

## Open Architectural Decisions

Each item below is to be decided during implementation, NOT now:

- Navigation-engine design: horizontal sibling-image and vertical sibling-folder flow, scan/ordering and state transitions (Wave 2).
- Fullscreen/zoom keybindings and zoom step sizes.
- Pan-then-navigate details while zoomed (Wave 4).
- Distribution-helper mechanism (script vs. executable).
- Config file contents/format.
- Design patterns beyond immediate-mode.
````

## 4. Step S3 — Consistency checks (after all three edits)

Verify ALL of the following; any failure → fix the file to match this plan (re-apply the verbatim draft) or STOP and report:

1. Fact table — every claim in the written docs matches the repository (all verified 2026-10-31 during planning; re-verify by reading):

| Fact | Source of truth (repo) |
| --- | --- |
| package `img-scroller` 0.1.0, edition 2024 | `Cargo.toml` |
| `eframe` =0.37.0, `default-features = false`, features glow/default_fonts/clipboard/links/wayland/x11 | `Cargo.toml` |
| `raw-window-handle` =0.6.2 | `Cargo.toml` |
| `windows` =0.62.2 cfg(windows), 6 features (Foundation/Imaging/Com/Shell/Shell_Common/WindowsAndMessaging) | `Cargo.toml` |
| base image `rust:1.99.1-bookworm`; image `img-scroller-rust:latest` | `Dockerfile`, `docker-compose.yml` |
| volume `img-scroller-cargo-target` at `/cargo-target`; `CARGO_TARGET_DIR=/cargo-target` | `docker-compose.yml` |
| `dist/windows-x64/img-scroller.exe` | `scripts/build-windows.sh`, `docs/build-windows.md` |
| 22-entry OBSERVED DLL list; audited 2026-10-31 | `scripts/build-windows.sh` (constant), `docs/build-windows.md` |
| 15 unit tests (4+4+2+5) | `src/app.rs`, `src/image_loading.rs`, `src/app_display.rs`, `src/scaling.rs` |
| dev-checks order + `ALL CHECKS PASSED`/`N CHECK(S) FAILED` + `logs/checks/` + `FORCE_FAIL` | `docs/build.md` |
| exe size 19,428,809 bytes | caller-provided Task 5 record (do not restate elsewhere than the two landed lines in §3.2/§3.3) |

2. Pointer validity (cross-file): `README.md` → `docs/build.md` and `docs/build-windows.md` exist; `tech.md` → `../../docs/build.md`, `../../docs/build-windows.md`, `../todos/20261030/20261030-todo-1.md`, `../../.kilo/plans/20261030-wave1-skeleton-open-image.md` all exist; `architecture.md` → `../../docs/build.md`, `../../docs/build-windows.md`, `../project-structure.md` exist.
3. Anchor validity: every `#anchor` in the two `## Contents` blocks resolves to a heading present in the same file (compare the heading text lists).
4. tech.md internally consistent: `## Development Environment and Setup` section name matches the reference made in the Build Environment first bullet (no `(Placeholder)` suffix anywhere).
5. Line counts: README = 89 lines (no TOC); tech.md = 121 lines; architecture.md = 118 lines. A mismatch > 2 lines means content drift → diff against this plan's draft and re-apply.
6. No config/style churn: `git diff --stat` shows ONLY the three files; `.gitignore`, scripts, `Cargo.toml`, `Cargo.lock`, `src/**` untouched.
7. Newline rule: no literal `\n` escape sequences inside the written markdown (spot-check the two new files); all multi-line content uses real newlines.

## 5. Step S4 — Gitignore compliance and commit

1. Read `.gitignore`; confirm none of `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md` matches any pattern (they do not; verified 2026-10-31).
2. `git status` → confirm the only modifications are the three target files; confirm nothing under `dist/`, `logs/`, `target/` is staged or untracked-and-added (all three paths are gitignored; nothing to unstage).
3. Stage EXACTLY: `git add README.md .agent/project-info/tech.md .agent/project-info/architecture.md`
4. `git status` → staged list is exactly those three paths.
5. Commit: `git commit -m "Document Wave 1 build and verification state"` (final message — Decision 7).
6. Verify: `git log --oneline -3` shows the new commit on top of `b2645f4`; `git status` clean.

Prohibitions in this step: NO `git push`, NO branch creation/switch/merge (Step 5 only), NO edits to `CHANGELOG.md`, `.agent/project-info/context.md`, `.agent/project-structure.md`, `.agent/todos/**`, any existing `.kilo/plans/*` file. TODO `[DONE]`/`[x]` marks belong to step 4.6, NOT here.

## 6. Verification mapping (TODO Task 6 sub-items × landed artifacts) — for step 4.5b

| TODO sub-item | Where satisfied |
| --- | --- |
| README: build/run placeholder → container-harness build commands (per `docs/build.md`, VM/docker compose usage, no host Rust) + Windows cross-build pointer (`docs/build-windows.md`) + "any system-registered format" constraint | §3.1 Edit C (both `###` subsections, both doc links, constraint paragraph); commands match `docs/build.md` §One-time image build/§Running the dev-check harness and `docs/build-windows.md` §Invocation verbatim |
| tech.md: fill "Development Environment and Setup", "Build and Run", "Testing" with what landed; record WIC mechanism + dependencies chosen | §3.2 sections 5, 7, 8 (placeholders gone); §3.2 sections 3–4 (dependencies incl. pins/features/edition/glow decision; WIC + dialog chain); Status header updated (no longer "nothing implemented") |
| tech.md build-environment facts: harness landed + OBSERVED audit recorded; keep Build Environment section; Status header reflects Wave 1 | §3.2 section 6 (lead-in updated; pin/volume/audit/harness-landing bullets landed; list NOT duplicated — pointer per Decision 1); Status line |
| architecture.md: replace "planned/no code" status lines invalidated by the wave (module layout, WIC mechanism); keep undecided items open | §3.3: Status, Logical Components annotations, `## Module Layout (Implemented, Wave 1)`, `## Open-Image and Display Flow (Implemented)`, `## Build and Release Pipeline` (realized), `## Project Paths` (Cargo project exists; module layout decided), `## Technical Decisions Summary` statuses; `## Open Architectural Decisions` keeps fullscreen keys / zoom / pan-navigate / distribution helper / config file / patterns beyond immediate-mode and adds the navigation-engine open item (Wave 2) |
| context.md: critical closing step | EXCLUDED — Planner session-closing step (global plan Task 6 row); implementer must not touch it |
| TOC rule (> 100 lines) | README 89 lines → no TOC; tech.md 121 → Contents; architecture.md 118 → Contents |
| No config/style churn beyond the placeholders | §4 check 6 |
| REAL newlines | §4 check 7 |
| Do NOT touch CHANGELOG.md | §5 prohibitions |

## 7. Risks / STOP conditions

- Placeholder-string drift vs file reality: S1 anchors are exact; ANY mismatch → STOP + report (no improvisation).
- Overreach: any file outside the §0 table being modified → STOP; restore via `git checkout -- <file>` if accidentally touched before commit.
- Stale facts: all recorded values were re-verified from repo files during planning (not from memory); S3 check 1 re-verifies after writing.
- Scope-creep guard for the junior implementer: nothing in this plan authorizes code changes, version bumps, or harness re-runs; the build/verification evidence quoted (19,428,809 bytes; ALL CHECKS PASSED; audit date) comes from the landed Task 5 records and is only transcribed into the two doc lines.

## 8. Explicit non-goals

- No `docs/build.md`/`docs/build-windows.md` content changes (they landed in Task 1 and are only referenced).
- No `.agent/project-structure.md` edit (verified accurate against `src/` on 2026-10-31).
- No `.agent/project-info/brief.md`/`product.md` edits (planned-status wording there is out of Task 6 scope; not invalidated placeholders listed by the TODO).
- No TODO-file `[DONE]` marking (step 4.6), no TODO rename (Step 5), no merge/push (Step 5).
