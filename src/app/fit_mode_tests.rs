//! App-state fit-mode tests (Wave-4 Task 3): initial mode, activation and
//! feedback, zoom interplay, the reset-on-open policy, and the pinned hotkey /
//! button-label literals with the open-image enablement gate.

use eframe::egui;

use super::tests::{TempImageFolder, sample_decoded_image};
use super::{ImagesScrollerApp, fit_mode_button, zoom};

#[test]
fn fresh_app_starts_in_the_whole_image_fit_mode() {
    let app = ImagesScrollerApp::empty_state();
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);
    assert_eq!(app.zoom_factor, crate::zoom::RESTORE_TO_FIT_FACTOR);
}

#[test]
fn fit_width_activation_resets_the_factor_and_announces_fit_width() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitWidth);
    assert_eq!(app.zoom_factor, 1.0);
    assert_eq!(app.message_text, Some(zoom::FIT_WIDTH_MESSAGE));
}

#[test]
fn fit_height_activation_resets_the_factor_and_announces_fit_height() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitHeight);
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitHeight);
    assert_eq!(app.zoom_factor, 1.0);
    assert_eq!(app.message_text, Some(zoom::FIT_HEIGHT_MESSAGE));
}

#[test]
fn fit_visible_activation_is_silent_and_resets_the_mode_and_factor() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    assert!(app.message_text.is_some());
    app.set_fit_mode(crate::scaling::FitMode::FitVisible);
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
}

#[test]
fn zoom_steps_keep_the_active_fit_mode() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    app.zoom_in_once();
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitWidth);
    assert_eq!(app.zoom_factor, 1.25);
}

#[test]
fn restore_fit_returns_to_the_whole_image_mode_at_the_fit_factor() {
    let mut app = ImagesScrollerApp::empty_state();
    app.set_fit_mode(crate::scaling::FitMode::FitHeight);
    app.zoom_in_once();
    app.restore_zoom_to_fit();
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);
    assert_eq!(app.zoom_factor, 1.0);
}

#[test]
fn zoom_out_at_fit_is_silent_then_fit_width_announces() {
    let mut app = ImagesScrollerApp::empty_state();
    app.zoom_out_once();
    assert!(app.message_text.is_none());
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    assert_eq!(app.message_text, Some(zoom::FIT_WIDTH_MESSAGE));
}

#[test]
fn every_successful_open_returns_to_the_whole_image_fit() {
    let folder = TempImageFolder::create("fit_reset_on_open");
    folder.create_file("a.png");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.set_fit_mode(crate::scaling::FitMode::FitWidth);
    app.zoom_in_once();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
}

#[test]
fn fit_mode_hotkeys_are_pinned_to_the_w_and_h_keys() {
    assert_eq!(zoom::FIT_WIDTH_KEY, egui::Key::W);
    assert_eq!(zoom::FIT_HEIGHT_KEY, egui::Key::H);
}

#[test]
fn fit_mode_button_labels_are_pinned() {
    assert_eq!(fit_mode_button::FIT_WIDTH_BUTTON_LABEL, "Fit width");
    assert_eq!(fit_mode_button::FIT_HEIGHT_BUTTON_LABEL, "Fit height");
}

#[test]
fn fit_mode_button_enablement_gate_is_the_open_image_state() {
    let mut app = ImagesScrollerApp::empty_state();
    assert!(!app.image_is_open());
    let folder = TempImageFolder::create("fit_button_gate");
    folder.create_file("a.png");
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert!(app.image_is_open());
}
