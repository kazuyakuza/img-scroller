//! Empty-state view: the fresh empty state's central layout carrying the ONE
//! centered Open button (no image AND no folder context). Extracted verbatim
//! from the app shell to keep `app.rs` within the per-file line cap. Part of
//! the chrome: rendered only while fullscreen is OFF (reachable only through
//! that dispatch gate).

use eframe::egui;

use super::ImagesScrollerApp;

impl ImagesScrollerApp {
    /// Empty-state layout: an empty central paint carrying the ONE center Open button (the fresh empty state: no image AND no folder context).
    pub(super) fn show_empty_state_layout(
        &mut self,
        open_dialog_frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        egui::CentralPanel::default().show(ui, |central_ui| {
            self.show_centered_open_button(open_dialog_frame, central_ui);
        });
    }

    /// The only Open control: anchored to the window center in the empty state;
    /// the click reuses the native open dialog seam (the retired toolbar's path).
    fn show_centered_open_button(
        &mut self,
        open_dialog_frame: &mut eframe::Frame,
        central_ui: &mut egui::Ui,
    ) {
        egui::Area::new(egui::Id::unique(EMPTY_STATE_OPEN_AREA_ID))
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(central_ui.ctx(), |area_ui| {
                let open_button =
                    egui::Button::new(egui::RichText::new(OPEN_LABEL_TEXT).size(OPEN_LABEL_SIZE))
                        .min_size(OPEN_BUTTON_SIZE);
                if area_ui.add(open_button).clicked() {
                    self.open_via_dialog(open_dialog_frame);
                }
            });
    }
}

const EMPTY_STATE_OPEN_AREA_ID: &str = "empty_state_open_button";
const OPEN_LABEL_TEXT: &str = "Open…";
const OPEN_LABEL_SIZE: f32 = 18.0;
const OPEN_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(180.0, 48.0);
