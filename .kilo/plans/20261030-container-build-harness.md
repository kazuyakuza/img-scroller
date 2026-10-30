# Global Plan — Containerized Build Harness Planning Update

- Date: 2026-10-30
- Source TODO: [`.agent/todos/20261030/20261030-todo-1.md`](../.agent/todos/20261030/20261030-todo-1.md)
- Scope: PLANNING ONLY (second planning session; mirrors the approved 2026-10-29 planning-wave cycle pattern). No app code, no `Cargo.toml`, no Docker files are CREATED in this session — only the 7 wave TODO files are refined and docs updated. The harness itself (Dockerfile/compose/scripts/docs) is BUILT by Wave 1's future session.
- User decisions (2026-10-30, bound): [1] ALL harness content lands in Wave 1 (todo-3): Dockerfile, docker-compose.yml, scripts/dev-checks.sh, scripts/build-windows.sh, docs/build.md, docs/build-windows.md, .gitignore entries, Cargo.lock/--locked policy; [2] release targets x64 AND x86 (x86 realized in Wave 7's matrix unless Wave 1 delivers both); [3] Rust base-image pin decided by Wave 1's architector (reference: `rust:1.82`); [4] "manual UX checks" in all waves = OPERATOR-RUN on the real Windows machine (agent-side GUI execution is impossible from the container); [5] cargo commands across all waves execute via the container harness through the `alpine-vm` MCP (`vm_status` first; exit code authoritative; stderr truncation tolerated per reference notes).
- Verified reference (2026-10-30): `rust-youtube-streamer` README build sections + `docs/build.md` + `docs/build-windows.md` + Dockerfile + docker-compose.yml + scripts — mutually consistent; adapted paths for us: VM share `/img-scroller`, working dir `/img-scroller`, volume name for artifacts (new name, e.g. `images-scroller-target`), `dist/` root, NO FFmpeg in the image.
- Front-end related flags (4.1a/4.5a): Task 1 = NO; Task 2 = NO (doc/planning edits; wave-file CONTENT describes future front-end waves but is not itself front-end work).
- Grouping: Task 1 and Task 2 are separate cycles (different artifact types: wave TODO files = Planner-only markdown rule; docs = Docs Specialist).

## Steps Mapping (Steps 2–6)

### Step 2 — Git Feature Branch Setup => implementer

- Commit the two new planning artifacts (`.agent/todos/20261030/20261030-todo-1.md`, `.kilo/plans/20261030-container-build-harness.md`), message: `docs: add container build harness planning todo and global plan`.
- On `main` (clean) → create and switch to `feat/container-build-harness`.

### Step 3 — Version Update => N/A

- Still no version manifest (no `Cargo.toml` — Wave 1 creates it). Recorded as skipped.

### Task 1 — Wave TODO files refinement — cycle 4.1–4.6 (front-end: NO)

- 4.1b architector — spec plan `.kilo/plans/20261030-task1-wave-files-spec.md`: exact per-file edit itemization for todo-3…todo-9 (harness task set + wording swaps for verification tasks + execution-notes line + Wave 7 x64+x86 matrix + Wave 1 architector decision list additions: Linux-target check strategy/cfg-gating for platform code, pinned rust image, wasm of `--locked` policy). Must read the TODO (this request), global plan, current 7 wave files, and the verified reference docs (URLs recorded in the TODO file) — the spec must give exact BEFORE→AFTER bullet text per file. Banned: real crate names/versions, real keybindings content, changelog instructions.
- Planner (this agent) applies the edits to the 7 wave TODO files per the spec (markdown rule: TODO files are Planner-only).
- 4.3 code-reviewer — review edits vs spec → fix plan `.kilo/plans/20261030-task1-wave-files-fixes.md` if needed; code-simplifier: expected N/A (no code; recorded either way).
- 4.4 docs-specialist — N/A (agent-facing TODO artifacts; recorded).
- 4.5b architector — adherence report `.kilo/plans/20261030-task1-wave-files-adherence.md`.
- 4.6 implementer — mark Task 1 `[DONE]` in the TODO (heading + sub-items `[]`→`[x]`), commit wave-file edits: `docs: wave todo files adopt container build harness (task 1)`.

### Task 2 — Source-of-truth docs update — cycle 4.1–4.6 (front-end: NO)

- 4.1b architector — doc-edit plan `.kilo/plans/20261030-task2-build-env-docs.md`: exact BEFORE→AFTER for `tech.md` new "Build Environment (Container on the Alpine VM)" section, `brief.md` Technology-Stack build-environment line, `architecture.md` build/release pipeline note, `README.md` Build-and-Run container-context line; ban .agent/todos/**, AGENTS.md, historical .kilo/plans edits.
- 4.2 docs-specialist — executes per plan; commits per logical group (exact messages fixed in its plan).
- 4.3 code-reviewer — adherence review → fix plan `.kilo/plans/20261030-task2-build-env-fixes.md` if needed; code-simplifier: expected N/A.
- 4.4 docs-specialist — merged into 4.2/4.3 for this doc-only cycle (recorded).
- 4.5b architector — adherence report `.kilo/plans/20261030-task2-build-env-adherence.md`.
- 4.6 implementer — mark Task 2 `[DONE]`, commit pending artifacts (4.1b plan + review/adherence artifacts) with the 4.6 commit.

### Step 5 — TODO File Completion => implementer

- Commit context-log closing update (Task 2 lands it — see TODO; if still pending at Step 5, implementer commits `docs: update context log for build harness update`), rename todo-1 → `20261030-todo-1-DONE.md` (content unchanged), verify no tmp files, merge `feat/container-build-harness` → `main`, delete the branch after verified merge, push `main` to `origin` ONLY (standing decision from the 2026-10-29 session; never other remotes).

### Step 6 — Finish (Planner)

- Summary + next-session handoff text pointing to [`20261029-todo-3.md`](../.agent/todos/20261029/20261029-todo-3.md) (Wave 1 — now carrying the harness tasks).

## Risk/Consistency Notes

- Wave-file edits must preserve every pre-existing binding correction (Addendum v2 items) while adding the harness wording — the spec must enforce no-loss.
- `cargo test` inside the container CANNOT execute Windows-targeted test binaries; the spec must keep tests for pure logic on the Linux target with platform code cfg-gated (decision falls to Wave 1's architector, recorded in todo-3's deferred list).
- The Alpine VM MCP is available in the agents' environment; wave files must still instruct future sessions to `vm_status` first and STOP with a user question if the VM share is unavailable.

---

# Addendum — 2026-10-30 (placeholder resolution for Step 2)

The implementer's secrets gate flagged `VM_SSH_PASSWORD` = literal VM login password inside the working-copy `M .opencode/opencode.json` (alpine-vm MCP registration block; `VM_SSH_HOST/PORT/USER`, `VM_VBOX_NAME`, `VM_ALLOWED_PREFIXES` are non-secret). The caller (user) explicitly chose **"Commit as-is"** on 2026-10-30 — accepting the credential into permanent history (local dev VM, default password). Per this approval: commit `.opencode/opencode.json` unchanged on `main` (`chore: register alpine-vm MCP server in opencode config`), then the two planning artifacts (`docs: add container build harness planning todo and global plan`), then create and switch to `feat/container-build-harness`.
