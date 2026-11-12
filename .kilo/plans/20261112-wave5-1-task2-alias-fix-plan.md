# Implementation Plan — Wave 5.1 Task 2 (4.1b): Shift+'+' Zoom-In Alias, Round 2 — FINAL

Task: `.agent/todos/20261112/20261112-todo-1.md` → Task 2 (Fix the Shift+'+' zoom-in alias, round 2).
Branch `fix/wave-5-1-operator-feedback` · version 0.5.1 · NOT front-end (no 4.1a sibling).

## 0. DECIDED — the round-2 fix (caller adjudication, binding)

The first research round STOPPED with the diagnostic-fallback question (the
operator's concrete delivery was not pinnable from the resolved crates alone).
The caller answered with the OPERATOR REPORT (verbatim):

> "When I press '+' key, a '+' is printed. And my keyboard is an spanish euro
> keyboard (contains ñ and € key, for example). However, that DOES NOT MATTER.
> This app must function whatever keyboard is using the user. Obviously that, if
> the user has incorrect keyboard configuration, that is not our responsibility."

Caller adjudication (ALL binding):
- The operator's layout is **Spanish (Spain, Euro/ISO-ES)**: '+' is UNSHIFTED on its
  own key; SHIFT + that key produces `'*'`. That is the failing delivery.
- Zoom-out works because the round-1 set ALREADY includes the shifted '-'-variant
  text `'_'` (Spanish Shift+'-' = `'_'`, same as US) — the SYMMETRIC pattern.
- **USER DIRECTIVE (binding)**: the app must function whatever keyboard the user
  has; do NOT hard-code layout assumptions; do NOT blame "incorrect keyboard
  configuration". The fix is the natural, layout-agnostic symmetric one: the
  SHIFT-GATED shifted variants of the '+' and '-' glyph keys (Spanish-and-likes:
  `'*'` zoom-in; `'_'` zoom-out already present), alongside the existing members.
- **Accepted documented side effect**: on US-like layouts Shift+8 also produces
  `'*'` with Shift held → it becomes a zoom-in alias too (harmless extra alias;
  document in §6 module docs; the strict per-key shift-gating is preserved:
  unshifted `'*'` stays inert like every plain key).
- The text path is the fix's primary (research-verified reliable on Windows —
  `Event::Text` fires unconditionally, no `wants_keyboard_input` gate). Add an
  egui `Key` enum member for `'*'` ONLY IF the resolved 0.37 enum has one.

**VERIFIED (this continuation round): egui 0.37.0 has NO asterisk key member.**
The full `Key` variant list (`src/data/key.rs:7-130`) contains no Asterisk/Star/
Multiply variant and `from_name` has NO `"*"` arm (grepped: zero matches).
Therefore the fix is **TEXT-ONLY: `ZOOM_IN_ALIAS_CHARACTERS += '*'`**; no key
member is added and none is invented.

### 0.1 The pinned real delivery (Spanish layout, verified end-to-end)

- Spanish KBDSP: scancode `0x1B` → `VK_OEM_PLUS` with shift states `+`/`*`
  (kbdlayout.info/kbdsp/virtualkeys + shiftstates table).
- winit 0.30.13: scancode 0x1B → `KeyCode::BracketRight` (reverse map,
  `src/platform_impl/windows/keyboard.rs:1108` region) → egui-winit
  `key_from_key_code` → `Key::CloseBracket` (egui-winit 0.37.0 `lib.rs` ~1700).
- Logical: `Key::Character("*")` → `from_name("*")` → None → the event's `key`
  member falls back to `Key::CloseBracket` (egui-winit 0.37.0 `lib.rs:1128,1156`).
- Text: `Event::Text("*")` fires unconditionally (egui-winit 0.37.0
  `lib.rs:1164-1184`; no focus gate).
- **Spanish Shift+'+'-key delivery**: `Event::Key { key: CloseBracket,
  physical_key: Some(CloseBracket), pressed: true, repeat: false,
  modifiers: SHIFT }` + `Event::Text("*")` — the round-1 set catches NEITHER
  (CloseBracket is not a member; `'*'` was not a text member) ⇒ the defect.
- **Spanish unshifted '+'-key delivery**: `Event::Text("+")` with shift=false
  (and key member `Plus` via `from_name("+")`, key.rs:531) — correctly inert by
  the shift gate; the operator's plain-key strictness test passed on exactly this.
- **Spanish Shift+'−'-key delivery (proven working, must keep)**:
  `Event::Text("_")` + key member `Minus` (Character("_") → `from_name("_")` →
  None → physical fallback `KeyCode::Minus → Key::Minus`; `'_'` IS a round-1
  zoom-out text member) — the symmetric template the fix extends to '+'.

## 1–4. Research base (from the first round; unchanged and still binding)

The full translation pipeline, exclusions, and adjudications are recorded in the
plan's research history and hold verbatim:
- **Pipeline**: physical member = scancode-based `key_from_key_code` map
  (`KeyCode::Equal => Key::Equals`, `Minus|NumpadSubtract => Key::Minus`,
  `NumpadAdd => Key::Plus`, `BracketRight => Key::CloseBracket`; egui-winit 0.37.0
  `lib.rs:1107-1112, 1677+`); logical member via `key_from_winit_key` →
  `egui::Key::from_name` (egui 0.37.0 `src/data/key.rs:503-545`; `"+"→Plus` 531,
  `"="→Equals` 532, `"-"→Minus` 529; NO `_`, NO `*`); the event `key` field =
  `logical_key.or(physical_key)` (egui-winit `lib.rs:1128,1156`); `Event::Text`
  unconditional for printable pressed text (`lib.rs:1164-1184`); egui's
  `key_pressed` = unfiltered `Event::Key{key, pressed:true}` count
  (egui 0.37.0 `input_state/mod.rs:403-414, 767-786`).
- **Excluded mechanisms** (all cited): Windows IME interception impossible
  (winit 0.30.13 creates Windows windows IME-disassociated, `window.rs:1161` +
  `ime.rs:141-150`; nobody re-enables — egui-winit toggles only from a
  never-true internal flag `lib.rs:196,1258-1273`; eframe 0.37.0 has zero IME
  references); synthetic-press drop only on focus changes (`lib.rs:448-457`);
  clipboard drops ctrl-gated (`lib.rs:1112-1143`); no egui-side filtering.
- **Adjudications**: the "accept `physical_key == Key::Equals`" candidate is
  PROVEN REDUNDANT (`key` field already = logical.or(physical)) — rejected, no
  such test; "plain unshifted `+` inert" stays binding (unshifting `'+'` would
  break the operator's own passed strictness test); text-member entries stay
  (`'_'` is precisely what makes the operator's Shift+'−' fire).

## 5. Round-2 accept-set (FINAL)

- **Zoom-in**: keys `[egui::Key::Equals, egui::Key::Plus]` (UNCHANGED — no
  asterisk enum member exists, §0) + characters `['+', '=', '*']` (the `'*'`
  member is NEW, shift-gated).
- **Zoom-out**: keys `[egui::Key::Minus]` + characters `['-', '\u{2212}', '_']`
  (UNCHANGED — `'_'` is the proven symmetric counterpart).
- **Gating**: EVERY member individually shift-gated through the existing
  `shift_alias_is_pressed` (`src/app/zoom_alias.rs:19-21`). NO helper movement is
  needed: the per-member gate already applies uniformly — the new `'*'` member
  flows through `typed_family_was_pressed` → `alias_character_was_pressed` →
  `shift_alias_is_pressed` exactly like `'+'`/`'='`/`'_'` do. The helper stays
  where it is; nothing is relocated.
- **Layout-agnostic by construction**: no layout detection, no hard-coded layout
  assumption; the set simply covers, for each glyph key, the character its
  SHIFTED state produces on the layouts where that glyph is unshifted — exactly
  the `'_'` precedent on the zoom-out side. The strict per-key shift-gating
  preserves "plain unshifted `+`/`=`/`-`/`*` inert" on EVERY layout.

## 6. Code changes (the 4.2 implementer; scope pinned exactly)

1. `src/app/zoom_alias.rs`:
   - `ZOOM_IN_ALIAS_CHARACTERS: [char; 2] = ['+', '=']` →
     `ZOOM_IN_ALIAS_CHARACTERS: [char; 3] = ['+', '=', '*']` (line 16).
   - Header doc (lines 1-8) gains the round-2 paragraph, covering: the Spanish
     real delivery (Shift+'+'-key → `Event::Text("*")` + key member
     `CloseBracket` via the physical fallback; from_name has no `'*'` arm), the
     layout-agnostic user directive (works whatever keyboard; no layout
     assumptions), and the accepted US side effect (Shift+8 = `'*'` with shift
     held also zooms in; unshifted `'*'` stays inert).
   - NO logic changes: `shift_alias_is_pressed`, the two
     `*_alias_was_pressed` functions, and the private helpers are untouched
     (the new member rides the existing text-family scan).
2. `src/app/zoom.rs` — the alias doc paragraph (lines 23-29) gains ONE sentence:
   round-2 adds the shifted '+'-key variant `'*'` (Spanish-class layouts) as a
   shift-gated text member, keeping the alias set layout-agnostic; on US-like
   layouts Shift+8 (`'*'`) is then also a zoom-in alias (accepted, documented).
   NO function changes (`zoom_in_chord_was_pressed`/`zoom_out_chord_was_pressed`
   untouched — the arrays are the only data change).
3. NOTHING else changes: `keyboard_intake.rs`, `crate::zoom`, `zoom_button.rs`,
   `floating_controls.rs`, `button_hint.rs`, `pill_style.rs`, ESC/pan surfaces,
   `message_text` sites, `Cargo.toml`/`Cargo.lock` (no new deps; 0.5.1 already
   bumped at Step 3).

### 6.1 Tests — extend `src/app/zoom_alias_tests.rs` (reusing the existing
Context-run harness: `frame_events`/`shift`/`pressed_key_events`/
`typed_text_events`/`one_frame_chords`, lines 14-72)

NEW tests (+4; census 150 → 154):
1. `shift_star_text_zooms_in` — `one_frame_chords(typed_text_events("*", true))`
   == `(true, false)` (the fix's primary path).
2. `plain_star_text_is_inert` — `one_frame_chords(typed_text_events("*", false))`
   == `(false, false)` (strictness: unshifted `'*'` stays inert).
3. `spanish_shift_plus_delivery_zooms_in` — the REAL Spanish delivery composite:
   `let mut events = pressed_key_events(egui::Key::CloseBracket, true);`
   `events.push(egui::Event::Text("*".to_owned()));` →
   `one_frame_chords(events)` == `(true, false)`. Documents the verified event
   sequence (key member `CloseBracket` from the physical fallback + `Text("*")`)
   and that the zoom fires via the text member.
4. `spanish_plus_key_member_alone_is_inert` —
   `one_frame_chords(pressed_key_events(egui::Key::CloseBracket, true))` ==
   `(false, false)`. Pins the layout-agnostic decision: the physical KEY position
   (Spanish '+' key = US-']' position) is NOT accepted — only the shift-gated
   `'*'` text member is (so US Shift+']' `'}'` does NOT zoom).

UPDATED tests (in place, no renames beyond the assertions):
5. `alias_characters_are_pinned` (line 81) →
   `assert_eq!(ZOOM_IN_ALIAS_CHARACTERS, ['+', '=', '*']);` (zoom-out pin line
   unchanged). `alias_key_sets_are_pinned` stays exactly as-is (no key-member
   change to pin).
6. ALL other existing tests unchanged — they ARE the required regressions:
   `typed_minus_variants_with_shift_zoom_out` (Shift+'−' zoom-out still fires,
   incl. `'_'`), `typed_alias_chars_without_shift_are_inert` and
   `plain_equals/plus/minus_key_is_inert` (plain `+`/`=`/`-` inert),
   `shift_only_frame_is_inert`, the numpad name pins, and the matrix test.
   `I`/`O`/`0`/`W`/`H` are outside this file's scope and unchanged by
   construction (no key-member change touches them).

File budget: `zoom_alias_tests.rs` 205 → ~235 lines (four tests + one assertion
edit; under the practical ceiling; if a later review pushes it past ~240, split
the round-2 delivery tests into `src/app/zoom_alias_round2_tests.rs` declared
next to `mod zoom_alias_tests;` at `src/app.rs:193`, reusing the harness helpers
via `super::zoom_alias_tests`). `zoom_alias.rs` 68 → ~76 lines (doc paragraph
+3-array edit) — cap 200 ✓. No new method signatures; max-args/depth rules
untouched.

## 7. Git segmentation (on `fix/wave-5-1-operator-feedback`)

1. `docs: add wave5.1 task2 plan` — this final plan file (committed by the 4.2
   implementer before the code; content is this file as-is).
2. `fix: accept the shifted plus variant for the zoom-in alias` — the §6 code +
   §6.1 tests (exact wording fixed; scope: `zoom_alias.rs`, `zoom.rs` doc
   paragraph, `zoom_alias_tests.rs`).
3. Gitignore Compliance Rule before every commit (`git status`; no ignored files
   staged); NO push in this task (Step 5 handles `main` → `origin` ONLY).

## 8. No-touch list (binding)

- `src/app/keyboard_intake.rs` — chord read order/dispatch chain (lines 25-113).
- `src/zoom.rs` (pure math) and the `zoom.rs` chord functions (lines 135-149).
- ESC/pan/styling surfaces: `esc_close_confirm.rs`, `pan.rs` (both levels),
  `pill_style.rs`, `floating_controls.rs`, `button_hint.rs`, `app_display.rs`,
  `central_surface.rs`, `empty_state_view.rs`, `empty_folder_view.rs`.
- `message_text` sites (silent-bounds contract: zoom actions write nothing).
- The zoom-out family and every existing zoom-in member (nothing removed).

## 9. Verification (deferred, unchanged)

Harness runs stay deferred to Task 5 per the global plan: fresh dev-checks in the
container via `alpine-vm` MCP (`vm_status` first; exit code authoritative; per
`docs/build.md`: fmt/check/test/clippy → `ALL CHECKS PASSED`), census 154
corroboration, release rebuild + DLL audit in Task 5. NO harness run inside this
task's cycle before Task 5.

## 10. TODO Task 2 sub-item compliance check

- "Root-cause ... against the RESOLVED egui-winit 0.37 source ... cite the exact
  translation code path" → §0.1 + §1-4 (all cited to file:line).
- "Fix the accept path so Shift+'+' zooms IN ... WITHOUT regressing Shift+'−'
  (proven), plain unshifted `+`/`=`/`-` inert, `I`/`O`/`0`/`W`/`H` unchanged,
  silent bounds, no new messages" → §5/§6/§8 (text member `'*'` shift-gated;
  regressions pinned in §6.1 items 6; zero message writes; no-touch list).
- "If the source research CANNOT pin the real delivery, STOP ... caller decision"
  → the first round STOPPED and returned the question; the caller answered; §0
  records the adjudication. No assumption was carried.
- "The fix is code, not the OPERATOR-RUN note; strictness remains per-key
  tested" → §6 (code-only fix) + §6.1 (per-key matrix incl. the new member's
  shift/unshifted pair).
