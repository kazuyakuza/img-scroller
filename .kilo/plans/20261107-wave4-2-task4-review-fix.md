# Review Fix Plan — Wave 4.2 Task 4: ESC Double-Press Close While Fullscreen OFF

**Source:** A-Task 4, Step 4.3 (code-reviewer) review of commits `f4e9a55` + `7e235fa`.
**Branch:** `fix/wave-4-2-operator-feedback`
**Scope:** Correct the formatting deviation found in the Task 4 implementation; re-run verification.

---

## Findings

### 1. Formatting deviation in Task 4 files (MUST FIX)

`cargo fmt --check` fails on the two Task 4 files that the implementer created/modified:

- `src/app/esc_close_confirm_tests.rs` — long `assert_eq!` calls and `escape_key_event(...)` argument lists exceed rustfmt's width threshold.
- `src/app/keyboard_intake.rs` — the in-closure `non_repeat_escape_pressed` read can be collapsed to one line per rustfmt.

The implementation plan §5.7 explicitly requires the implementer to run `cargo fmt` before committing. The committed files were not formatted, so this step was skipped.

### 2. Pre-existing build blocker discovered during verification (NOT caused by Task 4)

`cargo check --locked` fails at commit `7e235fa` with:

```text
error[E0433]: cannot find `Rot2` in `egui`
  --> src/app/pill_style.rs:77:26
error[E0433]: cannot find `PathStroke` in `egui`
  --> src/app/pill_style.rs:90:15
```

The same errors reproduce at the Task 4 baseline `5edea1c`, confirming they are pre-existing and unrelated to the ESC-close feature. This blocker prevents `scripts/dev-checks.sh` from passing and prevents the Task 4 unit-test census from being verified in the container. It must be resolved before A-Task 5, but it is out of scope for this Task 4 review fix.

### 3. Code-correctness review — all Task 4 requirements satisfied

Aside from formatting, the implementation matches the plan and binding constraints:

- File set is exactly the planned 5 files (`esc_close_confirm.rs`, `esc_close_confirm_tests.rs`, `app.rs`, `keyboard_intake.rs`, `fullscreen.rs`).
- Constants are exact: `POPUP_MESSAGE_TEXT = "Press ESC again to close"`, `POPUP_VISIBLE_SECONDS = 2.0`.
- `non_repeat_escape_press_occurred` correctly filters on `repeat: false`.
- `esc_close_action` branches exactly as specified; `ViewportCommand::Close` is sent in exactly one place.
- Pure `_at(now)` cores exist and are used by tests.
- Render is self-gated, centered (`CENTER_CENTER`), `Order::Foreground`, `interactable(false)`, and uses the unique salt `"esc_close_confirm_popup"` (no collision with `"floating_navigation_pill"` or `"empty_state_open_button"`).
- `keyboard_intake.rs` preserves read-before-dispatch order; the fullscreen-exit path keeps priority; `was_pressed` is untouched.
- `fullscreen.rs` disarms after the flag flip and before the command send; every production fullscreen toggle (F11 intake, ESC-exit, pill button) funnels through `toggle_fullscreen_and_send_command`.
- All 11 pinned tests are present with the exact names and cover the required invariants.
- Line budgets are within limits (146 / 161 / 197 / 104 / 58, all ≤ 200); `read_and_dispatch_keystrokes` body remains at the pre-declared ~53-line exemption.
- No commented-out code; max nesting depth ≤ 2; non-`self` params ≤ 2; private-by-default observed.

---

## Fix steps

1. In the Alpine container, run the formatter that the implementer skipped:
   ```sh
   docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt
   ```
   This will rewrite:
   - `src/app/esc_close_confirm_tests.rs`
   - `src/app/keyboard_intake.rs`
   - plus the pre-existing unformatted files `src/app/zoom_alias_tests.rs` and `src/app/zoom_tests.rs` (these are not Task 4 changes, but they are currently failing `fmt --check` and must be cleaned up so the project-wide formatter gate passes).

2. Inspect `git diff --stat` to confirm the formatting-only diff is limited to the four files above and contains no behavioral changes.

3. Stage and amend the formatting fixes into the existing feature commit `7e235fa` **or** create a follow-up fix commit. Either way, the final state on `fix/wave-4-2-operator-feedback` must have the Task 4 files formatted and must not introduce any functional change.

4. Escalate the pre-existing `pill_style.rs` compile failure separately (outside this Task 4 fix). Do **not** attempt to fold an egui API fix into this review cycle.

---

## Verification after fix

1. `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt --check` exits 0.
2. `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked` — expected to still fail on `pill_style.rs` until that pre-existing blocker is resolved; record the same two errors as evidence that Task 4 did not introduce them.
3. Once the `pill_style.rs` blocker is resolved independently, run `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` and confirm:
   - `ALL CHECKS PASSED`
   - test census = 127 passed, 0 failed

---

## Out of scope for this fix

- Any change to `src/app/pill_style.rs`.
- Any change to behavior, naming, or structure of the ESC-close feature.
- The OPERATOR-RUN checklist, exe rebuild, and DLL audit (A-Task 5).
