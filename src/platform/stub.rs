//! Non-Windows fallback of the platform facade: keeps container checks and
//! container test runs compiling and lets the app-level failure transition
//! stay testable on the Linux host target (decode always fails here).

use std::path::{Path, PathBuf};

use crate::image_loading::{DecodedImage, ImageLoadFailure};

pub(crate) fn show_open_image_dialog(_frame: &eframe::Frame) -> Option<PathBuf> {
    None
}

pub(crate) fn decode_image(_path: &Path) -> Result<DecodedImage, ImageLoadFailure> {
    Err(ImageLoadFailure::Unloadable)
}
