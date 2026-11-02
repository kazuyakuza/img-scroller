//! Application shell: the regular, resizable viewer window of Images Scroller.

use std::path::PathBuf;

use eframe::egui;

use crate::app_display::CurrentImageDisplay;
use crate::folder_listing::FolderListing;
use crate::image_loading::{self, DecodedImage, ImageLoadFailure};
use crate::last_viewed_images::LastViewedImages;
use crate::platform;
use crate::sibling_folder_listing::SiblingFolderListing;

/// Title shown on the viewer window title bar (final app name, user-decided 2026-10-29).
pub const APP_WINDOW_TITLE: &str = "Images Scroller";

pub struct ImagesScrollerApp {
    /// Path of the last successfully opened image (anchor of both navigation-seam axes).
    opened_image_path: Option<PathBuf>,
    /// Sibling-image listing of the opened image's folder in folder-listing order
    /// (rebuilt on every successful open; released when empty-folder mode is entered).
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
                self.opened_image_path = Some(path);
                self.decoded_image = Some(decoded_image);
                self.message_text = None;
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
    fn image_is_open(&self) -> bool {
        self.opened_image_path.is_some()
    }

    /// Pill layout: the floating pill plus the central surface -- the image paint when one is open, else the empty-folder message.
    fn show_navigation_layout(&mut self, ui: &mut egui::Ui) {
        floating_controls::show_floating_pill(self, ui);
        egui::CentralPanel::default().show(ui, |central_ui| {
            if let Some(decoded_image) = self.decoded_image.as_ref() {
                self.current_display.paint(central_ui, decoded_image);
            } else {
                empty_folder_view::show_empty_folder_message(central_ui);
            }
        });
    }

    /// Empty-state layout: an empty central paint carrying the ONE center Open button (the fresh empty state: no image AND no folder context).
    fn show_empty_state_layout(
        &mut self,
        open_dialog_frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) {
        egui::CentralPanel::default().show(ui, |central_ui| {
            self.show_centered_open_button(open_dialog_frame, central_ui);
        });
    }

    /// The only Open control: anchored to the window center in the empty state;
    /// the click reuses the native open dialog seam (the retired toolbar's path).
    fn show_centered_open_button(
        &mut self,
        open_dialog_frame: &mut eframe::Frame,
        central_ui: &mut egui::Ui,
    ) {
        egui::Area::new(egui::Id::unique(EMPTY_STATE_OPEN_AREA_ID))
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(central_ui.ctx(), |area_ui| {
                let open_button =
                    egui::Button::new(egui::RichText::new(OPEN_LABEL_TEXT).size(OPEN_LABEL_SIZE))
                        .min_size(OPEN_BUTTON_SIZE);
                if area_ui.add(open_button).clicked() {
                    self.open_via_dialog(open_dialog_frame);
                }
            });
    }
}

impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let left_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowLeft));
        let right_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowRight));
        let up_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowUp));
        let down_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowDown));
        if left_pressed {
            self.navigate(NavigationDirection::Previous);
        }
        if right_pressed {
            self.navigate(NavigationDirection::Next);
        }
        if up_pressed {
            self.navigate(NavigationDirection::PreviousSiblingFolder);
        }
        if down_pressed {
            self.navigate(NavigationDirection::NextSiblingFolder);
        }
        if self.navigation_context_is_active() {
            self.show_navigation_layout(ui);
        } else {
            self.show_empty_state_layout(frame, ui);
        }
    }
}

const EMPTY_STATE_OPEN_AREA_ID: &str = "empty_state_open_button";
const OPEN_LABEL_TEXT: &str = "Open…";
const OPEN_LABEL_SIZE: f32 = 18.0;
const OPEN_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(180.0, 48.0);

pub fn window_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_WINDOW_TITLE)
            .with_resizable(true)
            .with_maximized(false),
        ..eframe::NativeOptions::default()
    }
}

mod arrow_direction;

mod folder_navigation;

mod empty_folder_view;

mod floating_controls;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod floating_controls_tests;
#[cfg(test)]
mod folder_navigation_tests;
#[cfg(test)]
mod last_viewed_tests;

#[cfg(test)]
mod folder_navigation_continuation_tests;
