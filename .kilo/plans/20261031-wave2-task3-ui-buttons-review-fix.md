# Task 3 Code Review Fix Plan — Wave 2: On-Screen Navigation UI Buttons

Date: 2026-11-01
Review step: Wave-2 Task 3 → Step 4.3 (code-reviewer)
Source plan: `.kilo/plans/20261031-wave2-task3-ui-buttons.md`
Commit reviewed: `4b45ca7`

## Findings Summary

All K3 layout, enablement, privacy, and seam-reuse decisions are encoded correctly. The only MUST-FIX item is a single rustfmt overflow.

### Confirmed Correct

- Panel order in `src/app.rs` `ui()`: `Panel::top` → `image_is_open` rail block (`show_folder_rail`, then `show_image_rail`) → `CentralPanel` last. This gives the intended K3 geometry: top bar spans full width, right rail occupies the remaining right edge, bottom rail sits at the bottom starting from the left edge, and the central image fills the rest.
- Rails gate on `image_is_open()` (`opened_image_path.is_some()`); empty state shows only the top `Open…` panel.
- `message_bar` and `Panel::bottom("message_bar")` are gone from `src/`.
- `←`/`→` call `app.navigate(NavigationDirection::Previous/Next)` exactly; enablement uses `navigation_button_enabled` → `neighbor_image_path(...).is_some()` (STAY-edge parity with keys).
- `↑`/`↓` are `add_enabled(false, ...)` only and call no handler.
- `message_text` state is read but never mutated; `Some` renders a label, `None` renders an empty label (`unwrap_or("")`).
- All new items are `pub(super)` only where required; no `pub`/`pub(crate)` widened access.
- File/method/depth/param/single-section-boolean rules are satisfied.
- egui 0.37 API verification:
  - `Panel::right`/`Panel::bottom` + `.default_size(f32)` exist (source: `egui/containers/panel.rs`, lines 256–279, 372–379).
  - `Vec2::new` is `pub const fn` (source: `emath/vec2.rs`, line 148); `const ARROW_BUTTON_SIZE` is valid.
  - `Button::new`, `Button::min_size(Vec2)`, and `impl Widget for Button` exist (source: `egui/widgets/button.rs`, lines 27, 60, 217, 400–404).
  - Closure captures in `show_image_rail`/`image_rail_controls` follow the standard sequential egui pattern; enablement values are Copy and computed before the inner closure, so the mutable `app` borrow remains clean.
  - `Response` returned by `add_enabled` is not `must_use`; the `Button` is consumed by `add_enabled`, so its `#[must_use]` is satisfied.

### MUST-FIX

`src/app/rails.rs` line 19 exceeds the default rustfmt `max_width` of 100 columns (106 characters):

```rust
pub(super) fn navigation_button_enabled(app: &ImagesScrollerApp, direction: NavigationDirection) -> bool {
```

This will fail `cargo fmt --check`.

## Exact Edit

In `src/app/rails.rs`, replace the one-line signature with a wrapped signature:

```rust
pub(super) fn navigation_button_enabled(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> bool {
    app.neighbor_image_path(direction).is_some()
}
```

No other file changes are required.

## Verification Commands (run via `alpine-vm` MCP; call `vm_status` first)

After applying the edit:

```sh
cargo fmt --check
cargo check --locked
cargo test --locked
cargo clippy --locked -- -D warnings
```

Expected: all exit 0. The full dev-checks harness (`scripts/dev-checks.sh`) may be used in Task 4 for final evidence.
