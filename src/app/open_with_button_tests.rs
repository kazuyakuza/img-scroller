//! Test module of `open_with_button`: the availability, tooltip, launch and
//! Wave-6.1 Q-open/modal-gating seams (stub platform: no programs, launch fails).

use eframe::egui;

use super::esc_close_confirm_tests::key_event;
use super::folder_navigation_tests::{TempParentScope, app_opened_in};
use super::open_with_button::{
    NO_PROGRAMS_TOOLTIP_TEXT, message_line_is_warning, no_programs_tooltip_for, open_with_menu_id,
};
use super::tests::sample_decoded_image;
use super::{ImagesScrollerApp, NavigationDirection};
use crate::image_loading::{ImageLoadFailure, UNSUPPORTED_FILE_MESSAGE};
use crate::open_with_listing::{OPEN_WITH_LAUNCH_FAILURE_MESSAGE, RegisteredProgram};
use std::path::PathBuf;

fn two_programs_fixture() -> Vec<RegisteredProgram> {
    [("Paint", 0), ("Photos", 1)]
        .into_iter()
        .map(|(display_name, launch_index)| RegisteredProgram {
            display_name: String::from(display_name),
            launch_index,
        })
        .collect()
}

fn app_with_programs_stored() -> ImagesScrollerApp {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    app.open_with_programs = two_programs_fixture();
    app
}

/// One app plus ONE long-lived context; `open_menu_first` opens the popup
/// through the production id before the intake runs (the modal-guard frame).
struct MenuIntakeHarness {
    app: ImagesScrollerApp,
    context: egui::Context,
}

impl MenuIntakeHarness {
    fn with_image_open_and_programs_stored() -> Self {
        Self {
            app: app_with_programs_stored(),
            context: egui::Context::default(),
        }
    }

    fn pass(&mut self, events: Vec<egui::Event>, open_menu_first: bool) {
        let mut frame_input = egui::RawInput::default();
        frame_input.events = events;
        self.context
            .run_pass(frame_input, |frame_context| {
                if open_menu_first {
                    egui::Popup::open_id(frame_context, open_with_menu_id());
                }
                self.app.read_and_dispatch_keystrokes(frame_context);
            })
            .drop_without_applying_deltas();
    }

    fn menu_is_open(&self) -> bool {
        egui::Popup::is_id_open(&self.context, open_with_menu_id())
    }
}

#[test]
fn open_with_menu_available_when_image_open_and_programs_listed() {
    assert!(app_with_programs_stored().open_with_menu_is_available());
}

#[test]
fn open_with_menu_unavailable_when_no_image_is_open() {
    let mut app = ImagesScrollerApp::empty_state();
    app.open_with_programs = two_programs_fixture();
    assert!(!app.open_with_menu_is_available());
}

#[test]
fn open_with_menu_unavailable_while_the_list_is_empty() {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    assert_eq!(app.opened_image_path, Some(PathBuf::from("photo.png")));
    assert!(!app.open_with_menu_is_available());
}

#[test]
fn no_programs_tooltip_applies_only_while_a_file_is_open() {
    let mut app = ImagesScrollerApp::empty_state();
    assert_eq!(no_programs_tooltip_for(&app), None);
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    assert_eq!(
        no_programs_tooltip_for(&app),
        Some(NO_PROGRAMS_TOOLTIP_TEXT)
    );
}

#[test]
fn launch_failure_surfaces_the_static_failure_message() {
    let mut app = app_with_programs_stored();
    app.launch_open_with_program(0);
    assert_eq!(app.message_text, Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE));
}

#[test]
fn every_successful_open_reenumerates_and_replaces_the_stored_list() {
    let mut app = app_with_programs_stored();
    app.accept_open_outcome(PathBuf::from("other.png"), Ok(sample_decoded_image()));
    assert!(app.open_with_programs.is_empty());
}

#[test]
fn entering_empty_folder_mode_clears_the_stored_list() {
    let scope = TempParentScope::create("open_with_empty_mode");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_image("folder1", "img.png");
    let mut app = app_opened_in(&scope.child_image_path("folder1", "img.png"));
    app.open_with_programs = two_programs_fixture();
    app.navigate(NavigationDirection::NextSiblingFolder);
    assert!(app.open_with_programs.is_empty());
    assert!(!app.open_with_menu_is_available());
}

#[test]
fn open_with_menu_should_not_open_on_q_when_fullscreen_on() {
    let mut app = app_with_programs_stored();
    app.toggle_fullscreen();
    assert!(!app.open_with_menu_should_open_on_q(true));
}

#[test]
fn open_with_menu_should_not_open_on_q_when_no_programs_listed() {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    assert!(!app.open_with_menu_should_open_on_q(true));
}

#[test]
fn q_press_opens_the_menu_and_refreshes_the_stored_list() {
    let mut harness = MenuIntakeHarness::with_image_open_and_programs_stored();
    harness.pass(vec![key_event(egui::Key::Q, true, false)], false);
    assert!(harness.menu_is_open());
    assert!(harness.app.open_with_programs.is_empty());
}

#[test]
fn q_press_closes_the_open_menu() {
    let mut harness = MenuIntakeHarness::with_image_open_and_programs_stored();
    harness.pass(vec![key_event(egui::Key::Q, true, false)], true);
    assert!(!harness.menu_is_open());
    assert!(harness.app.full_ui_chrome_is_visible());
    assert!(harness.app.message_text.is_none());
}

#[test]
fn escape_closes_the_open_menu_without_arming_the_esc_close_popup() {
    let mut harness = MenuIntakeHarness::with_image_open_and_programs_stored();
    harness.pass(vec![key_event(egui::Key::Escape, true, false)], true);
    assert!(!harness.menu_is_open());
    assert!(!harness.app.esc_close_popup_is_visible());
    assert!(harness.app.esc_close_popup_expires_at.is_none());
}

#[test]
fn other_keys_are_suppressed_while_the_menu_is_open() {
    let mut harness = MenuIntakeHarness::with_image_open_and_programs_stored();
    let pressed_keys = vec![
        key_event(egui::Key::F11, true, false),
        key_event(egui::Key::I, true, false),
    ];
    harness.pass(pressed_keys, true);
    assert!(harness.menu_is_open());
    assert!(harness.app.full_ui_chrome_is_visible());
    assert_eq!(harness.app.zoom_factor, crate::zoom::RESTORE_TO_FIT_FACTOR);
}

#[test]
fn launch_failure_message_is_warning_flagged() {
    let mut app = app_with_programs_stored();
    app.launch_open_with_program(0);
    assert_eq!(app.message_text, Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE));
    assert!(message_line_is_warning(app.message_text));
}

#[test]
fn unsupported_file_message_is_not_warning_flagged() {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("bad.txt"), Err(ImageLoadFailure::Unsupported));
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert!(!message_line_is_warning(app.message_text));
}

#[test]
fn message_line_is_warning_matches_only_the_launch_failure() {
    let launch_failure = Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE);
    assert!(!message_line_is_warning(None));
    assert!(!message_line_is_warning(Some(UNSUPPORTED_FILE_MESSAGE)));
    assert!(message_line_is_warning(launch_failure));
}
