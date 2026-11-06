//! The central-surface bodies shared by the chrome and fullscreen layouts:
//! the ONE ImagePainting literal of the application (which decoded image
//! at WHICH session zoom factor paints) and the empty-folder
//! fallback of the chrome navigation layout.

use eframe::egui;

use super::{ImagesScrollerApp, empty_folder_view};
use crate::app_display::ImagePainting;

impl ImagesScrollerApp {
    /// The chrome navigation surface: the image paint when open, otherwise the
    /// centered empty-folder message.
    pub(super) fn show_navigation_central_surface(&mut self, central_ui: &mut egui::Ui) {
        if !self.image_is_open() {
            empty_folder_view::show_empty_folder_message(central_ui);
            return;
        }
        self.paint_open_image(central_ui);
    }

    /// The fullscreen surface: ONLY the image paint while open (no fallback).
    pub(super) fn show_central_image_while_open(&mut self, central_ui: &mut egui::Ui) {
        if self.image_is_open() {
            self.paint_open_image(central_ui);
        }
    }

    /// The single paint request: the decoded image at the whole-image fitted
    /// size, scaled by the session zoom factor (inside `CurrentImageDisplay`).
    fn paint_open_image(&mut self, central_ui: &mut egui::Ui) {
        let Some(decoded_image) = self.decoded_image.as_ref() else {
            return;
        };
        self.current_display.paint(
            central_ui,
            ImagePainting {
                decoded_image,
                zoom_factor: self.zoom_factor,
            },
        );
    }
}
