//! Alias-layer tests (Wave 4.2 A-Task 3): the pinned accept-set, the strictness
//! matrix per member, and real-delivery-class synthetic-input tests through the
//! egui-`Context` harness (one `context.run` frame reading the production chord
//! functions -- the first event-level keyboard simulation in this test suite).

use eframe::egui;

use super::zoom::{zoom_in_chord_was_pressed, zoom_out_chord_was_pressed};
use super::zoom_alias::{
    ZOOM_IN_ALIAS_CHARACTERS, ZOOM_IN_ALIAS_KEYS, ZOOM_OUT_ALIAS_CHARACTERS, ZOOM_OUT_ALIAS_KEYS,
    shift_alias_is_pressed,
};

fn frame_events(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        events,
        ..Default::default()
    }
}

fn shift() -> egui::Modifiers {
    egui::Modifiers {
        shift: true,
        ..Default::default()
    }
}

fn pressed_key_events(alias_key: egui::Key, shift_held: bool) -> Vec<egui::Event> {
    let event_modifiers = if shift_held {
        shift()
    } else {
        egui::Modifiers::default()
    };
    vec![
        egui::Event::ModifiersChanged(event_modifiers),
        egui::Event::Key {
            key: alias_key,
            physical_key: Some(alias_key),
            pressed: true,
            repeat: false,
            modifiers: event_modifiers,
        },
    ]
}

fn typed_text_events(typed_text: &str, shift_held: bool) -> Vec<egui::Event> {
    let event_modifiers = if shift_held {
        shift()
    } else {
        egui::Modifiers::default()
    };
    vec![
        egui::Event::ModifiersChanged(event_modifiers),
        egui::Event::Text(typed_text.to_owned()),
    ]
}

/// Runs exactly ONE egui frame with the given events and reads the PRODUCTION
/// chord functions from it (the same per-frame reads the intake performs).
fn one_frame_chords(events: Vec<egui::Event>) -> (bool, bool) {
    let frame_chords = std::cell::Cell::new((false, false));
    let context = egui::Context::default();
    context
        .run_pass(frame_events(events), |frame_context| {
            frame_chords.set((
                zoom_in_chord_was_pressed(frame_context),
                zoom_out_chord_was_pressed(frame_context),
            ));
        })
        .drop_without_applying_deltas();
    frame_chords.get()
}

#[test]
fn alias_key_sets_are_pinned() {
    assert_eq!(ZOOM_IN_ALIAS_KEYS, [egui::Key::Equals, egui::Key::Plus]);
    assert_eq!(ZOOM_OUT_ALIAS_KEYS, [egui::Key::Minus]);
}

#[test]
fn alias_characters_are_pinned() {
    assert_eq!(ZOOM_IN_ALIAS_CHARACTERS, ['+', '=', '*']);
    assert_eq!(ZOOM_OUT_ALIAS_CHARACTERS, ['-', '\u{2212}', '_']);
}

#[test]
fn shift_alias_is_pressed_matrix() {
    assert!(shift_alias_is_pressed(true, true));
    assert!(!shift_alias_is_pressed(true, false));
    assert!(!shift_alias_is_pressed(false, true));
    assert!(!shift_alias_is_pressed(false, false));
}

#[test]
fn shift_plus_key_event_zooms_in() {
    assert_eq!(
        one_frame_chords(pressed_key_events(egui::Key::Plus, true)),
        (true, false)
    );
}

#[test]
fn shift_equals_key_event_zooms_in() {
    assert_eq!(
        one_frame_chords(pressed_key_events(egui::Key::Equals, true)),
        (true, false)
    );
}

#[test]
fn plain_equals_key_is_inert() {
    assert_eq!(
        one_frame_chords(pressed_key_events(egui::Key::Equals, false)),
        (false, false)
    );
}

#[test]
fn plain_plus_key_is_inert() {
    assert_eq!(
        one_frame_chords(pressed_key_events(egui::Key::Plus, false)),
        (false, false)
    );
}

#[test]
fn plain_minus_key_is_inert() {
    assert_eq!(
        one_frame_chords(pressed_key_events(egui::Key::Minus, false)),
        (false, false)
    );
}

#[test]
fn shift_only_frame_is_inert() {
    let shift_only_events = vec![egui::Event::ModifiersChanged(shift())];
    assert_eq!(one_frame_chords(shift_only_events), (false, false));
}

#[test]
fn typed_plus_with_shift_zooms_in() {
    assert_eq!(
        one_frame_chords(typed_text_events("+", true)),
        (true, false)
    );
}

#[test]
fn typed_equals_with_shift_zooms_in() {
    assert_eq!(
        one_frame_chords(typed_text_events("=", true)),
        (true, false)
    );
}

#[test]
fn shift_star_text_zooms_in() {
    assert_eq!(
        one_frame_chords(typed_text_events("*", true)),
        (true, false)
    );
}

#[test]
fn plain_star_text_is_inert() {
    assert_eq!(
        one_frame_chords(typed_text_events("*", false)),
        (false, false)
    );
}

#[test]
fn spanish_shift_plus_delivery_zooms_in() {
    let mut events = pressed_key_events(egui::Key::CloseBracket, true);
    events.push(egui::Event::Text("*".to_owned()));
    assert_eq!(one_frame_chords(events), (true, false));
}

#[test]
fn spanish_plus_key_member_alone_is_inert() {
    assert_eq!(
        one_frame_chords(pressed_key_events(egui::Key::CloseBracket, true)),
        (false, false)
    );
}

#[test]
fn typed_minus_variants_with_shift_zoom_out() {
    assert_eq!(
        one_frame_chords(typed_text_events("-", true)),
        (false, true)
    );
    assert_eq!(
        one_frame_chords(typed_text_events("\u{2212}", true)),
        (false, true)
    );
    assert_eq!(
        one_frame_chords(typed_text_events("_", true)),
        (false, true)
    );
}

#[test]
fn typed_alias_chars_without_shift_are_inert() {
    assert_eq!(
        one_frame_chords(typed_text_events("+", false)),
        (false, false)
    );
    assert_eq!(
        one_frame_chords(typed_text_events("=", false)),
        (false, false)
    );
    assert_eq!(
        one_frame_chords(typed_text_events("-", false)),
        (false, false)
    );
    assert_eq!(
        one_frame_chords(typed_text_events("\u{2212}", false)),
        (false, false)
    );
    assert_eq!(
        one_frame_chords(typed_text_events("_", false)),
        (false, false)
    );
}

#[test]
fn numpad_names_resolve_into_the_pinned_family_members() {
    assert_eq!(egui::Key::from_name("NumpadAdd"), Some(egui::Key::Plus));
    assert_eq!(
        egui::Key::from_name("NumpadSubtract"),
        Some(egui::Key::Minus)
    );
    assert_eq!(egui::Key::from_name("NumpadEqual"), Some(egui::Key::Equals));
    assert_eq!(egui::Key::from_name("\u{2212}"), Some(egui::Key::Minus));
}
