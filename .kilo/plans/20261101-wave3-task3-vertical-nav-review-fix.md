# Task 3 Code Review Fix Plan

Commit under review: `5d503a2` (`feat: navigate sibling folders with up/down arrows and buttons`) on `feat/wave-3-vertical-scroll`; diff range `36be11b..5d503a2`.

## Adjudication of implementer-reported deviations

- **A) `mod folder_navigation;` addition** — accepted with rationale. The declaration is required for compilation (the child module must be declared) and is present at `src/app.rs:182`. The placement groups it with the other non-test child modules. It is not strictly alphabetical (`folder_navigation` precedes `empty_folder_view`), but the ordering is functionally harmless and follows the plan's insertion-point intent.
- **B) `app.rs` field doc lines dropped** — accepted with rationale. The two new field doc lines were dropped per the §3.2 contingency. No other doc lines were changed or removed; all 15 ledger items are otherwise present. Final `app.rs` line count is 199, within the 200-line cap.
- **C) `navigate` restructured with `matches!` boolean guard** — accepted with rationale. The `is_folder_axis` flag is computed from `direction` before the `let-else` and produces the same dispatch as the sketch's post-move `match`: horizontal → `open_image`, vertical → `enter_sibling_folder`. No `Copy` derive was added.
- **D) Marker-removal line-count changes** — accepted with rationale. In `src/sibling_folder_listing.rs` (129 lines) and `src/last_viewed_images.rs` (39 lines) only `#[allow(dead_code)]` attribute lines were removed; nothing else changed.
- **E) `app_opened_in(&PathBuf)`** — **fix needed**. The `&PathBuf` parameter will trigger `clippy::ptr_arg` when Task 4 runs with `-D warnings`. See Fix 1 below.
- **F) Staged set = 12 paths** — accepted with rationale. `git diff --name-only 36be11b 5d503a2` reports exactly the 12 paths: the two plan files plus ten source files including both test modules.

## Fixes required

### Fix 1 — `app_opened_in` parameter type (`clippy::ptr_arg`)

- **File:** `src/app/folder_navigation_tests.rs`
- **Lines:** 6, 48–50
- **What:** The shared helper is declared `pub(super) fn app_opened_in(scope_image_path: &PathBuf)`. Clippy's `ptr_arg` lint fires on `&PathBuf` and will break the Task-4 build under `-D warnings`.
- **Exact minimal junior-executable fix:**
  1. Add `use std::path::Path;` next to the existing `PathBuf` import at line 6.
  2. Change the signature to `pub(super) fn app_opened_in(scope_image_path: &Path)`.
  3. Change the body call from `scope_image_path.clone()` to `scope_image_path.to_path_buf()`.
- **Call sites:** All existing call sites (`app_opened_in(&scope.child_image_path(...))`) continue to compile because `&PathBuf` derefs to `&Path`.
- **Source of violation:** Plan §3.9 specified the literal `&PathBuf` signature, which conflicts with the project's clippy rules.

### Fix 2 — `vertical_navigation_stays_silent_at_the_sibling_list_edges` body exceeds 50 lines

- **File:** `src/app/folder_navigation_continuation_tests.rs`
- **Lines:** 28–85
- **What:** The test function body is ~56 lines, violating `.kilo/rules/max-lines-per-method.md` (method body ≤ 50 lines).
- **Exact minimal junior-executable fix:** Extract the image-mode edge scenario into a module-private helper, reducing the named test body to ≤50 lines while preserving the locked test name and all assertions.
- **Source of violation:** `.kilo/rules/max-lines-per-method.md`.

#### Suggested extraction

Add immediately above the existing `#[test]`:

```rust
fn assert_image_mode_sibling_edges_are_silent() {
    let scope = TempParentScope::create("sibling_edges");
    scope.create_child_folder("folder1");
    scope.create_child_folder("folder2");
    scope.create_image("folder1", "a.png");
    scope.create_image("folder2", "b.png");
    let mut app = app_opened_in(&scope.child_image_path("folder1", "a.png"));
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::PreviousSiblingFolder),
        None
    );
    app.navigate(NavigationDirection::PreviousSiblingFolder);
    assert_eq!(
        app.opened_image_path,
        Some(scope.child_image_path("folder1", "a.png"))
    );
    assert!(app.message_text.is_none());
    app.accept_open_outcome(
        scope.child_image_path("folder2", "b.png"),
        Ok(sample_decoded_image()),
    );
    assert_eq!(
        app.neighbor_image_path(NavigationDirection::NextSiblingFolder),
        None
    );
    assert!(app.message_text.is_none());
}
```

Then replace the first ~26 lines of `vertical_navigation_stays_silent_at_the_sibling_list_edges` with a single call:

```rust
#[test]
fn vertical_navigation_stays_silent_at_the_sibling_list_edges() {
    assert_image_mode_sibling_edges_are_silent();
    let empty_scope = TempParentScope::create("sibling_edges_empty_mode");
    // ... remainder unchanged
}
```

## Evidence of core correctness checks passed

- **D5:** `open_image` canonicalizes at `src/app.rs:57` and feeds the canonical path into `accept_open_outcome`. The Ok arm rebuilds `current_sibling_folders` (`src/app.rs:71–72`), clears `browsed_folder_context` (`src/app.rs:73`), and only then commits `opened_image_path = Some(path)` (`src/app.rs:74`). The Task-2 hook line (`src/app.rs:70`) is byte-identical. The Err arm remains the single line at `src/app.rs:80`. `browsed_folder_context` is touched at exactly three sites: `empty_state` init (`src/app.rs:47`), Ok-arm clear (`src/app.rs:73`), and `enter_empty_folder_mode` set (`src/app/folder_navigation.rs:105`).
- **D2-final:** `navigate` dispatches vertical movement to `enter_sibling_folder`, which applies the priority chain `sibling_folder_target_image_path` (last-viewed then first listed image) and either opens the target or enters empty-folder mode. No skip/wrap logic exists (verified by grep: only doc comments contain "skip"/"wrap"). Empty-folder mode clears `opened_image_path`, `decoded_image`, `message_text`, and `current_folder_listing`, sets `browsed_folder_context`, and re-anchors the sibling list with `folder.join(LISTING_ANCHOR_FILE_PLACEHOLDER)` passed verbatim to `SiblingFolderListing::from_opened_image`.
- **Vertical seam:** `neighbor_image_path` returns a folder path for vertical variants. `vertical_anchor_path` uses the opened image verbatim or `browsed_folder.join(LISTING_ANCHOR_FILE_PLACEHOLDER)`. List-edge variants decode to `None`. `navigation_context_is_active` is the sanctioned `||` extraction. `floating_controls.rs` logic diff is comments-only.
- **UI:** `ArrowUp`/`ArrowDown` intake occurs at `src/app.rs:143–156` before the layout branch. The gate uses `navigation_context_is_active()`. `show_navigation_layout` includes the central-panel else-arm for the empty-folder message. Center Open is unreachable while `browsed_folder_context` is set.
- **Arrow mapping/labels:** `FolderUp` → `PreviousSiblingFolder`, `FolderDown` → `NextSiblingFolder`; labels are `"Previous sibling folder"` / `"Next sibling folder"`; no Wave-3-reserved wording remains.
- **Folder listing:** Only `first_listed_image` was added; tests unchanged.
- **Marker census:** 5 removed (`sibling_folder_listing.rs` enum, `from_opened_image`, `previous_neighbor_of`, `next_neighbor_of`; `last_viewed_images.rs` `last_displayed_image`), 7 kept (sibling `listed_index`/`listed_folder_count`; folder_listing `listed_index`/`listed_image_count`; image_loading 3).
- **Tests:** All 9 locked test names are present char-for-char across the two files; the shared `TempParentScope` fixture and `app_opened_in` helper are `pub(super)`. The unchanged test files `src/folder_listing/tests.rs`, `src/sibling_folder_listing/tests.rs`, `src/app/tests.rs`, `src/app/floating_controls_tests.rs`, and `src/app/last_viewed_tests.rs` are byte-identical (no diff in the commit range).
- **Rules:** All source files are ≤200 lines. All non-test methods are ≤50 lines except the one test flagged in Fix 2. All functions have ≤2 non-`self` parameters. Max nesting depth is ≤2. Private-by-default is respected. No commented-out code. `if self.navigation_context_is_active()` is a single-section boolean. Edition remains 2024; `Cargo.toml`/`Cargo.lock` are untouched.
- **Greps:** `fn navigate` → 1 hit; `fn neighbor_image_path` → 1 hit; `fn navigation_context_is_active` → 1 hit; `fn enter_sibling_folder` / `fn enter_empty_folder_mode` / `fn sibling_folder_neighbor_path` / `fn vertical_anchor_path` / `fn sibling_folder_target_image_path` → 1 hit each; `EMPTY_FOLDER_MESSAGE` → 4 hits (definition, show-fn usage, test import, test assertion); `LISTING_ANCHOR_FILE_PLACEHOLDER` → 4 hits; `first_listed_image` → 2 hits; `record_displayed_image` → 2 hits; `canonicalize` in `last_viewed_images.rs` → 0 code hits (only the doc comment "never canonicalized"); `ArrowUp`/`ArrowDown` intake → present.
