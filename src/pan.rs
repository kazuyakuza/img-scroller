//! Wave-5 pan math: per-axis overflow, clamped offsets, at-edge predicates,
//! the pan step, and the arrow pan-or-navigate decision: an axis that fits
//! entirely navigates directly; an overflowing axis pans on every press
//! (auto-repeats included) until its edge, where only a distinct non-repeat
//! press navigates. Authoritative for the MATH only; the reset policy lives in
//! the app (`src/app.rs` and `src/app/pan.rs`), and the paint consumes this
//! module in `src/app_display.rs`.

use eframe::egui;

/// One arrow press pans by this fraction of the available axis size.
pub(crate) const PAN_STEP_FRACTION: f32 = 0.125;
/// Float-jitter absorption for the at-edge predicates.
pub(crate) const PAN_EDGE_TOLERANCE: f32 = 0.5;

/// Overflow on one axis: max(0, draw - available).
pub(crate) fn axis_overflow(draw_axis: f32, available_axis: f32) -> f32 {
    (draw_axis - available_axis).max(0.0)
}

/// Half-width of the symmetric valid pan range `[-limit, +limit]`.
pub(crate) fn axis_pan_limit(overflow: f32) -> f32 {
    overflow * 0.5
}

/// Clamps one axis of a raw offset into `[-limit, +limit]`.
pub(crate) fn clamped_axis_offset(raw_offset: f32, overflow: f32) -> f32 {
    let limit = axis_pan_limit(overflow);
    raw_offset.clamp(-limit, limit)
}

/// At the negative (left/top) edge of the symmetric range.
pub(crate) fn axis_is_at_negative_edge(clamped_offset: f32, overflow: f32) -> bool {
    clamped_offset <= -axis_pan_limit(overflow) + PAN_EDGE_TOLERANCE
}

/// At the positive (right/bottom) edge of the symmetric range.
pub(crate) fn axis_is_at_positive_edge(clamped_offset: f32, overflow: f32) -> bool {
    clamped_offset >= axis_pan_limit(overflow) - PAN_EDGE_TOLERANCE
}

/// One pan step on one axis.
pub(crate) fn axis_pan_step(available_axis: f32) -> f32 {
    PAN_STEP_FRACTION * available_axis
}

/// Per-axis overflow pair.
pub(crate) fn axis_overflow_pair(draw_size: egui::Vec2, available_size: egui::Vec2) -> egui::Vec2 {
    egui::Vec2::new(
        axis_overflow(draw_size.x, available_size.x),
        axis_overflow(draw_size.y, available_size.y),
    )
}

/// Per-axis clamped offset pair.
pub(crate) fn clamped_pan_offset(raw_offset: egui::Vec2, overflow: egui::Vec2) -> egui::Vec2 {
    egui::Vec2::new(
        clamped_axis_offset(raw_offset.x, overflow.x),
        clamped_axis_offset(raw_offset.y, overflow.y),
    )
}

/// The axis an arrow key acts on.
#[derive(Clone, Copy)]
pub(crate) enum PanAxis {
    Horizontal,
    Vertical,
}

impl PanAxis {
    pub(crate) fn component(self, size: egui::Vec2) -> f32 {
        match self {
            PanAxis::Horizontal => size.x,
            PanAxis::Vertical => size.y,
        }
    }

    pub(crate) fn with_component(self, size: egui::Vec2, value: f32) -> egui::Vec2 {
        match self {
            PanAxis::Horizontal => egui::Vec2::new(value, size.y),
            PanAxis::Vertical => egui::Vec2::new(size.x, value),
        }
    }
}

/// The direction an arrow key pans toward: Negative = left/top,
/// Positive = right/bottom.
#[derive(Clone, Copy)]
pub(crate) enum PanSign {
    Negative,
    Positive,
}

impl PanSign {
    fn is_at_edge(self, clamped_offset: f32, overflow: f32) -> bool {
        match self {
            PanSign::Negative => axis_is_at_negative_edge(clamped_offset, overflow),
            PanSign::Positive => axis_is_at_positive_edge(clamped_offset, overflow),
        }
    }
}

/// Everything one arrow press needs (one param object: the max-args rule).
#[derive(Clone, Copy)]
pub(crate) struct ArrowPanInputs {
    pub(crate) raw_offset: egui::Vec2,
    pub(crate) draw_size: egui::Vec2,
    pub(crate) available_size: egui::Vec2,
    pub(crate) axis: PanAxis,
    pub(crate) sign: PanSign,
    pub(crate) press_is_non_repeat: bool,
}

pub(crate) enum ArrowPanOutcome {
    /// Signed delta for the axis offset (the caller re-clamps; 0.0 = a repeat
    /// press absorbed at an edge).
    Pan(f32),
    Navigate,
}

pub(crate) fn arrow_outcome(inputs: ArrowPanInputs) -> ArrowPanOutcome {
    let overflow_pair = axis_overflow_pair(inputs.draw_size, inputs.available_size);
    let overflow = inputs.axis.component(overflow_pair);
    if overflow <= 0.0 {
        return ArrowPanOutcome::Navigate;
    }
    let clamped_offset = clamped_axis_offset(inputs.axis.component(inputs.raw_offset), overflow);
    if inputs.sign.is_at_edge(clamped_offset, overflow) {
        return at_edge_outcome(inputs.press_is_non_repeat);
    }
    ArrowPanOutcome::Pan(signed_pan_step(
        inputs.axis.component(inputs.available_size),
        inputs.sign,
    ))
}

fn at_edge_outcome(press_is_non_repeat: bool) -> ArrowPanOutcome {
    if press_is_non_repeat {
        return ArrowPanOutcome::Navigate;
    }
    ArrowPanOutcome::Pan(0.0)
}

fn signed_pan_step(available_axis: f32, sign: PanSign) -> f32 {
    match sign {
        PanSign::Negative => -axis_pan_step(available_axis),
        PanSign::Positive => axis_pan_step(available_axis),
    }
}

#[cfg(test)]
mod pan_tests;
