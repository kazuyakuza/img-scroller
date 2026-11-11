//! Keyboard-only pan-or-navigate controller (Wave 5): the arrow-key seam that
//! decides, per overflowing axis, between panning one step and navigating at
//! the image edge. The pure math lives in `crate::pan` (referenced fully
//! qualified because `mod pan;` in `src/app.rs` shadows it inside app
//! children, the same note style as `src/app/zoom.rs`). Mouse pan is out of
//! scope; navigation reuses `navigate` unchanged (Wave-3 rule untouched).

use eframe::egui;

use super::ImagesScrollerApp;
use super::NavigationDirection;
use crate::pan;

/// One arrow-key press handed to the controller: the navigation direction the
/// key maps to and whether the event is a DISTINCT (non-repeat) press.
#[derive(Clone, Copy)]
pub(super) struct ArrowKeyPress {
    pub(super) direction: NavigationDirection,
    pub(super) non_repeat: bool,
}

/// True exactly for a non-repeat pressed `arrow_key` event in this frame's
/// events (the at-edge navigation gate; pan steps do NOT require it).
pub(super) fn non_repeat_arrow_press_occurred(
    frame_events: &[egui::Event],
    arrow_key: egui::Key,
) -> bool {
    frame_events.iter().any(|event| {
        matches!(
            event,
            egui::Event::Key {
                key,
                pressed: true,
                repeat: false,
                ..
            } if *key == arrow_key
        )
    })
}

fn arrow_pan_axis(direction: NavigationDirection) -> pan::PanAxis {
    match direction {
        NavigationDirection::Previous | NavigationDirection::Next => pan::PanAxis::Horizontal,
        NavigationDirection::PreviousSiblingFolder | NavigationDirection::NextSiblingFolder => {
            pan::PanAxis::Vertical
        }
    }
}

fn arrow_pan_sign(direction: NavigationDirection) -> pan::PanSign {
    match direction {
        NavigationDirection::Previous | NavigationDirection::PreviousSiblingFolder => {
            pan::PanSign::Negative
        }
        NavigationDirection::Next | NavigationDirection::NextSiblingFolder => {
            pan::PanSign::Positive
        }
    }
}

impl ImagesScrollerApp {
    /// Zeroes the pan offset (restore-to-fit path: the `0` key AND the
    /// restore button both funnel through `restore_zoom_to_fit`).
    pub(super) fn reset_pan_offset(&mut self) {
        self.pan_offset = egui::Vec2::ZERO;
    }

    /// The pan-or-navigate controller: no image open -> direct navigation
    /// (also the degenerate-payload guard); an image open -> the pure
    /// `pan::arrow_outcome` decision per the press's axis.
    pub(super) fn handle_arrow_key(&mut self, available_size: egui::Vec2, press: ArrowKeyPress) {
        if !self.image_is_open() {
            self.navigate(press.direction);
            return;
        }
        let Some(pan_inputs) = self.arrow_pan_inputs(available_size, press) else {
            self.navigate(press.direction);
            return;
        };
        self.apply_arrow_pan_outcome(pan_inputs, press);
    }

    /// Builds the pure request: draw size from the current payload and zoom
    /// factor over the intake available size; axis/sign from the direction.
    /// None when the payload or its fit is degenerate (unreachable in practice).
    fn arrow_pan_inputs(
        &self,
        available_size: egui::Vec2,
        press: ArrowKeyPress,
    ) -> Option<pan::ArrowPanInputs> {
        let decoded_image = self.decoded_image.as_ref()?;
        let fit_size = crate::scaling::fitted_size(available_size, decoded_image.size())?;
        Some(pan::ArrowPanInputs {
            raw_offset: self.pan_offset,
            draw_size: crate::zoom::zoomed_draw_size(fit_size, self.zoom_factor),
            available_size,
            axis: arrow_pan_axis(press.direction),
            sign: arrow_pan_sign(press.direction),
            press_is_non_repeat: press.non_repeat,
        })
    }

    /// Applies the pure outcome: Pan(delta) writes the clamped result back;
    /// Navigate calls the shared seam verbatim (Wave-3 rule untouched).
    fn apply_arrow_pan_outcome(&mut self, pan_inputs: pan::ArrowPanInputs, press: ArrowKeyPress) {
        match pan::arrow_outcome(pan_inputs) {
            pan::ArrowPanOutcome::Pan(signed_delta) => {
                self.apply_pan_delta(pan_inputs, signed_delta);
            }
            pan::ArrowPanOutcome::Navigate => self.navigate(press.direction),
        }
    }

    /// Adds the signed delta on the outcome's axis and stores the CLAMPED value
    /// (the last step lands exactly on the edge; a stale out-of-range stored
    /// value is normalized here -- the re-clamp-on-use rule).
    fn apply_pan_delta(&mut self, pan_inputs: pan::ArrowPanInputs, signed_delta: f32) {
        let axis = pan_inputs.axis;
        let overflow = pan::axis_overflow(
            axis.component(pan_inputs.draw_size),
            axis.component(pan_inputs.available_size),
        );
        let raw = axis.component(self.pan_offset);
        self.pan_offset = axis.with_component(
            self.pan_offset,
            pan::clamped_axis_offset(raw + signed_delta, overflow),
        );
    }
}
