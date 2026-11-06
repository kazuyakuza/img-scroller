# Wave 4.1 — Task 4 Investigation + Implementation Plan: Remove the stray zoom message

Status: PLAN ONLY (Critical Workflow step 4.1a investigation; caller-authorized combined 4.1a+4.1b for this SMALL task). No implementation performed in this step.

- TODO source: `.agent/todos/20261102/20261102-todo-1.md` Task 4 (verbatim binding in §0).
- Branch verified: `fix/wave-4-operator-fixes`; HEAD verified: `9c0625a` (`9c0625aa97f051fe14565c39a8aaefe36674f0b3`, "task: mark wave4.1 task3 fullscreen tuning done"); working tree CLEAN — no untracked/stray files. (Note: the project-info context log still records HEAD `db83dd4`; the actual repo HEAD is `9c0625a` — the Task-3 mark commit. This plan targets `9c0625a`.)

## 0. Binding (user-approved, verbatim 2026-11-02)

"Remove all that zoom related msgs. It is not required. The user will understand when tries to keep zooming in/out an nothing happens."

Concretely encoded here: REMOVE the zoom rejected-step MESSAGE writes (`Zoom max` / `Zoom min`) from ALL zoom paths — keyboard chords AND pill buttons. Nothing about zoom may write to `message_text` anymore. The visual affordance (button dimming/enablement at bounds) STAYS untouched. Fit-activation messages (`Fit width`/`Fit height`) and the unsupported-file message are NOT part of this task (Task 7 re-decides fit messages).

## 1. Investigation findings (code audit at HEAD `9c0625a`)

### 1.1 Where the "zoom in"-class message came from

The only zoom message writers in the entire tree are the two trap methods in `src/app/zoom.rs`:

- `src/app/zoom.rs` line 103: `self.message_text = Some(ZOOM_AT_MAXIMUM_MESSAGE);` inside `trap_zoom_maximum` (lines 99–105)
- `src/app/zoom.rs` line 112: `self.message_text = Some(ZOOM_AT_MINIMUM_MESSAGE);` inside `trap_zoom_minimum` (lines 107–114)

These fire when a zoom step is attempted while already at a bound (the D2 rejected-step scheme). The operator's observed "zoom in"-class message is `Zoom max` (written on a re-attempted zoom-in at the ×8 ceiling), e.g. pressing/holding the `+` path after hitting the cap.

### 1.2 Every zoom path funnels through the same two writers (single removal point)

- Keyboard path: `src/app/keyboard_intake.rs` lines 44–49 dispatch `self.zoom_in_once()` / `self.zoom_out_once()` for the `I`/`Shift+Plus` and `O`/`Shift+Minus` chords.
- Button path: `src/app/zoom_button.rs` `apply_action` (lines 89–95) calls `app.zoom_in_once()` / `app.zoom_out_once()` / `app.restore_zoom_to_fit()` for the pill `[+][−][restore-fit]` buttons.

Both paths call the shared app-seam methods in `src/app/zoom.rs`, so removing the message writes INSIDE those methods sanitizes both paths at once. No writer exists anywhere else (full grep of `ZOOM_AT_(MIN|MAX)IMUM_MESSAGE`, `Zoom min`, `Zoom max`, and all `message_text` sites confirmed).

### 1.3 Exact removal surface

REMOVE (all in `src/app/zoom.rs` unless stated):

1. Constant `ZOOM_AT_MINIMUM_MESSAGE` + its doc comment (lines 41–43).
2. Constant `ZOOM_AT_MAXIMUM_MESSAGE` + its doc comment (lines 44–45).
3. The `self.message_text = Some(ZOOM_AT_MAXIMUM_MESSAGE);` write inside `trap_zoom_maximum`.
4. The `self.message_text = Some(ZOOM_AT_MINIMUM_MESSAGE);` write inside `trap_zoom_minimum`.
5. The two `trap_zoom_*` methods are DELETED and their single-call predicate reads are inlined as single-section guard conditions inside the two step methods (exact shapes in §2 Step 2.3) — the "trap" name is message-era baggage; the inline single-call guards keep self-documenting code.
6. Doc-comment claims about the message scheme: zoom.rs module doc lines 1–8 ("at-bound pill-message assignment", the D2 "pill message fires ONLY…" sentences) and fn docs lines 48 / 56 ("reports \"Zoom max\"" / "reports \"Zoom min\"").

KEEP (intact, do not touch):

- `message_text` field and channel in `src/app.rs` (line 29) — the ONE channel stays for: unsupported-file message (`app.rs` lines 87/92, `image_loading::UNSUPPORTED_FILE_MESSAGE`), fit activations (`app/zoom.rs` `announce_fit_mode` → `fit_mode_activation_label`, lines 75–156), and the clearing writers (`folder_navigation.rs` line 107; `accept_open_outcome` clear).
- Bounds math and at-bound predicates in `src/zoom.rs` (`ZOOM_FACTOR_MINIMUM` = 1.0 / `ZOOM_FACTOR_MAXIMUM` = 8.0, tolerance, `*_is_at_minimum/maximum`) — BYTE-IDENTICAL (Task 5 re-encodes bounds; this plan must not pre-bake the symmetric-floor math).
- Button enablement/dimming: `zoom_can_grow` / `zoomed_beyond_fit` (app/zoom.rs lines 85–94) + `zoom_button_specs` (zoom_button.rs lines 45–51) + glyph-color dimming — the operator's bound affordance stays.
- `button_hint.rs` tooltips/captions ("Zoom in (I)" etc.) mention NOTHING about messages — untouched (verified).
- `floating_controls.rs` message-line renderer (`show_message_line`, generic over `message_text`) — untouched.
- Chord functions (`zoom_in_chord_was_pressed`, aliases) — untouched (Task 6 owns key matching).
- `set_fit_mode` / `restore_zoom_to_fit` behavior — untouched (Task 7 owns semantics).

### 1.4 Test fate + census delta (currently 97, verified by grep of `#[test]`)

| Test file | Test | Fate |
|---|---|---|
| `src/app/zoom_tests.rs` | `zoom_out_at_fit_is_rejected_and_reports_zoom_min` (lines 36–42) | REWORK: rename to `zoom_out_at_fit_is_rejected_silently`; drop the `ZOOM_AT_MINIMUM_MESSAGE` literal assertion; ADD `assert!(app.message_text.is_none());` (behavioral: zoom writes nothing) |
| `src/app/zoom_tests.rs` | `repeated_zoom_in_reaches_the_ceiling_then_reports_zoom_max` (lines 44–55) | REWORK: rename to `repeated_zoom_in_reaches_the_ceiling_then_stays_there`; drop the `ZOOM_AT_MAXIMUM_MESSAGE` literal; ADD `assert!(app.message_text.is_none());` after the extra at-cap step; keep factor-unchanged-at-8.0 assertions |
| `src/app/zoom_tests.rs` | `zoom_bound_feedback_texts_are_pinned` (lines 98–102) | DELETE — its subject (the two constants) is gone entirely |
| `src/app/fit_mode_tests.rs` | `fit_activation_message_overwrites_the_zoom_bound_message` (lines 65–72) | REWORK: rename to `zoom_out_at_fit_is_silent_then_fit_width_announces`; first assert `message_text.is_none()` after an at-fit zoom-out (no zoom message), then keep the `FIT_WIDTH_MESSAGE` overwrite assertion (fit constants stay) |

Untouched message assertions elsewhere (unsupported-file, fit-activation, folder-navigation) stay valid.

Census: 97 − 1 (deleted) = **96** (reworked tests keep their `#[test]` entries). `src/zoom.rs` inline tests (8) untouched.

### 1.5 Doc-comment victims (summary)

- `src/app/zoom.rs`: module doc lines 1–8 (message-scheme sentences), fn docs lines 48, 56, trap docs lines 96–98 + 107 (died with the methods).
- `src/zoom.rs`: module doc line 5–7 sentence "…so the rejected-step pill feedback and the reset-on-every-successful-open policy both live in the app seam" — remove the pill-feedback half.
- `src/app/keyboard_intake.rs`: doc lines 21–22 "Dispatch order is significant: the fit activations follow the zoom chords, so a same-frame fit message overwrites a zoom-bound message." — remove (no message significance remains); keep the "one read per chord" line.
- `src/app/zoom_tests.rs`: module doc line 2 "rejected-step feedback" — reword to the silent at-bound no-op.
- `src/app/fit_mode_tests.rs`: module doc mentions "feedback" generically — no change needed beyond §2 test task.

Out-of-src prose docs (`README.md`, `tech.md`, `architecture.md`, `context.md`) still describe the `Zoom max`/`Zoom min` scheme — NOT touched by this plan (Task 9 owns project docs). No `.agent/project-structure.md` change: no files added/removed.

## 2. Implementation plan (ordered, junior-executable)

### Step 2.1 — Verify start state (no writes)

1. `git branch --show-current` → `fix/wave-4-operator-fixes`; `git log -1 --format="%H %s"` → starts `9c0625a`.
2. `git status --short` → empty.

### Step 2.2 — Commit this plan file

3. `git add .kilo/plans/20261102-wave4-1-task4-message-removal-plan.md` then `git commit -m "docs: add wave4.1 task4 message-removal plan"`.

### Step 2.3 — Code change 1 of 3: `src/app/zoom.rs`

4. REPLACE the module doc lines 1–8 WITH:

```rust
//! Manual zoom (Wave-4 Task 2): the session zoom-factor seam, its keyboard
//! chords (I / Shift+Plus, O / Shift+Minus, 0 = restore fit), and the
//! at-bound step guards. Buttons live in `zoom_button.rs`; the pure math
//! lives in `crate::zoom`. Session state only: never persisted. A zoom
//! step attempted at a bound it already holds is a silent no-op (the
//! factor is unchanged and NO message is written); a step that merely
//! lands on a bound clamps silently too (D2). Zoom actions write NOTHING
//! to `message_text` (wave 4.1 Task 4). The reset-on-every-successful-open
//! policy is NOT here -- it lives in `src/app.rs` (`accept_open_outcome`).
```

(Keep the existing fit-mode paragraph ("Fit modes (Wave-4 Task 3) share this seam — … all funnel through it") and the "AI Agents" paragraph below it unchanged — both are still true. Pre-edit references above use the original line numbers; after the module-doc edit all later line numbers SHIFT DOWN by ~2 lines — always locate edits by CONTENT, never by stale line numbers.)

5. DELETE lines 41–45 (both `ZOOM_AT_*` constants + their doc comments). `FIT_WIDTH_MESSAGE`/`FIT_HEIGHT_MESSAGE` (lines 36–39) stay.
6. REPLACE `zoom_in_once` + `zoom_out_once` (+ their doc lines 48, 56) WITH:

```rust
    /// One zoom-in step; a step attempted at the ceiling is a silent no-op.
    pub(super) fn zoom_in_once(&mut self) {
        if zoom::zoom_factor_is_at_maximum(self.zoom_factor) {
            return;
        }
        self.zoom_factor = zoom::zoom_in_factor(self.zoom_factor);
    }

    /// One zoom-out step; a step attempted at the fit floor is a silent no-op.
    pub(super) fn zoom_out_once(&mut self) {
        if zoom::zoom_factor_is_at_minimum(self.zoom_factor) {
            return;
        }
        self.zoom_factor = zoom::zoom_out_factor(self.zoom_factor);
    }
```

7. DELETE `trap_zoom_maximum` (lines 96–105 incl. doc) and `trap_zoom_minimum` (lines 107–114 incl. doc) entirely. (The guarded no-op now lives inside the two step methods; each `if` condition is a single call — Single-Section Boolean rule holds. The zoom-out guard is kept despite today's clamp overlap so Task 5 re-encodes the rejected-step shape, not this plan.)

### Step 2.4 — Code change 2 of 3: `src/zoom.rs` (doc comment only)

8. In the module doc, REPLACE the sentence "Authoritative for the MATH only: the clamp below is silent, so the rejected-step pill feedback and the reset-on-every-successful-open policy both live in the app seam (`src/app/zoom.rs` and the `accept_open_outcome` wiring in `src/app.rs`)." WITH: "Authoritative for the MATH only: the clamp below is silent and zoom actions write no messages, so the silent at-bound no-op and the reset-on-every-successful-open policy both live in the app seam (`src/app/zoom.rs` and the `accept_open_outcome` wiring in `src/app.rs`)." — constants/functions/lines 9–111 byte-identical.

### Step 2.5 — Code change 3 of 3: `src/app/keyboard_intake.rs` (doc comment only)

9. DELETE doc lines 21–22 (the "Dispatch order is significant … overwrites a zoom-bound message." sentence). Keep lines 19–20 ("Reads every chord…", "One `key_pressed` read per chord…") and all code untouched.

### Step 2.6 — Test changes

10. `src/app/zoom_tests.rs` module doc lines 1–3: replace line 2 "the session zoom factor seam, its bounds and rejected-step feedback, the reset-on-open policy," with "the session zoom factor seam, its bounds and silent at-bound steps, the reset-on-open policy,".
11. REPLACE test `zoom_out_at_fit_is_rejected_and_reports_zoom_min` (lines 36–42) WITH:

```rust
#[test]
fn zoom_out_at_fit_is_rejected_silently() {
    let mut app = ImagesScrollerApp::empty_state();
    app.zoom_out_once();
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
}
```

12. REPLACE test `repeated_zoom_in_reaches_the_ceiling_then_reports_zoom_max` (lines 44–55) WITH:

```rust
#[test]
fn repeated_zoom_in_reaches_the_ceiling_then_stays_there() {
    let mut app = ImagesScrollerApp::empty_state();
    for _ in 0..10 {
        app.zoom_in_once();
    }
    assert!(!app.zoom_can_grow());
    assert_eq!(app.zoom_factor, 8.0);
    app.zoom_in_once();
    assert!(app.message_text.is_none());
    assert_eq!(app.zoom_factor, 8.0);
}
```

13. DELETE test `zoom_bound_feedback_texts_are_pinned` (lines 98–102). After this, `use …zoom;` remains used (other tests reference `zoom::RESTORE_FIT_KEY` etc.).

14. In `src/app/fit_mode_tests.rs`, REPLACE test `fit_activation_message_overwrites_the_zoom_bound_message` (lines 65–72) WITH:

```rust
#[test]
fn zoom_out_at_fit_is_silent_then_fit_width_announces() {
    let mut app = ImagesScrollerApp::empty_state();
    app.zoom_out_once();
    assert!(app.message_text.is_none());
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    assert_eq!(app.message_text, Some(zoom::FIT_WIDTH_MESSAGE));
}
```

(`zoom::FIT_WIDTH_MESSAGE` import stays valid; all other `message_text` assertions in that file stay.)

### Step 2.7 — Harness verification (container; exit code authoritative)

15. `alpine-vm` MCP: call `vm_status` FIRST; stop with a user question if the VM or `/img-scroller` share is unavailable.
16. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
    Expect: `PASS fmt-check`, `PASS check`, `PASS test … 96 passed; 0 failed`, `PASS clippy`, final `ALL CHECKS PASSED` (exit 0). The exact test count 96 is the Task-4 success evidence.
17. If fmt fails: `cargo fmt` fixes are NOT applied by the harness — the implementer applies `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` and re-runs the harness.

### Step 2.8 — Commit (feature)

18. Read `.gitignore`; stage ONLY the five touched sources: `git add src/app/zoom.rs src/zoom.rs src/app/keyboard_intake.rs src/app/zoom_tests.rs src/app/fit_mode_tests.rs`. Verify `git status` shows nothing else staged (no `target/`, `logs/`, `dist/`).
19. `git commit -m "fix: remove zoom bound messages"`.

### Step 2.9 — Code review pass (junior implementer self-review checklist)

- [ ] `grep -rn "ZOOM_AT" src` → zero hits.
- [ ] `grep -rn "Zoom min\\|Zoom max" src` → zero hits.
- [ ] `grep -rn "trap_zoom" src` → zero hits.
- [ ] `grep -n "message_text" src/app/zoom.rs` → exactly ONE code hit (the fit-activation write `self.message_text = fit_mode_activation_label(fit_mode);`) plus at most the module-doc mention of the no-write policy.
- [ ] `src/zoom.rs` diff shows ONLY the module-doc sentence change (constants/functions untouched).
- [ ] `keyboard_intake.rs` diff shows ONLY the deleted doc sentence; all reads/dispatch unchanged.
- [ ] `zoom_button.rs`, `button_hint.rs`, `floating_controls.rs`, `fit_mode_button.rs`, `central_surface.rs` untouched.
- [ ] Test census still exactly 96 in the harness output.
- [ ] Line budgets: `src/app/zoom.rs` ≈ 135 (well under the 200 cap).

## 3. Out-of-scope guard (hard boundaries for the implementer)

1. Fit-activation messages (`FIT_WIDTH_MESSAGE`/`FIT_HEIGHT_MESSAGE`, `announce_fit_mode`, `fit_mode_activation_label`) — UNTOUCHED; Task 7 re-decides them.
2. Unsupported-file message writer in `src/app.rs` — UNTOUCHED.
3. Button dimming/enablement at bounds (`zoom_can_grow`, `zoomed_beyond_fit`, spec enablement args) — UNTOUCHED (stays as the bound affordance).
4. `src/zoom.rs` bounds (1.0/8.0), step multiplier, tolerance, predicates, `zoomed_draw_size` — BYTE-IDENTICAL; do NOT re-encode Task 5's symmetric ÷8 floor here.
5. Key/chord matching (`zoom_in_chord_was_pressed`, alias functions, `keyboard_intake.rs` code) — UNTOUCHED; Task 6 owns Shift-alias corrections.
6. `set_fit_mode` / FitMode composition / open-reset policy — UNTOUCHED; Task 7 owns the one-shot remap.
7. `message_text` field/channel + `show_message_line` renderer — UNTOUCHED (the one channel survives for remaining users).
8. `README.md`, `tech.md`, `architecture.md`, `context.md`, `brief.md`, `product.md` — UNTOUCHED here (Task 9). Do NOT touch `CHANGELOG.md`. Do NOT modify the TODO file (caller marks at 4.6). Do NOT edit existing `.kilo/plans/*` files.
9. No key/edge behavior changes: the rejected-step no-op (factor unchanged at a bound) is preserved as behavior — only the message write disappears.

## 4. Pre-commit checklist (Gitignore Compliance Rule)

- [ ] `.gitignore` read before commit.
- [ ] `git status --short` reviewed; staged set = exactly the 5 source files (plan file committed separately at Step 2.2); no `target/`, `logs/`, `dist/`, or dependency dirs staged.
- [ ] No force-add of ignored files (`git add -f` never used).
- [ ] Diff review performed with `git diff --staged` before committing.
- [ ] Commit message exactly `fix: remove zoom bound messages` (no emoji, no scope prefix).

## 5. Return-digest (for the caller)

- Exact removal surface: the two `ZOOM_AT_*` constants + the two message writes — `src/app/zoom.rs` receives the ONLY code change; doc-only touches land in `src/zoom.rs` + `src/app/keyboard_intake.rs`; both chord and button paths funnel through the one writer pair — single point of removal, channel `message_text` stays for fit + unsupported-file uses.
- Test-count delta: 97 → 96 (1 test deleted `zoom_bound_feedback_texts_are_pinned`; 3 tests reworked to silent-no-op behavioral assertions: 2 in `zoom_tests.rs`, 1 in `fit_mode_tests.rs`).
- Doc-comment victims: `app/zoom.rs` module-doc message scheme + two fn docs + trap docs; `src/zoom.rs` module-doc pill-feedback sentence; `keyboard_intake.rs` dispatch-order/message sentence; `zoom_tests.rs` module-doc "rejected-step feedback" wording. Tooltips/button-hint/floating_controls: NO message mentions — untouched. Prose docs deferred to Task 9.

END OF PLAN — implementer must not execute beyond §2 Step 2.9/caller-gated steps.
