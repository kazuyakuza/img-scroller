//! Zoom math (Wave-4 Task 2): the multiplicative step scaling within the fixed
//! SYMMETRIC bounds `[ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM]` around the
//! neutral fit factor `1.0` (the "0" no-zoom baseline), the restore-to-fit
//! reset, the at-bound detection, and the fit-relative draw size. Zoom-out
//! BELOW fit is allowed and reaches the ÷8 floor when the ceiling is ×8.
//! Pure calculations only: no egui painting, no texture or pixel code.
//! Sibling of `src/scaling.rs`. Authoritative for the MATH only: the clamp
//! below is silent and zoom actions write no messages, so the silent at-bound
//! no-op and the reset-on-every-successful-open policy both live in the app
//! seam (`src/app/zoom.rs` and the `accept_open_outcome` wiring in `src/app.rs`).

use eframe::egui;

/// Fixed multiplicative factor of one zoom-in/zoom-out step.
pub(crate) const ZOOM_STEP_MULTIPLIER: f32 = 1.25;
/// Upper zoom bound: the largest supported zoom-in.
pub(crate) const ZOOM_FACTOR_MAXIMUM: f32 = 8.0;
/// Lower zoom bound: the SYMMETRIC zoom-out floor -- the exact inverse of
/// the ceiling (when the ceiling is ×8, zoom-out reaches ÷8 of the fit size).
pub(crate) const ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM;
/// The restore-to-fit target: the neutral fit baseline ("0" / no-zoom),
/// NOT the zoom-out floor -- fit is the zero point INSIDE the symmetric
/// range [ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM].
pub(crate) const RESTORE_TO_FIT_FACTOR: f32 = 1.0;
/// Absorption bound for the at-bound detections (fits-factor drift guard).
const ZOOM_BOUND_TOLERANCE: f32 = 1.0e-4;

/// The zoom factor after one zoom-in step from `factor`, clamped to the bounds.
pub(crate) fn zoom_in_factor(factor: f32) -> f32 {
    clamped_zoom_factor(factor * ZOOM_STEP_MULTIPLIER)
}

/// The zoom factor after one zoom-out step from `factor`, clamped at the
/// symmetric floor.
pub(crate) fn zoom_out_factor(factor: f32) -> f32 {
    clamped_zoom_factor(factor / ZOOM_STEP_MULTIPLIER)
}

/// Shared with the fit-factor math in `src/scaling.rs` (the ONE clamp source).
pub(crate) fn clamped_zoom_factor(factor: f32) -> f32 {
    factor.clamp(ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM)
}

/// Whether `factor` has reached the symmetric zoom-out floor (zoom-out is
/// inert there; the restore-fit button is NOT -- it re-anchors to 1.0).
pub(crate) fn zoom_factor_is_at_minimum(factor: f32) -> bool {
    factor <= ZOOM_FACTOR_MINIMUM + ZOOM_BOUND_TOLERANCE
}

/// Whether `factor` has reached the ceiling (zoom-in is inert there).
pub(crate) fn zoom_factor_is_at_maximum(factor: f32) -> bool {
    factor >= ZOOM_FACTOR_MAXIMUM - ZOOM_BOUND_TOLERANCE
}

/// Whether `factor` sits AWAY from the neutral fit baseline (1.0) outside
/// the bound tolerance -- the restore-fit-button enablement. True at both
/// bounds and everywhere between except fit itself.
pub(crate) fn zoom_factor_is_off_restore_target(factor: f32) -> bool {
    (factor - RESTORE_TO_FIT_FACTOR).abs() > ZOOM_BOUND_TOLERANCE
}

/// The per-frame draw size of the image at `fit_size` scaled by the zoom factor
/// (Wave-5 pan composes its offsets on this size; no scroll/pan this wave).
pub(crate) fn zoomed_draw_size(fit_size: egui::Vec2, zoom_factor: f32) -> egui::Vec2 {
    fit_size * zoom_factor
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui;

    #[test]
    fn zoom_in_step_multiplies_by_the_step_multiplier() {
        assert_eq!(zoom_in_factor(1.0), 1.25);
        assert_eq!(zoom_in_factor(4.0), 5.0); // both values exact in f32
    }

    #[test]
    fn zoom_in_clamps_at_the_maximum() {
        assert_eq!(zoom_in_factor(7.0), ZOOM_FACTOR_MAXIMUM); // 8.75 requested
        assert_eq!(zoom_in_factor(8.0), ZOOM_FACTOR_MAXIMUM);
    }

    #[test]
    fn zoom_out_step_divides_by_the_step_multiplier() {
        assert_eq!(zoom_out_factor(10.0), 8.0);
        assert_eq!(zoom_out_factor(5.0), 4.0);
    }

    #[test]
    fn zoom_out_clamps_at_the_symmetric_floor() {
        assert_eq!(zoom_out_factor(1.0), 0.8); // below fit is now REACHED
        assert_eq!(zoom_out_factor(0.1), ZOOM_FACTOR_MINIMUM); // 0.1/1.25 clamps
        assert_eq!(zoom_out_factor(ZOOM_FACTOR_MINIMUM), ZOOM_FACTOR_MINIMUM);
    }

    #[test]
    fn zoom_factor_is_at_minimum_is_exact_at_the_symmetric_floor() {
        assert!(zoom_factor_is_at_minimum(ZOOM_FACTOR_MINIMUM));
        assert!(!zoom_factor_is_at_minimum(1.0)); // the fit level is mid-range now
    }

    #[test]
    fn zoom_factor_is_at_maximum_is_exact_at_the_ceiling() {
        assert!(zoom_factor_is_at_maximum(8.0));
        assert!(!zoom_factor_is_at_maximum(5.0));
    }

    #[test]
    fn zoomed_draw_size_multiplies_the_fit_size_by_the_zoom_factor() {
        let fit_size = egui::Vec2::new(200.0, 100.0);
        assert_eq!(
            zoomed_draw_size(fit_size, 2.0),
            egui::Vec2::new(400.0, 200.0)
        );
        let small_fit = egui::Vec2::new(320.0, 240.0);
        assert_eq!(zoomed_draw_size(small_fit, 1.0), small_fit); // fit = no-op zoom
    }

    #[test]
    fn zoom_bounds_and_restore_constant_are_pinned() {
        assert_eq!(ZOOM_STEP_MULTIPLIER, 1.25);
        assert_eq!(ZOOM_FACTOR_MAXIMUM, 8.0);
        assert_eq!(ZOOM_FACTOR_MINIMUM, 0.125);
        assert_eq!(ZOOM_FACTOR_MINIMUM * ZOOM_FACTOR_MAXIMUM, 1.0); // symmetric inverse
        assert_eq!(RESTORE_TO_FIT_FACTOR, 1.0);
    }
}
