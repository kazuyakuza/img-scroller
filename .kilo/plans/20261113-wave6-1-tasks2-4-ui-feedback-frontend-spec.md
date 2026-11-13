# Wave 6.1 Tasks 2–4 Front-end Technical Specification

## Scope and binding decisions

- Source TODO: `.agent/todos/20261113/20261113-todo-1.md` Tasks 2, 3, 4 and the binding operator decisions.
- Global plan: `.kilo/plans/20261113-wave6-1-open-with-feedback.md` Cycle B.
- Landed code under scrutiny:
  - `src/app/open_with_button.rs` — the 11th pill button, upward `egui::Popup::menu`, state gates, two-point enumeration contract.
  - `src/app/button_hint.rs` — caption/tooltip data and the shared button-column wrapper.
  - `src/app/keyboard_intake.rs` — the one per-frame intake in fixed order.
  - `src/app/esc_close_confirm.rs` — ESC double-press close contract.
  - `src/app/floating_controls.rs` — pill row + message line presentation.
  - `src/app/pill_style.rs` — palette/outline constants.
  - `src/app.rs` — `ImagesScrollerApp` state, `open_with_programs` list, chrome gate.
  - `src/app/fullscreen.rs` — `full_ui_chrome_is_visible()` gate.
- Binding operator decisions (FIXED — this spec encodes HOW, not WHETHER):
  - Hotkey = `Q`; the key caption renders UNDER the symbol.
  - While the menu is open: ALL other functionalities OFF; `ESC` or `Q` again closes the menu.
  - The launch-failure message keeps its placement (pill's message line) but renders in a WARNING color.
  - Selecting a program MUST open the current file (D1 fix is separate Cycle A; this spec builds on the fixed launcher surface).

## Verified egui 0.37 API facts (cited)

Facts below are from the published `egui` 0.37.0 docs.rs API surface (`https://docs.rs/egui/0.37.0/egui/containers/struct.Popup.html` and `enum.PopupCloseBehavior.html`, fetched 2026-11-13):

- `egui::Popup::menu(&Response) -> Popup` opens a popup when its widget was clicked, sets a menu layout, and stores the open state in egui `Memory`.
- `Popup::is_open(&self) -> bool` returns whether the popup instance is open.
- `Popup::is_any_open(ctx: &Context) -> bool` returns whether ANY popup is currently open (assumes egui memory tracks popup state).
- `Popup::is_id_open(ctx: &Context, popup_id: Id) -> bool` returns whether the popup with the given `Id` is open.
- `Popup::open_id(ctx: &Context, popup_id: Id)` opens the given popup and closes all others.
- `Popup::close_id(ctx: &Context, popup_id: Id)` closes the given popup if open.
- `Popup::close_all(ctx: &Context)` closes all popups.
- `Popup::id(self, id: Id) -> Self` sets an explicit `Id` on the underlying `Area`.
- `Popup::default_response_id(response: &Response) -> Id` returns the default popup id for a widget response.
- `PopupCloseBehavior::CloseOnClick` — "Popup will be closed on click anywhere, inside or outside. It is used in crate::ComboBox and in crate::containers::menus." This is the default for `Popup::menu`.
- `PopupCloseBehavior::CloseOnClickOutside` — closes only on clicks outside the popup body.
- `PopupCloseBehavior::IgnoreClicks` — clicks ignored; popup closed manually or by pressing escape.

Consequences for this spec:
- A single explicit popup `Id` lets the keyboard intake query and control the menu state without needing the `Response`.
- `Popup::is_id_open` / `Popup::close_id` / `Popup::open_id` are the exact 0.37 APIs to implement modal gating and Q/ESC toggling.
- The default `CloseOnClick` gives item-click and outside-click dismissal for free; no override is required.

## Task 2 — `Q` hotkey + caption under the symbol

### 2.1 Caption data

In `src/app/button_hint.rs`:

- Change `hotkey_reference(PillButton::OpenWith)` from `None` to `Some("Q")`.
- The existing shared caption paint in `show_button_column` will automatically render the 9 pt outlined caption under the open-with glyph because the button flows through the same column helper as the six other keyboard-distinct buttons.
- The caption styling is unchanged: 9 pt (`CAPTION_TEXT_SIZE = 9.0`), fill color from `pill_style::glyph_state_color` (WHITE for enabled, theme-dimmed for disabled), and the 8-offset black outline ring from `pill_style::CAPTION_OUTLINE_OFFSETS`.
- Update the doc comment above `hotkey_reference` that currently says "`None` for open-with too: no hotkey in v1" to reflect the new `Q` binding.

### 2.2 Tooltip wording

- Keep the existing tooltip pattern. The enabled tooltip for OpenWith currently reads `"Open with..."`.
- Follow the established pattern `(Hotkey)` for keyboard-distinct buttons. Change `tooltip_text(PillButton::OpenWith)` to `"Open with... (Q)"`.
- Disabled-state tooltips keep their semantics:
  - Empty enumeration while an image is open: the overridden `"No programs registered for this file type"` tooltip (unchanged).
  - Empty-folder / no-image: the standard `"Open with... (Q)"` tooltip rendered dimmed via `on_disabled_hover_text` (the disabled-tooltip override is `None` in those states).

### 2.3 Q key intake placement

In `src/app/keyboard_intake.rs`:

- Add a constant `OPEN_WITH_MENU_KEY: egui::Key = egui::Key::Q`.
- The Q read participates in the same per-frame intake block as the other chords.
- Placement order: read Q immediately before the existing F11 read. Rationale: the modal gate (see Task 3) is evaluated first; when the menu is open, only Q/ESC close handling runs; when closed, Q is handled before any other action. F11 is the next read so the menu-open guard naturally suppresses it.
- Use the same `was_pressed` helper for the raw Q read as the other plain chords.
- Because a held key would otherwise open/close the menu every frame, the actual dispatch should use a non-repeat filter for Q (same distinct-press semantics as the ESC-close popup). A generic helper is recommended in §3.3.

## Task 3 — Modal menu behavior

### 3.1 Pop-up identity (the single source of truth)

In `src/app/open_with_button.rs`:

- Define a single explicit popup id:
  ```rust
  pub(super) const OPEN_WITH_MENU_ID: egui::Id = egui::Id::new("open_with_menu_popup");
  ```
- Attach it to the `Popup::menu` builder:
  ```rust
  egui::Popup::menu(&button_response)
      .id(OPEN_WITH_MENU_ID)
      .align(egui::RectAlign::TOP)
      .accessible_name(OPEN_WITH_BUTTON_LABEL)
      .show(|menu_ui| app.show_open_with_menu_entries(menu_ui));
  ```
- This id is the stable key for querying and controlling the menu from `keyboard_intake`.

### 3.2 Menu-open query helper

In `src/app/open_with_button.rs` (or `ImagesScrollerApp` impl):

- Add a pure query helper:
  ```rust
  pub(super) fn open_with_menu_is_open(ctx: &egui::Context) -> bool {
      egui::Popup::is_id_open(ctx, OPEN_WITH_MENU_ID)
  }
  ```
- This is the ONE predicate consulted by the modal gate and by the Q-open dispatch.

### 3.3 Recommended modal-gating mechanism

**Recommended option: a single early-return guard in `read_and_dispatch_keystrokes`.**

At the very top of `read_and_dispatch_keystrokes`:

1. Query `menu_is_open = open_with_menu_is_open(screen_context)`.
2. If `menu_is_open`:
   - Read `q_pressed` (non-repeat, see below).
   - Read `non_repeat_escape_pressed` (reuse the existing ESC-close filter).
   - If either is true, call `Popup::close_id(screen_context, open_with_button::OPEN_WITH_MENU_ID)`.
   - Return immediately. No other app functionality runs this frame.
3. Otherwise, continue with the normal intake block.

Why this option:
- It is one centralized gate; there is no risk that a future intake addition forgets to check the menu state.
- It leaves the popup's own keyboard navigation (arrow keys to move between menu items) untouched because it only skips app dispatch; it does not consume egui events.
- It composes cleanly with the existing ESC branch: when the menu is open, the guard returns before `escape_exit_fullscreen_pressed` or `dispatch_esc_close_action` run.

**Non-repeat filter for Q:**

- To avoid held-key flicker, the Q dispatch (both open and close) should use a distinct-press filter.
- Recommended helper (generalize the existing ESC filter or add alongside it):
  ```rust
  pub(super) fn non_repeat_key_press_occurred(events: &[egui::Event], key: egui::Key) -> bool {
      events.iter().any(|event| {
          matches!(
              event,
              egui::Event::Key {
                  key: matched_key,
                  pressed: true,
                  repeat: false,
                  ..
              } if *matched_key == key
          )
      })
  }
  ```
- The existing `non_repeat_escape_press_occurred` can delegate to this helper with `egui::Key::Escape`.
- If the implementer prefers to keep the existing helper verbatim, a separate `non_repeat_q_press_occurred` helper is acceptable; the behavior must be distinct-press only.

### 3.4 Q-open path (menu is closed)

After the modal guard, in the normal intake block:

- If `q_pressed` (non-repeat) AND `self.open_with_menu_is_available()` AND `self.full_ui_chrome_is_visible()`:
  - Refresh the stored program list exactly like the popup-opening click does (the two-point enumeration contract).
  - Open the popup via `egui::Popup::open_id(screen_context, open_with_button::OPEN_WITH_MENU_ID)`.

Implementation note: refactor the current `enumerate_when_menu_opens` refresh into a shared app method, e.g.:

```rust
fn refresh_open_with_programs_and_open_menu(&mut self, ctx: &egui::Context) {
    if let Some(opened_path) = self.opened_image_path.clone() {
        self.open_with_programs = platform::enumerate_open_with_programs(&opened_path);
    }
    egui::Popup::open_id(ctx, open_with_button::OPEN_WITH_MENU_ID);
}
```

The button click path can call this same method instead of its current inline enumeration, ensuring Q and click behave identically.

### 3.5 ESC-while-open routing

When the menu is open, the modal guard intercepts ESC before any other ESC branch:

- `Popup::close_id(ctx, OPEN_WITH_MENU_ID)` is called.
- The guard returns immediately.
- Therefore:
  - `escape_exit_fullscreen_pressed` never evaluates to true (fullscreen is OFF anyway when the menu can be open, but the guard is defense-in-depth).
  - `dispatch_esc_close_action` is not reached, so the ESC-close confirm popup is NOT armed and the application does NOT close.

This satisfies the binding: ESC closes the menu AND NOTHING ELSE.

### 3.6 Q-again-close routing

When the menu is open and Q is pressed (non-repeat):

- The modal guard closes the popup via `Popup::close_id`.
- It returns immediately.
- No other action runs.

This satisfies the binding: Q again closes the menu.

### 3.7 All-other-functionalities OFF while open

Because the modal guard returns early from `read_and_dispatch_keystrokes`, the following are suppressed while the menu is open:

- F11 fullscreen toggle.
- ESC fullscreen exit.
- ESC-close confirm popup arming/closing.
- Zoom in/out (`I`/`O`/Shift aliases).
- Restore fit (`0`).
- Fit width/height (`W`/`H`).
- Arrow keys (pan and navigation).

The pill buttons still render, but their click handlers are layout-level and do not fire while the popup is capturing pointer input over them. The popup's own item buttons still fire because they are inside the popup's `.show()` closure.

### 3.8 Outside-click dismissal

- Use the `Popup::menu` default close behavior (`PopupCloseBehavior::CloseOnClick`).
- This closes the menu when the user clicks anywhere: on a menu item (after the item's click handler runs), on the popup background, or outside the popup.
- No explicit `.close_behavior(...)` override is required.
- The binding decision to accept the egui popup default is therefore: **default `CloseOnClick` is the chosen behavior**.

### 3.9 State gates interplay

- Q is disabled when the button is disabled: the Q-open dispatch checks `open_with_menu_is_available()`, which requires `image_is_open() && !open_with_programs.is_empty()`. The disabled states (fresh empty, empty-folder browsed, image open with empty list) all fail this predicate, so Q does nothing.
- The menu cannot open in fullscreen ON: the Q-open dispatch additionally checks `full_ui_chrome_is_visible()` (`!fullscreen_on`). The pill button is not rendered while fullscreen is ON, so click-opening is also impossible.
- The menu cannot open in fresh-empty or empty-folder mode: `image_is_open()` is false, so `open_with_menu_is_available()` is false.
- While the menu is open, fullscreen toggle is suppressed by the modal guard, so the menu cannot be carried into fullscreen.

## Task 4 — Warning-colored failure message

### 4.1 Color choice

- Recommended constant in `src/app/pill_style.rs`:
  ```rust
  pub(super) const WARNING_MESSAGE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 165, 0);
  ```
  (orange; classic warning semantic, high luminance, readable on the pill's translucent dark frame `(20,20,20,180)` and over typical image backgrounds).
- Alternative considered: `Color32::from_rgb(255, 215, 0)` (gold). Gold is slightly brighter but less universally "warning". **Orange is the recommended choice.**

### 4.2 Message-kind state

In `src/app.rs`:

- Add a boolean flag to `ImagesScrollerApp`:
  ```rust
  message_text_is_warning: bool,
  ```
- Initialize to `false` in `empty_state()`.
- Set it to `false` when assigning the unsupported-file message and when clearing the message.
- Set it to `true` only when assigning `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` in `launch_open_with_program`.

This keeps the change scoped: only the open-with failure message is colored; the unsupported-file message and any future messages keep the default egui text color.

### 4.3 Message-line paint

In `src/app/floating_controls.rs`:

- Change `show_message_line` signature to accept the warning flag:
  ```rust
  fn show_message_line(
      message: Option<&'static str>,
      is_warning: bool,
      pill_ui: &mut egui::Ui,
  ) {
      let text = message.unwrap_or("");
      let color = if is_warning {
          pill_style::WARNING_MESSAGE_COLOR
      } else {
          pill_ui.visuals().text_color()
      };
      pill_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |line_ui| {
          line_ui.label(egui::RichText::new(text).size(MESSAGE_TEXT_SIZE).color(color));
      });
  }
  ```
- Update the call site in `show_pill_contents` to pass `app.message_text_is_warning`.

### 4.4 Scope preservation

- The unsupported-file message (`image_loading::UNSUPPORTED_FILE_MESSAGE`) must continue to render in the default egui text color.
- No other message surfaces are affected.
- The message placement (12 pt label directly above the pill button row) is unchanged.

## Expected unit-test additions

Current census: 177 unit tests / 28 test files.

Add the following pure-logic tests (no GUI context required):

1. `src/app/button_hint_tests.rs` — update existing `hotkey_reference_matches_the_spec`:
   - Change the `OpenWith` assertion from `None` to `Some("Q")`.
   - Change the `OpenWith` tooltip assertion from `"Open with..."` to `"Open with... (Q)"`.
   - These are edits, not additions; test count unchanged.

2. `src/app/button_hint_tests.rs` — add one new test:
   - Assert that `hotkey_reference(OpenWith)` returns a 1-character string `"Q"` (pins the caption data for the operator decision).

3. `src/app/open_with_button_tests.rs` — add tests:
   - `q_does_not_open_menu_when_fullscreen_on`: build an app with an image open and programs listed, set `fullscreen_on = true`, assert that the Q-open predicate (`open_with_menu_should_open_on_q`) is false.
   - `q_does_not_open_menu_when_no_programs_listed`: image open but empty list, assert false.
   - `open_with_failure_message_is_warning_flagged`: call `launch_open_with_program(0)` and assert `message_text_is_warning == true`.
   - `unsupported_file_message_is_not_warning_flagged`: simulate an unsupported load and assert `message_text_is_warning == false`.

4. `src/app/pill_style_tests.rs` — add one test:
   - Assert `WARNING_MESSAGE_COLOR` is not equal to `PILL_GLYPH_COLOR` or `OUTLINE_COLOR` and that its alpha is `255` (fully opaque warning color).

Expected census delta: **+3 to +5 unit tests**, all in existing test files; **test-file count stays 28**.
Recommended pinned target: **180 unit tests / 28 test files**.

## 4.5a conformance checklist

Use this checklist during front-end verification (step 4.5a):

- [ ] `button_hint::hotkey_reference(PillButton::OpenWith)` returns `Some("Q")`.
- [ ] `button_hint::tooltip_text(PillButton::OpenWith)` includes the `(Q)` hotkey reference.
- [ ] The open-with button renders the 9 pt outlined caption under the glyph using the existing `pill_style` helpers.
- [ ] `src/app/open_with_button.rs` defines `OPEN_WITH_MENU_ID` and attaches it to `Popup::menu` via `.id(...)`.
- [ ] `keyboard_intake.rs` has an early-return modal guard that queries `Popup::is_id_open(ctx, OPEN_WITH_MENU_ID)`.
- [ ] While the menu is open, only Q and ESC are read; all other intake branches are skipped.
- [ ] Q/ESC while the menu is open call `Popup::close_id` and return immediately.
- [ ] ESC while the menu is open does NOT arm the ESC-close confirm popup and does NOT send `ViewportCommand::Close`.
- [ ] The Q-open path checks `open_with_menu_is_available() && full_ui_chrome_is_visible()` before opening.
- [ ] The Q-open path refreshes `open_with_programs` and calls `Popup::open_id` (same two-point enumeration contract as the click).
- [ ] The click path and the Q path use the same shared refresh-and-open helper.
- [ ] `Popup::menu` uses the default `CloseOnClick` close behavior (item-click and outside-click dismissal).
- [ ] `ImagesScrollerApp` carries `message_text_is_warning: bool`, initialized to `false`, set `true` only on open-with launch failure, and reset on unsupported-file / clear.
- [ ] `floating_controls::show_message_line` applies `pill_style::WARNING_MESSAGE_COLOR` only when `is_warning == true`; unsupported-file message keeps the default text color.
- [ ] The warning color constant is `Color32::from_rgb(255, 165, 0)` in `pill_style.rs`.
- [ ] No per-frame texture re-upload or allocation churn is introduced.
- [ ] Test census matches the expected target (180/28 or explained delta).
- [ ] The implementer did not introduce architectural decisions beyond this spec (no new state surfaces, no changed popup mechanism, no new message surfaces).

## Open questions / none

None. All binding decisions are fixed; this spec encodes the exact mechanism choices for the 4.1b architector step.
