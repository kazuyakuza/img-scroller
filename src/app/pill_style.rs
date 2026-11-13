//! Shared pill-styling primitives (Wave 4.2 Task 2, extended Wave 6.1 Task 4):
//! the enabled glyph and caption color (WHITE) with the theme-dimmed disabled
//! palette, the warning-color message-line paint for open-with failures, the
//! 1-point black outline constants, the outlined shape builders, and the
//! outlined hotkey-caption paint (the offset-copy outline technique).

use eframe::egui;

/// The pill's icon and caption color for every ENABLED state (idle AND
/// hovered — WHITE cannot brighten further; the hover cue stays the pointing
/// cursor and the tooltip). Matches the painted icons' guardrail color.
pub(super) const PILL_GLYPH_COLOR: egui::Color32 = egui::Color32::WHITE;
/// The pill's outline color: BLACK, so white glyphs and captions keep a
/// legible edge over any background.
pub(super) const OUTLINE_COLOR: egui::Color32 = egui::Color32::BLACK;
/// The pill's thin outline stroke width in points (the operator's
/// "thin border" constraint: every painted glyph and caption).
pub(super) const OUTLINE_STROKE_WIDTH: f32 = 1.0;
/// The hotkey caption text size in points (moved from `button_hint`; the
/// value is unchanged from Wave 4.1 Task 2).
pub(super) const CAPTION_TEXT_SIZE: f32 = 9.0;
/// The 8 neighbor offsets of the caption outline copies (a 1-point ring
/// painted as 8 BLACK text copies around the one fill-colored caption).
pub(super) const CAPTION_OUTLINE_OFFSETS: [egui::Vec2; 8] = [
    egui::vec2(-1.0, -1.0),
    egui::vec2(0.0, -1.0),
    egui::vec2(1.0, -1.0),
    egui::vec2(-1.0, 0.0),
    egui::vec2(1.0, 0.0),
    egui::vec2(-1.0, 1.0),
    egui::vec2(0.0, 1.0),
    egui::vec2(1.0, 1.0),
];

/// The open-with launch-failure message color: classic warning ORANGE,
/// high-luminance and readable on the pill's translucent dark frame
/// `(20,20,20,180)` (Wave-6.1 Task 4; the ONLY warning-colored message).
pub(super) const WARNING_MESSAGE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 165, 0);

/// The message-line text color: warning color only for the open-with failure.
pub(super) fn message_text_color(is_warning: bool, pill_ui: &egui::Ui) -> egui::Color32 {
    if is_warning {
        return WARNING_MESSAGE_COLOR;
    }
    pill_ui.visuals().text_color()
}

/// The painted visual state of one pill glyph/caption (which the shared
/// palette resolves): DISABLED dimming, hovered, or enabled idle.
pub(super) enum PillGlyphState {
    Disabled,
    Hovered,
    Idle,
}

/// Maps the painted enablement and hover state to the pill's glyph state.
pub(super) fn glyph_state_for(enabled: bool, hovered: bool) -> PillGlyphState {
    if !enabled {
        return PillGlyphState::Disabled;
    }
    if hovered {
        return PillGlyphState::Hovered;
    }
    PillGlyphState::Idle
}

/// The pill's shared palette: DISABLED dims to the theme's noninteractive
/// stroke color; BOTH enabled states (idle and hovered) pin
/// `PILL_GLYPH_COLOR`. Replaces the four per-module theme-palette helpers.
pub(super) fn glyph_state_color(glyph_state: PillGlyphState, row_ui: &egui::Ui) -> egui::Color32 {
    match glyph_state {
        PillGlyphState::Disabled => row_ui.visuals().widgets.noninteractive.fg_stroke.color,
        PillGlyphState::Hovered | PillGlyphState::Idle => PILL_GLYPH_COLOR,
    }
}

/// The painted-arrow-family triangle glyph (struct-literal constructed at
/// the paint sites; the param object keeps the builder at one argument).
pub(super) struct TriangularGlyphSpec {
    pub(super) rect: egui::Rect,
    pub(super) rotation: f32,
    pub(super) fill_color: egui::Color32,
}

/// The arrow-family triangle with its black outline painted ENTIRELY OUTSIDE
/// the filled triangle geometry: the EXACT point set of
/// `Shape::rotated_triangle` (same three corners, same center rotation),
/// rendered through `convex_polygon` with an outside `PathStroke`.
pub(super) fn outlined_triangle_shape(glyph: TriangularGlyphSpec) -> egui::Shape {
    let rotation = egui::emath::Rot2::from_angle(glyph.rotation);
    let corner_points = [
        glyph.rect.left_top(),
        glyph.rect.right_top(),
        glyph.rect.center_bottom(),
    ];
    let rotated_points: Vec<egui::Pos2> = corner_points
        .into_iter()
        .map(|point| glyph.rect.center() + rotation * (point - glyph.rect.center()))
        .collect();
    egui::Shape::convex_polygon(
        rotated_points,
        glyph.fill_color,
        egui::epaint::PathStroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR).outside(),
    )
}

/// A filled rectangle with its black outline painted OUTSIDE the fill (the
/// fill interior keeps its exact size). egui 0.37 has no fill+stroke
/// `Shape::rect` constructor, so the builder composes fill + outside-stroke
/// outline of the SAME rect nested as one `Shape::Vec`.
pub(super) fn outlined_rect_shape(rect: egui::Rect, fill_color: egui::Color32) -> egui::Shape {
    egui::Shape::Vec(vec![
        egui::Shape::rect_filled(rect, egui::CornerRadius::ZERO, fill_color),
        egui::Shape::rect_stroke(
            rect,
            egui::CornerRadius::ZERO,
            egui::Stroke::new(OUTLINE_STROKE_WIDTH, OUTLINE_COLOR),
            egui::StrokeKind::Outside,
        ),
    ])
}

/// The per-caption paint call of `paint_outlined_caption`: its text and the
/// fill color the fill copy renders in (struct-literal constructed).
pub(super) struct OutlinedCaption {
    pub(super) text: &'static str,
    pub(super) fill_color: egui::Color32,
}

/// Paints one hotkey caption with the offset-copy outline (egui 0.37 has no
/// glyph-outline API): lay out ONCE as a PLACEHOLDER-colored galley,
/// allocate the caption's exact size like the previous 9 pt label, anchor it
/// CENTER_TOP, add 8 BLACK copies at the 1-point neighbor offsets, then add
/// ONE fill-colored copy at the base position (9 galley paints per caption).
pub(super) fn paint_outlined_caption(column_ui: &mut egui::Ui, caption: OutlinedCaption) {
    let font_id = egui::FontId::proportional(CAPTION_TEXT_SIZE);
    let galley = column_ui.painter().layout_no_wrap(
        caption.text.to_string(),
        font_id,
        egui::Color32::PLACEHOLDER,
    );
    let (caption_rect, _) = column_ui.allocate_exact_size(galley.size(), egui::Sense::hover());
    let caption_position = egui::Align2::CENTER_TOP
        .align_size_within_rect(galley.size(), caption_rect)
        .min;
    let caption_painter = column_ui.painter();
    for outline_offset in CAPTION_OUTLINE_OFFSETS {
        caption_painter.add(egui::Shape::galley(
            caption_position + outline_offset,
            galley.clone(),
            OUTLINE_COLOR,
        ));
    }
    caption_painter.add(egui::Shape::galley(
        caption_position,
        galley,
        caption.fill_color,
    ));
}

const OPEN_WITH_SQUARE_SIDE: f32 = 10.0;
const OPEN_WITH_SQUARE_CENTER_OFFSET: egui::Vec2 = egui::Vec2::new(-4.0, 4.0);
const OPEN_WITH_ARROW_LENGTH: f32 = 7.0;
const OPEN_WITH_ARROW_TRIANGLE_SIZE: egui::Vec2 = egui::Vec2::new(6.0, 6.0);
const OPEN_WITH_ARROW_SHAFT_THICKNESS: f32 = 2.5;
const OPEN_WITH_ARROW_SHAFT_OUTLINE_WIDTH: f32 =
    OPEN_WITH_ARROW_SHAFT_THICKNESS + 2.0 * OUTLINE_STROKE_WIDTH;
const OPEN_WITH_ARROW_HEAD_ROTATION: f32 = -std::f32::consts::TAU / 8.0;

/// The open-with button's composite glyph (a 10x10 square with a 7-point
/// arrow from its top-right corner pointing up-right: shaft plus triangle
/// head; paint order square, shaft BLACK under-stroke then glyph stroke,
/// head; the square shifts down-left) — the painted pill-glyph builder home.
pub(super) fn open_with_glyph_shapes(
    glyph_center: egui::Pos2,
    glyph_color: egui::Color32,
) -> Vec<egui::Shape> {
    let square_center = glyph_center + OPEN_WITH_SQUARE_CENTER_OFFSET;
    let square_rect =
        egui::Rect::from_center_size(square_center, egui::Vec2::splat(OPEN_WITH_SQUARE_SIDE));
    let shaft_start = square_rect.right_top();
    let shaft_rise = OPEN_WITH_ARROW_LENGTH * std::f32::consts::FRAC_1_SQRT_2;
    let shaft_end = shaft_start + egui::vec2(shaft_rise, -shaft_rise);
    vec![
        outlined_rect_shape(square_rect, glyph_color),
        egui::Shape::line_segment(
            [shaft_start, shaft_end],
            egui::Stroke::new(OPEN_WITH_ARROW_SHAFT_OUTLINE_WIDTH, OUTLINE_COLOR),
        ),
        egui::Shape::line_segment(
            [shaft_start, shaft_end],
            egui::Stroke::new(OPEN_WITH_ARROW_SHAFT_THICKNESS, glyph_color),
        ),
        outlined_triangle_shape(TriangularGlyphSpec {
            rect: egui::Rect::from_center_size(shaft_end, OPEN_WITH_ARROW_TRIANGLE_SIZE),
            rotation: OPEN_WITH_ARROW_HEAD_ROTATION,
            fill_color: glyph_color,
        }),
    ]
}
