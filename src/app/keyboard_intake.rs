//! Per-frame keyboard intake: every edge-triggered chord of the app (the
//! fullscreen F11 toggle and the ESC-while-ON exit, the four navigation arrows,
//! chords, and the fit-mode `W`/`H` keys) is read once per frame and
//! dispatched; the intake runs in EVERY mode (fullscreen included) and always
//! BEFORE the layout dispatch.
//!
//! AI Agents: this block was moved verbatim out of `src/app.rs` (held at the
//! 200-line cap) -- do NOT re-inline it; keep all reads collected before the
//! dispatch chain.

use eframe::egui;

use super::ImagesScrollerApp;
use super::NavigationDirection;
use super::fullscreen;
use super::zoom;

impl ImagesScrollerApp {
    /// Reads every chord for this frame and dispatches its shared action.
    /// One `key_pressed` read per chord key (no hidden chained collisions).
    /// Dispatch order is significant: the fit activations follow the zoom
    /// chords, so a same-frame fit message overwrites a zoom-bound message.
    pub(super) fn read_and_dispatch_keystrokes(&mut self, screen_context: &egui::Context) {
        let fullscreen_pressed = was_pressed(screen_context, fullscreen::FULLSCREEN_TOGGLE_KEY);
        let escape_exit_pressed = fullscreen::escape_exits_fullscreen(
            self.fullscreen_on,
            was_pressed(screen_context, fullscreen::FULLSCREEN_EXIT_KEY),
        );
        let zoom_in_pressed = zoom::zoom_in_chord_was_pressed(screen_context);
        let zoom_out_pressed = zoom::zoom_out_chord_was_pressed(screen_context);
        let restore_fit_pressed = was_pressed(screen_context, zoom::RESTORE_FIT_KEY);
        let fit_width_pressed = was_pressed(screen_context, zoom::FIT_WIDTH_KEY);
        let fit_height_pressed = was_pressed(screen_context, zoom::FIT_HEIGHT_KEY);
        let left_pressed = was_pressed(screen_context, egui::Key::ArrowLeft);
        let right_pressed = was_pressed(screen_context, egui::Key::ArrowRight);
        let up_pressed = was_pressed(screen_context, egui::Key::ArrowUp);
        let down_pressed = was_pressed(screen_context, egui::Key::ArrowDown);
        if fullscreen_pressed {
            self.toggle_fullscreen_and_send_command(screen_context);
        }
        if escape_exit_pressed {
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
        if fit_width_pressed {
            self.set_fit_mode(crate::scaling::FitMode::FitWidth);
        }
        if fit_height_pressed {
            self.set_fit_mode(crate::scaling::FitMode::FitHeight);
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
