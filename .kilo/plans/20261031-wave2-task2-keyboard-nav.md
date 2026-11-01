# Task 2 Implementation Plan — Wave 2: Previous/Next Image Navigation (Keyboard)

Date: 2026-10-31
Workflow step: Wave-2 Task 2 → Step 4.1b (architector)
Source TODO: [`.agent/todos/20261029/20261029-todo-4.md`](../../.agent/todos/20261029/20261029-todo-4.md) → `### Task 2: Previous/next image navigation — keyboard`
Global plan: [`.kilo/plans/20261031-wave2-horizontal-scroll.md`](../../.kilo/plans/20261031-wave2-horizontal-scroll.md) (§Task 2, §2.2 constraints, §2.3 decision inventory)
Front-end input: [`.kilo/plans/20261031-wave2-task2-keyboard-nav-frontend-spec.md`](../../.kilo/plans/20261031-wave2-task2-keyboard-nav-frontend-spec.md) (4.1a spec — its §9 pending decisions are decided HERE and nowhere else)
Predecessor: [`.kilo/plans/20261031-wave2-task1-folder-listing.md`](../../.kilo/plans/20261031-wave2-task1-folder-listing.md) (Task 1 module landed at `6615d0a`, fixed at `43b8c44`, marked DONE at `6a1cfad`)
Status: APPROVED-AUTO (user auto-approves per-task plans, decision dated 2026-10-31)

---

## 0. Scope Guard

Task 2 ships EXACTLY: keyboard-driven previous/next sibling-image navigation inside `src/app.rs` (+ the moved app test module `src/app/tests.rs`) and the two narrow dead-code attribute adjustments in `src/folder_listing.rs`. NOTHING else:

- NO on-screen UI buttons (Task 3 owns them; they will reuse `ImagesScrollerApp::navigate`).
- NO up/down folder navigation (Wave 3), NO pan-then-navigate (Wave 5 — arrows navigate DIRECTLY per TODO Task-2 sub-item 2), NO zoom/fullscreen (Waves 4+, LCD).
- NO documentation edits (Task 5 owns `README.md`/`tech.md`/`architecture.md`/`context.md`). NO `CHANGELOG.md` (wave binding). NO edits to existing `.kilo/plans/*` and no TODO-file `[DONE]` marks (workflow steps 4.6/5 own those).
- **NO cargo commands in Task 2** (caller instruction, Task-1 precedent): the host has no Rust/Cargo; container dev-checks belong to Task 4. The implementer self-verifies statically (file-level rules, style, shape) and REPORTS any compile-hazard concern in the step report instead of running cargo.
- `Cargo.toml` needs NOTHING (std-only; eframe/egui already present; version already 0.2.0 from step 3).

## 1. Decision 1 — Edge-of-list behavior: STAY (no wrap), silently

### 1.1 Options analyzed

| Option | On `previous_neighbor(0)` / `next_neighbor(last)` | Consequences |
| --- | --- | --- |
| (1) STAY — stay on the current image | `ListedNeighbor::ListBeginningReached` / `ListEndReached` map to a plain no-op; the current image stays; NOTHING is printed | Zero changes to the landed Task-1 module (the two edge variants are consumed as-is, and they exist precisely for this); matches classic gallery-era viewer behavior (Windows Photo Viewer family: pressing beyond the first/last image does nothing); a single-image folder yields prev AND next both no-ops, so no redundant re-decode of the same image; Wave 3's vertical (sibling-folder) navigation has no defined wrap order in the brief at all, so the same "stay at the boundary" rule reuses verbatim. |
| (2) WRAP — wrap-around continuity | First-position Left opens the LAST image; last-position Right opens the FIRST | Requires a NEW neighbor API in `src/folder_listing.rs` (wrap to `len-1`/`0` — `neighbor_at` is private; neither `ListBeginningReached` nor `ListEndReached` can be answered by Task 1's API today) → rewrites a landed, reviewed module and its tests mid-wave; contradicts classic viewer behavior the brief recreates ("recreates the behavior of the classic Windows gallery-era image viewer"); a single-image folder would re-open the SAME image on every press (pointless decode churn); Wave 3 folder navigation could not reuse wrap semantics (there is no "folder before the first folder" definition in the brief) → two inconsistent edge rules across the navigation engine. |

**PICK: Option (1) — STAY.** Decisive reasons: (i) it consumes the Task-1 `ListBeginningReached`/`ListEndReached` variants exactly as designed, with no API change; (ii) it matches the classic viewer behavior the brief names as the product goal; (iii) it is the behavior Wave 3's folder navigation can adopt identically (stay at the first/last sibling folder), giving the navigation engine ONE edge rule across both axes.

### 1.2 EXACT final edge semantics (binding)

- With the current image at listed index `0`: `ArrowLeft` is a **no-op** — no path change, no decode, no state change of any kind.
- With the current image at the LAST listed index: `ArrowRight` is a **no-op** — same.
- An edge hit shows **NO message**: the message bar stays exactly as it is (untouched). The Wave-1 invariant "message = a failed decode attempt only" is therefore preserved: `UNSUPPORTED_FILE_MESSAGE` appears exclusively after a real failed open, never for a boundary.
- Edge no-ops do NOT clear or overwrite any currently shown message either (a stale failure message remains visible until the next successful open clears it — existing `accept_open_outcome` behavior).
- A single-image folder: BOTH directions are no-ops (the stay rule; validated by test `navigation_at_the_first_and_last_list_positions_stays_silent`).

## 2. Decision 2 — App-state shape: `Option<FolderListing>` field, index DERIVED per press

### 2.1 Options analyzed

| Option | Shape | Analysis |
| --- | --- | --- |
| (1) Listing cached, index derived | `current_folder_listing: Option<FolderListing>` field; the current press resolves its own index via `listed_index_of(self.opened_image_path)` | The opened path is the SINGLE source of truth: re-derivation per navigation press is always consistent with whatever open last succeeded (dialog, CLI, or navigation). No second variable that can go stale; no extra bookkeeping in the failure arm. Cost: one `fs::canonicalize` + linear scan per key press (per EVENT, never per frame) — negligible next to a decode. |
| (2) Listing cached AND index stored explicitly | `current_folder_listing: Option<FolderListing>` + `current_listed_index: Option<usize>` | A second state variable must be maintained on EVERY path-entry point (CLI build in `main.rs`, dialog, navigation) and has no defined behavior after a failed open (the path stays but the index would be about nothing) — the same `listed_index_of` resolution is needed anyway to keep it honest, so the stored copy adds staleness risk with zero payoff. |

**PICK: Option (1).** The index is NEVER stored. Rationale: the Wave-1 `opened_image_path` is already the survivor of every failure path; deriving the index from it per press cannot drift.

### 2.2 EXACT state mechanics (binding)

- New private field on `ImagesScrollerApp`: `current_folder_listing: Option<FolderListing>` — `Some` from the first successful open onward; `None` initially.
- **Rebuild trigger — exactly ONE site**: the success arm of `accept_open_outcome` (which is the shared seam reached by ALL open paths: CLI argument via `main.rs` → `open_image`, in-app `Open…` dialog → `open_via_dialog` → `open_image`, and Task-2 keyboard navigation → `navigate` → `open_image`). The rebuild is a plain `Option` assignment — `Some(new_listing)` replaces the old one, and `None` from `from_opened_image` (bare name `photo.png`, unresolvable folder) is stored as-is, never unwrapped, so the Wave-1 app-level test `successful_open_stores_path_and_payload_and_clears_message` (path `photo.png`, no real folder) stays green.
- **Failure arm touches NOTHING**: a failed open keeps the previous listing (and previous path/image/display) — only `message_text` changes (existing behavior, unchanged).
- **Derivation**: each Left/Right press resolves the current index fresh via `listing.listed_index_of(opened_path)`; `None` (empty listing / file deleted / unresolvable path / not listed) ⇒ NO navigation at all (the Task-1 consumption contract §8-2).
- RAM-only: the field is plain state in the app struct, destroyed with the app; nothing persists (v1 constraint honored).

## 3. Decision 3 — Decode-failure navigation semantics: no advance, message only

Binding mechanics (per TODO/global-plan recommendation and the 4.1a spec §2.3):

- Navigation to the ±1 neighbor calls the EXISTING `ImagesScrollerApp::open_image(PathBuf)` seam with the neighbor's path — the navigation code contains NO decode logic, NO texture logic, NO direct state mutation.
- On decode failure the EXISTING `accept_open_outcome` error arm produces the complete required behavior with no new code:
  - `opened_image_path` stays the PREVIOUS image's path (the neighbor was NOT opened → navigation does NOT advance; the next press re-derives the index from the still-open path),
  - `decoded_image` + `current_display` stay untouched → the previous image remains visible behind the message bar,
  - `current_folder_listing` stays untouched (rebuild happens only in the success arm),
  - `message_text` = `image_loading::UNSUPPORTED_FILE_MESSAGE` (the single open-failure message).
- The state's opened path is therefore unchanged on failure, and THIS is what makes derivation-per-press self-consistent (§2): a failed step cannot corrupt the position.

## 4. Decision 4 — Key-intake integration and the navigation method shape

### 4.1 Key intake (per the 4.1a spec's pinned API, VERBATIM pattern)

Inside `ImagesScrollerApp::ui`, at the VERY TOP, before any panel creation, every frame:

```rust
let left_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowLeft));
let right_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowRight));
if left_pressed {
    self.navigate(NavigationDirection::Previous);
}
if right_pressed {
    self.navigate(NavigationDirection::Next);
}
```

- `key_pressed` ONLY (never `key_down` — single step per press/repeat event, spec §1.3).
- Reads run unconditionally every frame (spec §3.2); no manual key-state cache fields are added (spec §4 anti-pattern); no pan/zoom logic exists (Wave 5 owns interplay — TODO Task-2 sub-item 2 "navigate DIRECTLY" is encoded by this absence).
- There are no text-entry widgets in the app, so context-level key reading has no focus conflicts; v1 keyboard handling is deliberately unconditional.

### 4.2 Navigation method shape — ONE method + a private enum in an existing file

- **ONE private method**: `fn navigate(&mut self, direction: NavigationDirection)` — 1 non-self param (≤ max-2 rule), so NO param-object file is created.
- **Direction type**: private enum `NavigationDirection { Previous, Next }` — placed in the EXISTING file `src/app.rs` (Caller's "prefer ONE method + an enum in an existing file"). It stays file-private (`enum`, no `pub`): Task 3's buttons live in the same file and call the same private method; Wave 3 moves it only if a real cross-module need ever appears (no assumption made now).
- **Resolution helper**: ONE private method `fn neighbor_image_path(&self, direction: NavigationDirection) -> Option<PathBuf>` returning the `At` neighbor path, or `None` for: no opened image, no listing, unresolvable/unlisted index (`listed_index_of == None`), and BOTH edge variants (`ListBeginningReached`/`ListEndReached` → stay semantics of §1). This helper is also the container-testable seam (§5).

```rust
enum NavigationDirection {
    Previous,
    Next,
}
```

(renders immediately after the `ImagesScrollerApp` struct definition in `src/app.rs`)

```rust
/// Navigates one sibling image per arrow direction; a failed neighbor open
/// keeps the current image (Wave-1 open-flow failure rule).
fn navigate(&mut self, direction: NavigationDirection) {
    if let Some(neighbor_path) = self.neighbor_image_path(direction) {
        self.open_image(neighbor_path);
    }
}

/// Resolves the ±1 sibling path from the opened path and the current
/// listing; every "nothing to navigate to" case (no image, no listing,
/// unresolvable index, list beginning/end reached) resolves to None.
fn neighbor_image_path(&self, direction: NavigationDirection) -> Option<PathBuf> {
    let Some(opened_path) = self.opened_image_path.as_ref() else {
        return None;
    };
    let Some(listing) = self.current_folder_listing.as_ref() else {
        return None;
    };
    let Some(listed_index) = listing.listed_index_of(opened_path) else {
        return None;
    };
    let neighbor = match direction {
        NavigationDirection::Previous => listing.previous_neighbor(listed_index),
        NavigationDirection::Next => listing.next_neighbor(listed_index),
    };
    let ListedNeighbor::At { image_path, .. } = neighbor else {
        return None;
    };
    Some(image_path)
}
```

Rule compliance: every method ≤ 50 lines; nesting depth ≤ 2 everywhere; every `if`/`let-else` is a single-section condition; all new members private; no comments beyond ≤ 2-3-line method docs (self-documenting names).

### 4.3 `ui()` — final full shape (only the intake block is new)

```rust
impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let left_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowLeft));
        let right_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowRight));
        if left_pressed {
            self.navigate(NavigationDirection::Previous);
        }
        if right_pressed {
            self.navigate(NavigationDirection::Next);
        }

        egui::Panel::top("open_toolbar").show(ui, |ui| {
            self.open_toolbar_button(ui, frame);
        });

        if let Some(message) = self.message_text {
            egui::Panel::bottom("message_bar").show(ui, |ui| {
                ui.label(message);
            });
        }

        egui::CentralPanel::default().show(ui, |panel_ui| {
            if let Some(decoded_image) = self.decoded_image.as_ref() {
                self.current_display.paint(panel_ui, decoded_image);
            }
        });
    }
}
```

The display/texture path is UNTOUCHED (one upload per successful decode, Wave-1 rule).

## 5. App-level unit tests (container-runnable; `src/app/tests.rs`)

### 5.1 WHY the app tests move to a file

`app.rs` currently holds 4 inline tests (49 lines). Adding the navigation code (~40 lines) plus 5 new tests plus a temp-folder fixture (~32 lines) inline would exceed the 200-line file limit. REMEDY (established precedent: `src/folder_listing.rs` → `src/folder_listing/tests.rs`): move the app test module to a file — replace the inline `#[cfg(test)] mod tests { … }` block in `src/app.rs` with:

```rust
#[cfg(test)]
mod tests;
```

and CREATE `src/app/tests.rs` containing the 4 EXISTING tests MOVED VERBATIM plus the 5 new tests below. Both files stay ≤ 200 lines. The caller's "keep tests in src/app.rs tests mod" intent is preserved exactly: it IS the app.rs tests mod — materialized as the same file-based module pattern the project already uses (`src/app/tests.rs` is `app`'s own `tests` module; private items of `app` remain accessible to it, as they already are to the inline mod).

### 5.2 App-level test shape (`src/app/tests.rs`)

Header (moved imports + new ones):

```rust
use super::{ImagesScrollerApp, NavigationDirection};
use crate::image_loading::{DecodedImage, UNSUPPORTED_FILE_MESSAGE};
use std::fs;
use std::path::PathBuf;
```

(The 4 moved tests keep their bodies verbatim, incl. `super::APP_WINDOW_TITLE`.)

Fixture — temp folders under `std::env::temp_dir()` (container `/tmp`), NEVER inside the repo; automatic cleanup via `Drop` even on assertion panic:

```rust
struct TempImageFolder {
    folder_path: std::path::PathBuf,
}

impl TempImageFolder {
    fn create(test_case: &str) -> Self {
        let folder_path = std::env::temp_dir().join(format!(
            "img_scroller_app_navigation_{test_case}_{}",
            std::process::id()
        ));
        fs::create_dir_all(&folder_path).expect("temp image folder created");
        Self { folder_path }
    }

    fn create_file(&self, file_name: &str) {
        fs::write(self.folder_path.join(file_name), b"x").expect("fixture file written");
    }

    fn image_path(&self, file_name: &str) -> std::path::PathBuf {
        self.folder_path.join(file_name)
    }
}

impl Drop for TempImageFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.folder_path);
    }
}

fn sample_decoded_image() -> DecodedImage {
    DecodedImage {
        width: 2,
        height: 2,
        pixels: vec![0; 16],
    }
}
```

NEW TEST 1 — empty-state no-op (TODO/global-plan testable case):

```rust
#[test]
fn navigation_on_the_empty_state_does_not_change_any_state() {
    let mut app = ImagesScrollerApp::empty_state();
    app.navigate(NavigationDirection::Previous);
    app.navigate(NavigationDirection::Next);
    assert!(app.opened_image_path.is_none());
    assert!(app.current_folder_listing.is_none());
    assert!(app.decoded_image.is_none());
    assert!(app.message_text.is_none());
}
```

NEW TEST 2 — ±1 sibling resolution in Task-1 sort order:

```rust
#[test]
fn navigation_resolves_the_previous_and_next_sibling_in_sort_order() {
    let folder = TempImageFolder::create("sibling_order");
    folder.create_file("a.png");
    folder.create_file("b.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(folder.image_path("a.png"))
    );
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Next),
        Some(folder.image_path("c.png"))
    );
}
```

(`accept_open_outcome` stores path + payload + builds the listing; `b.png` exists on disk so `listed_index_of` canonicalizes; resolution through `previous_neighbor`/`next_neighbor` proves index derivation.)

NEW TEST 3 — edge STAY (decision §1) incl. single-image folder and message silence:

```rust
#[test]
fn navigation_at_the_first_and_last_list_positions_stays_silent() {
    let folder = TempImageFolder::create("list_edges");
    folder.create_file("a.png");
    folder.create_file("b.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert_eq!(app.neighbor_image_path(NavigationDirection::Previous), None);
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Next),
        Some(folder.image_path("b.png"))
    );
    app.accept_open_outcome(folder.image_path("c.png"), Ok(sample_decoded_image()));
    assert_eq!(app.neighbor_image_path(NavigationDirection::Next), None);
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(folder.image_path("b.png"))
    );
    let solo_folder = TempImageFolder::create("single_image_folder");
    solo_folder.create_file("only.png");
    app.accept_open_outcome(solo_folder.image_path("only.png"), Ok(sample_decoded_image()));
    assert_eq!(app.neighbor_image_path(NavigationDirection::Previous), None);
    assert_eq!(app.neighbor_image_path(NavigationDirection::Next), None);
    assert!(app.message_text.is_none());
}
```

(Under a WRAP decision the solo folder would resolve `Some(only.png)` on both sides — this test pins the STAY decision.)

NEW TEST 4 — neighbor decode failure keeps the current image (decision §3):

```rust
#[test]
fn failed_neighbor_open_keeps_the_current_image_and_shows_the_message() {
    let folder = TempImageFolder::create("failure_keeps_image");
    folder.create_file("a.png");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.navigate(NavigationDirection::Previous);
    assert_eq!(app.opened_image_path, Some(folder.image_path("b.png")));
    assert!(app.decoded_image.is_some());
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(folder.image_path("a.png"))
    );
}
```

(Container reality: `platform::decode_image` on the non-Windows stub deterministically fails, so `navigate(Previous)` → `open_image(a.png)` exercises the REAL failure arm deterministically: the path is still `b.png`, the previous payload still visible, the message set, the listing intact — the last assert proves the position did NOT advance after the failed step.)

NEW TEST 5 — listing rebuild tracks the opened folder:

```rust
#[test]
fn every_successful_open_rebuilds_the_listing_from_the_new_folder() {
    let first_folder = TempImageFolder::create("rebuild_first");
    first_folder.create_file("a.png");
    first_folder.create_file("b.png");
    first_folder.create_file("c.png");
    let second_folder = TempImageFolder::create("rebuild_second");
    second_folder.create_file("d.png");
    second_folder.create_file("e.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(first_folder.image_path("b.png"), Ok(sample_decoded_image()));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(first_folder.image_path("a.png"))
    );
    app.accept_open_outcome(second_folder.image_path("e.png"), Ok(sample_decoded_image()));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::Previous),
        Some(second_folder.image_path("d.png"))
    );
    assert_eq!(app.neighbor_image_path(NavigationDirection::Next), None);
}
```

(After opening `e.png`, a PREVIOUS returns `second`'s first sibling `d.png` — a stale first-folder listing would resolve `None` because `e.png` is not in it; the assert discriminates rebuild-vs-stale.)

### 5.3 Test coverage boundaries

- Skill-app-level tests are pure logic on private members (accessible from the file-based child `tests` module) — all 5 run in the Linux container under `cargo test --locked`.
- UI-FRAME tests (key-intake in `ui`) are NOT container-runnable: `eframe::App::ui` needs a live egui frame. Coverage responsibility: the 4.5a front-end verification report (static checklist §7 of the 4.1a spec) + the Task-4 OPERATOR-RUN on real Windows.
- Success-transition rendering (texture, actual display of a decoded neighbor) is likewise operator-run territory.

## 6. Files to modify EXACTLY

| Action | Path | Changes |
| --- | --- | --- |
| MODIFY | `src/app.rs` | (1) add `use crate::folder_listing::{FolderListing, ListedNeighbor};` after `use crate::app_display::CurrentImageDisplay;`, before `use crate::image_loading::…;` (alphabetical); (2) DELETE the `#[allow(dead_code)]` attribute line above `opened_image_path` (the field is now read by `neighbor_image_path`; keep its doc comment verbatim); (3) add the `current_folder_listing` field (snippet below); (4) add `current_folder_listing: None,` in `empty_state`; (5) add the rebuild line in the `accept_open_outcome` success arm (snippet below — ORDERING MATTERS); (6) add `enum NavigationDirection` immediately after the `ImagesScrollerApp` struct; (7) add `navigate` + `neighbor_image_path` inside `impl ImagesScrollerApp` after `open_via_dialog`; (8) add the intake block at the top of `ui()` (§4.3); (9) replace the inline `#[cfg(test)] mod tests { … }` block with `#[cfg(test)]\nmod tests;`. |
| CREATE | `src/app/tests.rs` | The moved 4 tests verbatim + fixture + `sample_decoded_image` + the 5 new tests of §5.2. |
| MODIFY | `src/folder_listing.rs` | ONLY the three attribute changes of §7 (dead-code marker retirement). NO logic/type/enum/signature changes whatsoever. |
| — none | `src/folder_listing/tests.rs` | NO changes needed: the §7 attribute changes do not alter any import or API consumed by the tests (they import `FolderListing`, `ListedNeighbor` — both unchanged). |
| — none | `src/main.rs`, `Cargo.toml`, `Cargo.lock` | NO changes (CLI open flows through `open_image` → rebuild automatic; no new deps, no version change). |

### 6.1 Exact snippets for `src/app.rs`

Field + initializer — before:

```rust
pub struct ImagesScrollerApp {
    /// Path of the last successfully decoded image (read again by Wave 2's navigation code).
    #[allow(dead_code)]
    opened_image_path: Option<PathBuf>,
```

after:

```rust
pub struct ImagesScrollerApp {
    /// Path of the last successfully decoded image (read again by Wave 2's navigation code).
    opened_image_path: Option<PathBuf>,
    /// Sibling-image listing of the opened image's folder in folder-listing order
    /// (rebuilt on every successful open; None until the first open succeeds).
    current_folder_listing: Option<FolderListing>,
```

```rust
    fn empty_state() -> Self {
        Self {
            opened_image_path: None,
            current_folder_listing: None,
            decoded_image: None,
            message_text: None,
            current_display: CurrentImageDisplay::initial(),
        }
    }
```

`accept_open_outcome` success arm — the rebuild MUST come FIRST because `path.as_path()` cannot be called after `Some(path)` moves `path`:

```rust
match outcome {
    Ok(decoded_image) => {
        self.current_folder_listing = FolderListing::from_opened_image(path.as_path());
        self.opened_image_path = Some(path);
        self.decoded_image = Some(decoded_image);
        self.message_text = None;
        self.current_display.accept_decoded_image();
    }
    Err(_failure) => {
        self.message_text = Some(image_loading::UNSUPPORTED_FILE_MESSAGE);
    }
}
```

(The error arm is UNCHANGED — the failure rule of §3 rests on it untouched.)

## 7. Dead-code marker retirement in `src/folder_listing.rs` (bound HERE per Task-1 plan §3.5)

Task 1's module-level `#![allow(dead_code)]` covered the entire module because NOTHING consumed it. Task 2 wires the first consumers (`FolderListing`, `ListedNeighbor`, `from_opened_image`, `listed_index_of`, `previous_neighbor`, `next_neighbor`), so the module-level marker is removed and replaced by the narrowest attributes for the two items that remain consumed ONLY by `cfg(test)` tests (test usage does not count for the dead-code lint; derived-trait usage is ignored by it):

1. DELETE the module attribute line 5:
   - from: `#![allow(dead_code)]` (immediately after the module doc comment) → REMOVED.
2. ADD one per-item attribute on the never-consumed-outside-tests method (Wave-1 per-item precedent, `src/image_loading.rs`):
   ```rust
   #[allow(dead_code)]
   pub(crate) fn listed_image_count(&self) -> usize {
       self.image_paths.len()
   }
   ```
3. ADD one per-field attribute on the never-read `At` variant field (`previous/next_neighbor` construct it; Task 2's let-else pattern reads only `image_path`):
   ```rust
   pub(crate) enum ListedNeighbor {
       At {
           #[allow(dead_code)]
           listed_index: usize,
           image_path: PathBuf,
       },
       ListBeginningReached,
       ListEndReached,
   }
   ```
   (First-position use pattern in `src/app.rs`: `let ListedNeighbor::At { image_path, .. } = neighbor else { return None; };`)

All other `folder_listing.rs` items have real Task-2 consumers (verified item-by-item in §10) — no further attributes. ATTRIBUTE SAFETY NOTE: an `#[allow(dead_code)]` is harmless even if a future consumer appears; if Task 4's container clippy turns out to flag either the kept attributes (as no-ops) or NEW dead items, that is a 4.3-fix-cycle matter — the implementer does NOT run cargo and must not pre-emptively allow-list anything else.

`src/folder_listing/tests.rs`: NO import or API changes — state this check explicitly during self-verification (imports `super::{FolderListing, ListedNeighbor}`; both names and all signatures unchanged by §7).

## 8. Implementation steps (in order)

1. Verify branch: `git branch --show-current` must print `feat/wave-2-horizontal-scroll`; on mismatch → STOP, return a question to the caller.
2. Edit `src/app.rs` (§6.1 exact snippets, in the listed item order 1–9; keep rustfmt-style: 4-space indent, ≤ 100 columns, existing import/comment style).
3. Create `src/app/tests.rs` per §5.2 (move the 4 tests VERBATIM first, then append the fixture, helper, and 5 new tests).
4. Apply the three attribute changes of §7 to `src/folder_listing.rs` (no other edit to that file).
5. Self-verify WITHOUT cargo (checklist in §10), fix any drift found by re-editing files only.
6. Commit per §9.
7. In the step report, note every suspected compile/rustfmt/clippy hazard (dead-code, borrows, fmt drift) as an open concern for Task 4 — DO NOT run cargo.

## 9. Terminal commands + Git actions

- Terminal: NONE. No cargo, no fmt, no clippy, no test runs in this step (Task-4 container dev-checks own verification; caller instruction).
- Git (branch already exists from workflow step 2 — never re-create here):
  1. Read `.gitignore`; run `git status` (gitignore compliance): ensure NOTHING matching `dist/`, `logs/`, `target/`, `*.log`, `.kilo/agent-manager.json` is or could be staged.
  2. Stage EXACTLY: `src/app.rs`, `src/app/tests.rs`, `src/folder_listing.rs`, `.kilo/plans/20261031-wave2-task2-keyboard-nav.md` (this plan), `.kilo/plans/20261031-wave2-task2-keyboard-nav-frontend-spec.md` (the task's untracked 4.1a artifact — committed alongside like Task 1's plan).
  3. Do NOT stage `.agent/project-info/context.md` (unstaged planner/architector upkeep — committed at wave closing, not by this step).
  4. One implementation commit: message EXACTLY `feat: previous/next keyboard navigation via arrow keys (Wave 2 Task 2)`.
  5. NO push in this step (push is restricted to workflow step 5, `main` → `origin` only). TODO `[DONE]` marks are NOT made here (step 4.6).

## 10. Implementer self-verification checklist (no cargo — read/inspect only)

1. `src/app.rs` ≤ 200 lines, `src/app/tests.rs` ≤ 200 lines, `src/folder_listing.rs` unchanged at 125 lines.
2. Every new method ≤ 50 lines; max nesting depth 2; every `if`/`let-else` single-section; `navigate(&mut self, NavigationDirection)` and `neighbor_image_path(&self, NavigationDirection)` are each 1 non-self param.
3. `NavigationDirection` enum is file-private (no `pub`); `current_folder_listing` field private; all new fn private.
4. `ui()` intake is the FIRST code in the method, before the top panel; uses `key_pressed` (search: NO `key_down` anywhere in Task-2 code); no key-state cache fields added to the struct.
5. The success arm rebuild line reads `path.as_path()` BEFORE `self.opened_image_path = Some(path);` (use-after-move prevention — re-read the method top-to-bottom to confirm the order).
6. The error arm, `open_image`, `open_via_dialog`, `open_toolbar_button`, `window_options`, display code: completely unchanged.
7. `folder_listing.rs`: module attribute gone; exactly the two new per-item attributes; everything else byte-identical to the pre-edit file (diff mentally / re-read; `git diff` shows only attribute lines).
8. `src/folder_listing/tests.rs` untouched.
9. No commented-out code; minimal comments (method docs ≤ 3 lines each).
10. `git status` after edit: only the expected files modified/created; nothing gitignored staged.
11. Dead-code consumer recheck (for the §7 boundary): `FolderListing`, `ListedNeighbor::At`+`image_path`, `ListBeginningReached`/`ListEndReached` (constructed in `previous_neighbor`/`next_neighbor`), `from_opened_image`, `listed_index_of`, `previous_neighbor`, `next_neighbor` — each read/used by `src/app.rs`; NOTHING else newly flagged is pre-allowed.

## 11. Risk notes

- **Dead-code retirement risk** (the known compile-hazard surface of this task): if container clippy (Task 4) finds the two §7 keeps insufficient/unnecessary (`listed_index` read through a derive? `listed_image_count` used?), handle in the Task-2 4.3 fix cycle — exact remediation is: add/remove the specific narrow attribute; NEVER re-add a module-level allow.
- **rustfmt drift risk** (Wave-1 precedent `windows.rs`): if `cargo fmt --check` fails in Task 4, run container-side `cargo fmt` in that step (or via 4.3-fix) — implementer matches existing style now to minimize this.
- **Behavior risk on real Windows** (operator-run): key focus, filename sort on NTFS (driver-letter casing resolved by canonicalize per Task-1 §3.3), and the single-image-folder stay behavior all fall to Task-4 OPERATOR-RUN; container tests pin the logic layer.
- `let-else` + enum field `..` patterns and `#[allow(dead_code)]` on an enum-variant field are stable, valid Rust (no edition/toolchain concern with the pinned `rust:1.99.1-bookworm` container).

## 12. Out of scope (explicit)

- On-screen left/right UI buttons and any UI layout (Task 3 — mandatory propose-and-ask gate there).
- Up/Down arrow folder navigation, last-viewed memory, sibling-folder first-image rule (Wave 3; vertical directions must NOT appear in `NavigationDirection` yet).
- Pan-then-navigate while zoomed (Wave 5; `NavigationDirection` must stay image-horizontal-only).
- Zoom in/out, fit-width/fit-height (Wave 4), fullscreen (later wave).
- "Open with..." menu, window-title-to-filename change, accessibility announcements.
- All documentation files, `CHANGELOG.md`, existing plan files, TODO files (`[DONE]` marking is steps 4.6/5).
- Note for later steps (docs-specialist, Task-2 step 4.4 / Task 5): `.agent/project-structure.md` must eventually list `src/folder_listing.rs`, `src/folder_listing/tests.rs`, `src/app/tests.rs` (it is already stale for the Task-1 modules) — planner/docs territory, never the 4.2 implementer.

## 13. Integration seam (recording for later tasks — do NOT implement now)

- Task 3 buttons: call `self.navigate(NavigationDirection::Previous/Next)` from the toolbar closure — ONE shared path, no separate handler.
- Task 4: dev-checks (`vm_status` first → `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` → `ALL CHECKS PASSED` exit 0 authoritative) + exe refresh (`sh scripts/build-windows.sh`) + the 5 new app-level tests are in the suite; expected project test count after this task: 25 (15 Wave-1 + 5 Task-1 + 5 Task-2).
- Wave 3: reuses `neighbor_image_path`-style resolution with vertical list machinery; the STAY edge rule (§1) applies unchanged; `NavigationDirection` then grows or is superseded per that wave's architector.

## 14. Verification mapping (TODO Task-2 sub-items → plan elements)

| TODO Task-2 sub-item | Covered by |
| --- | --- |
| Left arrow → previous sibling; Right arrow → next sibling in sorted order | §4.1 intake mapping; §4.2 resolution; TEST 2 |
| Arrows navigate DIRECTLY (pan-then-navigate is Wave 5) | §4.1 note — no pan/zoom state exists; no gating logic |
| First/last behavior decided in architector step | §1 — STAY, silent; TEST 3 |

4.1a-spec acceptance criteria 1–6: all reachable (keys via 4.5a/operator; criteria 2/3/4 also proven by TESTS 1/4 and the unchanged seam; criterion 6 evidence deferred to Task 4).

---

## Addendum — Decisions Summary for the caller (briefing later steps)

1. EDGE BEHAVIOR: STAY, silently (no wrap) — `ListBeginningReached`/`ListEndReached` and single-image folders are no-ops; NO message on an edge hit (message bar unchanged; only failed decodes show messages); chosen because it consumes Task-1's variants with zero API change, matches the classic viewer the brief recreates, and gives Wave 3 one identical edge rule.
2. STATE: `current_folder_listing: Option<FolderListing>` private field on `ImagesScrollerApp`, rebuilt at EXACTLY ONE site — `accept_open_outcome` success arm (first line, before the `Some(path)` move) — covering CLI, dialog, and keyboard opens; the current index is NEVER stored — derived per press via `listed_index_of(opened_image_path)` (opened path = single source of truth).
3. NAVIGATION SHAPE: ONE private method `ImagesScrollerApp::navigate(direction: NavigationDirection)` + file-private `enum NavigationDirection { Previous, Next }` in existing `src/app.rs` (1 non-self param each; no new param-object file); resolution seam `neighbor_image_path(&self, NavigationDirection) -> Option<PathBuf>` (doubles as the container-test seam).
4. FAILURE SEMANTICS: neighbor opens through the untouched `open_image` → `accept_open_outcome` seam; decode failure keeps path/image/display/listing untouched, shows `UNSUPPORTED_FILE_MESSAGE`, and does NOT advance.
5. TESTS: app test mod moved file-wise to `src/app/tests.rs` (4 existing verbatim + fixture + helper) with 5 new: `navigation_on_the_empty_state_does_not_change_any_state`, `navigation_resolves_the_previous_and_next_sibling_in_sort_order`, `navigation_at_the_first_and_last_list_positions_stays_silent`, `failed_neighbor_open_keeps_the_current_image_and_shows_the_message`, `every_successful_open_rebuilds_the_listing_from_the_new_folder`; UI-frame key-intake tests NOT container-runnable → 4.5a static report + Task-4 OPERATOR-RUN.
6. DEAD-CODE: module `#![allow(dead_code)]` REMOVED from `src/folder_listing.rs`; two narrow per-item attributes added (`listed_image_count`, `At.listed_index`); `opened_image_path`'s attribute removed (now read). NO cargo in 4.2 — implementer reports compile-hazard concerns; dev-checks in Task 4.
7. GIT: one commit `feat: previous/next keyboard navigation via arrow keys (Wave 2 Task 2)` staging `src/app.rs`, `src/app/tests.rs`, `src/folder_listing.rs`, both task-2 plan files; no push; no TODO markers (4.6).
