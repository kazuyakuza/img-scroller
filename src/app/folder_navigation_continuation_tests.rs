use super::floating_controls::navigation_button_enabled;
use super::folder_navigation_tests::{TempParentScope, app_opened_in};
use super::tests::sample_decoded_image;
use super::{ImagesScrollerApp, NavigationDirection};
use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;
use std::fs;

fn assert_enablement_matches_resolution(app: &ImagesScrollerApp) {
    assert_eq!(
        navigation_button_enabled(app, NavigationDirection::Previous),
        app.neighbor_image_path(NavigationDirection::Previous)
            .is_some()
    );
    assert_eq!(
        navigation_button_enabled(app, NavigationDirection::Next),
        app.neighbor_image_path(NavigationDirection::Next).is_some()
    );
    assert_eq!(
        navigation_button_enabled(app, NavigationDirection::PreviousSiblingFolder),
        app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder)
            .is_some()
    );
    assert_eq!(
        navigation_button_enabled(app, NavigationDirection::NextSiblingFolder),
        app.neighbor_image_path(NavigationDirection::NextSiblingFolder)
            .is_some()
    );
}

fn assert_image_mode_sibling_edges_are_silent() {
    let scope = TempParentScope::create("sibling_edges");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_image("folder1", "a.png");
    scope.create_image("folder2", "b.png");
    let mut app = app_opened_in(&scope.child_image_path("folder1", "a.png"));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder),
        None
    );
    app.navigate(NavigationDirection::PreviousSiblingFolder);
    assert_eq!(
        app.opened_image_path,
        Some(scope.child_image_path("folder1", "a.png"))
    );
    assert!(app.message_text.is_none());
    app.accept_open_outcome(
        scope.child_image_path("folder2", "b.png"),
        Ok(sample_decoded_image()),
    );
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::NextSiblingFolder),
        None
    );
    assert!(app.message_text.is_none());
}

#[test]
fn vertical_navigation_stays_silent_at_the_sibling_list_edges() {
    assert_image_mode_sibling_edges_are_silent();
    let empty_scope = TempParentScope::create("sibling_edges_empty_mode");
    empty_scope.create_child_folder("folder1");
    empty_scope.create_child_folder("folder2");
    empty_scope.create_image("folder2", "b.png");
    let mut empty_app = app_opened_in(&empty_scope.child_image_path("folder2", "b.png"));
    empty_app.navigate(NavigationDirection::PreviousSiblingFolder);
    assert_eq!(
        empty_app.browsed_folder_context,
        Some(empty_scope.child_folder_path("folder1"))
    );
    assert_eq!(
        empty_app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder),
        None
    );
    empty_app.navigate(NavigationDirection::PreviousSiblingFolder);
    assert_eq!(
        empty_app.browsed_folder_context,
        Some(empty_scope.child_folder_path("folder1"))
    );
    let single_scope = TempParentScope::create("sibling_edges_single");
    single_scope.create_child_folder("solo");
    single_scope.create_image("solo", "only.png");
    let single_app = app_opened_in(&single_scope.child_image_path("solo", "only.png"));
    assert_eq!(
        single_app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder),
        None
    );
    assert_eq!(
        single_app.neighbor_image_path(NavigationDirection::NextSiblingFolder),
        None
    );
}

#[test]
fn vertical_navigation_to_a_folder_whose_last_viewed_image_vanished_reports_the_standard_failure() {
    let scope = TempParentScope::create("vanished_target");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_image("folder1", "keep.png");
    scope.create_image("folder1", "vanish.png");
    scope.create_image("folder2", "live.png");
    let live_path = scope.child_image_path("folder2", "live.png");
    let mut app = app_opened_in(&scope.child_image_path("folder1", "vanish.png"));
    app.accept_open_outcome(live_path.clone(), Ok(sample_decoded_image()));
    fs::remove_file(scope.child_image_path("folder1", "vanish.png")).expect("vanish image removed");
    app.navigate(NavigationDirection::PreviousSiblingFolder);
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(app.opened_image_path, Some(live_path.clone()));
    assert!(app.decoded_image.is_some());
    assert_eq!(
        app.last_viewed_images
            .last_displayed_image(&scope.child_folder_path("folder2")),
        Some(live_path.as_path())
    );
}

#[test]
fn vertical_button_enablement_matches_the_neighbor_resolution_semantics() {
    let scope = TempParentScope::create("button_semantics");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_child_folder("folder3");
    scope.create_image("folder1", "a.png");
    scope.create_image("folder3", "c.png");
    fs::write(scope.child_folder_path("folder2").join("stray.txt"), b"x")
        .expect("stray file written");
    let fresh_app = ImagesScrollerApp::empty_state();
    assert_enablement_matches_resolution(&fresh_app);
    let mut app = app_opened_in(&scope.child_image_path("folder1", "a.png"));
    assert!(!navigation_button_enabled(
        &app,
        NavigationDirection::PreviousSiblingFolder
    ));
    assert!(navigation_button_enabled(
        &app,
        NavigationDirection::NextSiblingFolder
    ));
    assert!(
        app.sibling_folder_target_image_path(&scope.child_folder_path("folder2"))
            .is_none()
    );
    assert_enablement_matches_resolution(&app);
    app.navigate(NavigationDirection::NextSiblingFolder);
    assert!(navigation_button_enabled(
        &app,
        NavigationDirection::PreviousSiblingFolder
    ));
    assert!(navigation_button_enabled(
        &app,
        NavigationDirection::NextSiblingFolder
    ));
    assert!(!navigation_button_enabled(
        &app,
        NavigationDirection::Previous
    ));
    assert!(!navigation_button_enabled(&app, NavigationDirection::Next));
    assert_enablement_matches_resolution(&app);
}

#[test]
fn vertical_target_open_records_the_new_folders_last_viewed_memory() {
    let scope = TempParentScope::create("target_memory");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_image("folder1", "a.png");
    scope.create_image("folder1", "b.png");
    scope.create_image("folder2", "c.png");
    let mut app = app_opened_in(&scope.child_image_path("folder1", "b.png"));
    assert!(
        app.last_viewed_images
            .last_displayed_image(&scope.child_folder_path("folder2"))
            .is_none()
    );
    app.accept_open_outcome(
        scope.child_image_path("folder2", "c.png"),
        Ok(sample_decoded_image()),
    );
    assert_eq!(
        app.last_viewed_images
            .last_displayed_image(&scope.child_folder_path("folder2")),
        Some(scope.child_image_path("folder2", "c.png").as_path())
    );
    assert_eq!(
        app.sibling_folder_target_image_path(&scope.child_folder_path("folder1")),
        Some(scope.child_image_path("folder1", "b.png"))
    );
}
