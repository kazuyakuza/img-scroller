//! Window-title + fullscreen-name-popup wiring (Wave-8 Task 4): reads the
//! viewport's maximized state and outer width once per frame, recomposes the
//! title through the pure `crate::window_title` module with an egui-font
//! measure closure, sends `ViewportCommand::Title` ONLY when the computed
//! string changed, and arms/expires the 3-second top-center fullscreen name
//! popup (the `esc_close_confirm` wake precedent).

use std::path::PathBuf;
use std::time::Duration;

use eframe::egui;

use super::ImagesScrollerApp;
use crate::window_title::{
    self, FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS, FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS,
    TITLE_BAR_USABLE_WIDTH_FRACTION, TitleComposition,
};

const FULLSCREEN_NAME_POPUP_AREA_ID: &str = "fullscreen_image_name_popup";
const FULLSCREEN_NAME_POPUP_TOP_OFFSET: egui::Vec2 = egui::Vec2::new(0.0, 16.0);
const FULLSCREEN_NAME_POPUP_FILL_COLOR: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied_const(20, 20, 20, 220);
const FULLSCREEN_NAME_POPUP_CORNER_RADIUS: u8 = 10;
const FULLSCREEN_NAME_POPUP_INNER_MARGIN_POINTS: f32 = 16.0;
const FULLSCREEN_NAME_POPUP_TEXT_SIZE: f32 = 14.0;
const FULLSCREEN_NAME_POPUP_TEXT_COLOR: egui::Color32 = egui::Color32::WHITE;

/// The five Wave-8 Task-4 session fields (spec §2.1), grouped for the app.rs
/// cap (D14). RAM-only, never persisted.
pub(super) struct WindowTitleState {
    last_title_image_path: Option<PathBuf>,
    last_maximized_state: Option<bool>,
    last_sent_window_title: String,
    fullscreen_name_popup_expires_at: Option<f64>,
    fullscreen_name_popup_text: String,
}

impl WindowTitleState {
    pub(super) fn initial() -> Self {
        Self {
            last_title_image_path: None,
            last_maximized_state: None,
            last_sent_window_title: String::new(),
            fullscreen_name_popup_expires_at: None,
            fullscreen_name_popup_text: String::new(),
        }
    }
}

impl ImagesScrollerApp {
    pub(super) fn update_window_title_and_popups(&mut self, screen_context: &egui::Context) {
        let maximized_flag = screen_context.input(|input_state| input_state.viewport().maximized);
        let outer_width = screen_context
            .input(|input_state| input_state.viewport().outer_rect.map(|rect| rect.width()));
        let is_maximized = maximized_flag.unwrap_or(false);
        let now = screen_context.input(|input_state| input_state.time);
        self.expire_fullscreen_name_popup_if_due(now);
        self.request_fullscreen_name_popup_wake(screen_context, now);
        let path_changed = self.window_title_state.last_title_image_path.as_ref()
            != self.opened_image_path.as_ref();
        let maximized_changed = self.window_title_state.last_maximized_state != Some(is_maximized);
        if title_recomposition_is_due(path_changed, maximized_changed) {
            let available_width = TITLE_BAR_USABLE_WIDTH_FRACTION
                * outer_width.unwrap_or_else(|| screen_context.viewport_rect().width());
            let title = window_title::compose_window_title(TitleComposition {
                path: self.opened_image_path.as_deref(),
                available_width,
                is_maximized,
                measure: &mut |text| measure_text_width(screen_context, text),
            });
            self.apply_window_title(screen_context, title);
            self.window_title_state.last_title_image_path = self.opened_image_path.clone();
            self.window_title_state.last_maximized_state = Some(is_maximized);
        }
        if self.fullscreen_name_popup_rearm_is_due(path_changed) {
            self.arm_fullscreen_name_popup(screen_context, now);
        }
    }

    pub(super) fn disarm_fullscreen_name_popup(&mut self) {
        self.window_title_state.fullscreen_name_popup_expires_at = None;
    }

    pub(super) fn fullscreen_name_popup_is_visible(&self) -> bool {
        self.window_title_state
            .fullscreen_name_popup_expires_at
            .is_some()
    }

    fn apply_window_title(&mut self, screen_context: &egui::Context, title: String) {
        if title == self.window_title_state.last_sent_window_title {
            return;
        }
        screen_context.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
        self.window_title_state.last_sent_window_title = title;
    }

    fn fullscreen_name_popup_rearm_is_due(&self, path_changed: bool) -> bool {
        path_changed && self.fullscreen_on
    }

    fn arm_fullscreen_name_popup(&mut self, screen_context: &egui::Context, now: f64) {
        let Some(path) = self.opened_image_path.as_ref() else {
            return;
        };
        self.window_title_state.fullscreen_name_popup_text =
            window_title::file_name_with_extension(path);
        self.window_title_state.fullscreen_name_popup_expires_at =
            Some(now + FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS);
        screen_context.request_repaint_after(Duration::from_secs_f64(
            FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS
                + FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS,
        ));
    }

    fn expire_fullscreen_name_popup_if_due(&mut self, now: f64) {
        let Some(deadline) = self.window_title_state.fullscreen_name_popup_expires_at else {
            return;
        };
        if now >= deadline {
            self.window_title_state.fullscreen_name_popup_expires_at = None;
        }
    }

    fn request_fullscreen_name_popup_wake(&self, screen_context: &egui::Context, now: f64) {
        let Some(deadline) = self.window_title_state.fullscreen_name_popup_expires_at else {
            return;
        };
        let remaining_seconds = deadline - now;
        if remaining_seconds <= 0.0 {
            return;
        }
        screen_context.request_repaint_after(Duration::from_secs_f64(
            remaining_seconds + FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS,
        ));
    }
}

fn title_recomposition_is_due(path_changed: bool, maximized_changed: bool) -> bool {
    path_changed || maximized_changed
}

fn measure_text_width(screen_context: &egui::Context, text: &str) -> f32 {
    screen_context
        .fonts_mut(|fonts| {
            fonts.layout_no_wrap(
                text.to_owned(),
                egui::FontId::default(),
                egui::Color32::WHITE,
            )
        })
        .size()
        .x
}

pub(super) fn show_fullscreen_name_popup(app: &ImagesScrollerApp, screen_context: &egui::Context) {
    if !app.fullscreen_name_popup_is_visible() {
        return;
    }
    egui::Area::new(egui::Id::unique(FULLSCREEN_NAME_POPUP_AREA_ID))
        .anchor(egui::Align2::CENTER_TOP, FULLSCREEN_NAME_POPUP_TOP_OFFSET)
        .order(egui::Order::Foreground)
        .interactable(false)
        .show(screen_context, |popup_ui| {
            egui::Frame::new()
                .fill(FULLSCREEN_NAME_POPUP_FILL_COLOR)
                .corner_radius(FULLSCREEN_NAME_POPUP_CORNER_RADIUS)
                .inner_margin(FULLSCREEN_NAME_POPUP_INNER_MARGIN_POINTS)
                .show(popup_ui, |frame_ui| {
                    frame_ui.label(
                        egui::RichText::new(&app.window_title_state.fullscreen_name_popup_text)
                            .size(FULLSCREEN_NAME_POPUP_TEXT_SIZE)
                            .color(FULLSCREEN_NAME_POPUP_TEXT_COLOR),
                    );
                });
        });
}
