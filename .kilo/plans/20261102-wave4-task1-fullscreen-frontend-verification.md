# Wave 4 Task 1 — Fullscreen Toggle: Front-end Implementation Verification

Date: 2026-11-02
Spec: `.kilo/plans/20261102-wave4-task1-fullscreen-frontend-spec.md`
Plan: `.kilo/plans/20261102-wave-4-task1-fullscreen-plan.md`
Branch: `feat/wave-4-fullscreen-zoom`
Commits reviewed: `c0e018e` (feature) + `9253d4a` (simplification) + `4486b81` (docs); final HEAD verified.

## Per-checklist verdicts

### 1. Fullscreen state model conforms — CONFORMS

- Exactly one new field: `src/app.rs:34` `fullscreen_on: bool`.
- Initialized to `false`: `src/app.rs:54`.
- No persistence logic added anywhere in `src/`.
- Launch options unchanged: `src/app.rs:161-169` keeps `with_maximized(false)` and does not add `with_fullscreen`, `with_decorations(false)`, forced size, or any launch-time maximization.

### 2. Mechanism conforms — CONFORMS

- Single shared action: `src/app/fullscreen.rs:17-21` `toggle_fullscreen_and_send_command` flips `fullscreen_on` and calls `send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen_after_toggle))` exactly once per trigger.
- F11 trigger uses the shared action: `src/app.rs:138-140`.
- Pill-button trigger uses the same shared action: `src/app/fullscreen_button.rs:24-26`.
- Static search confirms only one `send_viewport_cmd` call site: `src/app/fullscreen.rs:20`.

### 3. F11 intake — CONFORMS

- Edge-triggered via `key_pressed`: `src/app.rs:135-137`.
- Evaluated in every mode and BEFORE layout dispatch: `src/app.rs:134-157` runs the F11 check first, then arrow keys, then `show_chrome_or_fullscreen_layout`.
- No interplay with arrow-key navigation: arrow handling is independent and unchanged.

### 4. UI toggle control (fifth pill button) — CONFORMS

- Placement after the folder group with a gap: `src/app/floating_controls.rs:75-76` (`FULLSCREEN_GROUP_GAP = 16.0`).
- Always enabled: `src/app/fullscreen_button.rs:19-22` reports `WidgetInfo::labeled(egui::Role::Button, true, "Toggle fullscreen")`.
- Painted filled square, no font glyphs: `src/app/fullscreen_button.rs:29-37` uses `egui::Shape::rect_filled` with `egui::CornerRadius::ZERO`.
- Click behavior identical to F11: both call `app.toggle_fullscreen_and_send_command(...)`.

### 5. Hiding contract conformance — CONFORMS

- Single visibility gate: `src/app/fullscreen.rs:26-28` `full_ui_chrome_is_visible() -> !self.fullscreen_on`.
- Fullscreen ON (`src/app/fullscreen.rs:33-39`): renders only `CentralPanel`; paints the decoded image if present, otherwise leaves the surface blank. No pill, no centered Open button, no empty-folder message.
- Fullscreen OFF (`src/app.rs:124-130`): dispatches to `show_chrome_layout`, which restores the v0.3.0 paths: `show_navigation_layout` (`src/app.rs:101-110`) includes the empty-folder message when no image is decoded, and `show_empty_state_layout` (`src/app/empty_state_view.rs:13-21`) shows the centered Open button.

### 6. Unit-test surface — CONFORMS

- `src/app/fullscreen_tests.rs` contains exactly 7 tests:
  1. `fresh_app_starts_with_fullscreen_off`
  2. `fullscreen_shortcut_is_f11`
  3. `each_toggle_trigger_flips_the_flag_exactly_once`
  4. `fullscreen_on_hides_the_chrome_with_an_image_open`
  5. `fullscreen_off_restores_the_chrome_in_every_reachable_mode`
  6. `navigation_context_stays_active_across_fullscreen_toggles`
  7. `successful_open_during_fullscreen_keeps_fullscreen_on`
- All assert the subjects described in plan §7 / spec §10.

### 7. Diff-quality issues — CONFORMS

- No spec-vs-implementation mismatch detected.
- No dead/commented-out code left in the implementation files.
- No stale doc claims in final HEAD source docs.
- File line counts respect the project cap: `src/app.rs` 199 lines, `src/app/floating_controls.rs` 176 lines, new modules under 90 lines.
- One intentional plan-vs-code refinement: `9253d4a` dropped the unused return value of `toggle_fullscreen_and_send_command` and tightened `show_centered_open_button` visibility to private. Both changes align with the project's `prefer-private-members` and `no-commented-code` rules and do not affect behavior.

## Diffs vs. spec

none

## Front-end quality notes

- `toggle_fullscreen_and_send_command` now returns `()`; tests and callers that need the flag call `toggle_fullscreen()` directly. This removes an unused return without changing the single-command-send contract.
- `show_centered_open_button` is private because it is only invoked within `empty_state_view.rs`, satisfying the `prefer-private-members` rule.
- `egui::CentralPanel` is invoked in exactly one branch per frame (fullscreen OR chrome layout), so egui's single-central-panel contract is preserved.
- Texture upload discipline is preserved: `src/app_display.rs` is untouched; `CurrentImageDisplay::paint` is the only image-rendering path and there is no per-frame decode/upload logic.
- `app.rs` remains at 199 lines, within the 200-line file cap, after extracting `empty_state_view.rs`.

## Steps to fix

none

## Operator-pending items

The following can only be validated on a real Windows 10/11 machine running the glow-backed executable; no agent-side GUI verification was performed:

- Actual borderless-fullscreen behavior (native title bar and borders hidden, correct monitor, restore size and position on toggle OFF).
- F11 key behavior on laptops that require the Fn modifier.
- Minimize / Alt-Tab / task-switch restore behavior while fullscreen.
- Pill-button hit-testing and hover/click rendering in a live window.
