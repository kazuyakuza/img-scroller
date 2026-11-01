# Task-1 Plan — Wave 3, Task 1: Sibling-Folder Enumeration (Auto-Approved, Per-Task Plan)

- Date: 2026-11-01
- Source TODO (binding): [`.agent/todos/20261029/20261029-todo-5.md`](../../.agent/todos/20261029/20261029-todo-5.md) → `### Task 1`
- Global plan (read-only reference): [`.kilo/plans/20261101-wave3-vertical-scroll.md`](20261101-wave3-vertical-scroll.md)
- 4.1b deliverable of the Critical Workflow — NOT front-end (no 4.1a spec required).
- Scope of THIS task: the enumeration module + its unit tests ONLY. Navigation wiring, memory map, and target-image priority are later tasks (see `## Deliberately Out of Scope`).

## 0. Ground Truth Used

- `src/folder_listing.rs` (125 lines): pure listing precedent — `FolderListing { image_paths: Vec<PathBuf> }`, `from_opened_image(&Path) -> Option<Self>`, private `compare_image_file_names(&str, &str) -> Ordering` (lowercase-fold then exact tie-break), sort via `to_string_lossy`, and `ListedNeighbor` enum (`At { listed_index, image_path }` / `ListBeginningReached` / `ListEndReached`) with `#[allow(dead_code)]` precedents on not-yet-wired items.
- `src/folder_listing/tests.rs` (174 lines): 5 tests, real temp-dir fixtures (`TempFolder` under `std::env::temp_dir()`), container-runnable, NO hidden-file filtering (`.concealed.png` IS listed).
- `src/app.rs`: `current_folder_listing: Option<FolderListing>`; horizontal neighbor flow = `listed_index_of(opened)` → `previous/next_neighbor` → `At` gives the path → `open_image`. NO changes allowed in this task.
- `src/main.rs`: `mod` list `app, app_display, folder_listing, image_loading, platform, scaling`.
- Collation decision to reuse (tech.md → `## Horizontal Navigation Mechanism (Implemented, Wave 2)`): names compare by lowercase form first; names equal ignoring case tie-break by exact characters. Brief `## Sorting Order`: ONE fixed alphanumeric order for BOTH folders and files.
- Build stance (tech.md → Build Environment): NO cargo on the host at any time; all checks run later in the container (Task 4). 4.2 write-only.

## 1. High-Level Approach

A new pure-logic module `sibling_folder_listing` mirrors `folder_listing`'s shape axis-for-axis, applied to DIRECTORIES of the parent scope:

- Input anchor: the opened image path → current folder = `path.parent()`; scope folder = current folder's parent (e.g. from `parent\folder2\img1`: current folder = `folder2`, scope = `parent`). The scope is read ONCE, non-recursively, directories only.
- The internal ordered list contains ONLY scope child directories and EXCLUDES the current folder itself (TODO-Task-1 binding text: enumerate sibling folders, never folders of the parent beyond this one level, never recursive).
- Because the current folder is excluded, the current folder's position anchor is resolved by the SAME collation (count of listed folders whose names fold before the current folder's name — the insertion index), instead of folder_listing's identity match. The `At { listed_index, folder_path }` / edge-report enum stays the exact mirror.
- Collation is extracted ONCE into a shared tiny module `name_collation` and reused by both listings (one fixed order per brief).
- Folder I/O sits behind one injectable reader seam (single agreed approach; no dual options), with a production default that calls `std::fs` — same legality basis as folder_listing's own `std::fs` use.

## 2. New Files (exact names + budgets)

| File | Content | Budget |
| --- | --- | --- |
| `src/name_collation.rs` | Shared entry-name compare fn + 2 inline tests | ≤ 60 lines total |
| `src/sibling_folder_listing.rs` | Pure sibling-folder listing logic | ≤ 120 lines total (well under the 200 hard cap; target ≈ 95) |
| `src/sibling_folder_listing/tests.rs` | 8 unit tests + `TempScope` fixture (child `mod tests` via `#[cfg(test)] mod tests;`) | hard cap 200 lines (target ≈ 190) |

Edited existing files (the ONLY permitted edits):

1. `src/main.rs` — two one-line registrations, keeping the alphabetical order:
   ```rust
   mod app;
   mod app_display;
   mod folder_listing;
   mod image_loading;
   mod name_collation;
   mod platform;
   mod scaling;
   mod sibling_folder_listing;
   ```
2. `src/folder_listing.rs` — collation extraction ONLY:
   - Remove `use std::cmp::Ordering;` (it is used only by the removed function — leaving it would be an unused-import compile error).
   - Add `use crate::name_collation::compare_entry_names;` next to the other `use crate::…` imports.
   - Delete the private `compare_image_file_names` function (lines 116–122).
   - In `collect_folder_images`, the `sort_by` closure calls `compare_entry_names(&left_path.to_string_lossy(), &right_path.to_string_lossy())` instead.
   - NOTHING else changes. `src/folder_listing/tests.rs` is UNTOUCHED (its sort test locks the horizontal collation behavior unchanged through the shared helper).

No other file is touched — `src/app.rs`, `src/app/*`, `src/image_loading.rs`, `Cargo.toml`, docs: all unchanged in this task (`.agent/project-structure.md` is updated later by the Task-5 docs step, not here).

## 3. New File Content — exact decisions and shapes

### 3.1 `src/name_collation.rs`

Self-documenting module header (`//!` doc, mirroring folder_listing/image_loading header style — the only comments allowed):

```rust
//! Fixed v1 alphanumeric entry-name collation shared by the image and folder
//! listings: compare by lowercase form first, tie-break by exact characters
//! (tech.md -> Horizontal Navigation Mechanism, Wave-2 decision; brief
//! `## Sorting Order`: one fixed order for files AND folders).
```

Single function (public surface of this module, `pub(crate)`, 2 params):

```rust
pub(crate) fn compare_entry_names(left_name: &str, right_name: &str) -> Ordering {
    let case_folded_order = left_name.to_lowercase().cmp(&right_name.to_lowercase());
    if case_folded_order != Ordering::Equal {
        return case_folded_order;
    }
    left_name.cmp(right_name)
}
```

Body is verbatim the current `compare_image_file_names` body (renamed only) — behavior locked. Inline `#[cfg(test)] mod tests` with exactly 2 tests (names LOCKED):

1. `lowercase_form_decides_order_before_exact_characters` — `compare_entry_names("alpha.gif", "ALPHA.jpg")` is `Ordering::Less` (the full names fold to `"alpha.gif"` vs `"alpha.jpg"`; the folded FULL name — including the extension — decides, so `g` < `j`); `compare_entry_names("alpha.gif", "zeta.jpg")` is `Ordering::Less`.
2. `exact_characters_tie_break_when_lowercase_forms_match` — `compare_entry_names("Alpha.jpg", "alpha.jpg")` is `Ordering::Less` (`A` 0x41 < `a` 0x61); `compare_entry_names("ALPHA", "Alpha")` is `Ordering::Less` (`L` < `l`). Import `std::cmp::Ordering` in the tests module.

### 3.2 `src/sibling_folder_listing.rs`

Module header (`//!` doc): state the contract in two sentences — the DIRECTORY entries of the parent of the opened image's folder only, one non-recursive read, self-excluded from destinations, sorted by the fixed v1 collation shared with the image listing.

Private I/O seam (the ONE agreed approach — see `## 5` for rationale):

```rust
type ScopeEntriesReader = dyn Fn(&Path) -> Option<Vec<ScopeEntry>>;
```

Private entry payload:

```rust
struct ScopeEntry {
    path: PathBuf,
    is_directory: bool,
}
```

Public surface (EVERY public item below — minimal surface, private-by-default; `#[allow(dead_code)]` mirrors folder_listing's precedent for items not yet wired by `app.rs`; Task 3 removes the attributes as it wires them):

```rust
#[allow(dead_code)]
pub(crate) enum ListedFolderNeighbor {
    At {
        #[allow(dead_code)]
        listed_index: usize,
        folder_path: PathBuf,
    },
    ListBeginningReached,
    ListEndReached,
}
```

(`#[derive(Debug, PartialEq)]` on the enum — required for the tests' `assert_eq!`, mirroring `ListedNeighbor`.)

```rust
pub(crate) struct SiblingFolderListing {
    sibling_folder_paths: Vec<PathBuf>,
}

impl SiblingFolderListing {
    #[allow(dead_code)]
    pub(crate) fn from_opened_image(opened_image_path: &Path) -> Option<Self>

    #[allow(dead_code)]
    pub(crate) fn listed_folder_count(&self) -> usize

    #[allow(dead_code)]
    pub(crate) fn previous_neighbor_of(&self, current_folder_path: &Path) -> Option<ListedFolderNeighbor>

    #[allow(dead_code)]
    pub(crate) fn next_neighbor_of(&self, current_folder_path: &Path) -> Option<ListedFolderNeighbor>
}
```

Every other item is PRIVATE:

- `fn from_opened_image_with_scope_entries(opened_image_path: &Path, read_scope_entries: &ScopeEntriesReader) -> Option<Self>` (2 args)
  Body (depth ≤ 1, all linear `?` propagation):
  1. `let current_folder_path = opened_image_path.parent()?;` (`parent\folder2\img1` → `folder2`; a bare `img.png` yields an empty parent whose canonicalization fails → `None`).
  2. `let resolved_current_folder = fs::canonicalize(current_folder_path).ok()?;` — the current folder must exist (mirrors folder_listing's canonicalize; missing/unreadable current folder → `None`).
  3. `let scope_parent_path = resolved_current_folder.parent()?;` — the scope = current folder's parent (the image's grandparent; a root-level current folder yields `None`).
  4. `let scope_entries = read_scope_entries(scope_parent_path).unwrap_or_default();` — **missing or unreadable scope folder ⇒ EMPTY listing, not `None`** (the decided behavior; see `## 4`). Only the current-folder level can produce `None`.
  5. `Some(Self::sorted_scope_folders(scope_entries, &resolved_current_folder))`.

- `fn sorted_scope_folders(scope_entries: Vec<ScopeEntry>, resolved_current_folder: &Path) -> Self` (2 args):
  1. `filter(|entry| entry.is_directory)` — directories only.
  2. `map(|entry| entry.path)`
  3. `filter(|folder_path| folder_path != resolved_current_folder)` — exact `PathBuf` equality excludes self (the scope is canonical, so entry paths share the current folder's canonical form; read_dir never yields `.`/`..`, and no explicit `..` handling exists anywhere).
  4. `sort_by(|left_path, right_path| compare_entry_names(&left_path.to_string_lossy(), &right_path.to_string_lossy()))` — the shared collation, same call shape as folder_listing.
  5. store in `sibling_folder_paths`.

- `fn position_of_current_folder(&self, current_folder_path: &Path) -> Option<usize>`:
  canonicalize the anchor; take `resolved.file_name()?`; position = `self.sibling_folder_paths.iter().take_while(|listed_path| preceding_folder_name(listed_path, current_name)).count()` (linear scan over the tiny list; entries are unique on disk, and because the anchor folder itself is excluded, `position` is exactly the insertion index into the sorted list — `paths[position]` is the "next" sibling, `paths[position - 1]` the "previous"). `None` only if the anchor cannot be canonicalized (deleted current folder) — this is why the neighbor methods return `Option<_>` (deviation from folder_listing's `assert!` style, justified: the anchor may legitimately vanish between listing build and navigation; `None` maps to "no neighbor" in Task 3 exactly like the horizontal `else { return None; }` flow).

- `fn preceding_folder_name(listed_path: &Path, current_name: &str) -> bool` — `compare_entry_names(&listed_path.file_name().unwrap_or_default().to_string_lossy(), current_name) == Ordering::Less` (single-section condition; `unwrap_or_default` is unreachable for read_dir-born paths). Split into its own named method per the single-section-boolean rule.

- `fn neighbor_at(&self, listed_index: usize) -> ListedFolderNeighbor` — builds `ListedFolderNeighbor::At { listed_index, folder_path: self.sibling_folder_paths[listed_index].clone() }` (caller guarantees bounds; see neighbor methods).

- `fn listed_folder_count(&self) -> usize` → `self.sibling_folder_paths.len()`.

- `pub(crate) fn previous_neighbor_of(&self, current_folder_path: &Path) -> Option<ListedFolderNeighbor>`:
  `let position = self.position_of_current_folder(current_folder_path)?;`
  `position == 0 → Some(ListedFolderNeighbor::ListBeginningReached)`;
  else `Some(self.neighbor_at(position - 1))`. (position ≤ len ⇒ bounds safe; no wrap, edges stay — folder_listing mirror.)

- `pub(crate) fn next_neighbor_of(&self, current_folder_path: &Path) -> Option<ListedFolderNeighbor>`:
  `let position = self.position_of_current_folder(current_folder_path)?;`
  `position == self.sibling_folder_paths.len() → Some(ListedFolderNeighbor::ListEndReached)`;
  else `Some(self.neighbor_at(position))`. For an empty listing position is always 0 = len ⇒ both directions resolve to the edge variants, never a path.

- `fn read_scope_entries(folder_path: &Path) -> Option<Vec<ScopeEntry>>` — the PRODUCTION reader passed to `from_opened_image_with_scope_entries` by the public constructor:
  ```rust
  let dir_entries = fs::read_dir(folder_path).ok()?;   // Err → None (mapped to empty listing upstream)
  Some(dir_entries
      .filter_map(|dir_entry| scope_entry_of(dir_entry.ok()?))
      .collect())
  ```
- `fn scope_entry_of(dir_entry: fs::DirEntry) -> Option<ScopeEntry>`:
  `let entry_type = dir_entry.file_type().ok()?;` — an entry whose file-type probe fails is silently skipped, mirroring folder_listing's `dir_entry.ok()?` / `file_type().ok()?` drop behavior. `is_directory = entry_type.is_dir()`. Path = `dir_entry.path()`.
  Attribute-policy notes encoded AS CODE SHAPE: the ONLY attribute probe is `is_dir()`; symbolic links and junctions are reparse/symlink entries per `read_dir`'s `FileType` (`is_symlink()` true, `is_dir()` false) and are therefore NOT listed as folders; hidden-attribute directories are INCLUDED (no hidden filtering — the same stance folder_listing applies to files, lock-tested by its existing `.concealed.png` assertion). NO junction/symlink re-probing, no `follow_link`, no canonicalization of entries.

Naming/shape rules re-checked: every fn ≤ 2 params incl. `self`? — associated fns with `&self` + 1 param = 2 total; `from_opened_image_with_scope_entries` = 2; `sorted_scope_folders` = 2. All bodies ≤ 12 lines. Max nesting depth: 1–2. No comments besides the `//!` module headers. No commented-out code. Edition 2024 (workspace-level, nothing to change).

### 3.3 Expected test fixture + test list (`src/sibling_folder_listing/tests.rs`)

Fixture `TempScope` (mirrors `TempFolder` of folder_listing's tests; real fs under `std::env::temp_dir()` so `fs::canonicalize` works, with prefix `img_scroller_sibling_scope_{test_case}_{pid}`):

- `fn create(test_case: &str) -> Self` — creates the scope dir, the current folder `mid` inside it, and a marker file `mid/shot.png` inside the current folder (marker not required by the code, kept for realism).
- `fn current_image_path(&self, file_name: &str) -> PathBuf` — `self.mid_folder_path.join(file_name)` (anchor input).
- `fn scope_child_path(&self, name: &str) -> PathBuf` — `self.scope_path.join(name)`.
- `fn create_scope_child_folder(&self, name: &str)` (skips write), `fn create_scope_child_file(&self, name: &str)` (`fs::write(b"x")`).
- `fn synthetic_entry(&self, name: &str, is_directory: bool) -> ScopeEntry` — struct literal `{ path: self.scope_child_path(name), is_directory }` (child module CAN build the private struct).
- `fn reader_entries(&self, names: &[(&str, bool)]) -> …`-style closure builder: `|path| path-based match…` — SIMPLEST agreed shape: each test that needs a SYNTHETIC reader defines its own local closure `|_: &Path| Some(vec![…])` or `|_: &Path| None` via `TempScope::synthetic_entry`; no shared closure builder is required.
- `impl Drop for TempScope` → `let _ = fs::remove_dir_all(&self.scope_path);`.

Tests use the injected entry via: `SiblingFolderListing::from_opened_image(scope.current_image_path("shot.png").as_path())` — the real reader is exercised by the REAL-directory tests because `from_opened_image` internally always calls `read_scope_entries`; synthetic readers exercise the seam directly in fault-injection tests. Verify no Windows-only APIs; the one platform-specific test uses `std::os::unix::fs::symlink` inside `#[cfg(test)]` (safe: tests run in the Linux container only; the windows-gnu `cargo check` compiles without test cfg).

The 8 tests (names LOCKED — assertion targets in parentheses):

1. `sibling_folders_sort_by_lowercase_form_then_exact_name_tie_break`
   Real reader. Scope child FOLDERS created: `ALPHA`, `Alpha`, `alpha`, `Bombay`, `ZULU` (current folder `mid`). Assert `listed_folder_count() == 5` and (folder_listing-mirror assertion) the direct field order:
   `assert_eq!(listing.sibling_folder_paths, vec![scope/ALPHA, scope/Alpha, scope/alpha, scope/Bombay, scope/ZULU])` — proves fold (`alpha` group before `bombay` < `zulu`) AND exact-tie-break ordering (ALPHA < Alpha < alpha).
2. `listing_enumerates_only_scope_child_directories`
   Real reader. Scope: folder `folder1`, folder `.concealed_dir`, files `stray.txt` + `.concealed_file.png`, current folder `mid`. Assert count == 2 and the direct field contents `[.concealed_dir, folder1]` — files excluded, hidden folders INCLUDED (no hidden-filter policy lock).
3. `current_folder_is_excluded_from_the_enumerated_siblings`
   Synthetic reader: `Some(vec![folder entry "first", folder entry "mid", folder entry "second"])` made via `TempScope::synthetic_entry` (the current folder `mid` is among the injected entries); assert count == 2 and field == `[first, second]`.
4. `listing_covers_only_immediate_scope_child_folders`
   Real reader. Scope: folder `folder_a` containing a subfolder `inner` and a file `leaf.txt` inside it; folder `folder_b`; folder `folder_c` containing a file `pic.png`; current folder `mid`. Assert field == `[folder_a, folder_b, folder_c]` — the grandchild `folder_a/inner`, the files, and everything under them NEVER appear (non-recursive proof; one read of the scope only; nothing grandparent-level can appear because the reader is never called above the scope).
5. `folder_neighbors_step_by_one_and_report_scope_edges`
   Synthetic reader `Some(vec![alpha, beta, gamma])` entries; real dirs `alpha`, `beta`, `gamma` exist (canonicalize). Build one listing per anchor via `from_opened_image(scope_child_path(X).join("img.png"))`:
   - anchor `beta`: previous == `Some(At { listed_index: 0, folder_path: alpha })`, next == `Some(At { listed_index: 1, folder_path: gamma })`
   - anchor `alpha`: previous == `Some(ListBeginningReached)`, next == `Some(At { listed_index: 0, folder_path: beta })`
   - anchor `gamma`: previous == `Some(At { listed_index: 1, folder_path: beta })`, next == `Some(ListEndReached)`
   (locks ±1 stepping, `listed_index` values, edge reports, and empty-navigation exclusion of self at all positions).
6. `scope_without_siblings_yields_empty_listing`
   Real reader; scope contains ONLY files (`stray.txt`) plus the current folder. Assert `listed_folder_count() == 0` (a Some-listing — assert via `listed_folder_count`, AVOID asserting `.is_some()`; `Option` unwrap is unnecessary since count is 0 on the real path anyway).
7. `missing_or_unreadable_scope_folder_yields_empty_listing_and_none_when_unanchorable`
   Synthetic reader `|_: &Path| None` over an existing scope dir → count 0 (unreadable ⇒ EMPTY, the decided behavior). AND `SiblingFolderListing::from_opened_image(Path::new("single_name.png")).is_none()` (parentless image path). AND `from_opened_image(scope/missing_dir/img.png).is_none()` (current folder's canonicalization fails ⇒ None; distinguish from the scope-missing-on-read case).
8. `symlinked_scope_entries_are_not_listed_as_directories`
   Real reader + `std::os::unix::fs::symlink(target_dir, scope/linked_dir)` where `target_dir` is a real dir OUTSIDE scope (create it beside the scope, e.g. `scope_parent` temp sibling, also removed in `Drop`). Assert the field contains the real folder `folder1` but NOT the symlink path (locks the is_dir-only policy; junctions on Windows behave per the same `read_dir::FileType` semantics — no Windows-side test needed/applicable).

If the test file would exceed 200 lines: STOP and return a question to the caller (never violate the cap silently).

## 4. Enumeration Semantics — decided behaviors (binding for implementer AND reviewers)

- Scope: ONE non-recursive `read_dir` of the current folder's parent (the scope). NEVER the grandparent level, never deeper. Enforced structurally (exactly one reader call on exactly the scope path).
- Entries: directories ONLY (`is_directory` flag on the seam; files reported too but filtered by the pure logic — lock tests 2/3/5).
- Self-exclusion: the current folder is NEVER among the enumerated siblings (PathBuf equality against the canonicalized current folder; lock tests 3/5). Consequence honestly stated for reviewers: this is the agreed DELIBERATE deviation from folder_listing's identity-match convention — folder_listing lists the opened image itself because the LISTED items are images (the opened one included); here the enumerated SET is "folders other than the one you are in", hence position-by-collation instead of identity. The enum, naming, edge semantics, and `At` shape still mirror folder_listing.
- Missing/unreadable SCOPE folder ⇒ **EMPTY listing** (Some, zero folders) — both directions then resolve to the edge variants → Task 3 navigation does nothing. Decided per the assigned step's parenthetical guidance ("empty list"). A MISSING/UNREADABLE CURRENT FOLDER or a parentless image path ⇒ `None` (cannot anchor; cannot even resolve the scope) — also feeds "no neighbor" downstream.
- Empty-but-readable scope (no dirs at all) ⇒ empty listing (Some, zero folders) — same no-neighbor outcome.
- Hidden directories: included, no attribute filtering (symmetric with folder_listing's file stance; `.concealed_dir` lock test).
- Symlinks/junctions: NOT listed as directories (only `is_dir()` is probed; reparse/symlink entries are excluded). No further attribute probing anywhere.
- `.`/`..`: read_dir never yields them; NO explicit handling exists (no special-casing, no trailing-`..` possibility by construction).
- Case-fold twins of the current folder (possible only on case-sensitive filesystems, e.g. Linux `ALPHA` + `alpha`): both would be listed and reachable; Windows (the sole target, case-insensitive NTFS) cannot create such twins. Deliberately unhandled; no code for it.
- Sort: the EXACT Wave-2 collation through the shared helper — one fixed order for both axes per brief `## Sorting Order`.

## 5. Test-Seam Decision (single agreed approach — ARCHITECTOR decision, no dual options)

PICKED: an injectable reader FUNCTION-TYPE seam + production default, exactly as § 3.2:

- Type: `type ScopeEntriesReader = dyn Fn(&Path) -> Option<Vec<ScopeEntry>>;` (private; used as `&ScopeEntriesReader` param in the private constructor).
- Production default: the private fn `read_scope_entries` (plain `std::fs` — same legality as folder_listing; the platform facade `CONFIG`/`platform` stays untouched).
- Why a seam and not folder_listing's pure real-fs testing: (1) the reader CAN'T-READ case must be lock-tested without platform-specific permission tricks (chmod-000 is unreliable as container-root; Windows ACL probing is impossible in the container) — the `|_: &Path| None` closure locks "unreadable ⇒ empty listing" portably; (2) the seam gives Task 3 a lifetime where navigation edge cases are testable without building directory forests; (3) the real-fs path is STILL covered because the real-directory tests run through `from_opened_image`, which always invokes the production reader.
- Alternatives consciously rejected: generic parameter `<F: Fn…>` (extra generic noise for no benefit), trait + struct injection (heavier than a fn alias for a 1-method seam), and pure real-fs with a platform-conditional unreadable-dir test (non-portable — violates container-only verification).

## 6. Verification Notes for the Container (informational for later tasks — NO builds in 4.2)

- Expected census after this task: 26 (current) + 2 (`name_collation`) + 8 (sibling) = **36 unit tests**, all container-runnable; OS-bound code remains untested by design.
- fmt/clippy conformity is DESIGNED-IN by this plan (the given shapes match the exact formatting conventions of the landed files; `unwrap_or_default`, iterator chains, and `?` propagation are clippy-clean at `-D warnings`).
- Contingency (encode for 4.3): if a later container run flags any private helper (e.g. `position_of_current_folder`) as dead — only possible while the neighbor methods are `#[allow(dead_code)]` stubs — the fix is the SAME per-item `#[allow(dead_code)]` marker with the Task-3 removal note, never a code restructuring.

## 7. Deliberately Out of Scope (Task 1 boundaries — reviewers enforce this list)

- NO keyboard/UI wiring: no `ArrowUp`/`ArrowDown` intake, no pill-button changes, no app.rs changes of ANY kind (the `↑`/`↓` pill buttons stay disabled Wave-3 reserves).
- NO session last-viewed memory map (Task 2).
- NO target-image priority rule and NO first-image-of-target-folder resolution (Task 3). Hook note for Task 3: target folder's first image CAN reuse `FolderListing::from_opened_image(target_folder.join("placeholder.jpg"))` (parent of a path inside the target folder = the target folder; first listed index is 0) — an existing seam, no folder_listing change needed; this is recorded for the wave, NOT implemented now.
- NO enablement logic (per-frame `is_some()` of folder-neighbor resolution is Task 3's intake concern).
- NO version/, TODO-marking, README/docs/project-structure edits (Task 5 / Step work owns those).
- NO removal of the `#[allow(dead_code)]` markers (Task 3 removes them as it wires the app).

## 8. Junior-Implementer Step-by-Step Checklist (4.2 — write-only, NO cargo, NO VM commands)

1. Verify context: `git status` (clean), `git branch --show-current` == `feat/wave-3-vertical-scroll`, `git log --oneline -3` shows the 0.3.0 version bump on top. If ANY check fails → stop and return a question (do not attempt recovery inside 4.2).
2. Re-read THIS plan + the two preceding files listed in `## 0` (fresh pressure check of names before writing).
3. Write `src/name_collation.rs` exactly per § 3.1 (fn verbatim-current-body, renamed; 2 locked tests).
4. Edit `src/folder_listing.rs` per § 2-edit-2 (remove `Ordering` import + `compare_image_file_names`; add the `use crate::name_collation::...`; swap the sort call). Do not touch anything else in the file; `src/folder_listing/tests.rs` must remain byte-identical.
5. Write `src/sibling_folder_listing.rs` per § 3.2 (order within the file: module doc → uses → type alias → `ScopeEntry` → `ListedFolderNeighbor` → `SiblingFolderListing` + impl (public methods first, private after) → `mod tests;` stub at the bottom) — write reader/entry helpers AFTER the impl so the doc-comment orders mirror folder_listing's layout.
6. Create `src/sibling_folder_listing/tests.rs` per § 3.3 (fixture first, then the 8 locked tests in the listed order).
7. Edit `src/main.rs` per § 2-edit-1 exactly (two one-line additions, alphabetical position maintained).
8. Self-review scan (no build): (a) every rule in the file-header block below holds by inspection; (b) the 8 test names match § 3.3 char-for-char; (c) no `mod` reorder or stray additions in main.rs; (d) `git diff --stat` shows exactly the 5 source files + this plan file.
9. Commit (gitignore compliance: `git status` must show no `target/`/`dist/`/`logs/` entries — none can exist because NO build ran):
   - Stage ONLY: `src/name_collation.rs src/sibling_folder_listing.rs src/sibling_folder_listing/tests.rs src/main.rs src/folder_listing.rs .kilo/plans/20261101-wave3-task1-sibling-folder-enumeration.md`
   - Single commit: `feat: add sibling-folder enumeration module`
   - No push (push is Step-5-restricted, `main` only).
10. Report: files created/edited, line counts per file, test census expectation (36), plan path, commit hash.

File-header rule block to re-check at step 8 (all from `.kilo/rules/`): files ≤ 200 lines; methods ≤ 50 lines; ≤ 2 args per method; nesting ≤ 2; private-by-default; self-documenting names; comments ONLY as `//!` module headers; no commented-out code; single-section boolean conditions (extract to a named method — done via `preceding_folder_name`); edition 2024 idioms (no legacy constructs).

## 9. Pre-Flight Checklist (re-read BETWEEN steps 3→4→5→6→7→8)

- [ ] Still on `feat/wave-3-vertical-scroll`? Working tree clean besides my own in-progress files?
- [ ] Did I run ANY cargo/VM command this step? (If yes — STOP, report; that violates 4.2 scope. There is no legit case in this task.)
- [ ] Names check: `compare_entry_names` / `name_collation` / `ScopeEntry` / `ScopeEntriesReader` / `SiblingFolderListing` / `ListedFolderNeighbor` / `from_opened_image` / `listed_folder_count` / `previous_neighbor_of` / `next_neighbor_of` — used EXACTLY so everywhere?
- [ ] `#[allow(dead_code)]` present on: `ListedFolderNeighbor` (enum), `from_opened_image`, `listed_folder_count`, `previous_neighbor_of`, `next_neighbor_of` — and ONLY those (+ the nested `listed_index` field)?
- [ ] folder_listing.rs diff contains ONLY the collation extraction (import removal + add, fn deletion, call-site swap), with tests.rs untouched?
- [ ] app.rs diff is EMPTY for this task?
- [ ] The 8 test files' assertions target the § 3.3 list — no extra tests, no deleted tests, no renamed tests?
- [ ] `unwrap_or_default()` correctly maps unreadable scope → EMPTY (not `None`), while current-folder-level failures still propagate `None`?
- [ ] Git staged set matches § 8.9 exactly (no `.agent/`, no docs, no Cargo files, no lockfile, no `dist/`)?

## 10. Review-Phase Guidance (4.3 preview, non-binding for this step)

- Verify: collation single-sourced (glob for `to_lowercase().cmp` — must exist ONLY in `src/name_collation.rs`); folder_listing behavior locked (its 5 tests unchanged); all § 7 boundaries respected; census match in the eventual container run (36) belongs to Task 4, not to this task's acceptance.
