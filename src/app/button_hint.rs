//! Button-hint plumbing for the floating pill (Wave 4.1 Task 2): the per-button
//! hotkey captions and the hover tooltip strings are keyed here, keyed by
//! button, so no pill button module duplicates them; the shared vertical
//! button-column layout wraps every pill button's paint code.

use eframe::egui;

use super::arrow_direction::ArrowControl;
use super::fit_button::FitButtonAction;
use super::zoom_button::ZoomButtonAction;

const CAPTION_TEXT_SIZE: f32 = 9.0;
const CAPTION_TEXT_GAP: f32 = 2.0;

#[derive(Clone, Copy)]
pub(super) enum PillButton {
    ImagePrevious,
    ImageNext,
    FolderUp,
    FolderDown,
    Fullscreen,
    ZoomIn,
    ZoomOut,
    RestoreFit,
    FitWidth,
    FitHeight,
}

impl PillButton {
    pub(super) fn from_arrow_control(control: ArrowControl) -> Self {
        match control {
            ArrowControl::ImagePrevious => Self::ImagePrevious,
            ArrowControl::ImageNext => Self::ImageNext,
            ArrowControl::FolderUp => Self::FolderUp,
            ArrowControl::FolderDown => Self::FolderDown,
        }
    }

    pub(super) fn from_zoom_action(action: ZoomButtonAction) -> Self {
        match action {
            ZoomButtonAction::ZoomIn => Self::ZoomIn,
            ZoomButtonAction::ZoomOut => Self::ZoomOut,
            ZoomButtonAction::RestoreFit => Self::RestoreFit,
        }
    }

    pub(super) fn from_fit_action(action: FitButtonAction) -> Self {
        match action {
            FitButtonAction::FitWidth => Self::FitWidth,
            FitButtonAction::FitHeight => Self::FitHeight,
        }
    }
}

/// The hotkey reference caption under the glyph (`None` for the four arrows:
/// each arrow is painted as the arrow key itself).
fn hotkey_reference(button: PillButton) -> Option<&'static str> {
    match button {
        PillButton::ImagePrevious
        | PillButton::ImageNext
        | PillButton::FolderUp
        | PillButton::FolderDown => None,
        PillButton::Fullscreen => Some("F11"),
        PillButton::ZoomIn => Some("I"),
        PillButton::ZoomOut => Some("O"),
        PillButton::RestoreFit => Some("0"),
        PillButton::FitWidth => Some("W"),
        PillButton::FitHeight => Some("H"),
    }
}

/// The hover tooltip of every pill button (its primary hotkey included).
fn tooltip_text(button: PillButton) -> &'static str {
    match button {
        PillButton::ImagePrevious => "Previous image (Left Arrow)",
        PillButton::ImageNext => "Next image (Right Arrow)",
        PillButton::FolderUp => "Previous sibling folder (Up Arrow)",
        PillButton::FolderDown => "Next sibling folder (Down Arrow)",
        PillButton::Fullscreen => "Fullscreen (F11)",
        PillButton::ZoomIn => "Zoom in (I)",
        PillButton::ZoomOut => "Zoom out (O)",
        PillButton::RestoreFit => "Restore fit (0)",
        PillButton::FitWidth => "Fit width (W)",
        PillButton::FitHeight => "Fit height (H)",
    }
}

/// What the shared column helper needs about one pill button: which button
/// (caption + tooltip source) and its painted size (the column's width; the
/// size consts stay in the button modules, single-source per module).
pub(super) struct PillButtonPlacement {
    button: PillButton,
    button_size: egui::Vec2,
}

impl PillButtonPlacement {
    pub(super) fn for_button(button: PillButton, button_size: egui::Vec2) -> Self {
        Self {
            button,
            button_size,
        }
    }
}

/// The pill row Ui being appended to, plus its button's placement (parameter
/// object keeping the shared helper at the two-argument limit).
pub(super) struct ButtonColumnSite<'row_ui> {
    row_ui: &'row_ui mut egui::Ui,
    placement: PillButtonPlacement,
}

impl<'row_ui> ButtonColumnSite<'row_ui> {
    pub(super) fn new(row_ui: &'row_ui mut egui::Ui, placement: PillButtonPlacement) -> Self {
        Self { row_ui, placement }
    }
}

/// Wraps one pill button: a top-aligned vertical column as wide as the button;
/// the paint closure owns the unchanged allocate/widget-info/glyph code; the
/// helper attaches the tooltip pair on the button Response and, when the
/// button has a hotkey reference, the 9 pt caption line under the glyph.
pub(super) fn show_button_column(
    ButtonColumnSite { row_ui, placement }: ButtonColumnSite<'_>,
    paint_button: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    let button = placement.button;
    let column_width = placement.button_size.x;
    let column_response = row_ui.allocate_ui_with_layout(
        egui::Vec2::new(column_width, 0.0),
        egui::Layout::top_down(egui::Align::Center),
        |column_ui| {
            column_ui.spacing_mut().item_spacing.y = CAPTION_TEXT_GAP;
            let painted_button = paint_button(column_ui);
            let tooltip = tooltip_text(button);
            let painted_button = painted_button
                .on_hover_text(tooltip)
                .on_disabled_hover_text(tooltip);
            if let Some(reference) = hotkey_reference(button) {
                column_ui.label(
                    egui::RichText::new(reference)
                        .size(CAPTION_TEXT_SIZE)
                        .color(column_ui.visuals().text_color()),
                );
            }
            painted_button
        },
    );
    column_response.inner
}

#[cfg(test)]
mod tests {
    use super::{PillButton, hotkey_reference, tooltip_text};

    #[test]
    fn hotkey_reference_matches_the_spec() {
        use PillButton::*;
        assert_eq!(hotkey_reference(ImagePrevious), None);
        assert_eq!(hotkey_reference(ImageNext), None);
        assert_eq!(hotkey_reference(FolderUp), None);
        assert_eq!(hotkey_reference(FolderDown), None);
        assert_eq!(hotkey_reference(Fullscreen), Some("F11"));
        assert_eq!(hotkey_reference(ZoomIn), Some("I"));
        assert_eq!(hotkey_reference(ZoomOut), Some("O"));
        assert_eq!(hotkey_reference(RestoreFit), Some("0"));
        assert_eq!(hotkey_reference(FitWidth), Some("W"));
        assert_eq!(hotkey_reference(FitHeight), Some("H"));
    }

    #[test]
    fn tooltip_text_matches_the_spec() {
        use PillButton::*;
        assert_eq!(tooltip_text(ImagePrevious), "Previous image (Left Arrow)");
        assert_eq!(tooltip_text(ImageNext), "Next image (Right Arrow)");
        assert_eq!(tooltip_text(FolderUp), "Previous sibling folder (Up Arrow)");
        assert_eq!(tooltip_text(FolderDown), "Next sibling folder (Down Arrow)");
        assert_eq!(tooltip_text(Fullscreen), "Fullscreen (F11)");
        assert_eq!(tooltip_text(ZoomIn), "Zoom in (I)");
        assert_eq!(tooltip_text(ZoomOut), "Zoom out (O)");
        assert_eq!(tooltip_text(RestoreFit), "Restore fit (0)");
        assert_eq!(tooltip_text(FitWidth), "Fit width (W)");
        assert_eq!(tooltip_text(FitHeight), "Fit height (H)");
    }
}
