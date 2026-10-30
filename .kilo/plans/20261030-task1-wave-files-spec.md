# Spec Plan — Task 1: Update the 7 Wave TODO Files for the Container Build Harness

- Date: 2026-10-30
- Task: Task 1 of [`.agent/todos/20261030/20261030-todo-1.md`](../../.agent/todos/20261030/20261030-todo-1.md) ONLY. Task 2 is NOT part of this spec.
- Author: architector (Critical Workflow 4.1b). Executor: the Planner Agent (markdown rule: TODO files are Planner-editable). Reviewer (4.3): code-reviewer.
- Session type: PLANNING-ONLY — this spec refines MARKDOWN TODO FILES only. No code, no `Cargo.toml`, no Docker files, no VM commands in this cycle. The harness itself is BUILT later by Wave 1's own implementing session.
- Target files (exactly 7): `.agent/todos/20261029/20261029-todo-3.md` … `20261029-todo-9.md`.

## 0. Binding Context

- User decisions bound in the TODO Task 1 + global plan [`.kilo/plans/20261030-container-build-harness.md`](20261030-container-build-harness.md) (Addendum incl. `VM_SSH_PASSWORD` acceptance recorded): ALL harness content lands in Wave 1; release targets x64 AND x86; Rust base-image pin decided by Wave 1's architector; manual UX checks = OPERATOR-RUN on the real Windows machine; all cargo commands across all waves run via the container harness through the `alpine-vm` MCP (`vm_status` first; exit code authoritative).
- Verified reference pattern (user-approved, verified 2026-10-30): `rust-youtube-streamer` — README `Build Checks (Docker via Alpine VM)` + `Release Build (Windows .exe)`, `docs/build.md`, `docs/build-windows.md`, `Dockerfile`, `docker-compose.yml`, `scripts/dev-checks.sh`, `scripts/build-windows.sh`.
- Ground truth verified 2026-10-30 in THIS repo: `.gitignore` ALREADY contains `dist/` and `logs/`/`*.log`. There is NO `target/` entry and no `Cargo.lock`-related entry.

## 1. Structural Decisions Encoded by This Spec (the Planner MUST NOT re-decide them)

1. **Wave-1 harness task POSITION = new Task 1** (the wave's FIRST task). Existing Wave-1 tasks renumber 2–6. Rationale (deterministic): the scaffold task requires `cargo build` evidence and Tasks 2–4 build/test through the container — the harness must exist before they execute; any placement after the scaffold creates an ordering contradiction. The caller constraint "before the verification task + renumber subsequent tasks" is satisfied. Final Wave-1 order: 1 = Container build harness (new) → 2 = Cargo project scaffold and window shell → 3 = Open Image — CLI argument and in-app open dialog → 4 = Display the image → 5 = Verification → 6 = Documentation updates.
2. **`.gitignore` entries (closed list)**: keep existing `dist/` and `logs/` (already present — do NOT duplicate); add EXACTLY one new entry: `target/`.
3. **`docs/build.md` scope**: dev-checks + Alpine-VM-usage + MCP-usage guide for THIS project — NOT a Linux-release-build guide (we ship NO Linux artifact; the reference's `docs/build.md` covers a Linux release build; that content is NOT copied).
4. **`docs/build-windows.md` scope**: x86_64-pc-windows-gnu cross-build guide only (i686/x86 is Wave 7 scope).
5. **Import audit scope: RETAINED (not trimmed)** — decided here per reference behavior. Adaptation: the reference script hardcodes the DLL list observed for ITS OWN app; our import set will differ. The Wave-1 implementing session runs the `x86_64-w64-mingw32-objdump` import audit once on OUR produced `.exe` and records the OBSERVED DLL list in the script's success-note line and in `docs/build-windows.md`. The reference's DLL list (`msvcrt.dll, kernel32.dll, …`) is NEVER copied verbatim into our files.
6. **FORCE_FAIL hook: RETAINED AS-IS** (settable to `fmt-check`|`check`|`test`|`clippy` → exactly that check fails with synthetic exit 7; unset in normal use). Recorded in Wave 1's deferred list as "RETAINED AS-IS — nothing further to decide".
7. **Bans maintained**: no real crate names/versions; no registry dates; Rust pin stays "decided in Wave 1's architector step"; named volume name, `dist/` subfolder + `.exe` naming, Cargo binary/package naming all stay architector-decided and are NOT fixed in the wave files.
8. **Wave-1 internal ordering**: the harness task lands files/docs; the harness commands are first exercised end-to-end in the renumbered Verification task once the Cargo scaffold exists (stated in the new task's final bullet).

## 2. Global Edit Rules

- Apply edits EXACTLY: BEFORE blocks are current verbatim text (verified against the files 2026-10-30); AFTER/INSERT blocks are required verbatim text.
- Reworded items keep the `- [ ]` checkbox state (only the wave's own 4.6 step flips to `[x]`).
- Representation note: in table cells and inline paragraphs of this spec, `\`` denotes a markdown-escaped LITERAL BACKTICK — when applying an edit, write a real backtick, never a backslash. Text inside fenced code blocks is literal as-is (no escaping inside them).
- The Planner must verify after all edits: no crate names/versions or registry dates; no keybinding/zoom/UI content beyond what exists; NO `CHANGELOG.md` instructions added anywhere; Waves 2–6 content outside the specified edits stays byte-identical (Addendum v2 correction texts NOT touched); Pattern C structure valid (`###` = one task); the bottom `<!-- DO NOT DELETE ... -->` block untouched in every file.

## 3. Shared New Text (insert exactly once per file)

### 3.1 "Future session execution notes" — Alpine-VM prerequisites bullet (INSERT)

Anchor (exists verbatim in all 7 files): the FIRST bullet of the `## Future session execution notes` section (shown fenced, fully literal):

```markdown
- Process this wave standalone in a future session via the Critical Workflow ([`.kilo/commands/critical-workflow.md`](../../../.kilo/commands/critical-workflow.md)); process tasks ONE 4.x cycle at a time.
```

Insert the new bullet immediately AFTER it.

Variant S1 (into todo-4 … todo-9 / Waves 2–7):

```markdown
- Alpine-VM build environment (applies to ALL tasks of this wave): the Alpine VM is running with the project shared at `/img-scroller`; Docker + the Compose plugin are available ONLY on the VM; the host machine needs NO Rust/Cargo; commands run through the `alpine-vm` MCP — call `vm_status` first (the command exit code is the authoritative success evidence); harness usage per `docs/build.md` (created in Wave 1); if the VM or the `/img-scroller` share is unavailable, STOP and return a user question.
```

Variant S2 (into todo-3 / Wave 1 — last middle clause differs):

```markdown
- Alpine-VM build environment (applies to ALL tasks of this wave): the Alpine VM is running with the project shared at `/img-scroller`; Docker + the Compose plugin are available ONLY on the VM; the host machine needs NO Rust/Cargo; commands run through the `alpine-vm` MCP — call `vm_status` first (the command exit code is the authoritative success evidence); harness usage per `docs/build.md` (created by Task 1 of this wave); if the VM or the `/img-scroller` share is unavailable, STOP and return a user question.
```

### 3.2 Shared verification bullets (Waves 2–7)

Harness dev-checks bullet (used per-file where the spec says so):

```markdown
- [ ] Dev checks via the container harness per `docs/build.md` (`ALL CHECKS PASSED`: fmt/check/test/clippy; run through the `alpine-vm` MCP — call `vm_status` first; the command exit code is the authoritative success evidence).
```

Operator-run prefix (prepended to each file's existing manual-UX text, which is preserved verbatim):

```markdown
- [ ] Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): 
```

Unit-test run-mode suffix (appended to existing unit-test bullets where the spec says so):

```markdown
 — run via the dev-checks harness in the container per `docs/build.md`
```

## 4. File A — `.agent/todos/20261029/20261029-todo-3.md` (Wave 1)

### A.EDIT-1 — Rename the 5 existing task headings (5 separate exact edits)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `### Task 1: Cargo project scaffold and window shell` | `### Task 2: Cargo project scaffold and window shell` |
| `### Task 2: Open Image — CLI argument and in-app open dialog` | `### Task 3: Open Image — CLI argument and in-app open dialog` |
| `### Task 3: Display the image` | `### Task 4: Display the image` |
| `### Task 4: Verification` | `### Task 5: Verification` |
| `### Task 5: Documentation updates` | `### Task 6: Documentation updates` |

(Each old heading occurs exactly once in the file.)

### A.EDIT-2 — INSERT the new harness Task 1

Anchor: immediately AFTER the exact line `## Tasks` (line 9), BEFORE the renamed `### Task 2: Cargo project scaffold and window shell`. Insert verbatim:

```markdown
### Task 1: Container build harness (Docker on the Alpine VM)

- [ ] `Dockerfile` (adapted from the verified reference pattern): pinned official Rust base image (the exact pin is decided in this wave's architector step — this TODO file must NOT record it); `rustup component add rustfmt clippy`; `gcc-mingw-w64-x86-64` (the MinGW-w64 x64 GNU cross toolchain: `gcc` as the linker route, `objdump` available for the Windows binary import audit); `rustup target add x86_64-pc-windows-gnu`; NO FFmpeg (the reference installs FFmpeg — this project does NOT need it and the adaptation drops it).
- [ ] `docker-compose.yml` (adapted): single `rust` service; bind mount `/img-scroller:/img-scroller`; `working_dir: /img-scroller`; a named volume for build artifacts with `CARGO_TARGET_DIR` pointing into it (volume name decided in this wave's architector step — do NOT fix it here); no `target/` leak into the repository root (intermediate Cargo output lives inside the volume); the only artifact generated in the root is the tracked `Cargo.lock`.
- [ ] `scripts/dev-checks.sh` (POSIX sh; adapted): runs in order `cargo fmt --check`, `cargo check --locked`, `cargo test --locked`, `cargo clippy --locked -- -D warnings`; emits a per-check `PASS|FAIL <name> exit=<code> (<duration> s)` line plus a final `ALL CHECKS PASSED` / `N CHECK(S) FAILED` summary; writes a UTC-timestamped log under the gitignored `logs/checks/` (path printed); exit code `0` when all checks pass / `1` when any fails; retains the optional `FORCE_FAIL` fault-injection hook (set to `fmt-check`|`check`|`test`|`clippy` → exactly that check fails with synthetic exit 7); never modifies source files and never applies `cargo fmt` fixes.
- [ ] `scripts/build-windows.sh` (POSIX sh; adapted): runs `cargo build --release --locked --target x86_64-pc-windows-gnu` inside the container; asserts cwd == `/img-scroller` and `CARGO_TARGET_DIR` set before building; copies ONLY the final `.exe` into the gitignored `dist/` subfolder — the subfolder and `.exe` naming are decided in this wave's architector step (do NOT copy the reference's names); typed failure pattern: every failure line starts with the literal prefix `build-windows: error:` and the command exits `1` deterministically (failure classes mirrored from the reference: wrong cwd; unset/empty `CARGO_TARGET_DIR`; cargo build failure incl. lockfile/manifest mismatch; expected release executable not found or empty; `dist/` directory-create failure; copy failure; final artifact missing or empty after the copy); cargo diagnostics passed through, never suppressed; a failed run leaves `dist/` untouched (previous artifact stays in place; nothing is deleted).
- [ ] Import-audit retained scope (DECIDED in this spec — RETAINED, not trimmed): the script does not itself run the audit but still reports the GNU-runtime static-linking evidence; the Wave-1 implementing session runs the `x86_64-w64-mingw32-objdump` import audit ONCE on OUR produced `.exe` and records the OBSERVED DLL list in the script's success-note line and in `docs/build-windows.md`; the reference project's DLL list is NEVER copied verbatim (our import set differs).
- [ ] `docs/build.md` (new; adapted from the verified reference doc with our paths/names): dev checks + usage through the Alpine VM — build the image once (`docker compose -f /img-scroller/docker-compose.yml build rust`), run the harness (`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`), per-check fallback commands, `FORCE_FAIL` usage, UTC log location, expected output semantics; `alpine-vm` MCP usage notes (`vm_status` first; the command exit code is the authoritative success evidence; cargo stderr may not appear in captured MCP output); container-only validation stance + operator-run Windows validation; explicitly NOT a Linux-release-build guide (this project ships no Linux artifact).
- [ ] `docs/build-windows.md` (new; adapted from the verified reference doc with our paths/names): x86_64-pc-windows-gnu cross-build guide — purpose, prerequisites (image build incl. the MinGW-w64 toolchain), the `docker compose … sh scripts/build-windows.sh` invocation, expected output path (the `dist/` subfolder per the architector decision), overwrite/rebuild behavior, the typed `build-windows: error:` failure table, toolchain/target notes, the GNU-runtime static-link/import-audit note (OBSERVED list per the retained-scope item above), and the container-only validation stance + operator-run Windows validation; ONLY 64-bit Windows is covered here (x86 belongs to Wave 7 if realized).
- [ ] `.gitignore`: the repo root `.gitignore` ALREADY contains `dist/`, `logs/`, `*.log` — verify those entries remain (do NOT duplicate them) and add EXACTLY one new entry: `target/` (the closed list needed so container/cargo artifacts never leak; with the named-volume redirect this is defensive).
- [ ] Policy: `Cargo.lock` is tracked in git; EVERY harness cargo invocation uses `--locked`.
- [ ] Alpine-VM/MCP operational notes (landed in `docs/build.md`): Alpine VM running with the project shared at `/img-scroller`; Docker + Compose plugin ONLY on the VM; the host needs NO Rust/Cargo; agents access the VM through the `alpine-vm` MCP — call `vm_status` first, then run commands; the command exit code is the authoritative success evidence; if the VM or the share is unavailable, STOP and return a user question.
- [ ] Ordering note: these harness commands are first EXERCISED end-to-end in Task 5 (Verification) once the Cargo project exists (Task 2); in THIS task the files/docs land and the image is built once so the toolchain is in place.
```

### A.EDIT-3 — Reword the scaffold build bullet (renamed Task 2)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] \`cargo build\` succeeds; \`cargo run\` shows the empty window.` | `- [ ] The scaffold builds through the container harness per \`docs/build.md\` (dev-checks apply once the Cargo manifest exists); the empty-window GUI run is validated in Task 5's operator-run item — the Linux container cannot run the Windows GUI.` |

### A.EDIT-4 — Replace the Verification task body (renamed Task 5; 3 old bullets → 4 new bullets)

BEFORE (exact, current lines 31–33):

```markdown
- [ ] `cargo build` and `cargo run` succeed on a Windows 10/11 machine.
- [ ] `cargo test` passes (add first unit tests only if this wave's architector step defines a test suite; a minimal smoke test of the decoding-selection logic is acceptable — the architector decides what is unit-testable given WIC is an OS-boundary).
- [ ] Manual UX check (during this wave's manual-run verification): open jpg/png from CLI argument and from dialog; try a file with a format not registered (e.g., an intentionally obscure extension) → message shown; window resize keeps the image visible.
```

AFTER (exact, 4 bullets):

```markdown
- [ ] Container dev-checks pass via the harness per `docs/build.md` (`ALL CHECKS PASSED`: fmt/check/test/clippy; run through the `alpine-vm` MCP — call `vm_status` first; the command exit code is the authoritative success evidence).
- [ ] `cargo test --locked` inside the container covers the unit tests the architector defined (add first unit tests only if this wave's architector step defines a test suite; a minimal smoke test of the decoding-selection logic is acceptable — the architector decides what is unit-testable given WIC is an OS-boundary; tests must remain runnable in the container for pure logic).
- [ ] The Windows release cross-build (`scripts/build-windows.sh`) exits `0` and leaves a non-empty `.exe` at the gitignored `dist/` location (subfolder/exe naming per the architector decision).
- [ ] Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): the operator runs the produced `.exe`; open jpg/png from CLI argument and from dialog; try a file with a format not registered (e.g., an intentionally obscure extension) → message shown; window resize keeps the image visible.
```

### A.EDIT-5 — Reword the README bullet of the Documentation task (renamed Task 6)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] \`README.md\`: replace the build/run placeholder with real commands (\`cargo run\`, release build), plus the "any system-registered format" constraint.` | `- [ ] \`README.md\`: replace the build/run placeholder with the container-harness build commands (build/validate through the Alpine VM per \`docs/build.md\` — no host Rust/Cargo needed; Windows release cross-build per \`docs/build-windows.md\`), plus the "any system-registered format" constraint.` |

### A.EDIT-6 — Reword the "Wave 1 scope notes" line

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `Wave 1 scope notes: WIC binding approach and src/ module layout are DEFERRED to this wave's architector step. Build/run documentation updates are part of Task 5.` | `Wave 1 scope notes: WIC binding approach and src/ module layout are DEFERRED to this wave's architector step. Harness documentation (\`docs/build.md\`, \`docs/build-windows.md\`) is part of Task 1; build/run documentation updates are part of Task 6.` |

### A.EDIT-7 — Deferred-decisions list: INSERT 6 bullets after the last existing deferred bullet

Anchor: immediately AFTER the exact line `- Whether/how unit tests can cover OS-bound WIC logic vs pure logic.` (line 51). Insert verbatim:

```markdown
- Exact pinned Rust base image (official rust base image; the tag is decided in this wave's architector step — never inherited from the reference and not recorded in this file).
- Linux-target check strategy for Windows-only platform code: cfg-gating platform modules vs windows-target-only checks — unit tests must remain runnable in the container for pure logic.
- Named volume name for build artifacts (`docker-compose.yml`; never the reference's volume name).
- `dist/` subfolder + exe naming (never the reference's names).
- Cargo binary/package naming (crate name → binary name → produced `.exe` name).
- FORCE_FAIL hook scope — RETAINED AS-IS (fmt-check|check|test|clippy → synthetic exit 7; nothing further to decide).
```

### A.EDIT-8 — Future session execution notes: INSERT bullet (§3.1, Variant S2).

### A.EDIT-9 — Everything else in todo-3 stays untouched (all Task 3/4 feature bullets, requirements-source block, bottom DO-NOT-DELETE block).

## 5. File B — `.agent/todos/20261029/20261029-todo-4.md` (Wave 2)

### B.EDIT-1 — Verification Task 4 unit-test bullet

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Unit tests (test suite exists since Wave 1): listing enumeration (folder-only scope), alphanumeric sort correctness, previous/next index stepping, first/last edge behavior.` | `- [ ] Unit tests (test suite exists since Wave 1): listing enumeration (folder-only scope), alphanumeric sort correctness, previous/next index stepping, first/last edge behavior — run via the dev-checks harness in the container per \`docs/build.md\`.` |

### B.EDIT-2 — Verification Task 4 combined cargo/UX bullet → TWO bullets

BEFORE (exact, line 31):

```markdown
- [ ] `cargo build`, `cargo test`, `cargo run`; manual UX check: keys and buttons move prev/next; nested subfolders are NOT entered; order matches the decided collation.
```

AFTER (exact, two bullets):

```markdown
- [ ] Dev checks via the container harness per `docs/build.md` (`ALL CHECKS PASSED`: fmt/check/test/clippy; run through the `alpine-vm` MCP — call `vm_status` first; the command exit code is the authoritative success evidence).
- [ ] Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): keys and buttons move prev/next; nested subfolders are NOT entered; order matches the decided collation.
```

### B.EDIT-3 — Future session execution notes: INSERT bullet (§3.1, Variant S1).

Nothing else in this file changes (all other bullets/corrections and the deferred list remain verbatim).

## 6. File C — `.agent/todos/20261029/20261029-todo-5.md` (Wave 3)

### C.EDIT-1 — Verification Task 4 unit-test bullet

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Unit tests: same-parent restriction (no grandparent/deeper jumps), sort order, target-image priority (visited → last-viewed; new folder → first image), last-viewed map set/get and per-session lifetime.` | `- [ ] Unit tests: same-parent restriction (no grandparent/deeper jumps), sort order, target-image priority (visited → last-viewed; new folder → first image), last-viewed map set/get and per-session lifetime — run via the dev-checks harness in the container per \`docs/build.md\`.` |

### C.EDIT-2 — Verification Task 4 combined cargo/UX bullet → TWO bullets

BEFORE (exact, line 34):

```markdown
- [ ] `cargo build`, `cargo test`, `cargo run`; manual UX check: up/down keys and buttons move folders under the same parent; revisiting a folder this session reopens its last viewed image; closing and relaunching loses the memory.
```

AFTER (exact, two bullets):

```markdown
- [ ] Dev checks via the container harness per `docs/build.md` (`ALL CHECKS PASSED`: fmt/check/test/clippy; run through the `alpine-vm` MCP — call `vm_status` first; the command exit code is the authoritative success evidence).
- [ ] Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): up/down keys and buttons move folders under the same parent; revisiting a folder this session reopens its last viewed image; closing and relaunching loses the memory.
```

### C.EDIT-3 — Future session execution notes: INSERT bullet (§3.1, Variant S1).

Nothing else changes (the Addendum v2 correction texts — the binding NO-same-filename rule in Task 3 — stay verbatim).

## 7. File D — `.agent/todos/20261029/20261029-todo-6.md` (Wave 4)

### D.EDIT-1 — Verification Task 4 unit-test bullet (absorbs the cargo commands)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Unit tests where pure logic exists (zoom-step math, fit-scale computation). \`cargo build\`, \`cargo test\`, \`cargo run\`.` | `- [ ] Unit tests where pure logic exists (zoom-step math, fit-scale computation) — run via the dev-checks harness in the container per \`docs/build.md\` (\`ALL CHECKS PASSED\`: fmt/check/test/clippy; \`alpine-vm\` MCP; \`vm_status\` first; the command exit code is the authoritative success evidence).` |

### D.EDIT-2 — Verification Task 4 manual UX bullet (prefix only; UX text preserved verbatim)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Manual UX check: launch never opens fullscreen; toggling ON hides the header and all controls leaving only the image; OFF restores controls with the image central; fit width/height computations correct on differently shaped images.` | `- [ ] Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): launch never opens fullscreen; toggling ON hides the header and all controls leaving only the image; OFF restores controls with the image central; fit width/height computations correct on differently shaped images.` |

### D.EDIT-3 — Future session execution notes: INSERT bullet (§3.1, Variant S1).

Nothing else changes.

## 8. File E — `.agent/todos/20261029/20261029-todo-7.md` (Wave 5)

### E.EDIT-1 — Verification Task 4 unit-test bullet (absorbs the cargo commands)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Unit tests: overflow detection per axis, edge detection state machine (pan position vs image bounds), navigation trigger conditions. \`cargo build\`, \`cargo test\`, \`cargo run\`.` | `- [ ] Unit tests: overflow detection per axis, edge detection state machine (pan position vs image bounds), navigation trigger conditions — run via the dev-checks harness in the container per \`docs/build.md\` (\`ALL CHECKS PASSED\`: fmt/check/test/clippy; \`alpine-vm\` MCP; \`vm_status\` first; the command exit code is the authoritative success evidence).` |

### E.EDIT-2 — Verification Task 4 manual UX bullet (prefix only)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Manual UX check: zoom wide → keys pan, navigation only at edge; zoom tall → same vertically; small image → direct navigation.` | `- [ ] Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): zoom wide → keys pan, navigation only at edge; zoom tall → same vertically; small image → direct navigation.` |

### E.EDIT-3 — Future session execution notes: INSERT bullet (§3.1, Variant S1).

Nothing else changes.

## 9. File F — `.agent/todos/20261029/20261029-todo-8.md` (Wave 6)

### F.EDIT-1 — Verification Task 4 unit-test bullet (absorbs the cargo commands)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Unit tests where pure logic exists (e.g., parsing/filtering of enumeration results represented in a testable seam). \`cargo build\`, \`cargo test\`, \`cargo run\`.` | `- [ ] Unit tests where pure logic exists (e.g., parsing/filtering of enumeration results represented in a testable seam) — run via the dev-checks harness in the container per \`docs/build.md\` (\`ALL CHECKS PASSED\`: fmt/check/test/clippy; \`alpine-vm\` MCP; \`vm_status\` first; the command exit code is the authoritative success evidence).` |

### F.EDIT-2 — Verification Task 4 manual UX bullet (prefix only)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Manual UX check: for a jpg/png file, the menu lists at least the system-associated programs; selecting one opens this exact file in that program; NOT dependent on any app-maintained list.` | `- [ ] Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): for a jpg/png file, the menu lists at least the system-associated programs; selecting one opens this exact file in that program; NOT dependent on any app-maintained list.` |

### F.EDIT-3 — Future session execution notes: INSERT bullet (§3.1, Variant S1).

Nothing else changes.

## 10. File G — `.agent/todos/20261029/20261029-todo-9.md` (Wave 7)

### G.EDIT-1 — Task 2 second bullet reworded (x64 AND x86 matrix; consume + extend Wave-1 script)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Release builds for x64 AND x86 Windows targets; exact target triples and build commands are decided in this wave's architector step.` | `- [ ] Release builds for x64 AND x86: this wave consumes AND extends Wave 1's \`scripts/build-windows.sh\` — the x86 build is an extension of Wave 1's harness: the \`i686-pc-windows-gnu\` target plus the \`gcc-mingw-w64-i686\` toolchain addition to the tracked Dockerfile are added when the x86 target is realized; the exact extension mechanism/details are decided in this wave's architector step.` |

### G.EDIT-2 — Task 2 third bullet reworded (packaging consumes the produced artifacts)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] The packaging mechanism (script vs program, archive tooling) is decided in this wave's architector step.` | `- [ ] The packaging mechanism (script vs program, archive tooling) is decided in this wave's architector step; the packaging consumes the produced x64/x86 artifacts (the produced \`.exe\` files) from the gitignored \`dist/\` locations.` |

### G.EDIT-3 — Task 4 Verification first bullet reworded

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Unit tests for any pure packaging/config logic that exists. \`cargo build --release\` (and the x64/x86 target build matrix decided in this wave), \`cargo test\`.` | `- [ ] Unit tests for any pure packaging/config logic that exists — run via the dev-checks harness in the container per \`docs/build.md\` (\`ALL CHECKS PASSED\`: fmt/check/test/clippy; \`alpine-vm\` MCP; \`vm_status\` first; the command exit code is the authoritative success evidence).` |

### G.EDIT-4 — Task 4 Verification: INSERT a release-evidence bullet AFTER the reworded unit-test bullet

```markdown
- [ ] x64 AND x86 release builds via the extended Wave-1 build script inside the container (per Task 2) — each command exits `0` and leaves a non-empty artifact at the gitignored `dist/` locations; run through the `alpine-vm` MCP (call `vm_status` first; the command exit code is the authoritative success evidence).
```

### G.EDIT-5 — Task 4 Verification manual UX bullet (prefix only)

| BEFORE (exact) | AFTER (exact) |
| --- | --- |
| `- [ ] Manual UX check: run the installer flow on a clean destination dir → binary + config file present and app launches; default-viewer helper/guidance performs or guides correctly; packaged app navigations from Waves 1–6 still work from the installed location.` | `- [ ] Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): run the installer flow on a clean destination dir → binary + config file present and app launches; default-viewer helper/guidance performs or guides correctly; packaged app navigations from Waves 1–6 still work from the installed location.` |

### G.EDIT-6 — Deferred-decisions list: INSERT 2 bullets AFTER the exact line `- Release target triples and the x64/x86 build matrix.` (line 45)

```markdown
- x86 (i686) build realization details (exactly how the Wave-1 `scripts/build-windows.sh` + Dockerfile are extended: script changes for `i686-pc-windows-gnu` + the `gcc-mingw-w64-i686` toolchain addition + artifact placement — decided in this wave's architector step).
- Whether both architectures land in one zip or two.
```

### G.EDIT-7 — Future session execution notes: INSERT bullet (§3.1, Variant S1).

### G.EDIT-8 — PRESERVE verbatim (do NOT touch):

```markdown
### Task 6: MVP checkpoint — first `CHANGELOG.md` entry (FINAL task of the wave)
```

plus its 2 bullets; ALL existing deferred bullets:

```markdown
- Config file contents/format.
- Default-viewer registration helper mechanism (script vs executable, exact mechanism).
- Packaging mechanism/tooling (script vs program, archive tooling).
- Release target triples and the x64/x86 build matrix.
- CHANGELOG entry wording (written at execution, not pre-drafted in this wave file).
```

and the execution-notes bullet:

```markdown
- `CHANGELOG.md` may be edited ONLY for the first MVP entry in its dedicated final task (Task 6 of this wave file).
```

## 11. Planner's Post-Edit Verification Checklist

1. **todo-3 numbering**: headings read Task 1 (harness) → 2 (scaffold) → 3 (Open Image) → 4 (Display) → 5 (Verification) → 6 (Documentation); scope-note line references Task 1 + Task 6; new Task 1's ordering note references Task 2 + Task 5; scaffold bullet references Task 5's operator-run item. No stale "Task 5"/"Task 6" misreferences anywhere.
2. **todo-4…todo-9 numbering unchanged** (each keeps Task 1…N as before; no renumbering beyond what the tables above specify).
3. **Ban check (all 7 files + this spec's inserts)**: no crate names/versions; no registry dates; no keybinding/zoom-step/UI-placement additions; NO `CHANGELOG.md` instructions added (Wave 7's changelog rule untouched); no reference-specific names (no `rust:1.82`, no `rust-streamer-target`, no `rust-youtube-streamer*` names, no reference DLL list) copied into the wave files.
4. **Operator-run line counted once per file** inside the Verification task (todo-4…todo-9) and once in todo-3's renamed Task 5 — exactly 7 total.
5. **Execution-notes bullet counted once per file** — exactly 7 inserts (Variant S2 in todo-3; Variant S1 in the other 6).
6. **Preservation**: Waves 2–6 files outside the specified edits are byte-identical; Wave 7's Task 6 + changelog bullets + existing deferred bullets untouched; all Addendum v2 texts untouched.
7. **Deferred lists**: exactly 6 new bullets in todo-3; exactly 2 new bullets in todo-9; zero changes to todo-4…todo-8 deferred lists.
8. **Checkbox states**: every inserted/reworded task sub-item keeps `- [ ]` (never `[x]`).

## 12. Out of Scope (NOT this spec / NOT the Planner's edits)

- Task 2 of `20261030-todo-1.md` (tech.md / brief.md / architecture.md / README.md / context.md doc updates) — separate cycle, separate plan.
- Creating the actual harness files (Dockerfile, docker-compose.yml, scripts, docs/build.md, docs/build-windows.md) — Wave 1's future session.
- Any code, `Cargo.toml`, `.gitignore` change, VM command, commit, push, or other `.md`/config edits.
- Modifying `.agent/todos/20261030/20261030-todo-1.md` or any historical `.kilo/plans/*` file.

## 13. Self-Check Record (avowals)

- Every BEFORE string was verified against the actual file contents read 2026-10-30 (quoted text exists verbatim: todo-3 lines 11/16/18/24/29/31–33/35/37/42/51; todo-4 lines 30–31; todo-5 lines 33–34; todo-6 lines 31–32; todo-7 lines 26–27; todo-8 lines 27–28; todo-9 lines 18–19/28–29/45/50 — the shared first-notes anchor exists in all 7 files).
- Reference behavior sourced from the verified GitHub files (README build sections, docs/build.md, docs/build-windows.md, Dockerfile, docker-compose.yml, scripts/dev-checks.sh, scripts/build-windows.sh) fetched 2026-10-30.
- Contradiction scan: the new wording does not contradict existing deferred lists (todo-3's OS-boundary test bullet and the new cfg-gating bullet address different aspects; todo-9's "Release target triples" bullet and the new x86-realization bullet address different aspects).
