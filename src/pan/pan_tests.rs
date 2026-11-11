//! Pure pan-math tests: per-axis overflow, the symmetric clamp, the pan step
//! sign and size, the at-edge decision, and the pinned constants. Every vector
//! is exact in f32, so `assert_eq!` comparisons stay exact.

use eframe::egui;

use super::{
    ArrowPanInputs, ArrowPanOutcome, PanAxis, PanSign, arrow_outcome, axis_overflow_pair,
    axis_pan_limit, axis_pan_step, clamped_axis_offset,
};

#[test]
fn wide_image_overflows_only_horizontally() {
    let overflow = axis_overflow_pair(egui::vec2(1600.0, 300.0), egui::vec2(800.0, 300.0));
    assert_eq!(overflow, egui::vec2(800.0, 0.0));
}

#[test]
fn tall_image_overflows_only_vertically() {
    let overflow = axis_overflow_pair(egui::vec2(300.0, 1600.0), egui::vec2(300.0, 800.0));
    assert_eq!(overflow, egui::vec2(0.0, 800.0));
}

#[test]
fn fitting_image_has_zero_overflow_on_both_axes() {
    let overflow = axis_overflow_pair(egui::vec2(400.0, 200.0), egui::vec2(800.0, 300.0));
    assert_eq!(overflow, egui::Vec2::ZERO);
}

#[test]
fn clamped_axis_offset_stays_inside_the_symmetric_bounds() {
    assert_eq!(clamped_axis_offset(1000.0, 800.0), 400.0);
    assert_eq!(clamped_axis_offset(-1000.0, 800.0), -400.0);
    assert_eq!(clamped_axis_offset(0.0, 800.0), 0.0);
}

#[test]
fn clamped_axis_offset_collapses_to_zero_without_overflow() {
    assert_eq!(clamped_axis_offset(500.0, 0.0), 0.0);
    assert_eq!(clamped_axis_offset(-500.0, 0.0), 0.0);
}

#[test]
fn arrow_outcome_navigates_directly_when_the_axis_fits() {
    let fitting_inputs = ArrowPanInputs {
        raw_offset: egui::Vec2::ZERO,
        draw_size: egui::vec2(300.0, 300.0),
        available_size: egui::vec2(800.0, 300.0),
        axis: PanAxis::Horizontal,
        sign: PanSign::Positive,
        press_is_non_repeat: false,
    };
    let fitting_outcome = arrow_outcome(fitting_inputs);
    assert!(matches!(fitting_outcome, ArrowPanOutcome::Navigate));
    let repeat_inputs = ArrowPanInputs {
        press_is_non_repeat: true,
        ..fitting_inputs
    };
    let repeat_outcome = arrow_outcome(repeat_inputs);
    assert!(matches!(repeat_outcome, ArrowPanOutcome::Navigate));
}

#[test]
fn arrow_outcome_pans_by_the_pinned_step_when_not_at_the_edge() {
    let positive_inputs = ArrowPanInputs {
        raw_offset: egui::Vec2::ZERO,
        draw_size: egui::vec2(1600.0, 300.0),
        available_size: egui::vec2(800.0, 300.0),
        axis: PanAxis::Horizontal,
        sign: PanSign::Positive,
        press_is_non_repeat: true,
    };
    let positive_outcome = arrow_outcome(positive_inputs);
    assert!(matches!(positive_outcome, ArrowPanOutcome::Pan(delta) if delta == 100.0));
    let negative_inputs = ArrowPanInputs {
        sign: PanSign::Negative,
        ..positive_inputs
    };
    let negative_outcome = arrow_outcome(negative_inputs);
    assert!(matches!(negative_outcome, ArrowPanOutcome::Pan(delta) if delta == -100.0));
}

#[test]
fn arrow_outcome_absorbs_a_repeat_press_at_the_edge() {
    let positive_edge_inputs = ArrowPanInputs {
        raw_offset: egui::vec2(400.0, 0.0),
        draw_size: egui::vec2(1600.0, 300.0),
        available_size: egui::vec2(800.0, 300.0),
        axis: PanAxis::Horizontal,
        sign: PanSign::Positive,
        press_is_non_repeat: false,
    };
    let positive_edge_outcome = arrow_outcome(positive_edge_inputs);
    assert!(matches!(positive_edge_outcome, ArrowPanOutcome::Pan(delta) if delta == 0.0));
    assert!(!matches!(positive_edge_outcome, ArrowPanOutcome::Navigate));
    let negative_edge_inputs = ArrowPanInputs {
        raw_offset: egui::vec2(-400.0, 0.0),
        sign: PanSign::Negative,
        ..positive_edge_inputs
    };
    let negative_edge_outcome = arrow_outcome(negative_edge_inputs);
    assert!(matches!(negative_edge_outcome, ArrowPanOutcome::Pan(delta) if delta == 0.0));
    assert!(!matches!(negative_edge_outcome, ArrowPanOutcome::Navigate));
}

#[test]
fn arrow_outcome_navigates_on_a_distinct_press_at_the_edge() {
    let positive_edge_inputs = ArrowPanInputs {
        raw_offset: egui::vec2(400.0, 0.0),
        draw_size: egui::vec2(1600.0, 300.0),
        available_size: egui::vec2(800.0, 300.0),
        axis: PanAxis::Horizontal,
        sign: PanSign::Positive,
        press_is_non_repeat: true,
    };
    let positive_edge_outcome = arrow_outcome(positive_edge_inputs);
    assert!(matches!(positive_edge_outcome, ArrowPanOutcome::Navigate));
    let negative_edge_inputs = ArrowPanInputs {
        raw_offset: egui::vec2(-400.0, 0.0),
        sign: PanSign::Negative,
        ..positive_edge_inputs
    };
    let negative_edge_outcome = arrow_outcome(negative_edge_inputs);
    assert!(matches!(negative_edge_outcome, ArrowPanOutcome::Navigate));
}

#[test]
fn an_overshooting_step_is_absorbed_by_the_clamp() {
    let overflow = 800.0;
    let limit = axis_pan_limit(overflow);
    let raw_offset = limit - 40.0;
    let landed_offset = clamped_axis_offset(raw_offset + axis_pan_step(800.0), overflow);
    assert_eq!(landed_offset, limit);
}

#[test]
fn pan_constants_are_pinned() {
    assert_eq!(super::PAN_STEP_FRACTION, 0.125);
    assert_eq!(super::PAN_EDGE_TOLERANCE, 0.5);
}
