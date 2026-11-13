//! Application shell of the Images Scroller viewer (the resizable, always-maximized viewer-window options moved to `src/main.rs` in Wave 5).

use std::path::PathBuf;

use eframe::egui;

use crate::app_display::CurrentImageDisplay;
use crate::folder_listing::FolderListing;
use crate::image_loading::{self, DecodedImage, ImageLoadFailure};
use crate::last_viewed_images::LastViewedImages;
use crate::open_with_listing::RegisteredProgram;
use crate::platform;
use crate::sibling_folder_listing::SiblingFolderListing;

/// Title shown on the viewer window title bar (final app name, user-decided 2026-10-29).
pub const APP_WINDOW_TITLE: &str = "Images Scroller";

pub struct ImagesScrollerApp {
    /// Path of the last successfully opened image (anchor of both navigation-seam axes).
    opened_image_path: Option<PathBuf>,
    /// Sibling-image listing of the opened image's folder (rebuilt on every successful open).
    current_folder_listing: Option<FolderListing>,
    /// Session memory: the last displayed image of every folder visited in-session.
    last_viewed_images: LastViewedImages,
    current_sibling_folders: Option<SiblingFolderListing>,
    browsed_folder_context: Option<PathBuf>,
    /// Decoded pixels of the last successfully opened image (read by the display surface).
    decoded_image: Option<DecodedImage>,
    message_text: Option<&'static str>,
    /// Texture state of the central image display (spec §4 "small wrapper" clause).
    current_display: CurrentImageDisplay,
    /// Session fullscreen flag: ON hides the native chrome and ALL UI controls; never persisted.
    fullscreen_on: bool,
    /// Session zoom factor (1.0 = whole-image fit); reset on every successful open; never persisted.
    zoom_factor: f32,
    /// ESC-close popup expiry deadline (Wave 4.2): `None` = hidden, else the absolute dismiss time.
    esc_close_popup_expires_at: Option<f64>,
    /// Pan offset from the centered position (Wave 5); reset on open and restore-to-fit.
    pan_offset: egui::Vec2,
    /// Open-with display list for the current file (Wave-6 Task 3); refreshed on every
    /// successful open and on the popup-opening click.
    open_with_programs: Vec<RegisteredProgram>,
}

use folder_navigation::NavigationDirection;

impl ImagesScrollerApp {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self::empty_state()
    }

    fn empty_state() -> Self {
        Self {
            opened_image_path: None,
            current_folder_listing: None,
            last_viewed_images: LastViewedImages::empty(),
            current_sibling_folders: None,
            browsed_folder_context: None,
            decoded_image: None,
            message_text: None,
            current_display: CurrentImageDisplay::initial(),
            fullscreen_on: false,
            zoom_factor: crate::zoom::RESTORE_TO_FIT_FACTOR,
            esc_close_popup_expires_at: None,
            pan_offset: egui::Vec2::ZERO,
            open_with_programs: Vec::new(),
        }
    }

    /// Opens an image synchronously: decodes now, then applies the outcome to the open-flow state.
    pub fn open_image(&mut self, path: PathBuf) {
        let outcome = image_loading::load_image(&path);
        let opened_path = std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        self.accept_open_outcome(opened_path, outcome);
    }

    /// Success applies the open transition (browsed folder cleared); failure updates only the message.
    fn accept_open_outcome(
        &mut self,
        path: PathBuf,
        outcome: Result<DecodedImage, ImageLoadFailure>,
    ) {
        match outcome {
            Ok(decoded_image) => {
                self.current_folder_listing = FolderListing::from_opened_image(path.as_path());
                self.last_viewed_images.record_displayed_image(&path);
                self.current_sibling_folders =
                    SiblingFolderListing::from_opened_image(path.as_path());
                self.browsed_folder_context = None;
                self.open_with_programs = platform::enumerate_open_with_programs(&path);
                self.opened_image_path = Some(path);
                self.decoded_image = Some(decoded_image);
                self.message_text = None;
                self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;
                self.pan_offset = egui::Vec2::ZERO;
                self.current_display.accept_decoded_image();
            }
            Err(_failure) => {
                self.message_text = Some(image_loading::UNSUPPORTED_FILE_MESSAGE);
            }
        }
    }

    fn open_via_dialog(&mut self, frame: &eframe::Frame) {
        if let Some(image_path) = platform::show_open_image_dialog(frame) {
            self.open_image(image_path);
        }
    }

    /// True while an image remains successfully opened (read by the navigation seam's pill-visibility gate).
    pub(super) fn image_is_open(&self) -> bool {
        self.opened_image_path.is_some()
    }

    /// Pill layout: the floating pill plus the central surface -- the image paint when one is open, else the empty-folder message.
    fn show_navigation_layout(&mut self, ui: &mut egui::Ui) {
        floating_controls::show_floating_pill(self, ui);
        egui::CentralPanel::default().show(ui, |central_ui| {
            self.show_navigation_central_surface(central_ui);
        });
    }

    /// Frame layout dispatch: fullscreen ON runs ONLY the central surface; OFF
    /// runs the standard chrome layouts (pill + central surface, or the centered
    /// Open button in the fresh empty state).
    fn show_chrome_or_fullscreen_layout(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        if self.full_ui_chrome_is_visible() {
            self.show_chrome_layout(frame, ui);
        } else {
            self.show_fullscreen_layout(ui);
        }
    }

    /// The 0.3.0 chrome-layout dispatch, unchanged (navigation-context gate preserved).
    fn show_chrome_layout(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        if self.navigation_context_is_active() {
            self.show_navigation_layout(ui);
        } else {
            self.show_empty_state_layout(frame, ui);
        }
        esc_close_confirm::show_esc_close_popup(self, ui.ctx());
    }
}

impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.read_and_dispatch_keystrokes(ui.ctx());
        self.show_chrome_or_fullscreen_layout(frame, ui);
    }
}

mod arrow_direction;
mod button_hint;
mod central_surface;
mod empty_folder_view;
mod empty_state_view;
mod esc_close_confirm;
mod fit_button;
mod floating_controls;
mod folder_navigation;
mod fullscreen;
mod fullscreen_button;
mod keyboard_intake;
mod open_with_button;
mod pan;
mod pill_style;
mod zoom;
mod zoom_alias;
mod zoom_button;

#[cfg(test)]
mod button_hint_tests;
#[cfg(test)]
mod esc_close_confirm_tests;
#[cfg(test)]
mod esc_close_expiry_tests;
#[cfg(test)]
mod fit_one_shot_tests;
#[cfg(test)]
mod floating_controls_tests;
#[cfg(test)]
mod folder_navigation_continuation_tests;
#[cfg(test)]
mod folder_navigation_tests;
#[cfg(test)]
mod fullscreen_tests;
#[cfg(test)]
mod last_viewed_tests;
#[cfg(test)]
mod open_with_button_tests;
#[cfg(test)]
mod pan_tests;
#[cfg(test)]
mod pill_style_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod zoom_alias_tests;
#[cfg(test)]
mod zoom_tests;
