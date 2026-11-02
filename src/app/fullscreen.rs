//! Fullscreen mode (Wave-4 Task 1): the session flag state, its edge-triggered
//! viewport-command toggle, the ONE control-visibility gate, and the
//! fullscreen-only central surface. OFF restores the exact 0.3.0 layout.

use eframe::egui;

use super::ImagesScrollerApp;

/// F11: the de-facto Windows fullscreen toggle (spec §7 user-approved choice;
/// the only exit while ON, since ALL controls are hidden in ON).
pub(super) const FULLSCREEN_TOGGLE_KEY: egui::Key = egui::Key::F11;

impl ImagesScrollerApp {
    /// Toggle action shared by the F11 intake and the pill button: flips the
    /// flag and sends the matching borderless-fullscreen command exactly once,
    /// in the frame the flag flips. Returns the post-toggle flag.
    pub(super) fn toggle_fullscreen_and_send_command(
        &mut self,
        screen_context: &egui::Context,
    ) -> bool {
        let fullscreen_after_toggle = self.toggle_fullscreen();
        screen_context
            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen_after_toggle));
        fullscreen_after_toggle
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
            if let Some(decoded_image) = self.decoded_image.as_ref() {
                self.current_display.paint(central_ui, decoded_image);
            }
        });
    }

    /// Pure flip seam: exactly one flag inversion per trigger, the post-toggle
    /// flag equals the command value sent by the shared toggle action.
    pub(super) fn toggle_fullscreen(&mut self) -> bool {
        self.fullscreen_on = !self.fullscreen_on;
        self.fullscreen_on
    }
}
