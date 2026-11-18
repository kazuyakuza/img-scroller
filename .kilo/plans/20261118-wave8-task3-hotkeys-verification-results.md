# Wave 8 Task 3 — hotkeys verification results

- Date: 2026-11-18
- Branch: `feat/wave-8-wizard-installer` (wave base `867f1ee`; verified tree HEAD `214ddc388529474964dc86fac5b249d0c68eb771`)
- Task: [`20261117-todo-1.md`](../../.agent/todos/20261117/20261117-todo-1.md) — Task 3 "Hotkeys list (verify, then complete)"
- Plan: [`20261118-wave8-task3-hotkeys-implementation-plan.md`](20261118-wave8-task3-hotkeys-implementation-plan.md)
- Venue: host-side file reads/greps on the working tree (no container, no VM)

**VERDICT: COMPLETE — zero gaps.** The README already IS the consolidated
hotkeys record the task asked for: the Controls table (README lines 45–54) plus
the per-feature text (README lines 38–43) list every one of the 16 inventory
rows WITH its use. The gap branch of the task ("add a consolidated hotkeys
table") is NOT taken.

## §1 Authoritative hotkey inventory (16 rows) and per-row README verdict

Source of truth: an exhaustive sweep of `egui::Key::` and `egui::Event::Text`
across `src/` (production code; test files excluded — test-only constants pin
the production ones and add no new keys).

| # | Key / chord | Use | Source reference (file:line) | README location | Verdict |
|---|---|---|---|---|---|
| 1 | `F11` | Toggle fullscreen ON/OFF (with the pill's fullscreen-square button; one shared action) | `src/app/fullscreen.rs:14` (`FULLSCREEN_TOGGLE_KEY = egui::Key::F11`) | Controls table line 49 ("Toggle fullscreen … F11 (ESC also exits while fullscreen is ON)") + feature 4 text line 40 | LISTED |
| 2 | `ESC` (while fullscreen ON) | Exits fullscreen (never toggles it ON; no-op otherwise) | `src/app/fullscreen.rs:17` (`FULLSCREEN_EXIT_KEY = egui::Key::Escape`) + the pure predicate `escape_exits_fullscreen` at `:21–23`, wired at `src/app/keyboard_intake.rs:41–44` | Controls table line 49 + feature 4 text line 40 ("F11 — or ESC — exits fullscreen while ON") | LISTED |
| 3 | `ESC` ×2 (while fullscreen OFF) | Close the application: FIRST distinct press arms the centered "Press ESC again to close" popup for 2 s; a SECOND DISTINCT press closes; held/autorepeat never counts; expiry re-arms; every fullscreen toggle disarms | `src/app/esc_close_confirm.rs:29` (`POPUP_MESSAGE_TEXT`), `:32` (`POPUP_VISIBLE_SECONDS = 2.0`), `:93–95` (`non_repeat_escape_press_occurred`), `:63–72` (`esc_close_action`); dispatch at `src/app/keyboard_intake.rs:148–165` | Controls table line 50 ("ESC pressed twice — … second distinct press closes (holding ESC never counts)") + feature 4 text line 40 | LISTED |
| 4 | `ESC` (while the Open-with menu is open) | Closes ONLY the open-with menu popup (never arms the ESC-close popup, never exits fullscreen) | `src/app/keyboard_intake.rs:184–189` (`open_with_menu_close_pressed`) + the modal guard `:136–144` | feature 7 text line 43 ("`ESC` or `Q` again closes the menu (`ESC` closes ONLY the menu …)") | LISTED |
| 5 | `I` | Zoom in one step (×1.25; silent at the ×8 ceiling) | `src/app/zoom.rs:46` (`ZOOM_IN_KEY = egui::Key::I`) | Controls table line 51 ("Zoom in / zoom out … I / O") + feature 5 text line 41 ("`I` zooms in") | LISTED |
| 6 | `O` | Zoom out one step (×1.25; silent at the ÷8 floor) | `src/app/zoom.rs:48` (`ZOOM_OUT_KEY = egui::Key::O`) | Controls table line 51 + feature 5 text line 41 ("`O` zooms out") | LISTED |
| 7 | `0` | Restore whole-image fit (zoom factor back to ×1.0; pan reset) | `src/app/zoom.rs:50` (`RESTORE_FIT_KEY = egui::Key::Num0`) + `restore_zoom_to_fit` `:75–78` | Controls table line 52 ("Restore whole-image fit … 0") + feature 5 text line 41 ("The `0` key … return to whole-image fit at ×1.0") | LISTED |
| 8 | `W` | One-shot fit-width size set (width fills the window; zoom keeps working from the set level; silent) | `src/app/zoom.rs:53` (`FIT_WIDTH_KEY = egui::Key::W`) + `set_zoom_to_fit_width` `:86–98` | Controls table line 53 ("Fit width / fit height (one-shot size set) … W / H") + feature 5 text line 41 | LISTED |
| 9 | `H` | One-shot fit-height size set (the W twin) | `src/app/zoom.rs:55` (`FIT_HEIGHT_KEY = egui::Key::H`) + `set_zoom_to_fit_height` `:101–113` | Controls table line 53 + feature 5 text line 41 | LISTED |
| 10 | `←` Left arrow | Pan-then-navigate: pans left while the zoomed image overflows horizontally; navigates to the PREVIOUS sibling image only at the image's left edge; direct when it fits | `src/app/keyboard_intake.rs:52` + the non-repeat read `:56–58`, routed through `handle_arrow_key` (`src/app/pan.rs`) | Controls table line 47 ("Navigate previous/next image … pans first …") + feature 6 text line 42 | LISTED |
| 11 | `→` Right arrow | Same rule, NEXT sibling image | `src/app/keyboard_intake.rs:53` + `:59–61` | Controls table line 47 + feature 6 text line 42 | LISTED |
| 12 | `↑` Up arrow | Pan-then-navigate on the vertical axis: PREVIOUS sibling folder | `src/app/keyboard_intake.rs:54` + `:62–63` | Controls table line 48 + feature 6 text line 42 | LISTED |
| 13 | `↓` Down arrow | Same rule, NEXT sibling folder | `src/app/keyboard_intake.rs:55` + `:64–66` | Controls table line 48 + feature 6 text line 42 | LISTED |
| 14 | `Q` | Open-with menu toggle: opens the menu exactly like the button click (fresh list refresh); pressing Q again closes it; non-repeat reads only | `src/app/open_with_button.rs:33` (`OPEN_WITH_MENU_KEY = egui::Key::Q`); read at `src/app/keyboard_intake.rs:175–182` | Controls table line 54 ("Q (opens the menu exactly like the button click; pressing Q again closes the open menu)") + feature 7 text line 43 | LISTED |
| 15 | Shift+`+` family (zoom-IN aliases) | Zoom in one step: the shift-gated key members `Equals`/`Plus` AND the shift-gated typed characters `'+'`, `'='`, `'*'` (the `'*'` member is the Wave-5.1 Spanish-layout shifted-plus variant; on US-like layouts Shift+8 also zooms in — documented side effect); plain unshifted keys inert | `src/app/zoom_alias.rs:23` (`ZOOM_IN_ALIAS_KEYS = [Equals, Plus]`), `:25` (`ZOOM_IN_ALIAS_CHARACTERS = ['+', '=', '*']`), per-member shift gate `:28–30` | Controls table line 51 ("strict shifted aliases: Shift+'+' — including its shifted-plus variants, e.g. the typed * glyph — or Shift+'=' in … unshifted +/=/*/− inert") + feature 5 text line 41 | LISTED |
| 16 | Shift+`−` family (zoom-OUT aliases) | Zoom out one step: the shift-gated key member `Minus` AND the shift-gated typed characters `'-'`, the true minus sign `U+2212`, and `'_'`; plain unshifted keys inert | `src/app/zoom_alias.rs:24` (`ZOOM_OUT_ALIAS_KEYS = [Minus]`), `:26` (`ZOOM_OUT_ALIAS_CHARACTERS = ['-', TRUE_MINUS_SIGN, '_']`) | Controls table line 51 ("Shift+'−' out; unshifted +/=/*/− inert") + feature 5 text line 41 ("recognized on BOTH the key and the typed-character channels … Shift+'−' zooms out") | LISTED |

All 16 rows read LISTED.

## §2 Sweep outputs (summarized)

### Sweep A — inventory (`egui::Key::` and `Event::Text` over `src/**/*.rs`)

Production-file matches are EXACTLY the pinned lines of §1 column 4:

- `src/app/fullscreen.rs:14`, `:17` — `F11`, `Escape`
- `src/app/esc_close_confirm.rs:94` — `Escape`
- `src/app/keyboard_intake.rs:52–55` (arrow keys) and `:57–65` (the non-repeat arrow reads)
- `src/app/zoom_alias.rs:23–24` — `[Equals, Plus]` / `[Minus]`
- `src/app/zoom.rs:46`, `:48`, `:50`, `:53`, `:55` — `I`, `O`, `Num0`, `W`, `H`
- `src/app/open_with_button.rs:33` — `Q`

The remaining matches are test-file pins (`esc_close_expiry_tests.rs`,
`esc_close_confirm_tests.rs`, `fit_one_shot_tests.rs`, `fullscreen_tests.rs`,
`open_with_button_tests.rs`, `zoom_alias_tests.rs`, `zoom_tests.rs`) and add no
new keys.

The ONLY `egui::Event::Text` code intake in production is
`src/app/zoom_alias.rs:70–76` (`:73` the typed-positive arm); the other
occurrences are module docs (`:3`, `:13`) and `zoom_alias_tests.rs`.

NO Ctrl/Alt/Meta chord exists anywhere in `src/`; opening images is
click/CLI-argument only; there are no other keyboard bindings. The inventory
above is COMPLETE.

### Sweep B — README coverage

Each row's pinned evidence was confirmed at the expected README line:
`F11` (40, 49); `Press ESC again to close` (40, 50); `ESC` or `Q` again closes
the menu (43); `I` zooms in (41); `O` zooms out (41); `The 0 key` (41) and
`Restore whole-image fit | 0` (52); `Fit width / fit height` (53);
`Navigate previous/next image` (47); `Navigate previous/next sibling folder`
(48); the `Q` hotkey (43, 54); `Shift+'+'` (41, 51); `Shift+'−'` (41, 51);
`typed-character channels` (41); `unshifted +/=/*/− inert` (41, 51).

### Sweep C — gap probe (`'_'` alias member)

Grep of `README.md` for `'_'`, `Shift+_`, `underscore` → NO README match.

COVERED note: the typed `'_'` member (`src/app/zoom_alias.rs:26`) is NOT a
separate user chord. On US-like layouts Shift+`-` types `_` — it IS what the
Shift+`−` chord physically produces — so the README sentence "Shift+'−' zooms
out", together with the explicit declaration that the aliases are "recognized
on BOTH the key and the typed-character channels", covers it. The full internal
member set (`'-'`/`U+2212`/`'_'`) is documented for the technical audience in
`.agent/project-info/tech.md:97` and `tech.md:107`. → COVERED, not a gap.

## §3 Alias-representation decision

The alias layer appears as the TWO user-facing chords — `Shift+'+'` (including
`Shift+'='` and the typed `*` shifted-plus variant) for zoom-in and `Shift+'−'`
for zoom-out — NOT one row per internal member. This is exactly how the README
already represents them (Controls table line 51; feature 5 text line 41).

## §4 Mouse-equivalents (recorded separately, no action)

The floating pill's 11 buttons are click-only (no hotkeys): 4 painted arrows,
fullscreen square, `[+]`/`[−]`/restore-fit, fit-width/fit-height, Open-with.
The README Controls table's "On-screen (bottom pill)" column (README lines
47–54) plus the caption/tooltip paragraph (line 56: the seven hotkey captions
`F11`/`I`/`O`/`0`/`W`/`H`/`Q` under their buttons and all 11 tooltips) cover
them. They are NOT hotkeys and stay OUT of the hotkey inventory; no
documentation action results from them.

## §5 Task-6 discharge note

TODO Task 6's README item "the hotkeys record (Task 3)" is DISCHARGED by this
verdict: Task 6 adds NO hotkeys table and NO hotkeys note to README; the
verified Controls table + per-feature text stand as the record. Task 6's README
work covers only its other content (installer-first instructions, GitHub
Releases pointer, window-title behavior).

## §6 No file was edited

NO documentation or source file was edited by Task 3. The zero-edit discipline
holds: `README.md`, `.agent/project-info/*`, `docs/*`, `src/**`, `Cargo.toml`,
and `Cargo.lock` are unchanged by this task. The only Task-3 artifacts are this
results file and the implementation plan file. No harness/build command was run
(there is no code to build or test); no `alpine-vm` usage occurred.
