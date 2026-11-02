//! Fit-scaling math for displaying a decoded image inside the viewer window.
//! Pure calculations only: no egui painting, no texture or pixel code.

use eframe::egui;

/// The largest uniform scale that keeps the image inside the available space,
/// applied to both image axes (same factor on both axes, aspect preserved).
/// Returns None when either size has a zero or negative dimension, the same
/// guard style as `pixel_buffer_size` in the decode seam.
pub(crate) fn fitted_size(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<egui::Vec2> {
    if !is_positive_size(image_size) {
        return None;
    }
    if !is_positive_size(available_size) {
        return None;
    }
    Some(image_size * scale_factor(available_size, image_size))
}

fn scale_factor(available_size: egui::Vec2, image_size: egui::Vec2) -> f32 {
    let width_ratio = available_size.x / image_size.x;
    let height_ratio = available_size.y / image_size.y;
    width_ratio.min(height_ratio)
}

fn is_positive_size(size: egui::Vec2) -> bool {
    if size.x <= 0.0 {
        return false;
    }
    if size.y <= 0.0 {
        return false;
    }
    true
}

/// The three session fit modes of the viewer (Wave-4 Task 3). `FitVisible` is
/// the whole-image fit carried since Wave 1; `FitWidth`/`FitHeight` are the
/// Windows-user expressions of "fit width"/"fit height". Session-only means:
/// no persistence lives here (the app seam owns the reset policy).
#[allow(clippy::enum_variant_names)] // the variants ARE the user-facing mode names
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FitMode {
    /// Whole-image fit: fully visible and centered.
    FitVisible,
    /// Width fit: the width spans the available width; vertical overflow allowed.
    FitWidth,
    /// Height fit: the height spans the available height; horizontal overflow allowed.
    FitHeight,
}

impl FitMode {
    /// The mode's fit-base size of `image_size` inside `available_size`.
    /// Returns None on any zero/negative dimension (the `fitted_size` guards,
    /// shared by every variant).
    pub(crate) fn base_size(
        self,
        available_size: egui::Vec2,
        image_size: egui::Vec2,
    ) -> Option<egui::Vec2> {
        match self {
            Self::FitVisible => fitted_size(available_size, image_size),
            Self::FitWidth => fit_width_size(available_size, image_size),
            Self::FitHeight => fit_height_size(available_size, image_size),
        }
    }
}

/// The width-fit draw size: the image WIDTH spans the available width; the
/// height follows the aspect ratio and may overflow vertically. Same
/// zero/negative-dimension rejection style as `fitted_size`.
pub(crate) fn fit_width_size(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<egui::Vec2> {
    if !is_positive_size(image_size) {
        return None;
    }
    if !is_positive_size(available_size) {
        return None;
    }
    Some(image_size * (available_size.x / image_size.x))
}

/// The height-fit twin of `fit_width_size`: the image HEIGHT spans the
/// available height; the width may overflow horizontally.
pub(crate) fn fit_height_size(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<egui::Vec2> {
    if !is_positive_size(image_size) {
        return None;
    }
    if !is_positive_size(available_size) {
        return None;
    }
    Some(image_size * (available_size.y / image_size.y))
}

#[cfg(test)]
mod tests {
    use super::fitted_size;
    use eframe::egui;

    // Test vectors are chosen with scale factors that are exact in binary
    // floating point (0.5, 2.0, 3.0), so `assert_eq!` comparisons are exact.

    #[test]
    fn wide_image_fits_the_available_height() {
        let available_size = egui::Vec2::new(800.0, 300.0);
        let image_size = egui::Vec2::new(400.0, 100.0);
        assert_eq!(
            fitted_size(available_size, image_size),
            Some(egui::Vec2::new(800.0, 200.0))
        );
    }

    #[test]
    fn tall_image_fits_the_available_width() {
        let available_size = egui::Vec2::new(300.0, 800.0);
        let image_size = egui::Vec2::new(100.0, 400.0);
        assert_eq!(
            fitted_size(available_size, image_size),
            Some(egui::Vec2::new(200.0, 800.0))
        );
    }

    #[test]
    fn oversized_image_shrinks_preserving_the_aspect_ratio() {
        let available_size = egui::Vec2::new(400.0, 400.0);
        let image_size = egui::Vec2::new(800.0, 400.0);
        assert_eq!(
            fitted_size(available_size, image_size),
            Some(egui::Vec2::new(400.0, 200.0))
        );
    }

    #[test]
    fn small_image_scales_up_to_fill_one_axis() {
        let available_size = egui::Vec2::new(400.0, 400.0);
        let image_size = egui::Vec2::new(100.0, 200.0);
        assert_eq!(
            fitted_size(available_size, image_size),
            Some(egui::Vec2::new(200.0, 400.0))
        );
    }

    #[test]
    fn empty_dimensions_reject_the_fit() {
        let usable = egui::Vec2::new(100.0, 100.0);
        assert_eq!(fitted_size(usable, egui::Vec2::new(0.0, 10.0)), None);
        assert_eq!(fitted_size(usable, egui::Vec2::new(10.0, 0.0)), None);
        assert_eq!(fitted_size(egui::Vec2::new(0.0, 100.0), usable), None);
        assert_eq!(fitted_size(egui::Vec2::new(100.0, 0.0), usable), None);
    }
}

#[cfg(test)]
mod fit_mode_tests;
