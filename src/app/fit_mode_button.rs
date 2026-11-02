//! The floating pill's fit-mode buttons (Wave-4 Task 3, user-approved INSIDE
//! the pill, after the zoom group): fit-width [W] and fit-height [H]. Painted
//! line-and-triangle glyphs of the SAME vector family as the arrows -- no font
//! glyphs. Enabled exactly while an image is open; rendered DISABLED while an
//! empty folder is browsed (fresh empty state: the pill is absent anyway).

use eframe::egui;

use super::ImagesScrollerApp;
use super::arrow_direction::ArrowDirection;
use crate::scaling::FitMode;

pub(super) const FIT_WIDTH_BUTTON_LABEL: &str = "Fit width";
pub(super) const FIT_HEIGHT_BUTTON_LABEL: &str = "Fit height";

const FIT_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const FIT_GROUP_GAP: f32 = 16.0;
const FIT_AXIS_LINE_LENGTH: f32 = 12.0;
const FIT_AXIS_LINE_THICKNESS: f32 = 2.5;
const FIT_ARROW_TRIANGLE_SIZE: egui::Vec2 = egui::Vec2::new(6.0, 6.0);
const FIT_ARROW_SPACING: f32 = 7.5;

enum FitModeButtonAction {
    FitWidth,
    FitHeight,
}

struct FitModeButtonSpec {
    action: FitModeButtonAction,
    enabled: bool,
}

/// Appends the fit-mode group to the pill row, separated by the standard
/// group gap after the zoom buttons (user-approved placement).
pub(super) fn show_fit_mode_pill_buttons(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    row_ui.add_space(FIT_GROUP_GAP);
    let button_specs = fit_mode_button_specs(app);
    for button_spec in &button_specs {
        button_spec.show_and_apply_action(app, row_ui);
    }
}

fn fit_mode_button_specs(app: &ImagesScrollerApp) -> [FitModeButtonSpec; 2] {
    let image_displayed = app.image_is_open();
    [
        FitModeButtonSpec::row_button(FitModeButtonAction::FitWidth, image_displayed),
        FitModeButtonSpec::row_button(FitModeButtonAction::FitHeight, image_displayed),
    ]
}

impl FitModeButtonSpec {
    fn row_button(action: FitModeButtonAction, enabled: bool) -> Self {
        Self { action, enabled }
    }

    /// Paints the glyph and, when clicked while enabled, sets the fit mode
    /// (the app seam resets the factor and gives the feedback).
    fn show_and_apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let (_, button_response) =
            row_ui.allocate_exact_size(FIT_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, self.accessible_label())
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_glyph(row_ui, &button_response);
        if self.accepts_click(&button_response) {
            self.apply_action(app);
        }
    }

    /// A disabled fit button never fires (the disabled-arrow pattern).
    fn accepts_click(&self, button_response: &egui::Response) -> bool {
        self.enabled_click_would_fire(button_response)
    }

    fn enabled_click_would_fire(&self, button_response: &egui::Response) -> bool {
        self.enabled && button_response.clicked()
    }

    fn apply_action(&self, app: &mut ImagesScrollerApp) {
        match self.action {
            FitModeButtonAction::FitWidth => app.set_fit_mode(FitMode::FitWidth),
            FitModeButtonAction::FitHeight => app.set_fit_mode(FitMode::FitHeight),
        }
    }

    fn accessible_label(&self) -> &'static str {
        match self.action {
            FitModeButtonAction::FitWidth => FIT_WIDTH_BUTTON_LABEL,
            FitModeButtonAction::FitHeight => FIT_HEIGHT_BUTTON_LABEL,
        }
    }

    fn fit_glyph_color(
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

    fn paint_glyph(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let glyph_center = button_response.rect.center();
        let glyph_color = self.fit_glyph_color(row_ui, button_response);
        for glyph_shape in self.action.glyph_shapes(glyph_center, glyph_color) {
            row_ui.painter().add(glyph_shape);
        }
    }
}

impl FitModeButtonAction {
    /// Per-action glyph geometry: one axis line plus the two opposing
    /// triangles of the painted-arrow family (`Shape::rotated_triangle`).
    fn glyph_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let horizontal = matches!(self, Self::FitWidth);
        let axis_offset = if horizontal {
            egui::vec2(FIT_ARROW_SPACING, 0.0)
        } else {
            egui::vec2(0.0, FIT_ARROW_SPACING)
        };
        let (minus_facing, plus_facing) = if horizontal {
            (ArrowDirection::Left, ArrowDirection::Right)
        } else {
            (ArrowDirection::Up, ArrowDirection::Down)
        };
        vec![
            if horizontal {
                horizontal_axis_line_shape(glyph_center, glyph_color)
            } else {
                vertical_axis_line_shape(glyph_center, glyph_color)
            },
            egui::Shape::rotated_triangle(
                egui::Rect::from_center_size(glyph_center - axis_offset, FIT_ARROW_TRIANGLE_SIZE),
                minus_facing.paint_rotation(),
                glyph_color,
            ),
            egui::Shape::rotated_triangle(
                egui::Rect::from_center_size(glyph_center + axis_offset, FIT_ARROW_TRIANGLE_SIZE),
                plus_facing.paint_rotation(),
                glyph_color,
            ),
        ]
    }
}

fn horizontal_axis_line_shape(glyph_center: egui::Pos2, glyph_color: egui::Color32) -> egui::Shape {
    let line_size = egui::Vec2::new(FIT_AXIS_LINE_LENGTH, FIT_AXIS_LINE_THICKNESS);
    egui::Shape::rect_filled(
        egui::Rect::from_center_size(glyph_center, line_size),
        egui::CornerRadius::ZERO,
        glyph_color,
    )
}

fn vertical_axis_line_shape(glyph_center: egui::Pos2, glyph_color: egui::Color32) -> egui::Shape {
    let line_size = egui::Vec2::new(FIT_AXIS_LINE_THICKNESS, FIT_AXIS_LINE_LENGTH);
    egui::Shape::rect_filled(
        egui::Rect::from_center_size(glyph_center, line_size),
        egui::CornerRadius::ZERO,
        glyph_color,
    )
}
