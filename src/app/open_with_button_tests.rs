//! Test module of `open_with_button`: the pure availability, tooltip and
//! launch-failure seams of the Wave-6 Task-3 menu button. Container
//! determinism comes from the stub platform: enumeration is ALWAYS empty and
//! the launcher ALWAYS fails.

use super::folder_navigation_tests::{TempParentScope, app_opened_in};
use super::open_with_button::{NO_PROGRAMS_TOOLTIP_TEXT, no_programs_tooltip_for};
use super::tests::sample_decoded_image;
use super::{ImagesScrollerApp, NavigationDirection};
use crate::open_with_listing::{OPEN_WITH_LAUNCH_FAILURE_MESSAGE, RegisteredProgram};
use std::path::PathBuf;

fn two_programs_fixture() -> Vec<RegisteredProgram> {
    vec![
        RegisteredProgram {
            display_name: String::from("Paint"),
            launch_index: 0,
        },
        RegisteredProgram {
            display_name: String::from("Photos"),
            launch_index: 1,
        },
    ]
}

#[test]
fn open_with_menu_available_when_image_open_and_programs_listed() {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    app.open_with_programs = two_programs_fixture();
    assert!(app.open_with_menu_is_available());
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
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    app.launch_open_with_program(0);
    assert_eq!(app.message_text, Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE));
}

#[test]
fn every_successful_open_reenumerates_and_replaces_the_stored_list() {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    app.open_with_programs = two_programs_fixture();
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
