# Cycle A → Step 4.3 Code Review Fix Plan

**Repository:** `C:\repo\img-scroller`  
**Branch:** `fix/centered-floating-controls`  
**Implementation commit reviewed:** `6ef0240`  
**Source plans:** `.kilo/plans/20261101-floating-controls.md` + `.kilo/plans/20261101-floating-controls-frontend-spec.md`  
**Review scope:** `src/app.rs`, `src/app/arrow_direction.rs`, `src/app/floating_controls.rs`, `src/app/floating_controls_tests.rs`; deleted `src/app/rails.rs` + `src/app/rails_tests.rs`.

## Findings summary

All plan-conformity items verified successfully **except two compile-time API mismatches** against egui 0.37.0. Both are local, decision-preserving fixes that change only the call sites/initializer noted below. No scope expansion, no behavioral changes, no architectural decisions.

## MUST-FIX 1 — `Area::show` receiver / second argument

**Problem:** The implementation passes `&mut egui::Ui` as the second argument to `egui::Area::show`, matching the incorrect example in the architector plan (§5.1, §5.3). The published egui 0.37.0 signature is:

```rust
pub fn show<R>(
    self,
    ctx: &Context,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R>
```

**Source:** docs.rs egui 0.37.0 `Area` — `show` takes `&Context`, not `&mut Ui`.

**Affected locations (exact):**

1. `src/app/floating_controls.rs:40`
2. `src/app.rs:136`

**Exact remediation:**

In `src/app/floating_controls.rs`, change:

```rust
        .show(host_ui, |area_ui| {
```

to:

```rust
        .show(host_ui.ctx(), |area_ui| {
```

In `src/app.rs`, change:

```rust
            .show(central_ui, |area_ui| {
```

to:

```rust
            .show(central_ui.ctx(), |area_ui| {
```

`Ui::ctx()` returns `&Context` (docs.rs `Ui::ctx`), which is the exact type `Area::show` requires.

## MUST-FIX 2 — `Color32::from_rgba_unmultiplied` is not `const` in 0.37.0

**Problem:** `floating_controls.rs` initializes a `const` with `Color32::from_rgba_unmultiplied(...)`. In egui 0.37.0 this function is **not** `const fn`; the const variant is `Color32::from_rgba_unmultiplied_const(...)`.

**Source:** docs.rs egui 0.37.0 `Color32` —

```rust
pub fn from_rgba_unmultiplied(r: u8, g: u8, b: u8, a: u8) -> Color32
pub const fn from_rgba_unmultiplied_const(r: u8, g: u8, b: u8, a: u8) -> Color32
```

**Affected location:** `src/app/floating_controls.rs:13-14`

**Exact remediation:**

Change:

```rust
const FLOATING_PILL_FILL_COLOR: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied(20, 20, 20, 180);
```

to:

```rust
const FLOATING_PILL_FILL_COLOR: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied_const(20, 20, 20, 180);
```

## Verified plan-conformity highlights (no fixes needed)

- Pill `Area` id/anchor/order/frame and `[←][→] gap [↑][↓]` order match the plan.
- `Shape::rotated_triangle(rect, rotation, fill)` usage matches docs.rs 0.37.0 signature and rotation table.
- `Frame::new().fill().corner_radius().inner_margin().show()` builders all exist in 0.37.0 with the names used.
- `Ui::allocate_exact_size` returns `(Rect, Response)`; `Sense::click()` includes hover and focusability.
- `Response::widget_info(|| WidgetInfo::labeled(Role::Button, enabled, label))` arity and semantics match the 0.37 accessibility example.
- `Response::on_hover_cursor(self, ...)` is correctly reassigned.
- `navigation_button_enabled` relocated verbatim; enablement test semantics unchanged.
- `app/tests.rs` untouched; test census = 26 (`#[test]` count across `src/`).
- No `open_toolbar`/`rails` remnants; single `Open…` label; no commented-out code.
- Line counts, method lengths, nesting depth, param counts, private-default/`pub(super)`, and single-section boolean rules satisfied.
- Key intake is frame-first and byte-identical; the new `Area`s do not capture arrow-key input (default `Sense::click()` Area is not a text/focus consumer of arrow keys).

## Container verification commands for the implementer

Run these in the repo root after applying the two MUST-FIX edits (expected: all pass):

```bash
cargo check
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Notes for the implementer

- Do **not** edit the existing architector plan files (`.kilo/plans/20261101-floating-controls.md` or the frontend spec). This review fix plan is the corrective artifact.
- The `Area::show` fix uses `host_ui.ctx()` / `central_ui.ctx()`. No other changes to closure captures or borrow structure are required; the existing mutable `app`/`self` captures remain valid.
- No new dependencies, no new files, no test changes, no TODO/docs updates.
