# Wave 4.2 Task 4 — ESC Double-Press Close Front-end Technical Specification

**Scope:** A-Task 4 of `.agent/todos/20261107/20261107-todo-1.md`. Produce only the front-end technical specification; no application code.

**Binding requirements (verbatim from 2026-11-07 chat):**

- When fullscreen is OFF, pressing ESC shows a CENTERED popup for 2 seconds with the message `"Press ESC again to close"`.
- ESC must be pressed 2 times — NOT held/autorepeated (two distinct presses).
- The app closes if ESC is pressed AGAIN WHILE the popup is visible ONLY.
- If the popup vanished, the next ESC press shows the popup again (re-armed).
- This feature works ONLY when fullscreen is OFF. Fullscreen ON keeps ESC = exits fullscreen (Wave-4.1 Task 3), and must NOT trigger popup or close.

---

## 1. UI surface

### 1.1 egui 0.37 mechanism

Use a top-level floating overlay, not a child of the pill:

```rust
egui::Area::new(egui::Id::unique(ESC_CLOSE_POPUP_AREA_ID))
    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
    .order(egui::Order::Foreground)
    .interactable(false)
    .show(ctx, |popup_ui| {
        egui::Frame::new()
            .fill(POPUP_FILL_COLOR)
            .corner_radius(POPUP_CORNER_RADIUS)
            .inner_margin(POPUP_INNER_MARGIN)
            .show(popup_ui, |frame_ui| {
                frame_ui.label(egui::RichText::new(POPUP_MESSAGE_TEXT).size(POPUP_TEXT_SIZE).color(POPUP_TEXT_COLOR));
            });
    });
```

- `Id::unique` with a fixed string salt prevents collisions with the existing pill (`"floating_navigation_pill"`) and empty-state Open button (`"empty_state_open_button"`) `Id::unique` overlays.
- `Order::Foreground` keeps the popup painted above the pill and the central surface.
- `anchor(Align2::CENTER_CENTER, Vec2::ZERO)` centers the popup in the viewport.
- `interactable(false)` makes the Area non-capturing: pointer clicks pass through to whatever is behind.

### 1.2 Visual style

Keep project-consistent with the pill's dark translucent frame, but make the popup larger and slightly more opaque so plain white text stays legible without adding a font outline:

| Constant | Value | Rationale |
| --- | --- | --- |
| `POPUP_FILL_COLOR` | `Color32::from_rgba_unmultiplied(20, 20, 20, 220)` | Same family as the pill fill (`20,20,20,180`), slightly more opaque because this popup is larger and the user must read a sentence. |
| `POPUP_CORNER_RADIUS` | `10` | Matches the pill corner radius. |
| `POPUP_INNER_MARGIN` | `16.0` | Larger than the pill's `8.0` because the message is a sentence, not a glyph. |
| `POPUP_TEXT_SIZE` | `14.0` | Readable centered message; larger than pill captions (9 pt). |
| `POPUP_TEXT_COLOR` | `Color32::WHITE` | Project-consistent enabled/caption color. |

### 1.3 Message string constant

Place the exact message in exactly one canonical location:

```rust
pub(super) const POPUP_MESSAGE_TEXT: &str = "Press ESC again to close";
```

This constant lives in the new `src/app/esc_close_confirm.rs` module. No other module may duplicate the literal string.

### 1.4 Pointer-interaction policy

The popup is a **pure non-capturing transient**. It does NOT intercept clicks, does NOT block hover, and does NOT provide a click-to-dismiss action. The only dismissal paths are:

1. The 2-second timeout expires.
2. Fullscreen is toggled ON (clears the popup).
3. A second distinct ESC press closes the entire application (not just the popup).

Justification: the user asked nothing about mouse interaction; the minimal behavior is non-capturing so it cannot accidentally interfere with the existing pill buttons, the Open button, or any other chrome.

---

## 2. Surface lifetime

### 2.1 Timer mechanism chosen: absolute deadline from `InputState::time`

Use `ctx.input(|i| i.time)` (an `f64` in seconds) to set an absolute expiry deadline when the popup is armed. Each frame, compare the current time against the stored deadline; if `current_time >= deadline`, clear the armed state.

```rust
const POPUP_VISIBLE_SECONDS: f64 = 2.0;

fn arm(&mut self, current_time: f64) {
    self.esc_close_popup_expires_at = Some(current_time + POPUP_VISIBLE_SECONDS);
}

fn update(&mut self, current_time: f64) {
    if let Some(deadline) = self.esc_close_popup_expires_at {
        if current_time >= deadline {
            self.esc_close_popup_expires_at = None;
        }
    }
}
```

Reason for picking this over per-frame `stable_dt` accumulation: it is frame-rate independent, one comparison per frame, and avoids accumulated floating-point drift. The docs.rs page for `InputState` in egui 0.37 confirms `pub time: f64` — "Time in seconds. Relative to whatever. Used for animation."

### 2.2 Re-arm semantics

- While the popup is visible (`esc_close_popup_expires_at.is_some()`), a non-repeat ESC press closes the app.
- After the deadline passes, `update` clears `esc_close_popup_expires_at` to `None`. The next non-repeat ESC press arms the popup again from scratch.
- Holding ESC from the first press generates repeat events (filtered out, see §4), so it does NOT re-arm prematurely.

---

## 3. State model

### 3.1 New session RAM field in `ImagesScrollerApp`

Add one field to the struct in `src/app.rs`:

```rust
/// Popup expiry deadline for the fullscreen-OFF ESC double-press close
/// affordance. `None` = popup is not visible. Absolute time in seconds from
/// `InputState::time`.
esc_close_popup_expires_at: Option<f64>,
```

Initialize it to `None` in `empty_state()`.

### 3.2 State helpers (new module `src/app/esc_close_confirm.rs`)

Implement these methods on `ImagesScrollerApp` inside the new module:

```rust
impl ImagesScrollerApp {
    /// True while the popup is currently armed/visible.
    pub(super) fn esc_close_popup_is_visible(&self) -> bool {
        self.esc_close_popup_expires_at.is_some()
    }

    /// Arms the popup now, setting the 2-second deadline.
    pub(super) fn arm_esc_close_popup(&mut self, ctx: &egui::Context) {
        let now = ctx.input(|input| input.time);
        self.esc_close_popup_expires_at = Some(now + POPUP_VISIBLE_SECONDS);
    }

    /// Clears the popup without closing the app. Called when fullscreen turns ON.
    pub(super) fn disarm_esc_close_popup(&mut self) {
        self.esc_close_popup_expires_at = None;
    }

    /// Expires the popup if its deadline has passed.
    pub(super) fn update_esc_close_popup(&mut self, ctx: &egui::Context) {
        let Some(deadline) = self.esc_close_popup_expires_at else { return };
        let now = ctx.input(|input| input.time);
        if now >= deadline {
            self.esc_close_popup_expires_at = None;
        }
    }
}
```

All helpers are `pub(super)` because they are internal to the `app` module family.

### 3.3 Intake ordering decision

The ESC branch in `keyboard_intake.rs` must be split by fullscreen state:

- **Fullscreen ON:** ESC exits fullscreen only, using the existing `fullscreen::escape_exits_fullscreen` predicate and `toggle_fullscreen_and_send_command`. The popup is never armed and never shown.
- **Fullscreen OFF:**
  - If popup is visible and a non-repeat ESC press occurred this frame → send `ViewportCommand::Close`.
  - Else if popup is not visible and a non-repeat ESC press occurred this frame → arm the popup.
  - Else do nothing.

No other key or action may arm or disarm the popup.

### 3.4 Fullscreen toggle must disarm the popup

The shared fullscreen toggle in `src/app/fullscreen.rs` must clear the popup whenever the fullscreen flag flips. This guarantees that:

- F11 pressed while popup is visible → popup hides immediately.
- ESC pressed while fullscreen ON → exits fullscreen and clears the popup (it was already clear, but clearing is idempotent).
- The next ESC press after exiting fullscreen does NOT close the app; it shows the popup again.

Update `toggle_fullscreen_and_send_command` to call `self.disarm_esc_close_popup()` immediately after computing the post-toggle flag:

```rust
pub(super) fn toggle_fullscreen_and_send_command(&mut self, screen_context: &egui::Context) {
    let fullscreen_after_toggle = self.toggle_fullscreen();
    self.disarm_esc_close_popup();
    screen_context
        .send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen_after_toggle));
}
```

---

## 4. Two-distinct-presses mechanism

### 4.1 Why `key_pressed` is insufficient

The existing `was_pressed` helper calls `input.key_pressed(Key)`. The egui 0.37 docs for `InputState::key_pressed` state explicitly: "Was the given key pressed this frame? **Includes key-repeat events.**"

Because `key_pressed` fires on autorepeat, a held ESC key would repeatedly trigger whatever action it gates. For the fullscreen-exit path this is already accepted behavior (Wave-4.1 Task 3); for the new close/arm path it violates the "two distinct presses, NOT held/autorepeated" requirement.

### 4.2 Mechanism chosen: scan `input.events` and filter `Event::Key { repeat: false }`

egui 0.37's `Event::Key` variant is:

```rust
Key {
    key: Key,
    physical_key: Option<Key>,
    pressed: bool,
    repeat: bool,
    modifiers: Modifiers,
}
```

The `repeat` field exists and is documented: "If this is a pressed event, is it a key-repeat?"

Define a pure helper in `src/app/esc_close_confirm.rs`:

```rust
pub(super) fn non_repeat_escape_press_occurred(events: &[egui::Event]) -> bool {
    events.iter().any(|event| {
        matches!(
            event,
            egui::Event::Key {
                key: egui::Key::Escape,
                pressed: true,
                repeat: false,
                ..
            }
        )
    })
}
```

### 4.3 Why not a pressed→released→pressed state machine

A state machine that waits for `pressed=true` → `pressed=false` → `pressed=true` across frames would also reject held-key repeats, but it is more complex and unnecessary because egui already classifies repeat events in the `repeat` flag. The `repeat: false` filter is the exact, minimal mechanism aligned with how egui's own input system works.

### 4.4 Read discipline: ALL reads before dispatch

In `keyboard_intake.rs`, collect every value before any mutation:

```rust
let fullscreen_pressed = was_pressed(screen_context, fullscreen::FULLSCREEN_TOGGLE_KEY);
let escape_exit_fullscreen_pressed = fullscreen::escape_exits_fullscreen(
    self.fullscreen_on,
    was_pressed(screen_context, fullscreen::FULLSCREEN_EXIT_KEY),
);
let escape_events = screen_context.input(|input| input.events.clone());
let non_repeat_escape_pressed =
    esc_close_confirm::non_repeat_escape_press_occurred(&escape_events);
// ... existing zoom/fit/arrow reads unchanged ...
```

Then dispatch in a mutually-exclusive ESC branch:

```rust
self.update_esc_close_popup(screen_context);

if fullscreen_pressed {
    self.toggle_fullscreen_and_send_command(screen_context);
}
if escape_exit_fullscreen_pressed {
    self.toggle_fullscreen_and_send_command(screen_context);
} else if self.esc_close_popup_is_visible() {
    if non_repeat_escape_pressed {
        screen_context.send_viewport_cmd(egui::ViewportCommand::Close);
    }
} else if non_repeat_escape_pressed {
    self.arm_esc_close_popup(screen_context);
}
// ... existing zoom/fit/arrow dispatch unchanged ...
```

Note: `update_esc_close_popup` runs first so an expired popup is cleared before the ESC reads decide whether to close or re-arm.

### 4.5 Held-ESC behavior

- First physical press: `non_repeat_escape_pressed = true`, popup arms.
- While held: only `repeat: true` events arrive; `non_repeat_escape_pressed = false`. Popup stays visible until timeout or release.
- Release and press again: a new `repeat: false` event arrives while popup is visible → app closes.
- Holding through the timeout: popup expires, state clears; repeats are still ignored; releasing and pressing again re-arms.

---

## 5. App-close mechanism

### 5.1 Verified egui 0.37 command

Use `egui::ViewportCommand::Close`.

docs.rs for `ViewportCommand::Close` states: "Request this viewport to be closed. For the root viewport, this usually results in the application shutting down."

Because `ImagesScrollerApp` runs as the root viewport in eframe, sending this command closes the window and terminates the application.

### 5.2 Send site

In `keyboard_intake.rs`, inside the `self.esc_close_popup_is_visible()` branch:

```rust
screen_context.send_viewport_cmd(egui::ViewportCommand::Close);
```

No other code path sends `Close`.

---

## 6. Interplay

### 6.1 Fullscreen toggles ON while popup is visible

- `toggle_fullscreen_and_send_command` calls `disarm_esc_close_popup()` before sending the viewport command.
- The popup hides in the same frame the fullscreen flag flips ON.
- The fullscreen-exit ESC path is the only ESC behavior while fullscreen is ON; it does not consult the popup state.

### 6.2 Fullscreen-exit ESC must not arm/close

At read time, `escape_exit_fullscreen_pressed` is computed using the fullscreen flag **before** dispatch. If `fullscreen_on` is `true`, that predicate is `true` (assuming ESC was pressed). The `else if` chain ensures that after exiting fullscreen, the same ESC event is not reused to arm the popup in the same frame.

### 6.3 Popup visibility in all fullscreen-OFF states

The popup render call is placed inside `show_chrome_layout` (the fullscreen-OFF branch), after the navigation layout or empty-state layout has run. Therefore it appears in:

- Fresh empty state (centered Open button visible behind/under the popup).
- Image open state (pill visible behind/under the popup).
- Empty-folder browsed state (central "Empty folder" message visible behind/under the popup).

It never appears while fullscreen is ON because `show_chrome_layout` is not called in that branch.

### 6.4 No ID collision

Existing `Id::unique` overlays:

- `"floating_navigation_pill"` (pill)
- `"empty_state_open_button"` (Open button)

Use a distinct salt: `"esc_close_confirm_popup"`. `Id::unique` is deterministic for a given salt string, so distinct salts guarantee distinct IDs.

### 6.5 Existing chrome is not disturbed

The popup is rendered as a separate `Area` at `Order::Foreground`. It does not change the pill layout, the message line, button enablement, or the central surface. It does not steal focus because it is `interactable(false)`.

---

## 7. Impact list

### 7.1 Files to touch

| File | Change |
| --- | --- |
| `src/app.rs` | Add `esc_close_popup_expires_at: Option<f64>` field and initialize to `None`. Add `esc_close_confirm` module declaration and test module declaration. Call `esc_close_confirm::show_esc_close_popup(self, ui.ctx())` in `show_chrome_layout` after the existing layout branch. |
| `src/app/esc_close_confirm.rs` | **New module.** Constants (message, duration, area id, styling), pure `non_repeat_escape_press_occurred` helper, `impl ImagesScrollerApp` helpers (`update_esc_close_popup`, `arm_esc_close_popup`, `disarm_esc_close_popup`, `esc_close_popup_is_visible`), and the `show_esc_close_popup` render function. |
| `src/app/keyboard_intake.rs` | Read the raw events list and compute `non_repeat_escape_pressed`. Add the update + mutually-exclusive ESC dispatch block before the existing zoom/arrow dispatch. |
| `src/app/fullscreen.rs` | Call `self.disarm_esc_close_popup()` inside `toggle_fullscreen_and_send_command`. |
| `src/app/esc_close_confirm_tests.rs` | **New test module.** State-machine and input-filter tests. |
| `.agent/project-structure.md` | Add the two new files to the `src/app/` folder description. (Documented by the docs-specialist in A-Task 6, but listed here for completeness.) |

### 7.2 Expected unit tests (census delta)

Add a new test file `src/app/esc_close_confirm_tests.rs` with at least these tests:

1. `popup_starts_invisible` — fresh app has no popup.
2. `arm_makes_popup_visible` — `arm_esc_close_popup` sets a future deadline.
3. `update_before_deadline_keeps_popup_visible` — time before deadline leaves it armed.
4. `update_at_deadline_clears_popup` — time at/after deadline clears it.
5. `non_repeat_escape_press_detected` — synthetic `Event::Key { Escape, pressed: true, repeat: false }` is recognized.
6. `repeat_escape_press_is_ignored` — synthetic `Event::Key { Escape, pressed: true, repeat: true }` is ignored.
7. `distinct_second_press_closes_when_popup_visible` — state transition test: arm → second non-repeat press while visible → `Close` command sent.
8. `fullscreen_toggle_disarms_popup` — toggle fullscreen ON while popup visible clears the deadline.
9. `fullscreen_exit_esc_does_not_arm_popup` — simulate fullscreen ON + ESC press → fullscreen OFF, popup stays invisible.
10. `popup_renders_in_all_fullscreen_off_states` — lightweight predicate tests for fresh/empty-folder/image states (no actual render needed; verify `esc_close_popup_is_visible()` can be true in each).

Expected test census increase: **+10** tests (one new test file). Current project census is 107; A-Task 4 implementation should land the test file at 117 before A-Task 5 verifies.

---

## 8. 4.5a conformance checklist

For verification after implementation, the following must all be true:

1. **Window options unchanged** — `app.rs` still uses `with_title("Images Scroller")`, `with_resizable(true)`, `with_maximized(true)`; no maximized/popup window options are added.
2. **Message literal is canonical** — The exact string `"Press ESC again to close"` appears only as `POPUP_MESSAGE_TEXT` in `src/app/esc_close_confirm.rs`.
3. **Popup is centered** — Uses `Area::anchor(Align2::CENTER_CENTER, Vec2::ZERO)`.
4. **Popup is non-capturing** — `Area::interactable(false)` is set.
5. **Popup layers above chrome** — `Area::order(Order::Foreground)` is set.
6. **Two-second lifetime** — `POPUP_VISIBLE_SECONDS` is exactly `2.0` and the deadline comparison uses `>=`.
7. **Frame-rate-independent timing** — Deadline is computed from `ctx.input(|i| i.time)`, not from `stable_dt` accumulation.
8. **Two distinct presses** — ESC arm/close uses `non_repeat_escape_press_occurred` over `input.events`, filtering `Event::Key { repeat: false }`.
9. **Held ESC does not arm or close** — Repeat events are filtered out.
10. **Fullscreen ON consumes ESC** — `escape_exit_fullscreen_pressed` uses `was_pressed`/`key_pressed` and runs through `toggle_fullscreen_and_send_command`; the popup branch is skipped.
11. **Fullscreen toggle disarms popup** — `toggle_fullscreen_and_send_command` calls `disarm_esc_close_popup`.
12. **Close command is correct** — The app-close path sends `egui::ViewportCommand::Close` and nothing else.
13. **Popup visible in all fullscreen-OFF states** — Render call is inside `show_chrome_layout`, not gated by `navigation_context_is_active()`.
14. **No ID collision** — The popup area salt is `"esc_close_confirm_popup"`, distinct from `"floating_navigation_pill"` and `"empty_state_open_button"`.
15. **State field initialized** — `esc_close_popup_expires_at` is `None` in `empty_state()`.
16. **No per-frame allocation churn** — The events vector read is reused for the single ESC scan; the popup renders only while armed; no texture or heavy object is recreated per frame.
17. **Tests added** — `src/app/esc_close_confirm_tests.rs` exists and covers show→close, expiry→re-arm, repeat-filter, fullscreen interplay, and disarm-on-toggle.

---

## 9. Research notes

- egui 0.37 `InputState::key_pressed`: "Includes key-repeat events." (docs.rs, verified 2026-11-10)
- egui 0.37 `Event::Key` fields include `pressed: bool` and `repeat: bool`. (docs.rs, verified 2026-11-10)
- egui 0.37 `ViewportCommand::Close`: "Request this viewport to be closed. For the root viewport, this usually results in the application shutting down." (docs.rs, verified 2026-11-10)
- egui 0.37 `Area::interactable(false)`: "If false, clicks goes straight through to what is behind us." (source/docs.rs, verified 2026-11-10)
- egui 0.37 `InputState::time`: `pub time: f64` — "Time in seconds. Relative to whatever. Used for animation." (docs.rs, verified 2026-11-10)
