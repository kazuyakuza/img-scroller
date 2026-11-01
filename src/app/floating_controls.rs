//! Floating centered bottom navigation controls: one overlay pill above the
//! central image holding the message line and four painted arrow buttons.
//! Previous/next enablement follows the keyboard seam per frame; up/down are
//! the Wave-3-reserved disabled pair. The caller keeps the image-open gate.

use eframe::egui;

use super::arrow_direction::ArrowControl;
use super::{ImagesScrollerApp, NavigationDirection};

const FLOATING_PILL_AREA_ID: &str = "floating_navigation_pill";
const FLOATING_PILL_BOTTOM_OFFSET: f32 = -24.0;
const FLOATING_PILL_FILL_COLOR: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied_const(20, 20, 20, 180);
const FLOATING_PILL_CORNER_RADIUS: u8 = 10;
const FLOATING_PILL_INNER_MARGIN_POINTS: f32 = 8.0;
const ARROW_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const ARROW_TRIANGLE_SIZE: egui::Vec2 = egui::Vec2::new(10.0, 10.0);
const FOLDER_GROUP_GAP: f32 = 16.0;
const MESSAGE_TEXT_SIZE: f32 = 12.0;

/// A navigation button is enabled exactly when the navigation seam resolves
/// a neighbor: no image, no listing, unresolvable index, and list edges disable.
pub(super) fn navigation_button_enabled(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> bool {
    app.neighbor_image_path(direction).is_some()
}

/// Shows the pill: one centered, anchored overlay Area above the image surface
/// (non-space-reserving; Order::Foreground keeps it painted above CentralPanel).
pub(super) fn show_floating_pill(app: &mut ImagesScrollerApp, host_ui: &mut egui::Ui) {
    egui::Area::new(egui::Id::unique(FLOATING_PILL_AREA_ID))
        .anchor(
            egui::Align2::CENTER_BOTTOM,
            egui::vec2(0.0, FLOATING_PILL_BOTTOM_OFFSET),
        )
        .order(egui::Order::Foreground)
        .show(host_ui.ctx(), |area_ui| {
            show_pill_contents(app, area_ui);
        });
}

fn show_pill_contents(app: &mut ImagesScrollerApp, area_ui: &mut egui::Ui) {
    let message = app.message_text;
    egui::Frame::new()
        .fill(FLOATING_PILL_FILL_COLOR)
        .corner_radius(FLOATING_PILL_CORNER_RADIUS)
        .inner_margin(FLOATING_PILL_INNER_MARGIN_POINTS)
        .show(area_ui, |pill_ui| {
            show_message_line(message, pill_ui);
            show_button_row(app, pill_ui);
        });
}

/// Shows the literal-K3 message: the failure text when Some, empty otherwise.
fn show_message_line(message: Option<&'static str>, pill_ui: &mut egui::Ui) {
    pill_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |line_ui| {
        line_ui.label(egui::RichText::new(message.unwrap_or("")).size(MESSAGE_TEXT_SIZE));
    });
}

fn show_button_row(app: &mut ImagesScrollerApp, pill_ui: &mut egui::Ui) {
    let button_specs = arrow_button_specs(app);
    pill_ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |row_ui| {
        for button_spec in &button_specs {
            add_folder_group_gap(row_ui, button_spec.control);
            button_spec.show_and_navigate(app, row_ui);
        }
    });
}

fn add_folder_group_gap(row_ui: &mut egui::Ui, arrow_control: ArrowControl) {
    if arrow_control == ArrowControl::FolderUp {
        row_ui.add_space(FOLDER_GROUP_GAP);
    }
}

fn arrow_button_specs(app: &ImagesScrollerApp) -> [ArrowButtonSpec; 4] {
    [
        ArrowButtonSpec::in_row(ArrowControl::ImagePrevious, app),
        ArrowButtonSpec::in_row(ArrowControl::ImageNext, app),
        ArrowButtonSpec::in_row(ArrowControl::FolderUp, app),
        ArrowButtonSpec::in_row(ArrowControl::FolderDown, app),
    ]
}

/// Per-frame row-button request: the control it draws plus the enablement
/// derived that frame. Buttons store no state between frames.
struct ArrowButtonSpec {
    control: ArrowControl,
    enabled: bool,
}

fn arrow_button_is_enabled(arrow_control: ArrowControl, app: &ImagesScrollerApp) -> bool {
    let Some(direction) = arrow_control.navigation_direction() else {
        return false;
    };
    navigation_button_enabled(app, direction)
}

impl ArrowButtonSpec {
    fn in_row(arrow_control: ArrowControl, app: &ImagesScrollerApp) -> Self {
        let enabled = arrow_button_is_enabled(arrow_control, app);
        Self {
            control: arrow_control,
            enabled,
        }
    }

    /// Paints the arrow and, when clicked while enabled, drives the keyboard
    /// navigation seam. Disabled arrows paint dimmed and stay inert.
    fn show_and_navigate(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let button_response = self.allocate_and_paint(row_ui);
        if let Some(direction) = self.click_direction(&button_response) {
            app.navigate(direction);
        }
    }

    fn allocate_and_paint(&self, row_ui: &mut egui::Ui) -> egui::Response {
        let (_, button_response) =
            row_ui.allocate_exact_size(ARROW_BUTTON_SIZE, egui::Sense::click());
        button_response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::Role::Button,
                self.enabled,
                self.control.accessible_label(),
            )
        });
        let button_response = button_response.on_hover_cursor(egui::CursorIcon::PointingHand);
        self.paint_triangle(row_ui, &button_response);
        button_response
    }

    fn click_direction(&self, button_response: &egui::Response) -> Option<NavigationDirection> {
        if !self.accepts_click(button_response) {
            return None;
        }
        self.control.navigation_direction()
    }

    /// A disabled arrow never navigates (the Wave-3 reserve stays handler-free).
    fn accepts_click(&self, button_response: &egui::Response) -> bool {
        self.enabled && button_response.clicked()
    }

    fn paint_triangle(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let triangle_rect =
            egui::Rect::from_center_size(button_response.rect.center(), ARROW_TRIANGLE_SIZE);
        let triangle = egui::Shape::rotated_triangle(
            triangle_rect,
            self.control.painted_direction().paint_rotation(),
            self.arrow_color(row_ui, button_response),
        );
        row_ui.painter().add(triangle);
    }

    /// Default egui button-text palette: idle in `inactive`, brightened on
    /// hover, dimmed in `noninteractive` while disabled (exact visual states).
    fn arrow_color(&self, row_ui: &egui::Ui, button_response: &egui::Response) -> egui::Color32 {
        if !self.enabled {
            return row_ui.visuals().widgets.noninteractive.fg_stroke.color;
        }
        if button_response.hovered() {
            return row_ui.visuals().widgets.hovered.fg_stroke.color;
        }
        row_ui.visuals().widgets.inactive.fg_stroke.color
    }
}
