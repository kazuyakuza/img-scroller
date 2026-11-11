//! Entry point of the Images Scroller viewer.

// Release runs windowed — no console.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod app_display;
mod folder_listing;
mod image_loading;
mod last_viewed_images;
mod name_collation;
mod pan;
mod platform;
mod scaling;
mod sibling_folder_listing;
mod zoom;

use std::path::PathBuf;

use eframe::egui;

fn window_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(app::APP_WINDOW_TITLE)
            .with_resizable(true)
            .with_maximized(true),
        ..eframe::NativeOptions::default()
    }
}

fn main() -> eframe::Result {
    let cli_image_path = cli_image_path();
    eframe::run_native(
        app::APP_WINDOW_TITLE,
        window_options(),
        Box::new(move |creation_context| {
            let mut application = app::ImagesScrollerApp::new(creation_context);
            if let Some(image_path) = cli_image_path {
                application.open_image(image_path);
            }
            Ok(Box::new(application))
        }),
    )
}

fn cli_image_path() -> Option<PathBuf> {
    std::env::args_os().nth(1).map(PathBuf::from)
}
