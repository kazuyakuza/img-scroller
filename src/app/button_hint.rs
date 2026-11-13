//! Button-hint plumbing for the floating pill (Wave 4.1 Task 2): the per-button
//! hotkey captions and the hover tooltip strings are keyed here, keyed by
//! button, so no pill button module duplicates them; the shared vertical
//! button-column layout wraps every pill button's paint code.
//! Captions render through the shared pill_style outlined-caption paint (WHITE).

use eframe::egui;

use super::arrow_direction::ArrowControl;
use super::fit_button::FitButtonAction;
use super::pill_style;
use super::zoom_button::ZoomButtonAction;

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
    OpenWith,
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
pub(super) fn hotkey_reference(button: PillButton) -> Option<&'static str> {
    match button {
        PillButton::ImagePrevious
        | PillButton::ImageNext
        | PillButton::FolderUp
        | PillButton::FolderDown
        | PillButton::OpenWith => None,
        PillButton::Fullscreen => Some("F11"),
        PillButton::ZoomIn => Some("I"),
        PillButton::ZoomOut => Some("O"),
        PillButton::RestoreFit => Some("0"),
        PillButton::FitWidth => Some("W"),
        PillButton::FitHeight => Some("H"),
    }
}

/// The hover tooltip of every pill button (its primary hotkey included).
pub(super) fn tooltip_text(button: PillButton) -> &'static str {
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
        PillButton::OpenWith => "Open with...",
    }
}

/// What the shared column helper needs about one pill button: which button
/// (caption + tooltip source), its painted size (the column's width; the
/// size consts stay in the button modules, single-source per module), and
/// its enablement (the caption fill color derives from it).
pub(super) struct PillButtonPlacement {
    button: PillButton,
    button_size: egui::Vec2,
    enabled: bool,
    disabled_tooltip: Option<&'static str>,
}

impl PillButtonPlacement {
    pub(super) fn for_button(button: PillButton, button_size: egui::Vec2) -> Self {
        Self {
            button,
            button_size,
            enabled: true,
            disabled_tooltip: None,
        }
    }

    /// Sets the painted enablement the caption color derives from
    /// (painted-disabled buttons keep the dimmed caption).
    pub(super) fn with_enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }

    /// Overrides the disabled-hover tooltip (the no-programs empty-result
    /// case); `None` keeps the button's standard tooltip in both states.
    pub(super) fn with_disabled_tooltip(self, disabled_tooltip: Option<&'static str>) -> Self {
        Self {
            disabled_tooltip,
            ..self
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
            let painted_tooltip = if placement.enabled {
                tooltip
            } else {
                placement.disabled_tooltip.unwrap_or(tooltip)
            };
            let painted_button = painted_button
                .on_hover_text(painted_tooltip)
                .on_disabled_hover_text(painted_tooltip);
            if let Some(reference) = hotkey_reference(button) {
                let caption_glyph_state =
                    pill_style::glyph_state_for(placement.enabled, painted_button.hovered());
                let caption_fill_color =
                    pill_style::glyph_state_color(caption_glyph_state, column_ui);
                pill_style::paint_outlined_caption(
                    column_ui,
                    pill_style::OutlinedCaption {
                        text: reference,
                        fill_color: caption_fill_color,
                    },
                );
            }
            painted_button
        },
    );
    column_response.inner
}
