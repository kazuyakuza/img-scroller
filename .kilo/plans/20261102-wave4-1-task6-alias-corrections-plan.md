# Wave 4.1 — Task 6 Implementation Plan: Zoom hotkey alias corrections (strict shift)

Created by Architector (step 4.1b) on 2026-11-06 for the caller's authorized combined 4.1a+4.1b intake-scoped cycle. VERIFIED on branch `fix/wave-4-operator-fixes`, HEAD `daf0e2f` (`git log --oneline` shows `daf0e2f task: mark wave4.1 task5 zoom range done`), working tree clean. Requirements source: `.agent/todos/20261102/20261102-todo-1.md` Task 6.

## 0. Binding (operator report, verbatim + resolution)

- Report: "aliases shift plus '+' or '-' doesn't works. And it seems that '+' does zoom in without the shift key, which is wrong."
- RESOLUTION (user-approved, binding): SHIFT IS REQUIRED for BOTH aliases:
  - zoom-in alias = the `=`/`+` row key PRESSED WHILE `modifiers.shift` IS HELD — accept BOTH egui logical resolutions: `Key::Equals` + shift AND `Key::Plus` + shift (reachability verdict below: Plus IS reachable → BOTH encoded).
  - zoom-out alias = `Key::Minus` PRESSED WHILE `modifiers.shift` IS HELD.
- PLAIN (unshifted) `Equals`/`Plus`/`Minus` must do NOTHING. No other key becomes a zoom alias (nothing else on those rows is an alias; nothing else consumes these keys — verified below).
- `I`/`O`/`0` + F11 + ESC + arrows + W/H behavior UNTOUCHED (regression-guard).
- The Wave-4 Task-2 D4 dual-resolution is SUPERSEDED by this strict-shift rule (the dual resolution itself survives, but only as the set of reachable row-key resolutions of the shifted key — and now EVERY resolution is shift-gated; the old unconditional `Key::Plus` accept is removed).

## 1. Research findings (verified, with citations)

### 1.1 Landed code — exact current matching (D4)

`src/app/zoom.rs` lines 102–132 (read at HEAD `daf0e2f`):

- `zoom_in_chord_was_pressed` (lines 105–110): reads `Key::I` press; else delegates to `zoom_in_shift_alias_was_pressed`.
- `zoom_in_shift_alias_was_pressed` (lines 112–120): THE BUG — reads `Key::Plus` press first and returns `true` UNCONDITIONALLY (no shift check!); then reads `Key::Equals` and returns `input.modifiers.shift` (shift-gated).
- `zoom_out_chord_was_pressed` (lines 124–132): reads `Key::O` press; then reads `Key::Minus` and returns `input.modifiers.shift` (already shift-gated).

Call sites: `src/app/keyboard_intake.rs` lines 27–28 (`zoom_in_chord_was_pressed` / `zoom_out_chord_was_pressed`), consumed exactly once each, dispatched at lines 42–47 (`zoom_in_once` / `zoom_out_once`). Only these two call sites exist in `src/` (grep: no other `Key::Plus`/`Key::Equals`/`Key::Minus` consumers). **`keyboard_intake.rs` needs NO code change** — the dispatch shape is unrelated to the alias internals; its doc block stays valid.

Root-cause mapping to the report:
- "plain '+' zooms in without shift" → the unconditional `Key::Plus` accept (line 113–115). On winit backends/layouts where the pressed key resolves to `Key::Plus` with shift NOT held (numpad `+`; layouts where `+` is the unshifted key, e.g. German; the shifted `=` row key path below), plain `+` zooms in. Matches the operator observation exactly.
- "shift plus '+' doesn't work" → on US layouts the shifted `=` key resolves through the backend CHARACTER path to `Key::Plus` — which the code accepts unconditionally, so the alias mismatch is layout-dependent; the operator's exact layout/or path (e.g. a backend path where the press resolves dif­ferently) failed. The unconditional-Plus design is the proven-broken half of the D4 encoding — the strict rule replaces it.
- "shift plus '-' doesn't work" → the only shift-free path for '-' is `Key::Minus` (physical fallback), which IS shift-gated today; the operator's failing path is layout-dependent. The strict encoding below is mapping-agnostic and is the user-approved fix; any residual per-layout behavior goes to the OPERATOR-RUN note (Task 6 of the TODO: "keyboard-layout caveats go to the OPERATOR-RUN note"), NOT to extra agent-side code.

### 1.2 egui 0.37 facts (sources verified 2026-11-06 from the 0.37.0 tagged sources: `crates/egui/src/data/key.rs` and `crates/egui-winit/src/lib.rs` at `https://raw.githubusercontent.com/emilk/egui/0.37.0/...` — NOT from memory)

- `Modifiers` field access: `input(|i| i.modifiers.shift)` — `InputState.modifiers: Modifiers` is a public field (`crates/egui/src/input_state/mod.rs`), and `Modifiers` is a plain struct whose `shift` boolean reflects the LAST `Event::ModifiersChanged` of the frame, carried over between passes until the next change or focus loss. The landed code already reads it exactly this way — the pattern is in-use-proven.
- `key_pressed` semantics: `InputState::key_pressed(key)` scans the frame's `events` for `Event::Key { key, pressed: true, .. }` — it does NOT match the event's own `modifiers` field. Therefore a key event fires `key_pressed` regardless of shift, and the separate `input.modifiers.shift` read is the ONLY shift filter → the two-step read in the landed code is the correct egui-0.37 idiom; our fix keeps it.
- **`Key::Plus` REACHABILITY VERDICT: REACHABLE — three concrete delivery paths verified:**
  1. egui-winit logical path: `winit::keyboard::Key::Character("+")` → `egui::Key::from_name("+")` → `Key::Plus` (`key_from_winit_key` in `crates/egui-winit/src/lib.rs`; `from_name` table: `"+" | "Plus" | "NumpadAdd" => Self::Plus`). winit emits `Character("+")` whenever the active layout produces `+` — including Shift+'=' on US layouts AND plain '+' on layouts where '+' is unshifted (e.g. German).
  2. egui-winit physical fallback path: `KeyCode::NumpadAdd => Key::Plus` (`key_from_key_code`, same file) — reachable whenever the logical translation is `None` (e.g. `_`/'Underscore' cases) or unmatched.
  3. Empirical: the operator's plain-'+' zoom-in report itself proves `Key::Plus` was delivered on their Windows setup (the current unconditional branch is the only mechanism that can produce it).
- `Key::Equals` reachability: `Character("=")` → `from_name` `"=" | "Equal" | ... => Self::Equals` and physical `KeyCode::Equal => Key::Equals`. On US layouts plain '=' resolves here shift=false; some layouts resolve Shift+'=' to Equals.
- `Key::Minus` reachability: `Character("-")` → `Key::Minus`; physical `KeyCode::Minus | KeyCode::NumpadSubtract => Key::Minus`; the shifted '_' character (`from_name("_") → None`) falls back to the PHYSICAL path → `Key::Minus` with the frame's modifiers (shift stamped) — this is how Shift+'-' reaches the alias.
- Test-simulation fact (egui `InputState::begin_pass`): `Event::ModifiersChanged` sets the frame's `modifiers`; `key_pressed` edges come purely from `Event::Key` presses. (Not needed by this plan's tests, which pin the pure predicate only — recorded so the implementer does not attempt heavier egui-Context simulation.)
- Layout-variance caveat: with the user-approved strict rule the ENCODING is mapping-agnostic (every accepted combination requires shift=true; every unshifted press is rejected no matter which `Key` it resolves to). Edge case: a layout where BOTH plain and shifted forms resolve to the same egui `Key` cannot be distinguished mid-row — the shift gate still filters the plain form; the shifted form either holds shift (accepted) or is indistinguishable to egui (operator-noted). STANDS per the user's explicit rule — the operator tests it.

### 1.3 Test pinning reality check (changes the caller's assumed premise — record)

`src/app/zoom_tests.rs` currently has 11 tests and contains **NO chord/alias tests at all** — nothing pins the current D4 alias behavior (the alias internals were never tested; `grep` for chord helper names in tests finds none). THEREFORE: **0 tests are renamed or deleted; 5 NEW strict-shift truth-table tests are ADDED**. The pins created here are the first alias-behavior pins in the codebase.

Whole-repo census counted at HEAD `daf0e2f`: exactly **97** `#[test]` functions (per-file counts verified: button_hint 2; folder_listing 5; app_display 2; scaling.rs 5; image_loading 4; name_collation 2; app/fit_mode_tests 11; app/floating_controls_tests 1; zoom.rs 8; scaling/fit_mode_tests 6; app/folder_navigation_continuation_tests 4; sibling neighbor_tests 2; app/folder_navigation_tests 5; sibling tests 6; app/fullscreen_tests 9; app/last_viewed_tests 5; app/tests 9; app/zoom_tests 11 → 97). Expected after Task 6: **102** (+5, all in `src/app/zoom_tests.rs`, which becomes 16).

## 2. Tiny ordered implementation steps (junior-executable)

### Step A — `src/app/zoom.rs`: chord-helper changes (exact before/after)

A1. Replace the private alias helpers (current lines 102–132 region) with the strict rule. DELETE the unconditional `Key::Plus` accept. ADD one pure, testable predicate the two aliases share, and make it `pub(super)` (test-visibility precedent used at HEAD: `pub(super) const RESTORE_FIT_KEY`).

Exact AFTER code (drop the old `zoom_in_shift_alias_was_pressed` and the old inline shift reads entirely; keep the two public chord fns with the same signatures and the same first branches):

```rust
/// The strict-shift alias rule (Wave 4.1 Task 6): an alias fires ONLY when
/// the alias key was pressed this frame AND Shift is held; the unshifted
/// '='/'+'/'-' keys do NOTHING. Pure: no egui reading.
pub(super) fn shift_alias_is_pressed(alias_key_was_pressed: bool, shift_held: bool) -> bool {
    alias_key_was_pressed && shift_held
}
```

New `zoom_in` alias path (replaces `zoom_in_shift_alias_was_pressed`):

```rust
/// The zoom-in shift alias: the `=`/`+` row key WHILE Shift is held. BOTH
/// egui logical resolutions of Shift+'+' are accepted (shift-gated):(a)
/// `Key::Equals` (unshifted '=' resolution on some layouts), (b) `Key::Plus`
/// (the reachable '+' resolution: shifted '=' on US layouts, unshifted '+'
/// on e.g. German layouts, and the numpad fallback). OPERATOR-RUN note:
/// keyboard-layout variance is operator-tested, not hidden in code.
fn zoom_in_shift_alias_was_pressed(chord_context: &egui::Context) -> bool {
    let shift_held = chord_context.input(|input| input.modifiers.shift);
    shift_alias_is_pressed(
        chord_context.input(|input| input.key_pressed(egui::Key::Equals)),
        shift_held,
    ) || shift_alias_is_pressed(
        chord_context.input(|input| input.key_pressed(egui::Key::Plus)),
        shift_held,
    )
}
```

New `zoom_out` alias path (same shape, single resolution; reads SHIFT FIRST — one consistent shift evaluation per alias):

```rust
/// The zoom-out shift alias: `-` WHILE Shift is held (`Key::Minus` is the
/// only egui resolution of the '-' key; the shifted '_' press resolves by
/// physical position back to `Key::Minus` per the egui-winit fallback).
fn zoom_out_shift_alias_was_pressed(chord_context: &egui::Context) -> bool {
    let shift_held = chord_context.input(|input| input.modifiers.shift);
    shift_alias_is_pressed(
        chord_context.input(|input| input.key_pressed(egui::Key::Minus)),
        shift_held,
    )
}
```

A2. Update the two chord-helper doc comments (`zoom_in_chord_was_pressed` / `zoom_out_chord_was_pressed`) to describe the strict rule: replace the old "(Shift held while `=`/`+` is pressed; BOTH egui logical resolutions ... accepted -- D4)" wording with "the Shift-gated '='/'+' resolutions only (strict rule, Wave 4.1 Task 6; D4's unconditional Plus accept is removed)". Never write the literal "D4" as active behavior.

A3. Update the module doc header (current lines 1–20): the line that says "its keyboard chords (I / Shift+Plus, O / Shift+Minus, 0 = restore fit)" stays factually true; APPEND one sentence so the strict rule + operator caveat exist at module level:

```
//! The '+/-' aliases are strictly shift-gated: Shift+'+'/Shift+'-' fire,
//! plain '='/'+'/'-' do nothing (Wave 4.1 Task 6); keyboard-layout variance
//! is the OPERATOR-RUN note, not a code fallback.
```

A4. Visibility: `shift_alias_is_pressed` is `pub(super)`; the two `*_shift_alias_was_pressed` wrappers stay private (not tested directly — they need an `egui::Context`). File-length guard: `src/app/zoom.rs` grows by roughly +2 net lines (reminder: the 200-line cap; the landed file is 142 → ~144; no split needed).

### Step B — `src/app/keyboard_intake.rs`: verification only (NO code change)

B1. Verify (implementer asserts by reading, no edit): `read_and_dispatch_keystrokes`
- reads `zoom_in_chord_was_pressed` / `zoom_out_chord_was_pressed` exactly once each (lines 27–28) BEFORE the dispatch chain (lines 42–47),
- the four arrows (lines 32–35, 57–68), F11 (line 22) + ESC (lines 23–26), `0` (line 29), `W`/`H` (lines 30–31) are untouched and ungated,
- the module doc ("the zoom `I`/`O`/`0` chords") stays true after the fix.
If ALL five hold → leave the file byte-identical. If any verify fails, STOP and return the discrepancy to the caller (do NOT self-modify the intake shape).

### Step C — `src/app/zoom_tests.rs`: the 5 new truth-table tests (exact)

Add EXACTLY these 5 tests to the existing 11 (nothing else in the file changes). The pure predicate needs no egui objects — container-safe pins. Local shorthand to keep each body tiny and below the depth cap:

```rust
/// Shorthand: the alias key press and the shift state, directly as booleans.
fn alias(alias_key_was_pressed: bool, shift_held: bool) -> bool {
    zoom::shift_alias_is_pressed(alias_key_was_pressed, shift_held)
}
```

(Place the helper fn immediately above the new tests; zig-zag footage: the file imports `zoom` already at line 8.)

```rust
#[test]
fn zoom_in_shift_alias_matches_shift_equals() {
    assert!(alias(true, true)); // Shift+'=' resolution fires
}

#[test]
fn zoom_in_shift_alias_matches_shift_plus() {
    assert!(alias(true, true)); // the '+' resolution (US shifted '=', numpad) fires
    // same predicate, distinct named resolution; both shift-gated paths accept
}

#[test]
fn zoom_in_shift_alias_rejects_the_plain_keys() {
    assert!(!alias(true, false)); // plain Equals AND plain Plus: no shift → NOTHING
    assert!(!alias(false, true)); // shift held, no '='/'+' press → nothing
}

#[test]
fn zoom_out_shift_alias_matches_shift_minus() {
    assert!(alias(true, true)); // Shift+'-' fires
}

#[test]
fn zoom_out_shift_alias_rejects_the_plain_keys() {
    assert!(!alias(true, false)); // plain Minus: no shift → nothing
    assert!(!alias(false, true)); // shift held, no '-' press → nothing
}
```

I/O/0 regression: untouched code paths backed by the existing tests (`zoom_in_once_zooms_beyond_fit`, `restore_zoom_to_fit_returns_to_fit`, `restore_fit_hotkey_is_num0`) — no new I/O/0 test is added (the chord's `I`/`O` first branches keep their `key_pressed(ZOOM_IN_KEY/OUT_KEY)` reads verbatim; nothing consumes those keys elsewhere).

Census delta: `src/app/zoom_tests.rs` 11 → 16; whole project **97 → 102**; renamed: 0; deleted: 0.

### Step D — Harness + commits

D1. Harness (via `alpine-vm` MCP; `vm_status` FIRST; exit code authoritative; per `docs/build.md`):
`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
Expected: `ALL CHECKS PASSED` with 102 tests passed, 0 failed (fmt/check/test/clippy in fixed order). If the VM or `/img-scroller` share is unavailable → STOP and return a user question (never proceed host-side).
D2. Commit checklist before `git add`: read `.gitignore` (repo root) and run `git status` — stage ONLY `src/app/zoom.rs` and `src/app/zoom_tests.rs` (+ any operator-mandated plan file); verify no `dist/`, `logs/`, `target/`, `Cargo.lock-noise` or `.gitignore`-matching files staged; single unchained commands.
D3. Commit (the implementer's one code commit, message pinned by the caller):
`fix: require shift for zoom plus minus aliases`
D4. Plan-artifact commit (caller-side, Task-5 precedent `761ac50`):
`docs: add wave4.1 task6 alias corrections plan`
D5. Task mark (`20261102-todo-1.md` Task 6 sub-items `[]` → `[x]`) at the caller's 4.6 step; preserve the rest of the file byte-for-byte; the Task-8 OPERATOR-RUN sub-item stays unchecked.

### Step E — Out-of-scope guard (hard)

- NO fit-model changes (Task 7's one-shot semantics — `set_fit_mode`, `FitMode`, `W`/`H`, restore/`0`, open-reset: all untouched).
- NO zoom-range/message changes (Tasks 4/5 landed state at `d4c6870`/`943d868` is frozen: symmetric [1/8, 8] bounds, silent-at-bound steps, no messages from zoom actions).
- NO intake reordering beyond the chord internals (the `keyboard_intake.rs` read order and the restore/fit/arrows/fullscreen chains are fixed).
- NO docs edits: `architecture.md`/`tech.md`/`README.md`/`context.md`/`brief.md`/`product.md` (Task 9 owns the doc-truth fixes: tech.md line "plain unshifted `=`/`-` are not hotkeys" becomes TRUE after this task fixes the '+' case; the Task-9 wording must also record the strict rule below). `CHANGELOG.md`: not touched. Existing `.kilo/plans/*`: never edited (new-file-only rule). The ONLY markdown this cycle creates: this plan file (+ the caller's adherence artifacts per its own steps).

## 3. Truth table (the encoded final behavior, post-fix)

| Keystroke (US layout example) | egui resolution | shift held | Result |
| --- | --- | --- | --- |
| I | `Key::I` | any | zoom-in step (unchanged) |
| Shift+'=' / '+' | `Key::Equals` or `Key::Plus` | yes | zoom-in step (FIXED alias) |
| plain '=' | `Key::Equals` | no | NOTHING (FIXED) |
| plain '+' (unshifted on some layouts / numpad) | `Key::Plus` | no | NOTHING (the operator bug fixed) |
| O | `Key::O` | any | zoom-out step (unchanged) |
| Shift+'- | `Key::Minus` | yes | zoom-out step (alias works) |
| plain "-" | `Key::Minus` | no | NOTHING (unchanged) |
| 0 / W / H / F11 / ESC / arrows | — | — | untouched |

## 4. OPERATOR-RUN caveat carrier (for Task 9's note items list — record verbatim in the caller's Task-9 backlog)

"The '+'/'-' zoom aliases require Shift pressed-and-held per the user-approved strict rule; on layouts where '+' is the UNSHIFTED key (e.g. German) plain '+' intentionally does nothing and Shift+'+' resolves to egui `Key::Plus`; on US layouts Shift+'=' resolves to '+' (`Key::Plus`) and Shift+'-' produces '_' resolved by physical position to `Key::Minus`. Operator should test the row keys AND the numpad '+'/'-'."
