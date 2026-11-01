# Wave 3 / Task 2 — 4.1b Analysis & Implementation Plan: Session Last-Viewed Memory (RAM-only)

- Date: 2026-11-01
- Source: [`.agent/todos/20261029/20261029-todo-5.md`](../../.agent/todos/20261029/20261029-todo-5.md) → `### Task 2` (binding)
- Global plan: [`.kilo/plans/20261101-wave3-vertical-scroll.md`](20261101-wave3-vertical-scroll.md) → "Session last-viewed memory (RAM-only)" pre-analysis
- Scope: Task 2 ONLY. NOT front-end. Per-task plan auto-approved by the user's prior "Approve Global and Tasks Plans" choice.
- Live state verified 2026-11-01: branch `feat/wave-3-vertical-scroll` carries Wave-3 Task 1 (landed `src/sibling_folder_listing.rs` + tests, `src/name_collation.rs`); unit-test census = **36** (app 9, app/floating_controls_tests 1, folder_listing 5, scaling 5, image_loading 4, app_display 2, name_collation 2, sibling_folder_listing 8).

## 1. Task Statement (binding, from the TODO)

- RAM-only map: current folder → last viewed image path (per folder visited this session). Updated whenever an image is displayed.
- Destroyed when the application closes; NO persistence to disk in v1 (no files written, no settings storage).
- View-front: state-only. NO `app.rs` behavior change beyond the map-update hook; NO UI changes; the map is consumed by Task 3 (target-priority rule) and previewed in tests only.

## 2. Locked Design Decisions (the ARCHITECT encodes these exactly — implementer does not re-decide)

### 2.1 Data shape

- Collection: `std::collections::HashMap<PathBuf, PathBuf>` — key = folder path, value = last displayed image path.
- Location: a dedicated module `src/last_viewed_images.rs` holding struct `LastViewedImages` with ONE private field wrapping the map. Justification:
  - `src/app.rs` is at 192 lines (≤200 cap is a hard rule); inline field + record/lookup methods would exceed the cap. The wave's architecture.md lists "Session Last-Viewed Memory" as its own component; a named module mirrors that and the Wave-2 precedent (`folder_listing.rs`), and keeps ALL map writing single-sourced in one file.
  - NO persistence, NO serialization, NO new crate (std only), no clone-heavy churn (the map stores path values once per successful open; lookups borrow).
- The struct is `pub(crate)`; the map field stays private ("prefer private members" rule).

### 2.2 Key derivation (ARCHITECT decision — exact)

- Key = `image_path.parent()` — the image's parent folder, plain. **NO `fs::canonicalize`** in the memory.
  - Rationale: zero extra I/O on the open hot path; deterministic and container-testable; every open path in a session comes from the same sources (dialog/CLI) so per-session consistency holds.
  - Rejected alternative: canonicalizing keys (as `folder_listing`/`sibling_folder_listing` do internally for their own index resolution). Not needed for the map: the map never indexes into a listing — it is only key→value storage. Simpler wins.
  - Bad-parent fallback (rare totality case, e.g. a root path): `parent()` is None → use `Path::new("")` as the key. Encoded in the code below.
- **Task-3 consumption contract (documented, not implemented here)**: Task 3's target-priority lookup calls `last_displayed_image(&target_folder_path)`. Task 3's 4.1b owns ensuring the folder-path form it passes matches this key derivation. If Task 3's architector finds a normalization need, it adds it at the Task-3 seam — NOT in this module.

### 2.3 Update semantics (ARCHITECT decision — exact)

- The update happens INSIDE `ImagesScrollerApp::accept_open_outcome`'s `Ok(…)` arm — the success transition — at the point where the path/payload are stored and the message cleared. This is the single point where an image becomes the displayed image ("displayed" in state terms = the success transition commits the decoded payload + pending texture upload; the next-frame paint is guaranteed by the pending-upload flag in `app_display.rs`, so texture-upload timing is NOT the update point).
- Exact placement: the hook line goes AFTER `self.current_folder_listing = FolderListing::from_opened_image(path.as_path());` and BEFORE `self.opened_image_path = Some(path);` — `path` is still a local binding there (the next line moves it), so the hook can borrow `&path` once with no clone/expect.
- Failure semantics: `accept_open_outcome`'s `Err(…)` arm is UNTOUCHED. A failed neighbor open keeps the previously displayed image, so the failing folder's last-viewed entry must stay the previously viewed image — the failure path never writes the map. Verified against `src/app.rs` lines 68-70 (~only sets `message_text`).
- First successful open (CLI `main.rs` / dialog `open_via_dialog` / any `open_image` route): all routes funnel through `open_image` → `accept_open_outcome` → the hook; `HashMap::insert` creates the folder entry on that first success. No separate "first open" code path.
- "Destroyed when the application closes": the map lives as a field of `ImagesScrollerApp` (created in `empty_state()`); eframe drops the app struct on close, freeing the RAM. NO `Drop` impl, NO cleanup code, NO exit hook. Persistence absence in v1 is satisfied by simply never writing anything.

### 2.4 Access policy (private-by-default + interim dead-code markers)

API of `LastViewedImages` (3 pub(crate) items, no more):

| Item | Visibility | Needed by | dead_code marker? |
| --- | --- | --- | --- |
| `empty()` | `pub(crate)` | `app.rs` `empty_state()` now | No (used) |
| `record_displayed_image(&mut self, image_path: &Path)` | `pub(crate)` | the `app.rs` success-hook now | No (used) |
| `last_displayed_image(&self, folder_path: &Path) -> Option<&Path>` | `pub(crate)` | Task 3 target-priority lookup | YES — bare `#[allow(dead_code)]` |

- `is_empty()` exists as `#[cfg(test)]` only (unit-test observation of map emptiness; compiled out of release builds, so no dead-code warning, no marker needed).
- Marker discipline mirrors the Task-1 precedent (`sibling_folder_listing.rs`: bare `#[allow(dead_code)]` attributes, no inline notes). Remove `#[allow(dead_code)]` from `last_displayed_image` in Wave-3 Task 3 4.2, when its first caller lands. Nobody removes it here.
- Tests reach the app's map through the app's private field (`app.last_viewed_images…`) from the app child-module test file — same privacy technique as `src/app/tests.rs` (fields/methods accessed from a `super::…` sibling child module).
- Task 3 gets exactly ONE lookup: `last_displayed_image`. No map mutation API beyond `record_displayed_image`. No Debug/Clone derivations.

## 3. Exact Code Changes

### 3.1 NEW FILE `src/last_viewed_images.rs` (write exactly this content)

```rust
//! Session last-viewed memory: the image last successfully displayed for
//! every folder visited this session. RAM-only -- created empty with the app,
//! updated at the open-success transition, and destroyed with the application
//! struct; v1 has no disk persistence.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) struct LastViewedImages {
    folder_to_image_paths: HashMap<PathBuf, PathBuf>,
}

impl LastViewedImages {
    pub(crate) fn empty() -> Self {
        Self {
            folder_to_image_paths: HashMap::new(),
        }
    }

    /// Records `image_path` as its folder's last displayed image. The key is
    /// the opened path's parent folder (plain `parent()`, never canonicalized
    /// -- Task 3 looks up with the same folder-path derivation).
    pub(crate) fn record_displayed_image(&mut self, image_path: &Path) {
        let folder_path = image_path.parent().unwrap_or_else(|| Path::new(""));
        self.folder_to_image_paths
            .insert(folder_path.to_path_buf(), image_path.to_path_buf());
    }

    /// The session's last displayed image of `folder_path`; `None` before the
    /// folder was visited. Task 3's target-priority lookup calls this.
    #[allow(dead_code)]
    pub(crate) fn last_displayed_image(&self, folder_path: &Path) -> Option<&Path> {
        self.folder_to_image_paths
            .get(folder_path)
            .map(|image_path| image_path.as_path())
    }

    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.folder_to_image_paths.is_empty()
    }
}
```

- ~38 lines. `HashMap::get(folder_path: &Path)` compiles via `impl Borrow<Path> for PathBuf` (std). Method/arg/depth rules: all bodies ≤5 lines, ≤2 args, depth ≤2 (chained expression, not nested blocks), private field, self-documenting naming.

### 3.2 EDIT `src/main.rs` (+1 line, 31 → 32)

Insert alphabetically between `mod image_loading;` (line 6) and `mod name_collation;` (line 7):

```rust
mod last_viewed_images;
```

### 3.3 EDIT `src/app.rs` (192 → 199 lines; five additions, nothing else)

1. Import — insert after line 9 (`use crate::image_loading::{…};`), alphabetically before `use crate::platform;`:
   ```rust
   use crate::last_viewed_images::LastViewedImages;
   ```
2. Field — insert immediately after the `current_folder_listing` field (lines 18-20), as:
   ```rust
   /// Session memory: the last displayed image of every folder visited in-session.
   last_viewed_images: LastViewedImages,
   ```
3. Init — in `empty_state()`, insert after `current_folder_listing: None,`:
   ```rust
   last_viewed_images: LastViewedImages::empty(),
   ```
4. Hook — in `accept_open_outcome`'s `Ok(decoded_image)` arm, insert AFTER the `current_folder_listing` assignment and BEFORE `self.opened_image_path = Some(path);`:
   ```rust
   self.last_viewed_images.record_displayed_image(&path);
   ```
   (The `Err(_failure)` arm keeps ONLY `self.message_text = Some(UNSUPPORTED_FILE_MESSAGE);` — do not touch it.)
5. Test-module declaration — insert at the very end of the file, after the existing `#[cfg(test)] mod floating_controls_tests;` block:
   ```rust
   #[cfg(test)]
   mod last_viewed_tests;
   ```

Line budget check: 192 + 1 (import) + 2 (field+doc) + 1 (init) + 1 (hook) + 2 (test-mod decl) = **199 ≤ 200**. HARD rule: no other line of `app.rs` may change (no behavior change beyond the hook; `NavigationDirection`, `navigate`, `neighbor_image_path`, the UI layers, and both arms of `accept_open_outcome` otherwise untouched).

### 3.4 UNTOUCHED files (explicit)

`src/app/floating_controls.rs`, `src/app/arrow_direction.rs`, `src/image_loading.rs`, `src/app_display.rs`, `src/platform/**`, `src/folder_listing.rs`, `src/sibling_folder_listing.rs`, `src/name_collation.rs`, `src/scaling.rs`, `Cargo.toml`, `Cargo.lock`, all docs/TODO/plan files, `*.sh` harness scripts. The Task-1 module stays read-ready for Task 3 only — nothing imports `sibling_folder_listing` in this task.

## 4. Tests — NEW FILE `src/app/last_viewed_tests.rs` (+2 lines already counted in `app.rs`)

- Placement decision (locked): a dedicated child-module test file, declared `#[cfg(test)] mod last_viewed_tests;` inside `app.rs`, because `src/app/tests.rs` is at 187/200 lines — adding 5 tests there would break the file cap. Fixture reuse unlocks via `use super::tests::{TempImageFolder, sample_decoded_image};` (the same pattern as `src/app/floating_controls_tests.rs` lines 3-6); both fixtures are `pub(super)` in `app::tests` and reachable from a sibling child module of `app`.
- Container environment fact (drives the test shapes): the harness stub decode ALWAYS fails, so real `open_image` calls always land on the failure path here; every "successful open" is simulated with `app.accept_open_outcome(path, Ok(sample_decoded_image()))` — exactly the seam that also carries the hook (therefore the hook is fully exercised). This matches the existing `app/tests.rs`/`floating_controls_tests.rs` style.

Write exactly:

```rust
//! Unit tests for the session last-viewed memory: empty on a fresh app,
//! updated on every successful open, unclobbered by failed opens, and
//! scoped per folder.

use super::tests::{TempImageFolder, sample_decoded_image};
use super::ImagesScrollerApp;
use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;
use std::path::PathBuf;

#[test]
fn fresh_app_starts_with_an_empty_last_viewed_memory() {
    let app = ImagesScrollerApp::empty_state();
    assert!(app.last_viewed_images.is_empty());
}

#[test]
fn successful_open_records_the_folders_last_viewed_image() {
    let folder = TempImageFolder::create("last_viewed_record");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    let image_path = folder.image_path("b.png");
    let folder_path = image_path.parent().expect("fixture path has a parent");
    app.accept_open_outcome(image_path.clone(), Ok(sample_decoded_image()));
    assert_eq!(
        app.last_viewed_images.last_displayed_image(folder_path),
        Some(image_path.as_path())
    );
}

#[test]
fn every_subsequent_open_in_a_folder_updates_the_last_viewed_entry() {
    let folder = TempImageFolder::create("last_viewed_update");
    folder.create_file("a.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    app.accept_open_outcome(folder.image_path("c.png"), Ok(sample_decoded_image()));
    let image_path = folder.image_path("c.png");
    let folder_path = image_path.parent().expect("fixture path has a parent");
    assert_eq!(
        app.last_viewed_images.last_displayed_image(folder_path),
        Some(image_path.as_path())
    );
}

#[test]
fn failed_open_keeps_the_folders_last_viewed_image() {
    let folder = TempImageFolder::create("last_viewed_failure");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.open_image(PathBuf::from("unregistered-in-container.bin"));
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    let image_path = folder.image_path("b.png");
    let folder_path = image_path.parent().expect("fixture path has a parent");
    assert_eq!(
        app.last_viewed_images.last_displayed_image(folder_path),
        Some(image_path.as_path())
    );
}

#[test]
fn each_folder_keeps_its_own_last_viewed_entry() {
    let first_folder = TempImageFolder::create("last_viewed_first");
    first_folder.create_file("a.png");
    let second_folder = TempImageFolder::create("last_viewed_second");
    second_folder.create_file("e.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(first_folder.image_path("a.png"), Ok(sample_decoded_image()));
    app.accept_open_outcome(
        second_folder.image_path("e.png"),
        Ok(sample_decoded_image()),
    );
    let first_image_path = first_folder.image_path("a.png");
    let first_folder_path = first_image_path.parent().expect("fixture path parent");
    let second_image_path = second_folder.image_path("e.png");
    let second_folder_path = second_image_path.parent().expect("fixture path parent");
    assert_eq!(
        app.last_viewed_images.last_displayed_image(first_folder_path),
        Some(first_image_path.as_path())
    );
    assert_eq!(
        app.last_viewed_images.last_displayed_image(second_folder_path),
        Some(second_image_path.as_path())
    );
    app.open_image(second_folder.image_path("unregistered-decoy.png"));
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_eq!(
        app.last_viewed_images.last_displayed_image(first_folder_path),
        Some(first_image_path.as_path())
    );
    assert_eq!(
        app.last_viewed_images.last_displayed_image(second_folder_path),
        Some(second_image_path.as_path())
    );
}
```

Test-name → requirement mapping (names LOCKED; reviewer checks them):

1. `fresh_app_starts_with_an_empty_last_viewed_memory` — per-session lifetime evidence (struct-scope: a fresh app struct's map is empty; hunger for no Drop/persistence).
2. `successful_open_records_the_folders_last_viewed_image` — map updated on successful open (folder→image entry; includes first-open creation).
3. `every_subsequent_open_in_a_folder_updates_the_last_viewed_entry` — entry updated on each subsequent successful open (navigation within a folder replaces the value).
4. `failed_open_keeps_the_folders_last_viewed_image` — a failed open does NOT clobber the last-viewed entry of the folder whose neighbor failed (the previously viewed image stays).
5. `each_folder_keeps_its_own_last_viewed_entry` — folder-scoping (independent entries) + a failed open touching one folder leaves the OTHER folder's entry intact.

Junior trap encoded: the `parent()` lookup borrows a `PathBuf` — tests bind `let image_path = folder.image_path("…");` FIRST and derive `folder_path` from THAT binding; inlining `folder.image_path("…").parent()…` would borrow a dropped temporary (compile error). The snippets above are shaped correctly; copy them as written.

### Census after Task 2

- Expected unit-test census: **36 + 5 = 41** (new: 5 in `src/app/last_viewed_tests.rs`).
- NOTE for the later docs step: `tech.md` → `## Testing` currently says 26 (stale — Wave-3 Task 1 landed 2 `name_collation` + 8 `sibling_folder_listing` tests); Task 5 owns refreshing it (record 41 there after Task 3 anyway).

## 5. Git (for the 4.2 implementer)

- Working branch (already exists, from the wave's Step 2): `feat/wave-3-vertical-scroll`. Work only there; NO branch creation, NO push in Task 2 4.2.
- Before staging: read `.gitignore` and run `git status` (Gitignore Compliance Rule) — stage NOTHING that matches `.gitignore` (`target/`, `dist/`, `logs/`, `*.log`).
- Stage exactly: `src/last_viewed_images.rs`, `src/app/last_viewed_tests.rs`, `src/app.rs`, `src/main.rs`, and this plan file `.kilo/plans/20261101-wave3-task2-last-viewed-memory.md`.
- Commit message (single commit, exact):
  ```
  feat: track per-folder last-viewed images in memory
  ```
- NO other git actions in Task 2 (no merge, no tags, no existing-plan edits).

## 6. 4.2 Junior-Implementer Checklist (write-only — NO cargo/NO VM commands in this step)

Pre-flight:

1. `git status` → confirm clean tree on `feat/wave-3-vertical-scroll` (that branch already carries Wave-3 Task 1).
2. `git log --oneline -3` → confirm HEAD is Wave-3 Task 1's landed state; do NOT re-read/expand other tasks.

Build steps (in this order; use write/edit tools, never shell file-writing):

3. CREATE `src/last_viewed_images.rs` with §3.1 content verbatim (module header is `//!` lines; real newlines, no `\n` literals).
4. EDIT `src/main.rs`: insert the one `mod last_viewed_images;` line per §3.2.
5. EDIT `src/app.rs`: apply the five additions of §3.3, in the listed spots, nothing else.
6. CREATE `src/app/last_viewed_tests.rs` with §4 content verbatim.
7. Self-check (read the files back): app.rs = 199 lines; every Ok/Err arm unchanged except the one hook line; only `last_displayed_image` carries `#[allow(dead_code)]`; no commented-out code anywhere; no file >200 lines.
8. `git add` exactly the 5 files from §5; `git status` re-check (no stray staging, nothing gitignored staged); commit with the exact message from §5.

Verification boundary: dev-checks (`fmt/check/test/clippy` via the container harness per `docs/build.md`, `vm_status` first) are NOT run in Task 2 4.2 — they belong to the later authorized steps (4.3 review / fix / Task 4) per the wave's execution order. If a later check run reports rustfmt drift in the new files, the fix step reformats with `cargo fmt` only (Wave-2 Task 1 precedent); do NOT hand-tune formatting now beyond what §3.1/§4 show.

## 7. 4.3 Reviewer Guidance (code-reviewer scan list)

- Hook placement: EXACTLY ONE `record_displayed_image` call in the whole crate, inside `accept_open_outcome`'s Ok arm, between the listing rebuild and the `opened_image_path` move; the Err arm untouched (a failed open must not write the map). `grep -n "record_displayed_image" src/` → 3 hits expected (definition, hook, none elsewhere); `grep -n "\.insert(" src/last_viewed_images.rs` → 1 hit (the map write is single-sourced).
- No persistence anywhere: `src/last_viewed_images.rs` imports ONLY `std::collections::HashMap` + `std::path::{Path, PathBuf}` — no `std::fs`, no serialization crates, no file/dialog/settings code; `Cargo.toml` unchanged (no new deps); `grep -rn "serde\|write\|canonicalize" src/last_viewed_images.rs` → no hits.
- Key-derivation consistency with Task-3 consumption: keys are plain `parent()` (§2.2 records the contract; reviewer confirms no canonicalization sneaked in and the §2.2 contract text is intact in this plan).
- Rule compliance: file ≤200 lines (app.rs 199; new files ~38/~100), methods ≤50 lines, ≤2 args/method, depth ≤2, private field, bare `#[allow(dead_code)]` on `last_displayed_image` ONLY (also confirm `is_empty` is `#[cfg(test)]` so it needs no marker), no comments beyond `//!`/short `///` doc lines, no commented-out code (No-Commented-Code rule), single-section boolean conditions (none exist), edition-2024 idioms.
- Diff scope: ONLY the files in §5; no UI/keyboard/`NavigationDirection`/`floating_controls`/`sibling_folder_listing` changes; test namespaces and names match §4 exactly; census math 36→41 acknowledged in the review notes (actual green run happens at the later harness step).
- Simplifier: do not merge `LastViewedImages` into `app.rs`, do not change the accessor's `Option<&Path>` return, do not introduce `Clone`/`Debug`/builder patterns; the struct exists to keep `app.rs` under the 200-line cap and to single-source map writes.

## 8. Deliberately Out of Scope (Task 2 — none of these may appear in the diff)

- NO integration with `src/sibling_folder_listing.rs` beyond type-readiness (module untouched, not even imported).
- NO keyboard/UI work: `ArrowUp`/`ArrowDown` intake, ↑/↓ button wiring, `NavigationDirection` variants — all Task 3.
- NO target-priority RESOLUTION (visited→last-viewed vs first-image): Task 3 only calls the `last_displayed_image` lookup; the `Option` → decision mapping is Task 3's.
- NO empty-sibling-folder handling, NO edge-of-list behavior (both are Task-3 4.1b deferred decisions).
- NO docs/TODO/project-structure edits; NO version bump (Step 3 owns it); NO `CHANGELOG.md` (wave rule); NO edits to existing plan files.
- NO persistence of any form, NO settings storage, NO `Drop`/close handling.
- NO new crate dependencies; NO changes to `Cargo.toml`/`Cargo.lock`.
