# Wave 4 Task 1 — Fullscreen: 4.3 Code Simplification Plan

Date: 2026-11-02 · Critical Workflow step 4.3 (code-simplifier) for Wave-4 Task 1
Branch: `feat/wave-4-fullscreen-zoom` · HEAD reviewed: `c0e018e`
Implementation plan reviewed: `.kilo/plans/20261102-wave-4-task1-fullscreen-plan.md` (§0 binding decisions respected — nothing below contradicts them)
Scope reviewed: `src/app.rs`, `src/app/empty_state_view.rs`, `src/app/fullscreen.rs`, `src/app/fullscreen_button.rs`, `src/app/floating_controls.rs`, `src/app/fullscreen_tests.rs`

## Verdict summary

The Task-1 implementation is clean: ONE shared toggle action verified at both trigger sites
(no F11-vs-button duplication slipped in), ONE visibility gate, ONE state field, no dead code,
no unused imports, no commented-out code, no needless allocations; all files ≤200 lines, all
method bodies ≤50 lines, depth ≤2, params ≤2, all conditions single-section. Two tiny
API-surface reductions apply (S1, S2). Everything else considered was REJECTED with reasons below.
No behavior changes anywhere. New TODO file NOT needed — both steps fit one small fix commit.

## S1 — `src/app/fullscreen.rs`: drop the unused `bool` return of the shared toggle action

Justification: both callers (`src/app.rs:139` F11 intake, `src/app/fullscreen_button.rs:25` pill
click) invoke it in statement position and ignore the value; no test calls this wrapper
(verified by grep over `src/`). The returned flag has zero consumers → minimal public surface
(`prefer-private-members` spirit). The inner seam `toggle_fullscreen()` keeps its `bool`
return — tests assert on it; only the wrapper's return is removed.

Location: `impl ImagesScrollerApp` block, first method (doc comment + signature + last body line).

Change (exact; replace these lines):

```rust
// BEFORE (lines 14–25):
    /// Toggle action shared by the F11 intake and the pill button: flips the
    /// flag and sends the matching borderless-fullscreen command exactly once,
    /// in the frame the flag flips. Returns the post-toggle flag.
    pub(super) fn toggle_fullscreen_and_send_command(
        &mut self,
        screen_context: &egui::Context,
    ) -> bool {
        let fullscreen_after_toggle = self.toggle_fullscreen();
        screen_context
            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen_after_toggle));
        fullscreen_after_toggle
    }

// AFTER:
    /// Toggle action shared by the F11 intake and the pill button: flips the
    /// flag and sends the matching borderless-fullscreen command exactly once,
    /// in the frame the flag flips.
    pub(super) fn toggle_fullscreen_and_send_command(&mut self, screen_context: &egui::Context) {
        let fullscreen_after_toggle = self.toggle_fullscreen();
        screen_context
            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen_after_toggle));
    }
```

Notes for the implementer: do NOT touch either call site (no `let _ =` — the value was already
discarded); the one-line signature is 96 chars, inside rustfmt's 100-char limit, so `cargo fmt`
keeps it as written. Diff ≈ 6 lines.

## S2 — `src/app/empty_state_view.rs`: tighten `show_centered_open_button` to private

Justification: `prefer-private-members` rule. The ONLY caller anywhere is line 17 of the same
file (`show_empty_state_layout`); grep over `src/` shows zero cross-module and zero test callers.
The plan's Step-2 note ("visibility … `pub(super)`") is factually justified only for
`show_empty_state_layout` (called from `app.rs` chrome dispatch) — not for this inner helper.

Location: `impl ImagesScrollerApp` block, second method signature.

Change (exact, one line):

```rust
// BEFORE (line 23):
    pub(super) fn show_centered_open_button(
// AFTER:
    fn show_centered_open_button(
```

Do NOT change `show_empty_state_layout` (stays `pub(super)`). Diff: 1 line.

## Considered and REJECTED (do not re-open at 4.5a without a new reason)

1. Merge `fullscreen_button.rs` into `fullscreen.rs`: plan §1 file map encodes the 4 new files;
   the split separates state/gate/layout impl methods from the pill widget-draw free function
   (distinct cohesion, mirrors repo one-concern-per-module precedent); both files are at half
   the ideal cap; a merge is a medium restructure with zero measurable complexity win.
2. Dedup `show_fullscreen_layout` vs `show_navigation_layout` (4-line central-panel overlap):
   sharing needs either a mode flag (bad) or restructuring the closure's empty-folder `else` arm,
   which must stay the verbatim 0.3.0 path (§0 "OFF: exact 0.3.0 layout returns"). Overlap kept.
3. Dedup `fullscreen_square_color` with `floating_controls::arrow_color`: reuse would widen the
   `floating_controls` API surface across modules (against `prefer-private-members`), and the
   semantics differ (always-enabled button vs. tri-state arrow palette). Kept separate.
4. Unify `FULLSCREEN_GROUP_GAP` and `FOLDER_GROUP_GAP` into one const: plan §1 explicitly adds
   the new const; they separate two semantically distinct control groups that may evolve
   independently; same value today is coincidence of design, not duplication. Kept.
5. Inline the 3-function division in `fullscreen_button.rs`: it mirrors the
   `paint_triangle`/`arrow_color` split of `ArrowButtonSpec` in `floating_controls.rs`;
   collapsing is style churn with no rule pressure. Kept.
6. Merge `show_chrome_or_fullscreen_layout` + `show_chrome_layout` into one early-return method:
   the two-step split is verbatim plan Step-4b encoding and keeps an auditable
   "0.3.0 chrome dispatch untouched" layer (§0 extended-by-composition). Kept.
7. Test trimming: count (7) and per-test subjects are binding; apparent assertion overlaps in
   tests 1/4/5 are required subjects. No test change.
8. Naming: everything is self-documenting; `fullscreen_on` name is §0-binding; no rename.

## Verification (owner: caller / implementer — NOT run by this step)

After S1+S2, rerun the container harness (`vm_status` first, then
`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`):
expect literal `ALL CHECKS PASSED` and 57/57 tests. Neither step changes behavior, so any
failure means a typo in applying the patch, not a design issue.
Commit form per the implementation plan §5: `fix: fullscreen review fixes (drop unused toggle
return, tighten open-button helper visibility)`.
