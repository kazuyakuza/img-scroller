# Wave 3 / Task 2 — 4.3 Code Simplification Plan (last-viewed memory, commit 1428c4b)

- Date: 2026-11-01
- Reviewed: `src/last_viewed_images.rs` (42), `src/app/last_viewed_tests.rs` (96), `src/app.rs` (+7 only, 199/200), `src/main.rs` (+1) — landed state of `1428c4b` on `feat/wave-3-vertical-scroll`.
- Binding plan: [`.kilo/plans/20261101-wave3-task2-last-viewed-memory.md`](20261101-wave3-task2-last-viewed-memory.md). NO locked decision is touched below (module split, plain-parent keys, hook placement, 3-item pub(crate) surface, `Option<&Path>` accessor, `#[cfg(test)]` `is_empty`, bare `#[allow(dead_code)]` marker, 5 locked test names + per-test semantics).
- Note on plan §3.1/§4 "write exactly": the landed code matches the plan verbatim. Items below deviate from that verbatim text in NON-locked details (method bodies / test internals). Caller approval required before the fix step applies them.
- Junior-fit: all three items are minor local edits, fully specified — within the 50%-restricted implementer's latitude; no architect escalation.

## Item S1 — `src/last_viewed_images.rs`: eager `unwrap_or` in `record_displayed_image`

- What: line 24, replace the lazy fallback closure.
- Exact change (whole-line replacement):
  - `        let folder_path = image_path.parent().unwrap_or_else(|| Path::new(""));`
  + `        let folder_path = image_path.parent().unwrap_or(Path::new(""));`
- Why: `Path::new("")` is a side-effect-free const-fn coercion — the lazy form falsely signals expensive computation, and modern clippy's `unnecessary_lazy_evaluations` targets exactly this shape (pre-empting a 4.4 dev-check round-trip). Behavior identical.
- Risk check: zero behavior change; no API/surface change; line count unchanged (stays byte-for-byte ≤ 42 pre-S2). fmt-stable (70 chars). If a future clippy build ever prefers the lazy form instead, revert this single line — no other edit depends on it.

## Item S2 — `src/last_viewed_images.rs`: function-reference `map` in `last_displayed_image`

- What: lines 33-35 (2-line chain + closure) collapse to one line with a function reference.
- Exact change:
  - ```
        self.folder_to_image_paths
            .get(folder_path)
            .map(|image_path| image_path.as_path())
    ```
  + `        self.folder_to_image_paths.get(folder_path).map(PathBuf::as_path)`
- Why: `PathBuf::as_path` is self-documenting and removes a trivial closure — the clippy-idiomatic form (`redundant_closure`). Shorter and clearer, same borrow semantics (`Borrow<Path>` lookup unchanged, `Option<&Path>` return unchanged — locked accessor untouched).
- Risk check: line result = 73 chars ≤ 100 so rustfmt keeps it single-line (verified width by hand; no `cargo fmt` run in this step). File 42 → 41 lines (far under 200). `PathBuf` already imported (line 7). No test change: all 5 tests call through the public accessor unchanged.

## Item S3 — `src/app/last_viewed_tests.rs`: single assert-helper for the repeated check block

- What: the "derive folder_path from the bound image path → assert the map entry" block (2 setup-ish lines + 4-line `assert_eq!`) repeats 7 times across tests 2-5. Extract one module-private helper; every locked test name, scenario, and assertion semantics stay byte-identical in intent.
- Exact change: REPLACE the whole file with the content below (Write tool, real newlines).

```rust
//! Unit tests for the session last-viewed memory: empty on a fresh app,
//! updated on every successful open, unclobbered by failed opens, and
//! scoped per folder.

use super::tests::{TempImageFolder, sample_decoded_image};
use super::ImagesScrollerApp;
use crate::image_loading::UNSUPPORTED_FILE_MESSAGE;
use std::path::{Path, PathBuf};

fn assert_folder_remembers(app: &ImagesScrollerApp, image_path: &Path) {
    let folder_path = image_path.parent().expect("fixture path has a parent");
    assert_eq!(
        app.last_viewed_images.last_displayed_image(folder_path),
        Some(image_path)
    );
}

#[test]
fn fresh_app_starts_with_an_empty_last_viewed_memory() {
    let app = ImagesScrollerApp::empty_state();
    assert!(app.last_viewed_images.is_empty());
}

#[test]
fn successful_open_records_the_folders_last_viewed_image() {
    let folder = TempImageFolder::create("last_viewed_record");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    let image_path = folder.image_path("b.png");
    app.accept_open_outcome(image_path.clone(), Ok(sample_decoded_image()));
    assert_folder_remembers(&app, &image_path);
}

#[test]
fn every_subsequent_open_in_a_folder_updates_the_last_viewed_entry() {
    let folder = TempImageFolder::create("last_viewed_update");
    folder.create_file("a.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    app.accept_open_outcome(folder.image_path("c.png"), Ok(sample_decoded_image()));
    let image_path = folder.image_path("c.png");
    assert_folder_remembers(&app, &image_path);
}

#[test]
fn failed_open_keeps_the_folders_last_viewed_image() {
    let folder = TempImageFolder::create("last_viewed_failure");
    folder.create_file("b.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(folder.image_path("b.png"), Ok(sample_decoded_image()));
    app.open_image(PathBuf::from("unregistered-in-container.bin"));
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    let image_path = folder.image_path("b.png");
    assert_folder_remembers(&app, &image_path);
}

#[test]
fn each_folder_keeps_its_own_last_viewed_entry() {
    let first_folder = TempImageFolder::create("last_viewed_first");
    first_folder.create_file("a.png");
    let second_folder = TempImageFolder::create("last_viewed_second");
    second_folder.create_file("e.png");
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(first_folder.image_path("a.png"), Ok(sample_decoded_image()));
    app.accept_open_outcome(
        second_folder.image_path("e.png"),
        Ok(sample_decoded_image()),
    );
    let first_image_path = first_folder.image_path("a.png");
    let second_image_path = second_folder.image_path("e.png");
    assert_folder_remembers(&app, &first_image_path);
    assert_folder_remembers(&app, &second_image_path);
    app.open_image(second_folder.image_path("unregistered-decoy.png"));
    assert_eq!(app.message_text, Some(UNSUPPORTED_FILE_MESSAGE));
    assert_folder_remembers(&app, &first_image_path);
    assert_folder_remembers(&app, &second_image_path);
}
```

- Why a real improvement: ~78 vs 96 lines; the single-sourced block kills 7 copies of the plan's documented junior trap (borrow the binding, never an inlined temporary — the helper receives an already-bound `&Path`, so the trap becomes impossible at every call site); `expect` message single-sourced. Test-helper precedent exists in this repo (`sibling_folder_listing/tests.rs` `listing_for`/`folder_neighbor`, `folder_listing/tests.rs`).
- Locked-semantics audit: all 5 names kept verbatim; each test asserts exactly what the plan §4 mapping states (helper reproduces the same parent-derivation and equality, just once). The `message_text` failure assertions and the decoy step remain inline in tests 4 and 5 — untouched logic.
- Risk check: helper has 2 args (rule OK), body 7 lines (≤50), depth ≤2; module-private fn, used by all tests (no dead-code). Visibility: test file is a child module of `app`, `app.last_viewed_images` field access unchanged. Line budget: standalone test file, 96 → ~78, no cap pressure. fmt-stable by construction (mirrors the already-committed shapes). Census unchanged: still 5 `#[test]`.

## Items considered and rejected (no action)

1. **Two `to_path_buf()` allocations in `record_displayed_image`** — asked explicitly. Honest judgment: unavoidable. The locked `HashMap<PathBuf, PathBuf>` must own the key string AND the value string (two distinct strings), so a single-allocation pattern does not exist without changing the API or data shape (both locked). The `clone` + `pop()` variant is still two allocations AND diverges from the locked plain-`parent()` key derivation (root/edge semantics) — strictly worse. No change.
2. **`src/app.rs`** — nothing proposed; 199/200 zero margin and every added line is plan §3.3-locked (hook, field+doc, init, import, test-mod decl). The only cosmetic finding (missing blank line between the last two `#[cfg(test)]` mod decls, lines 197/198, inconsistent with 193-196) would ADD a line to reach exactly 200/200, consuming Task 3's headroom — rejected as a net-negative.
3. **Doc-comment tightening** — the 3-line `record_displayed_image` doc carries the locked §2.2 Task-3 key-derivation contract; the module header carries the no-persistence statement. Nothing removable without losing locked information. Rejected.
4. **`empty()` → `Default`** — would change the locked 3-item `pub(crate)` surface. Rejected (boundary also forbids new derivations).
5. **Key canonicalization** — no disagreement to flag: the plain-parent key is consistent (the map never indexes into a listing) and the §2.2 Task-3 seam contract is the right place for any future normalization.
6. **`main.rs` +1** — minimal, correct alphabetical placement. Nothing.

## Application order (if approved)

S1 → S2 (same file; S2 last so S1's line number stays valid — both edits are in different methods, order-independent in fact) → S3 (whole-file write). No cargo/VM commands in the applying step; dev-checks belong to the authorized later harness step; any rustfmt drift there is fixed with `cargo fmt` only (Wave-2 precedent).
