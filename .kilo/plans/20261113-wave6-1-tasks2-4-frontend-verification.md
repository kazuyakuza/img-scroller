# Wave 6.1 Tasks 2+3+4 — Front-end Implementation Verification Report

## Verdict

PASS-WITH-ADJUSTMENTS

## Scope

- Source TODO: `.agent/todos/20261113/20261113-todo-1.md` Tasks 2, 3, 4 + binding operator decisions.
- Spec: `.kilo/plans/20261113-wave6-1-tasks2-4-ui-feedback-frontend-spec.md`.
- Implementation plan (supersedes spec where deviations D1–D12 are recorded): `.kilo/plans/20261113-wave6-1-tasks2-4-ui-feedback-implementation-plan.md`.
- Landed code: branch `fix/wave-6-1-open-with-feedback`, HEAD `c7fad26`.
- Verification method: static code review only; no build/test/run executed.

## Classified diff summary

### CONFORMS

- `button_hint::hotkey_reference(PillButton::OpenWith)` returns `Some("Q")`; `tooltip_text(PillButton::OpenWith)` returns `"Open with... (Q)"`.
- The open-with button renders the 9-pt outlined `Q` caption under the glyph through the unchanged `show_button_column` / `pill_style::paint_outlined_caption` path (WHITE enabled / theme-dimmed disabled).
- `src/app/open_with_button.rs` defines `open_with_menu_id()` (deterministic `Id::unique("open_with_menu_popup")`), `OPEN_WITH_MENU_KEY = egui::Key::Q`, and `open_with_menu_is_open`.
- The `Popup::menu` builder attaches `.id(open_with_menu_id())`, uses default `CloseOnClick`, and has no `.close_behavior(...)` override.
- `src/app/keyboard_intake.rs` has the early-return modal guard `handle_modal_open_with_menu_keys`; `update_esc_close_popup` runs before the guard (D6).
- While the menu is open, only the Q and ESC non-repeat reads run; every other chord read/dispatch is skipped.
- ESC-while-open closes the menu only; `ViewportCommand::Close` is not sent and the ESC-close popup is not armed.
- Q-again closes the menu; non-repeat filtering prevents held-key flicker.
- The Q-open path checks `open_with_menu_should_open_on_q` (press + `open_with_menu_is_available` + `full_ui_chrome_is_visible`) and calls `open_with_menu_from_q_press` (refresh + `Popup::open_id`).
- The click path refreshes via the same `refresh_open_with_programs` and opens via the builder's own click toggle (D1).
- Pill-row click dispatches are gated on `!open_with_menu_is_open` in arrows, fullscreen, zoom, fit, and the open-with refresh path (D2); the menu's own entries still launch.
- The popup builder gate is `menu_popup_is_wanted = enabled || open_with_menu_is_open` (D8).
- `pill_style::WARNING_MESSAGE_COLOR = Color32::from_rgb(255, 165, 0)`; `message_text_color` applies it only for the open-with failure; unsupported-file and cleared states keep `visuals().text_color()`.
- Warning coloring is scoped to `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` only via the `message_line_is_warning` predicate (D3).
- No per-frame texture re-upload or allocation churn is introduced.
- `src/app.rs` byte-diff is empty; no new source or test files were created.
- All touched source files are ≤200 lines; function bodies are ≤50 lines; methods have ≤2 parameters; no commented-out code.

### CONFORMS-with-recorded-adjustment

- D1: The click path and the Q path do **not** share one combined refresh-and-open helper; behavior is identical.
- D3: No `message_text_is_warning: bool` field on `ImagesScrollerApp`; warning discrimination is the pure `message_line_is_warning(message)` predicate.
- D4: `open_with_menu_id()` is a function returning `Id::unique(...)` instead of the spec's `const`.
- D8: The popup builder gate is `enabled || open_with_menu_is_open` rather than just `enabled`.
- D9: `OPEN_WITH_MENU_KEY` lives in `open_with_button.rs` rather than `keyboard_intake.rs`.
- D10: The open-with glyph constants and `open_with_glyph_shapes` moved to `pill_style.rs`.
- D12: `show_message_line` keeps its 2-parameter signature; the warning branch is computed inside the function.
- Test census: 189 unit tests verified. The repository contains 19 test files (pre-existing count; no new test files were added). The plan/context reference to "28 test files" appears to be a documentation inconsistency; the implementation did not add files and met the 189-test target.

### DEVIATION REQUIRING FIX

None.

## Quality findings (non-blocking)

1. `src/app/open_with_button_tests.rs` is at the 200-line hard cap (200 total / ~163 net logical lines). This is within current rules but leaves no headroom; consider splitting the module if more tests are added in future waves.
2. Pre-existing single-section boolean observations in `src/app/open_with_button.rs` (not introduced this wave): `no_programs_tooltip_for` uses `app.image_is_open() && app.open_with_programs.is_empty()` inside an `if`. No regression; recorded as a non-blocking observation only.
3. Plan/context test-file count mismatch: the implementation plan and caller context state "28 test files", but the repository contains 19 `*_tests.rs` / `tests.rs` files both before and after this wave. The implementation preserved the pre-wave file count and met the 189-test target, so no code fix is required; only the plan/context count should be reconciled.

## OPERATOR-RUN items (agent-side impossible)

1. Visual: confirm the `Q` caption renders under the open-with glyph in the 9-pt white/outlined style and the tooltip reads `"Open with... (Q)"`.
2. Visual: confirm the warning-orange failure message is readable on the pill's translucent dark frame.
3. Press `Q` while an image is open and programs are available → the menu opens.
4. While the menu is open, press `F11`, `ESC`, `I`/`O`/`0`/`W`/`H`, and arrow keys → none of these fire; the menu stays open.
5. Press `ESC` while the menu is open → the menu closes and the `"Press ESC again to close"` popup does **not** arm.
6. Press `Q` while the menu is open → the menu closes.
7. Click outside the menu body → the menu closes.
8. Verify `Q` is inert in disabled states: fresh-empty state, empty-folder state, fullscreen-ON state, and no-programs state.
9. Select a program in the menu → the current file opens in that program (D1 fix coverage).
10. Induce a launch failure → the warning-orange message appears in the pill's message line.

## What was NOT done

- No `cargo` build/test/clippy/dev-checks executed (agent boundary; caller context already states `ALL CHECKS PASSED` at `c7fad26`).
- No Windows release build or DLL import audit (Task 5 / Cycle C, out of scope for this step).
- No code edits.
- No edits to existing `.kilo/plans/*` files.
- No execution of subsequent workflow steps (4.5b+).
- No live GUI/runtime execution (agent-side impossible).
