# Adherence Report — Wave 5.1, Task 4, Step 4.5b (Overall Plan Adherence)

Branch: `fix/wave-5-1-operator-feedback` · HEAD: `645cea3` (working tree clean)
Adherence target: [`.kilo/plans/20261112-wave5-1-task4-name-order-plan.md`](20261112-wave5-1-task4-name-order-plan.md)
Executed chain: `68f3b6c` (plan) → `788b312` (feature) → `645cea3` (4.3-fix, incl. the review-fix plan doc).

**Verdict: CONFORMS on the mechanism, seam, listings, tests, census, budgets, and no-touch list — with ONE required doc-only finding (structure map). The routed `windows_collation.rs` item is ADJUDICATED (a): acceptable as-is, no consolidation owed.**

---

## 1. Plan-required items → landed evidence (all verified at HEAD `645cea3`)

| Plan item | Evidence | Status |
|---|---|---|
| §2/§3.2 `CompareStringEx` mechanism, windows-0.62.2 facts | `src/platform/windows_collation.rs:14-48`: `CompareStringEx(PCWSTR(locale…), NORM_IGNORECASE, &left_units, &right_units, None, None, None)`; UTF-16 slices WITHOUT trailing null (`encode_utf16().collect()`); `CSTR_LESS_THAN/EQUAL/GREATER_THAN` mapped, error (0) → `None`; `user_locale_name` uses `GetUserDefaultLocaleName` + `LOCALE_NAME_MAX_LENGTH` (85) buffer, `written <= 0` → `None` | CONFORMS |
| §3.2 pure fallback on any failure | `src/platform/windows.rs:157-160`: `system_collation_order(...).unwrap_or_else(\|\| compare_entry_names(...))` | CONFORMS |
| §3.2 facade + stub | `platform/mod.rs:19-23` third cfg-gated re-export pair (matches existing pattern); `stub.rs:18-20` routes to `compare_entry_names` (container behavior unchanged) | CONFORMS |
| §3.2 `Cargo.toml` features, no version change | `git show 788b312`: exactly `Win32_Globalization` + `Win32_System_SystemServices` added under the pinned `=0.62.2`; `Cargo.lock` NOT in any Task-4 diff | CONFORMS |
| §3.1 `EntryNameCompare`/`EntryNameOrder` | `src/name_collation.rs:16-40` — plan shape verbatim (`system()` → `crate::platform::compare_entry_names_system`; `with_comparator`; `compare`); `compare_entry_names` body byte-identical (lines 8-14). One plan-external `#[allow(dead_code)]` on `with_comparator` — reviewer-sanctioned (used only by `#[cfg(test)]` modules; dead in non-test builds) | CONFORMS |
| §3.3 horizontal listing via seam | `src/folder_listing.rs`: `from_opened_image` delegates to `EntryNameOrder::system()`; `from_opened_image_with_order` (2 params); `collect_folder_images` sorts via `entry_order.compare` — call-site signatures unchanged | CONFORMS |
| §3.4 vertical listing + anchor consumer | `src/sibling_folder_listing.rs`: stored `entry_order` field; private `SortScope` param object; `resolve_current_folder` extracted; `sorted_scope_folders` sorts and stores the order; `position_of_current_folder` + `listed_folder_name_precedes` read the STORED `self.entry_order` (sort and anchor can never disagree); free fn `preceding_folder_name` fully removed (`rg` → no hits) | CONFORMS |
| §3.5 zero app-layer edits; first-image/last-viewed consumers | Diff `c38a8df..645cea3` touches NO `app.rs`, `keyboard_intake.rs`, `folder_navigation.rs`, `last_viewed_images.rs`, or any pan/zoom/esc file — the first-image rule and last-viewed priority consume the same sorted listing / recorded paths untouched | CONFORMS |
| §4 +6 tests | `name_collation.rs` +2 (`system_order_matches_the_pure_fallback_for_ascii_names`, `injected_comparator_decides_the_entry_order`); `folder_listing/collation_tests.rs` 2; `sibling_folder_listing/collation_tests.rs` 2 — existing tests unchanged (`folder_listing/tests.rs` diff = `pub(super)` fixture widening only; `sibling_folder_listing/tests.rs` untouched — fixtures already `pub(super)` from Wave 3) | CONFORMS |
| §5 no-touch list | Full diff file list `c38a8df..645cea3` = the 11 plan files + `.agent/project-structure.md` + 2 plan docs. No navigation ports, first-image/last-viewed consumers, `Cargo.lock`, scripts, docs, or CHANGELOG touched | CONFORMS |
| §8 review checklist | `rg "compare_entry_names" src/`: definition + tests in `name_collation.rs`, stub fallback, windows fallback, and the two test-module injections — NO direct listing use; both vertical consumers read `self.entry_order`; no commented-out code; fns ≤ 2 params, depth ≤ 2 | CONFORMS |
| §6 5-commit chain | Commits 1-3 landed. Commit 4 (module docs) and commit 5 (4.6 mark) are PENDING their designated later workflow steps (consistent with the caller's 3-commit chain definition) — pending-by-design, NOT a deviation of this step. Note for commit 4: the two listing-module `//!` headers still read "fixed v1 alphanumeric collation" — that rewording is exactly the planned module-docs work, still owed there | MAPPED |

## 2. Census and budgets

- **Census 164**: `rg -c "#\[test\]" src/` sums to **164 across 26 files** = 158 baseline + 6 (Task-4 delta). Matches the review-fix plan's census line.
- **Budgets (all touched files ≤ 200)**: `name_collation.rs` 88 · `folder_listing.rs` 134 · `sibling_folder_listing.rs` 174 · `platform/mod.rs` 23 · `platform/stub.rs` 20 · `platform/windows.rs` 160 · `platform/windows_collation.rs` 48 · `folder_listing/tests.rs` 174 · `folder_listing/collation_tests.rs` 46 · `sibling_folder_listing/tests.rs` 158 · `sibling_folder_listing/collation_tests.rs` 51. **All ≤ 200 ✓.** `Cargo.lock` untouched (3,401 lines, absent from every Task-4 diff).

## 3. Routed informational item — ADJUDICATED: (a) ACCEPTABLE AS-IS

The implementer created `src/platform/windows_collation.rs` proactively; the plan's contingency (§3.2) said "Do NOT pre-create the file" and its trigger (inlined `windows.rs` > 200) was strictly NOT met (inlined ≈ 195 ≤ 200).

**Binding verdict: no correction owed; no consolidation back into `windows.rs`.** Reasons:

1. The "Do NOT pre-create" line was a scope-guard for the junior implementer against speculative file creation — not a design invariant of the mechanism. The landed outcome violates no rule the contingency protects.
2. Rule-conformant: 48-line module, `pub(super)` surface only, ≤ 2 params per fn, depth ≤ 2, no commented code, every file ≤ 200.
3. Behavior/mechanism identical to the plan's §3.2 design: same `CompareStringEx` call signature, same `NORM_IGNORECASE`-only flag set, same null-less UTF-16 slices, same `CSTR_*` mapping, same pure fallback, same consumer contract (`EntryNameOrder::system()` → `crate::platform::compare_entry_names_system` → `windows.rs:157` → `super::windows_collation`).
4. Cleaner facade separation: `windows.rs` (dialog/WIC) stays disjoint from collation and retains 40 lines of headroom; consolidation back would spend ~35 of them for zero functional gain.
5. The simplifier is correct that consolidation is out of its authority — and this adherence step confirms it is not owed at all.

## 4. Required finding (doc-only) — route to caller

**F-1: `.agent/project-structure.md` line 23 (the `platform/` entry) does not list `windows_collation.rs`.** The review-fix commit `645cea3` landed the reviewer's structure-map staleness fix for the two `collation_tests.rs` entries but left the `platform/` entry stale — the same REQUIRED-class finding as the reviewer's #3, one entry short. The project-structure rule requires the map to reflect the current tree. This is outside this step's write authority (plan-only). Suggested routing: fold the one-line addition into the pending commit 4 (`docs: wave5.1 task4 module docs`) or a one-line follow-up — do NOT open a new task chain for it.

## 5. Handoff notes (unchanged, §9 of the plan)

Task 5 verification plan: census expectation 164 re-derived at the discovery run; DLL import audit expected UNCHANGED (set-equal 22 — both new API calls are kernel32 exports, already in the linked set). Task 6 docs: brief addendum item 8 + README/tech/architecture/context collation revision + structure-map lines (F-1 closes the platform gap then).
