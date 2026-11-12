# Implementation Plan — Wave 5.1 Task 3 (4.1b): ESC-Close Popup 2-Second Expiry Fix — FINAL

Task: `.agent/todos/20261112/20261112-todo-1.md` → Task 3 (Fix the popup 2-second expiry).
Branch `fix/wave-5-1-operator-feedback` · version 0.5.1 (already bumped at Step 3) · NOT front-end (no 4.1a sibling; intake/state logic only).

## 1. DECIDED — the root cause (source-cited, pinned file:line)

**The clock is NOT the defect. The failing link is the MISSING DEADLINE WAKE: after the
arm pass, no egui pass runs again until the user produces an OS event, so the (correct)
expiry comparison is never re-evaluated.**

### 1.1 The time pipeline is INTACT (all three producers set advancing time)

Resolved crates read inside the container (`/usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`
after `cargo fetch --locked` in the SAME `docker compose … run --rm rust sh -c` — the
registry cache is EPHEMERAL per container run; the established pattern is fetch+grep in
one invocation). Resolved versions: eframe/egui/egui-winit/egui_glow 0.37.0, winit 0.30.13.

| Component | Fact | Citation |
| --- | --- | --- |
| egui `RawInput.time` | `pub time: Option<f64>` — "Monotonically increasing time, in seconds… If `None` is provided, egui will assume a time delta of `predicted_dt` (default 1/60)" | egui-0.37.0 `src/data/input/raw_input.rs` (`pub struct RawInput` field docs) |
| egui `InputState::begin_pass` | `let time = new.time.unwrap_or(self.time + new.predicted_dt as f64);` — `InputState.time` (pub, f64) = `RawInput.time` when Some | egui-0.37.0 `src/input_state/mod.rs:366–376` (field decl `:272`, default `0.0` `:351`) |
| egui-winit `take_egui_input` | `self.egui_input.time = Some(self.start_time.elapsed().as_secs_f64());` — `start_time` is a native monotonic `Instant` fixed at State construction (adjusted by ctx time at `lib.rs:177–179`) | egui-winit-0.37.0 `src/lib.rs:320–323`, `:94` |
| eframe glow per-frame input | `let mut raw_input = egui_winit.take_egui_input(window);` at BOTH loop sites | eframe-0.37.0 `src/native/glow_integration.rs:689`, `:1761` |
| eframe glow time override | `raw_input.time = Some(self.integration.beginning.elapsed().as_secs_f64());` | eframe-0.37.0 `glow_integration.rs:698`, `:1767` |
| eframe epi time override | `raw_input.time = Some(self.beginning.elapsed().as_secs_f64());` inside `prepare_raw_input`, immediately before `egui_ctx.run_ui(raw_input, \|ui\| { app.logic…; app.ui(ui, …) })` | eframe-0.37.0 `src/native/epi_integration.rs:421`, `:337–370` |
| egui pass input install | `viewport.input = …begin_pass(new_raw_input, …)` — the per-viewport `InputState` IS updated from the pass's `RawInput` | egui-0.37.0 `src/context.rs:459–520` (`ContextImpl::begin_pass`) |
| app call chain | `App::ui` → `read_and_dispatch_keystrokes(ui.ctx())` → `update_esc_close_popup` at `keyboard_intake.rs:53` — UNCONDITIONAL, before the ESC dispatch | `src/app.rs:145–150`, `src/app/keyboard_intake.rs:25–61` |

Conclusion: `input.time` on the production eframe/glow Windows stack is `Some(monotonic
elapsed)` and ADVANCES every pass. The todo's "non-advancing `input.time`" candidate and
the "gated/skipped update" candidate are BOTH ELIMINATED by source evidence (the update
call is unconditional; the time source is triple-redundant).

### 1.2 THE FAILING LINK — eframe's event loop sleeps at `ControlFlow::Wait`

- eframe native sets `event_loop.set_control_flow(ControlFlow::Wait)` (`src/native/run.rs:138`)
  and after every event batch re-derives the flow in `check_redraw_requests`
  (`run.rs:287–299`): `WaitUntil(next_repaint_time)` ONLY when a repaint is scheduled
  (`windows_next_repaint_times` / `windows_redraw_deadlines`), otherwise `ControlFlow::Wait`.
- A repaint gets scheduled ONLY through `Context::set_request_repaint_callback` →
  `UserEvent::RequestRepaint` → `event_loop_proxy.send_event(...)` (eframe
  `glow_integration.rs:367–379`) — i.e. ONLY when egui (or the app) calls
  `request_repaint{,_after}`. egui itself propagates `InputState::wants_repaint_after()`
  at the end of every pass (`context.rs:578–579`; `wants_repaint_after` at
  `input_state/mod.rs:676–700` returns `Some(ZERO)` when `!self.events.is_empty()` and
  `None` for a quiet static pass).
- **Therefore**: the arm pass (with the ESC event) ends with a ZERO-delay settle repaint,
  and the settle pass is quiet → after it the loop goes `ControlFlow::Wait`. The armed
  `esc_close_popup_expires_at` deadline is NEVER re-evaluated — `update_esc_close_popup`
  only runs inside a pass. The popup stays armed indefinitely until ANY user event
  (any key/mouse → "until used") or until a fullscreen toggle disarms it — EXACTLY the
  operator's 2026-11-12 evidence ("the msg of press again to exit should disappear after
  2 seconds"; it never does by itself).
- The round-1 unit tests could not catch this: they exercised only the pure `_at(now)`
  cores with explicit times (`esc_close_confirm_tests.rs:5–6` says so verbatim), and the
  round-1 ctx harness (`run_intake_frame`) builds a FRESH `Context::default()` per frame,
  so `input.time` never advances across frames in tests — the time+wiring interplay was
  never exercised (the shipped gap).

### 1.3 The DECIDED fix — keep `input.time`, schedule the egui wake at the deadline

Replacement-clock candidates were REJECTED with reasons (all fail the same way):

- `std::time::Instant` app-side clock: measuring time is NOT the problem — the
  comparison only executes inside a pass; swapping the clock changes nothing while the
  loop sleeps. It would also split arm/expiry time sources (breaking the proven
  same-source shape) and change the stored field's type/value basis. REJECTED.
- `input.stable_dt` accumulator: dt only accumulates across passes — the same
  missing-wake problem; dt is also unreliable under `Wait`. REJECTED.
- `eframe::Frame` time: not a wake mechanism either. REJECTED.

**FIX (single egui-sanctioned mechanism): while the popup is armed, `update_esc_close_popup`
requests `screen_context.request_repaint_after(remaining + margin)` on EVERY pass.**
`request_repaint_after` (`egui context.rs:2179`) is egui's documented "wake me when time
passes" API: the callback forwards `RequestRepaintInfo{delay}` (context.rs:53–65) to
eframe, which schedules `ControlFlow::WaitUntil(arm_wall + delay)` (run.rs:297–299); the
waking pass runs the intake, `input.time ≥ deadline`, the pure core clears the state, and
the popup disappears in that same pass (render gate `esc_close_popup.rs:126–128`).

Why RE-REQUEST EVERY PASS (not once at arm) — coalescing pinned from source:
- `request_repaint_after` fires its callback only when `delay < viewport.repaint.repaint_delay`
  (egui `context.rs:133–172`, "only the smallest duration will be considered").
- `begin_pass_repaint_logic` resets the viewport delay at every pass begin: `Duration::MAX`
  when `outstanding == 0`, `Duration::ZERO` otherwise (`context.rs:100–135`; fresh-context
  default `repaint_delay: Duration::MAX` at `context.rs:380`).
- A pass that begins with `outstanding > 0` therefore SWALLOWS any timed request — but that
  same branch always fires a ZERO-delay callback itself, guaranteeing at least one more
  pass; the next pass (delay reset to MAX) accepts the re-request. Repeating the request
  while armed CONVERGES: the wake is guaranteed to be registered within at most one extra
  pass, regardless of when the ESC press lands (including inside a settle pass).
- The arm pass needs NO extra request: it always contains the ESC event → `wants_repaint_after`
  → ZERO settle pass → the very next pass re-requests the remaining time and is accepted.
  Single insertion point, smallest diff.

Timing tolerance (binding): the wake lands at `deadline + POPUP_EXPIRY_WAKE_MARGIN_SECONDS`,
so the popup auto-dismisses ~2.05 s after arming. The 0.05 s margin keeps the waking pass's
`input.time` safely PAST the deadline (the arm-clock read precedes the wake call; OS timers
can fire marginally early; f64 ULP rounding must never defeat `now >= deadline`). This
satisfies the binding contract "auto-dismisses after exactly 2.0 s" as the operator-facing
"~2 seconds" (brief addendum item 7 wording: "for 2 seconds"; the Task-5 checklist item is
"~2 s"); the internal deadline stays exactly `now + POPUP_VISIBLE_SECONDS = 2.0`.

## 2. Code changes (the 4.2 implementer; scope pinned exactly)

### 2.1 `src/app/esc_close_confirm.rs` (146 → ~170 lines; cap 200 ✓)

1. Add the constant next to `POPUP_VISIBLE_SECONDS` (line 24):
   `pub(super) const POPUP_EXPIRY_WAKE_MARGIN_SECONDS: f64 = 0.05;`
2. Change the per-frame ctx wrapper ONLY (lines 101–104):
   ```rust
   pub(super) fn update_esc_close_popup(&mut self, screen_context: &egui::Context) {
       let now = current_time(screen_context);
       self.request_expiry_wake(screen_context, now);
       self.update_esc_close_popup_at(now);
   }
   ```
   (wake runs BEFORE the expiry core; on the clearing pass `remaining ≤ 0` makes the
   request a no-op — no pointless wakes.)
3. Add ONE private method (private-by-default rule; self + 2 params = the existing
   `handle_arrow_key` precedent shape):
   ```rust
   /// Keeps an egui wake scheduled at the popup deadline so the expiry is
   /// re-evaluated even while the eframe event loop sleeps at ControlFlow::Wait
   /// (eframe run.rs:297-299 - the 0.5.0 defect: passes only run on redraws,
   /// so without a wake the deadline comparison never executes). Re-requested
   /// every pass while armed: egui coalesces to the smallest pending delay
   /// (request_repaint_after) and passes that begin with a pending immediate
   /// repaint reset their delay to ZERO (begin_pass_repaint_logic), swallowing
   /// timed requests - but those passes always fire a zero-delay wake, so the
   /// next pass (delay = MAX) accepts the re-request and the wake converges
   /// onto the deadline. The margin keeps the waking pass's input.time safely
   /// past the deadline (OS timers may fire early).
   fn request_expiry_wake(&mut self, screen_context: &egui::Context, now: f64) {
       let Some(deadline) = self.esc_close_popup_expires_at else {
           return;
       };
       let remaining_seconds = deadline - now;
       if remaining_seconds <= 0.0 {
           return;
       }
       screen_context.request_repaint_after(std::time::Duration::from_secs_f64(
           remaining_seconds + POPUP_EXPIRY_WAKE_MARGIN_SECONDS,
       ));
   }
   ```
   (single-section boolean conditions per rule; `let`-else + one `if`, depth 1)
4. Header doc (lines 1–13): append ONE paragraph recording the defect + fix: the popup
   expiry needs a SCHEDULED PASS — eframe sleeps at `ControlFlow::Wait` between events
   (`run.rs:297–299`), so arming alone never re-evaluates the deadline; the arm/update
   wrappers schedule the egui wake (`request_repaint_after`) at the deadline with the
   0.05 s margin, repeated every pass while armed (coalescing convergence). `input.time`
   stays the single clock for arm AND expiry (same-source shape unchanged).

### 2.2 `src/app/keyboard_intake.rs` — UNCHANGED (verified, not assumed)

`update_esc_close_popup` is called UNCONDITIONALLY at `:53` between the reads and the
dispatch, inside `read_and_dispatch_keystrokes`, which runs every pass from `App::ui`
(`app.rs:147`). Nothing to repair there — the fix lives in the ctx wrapper the intake
already calls.

### 2.3 `src/app.rs` — ONE addition only

Add `#[cfg(test)] mod esc_close_expiry_tests;` immediately after
`#[cfg(test)] mod esc_close_confirm_tests;` (line 173; alphabetical order preserved).
195 → 197 lines, cap 200 ✓. NOTHING else in app.rs (the state field
`esc_close_popup_expires_at: Option<f64>` keeps its name, type, and value basis —
"absolute input-time seconds" is unchanged).

### 2.4 NEW `src/app/esc_close_expiry_tests.rs` (~120 lines; cap 200 ✓) — closes the round-1 gap

The ctx-level test file driving the REAL wiring. Self-contained helpers (the round-1
`escape_key_event` is private to `esc_close_confirm_tests` and is NOT cross-module
visible — define it locally; TempScope/`app_opened_in` NOT needed):

- `escape_key_event(pressed, repeat)` — the round-1 builder pattern verbatim.
- `run_timed_pass(app, context, time, events) -> egui::FullOutput` — `context.run_pass`
  with `egui::RawInput { time: Some(time), events, ..Default::default() }` and the closure
  `app.read_and_dispatch_keystrokes(frame_context)` (the PRODUCTION intake; ONE long-lived
  context across passes so `input.time` advances — unlike the round-1 fresh-per-frame
  harness). Caller drops the output via `drop_without_applying_deltas()` unless inspecting
  commands.
- `collect_wake_delays(context) -> Arc<Mutex<Vec<f64>>>` — installs
  `context.set_request_repaint_callback` (public, egui `context.rs:2268`) pushing
  `info.delay.as_secs_f64()`; fresh contexts accept the first timed request
  deterministically (`repaint_delay: Duration::MAX` at `context.rs:380`).

Tests (+4; census 150 → 154):

1. `expiry_clears_through_real_intake_after_deadline` — THE round-1-gap test:
   pass 1 `time 1.0` + distinct ESC press → assert armed AND
   `expires_at == Some(1.0 + POPUP_VISIBLE_SECONDS)`; pass 2 on the SAME context with
   `time 1.0 + POPUP_VISIBLE_SECONDS + POPUP_EXPIRY_WAKE_MARGIN_SECONDS`, no events →
   assert `!esc_close_popup_is_visible()` AND `expires_at.is_none()` — cleared by the
   REAL intake (`update_esc_close_popup` inside `run_pass`), no sleeping, no pure `_at`
   shortcut.
2. `expiry_before_deadline_keeps_popup_visible` — pass 1 arm at `time 1.0`; pass 2 at
   `time 2.5` (no events) → assert STILL visible AND `expires_at` unchanged (`3.0`).
3. `armed_popup_schedules_expiry_wake` — the failing-link regression: fresh context +
   collector; `app.update_esc_close_popup(&ctx)` while unarmed → collector EMPTY (the
   wake is not spammed); `app.arm_esc_close_popup(&ctx)` then `app.update_esc_close_popup(&ctx)`
   → assert the collector contains a delay within 1e-6 of
   `POPUP_VISIBLE_SECONDS + POPUP_EXPIRY_WAKE_MARGIN_SECONDS` (ctx time is a stable 0.0
   pre-pass, so remaining == 2.0). (Direct ctx-wrapper calls — still the production
   wrappers; the coalescing caveat that would swallow a timed request inside
   `run_pass`-driven settle passes is documented in the module doc.)
4. `wake_requests_stop_after_expiry` — arm at ctx time 0.0, drive `update_esc_close_popup`
   (collector gains the 2.05 entry), then `app.disarm_esc_close_popup()` +
   `app.update_esc_close_popup(&ctx)` → assert NO new entries (disarmed → the let-else
   no-op; pins the disarm/no-spam behavior every fullscreen toggle relies on).

Test-visibility facts: test modules are children of `app`, so the private field
`esc_close_popup_expires_at` and `ImagesScrollerApp::empty_state()` are accessible
(the round-1 file does exactly this at `esc_close_confirm_tests.rs:51`).

## 3. Proven contracts — non-regression mapping (binding)

| Contract (proven 0.4.2 + operator-passed) | Status after fix |
| --- | --- |
| Close on SECOND DISTINCT press | `esc_close_action` matrix + `dispatch_esc_close_action` + the ONE `ViewportCommand::Close` send untouched (`keyboard_intake.rs:117–134`) |
| `repeat:true` autorepeat never counts | `non_repeat_escape_press_occurred` untouched |
| Re-arm after expiry | pure `arm_esc_close_popup_at` + the dispatch arm path untouched; the wake is re-requested while armed, so a re-armed popup schedules its own next wake |
| Disarm on EVERY fullscreen toggle | `fullscreen.rs:31` `disarm_esc_close_popup()` untouched; after disarm the wrapper requests nothing (test 4) |
| Fullscreen-ON ESC exits fullscreen only | the priority branch (`keyboard_intake.rs:57–61`) untouched; the wrapper runs unarmed while ON → no wakes, no behavior change |
| Popup render (centered, non-capturing, self-gated) | `show_esc_close_popup` untouched — the popup disappears on the FIRST pass after the wake, which is the expiry pass itself |
| `POPUP_MESSAGE_TEXT` / `POPUP_VISIBLE_SECONDS = 2.0` / state field shape | untouched |
| `input.time` as the single arm+expiry clock | untouched (decided §1.3) |

## 4. Git segmentation (on `fix/wave-5-1-operator-feedback`)

1. `docs: add wave5.1 task3 plan` — this plan file (committed by the 4.2 implementer
   before the code; content is this file as-is).
2. `fix: wake the event loop so the esc-close popup expires after two seconds` — the §2
   code + §2.4 tests (exact wording fixed; scope: `esc_close_confirm.rs`,
   `esc_close_expiry_tests.rs`, the ONE mod declaration in `app.rs`).
3. Reviewer/simplifier fix commits if their cycles require (`fix: …` / `refactor: …`),
   each scoped to this task's files.
4. Gitignore Compliance Rule before every commit (`git status`; no `target/`, `dist/`,
   `logs/`, `Cargo.lock` surprises staged); NO push in this task (Step 5 pushes `main`
   to `origin` ONLY; remote-safety rule respected).

## 5. No-touch list (binding)

- `src/app/keyboard_intake.rs` — every read/dispatch line (the intake order is proven).
- The pure cores `arm_esc_close_popup_at` / `update_esc_close_popup_at`, `esc_close_action`,
  `non_repeat_escape_press_occurred`, `EscCloseAction`, `current_time`.
- `POPUP_MESSAGE_TEXT`, `POPUP_VISIBLE_SECONDS`, `ESC_CLOSE_POPUP_AREA_ID`, the popup
  render `show_esc_close_popup`, and `src/app/fullscreen.rs` (the disarm call site).
- `src/app.rs` beyond the single test-mod declaration (state field, open flow, layouts).
- `src/main.rs`, `Cargo.toml`, `Cargo.lock` (NO new dependencies — `request_repaint_after`
  and `std::time::Duration` are std/egui built-ins).
- Pan/zoom/fit/styling surfaces, `message_text` sites, `floating_controls.rs`,
  `button_hint.rs`, `pill_style.rs`, `app_display.rs`, `central_surface.rs`.
- Existing test files' assertions (round-1 tests stay as the pure-core regressions).

## 6. Budgets

- `esc_close_confirm.rs`: 146 → ~170 lines (cap 200 ✓); method bodies ≤ 12 lines.
- `src/app/esc_close_expiry_tests.rs`: NEW, ~120 lines (cap 200 ✓).
- `src/app.rs`: 195 → 197 lines (cap 200 ✓).
- Test census: 150 → 154 (+4, all in the new file); 23 → 24 test files.
- Zero dependency/feature changes; zero message writes (silent feature).

## 7. Verification (deferred, unchanged)

Harness runs stay deferred to Task 5 per the global plan: fresh dev-checks in the
container via `alpine-vm` MCP (`vm_status` first; exit code authoritative; per
`docs/build.md`: `cargo fmt --check` / `check --locked` / `test --locked` /
`clippy --locked -- -D warnings` → `ALL CHECKS PASSED`), census 154 corroboration, the
0.5.1 release rebuild + byte record, and the 22-DLL audit in Task 5. NO harness run
inside this task's cycle before Task 5. The real-Windows dismissal proof remains the
Task-5 OPERATOR-RUN sub-item (b): "the popup disappears by itself after ~2 s" — agent-side
GUI execution is impossible from the container (validation stance).

## 8. TODO Task 3 sub-item compliance check

- "Root-cause … pin the failing link (non-advancing `input.time`, a gated/skipped
  per-frame update, or another mechanism — evidence, not guesswork; cite file/line)" →
  §1.1/§1.2: both named candidates ELIMINATED by source citations; the pinned mechanism
  is the missing deadline wake (eframe `run.rs:138`, `:287–299`; egui
  `context.rs:133–172`, `:380`, `:578–579`; `input_state/mod.rs:676–700`).
- "Fix the expiry so the popup auto-dismisses after exactly `POPUP_VISIBLE_SECONDS`
  (2.0 s) on real Windows, WITHOUT regressing: close-on-second-distinct-press,
  re-arm-after-expiry, disarm-on-fullscreen-ON, fullscreen-ON ESC exit" → §1.3/§2/§3
  (wake at `deadline + 0.05 s` margin — the operator-facing "~2 s"; every proven contract
  mapped non-regressed; deadline basis unchanged).
- "Add the missing ctx-level unit test … drive `Context::run_pass` with advancing
  `RawInput.time` … assert the popup clears at the deadline through the REAL intake
  wiring" → §2.4 tests 1–2 (one long-lived context, explicit `time` values, the production
  `read_and_dispatch_keystrokes` inside `run_pass`, no sleeping, no `_at` shortcut) plus
  the wake-scheduling/no-spam regressions (tests 3–4). Census delta pinned: +4 → 154.
