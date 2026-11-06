//! One-shot fit app-state tests (Wave 4.1 Task 7): the factor-only set, its
//! silence, the no-op gates, the reset-to-1.0 policy, and the pinned hotkey /
//! button-label literals with the open-image enablement gate.

use eframe::egui;
use std::path::PathBuf;

use super::tests::sample_decoded_image;
use super::{ImagesScrollerApp, fit_button, zoom};
use crate::image_loading::DecodedImage;

const ONE_SHOT_AVAILABLE_SIZE: egui::Vec2 = egui::Vec2::new(400.0, 400.0);

fn tall_decoded_image() -> DecodedImage {
    // fitted at ONE_SHOT_AVAILABLE_SIZE = (200, 400): the WIDTH-first zoom
    // factor is 400/200 = 2.0, the HEIGHT-fit factor is 1.0 (height constrains).
    DecodedImage {
        width: 100,
        height: 200,
        pixels: vec![0; 100 * 200 * 4],
    }
}

fn wide_decoded_image() -> DecodedImage {
    // Mirror image: the HEIGHT one-shot factor is 2.0, the WIDTH one-shot 1.0.
    DecodedImage {
        width: 200,
        height: 100,
        pixels: vec![0; 200 * 100 * 4],
    }
}

fn open_decoded_image(image: DecodedImage) -> ImagesScrollerApp {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(image));
    app
}

#[test]
fn fit_width_sets_zoom_factor_to_width_fit_ratio() {
    let mut app = open_decoded_image(tall_decoded_image());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, 2.0); // avail.x / whole-image-fitted-width, exact
}

#[test]
fn fit_height_sets_zoom_factor_to_height_fit_ratio() {
    let mut app = open_decoded_image(wide_decoded_image());
    app.set_zoom_to_fit_height(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, 2.0); // avail.y / whole-image-fitted-height, exact
}

#[test]
fn fit_width_is_silent() {
    let mut app = open_decoded_image(tall_decoded_image());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    assert!(app.message_text.is_none());
}

#[test]
fn fit_height_is_silent() {
    let mut app = open_decoded_image(wide_decoded_image());
    app.set_zoom_to_fit_height(ONE_SHOT_AVAILABLE_SIZE);
    assert!(app.message_text.is_none());
}

#[test]
fn pressing_fit_width_twice_recomputes_the_same_factor() {
    let mut app = open_decoded_image(tall_decoded_image());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, 2.0);
    // No persistent mode may block zoom: steps run normal from the set level.
    app.zoom_out_once();
    assert_eq!(app.zoom_factor, 1.6); // 2.0 / 1.25, exact
    app.zoom_in_once();
    assert_eq!(app.zoom_factor, 2.0);
}

#[test]
fn fit_action_is_no_op_when_no_image_is_open() {
    let mut app = ImagesScrollerApp::empty_state();
    // The one `image_is_open` gate covers the fresh empty state AND the
    // browsed empty-folder state (both keep `opened_image_path` None).
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    app.set_zoom_to_fit_height(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, crate::zoom::RESTORE_TO_FIT_FACTOR);
    assert!(app.message_text.is_none());
}

#[test]
fn restore_fit_and_open_reset_zoom_factor_to_one() {
    let mut app = open_decoded_image(tall_decoded_image());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, 2.0);
    app.restore_zoom_to_fit();
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    app.accept_open_outcome(PathBuf::from("other.png"), Ok(wide_decoded_image()));
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
}

#[test]
fn fit_width_hotkey_is_w() {
    assert_eq!(zoom::FIT_WIDTH_KEY, egui::Key::W);
}

#[test]
fn fit_height_hotkey_is_h() {
    assert_eq!(zoom::FIT_HEIGHT_KEY, egui::Key::H);
}

#[test]
fn fit_width_button_label_is_pinned() {
    assert_eq!(fit_button::FIT_WIDTH_BUTTON_LABEL, "Fit width");
}

#[test]
fn fit_height_button_label_is_pinned() {
    assert_eq!(fit_button::FIT_HEIGHT_BUTTON_LABEL, "Fit height");
}

#[test]
fn fit_buttons_enabled_only_when_image_is_open() {
    let mut app = ImagesScrollerApp::empty_state();
    assert!(!app.image_is_open());
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    assert!(app.image_is_open());
}
