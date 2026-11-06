# Wave 4.1 Task 3 — Fullscreen tuning (F11 immediacy + ESC exit): Implementation Plan (4.1b)

Date: 2026-11-06
Branch: `fix/wave-4-operator-fixes` · Base HEAD: `892efc111db3ded8cf30c4c6eba2701535596f6c` (verified: `git log -1` = `892efc1... task: mark wave4.1 task2 button hints done`)
Spec: `.kilo/plans/20261102-wave4-1-task3-fullscreen-tuning-frontend-spec.md` (NEW, § refs below)
TODO duty: `.agent/todos/20261102/20261102-todo-1.md` § Task 3 (F11 agent-side investigation confirmed by spec §3; ESC exits fullscreen ON; no collisions; remaining operator-lane items documented — §6 of the spec, carried by Task 9, NOT here)
Sibling plans: `.kilo/plans/20261102-wave-4-task1-fullscreen-plan.md`, `.kilo/plans/20261102-wave4-task2-zoom-...*` (style reference ONLY — do NOT edit any existing plan file)

## 0. Verified facts at HEAD `892efc1` (measured, not assumed)

| File | Lines at HEAD | Role check |
|---|---|---|
| `src/app/fullscreen.rs` | 45 | holds `FULLSCREEN_TOGGLE_KEY` const + `toggle_fullscreen_and_send_command` + `toggle_fullscreen` (zoom.rs § precedent: chord helpers live beside their key consts in the owning app-seam module — the ESC predicate goes here too) |
| `src/app/keyboard_intake.rs` | 70 | reads-first block (10 reads, lines 24–33), dispatch chain (lines 34–63), module-local `was_pressed` helper (lines 68–70) |
| `src/app/fullscreen_tests.rs` | 84 | 7 tests; imports at line 6 pin `FULLSCREEN_TOGGLE_KEY` from `super::fullscreen` |
| `src/app.rs` | 183 | NO F11-only claim anywhere (checked lines 32–34, 116–125, module block) → app.rs needs ZERO changes in this task; the ~17-line headroom stays unused |
| `src/app/zoom.rs` | 157 | precedent only: `zoom_in_chord_was_pressed`/`zoom_out_chord_was_pressed` live beside their consts |
| Collision check | — | spec §4.3: no `Escape` use anywhere in `src/` (re-verified with grep at HEAD: no ESC match) |
| Test census | 95 currently (93 after Wave 4 + 2 from Task-2 button hints) | target after this task: **97 passed; 0 failed** |

Working-tree note (caller-visible): `.agent/project-info/context.md` is modified (uncommitted Task-3 spec entry from 4.1a) and the spec file is untracked. Step 5 disposes of both — implementer must NOT edit context.md content, only stage it.

## Conformance rules this plan is written against (implementer verifies at each step)

- ≤200 lines/file (src), ≤50 lines/method body, ≤2 args/method-or-fn, single-section boolean conditions, no commented-out code, prefer-private members (`pub(super)` scoping kept), self-documenting names, real newline characters in every file write.
- `docs: add wave4.1 task3 spec and plan` commit stages ONLY the three files named in Step 5; `.gitignore` Compliance + Remote-Safety checks run before every commit; push is NOT part of this step (Step 2 branch / Step 5 of the Critical Workflow owns it).

---

## Step 1 — `src/app/fullscreen.rs` (45 → 52 lines): ESC key constant + pure predicate

### 1.1 Fix the now-false F11 doc claim (line 10)

The comment "the only exit while ON" becomes factually wrong once ESC exits ON. Replace exactly the current lines 9–10:

```rust
/// F11: the de-facto Windows fullscreen toggle (spec §7 user-approved choice;
/// the only exit while ON, since ALL controls are hidden in ON).
```

with:

```rust
/// F11: the de-facto Windows fullscreen toggle (spec §7 user-approved choice;
/// one of the two ON exits -- F11 toggles both ways and ESC exits ON, since
/// ALL controls are hidden in ON).
```

(1:3 replacement; +2 lines.)

### 1.2 Add the ESC key constant immediately after the F11 constant block (after current line 11)

Exact text, including the caller-mandated comment string verbatim:

```rust
/// ESC exit: operator-requested additional exit from fullscreen ON; no other mode may consume it.
pub(super) const FULLSCREEN_EXIT_KEY: egui::Key = egui::Key::Escape;
```

(+2 lines.)

### 1.3 Add the pure exit predicate as a free function directly after the new constant (before `impl ImagesScrollerApp`)

Decision (caller-mandated home): the predicate lives in `fullscreen.rs` beside its key const — the same proof pattern as the zoom chord helpers living beside their consts in `src/app/zoom.rs`. Exact signature from spec §7:

```rust
/// True exactly when an ESC press this frame exits fullscreen: fullscreen must
/// be ON; ESC is a no-op everywhere else and never toggles fullscreen ON.
pub(super) fn escape_exits_fullscreen(fullscreen_on: bool, escape_pressed: bool) -> bool {
    fullscreen_on && escape_pressed
}
```

(+5 lines: 2 doc lines + signature + body + brace.)

### 1.4 File result

- `fullscreen.rs` ends at 52 lines. No other edits in this file. No imports change (`use eframe::egui;` already present).
- Verify with `wc -l src/app/fullscreen.rs` through the harness environment (or file read).

---

## Step 2 — `src/app/keyboard_intake.rs` (70 → 77 lines): reads-first ESC read + dispatch BEFORE zoom/fit

### 2.1 Module doc correction (line 2, 1:1 replacement)

Current line 2 says only "the fullscreen toggle". Replace line 2 exactly:

```rust
//! fullscreen toggle, the four navigation arrows, the zoom in/out/restore
```

with:

```rust
//! fullscreen F11 toggle and the ESC-while-ON exit, the four navigation arrows,
```

(Net line count unchanged; factual wording.)

### 2.2 ESC read — insert immediately after the `fullscreen_pressed` read (current line 24)

Keep the reads-first block intact: all reads stay collected before every dispatch. The ESC read combines the ONE gated condition into a named bool via the `fullscreen.rs` predicate (single-section rule downstream). Insert after line 24:

```rust
let escape_exit_pressed = fullscreen::escape_exits_fullscreen(
    self.fullscreen_on,
    was_pressed(screen_context, fullscreen::FULLSCREEN_EXIT_KEY),
);
```

(+4 lines.)

Spelling note (deterministic, no judgment): the caller's bound text quotes the read as `input(|i| i.key_pressed(fullscreen::FULLSCREEN_EXIT_KEY))`. The module-local helper called here is `was_pressed(screen_context, pressed_key)` whose body IS exactly `screen_context.input(|input| input.key_pressed(pressed_key))` (keyboard_intake.rs lines 68–70). The `was_pressed(...)` call is the file-consistent spelling of that same expression — use it; do NOT inline a second `input(...)` read.

### 2.3 ESC dispatch — insert immediately after the F11 dispatch block (after current line 36), BEFORE the `zoom_in_pressed` dispatch

Placement is chronology-mandated (spec §5.3): dispatching ESC before zoom/fit leaves the existing message-overwrite semantics untouched (`message_text`-bearing zoom-bound/fit-activation messages overwrite in the same order as today; ESC itself sends NO message — it only mirrors the F11 branch: gate + one `toggle_fullscreen_and_send_command` call). Insert after the `if fullscreen_pressed { ... }` block:

```rust
if escape_exit_pressed {
    self.toggle_fullscreen_and_send_command(screen_context);
}
```

(+3 lines.)

### 2.4 File result

- `keyboard_intake.rs` ends at 77 lines. Dispatch order after this step: fullscreen F11 → ESC-exit → zoom-in → zoom-out → restore-fit → fit-width → fit-height → ← → → → ↑ → ↓. All `if` conditions remain single-section named bools.
- No imports change (`use super::fullscreen;` already present).

---

## Step 3 — `src/app/fullscreen_tests.rs` (84 → ~99 lines): the +2 tests

### 3.1 Extend the existing import line (line 6)

Replace:

```rust
use super::fullscreen::FULLSCREEN_TOGGLE_KEY;
```

with:

```rust
use super::fullscreen::FULLSCREEN_TOGGLE_KEY;
use super::fullscreen::{escape_exits_fullscreen, FULLSCREEN_EXIT_KEY};
```

(+1 line; keep the existing line so the F11 pin diff stays minimal.)

### 3.2 Add test 1 immediately after `fullscreen_shortcut_is_f11` (groups the two key-pin tests; deterministic placement)

Exact name is caller-mandated (spec §7 names only the subjects — key-constant pin + predicate — and the caller fixed the fn names):

```rust
#[test]
fn fullscreen_escape_key_is_escape() {
    assert_eq!(FULLSCREEN_EXIT_KEY, egui::Key::Escape);
}
```

(+5 lines incl. separating blank line.)

### 3.3 Add test 2 immediately after test 1

The four truth combinations prove both the ON-exit branch and the OFF no-op (spec §7):

```rust
#[test]
fn escape_exits_fullscreen_matches_its_predicate() {
    assert!(escape_exits_fullscreen(true, true));
    assert!(!escape_exits_fullscreen(true, false));
    assert!(!escape_exits_fullscreen(false, true));
    assert!(!escape_exits_fullscreen(false, false));
}
```

(+7 lines incl. separating blank line.)

### 3.4 No dispatch test

Per the binding input: ESC's only action mirrors the F11 branch and the intake order cannot be unit-tested without an `egui::Context` mock (spec §7 last paragraph) — NO dispatch/intake test in this task. Exact test delta: **+2**, census 95 → **97**.

### 3.5 File result

- `fullscreen_tests.rs` ~97–99 lines, well under the 200 cap — extend the existing file; do NOT create a sibling test file.

---

## Step 4 — Harness verification (alpine-vm MCP)

Ordered exactly:

1. `alpine-vm_vm_status` — MUST be called first. If the VM or the `/img-scroller` share is unavailable: STOP and return a user question (per `docs/build.md`).
2. Run the full dev-checks harness (single command, exit code is the authoritative evidence):
   ```
   docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
   ```
3. Expected output: `ALL CHECKS PASSED` — `cargo fmt --check`, `cargo check --locked`, `cargo test --locked` (97 passed; 0 failed), `cargo clippy --locked -- -D warnings` all green. The captured log path lands in the gitignored `logs/checks/` tree; note its timestamped filename in the step report.
4. If clippy or fmt flags the new code (e.g. rustfmt reflowing the Step 2.2 call or the Step 3.1 import), fix the flagged formatting/lines exactly per the harness output and re-run the harness. No other refactoring is authorized.

---

## Step 5 — Git actions (branch `fix/wave-4-operator-fixes`, two commits, NO push IN THIS STEP)

Pre-commit checklist BEFORE each commit (Gitignore Compliance Rule):

1. Read `.gitignore`.
2. `git status` — confirm ONLY the planned files are staged; no `target/`, `dist/`, `logs/`, or any other gitignored path enters a commit; never force-add.
3. Commits are made on `fix/wave-4-operator-fixes` only (`git branch --show-current` confirms); origin push is a LATER workflow step — not here.

### 5.1 Commit 1 — planning artifacts

```
git add .kilo/plans/20261102-wave4-1-task3-fullscreen-tuning-frontend-spec.md
git add .kilo/plans/20261102-wave4-1-task3-fullscreen-tuning-plan.md
git add .agent/project-info/context.md
git commit -m "docs: add wave4.1 task3 spec and plan"
```

- The `context.md` staging is a caller-flagged decision: it holds the Task-3 spec status entry written during 4.1a (modified, uncommitted at `892efc1`). It is committed AS-IS — no content edit by the implementer — so the docs commit matches precedent (`fa8ec7e docs: add wave4.1 task2 spec and plan`). If the caller objects, this file moves to its own `docs: record task3 spec status in context log` commit (`19ff32c` style); default is the named two-commit sequence.

### 5.2 Commit 2 — feature

```
git add src/app/fullscreen.rs
git add src/app/keyboard_intake.rs
git add src/app/fullscreen_tests.rs
git commit -m "feat: escape exits fullscreen and f11 note"
```

Style check: lowercase type prefix, no period — consistent with `e3873f8 feat: window opens maximized and release console hidden` and `4a2e7f0 feat: button hotkey references and hover tooltips`.

### 5.3 Post-commit verification

- `git status` → working tree clean.
- `git log --oneline -3` → shows both new commits atop `892efc1`.

---

## Line-budget table (verified at HEAD; exact post-task targets)

| File | Lines at HEAD | Change | Post-task | Cap 200 |
|---|---|---|---|---|
| `src/app.rs` | 183 | NONE (no F11-only claim exists; ~17-line headroom reserved, unused) | 183 | OK |
| `src/app/fullscreen.rs` | 45 | comment fix 1:3 (+2), const +2, predicate +5 — no extraction needed | 52 | OK |
| `src/app/keyboard_intake.rs` | 70 | module-doc 1:1 (0), ESC read +4, ESC dispatch +3 | 77 | OK |
| `src/app/fullscreen_tests.rs` | 84 | import +1, two tests +12 | ~97 | OK |

Largest new fn body: 1 line (`escape_exits_fullscreen`). Every new `if` takes a single-section named bool. New free fn has exactly 2 args. Zero commented-out code introduced.

---

## Docs touchpoint — carried LATER, NOT in this task (binding spec §6)

The spec §6 OPERATOR-RUN note items (single short F11 tap vs held-key OS repeat delay; focus precondition; ESC exits ON / ESC no-op when OFF / no message-line or zoom/fit/nav effect; record Windows version + repeat-rate settings if a single tap still feels delayed) are carried by Wave 4.1 **Task 8** (the OPERATOR-RUN manual-UX gate) and **Task 9** (README/tech/architecture/context/brief/product doc updates). This plan performs NO documentation edits beyond the Step 5.1 planning-artifact commit; Task 9 owns recording the corrected behavior (F11 toggle + ESC-exit-ON, ESC no-op otherwise) exactly as landed.

---

## 4.5a crosswalk — spec §8 conformance checklist → this plan

| Spec §8 item | Covered by |
|---|---|
| 1. `FULLSCREEN_EXIT_KEY` in fullscreen.rs == `egui::Key::Escape` | Step 1.2 + test `fullscreen_escape_key_is_escape` |
| 2. State model unchanged (one `fullscreen_on`, default false) | No state edit — Step 1 does not touch the struct |
| 3. Toggle flips once, sends command exactly once | Unchanged `toggle_fullscreen_and_send_command` reused (Step 2.3) |
| 4. ESC read added to reads-first block | Step 2.2 (immediately after the F11 read) |
| 5. ESC dispatch after reads, before zoom/fit | Step 2.3 (chronology-mandated placement) |
| 6. ESC triggers only when ON; else no-op | Step 1.3 predicate + Step 2.2 named bool; test combos (false,true)/(false,false) prove the no-op |
| 7. No other key uses Escape; no collision | Spec §4.3 grep re-verified at HEAD `892efc1` — no ESC match in `src/` |
| 8. All intake `if`s single-section | Step 2.2 precomputes the compound gate into `escape_exit_pressed` |
| 9. `window_options()`/`NativeOptions` unchanged | app.rs untouched (Step 0 table) |
| 10. No texture/allocation churn | No display-path edit anywhere in this plan |
| 11. Unit tests +2 exactly | Step 3 (+2; census 95 → 97) |

---

## Out-of-scope guard (hard)

- NO fullscreen hover-reveal / auto-hide chrome (parked future-ideas items — no code, fields, or buttons).
- NO further key changes (zoom aliases, zoom-out range, fit semantics are Tasks 5–7 of this wave — untouched here).
- NO edits to `CHANGELOG.md`, `src/app.rs`, `window_options()`, the zoom/fit/navigation seams, or ANY existing `.kilo/plans/*` file (this plan is a NEW file only).
- NO new constants beyond `FULLSCREEN_EXIT_KEY`, no new imports beyond Step 3.1, no persistence, no Windows-native polling of F11/ESC timing (spec §3.3 forbids timers/`key_down` switches).
- Implementer creates NO new TODO file; if a defect outside these steps appears, STOP and return it to the caller.

## Expected completion signals

1. Harness: `ALL CHECKS PASSED` with `97 passed; 0 failed` and zero clippy/fmt findings.
2. `git log` shows exactly the two Step 5 commits atop `892efc1`; `git status` clean; working tree on `fix/wave-4-operator-fixes`.
3. The three touched src files match the budget table exactly; ESC: ON-status + Escape same frame → one flag flip + one `ViewportCommand::Fullscreen(false)`; any other time → no-op with no message sent.
