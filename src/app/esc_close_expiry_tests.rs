//! ESC-close popup EXPIRY tests (Wave 5.1 Task 3): the round-1 gap was the
//! missing scheduled pass. The pure `_at(now)` cores were tested with explicit
//! times, but the real per-frame wiring never saw an advancing egui clock, so
//! the shipped 0.5.0 defect (no wake; eframe sleeps at `ControlFlow::Wait`)
//! stayed invisible. These tests drive ONE long-lived egui `Context` whose
//! `RawInput.time` advances across passes and run the PRODUCTION intake
//! (`read_and_dispatch_keystrokes` -> `update_esc_close_popup`), plus the
//! wake-scheduling/no-spam regressions through the public repaint callback.
//!
//! egui coalescing caveat: `request_repaint_after` fires the callback only
//! when the requested delay is smaller than the pending one, and a pass that
//! begins with an outstanding repaint resets its delay to ZERO - so a timed
//! request issued inside a `run_pass`-driven settle pass may be swallowed.
//! Those passes always fire a zero-delay wake themselves, and the update
//! wrapper re-requests every pass while armed, so the wake converges onto the
//! deadline. The wake tests below call the ctx wrappers directly (no settle
//! pass in between) to pin the request shape deterministically.

use std::sync::{Arc, Mutex};

use eframe::egui;

use super::ImagesScrollerApp;
use super::esc_close_confirm::{POPUP_EXPIRY_WAKE_MARGIN_SECONDS, POPUP_VISIBLE_SECONDS};

const ARM_TIME: f64 = 1.0;

/// ESC key event builder (the round-1 `esc_close_confirm_tests` shape; that
/// builder is private to its own module).
fn escape_key_event(pressed: bool, repeat: bool) -> egui::Event {
    egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: Some(egui::Key::Escape),
        pressed,
        repeat,
        modifiers: egui::Modifiers::default(),
    }
}

/// One app plus ONE long-lived egui context, so `input.time` advances across
/// passes (the round-1 harness built a fresh context per frame and could never
/// exercise the time/deadline wiring).
struct TimedIntakeHarness {
    app: ImagesScrollerApp,
    context: egui::Context,
}

impl TimedIntakeHarness {
    fn with_empty_app() -> Self {
        Self {
            app: ImagesScrollerApp::empty_state(),
            context: egui::Context::default(),
        }
    }

    /// Runs ONE production intake frame at the given `RawInput.time` on the
    /// SAME context (no sleeping, no pure `_at` shortcut).
    fn run_pass(&mut self, time: f64, events: Vec<egui::Event>) {
        let Self { app, context } = self;
        context
            .run_pass(
                egui::RawInput {
                    time: Some(time),
                    events,
                    ..Default::default()
                },
                |frame_context| app.read_and_dispatch_keystrokes(frame_context),
            )
            .drop_without_applying_deltas();
    }
}

/// Installs the public repaint callback and records every requested delay in
/// seconds (a fresh context accepts the first timed request: its pending
/// repaint delay starts at `Duration::MAX`).
fn collect_wake_delays(context: &egui::Context) -> Arc<Mutex<Vec<f64>>> {
    let recorded_wake_delays = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&recorded_wake_delays);
    context.set_request_repaint_callback(move |repaint_info| {
        sink.lock()
            .expect("wake-delay collector lock")
            .push(repaint_info.delay.as_secs_f64());
    });
    recorded_wake_delays
}

#[test]
fn expiry_clears_through_real_intake_after_deadline() {
    let mut harness = TimedIntakeHarness::with_empty_app();
    harness.run_pass(ARM_TIME, vec![escape_key_event(true, false)]);
    assert!(harness.app.esc_close_popup_is_visible());
    assert_eq!(
        harness.app.esc_close_popup_expires_at,
        Some(ARM_TIME + POPUP_VISIBLE_SECONDS)
    );

    harness.run_pass(
        ARM_TIME + POPUP_VISIBLE_SECONDS + POPUP_EXPIRY_WAKE_MARGIN_SECONDS,
        Vec::new(),
    );
    assert!(!harness.app.esc_close_popup_is_visible());
    assert!(harness.app.esc_close_popup_expires_at.is_none());
}

#[test]
fn expiry_before_deadline_keeps_popup_visible() {
    let mut harness = TimedIntakeHarness::with_empty_app();
    harness.run_pass(ARM_TIME, vec![escape_key_event(true, false)]);
    harness.run_pass(2.5, Vec::new());
    assert!(harness.app.esc_close_popup_is_visible());
    assert_eq!(
        harness.app.esc_close_popup_expires_at,
        Some(ARM_TIME + POPUP_VISIBLE_SECONDS)
    );
}

#[test]
fn armed_popup_schedules_expiry_wake() {
    let mut app = ImagesScrollerApp::empty_state();
    let context = egui::Context::default();
    let recorded_wake_delays = collect_wake_delays(&context);

    app.update_esc_close_popup(&context);
    assert!(recorded_wake_delays.lock().unwrap().is_empty());

    app.arm_esc_close_popup(&context);
    app.update_esc_close_popup(&context);
    let expected_wake_delay = POPUP_VISIBLE_SECONDS + POPUP_EXPIRY_WAKE_MARGIN_SECONDS;
    let wake_scheduled = recorded_wake_delays
        .lock()
        .unwrap()
        .iter()
        .any(|delay| (delay - expected_wake_delay).abs() < 1e-6);
    assert!(wake_scheduled);
}

#[test]
fn wake_requests_stop_after_expiry() {
    let mut app = ImagesScrollerApp::empty_state();
    let context = egui::Context::default();
    let recorded_wake_delays = collect_wake_delays(&context);

    app.arm_esc_close_popup(&context);
    app.update_esc_close_popup(&context);
    let wake_count_before_disarm = recorded_wake_delays.lock().unwrap().len();
    assert!(wake_count_before_disarm > 0);

    app.disarm_esc_close_popup();
    app.update_esc_close_popup(&context);
    assert_eq!(
        recorded_wake_delays.lock().unwrap().len(),
        wake_count_before_disarm
    );
}
