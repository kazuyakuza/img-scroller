//! App-state pan tests: the pan-offset reset semantics, the per-axis
//! pan-or-navigate routing table, the landing-press clamp, the repeat
//! absorption at edges, the vertical Wave-3 trigger, and the no-image
//! direct-navigation route. Every controller call injects its available size.

use eframe::egui;

use super::folder_navigation_tests::{TempParentScope, app_opened_in};
use super::pan::ArrowKeyPress;
use super::tests::{TempImageFolder, sample_decoded_image};
use super::{ImagesScrollerApp, NavigationDirection};
use crate::pan;
use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;

fn open_first_of_two(test_case: &str) -> (TempImageFolder, ImagesScrollerApp) {
    let folder = TempImageFolder::create(test_case);
    folder.create_file("a.png");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    (folder, app)
}

#[test]
fn fresh_app_starts_with_zero_pan_offset() {
    let app = ImagesScrollerApp::empty_state();
    assert_eq!(app.pan_offset, egui::Vec2::ZERO);
}

#[test]
fn every_successful_open_resets_the_pan_offset() {
    let folder = TempImageFolder::create("pan_reset_on_open");
    folder.create_file("a.png");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.pan_offset = egui::vec2(10.0, 20.0);
    app.zoom_in_once();
    app.zoom_in_once();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert_eq!(app.pan_offset, egui::Vec2::ZERO);
    assert_eq!(app.zoom_factor, 1.0);
}

#[test]
fn restore_zoom_to_fit_resets_the_pan_offset() {
    let mut app = ImagesScrollerApp::empty_state();
    app.pan_offset = egui::vec2(10.0, 20.0);
    app.restore_zoom_to_fit();
    assert_eq!(app.pan_offset, egui::Vec2::ZERO);
}

#[test]
fn one_shot_fit_sets_do_not_reset_the_pan_offset() {
    let mut app = ImagesScrollerApp::empty_state();
    app.pan_offset = egui::vec2(10.0, 20.0);
    app.set_zoom_to_fit_width(egui::vec2(800.0, 300.0));
    assert_eq!(app.pan_offset, egui::vec2(10.0, 20.0));
}

#[test]
fn zoom_steps_do_not_reset_the_pan_offset() {
    let mut app = ImagesScrollerApp::empty_state();
    app.pan_offset = egui::vec2(10.0, 20.0);
    app.zoom_in_once();
    assert_eq!(app.pan_offset, egui::vec2(10.0, 20.0));
}

#[test]
fn arrow_pans_the_overflowing_axis_while_not_at_the_edge() {
    let (_folder, mut app) = open_first_of_two("pan_overflowing_axis");
    app.zoom_factor = 8.0;
    let available_size = egui::vec2(800.0, 300.0);
    let next_press = ArrowKeyPress {
        direction: NavigationDirection::Next,
        non_repeat: true,
    };
    app.handle_arrow_key(available_size, next_press);
    assert_eq!(app.pan_offset.x, 100.0);
    let previous_press = ArrowKeyPress {
        direction: NavigationDirection::Previous,
        non_repeat: true,
    };
    app.handle_arrow_key(available_size, previous_press);
    assert_eq!(app.pan_offset.x, 0.0);
}

#[test]
fn the_edge_landing_press_clamps_without_navigating() {
    let (folder, mut app) = open_first_of_two("pan_landing_press");
    app.zoom_factor = 8.0;
    let available_size = egui::vec2(800.0, 300.0);
    app.pan_offset.x = pan::axis_pan_limit(1600.0) - 40.0;
    let next_press = ArrowKeyPress {
        direction: NavigationDirection::Next,
        non_repeat: true,
    };
    app.handle_arrow_key(available_size, next_press);
    assert_eq!(app.pan_offset.x, pan::axis_pan_limit(1600.0));
    assert_eq!(app.opened_image_path, Some(folder.image_path("a.png")));
}

#[test]
fn a_distinct_press_at_the_right_edge_navigates_to_the_next_image() {
    let (folder, mut app) = open_first_of_two("pan_right_edge_navigation");
    app.zoom_factor = 8.0;
    app.pan_offset.x = pan::axis_pan_limit(1600.0);
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Next),
        Some(folder.image_path("b.png"))
    );
    let next_press = ArrowKeyPress {
        direction: NavigationDirection::Next,
        non_repeat: true,
    };
    app.handle_arrow_key(egui::vec2(800.0, 300.0), next_press);
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(app.opened_image_path, Some(folder.image_path("a.png")));
}

#[test]
fn a_repeat_press_at_an_edge_is_absorbed() {
    let (folder, mut app) = open_first_of_two("pan_repeat_absorption");
    app.zoom_factor = 8.0;
    app.pan_offset.x = -pan::axis_pan_limit(1600.0);
    let offset_before_press = app.pan_offset;
    let previous_press = ArrowKeyPress {
        direction: NavigationDirection::Previous,
        non_repeat: false,
    };
    app.handle_arrow_key(egui::vec2(800.0, 300.0), previous_press);
    assert_eq!(app.pan_offset, offset_before_press);
    assert_eq!(app.opened_image_path, Some(folder.image_path("a.png")));
}

#[test]
fn a_distinct_press_at_the_bottom_edge_navigates_to_the_next_sibling_folder() {
    let scope = TempParentScope::create("pan_bottom_edge_navigation");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_image("folder1", "a.png");
    scope.create_image("folder2", "b.png");
    let mut app = app_opened_in(&scope.child_image_path("folder1", "a.png"));
    app.zoom_factor = 8.0;
    let available_size = egui::vec2(300.0, 800.0);
    let vertical_overflow = pan::axis_overflow(2400.0, available_size.y);
    app.pan_offset.y = pan::axis_pan_limit(vertical_overflow);
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::NextSiblingFolder),
        Some(scope.child_folder_path("folder2"))
    );
    assert_eq!(
        app.sibling_folder_target_image_path(&scope.child_folder_path("folder2")),
        Some(scope.child_image_path("folder2", "b.png"))
    );
    let down_press = ArrowKeyPress {
        direction: NavigationDirection::NextSiblingFolder,
        non_repeat: true,
    };
    app.handle_arrow_key(available_size, down_press);
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(app.opened_image_path, Some(scope.child_image_path("folder1", "a.png")));
}

#[test]
fn a_fitting_image_arrow_navigates_directly() {
    let (folder, mut app) = open_first_of_two("pan_fitting_image");
    app.zoom_factor = 0.125;
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Next),
        Some(folder.image_path("b.png"))
    );
    let next_press = ArrowKeyPress {
        direction: NavigationDirection::Next,
        non_repeat: false,
    };
    app.handle_arrow_key(egui::vec2(800.0, 300.0), next_press);
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(app.opened_image_path, Some(folder.image_path("a.png")));
}

#[test]
fn arrows_without_an_open_image_route_directly_to_navigation() {
    let mut app = ImagesScrollerApp::empty_state();
    let previous_press = ArrowKeyPress {
        direction: NavigationDirection::Previous,
        non_repeat: true,
    };
    app.handle_arrow_key(egui::vec2(800.0, 300.0), previous_press);
    assert!(app.opened_image_path.is_none());
    assert_eq!(app.pan_offset, egui::Vec2::ZERO);
    assert!(app.message_text.is_none());
}
