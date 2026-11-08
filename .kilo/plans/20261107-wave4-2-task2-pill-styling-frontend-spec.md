# Front-end Technical Specification — Wave 4.2 Task 2: Pill Captions White + Thin Black Border on Icons and Captions

**Scope:** front-end styling only. No navigation, zoom, fullscreen, or image-display behavior changes.
**Base:** `main` @ `88c45f7` (0.4.1).
**Spec path:** `.kilo/plans/20261107-wave4-2-task2-pill-styling-frontend-spec.md`

---

## 1. Inventory of current pill-painted shapes and caption rendering

All pill icons are painted vector shapes (no font glyphs). The six keyboard-distinct buttons carry a 9 pt caption under the glyph; the four arrows are uncaptioned.

| # | Shape | File / lines | Current paint call | Current fill/stroke color source |
|---|-------|--------------|--------------------|----------------------------------|
| 1 | Left arrow triangle | `src/app/floating_controls.rs` : `ArrowButtonSpec::paint_triangle` (169–178) | `egui::Shape::rotated_triangle(triangle_rect, rotation, self.arrow_color(...))` | `arrow_color()` returns `visuals().widgets.{noninteractive,hovered,inactive}.fg_stroke.color` |
| 2 | Right arrow triangle | same as #1 | same as #1 | same as #1 |
| 3 | Up arrow triangle | same as #1 | same as #1 | same as #1 |
| 4 | Down arrow triangle | same as #1 | same as #1 | same as #1 |
| 5 | Zoom-in `[+]` horizontal bar | `src/app/zoom_button.rs` : `ZoomButtonSpec::crossed_bar_shapes` (134–157) | `egui::Shape::rect_filled(..., glyph_color)` | `zoom_glyph_color()` returns `visuals().widgets.{noninteractive,hovered,inactive}.fg_stroke.color` |
| 6 | Zoom-in `[+]` vertical bar | same as #5 | `egui::Shape::rect_filled(..., glyph_color)` | same as #5 |
| 7 | Zoom-out `[−]` bar | `src/app/zoom_button.rs` : `ZoomButtonSpec::single_bar_shapes` (159–174) | `egui::Shape::rect_filled(..., glyph_color)` | same as #5 |
| 8 | Restore-fit stroked square | `src/app/zoom_button.rs` : `ZoomButtonSpec::stroked_square_shapes` (176–195) | `egui::Shape::rect_stroke(..., egui::Stroke::new(FIT_MARKER_STROKE_WIDTH, glyph_color), egui::StrokeKind::Inside)` | `zoom_glyph_color()` (stroke color) |
| 9 | Fullscreen filled square | `src/app/fullscreen_button.rs` : `paint_fullscreen_square` (37–46) | `egui::Shape::rect_filled(..., fullscreen_square_color(...))` | `fullscreen_square_color()` returns `visuals().widgets.{hovered,inactive}.fg_stroke.color` |
| 10 | Fit-width axis line | `src/app/fit_button.rs` : `horizontal_axis_line_shape` (166–173) | `egui::Shape::rect_filled(..., glyph_color)` | `fit_glyph_color()` returns `visuals().widgets.{noninteractive,hovered,inactive}.fg_stroke.color` |
| 11 | Fit-height axis line | `src/app/fit_button.rs` : `vertical_axis_line_shape` (175–182) | `egui::Shape::rect_filled(..., glyph_color)` | same as #10 |
| 12 | Fit-width minus-facing triangle | `src/app/fit_button.rs` : `FitButtonAction::glyph_shapes` (152–161) | `egui::Shape::rotated_triangle(..., glyph_color)` | same as #10 |
| 13 | Fit-width plus-facing triangle | same as #12 | `egui::Shape::rotated_triangle(..., glyph_color)` | same as #10 |
| 14 | Fit-height minus-facing triangle | same as #12 | `egui::Shape::rotated_triangle(..., glyph_color)` | same as #10 |
| 15 | Fit-height plus-facing triangle | same as #12 | `egui::Shape::rotated_triangle(..., glyph_color)` | same as #10 |

**Caption rendering path today:** `src/app/button_hint.rs` : `show_button_column` (122–149). For the six captioned buttons it calls:

```rust
column_ui.label(
    egui::RichText::new(reference)
        .size(CAPTION_TEXT_SIZE)
        .color(column_ui.visuals().text_color()),
);
```

The caption uses the default label text style at 9 pt and the current visuals’ `text_color()` (theme-dependent). It is painted **after** the button glyph so it appears beneath the button in the column’s paint order, but it is the last element in the column.

---

## 2. Caption color

**Decision:** pin the enabled icon and caption color to one shared constant.

```rust
const PILL_GLYPH_COLOR: egui::Color32 = egui::Color32::WHITE;
```

- Both icons and captions use `PILL_GLYPH_COLOR` for the enabled (idle and hovered) state.
- This satisfies the operator constraint: captions are white and match the icons.
- Disabled state stays dimmed: keep deriving the disabled icon/caption color from `ui.visuals().widgets.noninteractive.fg_stroke.color` so the existing enablement/dimming behavior is unchanged.
- Hovered state: because `Color32::WHITE` is already the brightest value, use `PILL_GLYPH_COLOR` for hover as well. The hover cursor and tooltip already provide the interaction cue; no color brightening is possible above white.

All other caption properties stay byte-identical: `CAPTION_TEXT_SIZE = 9.0`, caption strings (`F11`, `I`, `O`, `0`, `W`, `H`), tooltip strings, layout geometry, and enablement gating.

---

## 3. Icon borders

**Decision:** every painted pill shape gains a 1-point black outline drawn outside the existing colored geometry, so the colored interior is unchanged.

### 3.1 Shared outline constants

```rust
const OUTLINE_COLOR: egui::Color32 = egui::Color32::BLACK;
const OUTLINE_STROKE_WIDTH: f32 = 1.0;
```

For `PathShape`-based shapes use:

```rust
egui::PathStroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR).outside()
```

For `RectShape`-based shapes use:

```rust
egui::Stroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR)
egui::StrokeKind::Outside
```

### 3.2 Per-shape border approach

| Shape | Current | New approach | Geometry preservation |
|-------|---------|--------------|-----------------------|
| 4 arrow triangles | `Shape::rotated_triangle` (filled, no stroke) | Replicate the same triangle geometry and use `Shape::convex_polygon(points, PILL_GLYPH_COLOR, outline_stroke.outside())` | Same rect and rotation; stroke is drawn outside |
| Zoom `[+]` bars | Two `Shape::rect_filled` | Replace each with `Shape::rect(..., PILL_GLYPH_COLOR, outline_stroke, StrokeKind::Outside)` | Fill rect unchanged; outline outside |
| Zoom `[−]` bar | One `Shape::rect_filled` | Replace with `Shape::rect(..., PILL_GLYPH_COLOR, outline_stroke, StrokeKind::Outside)` | Fill rect unchanged; outline outside |
| Restore-fit square | `Shape::rect_stroke` with glyph-colored stroke, `StrokeKind::Inside` | Draw a black outline first: `Shape::rect_stroke(enlarged_rect, ..., black_stroke, StrokeKind::Outside)`, then draw the original glyph-colored stroked square on top | The colored square stays the same size; black border sits just outside |
| Fullscreen square | `Shape::rect_filled` | Replace with `Shape::rect(..., PILL_GLYPH_COLOR, outline_stroke, StrokeKind::Outside)` | Fill rect unchanged; outline outside |
| Fit axis lines | `Shape::rect_filled` | Replace with `Shape::rect(..., PILL_GLYPH_COLOR, outline_stroke, StrokeKind::Outside)` | Fill rect unchanged; outline outside |
| 4 fit triangles | `Shape::rotated_triangle` | Same convex-polygon-with-outline approach as arrow triangles | Same rect and rotation; stroke outside |

### 3.3 Triangle geometry helper

`Shape::rotated_triangle` (egui 0.37 source) is equivalent to:

```rust
let rotation = egui::Rot2::from_angle(rotation);
let points = [rect.left_top(), rect.right_top(), rect.center_bottom()]
    .into_iter()
    .map(|point| rect.center() + rotation * (point - rect.center()))
    .collect();
egui::Shape::convex_polygon(points, fill, egui::Stroke::NONE)
```

The project helper must replicate this exactly, substituting the black outline stroke:

```rust
fn outlined_triangle_shape(
    rect: egui::Rect,
    rotation: f32,
    fill: egui::Color32,
) -> egui::Shape {
    let rotation = egui::Rot2::from_angle(rotation);
    let points = [rect.left_top(), rect.right_top(), rect.center_bottom()]
        .into_iter()
        .map(|point| rect.center() + rotation * (point - rect.center()))
        .collect();
    egui::Shape::convex_polygon(
        points,
        fill,
        egui::PathStroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR).outside(),
    )
}
```

This helper lives in the shared `pill_style` module (§5) and is used by both `floating_controls.rs` and `fit_button.rs`.

### 3.4 Rect outline helper

```rust
fn outlined_rect_shape(rect: egui::Rect, fill: egui::Color32) -> egui::Shape {
    egui::Shape::rect(
        rect,
        egui::CornerRadius::ZERO,
        fill,
        egui::Stroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR),
        egui::StrokeKind::Outside,
    )
}
```

Used for the zoom bars, fit axis lines, and fullscreen square.

### 3.5 Restore-fit square outline

Because the restore-fit icon is already a **stroked** square (not filled), add a black outline stroke shape behind it:

```rust
let outline_rect = marker_rect.expand(OUTLINE_STROKE_WIDTH);
let outline = egui::Shape::rect_stroke(
    outline_rect,
    egui::CornerRadius::ZERO,
    egui::Stroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR),
    egui::StrokeKind::Outside,
);
let marker = egui::Shape::rect_stroke(
    marker_rect,
    egui::CornerRadius::ZERO,
    egui::Stroke::new(FIT_MARKER_STROKE_WIDTH, glyph_color),
    egui::StrokeKind::Inside,
);
vec![outline, marker]
```

---

## 4. Caption font border

### 4.1 egui 0.37 API research

Verified against the pinned `egui =0.37.0` / `epaint =0.37.0` docs:

- `egui::TextFormat` has `underline: Stroke` and `strikethrough: Stroke`, but **no glyph outline / stroke field**.
- `epaint::TextShape` has `underline: Stroke`, but **no glyph outline / stroke field**.
- `egui::Painter::text`, `Painter::layout`, `Painter::galley` do not accept a glyph outline parameter.

**Conclusion:** egui 0.37 provides **no idiomatic text glyph outline/border mechanism**. The fallback offset-copy outline technique is required.

### 4.2 Offset-copy outline technique

Paint the caption in two passes:

1. **Outline pass:** paint the same text string in `OUTLINE_COLOR` at 8 surrounding offsets around the final text position.
2. **Fill pass:** paint the caption in `PILL_GLYPH_COLOR` (or the theme-derived disabled color) at the base position.

**Offsets (1 point, giving a 1 px-class outline at default DPI):**

```rust
const CAPTION_OUTLINE_OFFSETS: [egui::Vec2; 8] = [
    egui::vec2(-1.0, -1.0),
    egui::vec2( 0.0, -1.0),
    egui::vec2( 1.0, -1.0),
    egui::vec2(-1.0,  0.0),
    egui::vec2( 1.0,  0.0),
    egui::vec2(-1.0,  1.0),
    egui::vec2( 0.0,  1.0),
    egui::vec2( 1.0,  1.0),
];
```

**Number of copies:** 8 outline copies + 1 fill copy = 9 text paints per caption.

### 4.3 Caption helper implementation

Replace the `column_ui.label(...)` call in `button_hint.rs::show_button_column` with a helper that:

1. Lays out the caption once using `column_ui.painter().layout_no_wrap(...)` to obtain an `Arc<Galley>` and its size.
2. Allocates vertical space in the column equal to the galley height so the column cursor advances identically to a label.
3. Computes the center-top anchor position inside the allocated space.
4. Paints the 8 black outline copies at `anchor + offset` using `column_ui.painter().galley(...)`.
5. Paints the white fill copy at `anchor` using `column_ui.painter().galley(...)`.

Sketch:

```rust
fn paint_outlined_caption(
    ui: &mut egui::Ui,
    text: &str,
    fill_color: egui::Color32,
) {
    let font_id = egui::FontId::proportional(CAPTION_TEXT_SIZE);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, fill_color);
    let size = galley.size();
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let anchor = egui::Align2::CENTER_TOP.pos_in_rect(&rect, size);
    for offset in CAPTION_OUTLINE_OFFSETS {
        ui.painter().galley(anchor + offset, galley.clone(), OUTLINE_COLOR);
    }
    ui.painter().galley(anchor, galley, fill_color);
}
```

**Color passed to the helper:**
- Enabled buttons: `PILL_GLYPH_COLOR` (`Color32::WHITE`).
- Disabled buttons: `ui.visuals().widgets.noninteractive.fg_stroke.color` (preserves dimming).

The helper belongs in the shared `pill_style` module.

---

## 5. Shared helper module structure

Create a single shared styling module to avoid duplicating constants and geometry helpers across the five paint modules.

### 5.1 New file: `src/app/pill_style.rs`

Responsibilities:
- Hold all pill-styling constants (`PILL_GLYPH_COLOR`, `OUTLINE_COLOR`, `OUTLINE_STROKE_WIDTH`, caption offsets, etc.).
- Provide `glyph_color_for(enabled, hovered, ui)` so every button module resolves the same idle/hover/disabled palette.
- Provide `outlined_triangle_shape(rect, rotation, fill)`.
- Provide `outlined_rect_shape(rect, fill)`.
- Provide `paint_outlined_caption(ui, text, fill_color)`.
- Include inline unit tests for the constants and color resolution.

Keep the module under the 200-line file cap; if it grows larger, split tests into `src/app/pill_style_tests.rs`.

### 5.2 Module wiring

Add to `src/app.rs` module block (with the other `mod` declarations):

```rust
mod pill_style;
```

All exports from `pill_style.rs` are `pub(super)` so only `app` and its children can see them.

### 5.3 Consumer changes

| File | Change |
|------|--------|
| `src/app/floating_controls.rs` | Import `pill_style`; use `glyph_color_for` and `outlined_triangle_shape` in `paint_triangle`; remove local `arrow_color` helper if it becomes redundant. |
| `src/app/zoom_button.rs` | Import `pill_style`; use `glyph_color_for`, `outlined_rect_shape`; update `crossed_bar_shapes`, `single_bar_shapes`, and `stroked_square_shapes` to include the black outline. |
| `src/app/fullscreen_button.rs` | Import `pill_style`; use `glyph_color_for` and `outlined_rect_shape`; remove local `fullscreen_square_color` helper if it becomes redundant. |
| `src/app/fit_button.rs` | Import `pill_style`; use `glyph_color_for`, `outlined_rect_shape`, `outlined_triangle_shape`; update axis-line and triangle paint. |
| `src/app/button_hint.rs` | Import `pill_style`; replace `column_ui.label(...)` with `paint_outlined_caption`; pass the correct enabled/disabled fill color. |
| `src/app.rs` | Add `mod pill_style;`. |

### 5.4 Testability

Add pinned unit tests in `pill_style.rs` (inline) covering:
- `PILL_GLYPH_COLOR == Color32::WHITE`.
- `OUTLINE_COLOR == Color32::BLACK`.
- `OUTLINE_STROKE_WIDTH == 1.0`.
- `CAPTION_OUTLINE_OFFSETS` contains exactly the 8 expected neighbor offsets.
- A synthetic `glyph_color_for(true, false, &ui)` returns white; `glyph_color_for(false, _, &ui)` returns the noninteractive color.

Expected test-census delta: **+2 to +3 tests**.

---

## 6. Impact list

### Files to touch

1. `src/app.rs` — add `mod pill_style;`.
2. `src/app/pill_style.rs` — new shared styling module (constants + helpers + inline tests).
3. `src/app/floating_controls.rs` — arrow triangle color/outline.
4. `src/app/zoom_button.rs` — zoom `[+]` / `[−]` / restore-fit outline.
5. `src/app/fullscreen_button.rs` — fullscreen square outline.
6. `src/app/fit_button.rs` — fit axis lines and triangles outline.
7. `src/app/button_hint.rs` — caption white color + font outline.

### Files explicitly NOT touched

- `src/app/central_surface.rs` — central image painting is unrelated to pill styling.
- `src/app/keyboard_intake.rs` — no key behavior change.
- `src/app/fullscreen.rs`, `src/app/zoom.rs`, `src/app/folder_navigation.rs` — logic unchanged.
- `src/app_display.rs` — image display unchanged.

### Expected test-census delta

- Current: 104 unit tests.
- Add 2–3 tests in `pill_style.rs` for constants and color resolution.
- No tests removed.
- New expected range: **106–107 unit tests**.

---

## 7. 4.5a conformance checklist

During verification (Step 4.5a), confirm each item against this spec and the unchanged 0.4.1 behaviors:

- [ ] **Window / launch / fullscreen behavior unchanged:** no changes to shell, fullscreen, zoom, navigation, or image display.
- [ ] **Caption strings unchanged:** the six captions still read `F11`, `I`, `O`, `0`, `W`, `H`; the four arrows still have no caption.
- [ ] **Caption size unchanged:** `CAPTION_TEXT_SIZE` remains `9.0`.
- [ ] **Caption color white:** enabled captions render in `Color32::WHITE`; disabled captions still dim via the theme noninteractive color.
- [ ] **Caption font border:** each caption has a 1-point black outline via the 8-offset copy technique; no caption text appears without the outline.
- [ ] **Tooltip strings unchanged:** all 10 buttons still show the exact 0.4.1 tooltips.
- [ ] **Tooltip behavior unchanged:** tooltips appear on both enabled and disabled buttons.
- [ ] **Button enablement/dimming unchanged:** arrows, zoom, fit, fullscreen buttons enable/disable exactly as in 0.4.1.
- [ ] **Layout geometry unchanged:** button sizes, group gaps, pill margins, bottom offset, message line position unchanged.
- [ ] **All painted icons outlined:** every shape in the inventory (4 arrows, `[+]` bars, `[−]` bar, fullscreen filled square, restore-fit stroked square, fit-width/fit-height line-plus-triangle glyphs) has a visible 1-point black border.
- [ ] **Outline color/width consistent:** all outlines use `OUTLINE_COLOR = BLACK` and `OUTLINE_STROKE_WIDTH = 1.0`.
- [ ] **Colored geometry preserved:** filled shapes keep the same interior dimensions; borders are drawn outside (`StrokeKind::Outside` / `PathStroke::outside()`).
- [ ] **No per-frame allocation churn:** shapes are rebuilt each frame as today; no new persistent state or per-frame texture uploads introduced.
- [ ] **File-size / line-count rules respected:** no source file exceeds 200 lines; helpers are private-by-default; no new public API surface.
- [ ] **Tests pass:** container dev-checks `ALL CHECKS PASSED`; test census matches the expected +2/+3 delta.
- [ ] **No regression in OPERATOR-RUN checklist items:** all 0.4.1 behaviors other than the two styling changes remain identical.

---

## 8. Notes for the implementer

- Do **not** change the pill layout, button sizes, group gaps, or message line.
- Do **not** introduce new state fields on `ImagesScrollerApp`; this task is pure paint styling.
- The junior implementer must follow the exact constants and helpers named here; minor internal variable names inside helpers are the only allowable local latitude.
- If `pill_style.rs` exceeds 200 lines after adding tests, split the tests into `src/app/pill_style_tests.rs` and declare it in `app.rs` under `#[cfg(test)]`.
