//! Entry point of the Images Scroller viewer.

mod app;
mod image_loading;
mod platform;

use std::path::PathBuf;

fn main() -> eframe::Result {
    let cli_image_path = cli_image_path();
    eframe::run_native(
        app::APP_WINDOW_TITLE,
        app::window_options(),
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
