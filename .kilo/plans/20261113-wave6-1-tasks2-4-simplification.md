# Wave 6.1 Tasks 2+3+4 — Simplification Plan (4.3 simplifier)

**Date:** 2026-11-13
**Branch:** `fix/wave-6-1-open-with-feedback` (HEAD `3e74a2f`; NO branch/commit actions here)
**Scope:** TODO Tasks 2+3+4 landed sources only. Simplification of INTERNAL structure strictly inside the pinned boundaries: modal guard placement, Q/ESC routing semantics, the five D2 click gates, the warning-color predicate, caption/tooltip data, and every test's semantics are DECIDED and stay byte-identical in behavior. Census pinned 189 unit tests / 28 test files — this plan touches NO test file and adds/removes NO file.
**Implementer:** JUNIOR developer under 50% restriction — every step below is atomic and fully specified; no judgment calls.

## Verdict

Two micro-simplifications warranted, confined to `src/app/keyboard_intake.rs` (179 → 181 lines, still under the 200 cap) and `src/app/fit_button.rs` (158 → 155 lines). Both remove code duplication introduced/left in the new modal-guard path; the +2 net lines in `keyboard_intake.rs` buy the removal of a duplicated 6-line read (2 copies) and a duplicated close statement (2 copies). No new TODO file needed — not structural. All other touched files reviewed and left unchanged (see "Verified — no change").

## Item 1 — De-duplicate the Q non-repeat read and the modal close statement (`src/app/keyboard_intake.rs`)

The 6-line Q-press read block exists TWICE verbatim (lines 31–36 and 137–142) and `handle_modal_open_with_menu_keys` holds TWO identical `egui::Popup::close_id(screen_context, open_with_button::open_with_menu_id())` statements behind two ifs. Extract the shared read, fold the two close paths into ONE named single-section predicate (rule: single-section boolean conditions), emit ONE close statement. Semantics identical: the same non-consumed input reads, Q-closes and ESC-closes the same popup id, guard placement unchanged (line 28, right after `update_esc_close_popup`).

In `src/app/keyboard_intake.rs`:

1. In `read_and_dispatch_keystrokes`, replace the six lines
   ```rust
           let q_pressed = screen_context.input(|input| {
               esc_close_confirm::non_repeat_key_press_occurred(
                   &input.events,
                   open_with_button::OPEN_WITH_MENU_KEY,
               )
           });
   ```
   with
   ```rust
           let q_pressed = open_with_menu_key_pressed(screen_context);
   ```

2. In `handle_modal_open_with_menu_keys`, replace the fourteen lines (two reads + two identical if-close blocks)
   ```rust
           let q_pressed = screen_context.input(|input| {
               esc_close_confirm::non_repeat_key_press_occurred(
                   &input.events,
                   open_with_button::OPEN_WITH_MENU_KEY,
               )
           });
           let non_repeat_escape_pressed = screen_context
               .input(|input| esc_close_confirm::non_repeat_escape_press_occurred(&input.events));
           if q_pressed {
               egui::Popup::close_id(screen_context, open_with_button::open_with_menu_id());
           }
           if non_repeat_escape_pressed {
               egui::Popup::close_id(screen_context, open_with_button::open_with_menu_id());
           }
           true
   ```
   with
   ```rust
           if open_with_menu_close_pressed(screen_context) {
               egui::Popup::close_id(screen_context, open_with_button::open_with_menu_id());
           }
           true
   ```
   Do NOT change the method's `&mut self` signature, its guard `if !open_with_button::open_with_menu_is_open(screen_context) { return false; }`, or the final `true`.

3. Append at the end of the file (after the existing `was_pressed` helper) the two module-local helpers:
   ```rust
   /// The non-repeat read of the open-with menu hotkey (shared by the Q-open
   /// path and the modal close read).
   fn open_with_menu_key_pressed(screen_context: &egui::Context) -> bool {
       screen_context.input(|input| {
           esc_close_confirm::non_repeat_key_press_occurred(
               &input.events,
               open_with_button::OPEN_WITH_MENU_KEY,
           )
       })
   }

   /// The modal menu's close-key read: a non-repeat Q or a non-repeat ESC
   /// (binding decision: ONLY these two keys act while the menu is open).
   fn open_with_menu_close_pressed(screen_context: &egui::Context) -> bool {
       open_with_menu_key_pressed(screen_context)
           || screen_context.input(|input| {
               esc_close_confirm::non_repeat_escape_press_occurred(&input.events)
           })
   }
   ```

## Item 2 — Remove the single-use indirection in `FitButtonSpec::accepts_click` (`src/app/fit_button.rs`)

`enabled_click_would_fire` is now referenced ONLY by `accepts_click` (grep-verified: definition `fit_button.rs:91`, use `:87`; no test references it). Folding it in gives the EXACT shape of the sibling gates (`zoom_button.rs:90` and `floating_controls.rs:183`), one method instead of two, same semantics.

In `src/app/fit_button.rs`, replace the two methods
```rust
    /// A disabled fit button never fires (the disabled-arrow pattern); the
    /// open-with menu also suppresses the row dispatch (Wave-6.1 Task 3).
    fn accepts_click(&self, button_response: &egui::Response, row_ui: &egui::Ui) -> bool {
        self.enabled_click_would_fire(button_response)
            && !open_with_button::open_with_menu_is_open(row_ui.ctx())
    }

    fn enabled_click_would_fire(&self, button_response: &egui::Response) -> bool {
        self.enabled && button_response.clicked()
    }
```
with
```rust
    /// A disabled fit button never fires (the disabled-arrow pattern); the
    /// open-with menu also suppresses the row dispatch (Wave-6.1 Task 3).
    fn accepts_click(&self, button_response: &egui::Response, row_ui: &egui::Ui) -> bool {
        self.enabled
            && button_response.clicked()
            && !open_with_button::open_with_menu_is_open(row_ui.ctx())
    }
```

## Verification expectations (for the following harness step)

- `cargo fmt` in the container first (canonical line breaks after each replacement above; the snippets already follow the existing formatting of the sibling code — expect zero or near-zero reformat).
- dev-checks `ALL CHECKS PASSED` with **189 passed; 0 failed**, 28 test files — census and every test unchanged (no test file is edited by this plan).
- Expected file sizes: `keyboard_intake.rs` 181, `fit_button.rs` 155 (±rustfmt wrapping). If `rustc` rejects any step, REVERT that step only and report — do not improvise alternatives.

## Verified — no change (within reviewed boundaries)

- `src/app/pill_style.rs` (200, AT cap), `src/app/floating_controls.rs` (200, AT cap), `src/app/open_with_button.rs` (195) — this plan adds NOTHING to these; no compensating removal needed. Their new seams (`WARNING_MESSAGE_COLOR`, `message_text_color`, `show_message_line` color pre-computation [forced by the `with_layout` closure borrow], `message_line_is_warning`, `open_with_menu_id`/`open_with_menu_is_open`/`OPEN_WITH_MENU_KEY`, `accepts_click` + `menu_popup_is_wanted` gates, `refresh_open_with_programs`) are the plan-pinned D-decisions (D1–D8) — each single-use, correctly named, no duplication inside them.
- The D2 modal-gate call `!open_with_button::open_with_menu_is_open(...)` appears in five button modules — this is the pinned ONE modal-state query fanned out per gate (D2). A cross-module `enabled && clicked && !menu_open` merger would need 3 non-receiver params (violates the ≤2-args rule without a new param-object file → census change) and the gates differ legitimately (fullscreen always-enabled; open-with excludes its own popup toggle). REJECTED.
- `esc_close_confirm.rs` generic `non_repeat_key_press_occurred` + ESC delegator — clean dedup already landed by the implementation; no further change.
- `button_hint.rs` (185) — caption/tooltip data pinned by Task 2; unchanged.
- All test modules (`open_with_button_tests.rs`, `esc_close_confirm_tests.rs`, `pill_style_tests.rs`, `button_hint_tests.rs`) — pinned semantics, untouched.
- Rule sanity: method bodies ≤ 50 lines (`read_and_dispatch_keystrokes` stays the pre-existing documented long method, this plan makes it SHORTER by 5); new ifs single-section (the `||` lives inside the extracted named predicate, per the rule's remedy); nesting ≤ 2; ≤ 2 non-receiver params everywhere (`open_with_menu_key_pressed`/`open_with_menu_close_pressed`: 1 param); visibility minimal (helpers are private module-local free functions); no commented-out code; no magic numbers added.
