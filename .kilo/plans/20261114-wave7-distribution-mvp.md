# Wave 7 Global Plan — Distribution, Packaging, MVP Checkpoint

- Source TODO: `.agent/todos/20261029/20261029-todo-9.md` (Pattern C: `# Title` → `## Tasks` → six `###` tasks; each task gets its own 4.1–4.6 cycle).
- Date: 2026-11-14. Status: PENDING USER APPROVAL.
- Requirements source of truth: `.agent/project-info/brief.md` (`## Distribution and Default App Registration`, `## Technology Stack`, `## Out of Scope for v1`, addenda). Binding wave corrections (Addendum v2 item 2): MVP checkpoint = END of this wave; this is the ONLY wave allowed to touch `CHANGELOG.md`, in its dedicated final task (Task 6) only after Tasks 1–5 are `[DONE]`.

## Global Pre-Analysis

### Environment facts (verified 2026-11-14)

- Git: on `main`, `main` == `origin/main` == `0ec23e6`; version 0.6.2; working tree has exactly 3 unstaged files — `.agent/project-info/context.md`, `.agent/project-info/future-ideas.md`, `.agent/todos/20261113/20261113-todo-2-DONE.md` (the Wave-6.2 gate-closure recording, which rides this wave's Step-2 commit per context.md).
- Alpine VM: RUNNING, SSH reachable (verified via `vm_status` 2026-11-14). All Rust/Cargo/Docker work goes through the container harness via the `alpine-vm` MCP (`vm_status` first; exit code is the authoritative evidence). If the VM or the `/img-scroller` share becomes unavailable, STOP and return a user question.
- Harness: `Dockerfile` (pinned `rust:1.99.1-bookworm`, MinGW-w64 x86-64, target `x86_64-pc-windows-gnu`), `docker-compose.yml`, `scripts/dev-checks.sh` (fmt/check/test/clippy → `ALL CHECKS PASSED`), `scripts/build-windows.sh` (x64-only today; artifacts at gitignored `dist/windows-x64/img-scroller.exe`).
- Census pinned: 195 unit tests / 28 test-bearing files; DLL import audit SET-EQUAL 22 (recorded in `scripts/build-windows.sh` + `docs/build-windows.md` — never duplicated elsewhere).
- `CHANGELOG.md`: Keep-a-Changelog skeleton with `## [Unreleased]` + empty subsections and the banner "No entries yet — changelog updates begin only after the MVP version (full v1) exists."
- `.gitignore` already covers `dist/`, `logs/`, `target/`, `*.log` — packaging artifacts must NEVER be committed.

### Wave-level decisions (binding for every task; user approves with this plan)

1. **Branch**: `feat/wave-7-distribution` created in Step 2 from `main` (branch creation restricted to Step 2; `git push` restricted to Step 5, `main` → `origin` ONLY).
2. **Version bump (Step 3): 0.6.2 → 1.0.0** (major). Justification: the user's 2026-10-29 decision fixes MVP = end of Wave 7 = full v1; the `CHANGELOG.md` banner says entries begin at the MVP version (full v1); the wave's Task 6 writes the first entry summarizing **v1.0.0** content. A minor bump to 0.7.0 would contradict all three. Committed as `chore: bump version to 1.0.0`. Package/zip naming uses 1.0.0.
3. **Front-end determination**: NO task of Wave 7 is front-end related. Every task runs 4.1b only (no 4.1a frontend-spec) and 4.5b only (no 4.5a frontend verification). Rationale: the app's GUI is untouched; all Wave-7 deliverables are packaging/build artifacts, a config file, and a non-GUI helper. Standing constraint for Task 3's architector: the default-viewer helper must NOT add a new GUI surface to the app or the package (a GUI helper is out of v1 scope unless the user explicitly requests it).
4. **Process order**: tasks in file order — Task 1 (config) → Task 2 (installer/packaging) → Task 3 (helper) → Task 4 (verification) → Task 5 (docs) → Task 6 (MVP changelog, strictly last) → Step 5. Each task: one 4.1b → 4.2 → 4.3 → 4.4 → 4.5b → 4.6 cycle. Commit per step with meaningful messages.
5. **Packaging dependency sequencing**: the zip's content manifest (binary + config + helper/guidance) must be a single source in the packaging script so Task 3 can extend it with the helper files without re-architecting Task 2's script.
6. **Deferred decisions** (the wave requires these settled in the per-task architector steps, NOT pre-decided here): config contents/format/location (Task 1 4.1b); packaging mechanism/tooling, zip layout/naming, one-zip-vs-two, x86 script-extension details, build matrix (Task 2 4.1b); helper mechanism (Task 3 4.1b); CHANGELOG entry wording (written at execution in Task 6).
7. **House rules**: never edit existing `.kilo/plans/*` files (new plan files only); Gitignore Compliance Rule before every commit; `.agent/project-structure.md` must be updated when new tracked files/folders land; manual-UX (OPERATOR-RUN) sub-items stay `[ ]` — they are the user's acceptance gate, never agent-failed.

## Step Map (sub-agent assignments)

- **Step 2: Git Feature Branch Setup** => implementer — commit the 3 unstaged gate-closure files on `main` first (e.g. `docs: record wave 6.2 gate closure and idea 8`), then verify on `main`, create and switch to `feat/wave-7-distribution`.
- **Step 3: Version Update** => implementer — bump `Cargo.toml` to 1.0.0 (decision #2), refresh `Cargo.lock` version line via the harness if needed, commit `chore: bump version to 1.0.0`.
- **Task 1: Config file** => 4.1b architector → 4.2 implementer → 4.3 code-reviewer + code-simplifier → 4.4 docs-specialist → 4.5b architector → 4.6 implementer.
- **Task 2: Zip-style portable installer** => same cycle (architector → implementer → reviewers → docs → architector → implementer).
- **Task 3: Default-viewer registration helper/guidance** => same cycle.
- **Task 4: Verification** => same cycle (manual-UX sub-item stays `[ ]` for the user).
- **Task 5: Documentation updates** => same cycle.
- **Task 6: MVP checkpoint — first `CHANGELOG.md` entry (FINAL)** => same cycle; runs ONLY after Tasks 1–5 are all `[DONE]`.
- **Step 5: TODO File Completion** => implementer — rename the TODO to `20261029-todo-9-DONE.md`, tmp sweep, verify all committed, checkout `main`, fast-forward merge `feat/wave-7-distribution`, delete the branch after verified merge, push `main` → `origin` ONLY.
- **Step 6: Finish** => planner — summary + next-TODO handoff text.

## Task 1 — Config file (existence + format decision)

- TODO text: introduce the application config file shipped alongside the binary; contents/format decided in the architector step; contents minimal and consistent with architecture.md's "On disk" note; NO last-viewed persistence (v1 constraint — session memory stays RAM-only).
- Front-end related: NO.
- Pre-analysis for 4.1b (`.kilo/plans/20261114-wave7-task1-config-plan.md`): the config is a TRACKED source file (not under gitignored `dist/`) that the packaging step copies into the package (flat, next to the exe). Decision space: format (TOML / INI-style / plain text — plain Rust, no dependency if possible); contents (minimal placeholder consistent with "no settings exist in v1" and future-ideas' parked settings place — e.g. header comments + zero or one documented key); whether the v1 binary parses it (leaning: NO — reading a file with no settings adds dead code and a failure mode; if the architector decides a trivial read, it must fail safe: missing/unreadable config never blocks the viewer). App behavior must not change. Expected: no test census change unless a pure function is added.
- 4.6 commit pattern: marks the task `[DONE]` (preserve file content, `[]` → `[x]`).

## Task 2 — Zip-style portable installer

- TODO text: the user picks a destination folder; the package unzips the application binary AND the config file there. Release builds for x64 AND x86 — consume AND extend Wave 1's `scripts/build-windows.sh`: the x86 build is an extension of the Wave-1 harness (`i686-pc-windows-gnu` target + `gcc-mingw-w64-i686` toolchain addition to the tracked Dockerfile); the exact extension mechanism is decided in the architector step. The packaging mechanism (script vs program, archive tooling) is decided in the architector step; packaging consumes the produced x64/x86 `.exe` artifacts from the gitignored `dist/` locations.
- Front-end related: NO.
- Pre-analysis for 4.1b (`.kilo/plans/20261114-wave7-task2-installer-packaging-plan.md`): build extension = Dockerfile gains `gcc-mingw-w64-i686` + `rustup target add i686-pc-windows-gnu` (+ archive tooling if chosen, e.g. `zip`), image rebuild required; `scripts/build-windows.sh` extended to cover both targets (parameterized/looped — architector decides; keep the typed-failure prefix discipline and the atomic install pattern); x86 artifact at a mirrored gitignored location (leaning `dist/windows-i686/img-scroller.exe`, mirroring the target triple as `windows-x64` mirrors `x86_64`); packaging = a NEW script (e.g. `scripts/package-windows.sh`) consuming `dist/` artifacts + the Task-1 config into zip file(s) under gitignored `dist/` (leaning: one zip per architecture — a 32-bit user cannot run the 64-bit exe; names like `ImagesScroller-1.0.0-windows-x64.zip` / `-x86.zip`; flat layout: exe + config at zip root; helper/guidance included once Task 3 lands — see sequencing decision #5). "The user picks a destination" = the standard Windows zip-extract destination flow (no self-extracting exe in v1). Zip artifacts are gitignored (`dist/`) — NEVER committed.
- 4.2 implementation includes a smoke x86 build through the harness (exit 0, non-empty artifact) to validate the extension; the formal dual-architecture verification runs are Task 4's.

## Task 3 — Default-viewer registration helper/guidance

- TODO text: the package includes a helper and/or guidance that helps the user register the app as the default image viewer (file associations) in Windows; the helper mechanism (script vs executable, exact mechanism) is decided in the architector step — this wave file must not pre-select it.
- Front-end related: NO (standing constraint: no GUI helper — see Wave-level decision #3).
- Pre-analysis for 4.1b (`.kilo/plans/20261114-wave7-task3-viewer-registration-plan.md`): Windows 10/11 mediates default-app selection through Settings (programmatic default assignment is deprecated) — the realistic mechanism space: (a) a packaged script (PowerShell/.cmd) that registers the app's ProgID + extension association under HKCU pointing at the installed exe path and opens the Settings default-apps page; (b) a small console-mode executable (second build target — more scope); (c) guidance-only (a packaged README/instructions file + README section). The architector picks ONE and encodes everything (file names, exact registry keys/values with the `%1` argument passing, how the installed path is resolved at registration time, the guidance text, and the packaging-manifest extension per decision #5). The helper runs ONLY on Windows (operator territory) — agent-side verification is static review; real registration is validated by the Task-4 manual-UX OPERATOR-RUN.
- 4.4 docs-specialist: the guidance text inside the package + the README section may land with this task or be consolidated in Task 5 (architector decides; avoid duplicate maintenance).

## Task 4 — Verification

- TODO text: (1) unit tests for any pure packaging/config logic that exists — run via the dev-checks harness in the container per `docs/build.md` (`ALL CHECKS PASSED`; `alpine-vm` MCP; `vm_status` first; exit code authoritative); (2) x64 AND x86 release builds via the extended Wave-1 build script inside the container (per Task 2) — each command exits 0 and leaves a non-empty artifact at the gitignored `dist/` locations; (3) manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): installer flow on a clean destination dir → binary + config file present and app launches; default-viewer helper/guidance performs or guides correctly; packaged app navigations from Waves 1–6 still work from the installed location.
- Front-end related: NO.
- Pre-analysis for 4.1b (`.kilo/plans/20261114-wave7-task4-verification-plan.md`): (1) dev-checks run at wave HEAD — census re-pinned (195 + any new pure-logic tests; if Tasks 1–3 added none, the census stays 195); (2) the two release builds + the packaging runs recorded with exit codes and artifact byte sizes; DLL audit re-check for the NEW x86 artifact (the x86 import list is OBSERVED and recorded — expect the same 22 system DLLs; any delta adjudicated before proceeding); (3) the OPERATOR-RUN checklist written PLAIN and self-contained in the plan §, handed to the caller; the manual-UX sub-item stays `[ ]` — the user's gate (same pattern as every prior wave).

## Task 5 — Documentation updates

- TODO text: `README.md`: distribution/installation/registration instructions. `tech.md`, `architecture.md`, `context.md`: record config format, packaging mechanism, helper mechanism, build matrix.
- Front-end related: NO.
- Pre-analysis for 4.1b (`.kilo/plans/20261114-wave7-task5-documentation-plan.md`): README gains the distribution/install/registration section (user-facing); tech.md gains the packaging/build-matrix + config-format records (the "Distribution and Packaging (Planned)" section becomes implemented); architecture.md updates the pipeline section + decisions table + "On disk" note; context.md gets the wave entry + current-state/next-steps refresh; coherence sweep: `.agent/project-structure.md` (new tracked files), `docs/build-windows.md` (x86 + packaging record — build guide), `docs/build.md` only if harness semantics changed. Never edit existing `.kilo/plans/*`.

## Task 6 — MVP checkpoint: first `CHANGELOG.md` entry (FINAL task)

- TODO text: per user decision 2026-10-29, MVP = end of Wave 7 = full v1; this is the ONLY wave allowed to write the first `CHANGELOG.md` entry, only after all prior tasks are `[DONE]`; entry in the existing Keep-a-Changelog structure (SemVer); summarizes v1.0.0 content (the 7 features + distribution); actual entry text written by the implementing session, NOT pre-drafted.
- Front-end related: NO.
- Pre-analysis for 4.1b (`.kilo/plans/20261114-wave7-task6-changelog-mvp-plan.md`): placement = `## [Unreleased]` stays on top, the new `## [1.0.0] - <release date>` section follows it (Keep-a-Changelog newest-first); subsections filled only where content exists (Added for the 7 features + distribution; empty subsections may stay or be trimmed per the structure decision — architector encodes); wording drafted at execution from the landed tech.md/architecture.md facts; version headline 1.0.0 matches the Step-3 bump. The `> No entries yet` banner is removed/superseded by this entry (that edit is part of THIS task's authorization). NOTHING else in the repo may touch `CHANGELOG.md`.
- GATE: 4.2 implementer MUST verify Tasks 1–5 are `[DONE]` in the TODO file before editing `CHANGELOG.md`; if any is not, STOP and return a question.

## Error Handling / Risks

- VM/share unavailable at any point → STOP, return a user question (wave execution note).
- x86 toolchain/image-rebuild failures → typed failures via the script's `build-windows: error:` discipline; pause and ask the user before improvising.
- DLL-audit delta on the x86 artifact → adjudicate explicitly before proceeding (expect set-equal 22; any new MinGW runtime DLL means the build is not portable — fix before packaging).
- Any plan ambiguity encountered by a sub-agent → STOP and return the question to the Planner; never guess.
- Review-cycle cap: max 3 (4.3) cycles, then escalate to the user.
