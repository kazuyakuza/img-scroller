//! The floating pill's one-shot fit buttons (Wave 4.1 Task 7): fit-width [W] and fit-height [H]. This file is the renamed Wave-4 Task 3 `fit_mode_button` pill: the button labels, painted glyphs, sizes and gaps carried over verbatim; only the mode state was removed. Painted line-and-triangle glyphs of the SAME vector family as the arrows -- no font glyphs. Clicking set the zoom factor ONLY (one-shot, silent). Enabled exactly while an image is open; rendered DISABLED while an empty folder is browsed (fresh empty state: the pill is absent anyway).
//! Wave-4.2 Task 2: every glyph carries the shared 1-point black pill outline.

use eframe::egui;

use super::ImagesScrollerApp;
use super::arrow_direction::ArrowDirection;
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
use super::pill_style;

pub(super) const FIT_WIDTH_BUTTON_LABEL: &str = "Fit width";
pub(super) const FIT_HEIGHT_BUTTON_LABEL: &str = "Fit height";

const FIT_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const FIT_GROUP_GAP: f32 = 16.0;
const FIT_AXIS_LINE_LENGTH: f32 = 12.0;
const FIT_AXIS_LINE_THICKNESS: f32 = 2.5;
const FIT_ARROW_TRIANGLE_SIZE: egui::Vec2 = egui::Vec2::new(6.0, 6.0);
const FIT_ARROW_SPACING: f32 = 7.5;

#[derive(Clone, Copy)]
pub(super) enum FitButtonAction {
    FitWidth,
    FitHeight,
}

struct FitButtonSpec {
    action: FitButtonAction,
    enabled: bool,
}

/// Appends the one-shot fit button group to the pill row, separated by the
/// standard group gap after the zoom buttons (user-approved placement).
pub(super) fn show_fit_pill_buttons(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    row_ui.add_space(FIT_GROUP_GAP);
    let button_specs = fit_button_specs(app);
    for button_spec in &button_specs {
        button_spec.show_and_apply_action(app, row_ui);
    }
}

fn fit_button_specs(app: &ImagesScrollerApp) -> [FitButtonSpec; 2] {
    let image_displayed = app.image_is_open();
    [
        FitButtonSpec::row_button(FitButtonAction::FitWidth, image_displayed),
        FitButtonSpec::row_button(FitButtonAction::FitHeight, image_displayed),
    ]
}

impl FitButtonSpec {
    fn row_button(action: FitButtonAction, enabled: bool) -> Self {
        Self { action, enabled }
    }

    /// Paints the glyph and, when clicked while enabled, applies the one-shot
    /// fit action (sets the zoom factor only; no message, no persistent mode).
    fn show_and_apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let placement = PillButtonPlacement::for_button(
            PillButton::from_fit_action(self.action),
            FIT_BUTTON_SIZE,
        )
        .with_enabled(self.enabled);
        let button_column = ButtonColumnSite::new(row_ui, placement);
        let button_response = show_button_column(button_column, |column_ui| {
            self.allocate_and_paint(column_ui)
        });
        if self.accepts_click(&button_response) {
            self.apply_action(app, row_ui);
        }
    }

    fn allocate_and_paint(&self, column_ui: &mut egui::Ui) -> egui::Response {
        let (_, button_response) =
            column_ui.allocate_exact_size(FIT_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, self.accessible_label())
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_glyph(column_ui, &button_response);
        button_response
    }

    /// A disabled fit button never fires (the disabled-arrow pattern).
    fn accepts_click(&self, button_response: &egui::Response) -> bool {
        self.enabled_click_would_fire(button_response)
    }

    fn enabled_click_would_fire(&self, button_response: &egui::Response) -> bool {
        self.enabled && button_response.clicked()
    }

    fn apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &egui::Ui) {
        let available_size = row_ui.ctx().viewport_rect().size();
        match self.action {
            FitButtonAction::FitWidth => app.set_zoom_to_fit_width(available_size),
            FitButtonAction::FitHeight => app.set_zoom_to_fit_height(available_size),
        }
    }

    fn accessible_label(&self) -> &'static str {
        match self.action {
            FitButtonAction::FitWidth => FIT_WIDTH_BUTTON_LABEL,
            FitButtonAction::FitHeight => FIT_HEIGHT_BUTTON_LABEL,
        }
    }

    fn paint_glyph(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let glyph_center = button_response.rect.center();
        let glyph_state = pill_style::glyph_state_for(self.enabled, button_response.hovered());
        let glyph_color = pill_style::glyph_state_color(glyph_state, row_ui);
        for glyph_shape in self.action.glyph_shapes(glyph_center, glyph_color) {
            row_ui.painter().add(glyph_shape);
        }
    }
}

impl FitButtonAction {
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
            pill_style::outlined_triangle_shape(pill_style::TriangularGlyphSpec {
                rect: egui::Rect::from_center_size(
                    glyph_center - axis_offset,
                    FIT_ARROW_TRIANGLE_SIZE,
                ),
                rotation: minus_facing.paint_rotation(),
                fill_color: glyph_color,
            }),
            pill_style::outlined_triangle_shape(pill_style::TriangularGlyphSpec {
                rect: egui::Rect::from_center_size(
                    glyph_center + axis_offset,
                    FIT_ARROW_TRIANGLE_SIZE,
                ),
                rotation: plus_facing.paint_rotation(),
                fill_color: glyph_color,
            }),
        ]
    }
}

fn horizontal_axis_line_shape(glyph_center: egui::Pos2, glyph_color: egui::Color32) -> egui::Shape {
    let line_rect = egui::Rect::from_center_size(
        glyph_center,
        egui::Vec2::new(FIT_AXIS_LINE_LENGTH, FIT_AXIS_LINE_THICKNESS),
    );
    pill_style::outlined_rect_shape(line_rect, glyph_color)
}

fn vertical_axis_line_shape(glyph_center: egui::Pos2, glyph_color: egui::Color32) -> egui::Shape {
    let line_rect = egui::Rect::from_center_size(
        glyph_center,
        egui::Vec2::new(FIT_AXIS_LINE_THICKNESS, FIT_AXIS_LINE_LENGTH),
    );
    pill_style::outlined_rect_shape(line_rect, glyph_color)
}
