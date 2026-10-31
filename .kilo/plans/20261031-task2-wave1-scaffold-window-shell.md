# Implementation Plan — Wave 1, Task 2: Cargo Project Scaffold and Window Shell

- Date: 2026-10-31
- Workflow step: Critical Workflow §4.1b (Analysis & Planning) — Task 2 of TODO
  [`.agent/todos/20261029/20261029-todo-3.md`](../../.agent/todos/20261029/20261029-todo-3.md) ("### Task 2: Cargo project scaffold and window shell")
- Role: Architector. Plan ONLY — the file below is the deliverable of this step. No application code written here; no cargo/docker runs that modify the repo (research/verification reads only).
- Executing branch (already created in Step 2): `feat/wave-1-skeleton-open-image` — no branch operations in this task.
- Plan approval: AUTO-APPROVED (user chose "Approve Global and Tasks Plans", 2026-10-30). Save and return; do NOT ask for approval.
- Front-end input (workflow 4.1b requirement): Task 2 is front-end related. The 4.1a Front-end Technical Specification is in force as input: [`.kilo/plans/20261031-task2-wave1-app-shell-frontend-spec.md`](20261031-task2-wave1-app-shell-frontend-spec.md) (§5 static checklist for 4.5a; §6 rule table). Deviations are encoded in §5 of this plan — none require re-opening the spec.
- Global plan in force (do NOT modify): [`.kilo/plans/20261030-wave1-skeleton-open-image.md`](20261030-wave1-skeleton-open-image.md) — §0.2 (binding build environment), §0.3 (deferred decisions), §2 Task 2, §3 (risks).
- Task 1 bindings in force (do NOT restate here beyond what this section binds): [`.kilo/plans/20261031-task1-wave1-build-harness.md`](20261031-task1-wave1-build-harness.md) §1.1/§1.2/§8.
- Target implementer: JUNIOR developer, 50% restriction — every structural/naming/scope decision is encoded here; no judgment calls left behind.

## 0. Scope

### 0.1 In scope (TODO Task 2 sub-items, verbatim mapping)

| # | TODO sub-item (abbreviated) | Delivered by this plan |
| --- | --- | --- |
| 1 | Create `Cargo.toml` at repo root + first `src/` application code; remove `src/.gitkeep` once source exists | §3 (S2–S6), §2.1–§2.3, §2.8 |
| 2 | egui/eframe added via `cargo add` in the implementation step; exact crate identifiers/versions per architector (this is where they are decided) | §1.1 decisions c–d, §2.2, §3 (S4) |
| 3 | Regular, resizable viewer window (not maximized, not modal); window title = `Images Scroller` | §2.4 (draft `app.rs`); frontend-spec §1 mapped in §5 |
| 4 | Scaffold builds through the container harness per `docs/build.md`; empty-window GUI run validated in Task 5 operator-run item (container proves compile-only) | §3 (S5–S6); frontend-spec §8.2 recorded as operator-pending |

### 0.2 Out of scope (explicit)

- No image decode/render code, no WIC calls, no dialog code, no CLI parsing (Tasks 3), no texture/fit-scale code (Task 4), no release cross-build or import audit (Task 5), no README/tech/architecture/context updates (Task 6).
- No `CHANGELOG.md` edits; no edits to existing `.kilo/plans/*` files; no version bump beyond creating `0.1.0`; no push; no branch operations.
- The `windows` crate is NOT added in Task 2 (nothing uses it yet); its crate/version/features are decided in §1.1-h and executed by Task 3's 4.2 (single decision chain).
- No Windows GUI/runtime validation of any kind in this task (container is compile/check/test/lint only). The Task 5 operator run validates the empty window.

### 0.3 Binding constraints carried (Task 1 plan + harness)

- Harness commands: `docker compose -f /img-scroller/docker-compose.yml …`; dev-checks: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` (frontend of `docs/build.md`, Task 1).
- Package chain: `package.name = "img-scroller"`, `version = "0.1.0"`, NO `[[bin]]`, NO bin-name override; binary name `img-scroller` → source exe `$CARGO_TARGET_DIR/x86_64-pc-windows-gnu/release/img-scroller.exe` → deliverable `dist/windows-x64/img-scroller.exe`.
- `Cargo.lock` is tracked and committed; every harness cargo invocation uses `--locked` (this task generates the first `Cargo.lock`).
- Dev-checks semantics: ordered `fmt-check` → `check` → `test` → `clippy`, all four always run; final lines exactly `ALL CHECKS PASSED` (exit 0) / `N CHECK(S) FAILED` (exit 1); log under gitignored `logs/checks/`.
- Alpine-VM/MCP: call `vm_status` BEFORE every VM command; the command EXIT CODE is the authoritative success evidence; absent cargo stderr is NOT a failure signal; if VM or `/img-scroller` share unavailable → STOP + user question (binding rule).

## 1. Architector decisions — deferred items settled NOW (wave decisions core)

Verification date for all version facts: 2026-10-31. Sources recorded per row. Escape hatch: a version bump later is a deliberate, plan-gated change (never auto-bumped; never upgraded by the implementer).

### 1.1 Decision table

| # | Deferred item | Decision | Value / rationale (verified) |
| --- | --- | --- | --- |
| a | WIC binding approach | **`windows` crate (microsoft/windows-rs projection) v0.62.2, Win32 COM/WIC APIs: `CoCreateInstance` + `IWICImagingFactory::CreateDecoderFromFilename` → `IWICBitmapFrameDecode::CopyPixels`** | Official Microsoft-maintained projection (C++/WinRT tradition); WIC features via tiny feature flags (no bundled decoders — the codec DLLs are the user's system codecs, exactly the brief's constraint). The `windows` crate supports `x86_64-pc-windows-gnu` (its `windows_x86_64_gnu` import-lib crate exists specifically for the GNU target). The `image` crate is INELIGIBLE and is REJECTED (brief). Task 3 executes `cargo add`; Task 2 adds nothing (§0.2). Pinned exact: `windows@=0.62.2` (latest release-track crate of windows-rs as of 2026-10-31; releases 72–74 of 2026 touched only `windows-bindgen`/`windows-metadata`/`windows-rdl`, not `windows`). Candidate `win-tray`-style misc crates rejected: unmaintained/inapplicable. |
| b | src/ module layout | **Task 2: `src/main.rs` (entry) + `src/app.rs` (app shell). Recorded expansion points: Task 3 adds `src/image_loading.rs` + `src/platform/` tree (`src/platform/mod.rs`, `src/platform/windows.rs`, `src/platform/stub.rs`); Task 4 adds a pure fit-scale helper module (candidate name `src/scaling.rs`, finalized by Task 4's 4.1b).** | Files stay small (Max-Lines rules); every file ≤200 lines. Exact contents of new Task 3/4 files are theirs; the module names and the platform-facade seam in §8 bind them NOW. |
| c | GUI stack crates + versions (PINNED) | **`eframe = "=0.37.0"` as the ONLY declared GUI dependency; egui accessed via `eframe::egui` re-export. Feature set: `default-features = false`, `features = ["glow", "default_fonts", "clipboard", "links", "wayland", "x11"]`** | eframe 0.37.0 (published Oct 2026, `rust-version = 1.95` — compatible with rustc 1.99.1) verified: its lib re-exports `pub use {egui, egui::emath, egui::epaint}`. Exact pin `=0.37.0` — crates in the same semver-incompatible cycle MUST stay in lockstep; a caret pin could silently mix egui versions. Feature set decisions: (i) `glow` not `wgpu` — decision (h) below; (ii) `wayland` + `x11` REQUIRED — without at least one of them, `winit` hard-fails to compile for the Linux container target (`compile_error!("The platform you're compiling for is not supported by winit")`, egui issue #7106), and the whole dev-check harness would break; on Windows these features compile to nothing (their deps are Linux-target-gated); (iii) `accesskit`/`system_fonts`/`web_screen_reader`/`persistence`/`inspection` omitted — weight and yagni; a later wave may add features only via a plan-gated change. NOTE: no separate `egui`/`egui-winit` `cargo add` — they arrive transitively locked by eframe's own requirements; adding `egui` directly would re-unify features and invite drift. |
| d | Backend feature choice for x86_64-pc-windows-gnu | **glow (OpenGL/WGL). NOT wgpu.** | Verified from eframe 0.37 sources: `wgpu` is now the DEFAULT backend, and on Windows an eframe+wgpu build without explicit `dx12`/`vulkan` backend features panics at startup (egui issue #7943, 2026-03: "No wgpu backend feature that is implemented for the target platform was enabled"), i.e. extra wgpu feature/wiring plus DX12 shader-compiler DLL logistics — risk on a MinGW-w64 GNU cross build with an operator-run GUI we cannot test here. glow compiles with only the `glow` feature; with glow sole-enabled, `Renderer::default()` returns `Glow` automatically (verified in epi.rs default impl), so no renderer selection code is needed. OpenGL via WGL is the classic, dependency-light path for a 2D image viewer on Windows 10/11. glow is also smaller. Accepted residual risk: glow's windows-gnu cross-linking is exercised end-to-end only in Task 5's release build (§7-R6). |
| e | Open-dialog mechanism | **Native `IFileOpenDialog` COM dialog via the SAME `windows` crate chosen in (a). `rfd` REJECTED** (it is a wrapper over the same COM dialog — one more dependency and one more COM stack for zero capability gain; egui's own FAQ points to `rfd`, but the direct API costs nothing extra here). NOT an egui-drawn modal-only dialog (the window itself is the regular app window, not a modal; the native common dialog over it is the standard Windows pattern). Owner: the main window's HWND obtained through the winit window handle; a null owner is the acceptable degraded fallback. Task 3 implements; Task 2 adds nothing. Related `windows` features for Task 3 (decided NOW, pinned in §8): `Win32_Foundation`, `Win32_Graphics_Imaging`, `Win32_System_Com`, `Win32_UI_Shell`, `Win32_UI_Shell_Common`, `Win32_UI_WindowsAndMessaging`. |
| f | Unsupported-file message wording | **Constant `UNSUPPORTED_FILE_MESSAGE`, exact value: `"This file's format is not supported: no Windows image codec is registered for it."` — English, single line, to be displayed as an informational message inside the viewer window. Lives in `src/image_loading.rs` (created by Task 3).** | Decided now so the user-facing wording is testable and stable; Task 3 writes the constant and wires the message path. |
| g | Unit-test strategy | **(1) Pure logic = container-runnable inline `#[cfg(test)] mod tests` in the owning file. Task 2 ships the first test: `app.rs` asserts `APP_WINDOW_TITLE == "Images Scroller"` (user-facing contract; makes `cargo test` meaningful). (2) OS-bound WIC/COM/dialog code is NOT unit-tested — the container has no Windows and COM cannot run; it is validated by Task 5's operator-run item. (3) The seam that makes this clean: all OS-bound calls sit behind the `src/platform/` facade (Task 3); pure helpers (Task 2's shell n/a; Task 3 argument→path parsing; Task 4 fit-scale math) test on the Linux host target inside the container.** | Matches TODO Task 5: "add first unit tests only if this wave's architector step defines a test suite; a minimal smoke test … is acceptable". |
| h | Linux-target check strategy for Windows-only code | **cfg-gating with PAIRED implementations, same API on both sides. Exact pattern (binds Task 3): `src/platform/mod.rs` declares `#[cfg(target_os = "windows")] mod windows_impl;` and `#[cfg(not(target_os = "windows"))] mod stub_impl;`, then re-exports the SAME public facade (function signatures + `ImageDecodeResult`-style own enum) from whichever compiled. The Windows side holds ALL WIC/COM/`windows`-crate code; the stub side returns the facade's failure/degenerate values without any dependency on the `windows` crate (so the Linux container's `cargo check/test/clippy` never compiles `windows` APIs). NEVER a windows-target-only check (dev-checks must stay runnable in-container).** | This is the compromise that keeps `dev-checks.sh` (`check/test/clippy` on host target) green while the real code is Windows-only. No third cfg arm may be invented. |

### 1.2 Remaining deferred items (settled by Task 1's plan, restated here only as consumed)

- Rust base image: `rust:1.99.1-bookworm`; volume `img-scroller-cargo-target` with `CARGO_TARGET_DIR=/cargo-target`; exe `img-scroller.exe` at `dist/windows-x64/`; `FORCE_FAIL` hook untouched. Consumed verbatim; nothing to re-decide.

### 1.3 Cargo edition (remaining deferred item to settle)

- **Decision: `edition = "2024"`** in `package`. Rationale: edition 2024 stabilized in Rust 1.85, so rustc 1.99.1 (container toolchain) supports it fully; it is also cargo's own `cargo new` default at 1.99.x, keeping the manifest consistent with toolchain behavior. No code written here relies on edition-specific new syntax (the shell code is edition-agnostic).

## 2. Deliverables — full drafts (implementer materializes verbatim)

Why the drafts below look the way they do: **eframe/egui 0.37 changed the App trait** — the repaint callback is now `fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)` (required method, no default body); `update(&mut self, ctx, frame)` no longer exists in 0.37. The 4.1a frontend spec was written with the older `update` shape as the example; its own tolerance clause ("CentralPanel … or equivalent standard egui surface", spec §5) is what the code below satisfies — see §5 mapping. The CentralPanel form is copied from eframe 0.37's own library doctest (`egui::CentralPanel::default().show(ui, |ui| { … })`), so it is verified compile-correct by the crate's CI.

### 2.1 `Cargo.toml` (new, repository root) — package skeleton written FIRST

```toml
[package]
name = "img-scroller"
version = "0.1.0"
edition = "2024"

[dependencies]
```

The `[dependencies]` entry is added by `cargo add` inside the container (step S4) — NOT hand-written — so `Cargo.lock` and the manifest are produced by the real toolchain. Expected result after S4 (shape check; exact formatting may differ marginally, values must match):

```toml
[package]
name = "img-scroller"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "=0.37.0", default-features = false, features = [
    "glow",
    "default_fonts",
    "clipboard",
    "links",
    "wayland",
    "x11",
] }
```

Checks after S4 (each must hold, else STOP and report): `version = "=0.37.0"` (exact pin, with `=`); `default-features = false`; features exactly the six above; NO other packages in `[dependencies]`; `Cargo.lock` exists in the repo root afterwards.

Recorded-so-far note (for review): the `image` crate appears in the LOCKFILE as a transitive dependency of eframe native builds (eframe uses it internally for the default app icon). The app itself never declares or uses it; the brief's no-bundled-decoders constraint governs OUR decode path (WIC), which is untouched. This is expected and acceptable; do not "fix" it.

### 2.2 `src/main.rs` (new)

```rust
//! Entry point of the Images Scroller viewer.

mod app;

fn main() -> eframe::Result {
    eframe::run_native(
        app::APP_WINDOW_TITLE,
        app::window_options(),
        Box::new(|creation_context| {
            Ok(Box::new(app::ImagesScrollerApp::new(creation_context)))
        }),
    )
}
```

Semantics the implementer must be able to state back:
- `eframe::run_native` is called EXACTLY once (frontend-spec 1.1), with `app_name = APP_WINDOW_TITLE` ("Images Scroller" — eframe uses this as the viewport title/app id fallback as well, so the title is set at BOTH the call site and the viewport builder).
- The `AppCreator` closure returns `Ok(Box::new(...))` — eframe 0.38-era `AppCreator` is `FnOnce(&CreationContext) -> Result<Box<dyn App>, DynError>` (verified in eframe 0.37.0 epi.rs).
- `eframe::Result` is eframe 0.37's exported `Result<(), eframe::Error>`; returning it from `main` makes a graphics-context failure print an error and exit non-zero instead of panicking.

### 2.3 `src/app.rs` (new)

```rust
//! Application shell: the regular, resizable viewer window of Images Scroller.

use std::path::PathBuf;

use eframe::egui;

/// Title shown on the viewer window title bar (final app name, user-decided 2026-10-29).
pub const APP_WINDOW_TITLE: &str = "Images Scroller";

pub struct ImagesScrollerApp {
    // Placeholder for the Task 3 open-image pipeline (frontend-spec 2.6); Task 2 reads nothing.
    #[allow(dead_code)]
    opened_image_path: Option<PathBuf>,
}

impl ImagesScrollerApp {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            opened_image_path: None,
        }
    }
}

impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |_ui| {});
    }
}

pub fn window_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_WINDOW_TITLE)
            .with_resizable(true)
            .with_maximized(false),
        ..eframe::NativeOptions::default()
    }
}

#[cfg(test)]
mod tests {
    use super::APP_WINDOW_TITLE;

    #[test]
    fn window_title_is_the_final_app_name() {
        assert_eq!(APP_WINDOW_TITLE, "Images Scroller");
    }
}
```

Semantics the implementer must be able to state back:
- Window contract (frontend-spec §1): `.with_resizable(true)` (1.3), `.with_maximized(false)` (1.4), explicit `.with_title("Images Scroller")` (1.2). No position/size set → OS default placement (1.6). The primary `eframe::App` window is the regular window; nothing about it is a modal (1.5).
- Paint contract (frontend-spec §3): the single CentralPanel clear pass and NOTHING else — no labels, buttons, images, menus, or placeholder text (3.3). `CentralPanel::show` on the provided `ui` fills the panel with the current style background and establishes the Task 4 image surface anchor (3.1).
- `#![allow(dead_code)]`-style suppression is deliberately scoped to ONLY the placeholder field, with the reason stated at the definition; the field is mandated by frontend-spec 2.6 and stays private (private-by-default rule; `pub` on `ImagesScrollerApp`, `APP_WINDOW_TITLE`, and `window_options()` is the necessary exception — `main.rs` consumes them).
- The `#[cfg(test)]` module is the wave's first unit test (decision g): it pins the user-facing window title.
- Rule compliance check (pre-verify before writing): main.rs bodies ≤50 lines (yes); app.rs files ≤200 lines (yes); nesting depth ≤2 (yes); method args ≤2 (`new` takes 1, `ui` takes 2 — within limit); single-section boolean conditions (none used); no commented-out code (none).

### 2.4 `src/.gitkeep` (DELETE)

Delete the placeholder file once `src/main.rs` and `src/app.rs` exist (TODO sub-item 1; frontend-spec 4.3). Deletion happens right after S2 so the track-and-commit set in §4 is consistent.

### 2.5 `.agent/project-structure.md` — EXACT edit

The `# Folders in src/` block currently reads:

```markdown
# Folders in src/

# (no folders yet)
```

Replace that block with:

```markdown
# Files in src/

- app.rs - eframe application shell: viewer window options, app struct, window-title constant
- main.rs - entry point: starts the eframe native app

(no folders in src/)
```

Keep the file's other lines (`# Other folders`, the `.agent/`, `.kilo/`, … bullets, `docs/`, `scripts/`) UNTOUCHED.

## 3. Implementation steps (for 4.2 — ordered, atomic, verifiable)

File writes happen on the host with the normal file tools (the repo IS the `/img-scroller` share). VM commands go through the `alpine-vm` MCP. Rule: call `alpine-vm_vm_status` BEFORE EVERY command listed here; use timeoutMs as stated; exit code is authoritative.

- S1. Preconditions: `vm_status` → expect `running` + SSH reachable, else STOP + user question (binding). `vm_status`, then `ls /img-scroller/Cargo.toml` → expect a "No such file" style error (nothing exists yet); `ls /img-scroller/src` → expect `.gitkeep` listed. If Cargo.toml already exists → STOP: Task 2 was already (partially) implemented; report state to caller.
- S2. Write drafts verbatim: `Cargo.toml` (§2.1 package skeleton), `src/main.rs` (§2.2), `src/app.rs` (§2.3). LF line endings, trailing newline, UTF-8 without BOM.
- S3. Delete `src/.gitkeep` (§2.4) and apply the project-structure.md edit (§2.5).
- S4. Dependency add (the wave's only cargo mutation step), `vm_status` first:
  `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo add eframe@=0.37.0 --no-default-features --features glow,default_fonts,clipboard,links,wayland,x11`
  `timeoutMs: 180000`.
  - Then verify: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cat Cargo.toml` — diff against the §2.1 "expected result" shape: exact `=0.37.0` pin, `default-features = false`, exactly the six features, no extra deps. Then `docker compose -f /img-scroller/docker-compose.yml run --rm rust ls Cargo.lock` — must exist; if the lockfile is absent, run `cargo check` ONCE without `--locked` (fallback path to materialize it) and re-list.
  - Failure handling: cargo add failures of the "network/unreachable crates.io" kind → retry the IDENTICAL command ONCE (transient), and a persistent failure → STOP: leave files uncommitted, capture full output, escalate to the caller (risk §7-R3). Any other failure type (invalid feature name etc.) → the manifest/feature set is plan-owned; do NOT "correct" features on your own; STOP + report.
- S5. Harness first end-to-end exercise (TODO ordering note — dev-checks are first EXERCISED here):
  `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
  `timeoutMs: 1200000` (first container run compiles the whole eframe tree for the host target into `/cargo-target`; several minutes are normal).
  Expect exit 0 and output ending `PASS clippy exit=0 (…)`, `ALL CHECKS PASSED`. `cargo test --locked` with one test must show `test result: ok. 1 passed`.
  - Note: cargo may print warnings from cargo itself (e.g. "unused variable" style warnings would fail clippy — none should exist in the drafts). If `clippy` FAILS on a draft typo (e.g. a missing underscore-prefixed param), the fix is trivially in-draft: correct the draft in §2.2/§2.3, re-run S5, and note the fix in the summary. If the failure is structural (dependency resolution, toolchain, eframe version drift) → STOP + report with full output (risk §7-R1).
- S6. Git hygiene + commit per §4.
- S7. Return a structured summary: files created/deleted (level: full verbatim per §2), the VM commands with their exit codes, the dev-checks per-check lines and the final summary line, commit hash. Evidence lives in MCP captured outputs + summary — do NOT create evidence log files in the repo.

## 4. Commit points and messages (gitignore compliance BEFORE the commit)

Committer: implementer (4.2), after S5 succeeds so the committed scaffold is harness-verified.

Commit 1 — scaffold + window shell (single commit):
- Files: `Cargo.toml`, `Cargo.lock` (tracked by policy), `src/main.rs`, `src/app.rs`, `src/.gitkeep` (deletion), `.agent/project-structure.md`
- Message: `Add Cargo scaffold and egui/eframe window shell (Images Scroller)`

Procedure before the commit (Gitignore Compliance Rule):
1. Read `.gitignore` — confirm `dist/`, `logs/`, `*.log`, `target/` entries exist and nothing else changed.
2. `git status` — only the intended modified/new/deleted paths appear.
3. Stage ONLY the listed files (`git add Cargo.toml Cargo.lock src/main.rs src/app.rs`, then `git rm src/.gitkeep`, then `git add .agent/project-structure.md`); no `git add .`.
4. Re-run `git status`: staged list matches exactly; NO `dist/`, `logs/`, `*.log`, `target/`, or any gitignored path staged (the `logs/checks/` files from S5 are gitignored — verify none appear).
5. Commit with the exact message above.
6. `git status` confirms a clean tree afterward. Do NOT push (push restricted to Step 5, `origin` only). No branch operations.

## 5. Frontend-spec mapping (input → implementation; for 4.5a)

The 4.1a spec's §5 static checklist, item by item, mapped to the drafts (all verifiable by reading the code — no build/run):

| Frontend-spec §/checklist item | Draft location |
| --- | --- |
| §4.1 `Cargo.toml` name `img-scroller`, version `0.1.0` | §2.1 package block |
| §4.2 No `[[bin]]`; binary inherits package name → `img-scroller.exe` | §2.1 (no `[[bin]]` anywhere) |
| §4.3/§5 All source in `src/`; `src/.gitkeep` removed; ≥1 source file | §2.2–§2.4 |
| §1.1/§2.1 One `eframe::run_native` call, one `eframe::App` | §2.2 `main()` |
| §1.2/§5 Title exactly `"Images Scroller"` | §2.3 `APP_WINDOW_TITLE` + `.with_title(...)` + passed as `run_native` app_name |
| §1.3/§5 `resizable(true)` | §2.3 `.with_resizable(true)` |
| §1.4/§5 `maximized(false)` | §2.3 `.with_maximized(false)` |
| §1.5/§5 Not a modal; primary window | eframe root viewport, no viewport spawning |
| §1.6/§5 No forced position/size | §2.3 builder sets only title/resizable/maximized |
| §1.7/§5 No claimed bindings | No input handling in the drafts |
| §2.2/§5 No global/static mutable state | State lives in `ImagesScrollerApp` fields |
| §2.3/§5 Constructor accepting `cc` | §2.3 `new(_creation_context...)` |
| §2.4/§3.1/§5 Clear background paint, exactly one, via `CentralPanel` | §2.3 `ui()` → `CentralPanel::default().show(ui, |_ui| {})` |
| §2.5/§5 CentralPanel anchors the future image surface | Same call (Task 4's plug point) |
| §2.6/§5 Private placeholder `None` for image context | §2.3 `opened_image_path: Option<PathBuf>` |
| §5 No image rendering/dialog/CLI/navigation code; no TODO markers, no commented-out code | Whole drafts |
| §6 rules table (file/method sizes, depth, privates, self-documenting, no commented code) | §2.3 compliance list |

**Encoded adaptation (do NOT "fix" back):** spec §2.4 names `update(&mut self, ctx, frame)` as the repaint callback; eframe 0.37 renamed that path to `ui(&mut self, ui: &mut egui::Ui, frame)` (required, no default) — verified against the 0.37.0 published source (`epi.rs` App trait: `fn ui` is the required method; `update` no longer exists). The spec's own tolerance clause ("update() paints a clear background pass via CentralPanel (**or equivalent standard egui surface**)") covers this: the paint is a CentralPanel clear pass on the provided `ui`. 4.5a reads the checklist with this mapping; 4.4 (docs-specialist) sees this same row in the plan.

The Task 5 operator-run item (frontend-spec §8 items 2–5: exe launched on Windows opens the regular resizable un-maximized, non-modal window titled `Images Scroller` with painted background and no UI controls) is NEVER agent-executed; it stays operator-pending (recorded, not failed).

## 6. Verification mapping (for 4.5b — Overall Plan Adherence)

| TODO Task 2 sub-item | Verify in 4.5b by | Pass state |
| --- | --- | --- |
| `Cargo.toml` at root + first `src/` app code; `.gitkeep` removed | §2.1 post-S4 manifest shape (values, not whitespace); §2.2/§2.3/§2.4 drafts landed | Content match + `.gitkeep` absent |
| eframe added via `cargo add`; crate identifier/version per architector | Summarized S4 command + post-S4 manifest check in the 4.2 summary | `=0.37.0` exact, six features, single dep |
| Regular resizable non-maximized non-modal window titled `Images Scroller` | §5 mapping table read-through (static) + §2.3 code | All §5 rows pass |
| Scaffold builds through container harness; GUI runtime validated in Task 5 operator-run | S5 dev-checks final `ALL CHECKS PASSED` exit 0 + `1 passed` test result in the 4.2 summary; nothing Windows-side executed | Output lines exist and pass |
| Harness first exercised end-to-end in Task 2 (ordering note) | 4.2 summary shows S5 dev-checks run on the scaffold | Exercise evidence present |
| `project-structure.md` accurate | §2.5 edit landed verbatim | Content match |
| Commit hygiene (gitignore rule; Cargo.lock tracked; no push) | §4 procedure traces in 4.2 summary; single commit with the exact message | Git log/status check |
| Frontend-spec §8/TODO Task-5 operator item | Recorded as operator-pending (not failed) | Recorded |
| Workflow step 4.6 (marker duty) | After 4.3/4.4/4.5b, implementer flips Task 2 sub-item `[ ]`→`[x]` in `20261029-todo-3.md`, preserving all other content, then commits | Explicit follow-up below §3; not part of this plan's S-steps |

## 7. Risks and mitigations

| # | Risk | Mitigation / handling |
| --- | --- | --- |
| R1 | eframe/egui API drift vs drafts (e.g. the `update`→`ui` 0.37 change we already encoded) | Exact `=0.37.0` pin + drafts copied from 0.37.0's own doctest/sources (run_native signature, `AppCreator` Result, `pub use egui`, `Renderer::default()` behavior — all verified 2026-10-31); S5 compiles the drafts in the container before anything is committed. Implementer never bumps/patches API usage ad hoc; unknown compile failure → STOP + report. |
| R2 | windows-gnu backend quirks (glow/glutin link path) | glow chosen over wgpu explicitly (decision d, §7-R6); the cross-build only engages in Task 5's release build where it is verified on the produced exe. Windows behavior is operator-validated regardless. |
| R3 | `cargo add` network availability in the container | Retry the identical command once (S4); persistent failure → STOP with full captured output; the plan's manifest drafts match what `cargo add` would land, so a later recovery intended to write the dependency by hand still uses the plan values, but only after an explicit caller decision. |
| R4 | dev-checks first-run duration (full eframe tree compile for the Linux target) | `timeoutMs: 1200000`; subsequent runs are incremental (named volume keeps build state). |
| R5 | Clippy `-D warnings` on the placeholder field (`dead_code`) | Field-scoped `#[allow(dead_code)]` with an inline reason, mandate from frontend-spec 2.6; reviewed in 4.3. |
| R6 | If dev-checks fails on the scaffold | Capture output; fix ONLY trivially in-draft failures (draft typos: params, missing underscore, formatting); re-run; persistent/structural failure → STOP + report with full output. Never "fix" by changing decisions (features/versions/layout). |
| R7 | MCP capture may omit cargo stderr | Exit code is authoritative (binding from Task 1 docs); absent stderr is not interpreted as success or failure. |
| R8 | Share/VM availability mid-task | `vm_status` before every command; STOP + user question if unavailable (binding §4.3). |

## 8. Cross-task bindings (single decision chain — encode into Task 3/4/5 plans)

Bind to Task 3 (open image):
- Add: `cargo add windows@=0.62.2 --features Win32_Foundation,Win32_Graphics_Imaging,Win32_System_Com,Win32_UI_Shell,Win32_UI_Shell_Common,Win32_UI_WindowsAndMessaging` (baseline features that implement decisions a/e; if a Task 3 implementation detail proves one more feature necessary, its 4.1b adds it additively with rationale — never bumps the version).
- Pipeline (decision a): CLI path or dialog → `CreateDecoderFromFilename` on `IWICImagingFactory` from `CoCreateInstance` → frame → `CopyPixels` → app consumes decoded pixels (the `image` crate is NEVER used on our path).
- Modules (decision b/h): `src/image_loading.rs` (pure parse/classification + `UNSUPPORTED_FILE_MESSAGE` + seam to platform facade), `src/platform/{mod.rs,windows.rs,stub.rs}` with the exact cfg pattern of decision h (same facade API on both sides — windows_impl with WIC/COM/dialog, stub_impl returning the facade's failure values).
- Dialog (decision e): `IFileOpenDialog` via `windows` 0.62.2; owner HWND from the winit window handle; null-owner fallback; egui menu/button opens it.
- Message (decision f): constant `UNSUPPORTED_FILE_MESSAGE` = `"This file's format is not supported: no Windows image codec is registered for it."` displayed as an informational message in the window on unsupported/unloadable file.
- Tests (decision g): inline `#[cfg(test)]` for pure logic (arg→path classification, message constant exactness); platform/OS-bound code remains untested in container.

Bind to Task 4 (display):
- Image pixels from Task 3's decode are uploaded as an egui texture and shown in the SAME `CentralPanel` surface established by `ImagesScrollerApp::ui`; the fit-scale math is a pure helper module (candidate `src/scaling.rs`, name finalized by Task 4's 4.1b) with container-runnable tests (decision g); no zoom modes (Wave 4).

Bind to Task 5 (verification):
- Dev-checks re-run must stay `ALL CHECKS PASSED` (scaffold + Tasks 3/4 code).
- Release cross-build `scripts/build-windows.sh` must produce non-empty `dist/windows-x64/img-scroller.exe`; objdump import audit runs ONCE on OUR exe and fills the OBSERVED DLL list (Task 1 binding). If the audit or build exposes a glow/windows-gnu link problem, it surfaces here — that is the designed validation point (R2).
- Operator-run item covers the empty window shell contract from §5 plus Tasks 3/4 behaviors.
- Task 6 mirrors these decisions into `tech.md`/`architecture.md` (crate names, versions, WIC mechanism, module layout, backend choice).

## 9. Out-of-scope remainder

Nothing in this plan touches `Dockerfile`, `docker-compose.yml`, harness scripts, Task-1 docs, `.agent/project-info/*`, `README.md`, `CHANGELOG.md`. The 4.3/4.4/4.5b/4.6 cycle for Task 2 proceeds per the global plan §1 after this plan's 4.2 landing.
