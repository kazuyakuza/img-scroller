# Wave 4 Task 3 — Fit Width / Fit Height Modes: Review Fix Plan

Date: 2026-11-02
Source review: Critical Workflow step 4.3 for commit `f6facbb` on branch `feat/wave-4-fullscreen-zoom`
Scope: `src/app/fit_mode_button.rs` only — no behavioral change; only rule-compliance cleanup.

---

## Issue summary

`src/app/fit_mode_button.rs` (new in `f6facbb`) contains two rule violations that a tiny local refactor can resolve:

1. **Max-arguments-per-method rule** — two free helpers take 3 non-`self` arguments:
   - `axis_line_shape(glyph_center, glyph_color, horizontal)`
   - `fit_arrow_triangle_shape(arrow_center, arrow_facing, glyph_color)`
2. **Single-section-boolean-conditions rule** — `accepts_click` returns a two-section expression:
   - `self.enabled && button_response.clicked()`

The rest of the Task-3 implementation is clean and matches the approved plan. Only this file needs a follow-up edit.

---

## Exact fix steps (implement in `src/app/fit_mode_button.rs`)

### Step 1 — Split the axis-line helper into two 2-argument helpers

Replace `axis_line_shape(glyph_center, glyph_color, horizontal)` with:

```rust
fn horizontal_axis_line_shape(
    glyph_center: egui::Pos2,
    glyph_color: egui::Color32,
) -> egui::Shape {
    let line_size = egui::Vec2::new(FIT_AXIS_LINE_LENGTH, FIT_AXIS_LINE_THICKNESS);
    egui::Shape::rect_filled(
        egui::Rect::from_center_size(glyph_center, line_size),
        egui::CornerRadius::ZERO,
        glyph_color,
    )
}

fn vertical_axis_line_shape(
    glyph_center: egui::Pos2,
    glyph_color: egui::Color32,
) -> egui::Shape {
    let line_size = egui::Vec2::new(FIT_AXIS_LINE_THICKNESS, FIT_AXIS_LINE_LENGTH);
    egui::Shape::rect_filled(
        egui::Rect::from_center_size(glyph_center, line_size),
        egui::CornerRadius::ZERO,
        glyph_color,
    )
}
```

Update call sites:
- In `fit_width_glyph_shapes`: `axis_line_shape(glyph_center, glyph_color, true)` → `horizontal_axis_line_shape(glyph_center, glyph_color)`
- In `fit_height_glyph_shapes`: `axis_line_shape(glyph_center, glyph_color, false)` → `vertical_axis_line_shape(glyph_center, glyph_color)`

Delete the old `axis_line_shape` function.

### Step 2 — Inline the arrow-triangle helper to remove its 3-argument signature

Remove `fit_arrow_triangle_shape(...)` and inline the one-line `Shape::rotated_triangle` construction at the four call sites:

- `fit_arrow_triangle_shape(glyph_center - egui::vec2(FIT_ARROW_SPACING, 0.0), ArrowDirection::Left, glyph_color)`
  →
  ```rust
  egui::Shape::rotated_triangle(
      egui::Rect::from_center_size(glyph_center - egui::vec2(FIT_ARROW_SPACING, 0.0), FIT_ARROW_TRIANGLE_SIZE),
      ArrowDirection::Left.paint_rotation(),
      glyph_color,
  )
  ```
- Repeat for `ArrowDirection::Right`, `ArrowDirection::Up`, and `ArrowDirection::Down` with the matching centers from the existing calls.

This duplicates the `Rect::from_center_size` + `FIT_ARROW_TRIANGLE_SIZE` line four times but eliminates the >2-argument function.

### Step 3 — Extract the two-section click condition into a named helper

Change `accepts_click` from:

```rust
fn accepts_click(&self, button_response: &egui::Response) -> bool {
    self.enabled && button_response.clicked()
}
```

to:

```rust
fn accepts_click(&self, button_response: &egui::Response) -> bool {
    self.enabled_click_would_fire(button_response)
}

fn enabled_click_would_fire(&self, button_response: &egui::Response) -> bool {
    self.enabled && button_response.clicked()
}
```

The `if self.accepts_click(...)` call site remains a single-section condition.

---

## Verification

1. Run formatting:
   ```
   docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt
   ```
2. Run the dev-checks harness:
   ```
   docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
   ```
   Expected: `ALL CHECKS PASSED`, 93 tests, clippy `-D warnings` green.

---

## Out-of-scope guard

- Do **not** refactor `src/app/zoom_button.rs`; its identical `accepts_click` idiom is pre-existing and outside this task's commit.
- Do **not** change behavior, constants, glyph geometry, button enablement, or test assertions.
- Do **not** edit existing plans or `CHANGELOG.md`.
