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
- WIC decode chain: `CoCreateInstance(&CLSID_WICImagingFactory)` → `IWICImagingFactory::CreateDecoderFromFilename` (`GENERIC_READ`, `WICDecodeMetadataCacheOnDemand`) → `GetFrame(0)` → `GetSize` → `CreateFormatConverter` → `Initialize` to `GUID_WICPixelFormat32bppRGBA` (dither `WICBitmapDitherTypeNone`, palette `WICBitmapPaletteTypeCustom`) → `CopyPixels` read from the initialized converter (the conversion to straight RGBA materializes during this copy) into an RGBA8 byte buffer.
- Failure classification: `WINCODEC_ERR_COMPONENTNOTFOUND` → `Unsupported`; every other error → `Unloadable`; both surface the single unsupported-file message (constant in `src/image_loading.rs`) in the window's bottom message bar.

## Development Environment and Setup

- The Rust toolchain (stable, with rustfmt and clippy) exists ONLY inside the container image `img-scroller-rust:latest` (pinned base `rust:1.99.1-bookworm`) — nothing is installed on the host; the host needs no Rust/Cargo. Never install tools globally — in-project/container installs only (rule: [`.kilo/rules/never-global-installs.md`](../../.kilo/rules/never-global-installs.md)).
- The image also carries the MinGW-w64 x86-64 GNU cross toolchain (linker `x86_64-w64-mingw32-gcc`; `x86_64-w64-mingw32-objdump` backs the Windows import audit) and the rustup target `x86_64-pc-windows-gnu`.
- Agents reach the VM through the `alpine-vm` MCP: call `vm_status` first, then run commands; the command's exit code is the authoritative evidence; cargo stderr may be absent in captured MCP output (an absent stderr is NOT a failure signal). Full usage notes: [docs/build.md](../../docs/build.md).

## Build Environment (Container on the Alpine VM)

Build-environment decision recorded 2026-10-30 (source: [`.agent/todos/20261030/20261030-todo-1-DONE.md`](../todos/20261030/20261030-todo-1-DONE.md) → "Build Environment Definition"; user decisions bound in the container-harness plan). **Landed in Wave 1 Task 1 (2026-10-31)**: every harness file and both build guides exist and are exercised.

- Container/VM model: ALL Rust/Cargo commands (builds, checks, tests) execute inside a Linux container on the Alpine VM. The host machine that runs the AI agents does NOT need Rust or Cargo installed — the Rust toolchain exists only inside the container image, so the toolchain notes in `## Development Environment and Setup` above describe the toolchain shipped inside the container image, not a host install.
- Project share: the project is shared on the VM at `/img-scroller`; the container working directory is `/img-scroller`.
- Docker prerequisites: Docker with the Compose plugin is available ONLY on the VM — never on the host.
- Access pattern for AI agents: the VM is reached through the `alpine-vm` MCP — call `vm_status` first, then run commands; the command's exit code is the authoritative success evidence; cargo stderr may be absent in the MCP captured output (an absent stderr is NOT a failure signal). If the VM or the `/img-scroller` share is unavailable, STOP the task and return a question to the caller/user.
- Cargo invocations across all waves run through the container harness (`docker compose -f /img-scroller/docker-compose.yml …`); usage is documented in [docs/build.md](../../docs/build.md).
- Base-image pin (landed): official `rust:1.99.1-bookworm` → image name `img-scroller-rust:latest`; rebuild only after a Dockerfile change; toolchain upgrades are plan-gated.
- Named volume (landed): `img-scroller-cargo-target` mounted at `/cargo-target` via `CARGO_TARGET_DIR=/cargo-target` — intermediate Cargo output never lands in the repository; the only tracked root-level build artifact is `Cargo.lock`.
- Lock/artifact policy: `Cargo.lock` is tracked in git; every harness cargo invocation uses `--locked`; produced artifacts stay in the gitignored `dist/` (release `.exe`) and check logs in the gitignored `logs/`/`logs/checks/` — the repo root `.gitignore` contains `dist/`, `logs/`, `*.log`, and `target/`.
- Validation stance: validation is container-only — agent-side Windows GUI/runtime execution is impossible from the Linux container; runtime/GUI validation is OPERATOR-RUN on the real Windows 10/11 machine.
- Harness landing (DONE, Wave 1 Task 1): `Dockerfile`, `docker-compose.yml`, `scripts/dev-checks.sh`, `scripts/build-windows.sh`, `docs/build.md`, `docs/build-windows.md` all exist and are exercised (Task 5: `ALL CHECKS PASSED`; cross-build exit 0 → non-empty `dist/windows-x64/img-scroller.exe`).
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
