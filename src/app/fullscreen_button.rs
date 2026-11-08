//! The floating pill's FIFTH button: the fullscreen toggle (user-approved
//! Option A placement). Painted filled-square icon (the painted-shape family
//! of the vector-triangle arrows; font glyphs are avoided). The button is
//! ALWAYS enabled in the normal chrome state -- it is the only viewport-local
//! control, and it is not rendered while fullscreen is ON.
//! Wave-4.2 Task 2: the square carries the shared 1-point black pill outline.

use eframe::egui;

use super::ImagesScrollerApp;
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
use super::pill_style;

const FULLSCREEN_BUTTON_LABEL: &str = "Toggle fullscreen";
const FULLSCREEN_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const FULLSCREEN_SQUARE_SIZE: egui::Vec2 = egui::Vec2::new(10.0, 10.0);

/// Appends the ALWAYS-ENABLED fullscreen toggle button to the pill row.
pub(super) fn show_fullscreen_pill_button(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    let placement = PillButtonPlacement::for_button(PillButton::Fullscreen, FULLSCREEN_BUTTON_SIZE);
    let button_column = ButtonColumnSite::new(row_ui, placement);
    let painted_button = show_button_column(button_column, allocate_and_paint);
    if painted_button.clicked() {
        app.toggle_fullscreen_and_send_command(row_ui.ctx());
    }
}

fn allocate_and_paint(column_ui: &mut egui::Ui) -> egui::Response {
    let (_, button_response) =
        column_ui.allocate_exact_size(FULLSCREEN_BUTTON_SIZE, egui::Sense::click());
    button_response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::Role::Button, true, FULLSCREEN_BUTTON_LABEL)
    });
    let painted_button = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
    paint_fullscreen_square(column_ui, &painted_button);
    painted_button
}

fn paint_fullscreen_square(row_ui: &egui::Ui, button_response: &egui::Response) {
    let square_rect =
        egui::Rect::from_center_size(button_response.rect.center(), FULLSCREEN_SQUARE_SIZE);
    let glyph_state = pill_style::glyph_state_for(true, button_response.hovered());
    let square = pill_style::outlined_rect_shape(
        square_rect,
        pill_style::glyph_state_color(glyph_state, row_ui),
    );
    row_ui.painter().add(square);
}
