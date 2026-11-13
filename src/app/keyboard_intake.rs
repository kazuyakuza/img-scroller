//! Per-frame keyboard intake: every edge-triggered chord of the app (the
//! fullscreen F11 toggle and the ESC-while-ON exit, the ESC-close arm/close
//! branch and the per-frame popup-expiry update, the four pan-or-navigate
//! arrows, the zoom `I`/`O`/`0` chords, and the
//! one-shot fit `W`/`H` keys) is read once per frame and dispatched; the
//! intake runs in EVERY mode (fullscreen included) and always
//! BEFORE the layout dispatch.
//!
//! AI Agents: this block was moved verbatim out of `src/app.rs` (held at the
//! 200-line cap) -- do NOT re-inline it; keep all reads collected before the
//! dispatch chain.

use eframe::egui;

use super::ImagesScrollerApp;
use super::NavigationDirection;
use super::esc_close_confirm;
use super::fullscreen;
use super::open_with_button;
use super::pan;
use super::zoom;

impl ImagesScrollerApp {
    /// Reads every chord for this frame and dispatches its shared action.
    /// One `key_pressed` read per chord key (no hidden chained collisions).
    pub(super) fn read_and_dispatch_keystrokes(&mut self, screen_context: &egui::Context) {
        self.update_esc_close_popup(screen_context);
        if self.handle_modal_open_with_menu_keys(screen_context) {
            return;
        }
        let q_pressed = screen_context.input(|input| {
            esc_close_confirm::non_repeat_key_press_occurred(
                &input.events,
                open_with_button::OPEN_WITH_MENU_KEY,
            )
        });
        let fullscreen_pressed = was_pressed(screen_context, fullscreen::FULLSCREEN_TOGGLE_KEY);
        let escape_exit_fullscreen_pressed = fullscreen::escape_exits_fullscreen(
            self.fullscreen_on,
            was_pressed(screen_context, fullscreen::FULLSCREEN_EXIT_KEY),
        );
        let non_repeat_escape_pressed = screen_context
            .input(|input| esc_close_confirm::non_repeat_escape_press_occurred(&input.events));
        let zoom_in_pressed = zoom::zoom_in_chord_was_pressed(screen_context);
        let zoom_out_pressed = zoom::zoom_out_chord_was_pressed(screen_context);
        let restore_fit_pressed = was_pressed(screen_context, zoom::RESTORE_FIT_KEY);
        let fit_width_pressed = was_pressed(screen_context, zoom::FIT_WIDTH_KEY);
        let fit_height_pressed = was_pressed(screen_context, zoom::FIT_HEIGHT_KEY);
        let left_pressed = was_pressed(screen_context, egui::Key::ArrowLeft);
        let right_pressed = was_pressed(screen_context, egui::Key::ArrowRight);
        let up_pressed = was_pressed(screen_context, egui::Key::ArrowUp);
        let down_pressed = was_pressed(screen_context, egui::Key::ArrowDown);
        let left_non_repeat = screen_context.input(|input| {
            pan::non_repeat_arrow_press_occurred(&input.events, egui::Key::ArrowLeft)
        });
        let right_non_repeat = screen_context.input(|input| {
            pan::non_repeat_arrow_press_occurred(&input.events, egui::Key::ArrowRight)
        });
        let up_non_repeat = screen_context
            .input(|input| pan::non_repeat_arrow_press_occurred(&input.events, egui::Key::ArrowUp));
        let down_non_repeat = screen_context.input(|input| {
            pan::non_repeat_arrow_press_occurred(&input.events, egui::Key::ArrowDown)
        });
        if self.open_with_menu_should_open_on_q(q_pressed) {
            self.open_with_menu_from_q_press(screen_context);
        } else if fullscreen_pressed {
            self.toggle_fullscreen_and_send_command(screen_context);
        }
        if escape_exit_fullscreen_pressed {
            self.toggle_fullscreen_and_send_command(screen_context);
        } else {
            self.dispatch_esc_close_action(screen_context, non_repeat_escape_pressed);
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
            self.set_zoom_to_fit_width(screen_context.viewport_rect().size());
        }
        if fit_height_pressed {
            self.set_zoom_to_fit_height(screen_context.viewport_rect().size());
        }
        if left_pressed {
            self.handle_arrow_key(
                screen_context.viewport_rect().size(),
                pan::ArrowKeyPress {
                    direction: NavigationDirection::Previous,
                    non_repeat: left_non_repeat,
                },
            );
        }
        if right_pressed {
            self.handle_arrow_key(
                screen_context.viewport_rect().size(),
                pan::ArrowKeyPress {
                    direction: NavigationDirection::Next,
                    non_repeat: right_non_repeat,
                },
            );
        }
        if up_pressed {
            self.handle_arrow_key(
                screen_context.viewport_rect().size(),
                pan::ArrowKeyPress {
                    direction: NavigationDirection::PreviousSiblingFolder,
                    non_repeat: up_non_repeat,
                },
            );
        }
        if down_pressed {
            self.handle_arrow_key(
                screen_context.viewport_rect().size(),
                pan::ArrowKeyPress {
                    direction: NavigationDirection::NextSiblingFolder,
                    non_repeat: down_non_repeat,
                },
            );
        }
    }

    /// The modal gate (Wave-6.1 Task 3): while the open-with menu is open ONLY
    /// the Q toggle and the ESC close run this frame — every other app action is
    /// skipped, the ESC-close contract cannot arm, and the fullscreen exit never
    /// fires. Returns true when the menu was open (the caller skips the normal
    /// intake; the reads never consume events, so the menu's own keyboard
    /// navigation keeps working).
    fn handle_modal_open_with_menu_keys(&mut self, screen_context: &egui::Context) -> bool {
        if !open_with_button::open_with_menu_is_open(screen_context) {
            return false;
        }
        let q_pressed = screen_context.input(|input| {
            esc_close_confirm::non_repeat_key_press_occurred(
                &input.events,
                open_with_button::OPEN_WITH_MENU_KEY,
            )
        });
        let non_repeat_escape_pressed = screen_context
            .input(|input| esc_close_confirm::non_repeat_escape_press_occurred(&input.events));
        if q_pressed {
            egui::Popup::close_id(screen_context, open_with_button::open_with_menu_id());
        }
        if non_repeat_escape_pressed {
            egui::Popup::close_id(screen_context, open_with_button::open_with_menu_id());
        }
        true
    }

    /// The ESC-close branch: runs ONLY when the fullscreen-exit ESC path did
    /// not consume this frame's ESC (fullscreen OFF, or no ESC press).
    fn dispatch_esc_close_action(
        &mut self,
        screen_context: &egui::Context,
        non_repeat_escape_was_pressed: bool,
    ) {
        match esc_close_confirm::esc_close_action(
            self.esc_close_popup_is_visible(),
            non_repeat_escape_was_pressed,
        ) {
            esc_close_confirm::EscCloseAction::CloseApplication => {
                screen_context.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            esc_close_confirm::EscCloseAction::ShowPopup => {
                self.arm_esc_close_popup(screen_context);
            }
            esc_close_confirm::EscCloseAction::StayIdle => {}
        }
    }
}

/// The read entry every plain chord funnels through (module-local helper).
fn was_pressed(screen_context: &egui::Context, pressed_key: egui::Key) -> bool {
    screen_context.input(|input| input.key_pressed(pressed_key))
}
