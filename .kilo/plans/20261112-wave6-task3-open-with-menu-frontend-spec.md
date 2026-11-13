# Wave 6 Task 3 — "Open with..." Menu Frontend Technical Specification

**Date:** 2026-11-13  
**Branch:** `feat/wave-6-open-with`  
**Version target:** 0.6.0  
**Step:** Critical Workflow 4.1a (frontend specialist)  
**Source TODO:** `.agent/todos/20261029/20261029-todo-8.md` Task 3  
**Binding requirements:** `.agent/project-info/brief.md` → `### Fast "Open with..." Menu`

## 1. Scope

This spec covers only the **viewer UI entry point** for the Wave 6 "Open with..." feature. It consumes the already-landed service seams and proposes concrete placement options. The user/planner must approve one option before the 4.1b implementation plan is written.

Out of scope for this task:
- Runtime enumeration mechanism (Task 1, landed in `src/open_with_listing.rs` + `src/platform/`).
- Launcher hand-off mechanism (Task 2, landed in `src/platform/`).
- Documentation updates (Task 5).

## 2. Service seams consumed by the UI

The menu UI must use these seams exactly as they exist; no new service behavior is required.

| Seam | Location | Contract |
|------|----------|----------|
| `RegisteredProgram { display_name, launch_index }` | `src/open_with_listing.rs` | One menu row. `display_name` is shown verbatim. `launch_index` is passed to the launcher, which re-enumerates fresh. |
| `programs_from_handler_names` | `src/open_with_listing.rs` | Pure filter/dedup used by the Windows enumerator; already unit-tested. |
| `enumerate_open_with_programs(file_path)` | `src/platform/` facade | Returns `Vec<RegisteredProgram>` for the current file. Stub returns empty. |
| `open_file_with_selected_program(file_path, launch_index)` | `src/platform/` facade | Hands off to the selected program. Returns `Result<(), OpenWithLaunchFailure>`. |
| `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` / `OpenWithLaunchFailure::user_message` | `src/open_with_listing.rs` | Static failure text surfaced in the pill message line. |

## 3. App-state additions

To keep the implementation deterministic and avoid per-frame OS enumeration, the UI is allowed a **path-keyed cache of the display list only**. The launcher still re-enumerates on every hand-off, satisfying the "no service-layer cache" rule.

Add to `ImagesScrollerApp` (`src/app.rs`):

```rust
/// Programs registered for the currently opened file, refreshed on every
/// successful open. The launcher performs its own fresh re-enumeration;
/// this field is display-state only.
open_with_programs: Vec<RegisteredProgram>,
```

Refresh rule: in `accept_open_outcome`, after a successful decode, call
`platform::enumerate_open_with_programs(&path)` and store the result in
`open_with_programs`. Clear the vector on empty-state reset and empty-folder
mode entry.

No persistent open/closed state is required if the built-in `egui::Popup::menu`
API is used (recommended); that API stores the popup open/closed state in egui
memory.

## 4. Placement options

### Option A — 11th pill button (RECOMMENDED)

Add one more button to the existing centered floating pill, after the fit-width /
fit-height group. The button opens an upward popup menu listing the registered
programs.

#### Visual design
- Position: rightmost button in the pill row, separated by a 16 pt group gap
  matching `FULLSCREEN_GROUP_GAP` / `FIT_GROUP_GAP`.
- Size: same as all pill buttons, `36.0 × 32.0` points.
- Glyph (painted vector shape, no font glyph): an "open with" / external-launch
  icon — a 10×10 point square with a 7-point arrow emerging from its top-right
  corner and pointing up-right. The square is stroked/filled like the fullscreen
  square; the arrow is a short line plus a small triangle from the existing
  `outlined_triangle_shape` family. The glyph uses the standard pill palette:
  white fill, 1-point black outline.
- Caption: **none** for v1. There is no dedicated keyboard hotkey, so the
  Wave-4.1 affordance pattern does not apply.
- Tooltip: `"Open with..."`.
- Accessible label: `"Open with..."`.

#### egui 0.37 API mechanics
- Use the same `ButtonColumnSite` + `show_button_column` wrapper as the other
  pill buttons.
- After painting the custom button, obtain its `Response` and drive the popup
  with the egui 0.37 `Popup` builder:
  - `egui::Popup::menu(&button_response)`
  - `.align(egui::RectAlign::TOP)` — default placement centered above the
    button; egui automatically flips to `MENU_ALIGNS` alternatives if it does
    not fit.
  - `.accessible_name("Open with...")`
  - `.show(|ui| { /* menu items */ })`
- Menu entries inside the closure: `ui.button(&program.display_name)`.
- Long lists: wrap entries in
  `egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| { ... })`.
- Dismissal: `Popup::menu` automatically closes on item click, outside click,
  and `Esc`; no manual state is required.

#### Interaction flow
1. User clicks the pill button.
2. egui opens the popup above the button.
3. The closure calls `platform::enumerate_open_with_programs(self.opened_image_path)`
   (or reads the already-cached `open_with_programs` list) and renders one
   `ui.button(...)` per entry.
4. Clicking an entry calls
   `platform::open_file_with_selected_program(path, program.launch_index)`.
5. On `Err`, set `self.message_text = Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)`.
6. On success, the popup closes automatically (item click); the app stays on the
   current image.

#### Enablement / visibility rules
- **Fresh empty state:** pill is not rendered at all → entry unavailable.
- **Image open:** button enabled if `open_with_programs` is non-empty; disabled
  (dimmed, no menu) if the system returned no programs.
- **Empty-folder browsing:** `image_is_open()` is false → button disabled.
- **Fullscreen ON:** `full_ui_chrome_is_visible()` is false → pill and menu are
  hidden entirely.

#### Pros / cons
- **Pros:** Consistent with the existing pill UI; highly discoverable; reuses
  the established button-hint/accessibility machinery; no changes to the image
  surface or input handling.
- **Cons:** Adds one more button to an already 10-button pill; increases pill
  width by ~52 points (button + gap). Given the always-maximized launch window,
  this is acceptable.

---

### Option B — Right-click context menu on the central image surface

Attach a context menu to the `Response` of the displayed image. Right-clicking
anywhere on the image opens the program list at the pointer.

#### Visual design
- No new chrome. The menu appears at the cursor position like a standard
  context menu.
- Menu entries are the same verbatim `display_name` text buttons as Option A.
- Long lists use the same `ScrollArea` cap.

#### egui 0.37 API mechanics
- Modify the central image paint path so the image widget returns a
  `Response` (e.g., wrap the `egui::Image` call or the `ui.put` call in a
  `ui.interact`/`Response`).
- Call egui 0.37 `Response::context_menu` on that response:
  ```rust
  image_response.context_menu(|ui| {
      // render entries
  });
  ```
- The closure contents are identical to Option A: one button per
  `RegisteredProgram`, `ScrollArea` for long lists.
- `Response::context_menu` automatically opens on secondary click, closes on
  item/outside/Esc, and anchors at `PopupAnchor::PointerFixed`.

#### Interaction flow
Same as Option A, but triggered by secondary click on the image instead of a
pill button click.

#### Enablement / visibility rules
- **Fresh empty state:** no image is painted → no image `Response` → menu
  unavailable.
- **Image open + chrome visible:** context menu is attached.
- **Fullscreen ON:** the fullscreen layout also paints the image; the spec
  proposes **not** attaching the menu in fullscreen so controls stay hidden
  (`full_ui_chrome_is_visible()` gate around the `context_menu` call).
- **Empty-folder browsing:** no image is open → no menu.

#### Pros / cons
- **Pros:** Zero pill-width growth; familiar "right-click → Open with" pattern.
- **Cons:** Much less discoverable in a keyboard-first classic viewer; couples
  the menu to the image paint seam, requiring a `Response` return change;
  conflicts with any future mouse pan/wheel work (out of v1 scope but a likely
  future idea); inconsistent with the pill-based control surface the app uses
  for every other action.

---

### Option C — Both pill button and right-click context menu

Implement Option A **and** Option B, sharing one helper that builds the menu
contents. The pill button gives discoverability; the right-click gives a
power-user shortcut.

#### Visual design / API mechanics
- Pill button: identical to Option A.
- Context menu: identical to Option B.
- Both call a shared function, e.g., `open_with_menu::show_program_list(app, ui)`.

#### Enablement / visibility rules
Same per-surface rules as A and B.

#### Pros / cons
- **Pros:** Maximum discoverability and flexibility.
- **Cons:** Two code paths to maintain and test; duplication of surface area
  contradicts the wave's classic-viewer simplicity goal; the context-menu path
  still couples to the image paint seam.

---

## 5. Recommended option

**Option A — 11th pill button.**

Rationale:
- It matches the established pill-based control language of the app (every
  other viewer action already lives in the pill).
- It is discoverable without requiring a mouse right-click discoverability cue.
- It avoids altering the image surface or input handling.
- The pill-width increase is acceptable because the window launches maximized.
- Fullscreen behavior is trivially correct: the chrome gate hides everything.

## 6. Menu list UX

### Entry text
Each menu row shows `RegisteredProgram::display_name` verbatim. No icons, no
re-sorting, no truncation.

### Long lists
Wrap the entry list in `egui::ScrollArea::vertical().max_height(300.0)` (about
10–12 entries). The popup width grows to the widest entry. No pagination.

### Empty-enumeration behavior (options for the planner/architector)

| Option | Behavior | When enumeration runs |
|--------|----------|----------------------|
| A1 | Hide the pill button entirely when `open_with_programs` is empty. | Every frame while an image is open (expensive unless cached). |
| A2 (recommended) | Keep the pill button visible but **disabled**, with tooltip `"No programs registered for this file type"`. Menu never opens. | Once per successful open (cached in app state). |
| A3 | Keep the button enabled; clicking it opens a menu containing one disabled placeholder item `"(No programs registered)"`. | Only when the menu is opened. |

**Recommendation:** A2. It communicates the empty state clearly without
surprising the user with an empty menu, and it is cheap because the list is
refreshed only when the opened file changes.

## 7. Failure feedback

If `open_file_with_selected_program` returns `Err`:

```rust
self.message_text = Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE);
```

This surfaces in the pill's message line, which is visible while an image is
open. This fits the existing single-message surface; it is the same pattern
used for `ImageLoadFailure`. No additional toast, dialog, or sound is added.

## 8. Modules to touch (summary)

| File | Change |
|------|--------|
| `src/app.rs` | Add `open_with_programs` field; populate on successful open; add launcher helper method. |
| `src/app/button_hint.rs` | Add `PillButton::OpenWith`; tooltip `"Open with..."`; no caption. |
| `src/app/button_hint_tests.rs` | Add assertions for the new enum member. |
| `src/app/floating_controls.rs` | Render the 11th pill button after the fit group with a group gap. |
| `src/app/open_with_menu.rs` (new) | Button paint + popup contents helper. |
| `src/app/open_with_menu_tests.rs` (new) | Pure-logic tests for enablement, entry mapping, empty placeholder, failure message. |

If Option B or C is chosen, also touch:
| `src/app/central_surface.rs` / `src/app_display.rs` | Return a `Response` from the image paint path and attach `context_menu`. |

## 9. Expected unit-test additions

New test file: `src/app/open_with_menu_tests.rs`.

Approximate test census:

- `open_with_menu_available_when_image_open_and_chrome_visible`
- `open_with_menu_unavailable_when_no_image`
- `open_with_menu_unavailable_when_fullscreen_on`
- `open_with_menu_unavailable_in_empty_folder_mode`
- `open_with_menu_lists_programs_verbatim`
- `open_with_menu_shows_no_programs_placeholder_when_empty`
- `open_with_launch_failure_sets_message_text`

**Estimated census delta:** current **170 tests / 27 files** → **176 tests / 28 files** (+6 tests, +1 test file). The exact count may shift by ±1 depending on how the empty-state and failure tests are grouped.

## 10. 4.5a conformance checklist

Verification will check the landed implementation against this spec:

- [ ] `PillButton::OpenWith` exists in `button_hint.rs` with tooltip `"Open with..."` and `hotkey_reference(...) == None`.
- [ ] The new pill button is 36×32 points, uses the shared `ButtonColumnSite`, and paints a custom vector glyph (white fill, 1-point black outline).
- [ ] The button sits at the right end of the pill row, separated by a 16 pt group gap.
- [ ] Menu is driven by `egui::Popup::menu(&button_response)` (or the approved Option B/C equivalent).
- [ ] Menu default placement is above the button (`egui::RectAlign::TOP`) with egui's fallback alignment.
- [ ] Menu entries are `ui.button(&program.display_name)` verbatim.
- [ ] Long lists are wrapped in `egui::ScrollArea::vertical().max_height(300.0)`.
- [ ] Empty enumeration renders a disabled button with tooltip `"No programs registered for this file type"` (Option A2) or the chosen architector decision.
- [ ] Selecting an entry calls `platform::open_file_with_selected_program(path, launch_index)`.
- [ ] On launch error, `message_text` is set to `OPEN_WITH_LAUNCH_FAILURE_MESSAGE`.
- [ ] Button/menu is unavailable when no image is open, in empty-folder mode, and when fullscreen is ON.
- [ ] `open_with_programs` is refreshed only on successful open (no per-frame enumeration).
- [ ] Test census matches the estimated 176/28 (±1).
- [ ] No per-frame texture re-upload or allocation churn is introduced.

## 11. Open decisions awaiting user / planner approval

1. **Which placement option?** (A recommended, B, or C)
2. **Which empty-enumeration behavior?** (A2 recommended)
3. **Keyboard hotkey?** Recommended: none for v1.
4. **Exact popup max-height** (proposed 300 points) and placeholder wording.

---

**Spec file:** `.kilo/plans/20261112-wave6-task3-open-with-menu-frontend-spec.md`
