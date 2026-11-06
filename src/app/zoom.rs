//! Manual zoom (Wave-4 Task 2): the session zoom-factor seam, its keyboard
//! chords (I / Shift+Plus, O / Shift+Minus, 0 = restore fit), and the
//! at-bound step guards. Buttons live in `zoom_button.rs`; the pure math
//! lives in `crate::zoom`. Session state only: never persisted. The factor
//! lives in the symmetric range `[ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM]`
//! = `[1/8, 8]` with `1.0` the neutral fit / no-zoom baseline. Zoom-out steps
//! divide by 1.25 and keep going BELOW fit down to the floor; a step
//! attempted at that floor is a silent no-op, and a step landing on any bound
//! clamps silently. Zoom actions write NOTHING to `message_text` (wave 4.1
//! Task 4). The reset-on-every-successful-open policy is NOT here -- it lives
//! in `src/app.rs` (`accept_open_outcome`).
//!
//! One-shot fit semantics (Wave 4.1 Task 7): the two `set_zoom_to_fit_width`/
//! `set_zoom_to_fit_height` seams set the zoom factor ONLY; the `W`/`H` keys
//! and both fit buttons funnel through them. No mode state exists: after a
//! one-shot set the zoom in/out steps run from that level, `0`/restore and
//! every successful open reset to 1.0, and a window resize does NOT re-fit
//! (the paint multiplies the stored factor by the current whole-image fit
//! base every frame). The intake available size is the full egui viewport
//! rect (`viewport_rect().size()`; spec §5: one-shot display lands within a
//! few px of a full-window span). Fit actions are SILENT.
//!
//! The '+/-' aliases are strictly shift-gated: Shift+'+'/Shift+'-' fire,
//! plain '='/'+'/'-' do nothing (Wave 4.1 Task 6); keyboard-layout variance
//! is the OPERATOR-RUN note, not a code fallback.
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

impl ImagesScrollerApp {
    /// One zoom-in step; a step attempted at the ceiling is a silent no-op.
    pub(super) fn zoom_in_once(&mut self) {
        if zoom::zoom_factor_is_at_maximum(self.zoom_factor) {
            return;
        }
        self.zoom_factor = zoom::zoom_in_factor(self.zoom_factor);
    }

    /// One zoom-out step; a step attempted at the symmetric floor is a silent no-op.
    pub(super) fn zoom_out_once(&mut self) {
        if zoom::zoom_factor_is_at_minimum(self.zoom_factor) {
            return;
        }
        self.zoom_factor = zoom::zoom_out_factor(self.zoom_factor);
    }

    /// Hotkey `0` and the restore button: back to the neutral fit factor (1.0). Silent.
    pub(super) fn restore_zoom_to_fit(&mut self) {
        self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;
    }

    /// One-shot fit-width set (Wave 4.1 Task 7): the zoom factor ONLY -- the
    /// width-fit factor of the CURRENT image against the CURRENT available
    /// size (clamped inside by the pure math). No `message_text` write: fit
    /// actions are silent. No-op while no image is open (covers the fresh
    /// empty state AND the browsed empty-folder state -- one `image_is_open`
    /// gate) and on a non-positive decoded payload (unreachable in practice).
    pub(super) fn set_zoom_to_fit_width(&mut self, available_size: egui::Vec2) {
        if !self.image_is_open() {
            return;
        }
        let Some(decoded_image) = self.decoded_image.as_ref() else {
            return;
        };
        let Some(factor) = crate::scaling::fit_width_factor(available_size, decoded_image.size())
        else {
            return;
        };
        self.zoom_factor = factor;
    }

    /// The height one-shot twin of `set_zoom_to_fit_width` (``H``/button).
    pub(super) fn set_zoom_to_fit_height(&mut self, available_size: egui::Vec2) {
        if !self.image_is_open() {
            return;
        }
        let Some(decoded_image) = self.decoded_image.as_ref() else {
            return;
        };
        let Some(factor) = crate::scaling::fit_height_factor(available_size, decoded_image.size())
        else {
            return;
        };
        self.zoom_factor = factor;
    }

    /// Zoom-in-button enablement: enabled until the ×8 ceiling is reached.
    pub(super) fn zoom_can_grow(&self) -> bool {
        !zoom::zoom_factor_is_at_maximum(self.zoom_factor)
    }

    /// Zoom-out-button enablement: enabled while the factor is above the
    /// symmetric floor (`ZOOM_FACTOR_MINIMUM`) -- zoom-out now works BELOW
    /// fit, so the fit level itself does NOT disable it (only the floor does).
    pub(super) fn zoom_can_shrink(&self) -> bool {
        !zoom::zoom_factor_is_at_minimum(self.zoom_factor)
    }

    /// Restore-fit-button enablement: enabled whenever the factor is away
    /// from the neutral fit value 1.0 (including at the zoom-out floor,
    /// where the zoom-out button is already disabled).
    pub(super) fn zoomed_off_fit(&self) -> bool {
        zoom::zoom_factor_is_off_restore_target(self.zoom_factor)
    }
}

/// The zoom-in chord set: hotkey `I`, or the Shift-gated '='/'+' resolutions
/// only (strict rule, Wave 4.1 Task 6; D4's unconditional Plus accept is
/// removed).
pub(super) fn zoom_in_chord_was_pressed(chord_context: &egui::Context) -> bool {
    if chord_context.input(|input| input.key_pressed(ZOOM_IN_KEY)) {
        return true;
    }
    zoom_in_shift_alias_was_pressed(chord_context)
}

/// The strict-shift alias rule (Wave 4.1 Task 6): an alias fires ONLY when
/// the alias key was pressed this frame AND Shift is held; the unshifted
/// '='/'+'/'-' keys do NOTHING. Pure: no egui reading.
pub(super) fn shift_alias_is_pressed(alias_key_was_pressed: bool, shift_held: bool) -> bool {
    alias_key_was_pressed && shift_held
}

/// The zoom-in shift alias: the `=`/`+` row key WHILE Shift is held. BOTH
/// egui logical resolutions of Shift+'+' are accepted (shift-gated):(a)
/// `Key::Equals` (unshifted '=' resolution on some layouts), (b) `Key::Plus`
/// (the reachable '+' resolution: shifted '=' on US layouts, unshifted '+'
/// on e.g. German layouts, and the numpad fallback). OPERATOR-RUN note:
/// keyboard-layout variance is operator-tested, not hidden in code.
fn zoom_in_shift_alias_was_pressed(chord_context: &egui::Context) -> bool {
    let shift_held = chord_context.input(|input| input.modifiers.shift);
    shift_alias_is_pressed(
        chord_context.input(|input| input.key_pressed(egui::Key::Equals)),
        shift_held,
    ) || shift_alias_is_pressed(
        chord_context.input(|input| input.key_pressed(egui::Key::Plus)),
        shift_held,
    )
}

/// The zoom-out chord set: hotkey `O`, or the Shift-gated '-' resolution only
/// (strict rule, Wave 4.1 Task 6).
pub(super) fn zoom_out_chord_was_pressed(chord_context: &egui::Context) -> bool {
    if chord_context.input(|input| input.key_pressed(ZOOM_OUT_KEY)) {
        return true;
    }
    zoom_out_shift_alias_was_pressed(chord_context)
}

/// The zoom-out shift alias: `-` WHILE Shift is held (`Key::Minus` is the
/// only egui resolution of the '-' key; the shifted '_' press resolves by
/// physical position back to `Key::Minus` per the egui-winit fallback).
fn zoom_out_shift_alias_was_pressed(chord_context: &egui::Context) -> bool {
    let shift_held = chord_context.input(|input| input.modifiers.shift);
    shift_alias_is_pressed(
        chord_context.input(|input| input.key_pressed(egui::Key::Minus)),
        shift_held,
    )
}
