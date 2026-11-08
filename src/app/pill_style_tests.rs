//! Test module of `pill_style` (Wave 4.2 Task 2): the pinned styling
//! constants, the caption offset ring, and the shared palette resolution.

use eframe::egui;

use super::pill_style::{
    CAPTION_OUTLINE_OFFSETS, CAPTION_TEXT_SIZE, OUTLINE_COLOR, OUTLINE_STROKE_WIDTH,
    PILL_GLYPH_COLOR, glyph_state_color, glyph_state_for,
};

#[test]
fn pill_styling_colors_stroke_and_caption_size_pin_style_constants() {
    assert_eq!(PILL_GLYPH_COLOR, egui::Color32::WHITE);
    assert_eq!(OUTLINE_COLOR, egui::Color32::BLACK);
    assert_eq!(OUTLINE_STROKE_WIDTH, 1.0);
    assert_eq!(CAPTION_TEXT_SIZE, 9.0);
}

#[test]
fn caption_outline_offsets_match_the_one_point_neighbors() {
    let expected_offsets = [
        egui::vec2(-1.0, -1.0),
        egui::vec2(0.0, -1.0),
        egui::vec2(1.0, -1.0),
        egui::vec2(-1.0, 0.0),
        egui::vec2(1.0, 0.0),
        egui::vec2(-1.0, 1.0),
        egui::vec2(0.0, 1.0),
        egui::vec2(1.0, 1.0),
    ];
    assert_eq!(CAPTION_OUTLINE_OFFSETS, expected_offsets);
}

#[test]
fn glyph_state_resolves_the_enabled_white_and_disabled_theme_palette() {
    let context = egui::Context::default();
    context.run(Default::default(), |run_context| {
        egui::Area::new(egui::Id::new("pill_style_palette_test")).show(run_context, |test_ui| {
            let enabled_idle = glyph_state_color(glyph_state_for(true, false), test_ui);
            let enabled_hovered = glyph_state_color(glyph_state_for(true, true), test_ui);
            let disabled = glyph_state_color(glyph_state_for(false, true), test_ui);
            assert_eq!(enabled_idle, PILL_GLYPH_COLOR);
            assert_eq!(enabled_hovered, PILL_GLYPH_COLOR);
            let noninteractive_color = test_ui.visuals().widgets.noninteractive.fg_stroke.color;
            assert_eq!(disabled, noninteractive_color);
        });
    });
}
