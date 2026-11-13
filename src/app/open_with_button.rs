//! The floating pill's 11th button (Wave-6 Task 3, user-approved Option A):
//! "Open with...", painted as a square-with-emerging-arrow glyph. The enabled
//! button opens an upward `egui::Popup::menu` listing the programs the system
//! registered for the current file's type (display names verbatim in a
//! 300-point scroll area). A click hands the CURRENT canonical file to the
//! launcher keyed by `launch_index`; a failure surfaces
//! `OPEN_WITH_LAUNCH_FAILURE_MESSAGE`. The display list refreshes on every
//! successful open and on the popup-opening click (no per-frame OS
//! enumeration); empty list = disabled no-programs tooltip; no hotkey in v1.

use crate::open_with_listing::OPEN_WITH_LAUNCH_FAILURE_MESSAGE;
use crate::platform;

use eframe::egui;

use super::ImagesScrollerApp;
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
use super::pill_style;

pub(super) const NO_PROGRAMS_TOOLTIP_TEXT: &str = "No programs registered for this file type";
const OPEN_WITH_BUTTON_LABEL: &str = "Open with...";
const OPEN_WITH_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const OPEN_WITH_GROUP_GAP: f32 = 16.0;
const OPEN_WITH_MENU_MAX_HEIGHT: f32 = 300.0;
const OPEN_WITH_SQUARE_SIDE: f32 = 10.0;
const OPEN_WITH_SQUARE_CENTER_OFFSET: egui::Vec2 = egui::Vec2::new(-4.0, 4.0);
const OPEN_WITH_ARROW_LENGTH: f32 = 7.0;
const OPEN_WITH_ARROW_TRIANGLE_SIZE: egui::Vec2 = egui::Vec2::new(6.0, 6.0);
const OPEN_WITH_ARROW_SHAFT_THICKNESS: f32 = 2.5;
const OPEN_WITH_ARROW_SHAFT_OUTLINE_WIDTH: f32 =
    OPEN_WITH_ARROW_SHAFT_THICKNESS + 2.0 * pill_style::OUTLINE_STROKE_WIDTH;
const OPEN_WITH_ARROW_HEAD_ROTATION: f32 = -std::f32::consts::TAU / 8.0;

/// Appends the open-with button to the pill row after the fit group, with the standard group gap.
pub(super) fn show_open_with_pill_button(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    row_ui.add_space(OPEN_WITH_GROUP_GAP);
    let button_spec = OpenWithButtonSpec::for_app(app);
    button_spec.show_and_open_menu(app, row_ui);
}

/// Per-frame button request: painted enablement plus the disabled-tooltip override.
struct OpenWithButtonSpec {
    enabled: bool,
    disabled_tooltip: Option<&'static str>,
}

impl OpenWithButtonSpec {
    fn for_app(app: &ImagesScrollerApp) -> Self {
        Self {
            enabled: app.open_with_menu_is_available(),
            disabled_tooltip: no_programs_tooltip_for(app),
        }
    }

    /// Paints the glyph; the click frame re-enumerates (the menu enumerates WHEN
    /// OPENED) and the enabled button drives the upward menu popup.
    fn show_and_open_menu(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let placement =
            PillButtonPlacement::for_button(PillButton::OpenWith, OPEN_WITH_BUTTON_SIZE)
                .with_enabled(self.enabled)
                .with_disabled_tooltip(self.disabled_tooltip);
        let button_response =
            show_button_column(ButtonColumnSite::new(row_ui, placement), |column_ui| {
                self.allocate_and_paint(column_ui)
            });
        self.enumerate_when_menu_opens(app, &button_response);
        if self.enabled {
            egui::Popup::menu(&button_response)
                .align(egui::RectAlign::TOP)
                .accessible_name(OPEN_WITH_BUTTON_LABEL)
                .show(|menu_ui| app.show_open_with_menu_entries(menu_ui));
        }
    }

    fn allocate_and_paint(&self, column_ui: &mut egui::Ui) -> egui::Response {
        let (_, button_response) =
            column_ui.allocate_exact_size(OPEN_WITH_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::Role::Button, self.enabled, OPEN_WITH_BUTTON_LABEL)
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_glyph(column_ui, &button_response);
        button_response
    }

    /// A disabled open-with button never fires (the disabled-arrow pattern).
    fn accepts_click(&self, button_response: &egui::Response) -> bool {
        self.enabled && button_response.clicked()
    }

    /// The decided refresh contract: on the click frame that opens the popup,
    /// re-enumerate via the facade (no per-frame enumeration while it is open).
    fn enumerate_when_menu_opens(
        &self,
        app: &mut ImagesScrollerApp,
        button_response: &egui::Response,
    ) {
        if !self.accepts_click(button_response) {
            return;
        }
        let Some(opened_path) = app.opened_image_path.clone() else {
            return;
        };
        app.open_with_programs = platform::enumerate_open_with_programs(&opened_path);
    }

    fn paint_glyph(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let glyph_center = button_response.rect.center();
        let glyph_state = pill_style::glyph_state_for(self.enabled, button_response.hovered());
        let glyph_color = pill_style::glyph_state_color(glyph_state, row_ui);
        for glyph_shape in open_with_glyph_shapes(glyph_center, glyph_color) {
            row_ui.painter().add(glyph_shape);
        }
    }
}

/// A 10x10 square with a 7-point arrow from its top-right corner pointing
/// up-right (shaft plus triangle head); paint order square, shaft (BLACK
/// under-stroke then glyph stroke), head. The square shifts down-left.
fn open_with_glyph_shapes(
    glyph_center: egui::Pos2,
    glyph_color: egui::Color32,
) -> Vec<egui::Shape> {
    let square_center = glyph_center + OPEN_WITH_SQUARE_CENTER_OFFSET;
    let square_rect =
        egui::Rect::from_center_size(square_center, egui::Vec2::splat(OPEN_WITH_SQUARE_SIDE));
    let shaft_start = square_rect.right_top();
    let shaft_rise = OPEN_WITH_ARROW_LENGTH * std::f32::consts::FRAC_1_SQRT_2;
    let shaft_end = shaft_start + egui::vec2(shaft_rise, -shaft_rise);
    vec![
        pill_style::outlined_rect_shape(square_rect, glyph_color),
        egui::Shape::line_segment(
            [shaft_start, shaft_end],
            egui::Stroke::new(
                OPEN_WITH_ARROW_SHAFT_OUTLINE_WIDTH,
                pill_style::OUTLINE_COLOR,
            ),
        ),
        egui::Shape::line_segment(
            [shaft_start, shaft_end],
            egui::Stroke::new(OPEN_WITH_ARROW_SHAFT_THICKNESS, glyph_color),
        ),
        pill_style::outlined_triangle_shape(pill_style::TriangularGlyphSpec {
            rect: egui::Rect::from_center_size(shaft_end, OPEN_WITH_ARROW_TRIANGLE_SIZE),
            rotation: OPEN_WITH_ARROW_HEAD_ROTATION,
            fill_color: glyph_color,
        }),
    ]
}

/// The no-programs tooltip applies ONLY while a file is open with an empty
/// enumeration; in empty-folder mode the standard tooltip applies instead.
pub(super) fn no_programs_tooltip_for(app: &ImagesScrollerApp) -> Option<&'static str> {
    if app.image_is_open() && app.open_with_programs.is_empty() {
        return Some(NO_PROGRAMS_TOOLTIP_TEXT);
    }
    None
}

impl ImagesScrollerApp {
    /// Available exactly while an image is open AND the stored enumeration found
    /// registered programs (the approved disabled-button empty-result behavior).
    pub(super) fn open_with_menu_is_available(&self) -> bool {
        self.image_is_open() && !self.open_with_programs.is_empty()
    }

    /// Hands the CURRENT canonical file to the selected program (the launcher
    /// re-enumerates fresh and indexes `program_launch_index`); a failure surfaces
    /// the static failure message, success changes nothing.
    pub(super) fn launch_open_with_program(&mut self, program_launch_index: usize) {
        let Some(opened_path) = self.opened_image_path.clone() else {
            return;
        };
        if platform::open_file_with_selected_program(&opened_path, program_launch_index).is_err() {
            self.message_text = Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE);
        }
    }

    /// The popup's contents: the stored program list in a vertically scrolling menu.
    pub(super) fn show_open_with_menu_entries(&mut self, menu_ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .max_height(OPEN_WITH_MENU_MAX_HEIGHT)
            .show(menu_ui, |scroll_ui| self.show_menu_entry_buttons(scroll_ui));
    }

    fn show_menu_entry_buttons(&mut self, scroll_ui: &mut egui::Ui) {
        for program_index in 0..self.open_with_programs.len() {
            let entry_response = {
                let display_name = self.open_with_programs[program_index].display_name.as_str();
                scroll_ui.button(display_name)
            };
            if entry_response.clicked() {
                self.launch_open_with_program(program_index);
            }
        }
    }
}
