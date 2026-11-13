# Wave 6 Task 3 — "Open with..." Menu Front-end Implementation Verification

**Date:** 2026-11-13  
**Branch:** `feat/wave-6-open-with` (HEAD `61cc1c8`)  
**Step:** Critical Workflow 4.5a (front-end specialist)  
**Source TODO:** `.agent/todos/20261029/20261029-todo-8.md` Task 3  
**Spec verified against:** `.kilo/plans/20261112-wave6-task3-open-with-menu-frontend-spec.md`  
**Implementation plan:** `.kilo/plans/20261112-wave6-task3-open-with-menu-implementation-plan.md`

---

## Summary verdict

**PASS-WITH-ADJUSTMENTS**

The landed implementation matches the approved Option A design and every binding user decision. All deviations from the literal 4.1a spec text are the two adjustments already recorded in the implementation plan (popup-open re-enumeration point; test census 177 vs. the spec's 176 estimate), plus the module-name refinement (`open_with_button.rs` vs. the spec's earlier working name `open_with_menu.rs`). No front-end code changes are required.

---

## 4.5a conformance checklist — per-point results

| # | Spec expectation | Landed reality | Result |
|---|------------------|----------------|--------|
| 1 | `PillButton::OpenWith` exists in `button_hint.rs` with tooltip `"Open with..."` and `hotkey_reference(...) == None`. | `OpenWith` variant added; `tooltip_text(OpenWith) == "Open with..."`; `hotkey_reference(OpenWith) == None`; `button_hint_tests.rs` asserts both. | **CONFORMS** |
| 2 | New pill button is `36 × 32` points, uses the shared `ButtonColumnSite`/`show_button_column` wrapper, and paints a custom vector glyph (white fill, 1-point black outline). | `OPEN_WITH_BUTTON_SIZE = Vec2::new(36.0, 32.0)`; rendered via `ButtonColumnSite::new`/`show_button_column`; glyph uses `pill_style::outlined_rect_shape`, black under-stroked shaft, and `pill_style::outlined_triangle_shape`. | **CONFORMS** |
| 3 | Button sits at the right end of the pill row, separated by a 16 pt group gap. | `floating_controls.rs` appends `open_with_button::show_open_with_pill_button` after `fit_button::show_fit_pill_buttons`; the module adds `row_ui.add_space(OPEN_WITH_GROUP_GAP)` where `OPEN_WITH_GROUP_GAP = 16.0`. | **CONFORMS** |
| 4 | Menu is driven by `egui::Popup::menu(&button_response)`. | `open_with_button.rs` calls `egui::Popup::menu(&button_response).align(egui::RectAlign::TOP).accessible_name(...).show(...)`. | **CONFORMS** |
| 5 | Menu default placement is above the button (`RectAlign::TOP`) with egui's automatic fallback alignment. | `.align(egui::RectAlign::TOP)` is present; egui handles fallback symmetries/`MENU_ALIGNS` automatically. | **CONFORMS** |
| 6 | Menu entries are `ui.button(&program.display_name)` verbatim. | `show_menu_entry_buttons` renders `scroll_ui.button(self.open_with_programs[program_index].display_name.as_str())` with no re-sort, truncation, or icons. | **CONFORMS** |
| 7 | Long lists are wrapped in `egui::ScrollArea::vertical().max_height(300.0)`. | `show_open_with_menu_entries` uses `egui::ScrollArea::vertical().max_height(OPEN_WITH_MENU_MAX_HEIGHT)` with `OPEN_WITH_MENU_MAX_HEIGHT = 300.0`. | **CONFORMS** |
| 8 | Empty enumeration renders a disabled button with tooltip `"No programs registered for this file type"`. | `open_with_menu_is_available()` returns `image_is_open() && !open_with_programs.is_empty()`; when false the button is painted disabled. `no_programs_tooltip_for` returns the no-programs tooltip only when a file is open with an empty list; empty-folder mode falls back to the standard `"Open with..."` tooltip. | **CONFORMS** |
| 9 | Selecting an entry calls `platform::open_file_with_selected_program(path, launch_index)`. | `launch_open_with_program` clones the current `opened_image_path` and calls `platform::open_file_with_selected_program(&opened_path, program_launch_index)`. | **CONFORMS** |
| 10 | On launch error, `message_text` is set to `OPEN_WITH_LAUNCH_FAILURE_MESSAGE`. | `launch_open_with_program` sets `self.message_text = Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)` on `Err`; success leaves state unchanged. | **CONFORMS** |
| 11 | Button/menu unavailable when no image is open, in empty-folder mode, and hidden when fullscreen is ON. | Enablement requires `image_is_open()` and a non-empty list; `enter_empty_folder_mode` clears the list; fullscreen hides the whole pill via the existing `full_ui_chrome_is_visible()` gate. | **CONFORMS** |
| 12 | `open_with_programs` refreshed only on successful open (no per-frame enumeration). | Refreshed in `accept_open_outcome` success arm AND on the popup-opening click frame (`enumerate_when_menu_opens`). The popup contents closure reads only the stored list. | **CONFORMS-with-recorded-adjustment** (plan §2 D5 / §9 notes) |
| 13 | Test census matches the estimated 176/28 (±1). | Census landed at **177 unit tests / 28 test files** (170 + 7 new tests in `open_with_button_tests.rs`). | **CONFORMS-with-recorded-adjustment** (plan §4 / §2 D12) |
| 14 | No per-frame texture re-upload or allocation churn is introduced. | No changes to texture/display pipeline; the popup path only allocates egui shapes/widgets; program list is stored once and borrowed. | **CONFORMS** |

---

## Classified diff summary (spec expectation → landed reality)

1. **Popup-open re-enumeration point** — *CONFORMS-with-recorded-adjustment*  
   - Spec §3 wording says the display list refreshes "only on successful open".  
   - Approved user decision A8 and plan §2 D5 added a second refresh: the click frame that opens the popup enumerates fresh so the menu lists current registrations when opened.  
   - Landed code implements exactly these two points and no per-frame enumeration inside the popup.

2. **Test census** — *CONFORMS-with-recorded-adjustment*  
   - Spec §9 estimated 176 tests / 28 files (±1).  
   - Plan §2 D12 pinned 7 new tests → 177/28.  
   - Landed `open_with_button_tests.rs` contains the 7 pinned tests.

3. **Module/file name** — *CONFORMS-with-recorded-adjustment*  
   - Spec §8 proposed `src/app/open_with_menu.rs` + `src/app/open_with_menu_tests.rs`.  
   - Plan §2 D10 renamed to `src/app/open_with_button.rs` + `src/app/open_with_button_tests.rs` to match the single-button scope.  
   - Landed files use the plan-pinned names.

4. **`OpenWithButtonSpec` simplification** — *acceptable local refactor, not a deviation*  
   - Plan skeleton showed the spec object carrying a `disabled_tooltip` field; the simplification commit (`fa0a6ac`) computes `no_programs_tooltip_for(app)` inline.  
   - Behavior is identical; this stays within the junior-developer 50% latitude for minor local details.

No **DEVIATION REQUIRING FIX** was found.

---

## Front-end quality findings

| Area | Finding | Severity | Fix required? |
|------|---------|----------|---------------|
| Discoverability | The new button is the rightmost pill button with a tooltip and a painted glyph, consistent with every other viewer action. | — | No |
| Glyph visual consistency | Glyph uses the shared `pill_style` outlined-rect/triangle builders and the standard white-fill/1-pt-black-outline palette; geometry matches plan constants. | — | No |
| Popup positioning | `Popup::menu` + `RectAlign::TOP` places the menu above the pill; egui automatically flips if it does not fit. | — | No |
| Popup dismissal | `Popup::menu` closes automatically on item click, outside click, and `Esc`; no manual state is needed. | — | No |
| Disabled-state affordance | The button is *painted* disabled (dimmed glyph via `glyph_state_for`) and omits the popup attachment; the cursor is still `PointingHand`, matching the existing arrow-button disabled pattern. The no-programs tooltip is shown on hover. | Minor observation | No — behavior is consistent with the existing pill; no approved requirement to change the cursor |
| Tooltip correctness | Enabled button hover shows `"Open with..."`. Disabled + file open + empty enumeration shows `"No programs registered for this file type"`. Disabled in empty-folder mode shows `"Open with..."`. | — | No |
| Failure surfacing | `launch_open_with_program` writes `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` to `message_text`, which is rendered by the existing pill message line. | — | No |
| Keyboard behavior inside popup | Default egui menu keyboard handling applies (arrow keys navigate, Enter/Space activates, Esc closes). This is an accepted platform default and was not overridden. | — | No |

**No fixes are required.**

---

## OPERATOR-RUN items (cannot be verified from the Linux container)

The following must be checked on the real Windows 10/11 target machine because the Alpine container cannot run the Windows GUI or the COM shell-association APIs:

1. **Basic menu content** — Open a `.jpg` or `.png`; click the new pill button; confirm the popup lists at least the system-registered programs (e.g., Paint, Photos) and that names match the system "Open with" dialog.
2. **Launch hand-off** — Select a listed program; confirm the *same* file opens in that program and not a different file.
3. **No internal database** — Temporarily change the default program for the file type or register/unregister a handler; confirm the menu reflects the live system state on the next popup-open click.
4. **Empty-enumeration tooltip** — Open a file type with no registered handlers (or use a custom extension); hover the disabled button and confirm `"No programs registered for this file type"`.
5. **Disabled in empty-folder mode** — Navigate into an empty sibling folder; confirm the open-with button is disabled and its tooltip is `"Open with..."` (not the no-programs tooltip).
6. **Fullscreen hiding** — Enter fullscreen; confirm the whole pill (including the new button) disappears.
7. **Popup interaction** — With a populated menu, verify outside-click and `Esc` dismiss the popup; verify arrow-key navigation and Enter selection inside the popup.
8. **Long-list scrolling** — If more than ~12 programs are registered, confirm the popup caps height and scrolls vertically.
9. **Failure message line** — If a launch can be made to fail (e.g., handler executable removed), confirm the pill shows `"Windows could not open this file with the selected program."`.
10. **Visual/DPI sanity** — Check that the glyph aligns with adjacent pill glyphs, the popup opens above the button, and scaling on high-DPI displays is sane.

---

## Junior-developer 50% restriction check

The implementer did not overstep the 50% restriction:

- All structural decisions (placement Option A, popup above, empty-result disabled tooltip, verbatim display names, 300-point scroll cap, no hotkey, failure message line) are user-approved/plan-pinned.
- The new app-state field, the two enumeration refresh points, the module split, and the `PillButtonPlacement` tooltip override follow the plan exactly.
- The only post-plan change is the `fa0a6ac` simplification, which is a local refactor of internal helper shape and does not alter behavior or architecture.

---

## What was NOT done

- No source code was modified.
- No existing `.kilo/plans` file was edited.
- No `git` commit, push, or branch operation was performed.
- No `cargo`/`docker`/build/test command was run (front-end verification is static code review only; the `ALL CHECKS PASSED` / 177-test census status is taken from the task context and the implementation plan).
- No manual GUI/runtime verification was attempted (impossible from the Linux container; listed as OPERATOR-RUN items).

---

**Report file:** `.kilo/plans/20261112-wave6-task3-frontend-verification.md`
