//! Path segmentation of the window-title composition (Wave-8 Task 4): the
//! read-only display-form conversion of the stored canonical path, the `\`/`/`
//! split, and the drive/folder/file-name role partition (local drive, UNC
//! share, or no drive).

use std::path::Path;

use super::{PATH_SEGMENT_SEPARATOR, TITLE_ELLIPSIS};
use crate::open_with_listing::shell_parseable_path;

const UNC_PATH_MARKER: &str = r"\\";

pub(super) struct PathSegments {
    pub(super) drive: Option<String>,
    pub(super) folders: Vec<String>,
    pub(super) file_name: String,
}

/// Partition the shell-parseable display path into its drive / folder-chain /
/// file-name roles; an empty path yields `None` drive, no folders, `...` name.
pub(super) fn path_segments(path: &Path) -> PathSegments {
    let display_path = title_display_path(path);
    let mut segments = split_display_path(&display_path);
    if segments.is_empty() {
        return PathSegments {
            drive: None,
            folders: Vec::new(),
            file_name: TITLE_ELLIPSIS.to_owned(),
        };
    }
    let drive = extract_drive_segment(&mut segments, display_path.starts_with(UNC_PATH_MARKER));
    let file_name = segments.pop().unwrap_or_else(|| TITLE_ELLIPSIS.to_owned());
    PathSegments {
        drive,
        folders: segments,
        file_name,
    }
}

fn title_display_path(path: &Path) -> String {
    shell_parseable_path(path).to_string_lossy().into_owned()
}

fn split_display_path(display: &str) -> Vec<String> {
    display
        .split(['\\', '/'])
        .filter(|segment| !segment.is_empty())
        .map(str::to_owned)
        .collect()
}

fn extract_drive_segment(segments: &mut Vec<String>, is_unc: bool) -> Option<String> {
    if unc_drive_is_present(segments, is_unc) {
        let server = segments.remove(0);
        let share = segments.remove(0);
        return Some(format!(
            "{UNC_PATH_MARKER}{server}{PATH_SEGMENT_SEPARATOR}{share}"
        ));
    }
    if is_local_drive_segment(&segments[0]) {
        return Some(segments.remove(0));
    }
    None
}

fn unc_drive_is_present(segments: &[String], is_unc: bool) -> bool {
    is_unc && segments.len() >= 2
}

fn is_local_drive_segment(segment: &str) -> bool {
    segment.chars().count() == 2 && segment.ends_with(':')
}
