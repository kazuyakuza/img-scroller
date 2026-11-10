//! Fullscreen mode (Wave-4 Task 1): the session flag state, its edge-triggered
//! viewport-command toggle, the ONE control-visibility gate, and the
//! fullscreen-only central surface. OFF restores all controls, image central.
//! Wave-4.1 Task 3 addition: fullscreen exits via F11 OR ESC; nothing else changes.
//! Wave-4.2 Task 4 addition: every toggle disarms the ESC-close popup.

use eframe::egui;

use super::ImagesScrollerApp;

/// F11: the de-facto Windows fullscreen toggle (spec §7 user-approved choice;
/// one of the two ON exits -- F11 toggles both ways and ESC exits ON, since
/// ALL controls are hidden in ON).
pub(super) const FULLSCREEN_TOGGLE_KEY: egui::Key = egui::Key::F11;

/// ESC exit: operator-requested additional exit from fullscreen ON; no other mode may consume it.
pub(super) const FULLSCREEN_EXIT_KEY: egui::Key = egui::Key::Escape;

/// True exactly when an ESC press this frame exits fullscreen: fullscreen must
/// be ON; ESC is a no-op everywhere else and never toggles fullscreen ON.
pub(super) fn escape_exits_fullscreen(fullscreen_on: bool, escape_pressed: bool) -> bool {
    fullscreen_on && escape_pressed
}

impl ImagesScrollerApp {
    /// Toggle action shared by the F11 intake and the pill button: flips the
    /// flag and sends the matching borderless-fullscreen command exactly once,
    /// in the frame the flag flips.
    pub(super) fn toggle_fullscreen_and_send_command(&mut self, screen_context: &egui::Context) {
        let fullscreen_after_toggle = self.toggle_fullscreen();
        self.disarm_esc_close_popup();
        screen_context
            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen_after_toggle));
    }

    /// The ONE control-visibility gate: EVERY overlay control (pill, empty-state
    /// Open button, empty-folder message, every future body control) renders
    /// only while fullscreen is OFF. Fullscreen keeps the central image surface.
    pub(super) fn full_ui_chrome_is_visible(&self) -> bool {
        !self.fullscreen_on
    }

    /// Fullscreen layout: the central surface only; the decoded image when one
    /// is open, otherwise the BLANK surface (no pill, no Open button, no
    /// "Empty folder" message).
    pub(super) fn show_fullscreen_layout(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |central_ui| {
            self.show_central_image_while_open(central_ui);
        });
    }

    /// Pure flip seam: exactly one flag inversion per trigger, the post-toggle
    /// flag equals the command value sent by the shared toggle action.
    pub(super) fn toggle_fullscreen(&mut self) -> bool {
        self.fullscreen_on = !self.fullscreen_on;
        self.fullscreen_on
    }
}
