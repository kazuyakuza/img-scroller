# Task-3 Plan — Wave 3, Task 3: Vertical Navigation (Sibling Folders) — Keyboard + Buttons (4.1b)

- Date: 2026-11-01
- Source TODO (binding): [`.agent/todos/20261029/20261029-todo-5.md`](../../.agent/todos/20261029/20261029-todo-5.md) → `### Task 3` + `## Deferred decisions` (this step settles ALL of them)
- Front-end input (4.1a, READ and consumed): [`.kilo/plans/20261101-wave3-task3-vertical-nav-frontend-spec.md`](20261101-wave3-task3-vertical-nav-frontend-spec.md)
- Global plan (read-only reference): [`.kilo/plans/20261101-wave3-vertical-scroll.md`](20261101-wave3-vertical-scroll.md) → "Task 3" pre-analysis
- Prior landed wave tasks (context, NOT re-planned): Task 1 `sibling_folder_listing.rs` (+ tests, 36-census), Task 2 `last_viewed_images.rs` (+ hook, 41-census)
- Step of the Critical Workflow: 4.1b (design + plan only). 4.2 is write-only (NO cargo, NO VM). Exercise/dev-checks belong to Task 4.

## 0. Ground Truth (verified 2026-11-01, current landed state)

| File | Lines | Relevant facts |
| --- | --- | --- |
| `src/app.rs` | **199/200 HARD CAP** | `NavigationDirection { Previous, Next }` (line 31, private, no derives); `navigate` (87-91) → single seam via `neighbor_image_path` (93-108, 16 lines); `accept_open_outcome` Ok arm: listing rebuild (66) → Task-2 record hook (67) → `opened_image_path = Some(path)` (68); `empty_state()` inits; per-frame intake in `ui()` (158-165: two `let *_pressed` + two `if` blocks) BEFORE the gate; bottom mods `arrow_direction; floating_controls; tests; floating_controls_tests; last_viewed_tests`; bottom consts 174-177 |
| `src/app/arrow_direction.rs` | 65 | `ArrowControl::{ImagePrevious, ImageNext, FolderUp, FolderDown}`; `navigation_direction()` returns `None` for `FolderUp | FolderDown` (line 58-64); labels `"Up folder"` / `"Down folder"` (48-55) |
| `src/app/floating_controls.rs` | 170 | `navigation_button_enabled(app, direction) = app.neighbor_image_path(direction).is_some()` (24-29); `arrow_button_is_enabled` (96-101) calls it per `ArrowControl` (so enablement is FULLY generic — needs no change); `show_and_navigate` → `app.navigate(direction)`; module header line 3-4 says ↑/↓ are "Wave-3-reserved disabled pair" (stale after this task); line 143 comment "(the Wave-3 reserve stays handler-free)" |
| `src/app/tests.rs` | 187/200 | Fixture style: real temp dirs (`TempImageFolder`, `create_file`, `image_path`, `Drop`), success simulated by `app.accept_open_outcome(path, Ok(sample_decoded_image()))` (container stub decode ALWAYS fails); navigation assertions via `app.neighbor_image_path(direction)` and `app.navigate(direction)` |
| `src/app/floating_controls_tests.rs` | 32 | 1 test: `arrow_button_enablement_follows_the_navigation_edges` (horizontal, via `navigation_button_enabled`) |
| `src/app/last_viewed_tests.rs` | 78 | 5 tests; reach `app.last_viewed_images.last_displayed_image(folder)` from a sibling child module (privacy precedent) |
| `src/sibling_folder_listing.rs` | 133 | Task-1 API (EXACT): `from_opened_image(&Path) -> Option<Self>` (Option: current folder unanchorable → None; unreadable/unempty-able scope → EMPTY listing); `previous_neighbor_of(&self, opened_image_path)` / `next_neighbor_of(&self, opened_image_path)` take the OPENED IMAGE path verbatim (they canonicalize the anchor INTERNALLY — no caller canonicalization needed) and return `Option<ListedFolderNeighbor>`; `ListedFolderNeighbor::{At { listed_index, folder_path }, ListBeginningReached, ListEndReached}` — `folder_path` is CANONICAL-scope-derived; `listed_folder_count()` used only by module tests; one injectable reader seam (`from_opened_image_with_scope_entries`) exported to its child tests via `super::`. `#[allow(dead_code)]` on: enum (21), `listed_index` field (24), `from_opened_image` (38), `listed_folder_count` (43), `previous_neighbor_of` (48), `next_neighbor_of` (60) |
| `src/sibling_folder_listing/tests.rs` | **200/200 (AT CAP)** | 8 tests; UNTOUCHED by this task (no budget for edits) |
| `src/last_viewed_images.rs` | 40 | `record_displayed_image(&Path)` key = `path.parent()` verbatim, NEVER canonicalized (Task-2 §2.2 contract + its reviewer grep `canonicalize` → no hits); `last_displayed_image(&Path) -> Option<&Path>` carries the single Task-3 marker (31); `is_empty()` is `#[cfg(test)]`-only |
| `src/folder_listing.rs` | 117 | 17-extension whitelist; `from_opened_image` canonicalizes the FOLDER internally (listed paths are canonical-parent entries); accessors: `listed_image_count` (marked: tests-only), `listed_index_of`, `previous_neighbor(i)`, `next_neighbor(i)`. **NO first-element accessor exists yet** |
| `src/image_loading.rs` | 86 | `load_image`; every failure → the single `UNSUPPORTED_FILE_MESSAGE`; container stub ⇒ every real `open_image` FAILS in tests |
| Test census | **41** baseline (Task-2 landed): app 9, floating_controls 1, folder_navigation-tests 0, folder_listing 5, sibling 8, last_viewed 5, name_collation 2, image_loading 4, app_display 2, scaling 5 |
| dead_code markers | 12 today: sibling 6, last_viewed 1, folder_listing 2, image_loading 3 |

## 1. High-Level Approach (single seam, both axes)

Wave-2's shape extends rather than forks: the pill's ↑/↓ slots and the `ArrowUp`/`ArrowDown` keys all flow through the ONE `navigate(NavigationDirection)` seam whose resolution gains a folder axis:

1. `NavigationDirection` gains two variants — **`PreviousSiblingFolder`** / **`NextSiblingFolder`** — alongside `Previous` / `Next`.
2. The shared resolution `neighbor_image_path` becomes a CROSS-AXIS dispatch: image variants step `current_folder_listing` ±1 (existing Wave-2 logic, moved verbatim); folder variants step `current_sibling_folders` ±1 via the Task-1 API, then apply the BINDING target-image priority (visited this session → `last_displayed_image(target)`; otherwise ALWAYS first whitelisted image via the Task-1 § 7 placeholder trick on `FolderListing`; empty/unsuccessful target → no neighbor per D2).
3. The per-frame pill enablement (`navigation_button_enabled`) and per-frame key dispatch need NO generic changes — they already call the seam per `ArrowControl::navigation_direction()`.
4. Every vertical open routes through `open_image` → `accept_open_outcome` → Task-2's record hook, so "the newly shown image updates the map" is automatic (verified; one canonicalization line added to `open_image`, see § 3.2/D5).
5. `app.rs` is 1 line under the cap → the whole seam (`navigate`, `neighbor_image_path`, both resolvers, the priority rule) plus the `NavigationDirection` enum extract into ONE new child module `src/app/folder_navigation.rs` (the step's sanctioned extraction). app.rs drops to 195 with 5 lines headroom; call sites (`self.navigate(...)`, `app.neighbor_image_path(...)`) are UNCHANGED everywhere.

## 2. Proposed deferred-decision resolutions (Planner: present this package to the user right after)

### D1 — UI layout placement of ↑/↓ buttons: ACTIVATE THE EXISTING SLOTS IN PLACE (CONFIRMED PROPOSAL)

- The 0.2.1 operator-directed floating bottom pill already renders `[←] [→] [↑] [↓]` with ↑/↓ as painted, visible, DISABLED Wave-3 reserves. Wave 3 wires the two existing slots in place: zero layout change, zero new widgets, same `Area` id/anchor/frame, same message line, same button geometry (4.1a spec §3.2 + §7 items 6-8).
- Concrete mechanics: `ArrowControl::FolderUp`/`FolderDown` map to the new `NavigationDirection::PreviousSiblingFolder`/`NextSiblingFolder` (§ 3.4); the pill's generic `arrow_button_is_enabled`→`navigation_button_enabled`→`neighbor_image_path` chain then enables/dims them per frame with NO `floating_controls.rs` logic change.
- Rationale: pre-settled by operator directive (global plan pre-analysis bullet 4; context.md "↑/↓ reserved until Wave 3"); activating in place also satisfies 4.5a checklist items verbatim.

### D2 — Empty/no-image sibling folder: treat as NO NEIGHBOR (disable the direction; silence)

- Behavior chosen: **disable** (4.1a spec §4.3 option 1). A sibling folder that yields no target through the priority chain resolves to `None` → the corresponding ↑/↓ button is dimmed/disabled and `navigate` does NOTHING: no navigation, no message, current image stays. This is consistent with the Wave-2 horizontal edge precedent and with the pill's single fixed message semantics (it messages only unsupported OPEN failures — never "nothing here").
- Exact resolution contract (what "no image" means mechanically): adjacent folder F is reachable only if `last_displayed_image(F)` is `Some` (priority (a)) XOR falls back to `FolderListing::first_listed_image(F)` (priority (b)) — if the lookup is `None` AND the folder lists NO whitelisted image (empty, files-only, unreadable scope mapping to empty listing) → `sibling_folder_image_path` returns `None`.
- Consequence that MUST be understood (and is test-locked): with skipping explicitly NOT chosen, an image-less folder is a HARD barrier — e.g. scope `[folder1 (imgs), folder2 (no imgs), folder3 (imgs)]`: from folder1, `nexts` is disabled although folder3 exists; folder3 can never be reached by ↑ without moving to it first. This is the cost of the disable option, accepted for determinism and one-step-arrow semantics.
- What previous/next resolution returns for such an adjacent folder: `Some(ListedFolderNeighbor::At { folder_path })` from the Task-1 API (the folder EXISTS in the parent scope), but the app-level `neighbor_image_path(direction)` returns `None` after the priority chain finds no displayable target.
- Visited-folder-edged case (prompt D2 sub-question): if a folder WAS visited this session but its recorded last-viewed image has since been DELETED, the priority chain still returns the stale path (`Some`) → the button stays ENABLED and pressing it attempts the open → the open FAILS through the untouched Wave-1 failure rule (current image kept + `UNSUPPORTED_FILE_MESSAGE`). No special probing, no map rewrite (record only succeeds). Test 6 locks this.
- Keyboard behaves identically (same seam, unconditional intake, resolution `None` → no-op).

### D3 — Edge-of-list behavior: MIRROR WAVE 2 (first/last sibling stays silently)

- At the first/last sibling under the same parent, navigation STAYS SILENT: the Task-1 edge variants `ListBeginningReached`/`ListEndReached` decode to `None` in the app-level seam → no wrap, no message, button disabled that frame. No new behavior types are introduced. (Locked as test 5; consistent with the horizontal precedent and the global plan pre-analysis.)

### D4 — Unit-test seams around folder I/O: REAL TEMP-DIRECTORY CONSTRUCTION ONLY (single approach)

- Chosen approach: app-level vertical tests build real parent scopes with the EXISTING public constructors (`SiblingFolderListing::from_opened_image` via the Ok-arm rebuild; `FolderListing` internally for first-image fallback) using a small real-fs fixture `TempParentScope` in a NEW child test file — EXACTLY the pattern `src/app/tests.rs` uses for horizontal tests (`TempImageFolder` real temp dirs + `app.accept_open_outcome(path, Ok(sample_decoded_image()))` to simulate opens; never a real decode, which always fails in the container).
- Grounding: every horizontal navigation test in `src/app/tests.rs` resolves through REAL listings (canonicalize identity holds on Linux `/tmp` fixture paths — the existing 9 tests depend on it and pass); `sibling_folder_listing/tests.rs` already proves the listing module's fault paths through its OWN `ScopeEntriesReader` seam, so an app-level injection would duplicate machinery without new coverage.
- NO app-level injection seam, NO test-injector constructor/setter, no `ScopeEntriesReader` exposure beyond `super::` (existing). Test 4's unreadable-scope expectation is covered at module level (its test 7), not duplicated.

### D5 — (ADDITIONAL surfaced amendment, flagged for the user) one canonicalization line in `open_image`

- **Situation found during the mandated "verify the hook" check:** the last-viewed map keys are the VERBATIM `parent()` of each opened path (Task-2, no canonicalization), while Task-1's sibling listing necessarily yields CANONICAL target-folder paths. On real Windows, `fs::canonicalize` returns `\\?\`-prefixed verbatim paths, so a session that starts with a dialog/CLI-opened image would record key `C:\photos\a` but later look up `\\?\C:\photos\a` — the priority-(a) lookup would MISS and mis-open the first image instead of the last-viewed, breaking the BINDING rule for every non-listing-born folder (including the very first folder).
- **Chosen fix (`Yes`):** `open_image` canonicalizes the candidate path once after a successful decode and feeds the canonical path into `accept_open_outcome` (§ 3.2). Every recorded key/value then shares the canonical frame that the sibling listing and the first-image fallback already produce; container tests are unaffected (identity canonicalization on `/tmp` fixtures — the landed horizontal tests already rely on it); direct `accept_open_outcome` calls (tests) stay verbatim and still match by identity.
- Scope cleanliness: the Task-2 hook line `self.last_viewed_images.record_displayed_image(&path);` stays BYTE-IDENTICAL (the `path` now arrives canonical); `last_viewed_images.rs` is untouched (its reviewer grep `canonicalize` → no hits still holds); the normalization enters at the WAVE-1 `open_image` seam, and the resolution functions add NO canonicalization (prompt item 7 honored — anchors pass verbatim into the Task-1 API).
- If the user REJECTS D5: do NOT let the implementer "just drop the line" silently — that re-opens the binding-rule bug; Planner must send Task 3 4.1b back for a re-plan.

## 3. Exact Code Changes

### 3.1 NEW FILE `src/app/folder_navigation.rs` (the extraction; ~85 lines, cap fine)

Inherent `impl ImagesScrollerApp` in a child module is legal; the child can see the app's private FIELDS and the private `NavigationDirection`... — per the chosen layout the ENUM LIVES HERE (budget math, § 3.6). Item order: header doc → uses → `const` → enum → inherent impl (`navigate`, `neighbor_image_path`) → private resolvers.

```rust
//! Shared sibling-navigation seam for the arrow keys and the pill buttons:
//! the ±1 neighbor target path per `NavigationDirection` on both axes -- the
//! current folder's image listing (horizontal) and the sibling-folder listing
//! under the same parent (vertical), with the vertical target-image priority:
//! a folder visited this session reopens its last-viewed image, any other
//! folder ALWAYS opens its first whitelisted image in the fixed v1 collation
//! (no same-filename attempt, no other fallback; a folder whose priority
//! chain yields nothing resolves to no neighbor: deferred decisions D2/D3).

use crate::folder_listing::{FolderListing, ListedNeighbor};
use crate::sibling_folder_listing::ListedFolderNeighbor;
use std::path::{Path, PathBuf};

use super::ImagesScrollerApp;

/// Placeholder entry name used ONLY to aim `FolderListing::from_opened_image`
/// at the TARGET folder (the path's parent IS that folder; the name itself is
/// never read). Task-1 plan section 7's first-image trick.
const FIRST_IMAGE_LISTING_PLACEHOLDER: &str = "first_image_anchor.bin";

/// Arrow directions of the shared navigation seam: the image axis steps the
/// current folder's sibling images; the folder axis steps the sibling folders
/// under the same parent.
pub(super) enum NavigationDirection {
    Previous,
    Next,
    PreviousSiblingFolder,
    NextSiblingFolder,
}

impl ImagesScrollerApp {
    /// Navigates one sibling neighbor (image or folder axis) per direction;
    /// a failed neighbor open keeps the current image (Wave-1 failure rule).
    pub(super) fn navigate(&mut self, direction: NavigationDirection) {
        if let Some(neighbor_path) = self.neighbor_image_path(direction) {
            self.open_image(neighbor_path);
        }
    }

    /// The ±1 neighbor target behind the keyboard intake and both pill-button
    /// rows; every "nothing to navigate to" case (no open image, no listing,
    /// unresolvable anchor, list edge, folder whose priority chain yields
    /// nothing) resolves to None. Also drives per-frame arrow enablement.
    fn neighbor_image_path(&self, direction: NavigationDirection) -> Option<PathBuf> {
        match direction {
            NavigationDirection::Previous | NavigationDirection::Next => {
                listed_image_neighbor(self, direction)
            }
            NavigationDirection::PreviousSiblingFolder
            | NavigationDirection::NextSiblingFolder => {
                sibling_folder_image_path(self, direction)
            }
        }
    }
}

fn listed_image_neighbor(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> Option<PathBuf> {
    let opened_path = app.opened_image_path.as_ref()?;
    let listing = app.current_folder_listing.as_ref()?;
    let listed_index = listing.listed_index_of(opened_path)?;
    let neighbor = match direction {
        NavigationDirection::Previous => listing.previous_neighbor(listed_index),
        NavigationDirection::Next => listing.next_neighbor(listed_index),
        NavigationDirection::PreviousSiblingFolder | NavigationDirection::NextSiblingFolder => {
            return None;
        }
    };
    let ListedNeighbor::At { image_path, .. } = neighbor else {
        return None;
    };
    Some(image_path)
}

/// Resolves the sibling-folder neighbor for the direction and maps the reached
/// folder onto its target image; the OPENED PATH is passed VERBATIM to the
/// Task-1 API (it canonicalizes the anchor internally; no caller
/// canonicalization).
fn sibling_folder_image_path(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> Option<PathBuf> {
    let listing = app.current_sibling_folders.as_ref()?;
    let opened_path = app.opened_image_path.as_ref()?;
    let folder_neighbor = match direction {
        NavigationDirection::PreviousSiblingFolder => listing.previous_neighbor_of(opened_path),
        NavigationDirection::NextSiblingFolder => listing.next_neighbor_of(opened_path),
        NavigationDirection::Previous | NavigationDirection::Next => return None,
    }?;
    let ListedFolderNeighbor::At { folder_path, .. } = folder_neighbor else {
        return None;
    };
    target_image_path(app, &folder_path)
}

/// Target-image priority (TODO Task 3, binding): visited this session -> its
/// recorded last-viewed image; otherwise ALWAYS the folder's FIRST whitelisted
/// image in the fixed collation. NO same-filename attempt, NO other fallback;
/// nothing resolvable -> None (D2).
fn target_image_path(app: &ImagesScrollerApp, target_folder_path: &Path) -> Option<PathBuf> {
    if let Some(last_viewed_path) = app.last_viewed_images.last_displayed_image(target_folder_path) {
        return Some(last_viewed_path.to_path_buf());
    }
    first_image_path_of_folder(target_folder_path)
}

fn first_image_path_of_folder(target_folder_path: &Path) -> Option<PathBuf> {
    let placeholder_path = target_folder_path.join(FIRST_IMAGE_LISTING_PLACEHOLDER);
    let folder_listing = FolderListing::from_opened_image(&placeholder_path)?;
    folder_listing.first_listed_image()
}
```

Rules check: methods ≤ 10 lines each; every fn ≤ 2 args (`&ImagesScrollerApp` + 1); depth ≤ 2; `pub(super)` ONLY for the enum and the two methods the app/other children need; no compound conditions; the two `match`-arm `return None` arms are explicit direction guards, not commented-out code.

### 3.2 EDIT `src/app.rs` (file cap: 199 → **195**; exact delta ledger)

| # | Change | Δ lines |
| --- | --- | --- |
| 1 | Import after `use crate::platform;` (line 11): `use crate::sibling_folder_listing::SiblingFolderListing;` | +1 |
| 2 | Trim the folder_listing import to `use crate::folder_listing::FolderListing;` (`ListedNeighbor` leaves app.rs with the resolver) | 0 |
| 3 | Field after `last_viewed_images` (line 23), with a one-line doc: `/// Sibling folders under the opened image's parent (rebuilt on every successful open).` + `current_sibling_folders: Option<SiblingFolderListing>,` | +2 |
| 4 | `empty_state()`: `current_sibling_folders: None,` after `last_viewed_images: LastViewedImages::empty(),` | +1 |
| 5 | `open_image` gains ONE line between `let outcome = …` and `self.accept_open_outcome(...)`: `let opened_path = std::fs::canonicalize(&path).unwrap_or_else(\|_\| path.clone());` and the call becomes `self.accept_open_outcome(opened_path, outcome);` (D5; fully-qualified `std::fs::` so NO new import) | +1 |
| 6 | `accept_open_outcome` Ok arm: after the UNTOUCHED Task-2 hook line, BEFORE `self.opened_image_path = Some(path);` insert the wrapped statement `self.current_sibling_folders =` / `SiblingFolderListing::from_opened_image(path.as_path());` (placement rationale: Task-2's documented hook adjacency preserved; both listings rebuilt before the display-state commit) | +2 |
| 7 | Delete `enum NavigationDirection { … }` (4 lines — moved to folder_navigation.rs) and delete methods `navigate` (5 lines) and `neighbor_image_path` (16 lines) — moved into the child inherent impl; add `use folder_navigation::NavigationDirection;` where the enum was | −4 +1 |
| 8 | `ui()`: after the existing `right_pressed` if-block, add `let up_pressed = ui.ctx().input(\|i\| i.key_pressed(egui::Key::ArrowUp));` + `let down_pressed = …(egui::Key::ArrowDown));` + `if up_pressed { self.navigate(NavigationDirection::PreviousSiblingFolder); }` + `if down_pressed { self.navigate(NavigationDirection::NextSiblingFolder); }` — SAME pattern as `ArrowLeft`/`ArrowRight`, before the `image_is_open()` branch (4.1a spec §2.1/§2.2; no pan interplay, no key-repeat handling) | +8 |
| 9 | After `mod arrow_direction;` add `mod folder_navigation;` (+ its blank line) | +2 |
| 10 | At the bottom test-mod block append `#[cfg(test)]` + `mod folder_navigation_tests;` (+ adjacent blank) | +3 |

Sum: 199 +1 +0 +2 +1 +1 +2 −4 +1 +8 +2 +3 −21 = **195 ≤ 200** (−21 = navigate 5 + neighbor_image_path 16). NOTHING else moves; `image_is_open`, layouts, consts, `open_via_dialog`, `empty_state` naming all unchanged. The Err arm of `accept_open_outcome` stays EXACTLY one line (`UNSUPPORTED_FILE_MESSAGE`) — failure interplay preserved (item 6 of the mandate).

Contingency (deterministic, in order, only if the post-write count exceeds 200): (a) drop the field doc line (−1); (b) place `mod folder_navigation;` directly adjacent to `mod arrow_direction;` without the blank line (−1). If STILL > 200: STOP and return a question — never violate the cap silently.

### 3.3 EDIT `src/folder_listing.rs` (117 → ~123; ONLY this addition)

After `listed_image_count` (grouped with accessors), add the first-element accessor (new resolution's priority-(b) fallback needs it; NO `first`-style accessor exists today and none may be synthesized via `next_neighbor(0)` which reports the SECOND entry):

```rust
    /// The FIRST whitelisted image in the fixed collation (the vertical
    /// navigation's new-folder target); None for an empty listing.
    pub(crate) fn first_listed_image(&self) -> Option<PathBuf> {
        self.image_paths.first().cloned()
    }
```

No marker (called immediately by `folder_navigation.rs`). Everything else byte-identical; `src/folder_listing/tests.rs` UNTOUCHED.

### 3.4 EDIT `src/app/arrow_direction.rs` (65 → ~66; UI contract of 4.1a §3.2/§5)

- `navigation_direction`: `Self::FolderUp => Some(NavigationDirection::PreviousSiblingFolder),` and `Self::FolderDown => Some(NavigationDirection::NextSiblingFolder),` replacing the `FolderUp | FolderDown => None` arm; replace its doc line with `/// Concrete direction per button; all four arrows navigate.` (spec §7 item 9-10).
- `accessible_label`: `"Up folder"` → `"Previous sibling folder"`; `"Down folder"` → `"Next sibling folder"` (spec §5).
- Update the two stale doc lines (27-29 header sentence and the `None for the Wave-3-reserved…` line) to the active state; the variant doc "disabled in Wave 2 and follow Wave 3" wording must not survive.
- NO layout, paint, cursor, or widget_info change.

### 3.5 EDIT `src/app/floating_controls.rs` (170 → 170; DOC-ONLY)

- Logic untouched: `arrow_button_is_enabled`, `navigation_button_enabled`, specs, click handling already work for the new directions through the generic chains.
- Update module header line 3-4 (↑/↓ are no longer a reserved disabled pair) and the line-143 parenthetical "(the Wave-3 reserve stays handler-free)" → "(a disabled arrow stays handler-free)". Conformance: spec §7 items 6-8, 11-12, 14-17 remain satisfied without logic edits.

### 3.6 EDIT `src/sibling_folder_listing.rs` (133 → 133; marker removals + KEEP list)

REMOVE `#[allow(dead_code)]` from EXACTLY: the `ListedFolderNeighbor` enum (line 21), `from_opened_image` (38), `previous_neighbor_of` (48), `next_neighbor_of` (60) — each gains a real production caller in `folder_navigation.rs`/the Ok arm.

KEEP the markers on EXACTLY: `At.listed_index` (24 — constructed but read only by module tests, so the field is dead in release builds) and `listed_folder_count` (43 — module-test-only accessor). `#[cfg(test)] mod tests;` and `src/sibling_folder_listing/tests.rs` (AT CAP) stay byte-identical.

### 3.7 EDIT `src/last_viewed_images.rs` (40 → 40; marker removal)

Remove `#[allow(dead_code)]` from `last_displayed_image` (line 31) — its caller `target_image_path` lands now. Everything else byte-identical (grep `canonicalize` in this file still yields no hits — D5 did NOT land here).

### 3.8 NEW FILE `src/app/folder_navigation_tests.rs` (~185 lines; hard cap 200)

Fixture `TempParentScope` (mirrors `src/app/tests.rs` style; real fs; prefix `img_scroller_folder_nav_{test_case}_{pid}`; `Drop` → `remove_dir_all`): `create(test_case)`, `create_child_folder(name)`, `create_child_file(folder_name, file_name)` (`fs::write(b"x")`), `child_image_path(folder_name, file_name)` → `PathBuf`. All `pub(super)`-free (private to this test file).

Imports: `use super::tests::sample_decoded_image;`, `use super::{ImagesScrollerApp, NavigationDirection};`, `use super::floating_controls::navigation_button_enabled;`, `use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;`, `use std::fs;`.

The 8 tests (names LOCKED; container-safe: success only via `accept_open_outcome`, real opens always fail through the stub):

1. `vertical_navigation_resolves_the_previous_and_next_sibling_folder_in_sort_order` — scope with `folder1/f1.png`, `folder2/f2.png`, `folder3/f3.png`; app opened `folder2/f2` via OK-accept; `neighbor_image_path(PreviousSiblingFolder)` == folder1/f1 AND `neighbor_image_path(NextSiblingFolder)` == folder3/f3.
2. `vertical_navigation_reopens_the_visited_folders_last_viewed_image` — `folder1{name1, name2}`, `folder2/single`; OK-accept `folder1/name2` (last-viewed = name2), then OK-accept `folder2/single`; from folder2: `neighbor_image_path(PreviousSiblingFolder)` == `folder1/name2` — NOT `name1` (priority (a) beats first-image; Addendum v2: no same-filename attempt exists anywhere in this chain).
3. `vertical_navigation_opens_the_first_image_of_a_new_folder` — `folder1/only.png` opened; `folder2{b.png, a.png}` unvisited; `neighbor_image_path(NextSiblingFolder)` == `folder2/a.png` (collation FIRST image, not sort-last, not same-name).
4. `vertical_navigation_treats_a_folder_without_images_as_no_neighbor` (D2) — scope `folder1/img.png`, `folder2` (files-only `stray.txt`), from folder1: `neighbor_image_path(NextSiblingFolder)` == None; `navigation_button_enabled(&app, NextSiblingFolder)` == false; `app.navigate(NextSiblingFolder)` leaves `opened_image_path`, `message_text` (None) and all state unchanged; prev/next around the empty folder never see THROUGH it (folder3 behind folder2 unreachable — assert with a third folder's image present that NextSiblingFolder stays None from folder1).
5. `vertical_navigation_stays_silent_at_the_sibling_list_edges` (D3) — scope `folder1/a.png`, `folder2/b.png`; from folder1: PreviousSiblingFolder == None; from folder2 (after OK-accept b): NextSiblingFolder == None; no message set at any point; single-child scope → both directions None.
6. `vertical_navigation_to_a_folder_whose_last_viewed_image_vanished_reports_the_standard_failure` — `folder1{keep.png, vanish.png}`, `folder2/live.png`; OK-accept `folder1/vanish` then OK-accept `folder2/live`; `fs::remove_file(vanish)`; `app.navigate(PreviousSiblingFolder)` → `message_text` == `UNSUPPORTED_FILE_MESSAGE`, `opened_image_path` still `folder2/live`, decoded payload still present (Wave-1 failure rule; map entry NOT rewritten by the failure).
7. `vertical_button_enablement_matches_the_folder_neighbor_resolution` — fresh app: both folder directions disabled; per the folder-edges/D2 shape (fixture of tests 4+5 combined): edge directions disabled exactly when `neighbor_image_path` is None, adjacent valid directions enabled (mirrors `floating_controls_tests.rs`' horizontal edges test, vertical axis; key/dispatch parity is covered by `app.navigate` driving the SAME resolution — assert equality of `navigation_button_enabled(dir)` and `app.neighbor_image_path(dir).is_some()` for all four directions at two app states).
8. `vertical_target_open_records_the_new_folders_last_viewed_memory` — `folder1{a, b}`, `folder2{c}`; OK-accept `folder1/b` → `last_displayed_image(folder2)` is None; OK-accept `folder2/c` (the "vertical open" simulation — the hook Task 3 relies on, exactly like `last_viewed_tests.rs`) → `last_displayed_image(folder2)` == Some(folder2/c) AND from folder2 the PreviousSiblingFolder resolution now returns `folder1/b` (visited-priority re-entry point back into the navigated folder).

Budget guard: if any edit pushes this file over 200 lines → STOP and return a question (same rule as Task-1's test file).

## 4. dead_code Marker Audit (mandate item 5 — exact census)

Before: sibling 6 (`sibling_folder_listing.rs` 21/24/38/43/48/60) + last_viewed 1 (31) + folder_listing 2 (22/53) + image_loading 3 (18/25/37) = 12. After Task 3:

- Removed (4+1=5): `sibling_folder_listing.rs` 21 (enum), 38 (`from_opened_image`), 48 (`previous_neighbor_of`), 60 (`next_neighbor_of`); `last_viewed_images.rs` 31 (`last_displayed_image`).
- Added (0): the new `first_listed_image` accessor has an immediate caller.
- Kept (7): sibling 24 (`listed_index`), sibling 43 (`listed_folder_count`), folder_listing 22 (`ListedNeighbor.listed_index`), folder_listing 53 (`listed_image_count`), image_loading 18/25/37 (`Unsupported`, `user_message`, `pixel_buffer_size`) — all test-only/unused, still correct as-is.
- Keyboard intake needs NO markers (plain let/if, no dead fields).
- NEW `folder_navigation.rs`/`folder_navigation_tests.rs`: no dead-code markers anywhere.

## 5. Tests & Census (mandate items 8-9)

- New tests: 8, ALL in the NEW `src/app/folder_navigation_tests.rs` (rationale: `src/app/tests.rs` is at 187/200 — 13 lines of headroom cannot absorb a 30-line sibling fixture; `floating_controls_tests.rs` stays untouched at 32 — its enablement coverage continues on the horizontal axis and the vertical counterpart lives beside the vertical fixture). Test names LOCKED in § 3.8; reviewers diff-check them char-for-char.
- Keyboard `key_pressed` intake itself is NOT unit-testable in the container (egui/no frame) — horizontal precedent likewise untested; 4.5a conformance checklist (spec §7) + Task-4 OPERATOR-RUN cover it.
- Census: 41 → **49** (+8) = app 9, folder_navigation_tests 8 (new), floating_controls 1, folder_listing 5, sibling 8, last_viewed 5, name_collation 2, image_loading 4, app_display 2, scaling 5. Dev-check runs (fmt/check/test/clippy through the Alpine-VM harness, `vm_status` FIRST, exit code authoritative, target `ALL CHECKS PASSED`) are Task-4 scoped — NEVER in 4.2.

## 6. Git (4.2 — mandate item 10)

- Branch already exists from Step 2: `feat/wave-3-vertical-scroll`. NO branch creation, NO push (push is Step-5-restricted, `main`→`origin` ONLY), no merges, no tags.
- Before staging: read `.gitignore`, run `git status` (Gitignore Compliance Rule) — nothing built in 4.2, so no `target/`/`dist/`/`logs/` risk; verify none staged anyway.
- Stage EXACTLY (9 paths): `src/app.rs`, `src/app/folder_navigation.rs`, `src/app/folder_navigation_tests.rs`, `src/app/arrow_direction.rs`, `src/app/floating_controls.rs`, `src/folder_listing.rs`, `src/sibling_folder_listing.rs`, `src/last_viewed_images.rs`, `.kilo/plans/20261101-wave3-task3-vertical-nav-navigation.md`
- Single commit, message EXACT: `feat: navigate sibling folders with up/down arrows and buttons`

## 7. 4.2 Junior-Implementer Step-by-Step Checklist (WRITE-ONLY — NO cargo/VM/build commands)

PREFLIGHT (stop and ask the caller if ANY fails):
1. `git branch --show-current` == `feat/wave-3-vertical-scroll`; `git status` → clean tree; `git log --oneline -5` shows Step-3 version bump (0.3.0) then Task-1 and Task-2 commits.
2. Re-read THIS plan + the 4.1a spec + `src/app/tests.rs` once more before writing (fresh name pressure check).

BUILD (in this order; write/edit tools only, real newlines, no `\n` literals):
3. Create `src/app/folder_navigation.rs` per § 3.1 (order: header doc → uses → const → enum → inherent impl (navigate, neighbor_image_path) → the three private resolvers).
4. Edit `src/folder_listing.rs`: add `first_listed_image` per § 3.3, nothing else.
5. Edit `src/app.rs` per § 3.2 delta ledger items 1-10 in order (contact points: imports; field; `empty_state`; `open_image` canonicalization + `accept_open_outcome(opened_path, …)`; Ok-arm sibling rebuild; delete enum/navigate/neighbor_image_path bodies; add the use line; ui() intake; two mod declarations).
6. Edit `src/app/arrow_direction.rs` per § 3.4 (mapping, labels, two doc lines).
7. Edit `src/app/floating_controls.rs` doc lines per § 3.5 (NO logic edits — verify by re-reading the final diff: only comments may differ).
8. Edit `src/sibling_folder_listing.rs` marker removals per § 3.6 (4 removed, 2 kept); edit `src/last_viewed_images.rs` per § 3.7 (1 removed).
9. Create `src/app/folder_navigation_tests.rs` per § 3.8 (fixture first, then the 8 locked tests in the listed order).

SELF-CHECK (read files back; no builds):
10. Line counts: `app.rs` ≤ 200 (expect 195; apply § 3.2 contingency if needed), `folder_navigation.rs` ≤ 200 (expect ~85), `folder_navigation_tests.rs` ≤ 200 (expect ~185), `folder_listing.rs` ≤ 200 (expect ~123).
11. Audit: `accept_open_outcome` Err arm one-line UNTOUCHED; Task-2 hook line byte-identical; only § 3.2 lines changed in `app.rs`; `floating_controls.rs` diff is comments-only; marker census = § 4 (7 kept, 0 extra); no new deps (`Cargo.toml`/`Cargo.lock` untouched); no commented-out code; edition-2024 idioms.
12. `git status` → exactly the 9 § 6 files (plus this plan); `git add` them; re-run `git status`; commit with the § 6 message; report files, line counts, expected census 49, commit hash, plan path.

## 8. 4.3 Reviewer Guidance (code-reviewer / code-simplifier)

- Structure: ONE `navigate` and ONE `neighbor_image_path` total, in `src/app/folder_navigation.rs` (an `impl ImagesScrollerApp` child-module block — plan-mandated; do NOT "fix" them back into app.rs); app.rs keeps only intake + state + layouts. Call sites unchanged in `floating_controls.rs` (logic) and existing tests.
- Greps: `fn navigate`/`fn neighbor_image_path` → exactly 2 hits each (both in folder_navigation.rs); `record_displayed_image` → 3 hits (definition, Task-2 hook, none elsewhere); `canonicalize` in `last_viewed_images.rs` → 0 hits (D5 landed in `open_image` only); `first_listed_image` → definition + 1 caller; `FIRST_IMAGE_LISTING_PLACEHOLDER` → 1 site.
- Confirm: same-filename rule ABSENT (no cross-folder path reuse anywhere); NO skip-through-empty-folder logic; NO wrap logic; per-frame enablement flows through the unchanged generic chain; sibling test file and floating_controls tests untouched; census math 41→49 acknowledged (green run = Task 4).
- Simplifier boundaries: do not merge the axes into one resolver body, do not add derives, do not unwrap/expect on `Option`s that encode D2/D3 per the plan.

## 9. Pre-Flight Re-Check List (between write steps)

- [ ] Branch/HEAD/tree still as preflight? No cargo/VM/build command executed?
- [ ] Names exact: `NavigationDirection::{PreviousSiblingFolder, NextSiblingFolder}`, `current_sibling_folders`, `folder_navigation::`, `first_listed_image`, `FIRST_IMAGE_LISTING_PLACEHOLDER`, `last_displayed_image`, `previous_neighbor_of`, `next_neighbor_of`, `ListedFolderNeighbor::At { folder_path, .. }` decode pattern.
- [ ] The 8 test names match § 3.8 exactly; no extra/renamed tests anywhere.
- [ ] `src/folder_listing/tests.rs`, `src/sibling_folder_listing/tests.rs`, `src/app/tests.rs`, `src/app/floating_controls_tests.rs`, `src/app/last_viewed_tests.rs` byte-identical.
- [ ] Staged set == § 6 list; commit message verbatim; no push.

## 10. Deliberately Out of Scope (4.2 boundary — reviewers enforce)

- NO pan/pan-then-navigate (Wave 5); NO zoom/fullscreen/OpenWith; NO CHANGELOG edits (wave rule); NO docs/README/tech/architecture/project-structure edits (Task 5); NO dev-check/container harness runs or exe rebuild (Task 4); NO version bump (Step 3 done, 0.3.0); NO `[DONE]` TODO marking (4.6); NO edits to existing plan files; NO new crate deps / Cargo.toml/lock changes; NO persistence of the last-viewed map beyond app-lifetime; NO same-filename logic; NO skip-traversal past empty folders; NO wrap-around navigation; NO new message constants (single unsupported-file message only); NO UI layout changes beyond the doc/label/mapping edits in § 3.4/3.5.

## 11. 4.5a Conformance Cross-Map (for the later frontend verification, spec §7)

Intake items → § 3.2 (ui() block placement/order); buttons/labels → § 3.4; enablement → § 3.5 (unchanged chain + new resolution) + § 3.1 D2/D3 semantics; "no direct mutation of `opened_image_path`/`decoded_image`/`message_text`/`last_viewed_images` from UI" → unchanged (resolution/seam only); layout unchanged → § 3.5 doc-only. The four §8 pending decisions of the spec are answered by: item 1 = this plan (new enum variants + single seam), item 2 = D2 (disable), item 3 = D3 (silent edges), item 4 = D4 (real temp-dir construction).
