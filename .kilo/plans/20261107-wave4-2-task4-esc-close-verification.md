# Wave 4.2 Task 4 — ESC Double-Press Close Front-end Verification Report

**Verification step:** A-Task 4, Step 4.5a (Front-end Implementation Verification)  
**Spec:** `.kilo/plans/20261107-wave4-2-task4-esc-close-frontend-spec.md`  
**Implementation plan:** `.kilo/plans/20261107-wave4-2-task4-esc-close-implementation-plan.md`  
**Review range:** `5edea1c..7e235fa` (feature commits) + `7e235fa..f72d076` (API-path corrections)  
**HEAD at review:** `f72d076`  
**Method:** Static code review against the spec checklist (read/grep/git-diff). Runtime evidence is the already-green dev-checks output (127 tests passing, `ALL CHECKS PASSED`).

---

## 1. Implementation delta summary

| File | Change |
| --- | --- |
| `src/app.rs` | Added `esc_close_popup_expires_at: Option<f64>` field, `None` init, `mod esc_close_confirm`, `#[cfg(test)] mod esc_close_confirm_tests`, and the `show_esc_close_popup` render call inside `show_chrome_layout`. |
| `src/app/esc_close_confirm.rs` | New module: constants, `EscCloseAction` decision enum, `non_repeat_escape_press_occurred` filter, state helpers (arm/disarm/update + pure `_at` cores), and the centered non-capturing render. |
| `src/app/keyboard_intake.rs` | Added non-repeat ESC scan, `update_esc_close_popup` before dispatch, and `dispatch_esc_close_action` helper with `ViewportCommand::Close` arm. |
| `src/app/fullscreen.rs` | `toggle_fullscreen_and_send_command` now calls `disarm_esc_close_popup()` after the flag flip. |
| `src/app/esc_close_confirm_tests.rs` | 11 new tests covering state machine, input filter, decision matrix, fullscreen interplay, expiry/re-arm, and arming in every fullscreen-OFF state. |
| `src/app/pill_style.rs`, `src/app/pill_style_tests.rs`, `src/app/zoom_alias.rs`, `src/app/zoom_alias_tests.rs`, `src/app/zoom_tests.rs` | API-path/formatting corrections only (see §3). |

---

## 2. Spec §8 conformance checklist

| # | Requirement | Status | Evidence (file:line) |
| --- | --- | --- | --- |
| 1 | Window options unchanged (`Images Scroller`, resizable, maximized) | **PASS** | `src/app.rs:147-155` — `with_title(APP_WINDOW_TITLE)`, `with_resizable(true)`, `with_maximized(true)`; no popup/modal viewport options added. |
| 2 | Message literal canonical | **PASS** | `src/app/esc_close_confirm.rs:21` — `pub(super) const POPUP_MESSAGE_TEXT: &str = "Press ESC again to close";`. Grep of `src/` finds exactly this one occurrence. |
| 3 | Popup centered | **PASS** | `src/app/esc_close_confirm.rs:130` — `.anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)`. |
| 4 | Popup non-capturing | **PASS** | `src/app/esc_close_confirm.rs:132` — `.interactable(false)`. |
| 5 | Popup layers above chrome | **PASS** | `src/app/esc_close_confirm.rs:131` — `.order(egui::Order::Foreground)`. |
| 6 | Two-second lifetime, `>=` comparison | **PASS** | `src/app/esc_close_confirm.rs:24` — `POPUP_VISIBLE_SECONDS: f64 = 2.0`; line 111 — `if now >= deadline`. |
| 7 | Frame-rate-independent timing from `InputState::time` | **PASS** | `src/app/esc_close_confirm.rs:118` — `screen_context.input(|input| input.time)`; no `stable_dt` accumulation. |
| 8 | Two distinct presses via `Event::Key { repeat: false }` | **PASS** | `src/app/esc_close_confirm.rs:64-76` — `non_repeat_escape_press_occurred` scans `input.events` for `Escape` with `pressed: true, repeat: false`. |
| 9 | Held ESC does not arm or close | **PASS** | Same filter ignores `repeat: true` events; `repeat_escape_press_is_ignored` test covers it. |
| 10 | Fullscreen ON consumes ESC | **PASS** | `src/app/keyboard_intake.rs:25-28, 44-45` — `escape_exit_fullscreen_pressed` predicate drives `toggle_fullscreen_and_send_command`; the popup branch is in the `else` block. |
| 11 | Fullscreen toggle disarms popup | **PASS** | `src/app/fullscreen.rs:31` — `self.disarm_esc_close_popup()` immediately after `toggle_fullscreen()`. |
| 12 | Close command is `egui::ViewportCommand::Close` | **PASS** | `src/app/keyboard_intake.rs:90` — single `send_viewport_cmd(egui::ViewportCommand::Close)` arm. |
| 13 | Popup visible in all fullscreen-OFF states | **PASS** | `src/app/app.rs:136` — `esc_close_confirm::show_esc_close_popup(self, ui.ctx())` is the last statement of `show_chrome_layout`, which runs whenever `full_ui_chrome_is_visible()`. Not gated by `navigation_context_is_active()`. |
| 14 | No ID collision | **PASS** | `src/app/esc_close_confirm.rs:28` — salt `"esc_close_confirm_popup"`; existing salts are `"floating_navigation_pill"` (`src/app/floating_controls.rs:21`) and `"empty_state_open_button"` (`src/app/empty_state_view.rs:43`). |
| 15 | State field initialized `None` | **PASS** | `src/app.rs:63` — `esc_close_popup_expires_at: None` in `empty_state()`. |
| 16 | No per-frame allocation churn | **PASS** | `src/app/keyboard_intake.rs:29-30` scans `input.events` inside the closure (no `.clone()`); `show_esc_close_popup` returns early when not armed. |
| 17 | Tests added | **PASS** | `src/app/esc_close_confirm_tests.rs` exists with 11 `#[test]` functions; total `#[test]` census across `src/` is 127, matching the green dev-checks claim. |

---

## 3. API-path corrections do not alter spec behavior

The `7e235fa..f72d076` diff contains only mechanical egui 0.37 API/formatter changes:

- `egui::Rot2` → `egui::emath::Rot2` and `egui::PathStroke` → `egui::epaint::PathStroke` in `src/app/pill_style.rs`.
- `Context::run` → `Context::run_pass` + `drop_without_applying_deltas()` in test harnesses (`esc_close_confirm_tests.rs`, `pill_style_tests.rs`, `zoom_alias_tests.rs`).
- `egui::Id::new(...)` → `egui::Id::unique(...)` in `pill_style_tests.rs`.
- Minor rustfmt wrapping in `keyboard_intake.rs`, test files, and `zoom_alias.rs` (`typed_text.starts_with(alias_character)`).

None of these touch the ESC-close production logic, constants, timing, input filter, or render. The only spec-related file changed is `keyboard_intake.rs`, and the change is a line-wrap of the non-repeat ESC scan (`src/app/keyboard_intake.rs:29-30`). Therefore the API corrections do not alter any Task-4 spec behavior.

---

## 4. Front-end quality observations (operator-run manual gate)

These are subjective/operational checks for the A-Task 5 manual pass; they are not agent-failable from static review:

1. **Popup legibility over varied image backgrounds** — The fill is `Color32::from_rgba_unmultiplied_const(20, 20, 20, 220)` with white 14 pt text. This is slightly more opaque than the pill (`20,20,20,180`), but over very dark images the contrast may still be marginal. Operator should verify readability against a near-black image.
2. **Popup vs pill layering** — `Order::Foreground` and `interactable(false)` place the popup above the pill without stealing input; static review confirms this. Operator should confirm visually that the pill remains clickable underneath.
3. **Two-second feel** — The deadline is exactly 2.0 s from `InputState::time`; frame-rate independent. Operator should verify it feels like ~2 s on both 60 Hz and high-refresh displays.
4. **Centering on maximized window** — The anchor is viewport-centered, so it should remain centered after the initial maximized launch and after any manual resize. Operator should check centering on a maximized 1080p/1440p monitor.

---

## 5. Nonconformances and fix steps

**None identified.**

All 17 spec checklist items pass. The implementation matches the front-end technical specification, and the API-path corrections are mechanical only.

If operator-run feedback later reveals any of the quality observations in §4 to be problematic, the fix steps would be localized to `src/app/esc_close_confirm.rs`:
- Legibility issue → increase fill opacity or add a subtle stroke/shadow to the popup frame.
- Layering issue → confirm `Order::Foreground` is still highest; if needed, raise to `Order::Tooltip` (but this is unlikely).
- Timing feel issue → adjust `POPUP_VISIBLE_SECONDS` (requires operator sign-off).
- Off-center issue → verify `Align2::CENTER_CENTER` and that no parent layout shifts the Area; the current code has no such parent layout influence.

---

## 6. Conclusion

The implementation at `f72d076` conforms to `.kilo/plans/20261107-wave4-2-task4-esc-close-frontend-spec.md`. No src edits, commits, or pushes were made by this verification step.
