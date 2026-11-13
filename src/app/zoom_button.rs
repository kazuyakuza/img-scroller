//! The floating pill's zoom buttons (Wave-4 Task 2, user-approved placement
//! INSIDE the pill, after the fullscreen square): zoom-in [+], zoom-out [-],
//! and restore-zoom-to-fit -- painted rectangle-shape glyphs, no font glyphs.
//! Enablement follows the at-bound scheme (D3); the restore-fit marker is a
//! stroked (unfilled) square to stay distinct from the fullscreen filled one.
//! Wave-4.2 Task 2: every glyph carries the shared 1-point black pill outline.

use eframe::egui;

use super::ImagesScrollerApp;
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
use super::open_with_button;
use super::pill_style;

const ZOOM_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const ZOOM_GROUP_GAP: f32 = 16.0;
const GLYPH_BAR_LENGTH: f32 = 12.0;
const GLYPH_BAR_THICKNESS: f32 = 2.5;
const FIT_MARKER_SIDE: f32 = 11.0;
const FIT_MARKER_STROKE_WIDTH: f32 = 2.0;

pub(super) const ZOOM_IN_BUTTON_LABEL: &str = "Zoom in";
pub(super) const ZOOM_OUT_BUTTON_LABEL: &str = "Zoom out";
pub(super) const RESTORE_FIT_BUTTON_LABEL: &str = "Restore zoom to fit";

#[derive(Clone, Copy)]
pub(super) enum ZoomButtonAction {
    ZoomIn,
    ZoomOut,
    RestoreFit,
}

struct ZoomButtonSpec {
    action: ZoomButtonAction,
    enabled: bool,
}

/// Appends the zoom button group to the pill row, separated by the standard
/// group gap after the fullscreen square (user-approved placement).
pub(super) fn show_zoom_pill_buttons(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    row_ui.add_space(ZOOM_GROUP_GAP);
    let button_specs = zoom_button_specs(app);
    for button_spec in &button_specs {
        button_spec.show_and_apply_action(app, row_ui);
    }
}

fn zoom_button_specs(app: &ImagesScrollerApp) -> [ZoomButtonSpec; 3] {
    [
        ZoomButtonSpec::row_button(ZoomButtonAction::ZoomIn, app.zoom_can_grow()),
        ZoomButtonSpec::row_button(ZoomButtonAction::ZoomOut, app.zoom_can_shrink()),
        ZoomButtonSpec::row_button(ZoomButtonAction::RestoreFit, app.zoomed_off_fit()),
    ]
}

impl ZoomButtonSpec {
    fn row_button(action: ZoomButtonAction, enabled: bool) -> Self {
        Self { action, enabled }
    }

    /// Paints the glyph and, when clicked while enabled, applies its action.
    fn show_and_apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let placement = PillButtonPlacement::for_button(
            PillButton::from_zoom_action(self.action),
            ZOOM_BUTTON_SIZE,
        )
        .with_enabled(self.enabled);
        let button_column = ButtonColumnSite::new(row_ui, placement);
        let button_response = show_button_column(button_column, |column_ui| {
            self.allocate_and_paint(column_ui)
        });
        if self.accepts_click(&button_response, row_ui) {
            self.apply_action(app);
        }
    }

    fn allocate_and_paint(&self, column_ui: &mut egui::Ui) -> egui::Response {
        let (_, button_response) =
            column_ui.allocate_exact_size(ZOOM_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, self.accessible_label())
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_glyph(column_ui, &button_response);
        button_response
    }

    /// A disabled zoom button never fires (the disabled-arrow pattern); the
    /// open-with menu also suppresses the row dispatch (Wave-6.1 Task 3).
    fn accepts_click(&self, button_response: &egui::Response, row_ui: &egui::Ui) -> bool {
        self.enabled
            && button_response.clicked()
            && !open_with_button::open_with_menu_is_open(row_ui.ctx())
    }

    fn apply_action(&self, app: &mut ImagesScrollerApp) {
        match self.action {
            ZoomButtonAction::ZoomIn => app.zoom_in_once(),
            ZoomButtonAction::ZoomOut => app.zoom_out_once(),
            ZoomButtonAction::RestoreFit => app.restore_zoom_to_fit(),
        }
    }

    fn accessible_label(&self) -> &'static str {
        match self.action {
            ZoomButtonAction::ZoomIn => ZOOM_IN_BUTTON_LABEL,
            ZoomButtonAction::ZoomOut => ZOOM_OUT_BUTTON_LABEL,
            ZoomButtonAction::RestoreFit => RESTORE_FIT_BUTTON_LABEL,
        }
    }

    /// Paints the button's per-action glyph (zoom icons are rects/outline
    /// only, D8) at the disabled/idle/hovered palette.
    fn paint_glyph(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let glyph_center = button_response.rect.center();
        let glyph_state = pill_style::glyph_state_for(self.enabled, button_response.hovered());
        let glyph_color = pill_style::glyph_state_color(glyph_state, row_ui);
        let glyph_shapes = match self.action {
            ZoomButtonAction::ZoomIn => self.crossed_bar_shapes(glyph_center, glyph_color),
            ZoomButtonAction::ZoomOut => self.single_bar_shapes(glyph_center, glyph_color),
            ZoomButtonAction::RestoreFit => self.stroked_square_shapes(glyph_center, glyph_color),
        };
        for glyph_shape in glyph_shapes {
            row_ui.painter().add(glyph_shape);
        }
    }

    /// Zoom-in [+]: a horizontal bar crossed by a vertical one, both filled.
    fn crossed_bar_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let horizontal_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(GLYPH_BAR_LENGTH, GLYPH_BAR_THICKNESS),
        );
        let vertical_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(GLYPH_BAR_THICKNESS, GLYPH_BAR_LENGTH),
        );
        vec![
            pill_style::outlined_rect_shape(horizontal_rect, glyph_color),
            pill_style::outlined_rect_shape(vertical_rect, glyph_color),
        ]
    }

    /// Zoom-out [−]: one horizontal filled bar.
    fn single_bar_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let bar_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(GLYPH_BAR_LENGTH, GLYPH_BAR_THICKNESS),
        );
        vec![pill_style::outlined_rect_shape(bar_rect, glyph_color)]
    }

    /// Restore-fit icon: a stroked square outline (fit marker; distinct from
    /// the fullscreen button's FILLED square). `rect_stroke` REQUIRES an
    /// explicit `StrokeKind` (mandatory since egui 0.31) -- keep that argument.
    /// Wave-4.2: the marker squares gain a black outline painted OUTSIDE of
    /// the marker rect; the glyph-colored marker stroke itself is unchanged.
    fn stroked_square_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let marker_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(FIT_MARKER_SIDE, FIT_MARKER_SIDE),
        );
        let outline_rect = marker_rect.expand(pill_style::OUTLINE_STROKE_WIDTH);
        let outline = egui::Shape::rect_stroke(
            outline_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(pill_style::OUTLINE_STROKE_WIDTH, pill_style::OUTLINE_COLOR),
            egui::StrokeKind::Outside,
        );
        let marker = egui::Shape::rect_stroke(
            marker_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(FIT_MARKER_STROKE_WIDTH, glyph_color),
            egui::StrokeKind::Inside,
        );
        vec![outline, marker]
    }
}
