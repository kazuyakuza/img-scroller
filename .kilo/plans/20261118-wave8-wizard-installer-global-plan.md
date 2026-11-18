# Wave 8 Global Plan — Wizard Installer, GitHub Releases, and UX Additions

- Source TODO: [`.agent/todos/20261117/20261117-todo-1.md`](../../.agent/todos/20261117/20261117-todo-1.md) (Pattern C — 7 tasks under `## Tasks`).
- Binding requirements source: the user's 2026-11-17 request (recorded verbatim in the TODO preamble).
- Branch (Step 2): `feat/wave-8-wizard-installer` — created from `main`; branch creation RESTRICTED to Step 2.
- Version bump (Step 3): `1.0.0` → `1.1.0` (minor — features; architector confirms in Task-1 4.1b). Version update RESTRICTED to Step 3.
- Git push RESTRICTED to workflow Step 5 (`main` → `origin` ONLY). The Task-7 GitHub RELEASE publish is a GitHub-API action governed by its own task, not a push.
- Front-end flags (for 4.1a/4.5a): Task 4 = FRONT-END (egui window title + fullscreen popup). Tasks 1, 2, 3, 5, 6, 7 = NOT front-end (installer technology / docs / verification / publishing — no egui `src/` work; Task 1's wizard UI is NSIS-script-owned, decided in its architector step).

## Current State Snapshot (verified 2026-11-18)

- `main` == `ea9dbee`, working tree clean, 5 commits ahead of `origin/main` (the 2026-11-17 planning-session commits — UNPUSHED, riding this wave's Step-5 push per established precedent).
- Version `1.0.0`; census 195 unit tests / 28 test-bearing files; dev-checks `ALL CHECKS PASSED`.
- Wave 7 landed: zip packaging (`scripts/package-windows.sh`, one flat 4-entry zip per architecture), the config file `packaging/img-scroller.toml`, the HKCU-only registration helper `packaging/register-default-viewer.cmd` + `packaging/DEFAULT-VIEWER-GUIDE.txt`.
- Build environment: ALL Rust/Cargo inside the container on the Alpine VM (`alpine-vm` MCP — `vm_status` FIRST, exit code authoritative; `/img-scroller` share; harness per [docs/build.md](../../docs/build.md)); the host needs NO Rust/Cargo. GUI/runtime validation is OPERATOR-RUN on real Windows 10/11.

## Global Pre-Analysis

- Task 7 is the FINAL task of the wave and runs ONLY after Tasks 1–6 are `[DONE]` (the TODO's binding order).
- The OS constraint (TODO preamble, binding): the installer REGISTERS the user-selected types HKCU-style (the Wave-7 helper registry surface: ProgID `ImagesScroller.Image`, per-extension OpenWithProgids, Capabilities/RegisteredApplications) and MAY open `ms-settings:defaultapps` for the final Settings confirmation; writing the hash-protected `UserChoice` key programmatically is FORBIDDEN (Windows 10/11 do not support it; the Wave-7 precedent governs).
- Installer technology candidates for the Task-1 architector: NSIS (`nsis`/`makensis` — the container-friendly Debian candidate; feasibility VERIFIED in the pinned image before deciding — MUI2 wizard pages, custom association-selection page with per-type checkboxes, HKCU registry writes, uninstall section) vs Inno Setup (Windows-only compiler — NOT container-friendly) vs WiX (.NET-heavy). Container cross-build is preferred so agent-side verification stays possible via the VM; a Windows-side build step would be operator-assisted only. Installer file name/versioning decided there (expected shape: `ImagesScroller-<version>-windows-installer.exe`, version read from `Cargo.toml`).
- Registry scope decision leaning (architector confirms): per-user HKCU — no elevation, consistent with the Wave-7 helper; association-selection page defaults: none preselected (recommended — the user explicitly picks, e.g. only `.png`); the Wave-7 zips stay produced (retire decision for the architector; recommended: keep both published — the installer supersedes, zips remain the portable alternative).
- Fate of the Wave-7 helper script: possibly retired from the installer payload (the installer's association page replaces it) — architector decides; the zips may still ship it.
- `gh` CLI availability/auth must be VERIFIED FIRST for Task 7 (host or VM); if agent-side publishing is impossible → STOP and ask the user (the user executes the publish or provides authorization).
- Every verification command runs through the `alpine-vm` MCP with `vm_status` first; exit code is the authoritative evidence; if the VM or the `/img-scroller` share is unavailable → STOP and return a user question.
- TODO file compliance: mark each task `[DONE]` at 4.6 (preserve content, `[]` → `[x]`); the Task-5 manual-UX sub-item (OPERATOR-RUN) STAYS `[ ]` until the user reports, per the standing precedent.

## Step Map (each entry = separate `task` tool invocation)

- Step 2: Git Feature Branch Setup → implementer (`feat/wave-8-wizard-installer` from `main`; commit any unstaged files; gitignore compliance).
- Step 3: Version Update → implementer (bump `Cargo.toml` to 1.1.0; commit `chore: bump version to 1.1.0`; `Cargo.lock` version line refresh as needed).
- Task 1: 4.1b Analysis & Planning (installer technology + build mechanism + all deferred decisions) → architector
- Task 1: 4.2 Implementation → implementer
- Task 1: 4.3 Code Review & Simplification → code-reviewer + code-simplifier; 4.3-fix → implementer
- Task 1: 4.4 Documentation → docs-specialist
- Task 1: 4.5b Overall Plan Adherence → architector
- Task 1: 4.6 Task Completion → implementer
- Task 2: 4.1b → architector → 4.2 implementer → 4.3 reviewer+simplifier → 4.4 docs-specialist → 4.5b architector → 4.6 implementer
- Task 3: 4.1b → architector → 4.2 implementer → 4.3 reviewer+simplifier → 4.4 docs-specialist → 4.5b architector → 4.6 implementer
- Task 4: 4.1a Front-end Spec → frontend-specialist; 4.1b → architector → 4.2 implementer → 4.3 reviewer+simplifier → 4.4 docs-specialist → 4.5a Front-end Verification → frontend-specialist; 4.5b → architector → 4.6 implementer
- Task 5: 4.1b → architector → 4.2 implementer (agent-side verification runs) → 4.3 reviewer+simplifier → 4.4 docs-specialist (verification results record) → 4.5b → architector → 4.6 implementer (manual-UX sub-item STAYS `[ ]`)
- Task 6: 4.1b → architector → 4.2 implementer → 4.3 reviewer+simplifier → 4.4 docs-specialist → 4.5b → architector → 4.6 implementer
- Task 7: 4.1b → architector (gh availability/auth verification FIRST) → 4.2 implementer → 4.3 reviewer+simplifier → 4.4 docs-specialist → 4.5b → architector → 4.6 implementer
- Step 5: TODO File Completion → implementer (rename `-DONE`; tmp sweep; merge `feat/wave-8-wizard-installer` → `main`; push `main` to `origin` ONLY; delete branch)

## Per-Task Pre-Analysis

### Task 1 — Executable wizard installer (NOT front-end)

Deliverable: a Windows installer executable with a wizard UI — destination-folder page; installs `img-scroller.exe` + config (+ additional files per architector decision); a file-association selection page (per-type checkboxes over the 17 supported types, e.g. only `.png`); the decided shortcut set; an uninstaller removing installed files, shortcuts, and the associations the install created. All deferred decisions (technology, build mechanism, file name/versioning, install scope, association defaults, registry surface, shortcuts, installed-file set, zips' fate, helper-script fate) settle in 4.1b. Registry surface reuses the Wave-7 helper mechanism; NEVER writes `UserChoice`. Expected new tracked files under an installer source location (e.g. `installer/` or `packaging/nsis/`) + a build script (e.g. `scripts/build-installer.sh`) + a docs guide; `.agent/project-structure.md` updated. Unit tests: none expected (installer script logic is not Rust; any pure seam the architector introduces would gain tests — recorded at 4.1b).

### Task 2 — README TOC + long-doc indexes (NOT front-end)

`README.md` gains a TOC near the top; VERIFY every >100-line doc file (`README.md`, `.agent/project-info/tech.md`, `architecture.md`, `context.md`, `docs/*.md`) has an initial index/TOC; add one where missing (`tech.md`/`architecture.md` carry `## Contents`; check `docs/build.md`, `docs/build-windows.md`, any other long doc). Doc-only; no code; census unchanged.

### Task 3 — Hotkeys list (verify, then complete) (NOT front-end)

VERIFY FIRST whether README/docs already list ALL program hotkeys and uses (the Controls table + per-feature text cover F11/ESC/I/O/0/W/H/arrows/Q as of 2026-11-17); if complete → record the verification verdict only; if gaps → add a consolidated hotkeys table (key → use) in one place. Doc-only.

### Task 4 — Window title shows the visible image's path (FRONT-END)

Behavior (TODO verbatim): the title shows the VISIBLE image's path, updated on every navigation; full path if it fits; else the image NAME + extension completely (last-chars-only if the name itself is too long); then — first the containing folder, second the drive — with `...` between drive and the visible folder chain; more parent folders as space allows. NOT maximized → ONLY the file name (+extension, last-chars-truncated). Fullscreen ON → on every visible-image change, a TOP-CENTER popup with name + extension for 3 seconds. Exact measurement/truncation/priority details (pixel measurement method, thresholds, ellipsis rendering, popup visuals) settle in 4.1b after the 4.1a front-end spec. Expected shape (architector confirms): new pure module `src/window_title.rs` (+ tests) for the composition/truncation logic (container-testable); a `src/app/` seam applying it (egui `ViewportCommand::Title`); a `src/app/` fullscreen popup overlay module (self-gated centered-top `Area`, `Popup`/Area pattern like the ESC-close popup, with an expiry wake — the Wave-5.1 `request_expiry_wake` precedent); census grows.

### Task 5 — Verification (NOT front-end)

Agent-side: dev-checks harness run (`ALL CHECKS PASSED` exit 0: fmt/check/test/clippy; census corroboration — the delta pinned at 4.1b); unit tests for the new pure logic (title composition/truncation from Task 4; installer/association pure logic where any exists); the installer build via the decided mechanism (exit 0 + non-empty artifact) and the zip build if retained; both through the `alpine-vm` MCP (`vm_status` first; exit code authoritative). Manual UX = OPERATOR-RUN checklist (plain form, handed to the caller; the sub-item STAYS `[ ]`): run the wizard installer → destination; association page picks types (e.g. only `.png`) and registration lands (Open With lists; Settings confirmation); app launches from the install location; Waves-1–7 behaviors still work from there; window title per Task 4; uninstaller removes files/shortcuts/associations; GitHub release visible and artifacts download.

### Task 6 — Documentation updates (NOT front-end)

`README.md`: installer-first installation instructions (replacing/adjusting the zip-extract flow), GitHub Releases pointer, hotkeys record, window-title behavior. `tech.md`/`architecture.md`/`context.md`: installer mechanism, releases flow, association-selection registry surface, title mechanism, build matrix. `brief.md`: the distribution model gains the user's 2026-11-17 wizard-installer decision as a BINDING addendum item. `CHANGELOG.md`: the 1.1.0 entry under `## [Unreleased]` released with the version bump (post-MVP entries allowed).

### Task 7 — GitHub Releases publishing (FINAL task; NOT front-end)

Publish a GitHub Release on `origin` tagged `v1.1.0` with the installer executable attached (+ zips if retained) and release notes from the `CHANGELOG.md` entry. Runs ONLY after Tasks 1–6 are `[DONE]`. The 4.1b architector step VERIFIES FIRST `gh` CLI availability/auth (host or VM; `gh auth status`); if agent-side publishing is impossible → STOP-and-ask the user (the user executes the publish or provides authorization). Publishing is a GitHub-API action — NOT a git push; pushing Git refs stays restricted to workflow Step 5 (`main` → `origin` ONLY).

## Error Handling

- On any sub-step failure: log details, commit safe changes if possible, notify the user, pause; resume/retry/split per the Critical Workflow; escalate after repeated failures.
- Max 3 review cycles in 4.3; escalate to the user beyond.
- VM/share unavailability → STOP and return a user question.
