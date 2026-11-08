# Implementation Plan — Wave 4.2 A-Task 3, Step 4.1b: Zoom Shift-Alias Fix (item 11 defect)

Plan step: Critical Workflow 4.1b (architector) for A-Task 3 of
[`.agent/todos/20261107/20261107-todo-1.md`] (Task 3: "Fix the strict zoom Shift-aliases"), executed
on branch `fix/wave-4-2-operator-feedback` per
[`.kilo/plans/20261107-wave42-then-wave5-global-plan.md`] §"A-Task 3".
The caller commits this plan file with `docs: add wave4.2 task3 plan`.

**Binding constraints (from the caller, TOP PRIORITY):**

- Shift+'+' MUST zoom IN and Shift+'−' MUST zoom OUT on real Windows keyboard input.
- The strict rule STAYS: plain (unshifted) `+`, `=`, `-` remain inert.
- `I` / `O` / `0` behavior unchanged; zoom stays SILENT (zero `message_text` writes).
- No regressions anywhere else (fullscreen, navigation, fit seams, intake order).
- NOT front-end: no 4.1a (Wave-4.1 Task-6 precedent).

---

## 1. Research findings (pinned, with sources)

Pinned stack: `eframe =0.37.0` → `egui` `0.37.0`, `egui-winit` `0.37.0`, `winit` `0.30.x`. All facts
below were verified against sources for those exact versions.

### 1.1 egui 0.37 `Key` enum (source: docs.rs `egui/0.37.0/enum.Key.html` + GitHub tag `0.37.0`
`crates/egui/src/data/key.rs`)

- Relevant members EXACTLY: `Equals`, `Plus`, `Minus` (also `Slash`, `Period`, `Semicolon`, …).
- **There is NO `NumpadAdd` / `NumpadSubtract` / `NumpadEqual` VARIANT in the enum.** The enum has
  no numpad variants at all. Numpad deliveries resolve into ordinary members (§1.2/§1.4).
- Related newer members exist in 0.37: physical modifier variants (`ShiftLeft`/`ShiftRight`, …),
  `CapsLock`, `NumLock`, `ScrollLock`, `PrintScreen`, `Pause`, `Menu`, `Fn`, `Eject`, `Help`,
  `Power`, `Sleep`, `Clear`, `IntlBackslash` — none participate in the alias family.

### 1.2 `egui::Key::from_name` (source: same `key.rs`, pinned)

`from_name(&str)` is the parser egui-winit feeds winit's typed characters through. Pinned arms:

```
"-" | "−" | "Minus" | "NumpadSubtract" => Key::Minus
"+" | "Plus" | "NumpadAdd"             => Key::Plus
"=" | "Equal" | "Equals" | "NumpadEqual" => Key::Equals
```

- The `−` arm is the TRUE MINUS SIGN `U+2212` (not hyphen-minus) — so a typed `−` resolves to
  `Key::Minus` precisely because of this arm.
- Consequence: the named "numpad members" (`NumpadAdd`/`NumpadSubtract`) are NOT enum variants; when
  they appear (as typed names / winit names) they resolve INTO `Plus` / `Minus` / `Equals`. The
  research mandate's "PLUS/MINUS-family members" therefore pins to exactly `{Equals, Plus}` for
  zoom-in and `{Minus}` for zoom-out.
- `from_name("*")` → `None`. Multicharacter strings (e.g. a Windows dead-key two-char text
  `"´+"`) → `None` — `from_name` does NOT match on the first character. (This matters for §3.)

### 1.3 egui-winit 0.37 translation (source: GitHub tag `0.37.0`, `crates/egui-winit/src/lib.rs`,
function `on_keyboard_input` + helpers)

For every winit `KeyboardInput`:

```
logical_key  → key_from_winit_key(): Named → key_from_named_key(),
                                  Character(str) → egui::Key::from_name(str),
                                  Dead/Unidentified → None
physical_key → key_from_key_code(KeyCode): KeyCode::Minus | NumpadSubtract => Key::Minus,
                                           KeyCode::NumpadAdd          => Key::Plus,
                                           KeyCode::Equal              => Key::Equals
active_key   = logical_key.or(physical_key)          // logical wins, physical is the fallback
events.push(egui::Event::Key { key: active_key, physical_key, pressed, repeat: false, modifiers })
```

- `egui::Event::Key` in 0.37 therefore HAS the fields `key`, `physical_key: Option<Key>`,
  `pressed: bool`, **`repeat: bool`** (egui-winit passes `false`; egui's `InputState` fills it in
  during `begin_pass`), and `modifiers`. (Fact for this task: nothing reads `repeat` here. Fact
  recorded for A-Task 4's ESC double-press: the field exists in 0.37.)
- egui-winit ALSO pushes a separate `egui::Event::Text(text)` for the same press (text from winit,
  printable, pressed, no Ctrl/Cmd). The doc-documented winit semantics: `logical_key` is "affected
  by all modifiers except Ctrl" — i.e. Shift SHOULD be applied — and winit 0.30 has documented
  Windows-specific quirks (dead keys, "shift + numpad" special handling noted in its changelog).
- Pinned expected deliveries for the operator's chords (US layout, winit docs semantics):
  Shift+'=' press → `Event::Key { key: Key::Plus, … }` + `Event::Text("+")`; Shift+'-' press →
  `Event::Key { key: Key::Minus, … }` + `Event::Text("_")`; plain '=' → `Key::Equals`; plain '-' →
  `Key::Minus`; numpad '+' → `Key::Plus`; numpad '-' → `Key::Minus`.
- Layout-variance class (researched, NOT pinnable from a Linux container): on layouts where the '+'
  key is the unshifted glyph of that physical key (German QWERTZ '+', Swiss, …), a plain press
  yields `Character("+")` → `Key::Plus` (inert without Shift ✓), and a SHIFT port of that key yields
  a DIFFERENT character (`*` on German) whose `from_name` is `None` → the event falls back to the
  PHYSICAL resolution (`KeyCode::Equal` → `Key::Equals` on US-position keys; other `KeyCode`s on
  non-US ISO positions such as `BracketLeft`/`Backslash` → `Key::OpenBracket`/`Key::Backslash`,
  which are outside the current family). This class is exactly the Wave-4.1 Task-6 doc sentence
  "keyboard-layout variance is the OPERATOR-RUN note".

### 1.4 Root-cause analysis of the operator defect (todo Task 3 row 1)

- **The unit harness never reproduced egui's event resolution.** `src/app/zoom_tests.rs` tests the
  alias layer ONLY through the pure predicate `shift_alias_is_pressed(bool, bool)` called with
  literal booleans — zero `egui::Event` simulation exists anywhere in `src/app/*_tests.rs` for
  keys. "Tests PASS in the container" therefore proves nothing about what real egui-winit delivers
   on Windows; the coverage gap is now a structural fact (tests for the fix close it, §3.5).
- The accept-set mechanism is correct-by-construction for the pinned US deliveries (§1.3): both
  `Key::Plus` and `Key::Equals` (and `Key::Minus`) are individually shift-gated members today.
  What real Windows may deliver for this chord outside the pinned US shape:
  (a) an alternate unshifted-logical glyph (`Character("=")`-with-shift where a winit build applies
  no Shift, or German/Swiss shifted glyphs like `*` falling back through non-family `KeyCode`s);
  (b) Text-only delivery — historically real on Windows (emilk/egui discussion #4703:
  "shift + '2' is reported as a Text event '@'" with no Key event, on older egui; issue #3415:
  shifted symbol keys need the physical-key fallback emilk points to; issue #3653 the
  logical-vs-physical tracking issue) — and for a delivery where BOTH `key_from_winit_key` is
  `None` AND `key_from_key_code` is `None`, egui pushes NO `Event::Key` at all, only
  `Event::Text`. In all three classes the ONLY egui channel that still carries the typed '+' or
  '−' meaning is `Event::Text`.
- **Decision (mandate-sanctioned)**: full platform pinning is impossible from the container; the
  fix accepts what winit ACTUALLY delivers by broadening the accept-set across the whole
  PLUS/MINUS family (key members `Equals`/`Plus`; `Minus`) PLUS the typed-character members, WHERE
  EVERY MEMBER STAYS INDIVIDUALLY SHIFT-GATED so the strict rule holds for EVERY layout. Per the
  todo's deferred-decision line ("typed-character events … and which combination the fix accepts
  — strict-gate preserved") the typed channel is explicitly in scope for THIS 4.1b step.
- Residual risk (explicitly encoded, not hidden): same-frame modifier-ordering on real Windows
  (Shift-down and '+'-down landing in one frame with the Key event before the
  `ModifiersChanged`) cannot be pinned from here; the stateless per-frame predicate keeps the
  operator's re-run (A-Task 5 OPERATOR-RUN, checklist item 11) as the acceptance gate — exactly how
  this project closes GUI defects. A stateful "shift-arrival edge" repair was CONSIDERED AND
  REJECTED in this task: it needs session RAM, cannot be verified container-side, and it would make
  "hold plain '+' (no Shift) then add Shift late" zoom IN — a strictness-adjacent behavior change
  outside the mandate, so the mandate's stateless accept-set fix is exactly what ships.

---

## 2. The fix (binding decisions)

### 2.1 Accept-set (constants, per-member shift-gated)

- Zoom-in key members: `ZOOM_IN_ALIAS_KEYS = [egui::Key::Equals, egui::Key::Plus]`.
- Zoom-in typed characters: `ZOOM_IN_ALIAS_CHARACTERS = ['+', '=']` (`'='` covers the
  logical-without-shift delivery class; its plain twin stays inert via the gate).
- Zoom-out key members: `ZOOM_OUT_ALIAS_KEYS = [egui::Key::Minus]`.
- Zoom-out typed characters: `ZOOM_OUT_ALIAS_CHARACTERS = ['-', '−', '_']` — `'−'` is the true
  minus sign defined as `const TRUE_MINUS_SIGN: char = '\u{2212}';` (avoids any non-ASCII risk in
  source files; the `from_name` arms of §1.2 prove egui treats `U+2212` as minus-family); `'_'` is
  what a US-layout Shift+'-' press TYPES, so it is the minus-family text member for that delivery
  class. `'_'` with Shift is not producible by unrelated keys on US/German layouts (the French
  plain-'8' caveat is pinned as a reject test in §3.5 test 13, keeping strictness per-layout).
- Every member fires ONLY when `input.modifiers.shift` is true in the SAME frame
  (`shift_alias_is_pressed(alias_member_seen, shift_held)` — the EXISTING pure predicate, moved
  verbatim into the new module). All member reads of one frame resolve against the SAME per-frame
  `chord_context` state — repeated `ctx.input(...)` closures over one frame read one consistent
  `InputState` snapshot, so the gate and the member reads cannot disagree within a frame.
- Strictness invariants (tests in §3.5 prove them PER MEMBER, not per set):
  - plain `=` (key or text, no Shift) → inert; plain `+` (key or text, no Shift) → inert;
    plain `-` (key or text, no Shift) → inert; plain `−` and plain `_` (text, no Shift) → inert.
  - Numpad '+' plain (`Key::Plus`, no Shift) → inert; numpad '-' plain → inert.
  - `I` / `O` / `0` untouched (`ZOOM_IN_KEY`/`ZOOM_OUT_KEY`/`RESTORE_FIT_KEY` in `zoom.rs`
    unchanged, the chord entry functions unchanged in shape).
- Single-dispatch invariant: the chord functions stay PER-FRAME BOOLEANS; the intake dispatches at
  most ONE zoom step per frame. Members are OR-combined inside the chord predicate — when a US
  Shift+'=' press delivers BOTH `Key::Plus` AND `Text("+")` in one frame, that is still
  exactly one `zoom_in_once` step from the intake (no double-step). No code needs changing for
  this (intake dispatch is one `if`), but reviewers must verify the alias helpers do NOT return
  per-hit counts and zoom.rs/intake gain no per-member dispatch.

### 2.2 Text-member semantics (pinned so no judgment is needed)

- A typed-character member counts as seen this frame when the frame's `input.events` contain
  `egui::Event::Text(text)` whose FIRST character equals the member (`text.chars().next() ==
  Some(alias_character)`). First-char semantics is the deliberate choice: `Key::from_name` matches
  the WHOLE string (§1.2, multichar dead-key text would otherwise be lost), while the typed family
  is defined per single character.
- The scan runs against the SAME per-frame `chord_context` that the key-member reads use (one
  alias query per frame; `input.events` is this frame's event list — the same list the app's
  runtime intake sees).

---

## 3. Code changes (atomic, junior-exact)

Line-count checks (measured now): `src/app/zoom.rs` 178 raw / 161 non-blank (cap 200; after
§3.2 ≈ 135 raw); `src/app/zoom_tests.rs` 157 raw / ~126 non-blank (after §3.4 ≈ 127);
`src/app.rs` 185 raw / 166 non-blank (after §3.5 ≈ 187 — under cap). The alias layer move is
REQUIRED by the ≤200-line budget: adding the typed-family scan to `zoom.rs` in place would exceed
the cap.

### 3.1 NEW file `src/app/zoom_alias.rs` (~100 raw lines)

Module doc comment: one short block stating (a) it owns the strict shift-gated zoom-alias layer
extracted from `zoom.rs` (Wave 4.2 A-Task 3), (b) the accept-set: egui key resolutions AND typed
`Event::Text` characters, every member individually Shift-gated (plain `+`/`=`/`-` inert for EVERY
keyboard layout), (c) the researched delivery uncertainty — real Windows may deliver the chord as
key members, alternate/physical resolutions, typed text, or none of these; the OPERATOR-RUN
(A-Task 5, checklist item 11) is the acceptance gate. NO commented-out code; self-documenting
names; no comments inside functions.

Contents, in order (visibility per the prefer-private rule: the four family constants, the pure
predicate, and the two entry points are `pub(super)`; `TRUE_MINUS_SIGN` and all helpers are
private):

1. `const TRUE_MINUS_SIGN: char = '\u{2212}';`
2. `pub(super) const ZOOM_IN_ALIAS_KEYS: [egui::Key; 2] = [egui::Key::Equals, egui::Key::Plus];`
3. `pub(super) const ZOOM_OUT_ALIAS_KEYS: [egui::Key; 1] = [egui::Key::Minus];`
4. `pub(super) const ZOOM_IN_ALIAS_CHARACTERS: [char; 2] = ['+', '='];`
5. `pub(super) const ZOOM_OUT_ALIAS_CHARACTERS: [char; 3] = ['-', TRUE_MINUS_SIGN, '_'];`
6. The pure predicate MOVED verbatim from `zoom.rs` (same name, same body):

   ```rust
   pub(super) fn shift_alias_is_pressed(alias_member_was_seen: bool, shift_held: bool) -> bool {
       alias_member_was_seen && shift_held
   }
   ```

7. Entry points (one body each, ≤10 lines, no nested-`if` beyond depth 2; `if`/early-return with a
   SINGLE-section condition):

   ```rust
   pub(super) fn zoom_in_alias_was_pressed(chord_context: &egui::Context) -> bool {
       key_family_was_pressed(chord_context, &ZOOM_IN_ALIAS_KEYS)
           || typed_family_was_pressed(chord_context, &ZOOM_IN_ALIAS_CHARACTERS)
   }
   pub(super) fn zoom_out_alias_was_pressed(chord_context: &egui::Context) -> bool {
       key_family_was_pressed(chord_context, &ZOOM_OUT_ALIAS_KEYS)
           || typed_family_was_pressed(chord_context, &ZOOM_OUT_ALIAS_CHARACTERS)
   }
   ```

8. Shared family helpers (each ≤2 params; ≤2 nesting depth):

   ```rust
   fn key_family_was_pressed(chord_context: &egui::Context, family_keys: &[egui::Key]) -> bool {
       family_keys // early-return true on the first shift-gated member
           .iter()
           .any(|family_key| alias_member_was_pressed(chord_context, *family_key))
   }
   fn typed_family_was_pressed(chord_context: &egui::Context, family_characters: &[char]) -> bool {
       family_characters
           .iter()
           .any(|family_character| alias_character_was_pressed(chord_context, *family_character))
   }
   ```

   (`.any(...)` has the same short-circuit-on-first-hit behavior as an explicit loop; keep the
   `.any(...)` form exactly as shown — no reviewer choice points.)

9. Per-member resolvers (the ONLY places that read egui input; each exactly one 2-parameter form,
   respecting the max-2-params rule):

   ```rust
   fn alias_member_was_pressed(chord_context: &egui::Context, alias_key: egui::Key) -> bool {
       let alias_key_was_pressed = chord_context.input(|input| input.key_pressed(alias_key));
       let shift_held = alias_shift_held(chord_context);
       shift_alias_is_pressed(alias_key_was_pressed, shift_held)
   }

   fn alias_character_was_pressed(
       chord_context: &egui::Context,
       alias_character: char,
   ) -> bool {
       let alias_character_was_seen =
           typed_alias_character_was_seen(chord_context, alias_character);
       let shift_held = alias_shift_held(chord_context);
       shift_alias_is_pressed(alias_character_was_seen, shift_held)
   }

   fn alias_shift_held(chord_context: &egui::Context) -> bool {
       chord_context.input(|input| input.modifiers.shift)
   }

   fn typed_alias_character_was_seen(chord_context: &egui::Context, alias_character: char) -> bool {
       chord_context.input(|input| {
           input.events.iter().any(|event| match event {
               egui::Event::Text(typed_text) => {
                   typed_text.chars().next() == Some(alias_character)
               }
               _ => false,
           })
       })
   }
   ```

Rules checklist for the junior implementer (4.3 reviewers verify the same points): no more than 2
levels of nesting anywhere (the `match` arm + `.any` closure is depth 2), max-2-params per method
(param-object NOT needed here — every call has ≤2 params), no method body over 50 lines, file
≤200 raw lines, everything private except the four items listed `pub(super)`, zero `message_text`
writes, zero new dependencies, no changes to constants of the chord entry functions
(`ZOOM_IN_KEY`/`ZOOM_OUT_KEY`/`RESTORE_FIT_KEY`, `FIT_WIDTH_KEY`/`FIT_HEIGHT_KEY`).

### 3.2 EDIT `src/app/zoom.rs` (replaces the alias layer; intake shape untouched)

1. REMOVE the functions `shift_alias_is_pressed`, `zoom_in_shift_alias_was_pressed`, and
   `zoom_out_shift_alias_was_pressed` (lines ~136–158 area: the `/// The strict-shift alias rule…`,
   `/// The zoom-in shift alias…`, `/// The zoom-out shift alias…` blocks), and REPLACE their call
   sites inside `zoom_in_chord_was_pressed` / `zoom_out_chord_was_pressed` with
   `zoom_alias::zoom_in_alias_was_pressed(chord_context)` /
   `zoom_alias::zoom_out_alias_was_pressed(chord_context)` — the two chord members themselves are
   otherwise byte-identical (I-key check first, alias second; `zoom_out_chord_was_pressed` keeps
   its shape).
2. Add `use super::zoom_alias;` next to the existing `use super::ImagesScrollerApp;` (keep
   `use eframe::egui;` and `use crate::zoom;` untouched).
3. REPLACE doc-comment lines 22–25 (the 'The '+/-' aliases are strictly shift-gated…' paragraph)
   with the updated strict-rule paragraph: the aliases are strictly shift-gated per member over
   BOTH the egui key resolutions (`Equals`/`Plus` → in, `Minus` → out) AND the typed character
   deliveries ('+'/'=' → in, '-'/'−'/'_' → out); plain (unshifted) `+`, `=`, `-` do nothing for every
   keyboard layout (Wave 4.1 Task 6 strictness preserved; Wave 4.2 A-Task 3 broadened the member set
   to what real egui-winit can deliver — layout/typing variance is still an OPERATOR-RUN gate).
4. DO NOT touch: `zoom_in_once`/`zoom_out_once`/`restore_zoom_to_fit`/
   `set_zoom_to_fit_width`/`set_zoom_to_fit_height`/`zoom_can_grow`/`zoom_can_shrink`/
   `zoomed_off_fit`, all key constants, the whole produce-no-messages discipline.

### 3.3 EDIT `src/app.rs` — two module declarations ONLY

Insert BOTH lines in the existing lists (alphabetical order of the surrounding names):

- after `mod zoom;` and before `mod zoom_button;` (module list block, indented 4):
  `mod zoom_alias;`
- after `mod tests;` and before `mod zoom_tests;` (test list block):
  `mod zoom_alias_tests;`

Nothing else in `app.rs` changes.

### 3.4 EDIT `src/app/zoom_tests.rs` — remove the moved alias coverage ONLY

DELETE cleanly (no dangling imports left): the `/// Shorthand…` helper block
(`fn alias(alias_key_was_pressed: bool, shift_held: bool) -> bool` with its doc comment) and the
FIVE alias tests `zoom_in_shift_alias_matches_shift_equals`,
`zoom_in_shift_alias_matches_shift_plus`, `zoom_in_shift_alias_rejects_the_plain_keys`,
`zoom_out_shift_alias_matches_shift_minus`, `zoom_out_shift_alias_rejects_the_plain_keys`
(file tail from the shorthand to EOF, ~lines 126–157). Everything above it stays byte-identical
(harness app-state tests, badge/label pins, bounds/silence tests). Result ≈ 127 raw lines
(≤200 ✓, ≈115 non-blank).

### 3.5 NEW file `src/app/zoom_alias_tests.rs` (~170 raw lines; keep ≤200 — split into
`zoom_alias_key_tests.rs` + `zoom_alias_text_tests.rs` following the
`sibling_folder_listing/tests+neighbor_tests` precedent ONLY if the single file would exceed 200
raw lines; default to the single file)

Module doc: Wave 4.2 A-Task 3 — the alias accept-set pin, the strictness matrix per member, and
real-delivery-class synthetic-input tests through the pinned egui-`Context` harness pattern of
`src/app/pill_style_tests.rs` (`egui::Context::default()` + `context.run(egui::RawInput { … })`,
reading the production chord functions inside the frame closure — the first event-level keyboard
simulation in this test suite, closing the §1.4 coverage gap).

Imports:

```rust
use eframe::egui;
use super::zoom::{zoom_in_chord_was_pressed, zoom_out_chord_was_pressed};
use super::zoom_alias::{
    shift_alias_is_pressed, ZOOM_IN_ALIAS_CHARACTERS, ZOOM_IN_ALIAS_KEYS,
    ZOOM_OUT_ALIAS_CHARACTERS, ZOOM_OUT_ALIAS_KEYS,
};
```

Harness helpers (top of file; the ONLY plumbing; keep exactly these five functions in this order):

CONCRETE helper shapes (copy semantics, not necessarily byte-for-byte):

```rust
fn frame_events(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput { events, ..Default::default() }
}

fn shift() -> egui::Modifiers {
    egui::Modifiers { shift: true, ..Default::default() }
}

fn pressed_key_events(alias_key: egui::Key, shift_held: bool) -> Vec<egui::Event> {
    let event_modifiers = if shift_held { shift() } else { egui::Modifiers::default() };
    vec![
        egui::Event::ModifiersChanged(event_modifiers),
        egui::Event::Key {
            key: alias_key,
            physical_key: Some(alias_key),
            pressed: true,
            repeat: false, // egui 0.37 Event::Key has the repeat: bool field (research §1.3)
            modifiers: event_modifiers,
        },
    ]
}

fn typed_text_events(typed_text: &str, shift_held: bool) -> Vec<egui::Event> {
    let event_modifiers = if shift_held { shift() } else { egui::Modifiers::default() };
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
    context.run(frame_events(events), |frame_context| {
        frame_chords.set((
            zoom_in_chord_was_pressed(frame_context),
            zoom_out_chord_was_pressed(frame_context),
        ));
    });
    frame_chords.get()
}
```

Note for the implementer: raw `egui::RawInput { ..Default::default() }` + explicit
`Event::ModifiersChanged` first is the MUST pattern (do not rely on any `RawInput` modifier field
or on event-stamped-only modifiers; egui's `InputState` honors `Event::ModifiersChanged`, and the
SAME pattern keeps every frame self-contained). `Context::run` is a ONE-frame call
(`RawInput` in, one UI pass, `impl FnOnce` closure) — the sketched shape of `one_frame_chords`
(with the `std::cell::Cell` result holder) is the pattern to copy so the per-frame booleans escape
the closure without borrow friction.

Tests (EXACTLY these 14, in this order; each is an INDEPENDENT `#[test]`; the pure-predicate and
the synthetic-input classes live side by side):

1. `alias_key_sets_are_pinned` — `assert_eq!(ZOOM_IN_ALIAS_KEYS, [egui::Key::Equals, egui::Key::Plus]);`
   `assert_eq!(ZOOM_OUT_ALIAS_KEYS, [egui::Key::Minus]);`
2. `alias_characters_are_pinned` —
   `assert_eq!(ZOOM_IN_ALIAS_CHARACTERS, ['+', '=']);`
   `assert_eq!(ZOOM_OUT_ALIAS_CHARACTERS, ['-', '\u{2212}', '_']);`
3. `shift_alias_is_pressed_matrix` — the full 2×2 move of the old predicate tests:
   true/true fires; true/false inert; false/true inert; false/false inert (four asserts, one test;
   replaces the five removed zoom_tests entries without losing a case).
4. `shift_plus_key_event_zooms_in` — `one_frame_chords(pressed_key_events(Key::Plus, true))` →
   `(true, false)`.
5. `shift_equals_key_event_zooms_in` — same with `Key::Equals` → `(true, false)`.
6. `plain_equals_key_is_inert` — `pressed_key_events(Key::Equals, false)` → `(false, false)`.
7. `plain_plus_key_is_inert` — `pressed_key_events(Key::Plus, false)` → `(false, false)`.
8. `plain_minus_key_is_inert` — `pressed_key_events(Key::Minus, false)` → `(false, false)`.
9. `shift_only_frame_is_inert` — a frame with the event list
   `[egui::Event::ModifiersChanged(shift())]` → `(false, false)`.
10. `typed_plus_with_shift_zooms_in` — `typed_text_events("+", true)` → `(true, false)`.
11. `typed_equals_with_shift_zooms_in` — `typed_text_events("=", true)` → `(true, false)`.
12. `typed_minus_variants_with_shift_zoom_out` — THREE independent frames inside the one test:
    `("-", true)`, `("\u{2212}", true)`, `("_", true)` → each `(false, true)`.
13. `typed_alias_chars_without_shift_are_inert` — separate no-shift frames for `"+"`, `"="`, `"-"`,
    `"\u{2212}"`, `"_"` → EACH `(false, false)` (the per-layout strictness pin: a German-layout
    plain '+' press, a French-layout plain '_' press, etc. all stay inert because of the gate).
14. `numpad_names_resolve_into_the_pinned_family_members` — the mandate's Numpad-members pin via
    the researched resolutions (NO NumpadAdd/NumpadSubtract variants exist §1.1/§1.2):

    ```rust
    assert_eq!(egui::Key::from_name("NumpadAdd"), Some(egui::Key::Plus));
    assert_eq!(egui::Key::from_name("NumpadSubtract"), Some(egui::Key::Minus));
    assert_eq!(egui::Key::from_name("NumpadEqual"), Some(egui::Key::Equals));
    assert_eq!(egui::Key::from_name("\u{2212}"), Some(egui::Key::Minus));
    ```

Rules checklist for the test file: ≤200 raw lines, helpers ≤2 params, each test ≤15 lines,
asserts limited to the pinned constants and the two chord booleans, no `message_text` assertions
here (zoom is silent — already pinned in `zoom_tests.rs`; do not duplicate).

### 3.6 Expected test census

- Before this task (post Task 2): **107** tests.
- Removed: 5 (the moved/merged zoom_tests alias entries).
- Added: 14 (§3.5).
- Expected after this task: **116** (`107 − 5 + 14`). A-Task 5 pins the final exact number.

---

## 4. Git actions (branch `fix/wave-4-2-operator-feedback`)

- Plan file (caller): `docs: add wave4.2 task3 plan` — ONLY
  `.kilo/plans/20261107-wave4-2-task3-zoom-alias-fix-implementation-plan.md`.
- Implementation (4.2), ONE commit: `fix: accept windows real delivery for the zoom shift aliases` —
  files: `src/app/zoom_alias.rs` (new), `src/app/zoom_alias_tests.rs` (new), `src/app/zoom.rs`,
  `src/app/zoom_tests.rs`, `src/app.rs`. (Small-commit style: code + tests together is this repo's
  `feat:` precedent; do NOT split unrelated hunks.)
- 4.3 review fixes (if any): separate small `fix:` commit(s), scoped.
- 4.4 module docs (docs-specialist): `docs:` commit ONLY if the §3.1/§3.5 doc comments need polish
  after review; "not required" is an accepted outcome (no separate file edits).
- 4.6 mark: `docs: …` todo-mark commit (todo 20261107 Task 3 sub-items `[]` → `[x]`, preserving all
  other content; OVERWRITE-TODO-PREVENTION rule applies).
- Adapt to Gitignore Compliance Rule before every commit (`git status` reading `.gitignore`; no
  `target/`, no `dist/`, no logs). Version bump is Step A.3's concern, NOT this task. NO
  `CHANGELOG.md` edits this wave.

---

## 5. Review + verification steps

- **4.3 review checklist** (code-reviewer + code-simplifier): the §2.1 accept-set members EXACTLY;
  every member individually shift-gated via the moved `shift_alias_is_pressed`; intake
  (`keyboard_intake.rs`) UNTOUCHED; zoom stays silent; no new public surface beyond the §3.1 items
  (four family constants `pub(super)`, `shift_alias_is_pressed` re-homed `pub(super)`, the two entry
  points `pub(super)`; everything else private); rules (≤200 lines/file, ≤50 lines/method, ≤2
  nesting, ≤2 params, single-section booleans, no commented code, self-documenting names);
  `zoom.rs` chord functions
  unchanged in shape (I/O/0-first behavior intact); no per-member double-dispatch (§2.1 invariant).
- **4.5a not applicable** (no front-end spec for this task — the caller's directive; record this as
  the reviewer/architector-side note only).
- **DEFERRED VERIFICATION NOTE (binding)**: harness execution is DEFERRED to A-Task 5 per the wave
  plan (Task-2 precedent: implementer runs NO cargo commands in 4.2; syntax/lint/test failures are
  A-Task-5 findings routed back through the caller). A-Task 5 then runs the container dev-checks
  (`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`;
  `vm_status` FIRST; exit code authoritative) expecting `ALL CHECKS PASSED` with the pinned census
  (§3.6, final exact number pinned there), rebuilds the 0.4.2 exe, re-checks the 22-DLL import
  audit constant, and runs the OPERATOR-RUN re-check of todo item 11 ON REAL WINDOWS:
  "ALIASES STRICT: Shift+'+' zooms IN; Shift+'-' zooms OUT; pressing plain '=', '+' or '-' WITHOUT
  Shift does NOTHING (all three inert)" — plus item 8/10's alias mentions (Shift+'+' under ZOOM IN,
  Shift+'-' under ZOOM OUT BELOW FIT) and the no-message line (item 15).
- **Do NOT edit existing `.kilo/plans/*` files**: any follow-up plan is a NEW plan file.

## 6. No-touch list (hard)

- `src/app/keyboard_intake.rs` — chord reads/dispatch order UNCHANGED (the fix lives under the
  existing zoom chord functions).
- `src/zoom.rs` — pure zoom math UNTOUCHED.
- `src/app/zoom_button.rs`, `src/app/fit_button.rs`, `src/app/fullscreen.rs`,
  `src/app/fullscreen_button.rs`, `src/app/button_hint.rs`, `src/app/floating_controls.rs`,
  `src/app/pill_style.rs` — UNTOUCHED.
- `message_text` semantics — UNTOUCHED (zoom/fit stays 100% silent; the only message remains the
  unsupported-file failure text).
- `Cargo.toml` / `Cargo.lock` — UNTOUCHED (no dependency or version changes in this task).
- The fit/W/H/0 seams (`FIT_WIDTH_KEY`/`FIT_HEIGHT_KEY`/`RESTORE_FIT_KEY` and
  `set_zoom_to_fit_width`/`set_zoom_to_fit_height`), and all button enablement predicates —
  UNTOUCHED.

## 7. Plan self-verification vs the assigned task

| Task requirement | Covered by |
| --- | --- |
| Root-cause the real-Windows failure (todo Task 3 row 1) | §1.4 (unit-harness gap = the proven structural fact; delivery classes (a)/(b)/text-only researched; US-pin vs layout variance; ambiguity encoded, not assumed) |
| context7-first library facts; no guessed API shapes | §1.1–§1.3 (context7 `/emilk/egui/0.37.0` + `/websites/rs_egui` + directly fetched `0.37.0` tag sources for `key.rs` and egui-winit `lib.rs`; `Event::Key.repeat: bool` existence pinned for Task 4) |
| Key enum members incl. NumpadAdd/NumpadSubtract "if present" | §1.1/§1.2 — pinned ABSENT; resolutions via `from_name` + `key_from_key_code` encoded; test 14 pins it |
| Final accept-set + shared shift-gate refactor (helper family, ≤2-param rule, zoom.rs ≤200, possible `zoom_alias.rs` split) | §2.1–§2.2, §3.1–§3.3 (split REQUIRED by measured budget; all decisions made, no judgment left) |
| Synthetic-input unit tests in the EXISTING egui-Context harness pattern; Shift-held accepted per member; plain each key rejected WITHOUT Shift, per-key | §3.5 tests 3–9, 12, 13 (pill_style_tests.rs `context.run` harness reused; per-key strictness) |
| NOT front-end (no 4.1a) | §5 |
| Pin test of the accept-set | Tests 1, 2, 14 |
| Git segmentation (`docs: add wave4.2 task3 plan` → `fix: …`; small-commit style) | §4 |
| No-touch list (intake order; `src/zoom.rs` math; `message_text`) | §6 |
| Expected census delta | §3.6 (107 → 116) |
| File-size budgets | §3 preamble + per-file numbers |
| Deferred verification note (A-Task 5) | §5 |
| Script/no vague instructions | All step bodies specify exact names, signatures, assertions, and line counts; no "choose"/"optimize" language |
