//! Application shell: the regular, resizable viewer window of Images Scroller.

use std::path::PathBuf;

use eframe::egui;

use crate::image_loading::{self, DecodedImage, ImageLoadFailure};
use crate::platform;

/// Title shown on the viewer window title bar (final app name, user-decided 2026-10-29).
pub const APP_WINDOW_TITLE: &str = "Images Scroller";

pub struct ImagesScrollerApp {
    /// Path of the last successfully decoded image (read again from Task 4's display code).
    #[allow(dead_code)]
    opened_image_path: Option<PathBuf>,
    /// Decoded pixels of the last successfully opened image (read again from Task 4's display code).
    #[allow(dead_code)]
    decoded_image: Option<DecodedImage>,
    message_text: Option<&'static str>,
}

impl ImagesScrollerApp {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self::empty_state()
    }

    fn empty_state() -> Self {
        Self {
            opened_image_path: None,
            decoded_image: None,
            message_text: None,
        }
    }

    /// Opens an image synchronously: decodes now, then applies the outcome to the open-flow state.
    pub fn open_image(&mut self, path: PathBuf) {
        let outcome = image_loading::load_image(&path);
        self.accept_open_outcome(path, outcome);
    }

    /// Failure keeps any previously loaded image and state untouched except the message.
    fn accept_open_outcome(
        &mut self,
        path: PathBuf,
        outcome: Result<DecodedImage, ImageLoadFailure>,
    ) {
        match outcome {
            Ok(decoded_image) => {
                self.opened_image_path = Some(path);
                self.decoded_image = Some(decoded_image);
                self.message_text = None;
            }
            Err(_failure) => {
                self.message_text = Some(image_loading::UNSUPPORTED_FILE_MESSAGE);
            }
        }
    }

    fn open_toolbar_button(&mut self, ui: &mut egui::Ui, frame: &eframe::Frame) {
        ui.horizontal(|ui| {
            if ui.button("Open…").clicked() {
                self.open_via_dialog(frame);
            }
        });
    }

    fn open_via_dialog(&mut self, frame: &eframe::Frame) {
        if let Some(image_path) = platform::show_open_image_dialog(frame) {
            self.open_image(image_path);
        }
    }
}

impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::Panel::top("open_toolbar").show(ui, |ui| {
            self.open_toolbar_button(ui, frame);
        });

        if let Some(message) = self.message_text {
            egui::Panel::bottom("message_bar").show(ui, |ui| {
                ui.label(message);
            });
        }

        egui::CentralPanel::default().show(ui, |_ui| {});
    }
}

pub fn window_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_WINDOW_TITLE)
            .with_resizable(true)
            .with_maximized(false),
        ..eframe::NativeOptions::default()
    }
}

#[cfg(test)]
mod tests {
    use super::ImagesScrollerApp;
    use crate::image_loading::{DecodedImage, UNSUPPORTED_FILE_MESSAGE};
    use std::path::PathBuf;

    #[test]
    fn window_title_is_the_final_app_name() {
        assert_eq!(super::APP_WINDOW_TITLE, "Images Scroller");
    }

    #[test]
    fn failed_open_via_cli_reports_the_message_and_keeps_no_path() {
        let mut app = ImagesScrollerApp::empty_state();
        app.open_image(PathBuf::from("unregistered-in-container.bin"));
        assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
        assert!(app.opened_image_path.is_none());
        assert!(app.decoded_image.is_none());
    }

    #[test]
    fn successful_open_stores_path_and_payload_and_clears_message() {
        let mut app = ImagesScrollerApp::empty_state();
        let image_path = PathBuf::from("photo.png");
        let decoded = DecodedImage {
            width: 1,
            height: 1,
            pixels: vec![0, 0, 0, 255],
        };
        app.accept_open_outcome(image_path.clone(), Ok(decoded));
        assert_eq!(app.opened_image_path, Some(image_path));
        assert!(app.decoded_image.is_some());
        assert!(app.message_text.is_none());
    }

    #[test]
    fn failure_after_success_keeps_the_previous_image_visible() {
        let mut app = ImagesScrollerApp::empty_state();
        let decoded = DecodedImage {
            width: 2,
            height: 2,
            pixels: vec![0; 16],
        };
        app.accept_open_outcome(PathBuf::from("photo.png"), Ok(decoded));
        app.open_image(PathBuf::from("unregistered-in-container.bin"));
        assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
        assert_eq!(app.opened_image_path, Some(PathBuf::from("photo.png")));
        assert!(app.decoded_image.is_some());
    }
}
