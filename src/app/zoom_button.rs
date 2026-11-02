//! The floating pill's zoom buttons (Wave-4 Task 2, user-approved placement
//! INSIDE the pill, after the fullscreen square): zoom-in [+], zoom-out [-],
//! and restore-zoom-to-fit -- painted rectangle-shape glyphs, no font glyphs.
//! Enablement follows the at-bound scheme (D3); the restore-fit marker is a
//! stroked (unfilled) square to stay distinct from the fullscreen filled one.

use eframe::egui;

use super::ImagesScrollerApp;

const ZOOM_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const ZOOM_GROUP_GAP: f32 = 16.0;
const GLYPH_BAR_LENGTH: f32 = 12.0;
const GLYPH_BAR_THICKNESS: f32 = 2.5;
const FIT_MARKER_SIDE: f32 = 11.0;
const FIT_MARKER_STROKE_WIDTH: f32 = 2.0;

pub(super) const ZOOM_IN_BUTTON_LABEL: &str = "Zoom in";
pub(super) const ZOOM_OUT_BUTTON_LABEL: &str = "Zoom out";
pub(super) const RESTORE_FIT_BUTTON_LABEL: &str = "Restore zoom to fit";

enum ZoomButtonAction {
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
        ZoomButtonSpec::row_button(ZoomButtonAction::ZoomOut, app.zoomed_beyond_fit()),
        ZoomButtonSpec::row_button(ZoomButtonAction::RestoreFit, app.zoomed_beyond_fit()),
    ]
}

impl ZoomButtonSpec {
    fn row_button(action: ZoomButtonAction, enabled: bool) -> Self {
        Self { action, enabled }
    }

    /// Paints the glyph and, when clicked while enabled, applies its action.
    fn show_and_apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let (_, button_response) =
            row_ui.allocate_exact_size(ZOOM_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, self.accessible_label())
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_glyph(row_ui, &button_response);
        if self.accepts_click(&button_response) {
            self.apply_action(app);
        }
    }

    /// A disabled zoom button never fires (the disabled-arrow pattern).
    fn accepts_click(&self, button_response: &egui::Response) -> bool {
        self.enabled && button_response.clicked()
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

    fn zoom_glyph_color(
        &self,
        row_ui: &egui::Ui,
        button_response: &egui::Response,
    ) -> egui::Color32 {
        if !self.enabled {
            return row_ui.visuals().widgets.noninteractive.fg_stroke.color;
        }
        if button_response.hovered() {
            return row_ui.visuals().widgets.hovered.fg_stroke.color;
        }
        row_ui.visuals().widgets.inactive.fg_stroke.color
    }

    /// Paints the button's glyph shapes at the disabled/idle/hovered palette.
    fn paint_glyph(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let glyph_center = button_response.rect.center();
        let glyph_color = self.zoom_glyph_color(row_ui, button_response);
        for glyph_shape in self.zoom_glyph_shapes(glyph_center, glyph_color) {
            row_ui.painter().add(glyph_shape);
        }
    }

    /// Per-action glyph geometry (zoom icons are rects/outline only, D8).
    fn zoom_glyph_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        match self.action {
            ZoomButtonAction::ZoomIn => self.crossed_bar_shapes(glyph_center, glyph_color),
            ZoomButtonAction::ZoomOut => self.single_bar_shapes(glyph_center, glyph_color),
            ZoomButtonAction::RestoreFit => self.stroked_square_shapes(glyph_center, glyph_color),
        }
    }

    /// Zoom-in [+]: a horizontal bar crossed by a vertical one, both filled.
    fn crossed_bar_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let horizontal = egui::Shape::rect_filled(
            egui::Rect::from_center_size(
                glyph_center,
                egui::Vec2::new(GLYPH_BAR_LENGTH, GLYPH_BAR_THICKNESS),
            ),
            egui::CornerRadius::ZERO,
            glyph_color,
        );
        let vertical = egui::Shape::rect_filled(
            egui::Rect::from_center_size(
                glyph_center,
                egui::Vec2::new(GLYPH_BAR_THICKNESS, GLYPH_BAR_LENGTH),
            ),
            egui::CornerRadius::ZERO,
            glyph_color,
        );
        vec![horizontal, vertical]
    }

    /// Zoom-out [−]: one horizontal filled bar.
    fn single_bar_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let bar = egui::Shape::rect_filled(
            egui::Rect::from_center_size(
                glyph_center,
                egui::Vec2::new(GLYPH_BAR_LENGTH, GLYPH_BAR_THICKNESS),
            ),
            egui::CornerRadius::ZERO,
            glyph_color,
        );
        vec![bar]
    }

    /// Restore-fit icon: a stroked square outline (fit marker; distinct from
    /// the fullscreen button's FILLED square). `rect_stroke` REQUIRES an
    /// explicit `StrokeKind` (mandatory since egui 0.31) -- keep that argument.
    fn stroked_square_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let marker_rect = egui::Rect::from_center_size(
            glyph_center,
            egui::Vec2::new(FIT_MARKER_SIDE, FIT_MARKER_SIDE),
        );
        let marker = egui::Shape::rect_stroke(
            marker_rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(FIT_MARKER_STROKE_WIDTH, glyph_color),
            egui::StrokeKind::Inside,
        );
        vec![marker]
    }
}
