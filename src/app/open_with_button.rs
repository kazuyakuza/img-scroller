//! The floating pill's 11th button (Wave-6 Task 3), rightmost after the fit
//! group: "Open with...", a square-with-emerging-arrow glyph with the `Q`
//! hotkey caption under it (Wave-6.1 Task 2). The button opens an upward
//! MODAL `egui::Popup::menu` listing the programs registered for the current
//! file (all other functionalities OFF while open; `ESC` or `Q` closes --
//! Wave-6.1 Task 3); launch failures render warning-colored (Task 4).
//! `open_with_menu_is_open` is the single modal-state query the intake guard
//! AND the five pill-row button modules' click gates (D2) consult; the Q-open
//! path opens programmatically via `Popup::open_id` (D1).
//! AI Agents: Decisions D1–D12: .kilo/plans/20261113-wave6-1-tasks2-4-*.md;
//! full behavior doc in tech.md (Task 6).

use crate::open_with_listing::OPEN_WITH_LAUNCH_FAILURE_MESSAGE;
use crate::platform;

use eframe::egui;

use super::ImagesScrollerApp;
use super::button_hint::{ButtonColumnSite, PillButton, PillButtonPlacement, show_button_column};
use super::pill_style;

pub(super) const NO_PROGRAMS_TOOLTIP_TEXT: &str = "No programs registered for this file type";

/// The menu popup's stable identity: the ONE id the intake guard, the Q
/// toggle, and the row gates consult (`Id::unique` hashes with FIXED seeds,
/// so every call yields the same id; `Id::new` is deprecated in egui 0.37).
pub(super) fn open_with_menu_id() -> egui::Id {
    egui::Id::unique("open_with_menu_popup")
}

/// The open-with button's keyboard hotkey (Wave-6.1 Task 2; read by the
/// intake's non-repeat filter for BOTH the open and the close toggle).
pub(super) const OPEN_WITH_MENU_KEY: egui::Key = egui::Key::Q;

/// True while the open-with menu popup is open (the ONE modal-state query:
/// the intake guard AND the row click gates consult it).
pub(super) fn open_with_menu_is_open(screen_context: &egui::Context) -> bool {
    egui::Popup::is_id_open(screen_context, open_with_menu_id())
}

const OPEN_WITH_BUTTON_LABEL: &str = "Open with...";
const OPEN_WITH_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(36.0, 32.0);
const OPEN_WITH_GROUP_GAP: f32 = 16.0;
const OPEN_WITH_MENU_MAX_HEIGHT: f32 = 300.0;

/// Appends the open-with button to the pill row after the fit group, with the standard group gap.
pub(super) fn show_open_with_pill_button(app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
    row_ui.add_space(OPEN_WITH_GROUP_GAP);
    let button_spec = OpenWithButtonSpec::for_app(app);
    button_spec.show_and_open_menu(app, row_ui);
}

/// Per-frame button request: the painted enablement. Buttons store no state between frames.
struct OpenWithButtonSpec {
    enabled: bool,
}

impl OpenWithButtonSpec {
    fn for_app(app: &ImagesScrollerApp) -> Self {
        Self {
            enabled: app.open_with_menu_is_available(),
        }
    }

    /// Paints the glyph; the click frame re-enumerates (the menu enumerates WHEN
    /// OPENED) and the enabled button drives the upward menu popup.
    fn show_and_open_menu(&self, app: &mut ImagesScrollerApp, row_ui: &mut egui::Ui) {
        let placement =
            PillButtonPlacement::for_button(PillButton::OpenWith, OPEN_WITH_BUTTON_SIZE)
                .with_enabled(self.enabled)
                .with_disabled_tooltip(no_programs_tooltip_for(app));
        let button_response =
            show_button_column(ButtonColumnSite::new(row_ui, placement), |column_ui| {
                self.allocate_and_paint(column_ui)
            });
        if self.accepts_click(&button_response, row_ui) {
            app.refresh_open_with_programs();
        }
        if self.menu_popup_is_wanted(row_ui) {
            egui::Popup::menu(&button_response)
                .id(open_with_menu_id())
                .align(egui::RectAlign::TOP)
                .accessible_name(OPEN_WITH_BUTTON_LABEL)
                .show(|menu_ui| app.show_open_with_menu_entries(menu_ui));
        }
    }

    /// A disabled open-with button never fires, and a click while the menu is
    /// open only closes it via the popup's own toggle (no refresh re-run).
    fn accepts_click(&self, button_response: &egui::Response, row_ui: &egui::Ui) -> bool {
        self.enabled && button_response.clicked() && !open_with_menu_is_open(row_ui.ctx())
    }

    /// The popup builder's gate: the painted enablement, OR an already-open
    /// popup (once open it renders every frame until closed, even if a fresh
    /// enumeration would flip the enablement — no invisible modal lock).
    fn menu_popup_is_wanted(&self, row_ui: &egui::Ui) -> bool {
        self.enabled || open_with_menu_is_open(row_ui.ctx())
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

    fn paint_glyph(&self, row_ui: &egui::Ui, button_response: &egui::Response) {
        let glyph_center = button_response.rect.center();
        let glyph_state = pill_style::glyph_state_for(self.enabled, button_response.hovered());
        let glyph_color = pill_style::glyph_state_color(glyph_state, row_ui);
        for glyph_shape in pill_style::open_with_glyph_shapes(glyph_center, glyph_color) {
            row_ui.painter().add(glyph_shape);
        }
    }
}

/// The no-programs tooltip applies ONLY while a file is open with an empty
/// enumeration; in empty-folder mode the standard tooltip applies instead.
pub(super) fn no_programs_tooltip_for(app: &ImagesScrollerApp) -> Option<&'static str> {
    if app.image_is_open() && app.open_with_programs.is_empty() {
        return Some(NO_PROGRAMS_TOOLTIP_TEXT);
    }
    None
}

/// True exactly while the message line shows the open-with launch failure —
/// the ONLY warning-colored message (Wave-6.1 Task 4): the failure text is a
/// unique constant, so equality with it is the lossless discriminator; the
/// unsupported-file message and cleared states keep the default color.
pub(super) fn message_line_is_warning(message: Option<&'static str>) -> bool {
    message == Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)
}

impl ImagesScrollerApp {
    /// True exactly when a distinct Q press should OPEN the menu: the press, the
    /// button's enablement (image open AND programs listed), and the pill chrome
    /// visible (fullscreen ON hides the button; a disabled state stays inert).
    pub(super) fn open_with_menu_should_open_on_q(&self, q_pressed: bool) -> bool {
        q_pressed && self.open_with_menu_is_available() && self.full_ui_chrome_is_visible()
    }

    /// The refresh contract's Q-open point: re-enumerate for the CURRENT file,
    /// then open the popup programmatically (the click path opens via the
    /// Popup::menu builder's own click toggle — popup.rs:237-240, 556-577).
    pub(super) fn open_with_menu_from_q_press(&mut self, screen_context: &egui::Context) {
        self.refresh_open_with_programs();
        egui::Popup::open_id(screen_context, open_with_menu_id());
    }

    /// Re-enumerates the programs registered for the CURRENT canonical file into
    /// the stored list (the refresh contract: every successful open, the
    /// popup-opening click, and the Q-open press).
    pub(super) fn refresh_open_with_programs(&mut self) {
        let Some(opened_path) = self.opened_image_path.clone() else {
            return;
        };
        self.open_with_programs = platform::enumerate_open_with_programs(&opened_path);
    }

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
            let entry_response =
                scroll_ui.button(self.open_with_programs[program_index].display_name.as_str());
            if entry_response.clicked() {
                self.launch_open_with_program(program_index);
            }
        }
    }
}
