//! Test module of `button_hint`: the pinned hotkey-reference and tooltip tables.

use super::button_hint::{PillButton, hotkey_reference, tooltip_text};

#[test]
fn hotkey_reference_matches_the_spec() {
    use PillButton::*;
    assert_eq!(hotkey_reference(ImagePrevious), None);
    assert_eq!(hotkey_reference(ImageNext), None);
    assert_eq!(hotkey_reference(FolderUp), None);
    assert_eq!(hotkey_reference(FolderDown), None);
    assert_eq!(hotkey_reference(Fullscreen), Some("F11"));
    assert_eq!(hotkey_reference(ZoomIn), Some("I"));
    assert_eq!(hotkey_reference(ZoomOut), Some("O"));
    assert_eq!(hotkey_reference(RestoreFit), Some("0"));
    assert_eq!(hotkey_reference(FitWidth), Some("W"));
    assert_eq!(hotkey_reference(FitHeight), Some("H"));
}

#[test]
fn tooltip_text_matches_the_spec() {
    use PillButton::*;
    assert_eq!(tooltip_text(ImagePrevious), "Previous image (Left Arrow)");
    assert_eq!(tooltip_text(ImageNext), "Next image (Right Arrow)");
    assert_eq!(tooltip_text(FolderUp), "Previous sibling folder (Up Arrow)");
    assert_eq!(tooltip_text(FolderDown), "Next sibling folder (Down Arrow)");
    assert_eq!(tooltip_text(Fullscreen), "Fullscreen (F11)");
    assert_eq!(tooltip_text(ZoomIn), "Zoom in (I)");
    assert_eq!(tooltip_text(ZoomOut), "Zoom out (O)");
    assert_eq!(tooltip_text(RestoreFit), "Restore fit (0)");
    assert_eq!(tooltip_text(FitWidth), "Fit width (W)");
    assert_eq!(tooltip_text(FitHeight), "Fit height (H)");
}
