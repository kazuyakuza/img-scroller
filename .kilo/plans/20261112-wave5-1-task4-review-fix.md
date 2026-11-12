# Review Fix Plan — Wave 5.1, Task 4 (Navigation Name Order)

Branch: `fix/wave-5-1-operator-feedback`  
Reviewed commits: `68f3b6c` (plan) + `788b312` (implementation).  
Census: 158 baseline → 164 tests (+6), matching plan.

## Findings

### 1. CRITICAL — test failure in `anchor_position_follows_the_injected_order`

**File:** `src/sibling_folder_listing/collation_tests.rs:49`  
**Failure:**

```text
left:  Some(At { listed_index: 1, folder_path: ".../aaa" })
right: Some(At { listed_index: 0, folder_path: ".../aaa" })
```

**Cause:** With the reversed comparator the sorted siblings are `[zzz, aaa]` (indices 0 and 1). The current folder `mid` would be inserted at index 1, so `next_neighbor_of` correctly returns `aaa` at index 1. The test expectation wrongly uses `folder_neighbor(0, aaa)`.

**Fix:** Change the expected index from `0` to `1`:

```rust
assert_eq!(
    listing.next_neighbor_of(anchor.as_path()),
    Some(folder_neighbor(1, scope.scope_child_path("aaa")))
);
```

No production code change is required — the anchor-position logic is correct.

### 2. REQUIRED — `cargo fmt` failure

**File:** `src/name_collation.rs:65`  
**Failure:** First `assert_eq!` in `system_order_matches_the_pure_fallback_for_ascii_names` exceeds the fmt line budget.

**Fix:** Apply `cargo fmt` (verified: only this one line changes). The line becomes:

```rust
assert_eq!(
    entry_order.compare("alpha.gif", "ALPHA.jpg"),
    Ordering::Less
);
```

### 3. REQUIRED — stale `.agent/project-structure.md`

**File:** `.agent/project-structure.md`  
**Cause:** The two new `collation_tests.rs` child modules are not listed under their parent folders, violating the project-structure rule that the map must reflect the current tree.

**Fix:** Update the two folder descriptions:

- `folder_listing/` → `child module: tests.rs (listing/collation/neighbor unit tests), collation_tests.rs (injected-order seam tests for Japanese/ASCII fallback ordering)`
- `sibling_folder_listing/` → add `collation_tests.rs (injected-order seam tests proving vertical sort and anchor position use the same comparator)`

## Verified correct (no fix needed)

- Diff scope: feature commit touches only the 11 files from the plan (`Cargo.toml` + 10 `src/` files).
- `CompareStringEx` caller: only `Win32_Globalization` + `Win32_System_SystemServices` features added; `NORM_IGNORECASE` only; `GetUserDefaultLocaleName` + `LOCALE_NAME_MAX_LENGTH` buffer; `CSTR_*` mapping; pure fallback on `None`.
- Stub fallback routes to `compare_entry_names`; facade re-exports are cfg-gated and match existing pattern.
- `compare_entry_names` body is byte-identical; `EntryNameOrder` shape matches plan.
- Both listings sort through the seam; `SortScope` param object is private and ≤ 2 args; anchor position reads the stored `self.entry_order`; `preceding_folder_name` is fully removed.
- First-image/last-viewed consumers (`app.rs`, `app/folder_navigation.rs`, `last_viewed_images.rs`) are untouched.
- `#[allow(dead_code)]` on test-only seams and `pub(super)` widening of fixture helpers are necessary and minimal.
- `Cargo.lock` is untouched; no new crates; all touched source files are ≤ 200 lines.

## Re-verification after fixes

1. `cargo fmt --check` passes.
2. `cargo test` passes all 164 tests.
3. `scripts/dev-checks.sh` reports `ALL CHECKS PASSED`.

## Commit suggestion

Single `fix: wave5.1 task4 review fixes` commit containing the three fixes above.
