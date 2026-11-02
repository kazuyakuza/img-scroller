//! Per-frame keyboard intake: every edge-triggered chord of the app (the
//! fullscreen toggle, the four navigation arrows, the zoom in/out/restore
//! chords) is read once per frame and dispatched; the intake runs in EVERY
//! mode (fullscreen included) and always BEFORE the layout dispatch.

use eframe::egui;

use super::ImagesScrollerApp;
use super::NavigationDirection;
use super::fullscreen;
use super::zoom;

impl ImagesScrollerApp {
    /// Reads every chord for this frame and dispatches its shared action.
    /// One `key_pressed` read per chord key (no hidden chained collisions).
    pub(super) fn read_and_dispatch_keystrokes(&mut self, screen_context: &egui::Context) {
        let fullscreen_pressed = was_pressed(screen_context, fullscreen::FULLSCREEN_TOGGLE_KEY);
        let zoom_in_pressed = zoom::zoom_in_chord_was_pressed(screen_context);
        let zoom_out_pressed = zoom::zoom_out_chord_was_pressed(screen_context);
        let restore_fit_pressed = was_pressed(screen_context, zoom::RESTORE_FIT_KEY);
        let left_pressed = was_pressed(screen_context, egui::Key::ArrowLeft);
        let right_pressed = was_pressed(screen_context, egui::Key::ArrowRight);
        let up_pressed = was_pressed(screen_context, egui::Key::ArrowUp);
        let down_pressed = was_pressed(screen_context, egui::Key::ArrowDown);
        if fullscreen_pressed {
            self.toggle_fullscreen_and_send_command(screen_context);
        }
        if zoom_in_pressed {
            self.zoom_in_once();
        }
        if zoom_out_pressed {
            self.zoom_out_once();
        }
        if restore_fit_pressed {
            self.restore_zoom_to_fit();
        }
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
    }
}

/// The read entry every plain chord funnels through (module-local helper).
fn was_pressed(screen_context: &egui::Context, pressed_key: egui::Key) -> bool {
    screen_context.input(|input| input.key_pressed(pressed_key))
}
