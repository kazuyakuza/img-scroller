//! Application shell: the regular, resizable viewer window of Images Scroller.

use std::path::PathBuf;

use eframe::egui;

use crate::app_display::CurrentImageDisplay;
use crate::folder_listing::{FolderListing, ListedNeighbor};
use crate::image_loading::{self, DecodedImage, ImageLoadFailure};
use crate::platform;

/// Title shown on the viewer window title bar (final app name, user-decided 2026-10-29).
pub const APP_WINDOW_TITLE: &str = "Images Scroller";

pub struct ImagesScrollerApp {
    /// Path of the last successfully decoded image (read again by Wave 2's navigation code).
    opened_image_path: Option<PathBuf>,
    /// Sibling-image listing of the opened image's folder in folder-listing order
    /// (rebuilt on every successful open; None until the first open succeeds).
    current_folder_listing: Option<FolderListing>,
    /// Decoded pixels of the last successfully opened image (read by the display surface).
    decoded_image: Option<DecodedImage>,
    message_text: Option<&'static str>,
    /// Texture state of the central image display (spec §4 "small wrapper" clause).
    current_display: CurrentImageDisplay,
}

enum NavigationDirection {
    Previous,
    Next,
}

impl ImagesScrollerApp {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self::empty_state()
    }

    fn empty_state() -> Self {
        Self {
            opened_image_path: None,
            current_folder_listing: None,
            decoded_image: None,
            message_text: None,
            current_display: CurrentImageDisplay::initial(),
        }
    }

    /// Opens an image synchronously: decodes now, then applies the outcome to the open-flow state.
    pub fn open_image(&mut self, path: PathBuf) {
        let outcome = image_loading::load_image(&path);
        self.accept_open_outcome(path, outcome);
    }

    /// Failure keeps any previously loaded image and state untouched except the message.
    fn accept_open_outcome(
        &mut self,
        path: PathBuf,
        outcome: Result<DecodedImage, ImageLoadFailure>,
    ) {
        match outcome {
            Ok(decoded_image) => {
                self.current_folder_listing = FolderListing::from_opened_image(path.as_path());
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

    /// Navigates one sibling image per arrow direction; a failed neighbor open
    /// keeps the current image (Wave-1 open-flow failure rule).
    fn navigate(&mut self, direction: NavigationDirection) {
        if let Some(neighbor_path) = self.neighbor_image_path(direction) {
            self.open_image(neighbor_path);
        }
    }

    /// Resolves the ±1 sibling path from the opened path and the current
    /// listing; every "nothing to navigate to" case (no image, no listing,
    /// unresolvable index, list beginning/end reached) resolves to None.
    fn neighbor_image_path(&self, direction: NavigationDirection) -> Option<PathBuf> {
        let opened_path = self.opened_image_path.as_ref()?;
        let listing = self.current_folder_listing.as_ref()?;
        let listed_index = listing.listed_index_of(opened_path)?;
        let neighbor = match direction {
            NavigationDirection::Previous => listing.previous_neighbor(listed_index),
            NavigationDirection::Next => listing.next_neighbor(listed_index),
        };
        let ListedNeighbor::At { image_path, .. } = neighbor else {
            return None;
        };
        Some(image_path)
    }

    /// True while an image remains successfully opened (the floating controls' empty-state gate).
    fn image_is_open(&self) -> bool {
        self.opened_image_path.is_some()
    }

    /// Image-open layout: the centered floating control pill plus the central image paint.
    fn show_image_layout(&mut self, ui: &mut egui::Ui) {
        floating_controls::show_floating_pill(self, ui);
        egui::CentralPanel::default().show(ui, |central_ui| {
            if let Some(decoded_image) = self.decoded_image.as_ref() {
                self.current_display.paint(central_ui, decoded_image);
            }
        });
    }

    /// Empty-state layout: an empty central paint carrying the ONE center Open button.
    fn show_empty_state_layout(&mut self, open_dialog_frame: &mut eframe::Frame, ui: &mut egui::Ui) {
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
        egui::Area::new(egui::Id::new(EMPTY_STATE_OPEN_AREA_ID))
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(central_ui, |area_ui| {
                let open_button = egui::Button::new(
                    egui::RichText::new(OPEN_LABEL_TEXT).size(OPEN_LABEL_SIZE),
                )
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
        if left_pressed {
            self.navigate(NavigationDirection::Previous);
        }
        if right_pressed {
            self.navigate(NavigationDirection::Next);
        }
        if self.image_is_open() {
            self.show_image_layout(ui);
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

mod floating_controls;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod floating_controls_tests;
