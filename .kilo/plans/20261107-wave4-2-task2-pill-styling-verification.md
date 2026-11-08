# Front-end Implementation Verification Report — Wave 4.2 Task 2: Pill Captions White + Thin Black Border

**Task:** A-Task 2 Step 4.5a (Front-end Implementation Verification)  
**Branch:** `fix/wave-4-2-operator-feedback`  
**HEAD reviewed:** `d90e9ee` (`fix: wave4.2 task2 simplification (dedupe fit glyph builders, sibling button-hint tests)`)  
**Implementation commits:** `4c3c959` (feature) + `d90e9ee` (4.3 simplifications)  
**Spec:** `.kilo/plans/20261107-wave4-2-task2-pill-styling-frontend-spec.md`  
**Report path:** `.kilo/plans/20261107-wave4-2-task2-pill-styling-verification.md`

---

## 1. Scope & method

Static code review of `src/` against the 4.1a spec. No source edits, commits, or pushes performed.
Working-tree note: one untracked plan file exists (`.kilo/plans/20261107-wave4-2-task2-simplification.md`, the 4.3 simplification plan); it is outside `src/` and outside the front-end spec scope.

---

## 2. Executive verdict

**All spec-conformance items pass code review.** The implementation matches the spec for caption color, caption font outline, icon outline coverage, outline constants, geometry preservation, layout/string/enablement preservation, shared-module structure, and private-by-default API surface.

The two 4.3 post-spec simplifications (`fit_button.rs` axis-line/triangle fold; `button_hint.rs` inline tests → sibling `button_hint_tests.rs`) are behavior-neutral versus the spec.

Two **OPERATOR-RUN** visual-quality observations are recorded for the manual gate (not agent-failable).

Build/test pass is pending the implementer/architector harness evidence expected in A-Task 5 Step 4.2/4.5b.

---

## 3. §7 conformance checklist — item-by-item

### 3.1 Window / launch / fullscreen behavior unchanged
**Status:** PASS  
**Evidence:** `git diff 8eefdde..d90e9ee -- src/` touches only `src/app/*.rs`; `src/app.rs` changes are limited to `mod pill_style;` and the `button_hint_tests` registration. No changes to `window_options()`, `fullscreen_on`, `show_chrome_or_fullscreen_layout()`, `keyboard_intake.rs`, `zoom.rs`, `folder_navigation.rs`, or `app_display.rs`.

### 3.2 Caption strings unchanged
**Status:** PASS  
**Evidence:** `src/app/button_hint.rs:58-70` (`hotkey_reference`) and `:74-86` (`tooltip_text`) retain the exact strings: `F11`, `I`, `O`, `0`, `W`, `H`. `src/app/button_hint_tests.rs:6-32` re-asserts all 24 string pins byte-identically.

### 3.3 Caption size unchanged
**Status:** PASS  
**Evidence:** `src/app/pill_style.rs:21` declares `pub(super) const CAPTION_TEXT_SIZE: f32 = 9.0;`. `src/app/pill_style_tests.rs:15` asserts it equals `9.0`.

### 3.4 Caption color white
**Status:** PASS  
**Evidence:** `src/app/pill_style.rs:12` declares `PILL_GLYPH_COLOR = egui::Color32::WHITE`. `glyph_state_color` (`pill_style.rs:57-61`) returns `PILL_GLYPH_COLOR` for both `Idle` and `Hovered`, and the theme `noninteractive.fg_stroke.color` for `Disabled`. The caption path in `button_hint.rs:149-152` resolves the same state/color as the glyph, so enabled captions are white and disabled captions are dimmed identically to 0.4.1.

### 3.5 Caption font border
**Status:** PASS  
**Evidence:** `src/app/pill_style.rs:24-33` defines the 8 one-point neighbor offsets. `paint_outlined_caption` (`pill_style.rs:122-146`) lays out one placeholder-colored galley, allocates the exact size, then paints 8 black outline copies (`OUTLINE_COLOR`) at the neighbor offsets plus one fill-colored copy at the base position. No caption is painted without the outline ring. `pill_style_tests.rs:20-31` asserts the offset ring matches the spec exactly.

### 3.6 Tooltip strings unchanged
**Status:** PASS  
**Evidence:** Same source as 3.2; `tooltip_text` table unchanged. Strings: `"Previous image (Left Arrow)"`, `"Next image (Right Arrow)"`, `"Previous sibling folder (Up Arrow)"`, `"Next sibling folder (Down Arrow)"`, `"Fullscreen (F11)"`, `"Zoom in (I)"`, `"Zoom out (O)"`, `"Restore fit (0)"`, `"Fit width (W)"`, `"Fit height (H)"`.

### 3.7 Tooltip behavior unchanged
**Status:** PASS  
**Evidence:** `src/app/button_hint.rs:145-147` attaches both `.on_hover_text(tooltip)` and `.on_disabled_hover_text(tooltip)` to the same response, preserving 0.4.1 behavior for enabled and disabled buttons.

### 3.8 Button enablement/dimming unchanged
**Status:** PASS  
**Evidence:** Consumer modules pass their existing `self.enabled` into `PillButtonPlacement::with_enabled`: `floating_controls.rs:132-136`, `zoom_button.rs:62-66`, `fit_button.rs:58-62`. `fullscreen_button.rs` keeps the default `enabled: true`. `glyph_state_color` maps disabled to `widgets.noninteractive.fg_stroke.color`, matching the removed per-module `arrow_color`/`zoom_glyph_color`/`fit_glyph_color`/`fullscreen_square_color` logic. Grep confirms zero residue of those helpers in `src/`.

### 3.9 Layout geometry unchanged
**Status:** PASS  
**Evidence:** All pill-layout constants remain untouched: `FLOATING_PILL_*` in `floating_controls.rs:21-33`, `ARROW_BUTTON_SIZE`/`ARROW_TRIANGLE_SIZE` (`:27-28`), `FOLDER_GROUP_GAP`/`FULLSCREEN_GROUP_GAP` (`:29-32`), `ZOOM_BUTTON_SIZE`/`ZOOM_GROUP_GAP`/`GLYPH_BAR_*`/`FIT_MARKER_*` in `zoom_button.rs:14-19`, `FIT_BUTTON_SIZE`/`FIT_GROUP_GAP`/`FIT_AXIS_LINE_*`/`FIT_ARROW_*` in `fit_button.rs:14-19`, `FULLSCREEN_BUTTON_SIZE`/`FULLSCREEN_SQUARE_SIZE` in `fullscreen_button.rs:15-16`, `CAPTION_TEXT_GAP = 2.0` in `button_hint.rs:14`.

### 3.10 All painted icons outlined — inventory of 15 shapes
**Status:** PASS  
All 15 inventoried shapes now route through the shared outline builders or an explicit black ring:

| # | Shape | Location | Outline evidence |
|---|-------|----------|------------------|
| 1 | Left arrow triangle | `src/app/floating_controls.rs:173-182` | `pill_style::outlined_triangle_shape(...)` with `PathStroke::outside()` |
| 2 | Right arrow triangle | same as #1 | same as #1 |
| 3 | Up arrow triangle | same as #1 | same as #1 |
| 4 | Down arrow triangle | same as #1 | same as #1 |
| 5 | Zoom-in `[+]` horizontal bar | `src/app/zoom_button.rs:130-141` | `pill_style::outlined_rect_shape(horizontal_rect, ...)` |
| 6 | Zoom-in `[+]` vertical bar | same as #5 | same as #5 |
| 7 | Zoom-out `[−]` bar | `src/app/zoom_button.rs:150-154` | `pill_style::outlined_rect_shape(bar_rect, ...)` |
| 8 | Restore-fit stroked square | `src/app/zoom_button.rs:167-184` | Black `rect_stroke` on `outline_rect.expand(OUTLINE_STROKE_WIDTH)` with `StrokeKind::Outside`, original marker on top |
| 9 | Fullscreen filled square | `src/app/fullscreen_button.rs:39-47` | `pill_style::outlined_rect_shape(square_rect, ...)` |
| 10 | Fit-width axis line | `src/app/fit_button.rs:141-150` | `pill_style::outlined_rect_shape(line_rect, ...)` |
| 11 | Fit-height axis line | same as #10 | same as #10 |
| 12 | Fit-width minus-facing triangle | `src/app/fit_button.rs:142-148` / `:151-152` | `pill_style::outlined_triangle_shape(...)` |
| 13 | Fit-width plus-facing triangle | same as #12 | same as #12 |
| 14 | Fit-height minus-facing triangle | same as #12 | same as #12 |
| 15 | Fit-height plus-facing triangle | same as #12 | same as #12 |

### 3.11 Outline color/width consistent
**Status:** PASS  
**Evidence:** `src/app/pill_style.rs:15` `OUTLINE_COLOR = egui::Color32::BLACK`; `:18` `OUTLINE_STROKE_WIDTH = 1.0`. `zoom_button.rs:171-176` reads the same constants for the restore-fit black ring. `pill_style_tests.rs:13-15` asserts all three pinned values.

### 3.12 Colored geometry preserved
**Status:** PASS  
**Evidence:** Triangles use `PathStroke::outside()` (`pill_style.rs:90`). Rectangles use `StrokeKind::Outside` (`pill_style.rs:105` and `zoom_button.rs:176`). Filled rect interiors use the original un-expanded rect (`pill_style.rs:100`). The restore-fit marker keeps its original `marker_rect` and `FIT_MARKER_STROKE_WIDTH` (`zoom_button.rs:178-183`).

### 3.13 No per-frame allocation churn
**Status:** PASS  
**Evidence:** No new persistent state fields were added to `ImagesScrollerApp`. Shapes are rebuilt each frame as in 0.4.1; the only new per-frame work is the caption galley layout (once per caption per frame, equivalent to the previous label layout) and the 9-shape caption paint. No texture uploads or persistent allocations are introduced.

### 3.14 File-size / line-count rules respected
**Status:** PASS  
**Evidence:** Reviewed file line counts (last line number from `read`):

- `src/app/pill_style.rs` — 146 lines
- `src/app/button_hint.rs` — 165 lines
- `src/app/button_hint_tests.rs` — 33 lines
- `src/app/fit_button.rs` — 155 lines
- `src/app/floating_controls.rs` — 184 lines
- `src/app/fullscreen_button.rs` — 48 lines
- `src/app/zoom_button.rs` — 186 lines
- `src/app/pill_style_tests.rs` — 48 lines
- `src/app.rs` — 185 lines

All are ≤ 200 lines. Method signatures are ≤ 2 parameters (the two shape builders take a single struct-literal argument; `paint_outlined_caption` takes a single struct-literal argument). All `pill_style` exports are `pub(super)`; consumer imports are `use super::pill_style;`. No new crate-public API surface.

### 3.15 Tests pass
**Status:** PENDING — awaiting implementer/architector harness evidence  
**Evidence:** Code review shows the expected test census. Base was 104 unit tests. `pill_style_tests.rs` adds 3 new tests (`pill_styling_colors_stroke_and_caption_size_pin_style_constants`, `caption_outline_offsets_match_the_one_point_neighbors`, `glyph_state_resolves_the_enabled_white_and_disabled_theme_palette`). The 2 existing `button_hint` tests moved verbatim to `button_hint_tests.rs` with no net change. Expected total: 107, which matches the spec range 106–107. The actual `cargo test` pass/fail count must be confirmed by the A-Task 5 harness run (`scripts/dev-checks.sh` literal `ALL CHECKS PASSED`).

### 3.16 No regression in OPERATOR-RUN checklist items
**Status:** PENDING — manual gate  
**Evidence:** No code changes outside the pill paint/caption paths. All 0.4.1 behaviors outside the two styling changes (white captions + black outlines) remain structurally unchanged.

---

## 4. Post-spec simplifications verification

### S1 — `fit_button.rs` fold of axis-line helpers and triangle literals
**Status:** Behavior-neutral  
**Evidence:** `git diff 8eefdde..d90e9ee -- src/app/fit_button.rs` removes `horizontal_axis_line_shape` and `vertical_axis_line_shape` and inlines their geometry into `FitButtonAction::glyph_shapes`. The `horizontal` boolean selects the same `(LENGTH, THICKNESS)` / `(THICKNESS, LENGTH)` pairings, the same `axis_offset`, the same `(minus_facing, plus_facing)` pair, and paints in the same order `[axis_line, minus_triangle, plus_triangle]`. Triangle centers remain `glyph_center ± axis_offset` with identical `FIT_ARROW_TRIANGLE_SIZE` and `paint_rotation()`. The closure `outlined_triangle_at` is not a new named function, so the ≤2-param rule is unaffected.

### S2 — `button_hint.rs` inline tests → sibling `button_hint_tests.rs`
**Status:** Behavior-neutral  
**Evidence:** The diff moves the two test bodies verbatim; the only changes are the module wrapper and the `use` line. The two data functions `hotkey_reference` and `tooltip_text` were widened from private to `pub(super)`, which is still module-family-private and matches the existing sibling-test pattern (e.g., `pill_style_tests.rs`). No assertions were added or removed; test census unchanged.

---

## 5. Front-end quality issues — OPERATOR-RUN observations

These are visual/UX concerns to be checked at the manual gate. They are **not** treated as agent-failable code defects because the implementation exactly follows the spec constants.

1. **White-on-black 1 pt stroke legibility at default DPI**  
   The caption outline and icon outline are both 1 point. On high-contrast backgrounds the thin black ring may be subtle; the operator should verify that captions and icons remain crisp and readable against both dark and light image content.

2. **Dimming visibility of outlined disabled states**  
   Disabled glyphs/captions use the theme `noninteractive.fg_stroke.color` (typically a mid-gray). The 1-point black outline is still drawn around the disabled shape/caption. The operator should confirm that disabled buttons are visually distinguishable from enabled white buttons and that the black outline does not make disabled elements appear active.

---

## 6. Nonconformance list

None. All checklist items conform at the code-review level; the only pending items are external build/test evidence (3.15) and manual operator observations (3.16 / §5).

---

## 7. Fix steps

No fix steps are required for spec nonconformance.

For the OPERATOR-RUN observations in §5, if the manual gate finds the 1 pt outline insufficient, the fix would be a scope change (not this task) to increase `OUTLINE_STROKE_WIDTH` and/or adjust `CAPTION_OUTLINE_OFFSETS` after operator approval.

(End of report)
