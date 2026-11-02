//! Empty-folder browsing view: the one centered status message drawn in the
//! blank central surface while the app browses a folder without an opened
//! image (D2-final). The pill's message line stays empty in this mode.

use eframe::egui;

/// The blank-surface status: the browsed folder holds no displayable image.
pub(super) const EMPTY_FOLDER_MESSAGE: &str = "Empty folder";

/// Draws the message centered in the central surface -- NOT in the pill's
/// message line, which the pill keeps empty in empty-folder mode.
pub(super) fn show_empty_folder_message(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::centered_and_justified(egui::Direction::TopDown),
        |message_ui| {
            message_ui.label(EMPTY_FOLDER_MESSAGE);
        },
    );
}
