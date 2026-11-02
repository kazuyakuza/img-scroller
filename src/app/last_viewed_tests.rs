//! Unit tests for the session last-viewed memory: empty on a fresh app,
//! updated on every successful open, unclobbered by failed opens, and
//! scoped per folder.

use super::ImagesScrollerApp;
use super::tests::{TempImageFolder, sample_decoded_image};
use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;
use std::path::{Path, PathBuf};

fn assert_folder_remembers(app: &ImagesScrollerApp, image_path: &Path) {
    let folder_path = image_path.parent().expect("fixture path has a parent");
    assert_eq!(
        app.last_viewed_images.last_displayed_image(folder_path),
        Some(image_path)
    );
}

#[test]
fn fresh_app_starts_with_an_empty_last_viewed_memory() {
    let app = ImagesScrollerApp::empty_state();
    assert!(app.last_viewed_images.is_empty());
}

#[test]
fn successful_open_records_the_folders_last_viewed_image() {
    let folder = TempImageFolder::create("last_viewed_record");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    let image_path = folder.image_path("b.png");
    app.accept_open_outcome(image_path.clone(), Ok(sample_decoded_image()));
    assert_folder_remembers(&app, &image_path);
}

#[test]
fn every_subsequent_open_in_a_folder_updates_the_last_viewed_entry() {
    let folder = TempImageFolder::create("last_viewed_update");
    folder.create_file("a.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    app.accept_open_outcome(folder.image_path("c.png"), Ok(sample_decoded_image()));
    let image_path = folder.image_path("c.png");
    assert_folder_remembers(&app, &image_path);
}

#[test]
fn failed_open_keeps_the_folders_last_viewed_image() {
    let folder = TempImageFolder::create("last_viewed_failure");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.open_image(PathBuf::from("unregistered-in-container.bin"));
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    let image_path = folder.image_path("b.png");
    assert_folder_remembers(&app, &image_path);
}

#[test]
fn each_folder_keeps_its_own_last_viewed_entry() {
    let first_folder = TempImageFolder::create("last_viewed_first");
    first_folder.create_file("a.png");
    let second_folder = TempImageFolder::create("last_viewed_second");
    second_folder.create_file("e.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(first_folder.image_path("a.png"), Ok(sample_decoded_image()));
    app.accept_open_outcome(
        second_folder.image_path("e.png"),
        Ok(sample_decoded_image()),
    );
    let first_image_path = first_folder.image_path("a.png");
    let second_image_path = second_folder.image_path("e.png");
    assert_folder_remembers(&app, &first_image_path);
    assert_folder_remembers(&app, &second_image_path);
    app.open_image(second_folder.image_path("unregistered-decoy.png"));
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_folder_remembers(&app, &first_image_path);
    assert_folder_remembers(&app, &second_image_path);
}
