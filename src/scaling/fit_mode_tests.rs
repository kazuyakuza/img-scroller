use super::{FitMode, fit_height_size, fit_width_size, fitted_size};
use eframe::egui;

/// Exact-in-f32 test vectors only (powers of two divisors), matching the
/// `fitted_size` test conventions.

#[test]
fn fit_width_spans_the_available_width_and_overflows_vertically() {
    let available_size = egui::Vec2::new(400.0, 400.0);
    let image_size = egui::Vec2::new(100.0, 200.0);
    assert_eq!(
        fit_width_size(available_size, image_size),
        Some(egui::Vec2::new(400.0, 800.0))
    );
}

#[test]
fn fit_height_spans_the_available_height_and_overflows_horizontally() {
    let available_size = egui::Vec2::new(400.0, 400.0);
    let image_size = egui::Vec2::new(200.0, 100.0);
    assert_eq!(
        fit_height_size(available_size, image_size),
        Some(egui::Vec2::new(800.0, 400.0))
    );
}

#[test]
fn square_image_width_height_and_visible_fits_agree() {
    let available_size = egui::Vec2::new(400.0, 400.0);
    let image_size = egui::Vec2::new(100.0, 100.0);
    assert_eq!(
        FitMode::FitVisible.base_size(available_size, image_size),
        Some(egui::Vec2::new(400.0, 400.0))
    );
    assert_eq!(
        FitMode::FitWidth.base_size(available_size, image_size),
        Some(egui::Vec2::new(400.0, 400.0))
    );
    assert_eq!(
        FitMode::FitHeight.base_size(available_size, image_size),
        Some(egui::Vec2::new(400.0, 400.0))
    );
}

#[test]
fn fit_mode_base_sizes_difference_is_exact_on_a_wide_image() {
    let available_size = egui::Vec2::new(800.0, 300.0);
    let image_size = egui::Vec2::new(400.0, 100.0);
    assert_eq!(
        FitMode::FitVisible.base_size(available_size, image_size),
        Some(egui::Vec2::new(800.0, 200.0))
    );
    assert_eq!(
        FitMode::FitWidth.base_size(available_size, image_size),
        Some(egui::Vec2::new(800.0, 200.0))
    );
    assert_eq!(
        FitMode::FitHeight.base_size(available_size, image_size),
        Some(egui::Vec2::new(1200.0, 300.0))
    );
}

#[test]
fn fit_visible_base_size_matches_the_whole_image_fitted_size() {
    let available_size = egui::Vec2::new(300.0, 800.0);
    let image_size = egui::Vec2::new(100.0, 400.0);
    assert_eq!(
        FitMode::FitVisible.base_size(available_size, image_size),
        fitted_size(available_size, image_size)
    );
}

#[test]
fn non_positive_dimensions_reject_every_fit_mode() {
    let usable = egui::Vec2::new(100.0, 100.0);
    for fit_mode in [FitMode::FitVisible, FitMode::FitWidth, FitMode::FitHeight] {
        assert_eq!(fit_mode.base_size(usable, egui::Vec2::new(0.0, 10.0)), None);
        assert_eq!(
            fit_mode.base_size(usable, egui::Vec2::new(-4.0, 10.0)),
            None
        );
        assert_eq!(
            fit_mode.base_size(egui::Vec2::new(0.0, 100.0), usable),
            None
        );
        assert_eq!(
            fit_mode.base_size(egui::Vec2::new(100.0, -1.0), usable),
            None
        );
    }
}
