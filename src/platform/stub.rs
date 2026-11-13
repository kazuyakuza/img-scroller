//! Non-Windows fallback of the platform facade: keeps container checks and
//! container test runs compiling and lets the app-level failure transition
//! stay testable on the Linux host target (decode always fails here; the
//! system entry-name order resolves to the pure name_collation fallback; the
//! open-with queries return an empty enumeration and a failed launcher).

use std::cmp::Ordering;
use std::path::{Path, PathBuf};

use crate::image_loading::{DecodedImage, ImageLoadFailure};
use crate::open_with_listing::{OpenWithLaunchFailure, RegisteredProgram};

pub(crate) fn show_open_image_dialog(_frame: &eframe::Frame) -> Option<PathBuf> {
    None
}

pub(crate) fn decode_image(_path: &Path) -> Result<DecodedImage, ImageLoadFailure> {
    Err(ImageLoadFailure::Unloadable)
}

pub(crate) fn compare_entry_names_system(left_name: &str, right_name: &str) -> Ordering {
    crate::name_collation::compare_entry_names(left_name, right_name)
}

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn enumerate_open_with_programs(_file_path: &Path) -> Vec<RegisteredProgram> {
    Vec::new()
}

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn open_file_with_selected_program(
    _file_path: &Path,
    _program_index: usize,
) -> Result<(), OpenWithLaunchFailure> {
    Err(OpenWithLaunchFailure::LaunchFailed)
}
