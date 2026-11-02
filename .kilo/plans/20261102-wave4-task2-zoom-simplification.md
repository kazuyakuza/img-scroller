# Wave 4 Task 2 — Zoom In/Out: 4.3 Code Simplification Plan

Date: 2026-11-02 · Critical Workflow step 4.3 (code-simplifier) for Wave-4 Task 2
Branch: `feat/wave-4-fullscreen-zoom` · HEAD reviewed: `e55838f` (feature commit; `ef60c80`/`e5a6f5d` planning artifacts out of scope)
Implementation plan reviewed: `.kilo/plans/20261102-wave-4-task2-zoom-plan.md` (§0 binding D1–D9 respected — nothing below contradicts them)
Scope reviewed: `src/zoom.rs`, `src/app/zoom.rs`, `src/app/zoom_button.rs`, `src/app/keyboard_intake.rs`, `src/app/zoom_tests.rs`, `src/app.rs`, `src/main.rs`, `src/app_display.rs`, `src/app/fullscreen.rs`, `src/app/floating_controls.rs`

## Verdict summary

The Task-2 implementation is clean and faithful to the plan: the intake extraction is
verified copied-behavior (S4 check passed — no logic drift vs the pre-extraction block), the
shared-action seams are single-source (§12.4 — no duplicate zoom implementation), no dead code
(harness clippy runs `-D warnings` and passed — any unused seam would have failed the build),
no unused imports, no commented-out code, no needless clones, all files ≤200 lines, all method
bodies ≤50 lines, depth ≤2, params ≤2, all conditions single-section. Three tiny, purely
non-functional doc-surface trims apply (S1–S3); they change zero code and buy line-cap headroom
in the two pressure files (`zoom_button.rs` 192→188, `app.rs` 198→197). Everything structural
considered was REJECTED with reasons below. No behavior/API change anywhere. No new TODO file
needed — all three steps fit one small fix commit.

## S1 — `src/app/zoom_button.rs`: remove the palette doc duplicated verbatim from `ArrowButtonSpec::arrow_color`

Justification: lines 93–94 are a word-for-word copy of the doc on `floating_controls.rs`
`arrow_color` (lines 168–169 — task-1 simplification plan REJECTED #3 already established the
two color ladders stay as separate functions; only the duplicate documentation is removed). The
three-return ladder is self-evident from the code (`noninteractive`/`hovered`/`inactive` appear
in each return expression). Removing the copy keeps one canonical instance of the wording.

Location: `impl ZoomButtonSpec`, method `zoom_glyph_color` — doc comment only, not the signature
or body.

Change (exact; delete these two lines):

```rust
// BEFORE (lines 93–94, immediately above the fn signature):
    /// Default egui button-text palette: idle in `inactive`, brightened on
    /// hover, dimmed in `noninteractive` while disabled (exact visual states).

// AFTER: (no doc line; `fn zoom_glyph_color(` follows the previous method's closing `}` directly)
```

Notes for the implementer: delete ONLY the two `///` lines; do not touch the function, its
signature, or any other file. Diff: 2 lines deleted.

Verification claim: `wc -l src/app/zoom_button.rs` == 190 after this step and `grep -n
"Default egui button-text palette" src/app/zoom_button.rs` returns nothing (still exactly one
hit in `floating_controls.rs`); harness `ALL CHECKS PASSED` with 76/76 tests (a comment-only
edit cannot alter behavior).

## S2 — `src/app/zoom_button.rs`: drop the two single-line docs above the glyph-geometry const pairs

Justification: `GLYPH_BAR_LENGTH`/`GLYPH_BAR_THICKNESS` (lines 13–15) and `FIT_MARKER_SIDE`/
`FIT_MARKER_STROKE_WIDTH` (lines 16–18) are private consts whose names already state the
`self-documenting-code` rule's requirement ("bar length/thickness", "marker side/stroke width");
the D8 icon-design record lives in the module doc and the `crossed_bar_shapes`/
`single_bar_shapes`/`stroked_square_shapes` method docs, which stay untouched. Net: removes two
doc lines that restate adjacent const names.

Location: const block after `use super::ImagesScrollerApp;`.

Change (exact; delete these two lines):

```rust
// BEFORE (line 13):
/// One zoom-glyph bar length/thickness (the + and - icon geometry).
// BEFORE (line 16):
/// The restore-fit marker: a stroked square outline.

// AFTER: the four const lines stand in order with no /// lines between them:
const ZOOM_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const ZOOM_GROUP_GAP: f32 = 16.0;
const GLYPH_BAR_LENGTH: f32 = 12.0;
const GLYPH_BAR_THICKNESS: f32 = 2.5;
const FIT_MARKER_SIDE: f32 = 11.0;
const FIT_MARKER_STROKE_WIDTH: f32 = 2.0;
```

Notes for the implementer: do NOT rename, reorder, or change any const value; delete only the two
`///` lines. Diff: 2 lines deleted.

Verification claim: `wc -l src/app/zoom_button.rs` == 188 after S1+S2; every `const` name and
value is unchanged (`git diff` shows comment-only hunks); harness `ALL CHECKS PASSED` with 76/76
(zt11 label pins and zp8 bound pins are untouched — no const used by tests changed).

## S3 — `src/app.rs`: rewrap the Task-2 `zoom_factor` field doc from 3 to 2 physical lines

Justification: line-budget pressure (the caller-designated simplification motive): `app.rs`
sits at 198/200. This is the only zero-risk headroom inside Task-2-owned lines: the doc text is
preserved verbatim (every D1/D6 fact kept — "1.0 = fit", reset-on-open, never-persisted), merely
rewrapped at the repo's ~96-column style. No other app.rs line is touched (pre-existing docs
belong to earlier tasks' review scope — Preserve-Existing-Code rule).

Location: `pub struct ImagesScrollerApp`, the last field's doc comment (lines 35–37).

Change (exact; replace these three lines with these two — same words):

```rust
// BEFORE (lines 35–37):
    /// Session zoom factor (Wave-4 Task 2): 1.0 = whole-image fit; read by the
    /// display path; reset on every successful open; never persisted.
    zoom_factor: f32,

// AFTER:
    /// Session zoom factor (Wave-4 Task 2): 1.0 = whole-image fit; read by the display
    /// path; reset on every successful open; never persisted.
    zoom_factor: f32,
```

Notes for the implementer: the field line `zoom_factor: f32,` stays exactly as-is; the two new
comment lines are 91 and 64 chars (under rustfmt's 100-column limit, and `cargo fmt` never
rewraps comments, so the formatting is stable). Diff: 1 line deleted.

Verification claim: `wc -l src/app.rs` == 197 after this step and `git diff` shows a comment-only
hunk in the struct block; harness `ALL CHECKS PASSED` with 76/76 (zt1 asserts the init value via
code, not the doc).

## Considered and REJECTED (do not re-open at 4.5a without a new reason)

1. Dedup the inline `input.key_pressed` reads in `src/app/zoom.rs` chord helpers with
   `keyboard_intake.rs::was_pressed`: `was_pressed` is module-private by plan S4 ("module-local
   helper"); sharing needs a visibility widening across the two sibling modules = a
   module-boundary/API change the decision space forbids. The plan's S2 chord code is exactly
   what landed. Kept as-is.
2. Collapse the "all reads first, then dispatches" structure of `read_and_dispatch_keystrokes`
   into per-chord `if was_pressed(...) { dispatch }`: behavior-identical under egui's per-frame
   input snapshot, but plan S4 note ("reads happen FIRST, dispatches AFTER … preserved verbatim")
   and conformance §12.7 ("reads collected BEFORE the layout dispatch") bind this shape —
   copied-behavior per the task brief's own litmus, and no strictly simpler equivalent exists
   without breaking that clause. Kept.
3. Replace the `Vec<egui::Shape>`-returning glyph builders with direct painter calls: removes
   three per-frame 1–2-element Vec allocations, but needs `(row_ui, center, color)` 3-arg
   signatures (≤2-args rule violation) or a new param type — plan S3 prescribes exactly the
   current builder shape. Medium redesign against an encoded design decision. Kept.
4. Inline `zoom::zoomed_draw_size` at the single app_display call site: it is pinned by pure
   test zp7 and by conformance §12.10 ("Display size per frame = … via `zoom::zoomed_draw_size`");
   removing it drops a test (19-test census is binding — hard block). Kept.
5. Unify the `trap_zoom_maximum`/`trap_zoom_minimum` twins into one parameterized trap: saves
   ~5 lines in `src/app/zoom.rs` (file not under cap pressure: 109 vs plan-expected 110, hard
   cap 200) but trades two self-named D2 encodings for a `(bool, &str)` arg pass — not strictly
   simpler; plan S2 names them explicitly. Kept.
6. Share the near-identical central-panel paint block between `show_navigation_layout` (app.rs)
   and `show_fullscreen_layout` (fullscreen.rs): a shared helper needs a mode flag for the
   empty-folder `else` arm (fullscreen shows blank, nav shows the message) — this restates task-1
   simplification REJECTED #2 and touches D9's untouched fullscreen dispatch. Kept separate.
7. Collapse the blank lines between the `mod` declarations in `app.rs` (161–179, would free
   ~9 lines): a stylistic pattern predating Task 2; plan S5.7 directed the new decls to follow
   "the existing order (after `mod fullscreen_button;`)" — churning pre-existing structure for
   formatting only violates Preserve-Existing-Code. Kept.
8. Delete `RESTORE_TO_FIT_FACTOR` and use `ZOOM_FACTOR_MINIMUM` directly: D2 semantics ("a step
   that lands on a bound silently clamps") and test zp8 + both app.rs reset sites name the
   restore target as its own concept; plan S1 defines the alias explicitly. Kept.
9. Hoist the double `app.zoomed_beyond_fit()` call in `zoom_button_specs` to one local: two
   side-effect-free float comparisons; hoisting costs +1 physical line in a cap-pressured file
   for zero measurable gain. Kept.
10. Test simplification: the 19-test census is binding (§4/S9); fixture reuse (zt8/zt9 use the
   existing `TempImageFolder`/`sample_decoded_image`) already follows the fullscreen_tests
    precedent. No test change.
11. Line-budget trims in the remaining files: `zoom.rs` 109/150-expected, `app/zoom.rs` 109/110,
   `keyboard_intake.rs` 55/60, `zoom_tests.rs` 109/130, `main.rs` 33, `app_display.rs` 118/125,
   `fullscreen.rs` 54, `floating_controls.rs` 179 (zoom addition = 2 code lines + doc rewrap,
   all used; `image_is_open` and `navigation_button_enabled` were grep-verified live — no dead
   seams found). No verdict: nothing to trim.

## Informational (no patch — caller/Task-3 planning input)

`src/app.rs` after S3 is 197/200 (3 lines of headroom). Wave-4 Task 3 (fit width/height) will
append at least `mod` declarations and likely a field to `app.rs` and will breach the 200 cap
unless the Task-3 plan budgets headroom (that task's own mod-decl/doc strategy, or a caller
decision). Flagged here only; fixing it now would require the REJECTED #7 style churn.

## Verification (owner: caller / implementer — NOT run by this step)

After S1+S2+S3, rerun the container harness (`alpine-vm_vm_status` first, then
`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`):
expect literal `ALL CHECKS PASSED` and 76/76 tests. All three steps are comment-only edits — any
failure means a typo in applying the patch, not a design issue. Expected line counts after
applying: `src/app.rs` 197, `src/app/zoom_button.rs` 188, all other files unchanged.
Commit form per the implementation plan §6 style: `fix: zoom review fixes (trim duplicated
docs, rewrap zoom factor field doc for line-cap headroom)`.
