# Task-1 Code-Review Fix Plan — Wave 3, Task 1: Sibling-Folder Enumeration

- Reviewer: code-reviewer
- Commit reviewed: `bc7261a` on `feat/wave-3-vertical-scroll`
- Source plan: `.kilo/plans/20261101-wave3-task1-sibling-folder-enumeration.md`

## Summary

One correctness issue found: `SiblingFolderListing::position_of_current_folder` canonicalizes the value passed to `previous_neighbor_of` / `next_neighbor_of` directly and extracts its `file_name()`. The plan's own tests (and the natural Task-3 caller) pass the **opened image path** to those methods, so the current code either:

- fails to canonicalize a non-existent image file (test 5 anchor `scope/beta/img.png` is never created), returning `None` and failing the assertions; or
- if the image file happened to exist, would compare sibling folder names against the **image file name** instead of the current folder name, producing a wrong insertion index.

The rest of the implementation matches the plan: file set, public/private surface, `#[allow(dead_code)]` placement, naming, folder-listing extraction, main.rs mod additions, byte-identical `folder_listing/tests.rs`, line-count budgets, and rule compliance all check out.

## Findings

### Issue 1: `position_of_current_folder` treats the anchor as a folder path instead of deriving the folder from the opened image path

- **File:** `src/sibling_folder_listing.rs`
- **Lines:** 83–92
- **What is wrong:**
  - The method canonicalizes `current_folder_path` and reads `resolved_folder.file_name()`.
  - Callers pass the opened image path (see plan § 3.3 test 5: `previous_neighbor_of(beta_anchor.as_path())` where `beta_anchor = scope/.../beta/img.png`).
  - The image file is never created in that test, so `fs::canonicalize` returns `None`, the neighbor methods return `None`, and the `assert_eq!` expecting `Some(At { ... })` / `Some(List*Reached)` fails.
  - Even if the file existed, the comparison would use `"img.png"` instead of `"beta"`, giving an incorrect insertion index for any image name that does not happen to sort exactly at the folder's position.
- **Plan/TODO violation:**
  - Plan § 3.2 `position_of_current_folder` body shape ("canonicalize the anchor; take `resolved.file_name()`") is inconsistent with the plan's own caller contract in § 3.3 test 5 and with the intended Task-3 usage passing the opened image path.
  - Plan § 4 semantics: "position-by-collation" must use the **current folder's name**, not the anchor file's name.
- **Exact minimal fix (junior-executable, no architectural changes):**
  - Derive the current folder from the anchor path exactly as `from_opened_image_with_scope_entries` does: call `.parent()?` first, then canonicalize the folder.
  - Rename the private-method parameter to `anchor_image_path` to make the contract self-documenting (private helper, minor local rename).

```rust
// src/sibling_folder_listing.rs
fn position_of_current_folder(&self, anchor_image_path: &Path) -> Option<usize> {
    let current_folder_path = anchor_image_path.parent()?;
    let resolved_folder = fs::canonicalize(current_folder_path).ok()?;
    let current_name = resolved_folder.file_name()?.to_string_lossy();
    let preceding_folder_count = self
        .sibling_folder_paths
        .iter()
        .take_while(|listed_path| preceding_folder_name(listed_path.as_path(), &current_name))
        .count();
    Some(preceding_folder_count)
}
```

- **Verification after fix:**
  - Test 5 anchors (`scope/.../alpha/img.png`, etc.) resolve to folders `alpha`, `beta`, `gamma` regardless of whether `img.png` exists.
  - `beta` anchor: position 1 → previous `alpha@0`, next `gamma@1`.
  - `alpha` anchor: position 0 → previous `ListBeginningReached`, next `beta@0`.
  - `gamma` anchor: position 2 → previous `beta@1`, next `ListEndReached`.

## Non-Issues / Evidence

- **Plan adherence:** Exact file set (`src/name_collation.rs`, `src/sibling_folder_listing.rs`, `src/sibling_folder_listing/tests.rs`), exact public items/names (`compare_entry_names`, `ScopeEntry`, `ScopeEntriesReader`, `SiblingFolderListing`, `ListedFolderNeighbor`, `from_opened_image`, `listed_folder_count`, `previous_neighbor_of`, `next_neighbor_of`), and `#[allow(dead_code)]` placement match plan § 3.2.
- **folder_listing.rs:** Diff contains only the collation extraction (import swap, `compare_image_file_names` deletion, call-site rename); `src/folder_listing/tests.rs` is byte-identical (`git diff 89e1db4..bc7261a -- src/folder_listing/tests.rs` empty).
- **main.rs:** Exactly two alphabetical `mod` additions; no other changes.
- **app.rs / Cargo.toml / docs / .agent:** Empty diff.
- **Line counts:** `name_collation.rs` 33 lines (budget 60), `sibling_folder_listing.rs` 120 lines (budget 200), `sibling_folder_listing/tests.rs` 197 lines (budget 200).
- **Rule compliance:** All methods ≤ 50 lines, ≤ 2 args, nesting ≤ 2, private-by-default, only `//!` headers, no commented-out code, single-section boolean conditions.

## Files Requiring Change

- `src/sibling_folder_listing.rs` (lines 83–92 only)
