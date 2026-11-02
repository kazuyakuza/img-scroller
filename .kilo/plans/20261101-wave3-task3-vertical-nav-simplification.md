# Task-3 4.3 — Code Simplification Plan (Wave 3, Task 3, vertical navigation)

- Date: 2026-11-01
- Author: code-simplifier (4.3 concurrent pair)
- Scope reviewed: commit `5d503a2` (range `36be11b..5d503a2`, exactly 1 commit — verified `git log --oneline 36be11b..5d503a2`):
  `src/app/folder_navigation.rs` (171), `src/app/empty_folder_view.rs` (19),
  `src/app/folder_navigation_tests.rs` (190), `src/app/folder_navigation_continuation_tests.rs` (177),
  `src/app.rs` (199), `src/app/arrow_direction.rs` (66), `src/app/floating_controls.rs` (170, comments-only),
  `src/folder_listing.rs` (123), `src/sibling_folder_listing.rs` (129), `src/last_viewed_images.rs` (39)
- Binding inputs honored: plan v2 (`20261101-wave3-task3-vertical-nav-navigation-v2.md`), test-split addendum, TODO Task 3, `.kilo/rules/`.
- Method: static read/grep only. NO cargo / VM / build executed (4.2/4.3 boundary; Task 4 owns the harness). No sources edited.
- Verdict: **1 required item** (gate necessity, junior-executable, 0-line-delta). All other candidate simplifications were evaluated and **rejected with evidence** (§3). Report-only notes for the 4.5a/doc owner and the reviewer in §4.

---

## 1. S1 — REQUIRED (Task-4 compile-gate): `clippy::ptr_arg` on `app_opened_in(&PathBuf)`

The implementer's flag is REAL (not rubber-stamped): verified two ways.

1. Landed text: `rg "&PathBuf|&String|&Vec<" src` → **exactly one hit** in the whole crate:
   `src/app/folder_navigation_tests.rs:48: pub(super) fn app_opened_in(scope_image_path: &PathBuf) -> ImagesScrollerApp {`
2. Gate: `docs/build.md` line 31 — the harness runs `cargo clippy --locked -- -D warnings` → any default-warn lint is a hard Task-4 failure.
   Clippy `ptr_arg` = **style / warn-by-default** ("Checks for function arguments of type `&String`, `&Vec`, `&PathBuf`, and `Cow<_>`… it will also suggest you replace `.clone()` calls with the appropriate `.to_owned()` calls").
   The signature is `pub(super)` (not exported API) → no `avoid-breaking-exported-api` escape.

Origin note: the defect comes from the plan itself — v2 §3.9 imports block literally writes `fn app_opened_in(scope_image_path: &PathBuf)`. This fix is the minimal deviation that keeps every locked property (test names, file split, fixture shape, assertions, `pub(super)` seam surface) while clearing the gate. No test semantics change.

### Exact edits (atomic, junior-executable — single file, 2 edits)

File: `src/app/folder_navigation_tests.rs`

**Edit 1 — line 6 (import group):**

- `use std::path::PathBuf;`
+ `use std::path::{Path, PathBuf};`

(`PathBuf` stays used: struct field line 9, return types lines 22/33/37 → no unused-import warning.)

**Edit 2 — lines 48–50 (helper signature + the one `.clone()` it feeds):**

```rust
// BEFORE
pub(super) fn app_opened_in(scope_image_path: &PathBuf) -> ImagesScrollerApp {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(scope_image_path.clone(), Ok(sample_decoded_image()));

// AFTER
pub(super) fn app_opened_in(scope_image_path: &Path) -> ImagesScrollerApp {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(scope_image_path.to_path_buf(), Ok(sample_decoded_image()));
```

Line 51 (`app`) and 52 (`}`) unchanged. `to_path_buf()` (not `to_owned()`) chosen to mirror the landed seam idiom `folder_navigation.rs:82` (`last_viewed.to_path_buf()`); neither form is linted.

**No call-site edits.** All 11 call sites pass `&<PathBuf expression>` — `folder_navigation_tests.rs` 63, 82, 105, 124, 155 and `folder_navigation_continuation_tests.rs` 34, 58, 76, 96, 122, 158 — and `&PathBuf → &Path` is automatic deref coercion at a function-argument coercion site; the temporary `PathBuf` lives to end of statement and the helper still takes an owned copy internally, so behavior is byte-identical.

### S1 checks

- Behavior: identical (same owned `PathBuf` reaches `accept_open_outcome`; identical paths, same temp-dir fixtures).
- Line budget: **0 net lines** — file stays 190 (cap 200, guard intact); continuation file untouched (177); `app.rs` untouched (199).
- Caps/rules: still ≤2 args per fn, depth ≤2, no new comments, no commented-out code, `pub(super)` visibility unchanged (NOT widened — §3.9/§8 seam surface untouched), no derives, no unwrap/expect additions.
- rustfmt: new body line = 88 chars, call args = 58 chars ≤ `fn_call_width` 60 → stays one line; import line = one line; no reflow risk under `cargo fmt --check`.
- Locked-shape safety: test names (9), file split, `TempParentScope` shape/5 methods, `sample_decoded_image` reuse pattern, assertion targets — all untouched.

### Verification the implementer must perform (read-only, no builds)

1. Re-read lines 1–6 and 44–53 of `folder_navigation_tests.rs`.
2. `rg -n "&PathBuf" src` → expected **0 hits**.
3. `rg -n "app_opened_in" src` → expected 1 definition + 11 call sites + 1 cross-file import (unchanged count).
4. File line count still 190.

---

## 2. Other clippy-predictable items: PROVEN-ABSENT (evidence, so Task 4 is not left to chance)

Each landed pattern was checked against default-on clippy groups; none fires:

- `unnecessary_wraps` — **pedantic / allow-by-default** (clippy lint list verified): the all-`Some` `ArrowControl::navigation_direction` (arrow_direction.rs:58–65) is NOT a gate risk. Its `Option` shape is also plan-mandated (§3.4); converting to a bare return would force `floating_controls.rs` **logic** edits (forbidden) — see §3 item R5.
- `option_if_let_else` — **nursery / allow**: seam `if let Some(..) { return Some(..) } fallback` shapes (folder_navigation.rs:81–84, 160–164) are safe; the nursery form (`map_or_else`) would read worse anyway.
- `unnecessary_lazy_evaluations` / `or_fun_call` — do not fire on `open_image`'s `unwrap_or_else(|_| path.clone())` (capturing closure = the clippy-recommended lazy form; app.rs:57).
- `uninlined_format_args` — the fixture `format!("img_scroller_folder_nav_{test_case}_{}", std::process::id())` is fine: the positional arg is a call expression (not inlinable) and `test_case` is already inline; the identical pattern already lives in the fmt/clippy-verified `src/app/tests.rs:12–15`.
- `needless_return` / `redundant_closure` / `bool_assert_comparison` / `len_zero` / `derivable_impls` / `items_after_statements` — no hits in the landed text (all fns end with an expression; match-arm `return None`s are the plan-sanctioned direction dispatch; assertions use `assert!`/`assert_eq!` correctly; `items_after_statements` only inspects blocks, the mid-file `use` at app.rs:34 is a top-level item).
- No new `&String`/`&Vec<_>` params; no `clone()` on `Copy` types (`ArrowControl` never cloned; `NavigationDirection` isn't `Copy` and is only moved — no derive added).
- `unused_imports`: every landed import has a use (checked file by file; §3.9 helper import `Path` lands only with S1, which adds it).

---

## 3. Simplification candidates evaluated and REJECTED (honest evaluation, no edits)

- **R1 — share a private fn between the two nav resolvers** (`listed_image_neighbor` vs `sibling_folder_neighbor_path`, folder_navigation.rs:114–154): the shared skeleton (listing → per-direction neighbor → `At` decode → `Some(path)`) spans two different listing types and two different neighbor enums with different anchor derivations (index-of-opened vs `vertical_anchor_path`). Deduping needs a trait/closure adapter = added surface + depth, violates "do not merge axes into one resolver"/no-derive/no-surface-growth, and would net MORE lines. Rejected.
- **R2 — `LISTING_ANCHOR_FILE_PLACEHOLDER` join idiom appears 3×** (lines 104, 164, 168): extracting `listing_anchor(&Path) -> PathBuf` replaces three 1-line expressions with three calls + a new 5-line fn (+ doc) = net +2 lines and one more indirection, and blurs the plan §8 placeholder grep audit. Rejected.
- **R3 — redundant `.clone()`s in the seam**: `vertical_anchor_path`'s `opened_path.clone()` (161) and `sibling_folder_target_image_path`'s `last_viewed.to_path_buf()` (82) are required — both return owned `PathBuf` from `&self` borrows; no borrow-trivial removal exists. `enter_sibling_folder`/`enter_empty_folder_mode` move `folder_path` with no clones. Rejected (nothing to remove).
- **R4 — stray-file fixture duplication across the two test files** (`fs::write(scope.child_folder_path("folder2").join("stray.txt"), b"x").expect("stray file written")` at folder_navigation_tests.rs:122–123, 153–154 and continuation:118–119): the only ways to dedupe are (a) `scope.create_image("folder2", "stray.txt")` — functionally valid (the fixture writes `b"x"` to any name) but it makes a method named `create_image` produce a non-image and silently reuses the `"fixture image written"` expect message = misleading, violates self-documenting-code; or (b) a new fixture method — changes the plan-locked §3.9 fixture shape / sanctioned addendum shape. 2-line × 3 duplication inside tests does not pay for either. Rejected; addendum's shared-fixture rule already prevents the worse (cross-file copy of fixture/helper).
- **R5 — `navigation_direction` returning `Option` although all four arms are `Some`** (arrow_direction.rs:58–65): genuinely dead `Option` in today's semantics, but the return shape is plan-mandated (§3.4 verbatim arms) and unwrapping it requires editing `floating_controls.rs` `arrow_button_is_enabled`/`click_direction` logic (forbidden by caller + plan §3.5/§8); the generic chain keeps its shape for a future disabled-arrow state. Rejected (and not a clippy risk, §2).
- **R6 — `navigate` dispatch shape** (`matches!` pre-computed bool + if/else, folder_navigation.rs:36–47): explicitly locked by the caller ("do NOT change the seam dispatch (matches! guard)"). Not touched.
- **R7 — test assertion idioms duplicated (`last_viewed_images.last_displayed_image(&scope.child_folder_path(..))`)**: a shared `assert_folder_memory(..)` helper would have to take `&ImagesScrollerApp` + a `&TempParentScope` + names (≥3 args → violates the ≤2-args rule) or become per-file; cross-file placement is already addendum-governed. Rejected.
- **R8 — blank line between the last two `#[cfg(test)] mod` declarations** (app.rs:197): removing it would give app.rs 1 line of headroom, but the blank is explicitly sanctioned by the addendum ("+ adjacent blank") and app.rs already satisfies the cap; a line-shaving edit with no maintainability value is churn. Rejected — and see §5's zero-margin rule instead.

---

## 4. Report-only (NOT Task-3 simplification edits; owners noted)

- **Doc wording for the 4.5a owner** (v1/wave residue in landed text; caller assigned doc wording to 4.5a):
  - `src/app.rs:18` — `opened_image_path` doc still says "(read again by Wave 2's navigation code)"; that code now lives in `folder_navigation.rs` (the §3.2 item-13 doc sweep only retitled `image_is_open`/`show_empty_state_layout`).
  - `src/last_viewed_images.rs:22,30` — "Task 3 looks up… / Task 3's target-priority lookup calls this": forward-looking wording that is now historical.
  - `src/app/folder_navigation.rs:17` "(the Task-1 plan § 7 trick…)" and `:135` "the Task-1 API" — plan-reference prose; matches project comment style, no action needed unless 4.5a standardizes.
- **Reviewer-facing grep deltas vs plan §8** (plan text, not code): `LISTING_ANCHOR_FILE_PLACEHOLDER` has **4 hits** (definition + `enter_empty_folder_mode` + `vertical_anchor_path` + `first_image_path_of_folder`), not 3 — §1.1's re-anchor trick mandates the extra use; counts for `EMPTY_FOLDER_MESSAGE` (3: def + show fn + test 4), `first_listed_image` (2), `fn navigate`/`fn neighbor_image_path`/`navigation_context_is_active` (1 each) and the D2 invariants (no `skip` logic, no wrap, no map write for empty folders, `floating_controls.rs` diff comments-only) verify CLEAN in the landed text.
- **Landed line counts vs plan §3.0 tables**: `sibling_folder_listing.rs` 129 (plan table said 133→133 while §3.6 removes 4 marker lines) and `last_viewed_images.rs` 39 (table 40→40 while §3.7 removes 1) — plan-table arithmetic drift, code correct; `folder_navigation.rs` 171 (vs ~130 estimate) still ≤200; every file inside caps.
- **Marker audit**: removed 5 markers (sibling 21/38/48/60 + last_viewed 31), kept 7 (sibling 24/43, folder_listing 22/53, image_loading 18/25/37), added 0 → matches plan §4 census.

---

## 5. Line-budget & cap snapshot (all files, post-S1 expectation)

| File | Now | After S1 | Cap |
| --- | --- | --- | --- |
| `src/app.rs` | **199/200 — ZERO-MARGIN WARNING** | 199 (untouched) | 200 |
| `src/app/folder_navigation.rs` | 171 | 171 | 200 |
| `src/app/folder_navigation_tests.rs` | 190 | **190 (0 delta)** | 200 |
| `src/app/folder_navigation_continuation_tests.rs` | 177 | 177 | 200 |
| `src/app/empty_folder_view.rs` | 19 | 19 | 200 |
| `src/app/arrow_direction.rs` | 66 | 66 | 200 |
| `src/app/floating_controls.rs` | 170 | 170 | 200 |
| `src/folder_listing.rs` | 123 | 123 | 200 |
| `src/sibling_folder_listing.rs` | 129 | 129 | 200 |
| `src/last_viewed_images.rs` | 39 | 39 | 200 |

**app.rs zero-margin rule for the implementer and Task 4:** `app.rs` sits at 199 (within the addendum ballot 197–199, but with 1 line of slack). ANY future edit to `app.rs` — including `cargo fmt` fixes or clippy responses — must net **≤ 0 added lines** or the plan §3.2 contingency (drop the two field doc lines / mod-adjacency blank) must be applied deliberately, never silently. S1 deliberately touches only the test file.

---

## 6. Step card for the junior implementer (only step in this plan)

1. Open `src/app/folder_navigation_tests.rs`.
2. Apply Edit 1 (§1) verbatim: line 6 import → `use std::path::{Path, PathBuf};`.
3. Apply Edit 2 (§1) verbatim: signature `&PathBuf` → `&Path`, body `.clone()` → `.to_path_buf()`. Nothing else in the file, and nothing in any other file.
4. Run the four read-only checks in §1 (grep + line count).
5. Report: file, 2 edits, line count 190, `rg "&PathBuf" src` = 0 hits.

No build/cargo/VM command; no commit (commit is the workflow's step, not this one's).
