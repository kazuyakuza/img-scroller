# Wave 6.1 — Task 6 (Cycle D) Documentation Plan (Critical Workflow step 4.1b, architector)

- Source TODO: `.agent/todos/20261113/20261113-todo-1.md` → `### Task 6: Documentation updates` (one sub-item).
- Global plan: `.kilo/plans/20261113-wave6-1-open-with-feedback.md` → `### Cycle D — Task 6 (documentation)` (this plan file is named there verbatim).
- Executor (step 4.2): docs-specialist — a JUNIOR agent under 50% restriction. Every edit below is FINAL (anchor + exact text); do NOT redesign, reword beyond the pinned sentences, or add sections the plan does not list. Minor latitude: only mechanical markdown line-wrapping inside the prescribed sentences.
- Wave state at planning time: branch `fix/wave-6-1-open-with-feedback`, HEAD `71b52bb`, working tree clean, version 0.6.1, census 189 unit tests / 28 test-bearing files, exe `dist/windows-x64/img-scroller.exe` = 19,645,219 bytes, DLL audit SET-EQUAL 22, dev-checks `ALL CHECKS PASSED`.
- This cycle is DOCS-ONLY: NO application-code edits, NO Cargo changes, NO harness/VM runs (pure Markdown edits — nothing to compile), NO `CHANGELOG.md`, NO edits to existing `.kilo/plans/*` files, NO TODO marks (4.6 is a separate step), NO merge/push (Step 5 of the global plan is NOT this cycle).

---

## §0 WAVE FACTS — THE SOURCE OF TRUTH FOR EVERY EDIT (all verified this session)

| # | Fact | Value / source |
| --- | --- | --- |
| F1 | D1 ROOT CAUSE | the launched data object was built via `SHCreateShellItemArrayFromShellItem` requesting `IID_IDataObject` — off-contract for a shell item array (its documented interface set is `IUnknown` + `IShellItemArray` only) → the request failed BEFORE `IAssocHandler::Invoke` on EVERY selection, for every handler kind, every file type (the 100% failure). Evidence: T1 plan §0 suspect (A). |
| F2 | D1 FIX (commit `befcf9e`) | the documented single-item bind `IShellItem::BindToHandler(None::<&IBindCtx>, &BHID_DataObject)` in `src/platform/windows_open_with.rs::file_data_object` (module doc lines 9–15 record the root cause); the array route removed (import + call; zero code residue — the only tree mention of the removed symbol is the prescribed root-cause doc comment at `windows_open_with.rs:11`); zero new `windows`-crate features; NO semantics change (enumeration content, refresh contract, `launch_index` pairing, state gates, failure message all unchanged). |
| F3 | Q HOTKEY | `pub(super) const OPEN_WITH_MENU_KEY: egui::Key = egui::Key::Q;` (`src/app/open_with_button.rs:33`); caption `Some("Q")` under the symbol via `button_hint::hotkey_reference` (`src/app/button_hint.rs:66`) painted by the unchanged shared 9-pt white-with-black-outline caption path; enabled tooltip `"Open with... (Q)"` (`button_hint.rs:89`). Q opens programmatically (`open_with_menu_should_open_on_q` = DISTINCT non-repeat press + `open_with_menu_is_available()` + `full_ui_chrome_is_visible()`; then `open_with_menu_from_q_press` = `refresh_open_with_programs()` + `egui::Popup::open_id`) — the refresh contract gains the Q-open point; Q again closes (`Popup::close_id`). |
| F4 | MODAL MENU | while the menu is open ALL other functionalities are OFF: the first-guard `handle_modal_open_with_menu_keys` in `read_and_dispatch_keystrokes` (placed AFTER the first-line `update_esc_close_popup`, so an armed ESC-close popup still expires behind the menu) skips F11/fullscreen-exit/ESC-close-arm/zoom/fit/arrows and closes the menu via `Popup::close_id` on a distinct Q or ESC; the FIVE pill-row click gates (`!open_with_button::open_with_menu_is_open(...)`) suppress arrow/fullscreen/zoom/fit/open-with-refresh clicks (arrows `floating_controls.rs:186`, fullscreen `fullscreen_button.rs:32`, zoom `zoom_button.rs:93`, fit `fit_button.rs:89`, open-with refresh `open_with_button.rs:91`); ESC-while-open closes ONLY the menu (never arms the close popup, never exits fullscreen even when ON); outside click dismisses (egui default `PopupCloseBehavior::CloseOnClick`, no override). Popup identity: the deterministic `open_with_menu_id()` = `Id::unique("open_with_menu_popup")`; builder gate `enabled || is_id_open`. |
| F5 | WARNING COLOR | `pill_style::WARNING_MESSAGE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 165, 0)` (`src/app/pill_style.rs:38`) — classic warning ORANGE, readable on the pill's translucent dark frame; the pure predicate `message_line_is_warning(message)` (`open_with_button.rs:135–137`) matches ONLY `message == Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)`; `pill_style::message_text_color` applies the warning color ONLY for that message; other messages (unsupported-file, cleared states) keep the default theme text color; placement unchanged (the pill's message line). NO app-state field (D3). |
| F6 | VERIFICATION (Task 5, results at `.kilo/plans/20261113-wave6-1-task5-verification-results.md`, adjudicated VERIFIED at `74567db`) | dev-checks `ALL CHECKS PASSED` (189 passed; 0 failed); windows-target compile exit 0 (only the pre-existing benign dead-code warning on `compare_entry_names`); release rebuild → `dist/windows-x64/img-scroller.exe` = 19,645,219 bytes (0.6.1; the 0.6.0 artifact 19,647,219 B overwritten atomically); DLL audit SET-EQUAL 22 (`BindToHandler` = a vtable call through the already-imported shell COM surface; the removed `SHCreateShellItemArrayFromShellItem` and the kept `SHAssocEnumHandlers`/`SHCreateItemFromParsingName` are all shell32 exports); `git diff 46d1d22..HEAD -- Cargo.toml Cargo.lock` EMPTY (zero Cargo drift since the bump). |
| F7 | PENDING GATE | the SINGLE 0.6.1 OPERATOR-RUN — the PLAIN 10-item checklist at `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9 (verbatim in §5 C3 below) + the STANDING NOTE (if item 1 still fails 100% → the DIAGNOSTIC debug-build fallback, explicit caller authorization required, T1 plan §6). It SUPERSEDES the pending 0.6.0 checklist in one run. |
| F8 | CENSUS CORRECTION (Task-5 4.5b carry-forward — record VERBATIM) | **28 test-bearing files = 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `zoom.rs`, `scaling.rs`)** — superseding the 4.5a report's "19/inconsistency" miscount wording (the 4.5a report file is NOT edited; the prohibition stands). |
| F9 | WAVE-6.1 TEST DELTAS | `src/app/open_with_button_tests.rs` 7 → 16 (+9: the modal-gating / Q open-close / warning-predicate tests on the real-intake harness); `src/app/pill_style_tests.rs` 3 → 5 (+2: the warning-color constant + the color-selector fn); `src/app/esc_close_confirm_tests.rs` 11 → 12 (+1: the generic `non_repeat_key_press_occurred` filter). Census 177 → 189. |
| F10 | WAVE COMMIT CHAIN (chronological, verbatim) | `097d2e4` (report+TODO), `5a6ed36` (global plan), `46d1d22` (0.6.1 bump), `18ed719` (T1 plan), `befcf9e` (D1 fix), `ad73b4e` (T1 mark), `decc537` (T2-4 spec+plan), `3e74a2f` (feature), `85fe6b0` (simplification), `c7fad26` (docs), `c51315a` (T2-4 marks), `1d4d28e`/`12a18d3` (T5 plan+results), `74567db` (adjudication), `71b52bb` (T5 mark). |
| F11 | WAVE IDENTITY | branch `fix/wave-6-1-open-with-feedback`; version 0.6.1 (patch defect-fix round on the 0.6.x line, Wave-4.1/4.2/5.1/5.2 precedent); the 0.6.0 OPERATOR-RUN report (2026-11-13) is the wave's origin; Task 6 is the current cycle; Step 5 (rename/merge/push) and the Step-6 chat close follow. |
| F12 | FILES TOUCHED BY THE WAVE (docs targets to describe, none new) | `src/platform/windows_open_with.rs` (fix + module-doc root-cause note); `src/app/open_with_button.rs` (menu id/key/state API, Q-open path, warning predicate, refresh helper; glyph block moved OUT); `src/app/pill_style.rs` (WARNING_MESSAGE_COLOR + message_text_color + the open-with glyph builder moved IN); `src/app/keyboard_intake.rs` (first-line expiry update + the modal guard + the Q read/dispatch); `src/app/button_hint.rs` (OpenWith caption/tooltip arms); `src/app/floating_controls.rs` (message color + arrow gate); `src/app/zoom_button.rs`/`fit_button.rs`/`fullscreen_button.rs` (click gates); `src/app/esc_close_confirm.rs` (generalized non-repeat filter). NO new source or test files. |

---

## §1 EXECUTOR MODEL

- Work ONLY on: `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md` (the four edit targets) — plus the commit actions in §8. Nothing else.
- Every edit below names its anchor (the current text to find) and the EXACT replacement/insert. Apply them in the listed order per file. Where the anchor text spans a long line, match on the quoted fragment (unique within the file) and replace only that fragment.
- Do NOT renumber or reorder untouched lines; do NOT touch the `<!-- DO NOT DELETE NEXT SECTION -->` blocks; do NOT reformat the rest of any file.
- No harness, no `alpine-vm` MCP call, no build, no tests: this cycle edits Markdown only.

---

## §2 README.md — EXACT EDITS

### R1 — Controls-table row (the open-with row)

- FIND (the table row): `| Open the current file with a system-registered program | — (no hotkey in v1) | the rightmost [Open with...] button (enabled while an image is open with registered programs; opens the program popup above the pill) |`
- REPLACE WITH:
  `| Open the current file with a system-registered program | Q (opens the menu exactly like the button click; pressing Q again closes the open menu) | the rightmost [Open with...] button (enabled while an image is open with registered programs; opens the program popup above the pill) |`

### R2 — Feature 7 bullet (two fragment replacements inside the long bullet)

- R2a — FIND: `The stored list is refreshed at exactly two points — every successful image open and the click that opens the popup (the popup itself reads the stored list; no per-frame enumeration).`
  REPLACE WITH: `The stored list is refreshed at exactly three points — every successful image open, the click that opens the popup, and the `Q` press that opens it (the popup itself reads the stored list; no per-frame enumeration).`
- R2b — FIND: `the button has NO hotkey in v1 (click-only); if the system launch hand-off fails, the pill's message line shows the single static message "Windows could not open this file with the selected program."`
  REPLACE WITH: `the button's hotkey is `Q` — pressing `Q` opens the menu exactly like the click (only while the button is enabled and the pill chrome is visible), the caption `Q` renders under the symbol in the shared 9-pt white-with-black-outline style, and the enabled tooltip reads "Open with... (Q)"; while the menu is open ALL other app functionalities are OFF — arrow navigation/pan, the zoom and fit keys and buttons, the fullscreen toggle, and the ESC-close contract all do nothing — and `ESC` or `Q` again closes the menu (`ESC` closes ONLY the menu: it never arms the close-confirmation popup and never exits fullscreen even when fullscreen is ON); clicking outside the menu dismisses it; if the system launch hand-off fails, the pill's message line shows the single static message "Windows could not open this file with the selected program." in WARNING ORANGE (the open-with failure message is the ONLY warning-colored message; every other message keeps the default text color). Since Wave 6.1 the launch data object is built with the documented `IShellItem::BindToHandler` single-item bind — the 0.6.0 shell-item-array route requested `IID_IDataObject` off-contract and failed on real Windows before the launch hand-off (the D1 root cause, fixed in 0.6.1).`

### R3 — The pill paragraph (two fragment replacements)

- R3a — FIND: `since Wave 4.1 a small hotkey caption is painted beneath the glyph of the six keyboard-distinct buttons — `F11` (fullscreen square), `I` (+), `O` (−), `0` (restore-fit), `W` (fit-width), `H` (fit-height) — while the four arrows and the Open-with button carry no caption (the Open-with button has no hotkey in v1)`
  REPLACE WITH: `since Wave 4.1 a small hotkey caption is painted beneath the glyph of the keyboard-distinct buttons — six since Wave 4.1 (`F11` (fullscreen square), `I` (+), `O` (−), `0` (restore-fit), `W` (fit-width), `H` (fit-height)) and seven since Wave 6.1 (`Q` (Open with...)) — while the four arrows carry no caption`
- R3b — FIND (inside the tooltip list): `"Fit height (H)", "Open with..." (data: `src/app/button_hint.rs`)`
  REPLACE WITH: `"Fit height (H)", "Open with... (Q)" (data: `src/app/button_hint.rs`)`

### R4 — Technology Stack intro sentence (the wave list)

- FIND: `and the Wave-6 "Open with..." menu — all seven v1 features are landed`
- REPLACE WITH: `and the Wave-6 "Open with..." menu, and the Wave-6.1 operator-feedback corrections (the launcher root-cause fix, the `Q` hotkey, the modal menu, and the warning-colored failure message) — all seven v1 features are landed`

### R5 — Technology Stack table (version cell)

- FIND: `Implemented: Cargo package `img-scroller` 0.6.0, edition 2024.`
- REPLACE WITH: `Implemented: Cargo package `img-scroller` 0.6.1, edition 2024.`

### R6 — Project Status paragraph (two fragment edits)

- R6a — FIND: `Pending operator acceptance: the 0.6.0 OPERATOR-RUN (the wave's "Open with..." manual-UX checklist plus the standing regressions — one run, checklist at `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11).`
  REPLACE WITH: `Pending operator acceptance: the 0.6.0 OPERATOR-RUN (the wave's "Open with..." manual-UX checklist plus the standing regressions — one run, checklist at `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11; SUPERSEDED by the single 0.6.1 OPERATOR-RUN below).`
- R6b — APPEND at the end of that same paragraph (after R6a's replaced sentence), one new sentence:
  ` Wave 6.1 landed (branch `fix/wave-6-1-open-with-feedback`, version 0.6.1): the 0.6.0 operator run reported the "Open with..." launcher 100% defective (every selection showed only the failure message) — root-caused as an off-contract `IID_IDataObject` request on a shell item array and fixed by building the launch data object with the documented `IShellItem::BindToHandler(BHID_DataObject)` single-item bind (commit `befcf9e`; the array route removed, zero new crate features, no semantics change); the pill's Open-with button gained the `Q` hotkey (the `Q` caption under the symbol, the "Open with... (Q)" tooltip) and its popup became MODAL while open (all other functionalities OFF; `ESC` or `Q` again closes it — ESC closes ONLY the menu — and outside click dismisses); the launch-failure message now renders in warning orange. Verification: dev checks `ALL CHECKS PASSED` (189 unit tests, 0 failed, across 28 test-bearing files — the corrected breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules), the release executable rebuilt at `dist/windows-x64/img-scroller.exe` (19,645,219 bytes at 0.6.1), and the DLL import audit SET-EQUAL (the recorded 22-DLL set). Pending operator acceptance: the SINGLE 0.6.1 OPERATOR-RUN — it supersedes the pending 0.6.0 checklist in one run (the plain 10-item checklist at `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9); if selections STILL fail 100% on the 0.6.1 exe, the next step is the DIAGNOSTIC debug-build fallback requiring explicit caller authorization.`

---

## §3 `.agent/project-info/tech.md` — EXACT EDITS

### T1 — Status paragraph (append one sentence at the very end of the `## Status` paragraph)

- APPEND (after the final Wave-6 sentence ending `...(mechanism: `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` below).`):
  ` Landed for Wave 6.1 (branch `fix/wave-6-1-open-with-feedback`, version 0.6.1, 2026-11-13): the 0.6.0 operator report's corrections — the launcher root-cause fix (the documented `IShellItem::BindToHandler` data-object route replacing the failed shell-item-array route), the `Q` hotkey with its caption under the symbol, the modal menu (all other functionalities OFF while open; `ESC`/`Q` closes), and the warning-orange launch-failure message (mechanism: `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` below, extended with the Wave-6.1 bullets).`

### T2 — Technology Stack table (version cell)

- FIND: `| Language | Rust, edition 2024 | Cargo package `img-scroller` version 0.6.0 (single binary; tracked `Cargo.lock`). |`
- REPLACE WITH: `| Language | Rust, edition 2024 | Cargo package `img-scroller` version 0.6.1 (single binary; tracked `Cargo.lock`). |`

### T3 — Launcher-mechanism bullet (the FIXED launcher route)

- FIND (inside the `Launcher mechanism:` bullet): `builds the payload `IDataObject` from the CURRENT canonical file (`SHCreateItemFromParsingName` → `SHCreateShellItemArrayFromShellItem`), and hands off via `IAssocHandler::Invoke(&data_object)``
- REPLACE WITH: `builds the payload `IDataObject` from the CURRENT canonical file (`SHCreateItemFromParsingName` → `IShellItem::BindToHandler(None::<&IBindCtx>, &BHID_DataObject)`), and hands off via `IAssocHandler::Invoke(&data_object)`` — then, immediately after that bullet's existing final sentence (`...a successful launch changes nothing.`), APPEND:
  ` Wave 6.1 FIXED this route (commit `befcf9e`): the 0.6.0 mechanism requested `IID_IDataObject` from `SHCreateShellItemArrayFromShellItem`'s shell item array — an off-contract interface request on an object whose documented interface set is `IUnknown` + `IShellItemArray` — so it failed BEFORE `IAssocHandler::Invoke` on EVERY selection, for every handler kind and file type (the operator's 100% D1 failure); the fix binds `BHID_DataObject` on the single-file `IShellItem` (the Microsoft-documented way to obtain an item's data object, verified working end-to-end for handler invocation in independent sources — the T1 plan §0/§1 evidence chain). Zero new `windows`-crate features (`BHID_DataObject`/`IBindCtx` are behind the six already-enabled features); NO semantics change (enumeration content, refresh contract, `launch_index` pairing, state gates, and the failure message are all unchanged); the module doc at `src/platform/windows_open_with.rs:9–15` records the root cause.`

### T4 — Refresh-contract bullet (two-point → three points)

- FIND (the bullet lead): `- Two-point enumeration contract: the stored session list `open_with_programs` (`Vec<RegisteredProgram>` on `ImagesScrollerApp`) is refreshed ONLY (a) at every successful image open (`accept_open_outcome`, before the path move) and (b) on the click frame that opens the popup (`enumerate_when_menu_opens`); the popup reads the STORED list — no per-frame OS enumeration anywhere; entering empty-folder mode clears the list (`src/app/folder_navigation.rs`).`
- REPLACE WITH: `- Refresh contract (three points since Wave 6.1): the stored session list `open_with_programs` (`Vec<RegisteredProgram>` on `ImagesScrollerApp`) is refreshed ONLY (a) at every successful image open (`accept_open_outcome`, before the path move), (b) on the click frame that opens the popup (the click path opens via the `Popup::menu` builder's own click toggle), and (c) on the distinct `Q` press that opens the menu (`open_with_menu_from_q_press` — refresh, then `egui::Popup::open_id`; Wave 6.1); the popup reads the STORED list — no per-frame OS enumeration anywhere; entering empty-folder mode clears the list (`src/app/folder_navigation.rs`).`

### T5 — FOUR NEW Wave-6.1 bullets appended at the END of the `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` section (after the existing `Verification (Wave-6 Task 4)` bullet, before `## Development Environment and Setup`), in this order:

1. `- Wave-6.1 `Q` hotkey (Task 2): `pub(super) const OPEN_WITH_MENU_KEY: egui::Key = egui::Key::Q` (`src/app/open_with_button.rs:33`); the caption `Some("Q")` renders UNDER the symbol through the unchanged shared 9-pt caption paint (WHITE fill + the 8-offset black outline ring; `src/app/button_hint.rs:66` hotkey-reference map — the arrows stay uncaptioned); the enabled tooltip reads `"Open with... (Q)"` (`button_hint.rs:89`); the disabled-state tooltips keep their semantics (the no-programs override still applies only while a file is open with an empty enumeration); the Q-open requires `open_with_menu_should_open_on_q` (a DISTINCT non-repeat press + `open_with_menu_is_available()` + `full_ui_chrome_is_visible()`) and opens programmatically via `refresh_open_with_programs()` + `egui::Popup::open_id` — the click path opens via the `Popup::menu` builder's own click toggle (egui `Popup::menu` TOGGLES popup memory on the click frame, so a shared open helper would toggle-close the click-open); a further distinct `Q` press while open closes the menu via `Popup::close_id`.`
2. `- Wave-6.1 MODAL menu (Task 3): while the menu is open ALL other functionalities are OFF — the early-return guard `handle_modal_open_with_menu_keys` sits in `read_and_dispatch_keystrokes` (`src/app/keyboard_intake.rs`) immediately AFTER the first-line `update_esc_close_popup` (so an armed ESC-close popup still expires on time behind the menu) and BEFORE every other read/dispatch: it closes the menu on a distinct `Q` OR `ESC` press (`Popup::close_id`) and returns — F11, the fullscreen-ON ESC exit, ESC-close arming, zoom, fit, and the arrows are all skipped. The FIVE pill-row click gates (`!open_with_button::open_with_menu_is_open(...)`) suppress the arrow/fullscreen/zoom/fit/open-with-refresh click dispatches (arrows `floating_controls.rs:186`, fullscreen `fullscreen_button.rs:32`, zoom `zoom_button.rs:93`, fit `fit_button.rs:89`, open-with refresh `open_with_button.rs:91`) — egui does NOT consume outside clicks, so explicit gates are required. `ESC` while the menu is open closes ONLY the menu — it never arms the ESC-close popup and never exits fullscreen even when fullscreen is ON. Outside-click dismissal is egui's default `PopupCloseBehavior::CloseOnClick` (no override). The popup identity is the deterministic `open_with_menu_id()` = `Id::unique("open_with_menu_popup")` (`Id::new` is deprecated and non-const in egui 0.37 — the same value every call, fixed-seed hash); the popup builder's gate is `enabled || open_with_menu_is_open(...)` so a fresh enumeration that empties the list cannot produce an invisible modal lock.`
3. `- Wave-6.1 warning color (Task 4): `pill_style::WARNING_MESSAGE_COLOR = egui::Color32::from_rgb(255, 165, 0)` (`src/app/pill_style.rs:38`) — classic warning ORANGE, high-luminance and readable on the pill's translucent dark frame; the pure predicate `message_line_is_warning(message)` (`src/app/open_with_button.rs:135–137`) matches ONLY `message == Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)` (content equality — the failure text is a unique constant), and `pill_style::message_text_color(is_warning, pill_ui)` applies the warning color ONLY for that message; the unsupported-file failure and every cleared state keep the theme's default text color; placement unchanged (the pill's message line). NO app-state field exists — the predicate is stateless (app.rs stays untouched).`
4. `- Wave-6.1 census correction + verification: the corrected census breakdown (the Task-5 4.5b carry-forward, verbatim) — **28 test-bearing files = 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `zoom.rs`, `scaling.rs`)** — superseding the 4.5a report's "19/inconsistency" miscount wording (the 4.5a report file is NOT edited). Wave-6.1 test deltas: `src/app/open_with_button_tests.rs` 7 → 16 (+9: the modal-gating, Q open/close, and warning-predicate tests driven through the REAL intake on the long-lived-Context harness), `src/app/pill_style_tests.rs` 3 → 5 (+2), `src/app/esc_close_confirm_tests.rs` 11 → 12 (+1 the generic `non_repeat_key_press_occurred` filter the ESC filter delegates to). Verification (Task 5, adjudicated VERIFIED): dev checks `ALL CHECKS PASSED` — 189 unit tests, 0 failed, across 28 test-bearing files (census: `## Testing`); the windows-target compile (`cargo check --locked --target x86_64-pc-windows-gnu`) exit 0 carrying ONLY the pre-existing benign dead-code warning on `compare_entry_names`; the 0.6.1 exe rebuilt at `dist/windows-x64/img-scroller.exe` = 19,645,219 bytes (the 0.6.0 artifact 19,647,219 B overwritten atomically); the DLL import audit SET-EQUAL to the recorded 22-DLL constant (`BindToHandler` is a virtual call through the already-imported shell COM surface; the removed `SHCreateShellItemArrayFromShellItem` and the kept `SHAssocEnumHandlers`/`SHCreateItemFromParsingName` are all shell32 exports — the deciding evidence, never an assumption); NO dependency changes (`git diff 46d1d22..HEAD -- Cargo.toml Cargo.lock` EMPTY). The pending 0.6.1 OPERATOR-RUN checklist lives verbatim (PLAIN form) in `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9.`

### T6 — The Wave-6 verification bullet inside the section (pending-gate pointer only)

- FIND: `The pending 0.6.0 OPERATOR-RUN checklist lives verbatim in `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11.`
- REPLACE WITH: `The pending 0.6.0 OPERATOR-RUN checklist lives verbatim in `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11 (SUPERSEDED by the single 0.6.1 OPERATOR-RUN — the Wave-6.1 bullet above).`

### T7 — Stub-path bullet (test count)

- FIND: `the availability/tooltip/failure seams stay container-testable (7 tests in `src/app/open_with_button_tests.rs`, 6 in `src/open_with_listing/tests.rs`)`
- REPLACE WITH: `the availability/tooltip/failure seams stay container-testable (16 tests in `src/app/open_with_button_tests.rs` since Wave 6.1, 6 in `src/open_with_listing/tests.rs`)`

### T8 — `## Testing` census paragraph (three per-file counts + the grand total)

- T8a — FIND: `src/app/esc_close_confirm_tests.rs` — 11;`  REPLACE WITH: `src/app/esc_close_confirm_tests.rs` — 12 (Wave-6.1: +1 the generic `non_repeat_key_press_occurred` filter test);`
- T8b — FIND: `src/app/open_with_button_tests.rs` — 7 (Wave-6 Task 3: the availability, tooltip, and launch-failure seams);`  REPLACE WITH: `src/app/open_with_button_tests.rs` — 16 (Wave-6 Task 3: the availability, tooltip, and launch-failure seams; Wave-6.1: 7 → 16 — the modal-gating, Q open/close, and warning-predicate tests through the REAL intake);`
- T8c — FIND: `src/app/pill_style_tests.rs` — 3;`  REPLACE WITH: `src/app/pill_style_tests.rs` — 5 (Wave-6.1: +2 — the `WARNING_MESSAGE_COLOR` constant and the `message_text_color` selector);`
- T8d — FIND: `Grand total 177 across 28 test files (the 164 pre-Wave-6 tests unchanged + 13 Wave-6 tests: +6 the open-with listing seam, +7 the open-with button seams).`
  REPLACE WITH: `Grand total 189 across 28 test-bearing files — the CORRECTED breakdown (Wave-6.1, recorded verbatim): **28 test-bearing files = 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `zoom.rs`, `scaling.rs`)** — the 177 pre-Wave-6.1 tests unchanged + 12 Wave-6.1 tests (+9 the open-with modal/Q/warning tests, +2 the pill_style warning-color tests, +1 the generic non-repeat filter test).`

NOTE: the historical Wave-6 numbers inside the Wave-6 verification bullet (T6's anchor bullet: `177 unit tests, 0 failed, across 28 test files`) stay UNCHANGED — they were true for Wave 6 and are now a historical record.

---

## §4 `.agent/project-info/architecture.md` — EXACT EDITS

### A1 — Status paragraph (append one sentence at the very end)

- APPEND (after the Wave-6 sentence ending `...and the DLL import audit stayed SET-EQUAL 22.`):
  ` Wave 6.1 (branch `fix/wave-6-1-open-with-feedback`, version 0.6.1, 2026-11-13) corrected the operator-reported launcher defect and landed the Q/modal/warning feedback on the same architecture (the Fast Open-with bullet and the decisions table below); dev checks ended `ALL CHECKS PASSED` (189 unit tests), the exe was rebuilt at `dist/windows-x64/img-scroller.exe` (19,645,219 bytes at 0.6.1), and the DLL import audit stayed SET-EQUAL 22.`

### A2 — The `Fast "Open with..." Service — LANDED (Wave 6)` bullet (four fragment edits)

- A2a — FIND: `invokes the selected handler with `IAssocHandler::Invoke` over an `IDataObject` built from the current canonical file (`SHCreateItemFromParsingName` → `SHCreateShellItemArrayFromShellItem`);`
  REPLACE WITH: `invokes the selected handler with `IAssocHandler::Invoke` over an `IDataObject` built from the current canonical file (`SHCreateItemFromParsingName` → `IShellItem::BindToHandler(None::<&IBindCtx>, &BHID_DataObject)` — the Wave-6.1 fix, commit `befcf9e`: the removed array route requested `IID_IDataObject` off-contract from a shell item array — an object whose documented interface set is only `IUnknown` + `IShellItemArray` — and failed BEFORE `Invoke` on EVERY selection, the 100% D1 failure; zero new crate features, no semantics change);`
- A2b — FIND: `the pill's rightmost 11th button (painted square-with-emerging-arrow glyph, 36×32, NO hotkey in v1) opens an upward `egui::Popup::menu``
  REPLACE WITH: `the pill's rightmost 11th button (painted square-with-emerging-arrow glyph, 36×32; the `Q` hotkey since Wave 6.1 — the caption `Some("Q")` under the symbol, the enabled tooltip `"Open with... (Q)"`) opens an upward `egui::Popup::menu``
- A2c — FIND: `the stored list refreshes ONLY at the two decided points — every successful open and the popup-opening click (the popup reads the STORED list; no per-frame enumeration);`
  REPLACE WITH: `the stored list refreshes ONLY at the three decided points — every successful open, the popup-opening click, and the `Q`-open press (refresh, then `Popup::open_id`; Wave 6.1) (the popup reads the STORED list; no per-frame enumeration);`
- A2d — APPEND at the end of the same bullet (after its `Mechanism detail: [tech.md](tech.md) → `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)`.` sentence):
  ` Wave 6.1: while the menu is open ALL other functionalities are OFF — the intake's early-return modal guard (`handle_modal_open_with_menu_keys`, `src/app/keyboard_intake.rs`) skips every other read/dispatch (the ESC-close expiry update keeps running first), the FIVE pill-row click gates suppress the arrow/fullscreen/zoom/fit/open-with-refresh clicks (egui does not consume outside clicks), `ESC` closes ONLY the menu (never arms the close popup, never exits fullscreen), a further `Q` closes it, and outside click dismisses via egui's default `CloseOnClick`; the launch-failure message renders in the warning orange `pill_style::WARNING_MESSAGE_COLOR` (`Color32::from_rgb(255, 165, 0)`) via the pure predicate `message_line_is_warning` — the ONLY warning-colored message.`

### A3 — The `src/app/open_with_button.rs` module bullet (Wave-6.1 facts + test count)

- FIND (inside the bullet): `NO hotkey caption in v1; tooltip "Open with..."); the enabled click re-enumerates via the facade`
  REPLACE WITH: `the `Q` hotkey since Wave 6.1 (`OPEN_WITH_MENU_KEY = Key::Q` at `:33`; the caption `Some("Q")`; the tooltip `"Open with... (Q)"`; the deterministic popup identity `open_with_menu_id()` = `Id::unique("open_with_menu_popup")`; the ONE modal-state query `open_with_menu_is_open` consumed by the intake guard AND the five row click gates; the Q-open path `open_with_menu_should_open_on_q` + `open_with_menu_from_q_press` = refresh + `Popup::open_id`; the pure warning predicate `message_line_is_warning` matching ONLY the launch-failure constant); the enabled click re-enumerates via the facade`
- FIND (the same bullet's test-count tail): `7 tests in `src/app/open_with_button_tests.rs``
  REPLACE WITH: `16 tests in `src/app/open_with_button_tests.rs` (7 Wave-6 + 9 Wave-6.1: the modal-gating, Q open/close, and warning-predicate tests)`

### A4 — Path 5 in `## Navigation Critical Paths` (refresh points)

- FIND: `the stored list refreshed at the two decided points (every successful open; this popup-opening click)`
- REPLACE WITH: `the stored list refreshed at the three decided points (every successful open; this popup-opening click; the `Q`-open press — Wave 6.1)`

### A5 — `## Data and State` (the `open_with_programs` clause)

- FIND: `refreshed ONLY at every successful open and at the popup-opening click, cleared when empty-folder mode is entered`
- REPLACE WITH: `refreshed ONLY at every successful open, at the popup-opening click, and at the `Q`-open press (Wave 6.1), cleared when empty-folder mode is entered`

### A6 — The Viewer-UI bullet (caption count)

- FIND: `since Wave 4.1 the six keyboard-distinct buttons carry hotkey captions while all eleven carry hover tooltips`
- REPLACE WITH: `since Wave 4.1 the keyboard-distinct buttons carry hotkey captions (six since Wave 4.1; seven since Wave 6.1 — the Open-with button gained `Q`) while all eleven carry hover tooltips`

### A7 — Decisions table, `Pill affordances` row

- FIND: `| Pill affordances: hotkey captions under the six keyboard-distinct buttons; hover tooltips on all eleven buttons (enabled + disabled) | landed (Wave 4.1) | 2026-11-02 operator correction (brief addendum) |`
- REPLACE WITH: `| Pill affordances: hotkey captions under the keyboard-distinct buttons (six since Wave 4.1; seven since Wave 6.1 — the Open-with button gained `Q`); hover tooltips on all eleven buttons (enabled + disabled) | landed (Wave 4.1; the Open-with `Q` caption Wave 6.1) | 2026-11-02 operator correction (brief addendum) |`

### A8 — Decisions table, `Open-with mechanism` row (append the Wave-6.1 clause)

- In the row's Decision cell, APPEND after `...disabled + "No programs registered for this file type" on an empty list, NO hotkey in v1`:
  `; Wave 6.1: the launcher data object built via the documented `IShellItem::BindToHandler(BHID_DataObject)` single-item bind (the off-contract array→`IID_IDataObject` route removed — the D1 root cause, commit `befcf9e`), the `Q` hotkey (caption under the symbol; the Q-open refresh point), the MODAL menu (all other functionalities OFF while open; `ESC`/`Q` closes; the five row click gates; outside-click `CloseOnClick`), and the warning-orange failure message (`WARNING_MESSAGE_COLOR`, predicate-only)`
  (the `NO hotkey in v1` clause stays — it is the Wave-6 record; the appended clause supersedes it explicitly.)
- In the same row's Status cell, FIND `landed (Wave 6)` REPLACE WITH `landed (Wave 6; the Q/modal/warning + launcher-route corrections Wave 6.1)`.

### A9 — Project Paths (version)

- FIND: `Cargo.toml` (package `img-scroller` 0.6.0, edition 2024)`
- REPLACE WITH: `Cargo.toml` (package `img-scroller` 0.6.1, edition 2024)`

---

## §5 `.agent/project-info/context.md` — EXACT EDITS

### C1 — `## Current Work Focus` rewrite (the 2026-11-13 paragraph's opening)

- REPLACE ONLY the paragraph's opening (from `Dated 2026-11-13.` up to and including `...must be presented plainly**.`) with the text below; KEEP everything from `Wave 6 (below) is COMPLETE on the agent side.` onward byte-identical:
  `Dated 2026-11-13. **Wave 6.1 (`.agent/todos/20261113/20261113-todo-1.md`) is agent-side COMPLETE — Tasks 1–5 all `[DONE]` except the Task-5 manual-UX sub-item, which stays `[ ]` as the user's gate; Task 6 (documentation) is the current cycle; the Step-5 mechanics (todo rename → `-DONE`, fast-forward merge into `main`, push `main` → `origin` ONLY, branch delete) are PENDING**. The wave fixed the 0.6.0 operator report's D1 defect: the launched data object was built via `SHCreateShellItemArrayFromShellItem` requesting `IID_IDataObject` — off-contract for a shell item array (its documented interface set is only `IUnknown` + `IShellItemArray`) — so it failed BEFORE `IAssocHandler::Invoke` on EVERY selection (the 100% failure); the fix (commit `befcf9e`) builds it with the documented single-item bind `IShellItem::BindToHandler(None::<&IBindCtx>, &BHID_DataObject)`, zero new `windows`-crate features, no semantics change; the DIAGNOSTIC debug-build fallback remains ARMED (explicit caller authorization required) if the operator's re-test still fails 100%. The pill's Open-with button gained the `Q` hotkey (the caption `Some("Q")` under the symbol, 9-pt white + black outline; the tooltip "Open with... (Q)"; Q opens programmatically after refreshing the stored list — the refresh contract gains the Q-open point — and Q again closes it), the menu is MODAL while open (the intake's first-guard skips every other read/dispatch while the ESC-close expiry keeps running; the five pill-row click gates suppress arrow/fullscreen/zoom/fit/open-with-refresh clicks; ESC closes ONLY the menu — it never arms the close popup and never exits fullscreen; outside click dismisses), and the launch-failure message renders in warning orange (`pill_style::WARNING_MESSAGE_COLOR` = `Color32::from_rgb(255, 165, 0)`; the pure predicate `message_line_is_warning` matches ONLY `OPEN_WITH_LAUNCH_FAILURE_MESSAGE`). Verification (Task 5, adjudicated VERIFIED): dev-checks `ALL CHECKS PASSED` (189/0, 28 test-bearing files — the corrected breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules, verbatim in the Recent-Changes entry); the windows-target compile exit 0 (only the pre-existing benign dead-code warning); the 0.6.1 rebuild → exe 19,645,219 bytes; the DLL audit SET-EQUAL 22. PENDING ACCEPTANCE GATE: the SINGLE 0.6.1 OPERATOR-RUN (Next-Steps #1 — the PLAIN 10-item checklist at `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9 + the diagnostic-fallback note); it supersedes the pending 0.6.0 checklist in one run. Wave 6 (below) is COMPLETE on the agent side.`

### C2 — `## Recent Changes`: ONE new entry prepended at the TOP of the numbered list

- INSERT as the new first list item (keeping the file's `1.` list style), verbatim:
  `1. 2026-11-13: **Wave 6.1 Tasks 1–5 landed — the D1 launcher root cause + fix, the `Q` hotkey, the MODAL menu, and the warning-orange failure message (this step: Task 6 documentation).** Branch `fix/wave-6-1-open-with-feedback`, version 0.6.1. Wave commit chain (verbatim, chronological): `097d2e4` (report+TODO), `5a6ed36` (global plan), `46d1d22` (0.6.1 bump), `18ed719` (T1 plan), `befcf9e` (D1 fix), `ad73b4e` (T1 mark), `decc537` (T2-4 spec+plan), `3e74a2f` (feature), `85fe6b0` (simplification), `c7fad26` (docs), `c51315a` (T2-4 marks), `1d4d28e`/`12a18d3` (T5 plan+results), `74567db` (adjudication), `71b52bb` (T5 mark). (Task 1) D1 ROOT CAUSE: the launched data object was built via `SHCreateShellItemArrayFromShellItem` requesting `IID_IDataObject` — off-contract for a shell item array (only `IUnknown` + `IShellItemArray`) → it failed BEFORE `IAssocHandler::Invoke` on EVERY selection (the operator's 100% failure; suspects B/C/D evidence-downgraded in the T1 plan §0); FIX (`befcf9e`): the documented single-item bind `IShellItem::BindToHandler(None::<&IBindCtx>, &BHID_DataObject)` in `src/platform/windows_open_with.rs::file_data_object`, the array route removed, ZERO new crate features, NO semantics change; the DIAGNOSTIC debug-build fallback remains armed (caller authorization) if the operator's re-test still fails 100%. (Tasks 2+3+4, commits `3e74a2f` + `85fe6b0`) the `Q` hotkey — `OPEN_WITH_MENU_KEY = Key::Q` (`open_with_button.rs:33`), the caption `Some("Q")` UNDER the symbol (9-pt white + black outline via the unchanged shared caption paint), the tooltip "Open with... (Q)", the Q-open = distinct press + `open_with_menu_is_available()` + `full_ui_chrome_is_visible()` then refresh + `Popup::open_id` (the two-point contract gains the Q-open refresh point — three points now), Q again closes via `Popup::close_id`; the MODAL menu — the first-guard `handle_modal_open_with_menu_keys` in `read_and_dispatch_keystrokes` (after the first-line `update_esc_close_popup`) skips F11/ESC-exit-fullscreen/ESC-close-arm/zoom/fit/arrows, the FIVE pill-row click gates suppress arrow/fullscreen/zoom/fit/open-with-refresh clicks, ESC-while-open closes ONLY the menu, outside click dismisses (egui default `CloseOnClick`); the warning color — `pill_style::WARNING_MESSAGE_COLOR` = orange `Color32::from_rgb(255, 165, 0)` (`pill_style.rs:38`), the pure predicate `message_line_is_warning` matches ONLY `OPEN_WITH_LAUNCH_FAILURE_MESSAGE`, other messages keep the default color, placement unchanged. (Task 5, VERIFIED) dev-checks `ALL CHECKS PASSED` (189/0); windows-target compile exit 0 (the pre-existing benign dead-code warning); the 0.6.1 exe rebuilt `dist/windows-x64/img-scroller.exe` = 19,645,219 bytes (the 0.6.0 artifact 19,647,219 B overwritten atomically); DLL audit SET-EQUAL 22; `Cargo.toml`/`Cargo.lock` untouched since the bump. CENSUS CORRECTION (the Task-5 4.5b carry-forward, VERBATIM): **28 test-bearing files = 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `zoom.rs`, `scaling.rs`)** — superseding the 4.5a report's "19/inconsistency" miscount wording (the 4.5a report file is NOT edited; the prohibition stands). PENDING ACCEPTANCE GATE: the SINGLE 0.6.1 OPERATOR-RUN (Next-Steps #1 — the PLAIN 10-item checklist at `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9 + the diagnostic-fallback note); it supersedes the pending 0.6.0 checklist in one run. NEXT after the gate closes: Step 5 (todo rename/merge/push), then Wave 7 = `.agent/todos/20261029/20261029-todo-9.md` in a NEW chat.`

### C3 — `## Immediate Next Steps` rewrite (items 1–3)

- REPLACE the ENTIRE numbered list (items 1–3) with the three items below. Item 1 carries the PLAIN checklist VERBATIM from `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9 (copy the ten items and the STANDING NOTE exactly as written there):

  1. **Pending 0.6.1 Wave-6.1 OPERATOR-RUN gate — the user's run on a real Windows 10/11 machine**, running the freshly rebuilt 0.6.1 exe at `dist\windows-x64\img-scroller.exe`, with an image open in a folder whose file type has registered programs (the PLAIN checklist, verbatim from `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9): (1) D1 FIXED — select a program in the "Open with..." list: the CURRENT file opens in that program (no failure message). (2) Press `Q`: the "Open with..." menu opens (when programs exist for the file type). (3) While the menu is open, press other keys and click other buttons (arrows, zoom, fit, fullscreen): they all do NOTHING. (4) Press `ESC` while the menu is open: the menu closes, and the "Press ESC again to close" app-exit popup does NOT appear. (5) Press `Q` again: the menu closes. (6) Click outside the menu: the menu dismisses. (7) `Q` stays inert when: no programs are registered for the type, in empty-folder mode, and in fullscreen. (8) If a failed launch is reproducible: the failure message shows in WARNING (orange) color on the pill's message line. (9) The `Q` caption renders under the open-with button symbol (white, thin black outline). (10) Regression block — previous functionalities still work: arrow navigation and folder navigation; zoom in/out/restore (`I`/`O`/`0`, Shift+'+' / Shift+'−'); fit width/height (`W`/`H`); fullscreen (`F11`, ESC exit); double-ESC close contract; pan at zoomed image edges; navigation name order; opening files. STANDING NOTE (do not skip): if item 1 STILL fails 100% of selections on the 0.6.1 exe → the next step is the DIAGNOSTIC debug-build fallback (a temporary DEBUG-ONLY build showing per-selection invoke result codes), which REQUIRES explicit caller authorization before any work — plan `.kilo/plans/20261113-wave6-1-task1-launcher-fix-implementation-plan.md` §6; it is never an assumption and never ships in a release artifact. The 0.6.0 gate is SUPERSEDED by this single 0.6.1 run; one run, one gate.
  2. **Wave-6.1 Step-5 mechanics are PENDING** (after Task 6's 4.6 mark): the TODO renames to `-DONE` (pure rename), a tmp sweep runs, `main` is checked out, the branch `fix/wave-6-1-open-with-feedback` is fast-forward merged into `main` and verified and deleted, and `main` is pushed → `origin` ONLY; the Step-6 closing context.md touch and the PLAIN checklist presented in the chat follow.
  3. **NEXT SESSION — Wave 7 (`.agent/todos/20261029/20261029-todo-9.md`, the x64 AND x86 packaging matrix) runs in a NEW chat after Wave 6.1 closes at Step 5**; the 0.6.1 gate (item 1) stays the user's responsibility and closes when the user reports PASS; the parked future-ideas list stays OUT of scope until scheduled.

### C4 — `## Current Project State` (three micro-edits)

- C4a — INSERT as the new FIRST bullet of the section, verbatim:
  `- **Wave 6.1 agent-side COMPLETE on branch `fix/wave-6-1-open-with-feedback` (version 0.6.1; the Step-5 merge/push PENDING)**: the D1 launcher root cause fixed — the launch data object built via the documented `IShellItem::BindToHandler(BHID_DataObject)` single-item bind replacing the off-contract `SHCreateShellItemArrayFromShellItem` → `IID_IDataObject` route (commit `befcf9e`; zero new `windows` features; no semantics change; the DIAGNOSTIC debug-build fallback armed); the `Q` hotkey + the MODAL menu + the warning-orange failure message landed (commits `3e74a2f`/`85fe6b0`); census 189 unit tests / 28 test-bearing files (19 `*_tests.rs` + 4 `tests.rs` + 5 in-file); dev-checks `ALL CHECKS PASSED` (189/0); exe rebuilt 19,645,219 bytes (0.6.1; the 0.6.0 artifact 19,647,219 B overwritten atomically); DLL audit set-equal 22; the pending 0.6.1 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1).`
- C4b — In the `Release artifact:` bullet, FIND: `gitignored `dist/windows-x64/img-scroller.exe` (19,647,219 bytes, 0.6.0 Wave-6 rebuild; the 0.5.2 artifact 19,605,828 B overwritten atomically)`
  REPLACE WITH: `gitignored `dist/windows-x64/img-scroller.exe` (19,645,219 bytes, 0.6.1 Wave-6.1 rebuild; the 0.6.0 artifact 19,647,219 B overwritten atomically)`
- C4c — In the `TODO state:` bullet, INSERT as the new first TODO sentence (before the existing `Wave 6 LANDED...` / `20261029-todo-8-DONE.md` sentence), verbatim:
  `**Wave 6.1**: `.agent/todos/20261113/20261113-todo-1.md` — Tasks 1–5 marked `[DONE]`; the Task-5 manual-UX (OPERATOR-RUN) sub-item STAYS `[ ]` (the user's 0.6.1 gate, Next-Steps #1); Task 6 (documentation) is the current cycle (this step); the rename/merge/push is pending at Step 5. `

---

## §6 VERIFY-ONLY FILES (expected NO-EDIT verdicts — READ, then record the verdict; do NOT edit)

1. `.agent/project-structure.md` — Wave 6.1 added NO new files and NO new test modules (F12); the map's existing descriptions are at a generic-enough abstraction (e.g. `pill_style.rs` = "the shared pill-styling primitives", `open_with_listing.rs` = "runtime open-with enumeration + launcher seam"). EXPECTED VERDICT: ACCURATE — NO edit. If any line is found FACTUALLY WRONG (a description contradicting the landed code), STOP and return the discrepancy to the caller instead of editing.
2. `.agent/project-info/brief.md` — `### Fast "Open with..." Menu` already mandates "Selecting a program opens the current file" (the wave CORRECTS the landed behavior to MATCH the brief — no addendum, the Wave-5.2 item-8 precedent is not repeated; nothing in the wave adds a NEW requirement beyond the brief's text). EXPECTED VERDICT: NO addendum — NO edit.
3. `docs/build-windows.md` — the Wave-6.1 DLL audit was SET-EQUAL to the recorded 22-DLL list (F6); the "Observed DLL import list" block is unchanged. EXPECTED VERDICT: NO OBSERVED-list edit — NO edit.

Record each verdict in the completion summary (what was verified, what was NOT edited).

---

## §7 POST-EDIT COHERENCE SWEEP (host, bash tool, single `rg` commands — every check must match; any miss → fix per this plan, then re-sweep)

1. `rg -n "no hotkey in v1" README.md .agent/project-info/tech.md .agent/project-info/architecture.md` → ZERO matches (case-sensitive lowercase pattern; the decisions-table A8 cell intentionally KEEPS its historical UPPERCASE `NO hotkey in v1` clause inside the Wave-6 record — it must not be flagged by this check and must not be removed).
2. `rg -n "two points|two decided points|Two-point" README.md .agent/project-info/tech.md .agent/project-info/architecture.md` → ZERO matches in the three files (context.md historical Wave-6 entries are EXCLUDED from this check).
3. `rg -n "0\.6\.0, edition 2024" README.md .agent/project-info/tech.md .agent/project-info/architecture.md` → ZERO matches (the stack-table cells and the Cargo paths line carry 0.6.1; historical wave-narrative sentences that name 0.6.0 as a PAST version are acceptable — this pattern targets the CURRENT-state cells only).
4. `rg -n "version 0\.6\.0 \(" .agent/project-info/tech.md` → ZERO matches (the tech stack table's version cell carries 0.6.1 after T2).
5. `rg -n "BindToHandler" README.md .agent/project-info/tech.md .agent/project-info/architecture.md` → matches present in all three (the fixed launcher route is recorded; README R2b/R6b, tech T3, arch A2a).
6. `rg -n "WARNING_MESSAGE_COLOR|255, 165, 0" README.md .agent/project-info/tech.md .agent/project-info/architecture.md` → matches present (README R2b/R6b, tech T5-bullet-3, arch A2d).
7. `rg -n "19,645,219" .agent/project-info/tech.md .agent/project-info/architecture.md` → matches present (tech T5-bullet-4, arch A1).
8. `rg -n "OPEN_WITH_MENU_KEY|handle_modal_open_with_menu_keys|open_with_menu_id" .agent/project-info/tech.md` → matches present (the Q/modal mechanism bullets).

---

## §8 COMMIT PROTOCOL (before EACH commit: read `.gitignore` + run `git status`; stage ONLY the listed files; NO gitignored path staged — `dist/`, `logs/`, `target/`, `*.log` are gitignored and never force-added)

1. Commit 1 (cycle start): stage ONLY `.kilo/plans/20261113-wave6-1-task6-documentation-plan.md` (this plan file, created by the architector). Message: `docs: add wave6.1 task6 documentation plan`. Re-run `git status` → clean tree.
2. Apply §2–§5 edits, then §6 read-only verdicts, then §7 sweep.
3. Commit 2: stage ONLY `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`. Message: `docs: wave6.1 task6 documentation`. Re-run `git status` → clean tree (all four docs files staged together is the single-purpose documentation commit — no other file may join).
4. NO push (Step-5-restricted), NO merge, NO tag, NO CHANGELOG.

---

## §9 SCOPE BOUNDARIES (HARD)

- Edit targets: EXACTLY the four docs files in §2–§5. NO other file is created or modified (the verify-only files in §6 are read-only this cycle).
- NO `CHANGELOG.md`. NO edits to existing `.kilo/plans/*` files (including the 4.5a report flagged in F8 — its miscount wording stays, superseded in prose only). NO application-code edits, NO `Cargo.toml`/`Cargo.lock`, NO TODO marks, NO merge, NO push.
- NO new sections in the docs files beyond what §2–§5 prescribe (tech.md's T5 adds bullets INSIDE the existing Wave-6 section — no new `##` section, so the Contents list is UNCHANGED).
- Ambiguity or anchor-not-found at any edit → STOP and return the exact anchor/discrepancy to the caller. NEVER improvise replacement text.

---

## §10 REVIEWER / ADHERENCE HOOKS + FOLLOW-THROUGH (NOT executed by this plan)

- 4.3 code-reviewer: verify every §0 fact (F1–F12) appears correctly in the edited files; verify no fact contradicts the pinned code anchors (`windows_open_with.rs:9–15`/`file_data_object`, `open_with_button.rs:33`/`:91`/`:135–137`, `pill_style.rs:38`, `button_hint.rs:66`/`:89`, `keyboard_intake.rs:35–38`/`:136–144`/`:179`, the five gate sites F4 lists). code-simplifier: expected NO-CHANGE verdicts (docs-only; the edits are prescribed verbatim).
- 4.4 docs-specialist (guides check): confirm the four files' internal cross-links still resolve (the tech.md section anchor `#open-with-enumeration-and-launcher-mechanism-implemented-wave-6` is UNCHANGED — no heading was renamed; the checklist pointer `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9 exists).
- 4.5b architector adherence: diff the landed docs against this plan's prescriptions (anchor coverage + fact fidelity).
- 4.6 implementer (separate step): mark TODO Task 6 `[DONE]` (sub-item `[]` → `[x]`; preserve the rest of the file) — commit per the workflow.
- Wave close (Step 6, Planner): present the §9 PLAIN checklist in the chat verbatim; Step 5 mechanics follow.

---

## §11 TODO TASK-6 CONFORMANCE MAP (verbatim comparison — each required record → the sites that carry it)

| TODO Task 6 requirement | Carried by |
| --- | --- |
| the launcher root cause + fix | README R2b/R6b; tech T3 (+ module-doc pointer); arch A2a/A8; context C1/C2/C4a |
| the `Q` hotkey | README R1/R2b/R3/R6b; tech T5-bullet-1; arch A2b/A3/A6/A7/A8; context C1/C2 |
| the modal menu behavior | README R2b/R6b; tech T5-bullet-2; arch A2d; context C1/C2 |
| the warning color | README R2b/R6b; tech T5-bullet-3; arch A2d/A8; context C1/C2 |
| the 0.6.1 state | README R5/R6b; tech T1/T2/T5-bullet-4/T8; arch A1/A9; context C1/C2/C3/C4 |
| remaining open items | the 0.6.1 OPERATOR-RUN gate + the diagnostic-fallback fallback note: README R6b; tech T5-bullet-4; context C1/C2/C3 (#1 + the STANDING NOTE verbatim); Wave 7 next: context C3 (#3) |
| the census-correction carry-forward (caller-pinned, verbatim) | tech T5-bullet-4 + T8d; context C1/C2 |

---

## §12 SOURCES

- `.agent/todos/20261113/20261113-todo-1.md` (Task 6 verbatim + the wave execution notes).
- `.kilo/plans/20261113-wave6-1-open-with-feedback.md` (Cycle D names this plan file).
- `.kilo/plans/20261113-wave6-1-task1-launcher-fix-implementation-plan.md` (§0 root-cause evidence; §1 the fix; §6 the diagnostic-fallback trigger).
- `.kilo/plans/20261113-wave6-1-tasks2-4-ui-feedback-implementation-plan.md` (the Q/modal/warning mechanism + decisions D1–D12; the census pin).
- `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` (§9 the PLAIN checklist — copied VERBATIM into context C3; §0 the settled census) and `.kilo/plans/20261113-wave6-1-task5-verification-results.md` (the observed values F6).
- Landed docs read this session: `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`, `.agent/project-info/brief.md`, `.agent/project-structure.md`, `docs/build-windows.md`.
- Landed code verified this session at HEAD `71b52bb`: `src/platform/windows_open_with.rs`, `src/app/pill_style.rs`, `src/app/open_with_button.rs` (+ the Task-5 sweep anchors on record).
