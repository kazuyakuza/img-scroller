# Wave 4.2 Task 2 — Pill Captions White + Thin Black Border on Icons and Captions: Implementation Plan

**Date:** 2026-11-07
**Workflow step:** 4.1b (architector) — referencing the 4.1a spec
**Source TODO:** [`.agent/todos/20261107/20261107-todo-1.md`](../../.agent/todos/20261107/20261107-todo-1.md) §Task 2
**Front-end spec (BINDING input, read in full):** [`.kilo/plans/20261107-wave4-2-task2-pill-styling-frontend-spec.md`](20261107-wave4-2-task2-pill-styling-frontend-spec.md)
**Global plan:** [`.kilo/plans/20261107-wave42-then-wave5-global-plan.md`](20261107-wave42-then-wave5-global-plan.md) → Part A, A-Task 2, steps 4.2 → 4.6
**Branch:** `fix/wave-4-2-operator-feedback`
**Base:** `main` @ `88c45f7` (0.4.1); branch steps A.2/A.3 (branch + version 0.4.2) and A-Task 1 (acceptance-record marks) must already be landed on this branch.

---

## 0. Decisions locked by this plan

All sourced from the approved 4.1a spec §2–§5 plus project rules (`max-arguments-per-method.md`, `prefer-private-members.md`, `max-lines-per-file.md`). Where the spec's sketches conflicted with the PINNED egui 0.37 API or with project rules, the decision below is the 0.37-verified equivalent of the spec's unambiguous intent — see §0.1 (spec reconciliation) for each deviation and its docs.rs evidence.

1. **Shared module (NEW):** `src/app/pill_style.rs` — single source for: the pill glyph/caption palette (`PILL_GLYPH_COLOR` = `Color32::WHITE` for every ENABLED state: idle AND hovered; `Color32::BLACK`-free disabled palette = theme `noninteractive.fg_stroke.color`), the outline constants (`OUTLINE_COLOR = Color32::BLACK`, `OUTLINE_STROKE_WIDTH = 1.0`), the caption constants (`CAPTION_TEXT_SIZE = 9.0`, the 8 outline offsets), the outlined shape builders, and the outlined hotkey-caption paint. All items `pub(super)` (module-family scope; house convention) — the sibling test file must see the pinned constants.
2. **Test module (NEW):** `src/app/pill_style_tests.rs`, registered in `src/app.rs` under `#[cfg(test)]` — 3 pinned tests (§14). SPLIT chosen instead of the spec's inline-test preference because the estimated inline total (~190 lines with fmt-wrap risk) endangers the 200-line cap; the split also matches the dominant `app/` test-file house pattern. `CAPTION_TEXT_SIZE` MOVES from `button_hint.rs` to `pill_style.rs` (single source; `button_hint` no longer needs it after the caption paint moves; value byte-identical 9.0).
3. **Enablement plumbing:** `PillButtonPlacement` gains one private field `enabled: bool` set by a new 2-arg builder `with_enabled(self, enabled: bool) -> Self`; every construction site passes its enablement (`fullscreen_button`: literal `true`; arrows/zoom/fit: `self.enabled`). This is the ONLY way `show_button_column` can paint dimmed DISABLED captions (the modules' disabled state is a manual flag, not an egui-disabled state — same reason as Wave-4.1 Task 2's tooltip decision).
4. **Per-shape outline composition (exact forms in §3 and §7–§10):**
   - Triangles (4 pill arrows + 4 fit triangles): `pill_style::outlined_triangle_shape(TriangularGlyphSpec { rect, rotation, fill_color })` — EXACT `Shape::rotated_triangle` point geometry (same 3 corners, same rotation) rendered via `Shape::convex_polygon(points, fill, egui::PathStroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR).outside())` — outline painted entirely OUTSIDE the fill.
   - Rect glyphs (zoom `[+]` two bars, `[−]` bar, fullscreen square, both fit axis lines): `pill_style::outlined_rect_shape(rect, fill_color)` = `Shape::Vec` of `Shape::rect_filled(rect, CornerRadius::ZERO, fill)` + `Shape::rect_stroke(rect, CornerRadius::ZERO, Stroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR), StrokeKind::Outside)` — 0.37 has NO 5-arg `Shape::rect` constructor (§0.1); the composition preserves the fill interior exactly and paints the black border outside.
   - Restore-fit stroked square (already stroked, not filled): TWO `Shape::rect_stroke` shapes in order [outline, marker] — outline: `marker_rect.expand(OUTLINE_STROKE_WIDTH)` with the BLACK stroke, `StrokeKind::Outside`; marker: the UNCHANGED glyph-colored `FIT_MARKER_STROKE_WIDTH` stroke on `marker_rect`, `StrokeKind::Inside`.
5. **Caption font border (offset-copy):** `pill_style::paint_outlined_caption(column_ui, OutlinedCaption { text, fill_color })` — ONE `layout_no_wrap` per caption into a PLACEHOLDER-colored galley, then `column_ui.allocate_exact_size(galley.size(), Sense::hover())` (space identical to the old 9 pt Label), anchor `CENTER_TOP` via `align_size_within_rect(...).min`, then 9 galley renders: 8 BLACK copies at `CAPTION_OUTLINE_OFFSETS` + 1 `fill_color` copy at the base (9 paints = 8 outline + 1 fill; spec §4.2). Recolor works ONLY via a placeholder-colored galley + the `Shape::galley` fallback (§0.1 third entry).
6. **Call sites not touched shape-wise:** allocation, widget-info, hover cursor, click guards, gap constants, group order, tooltip strings/behavior, `CAPTION_TEXT_SIZE`, caption strings, layout geometry — all byte-identical. Only the color source and the outline stroke change.
7. **Verification ownership (binding caller note):** NO container/harness invocation happens in THIS task (no `cargo fmt` round, no dev-checks, no rebuild, no audit). ALL harness execution belongs to A-Task 5 per the wave plan (`cargo fmt` round first, then `scripts/dev-checks.sh`; `vm_status` first; exit code authoritative; per `docs/build.md`). This plan RECORDS the expected A-Task-5 results (§12) so A-Task 5 can pin them; the implementer writes rustfmt-clean code by hand following the neighbor files' style.

### 0.1 Spec reconciliation (verified deviations from the 4.1a sketches — NOT silent assumptions)

All four below were verified 2026-11-07 against docs.rs of the PINNED `egui =0.37.0` / `epaint =0.37.0` (egui 0.37.0 `Shape` page; epaint 0.37.0 `PathStroke`/`Align2` re-export pages). The spec's INTENT is unambiguous in each case; the plan encodes the 0.37-realization:

| # | Spec sketch | Verified 0.37 fact | Plan's form |
|---|---|---|---|
| 1 | `egui::Shape::rect(rect, corner_radius, fill, stroke, StrokeKind::Outside)` (spec §3.2/§3.4) | NO `Shape::rect` constructor exists in 0.37 (only `rect_filled`/`rect_stroke` + `Painter::rect`) | `Shape::Vec([rect_filled(fill), rect_stroke(BLACK, Outside)])` nested in one builder (§0 decision 4 row 2) |
| 2 | `egui::Align2::CENTER_TOP.pos_in_rect(&rect, size)` (2-arg, spec §4.3) | 0.37 `Align2::pos_in_rect(self, frame: &Rect)` is 1-arg; the 2-arg call does not compile | `egui::Align2::CENTER_TOP.align_size_within_rect(galley.size(), caption_rect).min` (0.37-verified `align_size_within_rect(self, size, frame) -> Rect`) |
| 3 | Spec §4.3 outlines by painting the SAME (fill-colored) galley with `painter().galley(pos, galley, OUTLINE_COLOR)` | A galley colored with a NON-placeholder color IGNORES the fallback (epaint: "any non-placeholder color in the galley takes precedence") — the sketch would render 9 identical copies, i.e. NO outline | Layout ONCE with `Color32::PLACEHOLDER`; each of the 9 passes uses `Shape::galley(pos, galley, desired_color)` so the fallback substitutes the desired color |
| 4 | `glyph_color_for(enabled, hovered, ui)` (3 params), `paint_outlined_caption(ui, text, fill_color)` (3 params), `outlined_triangle_shape(rect, rotation, fill)` (3 params) | Project rule `max-arguments-per-method.md`: ≤2 params | `PillGlyphState` split: `glyph_state_for(enabled, hovered)` + `glyph_state_color(glyph_state, row_ui)`; param objects `outlined_triangle_shape(TriangularGlyphSpec)` and `paint_outlined_caption(column_ui, OutlinedCaption)` with `pub(super)` struct-literal fields (struct literals are not function calls) |

### 0.2 Verified egui 0.37.0 API facts (all required by the steps below)

- `egui::PathStroke` (re-export of `epaint::PathStroke { width, color, kind }`) — `PathStroke::new(width: f32, color: impl Into<Color32>) -> Self`, `.outside() -> Self` ("painted entirely outside of the shape"), `.inside()`, `PathStroke::NONE`; `Shape::convex_polygon(points: Vec<Pos2>, fill: impl Into<Color32>, stroke: impl Into<PathStroke>) -> Shape` ✓.
- `Shape::rect_filled(rect, corner_radius: impl Into<CornerRadius>, fill: impl Into<Color32>)` ✓; `Shape::rect_stroke(rect, corner_radius, stroke: impl Into<Stroke>, stroke_kind: StrokeKind)` ✓; `Shape::rotated_triangle(rect, rotation: f32, fill: impl Into<Color32>)` — 0.0 points down, `TAU/4` left, `TAU/2` up, `-TAU/4` right (matches `ArrowDirection::paint_rotation`, unchanged) ✓; `Shape::Vec(Vec<Shape>)` variant ✓; `Shape::galley(pos: Pos2, galley: Arc<Galley>, fallback_color: Color32) -> Shape` — placeholder parts recolored by the fallback ✓.
- `Rect::expand(delta: f32) -> Rect` ✓; `StrokeKind::{Outside, Inside}` ✓; `Color32::PLACEHOLDER` ✓; `Color32::{WHITE, BLACK}` ✓.
- `Ui::allocate_exact_size(size, sense) -> (Rect, Response)` (house-proven) ✓; `Ui::painter() -> &Painter` ✓; `Painter::layout_no_wrap(text: String, font_id: FontId, color: Color32) -> Arc<Galley>` ✓; `Painter::add(impl Into<Shape>)` (house-proven) ✓; `Galley::size() -> Vec2` ✓; `FontId::proportional(size: f32) -> FontId` ✓.
- `Align2::CENTER_TOP` + `Align2::align_size_within_rect(self, size: Vec2, frame: Rect) -> Rect` ✓ (§0.1 entry 2).
- `egui::Context::default()` + `Context::run(input, |&Context|)` + `Area::new(Id).show(&Context, |&mut Ui|)` for the palette test harness (house-proven pattern) ✓.

---

## 1. Pre-flight checks (STRICT; STOP on any mismatch)

Run from the repository root; read-only (NO harness, NO git mutations):

1. `git branch --show-current` → must print `fix/wave-4-2-operator-feedback`. If not → STOP and return the question to the caller (branch creation is workflow Step A.2's job).
2. `git log --oneline -6` → must contain, in this branch's short history: a `chore: bump version to 0.4.2` commit, an A-Task 1 acceptance-recording mark commit, and the A-Task 2 4.1a/spec-related plan commits if already made. If the version bump or A-Task-1 marks are missing → STOP (earlier steps did not run).
3. `Cargo.toml` `version` reads `0.4.2` (read-only check).
4. `git status` → expected: untracked `.kilo/plans/20261107-wave4-2-task2-pill-styling-frontend-spec.md` and untracked `.kilo/plans/20261107-wave4-2-task2-pill-styling-implementation-plan.md` (this file), plus AT MOST one modified file: `.agent/project-info/context.md` whose diff contains ONLY the Wave-4.2 Task-2 4.1a context-log entries (documenting the 4.1a spec event). If ANY other modified/untracked file appears → STOP and return the question to the caller.
5. Read `.gitignore`: `target/`, `dist/`, `logs/`, `*.log` must never be staged; the files staged below match no ignore pattern.
6. Read each file once before editing (line numbers below are HEAD-relative on the branch, matching `main` @ `88c45f7`): `src/app.rs` (180 lines), `src/app/floating_controls.rs` (191), `src/app/button_hint.rs` (184), `src/app/zoom_button.rs` (196), `src/app/fit_button.rs` (182), `src/app/fullscreen_button.rs` (55), `src/app/arrow_direction.rs` (66 — UNTOUCHED).

## 2. Git actions for this step

- Branch creation / switching / `git push` are NOT part of this step (branch already exists; push is restricted to workflow Step A.5, `main` → `origin` ONLY). Work commits happen ON `fix/wave-4-2-operator-feedback`.
- **Commit 1 (docs)** — message exactly `docs: add wave4.2 task2 spec and implementation plan`:
  - `git add .kilo/plans/20261107-wave4-2-task2-pill-styling-frontend-spec.md .kilo/plans/20261107-wave4-2-task2-pill-styling-implementation-plan.md`
  - `git add .agent/project-info/context.md` — FIRST inspect its diff (`git diff .agent/project-info/context.md`): it must contain only the Task-2 4.1a context entries. If other content is in the diff → STOP, ask the caller.
  - Do NOT modify existing entries in `context.md` (overwrite-prevention rule); append/prepend only per its list structure.
  - Run the pre-commit checklist (§16), then commit.
- **Commit 2 (feat)** — message exactly `feat: white pill captions and outlined glyph shapes`, staged AFTER ALL code steps (§3–§10) are complete: `git add src/app.rs src/app/pill_style.rs src/app/pill_style_tests.rs src/app/button_hint.rs src/app/floating_controls.rs src/app/fullscreen_button.rs src/app/zoom_button.rs src/app/fit_button.rs` (the eight touched/created files ONLY), then commit. NOTE: with the harness deferred to A-Task 5 (§0 decision 7), commit 2 lands before the automated checks; A-Task 5's harness verifies the committed code and any remediation is a follow-up review-fix commit.
- **Commit 3 (docs, closing)** — message exactly `docs: record wave4.2 task2 pill styling state`: `.agent/project-info/context.md` (closing-step entry per §15) + `.agent/project-structure.md` (per §15).
- NO `CHANGELOG.md` edits; NO edits to any existing `.kilo/plans/*` file (new files only); NO TODO-file edits (the `[DONE]` mark belongs to 4.6). Later cycles: 4.3 review-fix commit(s) (§19), 4.4 module-doc polish if any (§20), 4.6 mark commit `task: mark wave4.2 task2 pill styling done`.

## 3. Step 1 — create `src/app/pill_style.rs` (NEW file, ≈135 lines, cap 200)

Create `src/app/pill_style.rs` with EXACTLY the following content (constants, signatures, compositions and doc lines are binding; only rustfmt-level reflow is implementer latitude):

```rust
//! Shared pill-styling primitives (Wave 4.2 Task 2): the enabled glyph and
//! caption color (WHITE) with the theme-dimmed disabled palette, the 1-point
//! black outline constants every painted pill shape outlines itself with,
//! the outlined shape builders for the painted glyphs, and the outlined
//! hotkey-caption paint (the offset-copy outline technique).

use eframe::egui;

/// The pill's icon and caption color for every ENABLED state (idle AND
/// hovered — WHITE cannot brighten further; the hover cue stays the pointing
/// cursor and the tooltip). Matches the painted icons' guardrail color.
pub(super) const PILL_GLYPH_COLOR: egui::Color32 = egui::Color32::WHITE;
/// The pill's outline color: BLACK, so white glyphs and captions keep a
/// legible edge over any background.
pub(super) const OUTLINE_COLOR: egui::Color32 = egui::Color32::BLACK;
/// The pill's thin outline stroke width in points (the operator's
/// "thin border" constraint: every painted glyph and caption).
pub(super) const OUTLINE_STROKE_WIDTH: f32 = 1.0;
/// The hotkey caption text size in points (moved from `button_hint`; the
/// value is unchanged from Wave 4.1 Task 2).
pub(super) const CAPTION_TEXT_SIZE: f32 = 9.0;
/// The 8 neighbor offsets of the caption outline copies (a 1-point ring
/// painted as 8 BLACK text copies around the one fill-colored caption).
pub(super) const CAPTION_OUTLINE_OFFSETS: [egui::Vec2; 8] = [
    egui::vec2(-1.0, -1.0),
    egui::vec2(0.0, -1.0),
    egui::vec2(1.0, -1.0),
    egui::vec2(-1.0, 0.0),
    egui::vec2(1.0, 0.0),
    egui::vec2(-1.0, 1.0),
    egui::vec2(0.0, 1.0),
    egui::vec2(1.0, 1.0),
];

/// The painted visual state of one pill glyph/caption (which the shared
/// palette resolves): DISABLED dimming, hovered, or enabled idle.
pub(super) enum PillGlyphState {
    Disabled,
    Hovered,
    Idle,
}

/// Maps the painted enablement and hover state to the pill's glyph state.
pub(super) fn glyph_state_for(enabled: bool, hovered: bool) -> PillGlyphState {
    if !enabled {
        return PillGlyphState::Disabled;
    }
    if hovered {
        return PillGlyphState::Hovered;
    }
    PillGlyphState::Idle
}

/// The pill's shared palette: DISABLED dims to the theme's noninteractive
/// stroke color; BOTH enabled states (idle and hovered) pin
/// `PILL_GLYPH_COLOR`. Replaces the four per-module theme-palette helpers.
pub(super) fn glyph_state_color(
    glyph_state: PillGlyphState,
    row_ui: &egui::Ui,
) -> egui::Color32 {
    match glyph_state {
        PillGlyphState::Disabled => row_ui.visuals().widgets.noninteractive.fg_stroke.color,
        PillGlyphState::Hovered | PillGlyphState::Idle => PILL_GLYPH_COLOR,
    }
}

/// The painted-arrow-family triangle glyph (struct-literal constructed at
/// the paint sites; the param object keeps the builder at one argument).
pub(super) struct TriangularGlyphSpec {
    pub(super) rect: egui::Rect,
    pub(super) rotation: f32,
    pub(super) fill_color: egui::Color32,
}

/// The arrow-family triangle with its black outline painted ENTIRELY OUTSIDE
/// the filled triangle geometry: the EXACT point set of
/// `Shape::rotated_triangle` (same three corners, same center rotation),
/// rendered through `convex_polygon` with an outside `PathStroke`.
pub(super) fn outlined_triangle_shape(glyph: TriangularGlyphSpec) -> egui::Shape {
    let rotation = egui::Rot2::from_angle(glyph.rotation);
    let corner_points = [
        glyph.rect.left_top(),
        glyph.rect.right_top(),
        glyph.rect.center_bottom(),
    ];
    let rotated_points: Vec<egui::Pos2> = corner_points
        .into_iter()
        .map(|point| glyph.rect.center() + rotation * (point - glyph.rect.center()))
        .collect();
    egui::Shape::convex_polygon(
        rotated_points,
        glyph.fill_color,
        egui::PathStroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR).outside(),
    )
}

/// A filled rectangle with its black outline painted OUTSIDE the fill (the
/// fill interior keeps its exact size). egui 0.37 has no fill+stroke
/// `Shape::rect` constructor, so the builder composes fill + outside-stroke
/// outline of the SAME rect nested as one `Shape::Vec`.
pub(super) fn outlined_rect_shape(rect: egui::Rect, fill_color: egui::Color32) -> egui::Shape {
    egui::Shape::Vec(vec![
        egui::Shape::rect_filled(rect, egui::CornerRadius::ZERO, fill_color),
        egui::Shape::rect_stroke(
            rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR),
            egui::StrokeKind::Outside,
        ),
    ])
}

/// The per-caption paint call of `paint_outlined_caption`: its text and the
/// fill color the fill copy renders in (struct-literal constructed).
pub(super) struct OutlinedCaption {
    pub(super) text: &'static str,
    pub(super) fill_color: egui::Color32,
}

/// Paints one hotkey caption with the offset-copy outline (egui 0.37 has no
/// glyph-outline API): lay out ONCE as a PLACEHOLDER-colored galley,
/// allocate the caption's exact size like the previous 9 pt label, anchor it
/// CENTER_TOP, add 8 BLACK copies at the 1-point neighbor offsets, then add
/// ONE fill-colored copy at the base position (9 galley paints per caption).
pub(super) fn paint_outlined_caption(column_ui: &mut egui::Ui, caption: OutlinedCaption) {
    let font_id = egui::FontId::proportional(CAPTION_TEXT_SIZE);
    let galley = column_ui.painter().layout_no_wrap(
        caption.text.to_string(),
        font_id,
        egui::Color32::PLACEHOLDER,
    );
    let (caption_rect, _) = column_ui.allocate_exact_size(galley.size(), egui::Sense::hover());
    let caption_position = egui::Align2::CENTER_TOP
        .align_size_within_rect(galley.size(), caption_rect)
        .min;
    let caption_painter = column_ui.painter();
    for outline_offset in CAPTION_OUTLINE_OFFSETS {
        caption_painter.add(egui::Shape::galley(
            caption_position + outline_offset,
            galley.clone(),
            OUTLINE_COLOR,
        ));
    }
    caption_painter.add(egui::Shape::galley(
        caption_position,
        galley,
        caption.fill_color,
    ));
}
```

Design notes to hold during review:
- `PILL_GLYPH_COLOR` is consumed ONLY inside `pill_style` (`glyph_state_color`); consumers never read the constants directly except `zoom_button`'s stroked-square composition (§7, which uses `pill_style::OUTLINE_STROKE_WIDTH`/`pill_style::OUTLINE_COLOR` because it must build a stroke around a NOT-filled rect).
- `TriangularGlyphSpec`/`OutlinedCaption` fields are `pub(super)` so the sibling paint modules can struct-literal-construct them; nothing else is exposed beyond `pub(super)`.
- Method bodies: longest = `paint_outlined_caption` (~20 lines) and `outlined_triangle_shape` (~15) — both ≤50; max nesting depth 1 inside functions; every condition single-section.
- The 8-offset ring and stroke width reproduce the spec §4.2 constants verbatim.

## 4. Step 2 — create `src/app/pill_style_tests.rs` (NEW file, ≈60 lines, cap 200)

Create `src/app/pill_style_tests.rs` with EXACTLY:

```rust
//! Test module of `pill_style` (Wave 4.2 Task 2): the pinned styling
//! constants, the caption offset ring, and the shared palette resolution.

use eframe::egui;

use super::pill_style::{
    CAPTION_OUTLINE_OFFSETS, CAPTION_TEXT_SIZE, OUTLINE_COLOR, OUTLINE_STROKE_WIDTH,
    PILL_GLYPH_COLOR, glyph_state_color, glyph_state_for,
};

#[test]
fn pill_styling_colors_stroke_and_caption_size_pin_style_constants() {
    assert_eq!(PILL_GLYPH_COLOR, egui::Color32::WHITE);
    assert_eq!(OUTLINE_COLOR, egui::Color32::BLACK);
    assert_eq!(OUTLINE_STROKE_WIDTH, 1.0);
    assert_eq!(CAPTION_TEXT_SIZE, 9.0);
}

#[test]
fn caption_outline_offsets_match_the_one_point_neighbors() {
    let expected_offsets = [
        egui::vec2(-1.0, -1.0),
        egui::vec2(0.0, -1.0),
        egui::vec2(1.0, -1.0),
        egui::vec2(-1.0, 0.0),
        egui::vec2(1.0, 0.0),
        egui::vec2(-1.0, 1.0),
        egui::vec2(0.0, 1.0),
        egui::vec2(1.0, 1.0),
    ];
    assert_eq!(CAPTION_OUTLINE_OFFSETS, expected_offsets);
}

#[test]
fn glyph_state_resolves_the_enabled_white_and_disabled_theme_palette() {
    let context = egui::Context::default();
    context.run(Default::default(), |run_context| {
        egui::Area::new(egui::Id::new("pill_style_palette_test")).show(run_context, |test_ui| {
            let enabled_idle = glyph_state_color(glyph_state_for(true, false), test_ui);
            let enabled_hovered = glyph_state_color(glyph_state_for(true, true), test_ui);
            let disabled = glyph_state_color(glyph_state_for(false, true), test_ui);
            assert_eq!(enabled_idle, PILL_GLYPH_COLOR);
            assert_eq!(enabled_hovered, PILL_GLYPH_COLOR);
            let noninteractive_color = test_ui.visuals().widgets.noninteractive.fg_stroke.color;
            assert_eq!(disabled, noninteractive_color);
        });
    });
}
```

## 5. Step 3 — register the modules in `src/app.rs` (180 → 183)

Insert EXACTLY THREE lines, keeping every other row byte-identical:

1. In the module block, between line 161 `mod keyboard_intake;` and line 162 `mod zoom;` (alphabetical position):
```rust
mod pill_style;
```
(ONE new line — the existing block has no blank lines between mods, see lines 151–163; do not introduce any.)

2. In the test-module block, between lines 175–176 (`#[cfg(test)]` + `mod last_viewed_tests;`) and lines 177–178 (`#[cfg(test)] mod tests;`):
```rust
#[cfg(test)]
mod pill_style_tests;
```

Resulting blocks (for verification):
```rust
mod keyboard_intake;
mod pill_style;
mod zoom;
```
and
```rust
#[cfg(test)]
mod last_viewed_tests;
#[cfg(test)]
mod pill_style_tests;
#[cfg(test)]
mod tests;
```
No other change in `app.rs` — NO `ImagesScrollerApp` field changes (§18).

## 6. Step 4 — `src/app/button_hint.rs` (184 → ≈189): caption white + outlined paint + enablement plumbing

4.1 Module header: after line 4 (`//! button-column layout wraps every pill button's paint code.`) add ONE line:
```rust
//! Captions render through the shared pill_style outlined-caption paint (WHITE).
```

4.2 Imports: after line 10 (`use super::zoom_button::ZoomButtonAction;`) add:
```rust
use super::pill_style;
```
(std-order: the house sort is ASCII; `pill_style` sorts after `zoom_button`? It does NOT — `pill_style` < `zoom_button` alphabetically. Place it BEFORE line 10 instead, i.e. after line 8 (`use super::arrow_direction::ArrowControl;`) and before line 9 (`use super::fit_button::FitButtonAction;`) so the super-import order reads `arrow_direction`, `fit_button`, `pill_style`, `zoom_button`.)

4.3 Remove line 12 (`const CAPTION_TEXT_SIZE: f32 = 9.0;`) — the constant moved to `pill_style.rs` (§3). Line 13 `const CAPTION_TEXT_GAP: f32 = 2.0;` stays UNTOUCHED.

4.4 `PillButtonPlacement` gains the enablement datum: between the `pub(super) fn for_button` constructor impl block and `ButtonColumnSite` (currently lines 103–117), change the struct + impl to:

```rust
/// What the shared column helper needs about one pill button: which button
/// (caption + tooltip source), its painted size (the column's width; the
/// size consts stay in the button modules, single-source per module), and
/// its enablement (the caption fill color derives from it).
pub(super) struct PillButtonPlacement {
    button: PillButton,
    button_size: egui::Vec2,
    enabled: bool,
}

impl PillButtonPlacement {
    pub(super) fn for_button(button: PillButton, button_size: egui::Vec2) -> Self {
        Self {
            button,
            button_size,
            enabled: true,
        }
    }

    /// Sets the painted enablement the caption color derives from
    /// (painted-disabled buttons keep the dimmed caption).
    pub(super) fn with_enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }
}
```
(`for_button` defaults `enabled: true`; every caller that carries a real enablement chains `with_enabled`. The 2-arg limit holds: `for_button` = 2 args; `with_enabled(self, bool)` = 2.)

4.5 Replace the caption block inside `show_button_column` (lines 138–144) with:
```rust
            if let Some(reference) = hotkey_reference(button) {
                let caption_glyph_state =
                    pill_style::glyph_state_for(placement.enabled, painted_button.hovered());
                let caption_fill_color =
                    pill_style::glyph_state_color(caption_glyph_state, column_ui);
                pill_style::paint_outlined_caption(
                    column_ui,
                    pill_style::OutlinedCaption {
                        text: reference,
                        fill_color: caption_fill_color,
                    },
                );
            }
```
- The fill color resolves DISABLED → theme `noninteractive` dim (placed-dimming preserved, spec §2); enabled (incl. hovered) → `PILL_GLYPH_COLOR` WHITE.
- The color borrow completes BEFORE `paint_outlined_caption` mutably borrows `column_ui` (order is binding — two separate `let`s, not one expression).
- Everything else in `show_button_column` stays byte-identical (allocation, tooltip chain, `column_response.inner` return).
- The two inline tests of `button_hint.rs` (`hotkey_reference_matches_the_spec`, `tooltip_text_matches_the_spec`) stay UNTOUCHED.

## 7. Step 5 — `src/app/floating_controls.rs` (191 → ≈188): arrow triangles outlined + white

7.1 Module header: after line 7 add TWO lines (replace nothing):
```rust
//! (Wave-4.2 Task 2:) every painted glyph carries the shared 1-point BLACK
//! pill outline (`src/app/pill_style.rs`); enabled glyphs + captions are WHITE.
```

7.2 Imports: insert after line 11 (`use super::fullscreen_button;`) and before line 15 (`use super::zoom_button;`):
```rust
use super::pill_style;
```
(super-import order reads: arrow_direction, button_hint, fit_button, fullscreen_button, pill_style, zoom_button.)

7.3 Replace `paint_triangle` (lines 169–178) with:
```rust
    fn paint_triangle(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let triangle_rect =
            egui::Rect::from_center_size(button_response.rect.center(), ARROW_TRIANGLE_SIZE);
        let glyph_state = pill_style::glyph_state_for(self.enabled, button_response.hovered());
        let triangle = pill_style::outlined_triangle_shape(pill_style::TriangularGlyphSpec {
            rect: triangle_rect,
            rotation: self.control.painted_direction().paint_rotation(),
            fill_color: pill_style::glyph_state_color(glyph_state, row_ui),
        });
        row_ui.painter().add(triangle);
    }
```
The triangle geometry (rect from `ARROW_TRIANGLE_SIZE` centered on the button response, rotation from `ArrowDirection::paint_rotation`, unmodified) is EXACTLY the old `Shape::rotated_triangle` geometry; only the color source and the outside black outline change.

7.4 DELETE `arrow_color` and its doc comment (lines 180–190) — the shared `pill_style` palette replaces it. NOTHING else changes (allocation, widget info, cursor, `click_direction`, `accepts_click`, pills' other fns all untouched).

## 8. Step 6 — `src/app/fullscreen_button.rs` (55 → ≈52): filled square outlined + white

8.1 Module header: after line 5 add ONE line:
```rust
//! Wave-4.2 Task 2: the square carries the shared 1-point black pill outline.
```

8.2 Imports: after line 10 (`use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};`) add:
```rust
use super::pill_style;
```

8.3 Replace `paint_fullscreen_square` (lines 37–46) with:
```rust
fn paint_fullscreen_square(row_ui: &egui::Ui, button_response: &egui::Response) {
    let square_rect =
        egui::Rect::from_center_size(button_response.rect.center(), FULLSCREEN_SQUARE_SIZE);
    let glyph_state = pill_style::glyph_state_for(true, button_response.hovered());
    let square = pill_style::outlined_rect_shape(
        square_rect,
        pill_style::glyph_state_color(glyph_state, row_ui),
    );
    row_ui.painter().add(square);
}
```
(`true` is the button's documented ALWAYS-ENABLED fact, no behavior change; the fill rect is EXACTLY the old `rect_filled` rect; the black outline paints outside.)
`FULLSCREEN_SQUARE_SIZE` and the allocate/widget-info/cursor code stay unchanged.

8.4 DELETE `fullscreen_square_color` and its doc comment (lines 48–54).

## 9. Step 7 — `src/app/zoom_button.rs` (196 → ≈184): bars/marker outlined + white, palette helper removed

9.1 Module header: after line 5 add ONE line:
```rust
//! Wave-4.2 Task 2: every glyph carries the shared 1-point black pill outline.
```

9.2 Imports: after line 10 (`use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};`) add:
```rust
use super::pill_style;
```

9.3 In `paint_glyph` (line 123), replace
```rust
        let glyph_color = self.zoom_glyph_color(row_ui, button_response);
```
with
```rust
        let glyph_state = pill_style::glyph_state_for(self.enabled, button_response.hovered());
        let glyph_color = pill_style::glyph_state_color(glyph_state, row_ui);
```

9.4 DELETE `zoom_glyph_color` (lines 105–117).

9.5 Replace the BODY of `crossed_bar_shapes` (doc comment line 134 stays; body lines 135–157) with:
```rust
    fn crossed_bar_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let horizontal_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(GLYPH_BAR_LENGTH, GLYPH_BAR_THICKNESS),
        );
        let vertical_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(GLYPH_BAR_THICKNESS, GLYPH_BAR_LENGTH),
        );
        vec![
            pill_style::outlined_rect_shape(horizontal_rect, glyph_color),
            pill_style::outlined_rect_shape(vertical_rect, glyph_color),
        ]
    }
```
Bar rects are EXACTLY the old `rect_filled` rects (13×2.5 / 2.5×13, `CornerRadius::ZERO`); each gains the outside black border.

9.6 Replace the BODY of `single_bar_shapes` (doc line 159 stays; body 160–174) with:
```rust
    fn single_bar_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let bar_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(GLYPH_BAR_LENGTH, GLYPH_BAR_THICKNESS),
        );
        vec![pill_style::outlined_rect_shape(bar_rect, glyph_color)]
    }
```

9.7 Update doc + body of `stroked_square_shapes` (lines 176–195): extend the doc with one line
```rust
    /// Wave-4.2: the marker squares gain a black outline painted OUTSIDE of
    /// the marker rect; the glyph-colored marker stroke itself is unchanged.
```
and replace the body with:
```rust
    fn stroked_square_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let marker_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(FIT_MARKER_SIDE, FIT_MARKER_SIDE),
        );
        let outline_rect = marker_rect.expand(pill_style::OUTLINE_STROKE_WIDTH);
        let outline = egui::Shape::rect_stroke(
            outline_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(pill_style::OUTLINE_STROKE_WIDTH, pill_style::OUTLINE_COLOR),
            egui::StrokeKind::Outside,
        );
        let marker = egui::Shape::rect_stroke(
            marker_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(FIT_MARKER_STROKE_WIDTH, glyph_color),
            egui::StrokeKind::Inside,
        );
        vec![outline, marker]
    }
```
(Paint order is binding: BLACK outline FIRST, glyph-colored marker SECOND — the glyph stroke stays `StrokeKind::Inside` on the unexpanded `marker_rect`, so the visible square keeps its exact size; the `rect_stroke` REQUIRES explicit `StrokeKind` note in the old doc stays valid.)
Everything else in the module is UNCHANGED (consts, `ZOOM_{IN,OUT,RESTORE_FIT}_BUTTON_LABEL`, `ZOOM_GROUP_GAP`, specs/labels/apply/accepts fns).

## 10. Step 8 — `src/app/fit_button.rs` (182 → ≈175): axis lines + triangles outlined + white

10.1 Module header: after line 1 add ONE line:
```rust
//! Wave-4.2 Task 2: every glyph carries the shared 1-point black pill outline.
```

10.2 Imports: after line 7 (`use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};`) add:
```rust
use super::pill_style;
```

10.3 DELETE `fit_glyph_color` (lines 104–116).

10.4 In `paint_glyph` (line 120), replace
```rust
        let glyph_color = self.fit_glyph_color(row_ui, button_response);
```
with
```rust
        let glyph_state = pill_style::glyph_state_for(self.enabled, button_response.hovered());
        let glyph_color = pill_style::glyph_state_color(glyph_state, row_ui);
```
(`FitButtonAction::glyph_shapes` keeps receiving `glyph_color: egui::Color32` and stays UNCHANGED in signature — only the Vec content changes below.)

10.5 In `FitButtonAction::glyph_shapes` (body lines 130–163), replace the `vec![...]` block (lines 146–162) with:
```rust
        vec![
            if horizontal {
                horizontal_axis_line_shape(glyph_center, glyph_color)
            } else {
                vertical_axis_line_shape(glyph_center, glyph_color)
            },
            pill_style::outlined_triangle_shape(pill_style::TriangularGlyphSpec {
                rect: egui::Rect::from_center_size(
                    glyph_center - axis_offset,
                    FIT_ARROW_TRIANGLE_SIZE,
                ),
                rotation: minus_facing.paint_rotation(),
                fill_color: glyph_color,
            }),
            pill_style::outlined_triangle_shape(pill_style::TriangularGlyphSpec {
                rect: egui::Rect::from_center_size(
                    glyph_center + axis_offset,
                    FIT_ARROW_TRIANGLE_SIZE,
                ),
                rotation: plus_facing.paint_rotation(),
                fill_color: glyph_color,
            }),
        ]
```
(Triangle rects, offsets `FIT_ARROW_SPACING`, directions, and the doc comment above `glyph_shapes` are unchanged; only the shape call differs.)

10.6 Replace the two axis-line helpers (lines 166–182) with:
```rust
fn horizontal_axis_line_shape(glyph_center: egui::Pos2, glyph_color: egui::Color32) -> egui::Shape {
    let line_rect = egui::Rect::from_center_size(
        glyph_center,
        egui::Vec2::new(FIT_AXIS_LINE_LENGTH, FIT_AXIS_LINE_THICKNESS),
    );
    pill_style::outlined_rect_shape(line_rect, glyph_color)
}

fn vertical_axis_line_shape(glyph_center: egui::Pos2, glyph_color: egui::Color32) -> egui::Shape {
    let line_rect = egui::Rect::from_center_size(
        glyph_center,
        egui::Vec2::new(FIT_AXIS_LINE_THICKNESS, FIT_AXIS_LINE_LENGTH),
    );
    pill_style::outlined_rect_shape(line_rect, glyph_color)
}
```
(All six glyph-geometry consts and the `pub(super)` labels stay UNTOUCHED.)

## 11. Step 9 — files that must NOT change in this task

`src/app/arrow_direction.rs` (66 — the triangles' rotation space is byte-identical), `src/app/central_surface.rs`, `src/app/keyboard_intake.rs`, `src/app/fullscreen.rs`, `src/app/zoom.rs`, `src/app/folder_navigation.rs`, `src/app_display.rs`, `src/scaling.rs`, `src/zoom.rs`, all `tests`/`*_tests.rs` EXCEPT the new `pill_style_tests.rs`, `src/main.rs`, `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, all existing `.kilo/plans/*`, all docs (`README.md`, `tech.md`, `architecture.md` — A-Task 6's authority). If any of these seems needed → STOP and ask the caller.

## 12. Verification ownership and expected A-Task-5 results (RECORDED HERE, NOT RUN HERE)

Per the wave plan (global plan §A-Task 5; caller note), all harness work is A-Task 5's — this plan only RECORDS what A-Task 5 must confirm against THIS task's code:

1. Formatting round (FIRST, before dev checks; container on the Alpine VM — `vm_status` first, then `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt`; exit code authoritative; per `docs/build.md`).
2. Dev checks: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` → final line `ALL CHECKS PASSED` (exit 0); the `cargo test` line must read `107 passed; 0 failed` (§14 census). Gitignored logs land under `logs/checks/`.
3. Release rebuild + DLL audit = the recorded 22-DLL constant unchanged, 0.4.2 exe — A-Task 5's own sub-steps (not repeated here).
4. Line-budget re-measure after fmt: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "wc -l src/app.rs src/app/*.rs"` vs the §13 table.

## 13. Line-budget validation table (implementer + reviewers; hard cap 200 per file)

| File | HEAD | Expected after this task | Cap |
|---|---|---|---|
| `src/app/pill_style.rs` | — | ≈135 | ≤200 |
| `src/app/pill_style_tests.rs` | — | ≈60 | ≤200 |
| `src/app.rs` | 180 | 183 | ≤200 |
| `src/app/button_hint.rs` | 184 | ≈189 | ≤200 |
| `src/app/floating_controls.rs` | 191 | ≈188 | ≤200 |
| `src/app/fullscreen_button.rs` | 55 | ≈52 | ≤200 |
| `src/app/zoom_button.rs` | 196 | ≈184 | ≤200 |
| `src/app/fit_button.rs` | 182 | ≈175 | ≤200 |
| `src/app/arrow_direction.rs` | 66 | 66 (untouched) | ≤200 |

STOP conditions: any touched file lands >200 after the deferred fmt round (A-Task 5) → the overflow is a review-cycle fix (§19), and only local line re-wraps (including closure/`vec![]` bodies) are allowed locally; NO autonomous restructuring. Method-length rule (≤50 body lines) holds by construction (§3 notes). No new method/function gains a third argument anywhere (§0.1 entry 4 protects this).

## 14. Test plan

- **New (in `src/app/pill_style_tests.rs`, exactly these three names, in this order):**
  1. `pill_styling_colors_stroke_and_caption_size_pin_style_constants` — 4 constant pins (`WHITE`, `BLACK`, `1.0`, `9.0`).
  2. `caption_outline_offsets_match_the_one_point_neighbors` — the 8-entry offset ring pinned in spec order.
  3. `glyph_state_resolves_the_enabled_white_and_disabled_theme_palette` — idle/hovered → WHITE; disabled → theme `noninteractive.fg_stroke.color` (egui Context/Area test harness per §4).
- **Census: 104 → 107** (+3 `#[test]` items; no test removed; the two `button_hint` inline tests survive byte-identical). Census check is the A-Task-5 harness `cargo test` line (§12).
- GUI-bound painting (shapes render, outline visible) is intentionally untested per the house scope (pure logic only); operator validation of the outline/white visuals happens at the 0.4.2 OPERATOR-RUN.
- Exe rebuild / DLL audit / OPERATOR-RUN are NOT part of this task (A-Task 5 + the operator).

## 15. Documentation duties inside this step (commits 1 and 3) — content pinned

- **Commit 1** — the two plan/spec markdown files + `context.md` (its ALREADY-present Task-2 4.1a diff; inspect before staging; append-only).
- **Commit 3** —
  1. `.agent/project-info/context.md`: prepend ONE `Recent Changes` entry (dated 2026-11-07): Task 2 steps 4.1a+4.1b complete — the 4.1a spec + this implementation-plan paths, the pinned constants (captions/icons WHITE + 1-pt BLACK outlines), the new `src/app/pill_style.rs` + `pill_style_tests.rs` modules, +3 tests (104 → 107), and the commit hashes of commits 1–2 once known; update `Current Work Focus` to "Task 2 implementation committed (4.2) — remainder of the cycle pending"; bump the HEAD note. Preserve every other entry byte-identical.
  2. `.agent/project-structure.md`: in the `app/` bullet — insert `pill_style.rs (the shared pill-styling primitives: glyph/caption color palette, the 1-point black outline constants, the outlined shape builders, and the outlined hotkey-caption paint)` in the module list at its alphabetical position, and add `pill_style_tests.rs` to that bullet's test-module list; nothing else changes in the file.
- The six `//!` header bullets in the touched paint modules (§6–§10) are part of the outstanding feat code, owned by the implementer; 4.4's docs-specialist may polish their wording without changing any fact (`docs: wave4.2 task2 pill style module docs`); `README.md`/`tech.md`/`architecture.md` are A-Task 6's authority (deferred per the wave plan), never touched here.

## 16. Pre-commit checklist (run before EVERY commit)

1. Read `.gitignore`; `git status` clean of ignored-pattern files; nothing under `target/`, `dist/`, `logs/` staged.
2. `git diff --cached` matches exactly the named file set for that commit (§2).
3. No `CHANGELOG.md` change, no existing `.kilo/plans/*` modification, no TODO-file change included.
4. Commit message matches §2 verbatim.
5. `git log --oneline -4` afterwards shows the expected chain; `git branch --show-current` still `fix/wave-4-2-operator-feedback` (no merge/push actions — workflow Step A.5).
6. Consecutive tasks in this wave reuse this checklist; a failed checklist → fix, re-add, rewrite the commit only if unpushed.

## 17. Code-review heads-up (for 4.3 reviewers)

- Nesting: `show_button_column`'s caption `if let` is the already-flagged 3rd-level shape from Wave-4.1 Task 2 — same disposition: extraction is a simplification-plan decision, NOT an implementer-side change here.
- `zoom_button.rs` lands ≈184/200 — safe margin, no planned merges; do NOT re-open the shape function split.
- The `Shape::Vec` nesting in `outlined_rect_shape` each frame allocates a small `Vec` per outlined rect glyph — assessed negligible (counts: zoom 3, fullscreen 1, fit 2 → 6 small vecs/frame; no textures, no persistent state; matches the 0.4.1 repaint model), keep unless the simplifier measures otherwise.
- The 8 offset copies per captioned button (6 buttons × 8 = 48 black text shapes/frame) replace 6 labels — Arc-clone-only, layout once per galley; same per-frame allocation class as the previous Labels.
- `glyph_state_color`'s `Hovered | Idle` arm-or pattern is deliberate (both pin WHITE despite different request states); do not split it.
- `with_enabled` returning `Self { enabled, ..self }` relies on the struct's field-wise syntax; keep the builder 2-arg, do not fold into `for_button`.

## 18. Out-of-scope guard (hard boundaries for the implementer)

- Clicks, enablement predicates, navigation/zoom/fit seams, keyboard intake, fullscreen gating, zoom clamps, fit factors — ALL unchanged. The pill renders inside the chrome gate, so while fullscreen is ON nothing renders — no extra gating introduced.
- No `ImagesScrollerApp` field added; no stored caption/outline state; `zoom_factor`/`fullscreen_on` untouched.
- Group gaps (`FOLDER_GROUP_GAP`, `FULLSCREEN_GROUP_GAP`, `ZOOM_GROUP_GAP`, `FIT_GROUP_GAP`), button sizes, glyph sizes/consts, message line, pill frame/margins/offset — unchanged (the pill may grow ~12 px taller ONLY as the caption block already does since Wave 4.1; this task changes NO layout value).
- Caption strings (`F11`/`I`/`O`/`0`/`W`/`H`) and tooltip strings byte-identical; tooltips still attach to both enabled and disabled buttons.
- Nothing in Wave-4.2 Tasks 3–4 territory (Shift-alias fix, ESC popup) may be touched, even where files could overlap. Neither the parked future-ideas nor any rule beyond this plan.

## 19. Code review + simplification cycle steps (4.3, after 4.2)

1. Reviewer reads the landed diff against §0–§13; produces a findings report; if any finding requires a code change, the reviewer writes a plan file (NEW): `.kilo/plans/20261107-wave4-2-task2-pill-styling-review-fix.md`; the implementer applies it in a follow-up commit (message form `fix: wave4.2 task2 review corrections` — adapt to the actual finding; keep the task file set identical).
2. Simplifier may then propose non-behavioral simplifications as (NEW) `.kilo/plans/20261107-wave4-2-task2-pill-styling-simplification.md`; applied only if they keep §18 intact.
3. Max 3 review cycles per task, then escalate to the caller (Critical Workflow rule).
4. A-Task 5's harness remains the AUTOMATED gate: any compile/clippy/fmt defect discovered there is remediated through the same review-fix path (never a silent edit inside the verification step).

## 20. Conformance crosswalk → 4.5a frontend verification (spec §7 item ↔ plan step)

| 4.5a checklist item (spec §7) | Plan step |
|---|---|
| Window/launch/fullscreen/navigation/display behavior unchanged | §11 + §18 (no logic files touched) |
| Caption strings unchanged (`F11/I/O/0/W/H`; arrows uncaptioned) | §6.5 (resolves via unchanged `hotkey_reference`); existing pinned tests |
| Caption size 9.0 | §3 constant + §14 test 1 |
| Caption color WHITE (enabled); dims via theme when disabled | §3 `glyph_state_color` + §6.5 + §14 test 3 |
| 1-pt black caption outline via 8-offset copies; no un-outlined caption | §3 `paint_outlined_caption` + §6.5 (every caption path routes through it) |
| Tooltip strings + both-state tooltips unchanged | §6 (block replaced BELOW the tooltip chain — chain untouched) |
| Button enablement/dimming unchanged (0.4.1 semantics recreated) | §0 decision 3 + §6–§10 (disabled → theme noninteractive, matching old disabled branch) |
| Layout geometry unchanged | §18 (only color/outline/stroke changed) |
| ALL painted icons outlined (15-shape inventory: 4 arrows, 2+1 zoom bars, restore-fit square, fullscreen square, 2 fit lines + 4 fit triangles) | §7 (arrows), §9 (zoom bars/marker), §8 (fullscreen), §10 (fit glyphs) |
| Outline color/width consistent (`BLACK`, `1.0`) | §3 consts (single source) + §14 test 1 |
| Colored geometry preserved (outside strokes only) | §3 builders + §9.7 double-stroke composition |
| No per-frame allocation churn class change | §17 (assessed: 6 vec + galley reuse; no new persistent state) |
| File-size/line-count rules respected; helpers private-default (`pub(super)` only) | §13 table + §1 cap + §0 notes |
| Tests pass; census +2/+3 | §14 (107 expected; the spec's +2–3 range upper bound) |
| 0.4.1 OPERATOR-RUN items unaffected | §18 + A-Task-5 checklist |

## 21. Downstream notes for the caller (4.2 → 4.6)

- **4.2 (implementer):** execute §1–§11, then §15/§16 commits; the code snippets are authoritative shapes — minor rustfmt reflow is fine, semantic changes are NOT. NO harness invocation (§12).
- **4.3 (reviewer + simplifier):** envelope = §17 + §19; expect a small/no-change report (mechanical styling task); findings route through the named NEW plan files.
- **4.4 (docs-specialist):** module-header bullets already land with the code (§6–§10); only polish permitted; the user-facing docs belong to A-Task 6 (README/tech/architecture/context + brief addendum line).
- **4.5a (frontend-specialist):** verify against the 4.1a spec §7 using the §20 crosswalk; record the four §0.1 reconciliations as the plan-side API adaptations (they preserve the spec's intent exactly).
- **4.5b (architector):** adherence report comparing the landed diff to this plan's step list; §13 budget within ±2 lines excluding the two NEW files' estimated ranges.
- **4.6 (implementer):** mark Task 2 sub-items `[x]` on `[ ]` only in `.agent/todos/20261107/20261107-todo-1.md`; commit `task: mark wave4.2 task2 pill styling done`.
- **Operator-run relevance (A-Task 5 checklist note):** captions render WHITE with the black font border over any background; every painted icon shows its 1-pt black border; dimmed-disabled states keep the theme-dim color + outline; all other 0.4.1 visuals unchanged.

---

**Verification against TODO Task 2:** all three sub-items are covered — (1) the six hotkey captions render WHITE (`PILL_GLYPH_COLOR` pin + palette test); (2) EVERY painted pill icon gains the 1-point black outline stroke at its exact paint call-site (§7–§10, 15-shape inventory fully enumerated); (3) every caption gains the black font border via the offset-copy outline with the verified 0.37 galley recolor (§3 `paint_outlined_caption`). Front-end loop honored: 4.1a spec consumed whole, deviations documented with docs.rs evidence (§0.1), A-Task 5 owns all harness execution per the wave plan.
