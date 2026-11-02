# Wave 4 Task 1 — Fullscreen Toggle: Front-end Technical Specification

Date: 2026-11-02  
Source TODO: `.agent/todos/20261029/20261029-todo-6.md`  
Base: `feat/wave-4-fullscreen-zoom` @ v0.4.0 (0.3.0 acceptance gate closed)  
Spec file: `.kilo/plans/20261102-wave4-task1-fullscreen-frontend-spec.md`  

## 1. Binding behavior (source of truth)

- Default fullscreen state at launch: **OFF**. The window opens normal or maximized exactly as the user placed/sized it.
- Fullscreen **ON**: hide the native window title bar and borders, and hide **ALL** UI controls. Only the decoded image is shown, filling the client area.
- Fullscreen **OFF**: all UI controls are visible again, laid out exactly as in v0.3.0; the image takes almost all of the central space.
- Toggle is available through **both** a keyboard shortcut **and** a UI control. The exact keybinding is proposed in this spec and encoded by the architector in step 4.1b.

## 2. Verified egui 0.37 API facts

- `egui::ViewportCommand::Fullscreen(bool)` is documented as: **"Turn borderless fullscreen on/off."** (docs.rs/egui/0.37.0).
- `egui::ViewportCommand::Decorations(bool)` is documented as: **"Show window decorations, i.e. the chrome around the content with the title bar, close buttons, resize handles, etc."**
- `egui::ViewportBuilder::with_fullscreen` is documented for **launch-time** fullscreen only: "Sets whether the window should be put into fullscreen upon creation. The default is None." It must **not** be used because launch fullscreen is forbidden.
- `egui::ViewportBuilder::with_maximized` defaults to `false`; the existing `with_maximized(false)` in `src/main.rs` is correct and must stay.
- `egui::ViewportBuilder::with_decorations` defaults to `true`; no launch-time decoration change is allowed.
- The runtime command is sent via `ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(bool))` (verified method name in egui 0.37.0).
- Keyboard intake uses `ui.ctx().input(|i| i.key_pressed(egui::Key::...))`, the same pattern already used for the arrow keys in `src/app.rs`.

## 3. Chosen fullscreen mechanism

**Candidate: runtime `ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen_on))`.**

- This is the single mechanism to encode. It is the only runtime command whose documented purpose is borderless fullscreen, which satisfies "hide the native window title bar (and borders)".
- The command is sent from the app's `update`/`ui` method whenever the session flag changes (edge-triggered: only on the frame the flag flips, not every frame).
- No `NativeOptions` or `ViewportBuilder` option may force size, maximized, or fullscreen at launch.

### Per-runtime caveats

- The app targets **Windows 10/11** only, using the **glow** renderer of eframe 0.37.0. The borderless-fullscreen behavior is ultimately executed by `egui-winit` + winit on that platform.
- Logical-point sizing and DPI are handled by egui; the image fit-scaling (`scaling::fitted_size`) already recomputes per frame from `panel_ui.available_size()`, so monitor DPI changes do not require extra code.
- Multi-monitor placement beyond the current monitor is **out of scope** for Task 1. `ViewportCommand::SetMonitor(usize)` exists but must not be used unless the user/planner explicitly expands scope later.

### Failure / edge cases

- **Minimize from fullscreen**: when the user Alt-Tabs or minimizes a borderless fullscreen window, Windows will minimize it. Toggling OFF while minimized must restore the windowed state. The implementation must send `Fullscreen(false)` on toggle-OFF; egui/winit is expected to restore decorations and previous size. If the restore size is wrong, that is an operator-run defect, not a spec deviation.
- **Multi-monitor**: the fullscreen occupies the monitor that currently contains the window. Moving the fullscreen window across monitors via OS shortcuts is OS-defined behavior.
- **No image open**: if fullscreen is toggled while no image is decoded (fresh empty state or empty-folder browsing), the central surface is blank and all controls are still hidden. This is acceptable because the primary use case is viewing an image.
- **Lost focus / close**: closing the app while fullscreen must terminate cleanly; no special cleanup is required beyond normal app shutdown.

### Rejected alternatives

- `ViewportCommand::Decorations(false)` + manual maximize: this would hide only the title bar and still leave the window as a resizable rectangle; it is not true borderless fullscreen and is rejected.
- `ViewportBuilder::with_fullscreen(true)` or `with_maximized(true)` at launch: violates the "default OFF / placement respected" requirement and is rejected.

## 4. Fullscreen state model

Add **one** field to `ImagesScrollerApp` in `src/app.rs`:

```rust
fullscreen_on: bool,
```

Rules:

- Initialized to `false` in `empty_state()` (default OFF at launch).
- Session-only RAM state; **no persistence** to disk in v1.
- Toggled by the keyboard shortcut and by the UI control.
- No state lives in `NativeOptions`; `window_options()` must keep `with_maximized(false)` and must not add `with_fullscreen`, `with_decorations(false)`, or any forced initial size.

## 5. UI-control hiding contract

### Controls that exist in v0.3.0 (must all be gated)

1. **Floating navigation pill** (`src/app/floating_controls.rs`):
   - Message line.
   - Four painted vector-triangle arrow buttons (`[←] [→] [↑] [↓]`).
2. **Empty-state center "Open…" button** (`src/app.rs`, `show_centered_open_button`).

### Fullscreen ON

- `floating_controls::show_floating_pill` must **not** be called.
- `show_centered_open_button` must **not** be called.
- The `CentralPanel` still occupies the full client area.
- If `decoded_image` is `Some`, paint it via `CurrentImageDisplay::paint`.
- If `decoded_image` is `None`, the central surface is blank (no "Empty folder" message, no Open button).

### Fullscreen OFF

- Render exactly the v0.3.0 layout:
  - Navigation context active (image open or empty-folder browsed) → show the floating pill + central panel with image or empty-folder message.
  - Fresh empty state → show the big centered Open button only.

## 6. Exit interplay

Because **all** UI controls are hidden in fullscreen ON, the only built-in way to leave fullscreen is the keyboard shortcut. The UI toggle control is intentionally invisible while fullscreen is ON. The architector must choose a keybinding that is one-handed and standard for Windows fullscreen toggles.

## 7. Keybinding candidates (proposed to the architector)

| Candidate | Binding | Pros | Cons |
|---|---|---|---|
| **F11** | `egui::Key::F11` | De-facto Windows/media fullscreen toggle; one-handed; does not conflict with arrows. | Some laptops require Fn key, but still standard. |
| **Esc** | `egui::Key::Escape` | Universally understood as "exit mode"; one-handed. | Traditionally exits rather than toggles; may be wanted later for dismissing menus/d dialogs. |

**Recommendation: F11 as the primary toggle.** It is the most conventional one-key fullscreen toggle on Windows and leaves Esc available for future dismiss/cancel semantics.

The architector encodes **one** of these; the final binding is recorded in the implementation plan.

## 8. Keyboard intake contract

In `ImagesScrollerApp::ui` (`src/app.rs`):

- Evaluate the toggle shortcut per frame, alongside the existing arrow-key intake, using the same pattern:
  ```rust
  let fullscreen_toggle_pressed = ui.ctx().input(|i| i.key_pressed(SELECTED_KEY));
  ```
- When `fullscreen_toggle_pressed` is `true`, flip `self.fullscreen_on` and send the viewport command once:
  ```rust
  if fullscreen_toggle_pressed {
      self.fullscreen_on = !self.fullscreen_on;
      ui.ctx().send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen_on));
  }
  ```
- Edge-triggered: `key_pressed` is true for only one frame per physical press, so no per-frame re-toggle occurs while the key is held.
- No interplay with the arrow keys (`ArrowLeft`, `ArrowRight`, `ArrowUp`, `ArrowDown`) or the navigation seam.
- The shortcut must be evaluated **before** the fullscreen-OFF layout decision so the correct layout is rendered in the same frame.

## 9. UI toggle control placement proposals (propose-and-ask)

The following options are presented to the Planner/user for approval. Only the **OFF-state** placement matters; the control is invisible in fullscreen ON.

### Option A — Fifth button inside the existing floating pill

- Placement: add a square/maximize-style button as a fifth element in the pill row, e.g. after the folder buttons (`[←] [→] [↑] [↓] [□]`).
- Pros: single consolidated control surface; no new overlay area; discoverable alongside navigation.
- Cons: widens the pill; the fullscreen icon must be painted (like the arrows) because font glyphs are avoided; slightly reduces visual focus on navigation.

### Option B — Small separate corner button near the pill

- Placement: a small square button anchored bottom-right of the window, a few pixels away from the pill, rendered as a second `egui::Area`.
- Pros: keeps the pill dedicated to navigation; fullscreen action is spatially separated and harder to mis-click.
- Cons: two overlay areas to maintain; may visually compete with the image corner.

### Option C — Top-right corner overlay button

- Placement: a small square button anchored to the top-right of the window content area, separate from the pill.
- Pros: matches the conventional "window chrome" location that users associate with maximizing/fullscreen; does not interfere with the bottom-center navigation habit.
- Cons: a third overlay area; top-right is far from the bottom pill, so the UI is less consolidated.

**Recommendation for Planner presentation: Option A (fifth pill button)** is the simplest evolution of the existing one-pill layout and keeps the UI surface unified. Option B is a reasonable alternative if the user wants navigation and window-mode actions separated.

## 10. Unit-test surface

Container-runnable tests (pure logic, no OS/GUI runtime):

- `control_visibility_gate(fullscreen_on, navigation_context_active) -> bool`: returns whether the pill and Open button should be rendered.
- `should_enter_fullscreen_image_view(fullscreen_on, decoded_image_present) -> bool`: returns whether to paint the image in fullscreen mode vs. blank surface.
- Tests for the state transition helper that flips `fullscreen_on` and produces the matching `ViewportCommand::Fullscreen` value.

OS-bound viewport behavior (actual borderless fullscreen, title-bar hiding, restore) is **operator-run** on a real Windows 10/11 machine and is not unit-testable in the Linux container.

## 11. 4.5a conformance checklist

For later front-end verification against this spec:

1. `ImagesScrollerApp` contains exactly one new field: `fullscreen_on: bool`.
2. `fullscreen_on` initializes to `false` and is never persisted.
3. `window_options()` does **not** add `with_fullscreen`, `with_decorations(false)`, forced size, or any launch-time maximization beyond existing `with_maximized(false)`.
4. Fullscreen ON uses `ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true))` exactly on the toggle frame.
5. Fullscreen OFF uses `ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false))` exactly on the toggle frame.
6. Fullscreen ON hides the floating pill (all four arrows + message line) and the empty-state center Open button.
7. Fullscreen OFF renders the v0.3.0 layout unchanged.
8. The keyboard shortcut is evaluated with `ui.ctx().input(|i| i.key_pressed(...))`, edge-triggered, and does not interfere with arrow-key navigation.
9. The UI toggle control placement matches the user-approved option.
10. No per-frame texture re-upload churn: `CurrentImageDisplay` upload discipline remains one upload per successful decode.

## 12. Explicitly out of scope

- Zoom in/out (Task 2).
- Fit width / fit height modes (Task 3).
- Pan-then-navigate while zoomed (Wave 5).
- Multi-monitor fullscreen selection (`ViewportCommand::SetMonitor`).
- The parked future idea (subfolder picker on image-less root folder).
