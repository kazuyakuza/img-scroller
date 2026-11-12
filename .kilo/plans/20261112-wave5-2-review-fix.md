# Review Fix Plan — Wave 5.2, Tasks 2+3 Combined Cycle (Step 4.3)

**Commits reviewed:** `437640a` + `d188b0b` (`064882a..d188b0b`)  
**Branch:** `fix/wave-5-2-name-order`

## Finding

`cargo fmt --check` fails on `src/platform/windows_collation.rs`.
All other checks pass (`cargo check --locked`, `cargo test --locked`, `cargo clippy --locked -- -D warnings`), with the expected 164 tests / 26 test-file census.

The failure is mechanical formatting only; the functional implementation matches the binding plan constraints.

### Failing diff (rustfmt output)

```diff
Diff in src/platform/windows_collation.rs:5:

 use std::cmp::Ordering;

-use windows::core::PCWSTR;
 use windows::Win32::UI::Shell::StrCmpLogicalW;
+use windows::core::PCWSTR;

 pub(super) fn system_collation_order(left_name: &str, right_name: &str) -> Ordering {
     let left_units: Vec<u16> = left_name.encode_utf16().chain([0]).collect();

Diff in src/platform/windows_collation.rs:13:
     let right_units: Vec<u16> = right_name.encode_utf16().chain([0]).collect();
-    let result = unsafe {
-        StrCmpLogicalW(PCWSTR(left_units.as_ptr()), PCWSTR(right_units.as_ptr()))
-    };
+    let result =
+        unsafe { StrCmpLogicalW(PCWSTR(left_units.as_ptr()), PCWSTR(right_units.as_ptr())) };
     result.cmp(&0)
 }
```

## Fix action

Run `cargo fmt` (or apply the diff above) on `src/platform/windows_collation.rs`.
This reorders the two `windows::*` imports alphabetically and collapses the single-expression `unsafe` block to rustfmt's preferred single-line form.

**No functional change.** The `encode_utf16().chain([0])` null terminators, the `PCWSTR(...)` construction, and `result.cmp(&0)` mapping are preserved.

## Verification after fix

1. Re-run `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`.
2. Expected outcome: `ALL CHECKS PASSED`, census 164 tests.

## Commit segmentation

A single review-fix commit per plan §7 item 3:

```text
fix: wave5.2 tasks 2-3 review fixes
```

Contents: the `cargo fmt` result for `src/platform/windows_collation.rs` only.

## Functional review summary (all satisfied)

- `src/platform/windows_collation.rs`:
  - Uses `StrCmpLogicalW` with `encode_utf16().chain([0])` null-terminated UTF-16 buffers.
  - Japanese-name encoding path is correct: `str::encode_utf16` produces the proper UTF-16 code units for `智乃`; this mirrors the existing `windows.rs:99-102` `OsStr::encode_wide().chain([0]).collect()` + `PCWSTR(...)` pattern.
  - Returns `Ordering` directly; no `Option` or fallback layer.
  - `result.cmp(&0)` maps negative/zero/positive to `Less`/`Equal`/`Greater`.
- `src/platform/windows.rs`:
  - Unused `compare_entry_names` import removed.
  - Wrapper calls `windows_collation::system_collation_order` directly with no fallback.
  - Module doc third sentence accurately describes the system file manager's logical comparison.
- `Cargo.toml`: exactly the two unused features removed; six features remain in original order; version stays `0.5.2`; `Cargo.lock` is untouched.
- No-touch verification: `name_collation.rs`, `folder_listing.rs`, `sibling_folder_listing.rs` changed only in module docs; all listing code, anchor logic, and container test files are untouched.
- Budgets: `windows_collation.rs` 18 lines, `windows.rs` 159 lines (both ≤200); methods ≤50 lines, ≤2 params, depth ≤2.

## Escalation threshold

If the fix commit does not make `dev-checks.sh` pass, escalate after at most three review cycles per plan §7.
