//! Unit tests for the corner-rail arrow-button enablement
//! (`super::rails::navigation_button_enabled`).

use super::tests::{TempImageFolder, sample_decoded_image};
use super::{ImagesScrollerApp, NavigationDirection};

#[test]
fn arrow_button_enablement_follows_the_navigation_edges() {
    let folder = TempImageFolder::create("button_edges");
    folder.create_file("a.png");
    folder.create_file("b.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    assert!(!super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Previous
    ));
    assert!(!super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Next
    ));
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert!(!super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Previous
    ));
    assert!(super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Next
    ));
    app.accept_open_outcome(folder.image_path("c.png"), Ok(sample_decoded_image()));
    assert!(super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Previous
    ));
    assert!(!super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Next
    ));
}
