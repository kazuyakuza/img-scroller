//! App-state zoom tests: the session zoom factor seam, its bounds and silent
//! at-bound steps, the reset-on-open policy, and the pinned hotkey /
//! button-label literals.

use eframe::egui;

use super::tests::{TempImageFolder, sample_decoded_image};
use super::{ImagesScrollerApp, zoom, zoom_button};
use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;
use std::path::PathBuf;

#[test]
fn fresh_app_starts_zoomed_to_fit() {
    let app = ImagesScrollerApp::empty_state();
    assert_eq!(app.zoom_factor, crate::zoom::RESTORE_TO_FIT_FACTOR);
    assert!(!app.zoomed_off_fit()); // the restore-fit button is DISABLED at fit
    assert!(app.zoom_can_shrink()); // zoom-out is now ENABLED at fit (goes below)
    assert!(app.zoom_can_grow());
}

#[test]
fn zoom_in_once_zooms_beyond_fit() {
    let mut app = ImagesScrollerApp::empty_state();
    app.zoom_in_once();
    assert_eq!(app.zoom_factor, 1.25);
    assert!(app.zoomed_off_fit());
    assert!(app.zoom_can_shrink());
    assert!(app.zoom_can_grow());
}

#[test]
fn restore_zoom_to_fit_returns_to_fit() {
    let mut app = ImagesScrollerApp::empty_state();
    app.zoom_in_once();
    app.restore_zoom_to_fit();
    assert_eq!(app.zoom_factor, 1.0);
}

#[test]
fn zoom_out_from_fit_shrinks_below_fit() {
    let mut app = ImagesScrollerApp::empty_state();
    app.zoom_out_once();
    assert_eq!(app.zoom_factor, 0.8); // 1.0 / 1.25, exact in f32
    assert!(app.zoom_can_shrink());
    assert!(app.message_text.is_none()); // silent -- Task-4 binding in force
    app.zoom_out_once();
    assert_eq!(app.zoom_factor, 0.64);
    assert!(app.message_text.is_none());
}

#[test]
fn repeated_zoom_out_steps_reach_the_floor_then_stay() {
    let mut app = ImagesScrollerApp::empty_state();
    for _ in 0..10 {
        app.zoom_out_once(); //            10 × ÷1.25 from 1.0 ≈ 0.107 < 0.125
    }
    assert_eq!(app.zoom_factor, crate::zoom::ZOOM_FACTOR_MINIMUM);
    assert!(!app.zoom_can_shrink()); //    disabled at the floor
    assert!(app.zoomed_off_fit()); //      restore-fit still ENABLED at the floor
    app.zoom_out_once(); //                inert silent no-op at the floor
    assert!(app.message_text.is_none());
    assert_eq!(app.zoom_factor, crate::zoom::ZOOM_FACTOR_MINIMUM);
}

#[test]
fn repeated_zoom_in_reaches_the_ceiling_then_stays_there() {
    let mut app = ImagesScrollerApp::empty_state();
    for _ in 0..10 {
        app.zoom_in_once();
    }
    assert!(!app.zoom_can_grow());
    assert_eq!(app.zoom_factor, 8.0);
    app.zoom_in_once();
    assert!(app.message_text.is_none());
    assert_eq!(app.zoom_factor, 8.0);
}

#[test]
fn restore_fit_hotkey_is_num0() {
    assert_eq!(zoom::RESTORE_FIT_KEY, egui::Key::Num0);
}

#[test]
fn fullscreen_toggles_do_not_touch_the_zoom_state() {
    let mut app = ImagesScrollerApp::empty_state();
    app.zoom_in_once();
    app.toggle_fullscreen();
    assert!(!app.full_ui_chrome_is_visible());
    app.toggle_fullscreen();
    assert!(app.full_ui_chrome_is_visible());
    assert_eq!(app.zoom_factor, 1.25);
}

#[test]
fn every_successful_open_resets_the_zoom_to_fit() {
    let folder = TempImageFolder::create("zoom_reset_on_open");
    folder.create_file("a.png");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.zoom_in_once();
    app.zoom_in_once();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
}

#[test]
fn failed_open_keeps_the_zoom_factor() {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    app.zoom_in_once();
    app.open_image(PathBuf::from("unregistered-in-container.bin"));
    assert_eq!(app.zoom_factor, 1.25);
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert!(app.decoded_image.is_some());
}

#[test]
fn zoom_button_accessible_labels_are_pinned() {
    assert_eq!(zoom_button::ZOOM_IN_BUTTON_LABEL, "Zoom in");
    assert_eq!(zoom_button::ZOOM_OUT_BUTTON_LABEL, "Zoom out");
    assert_eq!(zoom_button::RESTORE_FIT_BUTTON_LABEL, "Restore zoom to fit");
}

