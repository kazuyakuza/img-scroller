use super::empty_folder_view::EMPTY_FOLDER_MESSAGE;
use super::tests::sample_decoded_image;
use super::{ImagesScrollerApp, NavigationDirection};
use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;
use std::fs;
use std::path::PathBuf;

pub(super) struct TempParentScope {
    parent_path: PathBuf,
}

impl TempParentScope {
    pub(super) fn create(test_case: &str) -> Self {
        let parent_path = std::env::temp_dir().join(format!(
            "img_scroller_folder_nav_{test_case}_{}",
            std::process::id()
        ));
        fs::create_dir_all(&parent_path).expect("temp parent scope created");
        Self { parent_path }
    }

    pub(super) fn create_child_folder(&self, folder_name: &str) -> PathBuf {
        let child_path = self.parent_path.join(folder_name);
        fs::create_dir(&child_path).expect("child folder created");
        child_path
    }

    pub(super) fn create_image(&self, folder_name: &str, file_name: &str) {
        fs::write(self.parent_path.join(folder_name).join(file_name), b"x")
            .expect("fixture image written");
    }

    pub(super) fn child_image_path(&self, folder_name: &str, file_name: &str) -> PathBuf {
        self.parent_path.join(folder_name).join(file_name)
    }

    pub(super) fn child_folder_path(&self, folder_name: &str) -> PathBuf {
        self.parent_path.join(folder_name)
    }
}

impl Drop for TempParentScope {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.parent_path);
    }
}

pub(super) fn app_opened_in(scope_image_path: &PathBuf) -> ImagesScrollerApp {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(scope_image_path.clone(), Ok(sample_decoded_image()));
    app
}

#[test]
fn vertical_navigation_resolves_the_adjacent_sibling_folders_in_sort_order() {
    let scope = TempParentScope::create("adjacent_folders");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_child_folder("folder3");
    scope.create_image("folder1", "f1.png");
    scope.create_image("folder2", "f2.png");
    scope.create_image("folder3", "f3.png");
    let app = app_opened_in(&scope.child_image_path("folder2", "f2.png"));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder),
        Some(scope.child_folder_path("folder1"))
    );
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::NextSiblingFolder),
        Some(scope.child_folder_path("folder3"))
    );
}

#[test]
fn vertical_navigation_reopens_the_visited_folders_last_viewed_image() {
    let scope = TempParentScope::create("visited_priority");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_image("folder1", "name1.png");
    scope.create_image("folder1", "name2.png");
    scope.create_image("folder2", "single.png");
    let mut app = app_opened_in(&scope.child_image_path("folder1", "name2.png"));
    app.accept_open_outcome(
        scope.child_image_path("folder2", "single.png"),
        Ok(sample_decoded_image()),
    );
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder),
        Some(scope.child_folder_path("folder1"))
    );
    assert_eq!(
        app.sibling_folder_target_image_path(&scope.child_folder_path("folder1")),
        Some(scope.child_image_path("folder1", "name2.png"))
    );
}

#[test]
fn vertical_navigation_opens_the_first_image_of_a_new_folder() {
    let scope = TempParentScope::create("first_image_priority");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_image("folder1", "only.png");
    scope.create_image("folder2", "b.png");
    scope.create_image("folder2", "a.png");
    let app = app_opened_in(&scope.child_image_path("folder1", "only.png"));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::NextSiblingFolder),
        Some(scope.child_folder_path("folder2"))
    );
    assert_eq!(
        app.sibling_folder_target_image_path(&scope.child_folder_path("folder2")),
        Some(scope.child_image_path("folder2", "a.png"))
    );
}

#[test]
fn entering_an_image_less_sibling_folder_clears_the_image_and_reanchors_the_listing() {
    let scope = TempParentScope::create("empty_mode_entry");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_image("folder1", "img.png");
    fs::write(scope.child_folder_path("folder2").join("stray.txt"), b"x")
        .expect("stray file written");
    let mut app = app_opened_in(&scope.child_image_path("folder1", "img.png"));
    app.navigate(NavigationDirection::NextSiblingFolder);
    assert!(app.opened_image_path.is_none());
    assert!(app.decoded_image.is_none());
    assert_eq!(app.browsed_folder_context, Some(scope.child_folder_path("folder2")));
    assert!(app.current_folder_listing.is_none());
    assert!(app.message_text.is_none());
    assert!(app.navigation_context_is_active());
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder),
        Some(scope.child_folder_path("folder1"))
    );
    assert!(
        app.last_viewed_images
            .last_displayed_image(&scope.child_folder_path("folder2"))
            .is_none()
    );
    assert!(app.neighbor_image_path(NavigationDirection::Next).is_none());
    assert_eq!(EMPTY_FOLDER_MESSAGE, "Empty folder");
}

#[test]
fn empty_folder_mode_continues_vertically_in_both_folder_kinds() {
    let scope = TempParentScope::create("empty_mode_continuation");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_child_folder("folder3");
    scope.create_image("folder1", "a.png");
    scope.create_image("folder3", "c.png");
    fs::write(scope.child_folder_path("folder2").join("stray.txt"), b"x")
        .expect("stray file written");
    let mut app = app_opened_in(&scope.child_image_path("folder1", "a.png"));
    app.navigate(NavigationDirection::NextSiblingFolder);
    assert_eq!(app.browsed_folder_context, Some(scope.child_folder_path("folder2")));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::NextSiblingFolder),
        Some(scope.child_folder_path("folder3"))
    );
    assert_eq!(
        app.sibling_folder_target_image_path(&scope.child_folder_path("folder3")),
        Some(scope.child_image_path("folder3", "c.png"))
    );
    app.navigate(NavigationDirection::NextSiblingFolder);
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(app.browsed_folder_context, Some(scope.child_folder_path("folder2")));
    assert!(app.opened_image_path.is_none());
    assert!(app.decoded_image.is_none());
    app.accept_open_outcome(
        scope.child_image_path("folder3", "c.png"),
        Ok(sample_decoded_image()),
    );
    assert!(app.browsed_folder_context.is_none());
    assert_eq!(
        app.opened_image_path,
        Some(scope.child_image_path("folder3", "c.png"))
    );
    assert!(app.decoded_image.is_some());
    assert!(app.message_text.is_none());
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder),
        Some(scope.child_folder_path("folder2"))
    );
    app.navigate(NavigationDirection::PreviousSiblingFolder);
    assert_eq!(app.browsed_folder_context, Some(scope.child_folder_path("folder2")));
    assert!(app.opened_image_path.is_none());
    assert!(app.message_text.is_none());
}
