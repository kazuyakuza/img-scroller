//! The floating pill's FIFTH button: the fullscreen toggle (user-approved
//! Option A placement). Painted filled-square icon (the painted-shape family
//! of the vector-triangle arrows; font glyphs are avoided). The button is
//! ALWAYS enabled in the normal chrome state -- it is the only viewport-local
//! control, and it is not rendered while fullscreen is ON.

use eframe::egui;

use super::ImagesScrollerApp;

const FULLSCREEN_BUTTON_LABEL: &str = "Toggle fullscreen";
const FULLSCREEN_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const FULLSCREEN_SQUARE_SIZE: egui::Vec2 = egui::Vec2::new(10.0, 10.0);

/// Appends the ALWAYS-ENABLED fullscreen toggle button to the pill row.
pub(super) fn show_fullscreen_pill_button(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    let (_, button_response) =
        row_ui.allocate_exact_size(FULLSCREEN_BUTTON_SIZE, egui::Sense::click());
    button_response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::Role::Button, true, FULLSCREEN_BUTTON_LABEL)
    });
    let painted_button = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
    paint_fullscreen_square(row_ui, &painted_button);
    if painted_button.clicked() {
        app.toggle_fullscreen_and_send_command(row_ui.ctx());
    }
}

fn paint_fullscreen_square(row_ui: &egui::Ui, button_response: &egui::Response) {
    let square_rect =
        egui::Rect::from_center_size(button_response.rect.center(), FULLSCREEN_SQUARE_SIZE);
    let square = egui::Shape::rect_filled(
        square_rect,
        egui::CornerRadius::ZERO,
        fullscreen_square_color(row_ui, button_response),
    );
    row_ui.painter().add(square);
}

/// Default egui button-text palette: inactive idle, brightened on hover
/// (the enabled-arrow palette minus the disabled branch -- always enabled).
fn fullscreen_square_color(row_ui: &egui::Ui, button_response: &egui::Response) -> egui::Color32 {
    if button_response.hovered() {
        return row_ui.visuals().widgets.hovered.fg_stroke.color;
    }
    row_ui.visuals().widgets.inactive.fg_stroke.color
}
