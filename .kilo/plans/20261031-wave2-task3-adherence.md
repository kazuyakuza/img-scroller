# Wave 2 Task 3 Adherence Micro-Amendment — 4.5a MUST-FIX 1 (`src/app/tests.rs` > 200 lines)

**Date:** 2026-11-01
**Workflow step:** Wave-2 Task 3 → Step 4.5b-facing micro-amendment of the Task-3 plan (Step 4.1b artifact; this is the diff-bearing record the caller requested)
**Branch:** `feat/wave-2-horizontal-scroll` (verify with `git branch --show-current` before any edit; on mismatch STOP and return a question)
**Fixes ONLY:** 4.5a MUST-FIX 1 from [`.kilo/plans/20261031-wave2-task3-ui-buttons-frontend-verification.md`](./20261031-wave2-task3-ui-buttons-frontend-verification.md) §5: `src/app/tests.rs` = 222 lines, over the 200-line source-file cap (rule: [`.kilo/rules/max-lines-per-file.md`](../../.kilo/rules/max-lines-per-file.md)).
**Binding source files (NOT editable):**
- Plan: [`.kilo/plans/20261031-wave2-task3-ui-buttons.md`](./20261031-wave2-task3-ui-buttons.md)
- Review fix: [`.kilo/plans/20261031-wave2-task3-ui-buttons-review-fix.md`](./20261031-wave2-task3-ui-buttons-review-fix.md) (rustfmt wrap of `rails.rs:19` — already landed)
- 4.5a report: [`.kilo/plans/20261031-wave2-task3-ui-buttons-frontend-verification.md`](./20261031-wave2-task3-ui-buttons-frontend-verification.md)

**Wave binding honored:** original `.kilo/plans/*` files are NOT edited (this is a NEW plan file); no TODO `[DONE]` marks; no documentation files touched.

---

## 0. Scope Guard

This amendment changes EXACTLY three source files and creates ONE. NOTHING else:

| File | Action | Content of change |
| --- | --- | --- |
| `src/app/rails_tests.rs` | CREATE (40 lines) | the moved `arrow_button_enablement_follows_the_navigation_edges` test, verbatim |
| `src/app/tests.rs` | EDIT (222 → 187 lines) | delete the moved test (35 lines), add 5 `pub(super)` visibility prefixes to the fixture (0 added lines) |
| `src/app.rs` | EDIT (160 → 163 lines) | append 3 lines: blank + `#[cfg(test)]` + `mod rails_tests;` |

FORBIDDEN: renaming any test, changing any assertion, moving more than the ONE test, rewriting the fixture, restructuring `src/app/tests.rs` into a directory module, touching `src/main.rs`, `src/app/rails.rs`, `Cargo.toml`, `Cargo.lock`, any docs or any other plan file, any manual reflow ("cargo fmt" reproduction by hand of untouched lines).

## 1. Binding decision — chosen layout (option (a), shared fixture)

**NEW sibling test module `src/app/rails_tests.rs` holding EXACTLY the one arrow-button test**, registered in `src/app.rs`, with the EXISTING fixture shared from `src/app/tests.rs` by widening it `pub(super)` (5 one-word edits, zero added lines).

### 1.1 Why this option (decisive)

- **Option (a) chosen.** The overflow is −22 lines with the fixture-shared variant costing ZERO added lines anywhere and moving exactly the test the 4.5a report fingered. The task-3 plan §3.3 pre-authorized this exact contingency: "if rustfmt later wraps an assert in a 4.3/Task-4 fix cycle, move the whole new test module-wise then — handle via fix cycle, never pre-emptively."
- **Fixture choice inside (a): shared over local duplicate.** A minimal local copy of `TempImageFolder` + `sample_decoded_image` inside `rails_tests.rs` would add ~30 duplicated fixture lines (DRY violation, two diverging fixtures) for zero benefit. The shared variant costs 5 one-word visibility edits, all inside the `#[cfg(test)]` test tree (rule-consistent: registered via `#[cfg(test)] mod tests;`/`mod rails_tests;`, so the widened visibility never exists in non-test builds; no `pub`/`pub(crate)`; `pub(super)` from `tests` = visible to all of `app`'s subtree, which is exactly the two sibling test modules — the minimal widening, with the stated consumer being `rails_tests`).
- **Option (b) rejected** (move 2–3 whole tests): needs the same fixture widening but trims tests.rs far below the required level (e.g. 187 → ~150), splitting sibling navigation tests that share the fixture across files — churn beyond the must-fix, violating the plan's minimal-churn instruction.
- **Option (c) rejected** (directory module `src/app/tests/{mod.rs,..}`): a full restructure turn (move 10 tests + fixture into `tests/mod.rs`, update `.agent/project-structure.md` implications) for a 22-line overflow — infeasible-by-cost when (a) is feasible.
- **Registration point:** `src/app.rs`, matching the established pattern (the `#[cfg(test)]` child-mod declaration inside `src/app.rs`; sibling files under `src/app/`). This consciously supersedes the 4.5a §5 example sketch (`src/app/tests/buttons.rs` declared from `src/app/tests.rs`): that report lists unbound options; the caller-directed layout is the `src/app.rs`-registered sibling, which is also the pattern-consistent one.

### 1.2 Privacy audit (unchanged in spirit, verified for the split)

- `rails_tests` is a child module of `app`, so the file-private `NavigationDirection`, `navigate`, `neighbor_image_path`, `accept_open_outcome`, `message_text` remain PRIVATE (Task-2/Task-3 binding preserved; zero privacy changes to production items).
- `super::rails::navigation_button_enabled` stays `pub(super)` in `rails.rs` = visible to `app`'s subtree, including `rails_tests`. UNTOUCHED (review-fix version, 4-line wrapped signature, stays byte-identical).

## 2. Line-count arithmetic (whole src/ tree, at the amendment)

Current verified counts (landed branch, matches 4.5a §4):

- `src/app.rs` = 160, `src/app/rails.rs` = 74 (74 = the 65-line draft + the review-fix 4-line wrap; `65` from the caller-context is pre-review-fix — the 4.5a report's 74 is the landed truth), `src/app/tests.rs` = 222.

After this amendment:

| File | Arithmetic | Final | ≤ 200 |
| --- | --- | --- | --- |
| `src/app.rs` | 160 + 1 blank + 2 (`#[cfg(test)]` / `mod rails_tests;`) | **163** | ✓ |
| `src/app/rails.rs` | 74, UNTOUCHED | **74** | ✓ |
| `src/app/tests.rs` | 222 − 35 (35 = the blank separator line 188 + test block lines 189–222); the 5 visibility prefixes change words, not lines | **187** | ✓ |
| `src/app/rails_tests.rs` | NEW: 2 doc + 1 blank + 2 imports + 1 blank + 34 (test block, verbatim) | **40** | ✓ |

Other source files: UNTOUCHED by this amendment. 4.5a §4 recorded the full-file audit and found NO other source file over the cap ("Only `tests.rs` crosses the project hard limit") — `src/folder_listing/tests.rs` 174, `src/image_loading.rs` 86, `src/app_display.rs`, `src/scaling.rs`, `src/main.rs`, `src/folder_listing.rs`, `src/platform/*` all unchanged since their Task-3/Task-2 verified states.

## 3. Stepwise implementer instructions (in order)

### E1 — Verify branch and clean working tree

- `git branch --show-current` → must print `feat/wave-2-horizontal-scroll`; else STOP, ask the caller.
- `git status` → working tree must be clean of unstaged source changes (expect only this amendment file as untracked). Read `.gitignore` first per the Gitignore Compliance Rule; none of the touched paths (`src/*`, `.kilo/plans/*`) match any ignore pattern.

### E2 — EDIT `src/app/tests.rs` (5 word edits + 1 block deletion, in this order)

Visibility prefixes (exact current text → exact replacement; each stays ONE line, no reflow):

| Line | Was | Becomes |
| --- | --- | --- |
| 6 | `struct TempImageFolder {` | `pub(super) struct TempImageFolder {` |
| 11 | `    fn create(test_case: &str) -> Self {` | `    pub(super) fn create(test_case: &str) -> Self {` |
| 20 | `    fn create_file(&self, file_name: &str) {` | `    pub(super) fn create_file(&self, file_name: &str) {` |
| 24 | `    fn image_path(&self, file_name: &str) -> std::path::PathBuf {` | `    pub(super) fn image_path(&self, file_name: &str) -> std::path::PathBuf {` |
| 35 | `fn sample_decoded_image() -> DecodedImage {` | `pub(super) fn sample_decoded_image() -> DecodedImage {` |

NOT touched in this file: the `folder_path` field (stays private, read only by tests-local impl), the `Drop` impl, all 9 remaining tests, all blank lines and formatting anywhere else.

Test deletion: DELETE the contiguous block starting at the blank line after `every_successful_open_rebuilds_the_listing_from_the_new_folder`'s closing `}` (line 188, currently blank) through the end of file (line 222, `}` of the arrow test) — i.e. exactly the 35 lines that are `#[test] fn arrow_button_enablement_follows_the_navigation_edges() { ... }` plus its preceding blank separator. The file then ends at line 187 (`}` of the rebuild test) with no trailing blank. Result: **187 lines**.

### E3 — CREATE `src/app/rails_tests.rs` (40 lines, EXACT content)

The test body MUST be byte-identical to the landed test (including the fmt-wrapped `assert!` + path blocks, 34 lines, lines 189–222 of the pre-amendment `tests.rs`); only the header and imports are new:

```rust
//! Unit tests for the corner-rail arrow-button enablement
//! (`super::rails::navigation_button_enabled`).

use super::{ImagesScrollerApp, NavigationDirection};
use super::tests::{TempImageFolder, sample_decoded_image};

#[test]
fn arrow_button_enablement_follows_the_navigation_edges() {
    let folder = TempImageFolder::create("button_edges");
    folder.create_file("a.png");
    folder.create_file("b.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    assert!(!super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Previous
    ));
    assert!(!super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Next
    ));
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert!(!super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Previous
    ));
    assert!(super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Next
    ));
    app.accept_open_outcome(folder.image_path("c.png"), Ok(sample_decoded_image()));
    assert!(super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Previous
    ));
    assert!(!super::rails::navigation_button_enabled(
        &app,
        NavigationDirection::Next
    ));
}
```

Notes: import order is rustfmt-canonical (`super::{...}` before its child path `super::tests::{...}`; uppercase-first within braces, matching the landed `use super::{ImageLoadFailure, UNSUPPORTED_FILE_MESSAGE, pixel_buffer_size};` precedent in `src/image_loading.rs`). The test name, statement order, and all six assertions are IDENTICAL to the landed test. `super::rails::...` resolves from `rails_tests` exactly as it did from `tests` (both are children of `app`).

### E4 — EDIT `src/app.rs` (3 appended lines, ONLY)

Append at the very end of the file (after the current line 160, `mod tests;`):

```rust

#[cfg(test)]
mod rails_tests;
```

Nothing else in `src/app.rs` changes (lines 1–160 stay byte-identical). Result: 157 `mod rails;`, 158 blank, 159 `#[cfg(test)]`, 160 `mod tests;`, 161 blank, 162 `#[cfg(test)]`, 163 `mod rails_tests;` → **163 lines**.

### E5 — Static self-check (no cargo on the host)

1. `wl` per-file line counts: `src/app.rs` 163 / `src/app/rails.rs` 74 / `src/app/tests.rs` 187 / `src/app/rails_tests.rs` 40 — all ≤ 200.
2. All 10 app-module test names intact: in `tests.rs` exactly 9 (`window_title_is_the_final_app_name`, `failed_open_via_cli_reports_the_message_and_keeps_no_path`, `successful_open_stores_path_and_payload_and_clears_message`, `failure_after_success_keeps_the_previous_image_visible`, `navigation_on_the_empty_state_does_not_change_any_state`, `navigation_resolves_the_previous_and_next_sibling_in_sort_order`, `navigation_at_the_first_and_last_list_positions_stays_silent`, `failed_neighbor_open_keeps_the_current_image_and_shows_the_message`, `every_successful_open_rebuilds_the_listing_from_the_new_folder`), in `rails_tests.rs` exactly 1 (`arrow_button_enablement_follows_the_navigation_edges`). Zero renames, zero assertions changed, fixture semantics identical.
3. `git diff --stat` touches ONLY the three files of §0 (plus the untracked plan + new source file).
4. No commented-out code introduced; the only comment is the 2-line `//!` module doc of E3.

## 4. Container verification (via `alpine-vm` MCP; host has NO cargo)

Per `docs/build.md`. `vm_status` FIRST; if the VM or `/img-scroller` share is unavailable, STOP and return a user question. Exit code is authoritative.

1. `vm_status` → must be running and SSH-reachable.
2. Full harness (all four checks, fixed order, one run):
   `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
   Expected output ends `ALL CHECKS PASSED`, exit 0 (`PASS fmt-check`, `PASS check`, `PASS test`, `PASS clippy`).
   Or the equivalent per-check fallback commands (same file, §Per-check) if the harness path is rejected by the caller:
   - `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt --check`
   - `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked`
   - `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo test --locked`
   - `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo clippy --locked -- -D warnings`
3. Test expectation: `26 passed; 0 failed` — the 16 unchanged (folder_listing 5 + scaling 5 + image_loading 4 + app_display 2, verifed by `#[test]` grep) + 10 app-module (E5-2 list). No test may disappear, fail, or be counted twice.
4. `cargo fmt --check` must PASS with the E3 content as written (the wrapped assert blocks are copied from the landed, fmt-verified form; the import order is rustfmt-canonical). If it still fails, REPORT it as a hazard for Task 4 — never hand-reflow (Task-3 plan §8.1 ban).

## 5. Git actions (when this fix is applied)

- Read `.gitignore`, then `git status` (pre-staging). None of the touched paths match ignore patterns (`logs/`, `dist/`, `target/`, IDE/OS patterns are the only matches).
- Stage EXACTLY: `src/app.rs` `src/app/tests.rs` `src/app/rails_tests.rs` and this amendment `.kilo/plans/20261031-wave2-task3-adherence.md`. Nothing else (no TODO files, no `Cargo.*`, no context.md upkeep).
- ONE commit, suggested message EXACTLY: `test: move the arrow-button enablement test to app/rails_tests (200-line cap)`.
- NO amend, NO force, NO push (push stays restricted to workflow step 5).

## 6. Diff vs plan (the 4.5b-facing record)

- **The ONLY in-scope deviation flagged by 4.5a is MUST-FIX 1** (report §5: `src/app/tests.rs` = 222 > 200; report §6 "One FAIL"). THIS amendment resolves it (187 ≤ 200; §2).
- **All other 4.5a items PASS** (cite report): the §1 conformance table items 1–17 and the §2 egui API cross-check are PASS/N-A; §3 lists only operator-perceivable quality notes including the user-confirmed K3 empty-state message consequence (recorded as approved behavior, not a defect); §4 records two already-accounted landed deltas — line-count drift caused by the `cargo fmt` wraps (grandfathered by the review-fix and this amendment) and the clippy-driven `egui::Button<'_>` lifetime (`4b45ca7`/`6f00e6b`), both compile-driven local details inside the plan's §8.1 hazard mechanism and now closed-out.
- **Local structural delta recorded:** the Task-3 plan §6.2/§3.3 placed the new test in `src/app/tests.rs` (predicted 198). With fmt wraps the file landed at 222; this amendment executes the plan's own pre-authorized fallback ("move the whole new test module-wise", plan §3.3) by relocating that ONE test to `src/app/rails_tests.rs` with `pub(super)` fixture sharing. Test names, semantics, assertions, fixture behavior, and every privacy binding (§1.2) are unchanged; §3.2/§3.3 arithmetic of the parent plan is otherwise superseded only where it predicted `tests.rs` 198 (`164+17` → now 9+1 split across two files) and `app.rs` 166 (landed 160 — fewer lines, already certified by 4.5a items 5/6/17).
- Registration-side delta vs 4.5a §5's example: the split module is declared in `src/app.rs` (NOT nested under `tests.rs`) — the established `#[cfg(test)]`-child-in-`app.rs` pattern; the 4.5a sketch listed options, not bindings.

## 7. Out of scope (explicit)

- Any further test moves, fixture rewrites, `cargo fmt` hand-edits, `.agent/project-structure.md` edits (Task 5), TODO `[DONE]` marks (step 4.6), docs updates, `CHANGELOG.md`, edits to any existing plan file, Task-4 harness extensions, and the Task-3 Task-4 OPERATOR-RUN checklist (caller-side).

## 8. Completion evidence checklist for the implementer (report back)

1. Final counts §2 (paste `wc -l`-equivalent evidence or file-tail line numbers).
2. Harness run header/log line + exit code 0 (`ALL CHECKS PASSED`).
3. `cargo test` line showing `26 passed; 0 failed`.
4. `git status` clean after the commit; `git log -1 --stat` staging exactly §5's four paths.
