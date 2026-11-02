//! Fullscreen unit tests: launch state, the pinned F11 shortcut, the shared
//! toggle's exactly-once flip, and the one chrome-visibility gate in every
//! reachable mode.

use super::ImagesScrollerApp;
use super::fullscreen::FULLSCREEN_TOGGLE_KEY;
use super::tests::{TempImageFolder, sample_decoded_image};
use eframe::egui;

#[test]
fn fresh_app_starts_with_fullscreen_off() {
    let app = ImagesScrollerApp::empty_state();
    assert!(!app.fullscreen_on);
    assert!(app.full_ui_chrome_is_visible());
}

#[test]
fn fullscreen_shortcut_is_f11() {
    assert_eq!(FULLSCREEN_TOGGLE_KEY, egui::Key::F11);
}

#[test]
fn each_toggle_trigger_flips_the_flag_exactly_once() {
    let mut app = ImagesScrollerApp::empty_state();
    assert!(app.toggle_fullscreen());
    assert!(app.fullscreen_on);
    assert!(!app.toggle_fullscreen());
    assert!(!app.fullscreen_on);
    assert!(app.toggle_fullscreen());
    assert!(app.fullscreen_on);
}

#[test]
fn fullscreen_on_hides_the_chrome_with_an_image_open() {
    let folder = TempImageFolder::create("fullscreen_hides_chrome");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    assert!(app.full_ui_chrome_is_visible());
    app.toggle_fullscreen();
    assert!(!app.full_ui_chrome_is_visible());
    assert!(app.fullscreen_on);
}

#[test]
fn fullscreen_off_restores_the_chrome_in_every_reachable_mode() {
    let fresh_app = ImagesScrollerApp::empty_state();
    assert!(fresh_app.full_ui_chrome_is_visible());
    let folder = TempImageFolder::create("fullscreen_restore_chrome");
    folder.create_file("b.png");
    let mut image_app = ImagesScrollerApp::empty_state();
    image_app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    assert!(image_app.navigation_context_is_active());
    assert!(image_app.full_ui_chrome_is_visible());
    let mut toggled_app = ImagesScrollerApp::empty_state();
    toggled_app.toggle_fullscreen();
    assert!(!toggled_app.full_ui_chrome_is_visible());
    toggled_app.toggle_fullscreen();
    assert!(toggled_app.full_ui_chrome_is_visible());
}

#[test]
fn navigation_context_stays_active_across_fullscreen_toggles() {
    let folder = TempImageFolder::create("fullscreen_navigation_context");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.toggle_fullscreen();
    assert!(app.navigation_context_is_active());
    app.toggle_fullscreen();
    assert!(app.navigation_context_is_active());
}

#[test]
fn successful_open_during_fullscreen_keeps_fullscreen_on() {
    let folder = TempImageFolder::create("fullscreen_open_stays_on");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.toggle_fullscreen();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    assert!(!app.full_ui_chrome_is_visible());
    assert!(app.decoded_image.is_some());
    assert!(app.fullscreen_on);
}
