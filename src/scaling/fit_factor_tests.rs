//! One-shot fit-factor tests: exact-in-f32 vectors (the `fitted_size`
//! conventions) plus the clamp-at-ceiling case. The zoom-out FLOOR clamp is
//! NOT testable here: every factor is mathematically `>= 1.0` (the fitted
//! size never exceeds the available area), so the floor arm of
//! `clamped_zoom_factor` is a latent safety net with no reachable input.

use super::{fit_height_factor, fit_width_factor};
use eframe::egui;

#[test]
fn fit_width_factor_is_one_for_a_wide_image() {
    let available_size = egui::Vec2::new(800.0, 300.0); // width constrains
    let image_size = egui::Vec2::new(400.0, 100.0);
    assert_eq!(fit_width_factor(available_size, image_size), Some(1.0));
}

#[test]
fn fit_width_factor_exceeds_one_for_a_tall_image() {
    let available_size = egui::Vec2::new(300.0, 800.0);
    let image_size = egui::Vec2::new(100.0, 400.0); // fitted (200, 800)
    assert_eq!(fit_width_factor(available_size, image_size), Some(1.5));
}

#[test]
fn fit_height_factor_is_one_for_a_tall_image() {
    let available_size = egui::Vec2::new(300.0, 800.0); // height constrains
    let image_size = egui::Vec2::new(100.0, 400.0);
    assert_eq!(fit_height_factor(available_size, image_size), Some(1.0));
}

#[test]
fn fit_height_factor_exceeds_one_for_a_wide_image() {
    let available_size = egui::Vec2::new(800.0, 300.0);
    let image_size = egui::Vec2::new(400.0, 100.0); // fitted (800, 200)
    assert_eq!(fit_height_factor(available_size, image_size), Some(1.5));
}

#[test]
fn square_image_fit_factors_are_one() {
    let available_size = egui::Vec2::new(400.0, 400.0);
    let image_size = egui::Vec2::new(100.0, 100.0);
    assert_eq!(fit_width_factor(available_size, image_size), Some(1.0));
    assert_eq!(fit_height_factor(available_size, image_size), Some(1.0));
}

#[test]
fn fit_factors_reject_non_positive_dimensions() {
    let usable = egui::Vec2::new(100.0, 100.0);
    assert_eq!(fit_width_factor(usable, egui::Vec2::new(0.0, 10.0)), None);
    assert_eq!(fit_width_factor(usable, egui::Vec2::new(10.0, 0.0)), None);
    assert_eq!(fit_width_factor(egui::Vec2::new(0.0, 100.0), usable), None);
    assert_eq!(fit_width_factor(egui::Vec2::new(100.0, -1.0), usable), None);
    assert_eq!(fit_height_factor(usable, egui::Vec2::new(0.0, 10.0)), None);
    assert_eq!(fit_height_factor(usable, egui::Vec2::new(10.0, 0.0)), None);
    assert_eq!(fit_height_factor(egui::Vec2::new(0.0, 100.0), usable), None);
    assert_eq!(
        fit_height_factor(egui::Vec2::new(100.0, -1.0), usable),
        None
    );
}

#[test]
fn fit_width_factor_clamps_at_the_zoom_ceiling() {
    let available_size = egui::Vec2::new(10.0, 10.0);
    let image_size = egui::Vec2::new(10.0, 1000.0); // raw ratio ~100 >> 8
    assert_eq!(fit_width_factor(available_size, image_size), Some(8.0));
}
