# Wave 6 — Task 5 (Cycle D) Documentation Plan: "Open with..." Menu

- Source TODO (scope of THIS step): `.agent/todos/20261029/20261029-todo-8.md` → `### Task 5: Documentation updates` (1 sub-item, verbatim below).
- Global plan: `.kilo/plans/20261112-wave6-open-with.md` (Wave 6). Cycle-D 4.1b step: THIS plan file.
- Executor of 4.2: the docs-specialist. This plan encodes EXACTLY what each doc file gains/changes — section names, placement, and concrete content. Every item below is FINAL; no redesign, no extra sweeps, no out-of-scope fixes.

## TODO Task 5 verbatim (scope of this step)

> - [ ] `README.md`, `tech.md`, `architecture.md`, `context.md`: record the enumeration mechanism chosen, constraints, remaining open items.

Sub-item stays `[ ]` until the 4.6 step marks it (NOT this step's scope).

---

## Baseline facts (pinned; all verified at plan time)

| Fact | Value | Source |
| --- | --- | --- |
| Branch / HEAD | `feat/wave-6-open-with` @ `6b6a769` (Task-4 mark), tree clean | `git status`, `git log --oneline -25` |
| Version | 0.6.0 (`Cargo.toml:3`) | `Cargo.toml` |
| Census | **177 unit tests / 28 test files** (pre-wave 164/26; +6 `src/open_with_listing/tests.rs`, +7 `src/app/open_with_button_tests.rs`) | Task-4 verification |
| Last dev-checks | `ALL CHECKS PASSED` (177 passed; 0 failed) | Task-4 verification |
| Exe | `dist/windows-x64/img-scroller.exe` = **19,647,219 bytes** (0.6.0; the 0.5.2 artifact 19,605,828 B overwritten atomically) | Task-4 verification; gitignored `dist/` |
| DLL audit | **SET-EQUAL 22** (the recorded 22-DLL constant; the new Wave-6 entry points are shell32/combase exports already covered) | Task-4 verification |
| `windows` crate features | EXACTLY six, unchanged since Wave 5.2 (`Win32_Foundation`, `Win32_Graphics_Imaging`, `Win32_System_Com`, `Win32_UI_Shell`, `Win32_UI_Shell_Common`, `Win32_UI_WindowsAndMessaging`) | `Cargo.toml:18–25` |
| Dependency drift this wave | `git diff f0b2274..HEAD -- Cargo.toml Cargo.lock` = EMPTY (only the version bump) | verified at plan time |
| Operating artifact note | a one-word commit-order transposition stands in `.kilo/plans/20261112-wave6-task4-verification-plan.md` **Step 0 item 3** (the expected top-5 chain lists `caa1aa0` BEFORE `c5cb56b`; the actual order is `c5cb56b` then `caa1aa0` — confirmed via `git log` at plan time). The document's own **Step 6 full-chain check** (which lists the correct order) SUPERSEDES it. **Existing plan files must NOT be edited** — this note is recorded in `context.md` only (§4 C2). | `git log`, task context |

---

## Canonical Wave-6 fact sheet (the facts every doc records; verified against `src/` at plan time)

1. **Enumeration mechanism**: `SHAssocEnumHandlers` queried with the current file's extension (`.ext` null-terminated PCWSTR) with `ASSOC_FILTER_NONE` (NO app-side handler filter — **UWP/Store apps INCLUDED by design**) → the returned `IEnumAssocHandlers` is drained one `Next` at a time → each `IAssocHandler::GetUIName` (freed with `CoTaskMemFree`) yields the raw display-name list, fed through the pure seam `programs_from_handler_names` (`src/open_with_listing.rs`). COM init reuses the shared `ensure_com_initialized` discipline from `windows.rs`. Every failure path returns the empty/default value, never panics.
2. **Zero dependency surface change**: zero new `windows`-crate features (the six remain); no internal program database anywhere; `Cargo.toml` touched only by the 0.6.0 version bump.
3. **Pure seam rules** (`src/open_with_listing.rs`, pinned by its 6 tests): blank names dropped; the system enumeration order preserved VERBATIM (no re-sort); display names dedup case-insensitively keeping the FIRST occurrence — whose raw input position survives as `launch_index`; menu item = `RegisteredProgram { display_name, launch_index }`.
4. **Launcher mechanism**: `open_file_with_selected_program` re-enumerates FRESH per call (same query — nothing cached or persisted across calls), indexes the fresh handler list with the stored `launch_index`, builds the payload `IDataObject` from the CURRENT canonical file (`SHCreateItemFromParsingName` → `SHCreateShellItemArrayFromShellItem`), and hands off via `IAssocHandler::Invoke(&data_object)`. Every launch failure collapses into the single static message `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` = `Windows could not open this file with the selected program.` on the pill's message line (the app's only message surface takes `&'static str`); a successful launch changes nothing.
5. **Two-point enumeration contract**: the stored session list `open_with_programs` (`Vec<RegisteredProgram>` RAM field on `ImagesScrollerApp`, `src/app.rs:42`) is refreshed ONLY at (a) every successful image open (`accept_open_outcome`, `src/app.rs:90`, before the path move) and (b) the click frame that opens the popup (`enumerate_when_menu_opens`, `src/app/open_with_button.rs:97/:108`). The popup reads the STORED list — no per-frame OS enumeration anywhere. Entering empty-folder mode CLEARS the list (`src/app/folder_navigation.rs:110`).
6. **UI (user-approved Option A)**: the pill's **11th button** — rightmost after the fit group, appended with the standard group gap (`src/app/floating_controls.rs:89`); painted **square-with-emerging-arrow glyph** (10-pt square shifted down-left + 7-pt shaft + 6-pt triangle head, all routed through the `pill_style` outline discipline); **36×32** button; **NO hotkey caption in v1** (the six keyboard-distinct buttons stay the only captioned ones); tooltip `"Open with..."` (`src/app/button_hint.rs:89`). The enabled click opens an **upward `egui::Popup::menu`** (`egui 0.37`, `RectAlign::TOP`) ABOVE the pill listing the stored programs' display names VERBATIM inside `egui::ScrollArea::vertical().max_height(300.0)`; entry click → `launch_open_with_program(index)`.
7. **State gates**: fresh empty state → the pill itself is absent (no button); image open + non-empty stored list → button ENABLED; image open + empty stored list → button DISABLED + dedicated tooltip `No programs registered for this file type` (`NO_PROGRAMS_TOOLTIP_TEXT`, applies ONLY while an image is open); empty-folder browsed → button DISABLED with the STANDARD `"Open with..."` tooltip; fullscreen ON → the WHOLE pill hidden by the existing chrome gate.
8. **Stub path** (`src/platform/stub.rs`): `enumerate_open_with_programs` → `Vec::new()`; `open_file_with_selected_program` → `Err(LaunchFailed)` — keeps the availability/tooltip/failure seams container-testable (`src/app/open_with_button_tests.rs`, 7 tests).
9. **Verification (Task 4, Cycle C)**: dev-checks `ALL CHECKS PASSED` (177/0, 28 files); windows-target compile (`cargo check --locked --target x86_64-pc-windows-gnu`) exit 0 — carrying the PRE-EXISTING benign dead-code warning on `compare_entry_names` (the Wave-5.2 recorded note, unchanged); release rebuild → exe 19,647,219 bytes; DLL audit SET-EQUAL 22; the pending 0.6.0 **OPERATOR-RUN gate** = the 21-item checklist verbatim at `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11.
10. **Wave-6 commit chain** (verbatim, chronological): `59068cf` (gate closure), `42d6237` (global plan), `f0b2274` (version 0.6.0), `d9dffe9` (Cycle-A plan), `bfa20e1` (enumeration+launcher), `ba66ce2` (test dedup), `7d0a4a3` (module docs), `84613cc` (Tasks 1+2 marks), `3f43439` (Task-3 plan+spec), `c00d427` (menu feature), `fa0a6ac` (simplification), `61cc1c8` (menu docs), `c5cb56b` (Task-3 mark), `49b3b01` (verification report+wording), `caa1aa0` (structure map), `b217dfa` (verification plan), `6b6a769` (Task-4 mark). Census 164→177 / 26→28; exe 19,605,828→19,647,219.

---

## §1 `README.md` (repository root)

R1. **Feature 7 bullet (line 26)** — REPLACE the current single sentence with the landed record (verbatim text to insert):

> 7. **Fast "Open with..." Menu** — (landed in Wave 6) the floating bottom pill's rightmost (11th) button opens a popup ABOVE the pill listing the programs the Windows system has registered for the currently viewed file's type, and selecting a program opens the CURRENT file with it. The list is built from system registration data at runtime (`SHAssocEnumHandlers` shell enumeration with no app-side filter — UWP/Store apps are included by design); the app maintains NO internal program database. Each entry shows the program's system display name verbatim, in the system's enumeration order (blank names are dropped and case-insensitive duplicates are deduplicated). The stored list is refreshed at exactly two points — every successful image open and the click that opens the popup (the popup itself reads the stored list; no per-frame enumeration). With no programs registered for the file's type the button is DISABLED and its tooltip reads "No programs registered for this file type"; the button has NO hotkey in v1 (click-only); if the system launch hand-off fails, the pill's message line shows the single static message "Windows could not open this file with the selected program."

R2. **Controls table (lines 28–36)** — APPEND one row after the "Fit width / fit height (one-shot size set)" row:

> | Open the current file with a system-registered program | — (no hotkey in v1) | the rightmost [Open with...] button (enabled while an image is open with registered programs; opens the program popup above the pill) |

R3. **Pill paragraph (line 38)** — four in-sentence updates, preserving everything else byte-for-byte:
   1. "The row shows 10 buttons" → "The row shows 11 buttons".
   2. The caption clause "...while the four arrows carry no caption" → "...while the four arrows and the Open-with button carry no caption (the Open-with button has no hotkey in v1)".
   3. "every one of the 10 buttons shows a hover tooltip in BOTH its enabled and disabled state" → "every one of the 11 buttons shows a hover tooltip in BOTH its enabled and disabled state".
   4. The pinned tooltip string list ending `..., "Fit width (W)", "Fit height (H)"` → append `, "Open with..."`.
   5. The enablement sentence "(each zoom button paints disabled exactly at its bound — [+] at ×8, [−] at the ÷8 floor, restore-fit only at ×1.0 — and the two fit buttons also stay disabled while an empty folder is browsed)" → append after it: ", and the Open-with button is disabled while no image is open, while an empty folder is browsed (with the standard "Open with..." tooltip), and whenever the stored enumeration for the open file's type is empty (then with the dedicated "No programs registered for this file type" tooltip)".

R4. **Technology Stack intro (line 48)** — REPLACE the trailing clause "...the Wave-5.2 name-order mechanism swap; the remaining v1 feature ("Open with...") is still open - see [Project Status](#project-status) and [.agent/project-info/architecture.md](.agent/project-info/architecture.md)." with:

> ...the Wave-5.2 name-order mechanism swap, and the Wave-6 "Open with..." menu — all seven v1 features are landed; see [Project Status](#project-status) and [.agent/project-info/architecture.md](.agent/project-info/architecture.md).

R5. **Technology Stack table, Language row (line 52)** — "Cargo package `img-scroller` 0.5.2" → "Cargo package `img-scroller` 0.6.0".

R6. **Project Status paragraph (line 68)** — two updates:
   1. The mid-paragraph sentence "The remaining v1 feature ("Open with...") is open." → "The remaining v1 feature ("Open with...") was landed in Wave 6 (the wave record below)."
   2. APPEND at the very end of the paragraph (after the Wave-5.2 pending-acceptance sentence):

> Wave 6 landed (branch `feat/wave-6-open-with`, version 0.6.0): the fast "Open with..." menu — the pill's rightmost 11th button opens a popup above the pill listing the programs the Windows system registered for the current file's type (runtime `SHAssocEnumHandlers` enumeration, UWP/Store apps included by design, NO internal program database), and selecting one opens the current file with it (`IAssocHandler::Invoke`); entries show the system display names verbatim in the system's enumeration order; the stored list refreshes at every successful open and at the popup-opening click; with no registered programs the button is disabled with the "No programs registered for this file type" tooltip; a failed launch shows the static "Windows could not open this file with the selected program." message on the pill's message line; the button has no hotkey in v1. Verification: dev checks `ALL CHECKS PASSED` (177 unit tests, 0 failed, across 28 test files), the release executable rebuilt at `dist/windows-x64/img-scroller.exe` (19,647,219 bytes), and the DLL import audit SET-EQUAL (the recorded 22-DLL set). Pending operator acceptance: the 0.6.0 OPERATOR-RUN (the wave's "Open with..." manual-UX checklist plus the standing regressions — one run, checklist at `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11).

No other README edit (Build and Run, Problem Statement, Overview, License: untouched).

---

## §2 `.agent/project-info/tech.md`

T1. **Status paragraph (line 5)** — APPEND at the very end (after the Wave-5.2 sentence):

> Landed for Wave 6 (branch `feat/wave-6-open-with`, version 0.6.0, 2026-11-12): the fast "Open with..." menu — runtime enumeration of the programs the system registered for the current file's type, the selected-program launcher, and the pill's 11th button with its upward popup (mechanism: `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` below).

T2. **Contents list (lines 9–26)** — INSERT as new item 11, after item 10 (Entry-Name System Order):

> 11. [Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)](#open-with-enumeration-and-launcher-mechanism-implemented-wave-6)

and RENUMBER the following items 11→12 (Development Environment), 12→13 (Build Environment), 13→14 (Build and Run), 14→15 (Testing), 15→16 (Technical Constraints), 16→17 (Distribution and Packaging), 17→18 (Tool Usage Patterns), 18→19 (References).

T3. **Technology Stack table, Language row (line 32)** — "version 0.5.2" → "version 0.6.0".

T4. **Dependencies section** — NO edit required (the six-feature list, the pin policy, and the no-other-dependencies bullet are all already correct; the wave changed nothing but the version).

T5. **NEW SECTION** `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` — placed IMMEDIATELY after the `## Entry-Name System Order (Implemented, Wave 5.1; Mechanism Wave 5.2)` section and BEFORE `## Development Environment and Setup`. Content (bulleted, this structure and these facts):

- Landed in Wave 6 (branch `feat/wave-6-open-with`, version 0.6.0; the LAST of the brief's seven v1 features — pillar 3 complete). Implemented in `src/open_with_listing.rs` (the pure listing seam + the failure-message constant), `src/platform/windows_open_with.rs` (the Windows enumeration + launcher), `src/platform/stub.rs` (the container fallbacks: an empty enumeration and a failed launcher), and `src/app/open_with_button.rs` (the 11th pill button + popup; UI-side mechanism: [architecture.md](architecture.md) → `## Module Layout (Implemented)` and the wave plans `.kilo/plans/20261112-wave6-tasks1-2-enumeration-launcher-implementation-plan.md` + `.kilo/plans/20261112-wave6-task3-open-with-menu-implementation-plan.md`).
- Enumeration mechanism (the architector decision): `SHAssocEnumHandlers` queried with the current file's extension (`.ext` null-terminated PCWSTR, `ASSOC_FILTER_NONE`) → the returned `IEnumAssocHandlers` drained one `Next` at a time → each `IAssocHandler::GetUIName` (freed with `CoTaskMemFree`) yields the raw display-name list, fed through the pure seam `programs_from_handler_names`. NO app-side handler filter — UWP/Store apps are INCLUDED by design. COM init reuses the shared `ensure_com_initialized` discipline from `windows.rs`; every failure path returns the empty/default value, never panics.
- Zero dependency surface change: NO new `windows`-crate features — all Wave-6 entry points are shell32/COM exports already covered by the six features present since Wave 5.2; the DLL import audit stayed SET-EQUAL to the recorded 22-DLL constant (the deciding evidence, never an assumption).
- Pure seam rules (`src/open_with_listing.rs`, pinned by its tests module): blank names are dropped; the system enumeration order is preserved VERBATIM (no re-sort); display names dedup case-insensitively keeping the FIRST occurrence — whose raw input position survives as `launch_index`; the menu item is `RegisteredProgram { display_name, launch_index }`. NO internal program database exists anywhere.
- Launcher mechanism: `open_file_with_selected_program` re-enumerates FRESH per call (the same `SHAssocEnumHandlers` query — nothing cached or persisted across calls), indexes the fresh handler list with the stored `launch_index`, builds the payload `IDataObject` from the CURRENT canonical file (`SHCreateItemFromParsingName` → `SHCreateShellItemArrayFromShellItem`), and hands off via `IAssocHandler::Invoke(&data_object)`; every failure path collapses into the single static message `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` = "Windows could not open this file with the selected program." on the pill's message line (the app's only message surface takes `&'static str`); a successful launch changes nothing.
- Two-point enumeration contract: the stored session list `open_with_programs` (`Vec<RegisteredProgram>` on `ImagesScrollerApp`) is refreshed ONLY (a) at every successful image open (`accept_open_outcome`, before the path move) and (b) on the click frame that opens the popup (`enumerate_when_menu_opens`); the popup reads the STORED list — no per-frame OS enumeration anywhere; entering empty-folder mode clears the list (`src/app/folder_navigation.rs`).
- Stub path: the container/stub build's `enumerate_open_with_programs` returns an empty list and `open_file_with_selected_program` returns `LaunchFailed` — the availability/tooltip/failure seams stay container-testable (7 tests in `src/app/open_with_button_tests.rs`, 6 in `src/open_with_listing/tests.rs`).
- Verification (Wave-6 Task 4): dev checks `ALL CHECKS PASSED` — 177 unit tests, 0 failed, across 28 test files (census: `## Testing`); the windows-target compile (`cargo check --locked --target x86_64-pc-windows-gnu`) exit 0, carrying the PRE-EXISTING benign dead-code warning on `compare_entry_names` (the Wave-5.2 recorded note); the 0.6.0 exe rebuilt at `dist/windows-x64/img-scroller.exe` = 19,647,219 bytes; the DLL import audit SET-EQUAL to the recorded 22-DLL constant (the list NOT duplicated here — docs-split rule); NO dependency changes (`Cargo.toml`/`Cargo.lock` untouched by the wave beyond the version bump). The pending 0.6.0 OPERATOR-RUN checklist lives verbatim in `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11.

T6. **Testing section (§ Testing)** — the census 164→177 / 26→28:
   1. Opening sentence: "164 unit tests" → "177 unit tests".
   2. Per-file list: INSERT `src/app/open_with_button_tests.rs` — 7 (Wave-6 Task 3: the availability, tooltip, and launch-failure seams) between `src/app/last_viewed_tests.rs` — 5 and `src/app/pan_tests.rs` — 12; INSERT `src/open_with_listing/tests.rs` — 6 (Wave-6 Tasks 1–2: the blank-name filter, system-order preservation, and the launch_index/dedup contract) between `src/name_collation.rs` — 4 and `src/pan/pan_tests.rs` — 11.
   3. Grand-total sentence → "Grand total 177 across 28 test files (the 164 pre-Wave-6 tests unchanged + 13 Wave-6 tests: +6 the open-with listing seam, +7 the open-with button seams)."

T7. **Technical Constraints** — APPEND one bullet after the sort-order bullet:

> - The "Open with..." program list comes exclusively from the runtime system enumeration — NO internal/hardcoded program database exists; UWP/Store apps are included by design (no app-side handler filter); the launcher re-enumerates fresh per hand-off (nothing cached); the menu mechanism and its constraints are recorded in `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` above.

---

## §3 `.agent/project-info/architecture.md`

A1. **Status paragraph (line 5)** — APPEND at the very end (after the Wave-5.2 sentence):

> Wave 6 (branch `feat/wave-6-open-with`, version 0.6.0, 2026-11-12) landed the final v1 pillar — the fast "Open with..." service: runtime `SHAssocEnumHandlers` enumeration of the system-registered programs for the current file's type, the `IAssocHandler::Invoke` launcher, and the pill's 11th button with its upward popup (mechanism: [tech.md](tech.md) → `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)`); dev checks ended `ALL CHECKS PASSED` (177 unit tests), the exe was rebuilt at `dist/windows-x64/img-scroller.exe` (19,647,219 bytes at 0.6.0), and the DLL import audit stayed SET-EQUAL 22.

A2. **Overview, third bullet (line 24)** — REPLACE "Wave 1 landed pillars 1–2 as the open-image slice; pillar 3 ("Open with...") is planned." with:

> Wave 1 landed pillars 1–2 as the open-image slice; Wave 6 landed pillar 3 — the "Open with..." menu (runtime system enumeration, NO internal program database).

A3. **Logical Components**:
   1. Viewer-UI bullet (line 30): REPLACE the trailing "the "Open with..." menu entry remains planned." with "the "Open with..." menu entry (the pill's 11th button with its upward popup) is LANDED (Wave 6 — the dedicated "Fast Open-with Service" bullet below)."
   2. **Fast "Open with..." Service bullet (line 39)** — REPLACE the whole planned bullet with the LANDED bullet (verbatim text):

> - **Fast "Open with..." Service — LANDED (Wave 6)**: at runtime queries the Windows system via `SHAssocEnumHandlers` (the current file's extension, `ASSOC_FILTER_NONE` — NO app-side filter, so UWP/Store apps are INCLUDED by design) for the handlers registered for that type, drains `IEnumAssocHandlers`, reads each `IAssocHandler::GetUIName`, and feeds the raw names through the pure seam `src/open_with_listing.rs` (blank names dropped; the system enumeration order preserved verbatim — no re-sort; case-insensitive dedup keeping the FIRST occurrence, whose raw position survives as `launch_index`; `RegisteredProgram { display_name, launch_index }`). NO internal program database; zero new `windows`-crate features (the six remain; the DLL audit stayed SET-EQUAL 22). The launcher re-enumerates FRESH per hand-off and invokes the selected handler with `IAssocHandler::Invoke` over an `IDataObject` built from the current canonical file (`SHCreateItemFromParsingName` → `SHCreateShellItemArrayFromShellItem`); every failure collapses into the single static message "Windows could not open this file with the selected program." on the pill's message line. The UI side (user-approved Option A): the pill's rightmost 11th button (painted square-with-emerging-arrow glyph, 36×32, NO hotkey in v1) opens an upward `egui::Popup::menu` (`RectAlign::TOP`) listing the stored programs' display names verbatim inside a `ScrollArea::vertical().max_height(300.0)`; the stored list refreshes ONLY at the two decided points — every successful open and the popup-opening click (the popup reads the STORED list; no per-frame enumeration); with an image open and an empty list the button is DISABLED with the tooltip "No programs registered for this file type"; in empty-folder mode it is disabled with the standard "Open with..." tooltip; the fresh empty state has no pill at all; fullscreen ON hides the whole pill (the existing chrome gate). Mechanism detail: [tech.md](tech.md) → `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)`.

A4. **Module Layout (Implemented)**:
   1. `src/app.rs` bullet (line 47): the state enumeration gains `open_with_programs` — after `esc_close_popup_expires_at` insert `/open_with_programs` in the `fullscreen_on`/`zoom_factor`/`esc_close_popup_expires_at` slash-list and add the parenthetical "(the Wave-6 stored open-with list, refreshed at the two decided points)". The child-module list gains `src/app/open_with_button.rs` (inserted alphabetically between `src/app/keyboard_intake.rs` and `src/app/pan.rs`); the test-module list gains `src/app/open_with_button_tests.rs` (inserted after `src/app/last_viewed_tests.rs`).
   2. NEW bullet for `src/open_with_listing.rs` — INSERT after the `src/name_collation.rs` bullet (before the `src/app/floating_controls.rs` bullet):

> - `src/open_with_listing.rs` — the pure open-with program listing seam (Wave 6 Tasks 1–2): `programs_from_handler_names` turns the raw system handler names into `Vec<RegisteredProgram>` — blank names dropped, the system enumeration order preserved verbatim, case-insensitive dedup keeping the FIRST occurrence (its raw index survives as `launch_index`); `RegisteredProgram { display_name, launch_index }` and `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` (the single static launch-failure message) live here; 6 tests in `src/open_with_listing/tests.rs`.

   3. NEW bullet for `src/app/open_with_button.rs` — INSERT after the `src/app/keyboard_intake.rs` bullet (before the `src/app/pan.rs` bullet):

> - `src/app/open_with_button.rs` — the pill's 11th "Open with..." button (Wave 6 Task 3, user-approved Option A): appended rightmost after the fit group (36×32 button; painted square-with-emerging-arrow glyph routed through the `pill_style` outline discipline; NO hotkey caption in v1; tooltip "Open with..."); the enabled click re-enumerates via the facade (the popup-opening point of the two-point contract) and opens the upward `egui::Popup::menu` (`RectAlign::TOP`) listing the stored programs' display names verbatim inside `egui::ScrollArea::vertical().max_height(300.0)`; entry clicks launch through `launch_open_with_program` (the facade launcher; failure → the static message on the pill's message line); `open_with_menu_is_available` = image open AND a non-empty stored list; the no-programs tooltip applies ONLY while an image is open with an empty enumeration (empty-folder mode keeps the standard tooltip); 7 tests in `src/app/open_with_button_tests.rs`.

   4. `src/app/floating_controls.rs` bullet (line 52): APPEND: "the pill row composes the arrow group, the fullscreen square, the zoom and fit groups, and the Wave-6 `open_with_button::show_open_with_pill_button` 11th button appended after the fit group (11 buttons total)."
   5. `src/platform/` bullet (line 68): the re-export list gains the Wave-6 pair — extend "mod.rs re-exports `decode_image`, `show_open_image_dialog` and `compare_entry_names_system`" to "...and, since Wave 6, the open-with pair `enumerate_open_with_programs`/`open_file_with_selected_program` consumed by `src/app/open_with_button.rs`"; the `windows.rs` mention stays; INSERT into the folder description (after the `windows_collation.rs` clause): "`windows_open_with.rs` (Wave 6) implements the open-with enumeration (`SHAssocEnumHandlers` → `IEnumAssocHandlers` → `IAssocHandler::GetUIName`) and the launcher (`IAssocHandler::Invoke` over the `IDataObject` built from the current canonical file)"; the `stub.rs` description gains "the open-with queries return an empty enumeration and a failed launcher".

A5. **Navigation Critical Paths**:
   1. Intro sentence (line 83): REPLACE the trailing "; "Open with..." remains planned." with "; Path 5's "Open with..." service is LANDED (Wave 6 — the path below)."
   2. Path 5 (line 89) — REPLACE with:

> 5. **Open with (Wave-6 slice LANDED)**: the pill's rightmost button click → the stored list refreshed at the two decided points (every successful open; this popup-opening click) → the upward popup lists the system-registered programs (the system's enumeration order, display names verbatim) → selecting an entry re-enumerates fresh and invokes the selected handler (`IAssocHandler::Invoke` with the current canonical file's `IDataObject`) → a failure shows the single static launch-failure message on the pill's message line.

A6. **Data and State**: in the "In memory (per session; Waves 3–4 landed)" bullet — change the heading to "(per session; Waves 3–4 and 6 landed)" and APPEND at its end:

> and the session `open_with_programs` list (Wave 6: `Vec<RegisteredProgram>` — the stored enumeration of the system-registered programs for the current file's type, refreshed ONLY at every successful open and at the popup-opening click, cleared when empty-folder mode is entered) — all RAM-only, destroyed on close; never persisted in v1.

A7. **Technical Decisions Summary** — INSERT one row before the "Distribution: zip-style installer" row:

> | Open-with mechanism: runtime `SHAssocEnumHandlers` (`ASSOC_FILTER_NONE` — UWP/Store apps included) → `IEnumAssocHandlers` → `IAssocHandler::GetUIName` through the pure `open_with_listing` seam (blank-name filter, system order verbatim, case-insensitive dedup keeping the first `launch_index`); launcher = fresh re-enumeration + `IAssocHandler::Invoke` with the current canonical file's `IDataObject`; failures → the single static message; NO internal program database; zero new `windows` features (the six remain); menu UI = the pill's 11th button + upward `Popup::menu` (user-approved Option A), 300-point scroll cap, the two-point enumeration contract, disabled + "No programs registered for this file type" on an empty list, NO hotkey in v1 | landed (Wave 6) | `### Fast "Open with..." Menu` |

A8. **Project Paths** (line 112): the repository-layout enumeration gains `open_with_listing` (+ its `open_with_listing/tests` child module) — inserted after `name_collation`; the app child-module list gains `open_with_button`; the `platform/` facade mention gains `windows_open_with`.

---

## §4 `.agent/project-info/context.md`

C1. **Current Work Focus** — REWRITE the FIRST dated paragraph (the one opening "Dated 2026-11-12. **Wave 6 ... is STARTING ...**") into the landed-state paragraph, PRESERVING the 0.5.2 gate CLOSED note inside it (verbatim replacement text):

> Dated 2026-11-12. **Wave 6 (Fast "Open with..." Menu, `.agent/todos/20261029/20261029-todo-8.md`) is agent-side COMPLETE through Task 5 documentation (this step) — Tasks 1–5 all `[DONE]` except the Task-4 manual-UX sub-item, which stays `[ ]` as the user's gate — and the 0.5.2 acceptance gate is CLOSED**: the user reported at the Wave-6 session start that navigation now follows the correct order (the 0.5.2 OPERATOR-RUN name-order re-test PASSED; NO defect report; "all the rest OK" stands). Wave 6 landed the v1 feature pillar 3: runtime enumeration of the programs Windows registered for the currently viewed file's type (`SHAssocEnumHandlers` with `ASSOC_FILTER_NONE` → `IEnumAssocHandlers` → `IAssocHandler::GetUIName`; UWP/Store apps included by design; NO internal program database; zero new `windows` features — the six remain), the launcher (`IAssocHandler::Invoke` with an `IDataObject` built from the current canonical file via `SHCreateItemFromParsingName` → `SHCreateShellItemArrayFromShellItem`; fresh re-enumeration per hand-off; failures → the single static "Windows could not open this file with the selected program." message on the pill's message line), and the UI (user-approved Option A): the pill's 11th button (rightmost after the fit group, painted square-with-emerging-arrow glyph, 36×32, no hotkey in v1) opening an upward `egui::Popup::menu` (`RectAlign::TOP`) above the pill with a 300-point `ScrollArea` cap, entries verbatim, the two-point enumeration contract (every successful open + the popup-opening click; the popup reads the STORED list), and the state gates (fresh empty: pill absent; image open + non-empty: enabled; image open + empty: DISABLED + "No programs registered for this file type"; empty-folder browsed: disabled + the standard tooltip; fullscreen ON: the whole pill hidden). Branch `feat/wave-6-open-with`; version 0.6.0 (bumped at `f0b2274`). Verification (Task 4): dev-checks `ALL CHECKS PASSED` (177 tests / 0 failed, 28 files — census 164→177, 26→28); the windows-target compile exit 0 (the pre-existing benign dead-code warning on `compare_entry_names` stands — the Wave-5.2 recorded note); the release rebuild → `dist/windows-x64/img-scroller.exe` = 19,647,219 bytes (the 0.5.2 artifact 19,605,828 B overwritten atomically); the DLL audit SET-EQUAL 22. PENDING ACCEPTANCE GATE: the 0.6.0 OPERATOR-RUN (Next-Steps #1 — the 21-item checklist at `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11).

The second dated paragraph (the Wave-5.2 COMPLETE closing record) stays byte-identical — historical record, do not touch.

C2. **Recent Changes** — INSERT a NEW #1 entry at the TOP of the list (the file styles every entry as "1. 2026-11-12:" — follow that style exactly; the existing entries shift down as a list). Content (verbatim):

> 1. 2026-11-12: **Wave 6 Tasks 1–4 landed; Task 5 documentation landed (this step) — the "Open with..." menu is IN (version 0.6.0, branch `feat/wave-6-open-with`).** The full four-cycle commit chain (verbatim): `59068cf` (0.5.2 gate closure — the user's name-order re-test PASSED, the todo-2-DONE manual-UX sub-item marked `[x]`), `42d6237` (Wave-6 global plan), `f0b2274` (version bump 0.6.0), `d9dffe9` (Cycle-A implementation plan), `bfa20e1` (enumeration + launcher feature), `ba66ce2` (test dedup), `7d0a4a3` (module docs), `84613cc` (Tasks 1+2 marks), `3f43439` (Task-3 plan + spec), `c00d427` (menu feature), `fa0a6ac` (simplification), `61cc1c8` (menu docs), `c5cb56b` (Task-3 mark), `49b3b01` (verification report + wording), `caa1aa0` (structure map), `b217dfa` (verification plan), `6b6a769` (Task-4 mark). What landed: (Tasks 1+2, Cycle A) the runtime enumeration — `SHAssocEnumHandlers` (extension query, `ASSOC_FILTER_NONE` — UWP/Store apps included by design) → `IEnumAssocHandlers` → `IAssocHandler::GetUIName` feeding the pure seam `src/open_with_listing.rs` (`RegisteredProgram { display_name, launch_index }`; blank-name filter; the system enumeration order preserved; case-insensitive dedup keeping the first raw `launch_index`), plus the launcher (`IAssocHandler::Invoke` with the `IDataObject` built from the current canonical file via `SHCreateItemFromParsingName` → `SHCreateShellItemArrayFromShellItem`; fresh per-call enumeration; every failure → the single static `OPEN_WITH_LAUNCH_FAILURE_MESSAGE`); ZERO new `windows`-crate features (the six remain) and NO internal program database. (Task 3, Cycle B, user-approved Option A) the pill's 11th button — rightmost after the fit group, painted square-with-emerging-arrow glyph, 36×32, no hotkey in v1 — opening the upward `egui::Popup::menu` (`RectAlign::TOP`) above the pill with `ScrollArea::vertical().max_height(300.0)`, entries verbatim `display_name`, the two-point enumeration contract (every successful open + the popup-opening click; the popup reads the STORED list), and the state gates (fresh empty: pill absent; image open + non-empty: enabled; image open + empty: DISABLED + "No programs registered for this file type"; empty-folder browsed: disabled + the standard "Open with..." tooltip; fullscreen ON: the whole pill hidden by the chrome gate). (Task 4, Cycle C) verification GREEN: dev-checks `ALL CHECKS PASSED` (177 unit tests / 0 failed, 28 test files — census 164→177, 26→28: +6 `src/open_with_listing/tests.rs`, +7 `src/app/open_with_button_tests.rs`); the windows-target compile exit 0 (the pre-existing benign dead-code warning on `compare_entry_names` — the Wave-5.2 recorded note); the release rebuild → `dist/windows-x64/img-scroller.exe` = 19,647,219 bytes (the 0.5.2 artifact 19,605,828 B overwritten atomically); the DLL audit SET-EQUAL 22. (Task 5, this step) `README.md`, `tech.md`, `architecture.md`, `context.md` updated to the Wave-6 facts (the new `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` section in tech.md; the LANDED component/module/state/paths/decisions in architecture.md; the README feature-7/controls/status records; this entry); `brief.md` verified — `### Fast "Open with..." Menu` already mandates the wave's behavior, NO addendum (verify-only, NO edit); `.agent/project-structure.md` verified current (already updated at `caa1aa0`, verify-only, NO edit); `docs/build-windows.md` verified — the audit was SET-EQUAL, NO OBSERVED-list edit (verify-only, NO edit). ARTIFACT NOTE (recorded, do not act): a one-word commit-order transposition stands in `.kilo/plans/20261112-wave6-task4-verification-plan.md` Step 0 item 3 (the expected top-5 chain lists `caa1aa0` before `c5cb56b`; the actual order is `c5cb56b` then `caa1aa0`); the document's own Step 6 full-chain check SUPERSEDES it; existing plan files must NOT be edited. PENDING ACCEPTANCE GATE: the 0.6.0 OPERATOR-RUN (Next-Steps #1).

C3. **Immediate Next Steps** — REPLACE items 1–3 (including the old 0.5.2 verbatim checklist block, its CONCRETE EXPECTATION paragraph, and its REGRESSION NOTE — the 0.5.2 gate is CLOSED) with the new three:

> 1. **Pending 0.6.0 Wave-6 OPERATOR-RUN gate — the user's run on a real Windows 10/11 machine**, running the freshly rebuilt 0.6.0 exe at `dist\windows-x64\img-scroller.exe`. The 21-item manual-UX checklist is preserved verbatim at [`.kilo/plans/20261112-wave6-task4-verification-plan.md`](../../.kilo/plans/20261112-wave6-task4-verification-plan.md) §11: TODO Task-4 sub-item 2 verbatim ("for a jpg/png file, the menu lists at least the system-associated programs; selecting one opens this exact file in that program; NOT dependent on any app-maintained list") + the 10 Wave-6 "Open with..." items (basic menu content; launch hand-off; no internal database; empty-enumeration tooltip; disabled in empty-folder mode; fullscreen hiding; popup interaction; long-list scrolling; failure message line; visual/DPI sanity) + items 11–21, the standing regressions (pan/edge-navigate; ←/→/↑/↓ lists in the system natural order incl. the 智乃 example; silent edges; buttons; I/O/0/W/H; the ESC contract incl. popup expiry; Shift+'+' / Shift+'−' aliases; captions/tooltips; always-maximized launch + hidden console; empty-folder mode; last-viewed memory). The 0.5.2 gate is CLOSED — the standing regressions re-run inside this single 0.6.0 run; one run, one gate.
> 2. **Wave-6 remaining agent-side steps after Task 5**: the Task-5 4.5b coherence pass over the four docs, then 4.6 (the TODO Task-5 sub-item marked `[x]`; the Task-4 manual-UX sub-item stays `[ ]`), then the global-plan Step 5 (tmp sweep, `main` checked out, `feat/wave-6-open-with` fast-forward merged, verified, and deleted, `main` pushed → `origin` ONLY, the TODO renamed `-DONE`, and the final context.md closing touch recording the merged/pushed state).
> 3. **NEXT SESSION — Wave 7 (`.agent/todos/20261029/20261029-todo-9.md`, the x64 AND x86 packaging matrix)** runs in a NEW chat after Wave 6 closes at Step 5; the 0.6.0 gate (item 1) stays the user's responsibility and closes when the user reports PASS; the parked future-ideas list stays OUT of scope until scheduled.

---

## §5 Verify-only items (expected NO-EDIT verdicts — confirm, then leave untouched)

V1. **`.agent/project-structure.md`** — VERIFIED current at plan time: the Wave-6 entries are all present (root `open_with_listing.rs` with its `open_with_listing/tests.rs` child module; the `app/` entry names `open_with_button.rs` + `open_with_button_tests.rs` + the 11-button row; the `platform/` entry names `windows_open_with.rs` and the enumerate/launch pair consumed by `open_with_button.rs`; updated at `caa1aa0`). Verdict: NO edit. IF the docs-specialist finds a factual mismatch during the pass → STOP and report to the caller (do NOT edit without caller approval).

V2. **`.agent/project-info/brief.md`** — VERIFIED at plan time: `### Fast "Open with..." Menu` (brief lines 73–77) already mandates the landed behavior (menu entry; list from system registration data at runtime; selecting opens the current file; the app does NOT maintain its own program database). No contradicting sentence exists. Verdict: NO addendum, NO edit.

V3. **`docs/build-windows.md`** — VERIFIED at plan time: the Task-4 DLL audit verdict was SET-EQUAL 22, so the OBSERVED DLL import list (lines 58–62) and the audit note stay byte-identical. Verdict: NO edit.

---

## §6 Harness / verification needs

- This is a DOCS-ONLY step: the four edited files are markdown; nothing in `src/`, `Cargo.toml`, or the harness changes.
- **A dev-checks harness run is NOT required** for this step (prescribed: do NOT run it). The 4.2 step does not need the Alpine VM at all; do not call `alpine-vm` for a docs-only landing. (The standing 177/0 evidence from Task 4 remains the wave's authoritative harness record.)
- Static self-check for the 4.2 executor (no cargo): after the edits, re-read each of the four files and confirm every §1–§4 item above is present and no unrelated sentence was altered; confirm `brief.md`, `project-structure.md`, and `build-windows.md` are byte-untouched (`git status` shows only the four docs + the plan file as changed/staged).

---

## §7 Commit protocol (Gitignore Compliance Rule before EVERY commit)

1. Read `.gitignore` and run `git status` before staging; verify no `dist/`/`logs/`/`*.log`/`target/` file is staged; never `git add -A` blindly; never force-add tracked files matching ignore patterns.
2. **Commit 1 (plan landing)**: stage ONLY `.kilo/plans/20261112-wave6-task5-documentation-plan.md` → commit message `docs: add wave6 task5 documentation plan`. (Wave-5.2 precedent: `47b4ec0`.)
3. **Commit 2 (docs landing)**: stage EXACTLY the four files — `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md` → commit message `docs: wave6 task5 documentation`. (Wave-5.2 precedent: `51d2b27`.)
4. If the 4.5b coherence pass (a SUBSEQUENT step, not this plan's scope) produces fixes, they land as a follow-up `docs: wave6 task5 coherence fixes` commit (Wave-5.2 precedent: `e0eb8bf`) — noted here for the commit-message convention only.
5. Final `git status` → clean working tree on `feat/wave-6-open-with`.

## §8 Scope boundaries (binding)

- EDIT exactly: `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`, plus creating THIS plan file.
- NO code changes; NO `Cargo.toml`/`Cargo.lock` edits; NO `CHANGELOG.md` (the wave's standing note); NO edits to existing `.kilo/plans/*` files (the artifact note is recorded in context.md, never fixed in place); NO edits to `brief.md` / `.agent/project-structure.md` / `docs/build-windows.md` (verify-only); NO TODO edits (the 4.6 step owns the `[x]` mark); NO branch creation, NO version edit, NO merge, NO `git push` (Step-5 restricted).
- Only the content items prescribed in §1–§4; nothing else in any file may be reworded or reordered.

---

## §9 Cross-check against TODO Task 5 verbatim

- "`README.md`, `tech.md`, `architecture.md`, `context.md`: record the enumeration mechanism chosen" → README R1/R6 (user-facing mechanism + status record), tech.md T5 (the full mechanism section: enumeration, seam rules, launcher, two-point contract, stub path, verification), architecture.md A3/A4/A5 (the LANDED service bullet, module bullets, critical path 5). ✓
- "...constraints" → the no-internal-database rule, the UWP/Store inclusion, the no-cache/fresh-per-hand-off launcher, the session-RAM-only list, the two-point refresh contract, the no-hotkey-in-v1 decision, the zero-dependency surface — recorded in tech.md T5+T7, architecture.md A3/A7, README R1. ✓
- "...remaining open items" → the pending 0.6.0 OPERATOR-RUN gate (the 21-item §11 checklist) recorded in README R6, tech.md T5 (verification bullet), architecture.md A1 (implicitly via status), context.md C1/C3 (Next-Steps #1). ✓
- Verify-only triple (project-structure.md / brief.md / build-windows.md) → §5 with expected NO-EDIT verdicts. ✓
- Harness needs → §6 (NOT required). Commit protocol → §7. Scope boundaries → §8. ✓

**Plan file:** `.kilo/plans/20261112-wave6-task5-documentation-plan.md`
