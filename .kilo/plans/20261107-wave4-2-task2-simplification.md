# Wave 4.2 Task 2 — Pill Captions White + Thin Black Border: 4.3 Code Simplification Plan

Date: 2026-11-07 · Critical Workflow step 4.3 (code-simplifier) for Wave-4.2 Task 2
Branch: `fix/wave-4-2-operator-feedback` · HEAD reviewed: `c53fbf0` (scope `f2857c5..c53fbf0`; `4c3c959` is the feat commit, `c53fbf0` docs-only)
Implementation plan reviewed: `.kilo/plans/20261107-wave4-2-task2-pill-styling-implementation-plan.md` (§0 locked decisions + §17 reviewer notes respected — only §19.2-authorized non-behavioral simplifications below)
Scope reviewed (full current content read): `src/app/pill_style.rs` (146), `src/app/pill_style_tests.rs` (48), `src/app.rs` (183), `src/app/button_hint.rs` (200), `src/app/floating_controls.rs` (184), `src/app/fullscreen_button.rs` (48), `src/app/zoom_button.rs` (186), `src/app/fit_button.rs` (176)

NOTE (naming): the implementation plan §19.2 anticipated the name
`20261107-wave4-2-task2-pill-styling-simplification.md`; the caller's step instruction pinned
`20261107-wave4-2-task2-simplification.md` — this file uses the CALLER's pinned path. Flagged to the
caller for the 4.5b adherence report.

## Verdict summary

The styling change is faithful to the plan: ONE shared `pill_style` palette + shape builders used by
all five paint/caption consumers, the four removed per-module color helpers left ZERO residue
(grep-verified: `arrow_color|zoom_glyph_color|fit_glyph_color|fullscreen_square_color` → 0 hits in
`src/`; every `pill_style` item and every `*_BUTTON_LABEL` const has a live consumer; no unused
imports), no commented-out code, no TODO/FIXME, all files ≤200, all bodies ≤50, depth ≤2, all
signatures ≤2 params. TWO minor behavior-preserving simplifications apply: **S1** removes the last
genuine outline-construction duplication (the near-twin fit axis-line helpers + the twin triangle
struct-literals in `fit_button.rs`); **S2** resolves the one file at the hard cap
(`button_hint.rs` exactly 200/200 vs the §13 estimate ≈189 — zero headroom before 4.4's doc polish)
by moving its inline tests to the sibling test-file pattern that already dominates `src/app`
(9 registered `*_tests` siblings; `button_hint.rs:168` is the ONLY inline `mod tests` in the
module family). No behavior, no painted geometry, no string, no test-count change anywhere
(census stays 107). All other candidates were REJECTED with reasons (§Considered and REJECTED);
doc-bullet WORDING belongs to step 4.4's docs authority and is deliberately untouched here.
Everything fits in ONE small follow-up commit by the junior implementer — no architectural move,
no architector escalation needed.

## S1 — `src/app/fit_button.rs`: dedupe the glyph-shape construction

Justification: `horizontal_axis_line_shape` (162–168) and `vertical_axis_line_shape` (170–176) are
near-twins differing ONLY in the two components' order inside one `egui::Vec2` — each is a
2-call-function whose body is `from_center_size` + `pill_style::outlined_rect_shape`. The remaining
duplication inside `glyph_shapes` is the twin 8-line `TriangularGlyphSpec` struct-literals differing
only by `∓axis_offset` and facing. Folding both into the ONE function that already computes
`horizontal`/`axis_offset`/facing removes 2 whole functions + 16 duplicated lines, keeps every shape
in the EXACT paint order `[axis, minus, plus]` with byte-identical geometry, and all signatures stay
≤2 params (no new function is introduced; the fold is a closure, which is not a method per the
rule). The §10.5/§10.6 shapes were step-4.2 encoding; §19.2 authorizes 4.3 non-behavioral
simplifications and §17's "do not re-open the shape function split" is scoped to `zoom_button.rs` —
no fit module const, the `glyph_shapes` signature, or any other function is touched.

Change 1 — replace the body of `FitButtonAction::glyph_shapes` (anchor: the exact current line
`    fn glyph_shapes(` through its closing `    }`) with:

```rust
    fn glyph_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let horizontal = matches!(self, Self::FitWidth);
        let axis_offset = if horizontal {
            egui::vec2(FIT_ARROW_SPACING, 0.0)
        } else {
            egui::vec2(0.0, FIT_ARROW_SPACING)
        };
        let line_size = if horizontal {
            egui::vec2(FIT_AXIS_LINE_LENGTH, FIT_AXIS_LINE_THICKNESS)
        } else {
            egui::vec2(FIT_AXIS_LINE_THICKNESS, FIT_AXIS_LINE_LENGTH)
        };
        let (minus_facing, plus_facing) = if horizontal {
            (ArrowDirection::Left, ArrowDirection::Right)
        } else {
            (ArrowDirection::Up, ArrowDirection::Down)
        };
        let line_rect = egui::Rect::from_center_size(glyph_center, line_size);
        let outlined_triangle_at = |triangle_center: egui::Pos2, facing: ArrowDirection| {
            pill_style::outlined_triangle_shape(pill_style::TriangularGlyphSpec {
                rect: egui::Rect::from_center_size(triangle_center, FIT_ARROW_TRIANGLE_SIZE),
                rotation: facing.paint_rotation(),
                fill_color: glyph_color,
            })
        };
        vec![
            pill_style::outlined_rect_shape(line_rect, glyph_color),
            outlined_triangle_at(glyph_center - axis_offset, minus_facing),
            outlined_triangle_at(glyph_center + axis_offset, plus_facing),
        ]
    }
```

The doc comment above `glyph_shapes` (the two `///` lines) stays UNTOUCHED.

Change 2 — DELETE both now-unused free functions and the blank line that precedes
`fn horizontal_axis_line_shape`: from the blank line after the `impl FitButtonAction` closing
`}` to EOF (`grep -c "axis_line_shape" src/app/fit_button.rs` must be 0 afterwards — the call sites
were replaced by Change 1; the two functions are the ONLY other references, grep-verified).

Notes for the implementer: apply Change 1 BEFORE Change 2. No const, no other function, no doc line
outside `glyph_shapes` is touched. Longest new line ≤ 90 cols (rustfmt-clean by hand, 100-col
budget). Max nesting: closure body = depth 1, struct literal = depth 2 — within the rule.

Neutrality verification: `horizontal` drives all three `if/else` selections with the SAME
`(LENGTH, THICKNESS)` / `(THICKNESS, LENGTH)` pairing the two helpers hard-coded; triangle rects,
offsets, rotations, fill color, and the `vec!` order are byte-equivalent to the removed literals →
identical `egui::Shape` values. `wc -l src/app/fit_button.rs` ≈ 157 (from 176; ±2 fmt drift =
application typo, not design). `git diff --stat` after S1: exactly one file.

## S2 — `button_hint.rs` inline tests → sibling `button_hint_tests.rs` (regains the cap headroom)

Justification: `button_hint.rs` landed at EXACTLY 200 lines (the hard `max-lines-per-file` cap;
§13 estimated ≈189). Step 4.4 may add header wording and any later caption/tooltip edit would break
the cap. The `src/app` house pattern for test modules is the SIBLING file (9 registered siblings:
`fit_one_shot_tests`, `floating_controls_tests`, …, `pill_style_tests`; `button_hint.rs:168` is the
lone inline `mod tests` in the family). The two test BODIES move verbatim (assertions and string
counts unchanged → census stays 107). Sibling access requires the two pure data functions widened
exactly to `pub(super)` — the module-family scope that is the house convention for test-visible data
(`floating_controls::navigation_button_enabled` precedent) and still satisfies
`prefer-private-members` against the crate. This supersedes, by the caller's explicit 4.3 request,
the Wave-4.1 note "do not widen hotkey_reference/tooltip_text" (that note guarded the 4.5a lane of
its own cycle; the widening is `pub(super)`-bounded, not `pub`).

Change 1 — in `src/app/button_hint.rs`, widen the two data functions (anchors are the exact lines):
```rust
pub(super) fn hotkey_reference(button: PillButton) -> Option<&'static str> {
pub(super) fn tooltip_text(button: PillButton) -> &'static str {
```
(everything else in their bodies/attrs/docs byte-identical).

Change 2 — in `src/app/button_hint.rs`, DELETE the trailing test module: from the blank line
directly above `#[cfg(test)]` (the one after the `show_button_column` closing `}`) through EOF
(the `mod tests { ... }` block). The file ends with the `}` of `show_button_column`.

Change 3 — CREATE `src/app/button_hint_tests.rs` with EXACTLY this content (the two `#[test]`
bodies are the deleted ones copied VERBATIM — every assertion line byte-identical):

```rust
//! Test module of `button_hint`: the pinned hotkey-reference and tooltip tables.

use super::button_hint::{PillButton, hotkey_reference, tooltip_text};

#[test]
fn hotkey_reference_matches_the_spec() {
    use PillButton::*;
    assert_eq!(hotkey_reference(ImagePrevious), None);
    assert_eq!(hotkey_reference(ImageNext), None);
    assert_eq!(hotkey_reference(FolderUp), None);
    assert_eq!(hotkey_reference(FolderDown), None);
    assert_eq!(hotkey_reference(Fullscreen), Some("F11"));
    assert_eq!(hotkey_reference(ZoomIn), Some("I"));
    assert_eq!(hotkey_reference(ZoomOut), Some("O"));
    assert_eq!(hotkey_reference(RestoreFit), Some("0"));
    assert_eq!(hotkey_reference(FitWidth), Some("W"));
    assert_eq!(hotkey_reference(FitHeight), Some("H"));
}

#[test]
fn tooltip_text_matches_the_spec() {
    use PillButton::*;
    assert_eq!(tooltip_text(ImagePrevious), "Previous image (Left Arrow)");
    assert_eq!(tooltip_text(ImageNext), "Next image (Right Arrow)");
    assert_eq!(tooltip_text(FolderUp), "Previous sibling folder (Up Arrow)");
    assert_eq!(tooltip_text(FolderDown), "Next sibling folder (Down Arrow)");
    assert_eq!(tooltip_text(Fullscreen), "Fullscreen (F11)");
    assert_eq!(tooltip_text(ZoomIn), "Zoom in (I)");
    assert_eq!(tooltip_text(ZoomOut), "Zoom out (O)");
    assert_eq!(tooltip_text(RestoreFit), "Restore fit (0)");
    assert_eq!(tooltip_text(FitWidth), "Fit width (W)");
    assert_eq!(tooltip_text(FitHeight), "Fit height (H)");
}
```

(20 pinned strings + 4 `None` pins = the 24 assertions unchanged; the only deltas vs the inline
version are the file-level `use super::button_hint::{...}` replacing the inline
`use super::{...}` — brace order is ASCII per the `pill_style_tests.rs:6-9` precedent.)

Change 4 — register the sibling in `src/app.rs`: between the blank line after `mod zoom_button;`
and the first `#[cfg(test)]` of the test block, insert alphabetically BEFORE `fit_one_shot_tests`:
```rust
#[cfg(test)]
mod button_hint_tests;
```
The test block then opens: `#[cfg(test)]` / `mod button_hint_tests;` / `#[cfg(test)]` /
`mod fit_one_shot_tests;` (alphabetical order maintained — b < f; no blank lines introduced, the
block currently has none between pairs).

Notes for the implementer: apply Changes 1→2→3→4 in order; do NOT touch any other line of
`button_hint.rs` or `app.rs`.

Neutrality verification: `wc -l src/app/button_hint.rs` == 165 (200 − 35); `src/app.rs` 183 → 185;
new file ≈ 36 lines; both string tables and all 24 assertions byte-identical (a `git diff` of the
moved bodies shows only the `mod tests` wrapper and `use` line changes); no `#[test]` added or
removed → census unchanged at 107. `grep -c "#\[cfg(test)\]" src/app/button_hint.rs` == 0.

## Considered and REJECTED (do not re-open at 4.5a without a new reason)

1. Centralize `zoom_button::stroked_square_shapes`' black-ring `rect_stroke` into a new
   `pill_style` helper (it duplicates the ring half of `outlined_rect_shape`): REJECTED — the
   plan §0 decision 4 + §9.7 pin the exact TWO-`rect_stroke` composition (outline-on-expanded-rect,
   marker-inside), and the §3 design note deliberately makes `zoom_button` the ONE consumer
   permitted to read `OUTLINE_COLOR`/`OUTLINE_STROKE_WIDTH` directly ("because it must build a
   stroke around a NOT-filled rect"). Re-opening a deliberately-locked composition is out of the
   4.3 non-behavioral lane; the duplication is 1 five-line hunk.
2. Extract the caption `if let Some(reference)` block out of `show_button_column`: REJECTED —
   same disposition re-affirmed since Wave-4.1 Task 2 REJECTED #1 (extraction ≈6-line cost for 4,
   depth already ≤2: fn=0, closure=1, `if let`=2; plan §17 keeps the disposition with the
   simplifier and the simplifier declines again).
3. Replace `align_size_within_rect(galley.size(), caption_rect).min` with plain `caption_rect.min`:
   REJECTED — §0.1 row 2 pins the verified-0.37 anchored form; it documents the CENTER_TOP intent
   and survives any future size change; the simplification saves 2 lines and weakens the §20
   crosswalk row "no un-outlined caption" traceability.
4. Further `pill_style` centralization of the 2-line `glyph_state_for` + `glyph_state_color` pair
   (5 call sites): REJECTED — §0.1 row 4 deliberately SPLIT the 3-param sketch into the two 2-param
   fns for the parameter rule; a unified helper needs a borrow-carrying param object (new lifetime
   struct, net-worse complexity) and the pair is pinned by §14 test 3.
5. Any doc-line trims/rewraps in `pill_style.rs`/`zoom_button.rs`/`fit_button.rs`/`floating_controls.rs`
   (e.g. the provenance clauses "Replaces the four per-module theme-palette helpers", "moved from
   `button_hint`"): REJECTED for THIS step — §15/§21 assign module-doc WORDING to 4.4's
   docs-specialist authority; the simplifier must not pre-empt or collide with that step. No
   commented-out code exists anywhere in scope (grep-verified).
6. Split the `Hovered | Idle` arm pattern, fold `with_enabled` into `for_button`, touch the derives
   or the `PillButtonPlacement` field privacy: REJECTED — three explicit §17 locks.
7. `floating_controls.rs` / `fullscreen_button.rs` / `pill_style.rs`: NOTHING admissible — files
   sit 184/48/146 of 200; their outline paths already route through the shared builders (that IS
   the centralization this task landed); per-site locals are self-documenting; offsets ring is a
   pinned 8-entry const (§14 test 2) — a generated ring would be worse.

## Informational (no patch — caller awareness)

1. Residue audit EVIDENCE (task item 2): removed-function grep → 0 hits; `PILL_GLYPH_COLOR`/
   `OUTLINE_*`/`CAPTION_*`/`PillGlyphState`/both param structs/both builders/both palette fns all
   have live consumers; `FIT_MARKER_STROKE_WIDTH`, `CAPTION_TEXT_GAP`, all `*_GROUP_GAP`,
   `*_BUTTON_SIZE`, `ARROW_TRIANGLE_SIZE`, `FIT_ARROW_*`, `FIT_AXIS_*` all used; the five
   `*_BUTTON_LABEL` consts are used AND pinned by `fit_one_shot_tests`/`zoom_tests` —
   `pub(super)` is load-bearing, not residue.
2. §13 budget drift: `button_hint.rs` expected ≈189, landed 200 (resolved by S2); all other files
   landed within ±4 of the estimate. Post-plan budgets: `fit_button.rs` ≈157, `button_hint.rs` 165,
   `button_hint_tests.rs` ≈36, `app.rs` 185, others unchanged — every touched file gains ≥13 lines
   of headroom ahead of 4.4/Task 3–4.
3. S2 supersedes plan §6.5's "the two inline tests stay UNTOUCHED" (a 4.2-execution directive) —
   the moved assertion bodies stay byte-identical per §14's census intent; 4.5b should record the
   move as an authorized 4.3 simplification (§19.2), same class as the Wave-4.1 Task-3 `db83dd4`
   reviewer/simplifier follow-ups.

## Per-file verdicts

- `src/app/pill_style.rs` (146): NOTHING (the centralization itself; doc wording → 4.4).
- `src/app/pill_style_tests.rs` (48): NOTHING.
- `src/app.rs` (183): S2 Change 4 only (+2 registrations → 185).
- `src/app/button_hint.rs` (200): S2 (→165; two `pub(super)` widenings, test module moved out).
- `src/app/button_hint_tests.rs` (NEW ≈36): S2 Change 3 (verbatim bodies).
- `src/app/floating_controls.rs` (184): NOTHING.
- `src/app/fullscreen_button.rs` (48): NOTHING.
- `src/app/zoom_button.rs` (186): NOTHING (composition plan-locked; REJECTED #1).
- `src/app/fit_button.rs` (176): S1 (→≈157).

## Verification (owner: caller / implementer — NOT run by this step, per plan §0.7)

Apply S1 then S2 (each change anchors on exact text, not line numbers). Rerun the container harness
at A-Task 5 (`alpine-vm_vm_status` first, fmt round FIRST, then `scripts/dev-checks.sh`): expect
literal `ALL CHECKS PASSED` and `107 passed; 0 failed` (census UNCHANGED by both steps). Expected
`git diff --stat`: 3 modified files + 1 new file; zero changes in `pill_style*`,
`floating_controls*`, `fullscreen_button`, `zoom_button*`, every string table byte-identical,
every painted-shape geometry byte-identical. Suggested single commit (implementer executes; this
step does NOT commit):
`fix: wave4.2 task2 simplification (dedupe fit glyph builders, sibling button-hint tests)`.
