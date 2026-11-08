//! The strict shift-gated zoom-alias layer (Wave 4.2 A-Task 3), extracted from
//! `zoom.rs`: the accept-set spans BOTH the egui key resolutions (`Equals`/`Plus`
//! zoom in, `Minus` zooms out) AND the typed `Event::Text` characters ('+'/'='
//! zoom in; '-', the true minus sign U+2212, and '_' zoom out), and every member
//! is individually Shift-gated so plain `+`/`=`/`-` stay inert for EVERY keyboard
//! layout. Real Windows may deliver the chord as key members, alternate/physical
//! resolutions, typed text, or none of these; the OPERATOR-RUN re-check (A-Task 5,
//! checklist item 11) is the acceptance gate.

use eframe::egui;

const TRUE_MINUS_SIGN: char = '\u{2212}';

pub(super) const ZOOM_IN_ALIAS_KEYS: [egui::Key; 2] = [egui::Key::Equals, egui::Key::Plus];
pub(super) const ZOOM_OUT_ALIAS_KEYS: [egui::Key; 1] = [egui::Key::Minus];
pub(super) const ZOOM_IN_ALIAS_CHARACTERS: [char; 2] = ['+', '='];
pub(super) const ZOOM_OUT_ALIAS_CHARACTERS: [char; 3] = ['-', TRUE_MINUS_SIGN, '_'];

pub(super) fn shift_alias_is_pressed(alias_member_was_seen: bool, shift_held: bool) -> bool {
    alias_member_was_seen && shift_held
}

pub(super) fn zoom_in_alias_was_pressed(chord_context: &egui::Context) -> bool {
    key_family_was_pressed(chord_context, &ZOOM_IN_ALIAS_KEYS)
        || typed_family_was_pressed(chord_context, &ZOOM_IN_ALIAS_CHARACTERS)
}

pub(super) fn zoom_out_alias_was_pressed(chord_context: &egui::Context) -> bool {
    key_family_was_pressed(chord_context, &ZOOM_OUT_ALIAS_KEYS)
        || typed_family_was_pressed(chord_context, &ZOOM_OUT_ALIAS_CHARACTERS)
}

fn key_family_was_pressed(chord_context: &egui::Context, family_keys: &[egui::Key]) -> bool {
    family_keys
        .iter()
        .any(|family_key| alias_member_was_pressed(chord_context, *family_key))
}

fn typed_family_was_pressed(chord_context: &egui::Context, family_characters: &[char]) -> bool {
    family_characters
        .iter()
        .any(|family_character| alias_character_was_pressed(chord_context, *family_character))
}

fn alias_member_was_pressed(chord_context: &egui::Context, alias_key: egui::Key) -> bool {
    let alias_key_was_pressed = chord_context.input(|input| input.key_pressed(alias_key));
    let shift_held = alias_shift_held(chord_context);
    shift_alias_is_pressed(alias_key_was_pressed, shift_held)
}

fn alias_character_was_pressed(chord_context: &egui::Context, alias_character: char) -> bool {
    let alias_character_was_seen = typed_alias_character_was_seen(chord_context, alias_character);
    let shift_held = alias_shift_held(chord_context);
    shift_alias_is_pressed(alias_character_was_seen, shift_held)
}

fn alias_shift_held(chord_context: &egui::Context) -> bool {
    chord_context.input(|input| input.modifiers.shift)
}

fn typed_alias_character_was_seen(chord_context: &egui::Context, alias_character: char) -> bool {
    chord_context.input(|input| {
        input.events.iter().any(|event| match event {
            egui::Event::Text(typed_text) => typed_text.chars().next() == Some(alias_character),
            _ => false,
        })
    })
}
