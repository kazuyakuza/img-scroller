//! On-screen corner navigation rails: the right rail holds the Wave-3-reserved
//! folder arrows (disabled in Wave 2); the bottom rail holds the image arrows
//! and the open-failure message area (it replaced the Wave-1 message bar).

use eframe::egui;

use super::{ImagesScrollerApp, NavigationDirection};

const RAIL_SIZE: f32 = 40.0;
const ARROW_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(28.0, 24.0);
const ARROW_BUTTON_SPACING: f32 = 8.0;

fn arrow_button(glyph: &str) -> egui::Button<'_> {
    egui::Button::new(glyph).min_size(ARROW_BUTTON_SIZE)
}

/// A direction button is active exactly when the navigation seam resolves a
/// neighbor: no image, no listing, unresolvable index, and list edges disable.
pub(super) fn navigation_button_enabled(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> bool {
    app.neighbor_image_path(direction).is_some()
}

/// Shows the right rail with the up/down folder arrows, disabled in Wave 2.
pub(super) fn show_folder_rail(parent_ui: &mut egui::Ui) {
    egui::Panel::right("folder_rail")
        .default_size(RAIL_SIZE)
        .show(parent_ui, |rail_ui| {
            folder_rail_buttons(rail_ui);
        });
}

fn folder_rail_buttons(rail_ui: &mut egui::Ui) {
    rail_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |rail_ui| {
        rail_ui.add_enabled(false, arrow_button("↑"));
        rail_ui.add_space(ARROW_BUTTON_SPACING);
        rail_ui.add_enabled(false, arrow_button("↓"));
    });
}

/// Shows the bottom rail with the previous/next image arrows and the message area.
pub(super) fn show_image_rail(app: &mut ImagesScrollerApp, parent_ui: &mut egui::Ui) {
    egui::Panel::bottom("image_rail")
        .default_size(RAIL_SIZE)
        .show(parent_ui, |rail_ui| {
            image_rail_controls(app, rail_ui);
        });
}

fn image_rail_controls(app: &mut ImagesScrollerApp, rail_ui: &mut egui::Ui) {
    let previous_enabled = navigation_button_enabled(app, NavigationDirection::Previous);
    let next_enabled = navigation_button_enabled(app, NavigationDirection::Next);
    let message = app.message_text;
    rail_ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Center),
        |rail_ui| {
            if rail_ui
                .add_enabled(previous_enabled, arrow_button("←"))
                .clicked()
            {
                app.navigate(NavigationDirection::Previous);
            }
            if rail_ui
                .add_enabled(next_enabled, arrow_button("→"))
                .clicked()
            {
                app.navigate(NavigationDirection::Next);
            }
            rail_ui.label(message.unwrap_or(""));
        },
    );
}
