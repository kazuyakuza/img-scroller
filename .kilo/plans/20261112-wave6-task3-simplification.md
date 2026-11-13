# Wave 6 Task 3 — Simplification Plan (4.3 simplifier)

**Date:** 2026-11-13
**Branch:** `feat/wave-6-open-with` (HEAD `c00d427`; NO branch/commit actions here)
**Scope:** TODO Task 3 landed sources only. Simplification of INTERNAL structure within the approved boundaries of `.kilo/plans/20261112-wave6-task3-open-with-menu-implementation-plan.md` §0 (A1–A8) and §2 (D1–D12). Behavior, strings, names and tests are unchanged.
**Implementer:** JUNIOR developer under 50% restriction — every step below is atomic and fully specified; no judgment calls.

## Verdict

Two micro-simplifications warranted, BOTH confined to `src/app/open_with_button.rs` (197 → ~193 lines). Not structural — no new TODO file needed. All other files reviewed and left unchanged (see "Verified — no change").

## Item 1 — Remove the redundant `disabled_tooltip` field from `OpenWithButtonSpec`

The per-frame spec struct stores `disabled_tooltip`, but it is derivable from app state and `app` is already in scope at the single use site. Sibling per-frame specs (`ArrowButtonSpec`, `ZoomButtonSpec`, `FitButtonSpec`) store only `enabled` — this restores the pattern. The tooltip VALUE logic (`no_programs_tooltip_for`) is untouched — the approved D3/D8 tooltip scheme and the four state gates keep identical behavior.

In `src/app/open_with_button.rs`:

1. Replace the struct doc + definition
   ```rust
   /// Per-frame button request: painted enablement plus the disabled-tooltip override.
   struct OpenWithButtonSpec {
       enabled: bool,
       disabled_tooltip: Option<&'static str>,
   }
   ```
   with
   ```rust
   /// Per-frame button request: the painted enablement. Buttons store no state between frames.
   struct OpenWithButtonSpec {
       enabled: bool,
   }
   ```

2. Replace `for_app`
   ```rust
       fn for_app(app: &ImagesScrollerApp) -> Self {
           Self {
               enabled: app.open_with_menu_is_available(),
               disabled_tooltip: no_programs_tooltip_for(app),
           }
       }
   ```
   with
   ```rust
       fn for_app(app: &ImagesScrollerApp) -> Self {
           Self {
               enabled: app.open_with_menu_is_available(),
           }
       }
   ```

3. In `show_and_open_menu`, replace the line
   ```rust
                   .with_disabled_tooltip(self.disabled_tooltip);
   ```
   with
   ```rust
                   .with_disabled_tooltip(no_programs_tooltip_for(app));
   ```
   (the `placement` chain keeps `for_button(...).with_enabled(self.enabled).with_disabled_tooltip(...)`; `app` is borrowed immutably here — no conflict with the later mutable use.)

## Item 2 — Remove the unnecessary borrow-scoping block in `show_menu_entry_buttons`

Under NLL the immutable borrow in `scroll_ui.button(...)` already ends at the end of the statement, so the `{ }` block adds no borrow-safety (the plan note's intent — no clones, no allocation churn — is preserved: still zero clones). The index loop `0..len()` stays REQUIRED (an `iter().enumerate()` would hold the list borrow across the mutable `launch_open_with_program` call and fail to compile — do NOT convert it).

In `src/app/open_with_button.rs`, replace the body
```rust
        for program_index in 0..self.open_with_programs.len() {
            let entry_response = {
                let display_name = self.open_with_programs[program_index].display_name.as_str();
                scroll_ui.button(display_name)
            };
            if entry_response.clicked() {
                self.launch_open_with_program(program_index);
            }
        }
```
with
```rust
        for program_index in 0..self.open_with_programs.len() {
            let entry_response =
                scroll_ui.button(self.open_with_programs[program_index].display_name.as_str());
            if entry_response.clicked() {
                self.launch_open_with_program(program_index);
            }
        }
```

## Verification expectations (for the following harness step)

- `cargo fmt` in the container first (canonical line breaks after each replacement above).
- dev-checks `ALL CHECKS PASSED` with **177 passed; 0 failed**, 28 test files — census and every test unchanged.
- If `rustc` rejects Item 2 (unexpected borrow error), revert ONLY Item 2, keep Item 1, and report — do not improvise alternatives.

## Verified — no change (within reviewed boundaries)

- `src/app.rs` — exactly at the 200-line cap; this plan adds NOTHING there (no compensating removal needed).
- `src/app/button_hint.rs` (184), `src/app/floating_controls.rs` (186), `src/app/folder_navigation.rs` (172), `src/platform/mod.rs` (35) — the state-aware tooltip selection in `show_button_column` and the `PillButtonPlacement.disabled_tooltip` scheme are plan-pinned (D8); the sibling pattern duplication (`allocate_and_paint`/`accepts_click`/`paint_glyph`, `36.0×32.0` sizes per module) is the established cross-module pattern and any cross-module extraction is OUT of Task-3 scope.
- `src/app/open_with_button_tests.rs` + `src/app/button_hint_tests.rs` — pinned test semantics; untouched.
- `"Open with..."` appearing both as `OPEN_WITH_BUTTON_LABEL` (accessible name) and the `tooltip_text(OpenWith)` table entry is by-design: the button-module label constant + the hint-table tooltip are separate pinned sources (matches `ZOOM_*`/`FIT_*` label pattern); unifying them would invert the `button_hint.rs` single-source principle — REJECTED.
- Per-frame `Vec<Shape>` in `open_with_glyph_shapes` — plan-pinned skeleton (D7), matches `zoom_button.rs`/`fit_button.rs`; not touched.
- Rule sanity: every new-module method body ≤ 50 lines (max ~19), nesting ≤ 2, ≤ 2 non-receiver params, visibility minimal (`pub(super)` only for the pinned cross-module/test consumers), no commented-out code, named constants throughout.
