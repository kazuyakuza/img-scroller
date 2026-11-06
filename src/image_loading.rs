//! Open-image seam: decoded payload shape, failure classification, and the
//! unsupported-file message constant.

use std::path::Path;

use eframe::egui;

use crate::platform;

pub(crate) const UNSUPPORTED_FILE_MESSAGE: &str =
    "This file's format is not supported: no Windows image codec is registered for it.";

pub(crate) struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl DecodedImage {
    /// Image dimensions as `egui::Vec2`, shared by the display-paint base and
    /// the one-shot fit-factor seams (no duplicated u32→f32 casts).
    pub(crate) fn size(&self) -> egui::Vec2 {
        egui::Vec2::new(self.width as f32, self.height as f32)
    }
}

pub(crate) enum ImageLoadFailure {
    #[allow(dead_code)]
    Unsupported,
    Unloadable,
}

impl ImageLoadFailure {
    /// The single informational message shown for every open failure in Wave 1.
    #[allow(dead_code)]
    pub(crate) fn user_message(self) -> &'static str {
        UNSUPPORTED_FILE_MESSAGE
    }
}

pub(crate) fn load_image(path: &Path) -> Result<DecodedImage, ImageLoadFailure> {
    platform::decode_image(path)
}

/// Byte length of the RGBA8 payload for width×height pixels.
/// Returns None for empty dimensions or for sizes that overflow `usize`.
#[allow(dead_code)]
pub(crate) fn pixel_buffer_size(width: u32, height: u32) -> Option<usize> {
    if width == 0 {
        return None;
    }
    if height == 0 {
        return None;
    }
    let pixel_count = width.checked_mul(height)?;
    pixel_count
        .checked_mul(4)
        .map(|byte_count| byte_count as usize)
}

#[cfg(test)]
mod tests {
    use super::{ImageLoadFailure, UNSUPPORTED_FILE_MESSAGE, pixel_buffer_size};

    #[test]
    fn unsupported_message_matches_the_agreed_wording() {
        assert_eq!(
            UNSUPPORTED_FILE_MESSAGE,
            "This file's format is not supported: no Windows image codec is registered for it."
        );
    }

    #[test]
    fn every_failure_reports_the_unsupported_message() {
        assert_eq!(
            ImageLoadFailure::Unsupported.user_message(),
            UNSUPPORTED_FILE_MESSAGE
        );
        assert_eq!(
            ImageLoadFailure::Unloadable.user_message(),
            UNSUPPORTED_FILE_MESSAGE
        );
    }

    #[test]
    fn pixel_buffer_size_matches_width_height_times_four() {
        assert_eq!(pixel_buffer_size(1920, 1080), Some(1920 * 1080 * 4));
    }

    #[test]
    fn pixel_buffer_size_rejects_empty_and_overflowing_dimensions() {
        assert_eq!(pixel_buffer_size(0, 5), None);
        assert_eq!(pixel_buffer_size(5, 0), None);
        assert_eq!(pixel_buffer_size(u32::MAX, u32::MAX), None);
    }
}
