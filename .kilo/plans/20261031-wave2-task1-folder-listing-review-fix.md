# Review Fix Plan — Wave 2 Task 1: Current-Folder Image Listing

Date: 2026-10-31
Workflow step: Wave-2 Task 1 → Step 4.3-review cycle (code-reviewer findings)
Amends (WITHOUT editing): [`.kilo/plans/20261031-wave2-task1-folder-listing-fix.md`](20261031-wave2-task1-folder-listing-fix.md)
Constraint source: `.kilo/rules/max-lines-per-file.md`, `.kilo/rules/max-lines-per-method.md`, `.kilo/rules/max-depth.md`, `.kilo/rules/max-arguments-per-method.md`, `.kilo/rules/prefer-private-members.md`, `.kilo/rules/self-documenting-code.md`, `.kilo/rules/no-commented-code.md`, `.kilo/rules/single-section-boolean-conditions.md`

## 0. Scope guard of this fix plan

Resolves ONLY the issues that block `cargo check --locked`, `cargo clippy --locked -- -D warnings`, and `cargo fmt --check` for the Task 1 implementation. ALL design decisions of the original plan and its amendment stay UNCHANGED and VERBATIM: 17-entry extension whitelist, collation operator, helper shapes and APIs, `FolderListing`/`ListedNeighbor` types, the 5 test names and bodies, `TempFolder` fixture policy. No new features, no docs edits, no re-litigation. The ONLY changes are: one import correction and rustfmt formatting.

## 1. Findings summary

### 1.1 Critical compile error

- `src/folder_listing.rs` line 9 imports `OsStr` from `std::path`:
  ```rust
  use std::path::{OsStr, Path, PathBuf};
  ```
  `OsStr` is defined in `std::ffi`, not `std::path`. This causes:
  ```text
  error[E0603]: struct `OsStr` is private
   --> src/folder_listing.rs:9:17
  ```
  `cargo check --locked` fails with exit code 101.

### 1.2 Formatting failure

- `cargo fmt --check` reports diffs in:
  - `src/folder_listing.rs` — the `IMAGE_FILE_EXTENSIONS` array line break.
  - `src/folder_listing/tests.rs` — multiple `assert_eq!` layouts and a chained `.join(...)` expression.
- The implementation matches the plan's Step-B/Step-C shapes, but those shapes do not match the container's `rustfmt` defaults. This will fail the `fmt-check` step of the dev-checks harness.

### 1.3 Plan transcription verification (all correct)

- 17-entry extension whitelist matches exactly and is matched ASCII-case-insensitively via `eq_ignore_ascii_case`.
- Collation uses `str::to_lowercase()` primary + exact-name UTF-8 tie-break; extension is an ordinary name part; no numeric chunking.
- Strict one-level `read_dir`; directories skipped via `is_dir()`.
- `from_opened_image` canonicalizes the parent folder first; entries are canonical.
- `listed_index_of` canonicalizes the request and uses exact `PathBuf` equality.
- `ListedNeighbor` variants (`At { listed_index, image_path }`, `ListBeginningReached`, `ListEndReached`) and `listed_image_count` are present.
- All five test names, fixture semantics (`std::env::temp_dir()`, `Drop` cleanup, no repo-local fixtures), and assertions match the plan.
- `src/main.rs` modified by exactly one line (`mod folder_listing;`), placed alphabetically.
- No other source files touched.

### 1.4 Code-rule check

- File line counts: `src/folder_listing.rs` 126 lines, `src/folder_listing/tests.rs` 138 lines, `src/main.rs` 29 lines — all ≤ 200.
- Method/function bodies are all ≤ 50 lines.
- Max nested block depth is 2 (closures in `collect_folder_images`, assertion + `if` in neighbor methods).
- No function/method has more than 2 non-self parameters.
- Public surface is `pub(crate)` only where the plan requires Task 2/3 consumption; helpers are private.
- No design comments beyond the 3-line module doc; no commented-out code.
- All `if` conditions are single-section (`== 0`, `== len - 1`, `!= Ordering::Equal`, `is_dir()`, etc.).

### 1.5 Correctness hazards noted (no fix required; recorded for awareness)

- **Symlinks to images/directories**: `entry_type.is_dir()` returns `false` for symlinks, so a symlink with a whitelisted extension is listed under the symlink path, while `listed_index_of` resolves symlinks via `canonicalize`. This can cause a mismatch if the user opens the symlink target directly. The plan explicitly decided to skip only `is_dir()` entries; this edge case is outside the current decision and can be addressed when Task 2/5 defines symlink behavior.
- **Non-UTF-8 file names**: the sort comparator uses `to_string_lossy()`, which replaces invalid UTF-8 with `U+FFFD`. The plan's Step-B code block uses `to_string_lossy()`, so the implementation matches the plan; the prose §2.2 idealizes `&str` names. This is an existing plan inconsistency, not an implementer deviation.

### 1.6 Pre-existing verification blocker (NOT caused by Task 1)

- `scripts/dev-checks.sh` contains Windows CRLF line endings (`^M$`). Running `sh scripts/dev-checks.sh` in the container fails immediately with `set: Illegal option -`. This blocks the full harness but does **not** block running the individual cargo commands directly. It is outside the Task 1 source scope; flag to the caller for separate handling.

### 1.7 Minor deferred project-structure update

- `.agent/project-structure.md` does not yet list `folder_listing.rs` or `src/folder_listing/tests.rs`. The plan defers this update to Task 5 (docs step); the file is currently out of date but the deferral is binding per the approved plan.

## 2. Required fixes

### Fix 1 — Correct `OsStr` import

**File:** `src/folder_listing.rs`  
**Line:** 9  
**Old:**
```rust
use std::path::{OsStr, Path, PathBuf};
```
**New:**
```rust
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
```

This restores `cargo check --locked`.

### Fix 2 — Apply rustfmt formatting

**Files:** `src/folder_listing.rs`, `src/folder_listing/tests.rs`  
**Action:** Run the project's pinned formatter inside the container:
```sh
docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt
```
Then verify with:
```sh
docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt --check
```
This applies only whitespace/line-break changes; no logic, names, or plan decisions change.

## 3. Verification after fixes

Run, in order, inside the container:

```sh
docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt --check
docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked
docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo test --locked
docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo clippy --locked -- -D warnings
```

Expected: all four exit 0. `cargo test --locked` must discover and pass the same 5 tests from `src/folder_listing/tests.rs`.

## 4. Git actions

- Branch must remain `feat/wave-2-horizontal-scroll`.
- Before staging: run `git status` and confirm only `src/folder_listing.rs` and `src/folder_listing/tests.rs` are modified by the fixes (plus any pre-existing untracked files). Ensure nothing matching `.gitignore` patterns (`dist/`, `logs/`, `target/`, `*.log`) is staged.
- One fix commit is allowed at workflow Step 4.3/4.4; suggested message: `fix(folder_listing): correct OsStr import and apply rustfmt`.
- NO push in this step.
