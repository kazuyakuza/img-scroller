# Wave 4.1 Task 3 — Fullscreen tuning (F11 immediacy + ESC exit): Front-end Technical Specification

Date: 2026-11-06  
Branch: `fix/wave-4-operator-fixes` · Base HEAD: `892efc111db3ded8cf30c4c6eba2701535596f6c`  
Source TODO: `.agent/todos/20261102/20261102-todo-1.md` (Task 3)  
Related prior specs/plans: `.kilo/plans/20261102-wave4-task1-fullscreen-frontend-spec.md`, `.kilo/plans/20261102-wave-4-task1-fullscreen-plan.md`  
Spec file: `.kilo/plans/20261102-wave4-1-task3-fullscreen-tuning-frontend-spec.md` (NEW)

## 1. Binding behavior (source of truth)

- F11 remains a fullscreen TOGGLE (ON → OFF and OFF → ON) via the existing shared `toggle_fullscreen_and_send_command` action.
- ESC exits fullscreen when it is ON. When fullscreen is OFF, ESC does NOTHING (no-op). ESC does not collide with any other keybinding.
- Fullscreen toggles still never touch `zoom_factor` or `fit_mode`.
- The chrome-hiding contract and the single command-send per toggle frame remain unchanged.
- The perceived F11 "not always immediate" behavior must be root-caused IN CODE; any cause that only shows on a real Windows run is recorded for the OPERATOR-RUN note, not patched blindly.

## 2. Verified egui 0.37 API facts

- `InputState::key_pressed(&self, desired_key: Key) -> bool` is documented as: **"Was the given key pressed this frame? Includes key-repeat events."** (docs.rs/egui/0.37.0/source `input_state/mod.rs`).
- `Event::Key` carries a `repeat: bool` field. egui auto-detects repeats (`repeat = !first_press` on the raw `pressed: true` event). The public `key_pressed` does **not** filter repeats; it returns `true` for every repeat event in the frame.
- `egui::Key::Escape` and `egui::Key::F11` both exist in the egui 0.37 `Key` enum (source verified).
- `ViewportCommand::Fullscreen(bool)` is documented as: **"Turn borderless fullscreen on/off."** `ViewportCommand` is described as **"essentially a way to diff ViewportBuilders"**. The egui-winit integration only forwards the command when the requested value differs from the current state (`if new_fullscreen != self.fullscreen { commands.push(...) }`). Sending the same value repeatedly would therefore be a no-op.

## 3. Root-cause analysis surface for F11 "immediacy"

### 3.1 Agent-verifiable facts from the current code

1. `src/app/fullscreen.rs` sends `egui::ViewportCommand::Fullscreen(fullscreen_after_toggle)` exactly once per trigger, inside `toggle_fullscreen_and_send_command`.
2. `src/app/keyboard_intake.rs` reads F11 once per frame and dispatches the shared toggle action; the read happens before layout dispatch and cannot be gated out by any UI state.
3. `key_pressed` is true for the frame that contains the key-press event. A normal press/release cycle spans multiple frames, so the toggle fires on the first frame where the press event appears.
4. A press+release that both occur within a single 16 ms frame still leaves a `pressed: true` event in that frame (egui does not suppress it), so the toggle still fires that frame.
5. No per-frame command spam exists: the code only sends the command when the flag flips.

### 3.2 Candidate causes for the operator perception

| # | Candidate | Agent-fixable? | Evidence/expected test |
|---|-----------|----------------|------------------------|
| A | **OS key-repeat delay while the key is held.** `key_pressed` includes repeats, but the first OS repeat event is delayed by the standard Windows repeat-start delay (~250–500 ms). If the operator holds F11 expecting rapid toggling, the second toggle feels delayed. | No — this is OS/winit behavior. | Operator test: single short tap vs. held key. Single tap should toggle immediately; held key should show the OS repeat gap. |
| B | **Input is processed once per egui frame (~16 ms at 60 Hz).** This is inherent to the immediate-mode loop and is not a bug. | No — frame-bound input is correct by design. | Agent inspection confirms `keyboard_intake.rs` already reads once per frame; no queue backlog. |
| C | **Window-focus quirks on Windows.** If the window lacks keyboard focus when F11 is pressed, the event is not delivered. | No — environment/user-run issue. | Operator test: ensure the app window is focused before pressing F11. |
| D | **Repeated same-value `ViewportCommand::Fullscreen` commands causing layout sync storms.** | Not present in code — the toggle is edge-triggered, so the same value is never sent twice in a row. | Static code review: only `toggle_fullscreen_and_send_command` sends the command, and it only runs when `key_pressed` is true. |
| E | **Resize-event storm when entering fullscreen.** The central panel already recomputes fit from `available_size()` every frame; this is cheap and correct. | No change needed. | Display code in `app_display.rs` / `central_surface.rs` already handles per-frame resize; no texture re-upload occurs on resize. |

### 3.3 Recommended investigation path for the architector

1. Verify the current code matches §3.1 (it does per this review).
2. Do **not** change the toggle timing, add timers, or switch to `key_down` polling. Those would violate the edge-triggered contract and could create per-frame toggle storms.
3. If a purely cosmetic improvement is desired, the only minimal candidate is to document the OS repeat-delay behavior in the OPERATOR-RUN note. No code change is required for Task 3.
4. If the operator later reports that a single short F11 press is still delayed, that becomes a Windows/winit environment defect and must be reproduced/reported by the operator, not agent-fixed.

## 4. ESC exit contract (exact)

### 4.1 New constant

- Add in `src/app/fullscreen.rs`:
  ```rust
  pub(super) const FULLSCREEN_EXIT_KEY: egui::Key = egui::Key::Escape;
  ```

### 4.2 Behavior

- `fullscreen_on == true` and `Escape` is pressed this frame → call `self.toggle_fullscreen_and_send_command(screen_context)`. This exits fullscreen because the flag flips to `false` and the matching `ViewportCommand::Fullscreen(false)` is sent exactly once.
- `fullscreen_on == false` and `Escape` is pressed this frame → **no-op**. ESC must not toggle fullscreen ON.
- At all other times → no-op.

### 4.3 Collision check

A `grep` for `Escape|ESCAPE|escape` across `src/` returned **no matches**. Therefore ESC currently has no other use in the codebase and does not collide with any existing keybinding.

## 5. Keyboard intake placement

### 5.1 Reads-first block

In `src/app/keyboard_intake.rs`, add one read immediately after the F11 read:

```rust
let fullscreen_pressed = was_pressed(screen_context, fullscreen::FULLSCREEN_TOGGLE_KEY);
let escape_exit_pressed = self.fullscreen_on && was_pressed(screen_context, fullscreen::FULLSCREEN_EXIT_KEY);
```

The `escape_exit_pressed` bool encapsulates the two-section condition so that the downstream `if` remains single-section.

### 5.2 Dispatch block

Add dispatch immediately after the F11 dispatch and before the zoom dispatch:

```rust
if fullscreen_pressed {
    self.toggle_fullscreen_and_send_command(screen_context);
}
if escape_exit_pressed {
    self.toggle_fullscreen_and_send_command(screen_context);
}
```

### 5.3 Why this placement

- Keeping ESC next to F11 groups fullscreen controls together.
- Dispatching before zoom/fit preserves the existing message-overwrite semantics (zoom-bound messages can still be overwritten by fit activations in the same frame).
- The read-before-dispatch pattern is unchanged; ESC cannot be gated by UI state.

### 5.4 Single-section rule

The `if fullscreen_pressed { ... }` and `if escape_exit_pressed { ... }` conditions each remain a single boolean. The compound predicate (`fullscreen_on && was_pressed(...)`) is computed into a named bool in the reads block, satisfying the single-section-boolean-conditions rule.

## 6. OPERATOR-RUN note items

The following must be tested by the operator on a real Windows 10/11 machine:

1. **F11 perceived immediacy**
   - Test a **single short tap** of F11 in both directions. Expected: immediate toggle in the same frame.
   - Test **holding F11**. Expected: the first toggle is immediate; subsequent toggles follow the OS key-repeat interval (there will be a noticeable delay before repeats start). This is normal OS behavior, not a bug.
   - Ensure the window has focus before pressing F11.
2. **ESC exits fullscreen**
   - Press F11 to enter fullscreen, then press ESC. Expected: fullscreen exits, chrome returns, image display unchanged.
   - Press ESC while NOT in fullscreen (normal image view, empty state, empty-folder browsing). Expected: nothing happens; fullscreen does not turn ON.
3. **No other interplay**
   - F11 still toggles both ways.
   - ESC does not affect zoom, fit mode, navigation, or the message line.
4. **If a single short F11 press still feels delayed**
   - Record the Windows version, keyboard repeat-rate settings, and whether the window was focused. Do not agent-patch; report back for investigation.

## 7. Unit-test surface

Pure-logic tests only (no `egui::Context` mocking):

1. **Key constant pin**: assert `FULLSCREEN_EXIT_KEY == egui::Key::Escape`.
2. **Exit predicate helper** (recommended extraction):
   ```rust
   fn escape_exits_fullscreen(fullscreen_on: bool, escape_pressed: bool) -> bool {
       fullscreen_on && escape_pressed
   }
   ```
   Tests cover all four truth combinations and confirm the no-op when not fullscreen.

If the implementer chooses to inline the predicate rather than extract a helper, the helper test can be omitted, but the constant-pin test must remain. To keep test count predictable, this spec expects:

- **+2 unit tests** (constant pin + predicate).

Intake-order tests are not practical without mocking the egui context, so they are not required; the order is enforced by the spec and verified at 4.5a.

## 8. 4.5a conformance checklist

For later front-end verification against this spec:

1. `FULLSCREEN_EXIT_KEY` is defined in `src/app/fullscreen.rs` and equals `egui::Key::Escape`.
2. `ImagesScrollerApp` state model is unchanged (still exactly one `fullscreen_on: bool` field, default `false`).
3. `toggle_fullscreen_and_send_command` still flips the flag once and sends `ViewportCommand::Fullscreen(...)` exactly once per call.
4. The ESC read is added to the reads-first block in `src/app/keyboard_intake.rs`.
5. ESC dispatch happens after the reads and before zoom/fit dispatch.
6. ESC only triggers the shared toggle action when `fullscreen_on == true`; otherwise it is a no-op.
7. No other key uses Escape; no collision is introduced.
8. All `if` conditions in the intake remain single-section boolean variables.
9. `window_options()` and `NativeOptions` are unchanged by this task.
10. No per-frame texture re-upload or allocation churn is introduced (`CurrentImageDisplay` discipline remains one upload per decode).
11. Unit-test count increases by exactly 2 (or the equivalent if the predicate is inlined, in which case the constant-pin test is still required).

## 9. Out of scope (hard)

- Task 2 button hotkey references + hover tooltips (already done).
- Zoom range/alias/fit-semantics corrections (Tasks 5–7 of this wave).
- Fullscreen hover-reveal of the pill (future-idea item 2, parked in `future-ideas.md`).
- Pan-then-navigate behavior (Wave 5).
- `CHANGELOG.md` edits.
- Any change to `window_options()` launch state (Task 1 of this wave already owns that).
- Multi-monitor fullscreen selection (`ViewportCommand::SetMonitor`).
