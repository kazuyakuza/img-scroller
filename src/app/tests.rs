use super::{ImagesScrollerApp, NavigationDirection};
use crate::image_loading::{DecodedImage, UNSUPPORTED_FILE_MESSAGE};
use std::fs;
use std::path::PathBuf;

struct TempImageFolder {
    folder_path: std::path::PathBuf,
}

impl TempImageFolder {
    fn create(test_case: &str) -> Self {
        let folder_path = std::env::temp_dir().join(format!(
            "img_scroller_app_navigation_{test_case}_{}",
            std::process::id()
        ));
        fs::create_dir_all(&folder_path).expect("temp image folder created");
        Self { folder_path }
    }

    fn create_file(&self, file_name: &str) {
        fs::write(self.folder_path.join(file_name), b"x").expect("fixture file written");
    }

    fn image_path(&self, file_name: &str) -> std::path::PathBuf {
        self.folder_path.join(file_name)
    }
}

impl Drop for TempImageFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.folder_path);
    }
}

fn sample_decoded_image() -> DecodedImage {
    DecodedImage {
        width: 2,
        height: 2,
        pixels: vec![0; 16],
    }
}

#[test]
fn window_title_is_the_final_app_name() {
    assert_eq!(super::APP_WINDOW_TITLE, "Images Scroller");
}

#[test]
fn failed_open_via_cli_reports_the_message_and_keeps_no_path() {
    let mut app = ImagesScrollerApp::empty_state();
    app.open_image(PathBuf::from("unregistered-in-container.bin"));
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert!(app.opened_image_path.is_none());
    assert!(app.decoded_image.is_none());
}

#[test]
fn successful_open_stores_path_and_payload_and_clears_message() {
    let mut app = ImagesScrollerApp::empty_state();
    let image_path = PathBuf::from("photo.png");
    let decoded = DecodedImage {
        width: 1,
        height: 1,
        pixels: vec![0, 0, 0, 255],
    };
    app.accept_open_outcome(image_path.clone(), Ok(decoded));
    assert_eq!(app.opened_image_path, Some(image_path));
    assert!(app.decoded_image.is_some());
    assert!(app.message_text.is_none());
}

#[test]
fn failure_after_success_keeps_the_previous_image_visible() {
    let mut app = ImagesScrollerApp::empty_state();
    let decoded = DecodedImage {
        width: 2,
        height: 2,
        pixels: vec![0; 16],
    };
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(decoded));
    app.open_image(PathBuf::from("unregistered-in-container.bin"));
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(app.opened_image_path, Some(PathBuf::from("photo.png")));
    assert!(app.decoded_image.is_some());
}

#[test]
fn navigation_on_the_empty_state_does_not_change_any_state() {
    let mut app = ImagesScrollerApp::empty_state();
    app.navigate(NavigationDirection::Previous);
    app.navigate(NavigationDirection::Next);
    assert!(app.opened_image_path.is_none());
    assert!(app.current_folder_listing.is_none());
    assert!(app.decoded_image.is_none());
    assert!(app.message_text.is_none());
}

#[test]
fn navigation_resolves_the_previous_and_next_sibling_in_sort_order() {
    let folder = TempImageFolder::create("sibling_order");
    folder.create_file("a.png");
    folder.create_file("b.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(folder.image_path("a.png"))
    );
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Next),
        Some(folder.image_path("c.png"))
    );
}

#[test]
fn navigation_at_the_first_and_last_list_positions_stays_silent() {
    let folder = TempImageFolder::create("list_edges");
    folder.create_file("a.png");
    folder.create_file("b.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert_eq!(app.neighbor_image_path(NavigationDirection::Previous), None);
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Next),
        Some(folder.image_path("b.png"))
    );
    app.accept_open_outcome(folder.image_path("c.png"), Ok(sample_decoded_image()));
    assert_eq!(app.neighbor_image_path(NavigationDirection::Next), None);
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(folder.image_path("b.png"))
    );
    let solo_folder = TempImageFolder::create("single_image_folder");
    solo_folder.create_file("only.png");
    app.accept_open_outcome(solo_folder.image_path("only.png"), Ok(sample_decoded_image()));
    assert_eq!(app.neighbor_image_path(NavigationDirection::Previous), None);
    assert_eq!(app.neighbor_image_path(NavigationDirection::Next), None);
    assert!(app.message_text.is_none());
}

#[test]
fn failed_neighbor_open_keeps_the_current_image_and_shows_the_message() {
    let folder = TempImageFolder::create("failure_keeps_image");
    folder.create_file("a.png");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.navigate(NavigationDirection::Previous);
    assert_eq!(app.opened_image_path, Some(folder.image_path("b.png")));
    assert!(app.decoded_image.is_some());
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(folder.image_path("a.png"))
    );
}

#[test]
fn every_successful_open_rebuilds_the_listing_from_the_new_folder() {
    let first_folder = TempImageFolder::create("rebuild_first");
    first_folder.create_file("a.png");
    first_folder.create_file("b.png");
    first_folder.create_file("c.png");
    let second_folder = TempImageFolder::create("rebuild_second");
    second_folder.create_file("d.png");
    second_folder.create_file("e.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(first_folder.image_path("b.png"), Ok(sample_decoded_image()));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(first_folder.image_path("a.png"))
    );
    app.accept_open_outcome(second_folder.image_path("e.png"), Ok(sample_decoded_image()));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(second_folder.image_path("d.png"))
    );
    assert_eq!(app.neighbor_image_path(NavigationDirection::Next), None);
}
