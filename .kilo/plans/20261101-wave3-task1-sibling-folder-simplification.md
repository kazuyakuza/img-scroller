# Task-1 Simplification Plan — Wave 3, Task 1: Sibling-Folder Enumeration (4.3 Code-Simplifier)

- Date: 2026-11-01
- Reviewed commit: `bc7261a` on `feat/wave-3-vertical-scroll` (working tree identical to the commit at review time)
- Binding design plan (untouchable decisions): [`20261101-wave3-task1-sibling-folder-enumeration.md`](20261101-wave3-task1-sibling-folder-enumeration.md)
- Source TODO: [`.agent/todos/20261029/20261029-todo-5.md`](../../.agent/todos/20261029/20261029-todo-5.md) → `### Task 1` ONLY
- Stance: behavior-preserving simplifications only. The landed code is plan-conformant, rule-clean, and already tightly written — only TWO items rise above churn. Everything else is listed under `## Considered and Rejected` with reasons.
- Execution constraint: JUNIOR implementer, 50% restriction — follow the exact code blocks below verbatim. NO cargo, NO VM, NO builds (verification belongs to Task 4's container harness; both changes are behavior-preserving). NO commits by the implementer unless the workflow step instructs one.

## Item 1 — Rename the anchor param to `current_folder_path` in `src/sibling_folder_listing.rs`

- File: `src/sibling_folder_listing.rs` (lines 44–58)
- What: the public methods `previous_neighbor_of` / `next_neighbor_of` name their anchor argument `folder_path`; the design plan § 3.2 specifies `current_folder_path`, and the private callee they forward to is `position_of_current_folder(&self, current_folder_path: &Path)`. The landed name is a plan deviation AND the less self-documenting one (the argument is THE current folder, not any folder).
- Exact minimal change — replace lines 43–58 (both whole methods, attributes included) with:

```rust
    #[allow(dead_code)]
    pub(crate) fn previous_neighbor_of(
        &self,
        current_folder_path: &Path,
    ) -> Option<ListedFolderNeighbor> {
        let position = self.position_of_current_folder(current_folder_path)?;
        if position == 0 {
            return Some(ListedFolderNeighbor::ListBeginningReached);
        }
        Some(self.neighbor_at(position - 1))
    }
    #[allow(dead_code)]
    pub(crate) fn next_neighbor_of(
        &self,
        current_folder_path: &Path,
    ) -> Option<ListedFolderNeighbor> {
        let position = self.position_of_current_folder(current_folder_path)?;
        if position == self.sibling_folder_paths.len() {
            return Some(ListedFolderNeighbor::ListEndReached);
        }
        Some(self.neighbor_at(position))
    }
```

- Formatting note (already encoded above, do NOT judge it yourself): the renamed single-line signatures would exceed rustfmt's default 100-char max_width (99+11 and 95+11 chars), so the parameters wrap one-per-line — the same shape `from_opened_image_with_scope_entries` already uses in this file (lines 59–62). No `rustfmt.toml` exists; defaults apply.
- Why it is a real improvement: plan § 3.2/§ 9 dictated this exact parameter name (deviation correction), and the name now matches `position_of_current_folder` + `resolved_current_folder` used everywhere else in the module (self-documenting-names rule; the argument's ROLE becomes explicit at the public API Task 3 will call).
- Risk check: parameter rename is fully local — Rust callers pass positionally, so every test call (`tests.rs` lines 150–165) and every future caller compiles unchanged. No test name, no assertion target, no logic touched. File grows 120 → 126 lines (≤ 200 cap; ≤ 120 soft plan budget becomes 126 — see `## Budget note`).

## Item 2 — Name the fixture's current-folder literal `CURRENT_FOLDER_NAME` in `src/sibling_folder_listing/tests.rs`

- File: `src/sibling_folder_listing/tests.rs` (lines 3–5 header block, 10–19 `create`, 20–22 `current_image_path`, 104 test 3)
- What: the string `"mid"` (the fixture's CURRENT folder name) is a magic literal in 4 places, and one of them is a load-bearing coupling: test 3 (`current_folder_is_excluded_from_the_enumerated_siblings`) injects the synthetic entry `"mid"` at line 104 — it must equal the name `TempScope::create` actually makes on disk, or the self-exclusion being asserted silently stops testing self-exclusion. A named const states that coupling (code guidelines #13, avoid magic values).
- Exact minimal change — 4 edits:
  1. After line 3 (`use std::path::{Path, PathBuf};`), the existing blank line 4 stays and this block is inserted before `struct TempScope {`:

```rust
const CURRENT_FOLDER_NAME: &str = "mid";

```

  2. Replace `create` (lines 10–19) with:

```rust
    fn create(test_case: &str) -> Self {
        let scope_path = std::env::temp_dir().join(format!(
            "img_scroller_sibling_scope_{}_{}",
            test_case,
            std::process::id()
        ));
        let mid_folder_path = scope_path.join(CURRENT_FOLDER_NAME);
        fs::create_dir_all(&mid_folder_path).expect("scope fixture created");
        fs::write(mid_folder_path.join("shot.png"), b"x").expect("marker written");
        Self { scope_path }
    }
```

  3. Replace line 21 with:

```rust
        self.scope_path.join(CURRENT_FOLDER_NAME).join(file_name)
```

  4. Replace line 104 with:

```rust
            scope.synthetic_entry(CURRENT_FOLDER_NAME, true),
```

- Why the `mid_folder_path` binding inside `create` is mandatory (not judgment): substituting the const directly into the old line 17 would make it 104 chars (> 100) and rustfmt would wrap it, costing MORE lines; the binding keeps every line ≤ 100 and adds only 1 line net there.
- Why it is a real improvement: removes the undocumented fixture↔test coupling above, kills 4 duplicated literals with one name; behavior identical (`"mid"` value unchanged).
- Risk check: no test name changes, no assertion-target changes, only fixture internals + one literal→const swap inside test 3. Every other test is byte-identical.

## Budget note (FORCED FLAG — read before applying Item 2)

- `tests.rs`: 197 → exactly **200** lines (Item 2 adds 3: const + blank + binding). 200 does NOT violate the ≤ 200 rule ("mustn't exceed"), but leaves ZERO margin in this file — any later addition there (e.g., Task 3 tests) must restructure or split the fixture first. This trade (naming clarity vs. zero margin) is a judgment call above the junior's 50% restriction: the CALLER must accept or skip Item 2.
- `sibling_folder_listing.rs`: 120 → 126 lines; exceeds the design plan's soft `≤ 120 total` budget (§ 2 table) though well under the 200 hard cap. Also caller-accept territory; skipping Item 1 instead leaves a known plan-signature deviation in place.
- Items are independent: apply, accept-with-budget-note, or skip each separately. If the caller skips an item, do NOT partially apply it.

## Considered and Rejected (no action — evidence for the review pair)

1. `compare_entry_names` body as `folded.cmp(...).then_with(|| exact.cmp(...))` one-liner — plan § 3.1 locks the body as verbatim the Wave-2 function ("behavior locked"); collapsing it would be a plan deviation, not a simplification.
2. Collapsing `sorted_scope_folders`'s `filter → map → filter` chain into one predicate — compound condition inside the closure fights the single-section-boolean rule, hurts the step-by-step readability the plan spells out (its 4 numbered steps), and saves nothing.
3. Inlining test 5's four single-use thunks (`alpha_at_zero` …) into `Some(folder_neighbor(...))` at the asserts — the asserts then exceed 100 chars and wrap 3 lines each: net +8 lines → the file would blow past the 200 cap. Rejected by the cap, not by taste.
4. Removing the redundant `listed_folder_count()` asserts where a vec-equality assert already implies the count (tests 1/2) — the plan § 3.3 locks each test's assertion targets; deletion is a plan violation.
5. A shared synthetic-reader closure builder for tests 3/5 — plan § 3.3 explicitly decided per-test local closures; only 2 sites anyway.
6. Deduplicating `neighbor_at` / sort-closure / `entry.ok()?` plumbing between `sibling_folder_listing.rs` and `folder_listing.rs` — would edit the plan-locked `folder_listing.rs` ("NOTHING else changes") and is a medium redesign: out of the simplifier's authority, out of Task 1 scope. If the wave ever wants it, it goes through the architect.
7. Missing blank lines between impl methods (`sibling_folder_listing.rs`, `tests.rs`) vs. `folder_listing.rs`'s spaced style — cosmetic; rustfmt preserves both forms, Task 4 owns formatting. Not behavior, not complexity.
8. `ScopeEntriesReader` seam, `ScopeEntry` payload, `Option`-vs-`assert` deviation, self-exclusion-by-position design — all locked plan decisions (§ 4/§ 5); present deviations are intentional and documented in the plan.

## Plan-Deviation Report (reported only, NOT simplified)

- Item 1 IS the only found deviation from locked plan text (param names § 3.2); fixed there.
- 8 locked test names present in § 3.3 order, all assertion targets match; `name_collation.rs` 2 locked tests match; public surface, `#[allow(dead_code)]` placement, `main.rs` alphabetical registrations, and the `folder_listing.rs` extraction diff (import swap + fn deletion only) all conform.
- § 6 contingency NOT needed: `position_of_current_folder` is referenced by live code.

## Implementer Checklist (minor fixes if caller accepts; write-only, no cargo)

1. Apply Item 1 exactly (or skip per caller decision). Verify: no other line in `src/sibling_folder_listing.rs` changes; file = 126 lines.
2. Apply Item 2 exactly (or skip per caller decision — FORCED FLAG above). Verify: file = 200 lines, no test name changed, `git diff` on `src/sibling_folder_listing/tests.rs` touches ONLY the 4 specified regions.
3. Confirm `src/folder_listing/tests.rs`, `src/name_collation.rs`, `src/main.rs`, `src/app.rs` diffs are EMPTY.
4. Report line counts per edited file. No commit unless the 4.3 step explicitly instructs one.
