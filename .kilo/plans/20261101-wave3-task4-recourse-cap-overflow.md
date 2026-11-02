# Task-4 Recourse Plan b2 — fmt-driven line-cap overflow in two test files

- Date: 2026-11-02 (recorded in-session)
- Trigger: the sanctioned `cargo fmt` round (commit `bde6b7f`) re-wrapped long assert calls (default `fn_call_width=60`) and pushed two test files over the hard 200-line cap:
  - `src/sibling_folder_listing/tests.rs`: 200 → **220**
  - `src/app/folder_navigation_tests.rs`: 190 → **202**
- Constraint reality: fmt (`--check`) is a PASS/FAIL gate; the prior single-line shapes no longer satisfy it, so the ONLY compliant ways to get both files ≤200 are content-identical trims or a split. Rejecting (a) collapsing content (locked tests/assertions), (b) repo-wide `rustfmt.toml` policy change (mass re-wrap churn), (c) leaving files over-cap (violates the rule).

## Adjudication (Planner; caller-approved pattern from the Task-3 addendum)

### R1 — `src/app/folder_navigation_tests.rs` (202 → ≤200): SANCTIONED §3.9 TRIM ORDER ONLY

Apply exactly the v2-plan §3.9 trim order, content-identical:
1. Collapse blank lines inside/between test fns (blank groups → none) as needed.
2. Shorten fixture/helper `///` doc comments to single lines.
Do NOT touch any test name, assertion, fixture behavior, helper body, or code token. Target ≤198 (2-line margin). If still >200 after the full trim order → STOP and split per R2's shape (move test 1 to the continuation file).

### R2 — `src/sibling_folder_listing/tests.rs` (220 → ≤200): TRIM FIRST, SPLIT IF INSUFFICIENT

R2a. Apply the same two-kinds trim (blank collapse + doc one-liners). If the file reaches ≤198, done.
R2b. If trims alone cannot reach ≤200: SPLIT — sanctioned shape mirrors the Wave-3 Task-3 addendum precedent (shared fixture across test modules):
   1. In `src/sibling_folder_listing.rs`, beside the existing `#[cfg(test)] mod tests;`, add `#[cfg(test)] mod neighbor_tests;` (keep mod order: `tests` then `neighbor_tests`).
   2. In `tests.rs`, mark the fixture struct `TempScope` and the items the moved tests call (`create`, `current_image_path`, `scope_child_path`, and any helper the moved tests need) as `pub(super)` (test-only reachability; they are under `#[cfg(test)]`, so no release/global surface change, no new dead_code marker — `neighbor_tests` uses them).
   3. New file `src/sibling_folder_listing/neighbor_tests.rs`: exact `use super::tests::{TempScope, …needed helpers};` + `use super::{ListedFolderNeighbor, ScopeEntriesReader, ScopeEntry, SiblingFolderListing};`-style imports as required by the moved tests; move EXACTLY tests 5 (`folder_neighbors_step_by_one_and_report_scope_edges`) and 8 (`symlinked_scope_entries_are_not_listed_as_directories`) — bodies byte-identical (only blank-line trims allowed).
   4. File budgets after split: tests.rs ≤200 (6 remain) and neighbor_tests.rs ≤200 (2 + imports + local helpers).
   Test-name order note for reviewers: file order preserves the §3.3 list order (1-4,6,7 in tests.rs; 5,8 in neighbor_tests.rs); census STAYS 50 in total; name a `tests-split` note in the 4.6 report.

## After either path (R1/R2a/R2b)

- Re-run the container dev-checks (alpine-vm MCP `vm_status` first) — expect `ALL CHECKS PASSED` exit 0 with census 50 (per-file split recorded per file listing).
- Release artifacts: the 0.3.0 exe (19,502,691 bytes) is NOT touched by test-code trims (cfg(test) excluded from release) — NO rebuild required; the DLL audit and exe evidence from the 4.2 run remain valid.
- Commit discipline: one commit for the trims/split with EXACT message `fix: cap-conform test files after format round`; stage ONLY the touched `src/` test files (and the mod decl in sibling_folder_listing.rs when R2b);
- Bound: ONE trim/split round; any second over-cap file → STOP and escalate to the caller.
