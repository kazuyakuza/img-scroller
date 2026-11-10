# Implementation Plan — Wave 4.2 Task 4: ESC Double-Press Close While Fullscreen OFF (4.1b)

**Workflow step:** A-Task 4, Step 4.1b (architector) of `.kilo/plans/20261107-wave42-then-wave5-global-plan.md`, branch `fix/wave-4-2-operator-feedback`.
**TODO source of truth:** `.agent/todos/20261107/20261107-todo-1.md` → `### Task 4: ESC double-press close while fullscreen OFF (feature)`.
**Front-end input (pass-through, binding):** `.kilo/plans/20261107-wave4-2-task4-esc-close-frontend-spec.md` (4.1a). This plan implements the spec verbatim; where the spec leaves a choice open or its snippets are in tension, Section 1 pins ONE choice and records it. The spec is NOT reopened.
**Executed by:** implementer (JUNIOR, under 50% restriction). All structural, architectural, naming, ordering, and scope decisions are encoded below; nothing is left to judgment. Deviations from this plan are STOP-and-ask events.
**This step (4.1b) itself produced NO code changes** — this file is the deliverable.

---

## 0. Current state (verified 2026-11-10, all line numbers from the live working tree)

- Branch `fix/wave-4-2-operator-feedback`; `Cargo.toml` already at `0.4.2` (Step A.3 done). Working tree is expected to contain only the uncommitted 4.1a spec + this plan (the implementer confirms via `git status` in Step 1).
- `src/app.rs` (188 lines): struct `ImagesScrollerApp` fields at lines 17–38 (`fullscreen_on: bool` at 34, `zoom_factor: f32` at 37); `empty_state()` at 47–60; `show_chrome_layout` at 125–131 (navigation-context if/else, last statement ends line 130); `mod` list at 151–165 (`mod empty_state_view;` at 155); `#[cfg(test)] mod` list at 167–188 (`mod button_hint_tests;` at 169, `mod fit_one_shot_tests;` at 170).
- `src/app/keyboard_intake.rs` (75 lines): reads block at 22–35 (`escape_exit_pressed` local at 23–26), dispatch chain at 36–68 (fullscreen 36–38, ESC-exit 39–41, zoom/fit/arrows 42–68), `was_pressed` helper at 73–75. Header doc lines 1–9.
- `src/app/fullscreen.rs` (56 lines): `toggle_fullscreen_and_send_command` at 28–32; `escape_exits_fullscreen` predicate at 20–22; `FULLSCREEN_EXIT_KEY = Key::Escape` at 16.
- `src/app/central_surface.rs` (43 lines): NOT touched (popup renders in the layout dispatch, not the central paint seam).
- `src/app/floating_controls.rs` (184 lines) + `src/app/empty_state_view.rs` (46 lines): NOT touched; they own the two existing `Id::unique` overlay salts `"floating_navigation_pill"` (floating_controls.rs:21) and `"empty_state_open_button"` (empty_state_view.rs:43).
- Test harness precedents to reuse: `src/app/zoom_alias_tests.rs` (`Context::run` + `RawInput { events, ..Default::default() }` — event-level simulation, lines 14–62) and `src/app/folder_navigation_tests.rs` (`pub(super)` fixtures `TempParentScope::create / create_child_folder / create_image / child_image_path / child_folder_path` and `app_opened_in(&Path)`, cross-file reuse precedent at `folder_navigation_continuation_tests.rs:2`).
- **Live unit-test census (recounted this step by grep over `#[test]`): 116.** The 4.1a spec's §7.2 statement "Current project census is 107" is stale (it predates A-Task 3's `zoom_alias_tests.rs` landing). This plan pins the live numbers: baseline 116 → after Task 4: **127** (+11).

---

## 1. Decisions log (spec open points — PINNED here; the implementer does not re-decide)

| # | Decision | Pinned choice |
|---|---|---|
| D1 | Module names | `src/app/esc_close_confirm.rs` + `src/app/esc_close_confirm_tests.rs` (spec §3.2/§7.1 name; adopted verbatim). |
| D2 | Events read mechanism | Spec §4.4's snippet clones `input.events` per frame, which contradicts spec §8 item 16 ("no per-frame allocation churn"). **Pinned: scan INSIDE the `input` closure** — `screen_context.input(\|input\| esc_close_confirm::non_repeat_escape_press_occurred(&input.events))` — zero allocation, same pure helper signature `&[egui::Event]`, same read-before-dispatch discipline. Precedent: `src/app/zoom_alias.rs:61–67` already scans `input.events` inside `ctx.input(...)`. |
| D3 | Close-decision testability | Spec §7.2 test 7 wants "Close command sent" verified; viewport-output introspection from a headless `Context` is not an established codebase pattern. **Pinned: extract the branch decision into the pure function `esc_close_action(popup_is_visible, non_repeat_escape_was_pressed) -> EscCloseAction`** consumed by a one-line `match` in the intake (same seam shape as `fullscreen::escape_exits_fullscreen`, Wave-4.1 Task 3 precedent). The spec's §4.4 else-if chain is preserved exactly (fullscreen-exit priority first; popup logic in the `else`); the `send_viewport_cmd(egui::ViewportCommand::Close)` arm stays one-line glue in `keyboard_intake.rs`, review+OPERATOR-RUN verified like every existing command send. |
| D4 | Popup state type | The state is the single plain field `esc_close_popup_expires_at: Option<f64>` on `ImagesScrollerApp` (spec §3.1 — no wrapper struct). The only new type is the `EscCloseAction` decision enum (D3). |
| D5 | Pure state-machine surface | The ctx-taking helpers (`arm_esc_close_popup(ctx)`, `update_esc_close_popup(ctx)`) delegate to pure time-injected cores `arm_esc_close_popup_at(now: f64)` / `update_esc_close_popup_at(now: f64)` (spec §2.1's own pseudocode shape). Expiry/re-arm tests use the `_at` cores with explicit times — **never** through the `Context` harness (synthetic context time is not reliably controllable). Harness tests only assert time-independent outcomes (armed/not, fullscreen flag). |
| D6 | Spec test-10 name | Spec names it `popup_renders_in_all_fullscreen_off_states` but its own description says "no actual render needed". **Pinned name: `popup_arms_in_every_fullscreen_off_state`** — it drives the PRODUCTION intake with a synthetic ESC event in each of the three fullscreen-OFF states and asserts the popup armed. The render-call-site guarantee (popup actually painted in all OFF states, never ON) is verified by 4.5a code inspection (spec §6.3 + §8 item 13). |
| D7 | Test count / census | Spec §7.2 lists 10 tests ("at least"). **Pinned: 11 tests** — the spec's 10 (names kept, D6 rename excepted) + `expired_popup_re_arms_on_a_later_press` (the todo's re-arm requirement gets a dedicated sequence test). Census pin for A-Task 5: 116 → **127**. |
| D8 | Intake local rename | The ESC-exit local is renamed `escape_exit_pressed` → `escape_exit_fullscreen_pressed` (spec §4.4 naming). Pure rename; existing read order unchanged. |
| D9 | Intake method length | `read_and_dispatch_keystrokes`' body lands at ~53 lines (> the 50-line rule). **Accepted under the rule's explicit "unavoidable" exemption**: the pre-existing flat chain was already at ~47, and the only further split (a reads param-object) would require a new type/file and restructure EXISTING dispatch lines — out of this task's scope and a regression risk to the "intake order must NOT change" constraint. The 4.3 reviewers may propose the param-object refactor as a follow-up; the implementer must NOT do it proactively. |
| D10 | Disarm scope | ONLY these paths remove the popup: per-frame expiry, every `toggle_fullscreen_and_send_command` call (F11 key, ESC-exit, pill button — all funnel there), and the app close itself. `accept_open_outcome`, `navigate`, zoom/fit, and the Open dialog deliberately do NOT disarm (spec §1.4's closed dismissal list). |

---

## 2. High-level approach

1. One new leaf module `src/app/esc_close_confirm.rs` owns everything popup: pinned constants (message sentence, 2-second duration, Area salt, style), the pure two-distinct-presses input filter, the pure arm/idle/close branch decision, the popup state helpers on `ImagesScrollerApp` (arm / disarm / per-frame expiry; pure `_at` cores), and the centered non-capturing render. Almost everything is unit-testable without a screen.
2. `src/app.rs` gains the one session-RAM field, its `None` init, the module + test-module declarations, and the single render call inside `show_chrome_layout` (so the popup renders in EVERY fullscreen-OFF state and never while ON).
3. `src/app/keyboard_intake.rs` gains exactly one new read (the in-closure non-repeat ESC scan, D2) and restructures ONLY the ESC dispatch: expiry update first (spec-verbatim placement), then fullscreen-exit priority if/else, with the popup branch delegated to the pure decision (D3). All other reads and dispatch lines keep their existing order.
4. `src/app/fullscreen.rs`'s shared toggle gains the one disarm call (spec §3.4) — this covers F11 key, ESC-exit, and the pill button, because all three funnel through it.
5. New `src/app/esc_close_confirm_tests.rs` with 11 pinned tests (pure state machine + input filter + decision matrix + production-intake harness tests on the zoom_alias_tests pattern).
6. Git: docs commit (spec + this plan) → feat commit (all code) → review/simplification cycle → module-docs commit → verification reports commit → todo mark commit. Exe rebuild, DLL audit, and the OPERATOR-RUN checklist are deferred to A-Task 5 (this plan only pins the census delta and the test names it will verify).

---

## 3. File-by-file changes (exact)

### 3.1 NEW `src/app/esc_close_confirm.rs` (~140 lines; budget ≤200 ✓)

Module header doc (`//!`), first block — land this verbatim (4.4 only polishes):

```rust
//! The fullscreen-OFF ESC double-press close affordance (Wave 4.2 A-Task 4):
//! the centered confirm popup's pinned constants, the pure two-press input
//! filter, the pure arm/idle/close branch decision, the popup state helpers
//! on the app (arm / disarm / per-frame expiry), and the popup render.
//!
//! Behavior contract (2026-11-07 operator chat; spec §1-§6): while fullscreen
//! is OFF, the FIRST distinct ESC press arms a centered popup for
//! `POPUP_VISIBLE_SECONDS`; a SECOND distinct press while it is visible sends
//! the application Close command; expiry re-arms cleanly; held-key autorepeat
//! events never arm and never close. Fullscreen ON consumes ESC entirely (the
//! exit path in `keyboard_intake.rs` has priority) and every fullscreen toggle
//! disarms the popup. Only the intake arms/closes; only timeout, the
//! fullscreen toggle, and the second press remove the popup.
```

Imports: `use eframe::egui;` + blank line + `use super::ImagesScrollerApp;` (same shape as `fullscreen.rs:6–8`).

Constants (exact names, types, values — spec §1.2/§1.3/§2.1/§6.4):

```rust
/// The exact popup sentence (spec §1.3: this is the ONLY place the literal
/// may appear; tests and renders read it from here).
pub(super) const POPUP_MESSAGE_TEXT: &str = "Press ESC again to close";

/// Popup lifetime in seconds, then it dismisses itself (operator: 2 seconds).
pub(super) const POPUP_VISIBLE_SECONDS: f64 = 2.0;

/// Distinct Area salt (spec §6.4: no collision with "floating_navigation_pill"
/// or "empty_state_open_button").
const ESC_CLOSE_POPUP_AREA_ID: &str = "esc_close_confirm_popup";
const POPUP_FILL_COLOR: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied_const(20, 20, 20, 220);
const POPUP_CORNER_RADIUS: u8 = 10;
const POPUP_INNER_MARGIN_POINTS: f32 = 16.0;
const POPUP_TEXT_SIZE: f32 = 14.0;
const POPUP_TEXT_COLOR: egui::Color32 = egui::Color32::WHITE;
```

Note: the fill color uses the `_const` constructor (const-context requirement; precedent `floating_controls.rs:23–24`). Visibility: `POPUP_MESSAGE_TEXT` and `POPUP_VISIBLE_SECONDS` are `pub(super)` (read by tests); the rest stay private.

Decision enum + pure decision function (D3; derives REQUIRED for `assert_eq!` in tests):

```rust
/// What the ESC branch does with this frame's ESC reading (spec §3.3), after
/// the fullscreen-exit path has already had priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EscCloseAction {
    /// The popup is visible and a distinct ESC press arrived: close the app.
    CloseApplication,
    /// No popup is visible and a distinct ESC press arrived: arm the popup.
    ShowPopup,
    /// Nothing to do this frame.
    StayIdle,
}

/// The popup branch decision (spec §3.3): visible + distinct press = close;
/// not visible + distinct press = arm; anything else stays idle.
pub(super) fn esc_close_action(
    popup_is_visible: bool,
    non_repeat_escape_was_pressed: bool,
) -> EscCloseAction {
    match (popup_is_visible, non_repeat_escape_was_pressed) {
        (true, true) => EscCloseAction::CloseApplication,
        (false, true) => EscCloseAction::ShowPopup,
        _ => EscCloseAction::StayIdle,
    }
}
```

Pure input filter — spec §4.2 VERBATIM (name, signature, body):

```rust
/// True exactly for a first-of-two DISTINCT ESC presses: a pressed Escape key
/// event that is NOT a held-key autorepeat (spec §4.2; `key_pressed` alone
/// would also fire on repeats).
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

State helpers (`impl ImagesScrollerApp` block in this file — the codebase's split-impl precedent, e.g. `zoom.rs:54`):

```rust
impl ImagesScrollerApp {
    /// True while the popup is currently armed/visible.
    pub(super) fn esc_close_popup_is_visible(&self) -> bool {
        self.esc_close_popup_expires_at.is_some()
    }

    /// Arms the popup NOW, reading the context clock (the intake's arm path).
    pub(super) fn arm_esc_close_popup(&mut self, screen_context: &egui::Context) {
        let now = current_time(screen_context);
        self.arm_esc_close_popup_at(now);
    }

    /// Pure arm: deadline = now + POPUP_VISIBLE_SECONDS (spec §2.1).
    pub(super) fn arm_esc_close_popup_at(&mut self, now: f64) {
        self.esc_close_popup_expires_at = Some(now + POPUP_VISIBLE_SECONDS);
    }

    /// Clears the popup without closing the app (every fullscreen toggle calls this).
    pub(super) fn disarm_esc_close_popup(&mut self) {
        self.esc_close_popup_expires_at = None;
    }

    /// Expires the popup if its deadline has passed (the per-frame update).
    pub(super) fn update_esc_close_popup(&mut self, screen_context: &egui::Context) {
        let now = current_time(screen_context);
        self.update_esc_close_popup_at(now);
    }

    /// Pure expiry: `now >= deadline` clears the armed state (spec §2.1).
    pub(super) fn update_esc_close_popup_at(&mut self, now: f64) {
        let Some(deadline) = self.esc_close_popup_expires_at else {
            return;
        };
        if now >= deadline {
            self.esc_close_popup_expires_at = None;
        }
    }
}

fn current_time(screen_context: &egui::Context) -> f64 {
    screen_context.input(|input| input.time)
}
```

Render function (spec §1.1 snippet verbatim mechanics: centered anchor, `Order::Foreground`, `interactable(false)`, the §1.2 style constants; self-gated on visibility so the single call site in `app.rs` needs no gate):

```rust
/// The popup render (spec §1.1): a centered, Foreground, NON-CAPTURING Area
/// painted ONLY while armed; the chrome layout calls it in every
/// fullscreen-OFF state. Pure transient: no clicks, no focus, and no
/// dismissal besides timeout / fullscreen toggle / the closing second press.
pub(super) fn show_esc_close_popup(app: &ImagesScrollerApp, screen_context: &egui::Context) {
    if !app.esc_close_popup_is_visible() {
        return;
    }
    egui::Area::new(egui::Id::unique(ESC_CLOSE_POPUP_AREA_ID))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .order(egui::Order::Foreground)
        .interactable(false)
        .show(screen_context, |popup_ui| {
            egui::Frame::new()
                .fill(POPUP_FILL_COLOR)
                .corner_radius(POPUP_CORNER_RADIUS)
                .inner_margin(POPUP_INNER_MARGIN_POINTS)
                .show(popup_ui, |frame_ui| {
                    frame_ui.label(
                        egui::RichText::new(POPUP_MESSAGE_TEXT)
                            .size(POPUP_TEXT_SIZE)
                            .color(POPUP_TEXT_COLOR),
                    );
                });
        });
}
```

**STOP condition:** the 4.1a spec verified `Area::interactable` exists in egui 0.37 (spec §9, docs.rs 2026-11-10). If the compiler reports it missing, STOP and return the question to the caller — do NOT silently drop `.interactable(false)` or substitute another mechanism (spec §8 item 4 requires it).

### 3.2 `src/app.rs` edits (188 → ~193 lines)

1. **Field** — insert after line 37 (`zoom_factor: f32,`), before the struct's closing brace (line 38):
```rust
    /// Popup expiry deadline for the fullscreen-OFF ESC double-press close
    /// affordance (Wave 4.2 A-Task 4): `None` = popup not visible; the value is
    /// the absolute `InputState::time` (seconds) when the popup dismisses.
    esc_close_popup_expires_at: Option<f64>,
```
2. **Init** — inside `empty_state()` (lines 47–60), insert after line 58 (`zoom_factor: crate::zoom::RESTORE_TO_FIT_FACTOR,`):
```rust
            esc_close_popup_expires_at: None,
```
3. **Render call site** — in `show_chrome_layout` (lines 125–131), add the popup call as the LAST statement, after the existing if/else (i.e., after line 130's closing `}` of the else, before the fn's closing `}`):
```rust
        esc_close_confirm::show_esc_close_popup(self, ui.ctx());
```
Resulting body:
```rust
    fn show_chrome_layout(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        if self.navigation_context_is_active() {
            self.show_navigation_layout(ui);
        } else {
            self.show_empty_state_layout(frame, ui);
        }
        esc_close_confirm::show_esc_close_popup(self, ui.ctx());
    }
```
This placement (inside `show_chrome_layout`, which runs only when `full_ui_chrome_is_visible()`) is exactly spec §6.3/§8 item 13: popup renders in fresh empty state, image-open state, and empty-folder browsed state; never while fullscreen ON. No other layout fn (`show_fullscreen_layout`, `show_navigation_layout`, `show_empty_state_layout`, central_surface) is touched.
4. **Module declaration** — insert `mod esc_close_confirm;` after line 155 (`mod empty_state_view;`), before `mod fit_button;` (alphabetical: `empty_state_view` < `esc_close_confirm` < `fit_button`).
5. **Test module declaration** — insert after line 169 (`#[cfg(test)] mod button_hint_tests;`), before line 170 (`#[cfg(test)] mod fit_one_shot_tests;`):
```rust
#[cfg(test)]
mod esc_close_confirm_tests;
```
No `use` line is added to app.rs (child-module path `esc_close_confirm::…` resolves directly, like the existing child-module calls).

### 3.3 `src/app/keyboard_intake.rs` edits (75 → ~104 lines; budget ≤200 ✓)

1. **Header doc** — line 2 of the `//!` block gains the ESC-close branch in the chord list. Line 1–5 becomes (single doc paragraph change; the "moved verbatim" note at lines 7–9 stays untouched):
```rust
//! Per-frame keyboard intake: every edge-triggered chord of the app (the
//! fullscreen F11 toggle and the ESC-while-ON exit, the ESC-close arm/close
//! branch, the four navigation arrows, the zoom `I`/`O`/`0` chords, and the
//! one-shot fit `W`/`H` keys) is read once per frame and dispatched; the
//! intake runs in EVERY mode (fullscreen included) and always
//! BEFORE the layout dispatch.
```
2. **Import** — insert `use super::esc_close_confirm;` between line 14 (`use super::NavigationDirection;`) and line 15 (`use super::fullscreen;`) (ASCII order: `NavigationDirection` < `esc_close_confirm` < `fullscreen`).
3. **Reads (before ANY dispatch — the module's binding discipline):**
   - Rename the local at lines 23–26: `escape_exit_pressed` → `escape_exit_fullscreen_pressed` (D8). The read expression itself is unchanged (`fullscreen::escape_exits_fullscreen(self.fullscreen_on, was_pressed(screen_context, fullscreen::FULLSCREEN_EXIT_KEY))`).
   - Insert the ONE new read (D2) after line 26 (the renamed read), BEFORE the zoom reads at line 27:
```rust
        let non_repeat_escape_pressed = screen_context.input(|input| {
            esc_close_confirm::non_repeat_escape_press_occurred(&input.events)
        });
```
   - All other reads (lines 27–35: zoom in/out, restore, fit W/H, four arrows) keep their exact current order and form.
4. **Expiry update** — insert after the LAST read (after line 40, the `down_pressed` let) and BEFORE the first dispatch `if` (spec §4.4 verbatim placement — "runs first so an expired popup is cleared before the ESC reads decide"):
```rust
        self.update_esc_close_popup(screen_context);
```
5. **ESC dispatch restructure** — replace the current lines 39–41:
```rust
        if escape_exit_pressed {
            self.toggle_fullscreen_and_send_command(screen_context);
        }
```
with (fullscreen-exit keeps priority; the popup branch lives in the `else`, per spec §4.4's else-if chain):
```rust
        if escape_exit_fullscreen_pressed {
            self.toggle_fullscreen_and_send_command(screen_context);
        } else {
            self.dispatch_esc_close_action(screen_context, non_repeat_escape_pressed);
        }
```
6. **New private dispatch helper** — inside the same `impl` block, after `read_and_dispatch_keystrokes` (after current line 69):
```rust
    /// The ESC-close branch: runs ONLY when the fullscreen-exit ESC path did
    /// not consume this frame's ESC (fullscreen OFF, or no ESC press).
    fn dispatch_esc_close_action(
        &mut self,
        screen_context: &egui::Context,
        non_repeat_escape_was_pressed: bool,
    ) {
        match esc_close_confirm::esc_close_action(
            self.esc_close_popup_is_visible(),
            non_repeat_escape_was_pressed,
        ) {
            esc_close_confirm::EscCloseAction::CloseApplication => {
                screen_context.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            esc_close_confirm::EscCloseAction::ShowPopup => {
                self.arm_esc_close_popup(screen_context);
            }
            esc_close_confirm::EscCloseAction::StayIdle => {}
        }
    }
```
   - `ViewportCommand::Close` is sent in EXACTLY this one place in the codebase (spec §5.2).
   - The zoom/fit/arrow dispatch ifs (current lines 42–68) are untouched.
7. **`was_pressed` (lines 72–75)** — untouched.

Intake-order invariants the implementer must preserve and 4.3 will check: (a) every existing key's read order is byte-identical; (b) all reads still precede all dispatches; (c) `update_esc_close_popup` sits between reads and dispatch; (d) the fullscreen `if` and the ESC-exit `if/else` both precede the zoom dispatch. Projected body of `read_and_dispatch_keystrokes`: ~53 lines (D9 — accepted exemption, do not refactor).

### 3.4 `src/app/fullscreen.rs` edit (56 → ~57 lines)

1. In `toggle_fullscreen_and_send_command` (lines 28–32), insert the disarm call immediately after the flag flip, BEFORE the command send (spec §3.4 verbatim):
```rust
    pub(super) fn toggle_fullscreen_and_send_command(&mut self, screen_context: &egui::Context) {
        let fullscreen_after_toggle = self.toggle_fullscreen();
        self.disarm_esc_close_popup();
        screen_context
            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen_after_toggle));
    }
```
No `use` is needed — `disarm_esc_close_popup` is an inherent method (defined in `esc_close_confirm.rs`) and inherent methods need no import. This single edit covers all three toggle triggers: F11 key, ESC-while-ON, and the pill's fullscreen button (all funnel here).
2. Module header doc: extend the Wave-4.1 line 3–4 area with one clause — e.g. after line 3: `//! Wave-4.2 Task 4 addition: every toggle disarms the ESC-close popup.` (Full polish remains 4.4's job.)

### 3.5 NEW `src/app/esc_close_confirm_tests.rs` (~150 lines; 11 tests — D7)

Imports (rustfmt-governed order; dev-checks `cargo fmt --check` is the gate):

```rust
use eframe::egui;

use super::esc_close_confirm::{
    EscCloseAction, POPUP_VISIBLE_SECONDS, esc_close_action, non_repeat_escape_press_occurred,
};
use super::folder_navigation_tests::{TempParentScope, app_opened_in};
use super::fullscreen::FULLSCREEN_TOGGLE_KEY;
use super::{ImagesScrollerApp, NavigationDirection};
```

Shared fixtures (module-private helpers at the top, after the doc header):

```rust
const ARM_TIME: f64 = 1.0;

/// ESC key event builder: `pressed` + `repeat` flag it as a distinct press,
/// a held-key autorepeat, or a release.
fn escape_key_event(pressed: bool, repeat: bool) -> egui::Event {
    egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: Some(egui::Key::Escape),
        pressed,
        repeat,
        modifiers: egui::Modifiers::default(),
    }
}

/// Runs ONE production intake frame for the app with the given events (the
/// fresh-context-per-frame harness of zoom_alias_tests; assertions read the
/// app AFTER the frame).
fn run_intake_frame(app: &mut ImagesScrollerApp, events: Vec<egui::Event>) {
    let frame_input = egui::RawInput {
        events,
        ..Default::default()
    };
    let context = egui::Context::default();
    context.run(frame_input, |frame_context| {
        app.read_and_dispatch_keystrokes(frame_context);
    });
}
```

Module doc header (`//!`): one paragraph — "ESC-close-confirm tests (Wave 4.2 A-Task 4): the pure popup state machine (arm / expiry / re-arm at explicit times), the two-distinct-press input filter and branch decision, and the production-intake interplay with fullscreen through the egui-`Context` harness (the zoom_alias_tests pattern). Expiry is tested only through the pure `_at(now)` cores; synthetic context time is not controlled."

The 11 tests (EXACT names — spec §7.2's 10 with D6's rename, plus D7's sequence test):

1. `popup_starts_invisible` — `ImagesScrollerApp::empty_state()`: `!app.esc_close_popup_is_visible()` and `app.esc_close_popup_expires_at.is_none()`.
2. `arm_makes_popup_visible` — `arm_esc_close_popup_at(ARM_TIME)`: visible; field `== Some(ARM_TIME + POPUP_VISIBLE_SECONDS)` (pins the 2.0 constant).
3. `update_before_deadline_keeps_popup_visible` — arm at `ARM_TIME`, `update_esc_close_popup_at(ARM_TIME + POPUP_VISIBLE_SECONDS - 0.5)`: still visible, deadline unchanged.
4. `update_at_deadline_clears_popup` — arm at `ARM_TIME`, update at exactly `ARM_TIME + POPUP_VISIBLE_SECONDS`: invisible + field `None`; a further `update_esc_close_popup_at(10.0)` stays `None` (past-deadline idempotence).
5. `non_repeat_escape_press_detected` — `non_repeat_escape_press_occurred(&[escape_key_event(true, false)])` is `true`; `&[escape_key_event(false, false)]` (release) is `false`; `&[]` is `false`.
6. `repeat_escape_press_is_ignored` — `&[escape_key_event(true, true)]` is `false`; the distinct press `(true, false)` in the same test stays the positive control (`true`).
7. `distinct_second_press_closes_when_popup_visible` — the FULL decision matrix via `esc_close_action`: `(true, true)` → `EscCloseAction::CloseApplication`; `(true, false)` → `StayIdle`; `(false, true)` → `ShowPopup`; `(false, false)` → `StayIdle`. (D3: the `send_viewport_cmd` arm itself is one-line glue, review-verified, per the Wave-4.1 Task-3 precedent.)
8. `fullscreen_toggle_disarms_popup` — app fullscreen OFF, armed via `arm_esc_close_popup_at(ARM_TIME)` (pure; survives the harness frame's fresh-context expiry update because time ≈ 0 < 1.0), then `run_intake_frame` with an F11 press event built inline (`egui::Event::Key { key: FULLSCREEN_TOGGLE_KEY, physical_key: Some(FULLSCREEN_TOGGLE_KEY), pressed: true, repeat: false, modifiers: default }`): assert `app.fullscreen_on` is `true` AND the popup is invisible.
9. `fullscreen_exit_esc_does_not_arm_popup` — app put fullscreen ON via the pure `app.toggle_fullscreen()`; `run_intake_frame` with `escape_key_event(true, false)`: assert `!app.fullscreen_on` (exited — the exit path consumed ESC) AND `!app.esc_close_popup_is_visible()` (no arm, no close decision reached).
10. `expired_popup_re_arms_on_a_later_press` — arm at `ARM_TIME`, expire via `update_esc_close_popup_at(ARM_TIME + POPUP_VISIBLE_SECONDS)`, assert invisible; then `run_intake_frame` with `escape_key_event(true, false)`: assert visible again (production re-arm).
11. `popup_arms_in_every_fullscreen_off_state` (D6) — three states, each driven by the PRODUCTION intake with one synthetic distinct ESC press, each asserting `esc_close_popup_is_visible()` after the frame:
    - fresh empty state: `ImagesScrollerApp::empty_state()` → run ESC frame → armed;
    - image open: `TempParentScope::create("esc_popup_image_state")` + `create_child_folder("solo")` + `create_image("solo", "only.png")` + `app_opened_in(&folder.child_image_path("solo", "only.png"))` → run ESC frame → armed;
    - empty-folder browsed: `TempParentScope::create("esc_popup_empty_folder_state")` + `create_child_folder("folder1")` + `create_child_folder("folder2")` + `create_image("folder2", "b.png")` + `app_opened_in(&scope.child_image_path("folder2", "b.png"))` + `app.navigate(NavigationDirection::PreviousSiblingFolder)` (assert `app.browsed_folder_context == Some(scope.child_folder_path("folder1"))` first, the continuation-tests pattern) → run ESC frame → armed.

Test-file rules: real newlines (Newline Prevention Rule); no commented-out code; each test body ≤ ~15 lines (test 11's three states stay flat — max 1 nesting level); total file ≤200 lines; NO assertion of `POPUP_MESSAGE_TEXT` (the literal is single-sourced in the constant; spec §8 item 2 is 4.5a-inspected); no viewport-output introspection (D3); no test controls synthetic context time (D5).

---

## 4. Line budgets (verified projections — every touched file ≤200)

| File | Before | After (projected) | Notes |
|---|---|---|---|
| `src/app/esc_close_confirm.rs` | — | ~140 | new; constants + pure helpers + state + render |
| `src/app/esc_close_confirm_tests.rs` | — | ~150 | new; 11 tests + 2 helpers |
| `src/app.rs` | 188 | ~193 | +4 field/doc lines, +1 init, +1 call, +2 mod lines |
| `src/app/keyboard_intake.rs` | 75 | ~104 | +1 import, +1 doc line, +3 read, +1 update call, +3 else-block, +~20 helper |
| `src/app/fullscreen.rs` | 56 | ~57 | +1 disarm call (+1 doc line counts within header) |

`keyboard_intake.rs` projection 75 → ~104 (well under 200). `read_and_dispatch_keystrokes` body ~53 lines (D9 accepted exemption).

---

## 5. Execution steps for the implementer (Critical Workflow step 4.2)

1. **Step 1 — Git check + docs commit.** On the host: `git status` (read `C:\repo\img-scroller\.gitignore` first — Gitignore Compliance Rule; expected: clean except the untracked/modified `.kilo/plans/20261107-wave4-2-task4-esc-close-frontend-spec.md` and this plan file). Verify branch is `fix/wave-4-2-operator-feedback` (`git branch --show-current`). Stage ONLY the two `.kilo/plans/` files and commit: `docs: add wave4.2 task4 spec and implementation plan` (Task-2 precedent `f2857c5`).
2. **Step 2 — Create `src/app/esc_close_confirm.rs`** exactly per §3.1. Use the file-write tool with real newlines.
3. **Step 3 — Edit `src/app.rs`** per §3.2 (5 edits, current line numbers in §0).
4. **Step 4 — Edit `src/app/keyboard_intake.rs`** per §3.3.
5. **Step 5 — Edit `src/app/fullscreen.rs`** per §3.4.
6. **Step 6 — Create `src/app/esc_close_confirm_tests.rs`** per §3.5.
7. **Step 7 — Format + dev-checks in the container (alpine-vm MCP).** Call `alpine-vm_vm_status` FIRST; only proceed when the VM is running and SSH-reachable. Then, via `alpine-vm_vm_run_command`: (a) `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` (the harness never formats; fmt first); (b) `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — exit code 0 and the final `ALL CHECKS PASSED` line are the authoritative evidence (`fmt-check`, `check`, `test`, `clippy`; per `docs/build.md`). If the VM or `/img-scroller` share is unavailable, STOP and return a user question. **STOP condition:** if `cargo check` fails on `Area::interactable` (see §3.1), STOP and return the question — do not substitute.
8. **Step 8 — Census confirmation.** From the harness log (`logs/checks/dev-checks-<ts>.log`) or the `cargo test` output line `test result: ok`, confirm **127 passed; 0 failed**. If the observed total differs, report the OBSERVED number — the pinned DELTA (+11) stands; do not add or remove tests to hit 127.
9. **Step 9 — Feature commit.** `git status` → stage ONLY `src/app.rs`, `src/app/keyboard_intake.rs`, `src/app/fullscreen.rs`, `src/app/esc_close_confirm.rs`, `src/app/esc_close_confirm_tests.rs` → commit `feat: esc double press close while fullscreen off` (verify no `logs/`, `dist/`, or `target/` paths are staged — gitignored).
10. **Step 10 — Report.** Return: files changed, commit hash, dev-checks evidence line, observed census. What was NOT done: exe rebuild, DLL audit, OPERATOR-RUN (A-Task 5); project-structure.md/README/tech/architecture/context (A-Task 6); todo mark (4.6).

Console commands are container-only; the host needs NO Rust/Cargo (never run `cargo` host-side). File edits happen host-side with the edit/write tools (the VM share mirrors the tree; the vboxsf EBUSY fallback note in `context.md` covers container-side writes only).

---

## 6. 4.3 — Code review & simplification scope (next step after 4.2)

- **code-reviewer:** diffs against THIS plan (§3.1–§3.5 exactly; intake-order invariants in §3.3; D1–D10); check spec §8 items 1–17 conformance except the OPERATOR-RUN-gated ones; verify no other file changed (`git diff --stat HEAD~1` shows exactly the 5 files).
- **code-simplifier:** expected simplification surface is small by design. KNOWN, pre-declared items: (a) D9's ~53-line intake body (do NOT "fix" by param-object refactor — out of scope; propose as follow-up TODO only if deemed necessary); (b) the `_at` pure cores exist for testability — do not inline them away; (c) `current_time` is a 1-line helper — acceptable. Any fix/simplification plan → `.kilo/plans/20261110-wave4-2-task4-esc-close-review-fix.md` (new file; never edit existing plans), then implementer executes; max 3 cycles then escalate.

## 7. 4.4 — Documentation (module-header bullets; docs-specialist)

- `esc_close_confirm.rs` is BORN with its full `//!` header (§3.1) and `keyboard_intake.rs`/`fullscreen.rs` headers gain their one-line additions during 4.2 (§3.3.1, §3.4.2) so the code is never self-inconsistent. 4.4 only POLISHES: verify each header still matches the code, add cross-references (`esc_close_confirm.rs` ↔ `keyboard_intake.rs` ordering note ↔ `fullscreen.rs` disarm note), and keep every file within budget. No README/tech/architecture/context/structure edits here — those belong to A-Task 6 (per the global plan and spec §7.1's note). Commit: `docs: esc close confirm module docs` (Task-3 precedent `db83dd4`).

## 8. 4.5a crosswalk — spec §8 conformance item → where it lands

| Spec §8 item | Satisfied at |
|---|---|
| 1 window options unchanged | no edit near `window_options()` (app.rs:141–149) |
| 2 message literal canonical | `POPUP_MESSAGE_TEXT` sole definition (§3.1); grep `"Press ESC again to close"` → 1 hit in `src/` |
| 3 centered | `Area::anchor(Align2::CENTER_CENTER, Vec2::ZERO)` (§3.1 render) |
| 4 non-capturing | `.interactable(false)` (§3.1; STOP condition if absent) |
| 5 layers above chrome | `.order(egui::Order::Foreground)` (§3.1) |
| 6 two-second lifetime, `>=` | `POPUP_VISIBLE_SECONDS = 2.0` + inline `if now >= deadline` (§3.1) |
| 7 frame-rate-independent timing | `current_time` reads `InputState::time` (§3.1); no `stable_dt` accumulation |
| 8 repeat:false filter over `input.events` | `non_repeat_escape_press_occurred` + D2 in-closure scan (§3.3.3) |
| 9 held ESC never arms/closes | filter + tests 6 |
| 10 fullscreen-ON consumes ESC | if/else priority in the intake (§3.3.5) + test 9 |
| 11 toggle disarms popup | `disarm_esc_close_popup()` in the shared toggle (§3.4.1) + test 8 |
| 12 Close command correct | single `send_viewport_cmd(egui::ViewportCommand::Close)` arm (§3.3.6) |
| 13 visible in ALL OFF states | render call inside `show_chrome_layout` (§3.2.3) + test 11 |
| 14 no ID collision | salt `"esc_close_confirm_popup"` distinct from the two existing salts (§3.1) |
| 15 field initialized `None` | `empty_state()` init (§3.2.2) + test 1 |
| 16 no per-frame allocation churn | D2 (no events clone) + self-gated render (early return) |
| 17 tests added | §3.5: 11 tests, census 116 → 127 |

## 9. A-Task 5 handoff pins (harness deferred — do NOT execute in this task)

- Unit-test census: **127 expected** (116 baseline + 11; §5.8 governs if the live count differs).
- Dev-checks: `ALL CHECKS PASSED` via the container harness (re-run in A-Task 5 with the full wave scope).
- Release rebuild `scripts/build-windows.sh` → non-empty `dist/windows-x64/img-scroller.exe` (0.4.2); DLL import audit = the recorded 22-DLL constant: A-Task 5.
- OPERATOR-RUN checklist (manual, real Windows): popup shows CENTERED for ~2 s when fullscreen OFF (fresh state, image open, empty-folder browsed); second distinct press closes; expiry re-arms; holding ESC never arms or closes; fullscreen ON keeps ESC = exit-fullscreen with no popup; F11 while popup visible hides it and no close arms: A-Task 5 (copy into the A-Task 5 plan verbatim).

## 10. Out of scope / non-goals (hard boundaries)

- NO changes to: `central_surface.rs`, `floating_controls.rs`, `empty_state_view.rs`, `button_hint.rs`, `pill_style.rs`, zoom/fit modules, `main.rs`, `Cargo.toml` (already 0.4.2), `window_options()`, the existing `was_pressed` helper, any existing read/dispatch line's order, `.agent/project-structure.md` (A-Task 6), the todo file (4.6), any existing `.kilo/plans/*` file.
- NO popup interaction with mouse/pointer (spec §1.4: pure non-capturing transient), no popup on ANY trigger other than the intake's ESC branch, no disarm on open/navigate/zoom/fit (D10), no persistence of `esc_close_popup_expires_at` (session RAM, dies with the app).

## 11. Verification checklist vs the todo's Task-4 sub-items

| Todo sub-item | Covered by |
|---|---|
| First ESC (fullscreen OFF) → centered popup "Press ESC again to close" | §3.1 render + §3.3 ShowPopup arm + §3.2.3 call site |
| Popup visible 2 s, then self-dismisses | `POPUP_VISIBLE_SECONDS = 2.0` + per-frame expiry (§3.1) |
| Second distinct press WHILE visible closes (not held/autorepeat) | repeat:false filter + `CloseApplication` → `ViewportCommand::Close` (§3.3.6); tests 5–7 |
| Expired popup re-arms on next press | expiry → `ShowPopup` decision; test 10 |
| Fullscreen ON: ESC exits only, no popup/close | exit-path priority else-if (§3.3.5); tests 8–9 |
| Popup needs no other UI element | standalone `Area` in `show_chrome_layout` (§3.2.3) |
| Front-end task: 4.1a then 4.1b | 4.1a spec consumed (this file); 4.5a crosswalk §8 |
