//! Entry point of the Images Scroller viewer.

mod app;

fn main() -> eframe::Result {
    eframe::run_native(
        app::APP_WINDOW_TITLE,
        app::window_options(),
        Box::new(|creation_context| Ok(Box::new(app::ImagesScrollerApp::new(creation_context)))),
    )
}
