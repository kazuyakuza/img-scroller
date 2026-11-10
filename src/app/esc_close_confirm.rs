//! The fullscreen-OFF ESC double-press close affordance (Wave 4.2 A-Task 4):
//! the centered confirm popup's pinned constants, the pure two-press input
//! filter, the pure arm/idle/close branch decision, the popup state helpers
//! on the app (arm / disarm / per-frame expiry), and the popup render.
//!
//! Behavior contract (2026-11-07 operator chat; spec §1-§6): while fullscreen
//! is OFF, the FIRST distinct ESC press arms a centered popup for
//! `POPUP_VISIBLE_SECONDS`; a SECOND distinct press while it is visible sends
//! the application Close command; expiry re-arms cleanly; held-key autorepeat
//! events never arm and never close. Fullscreen ON consumes ESC entirely (the
//! exit path in `keyboard_intake.rs` has priority) and every fullscreen toggle
//! disarms the popup. Only the intake arms/closes; only timeout, the
//! fullscreen toggle, and the second press remove the popup.

use eframe::egui;

use super::ImagesScrollerApp;

/// The exact popup sentence (spec §1.3: this is the ONLY place the literal
/// may appear; tests and renders read it from here).
pub(super) const POPUP_MESSAGE_TEXT: &str = "Press ESC again to close";

/// Popup lifetime in seconds, then it dismisses itself (operator: 2 seconds).
pub(super) const POPUP_VISIBLE_SECONDS: f64 = 2.0;

/// Distinct Area salt (spec §6.4: no collision with "floating_navigation_pill"
/// or "empty_state_open_button").
const ESC_CLOSE_POPUP_AREA_ID: &str = "esc_close_confirm_popup";
const POPUP_FILL_COLOR: egui::Color32 =
    egui::Color32::from_rgba_unmultiplied_const(20, 20, 20, 220);
const POPUP_CORNER_RADIUS: u8 = 10;
const POPUP_INNER_MARGIN_POINTS: f32 = 16.0;
const POPUP_TEXT_SIZE: f32 = 14.0;
const POPUP_TEXT_COLOR: egui::Color32 = egui::Color32::WHITE;

/// What the ESC branch does with this frame's ESC reading (spec §3.3), after
/// the fullscreen-exit path has already had priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EscCloseAction {
    /// The popup is visible and a distinct ESC press arrived: close the app.
    CloseApplication,
    /// No popup is visible and a distinct ESC press arrived: arm the popup.
    ShowPopup,
    /// Nothing to do this frame.
    StayIdle,
}

/// The popup branch decision (spec §3.3): visible + distinct press = close;
/// not visible + distinct press = arm; anything else stays idle.
pub(super) fn esc_close_action(
    popup_is_visible: bool,
    non_repeat_escape_was_pressed: bool,
) -> EscCloseAction {
    match (popup_is_visible, non_repeat_escape_was_pressed) {
        (true, true) => EscCloseAction::CloseApplication,
        (false, true) => EscCloseAction::ShowPopup,
        _ => EscCloseAction::StayIdle,
    }
}

/// True exactly for a first-of-two DISTINCT ESC presses: a pressed Escape key
/// event that is NOT a held-key autorepeat (spec §4.2; `key_pressed` alone
/// would also fire on repeats).
pub(super) fn non_repeat_escape_press_occurred(events: &[egui::Event]) -> bool {
    events.iter().any(|event| {
        matches!(
            event,
            egui::Event::Key {
                key: egui::Key::Escape,
                pressed: true,
                repeat: false,
                ..
            }
        )
    })
}

impl ImagesScrollerApp {
    /// True while the popup is currently armed/visible.
    pub(super) fn esc_close_popup_is_visible(&self) -> bool {
        self.esc_close_popup_expires_at.is_some()
    }

    /// Arms the popup NOW, reading the context clock (the intake's arm path).
    pub(super) fn arm_esc_close_popup(&mut self, screen_context: &egui::Context) {
        let now = current_time(screen_context);
        self.arm_esc_close_popup_at(now);
    }

    /// Pure arm: deadline = now + POPUP_VISIBLE_SECONDS (spec §2.1).
    pub(super) fn arm_esc_close_popup_at(&mut self, now: f64) {
        self.esc_close_popup_expires_at = Some(now + POPUP_VISIBLE_SECONDS);
    }

    /// Clears the popup without closing the app (every fullscreen toggle calls this).
    pub(super) fn disarm_esc_close_popup(&mut self) {
        self.esc_close_popup_expires_at = None;
    }

    /// Expires the popup if its deadline has passed (the per-frame update).
    pub(super) fn update_esc_close_popup(&mut self, screen_context: &egui::Context) {
        let now = current_time(screen_context);
        self.update_esc_close_popup_at(now);
    }

    /// Pure expiry: `now >= deadline` clears the armed state (spec §2.1).
    pub(super) fn update_esc_close_popup_at(&mut self, now: f64) {
        let Some(deadline) = self.esc_close_popup_expires_at else {
            return;
        };
        if now >= deadline {
            self.esc_close_popup_expires_at = None;
        }
    }
}

fn current_time(screen_context: &egui::Context) -> f64 {
    screen_context.input(|input| input.time)
}

/// The popup render (spec §1.1): a centered, Foreground, NON-CAPTURING Area
/// painted ONLY while armed; the chrome layout calls it in every
/// fullscreen-OFF state. Pure transient: no clicks, no focus, and no
/// dismissal besides timeout / fullscreen toggle / the closing second press.
pub(super) fn show_esc_close_popup(app: &ImagesScrollerApp, screen_context: &egui::Context) {
    if !app.esc_close_popup_is_visible() {
        return;
    }
    egui::Area::new(egui::Id::unique(ESC_CLOSE_POPUP_AREA_ID))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .order(egui::Order::Foreground)
        .interactable(false)
        .show(screen_context, |popup_ui| {
            egui::Frame::new()
                .fill(POPUP_FILL_COLOR)
                .corner_radius(POPUP_CORNER_RADIUS)
                .inner_margin(POPUP_INNER_MARGIN_POINTS)
                .show(popup_ui, |frame_ui| {
                    frame_ui.label(
                        egui::RichText::new(POPUP_MESSAGE_TEXT)
                            .size(POPUP_TEXT_SIZE)
                            .color(POPUP_TEXT_COLOR),
                    );
                });
        });
}
