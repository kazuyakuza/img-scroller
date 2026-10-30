# Task 2 Plan — Record the Build Environment in the Project Source-of-Truth Docs

- Date: 2026-10-30
- Source TODO: [`.agent/todos/20261030/20261030-todo-1.md`](../../.agent/todos/20261030/20261030-todo-1.md) → `### Task 2` ONLY (Task 1 is already committed as `b2d6a5b`).
- Parent plan: [`.kilo/plans/20261030-container-build-harness.md`](20261030-container-build-harness.md) → "Task 2 — Source-of-truth docs update" cycle (4.1–4.6).
- Critical Workflow step this plan serves: **4.2 Implementation via docs-specialist** (this plan file is the 4.1b architector artifact). Front-end flag: **NO** (doc-only; Wave-1 content referenced as future work, not front-end work).
- Executor: **docs-specialist**, junior-precision execution: every structural decision is encoded here; the executor does NOT choose wording, scope, or architecture.
- Branch: `feat/container-build-harness` (verify — §10). No branch creation, no push (Step 5 duty), no version update (Step 3 N/A — no manifest exists).
- Greenfield rule: every edit must keep "planned/not-yet-implemented" phrasing. NOTHING in this task creates harness files.

## Plan Amendment (2026-10-30)

This plan file itself (`.kilo/plans/20261030-task2-build-env-docs.md`) is expected to appear as an untracked file when the executor starts 4.2. This is ACCEPTED (caller decision 2026-10-30): the plan file is committed LATER by the workflow's step-4.6 wrap-up (matching the Task-1 precedent, where the 4.1b spec plan and review/adherence artifacts were committed together with the 4.6 commit). It is NOT part of the executor's commit set — the executor commits EXACTLY the two §9.3 commits and leaves this plan file untracked.

---

## 1. Single-Source Fact Block (do NOT invent anything beyond this)

All content inserted by this plan is sourced ONLY from:

- `.agent/todos/20261030/20261030-todo-1.md` → "Build Environment Definition (source: user, verified 2026-10-30)" block + Task 2 sub-items.
- `.kilo/plans/20261030-container-build-harness.md` → bound user decisions + Task 2 step description.
- `.agent/todos/20261029/20261029-todo-3.md` → Wave 1 Task 1 (where the harness files land).
- Verified `.gitignore` state (checked 2026-10-30): repo root `.gitignore` ALREADY contains `dist/` (line 31), `logs/` (line 18), `*.log` (line 17). It does NOT yet contain `target/` (Wave 1 adds it).
- Reference attribution: pattern from the project `kazuyakuza/rust-youtube-streamer` (README build sections, `docs/build.md`, `docs/build-windows.md`, Dockerfile, `docker-compose.yml`, `scripts/*.sh`), verified mutually consistent 2026-10-30; that project pinned `rust:1.82` as its base image — cited as exemplar ONLY, no version adopted in this project.

`docs/build.md` and `docs/build-windows.md` do NOT exist yet (verified `docs/` = `how-to-set-up-git.md`, `how-to-write-todo-files.md` only). They are NEVER presented as existing files — always "to be created in Wave 1" phrasing, inline code only (no markdown links to them).

## 2. Scope Boundaries

Files the executor MAY modify (exactly four):

1. `.agent/project-info/tech.md`
2. `.agent/project-info/brief.md`
3. `.agent/project-info/architecture.md`
4. `README.md`

File the executor MUST NOT touch, with an explicit SKIP note (§8): `.agent/project-info/context.md`.

Nothing else: no `.agent/todos/**`, no `AGENTS.md`, no `.kilo/plans/*` edits, no `CHANGELOG.md`, no `docs/` files, no code/config files. New plan files in `.kilo/plans/` are NOT created by the executor (fix/adherence plans belong to 4.3/4.5b agents).

## 3. Pre-Flight Checks (STOP if any fails)

Run these two commands first (bash tool, single commands):

1. `git branch --show-current` → MUST print exactly `feat/container-build-harness`. Else STOP (§10).
2. `git status` → MUST report a clean TRACKED tree (no modified or staged tracked files) on `feat/container-build-harness`; the ONLY tolerated untracked entry is this plan file (`.kilo/plans/20261030-task2-build-env-docs.md`) — see the "Plan Amendment (2026-10-30)" note at the top. Any OTHER untracked or dirty entry → STOP (§10.2).

3. BEFORE-drift check: for each of the four edits below (§4–§7), the quoted BEFORE text must still match the live file exactly (read each file first). Any mismatch = drifted BEFORE → STOP (§10).

## 4. Edit A — `.agent/project-info/tech.md`: new "Build Environment (Container on the Alpine VM)" section

### 4.1 Placement anchor (exact)

Insert the new section BETWEEN the end of "## Development Environment and Setup (Placeholder)" and the "## Build and Run (Placeholder)" heading — i.e., immediately after the bullet:

`- Never install tools globally — in-project installs only (rule: [`.kilo/rules/never-global-installs.md`](../../.kilo/rules/never-global-installs.md")).`

and before the line `## Build and Run (Placeholder)`.

### 4.2 BEFORE (verbatim window, lines 31–35 of the current file)

```text
- Never install tools globally — in-project installs only (rule: [`.kilo/rules/never-global-installs.md`](../../.kilo/rules/never-global-installs.md)).

## Build and Run (Placeholder)
```

### 4.3 AFTER (insert the block below in the gap; the placeholder lines stay unchanged)

```markdown
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
```

Rules: the two placeholder sections (`## Development Environment and Setup (Placeholder)`, `## Build and Run (Placeholder)`) remain byte-identical — they are filled by Wave 1 Task 6, not now. No changes elsewhere in `tech.md`.

## 5. Edit B — `.agent/project-info/brief.md`: one build-environment decision line in `## Technology Stack`

### 5.1 Placement anchor (exact)

`## Technology Stack` ends with the bullet `- Target architectures: x64 and x86 builds.` (line 100, unique in the file) followed by the blank line before `## Out of Scope for v1 (Deferred)`.

### 5.2 BEFORE (verbatim window, lines 100–102)

```text
- Target architectures: x64 and x86 builds.

## Out of Scope for v1 (Deferred)
```

### 5.3 AFTER (one new bullet line inserted after the architectures bullet)

```text
- Target architectures: x64 and x86 builds.
- Build environment: builds and checks run inside a Linux container on the Alpine VM via Docker (Docker exists only on the VM; the host needs no Rust/Cargo); the release Windows `.exe` is cross-compiled from that container (x86_64-pc-windows-gnu); Windows targets confirmed x64 AND x86 (x86 realized in Wave 7 packaging) — decision date 2026-10-30.

## Out of Scope for v1 (Deferred)
```

Rules: exactly ONE added line. The section pre-amble ("These choices are confirmed decisions but not yet implemented") stays unchanged — the new line is consistent with it (decision recorded, nothing implemented). No other sections of `brief.md` are touched.

## 6. Edit C — `.agent/project-info/architecture.md`: new planned note "Build and Release Pipeline (Planned)"

### 6.1 Placement anchor (exact)

Insert the new section AFTER "## Data and State (Planned)" (whose last bullet is `- No other persistent state in v1; explicitly no persistence of the last-viewed memory.`) and BEFORE the `## Project Paths` heading.

### 6.2 BEFORE (verbatim window, lines 36–38)

```text
- No other persistent state in v1; explicitly no persistence of the last-viewed memory.

## Project Paths
```

### 6.3 AFTER (insert the block below in the gap)

```markdown
## Build and Release Pipeline (Planned)

Decision recorded 2026-10-30: all build/check activity runs inside a Linux container on the Alpine VM (details: [tech.md](tech.md) → `## Build Environment (Container on the Alpine VM)`); the pipeline exists only as a plan — no pipeline is implemented yet — and its files land in Wave 1.

- Dev checks (planned): a `scripts/dev-checks.sh` harness script runs inside the container — in order `cargo fmt --check`, `cargo check --locked`, `cargo test --locked`, `cargo clippy --locked -- -D warnings` — emitting per-check PASS/FAIL lines and a final `ALL CHECKS PASSED` summary, with a UTC-timestamped log under the gitignored `logs/` (`logs/checks/`).
- Windows release cross-build (planned): a `scripts/build-windows.sh` harness script runs `cargo build --release --locked --target x86_64-pc-windows-gnu` inside the container and copies the final `.exe` into the gitignored `dist/`; the x86 release matrix is realized in Wave 7 (consuming/extending this script and adding the i686 MinGW toolchain when the x86 target is added).
- Validation stance: container-only — all agent-side build/check validation happens inside the container; Windows runtime/GUI validation is OPERATOR-RUN on the real Windows 10/11 machine (the Linux container cannot execute the Windows GUI application).
- Not implemented: Dockerfile, `docker-compose.yml`, both scripts, and the build docs are created in Wave 1; until then no build system exists.
```

Rules: no changes to the `## Technical Decisions Summary` table, `## Open Architectural Decisions`, or any other section of `architecture.md` (the table is fed by brief sections only; the build-env line in the brief did not add a table requirement).

## 7. Edit D — `README.md`: extend the "Build and Run (Not Yet Implemented)" placeholder with container context

### 7.1 Placement anchor (exact)

Within `## Build and Run (Not Yet Implemented)` (lines 57–59), AFTER the existing placeholder paragraph (the paragraph starting `This section is a placeholder: nothing is implemented yet...`), BEFORE `## Note for AI Agents`.

### 7.2 BEFORE (verbatim section, lines 57–59)

```markdown
## Build and Run (Not Yet Implemented)

This section is a placeholder: nothing is implemented yet and no build system exists, so there is nothing to build or run at the moment. Once the first implementation wave creates the Rust crate, a standard Cargo workflow will apply once `Cargo.toml` exists, and the steps will be documented here and in [.agent/project-info/tech.md](.agent/project-info/tech.md). No runnable commands are provided until then.
```

### 7.3 AFTER (the existing paragraph stays byte-identical; one new paragraph appended in the same section)

```markdown
This section is a placeholder: nothing is implemented yet and no build system exists, so there is nothing to build or run at the moment. Once the first implementation wave creates the Rust crate, a standard Cargo workflow will apply once `Cargo.toml` exists, and the steps will be documented here and in [.agent/project-info/tech.md](.agent/project-info/tech.md). No runnable commands are provided until then.

One build decision is already recorded (2026-10-30): all cargo/build commands will run INSIDE a Linux container on the Alpine VM (accessed through the `alpine-vm` MCP) — the host machine needs no Rust or Cargo installed. The dedicated build documentation (dev-checks through the container and the Windows `.exe` cross-build) arrives with the first implementation wave: `docs/build.md` and `docs/build-windows.md` (both to be created in Wave 1).
```

Rules: the WAVE reference ("first implementation wave") stays generic — this file does not cite the Wave 1 TODO file; `docs/build.md`/`docs/build-windows.md` appear as inline code plus the explicit "(both to be created in Wave 1)" qualifier, NEVER as markdown links. No other README changes.

## 8. Edit E — `.agent/project-info/context.md`: EXPLICIT SKIP (execution note)

**DO NOT EDIT `.agent/project-info/context.md`.** The TODO lists it as the "critical closing step", and the parent plan routes that duty to the Critical Workflow **Step 5** implementer (context-log closing commit, message `docs: update context log for build harness update`). The docs-specialist executor performs NO context.md edit and NO TODO-file edit (the `[DONE]` marking is step 4.6). If the executor believes context.md needs updating inside 4.2, it must STOP and report instead of editing (§10).

## 9. Execution Order + Commit Commands + Verification

### 9.1 Execution order

1. Pre-flight (§3).
2. Read all four files fully; apply Edit A → B → C → D with the `edit` tool (built-in structured edit tool; NOT shell commands) in that order.
3. Verification grep sweep (9.2) BEFORE committing.
4. Commit 1 → verification re-check → Commit 2 (9.3).
5. Final status check (9.4) and completion report (§13).

### 9.2 Pre-commit verification grep anchors (use the built-in grep tool; each uses the repo root as search path)

- `tech.md`: contains exactly one `## Build Environment (Container on the Alpine VM)` heading; contains `Alpine` (≥1), `/img-scroller` (≥2), `alpine-vm` (≥1), `vm_status` (≥1), `--locked` (≥1), `Cargo.lock` (≥1), `dist/` (≥1), `logs/` (≥1), `rust:1.82` (exactly 1), `kazuyakuza/rust-youtube-streamer` (≥1), `2026-10-30` (≥1); still contains both unchanged `Placeholder` headings.
- `brief.md`: contains `Alpine VM` (exactly 1 new occurrence in the added line), `2026-10-30` (exactly 1), `Linux container` (exactly 1); `## Out of Scope` heading still intact.
- `architecture.md`: contains `## Build and Release Pipeline (Planned)` (exactly 1 heading), `Alpine VM` (≥1), `dev-checks.sh` (exactly 1), `build-windows.sh` (exactly 1), `x86_64-pc-windows-gnu` (exactly 1), `dist/` (≥1), `logs/` (≥1), `Wave 7` (exactly 1), `OPERATOR-RUN` (exactly 1).
- `README.md`: contains `alpine-vm` (exactly 1), `Linux container` (exactly 1), `to be created in Wave 1` (exactly 1), `docs/build.md` + `docs/build-windows.md` (1 each, as inline code — zero markdown links to them).
- Whole tree: `docs/build.md` and `docs/build-windows.md` must still NOT exist (glob check).
- Diff-scope check: `git diff --name-only` → exactly the four files: `README.md`, `.agent/project-info/architecture.md`, `.agent/project-info/brief.md`, `.agent/project-info/tech.md` — nothing else. `.agent/project-info/context.md` must NOT appear in the diff.

### 9.3 Commits (exact messages; bash tool, single git commands, in order)

After the pre-commit sweep passes, stage and commit per logical group:

1. Group 1 — project-info docs:
   - `git add .agent/project-info/tech.md`
   - `git add .agent/project-info/brief.md`
   - `git add .agent/project-info/architecture.md`
   - `git commit -m "docs: record container build environment in project info docs"`
2. Group 2 — README:
   - `git add README.md`
   - `git commit -m "docs: note alpine-vm container build context in README"`

Gitignore Compliance Rule: before each commit run `git status` and confirm NO `.gitignore`-matching files are staged (node_modules-style dirs don't exist here; still verify no `dist/`, `logs/`, `*.log`, `*.tmp` files sneak in). If any appear, unstage them and report.

### 9.4 Post-commit verification

- `git status` → clean TRACKED tree on `feat/container-build-harness`; the ONLY tolerated untracked extra remains this plan file (see the "Plan Amendment (2026-10-30)" note at the top).
- `git log --oneline -3` → shows the two new commit messages from §9.3 on top of `b2d6a5b`.
- Repeat the `git diff --name-only` style check expecting an empty working-tree diff (clean).

## 10. STOP Conditions (return a question to the caller — never guess)

1. Branch ≠ `feat/container-build-harness` (§3.1) → STOP: report the actual branch; do NOT create or switch branches.
2. Working tree not clean at start (§3.2) → STOP: report status; do NOT commit unrelated leftovers. EXCEPTION (caller decision 2026-10-30, see the "Plan Amendment (2026-10-30)" note at the top): a clean TRACKED tree with exactly ONE untracked entry — this plan file (`.kilo/plans/20261030-task2-build-env-docs.md`) — is ACCEPTED and is NOT a stop condition; it is committed later by step 4.6 and is NOT part of the executor's commit set. Any other untracked or dirty entry remains a STOP.
3. ANY BEFORE text from §4.2/§5.2/§6.2/§7.2 does not match the live file (drift) → STOP: report which file/window drifted; do NOT adapt the edit.
4. The verified `.gitignore` fact fails: `.gitignore` does NOT already contain `dist/`, `logs/`, and `*.log` → STOP: report; the tech.md sentence "already contains" must not be reworded autonomously.
5. Any instruction here is ambiguous or the executor is asked (by anyone/anything) to edit `context.md`, `.agent/todos/**`, `AGENTS.md`, `.kilo/plans/*`, `CHANGELOG.md`, or any code/config file → STOP per §8/§12 and report.
6. Any verification anchor in §9.2 fails after an edit → STOP before committing: report the failing anchor and revert the non-conforming edit by fixing only the wording THIS plan specified (if not fixable deterministically, return the question).

## 11. Documentation Tone + Consistency Requirements (apply during edits)

- Every new statement stays in planned status phrasing: "planned", "decision recorded 2026-10-30", "nothing is implemented yet", "lands in Wave 1". Match each file's existing tone (brief = decision bullets; tech/architecture = sections with the Status convention; README = plain paragraphs).
- Wave references: allowed as generic "Wave 1"/"Wave 7" phrasing (they already appear across the repo's planning docs); README uses only generic "first implementation wave" wording per §7.3.
- Real versions/crates: ONLY `rust:1.82` may appear, flagged as the reference project's exemplar (not adopted). No crate names, no keybindings, no toolchain versions beyond that, no new decisions invented.

## 12. Hard Bans (§12 — never do these)

- NO edits to `.agent/todos/**` (no `[DONE]` marking — that is step 4.6, a separate implementer sub-task).
- NO edits to `AGENTS.md`, `.agent/WORKFLOWS.md`, `.agent/rules/**`, `.kilo/rules/**`.
- NO edits to historical `.kilo/plans/*` files (only this 4.1b plan file was created by the architector; the executor creates NO plan files).
- NO `CHANGELOG.md` edits — nothing changelog-related exists in this task (and MVP-checkpoint rule defers all changelog writes to Wave 7).
- NO Cargo/code/manifest changes, no `Cargo.toml`, no Docker files, no `scripts/`, no `.gitignore` edits, no `docs/` file creation.
- NO branch creation/switch, NO `git push` (Step 5 duty), NO merging, NO TODO renames, NO version bumps.
- NO new plan/report files (4.3 fix plan and 4.5b adherence report are produced by those agents, not the executor).

## 13. Completion Report Contract

Signal completion with ALL of:

- `[Project Info: Active]` confirmation.
- The commit hashes + messages created (must be exactly the two from §9.3).
- The grep-anchor verification results (each item pass/fail).
- Explicit "what was NOT done": no `context.md` edit (Step 5 duty), no TODO `[DONE]` marking (4.6), no harness files created, no push/branch/merge.

With that, 4.2 hands over to the 4.3 code-reviewer adherence review (next step — NOT executed by the executor).
