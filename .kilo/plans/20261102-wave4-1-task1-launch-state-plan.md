# Wave 4.1 Task 1 — Launch Window State: Implementation Plan

Date: 2026-11-05
Critical-Workflow step: 4.1b (Implementation Plan) for Task 1 of `.agent/todos/20261102/20261102-todo-1.md`
Branch: `fix/wave-4-operator-fixes` at HEAD `53c2e60` (`chore: bump version to 0.4.1`) — VERIFIED 2026-11-05 (`git log -1 --oneline` shows exactly `53c2e60`).
Front-end input: `.kilo/plans/20261102-wave4-1-task1-launch-state-frontend-spec.md` (read fully; binding).
Target executor: JUNIOR implementer under 50% restriction (workflow step 4.2).

---

## 0. Verified pre-flight facts (do not re-derive)

| Fact | Verified value | Evidence |
|---|---|---|
| Branch / HEAD | `fix/wave-4-operator-fixes` @ `53c2e60` | `git branch --show-current`; `git log -3 --oneline` → `53c2e60` ← `7df1ef3` (wave 4.1 global plan) ← `2fd26c8` (todo) |
| `src/app.rs` length | **198 lines** (not at the 200 cap) | full-file read; `window_options()` spans lines 144–152 |
| `src/main.rs` length | **33 lines** | full-file read |
| `window_options()` doc comment | **NONE EXISTS** — the method has no doc comment, so the spec task "update the method doc if it mentions placement-respect" is a NO-OP (condition false) | full-file read lines 144–152 |
| Stale wording in app.rs | module doc line 1: `//! Application shell: the regular, resizable viewer window of Images Scroller.` — contains the superseeded "regular" wording. **NOT touched at 4.2** (see §1 note and §6 docs touchpoints) | full-file read line 1 |
| `src/` uses of `println!`/`eprintln!`/`log::`/`tracing::` | **none** | spec §3.3 + `grep window_options|windows_subsystem|with_maximized` over `src/*.rs` returns only `src/app.rs:144,149` and `src/main.rs:20` |
| `with_maximized(false)` occurrences | exactly ONE: `src/app.rs` line 149 | same grep |
| Any test pinning `window_options()` / `NativeOptions` | **none** | same grep |
| Untracked files at HEAD | only `.kilo/plans/20261102-wave4-1-task1-launch-state-frontend-spec.md` (and, after this step, this plan file) | `git status` 2026-11-05 |
| `.kilo/plans/*` tracked in git | yes (`docs: add wave 4.1 global plan` etc. precedent) | `git ls-files .kilo/plans` |
| Test census | 93 unit tests (`ALL CHECKS PASSED`, 93 passed / 0 failed) | `tech.md` §Testing; dev-checks logs |
| Cargo version already bumped | `Cargo.toml`/`Cargo.lock` = 0.4.1 (Step 3 landed; NOT part of Task 1) | HEAD commit message |

## Supersession guard (binding conflict note)

The todo's binding conflict note: the old rule "window opens normal or maximized exactly as the user
placed it" is SUPERSEDED (2026-11-02 correction): launch is ALWAYS MAXIMIZED. The stale wording still
present in `brief.md`/`product.md` belongs to Task 9 — never rely on it for this task.

---

## 1. Decisions encoded by the architector (binding; no junior judgment)

1. **Maximize literal flip**: `src/app.rs` line 149 `.with_maximized(false)` → `.with_maximized(true)`. No other
   `ViewportBuilder` call, no new `NativeOptions` fields, no `with_fullscreen`, no `with_inner_size`,
   no `with_decorations(false)`, no position/visible options (spec §3.1, §5.4).
2. **Method doc comment**: none exists → none is added. Adding one would consume the tight app.rs budget
   for documentation the spec does not require; the executable behavior is self-evident from the literal
   (self-documenting-code rule). The TASK-9/4.4 docs pass owns any wording change if review finds it necessary.
3. **Console hiding form**: `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` — the spec's
   recommended release-only `cfg_attr` (spec §3.2, 70–77). Rationale (fixed): debug builds keep a
   Windows console for future diagnostics; release `.exe` runs windowed; non-Windows container
   builds ignore the attribute entirely (Rust Reference), so harness runs are unaffected.
4. **Attribute placement**: `src/main.rs`, directly under the existing `//!` module doc header (line 1),
   before all `mod` declarations. There are NO other crate-level `#![...]` attributes in main.rs today.
5. **One explanatory comment line** above the attribute: `// Release runs windowed — no console.`
6. **Tests: NO new tests** (see §4 for the encoded reasoning; census stays 93).
7. **Commit messages** (§5): plan/spec commit `docs: add wave 4.1 task 1 launch-state spec and plan`;
   feature commit `feat: window opens maximized and release console hidden` (fix-behavior corrections in
   this wave use the `feat:` subject style of the repo's landed history).
8. **Output-diagnostics audit**: appending nothing that writes to stdout/stderr; the app has zero
   `println!`/`eprintln!`/`log`/`tracing` output, so hiding the release console loses nothing (spec §3.3).
9. **Docs touchpoints**: listed in §6 for the 4.4 docs-specialist and Task 9 — NOT executed by the 4.2 implementer.

---

## 2. Line-budget check (verified, not estimated)

| File | Before | Change | After | Cap | Status |
|---|---|---|---|---|---|
| `src/app.rs` | 198 | 1 line, single token (`false`→`true`, line 149); no lines added or removed | 198 | 200 | 2-line headroom; NOT endangered by this task |
| `src/main.rs` | 33 | +2 lines (1 attribute + 1 comment) | 35 | 200 | far under cap |

**Budget note for step 4.3 only (do NOT give to the 4.2 implementer)**: `src/app.rs` at 198/200 has a
2-line headroom. If the 4.3 simplifier/reviewer must add lines here, the sanctioned trim is compacting
the `mod` declaration block (space-separated groups instead of one `mod` per blank line, saving ~11
lines) — per simplifier precedent from prior waves. No other file approaches its cap.

---

## 3. Implementation steps for the 4.2 implementer (executed in this exact order)

### Step 3.0 — Pre-edit verification (read-only)

1. Run `vm_status` (alpine-vm MCP) FIRST; only proceed when the VM is running and SSH-reachable
   (`docs/build.md` usage notes; STOP and return a user question if unavailable).
2. Run `git status` on the branch: confirm you are on `fix/wave-4-operator-fixes`, HEAD `53c2e60`
   (already `chore: bump version to 0.4.1`), one Cargo file pair untouched...
3. Confirm with a file read:
   - `src/app.rs` line 149 is exactly `            .with_maximized(false),`
   - `src/main.rs` line 1 is the `//!` module doc and lines 3–12 are the nine `mod` declarations.

### Step 3.1 — Commit the planning artifacts (spec + this plan)

Stage and commit ONLY the two planning markdown files:

```
git add .kilo/plans/20261102-wave4-1-task1-launch-state-frontend-spec.md
git add .kilo/plans/20261102-wave4-1-task1-launch-state-plan.md
git commit -m "docs: add wave 4.1 task 1 launch-state spec and plan"
```

Pre-commit checklist (Gitignore Compliance Rule): run `git status`; confirm NOTHING matching
`.gitignore` (root `dist/`, `logs/`, `*.log`, `target/`) is staged; `Cargo.lock` must NOT be touched
by this task (already at 0.4.1 from Step 3); stage exactly the two paths above, nothing else.

### Step 3.2 — Edit 1: maximize at launch (`src/app.rs`)

Exact change, line 149 inside `window_options()` (current lines 144–152):

```rust
pub fn window_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_WINDOW_TITLE)
            .with_resizable(true)
            .with_maximized(true),
        ..eframe::NativeOptions::default()
    }
}
```

The ONLY diff: `false` → `true`. Everything else — the method body, its position in the file, the
`mod` block order — stays byte-identical. No doc comment is added (Decision 2).

### Step 3.3 — Edit 2: console hiding (`src/main.rs`)

Insert immediately after the `//!` module doc line (current line 1), before the `mod app;` block,
exactly these two lines:

```rust
// Release runs windowed — no console.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

Result: the file top becomes

```rust
//! Entry point of the Images Scroller viewer.

// Release runs windowed — no console.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
```

Rules honored: real newline characters (never literal `\n`), no commented-out code (this is an active
one-line explanation per the plan's Decision 5, not commented-out code), attribute at the crate root
before every item (valid Rust: inner doc comments and crate attributes may be interleaved before the
first item). Do NOT touch anything else in `main.rs`.

### Step 3.4 — Container verification (dev checks; exit code authoritative)

`vm_status` FIRST (Step 3.0 already covered it; re-check if time has passed). Then run the harness:

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
```

Expected final lines: `PASS test exit=0 (… s)` with **93 passed; 0 failed** and
`ALL CHECKS PASSED` (exit 0). Any failure:
- fmt drift → NOT expected (both edits are rustfmt-clean); if seen, STOP, report to the caller — do NOT run `cargo fmt` silently.
- check/clippy failure → suspect a typo/misplacement in either edit; re-read §3.2/§3.3 and fix only those spots.
- test failure → impossible by construction for these two literals; any test failure means an
  unrelated regression — STOP and report.

MCP notes: the command exit code is the authoritative evidence; an absent stderr in the captured
MCP output is NOT a failure signal; the run appends a timestamped log under gitignored `logs/checks/`.

### Step 3.5 — Commit the feature change

```
git add src/app.rs src/main.rs
git commit -m "feat: window opens maximized and release console hidden"
```

Pre-commit checklist: `git status` clean except the two staged files; verify no gitignored path
(`dist/`, `logs/`, `target/`) is staged; `Cargo.lock` unstaged. Both edits live in ONE commit.

### Step 3.6 — What the implementer does NOT do here (hard guard)

- NO `.agent/project-structure.md` edit (no file added/removed).
- NO README/docs edit (Task 9 / step 4.4 owns documentation).
- NO `.agent/todos/20261102/20261102-todo-1.md` mark yet (step 4.6 adds `[DONE]`/`[x]`).
- NO CHANGELOG edit (todo forbids CHANGELOG.md entirely).
- NO edit to any existing `.kilo/plans/*` file (new plan files only).
- NO parked-future-idea implementation (drag&drop, auto-hide chrome, etc.).
- NO ii-code for Tasks 2–7 (tooltips, ESC, stray message, zoom range, aliases, fit one-shot).
- NO `push` this sub-task (workflow restricts push to Step 5, `origin` only).

## 4. Tests decision (encoded reasoning — no junior discretion)

- **NO new unit tests are added by Task 1.** Census stays exactly **93** (`tech.md` §Testing).
- The change contains two configuration literals with no pure seam: `window_options()` constructs
  `eframe::NativeOptions` for the native (Windows-side) window-manager launch path; winit consumes it
  only outside the container. The container harness cannot exercise launch geometry.
- Grep evidence: no existing test (or any other code) reads `NativeOptions`/`window_options()`
  except `src/main.rs:20`; a would-be pin test would exist for the sole purpose of pinning a literal
  the OPERATOR-RUN directly observes, with no bug-catching surface (a wrong literal fails the same
  cargo check as a spelled-out right one). Spec §8 explicitly routes both facts ("window opens
  maximized", "console is hidden") to operator-run on real Windows.
- Harness coverage after the edit (fmt/check/clippy/test) still proves both files compile and the
  clippy attribute placement is lint-clean; that plus the 93-test green census is the sanctioned
  agent-side verification envelope for this task.

## 5. Git summary for Task 1 (complete chain; nothing else)

| # | Cmd | Message | Only stage |
|---|---|---|---|
| 1 | `git add .kilo/plans/20261102-wave4-1-task1-launch-state-frontend-spec.md .kilo/plans/20261102-wave4-1-task1-launch-state-plan.md` | `docs: add wave 4.1 task 1 launch-state spec and plan` | those 2 files |
| 2 | `git add src/app.rs src/main.rs` | `feat: window opens maximized and release console hidden` | those 2 files |

No force-add, no push, no tags. Later workflow steps own their own commits (review fixes / docs /
verification / TODO mark) with their own meaningful messages.

## 6. Downstream step pointers (NOT executed by 4.2)

- **4.3 (reviewer + simplifier)**: verify both edits match §3.2/§3.3/§5; use the §2 trim note only if
  budget pressure arises; simplify only real finds (none are expected for 2 lines).
- **4.4 (docs-specialist, code comments only)**: candidate touchpoint already visible —
  `src/app.rs` line 1 module doc reads `Application shell: the regular, resizable viewer window…`;
  should re-point to the always-maximized launch (e.g. `…the always-maximized, resizable viewer
  window…`) if kept faithful to the 2026-11-02 correction; docs-specialist owns wording/length there.
- **4.5a (frontend-specialist)**: run the spec's §6 checklist; conformance crosswalk below.
- **4.5b (architector)**: plan-adherence check against THIS file; §6/§9 of the spec govern.
- **4.6 + later**: TODO `[DONE]` marks (4.6); Task 8 wave verification rebuilds
  `dist/windows-x64/img-scroller.exe` (`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh`)
  and reruns the DLL audit; Task 9 rewrites the docs (next list). Task 1 itself does NOT execute those.

### 6.1 4.5a conformance crosswalk (spec §6 → landed evidence)

| Spec §6 item | Evidence expected after 4.2 |
|---|---|
| 1 `.with_maximized(true)` in `window_options()`, no other launch-size/mode change | `src/app.rs:149` literal `true`; grep shows one occurrence |
| 2 `.with_resizable(true)` and `.with_title(APP_WINDOW_TITLE)` retained | untouched lines 147–148 |
| 3 no `with_fullscreen`/`with_decorations(false)`/forced inner size/position | grep over `window_options()` body |
| 4 `fullscreen_on` still init `false`, never persisted | `src/app.rs:59` untouched |
| 5 F11 sends `ViewportCommand::Fullscreen(bool)` on the toggle frame | `src/app/fullscreen.rs` untouched (Task 3 scope) |
| 6 runtime fullscreen mechanism unchanged | no diff outside `src/app.rs` line 149, `src/main.rs` lines 3–4 |
| 7 crate-level console attribute in the recommended form | `src/main.rs` line 4 = `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` |
| 8 attribute at the crate root, before `mod`/`main` | position from §3.3 |
| 9 no stdout/stderr/log/tracing output lost | spec §3.3 grep enacted (zero hits in src/) |
| 10 no per-frame texture churn | no paint path touched |

**Operator-run residue (cannot be agent-verified, per spec §8)**: the actual maximized launch window
and the absence of a visible cmd window on the shipped `.exe`; recorded by Task 8's OPERATOR-RUN.

## 7. Docs touchpoints list for Task 9 (recorded ONLY; do not touch in this task)

Files listed in todo Task 9 (all corrected wordings, including the superseded
"opens normal or maximized as the user placed it" sentence):
`README.md`, `tech.md`, `architecture.md`, `context.md`, `brief.md`, `product.md`.
Task 9 also supersedes the launcher wording of the 15-item OPERATOR-RUN checklist item 1
(`.agent/todos/20261029/20261029-todo-6-DONE.md` / `.kilo/plans/20261102-wave-4-task4-verification-plan.md`
§11 "never auto-maximized") — the 0.4.1 run supersedes the 0.4.0 launch-state wording.

## 8. Out-of-scope guard (restated; binds the 4.2 executor)

Everything in this section is FIXED — no implementer latitude beyond the two file edits and the two
commits above. Scope = exactly the bottom two checkboxes of todo Task 1:
window ALWAYS MAXIMIZED at every launch; the console hiding on the release `.exe`.
Not in this task: tooltips/hotkey refs (2), F11 immediacy/ESC (3), stray message (4), zoom range (5),
aliases (6), fit one-shot (7), verification plan (8), documentation (9), parked future ideas.
