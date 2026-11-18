//! Wave-8 Task-4 title-composition matrix: the not-maximized name rule, the
//! maximized full-path / containing-folder + drive-`...` + parent-walk-up chain,
//! verbatim & UNC path handling, and the pinned constants -- all driven through
//! an injected fixed-width measure closure so the logic stays container-testable.

use std::path::Path;

use super::{
    FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS, FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS,
    PATH_SEGMENT_SEPARATOR, TITLE_BAR_USABLE_WIDTH_FRACTION, TITLE_ELLIPSIS, TitleComposition,
    compose_window_title, file_name_with_extension,
};

const CHAR_WIDTH_PX: f32 = 10.0;

const SHORT: &str = "C:/pix/img.png";
const DEEP: &str = "C:/photos/album/first/image.png";
const LONGNAME: &str = "C:/pix/a-very-long-image-file.png";
const QUAD: &str = "C:/aaaa/bbbb/cccc/dddd/image.png";
const UNIX: &str = "/home/user/pics/img.png";

fn compose(path_str: Option<&str>, budget_chars: usize, maximized: bool) -> String {
    compose_window_title(TitleComposition {
        path: path_str.map(Path::new),
        available_width: budget_chars as f32 * CHAR_WIDTH_PX,
        is_maximized: maximized,
        measure: &mut |text: &str| text.chars().count() as f32 * CHAR_WIDTH_PX,
    })
}

#[test]
fn none_path_returns_the_pinned_empty_state_title() {
    assert_eq!(compose(None, 50, true), "Images Scroller");
    assert_eq!(compose(None, 50, false), "Images Scroller");
}

#[test]
fn not_maximized_shows_only_the_name_with_extension() {
    assert_eq!(compose(Some(DEEP), 50, false), "image.png");
}

#[test]
fn not_maximized_name_that_fits_is_complete_without_ellipsis() {
    assert_eq!(compose(Some(DEEP), 9, false), "image.png");
}

#[test]
fn not_maximized_long_name_keeps_last_characters_with_ellipsis_prefix() {
    assert_eq!(compose(Some(LONGNAME), 12, false), "...-file.png");
}

#[test]
fn not_maximized_even_a_truncated_name_cannot_fit_yields_the_bare_ellipsis() {
    assert_eq!(compose(Some(LONGNAME), 2, false), "...");
}

#[test]
fn maximized_full_path_that_fits_is_shown_verbatim() {
    assert_eq!(
        compose(Some(DEEP), 31, true),
        "C:\\photos\\album\\first\\image.png"
    );
}

#[test]
fn maximized_containing_folder_added_first_when_it_fits() {
    assert_eq!(compose(Some(DEEP), 16, true), "first\\image.png");
}

#[test]
fn maximized_drive_added_with_ellipsis_when_folders_are_omitted() {
    assert_eq!(compose(Some(DEEP), 23, true), "C:\\...\\first\\image.png");
}

#[test]
fn maximized_parent_folders_walk_up_while_they_fit() {
    assert_eq!(
        compose(Some(DEEP), 29, true),
        "C:\\...\\album\\first\\image.png"
    );
}

#[test]
fn maximized_parent_walk_stops_at_the_first_parent_that_does_not_fit() {
    let title = compose(Some(QUAD), 30, true);
    assert_eq!(title, "C:\\...\\cccc\\dddd\\image.png");
    assert!(!title.contains("bbbb"));
    assert!(!title.contains("aaaa"));
}

#[test]
fn maximized_drive_skipped_when_it_does_not_fit_but_parents_are_still_added() {
    let title = compose(Some("C:/ab/album/image.png"), 20, true);
    assert_eq!(title, "ab\\album\\image.png");
    assert!(!title.contains("C:"));
    assert!(!title.contains("..."));
}

#[test]
fn maximized_truncated_name_consumes_the_width_so_no_folders_are_added() {
    assert_eq!(compose(Some(LONGNAME), 12, true), "...-file.png");
}

#[test]
fn maximized_unix_style_path_without_drive_composes_the_folder_chain() {
    assert_eq!(compose(Some(UNIX), 22, true), "home\\user\\pics\\img.png");
}

#[test]
fn verbatim_prefix_is_stripped_before_segmentation() {
    assert_eq!(
        compose(Some(r"\\?\C:\pix\img.png"), 50, true),
        "C:\\pix\\img.png"
    );
}

#[test]
fn unc_path_forms_a_single_drive_segment() {
    assert_eq!(
        compose(Some(r"\\?\UNC\server\share\pix\img.png"), 50, true),
        "\\\\server\\share\\pix\\img.png"
    );
}

#[test]
fn file_name_with_extension_extracts_the_last_segment() {
    assert_eq!(file_name_with_extension(Path::new(DEEP)), "image.png");
    assert_eq!(file_name_with_extension(Path::new(SHORT)), "img.png");
}

#[test]
fn path_with_no_file_name_falls_back_to_the_ellipsis_name() {
    assert_eq!(file_name_with_extension(Path::new("")), "...");
}

#[test]
fn task_constants_are_pinned() {
    assert_eq!(PATH_SEGMENT_SEPARATOR, "\\");
    assert_eq!(TITLE_ELLIPSIS, "...");
    assert_eq!(TITLE_BAR_USABLE_WIDTH_FRACTION, 0.85);
    assert_eq!(FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS, 3.0);
    assert_eq!(FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS, 0.05);
}
