# Wave 6.1 Tasks 2+3+4 — Implementation Plan (Critical Workflow step 4.1b, architector)

- Source TODO: `.agent/todos/20261113/20261113-todo-1.md` — Tasks 2 (Q hotkey + caption), 3 (modal menu), 4 (warning message) + the Binding operator decisions + Deferred decisions.
- Front-end spec (INPUT, not contract): `.kilo/plans/20261113-wave6-1-tasks2-4-ui-feedback-frontend-spec.md`.
- Global plan: `.kilo/plans/20261113-wave6-1-open-with-feedback.md` — Cycle B (4.2 implementer → 4.3 review → 4.4 docs → 4.5a/4.5b → 4.6).
- Base state: branch `fix/wave-6-1-open-with-feedback`, version 0.6.1, HEAD past the Cycle-A D1 fix, census 177 unit tests / 28 test files, `app.rs` EXACTLY 200 lines (the cap).
- The binding operator decisions are FIXED (not re-decided here): hotkey = `Q`; caption UNDER the symbol; while the menu is open ALL other functionalities OFF; `ESC` or `Q` again closes the menu; the failure message keeps its placement but renders in a WARNING color; selecting a program opens the current file (D1, Cycle A).
- This plan implements ONLY Tasks 2+3+4 on top of the landed (and Cycle-A-fixed) launcher surface.

## 0. Execution frame

- All Rust/Cargo runs in the container on the Alpine VM via the `alpine-vm` MCP: call `vm_status` FIRST; the command exit code is authoritative; VM/`/img-scroller` unavailable → STOP + user question (docs/build.md).
- No Cargo.toml / Cargo.lock changes (zero new dependencies — everything used is already in egui 0.37.0). No `CHANGELOG.md`. No edits to existing `.kilo/plans/*` files.
- No new source files and NO new test files (test-file census must stay 28): every edit lands in an existing module, and every new test in an existing test module.
- Code rules in force: files ≤200 lines; method bodies ≤50 lines; ≤2 params (receiver not counted); private by default; no commented-out code; single-section boolean conditions; named constants; self-documenting names.
- Line-cap budget (pre → expected post, ±fmt wrapping; if a file drifts over 200 after `cargo fmt`, apply ONLY the compressions pinned in this plan):

| File | Pre | Post (expected) | Delta cause |
|---|---|---|---|
| `src/app.rs` | 200 | 200 (UNTOUCHED) | the warning flag is NOT an app field — see deviation D3 |
| `src/app/open_with_button.rs` | 199 | ~171 | glyph block OUT, `enumerate_when_menu_opens` deleted, module doc trimmed, new menu-state API in |
| `src/app/pill_style.rs` | 146 | ~197 | glyph block IN, `WARNING_MESSAGE_COLOR` + `message_text_color` |
| `src/app/keyboard_intake.rs` | 140 | ~167 | modal guard + Q read/dispatch |
| `src/app/button_hint.rs` | 185 | ~186 | OpenWith caption/tooltip arms |
| `src/app/floating_controls.rs` | 186 | ~194 | message-line color + arrow gate |
| `src/app/zoom_button.rs` | 186 | ~191 | click gate |
| `src/app/fit_button.rs` | 155 | ~160 | click gate |
| `src/app/fullscreen_button.rs` | 48 | ~56 | click gate |
| `src/app/esc_close_confirm.rs` | 184 | ~183 | generalized non-repeat filter + delegation |
| test modules | 35/54/180/88 | 35/~85/~188/~170 | button_hint edits; +2 pill_style; +1 esc_close_confirm; +9 open_with_button |

## 1. Verified egui 0.37.0 API facts (source-cited, docs.rs `src/egui/containers/popup.rs.html` and `src/egui/id.rs.html`, fetched 2026-11-13)

The spec's pinned APIs all EXIST in 0.37.0, with two behavioral facts the spec did not know (F9, F10) and one compile fact it got wrong (F11):

1. `Popup::is_id_open(ctx: &Context, popup_id: Id) -> bool` — `ctx.memory(|mem| mem.is_popup_open(popup_id))` (popup.rs:736-738). Compatible with `Popup::menu` (doc: "assumes the use of either open_memory / from_toggle_button_response / menu / context_menu").
2. `Popup::open_id(ctx: &Context, popup_id: Id)` — `ctx.memory_mut(|mem| mem.open_popup(popup_id))`; opens the popup and (single-slot map) closes all others (popup.rs:747-750).
3. `Popup::close_id(ctx: &Context, popup_id: Id)` — `ctx.memory_mut(|mem| mem.close_popup(popup_id))`; no-op if a different popup is open (popup.rs:764-769).
4. `Popup::toggle_id`, `Popup::close_all`, `Popup::is_any_open`, `Popup::default_response_id(response)` — all present (popup.rs:722-774).
5. `Popup::id(self, id: Id) -> Self` — sets the Area id; `is_id_open` doc: "The popup id should be the same as either you set with Self::id or the default one from Self::default_response_id" (popup.rs:443-448). The builder override is the binding key.
6. `PopupCloseBehavior::CloseOnClick` is `#[default]` (popup.rs:76-91); `Popup::menu` = `from_toggle_button_response` + `PopupKind::Menu` + menu style (popup.rs:244-250). Item-click AND outside-click dismissal come free; NO `.close_behavior(...)` override.
7. F9 — **`Popup::menu` TOGGLES the memory state on the click frame** (popup.rs:237-240, 556-577: `open_memory(button_response.clicked().then_some(SetOpenCommand::Toggle))` → `Self::toggle_id(&ctx, id)` inside `show()`). Consequence: an explicit `Popup::open_id` call BEFORE the same-frame `Popup::menu(...).show()` would be immediately toggled back CLOSED. The Q-open and the click-open MUST use different open mechanisms (deviation D1).
8. F10 — **the close-on-click logic does NOT consume the click** (popup.rs:659-699: it only mutates memory / sets `response.set_close()`). A click on ANY widget outside the popup rect (e.g. the F11 pill button) reaches that widget's handler normally while the menu closes. The spec §3.7 claim ("pill buttons do not fire while the popup is capturing pointer input over them") is TRUE only for clicks over the popup rect; buttons elsewhere in the pill row WOULD still fire. Explicit click-dispatch gates are REQUIRED (deviation D2).
9. `Memory::keep_popup_open(id)` is a no-op for a closed popup and marks the matching open entry alive otherwise (memory/mod.rs:1131-1137) — so the intake's `close_id` is NOT resurrected by the same-frame builder call, and a popup opened via `open_id` stays alive while its builder runs every frame. egui also natively closes the menu on `key_pressed(Escape)` inside `Popup::show` (popup.rs:692-694) — idempotent with our guard's `close_id`.
10. F11 — **`egui::Id::new` is DEPRECATED in 0.37** ("Use `Id::unique` ... instead") and is NOT a `const fn` (id.rs:100-104: `pub fn unique(source: impl Hash + Debug) -> Self`, runtime `ahash::RandomState::with_seeds(1,2,3,4)` hashing). The spec §3.1 `pub(super) const OPEN_WITH_MENU_ID: egui::Id = egui::Id::new(...)` would not compile as a `const` and would fail clippy `-D warnings`. Replacement (deviation D4): a plain function returning `Id::unique("open_with_menu_popup")` — deterministic (fixed seeds → the same Id value every call, every frame, every run).
11. `Context::input` / context memory reads/writes outside a pass are safe (the landed `esc_close_expiry_tests` call `context.input(...)` outside `run_pass`), so tests may call `Popup::open_id` / `Popup::is_id_open` / `Popup::close_id` directly on a harness `Context`.

## 2. Pinned decisions and spec deviations (behavior unchanged; the binding decisions are the contract)

- **D1 (vs spec §3.4)**: the click path and the Q path do NOT share one refresh-AND-open helper. Shared = the enumeration refresh only (`refresh_open_with_programs`); the click opens via `Popup::menu`'s own click Toggle (landed mechanism, unchanged), the Q press opens via `Popup::open_id`. Reason: fact F9 — a shared helper calling `open_id` on the click frame would be toggled closed by the same-frame builder, breaking the landed click-open. The operator-visible behavior is identical (Q opens exactly like a click).
- **D2 (vs spec §3.7)**: every pill-row click DISPATCH gains an explicit open-with-menu gate (arrows, fullscreen, zoom ×3, fit ×2, open-with refresh). Reason: fact F10 — the binding "ALL other functionalities are OFF" includes buttons, and egui does not consume outside clicks.
- **D3 (vs spec §4.2)**: NO `message_text_is_warning: bool` app field. The warning discrimination is the pure predicate `message_line_is_warning(message)` = `message == Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)` (content equality of `&'static str`s). Reasons: (a) `app.rs` is exactly at the 200-line cap — the field + init + resets would force structural compression with zero headroom; (b) the flag would need maintenance at FIVE message-write sites (`app.rs` empty-state/Ok/Err, `folder_navigation.rs:108` empty-folder clear — a site the spec missed — and `open_with_button.rs` failure), a drift risk; (c) the failure text is already a unique constant, so equality is lossless, stateless, and testable. Behavior identical: only the open-with failure message is warning-colored.
- **D4 (vs spec §3.1)**: `open_with_menu_id()` function instead of the `const` (fact F11).
- **D5 (vs spec §2.3)**: the Q read is ONLY the generalized non-repeat event filter (`esc_close_confirm::non_repeat_key_press_occurred(events, OPEN_WITH_MENU_KEY)`); no separate raw `was_pressed(ctx, Q)` read (it would be dead code). This mirrors the landed ESC-close filter exactly. The generic filter lives in `esc_close_confirm.rs` and `non_repeat_escape_press_occurred` delegates to it (spec §3.3's recommendation).
- **D6 (vs spec §3.3 step placement)**: the modal guard sits AFTER `self.update_esc_close_popup(screen_context)`, which moves from its current mid-function position to the FIRST line of `read_and_dispatch_keystrokes`. Reason: the Wave-5.1 expiry-wake contract must keep evaluating while the menu is open, or an ESC-close popup armed just before the menu opened would freeze on screen behind the menu; the update consumes no events and dispatches nothing, so "all other functionalities OFF" is preserved. Everything after the guard (all chord reads + dispatches) is skipped while the menu is open.
- **D7 (same-frame Q+F11 resolution)**: the Q-open dispatch takes priority over the F11 toggle via `if q_should_open { ... } else if fullscreen_pressed { ... }` — a same-frame co-press cannot open the menu and enter fullscreen in one frame (preserves spec §3.9's "the menu cannot be carried into fullscreen"). Each `if` stays single-section.
- **D8 (builder gate hardening)**: the popup builder's gate changes from `if self.enabled` to `if self.menu_popup_is_wanted(row_ui)` = `self.enabled || open_with_menu_is_open(row_ui.ctx())`. Reason: if a fresh re-enumeration empties the stored list AFTER the menu was opened (Q-open frame), the old gate would skip the builder while the memory state stays open → an INVISIBLE modal lock. `||`-in-return matches the landed `open_with_menu_is_available` precedent.
- **D9**: `OPEN_WITH_MENU_KEY` lives in `open_with_button.rs` (not `keyboard_intake.rs` as spec §2.3 proposed) — the established per-feature key-constant home (`fullscreen.rs`, `zoom.rs` host their keys).
- **D10 (cap management)**: the open-with glyph composite (`open_with_glyph_shapes` + its 8 geometry constants) moves from `open_with_button.rs` to `pill_style.rs` (the painted pill-glyph builder home; `outlined_rect_shape`/`outlined_triangle_shape` already live there). No new module → `app.rs` untouched.
- **D11 (census)**: the spec estimated "+3..+5" but its own list totals 6, and the TODO Task 5 explicitly requires tests for "hotkey intake gating" and "menu-open state gating" (the spec listed none). PINNED: **189 unit tests / 28 test files** (177 + 9 open-with + 2 pill_style + 1 esc_close_confirm). The spec's extra "1-character Q caption" test is dropped (a near-duplicate of the edited `hotkey_reference_matches_the_spec` assertion).
- **D12**: `show_message_line` KEEPS its 2-parameter signature `(message, pill_ui)` — the warning branch is computed inside from the message itself, so no parameter object is needed (the ≤2-param rule holds unchanged).

## 3. File-by-file edit steps (atomic; implement in this order)

### Step 3.1 — `src/app/esc_close_confirm.rs`: generalize the distinct-press filter

Replace the body of `non_repeat_escape_press_occurred` (lines 77-89) with a delegation and add the generic filter above it:

```rust
/// True exactly for a NON-REPEAT pressed `key` event in this frame's events
/// (the generic distinct-press filter; the ESC-close filter delegates).
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

/// True exactly for a first-of-two DISTINCT ESC presses: a pressed Escape key
/// event that is NOT a held-key autorepeat (spec §4.2; `key_pressed` alone
/// would also fire on repeats).
pub(super) fn non_repeat_escape_press_occurred(events: &[egui::Event]) -> bool {
    non_repeat_key_press_occurred(events, egui::Key::Escape)
}
```

All existing callers (`keyboard_intake.rs`) and tests keep compiling unchanged.

### Step 3.2 — `src/app/open_with_button.rs`: menu identity, Q-open, refresh contract, warning predicate

1. **Module doc**: replace the 15-line header (lines 1-15) with ~6 lines recording: the 11th pill button; the `Q` hotkey with the caption under the glyph; the MODAL upward menu (all other functionalities OFF while open; `ESC`/`Q` closes); the warning-colored failure message via `message_line_is_warning`; the plans pointer `.kilo/plans/20261113-wave6-1-*`.
2. **Delete** the 8 glyph geometry constants (lines 31-38: `OPEN_WITH_SQUARE_SIDE` … `OPEN_WITH_ARROW_HEAD_ROTATION`) and the `open_with_glyph_shapes` fn + doc (lines 121-153) — they move to `pill_style.rs` (Step 3.3). The `paint_glyph` call site becomes `pill_style::open_with_glyph_shapes(glyph_center, glyph_color)`.
3. **Add** the menu identity + key constants (module level, near `NO_PROGRAMS_TOOLTIP_TEXT`):

```rust
/// The menu popup's stable identity: the ONE id the intake guard, the Q
/// toggle, and the row gates consult (`Id::unique` hashes with FIXED seeds,
/// so every call yields the same id; `Id::new` is deprecated in egui 0.37).
pub(super) fn open_with_menu_id() -> egui::Id {
    egui::Id::unique("open_with_menu_popup")
}

/// The open-with button's keyboard hotkey (Wave-6.1 Task 2; read by the
/// intake's non-repeat filter for BOTH the open and the close toggle).
pub(super) const OPEN_WITH_MENU_KEY: egui::Key = egui::Key::Q;

/// True while the open-with menu popup is open (the ONE modal-state query:
/// the intake guard AND the row click gates consult it).
pub(super) fn open_with_menu_is_open(screen_context: &egui::Context) -> bool {
    egui::Popup::is_id_open(screen_context, open_with_menu_id())
}
```

4. **Rewrite** `show_and_open_menu` (delete `enumerate_when_menu_opens`, lines 95-109; `accepts_click` gains the row context and the menu gate):

```rust
fn show_and_open_menu(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    let placement =
        PillButtonPlacement::for_button(PillButton::OpenWith, OPEN_WITH_BUTTON_SIZE)
            .with_enabled(self.enabled)
            .with_disabled_tooltip(no_programs_tooltip_for(app));
    let button_response =
        show_button_column(ButtonColumnSite::new(row_ui, placement), |column_ui| {
            self.allocate_and_paint(column_ui)
        });
    if self.accepts_click(&button_response, row_ui) {
        app.refresh_open_with_programs();
    }
    if self.menu_popup_is_wanted(row_ui) {
        egui::Popup::menu(&button_response)
            .id(open_with_menu_id())
            .align(egui::RectAlign::TOP)
            .accessible_name(OPEN_WITH_BUTTON_LABEL)
            .show(|menu_ui| app.show_open_with_menu_entries(menu_ui));
    }
}

/// A disabled open-with button never fires, and a click while the menu is
/// open only closes it via the popup's own toggle (no refresh re-run).
fn accepts_click(&self, button_response: &egui::Response, row_ui: &egui::Ui) -> bool {
    self.enabled
        && button_response.clicked()
        && !open_with_menu_is_open(row_ui.ctx())
}

/// The popup builder's gate: the painted enablement, OR an already-open
/// popup (once open it renders every frame until closed, even if a fresh
/// enumeration would flip the enablement — no invisible modal lock).
fn menu_popup_is_wanted(&self, row_ui: &egui::Ui) -> bool {
    self.enabled || open_with_menu_is_open(row_ui.ctx())
}
```

5. **Add** to the `impl ImagesScrollerApp` block:

```rust
/// True exactly when a distinct Q press should OPEN the menu: the press, the
/// button's enablement (image open AND programs listed), and the pill chrome
/// visible (fullscreen ON hides the button; a disabled state stays inert).
pub(super) fn open_with_menu_should_open_on_q(&self, q_pressed: bool) -> bool {
    q_pressed && self.open_with_menu_is_available() && self.full_ui_chrome_is_visible()
}

/// The refresh contract's Q-open point: re-enumerate for the CURRENT file,
/// then open the popup programmatically (the click path opens via the
/// Popup::menu builder's own click toggle — popup.rs:237-240, 556-577).
pub(super) fn open_with_menu_from_q_press(&mut self, screen_context: &egui::Context) {
    self.refresh_open_with_programs();
    egui::Popup::open_id(screen_context, open_with_menu_id());
}

/// Re-enumerates the programs registered for the CURRENT canonical file into
/// the stored list (the refresh contract: every successful open, the
/// popup-opening click, and the Q-open press).
pub(super) fn refresh_open_with_programs(&mut self) {
    let Some(opened_path) = self.opened_image_path.clone() else {
        return;
    };
    self.open_with_programs = platform::enumerate_open_with_programs(&opened_path);
}
```

(`launch_open_with_program` is UNCHANGED — the failure message assignment stays; the warning color is derived at paint time.)

6. **Add** the warning predicate (module level, next to `no_programs_tooltip_for`):

```rust
/// True exactly while the message line shows the open-with launch failure —
/// the ONLY warning-colored message (Wave-6.1 Task 4): the failure text is a
/// unique constant, so equality with it is the lossless discriminator; the
/// unsupported-file message and cleared states keep the default color.
pub(super) fn message_line_is_warning(message: Option<&'static str>) -> bool {
    message == Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)
}
```

### Step 3.3 — `src/app/pill_style.rs`: warning color, message color fn, glyph block in

1. Add after the palette consts (`OUTLINE_STROKE_WIDTH` / `CAPTION_TEXT_SIZE` block):

```rust
/// The open-with launch-failure message color: classic warning ORANGE,
/// high-luminance and readable on the pill's translucent dark frame
/// `(20,20,20,180)` (Wave-6.1 Task 4; the ONLY warning-colored message).
pub(super) const WARNING_MESSAGE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 165, 0);

/// The message-line text color: the warning color for the open-with launch
/// failure, the theme's default text color for every other message state.
pub(super) fn message_text_color(is_warning: bool, pill_ui: &egui::Ui) -> egui::Color32 {
    if is_warning {
        return WARNING_MESSAGE_COLOR;
    }
    pill_ui.visuals().text_color()
}
```

2. Append at the end of the file (moved VERBATIM from `open_with_button.rs`, with `pill_style::` references now unqualified): the 8 constants (`OPEN_WITH_SQUARE_SIDE`, `OPEN_WITH_SQUARE_CENTER_OFFSET`, `OPEN_WITH_ARROW_LENGTH`, `OPEN_WITH_ARROW_TRIANGLE_SIZE`, `OPEN_WITH_ARROW_SHAFT_THICKNESS`, `OPEN_WITH_ARROW_SHAFT_OUTLINE_WIDTH`, `OPEN_WITH_ARROW_HEAD_ROTATION` — keep their values and `pub(super)` visibility) and:

```rust
/// The open-with button's composite glyph (a 10x10 square with a 7-point
/// arrow from its top-right corner pointing up-right: shaft plus triangle
/// head; paint order square, shaft BLACK under-stroke then glyph stroke,
/// head; the square shifts down-left) — the painted pill-glyph builder home.
pub(super) fn open_with_glyph_shapes(
    glyph_center: egui::Pos2,
    glyph_color: egui::Color32,
) -> Vec<egui::Shape> {
    // body verbatim from open_with_button.rs:128-152, `pill_style::` prefixes dropped
}
```

### Step 3.4 — `src/app/button_hint.rs`: the Q caption + tooltip (Task 2)

1. `hotkey_reference`: move `PillButton::OpenWith` out of the `None` arm into its own arm `PillButton::OpenWith => Some("Q"),` (place it before the `Fullscreen` arm). Update the fn's doc comment: the four arrows stay `None`; OpenWith is now `Some("Q")` (the Wave-6.1 operator decision; the "no hotkey in v1" note is removed).
2. `tooltip_text`: `PillButton::OpenWith => "Open with... (Q)",` (the established `(Hotkey)` wording pattern).
3. NO other changes: the shared `show_button_column` caption paint automatically renders the 9 pt outlined `Q` under the glyph because the button already flows through `show_button_column` with `with_enabled(self.enabled)` (button_hint.rs:168-180 — WHITE fill when enabled, theme-dimmed when disabled, `CAPTION_OUTLINE_OFFSETS` ring).
4. Disabled-tooltip semantics UNCHANGED: the override `NO_PROGRAMS_TOOLTIP_TEXT` still applies only while a file is open with an empty enumeration (`no_programs_tooltip_for`); empty-folder mode keeps the standard `"Open with... (Q)"` tooltip dimmed via `on_disabled_hover_text`.

### Step 3.5 — `src/app/keyboard_intake.rs`: the modal guard + Q dispatch (Task 3 core)

1. Add the import: `use super::open_with_button;` (alphabetical: after `fullscreen`, before `pan`).
2. Rewrite `read_and_dispatch_keystrokes` — the guard FIRST (after the expiry update), the Q read before the F11 read, the Q-open dispatch winning over F11 (deviation D7):

```rust
pub(super) fn read_and_dispatch_keystrokes(&mut self, screen_context: &egui::Context) {
    self.update_esc_close_popup(screen_context);
    if self.handle_modal_open_with_menu_keys(screen_context) {
        return;
    }
    let q_pressed = screen_context.input(|input| {
        esc_close_confirm::non_repeat_key_press_occurred(
            &input.events,
            open_with_button::OPEN_WITH_MENU_KEY,
        )
    });
    let fullscreen_pressed = was_pressed(screen_context, fullscreen::FULLSCREEN_TOGGLE_KEY);
    // … every existing read below is UNCHANGED and stays in its current order:
    //   escape_exit_fullscreen_pressed, non_repeat_escape_pressed, zoom chords,
    //   restore-fit, fit W/H, the four arrows + their non-repeat reads …
    if self.open_with_menu_should_open_on_q(q_pressed) {
        self.open_with_menu_from_q_press(screen_context);
    } else if fullscreen_pressed {
        self.toggle_fullscreen_and_send_command(screen_context);
    }
    // … every existing dispatch below is UNCHANGED: ESC-exit/else, zoom,
    //   fit, arrows …
}
```

- `self.update_esc_close_popup(screen_context)` MOVES from its current position (after the reads) to the first line (deviation D6). The old call site is deleted.
- The `if fullscreen_pressed {` block is restructured into the `else if` above (single-section conditions; behavior when Q is not pressed is identical).

3. Add the guard method (deviations D5, D6; two separate single-section `if`s instead of an `||`):

```rust
/// The modal gate (Wave-6.1 Task 3): while the open-with menu is open ONLY
/// the Q toggle and the ESC close run this frame — every other app action is
/// skipped, the ESC-close contract cannot arm, and the fullscreen exit never
/// fires. Returns true when the menu was open (the caller skips the normal
/// intake; the reads never consume events, so the menu's own keyboard
/// navigation keeps working).
fn handle_modal_open_with_menu_keys(&mut self, screen_context: &egui::Context) -> bool {
    if !open_with_button::open_with_menu_is_open(screen_context) {
        return false;
    }
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
}
```

### Step 3.6 — `src/app/floating_controls.rs`: message color (Task 4) + arrow gate (Task 3)

1. `show_pill_contents` call site UNCHANGED: `show_message_line(message, pill_ui);`.
2. Rewrite `show_message_line` (keeps 2 params — deviation D12):

```rust
/// Shows the message line: the open-with launch failure renders in the
/// warning color (Wave-6.1 Task 4), every other state the default color.
fn show_message_line(message: Option<&'static str>, pill_ui: &mut egui::Ui) {
    let text_color = pill_style::message_text_color(
        open_with_button::message_line_is_warning(message),
        pill_ui,
    );
    pill_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |line_ui| {
        line_ui.label(
            egui::RichText::new(message.unwrap_or(""))
                .size(MESSAGE_TEXT_SIZE)
                .color(text_color),
        );
    });
}
```

3. Gate the arrow dispatch: `ArrowButtonSpec::accepts_click` gains the row context:

```rust
/// A disabled arrow never navigates (a disabled arrow stays handler-free);
/// the open-with menu also suppresses every row dispatch (Wave-6.1 Task 3).
fn accepts_click(&self, button_response: &egui::Response, row_ui: &egui::Ui) -> bool {
    self.enabled
        && button_response.clicked()
        && !open_with_button::open_with_menu_is_open(row_ui.ctx())
}
```

Call site in `click_direction`: `self.accepts_click(button_response, row_ui)`. (`open_with_button` is already imported in this module.)

### Step 3.7 — `src/app/zoom_button.rs`, `src/app/fit_button.rs`, `src/app/fullscreen_button.rs`: row click gates

1. `zoom_button.rs` (add `use super::open_with_button;`): `ZoomButtonSpec::accepts_click` gains `row_ui: &egui::Ui` + the same `&& !open_with_button::open_with_menu_is_open(row_ui.ctx())` clause; call site `self.accepts_click(&button_response, row_ui)`.
2. `fit_button.rs` (add the import): same signature change on `FitButtonSpec::accepts_click`; the body becomes `self.enabled_click_would_fire(button_response) && !open_with_button::open_with_menu_is_open(row_ui.ctx())`; call site gains the arg.
3. `fullscreen_button.rs` (add the import): the dispatch is gated by a named predicate (this module has no spec struct):

```rust
/// The click toggles only while the open-with menu is closed (the modal
/// menu suppresses the row's dispatches; Wave-6.1 Task 3).
fn click_toggles_fullscreen(painted_button: &egui::Response, row_ui: &egui::Ui) -> bool {
    painted_button.clicked() && !open_with_button::open_with_menu_is_open(row_ui.ctx())
}
```

Call site: `if click_toggles_fullscreen(&painted_button, row_ui) {`.

### Step 3.8 — `src/app.rs`: NO CHANGES

The warning flag is not an app field (deviation D3); no new modules; no new test modules. The file stays at its 200-line cap.

## 4. Tests (exact names; all in EXISTING test modules; pinned census 189 / 28)

### `src/app/button_hint_tests.rs` — 2 EDITS, count unchanged

- `hotkey_reference_matches_the_spec`: `assert_eq!(hotkey_reference(OpenWith), Some("Q"));` (replaces the `None` assertion).
- `tooltip_text_matches_the_spec`: `assert_eq!(tooltip_text(OpenWith), "Open with... (Q)");`.

### `src/app/esc_close_confirm_tests.rs` — +1 test (180 → ~188 lines)

```rust
#[test]
fn non_repeat_key_press_occurred_filters_repeats_and_foreign_keys() {
    let q_press = |repeat: bool| { /* egui::Event::Key { key: Key::Q, physical_key: Some(Key::Q), pressed: true, repeat, modifiers: default } */ };
    assert!(non_repeat_key_press_occurred(&[q_press(false)], egui::Key::Q));
    assert!(!non_repeat_key_press_occurred(&[q_press(true)], egui::Key::Q));
    assert!(!non_repeat_key_press_occurred(&[], egui::Key::Q));
    // the ESC filter is the delegation: identical results on an ESC event
    assert_eq!(
        non_repeat_escape_press_occurred(&[escape_key_event(true, false)]),
        non_repeat_key_press_occurred(
            &[escape_key_event(true, false)],
            egui::Key::Escape
        )
    );
}
```

(Exact builder shape: copy the module's `escape_key_event` pattern generalized to a `key` parameter.)

### `src/app/pill_style_tests.rs` — +2 tests (54 → ~85 lines)

1. `warning_message_color_is_distinct_and_fully_opaque` — `assert_eq!(WARNING_MESSAGE_COLOR, egui::Color32::from_rgb(255, 165, 0));` + `assert_ne!(WARNING_MESSAGE_COLOR, PILL_GLYPH_COLOR);` + `assert_ne!(WARNING_MESSAGE_COLOR, OUTLINE_COLOR);` + `assert_eq!(WARNING_MESSAGE_COLOR.a(), 255);`.
2. `message_text_color_selects_the_warning_and_theme_colors` — the landed `run_pass` + `Area::show` pattern (pill_style_tests.rs:34-53): inside the test Ui assert `message_text_color(true, ui) == WARNING_MESSAGE_COLOR` and `message_text_color(false, ui) == ui.visuals().text_color()`.

### `src/app/open_with_button_tests.rs` — +9 tests (88 → ~170 lines)

Local intake harness (the proven `esc_close_expiry_tests.rs` long-lived-Context shape):

```rust
struct MenuIntakeHarness {
    app: ImagesScrollerApp,
    context: egui::Context,
}

impl MenuIntakeHarness {
    /// Image open + the two-program fixture STORED (the availability state).
    fn with_image_open_and_programs_stored() -> Self { /* empty_state + accept_open_outcome(photo.png, Ok) + open_with_programs = two_programs_fixture() */ }

    fn run_pass(&mut self, events: Vec<egui::Event>) { /* context.run_pass(RawInput { events, ..Default::default() }, |fc| app.read_and_dispatch_keystrokes(fc)).drop_without_applying_deltas() */ }

    /// Opens the menu popup through the PRODUCTION id inside one pass.
    fn open_menu_pass(&mut self) {
        let Self { app, context } = self;
        context.run_pass(Default::default(), |frame_context| {
            egui::Popup::open_id(frame_context, open_with_button::open_with_menu_id());
            app.read_and_dispatch_keystrokes(frame_context);
        }).drop_without_applying_deltas();
    }

    fn menu_is_open(&self) -> bool {
        egui::Popup::is_id_open(&self.context, open_with_button::open_with_menu_id())
    }
}

fn key_event(key: egui::Key, pressed: bool, repeat: bool) -> egui::Event { /* the escape_key_event shape generalized */ }
```

Tests:

1. `open_with_menu_should_not_open_on_q_when_fullscreen_on` — image open + programs stored + `app.toggle_fullscreen()`; assert `!app.open_with_menu_should_open_on_q(true)`.
2. `open_with_menu_should_not_open_on_q_when_no_programs_listed` — image open, stored list empty; assert `!app.open_with_menu_should_open_on_q(true)`.
3. `q_press_opens_the_menu_and_refreshes_the_stored_list` — harness; `run_pass(vec![key_event(Key::Q, true, false)])`; assert `harness.menu_is_open()` AND `harness.app.open_with_programs.is_empty()` (the stub re-enumeration replaced the stored list = the refresh contract's Q-open point).
4. `q_press_closes_the_open_menu` — harness; `open_menu_pass()`; assert open; `run_pass(vec![Q …])`; assert `!harness.menu_is_open()` AND app unchanged (`app.full_ui_chrome_is_visible()` still true, `app.message_text` still None).
5. `escape_closes_the_open_menu_without_arming_the_esc_close_popup` — harness; `open_menu_pass()`; `run_pass(vec![key_event(Key::Escape, true, false)])`; assert `!harness.menu_is_open()` AND `!harness.app.esc_close_popup_is_visible()` AND `harness.app.esc_close_popup_expires_at.is_none()` (ESC closes the menu AND NOTHING ELSE).
6. `other_keys_are_suppressed_while_the_menu_is_open` — harness; `open_menu_pass()`; `run_pass(vec![key_event(Key::F11, true, false), key_event(Key::I, true, false)])`; assert `harness.menu_is_open()` (still open) AND `harness.app.full_ui_chrome_is_visible()` (F11 did NOT toggle) AND `harness.app.zoom_factor == crate::zoom::RESTORE_TO_FIT_FACTOR` (zoom-in did NOT fire).
7. `launch_failure_message_is_warning_flagged` — image open; `app.launch_open_with_program(0)` (the stub launcher ALWAYS fails); assert `app.message_text == Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)` AND `message_line_is_warning(app.message_text)`.
8. `unsupported_file_message_is_not_warning_flagged` — `app.accept_open_outcome(PathBuf::from("bad.txt"), Err(ImageLoadFailure::Unsupported))` (the existing Err-simulation shape used in `app/tests.rs`); assert `app.message_text == Some(UNSUPPORTED_FILE_MESSAGE)` AND `!message_line_is_warning(app.message_text)`.
9. `message_line_is_warning_matches_only_the_launch_failure` — pure: `assert!(!message_line_is_warning(None));` `assert!(!message_line_is_warning(Some(UNSUPPORTED_FILE_MESSAGE)));` `assert!(message_line_is_warning(Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)));`.

(Reuse the module's existing `two_programs_fixture` / `sample_decoded_image`; import `open_with_button::{message_line_is_warning, open_with_menu_id}` and `image_loading`'s `UNSUPPORTED_FILE_MESSAGE` / `ImageLoadFailure` the way `app/tests.rs` does.)

**PINNED CENSUS: 189 unit tests, 0 failed, 28 test files** (177 + 12; no existing test breaks — the only existing assertions pinning the old caption/tooltip strings are the two edited `button_hint_tests`).

## 5. Harness execution steps (alpine-vm MCP)

1. `vm_status` FIRST (only proceed when running + SSH-reachable; else STOP + user question).
2. Dev-checks (docs/build.md verbatim): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — expected every line PASS (`fmt-check`, `check`, `test`, `clippy`) and the final `ALL CHECKS PASSED` (exit 0 = authoritative). If `fmt-check` fails: apply mechanical rustfmt wrapping only (no logic changes) and re-run.
3. Census corroboration: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo test --locked` → `189 passed; 0 failed`; `grep -c "#\[test\]"` across `src/` → 189.
4. Windows-target compile (the stub-only dev-checks do not compile `platform/windows_*`): `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu` — expected exit 0 (the pre-existing benign dead-code warning on `compare_entry_names` stands — the Wave-5.2 recorded note; no NEW warnings).
5. NO release rebuild and NO DLL audit in this cycle — those belong to Task 5 (Cycle C).

## 6. Commits and Gitignore compliance

- Commit 1 (this step's artifact, before implementation): `docs: add wave6.1 tasks2-4 ui-feedback implementation plan` — contains this plan file + the dated `context.md` entry only.
- Commit 2 (after Step 4 is green): `feat: q hotkey modal open-with menu and warning failure message` — all `src/` edits.
- Before EACH commit: read `.gitignore` + run `git status`; ensure NO gitignored path is staged (the harness writes `logs/checks/*` — gitignored, never stage; `dist/` untouched in this cycle); unstage if found; never force-add.

## 7. Scope boundaries (HARD)

- NO Task 5/6 work in this cycle: no release rebuild, no DLL audit, no README/tech/architecture full-documentation pass, no OPERATOR-RUN checklist production (Task 5's 4.1b does that).
- NO Cargo.toml / Cargo.lock changes; NO `CHANGELOG.md`.
- NO new `.kilo/plans` edits (this file is the only new plan artifact).
- NO behavior beyond the binding decisions: no new message surfaces, no changed enumeration content, no changed launcher mechanics (Cycle A's landed surface is the base), no fullscreen/zoom/fit/pan behavior changes outside the modal gate.
- `app.rs` stays UNTOUCHED; no new source or test files.

## 8. 4.5a conformance checklist (this plan SUPERSEDES the spec checklist where deviations D1–D12 apply)

- [ ] `button_hint::hotkey_reference(PillButton::OpenWith)` returns `Some("Q")`; `tooltip_text(PillButton::OpenWith)` is `"Open with... (Q)"`.
- [ ] The open-with button renders the 9 pt outlined `Q` caption under the glyph via the unchanged `show_button_column` paint (WHITE enabled / theme-dimmed disabled).
- [ ] `open_with_button.rs` defines `open_with_menu_id()` (`Id::unique("open_with_menu_popup")` — deterministic) and `OPEN_WITH_MENU_KEY = Key::Q`; the builder attaches `.id(open_with_menu_id())`.
- [ ] `keyboard_intake.rs` has the early-return guard `handle_modal_open_with_menu_keys` querying `Popup::is_id_open`; `update_esc_close_popup` runs BEFORE the guard.
- [ ] While the menu is open: only the Q and ESC non-repeat reads run; every other read/dispatch is skipped; `Popup::close_id` fires for either key; the guard returns true.
- [ ] ESC while the menu open does NOT arm the ESC-close popup and does NOT send `ViewportCommand::Close`; the expiry update keeps running (an armed popup still expires on time).
- [ ] The Q-open path requires `open_with_menu_should_open_on_q` (press + availability + chrome visible) and calls `open_with_menu_from_q_press` (refresh + `Popup::open_id`).
- [ ] The click path refreshes via the SAME `refresh_open_with_programs` and opens via the builder's own Toggle (deviation D1); the open-with button's own click while open closes the menu without re-refreshing.
- [ ] Every pill-row click dispatch is gated on `open_with_menu_is_open` (arrows, fullscreen, zoom, fit, open-with refresh — deviation D2); the menu's OWN entries still launch.
- [ ] The popup builder gate is `menu_popup_is_wanted` = `enabled || is_id_open` (deviation D8).
- [ ] `Popup::menu` keeps the default `CloseOnClick` (item-click and outside-click dismissal); no `.close_behavior` override.
- [ ] `pill_style::WARNING_MESSAGE_COLOR = Color32::from_rgb(255, 165, 0)`; `message_text_color(is_warning, ui)` applies it ONLY for the open-with failure; the unsupported-file message and cleared states keep `visuals().text_color()` (deviation D3: predicate, no app field).
- [ ] No per-frame texture re-upload or allocation churn introduced (the id fn does an allocation-free fixed-seed hash).
- [ ] Test census EXACTLY 189 unit tests / 28 test files (or an explained delta approved by the caller).
- [ ] All touched files ≤200 lines after `cargo fmt`; bodies ≤50 lines; ≤2 params; single-section booleans; no commented-out code.
- [ ] No new source/test files; `app.rs` byte-diff empty.

## 9. Follow-through (next workflow steps, not executed by this plan)

- 4.3 code-review & simplification; 4.4 docs-specialist module-header updates (the touched modules' `//!` docs; `project-structure.md` verify-only — no new files); 4.5a front-end verification against §8; 4.5b adherence; 4.6 mark Tasks 2/3/4 `[DONE]` in the TODO (preserve the rest of the file).
