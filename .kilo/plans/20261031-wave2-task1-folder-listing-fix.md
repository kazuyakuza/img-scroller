# FIX Plan — Task 1 file-structure amendment: tests into `src/folder_listing/tests.rs`

Date: 2026-10-31
Workflow step: Wave-2 Task 1 → Step 4.3-review cycle (architector amendment)
Amends (WITHOUT editing): [`.kilo/plans/20261031-wave2-task1-folder-listing.md`](20261031-wave2-task1-folder-listing.md)
Constraint source: [`.kilo/plans/20261031-wave2-horizontal-scroll.md`](20261031-wave2-horizontal-scroll.md) §2.2 (≤200-line files; new plan files only)
Blocking cause: implementer (step 4.2) measured transcript of the original plan = `src/folder_listing.rs` 236 lines after the plan's own Step-D `TempFolder`-support-file remedy, violating `.kilo/rules/max-lines-per-file.md`.

## 0. Scope guard of this fix

Resolves ONLY the file-structure contradiction. ALL design decisions of the original plan stay UNCHANGED and VERBATIM: 17-entry extension whitelist, collation operator, helper shapes and APIs, `FolderListing`/`ListedNeighbor` types, the 5 test names and bodies, `TempFolder` fixture policy (temp dirs under `std::env::temp_dir()`, `Drop` cleanup). No new features, no docs edits, no re-litigation. The ONLY changes are: where the tests and fixture live, how `tests` is registered, and the voided Step-D remedy. The implementer executes the original plan's Steps A–E as amended by §2–§5 below.

## 1. Resolution — CHOSEN LAYOUT: sibling tests module file (idiomatic Rust)

`src/folder_listing.rs` declares `#[cfg(test)] mod tests;` at its END, pointing at the sibling file `src/folder_listing/tests.rs`. This is the standard Rust 2018+ layout: a non-`mod.rs` module file (`folder_listing.rs`) may own a same-named directory containing a child module file — a core rustc path-resolution mechanic (RFC 2126), NOT edition-gated, valid on every edition and guaranteed on the pinned `rust:1.99.1-bookworm` container. The plan's own module code already requires rustc ≥ 1.70 (`let-else`, `is_some_and`), so toolchain compatibility is a fortiori proven. No hard blocker exists; therefore the implementer's fallback (option A: `src/folder_listing_test_support.rs` registered from `main.rs`) is REJECTED and must NOT be created.

Why this beats option A: `main.rs` keeps EXACTLY the original-plan single registration line (`mod folder_listing;`) and gains NOTHING else; the tests stay attached to the module they test (private items of `folder_listing` remain directly visible to the child `tests` module with zero visibility surgery); `#![allow(dead_code)]` stays a single module-level attribute.

## 2. Exact final file inventory and shapes

| # | Action | Path | Content |
| --- | --- | --- | --- |
| 1 | CREATE | `src/folder_listing.rs` | EXACTLY the original plan's Step-B rust block (123 lines, unchanged — module doc comment, `#![allow(dead_code)]`, imports, const, `FolderListing`, `ListedNeighbor`, `impl FolderListing`, 4 free helper functions) PLUS one final line: `#[cfg(test)] mod tests;` |
| 2 | CREATE | `src/folder_listing/tests.rs` | EXACTLY the original plan's Step-C content (fixture + the 5 tests), as specified in §3 |
| 3 | MODIFY | `src/main.rs` | ONE line only, identical to the original plan's Step A: `mod folder_listing;` in the alphabetical mod list. NO further lines (no `folder_listing_test_support`, nothing). |
| 4 | NEVER CREATE | ~~`src/folder_listing_test_support.rs`~~ | Superseded concept from original Step D — must NOT exist |

The `#[cfg(test)] mod tests;` line placement: last line of `src/folder_listing.rs`, after `compare_image_file_names`, blank-line separated per file style. `#![allow(dead_code)]` stays in `folder_listing.rs` (module-level attribute, covers the module incl. its child; Task 2 still removes it).

## 3. `src/folder_listing/tests.rs` Exact Shape

The Step-C content moves VERBATIM with exactly these mechanical transforms:

1. DROP the wrapper opening: the lines `#[cfg(test)]` and `mod tests {` are removed (the file itself is the `tests` module). The wrapper's closing `}` (which the plan never rendered) is correspondingly absent.
2. DE-INDENT by one level: all step-C items sit at top level (0 indent); function bodies inherit normal single-level indents (4 spaces). All other whitespace follows `src/scaling.rs`'s test style exactly.
3. Imports stay EXACTLY as in the original Step C, one code-identical line each:
   ```rust
   use super::{FolderListing, ListedNeighbor};
   use std::fs;
   use std::path::Path;
   ```
   Rationale (item paths): the `tests` module is a CHILD of `folder_listing`, so `super` = `crate::folder_listing` — the original `use super::{...}` line resolves identically. Private items of the parent (`TempFolder`-unrelated helpers like `compare_image_file_names`) are DIRECTLY visible to the child module without any import; no test uses them by path, so no path changes exist anywhere.
4. Then the `TempFolder` fixture (struct + `impl TempFolder` + `impl Drop`, code-verbatim) followed by the 5 tests — `listed_images_sort_case_insensitively_then_by_exact_name_tie_break`, `listing_excludes_directories_and_files_without_listed_extensions`, `unlisted_and_missing_images_resolve_to_no_listing_index`, `neighbors_step_by_one_and_report_list_edges`, `empty_folder_yields_empty_listing_and_missing_folder_yields_none` — bodies, assertions and expected values unchanged from the original Step C.

### Visibility simplification check (binding answer)

NONE. Nothing may change `pub(crate)` → private: `FolderListing`, `ListedNeighbor`, `from_opened_image`, `listed_image_count`, `listed_index_of`, `previous_neighbor`, `next_neighbor` are consumed by Tasks 2/3 per original §3/§8. The child-module visibility trait is only used, never exploited to reduce the API surface.

## 4. Line-budget arithmetic (re-verified)

| File | Derived count | Bound | Verdict |
| --- | --- | --- | --- |
| `src/folder_listing.rs` | 123 (Step-B block, plan lines 187–309) + 1 (`#[cfg(test)] mod tests;`) = 124; realistic range 120–130 | ≤ 200 | OK |
| `src/folder_listing/tests.rs` | Step-C blocks: fixture 36 minus 2 wrapper lines = 34; test1 22; test2 22; test3 16; test4 29; test5 11 → 134; realistic range 130–140 | ≤ 200 | OK |
| `src/main.rs` | current 28 + 1 = 29 | ≤ 200 | OK |

The implementer's measured 236-line monolith is split 124 + 134. Rerun of the original Step-D remedy is IMPOSSIBLE/VOIDED: if——contrary to the arithmetic——any file exceeds 200 lines as written, the implementer must STOP and return a question to the caller (no self-directed restructuring).

## 5. Verification deltas (original Step E, applied to 3 files)

1. Re-read `src/folder_listing.rs`, `src/folder_listing/tests.rs`, `src/main.rs`. Confirm: mod list alphabetical in `main.rs`; `folder_listing.rs` ends with `#[cfg(test)] mod tests;`; all original-plan structural checks (single-section `if`s, ≤2 nesting, ≤50-line methods, ≤2 params, no commented-out code, derives `Debug, PartialEq` on `ListedNeighbor`) hold in BOTH new files; `tests.rs` has no wrapper `mod` of its own and no comments beyond the original ones (none existed).
2. Simultaneous `cargo test` discoverability is unchanged (5 tests, same names) — relevant for Task 4's dev-checks only, nothing to run here.
3. Project-structure map: `.agent/project-structure.md` gains the `folder_listing/` (tests) entry ONLY in Task 5's doc step — NOT now (no docs edits in Task 1, per binding).

## 6. Git deltas (original §7 amended)

- Expected staged set: `src/folder_listing.rs` (new), `src/folder_listing/` (new dir, one file: `tests.rs`), `src/main.rs` (modified), plus the plan files `.kilo/plans/20261031-wave2-task1-folder-listing.md` and this fix plan `.kilo/plans/20261031-wave2-task1-folder-listing-fix.md`. Commit message UNCHANGED: `feat: current-folder image listing for horizontal scroll (Wave 2 Task 1)`.
- Before staging: `.gitignore` + `git status` compliance per `.kilo/rules/gitignore-compliance.md` (nothing matching `dist/`, `logs/`, `target/`, `*.log`), branch check `feat/wave-2-horizontal-scroll`, branch-verify FIRST, no push — all exactly as the original §7.

## 7. Commandeer note (Task 4 reference)

NO commands in Task 1 — unchanged. Verification stays deferred to Task 4's container dev-checks (`vm_status` first → `sh scripts/dev-checks.sh` via `alpine-vm` MCP, exit code authoritative; `cargo test --locked` discovers the same 5 tests from `src/folder_listing/tests.rs` with the same expected outputs).
