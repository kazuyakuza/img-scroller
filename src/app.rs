//! Application shell: the regular, resizable viewer window of Images Scroller.

use std::path::PathBuf;

use eframe::egui;

/// Title shown on the viewer window title bar (final app name, user-decided 2026-10-29).
pub const APP_WINDOW_TITLE: &str = "Images Scroller";

pub struct ImagesScrollerApp {
    // Placeholder for the Task 3 open-image pipeline (frontend-spec 2.6); Task 2 reads nothing.
    #[allow(dead_code)]
    opened_image_path: Option<PathBuf>,
}

impl ImagesScrollerApp {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            opened_image_path: None,
        }
    }
}

impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
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
    use super::APP_WINDOW_TITLE;

    #[test]
    fn window_title_is_the_final_app_name() {
        assert_eq!(APP_WINDOW_TITLE, "Images Scroller");
    }
}
