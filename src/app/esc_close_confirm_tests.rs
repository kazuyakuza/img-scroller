//! ESC-close-confirm tests (Wave 4.2 A-Task 4): the pure popup state machine
//! (arm / expiry / re-arm at explicit times), the two-distinct-press input
//! filter and branch decision, and the production-intake interplay with
//! fullscreen through the egui-`Context` harness (the zoom_alias_tests
//! pattern). Expiry is tested only through the pure `_at(now)` cores;
//! synthetic context time is not controlled.

use eframe::egui;

use super::esc_close_confirm::{
    EscCloseAction, POPUP_VISIBLE_SECONDS, esc_close_action, non_repeat_escape_press_occurred,
};
use super::folder_navigation_tests::{TempParentScope, app_opened_in};
use super::fullscreen::FULLSCREEN_TOGGLE_KEY;
use super::{ImagesScrollerApp, NavigationDirection};

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

#[test]
fn popup_starts_invisible() {
    let app = ImagesScrollerApp::empty_state();
    assert!(!app.esc_close_popup_is_visible());
    assert!(app.esc_close_popup_expires_at.is_none());
}

#[test]
fn arm_makes_popup_visible() {
    let mut app = ImagesScrollerApp::empty_state();
    app.arm_esc_close_popup_at(ARM_TIME);
    assert!(app.esc_close_popup_is_visible());
    assert_eq!(app.esc_close_popup_expires_at, Some(ARM_TIME + POPUP_VISIBLE_SECONDS));
}

#[test]
fn update_before_deadline_keeps_popup_visible() {
    let mut app = ImagesScrollerApp::empty_state();
    app.arm_esc_close_popup_at(ARM_TIME);
    app.update_esc_close_popup_at(ARM_TIME + POPUP_VISIBLE_SECONDS - 0.5);
    assert!(app.esc_close_popup_is_visible());
    assert_eq!(app.esc_close_popup_expires_at, Some(ARM_TIME + POPUP_VISIBLE_SECONDS));
}

#[test]
fn update_at_deadline_clears_popup() {
    let mut app = ImagesScrollerApp::empty_state();
    app.arm_esc_close_popup_at(ARM_TIME);
    app.update_esc_close_popup_at(ARM_TIME + POPUP_VISIBLE_SECONDS);
    assert!(!app.esc_close_popup_is_visible());
    assert!(app.esc_close_popup_expires_at.is_none());
    app.update_esc_close_popup_at(10.0);
    assert!(app.esc_close_popup_expires_at.is_none());
}

#[test]
fn non_repeat_escape_press_detected() {
    assert!(non_repeat_escape_press_occurred(&[escape_key_event(true, false)]));
    assert!(!non_repeat_escape_press_occurred(&[escape_key_event(false, false)]));
    assert!(!non_repeat_escape_press_occurred(&[]));
}

#[test]
fn repeat_escape_press_is_ignored() {
    assert!(!non_repeat_escape_press_occurred(&[escape_key_event(true, true)]));
    assert!(non_repeat_escape_press_occurred(&[escape_key_event(true, false)]));
}

#[test]
fn distinct_second_press_closes_when_popup_visible() {
    assert_eq!(esc_close_action(true, true), EscCloseAction::CloseApplication);
    assert_eq!(esc_close_action(true, false), EscCloseAction::StayIdle);
    assert_eq!(esc_close_action(false, true), EscCloseAction::ShowPopup);
    assert_eq!(esc_close_action(false, false), EscCloseAction::StayIdle);
}

#[test]
fn fullscreen_toggle_disarms_popup() {
    let mut app = ImagesScrollerApp::empty_state();
    app.arm_esc_close_popup_at(ARM_TIME);
    let toggle_key_press = egui::Event::Key {
        key: FULLSCREEN_TOGGLE_KEY,
        physical_key: Some(FULLSCREEN_TOGGLE_KEY),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    };
    run_intake_frame(&mut app, vec![toggle_key_press]);
    assert!(app.fullscreen_on);
    assert!(!app.esc_close_popup_is_visible());
}

#[test]
fn fullscreen_exit_esc_does_not_arm_popup() {
    let mut app = ImagesScrollerApp::empty_state();
    app.toggle_fullscreen();
    run_intake_frame(&mut app, vec![escape_key_event(true, false)]);
    assert!(!app.fullscreen_on);
    assert!(!app.esc_close_popup_is_visible());
}

#[test]
fn expired_popup_re_arms_on_a_later_press() {
    let mut app = ImagesScrollerApp::empty_state();
    app.arm_esc_close_popup_at(ARM_TIME);
    app.update_esc_close_popup_at(ARM_TIME + POPUP_VISIBLE_SECONDS);
    assert!(!app.esc_close_popup_is_visible());
    run_intake_frame(&mut app, vec![escape_key_event(true, false)]);
    assert!(app.esc_close_popup_is_visible());
}

#[test]
fn popup_arms_in_every_fullscreen_off_state() {
    let mut fresh_app = ImagesScrollerApp::empty_state();
    run_intake_frame(&mut fresh_app, vec![escape_key_event(true, false)]);
    assert!(fresh_app.esc_close_popup_is_visible());

    let image_scope = TempParentScope::create("esc_popup_image_state");
    image_scope.create_child_folder("solo");
    image_scope.create_image("solo", "only.png");
    let mut image_app = app_opened_in(&image_scope.child_image_path("solo", "only.png"));
    run_intake_frame(&mut image_app, vec![escape_key_event(true, false)]);
    assert!(image_app.esc_close_popup_is_visible());

    let folder_scope = TempParentScope::create("esc_popup_empty_folder_state");
    folder_scope.create_child_folder("folder1");
    folder_scope.create_child_folder("folder2");
    folder_scope.create_image("folder2", "b.png");
    let mut empty_folder_app = app_opened_in(&folder_scope.child_image_path("folder2", "b.png"));
    empty_folder_app.navigate(NavigationDirection::PreviousSiblingFolder);
    assert_eq!(
        empty_folder_app.browsed_folder_context,
        Some(folder_scope.child_folder_path("folder1"))
    );
    run_intake_frame(&mut empty_folder_app, vec![escape_key_event(true, false)]);
    assert!(empty_folder_app.esc_close_popup_is_visible());
}
