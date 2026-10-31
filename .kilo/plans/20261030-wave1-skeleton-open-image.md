# Global Plan — Wave 1: Project Skeleton and Open Image

- Date: 2026-10-30
- Executor: Planner Agent (opencode, model glm-5.3-flash) via the Critical Workflow
- TODO file (source): [`.agent/todos/20261029/20261029-todo-3.md`](../../.agent/todos/20261029/20261029-todo-3.md) — pattern C: `## Tasks` → 6 `###` tasks
- Approvals: NOT auto-approved (string "Don't request me to approve plans" absent) → user approval required (global + per-task)

## 0. Global Pre-Analysis

### 0.1 Repository status
- Greenfield: NO `Cargo.toml`, no application code. `src/` holds only a `.gitkeep` placeholder.
- Docs/config scaffolding complete; `.gitignore` root already has `dist/`, `logs/`, `*.log` (verify, don't duplicate; add `target/` only).
- Git: working tree clean, on `main` @ `4306729`, up to date with `origin`.

### 0.2 Binding build environment (from context log / tech.md — apply to EVERY task)
- ALL Rust/Cargo execution happens inside a Linux Docker container on the Alpine VM (host needs NO Rust/Cargo).
- Access via `alpine-vm` MCP: call `vm_status` BEFORE any command; the command exit code is the authoritative success signal (cargo stderr may be absent in captured MCP output — not a failure signal).
- Project shared on the VM at `/img-scroller`; Docker + Compose plugin exist ONLY on the VM.
- Commands run as `docker compose -f /img-scroller/docker-compose.yml run --rm rust …` (Task 1 creates these files; before Task 1 lands there is NO harness).
- If VM or `/img-scroller` share is unavailable → STOP and return a user question.
- Validation stance: container-only. Windows GUI/runtime validation is OPERATOR-RUN on the real Windows machine — agent-impossible items are recorded as operator-pending, never failed.
- Reference pattern (user-approved): `kazuyakuza/rust-youtube-streamer` (Dockerfile/docker-compose.yml/scripts + docs/build*.md). Adapt: NO FFmpeg; NEVER copy reference names (volume, dist exe naming) or its Rust pin.

### 0.3 Deferred decisions — settled in the Wave-1 architector step (4.1b, Task 2) unless noted
1. Exact pinned official Rust base image tag (not `1.82` inheritance; architector decides and records it in the plan).
2. WIC binding mechanism/crates (no crate names anywhere before 4.1b; `image` crate is INELIGIBLE — no-bundled-decoders constraint).
3. `src/` module layout (first structure).
4. GUI crate selection/versions (egui/eframe via `cargo add`; pins decided in 4.1b).
5. Open-dialog mechanism (native Windows common dialog approach).
6. Unsupported-file message wording.
7. Unit-test strategy for OS-bound WIC logic vs pure logic (tests must stay container-runnable).
8. Linux-target check strategy for Windows-only platform code: cfg-gating vs windows-target-only check build.
9. Named volume name (build artifacts; never the reference's).
10. `dist/` subfolder + `.exe` naming (never the reference's).
11. Cargo package/binary naming (crate name → binary name → `.exe` name).
12. FORCE_FAIL hook: RETAINED AS-IS (`fmt-check|check|test|clippy` → synthetic exit 7) — nothing to decide.

### 0.4 Front-end related marking (pending user confirmation with plan approval)
- Tasks 2, 3, 4 involve the egui/eframe desktop GUI.
- Planner determination: egui/eframe is Rust-native immediate-mode desktop UI; the frontend-specialist agent is scoped to web stacks (Angular/Vue/TS/CSS + npm/npx tooling) which cannot run in this Rust-only project. → Marked **NOT front-end related**: NO 4.1a / 4.5a; the architector (4.1b) owns all GUI structural decisions, 4.5b verifies.
- User is asked to confirm/override this classification with the global-plan approval question.

### 0.5 Step 3 — Version Update
- No version exists before Task 2 (no `package.json`, no `Cargo.toml`) → Step 3 recorded as **N/A** for the run, before Task 2. In Task 2's plan the initial crate version is set (semver initial `0.1.0`); no bump of a pre-existing version happens this wave.

### 0.6 Project-structure maintenance
- `.agent/project-structure.md` is updated when folders appear in `src/` (Update Workflow: automatic). Branches into Task 2 implementation (implementer updates the file) and is re-verified in Task 6.

### 0.7 Constraints spanning all tasks
- Branch creation restricted to Step 2 (`feat/wave-1-skeleton-open-image` per TODO execution notes); `git push` restricted to Step 5 (`origin` ONLY).
- `.gitignore` compliance before every commit; commit per step with meaningful messages.
- `Cargo.lock` tracked; every harness cargo invocation uses `--locked` (policy; lands in Task 1 docs/scripts and is enforced from Task 2 onward).
- Do NOT touch `CHANGELOG.md`; do NOT edit existing `.kilo/plans/*` files (new plan files only; naming `.kilo/plans/<YYYYMMDD>-<name>`).
- Newline Prevention, Max-Lines/Depth/Args, private-members, self-documenting-code rules apply to all `src/` code (200 lines/file, ≤50 lines/method body per rule; boilerplate/exceptions encoded in task plans).
- TODO file sub-item checkboxes: `[]` → `[x]` at each task's 4.6 (preserve all other content verbatim).

## 1. Execution Sequence (each line = separate `task` tool invocation)

- Step 2: Git Feature Branch Setup → implementer  (create `feat/wave-1-skeleton-open-image` from `main`; tree already clean — nothing to commit)
- Step 3: Version Update → implementer  (N/A this wave — recorded, no version exists; no crate created yet)
- Task 1: 4.1b Analysis & Plan → architector → user approval of per-task plan (unless "Approve Global and Tasks Plans" chosen)
- Task 1: 4.2 Implementation → implementer  (harness files land; image built ONCE on the VM so toolchain is ready)
- Task 1: 4.3 Review & Simplification → code-reviewer + code-simplifier → 4.3-fix → implementer
- Task 1: 4.4 Documentation → docs-specialist  (docs/build.md + docs/build-windows.md already written in 4.2 per TODO; 4.4 reviews/polishes if needed)
- Task 1: 4.5b Overall Plan Adherence → architector
- Task 1: 4.6 Task Completion → implementer  (`[DONE]` marks, commit)
- Task 2: 4.1b Analysis & Plan → architector → user approval  (settles ALL `## 0.3` deferred decisions for the wave)
- Task 2: 4.2 Implementation → implementer  (Cargo.toml, src/ scaffold, window shell `Images Scroller`; harness first EXERCISED end-to-end here per TODO ordering note; project-structure.md updated)
- Task 2: 4.3 → 4.4 → 4.5b → 4.6 (same agents as Task 1 pattern)
- Task 3: 4.1b → 4.2 → 4.3 → 4.4 → 4.5b → 4.6  (CLI-arg + open dialog + WIC decode + unsupported-file message)
- Task 4: 4.1b → 4.2 → 4.3 → 4.4 → 4.5b → 4.6  (render decoded image, fit-scale, resize-sane)
- Task 5: 4.1b → 4.2 → 4.3 → 4.4 → 4.5b → 4.6  (dev-checks via MCP harness `ALL CHECKS PASSED`; unit tests; release cross-build exits 0 with non-empty `.exe` in `dist/`; OPERATOR-RUN manual UX item = recorded as operator-pending, NOT executed by agents)
- Task 6: 4.1b → 4.2 → 4.3 → 4.4 → 4.5b → 4.6  (README, tech.md, architecture.md updates; context.md handled by Planner as the session closing step)
- Step 5: TODO File Completion → implementer  (rename TODO `…-todo-3.md` → `…-todo-3-DONE.md`; tmp-file sweep; commit; merge to `main`; delete branch; push main to `origin` ONLY)

## 2. Per-Task Pre-Analysis (technical & architecture decisions to reach in each 4.1b)

### Task 1 — Container build harness (Docker on the Alpine VM)
- Deliverables (files land verbatim per TODO): `Dockerfile` (pinned official Rust image [decided here], `rustup component add rustfmt clippy`, `gcc-mingw-w64-x86-64`, `rustup target add x86_64-pc-windows-gnu`, NO FFmpeg), `docker-compose.yml` (single `rust` service; bind mount `/img-scroller:/img-scroller`; `working_dir: /img-scroller`; named volume + `CARGO_TARGET_DIR` into it — drop-in `volumes:` mapping), `scripts/dev-checks.sh`, `scripts/build-windows.sh`, `docs/build.md`, `docs/build-windows.md`; `.gitignore` gains exactly `target/`.
- `dev-checks.sh` semantics (from TODO, architector expands to exact code): ordered fmt --check → check --locked → test --locked → clippy --locked -- -D warnings; per-check `PASS|FAIL <name> exit=<code> (<duration> s)`; final `ALL CHECKS PASSED` / `N CHECK(S) FAILED`; UTC log under `logs/checks/` (gitignored); exit 0/1; FORCE_FAIL hook retained (exact check fails synthetic exit 7); never modifies sources / never applies fmt.
- `build-windows.sh` semantics: `cargo build --release --locked --target x86_64-pc-windows-gnu`; assert cwd == `/img-scroller` and `CARGO_TARGET_DIR` set; typed failures ALL prefixed `build-windows: error:` → deterministic exit 1 (classes: wrong cwd; unset/empty CARGO_TARGET_DIR; cargo failure incl. lockfile/manifest mismatch; expected exe not found/empty; `dist/` mkdir failure; copy failure; artifact missing/empty post-copy); cargo diagnostics passed through; failed run leaves `dist/` untouched.
- Import audit: script does NOT run it; script's success note + `docs/build-windows.md` carry placeholders for the OBSERVED DLL list (filled once by implementing session in Task 5 via `x86_64-w64-mingw32-objdump` on OUR `.exe`; reference DLL list never copied).
- Architector decisions here: exact Rust base-image pin; named volume name; `dist/` subfolder + exe naming; cargo package/binary naming (single decision chain used by later tasks).
- Exception decoded for plan: docs limited to Task-1 scope (build docs), NOT the Task-6 project-info updates.
- Verification in 4.5b compares landed files against this plan + TODO sub-items (image build exercised via MCP `vm_status` → docker compose build; harness runs deferred to Task 5 per TODO ordering note).

### Task 2 — Cargo project scaffold and window shell
- Architector settles ALL `## 0.3` deferred items (this is the wave's decisions core): crate name (→ binary name, e.g. `img-scroller`-family name consistent with repo — exact value decided), egui/eframe crates + versions via `cargo add` (exact trio pinned), WIC binding crates, module layout (first `src/` structure, small files per Max-Lines rules), dialog approach, message wording, unit-test strategy, Linux-target check strategy (recommendation candidate: cfg-gated platform modules so container `cargo check/test` stay runnable for pure logic).
- Implementation: `Cargo.toml` at repo root (version 0.1.0, edition pinned); remove `src/.gitkeep`; minimal eframe `main()` + app struct launching a regular resizable window titled `Images Scroller` (not maximized, not modal); empty-window only — no decoding yet.
- Harness first EXERCISED (TODO ordering note): `dev-checks.sh` runs end-to-end via the `alpine-vm` MCP here (fmt/check/test/clippy on the scaffold) — `cargo test` with no tests yet is exercised; image build already done in Task 1.
- `.agent/project-structure.md` updated automatically (folders appear in `src/`).
- 4.4: minimal module-level doc comments (self-documenting code rule — code first).

### Task 3 — Open Image (CLI argument and in-app open dialog)
- Layers (architector decides split): argument intake (CLI image path) → dialog open path (native Windows common dialog, cfg-gated) → WIC decode invocation (no bundled decoders) → unsupported/unloadable handling (informational message in window; wording pinned in 4.1b).
- Pure-logic units kept container-testable (path classification/listing helpers if introduced); OS-bound units cfg-gated per Task-2 strategy.
- Cross-build compile check via container (`cargo check`/`build-windows.sh` NOT required here — release build verified once in Task 5; container dev-checks required).

### Task 4 — Display the image
- Render decoded image inside the window; initial fit = simple whole-image fit (zoom modes are Wave 4 — do NOT implement); resize keeps display sane (no cropped/clipped render beyond fit scaling).
- egui upload of decoded pixels; scaling math as pure helper (container-testable) — architector decides integration point.

### Task 5 — Verification
- All agent-runnable items through the `alpine-vm` MCP (`vm_status` first; exit code authoritative):
  1. `scripts/dev-checks.sh` → final `ALL CHECKS PASSED` (fmt/check/test/clippy).
  2. `cargo test --locked` covers the container-runnable unit tests defined by architector decisions (Task 2; any Task 3/4 additions).
  3. `scripts/build-windows.sh` exits 0; non-empty `.exe` at the decided `dist/` location; once: `x86_64-w64-mingw32-objdump` import audit on OUR exe → record OBSERVED DLL list in script success-note + `docs/build-windows.md` (retain-scope item from Task 1).
- OPERATOR-RUN manual UX check (real Windows): CLI-arg open (jpg/png), dialog open, unregistered-format message, resize sanity — recorded as operator-pending; agents never execute it.

### Task 6 — Documentation updates
- `README.md`: replace build/run placeholder with container-harness commands (per `docs/build.md`, VM/`docker compose` usage, no host Rust) + Windows cross-build pointer (`docs/build-windows.md`) + "any system-registered format" constraint.
- `tech.md`: fill `## Development Environment and Setup (Placeholder)`, `## Build and Run (Placeholder)`, `## Testing (Placeholder)` with what landed (container toolchain, harness usage, test suite status); record WIC mechanism + chosen dependencies with versions.
- `architecture.md`: replace "planned/no code" status lines invalidated by this wave (module layout, WIC mechanism, build pipeline implemented); keep truly-undecided entries open.
- `context.md`: Planner closes the session (Critical Closing Step) with this wave's outcome — docs-specialist updates the other three files; final context.md write by Planner during finish.

## 3. Risks / Failure Handling
- VM/share unreachable → STOP, user question (binding).
- Container image build failure (e.g., MinGW apt package resolution, rustup target add) → retry logged; if persistent → pause + user.
- Cross-build cargo errors (manifest/lock mismatch) → typed failure surfaces as `build-windows: error:`; investigate lockfile state before retry.
- egui/eframe API drift vs newer pinned versions → architector pins exact versions and encodes API usage in the plan; implementer never upgrades independently.
- Max 3 review cycles per task (4.3 hard limit); beyond → escalate to user.
