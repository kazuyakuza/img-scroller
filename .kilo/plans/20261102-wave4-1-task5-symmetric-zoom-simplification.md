# Wave 4.1 Task 5 — Symmetric Zoom Floor: 4.3 Code Simplification Plan

Date: 2026-11-06 · Critical Workflow step 4.3 (code-simplifier) for Wave-4.1 Task 5
Branch: `fix/wave-4-operator-fixes` · HEAD reviewed: `d4c6870` (`fix: allow zoom out below fit with symmetric floor`)
Implementation plan reviewed: `.kilo/plans/20261102-wave4-1-task5-symmetric-zoom-plan.md` (§0/§1 bindings, silent steps §0.2, census 97 §2.5, hard blocks §4 — nothing below contradicts them)
Scope reviewed (exactly the HEAD diff set): `src/zoom.rs` (128), `src/app/zoom.rs` (142), `src/app/zoom_button.rs` (196), `src/app/zoom_tests.rs` (125), `src/app/fit_mode_tests.rs` (110 — caller brief said 111; verified 110 by `Get-Content).Count`)

## Verdict summary

The Task-5 implementation is faithful to the plan: constants re-pinned exactly per Steps 1.2/1.3, the predicate rename/re-pin of Step 2.1 landed byte-faithful (modulo em-dash→`--` ASCII), `zoomed_beyond_fit` is grep-dead (0 hits, Step 4 grid-check satisfied), the two `zoom_button_specs` lines match Step 3.1 exactly, and the census is verified 97 by grep at HEAD (8+11+11+67). No dead code (clippy `-D warnings` gate passed at HEAD), no unused imports, no commented-out code, all files ≤200, bodies ≤50, depth ≤2, params ≤2. Exactly ONE behavior-neutral trim applies (S1): the pre-existing trailing comment on `ZOOM_BOUND_TOLERANCE` in `src/zoom.rs` — a verbatim value + guard restatement of the adjacent plan-pinned doc line (the Task-2.1-S1/S2 precedent class). Zero code tokens change; census, line count, names, and every plan-encoded sentence are untouched. Everything else in the diff set is plan-encoded, test-verbatim, or sits on a Task-6/Task-7 seam and was REJECTED with reasons. No new TODO file needed — the single step fits one small fix commit.

## S1 — `src/zoom.rs`: delete the trailing restatement comment on `ZOOM_BOUND_TOLERANCE`

Justification: line 26 reads
`const ZOOM_BOUND_TOLERANCE: f32 = 1.0e-4; // 0.0001 f32 epsilon-class guard`.
The trailing `//` segment restates the const VALUE (`0.0001` is the literal of `1.0e-4`, written twice on one line) and the word `guard` copies the retained doc line 25 above it (`/// Absorption bound for the at-bound detections (fits-factor drift guard).`) — the exact Task-2.1 S1/S2 precedent class (value-restating docs above private consts deleted; `button_hint.rs` `CAPTION_TEXT_SIZE` is the canonical instance). This is pre-existing line-level content from Wave-4 Task 2 (it appears UNCHANGED as a context line in the `d4c6870` diff), so Task-5 plan Step 1.1's "NOT touched (verify only)" — which binds the implementation step's code surface — does not bar a 4.3 comment trim; the constant's NAME, VALUE, the `///` doc sentence, and semantics all stay byte-identical (the §0.3 binding keeps its full force). Zero churn risk for incoming tasks: Task 7, if it retunes the tolerance value at all, still edits exactly one short line (the comment's presence made that line longer, not shorter).

Location: `src/zoom.rs`, HEAD line 26, directly below the `/// Absorption bound ...` doc.

Change (exact; delete ONLY the trailing comment segment, keep the code token and semicolon):

```rust
// BEFORE (line 26):
const ZOOM_BOUND_TOLERANCE: f32 = 1.0e-4; // 0.0001 f32 epsilon-class guard

// AFTER:
const ZOOM_BOUND_TOLERANCE: f32 = 1.0e-4;
```

Notes for the implementer: match on the exact text `1.0e-4; // 0.0001 f32 epsilon-class guard` and delete the ` // 0.0001 f32 epsilon-class guard` tail. Do NOT touch line 25 (the `///` doc line SURVIVES — it is the single canonical statement), do NOT touch the value `1.0e-4`, do NOT touch any other line in the file. Diff: 1 line modified (−1/+1).

Neutrality verification: comment-only hunk; `src/zoom.rs` stays 128 lines (inline-comment deletion);
`rg -c "0.0001" src/zoom.rs` == 0 and `rg -c "epsilon-class" src/zoom.rs` == 0 (both deleted tails);
`rg -n "ZOOM_BOUND_TOLERANCE" src/zoom.rs` == 4 hits (1 definition + the three predicate bodies at lines 46/51/58 — unchanged); the retained doc line 25 is byte-identical (`fits-factor` wording is a reviewer-side item, see Pending #1). rustfmt never reflows a shortened comment line (result is 41 chars, far under 100). Census 97 unchanged; no const, predicate, test, or behavior token is touched.

## Considered and REJECTED (do not re-open at 4.5a without a new reason)

1. Inline `zoom_factor_is_off_restore_target` (`src/zoom.rs:57–59`) into its single app consumer `zoomed_off_fit`: REJECTED — the fn is plan Step 1.5 verbatim (plan-encoded, hard block), and inlining moves the expression into the app seam where `ZOOM_BOUND_TOLERANCE` is PRIVATE to `crate::zoom`: sharing it needs a cross-module visibility widening = an API change outside the decision space. Kept.
2. Unify the qualification style in `src/app/zoom.rs` (`zoom::...` alias for most math reads vs `crate::zoom::RESTORE_TO_FIT_FACTOR` at line 74): REJECTED — line 74 is the `set_fit_mode` body, which plan Step 2.4 pins ("NO change") and which sits ON the Task-7 fit seam (hard block); the fully-qualified form also matches the module doc's own convention note (lines 17–20). Kept.
3. Collapse the one-line forwarder `announce_fit_mode` (lines 78–80) into `set_fit_mode`: REJECTED — fit seam (Task 7 owns the whole mode-set-and-announce block's fate per the wave TODO Task-7 remap); trimming it now churns exactly the seam Task 7 will rewrite. Kept.
4. Trim the near-duplicate alias claims between the `ZOOM_IN_KEY`/`ZOOM_OUT_KEY` docs (lines 27–30: "alias family: the Shift+'+' chord below") and the chord-fn docs (lines 102–104, 122–123: "hotkey `I`, or the Shift+'+' alias…" incl. the D4 "BOTH egui logical resolutions" sentence): REJECTED — keyboard alias territory; Task 6 rewrites both the matching code and these docs (the wave's deferred-decisions list already SUPERSEDES the D4 double-resolution). Editing either side now is churn against the next task's lane (explicit caller preference). Kept.
5. Inline `clamped_zoom_factor` (`src/zoom.rs:39–41`) into the two step fns: REJECTED — plan Step 1.1 names it "NOT touched… no edit", and it single-sources the `clamp(ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM)` bound pair; inlining duplicates the pair in two places (net worse). Kept.
6. `src/app/zoom_button.rs` (196) — any trim: REJECTED — Task 5 touched only the two `zoom_button_specs` predicate lines (plan Step 3.1 encoded); Task-2.1 simplification REJECTED #9 already adjudicated this file "NONE admissible" with per-hunk reasons and no new reason exists; Informational #2 there reserves its 4 headroom lines for Task 6. Kept.
7. `src/app/zoom_tests.rs` (125) and `src/app/fit_mode_tests.rs` (110) — any trim, including the inline `//` assertions comments: REJECTED — every body and inline comment is plan §2.2/§2.3 verbatim snippet content (the `0.107 < 0.125`, `symmetric inverse`, `mid-range now`, `exact in f32` annotations are part of the pinned snippets); the 97 census is a hard block and the precedent rule is per-item "no change" (Wave-4 Task-2 #10, Task-2.1 #11). Kept.
8. Module docs (`src/zoom.rs:1–10`, `src/app/zoom.rs:1–20`) and the `zoom_out_once` method-doc's floor-sentence near-twin of the module-doc sentence (line 53 vs lines 8–9): REJECTED — both are plan-encoded verbatim contract statements (Steps 1.6, 2.3, 2.5 list the exact required sentences; Step 1 says "the doc comments listed below are CONTRACT statements, keep them"). The claim duplication is the plan's deliberate module-contract/method-statement layering, not a copy trim. Kept.
9. Dead-code sweep (the "exact-dead-code census exception"): NONE FOUND — `zoomed_beyond_fit` is fully removed (grep 0 hits), every `pub(crate)`/`pub(super)` name in the diff set has a live consumer (grep-verified: `ZOOM_FACTOR_MINIMUM`/`RESTORE_TO_FIT_FACTOR`/`zoom_factor_is_off_restore_target` each read from the app seam/tests), clippy `-D warnings` passed at HEAD. No census exception applies.

## Pending review-side adjudication (NO patch from this side — coordination)

1. `src/zoom.rs:25` doc parenthetical `(fits-factor drift guard)` — wording-accuracy call (reads as a "fit-factor"/"fits the factor" typo); accuracy fixes are outside this side's four sanctioned classes. Sequencing note: if the reviewer-side edits line 25, apply S1 first (S1 lands on line 26 only); their patch stands alone and this plan's grep checks are written so they do not collide.
2. Carried-over, NOT re-opened here: the Task-2.1 Pending #1 `row_ui` parameter-naming item in the paint chains (includes `zoom_button.rs`) — still reviewer-side; S1 does not touch that file.

## Informational (no patch — caller awareness)

1. Line budget after S1: `src/zoom.rs` 128 (unchanged — inline-comment delete), `src/app/zoom.rs` 142, `src/app/zoom_button.rs` 196 (4 lines headroom reserved for Task 6 per Task-2.1 Informational #2), `src/app/zoom_tests.rs` 125, `src/app/fit_mode_tests.rs` 110 (brief said 111; verified 110 — no action).
2. Census verified independently at HEAD by grep `#[test]` across `src`: 97 total (zoom.rs 8, zoom_tests 11, fit_mode_tests 11, rest 67). Tasks 6/7 plans must re-pin their own expected totals from 97.
3. `src/app/zoom.rs` lines 61–80 + 136–142 (restore/set_fit_mode/announce/label chain) are byte-stable at d4c6870+S1 for Task 7's remap; nothing in this plan consumes or churns them.

## Per-file verdicts

- `src/zoom.rs` (128): S1 (trailing-comment delete → still 128); constants/predicates/tests plan-encoded, kept.
- `src/app/zoom.rs` (142): NOTHING (predicate block + docs plan-encoded; chords = Task 6 lane; fit chain = Task 7 lane).
- `src/app/zoom_button.rs` (196): NOTHING (two propagated lines plan-encoded; prior adjudication stands).
- `src/app/zoom_tests.rs` (125): NOTHING (verbatim snippets + census hard block).
- `src/app/fit_mode_tests.rs` (110): NOTHING (verbatim snippet + census hard block; Task 7 rework lane).

## Verification (owner: caller / implementer — NOT run by this step)

Apply S1 alone (one exact-match edit in one file). Rerun the container harness per `docs/build.md`
(`alpine-vm_vm_status` first, then `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`):
expect literal `ALL CHECKS PASSED` and `97 passed; 0 failed` (census unchanged — a comment-only edit cannot alter it; any other total means a misapplied edit, revert and re-apply). `git diff --stat` must show exactly ONE file: `src/zoom.rs`, 1 insertion + 1 deletion, no whitespace hunks elsewhere. S1 grep checks: `0.0001` → 0 hits, `epsilon-class` → 0 hits, `ZOOM_BOUND_TOLERANCE` → 4 hits in `src/zoom.rs`. No const value, name, predicate, test, or behavior line is touched. Single small fix commit, precedent form (caller-approved wording): `fix: symmetric-zoom review fixes (trim restated tolerance-value comment)`. No push; TODO `[DONE]` marking belongs to 4.6, not this step.
