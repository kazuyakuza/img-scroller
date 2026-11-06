# Wave 4.1 Task 2 — Button Hotkey References + Hover Tooltips: 4.3 Code Simplification Plan

Date: 2026-11-05 · Critical Workflow step 4.3 (code-simplifier) for Wave-4.1 Task 2
Branch: `fix/wave-4-operator-fixes` · HEAD reviewed: `4a2e7f0` (feat commit; `fa8ec7e` planning artifacts out of scope)
Implementation plan reviewed: `.kilo/plans/20261102-wave4-1-task2-references-tooltips-plan.md` (§0 binding decisions + §17 reviewer notes respected — nothing below contradicts them)
Scope reviewed: `src/app.rs`, `src/app/button_hint.rs`, `src/app/floating_controls.rs`, `src/app/fullscreen_button.rs`, `src/app/zoom_button.rs`, `src/app/fit_mode_button.rs`

## Verdict summary

The Task-2 implementation is faithful to the plan: ONE shared column wrapper used by all four
button modules, the two pure string tables land byte-identical to the spec §2/§3 maps (pinned by
the two inline tests), the tooltip pair chains on the same click Response everywhere, the derives
are load-bearing (verified, not removable), no dead code, no unused imports or conversions, no
commented-out code, all files ≤200, all bodies ≤50, depth ≤2. Three behavior-neutral trims apply
(S1–S3): duplicate/verbatim-restatement doc removals in `button_hint.rs` — the Task-2-S1/S2 +
Task-3-S1 precedent class; they change zero tokens of code (188→184). No trim in any other file
qualifies inside the sanctioned classes; everything else considered was REJECTED with reasons.
No behavior/API/test change anywhere. No new TODO file needed — all three steps fit one small fix
commit. `src/app.rs` headroom: NOTHING admissible (see Informational #1 — caller decision reserved).

## S1 — `src/app/button_hint.rs`: remove the `CAPTION_TEXT_SIZE` doc (verbatim value restatement)

Justification: the doc line is `Pill-button caption size (9 pt; the spec's `RichText::size(9.0)`).`
— it restates the const NAME ("caption size" = `CAPTION_TEXT_SIZE`) and the const VALUE (`9.0`
written twice in the doc) for a private const whose single use site (
`RichText::new(reference).size(CAPTION_TEXT_SIZE)`) is self-explanatory. This is the exact
Task-2-simplification-S2 precedent (single-line docs above private glyph-geometry consts deleted;
the surviving landed state of `zoom_button.rs:12–17` is the shape this step recreates). Spec
traceability is unaffected: the 4.5a crosswalk item "Captions `RichText` 9 pt" checks the const
and its use, not the doc, and the plan §19 row cites "§3 consts".

Location: the line `/// Pill-button caption size (9 pt; the spec's `RichText::size(9.0)`).`
(HEAD line 12), directly above `const CAPTION_TEXT_SIZE: f32 = 9.0;`.

Change (exact; delete this ONE line):

```rust
// BEFORE (lines 12–15):
/// Pill-button caption size (9 pt; the spec's `RichText::size(9.0)`).
const CAPTION_TEXT_SIZE: f32 = 9.0;
/// Vertical gap between the painted glyph block and the caption line.
const CAPTION_TEXT_GAP: f32 = 2.0;

// AFTER (interim — S2 deletes the remaining doc line):
const CAPTION_TEXT_SIZE: f32 = 9.0;
/// Vertical gap between the painted glyph block and the caption line.
const CAPTION_TEXT_GAP: f32 = 2.0;
```

Notes for the implementer: do NOT touch either const declaration, its value, or any other line.
Delete ONLY the one `///` line matched by its exact text. Diff: 1 line deleted.

Neutrality verification: comment-only hunk; `wc -l src/app/button_hint.rs` == 187;
`grep -c "Pill-button caption size" src/app/button_hint.rs` == 0; both consts byte-identical
(`git diff` shows a deleted comment line only); rustfmt never reflows code after a comment
deletion and the two-adjacent-bare-consts shape already exists fmt-clean in `zoom_button.rs`.

## S2 — `src/app/button_hint.rs`: remove the `CAPTION_TEXT_GAP` doc (name + use site already say it)

Justification: `/// Vertical gap between the painted glyph block and the caption line.` paraphrases
the const name (`CAPTION_TEXT_GAP`) plus its single use site
(`column_ui.spacing_mut().item_spacing.y = CAPTION_TEXT_GAP;` — the `.y` IS the vertical-gap fact).
Same class as S1 and the Task-2-S2 precedent; the caption/glyph endpoints are obvious from the
column body immediately below. The plan-mandated values (`2.0`, `9.0`) and every code token stay.

Location: the line `/// Vertical gap between the painted glyph block and the caption line.`
(HEAD line 14), directly above `const CAPTION_TEXT_GAP: f32 = 2.0;`.

Change (exact; delete this ONE line):

```rust
// BEFORE (S1 state, lines 12–14 after S1):
const CAPTION_TEXT_SIZE: f32 = 9.0;
/// Vertical gap between the painted glyph block and the caption line.
const CAPTION_TEXT_GAP: f32 = 2.0;

// AFTER:
const CAPTION_TEXT_SIZE: f32 = 9.0;
const CAPTION_TEXT_GAP: f32 = 2.0;
```

Notes for the implementer: apply AFTER S1 (line numbers shift; anchor on the exact `///` text).
Diff: 1 line deleted.

Neutrality verification: comment-only hunk; `wc -l src/app/button_hint.rs` == 186;
`grep -c "Vertical gap between the painted" src/app/button_hint.rs` == 0; harness-neutral by the
same argument as S1.

## S3 — `src/app/button_hint.rs`: remove the `PillButton` enum doc (clause duplicated from the module header)

Justification: `/// All ten pill buttons, keyed by button (the caption/tooltip tables below
resolve per button; no per-module duplicate string tables).` repeats, in different words, the
module header's already-canonical sentence (lines 1–4: "…are keyed here, keyed by button, so no
pill button module duplicates them…"). Task-2-S1 removed the verbatim palette copy and Task-3-S1
removed the paraphrase copy — same rule, same fix: exactly ONE instance of the "keyed by button /
no duplication" claim survives, in the module header. "All ten" adds no information the file
doesn't show (the enum lists ten variants and both 10-assertion tests pin them); the variant names
are self-documenting. 4.5a crosswalk impact: none — its item checks the EXISTENCE of `PillButton`,
the accessors and the helper (§9 "exists and exports" = items, not docs).

Location: the two `///` lines directly above `#[derive(Clone, Copy)]` /
`pub(super) enum PillButton {` (HEAD lines 17–18).

Change (exact; delete these TWO lines):

```rust
// BEFORE (HEAD lines 16–20):
(blank)
/// All ten pill buttons, keyed by button (the caption/tooltip tables below
/// resolve per button; no per-module duplicate string tables).
#[derive(Clone, Copy)]
pub(super) enum PillButton {

// AFTER:
(blank)
#[derive(Clone, Copy)]
pub(super) enum PillButton {
```

Notes for the implementer: the derive attribute, the enum and all ten variants keep their bytes;
no `missing_docs`-style lint is enforced crate-wide (grep-verified: zero `missing_docs` attributes
in `src/`), and sibling bare-enum precedents exist (`ZoomButtonAction`/`FitModeButtonAction`
carry no doc). Diff: 2 lines deleted.

Neutrality verification: comment-only hunks; `wc -l src/app/button_hint.rs` == 184 after S1+S2+S3;
`grep -c "All ten pill buttons" src/app/button_hint.rs` == 0 and
`grep -c "resolve per button" src/app/button_hint.rs` == 0 (both deleted clauses); the canonical
"keyed" claim that SURVIVES is the module header — `grep -n "keyed here" src/app/button_hint.rs`
== exactly 1 hit (line 2; note its "keyed by button" phrase itself wraps lines 2–3, so a
single-line grep for that exact phrase legitimately reads 0 after this step); the two pin tests
and both string tables are untouched → all 20 pinned strings byte-identical.

## Pending review-side adjudication (NO patch from this side — coordination)

1. Paint-chain parameter NAMING: `floating_controls::allocate_and_paint/paint_triangle/arrow_color`,
   `zoom_button::allocate_and_paint/paint_glyph/zoom_glyph_color`,
   `fit_mode_button::allocate_and_paint/paint_glyph/fit_glyph_color` keep the pre-Task-2 `row_ui`
   parameter names although the argument is now the column `Ui` (plan §5.5 deliberately kept the
   signatures: "the paint closure passes `column_ui` into the existing `row_ui`-named parameters").
   A rename is behavior-neutral but is a RENAME, not one of the four sanctioned trim classes, and
   it re-opens a deliberate plan decision → routed to the parallel reviewer; if adjudicated, their
   fix commits stand alone (S1–S3 do not touch any of these hunks).
2. `use PillButton::*;` nested-glob imports in the two inline tests — plan §3 design notes assign
   the preference call to 4.3. Code-simplifier verdict: KEEP (globs are test-scope-local and save
   2 `use` lines in a file with the wave's tightest new-code budget). If the reviewer-side prefers
   explicit variant paths it is their patch; test COUNT (2) and assertions (20) are hard-blocked
   for both sides.

## Considered and REJECTED (do not re-open at 4.5a without a new reason)

1. Extract the caption `if let Some(reference)` out of the `show_button_column` closure (the plan
   §17 explicitly offers this to the simplifier): REJECTED — a new 2-arg private helper plus its
   signature/blank line costs ≈6 lines against the 4 it removes (net worse in the new file's
   budget), the rule already holds (the max-depth article counts nested blocks from the method
   body: closure = 1, `if let` = 2, ≤2 ✓, and the same fn→closure→if-shape landed at
   `show_button_row` since Wave-2), and the wrap's step order is plan-encoded (§0.4).
2. Collapse `PillButtonPlacement::for_button(…)` + `ButtonColumnSite::new(…)` into one nested
   expression per module (~−2 lines × 4 files): REJECTED — the two-step named-intermediate shape is
   plan §0.6's encoding of the ≤2-args compliance (one call taking `row_ui, button, size` =
   3 args = rule break; nesting still constructs both objects and rustfmt-wraps to ≈the same
   lines); the caller's decision space marks the wrapper shape as plan-encoded. Kept.
3. Drop `#[derive(Clone, Copy)]` anywhere: REJECTED — all three derives are LOAD-BEARING, verified:
   `PillButton` Copy is required because the paint closure consumes `button` twice
   (`tooltip_text(button)` line 138 + `hotkey_reference(button)` line 142 → E0382 without it);
   the `ZoomButtonAction`/`FitModeButtonAction` Copy is required to pass `self.action` out of
   `&self` into the `from_*` constructors (§0.5 upgrade, §5.3 "staying out-of-borrow legal").
   Not unused derivations. Kept.
4. Remove any `from_*` conversion or `PillButton` variant: REJECTED — all three constructors have
   exactly one call site each (floating 130, zoom 61, fit 62 — grep-verified), the ten variants are
   all constructed/matched; the keyed-by-button table is §0.5 + spec §5.2 binding ("no label
   duplication", "ONE shared column wrapper"). Kept.
5. Inline the meaningful locals in `show_button_column` (`button`, `column_width`, `tooltip`,
   `column_response`/`.inner`): REJECTED — self-documenting names, each used ≥1× at the exact
   egui-API boundary, ≤1-line savings, per Task-2 REJECTED #9 (no zero-gain line plays).
6. Delete `floating_controls.rs` header lines 6–7 as a duplicate of the `button_hint` module
   header: REJECTED — NOT a verbatim/paraphrase copy (S1-class requires near-identical wording);
   the pair is plan §17's deliberate "reflects the column behavior exactly once" split (consumer
   note vs owner note) and §5.1 explicitly adds the lines. Kept.
7. Delete the `fullscreen_button.rs::fullscreen_square_color` doc (48–49) as a palette-doc twin:
   REJECTED — its parenthetical carries the unique difference fact ("the enabled-arrow palette
   MINUS the disabled branch — always enabled") that the S1/S3-class copies lacked; the lines are
   Task-1-owned, already through their own 4.3, and the file sits 55/200 (zero pressure).
8. `fit_mode_button.rs`: inline `enabled_click_would_fire` into `accepts_click` (a pure-forwarder
   that the sibling zoom/arrow modules inline directly): REJECTED — it is the REVIEWER-SIDE
   adjudicated resolution of the single-section-boolean finding (Wave-4 Task-3 review-fix Step 3,
   commit `4af4c65`, with an explicit asymmetry guard at its §Out-of-scope: zoom's identical idiom
   "pre-existing … outside this task's commit"). Re-opening it in the simplification half is
   forbidden by the coordination rule; the asymmetry stays until the reviewer side re-decides.
9. `zoom_button.rs` (196) / `fit_mode_button.rs` (186) further trims: NONE admissible — the §7.4
   merge already landed the plan's own trim; §17 bars re-opening `row_button` and the shape
   functions; the near-twin `accepts_click` docs and the group-placement docs state each module's
   own contract (not copies — different subjects/verbs; S1 precedent needs a duplicated claim);
   the `widget_info(|| …)` closures and multi-line signatures are rustfmt- or API-forced.
10. app.rs blank-line/mod-region reflow: see Informational #1 (caller-decision lane, precedent
    Task-2 REJECTED #7 + Task-3 REJECTED #3 — "Kept unless the caller explicitly decides
    otherwise"). Not patched here.
11. Tests: census 2/95 is binding (hard block); both bodies are plan §3 verbatim modulo fmt wraps;
    per-item verdict: no change.

## Informational (no patch — caller awareness)

1. **`src/app.rs` at exactly 200/200 (zero headroom).** The caller prefers trims giving app.rs
   headroom, but no admissible app.rs trim exists inside the four sanctioned classes: Task 2
   added exactly two lines (blank + `mod button_hint;`), no docs; every multi-line doc in the file
   joins to >100 columns (no S3-style rewrap possible — all merge lengths 137–210 chars); the
   `use folder_navigation::…` line-42 placement is a move (out of class, pre-Task-2); and the
   mod-line relocation is caller-blocked (structural churn). The only real headroom (~19 lines) is
   collapsing the blank lines between the `mod` declarations — the trim Wave-4 Task-2 REJECTED #7
   and Task-3 REJECTED #3 both re-affirmed as "Kept unless the caller explicitly decides
   otherwise". Wave-4.1 Tasks 3–7 will each want app.rs lines. RECOMMENDATION: the caller decides
   between (a) explicitly authorizing the whitespace collapse in a later step, (b) requiring each
   later plan to carry its own trims, or (c) accepting the cap risk. Not decided in this half.
2. Post-step line budget: `button_hint.rs` 184, `app.rs` 200 (unchanged — cap, flag #1),
   `floating_controls.rs` 191, `fullscreen_button.rs` 55, `zoom_button.rs` 196 (Tasks 5–6 will
   edit this file: their plans must budget its 4 lines of headroom), `fit_mode_button.rs` 186.
3. `hotkey_reference`/`tooltip_text` module-privacy is the §3 recorded deliberate decision
   (prefer-private rule), not dead API surface — do not widen it at 4.5a.

## Per-file verdicts

- `src/app.rs` (200): no admissible trim; headroom → caller decision (Informational #1).
- `src/app/button_hint.rs` (188): S1 + S2 + S3 (→184); derives/conversions/param-objects verified load-bearing or plan-encoded.
- `src/app/floating_controls.rs` (191): NOTHING (header pair plan-encoded; closure wrap rustfmt-forced; naming item pending review-side).
- `src/app/fullscreen_button.rs` (55): NOTHING (fn-path closure already minimal; palette-doc variant kept).
- `src/app/zoom_button.rs` (196): NOTHING (plan §7.4 merge already landed; §17 bars the rest).
- `src/app/fit_mode_button.rs` (186): NOTHING (click-gate indirection is reviewer-owned fix `4af4c65`).

## Verification (owner: caller / implementer — NOT run by this step)

Steps apply S1→S2→S3 sequentially (anchor on exact `///` text, not line numbers). All three are
comment-only edits in ONE file: any harness failure means a typo when applying, not a design
issue. Rerun the container harness (`alpine-vm_vm_status` first, then
`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`):
expect literal `ALL CHECKS PASSED` and `95 passed; 0 failed` (census unchanged). Expected line
counts after applying: `src/app/button_hint.rs` 184; every other file byte-identical
(`git diff --stat` shows exactly one file, three comment-only hunks). Strings/tables neutrality:
the two pinned tests keep all 10 caption + 10 tooltip strings byte-identical; no test, const,
signature or behavior line is touched by S1–S3. Commit form per the precedent style, one small
fix commit: `fix: button-hint review fixes (trim docs duplicated from the module header)`.
