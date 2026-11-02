# Task-3 Plan v2 — Wave 3, Task 3: Vertical Navigation (Sibling Folders) — Keyboard + Buttons (4.1b REDO after user feedback)

- Date: 2026-11-01 (v1 = `20261101-wave3-task3-vertical-nav-navigation.md`, LEFT UNTOUCHED as the rejected-draft record — TODO rule: new plan files only)
- Source TODO (binding): [`.agent/todos/20261029/20261029-todo-5.md`](../../.agent/todos/20261029/20261029-todo-5.md) → `### Task 3` + `## Deferred decisions`
- Front-end input (4.1a, consumed): [`.kilo/plans/20261101-wave3-task3-vertical-nav-frontend-spec.md`](20261101-wave3-task3-vertical-nav-frontend-spec.md)
- Step of the Critical Workflow: 4.1b (design + plan only). 4.2 is write-only (NO cargo, NO VM). Dev-checks belong to Task 4.
- v2 trigger: the user REJECTED v1's D2 ("empty folder = disable/silence") and ADJUDICATED the **empty-folder mode** design (binding, encoded below). D1, D3, D4, D5 are all APPROVED and carried from v1 verbatim.

## 0. What survives from v1 unchanged (do NOT re-design)

- **Ground truth** (re-verified 2026-11-01 against current sources): `src/app.rs` 199 lines; `src/app/floating_controls.rs` 170; `src/app/arrow_direction.rs` 65; `src/sibling_folder_listing.rs` 133 (+8 module tests); `src/folder_listing.rs` 117 (+5 tests); `src/last_viewed_images.rs` 40 (+5 tests); test census **41**; dead_code markers **12** (sibling 6, last_viewed 1, folder_listing 2, image_loading 3). All v1 §0 facts stand.
- **D1**: activate the pill's existing disabled ↑/↓ slots in place; accessible labels `"Previous sibling folder"` / `"Next sibling folder"`; same `Area` id/anchor/frame/paint (4.1a spec §3.2/§7 items 6–8).
- **D3**: sibling list edges stay silently; `ListBeginningReached`/`ListEndReached` decode to `None`; at an edge in empty-folder mode the mode state just persists.
- **D4**: real temp-dir construction tests only; NO app-level injection seam; `ScopeEntriesReader` exposure unchanged (`super::` only).
- **D5 (APPROVED)**: the one canonicalization line in `open_image` feeding `accept_open_outcome` — §3.2 item 6 (v1's §3.2 item 5), byte-identical to v1.
- **Single-seam architecture**: ONE `navigate` + ONE `neighbor_image_path` in ONE new child module `src/app/folder_navigation.rs`; the generic pill enablement chain (`arrow_button_is_enabled` → `navigation_button_enabled` → seam `.is_some()`) stays INTACT; `floating_controls.rs` receives DOC-ONLY edits.
- `first_listed_image` accessor in `src/folder_listing.rs` (v1 §3.3, verbatim) and all marker removals (v1 §3.6/§3.7; §4 audit).

## 1. High-Level Approach (v2 — D2-final rework)

The seam keeps its one-step shape, but the vertical axis now resolves the ADJACENT FOLDER (never an image decision) and the image-vs-empty decision happens ONLY in `navigate`:

1. `NavigationDirection` gains the same two variants as v1: `PreviousSiblingFolder` / `NextSiblingFolder`.
2. `neighbor_image_path(direction)` becomes the cross-axis dispatch WITH A REDEFINED VERTICAL CONTRACT: horizontal variants return the sibling IMAGE path (v1's `listed_image_neighbor` logic moved verbatim); vertical variants return the adjacent sibling FOLDER path — `Some(folder_path)` for ANY adjacent folder (`ListedFolderNeighbor::At`), `None` at list edges / unresolvable anchor / no context. Because the image-vs-empty decision lives in `navigate`, the enablement path is unaffected by it, and `navigation_button_enabled` in `floating_controls.rs` keeps its byte-logic (`.is_some()` on the seam).
3. `navigate` branches per direction on the path kind the direction implies: horizontal → `self.open_image(path)` (Wave-1/2 rule unchanged); vertical → `self.enter_sibling_folder(folder_path)`, which applies the binding priority chain (via `sibling_folder_target_image_path`) and decides case (i) open (`open_image` → normal flow: message clear, Task-2 map write, listings rebuilt in `accept_open_outcome`) or case (ii) `enter_empty_folder_mode`.
4. Empty-folder mode is carried by ONE new state field `browsed_folder_context: Option<PathBuf>` (§1.1). The pill renders while an image is open OR while the context is set; the lone center Open button renders ONLY in the fresh empty state (neither).
5. `.kilo/rules/max-lines-per-file.md` caps: `app.rs` lands at 197 (§3.2 ledger) with the seam + pill-visibility predicate + mode transitions in `folder_navigation.rs`, and the blank-surface centered "Empty folder" message in a new child view module `src/app/empty_folder_view.rs`.

### 1.1 State model (design-work item 1 — encoded exactly)

| Field | Image mode | Empty-folder mode (case ii) | Fresh empty state |
| --- | --- | --- | --- |
| `browsed_folder_context: Option<PathBuf>` | `None` | `Some(canonical-scope empty folder)` | `None` |
| `opened_image_path` | `Some(path)` | `None` | `None` |
| `decoded_image` | `Some(payload)` | `None` (central surface blank) | `None` |
| `message_text` | `Option<&'static str>` (Wave-1 rule) | `None` (pure mode; see E1) | `None` |
| `current_folder_listing` | `Some(FolderListing)` | **`None` — RELEASED** (deterministic choice: no stale horizontal listing retained; ←/→ resolution is `None` because listing + opened path are gone) | `None` |
| `current_sibling_folders` | rebuilt anchored at the opened folder | **REBUILT re-anchored at the browsed empty folder** | `None` |
| `current_display` | normal lifecycle | NOT touched (paint is skipped via the `decoded_image` branch in the central panel; no new display API, no stale-texture handling) | initial |

Lifecycle of `browsed_folder_context`:
- Initialized `None` in `empty_state()`.
- SET in exactly ONE site: `enter_empty_folder_mode` (folder_navigation.rs) on case (ii).
- CLEARED on EVERY successful open: inside the `accept_open_outcome` Ok arm as the last state statement BEFORE `self.opened_image_path = Some(path);` — i.e. at the same success-transition point that commits image mode (the Ok arm is the atomic mode restore; case (i) always restores image mode). NO other site touches the field.
- Gate interplay: `image_is_open()` keeps its meaning (`opened_image_path.is_some()`) and is now read only by the new pill-visibility predicate; the `ui()` layout branch uses `navigation_context_is_active()` = image open OR browsed context → pill layout; otherwise → fresh empty state (center Open). The center-Open gate change ("no image open AND no folder context") is realized by that same predicate — the center Open is unreachable whenever the context exists.
- **Re-anchor trick (mirrors v1 §3.1's resolution contract):** inside `enter_empty_folder_mode`, the vertical listing re-anchor is built as `browsed_folder_context.join(LISTING_ANCHOR_FILE_PLACEHOLDER)` and passed VERBATIM into `SiblingFolderListing::from_opened_image(anchor.as_path())` — the Task-1 API derives `.parent()` = the empty folder, canonicalizes internally, scopes to that parent, and excludes the empty folder itself from destinations. No caller-side canonicalization anywhere in this crate path.

### 1.2 Edge states (documented explicitly; deterministic; no extra machinery)

- **E1 — failed attempt FROM empty-folder mode:** a case-(i) attempt whose target image cannot be decoded (e.g. a recorded last-viewed path deleted) hits the UNTOUCHED Wave-1 Err arm → `message_text = Some(UNSUPPORTED_FILE_MESSAGE)` while the browsed context STAYS set and the centered "Empty folder" label REMAINS in the blank surface. The Wave-1 failure rule overrides the "pill line stays empty in empty-folder mode" phrasing, which governs the PURE empty-mode state (entry, persistence, onward navigation). Test 5 locks E1.
- **E2 — browsed folder removed externally:** the re-anchor's `from_opened_image` returns `None` → `current_sibling_folders = None` → both vertical arrows disable (seam `None`), no movement, no message. Covered by a one-line doc note in `enter_empty_folder_mode`.

## 2. Adjudicated deferred-decision resolutions (ALL binding, user-set 2026-11-01)

### D1 — CONFIRMED (unchanged from v1)

Activate the pill's existing disabled ↑/↓ slots in place. Mechanics: `ArrowControl::FolderUp`/`FolderDown` map to the new variants (§3.4); the unchanged generic chain `arrow_button_is_enabled` → `navigation_button_enabled` → `neighbor_image_path(...).is_some()` enables/dims them per frame with NO `floating_controls.rs` logic change; zero layout change, zero new widgets.

### D2 — FINAL, per user adjudication ("empty-folder mode"; v1's disable/silence is REJECTED)

- Vertical navigation ALWAYS moves to the ADJACENT sibling folder. Folders without images are NOT barriers; NO skip-ahead logic.
- **Case (i):** the adjacent folder yields a target image through the binding priority chain (visited this session → its last-viewed image; else → its FIRST whitelisted image in the Wave-2 collation; NO same-filename attempt, NO other fallback) → open it through the NORMAL `open_image` flow: success clears the message, the Task-2 hook records the map entry, and the sibling listing rebuild happens in `accept_open_outcome` exactly as v1 planned.
- **Case (ii):** the adjacent folder yields NO target (no map entry AND no first whitelisted image — unreadable / empty / files-only) → ENTER "empty-folder mode": the previously displayed image is REMOVED (`opened_image_path = None`, `decoded_image = None`, display blank); `browsed_folder_context = Some(folder)`; `current_folder_listing = None`; `message_text = None`; the vertical sibling listing is REBUILT re-anchored at that folder; the fixed message "Empty folder" is drawn CENTERED in the blank image area (NOT in the pill's message line; the pill's own line stays empty in this mode). The pill REMAINS VISIBLE with its four arrows; ←/→ are DISABLED in this mode (no anchor image → horizontal resolution `None`); ↑/↓ keep working from the new folder context (cases (i)/(ii) may repeat). NO map entry is created for the empty folder.
- **Enablement semantics:** ↑/↓ enabled exactly when an ADJACENT sibling folder exists in that direction (`ListedFolderNeighbor::At`) — NOT gated on image resolution; edge / empty listing → disabled. ←/→ enabled exactly as before via image resolution (naturally `None` in empty-folder mode).
- **Center Open:** shows ONLY when the app has no folder context at all — the 0.2.1 gate ("no image open") becomes "no image open AND no folder context". In empty-folder mode: no center Open.

### D3 — CONFIRMED (unchanged from v1)

Sibling-list edges stay silently (edge variants decode to `None`; no wrap; no message). At an edge in empty-folder mode the mode state simply persists.

### D4 — CONFIRMED (unchanged from v1)

Real temp-dir construction tests only; app-level vertical tests build real parent scopes through the EXISTING public constructors (`SiblingFolderListing::from_opened_image` via the Ok-arm rebuild and the empty-mode re-anchor; `FolderListing` internally for the first-image probe) with a real-fs fixture `TempParentScope` in the test file — exactly the `src/app/tests.rs` pattern (`TempImageFolder` + `app.accept_open_outcome(path, Ok(sample_decoded_image()))`; real decodes ALWAYS fail in the container).

### D5 — CONFIRMED (APPROVED — v1 §3.2 item 5 AS WRITTEN)

`open_image` canonicalizes the candidate path once and feeds the canonical path into `accept_open_outcome` (§3.2 item 6 — v1's §3.2 item 5 AS WRITTEN, only renumbered). The Task-2 hook line stays byte-identical; `last_viewed_images.rs` gains no canonicalization (its reviewer grep stays clean); resolution functions pass anchors verbatim into the Task-1 API.

## 3. Exact Code Changes

### 3.0 PRE-COUNTED line budgets (cap 200 per file)

| File | Size after |
| --- | --- |
| `src/app.rs` | 199 → **197** (ledger §3.2; contingency §3.2) |
| `src/app/folder_navigation.rs` | NEW, expected ~130 |
| `src/app/empty_folder_view.rs` | NEW, expected ~18 |
| `src/app/folder_navigation_tests.rs` | NEW, expected ~195 (budget guard: STOP if > 200) |
| `src/folder_listing.rs` | 117 → 123 |
| `src/app/arrow_direction.rs` | 65 → 66 |
| `src/app/floating_controls.rs` | 170 → 170 (doc-only) |
| `src/sibling_folder_listing.rs` | 133 → 133 (markers) |
| `src/last_viewed_images.rs` | 40 → 40 (marker) |

### 3.1 NEW FILE `src/app/folder_navigation.rs` (~130 lines; the seam extraction, revised)

Inherent `impl ImagesScrollerApp` in a child module (legal; precedent = v1 §3.1); the ENUM LIVES HERE (v1 §3.6 budget math). Item order: header doc → uses → `const` → enum → inherent impl (`navigate`, `neighbor_image_path`, `navigation_context_is_active`, `sibling_folder_target_image_path`, `enter_sibling_folder`, `enter_empty_folder_mode`) → free fns (`listed_image_neighbor`, `sibling_folder_neighbor_path`, `vertical_anchor_path`, `first_image_path_of_folder`). Sketch — v1's file revised; vertical-contract lines changed, horizontal logic byte-preserved:

```rust
//! Shared navigation seam for the arrow keys and the pill buttons: the ±1
//! neighbor path per `NavigationDirection` -- the sibling-image listing of the
//! current folder on the horizontal axis, the ADJACENT sibling folder under
//! the same parent on the vertical axis. Vertical navigation ALWAYS moves to
//! the adjacent folder; its resolvable target image opens through the normal
//! open flow, otherwise empty-folder browsing mode is entered. No neighbor is
//! skipped, no wrap. The image-versus-empty decision happens ONLY in
//! `navigate`: the enablement path sees folder adjacency alone.

use crate::folder_listing::{FolderListing, ListedNeighbor};
use crate::sibling_folder_listing::{ListedFolderNeighbor, SiblingFolderListing};
use std::path::{Path, PathBuf};

use super::ImagesScrollerApp;

/// Placeholder file name a listing anchor always carries: the anchor's PARENT
/// folder is the real target; the name itself is never read (the Task-1 plan
/// § 7 trick, reused for the first-image probe and the empty-mode re-anchor).
const LISTING_ANCHOR_FILE_PLACEHOLDER: &str = "listing_anchor.bin";

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
    /// Navigates one step per direction: horizontal neighbors open their image
    /// path; vertical neighbors open the adjacent folder's target image or,
    /// when the folder yields none, enter empty-folder mode (D2-final).
    pub(super) fn navigate(&mut self, direction: NavigationDirection) {
        let Some(neighbor_path) = self.neighbor_image_path(direction) else {
            return;
        };
        match direction {
            NavigationDirection::Previous | NavigationDirection::Next => {
                self.open_image(neighbor_path);
            }
            NavigationDirection::PreviousSiblingFolder
            | NavigationDirection::NextSiblingFolder => {
                self.enter_sibling_folder(neighbor_path);
            }
        }
    }

    /// The ±1 neighbor path per direction: the sibling IMAGE path on the
    /// horizontal axis; the adjacent sibling FOLDER path on the vertical axis
    /// (the priority-chain and empty-folder decisions apply only in
    /// `navigate`). Every "nothing in this direction" case (no image or
    /// browsed folder, no listing, unresolvable anchor, reached list edge)
    /// resolves to None. Also drives the per-frame pill enablement.
    pub(super) fn neighbor_image_path(&self, direction: NavigationDirection) -> Option<PathBuf> {
        match direction {
            NavigationDirection::Previous | NavigationDirection::Next => {
                listed_image_neighbor(self, direction)
            }
            NavigationDirection::PreviousSiblingFolder
            | NavigationDirection::NextSiblingFolder => {
                sibling_folder_neighbor_path(self, direction)
            }
        }
    }

    /// The floating pill renders while an image is open, or while an
    /// empty-folder context is being browsed (D2-final); only the fresh empty
    /// state (neither) falls back to the lone center Open button.
    pub(super) fn navigation_context_is_active(&self) -> bool {
        self.image_is_open() || self.browsed_folder_context.is_some()
    }
```
(v1 extraction carries: `listed_image_neighbor` UNCHANGED from v1 §3.1; `navigate` replaces v1's unconditional `open_image` with the direction dispatch; `neighbor_image_path`'s vertical arm replaces `sibling_folder_image_path` per the redefined contract.)

Continuing the impl:

```rust
    /// Target-image priority (TODO Task 3, binding): visited this session -> its
    /// recorded last-viewed image; otherwise ALWAYS the folder's FIRST whitelisted
    /// image in the fixed collation. NO same-filename attempt, NO other fallback;
    /// an image-less folder resolves to None (the caller enters empty-folder mode).
    /// (pub(super) for the locked priority tests; internal caller: enter_sibling_folder.)
    pub(super) fn sibling_folder_target_image_path(&self, folder_path: &Path) -> Option<PathBuf> {
        if let Some(last_viewed) = self.last_viewed_images.last_displayed_image(folder_path) {
            return Some(last_viewed.to_path_buf());
        }
        first_image_path_of_folder(folder_path)
    }

    /// The vertical open decision: the folder's target image opens via the
    /// normal open flow (case i); anything else enters empty-folder mode for
    /// the folder (case ii). Never skips past a folder, never wraps.
    fn enter_sibling_folder(&mut self, folder_path: PathBuf) {
        match self.sibling_folder_target_image_path(&folder_path) {
            Some(target_image) => self.open_image(target_image),
            None => self.enter_empty_folder_mode(folder_path),
        }
    }

    /// Empty-folder mode (D2-final): the browsed folder replaces the open
    /// image -- opened path, decoded payload, horizontal listing and message
    /// released; the vertical sibling list re-anchors ON the browsed folder by
    /// passing `folder.join(placeholder)` verbatim to the Task-1 API; no
    /// last-viewed entry is recorded. (A vanished browsed folder yields an
    /// empty listing: both vertical arrows disable, no message.)
    fn enter_empty_folder_mode(&mut self, folder_path: PathBuf) {
        let anchor = folder_path.join(LISTING_ANCHOR_FILE_PLACEHOLDER);
        self.browsed_folder_context = Some(folder_path);
        self.opened_image_path = None;
        self.decoded_image = None;
        self.message_text = None;
        self.current_folder_listing = None;
        self.current_sibling_folders =
            SiblingFolderListing::from_opened_image(anchor.as_path());
    }
```

Free fns (module-private except where noted):

```rust
fn listed_image_neighbor(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> Option<PathBuf> {
    // v1 §3.1 BODY MOVED VERBATIM (opened path -> listing -> listed_index_of
    // -> previous_neighbor/next_neighbor -> let-else At decode -> Some).
}

/// The ADJACENT sibling FOLDER per direction from the vertical listing; the
/// anchor is passed VERBATIM to the Task-1 API (the open image in image mode;
/// the browsed empty folder plus the placeholder name in empty-folder mode --
/// the API canonicalizes internally, no caller canonicalization). List edges
/// resolve to None (D3); a missing listing resolves to None.
fn sibling_folder_neighbor_path(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> Option<PathBuf> {
    let listing = app.current_sibling_folders.as_ref()?;
    let anchor_path = vertical_anchor_path(app)?;
    let folder_neighbor = match direction {
        NavigationDirection::PreviousSiblingFolder => listing.previous_neighbor_of(&anchor_path),
        NavigationDirection::NextSiblingFolder => listing.next_neighbor_of(&anchor_path),
        NavigationDirection::Previous | NavigationDirection::Next => return None,
    }?;
    let ListedFolderNeighbor::At { folder_path, .. } = folder_neighbor else {
        return None;
    };
    Some(folder_path)
}

/// The vertical listing anchor: the open image verbatim (image mode) or the
/// browsed empty folder with the placeholder name (empty-folder mode). Image
/// mode wins if both were set (kept impossible by the atomic transitions).
fn vertical_anchor_path(app: &ImagesScrollerApp) -> Option<PathBuf> {
    if let Some(opened_path) = app.opened_image_path.as_ref() {
        return Some(opened_path.clone());
    }
    let browsed_folder = app.browsed_folder_context.as_ref()?;
    Some(browsed_folder.join(LISTING_ANCHOR_FILE_PLACEHOLDER))
}

fn first_image_path_of_folder(target_folder_path: &Path) -> Option<PathBuf> {
    let placeholder_path = target_folder_path.join(LISTING_ANCHOR_FILE_PLACEHOLDER);
    let folder_listing = FolderListing::from_opened_image(&placeholder_path)?;
    folder_listing.first_listed_image()
}
```

Rules check: every fn ≤ 2 args (`&App` + 1); depth ≤ 2 in every body (navigate's match arms are calls; `sibling_folder_target_image_path` has one `if let` + one call); ≤ 50 lines per method; `pub(super)` ONLY on: the enum, `navigate`, `neighbor_image_path`, `navigation_context_is_active`, `sibling_folder_target_image_path` (the last two read by app.rs/`ui()`/tests); the `||` inside `navigation_context_is_active` IS the single-section-extraction sanctioned by `.kilo/rules/single-section-boolean-conditions.md` (the `if` in `ui()` stays single-section); explicit match-arm `return None` guards are direction dispatch, not commented-out code.

### 3.2 EDIT `src/app.rs` (file cap: 199 → **196**; exact delta ledger)

| # | Change | Δ lines |
| --- | --- | --- |
| 1 | Import after `use crate::platform;` (line 11): `use crate::sibling_folder_listing::SiblingFolderListing;` | +1 |
| 2 | Trim line 8 to `use crate::folder_listing::FolderListing;` (`ListedNeighbor` leaves app.rs — only the extracted resolver used it) | 0 |
| 3 | Field after `last_viewed_images` (line 23), with a one-line doc: `/// Sibling folders under the opened image's folder (rebuilt on every successful open).` + `current_sibling_folders: Option<SiblingFolderListing>,` | +2 |
| 4 | Field after the item-3 insert, with a one-line doc: `/// Folder being browsed without an opened image (empty-folder mode).` + `browsed_folder_context: Option<PathBuf>,` | +2 |
| 5 | `empty_state()`: `current_sibling_folders: None,` + `browsed_folder_context: None,` after `last_viewed_images: LastViewedImages::empty(),` | +2 |
| 6 | `open_image`: between `let outcome = …` and the accept call add `let opened_path = std::fs::canonicalize(&path).unwrap_or_else(\|_\| path.clone());` and the call becomes `self.accept_open_outcome(opened_path, outcome);` (D5, APPROVED; fully-qualified so NO new import) | +1 |
| 7 | `accept_open_outcome` Ok arm — after the UNTOUCHED Task-2 hook line (`self.last_viewed_images.record_displayed_image(&path);`) and BEFORE `self.opened_image_path = Some(path);` insert the wrapped statement `self.current_sibling_folders =` / `SiblingFolderListing::from_opened_image(path.as_path());` THEN the single statement `self.browsed_folder_context = None;` (the context clear sits at the same success-transition point — last statement of the Ok arm's state group, atomic with the image commit) | +3 |
| 8 | Delete `enum NavigationDirection { … }` (lines 31-34) — moved to folder_navigation.rs — and add `use folder_navigation::NavigationDirection;` in its place (child `super::NavigationDirection` references still resolve through this private import) | −4 +1 |
| 9 | Delete methods `navigate` (lines 85-91, doc included) and `neighbor_image_path` (lines 93-108, doc included) — moved into the child inherent impl | −23 |
| 10 | `ui()`: after the existing `right_pressed` if-block (line 165) add, same pattern: `let up_pressed = ui.ctx().input(\|i\| i.key_pressed(egui::Key::ArrowUp));` + `let down_pressed = …(egui::Key::ArrowDown));` + `if up_pressed { self.navigate(NavigationDirection::PreviousSiblingFolder); }` + `if down_pressed { self.navigate(NavigationDirection::NextSiblingFolder); }` — intake STAYS BEFORE the layout branch (4.1a spec §2.1/§2.2; vertical keys are therefore NEVER swallowed by the gate in either mode; no pan interplay) | +8 |
| 11 | In `ui()` replace the branch test `if self.image_is_open() {` with `if self.navigation_context_is_active() {` (else arm unchanged: `show_empty_state_layout`) | 0 |
| 12 | Rename `show_image_layout` → `show_navigation_layout` + retitle its first doc line to `/// Pill layout: the floating pill plus the central surface -- the image paint when one is open, else the empty-folder message.` and in its `CentralPanel` closure replace the implicit-skip with an else arm: `} else { empty_folder_view::show_empty_folder_message(central_ui); }` (invariant: decoded=None inside this layout happens ONLY in empty-folder mode) | +2 |
| 13 | Update doc juice only (0 lines): `image_is_open`'s doc → "…(read by the navigation seam's pill-visibility gate)."; `show_empty_state_layout` doc gains "(the fresh empty state: no image AND no folder context)" | 0 |
| 14 | After `mod floating_controls;` add `mod empty_folder_view;` (+ adjacent blank) | +2 |
| 15 | After `#[cfg(test)] mod last_viewed_tests;` add `#[cfg(test)]` + `mod folder_navigation_tests;` | +2 |

Sum (recompute carefully): additions 1+2+2+2+1+3+8+2+2+2 = **25**; removals 4 (item 8) + 23 (item 9) = **27**; net −2 → 199 **− 2 = 197 ≤ 200**. NOTHING else moves: `window_options`, consts (174-177), `open_via_dialog`, `show_centered_open_button`, the Err arm (EXACTLY one line, `UNSUPPORTED_FILE_MESSAGE`), and the Task-2 hook line stay byte-identical.

Contingency (deterministic, in order, only if the post-write count exceeds 200): (a) drop the `browsed_folder_context` doc line (−1); (b) drop the `current_sibling_folders` doc line (−1); (c) place `mod empty_folder_view;` directly adjacent to `mod arrow_direction;` without a blank line (−1). If STILL > 200: STOP and return a question — never violate the cap silently.

### 3.3 EDIT `src/folder_listing.rs` (117 → 123; ONLY this addition — v1 §3.3 VERBATIM)

After `listed_image_count` (grouped with accessors; NO first-element accessor exists today and none may be synthesized via `next_neighbor(0)` which reports the SECOND entry):

```rust
    /// The FIRST whitelisted image in the fixed collation (the vertical
    /// navigation's new-folder target); None for an empty listing.
    pub(crate) fn first_listed_image(&self) -> Option<PathBuf> {
        self.image_paths.first().cloned()
    }
```

No dead_code marker (called immediately by `folder_navigation.rs`). Everything else byte-identical; `src/folder_listing/tests.rs` UNTOUCHED.

### 3.4 EDIT `src/app/arrow_direction.rs` (65 → 66; UI contract of 4.1a spec §3.2/§5 — v1 §3.4 VERBATIM)

- `navigation_direction`: `Self::FolderUp => Some(NavigationDirection::PreviousSiblingFolder),` and `Self::FolderDown => Some(NavigationDirection::NextSiblingFolder),` replacing the `FolderUp | FolderDown => None` arm; retitle its doc line to `/// Concrete direction per button; all four arrows navigate.`
- `accessible_label`: `"Up folder"` → `"Previous sibling folder"`; `"Down folder"` → `"Next sibling folder"` (D1 labels).
- Update the two stale doc lines (line 27-29 header sentence "…the folder arrows are disabled in Wave 2 and follow Wave 3" and the line-57 sentence `/// None for the Wave-3-reserved folder arrows (no navigation this wave).`) to the active state; no Wave-3-reserved wording may survive.
- NO layout, paint, cursor, or `widget_info` change.

### 3.5 EDIT `src/app/floating_controls.rs` (170 → 170; DOC-ONLY — logic diff must be EMPTY)

- Logic untouched: `navigation_button_enabled`, `arrow_button_is_enabled`, specs, click handling all work for the new directions through the generic chains (the seam's vertical arm now returns folder adjacency; the enablement chain reads `.is_some()` — unchanged code).
- Update: module header lines 3-4 (↑/↓ are no longer a reserved disabled pair — e.g. "All four arrow enablements follow the keyboard seam per frame; the caller keeps the pill-visibility gate."); the `navigation_button_enabled` doc sentence to cover both axes ("no image or browsed folder, no listing, unresolvable anchor, and list edges disable."); line 143 parenthetical "(the Wave-3 reserve stays handler-free)" → "(a disabled arrow stays handler-free)".
- Conformance: spec §7 items 6-8, 11-12, 14-17 remain satisfiable without logic edits.

### 3.6 EDIT `src/sibling_folder_listing.rs` (133 → 133; markers — v1 §3.6 VERBATIM)

REMOVE `#[allow(dead_code)]` from EXACTLY: the `ListedFolderNeighbor` enum (line 21), `from_opened_image` (38 — now called from the Ok arm AND the empty-mode re-anchor), `previous_neighbor_of` (48), `next_neighbor_of` (60).

KEEP the markers on EXACTLY: `At.listed_index` (24) and `listed_folder_count` (43 — module-test-only accessor). `#[cfg(test)] mod tests;` and `src/sibling_folder_listing/tests.rs` (AT CAP, 200/200) stay byte-identical.

### 3.7 EDIT `src/last_viewed_images.rs` (40 → 40; markers — v1 §3.7 VERBATIM)

Remove `#[allow(dead_code)]` from `last_displayed_image` (line 31) — its caller `sibling_folder_target_image_path` lands now. Everything else byte-identical (grep `canonicalize` in this file still yields NO hits — D5 did NOT land here).

### 3.8 NEW FILE `src/app/empty_folder_view.rs` (~18 lines; design-work item 3 — the centered blank-surface message)

Chosen approach: a dedicated child view module rather than inlining into app.rs (the else-arm call keeps app.rs within cap and gives the new message constant a self-documenting home). Content EXACTLY:

```rust
//! Empty-folder browsing view: the one centered status message drawn in the
//! blank central surface while the app browses a folder without an opened
//! image (D2-final). The pill's message line stays empty in this mode.

use eframe::egui;

/// The blank-surface status: the browsed folder holds no displayable image.
pub(super) const EMPTY_FOLDER_MESSAGE: &str = "Empty folder";

/// Draws the message centered in the central surface -- NOT in the pill's
/// message line, which the pill keeps empty in empty-folder mode.
pub(super) fn show_empty_folder_message(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::centered_and_justified(egui::Direction::TopDown),
        |message_ui| {
            message_ui.label(EMPTY_FOLDER_MESSAGE);
        },
    );
}
```

- Wording: the user's literal **"Empty folder"**, plain, NO punctuation — consistent with the existing `UNSUPPORTED_FILE_MESSAGE` style (no trailing period there either). The `Layout::centered_and_justified` + `Direction::TopDown` pattern is the landed precedent in `src/app_display.rs:42-47`.
- `pub(super)` on the const exists SOLELY so the locked test can assert its value; it also has a real production caller (the show fn), so NO dead_code marker.
- Rendering is a plain label, reached ONLY through the else-arm of `show_navigation_layout`'s central panel.

### 3.9 NEW FILE `src/app/folder_navigation_tests.rs` (~195 lines; hard cap 200; design-work item 6 — the 9 LOCKED tests)

Fixture `TempParentScope` (real fs; prefix `img_scroller_folder_nav_{test_case}_{pid}`; `Drop` → `fs::remove_dir_all`; private to this test file):

- `create(test_case) -> Self` — `std::env::temp_dir().join(format!(…)).expect("temp parent scope created")` + `fs::create_dir_all`.
- `create_child_folder(&self, folder_name) -> PathBuf` — `fs::create_dir` of `parent.join(name)`, returns the child folder path.
- `create_image(&self, folder_name, file_name)` — `fs::write(child.join(file), b"x")`.
- `child_image_path(&self, folder_name, file_name) -> PathBuf` — `parent.join(folder).join(file)`.
- `child_folder_path(&self, folder_name) -> PathBuf` — `parent.join(folder)`.

Imports: `use super::tests::sample_decoded_image;`, `use super::{ImagesScrollerApp, NavigationDirection};`, `use super::empty_folder_view::EMPTY_FOLDER_MESSAGE;`, `use super::floating_controls::navigation_button_enabled;`, `use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;`, `use std::fs;`. Shared helper (~6 lines): `fn app_opened_in(scope_image_path: &PathBuf) -> ImagesScrollerApp` — `empty_state()` + `accept_open_outcome(path.clone(), Ok(sample_decoded_image()))` (success simulation; real opens always fail through the container stub).

The 9 tests (names LOCKED; reviewers diff-check char-for-char):

1. **`vertical_navigation_resolves_the_adjacent_sibling_folders_in_sort_order`** — scope `parent/folder1/f1.png`, `parent/folder2/f2.png`, `parent/folder3/f3.png`; app opened `folder2/f2`; assert `app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder)` == `Some(parent/folder1)` AND `NextSiblingFolder` == `Some(parent/folder3)` — FOLDER paths (v2 resolution contract: adjacency, not image content; supersedes v1 test 1's image-path assertions).
2. **`vertical_navigation_reopens_the_visited_folders_last_viewed_image`** — `folder1{name1, name2}`, `folder2{single}`; OK-accept `folder1/name2` (visited, map remembers name2) then OK-accept `folder2/single`; assert `app.neighbor_image_path(PreviousSiblingFolder)` == `Some(folder1)` AND `app.sibling_folder_target_image_path(&folder1_path)` == `Some(folder1/name2)` — NOT `name1` (priority (a) beats first-image; NO same-filename attempt exists anywhere in this chain).
3. **`vertical_navigation_opens_the_first_image_of_a_new_folder`** — `folder1/only.png` opened; `folder2{b.png, a.png}` unvisited; assert `neighbor_image_path(NextSiblingFolder)` == `Some(folder2)` and `sibling_folder_target_image_path(&folder2_path)` == `Some(folder2/a.png)` (collation FIRST image, not sort-last, not same-name).
4. **`entering_an_image_less_sibling_folder_clears_the_image_and_reanchors_the_listing`** (D2 case ii entry) — `folder1/img.png`, `folder2/stray.txt` only; app opened `folder1/img`; `app.navigate(NextSiblingFolder)`; assert: `opened_image_path` is `None`; `decoded_image` is `None`; `browsed_folder_context` == `Some(parent/folder2)`; `current_folder_listing` is `None`; `message_text` is `None`; `navigation_context_is_active()` is `true`; the vertical listing is re-anchored (`neighbor_image_path(PreviousSiblingFolder)` == `Some(parent/folder1)`); NO map write (`last_displayed_image(&folder2)` is `None`); horizontal dead (`neighbor_image_path(NavigationDirection::Next)` is `None`); and `EMPTY_FOLDER_MESSAGE` == `"Empty folder"` (the locked centered-message wording).
5. **`empty_folder_mode_continues_vertically_in_both_folder_kinds`** (the ADDED empty-mode continuation; locks E1) — `folder1/a.png`, `folder2/stray.txt` only, `folder3/c.png`; app opened `folder1/a`; `app.navigate(NextSiblingFolder)` → empty mode at `folder2` (assert context == `Some(folder2)`); then `neighbor_image_path(NextSiblingFolder)` == `Some(parent/folder3)` (re-anchored) and the folder3 target == `Some(folder3/c.png)`; then `app.navigate(NextSiblingFolder)` → the REAL open fails (container stub) → assert E1: `message_text` == `UNSUPPORTED_FILE_MESSAGE`, `browsed_folder_context` STILL `Some(folder2)`, `opened_image_path` STILL `None`, blank display; then the recorded-image restore: `app.accept_open_outcome(folder3.image_path("c.png"), Ok(sample_decoded_image()))` (the success simulation) → assert image mode restored: `browsed_folder_context` `None`, `opened_image_path` `Some(folder3/c)`, `decoded_image` `Some`, `message_text` `None`, sibling listing rebuilt anchored at folder3; SECOND HALF: `app.navigate(PreviousSiblingFolder)` → folder2 (empty, unvisited) → empty mode re-entered: context == `Some(folder2)`, `opened_image_path` `None`, message `None` (the mode persists across empty-to-empty steps, re-anchored).
6. **`vertical_navigation_stays_silent_at_the_sibling_list_edges`** (D3, both modes) — image-mode edges: `folder1/a.png`, `folder2/b.png`; from folder1: `PreviousSiblingFolder` resolution `None` and `navigate` no-ops; from folder2 (after OK-accept b): `NextSiblingFolder` `None`; no message at any point; empty-mode edge persists: fresh scenario `folder1` empty, `folder2/b.png` opened, `navigate(PreviousSiblingFolder)` → empty mode at `folder1` (the FIRST listed sibling); from there `neighbor_image_path(PreviousSiblingFolder)` is `None`, `navigate` no-ops, and `browsed_folder_context` REMAINS `Some(folder1)` (the state just persists); single-child scope → both directions `None`.
7. **`vertical_navigation_to_a_folder_whose_last_viewed_image_vanished_reports_the_standard_failure`** — `folder1{keep.png, vanish.png}`, `folder2/live.png`; OK-accept `folder1/vanish` then OK-accept `folder2/live`; `fs::remove_file(vanish)`; `app.navigate(PreviousSiblingFolder)` (the case-(i) attempt back toward the visited folder1) → `message_text` == `UNSUPPORTED_FILE_MESSAGE`, `opened_image_path` still `folder2/live`, decoded payload still present; map entries NOT rewritten by the failure (`last_displayed_image(folder2)` still `live`; Wave-1 failure rule).
8. **`vertical_button_enablement_matches_the_neighbor_resolution_semantics`** (test-7 of v1 REVISED: vertical enablement = folder adjacency; horizontal parity unchanged) — scope `folder1/a.png`, `folder2/stray.txt` only, `folder3/c.png`; FRESH app: all four directions disabled; after OK-accept `folder1/a`: `navigation_button_enabled(PreviousSiblingFolder)` == false (edge), `navigation_button_enabled(NextSiblingFolder)` == TRUE although `sibling_folder_target_image_path(&folder2)` is `None` (THE D2-DELTA LOCK: adjacency, not image resolution), `Prev`/`Next` horizontal per image listing; after `navigate(NextSiblingFolder)` into empty mode at folder2: vertical prev/next BOTH enabled (folder1 / folder3 exist), horizontal both disabled; assert `navigation_button_enabled(dir)` == `app.neighbor_image_path(dir).is_some()` for all four directions at each app state.
9. **`vertical_target_open_records_the_new_folders_last_viewed_memory`** — `folder1{a, b}`, `folder2{c}`; OK-accept `folder1/b` → `last_displayed_image(folder2)` is `None`; OK-accept `folder2/c` (the "vertical open" simulation — the Task-2 hook Task 3 relies on, exactly like `last_viewed_tests.rs`) → `last_displayed_image(folder2)` == `Some(folder2/c)` AND `sibling_folder_target_image_path(&folder1)` now returns `folder1/b` (visited-priority re-entry point back into the navigated folder).

Budget guard: if any edit pushes this file over 200 lines → STOP and return a question (same rule as Task-1's test file). Trim order if needed (each −1 line, applied only on overflow): (a) `app_opened_in` helper inlines `Some(...)` per test — NO, helper stands; instead (a) collapse multi-assert blank groupings, (b) shorten fixture method docs to one line. Then STOP.

- Reachability note (pill-display gate at unit level): `navigation_context_is_active()` is `pub(super)`, so the pill-visibility GATE (fresh → false; image mode → true; empty-folder mode → true) IS unit-assertable (asserted in tests 4, 5, 8) even though the egui pill itself is not (no frame in the container).
- `src/app/floating_controls_tests.rs` stays UNTOUCHED (32): its horizontal enablement coverage continues to pass through the unchanged chain; the vertical counterpart lives beside the vertical fixture. Justification recorded here per caller mandate.
- Keyboard `key_pressed` intake is NOT unit-testable in the container (horizontal precedent likewise untested); 4.5a conformance checklist + Task-4 OPERATOR-RUN cover it.

## 4. dead_code Marker Audit (design-work item 7; mandate item 5 — exact census)

Before: sibling 6 (`sibling_folder_listing.rs` lines 21/24/38/43/48/60) + last_viewed 1 (31) + folder_listing 2 (22/53) + image_loading 3 (18/25/37) = **12**. After Task 3:

- Removed (4 + 1 = 5): `sibling_folder_listing.rs` 21 (enum), 38 (`from_opened_image`), 48 (`previous_neighbor_of`), 60 (`next_neighbor_of`); `last_viewed_images.rs` 31 (`last_displayed_image`).
- Added: **0**. The new `first_listed_image` accessor has an immediate caller; `EMPTY_FOLDER_MESSAGE` (the §3.8 allow-exception constant) is used by `show_empty_folder_message` and the locked test 4 — pub(super) items with real callers need NO marker; all `pub(super)` seam functions have production callers.
- Kept (7): sibling 24 (`listed_index`), sibling 43 (`listed_folder_count`), folder_listing 22 (`ListedNeighbor.listed_index`), folder_listing 53 (`listed_image_count`), image_loading 18/25/37 (`Unsupported`, `user_message`, `pixel_buffer_size`) — all test-only/unused, correct as-is.
- Keyboard intake needs NO markers (plain let/if, no dead fields).
- NEW `folder_navigation.rs` / `empty_folder_view.rs` / `folder_navigation_tests.rs`: no dead-code markers anywhere.

## 5. Tests & Census (mandate items 8-9)

- New tests: **9**, ALL in the NEW `src/app/folder_navigation_tests.rs` (rationale unchanged from v1: `src/app/tests.rs` sits at 187/200 and cannot absorb the sibling fixture; `floating_controls_tests.rs` stays untouched per §3.9).
- Census: 41 → **50** (+9) = app 9, folder_navigation_tests 9 (NEW), floating_controls 1, folder_listing 5, sibling 8, last_viewed 5, name_collation 2, image_loading 4, app_display 2, scaling 5. Dev-check runs (fmt/check/test/clippy through the Alpine-VM harness, `vm_status` FIRST, exit code authoritative, target `ALL CHECKS PASSED`) are Task-4 scoped — NEVER in 4.2.

## 6. Git (4.2 — mandate item 10)

- Branch already exists from Step 2: `feat/wave-3-vertical-scroll`. NO branch creation, NO push (push is Step-5-restricted, `main`→`origin` ONLY), no merges, no tags.
- Before staging: read `.gitignore`, run `git status` (Gitignore Compliance Rule) — nothing is built in 4.2, so no `target/`/`dist/`/`logs/` risk; verify none staged anyway.
- Stage EXACTLY (10 paths): `src/app.rs`, `src/app/folder_navigation.rs`, `src/app/empty_folder_view.rs`, `src/app/folder_navigation_tests.rs`, `src/app/arrow_direction.rs`, `src/app/floating_controls.rs`, `src/folder_listing.rs`, `src/sibling_folder_listing.rs`, `src/last_viewed_images.rs`, `.kilo/plans/20261101-wave3-task3-vertical-nav-navigation-v2.md` (v1 plan file stays committed/untouched as the rejected-draft record).
- Single commit, message EXACT: `feat: navigate sibling folders with up/down arrows and buttons` (still accurate: the empty-folder mode IS sibling-folder navigation via ↑/↓ keys and buttons).

## 7. 4.2 Junior-Implementer Step-by-Step Checklist (WRITE-ONLY — NO cargo/VM/build commands)

PREFLIGHT (stop and ask the caller if ANY fails):
1. `git branch --show-current` == `feat/wave-3-vertical-scroll`; `git status` → clean tree; `git log --oneline -5` shows the Step-3 version bump (0.3.0) then Task-1 and Task-2 commits.
2. Re-read THIS plan (v2), the 4.1a spec, and `src/app/tests.rs` once more before writing (fresh name pressure check).

BUILD (in this order; write/edit tools only, real newlines, no `\n` literals):
3. Create `src/app/folder_navigation.rs` per §3.1 (order: header doc → uses → const → enum → inherent impl (navigate, neighbor_image_path, navigation_context_is_active, sibling_folder_target_image_path, enter_sibling_folder, enter_empty_folder_mode) → free fns (listed_image_neighbor, sibling_folder_neighbor_path, vertical_anchor_path, first_image_path_of_folder)).
4. Create `src/app/empty_folder_view.rs` per §3.8 EXACTLY as written.
5. Edit `src/folder_listing.rs`: add `first_listed_image` per §3.3, NOTHING else.
6. Edit `src/app.rs` per §3.2 ledger items 1-15 in order (contact points: imports; the two fields; `empty_state`; `open_image` canonicalization + accept call; Ok-arm sibling rebuild + browsed clear; delete enum/navigate/neighbor_image_path; add the `use folder_navigation::NavigationDirection;` line; ui() intake + gate-swap; layout rename + else-arm; two mod declarations).
7. Edit `src/app/arrow_direction.rs` per §3.4 (mapping, labels, three doc lines).
8. Edit `src/app/floating_controls.rs` doc lines per §3.5 (NO logic edits — verify by re-reading the final diff: only comments may differ).
9. Edit `src/sibling_folder_listing.rs` marker removals per §3.6 (4 removed, 2 kept); edit `src/last_viewed_images.rs` per §3.7 (1 removed).
10. Create `src/app/folder_navigation_tests.rs` per §3.9 (fixture first, then the shared helper, then the 9 locked tests in the listed order).

SELF-CHECK (read files back; no builds):
11. Line counts: `app.rs` ≤ 200 (expect 197; apply the §3.2 contingency if needed); `folder_navigation.rs` ≤ 200 (expect ~130); `empty_folder_view.rs` ≤ 200 (expect ~18); `folder_navigation_tests.rs` ≤ 200 (expect ~195; trim order inside the §3.9 budget guard); `folder_listing.rs` 123; `arrow_direction.rs` 66; `floating_controls.rs` 170; `sibling_folder_listing.rs` 133; `last_viewed_images.rs` 40.
12. Audit: `accept_open_outcome` Err arm one-line UNTOUCHED; Task-2 hook line byte-identical; `browsed_folder_context` touched at exactly THREE sites (empty_state init, Ok-arm clear, enter_empty_folder_mode set); only §3.2 lines changed in `app.rs`; `floating_controls.rs` diff is comments-only; marker census = §4 (7 kept, 0 extra); no new deps (`Cargo.toml`/`Cargo.lock` untouched); no commented-out code; edition-2024 idioms.
13. `git status` → exactly the 10 §6 files (plus any caller-side artifacts; add only the §6 set); `git add` them; re-run `git status`; commit with the §6 message; report files, line counts, expected census 50, commit hash, and this plan's path.

## 8. 4.3 Reviewer Guidance (code-reviewer / code-simplifier)

- Structure: ONE `navigate` and ONE `neighbor_image_path` total, in `src/app/folder_navigation.rs` (plan-mandated child inherent impl; do NOT "fix" them back into app.rs); app.rs keeps intake + state + layouts + gates only. Call sites beyond app.rs (`floating_controls.rs`) stay UNCHANGED.
- Greps: `fn navigate` → 1 hit (folder_navigation.rs); `fn neighbor_image_path` → 1 hit (folder_navigation.rs, `pub(super)`); `fn navigation_context_is_active` → 1 hit; `fn enter_sibling_folder` / `fn enter_empty_folder_mode` / `fn sibling_folder_neighbor_path` / `fn vertical_anchor_path` / `fn sibling_folder_target_image_path` → 1 hit each; `EMPTY_FOLDER_MESSAGE` → 3 hits (definition, show fn, test); `LISTING_ANCHOR_FILE_PLACEHOLDER` → 3 (definition + first-image probe + vertical_anchor_path); `first_listed_image` → 2 (definition + 1 caller); `record_displayed_image` → 2 (definition + Task-2 hook); `canonicalize` in `last_viewed_images.rs` → 0 (D5 landed in `open_image` only); `up`/`down` intake → ArrowUp/ArrowDown before the layout branch in `ui()`.
- Naming caution (intentional, do NOT "fix"): `neighbor_image_path` returns a FOLDER path for the vertical variants — the name is the Wave-2 legacy kept because `src/app/tests.rs` (13 references, byte-identical file at cap) and the landed horizontal contract depend on it; the module + method docs state the dual meaning, and tests 1/3/8 lock the folder-path semantics. Renaming is FORBIDDEN for this task.
- Confirm D2-final: NO skip logic (grep `skip` → 0 in src/, excluding plan files); NO wrap; empty-folder mode writes NO map entry; the centered label renders ONLY via the central-panel else-arm; center Open unreachable while `browsed_folder_context` is set; `floating_controls.rs` diff is comments-only; `src/sibling_folder_listing/tests.rs`, `src/folder_listing/tests.rs`, `src/app/tests.rs`, `src/app/floating_controls_tests.rs`, `src/app/last_viewed_tests.rs` byte-identical; census math 41→50 acknowledged (green run = Task 4).
- Simplifier boundaries: do not merge the axes into one resolver body; do not add derives; do not unwrap/expect on the `Option`s encoding D2/D3; do not move the image-vs-empty decision out of `navigate` into the enablement path; do not add display/texture-clearing APIs.

## 9. Pre-Flight Re-Check List (between write steps)

- [ ] Branch/HEAD/tree still as preflight? No cargo/VM/build command executed?
- [ ] Names exact: `NavigationDirection::{PreviousSiblingFolder, NextSiblingFolder}`, `browsed_folder_context`, `current_sibling_folders`, `folder_navigation::`, `empty_folder_view::`, `LISTING_ANCHOR_FILE_PLACEHOLDER`, `first_listed_image`, `last_displayed_image`, `previous_neighbor_of`, `next_neighbor_of`, `navigation_context_is_active`, `sibling_folder_target_image_path`, `enter_sibling_folder`, `enter_empty_folder_mode`, `sibling_folder_neighbor_path`, `vertical_anchor_path`, `EMPTY_FOLDER_MESSAGE`, `show_empty_folder_message`, `ListedFolderNeighbor::At { folder_path, .. }` decode.
- [ ] The 9 test names match §3.9 exactly; no extra/renamed tests anywhere; `EMPTY_FOLDER_MESSAGE` == "Empty folder" (no punctuation).
- [ ] `src/folder_listing/tests.rs`, `src/sibling_folder_listing/tests.rs`, `src/app/tests.rs`, `src/app/floating_controls_tests.rs`, `src/app/last_viewed_tests.rs` byte-identical.
- [ ] Staged set == §6 list (10 paths); commit message verbatim; no push; v1 plan file untouched.

## 10. Deliberately Out of Scope (4.2 boundary — reviewers enforce)

- NO pan/pan-then-navigate (Wave 5); NO zoom/fullscreen/OpenWith; NO CHANGELOG edits (wave rule); NO docs/README/tech/architecture/project-structure edits (Task 5); NO dev-check/container harness runs or exe rebuild (Task 4); NO version bump (Step 3 done, 0.3.0); NO `[DONE]` TODO marking (4.6); NO edits to existing plan FILES (v1 stays as-is); NO new crate deps / Cargo.toml/lock changes; NO persistence of the last-viewed map beyond app-lifetime; NO same-filename logic; NO skip-traversal past empty folders; NO wrap-around navigation; NO map entries for empty folders; NO skip/pan handling on ↑/↓ in empty-folder mode.
- Message-constant exception (REPLACES v1's "NO new message constants" line): EXACTLY ONE new message constant is allowed — `EMPTY_FOLDER_MESSAGE` in `src/app/empty_folder_view.rs` (§3.8), rendered ONLY in the blank central surface, NEVER routed through `message_text` or the pill's message line. No other message strings may be added.
- NO pill message-line/geometry changes beyond the doc edits in §3.5; the pill keeps `message_text` as its only line and the mode's own line is empty.
- NO edits to `src/app/rails*` (deleted), no new widgets/panels; the empty-folder message is the ONLY new rendered element.

## 11. 4.5a Conformance Cross-Map (for the later frontend verification, spec §7)

- Intake items → §3.2 items 10-11: `ArrowUp`/`ArrowDown` read with `ui.ctx().input(...)` in `ImagesScrollerApp::ui()` BEFORE the layout branch (the branch is now `navigation_context_is_active()` — the spec's "image_is_open() branch" wording maps to this renamed gate; the ordering property is unchanged); keys route through the SAME `navigate` seam as the buttons; Left/Right unchanged; pan absent.
- Buttons/labels → §3.4; enablement → §3.5 (unchanged chain) + §3.1 (vertical arm returns folder adjacency — spec §4.1's helper pattern is kept verbatim via `neighbor_image_path`); disabled arrows paint dimmed via the existing `arrow_color` logic.
- Layout unchanged → §3.5 doc-only; pill message line unchanged in placement/formatting.
- **Superseded-item note (record for 4.5a):** spec §4.3's option table (disable/skip/error) and its "adjacent folder has no image" consequence rows are SUPERSEDED by the user-adjudicated empty-folder mode: the button is ENABLED for any adjacent folder, and instead of an option-table outcome the app enters empty-folder mode (blank surface + centered "Empty folder", ←/→ disabled, ↑/↓ active). The spec §7 checklist item "the message line remains the only status surface" holds FOR THE PILL; the centered blank-surface label is a NEW central-surface status element mandated by the adjudicated decision (spec did not foresee it).
- "No direct mutation of `opened_image_path`/`decoded_image`/`message_text`/`last_viewed_images` from UI" → unchanged (all transitions happen in the seam/Ok arm; UI code only calls `navigate`/`open_via_dialog`).
- The four §8 pending decisions of the spec are answered: item 1 = this plan (new enum variants + single seam, richer return semantics on the vertical arm); item 2 = D2-final (empty-folder mode, NOT disable/skip/error); item 3 = D3 (silent edges, persisting state in empty mode); item 4 = D4 (real temp-dir construction; no UI-side test doubles).
