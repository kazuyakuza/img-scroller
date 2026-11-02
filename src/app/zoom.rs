//! Manual zoom (Wave-4 Task 2): the session zoom-factor seam, its keyboard
//! chords (I / Shift+Plus, O / Shift+Minus, 0 = restore fit), and the
//! at-bound pill-message assignment. Buttons live in `zoom_button.rs`; the
//! pure math lives in `crate::zoom`. Session state only: never persisted.
//! The pill message fires ONLY when a step is attempted at a bound it already
//! holds; a step that merely lands on a bound clamps silently (D2). The
//! reset-on-every-successful-open policy is NOT here -- it lives in `src/app.rs`
//! (`accept_open_outcome`).
//!
//! Fit modes (Wave-4 Task 3) share this seam: `set_fit_mode` is the ONE
//! mode-set-and-reset call (hotkeys `W`/`H`, both pill buttons, the restore
//! path, and the open-success reset all funnel through it).
//!
//! AI Agents: this app seam (`super::zoom`) is a separate module from the
//! pure-math `crate::zoom` even though both share the leaf name `zoom`; keep the
//! two distinct (in `src/app.rs` the `mod zoom;` seam shadows the math, so the
//! math must be referenced fully-qualified as `crate::zoom::...`).

use eframe::egui;

use super::ImagesScrollerApp;
use crate::zoom;

/// Zoom-in hotkey `I` (alias family: the Shift+'+' chord below).
const ZOOM_IN_KEY: egui::Key = egui::Key::I;
/// Zoom-out hotkey `O` (alias family: the Shift+'-' chord below).
const ZOOM_OUT_KEY: egui::Key = egui::Key::O;
/// Restore-to-fit hotkey: the digit zero (egui's exact digit member).
pub(super) const RESTORE_FIT_KEY: egui::Key = egui::Key::Num0;

/// Fit-width hotkey `W` (Wave-4 Task 3, user-approved K1 binding).
pub(super) const FIT_WIDTH_KEY: egui::Key = egui::Key::W;
/// Fit-height hotkey `H` (the user-approved twin of `W`).
pub(super) const FIT_HEIGHT_KEY: egui::Key = egui::Key::H;

/// Pill label announced once, at fit-width activation (singular channel).
pub(super) const FIT_WIDTH_MESSAGE: &str = "Fit width";
/// Pill label announced once, at fit-height activation.
pub(super) const FIT_HEIGHT_MESSAGE: &str = "Fit height";

/// Pill message shown when a zoom-out step is rejected at the fit bound
/// (pinned by the zoom tests; reaches the pill through `message_text`).
pub(super) const ZOOM_AT_MINIMUM_MESSAGE: &str = "Zoom min";
/// Pill message shown when a zoom-in step is rejected at the x8 bound.
pub(super) const ZOOM_AT_MAXIMUM_MESSAGE: &str = "Zoom max";

impl ImagesScrollerApp {
    /// One zoom-in step; a step rejected at the ceiling reports "Zoom max".
    pub(super) fn zoom_in_once(&mut self) {
        if self.trap_zoom_maximum() {
            return;
        }
        self.zoom_factor = zoom::zoom_in_factor(self.zoom_factor);
    }

    /// One zoom-out step; a step rejected at the fit floor reports "Zoom min".
    pub(super) fn zoom_out_once(&mut self) {
        if self.trap_zoom_minimum() {
            return;
        }
        self.zoom_factor = zoom::zoom_out_factor(self.zoom_factor);
    }

    /// Hotkey `0` and the restore button: back to whole-image fit (Wave-4
    /// Task 3: the mode returns to `FitVisible` too). Silent: the fit state
    /// is its own feedback.
    pub(super) fn restore_zoom_to_fit(&mut self) {
        self.set_fit_mode(crate::scaling::FitMode::FitVisible);
    }

    /// Sets the session fit mode and anchors the zoom factor to that mode's
    /// 1× base (Option A interplay). Non-default modes announce themselves
    /// once, at activation; `FitVisible` stays silent and clears the standing
    /// message (it is also the restore / open-success reset target).
    pub(super) fn set_fit_mode(&mut self, fit_mode: crate::scaling::FitMode) {
        self.fit_mode = fit_mode;
        self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;
        self.announce_fit_mode(fit_mode);
    }

    fn announce_fit_mode(&mut self, fit_mode: crate::scaling::FitMode) {
        self.message_text = fit_mode_activation_label(fit_mode);
    }

    /// Zoom-in-button enablement: enabled until the x8 ceiling is reached.
    pub(super) fn zoom_can_grow(&self) -> bool {
        !zoom::zoom_factor_is_at_maximum(self.zoom_factor)
    }

    /// Enablement of BOTH the zoom-out and the restore-fit buttons:
    /// enabled everywhere except exactly at the fit floor.
    pub(super) fn zoomed_beyond_fit(&self) -> bool {
        !zoom::zoom_factor_is_at_minimum(self.zoom_factor)
    }

    /// Rejected-step feedback (D2): a step attempted while already at the
    /// bound leaves the factor unchanged; the message clears on the next
    /// successful open (the standing `message_text` policy).
    fn trap_zoom_maximum(&mut self) -> bool {
        if !zoom::zoom_factor_is_at_maximum(self.zoom_factor) {
            return false;
        }
        self.message_text = Some(ZOOM_AT_MAXIMUM_MESSAGE);
        true
    }

    /// The fit-floor twin of `trap_zoom_maximum` (D2).
    fn trap_zoom_minimum(&mut self) -> bool {
        if !zoom::zoom_factor_is_at_minimum(self.zoom_factor) {
            return false;
        }
        self.message_text = Some(ZOOM_AT_MINIMUM_MESSAGE);
        true
    }
}

/// The zoom-in chord set: hotkey `I`, or the Shift+'+' alias (Shift held
/// while `=`/`+` is pressed; BOTH egui logical resolutions of Shift+Plus are
/// accepted -- D4).
pub(super) fn zoom_in_chord_was_pressed(chord_context: &egui::Context) -> bool {
    if chord_context.input(|input| input.key_pressed(ZOOM_IN_KEY)) {
        return true;
    }
    zoom_in_shift_alias_was_pressed(chord_context)
}

fn zoom_in_shift_alias_was_pressed(chord_context: &egui::Context) -> bool {
    if chord_context.input(|input| input.key_pressed(egui::Key::Plus)) {
        return true;
    }
    if !chord_context.input(|input| input.key_pressed(egui::Key::Equals)) {
        return false;
    }
    chord_context.input(|input| input.modifiers.shift)
}

/// The zoom-out chord set: hotkey `O`, or the Shift+'-' alias (Shift held
/// while `-` resolves to `Key::Minus`).
pub(super) fn zoom_out_chord_was_pressed(chord_context: &egui::Context) -> bool {
    if chord_context.input(|input| input.key_pressed(ZOOM_OUT_KEY)) {
        return true;
    }
    if !chord_context.input(|input| input.key_pressed(egui::Key::Minus)) {
        return false;
    }
    chord_context.input(|input| input.modifiers.shift)
}

/// The once-at-activation pill label of a fit mode; the default `FitVisible`
/// is silent by design.
fn fit_mode_activation_label(fit_mode: crate::scaling::FitMode) -> Option<&'static str> {
    match fit_mode {
        crate::scaling::FitMode::FitVisible => None,
        crate::scaling::FitMode::FitWidth => Some(FIT_WIDTH_MESSAGE),
        crate::scaling::FitMode::FitHeight => Some(FIT_HEIGHT_MESSAGE),
    }
}
