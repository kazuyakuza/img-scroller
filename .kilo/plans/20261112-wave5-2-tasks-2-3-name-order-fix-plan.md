# Implementation Plan — Wave 5.2, Tasks 2+3 COMBINED Cycle: the Name-Order Mechanism Swap to the System File Manager's Comparator

Branch: `fix/wave-5-2-name-order` (already created at global-plan Step 2; version already bumped to 0.5.2 at `064882a` — NO branch/version actions in this cycle) · TODO source: [`.agent/todos/20261112/20261112-todo-2.md`](../../.agent/todos/20261112/20261112-todo-2.md) Tasks 2–3 · Global plan: [`.kilo/plans/20261112-wave5-2-name-order-global-plan.md`](20261112-wave5-2-name-order-global-plan.md) Tasks 2+3 · This is the 4.1b architector output for the COMBINED cycle (ONE 4.2→4.6 pass; BOTH todo headings marked at the shared 4.6). NOT front-end — no 4.1a exists or is needed.

## §0 The Diagnosis (caller-confirmed with the operator data — ENCODED VERBATIM, binding)

- The operator's real folder: `智乃 (1).jpg` … `智乃 (97).jpg` (Japanese 智乃 + " (N).jpg", N = 1..97).
- Lexical collation (BOTH the old code-point order AND Wave-5.1's `CompareStringEx` user-default-locale) orders digit runs lexically: `(1)`, `(10)`, `(11)`, …, `(19)`, `(2)`, `(20)`, … — the operator sees that and calls it wrong.
- The operator expects — and the system file manager shows — NATURAL/numeric order: `(1)`, `(2)`, …, `(9)`, `(10)`, … — Explorer's exact comparison is `StrCmpLogicalW` (digit-aware, case-insensitive, shlwapi).
- VERDICT: Suspect C confirmed (expectation/mechanism gap — NOT a runtime bug in `CompareStringEx`, which is now moot). The fix class = the mechanism swap to the Explorer-exact comparator — the fulfillment of brief addendum item 8 ("the system order" = the system file manager's order), pre-approved by the addendum + the operator's confirmed expectation. NO further caller approval needed.
- Suspect-A silent-fallback audit line (record, one sentence): the Wave-5.1 mechanism is replaced before its runtime behavior was ever confirmed on real Windows; the operator data does not distinguish "CompareStringEx failed silently" from "CompareStringEx worked but sorts digits lexically" — both are superseded by the mechanism swap.
- Suspect-B wiring audit result (verified this session): EVERY production construction path routes through `EntryNameOrder::system()` — `src/app.rs:86` (`FolderListing::from_opened_image` → `src/folder_listing.rs:33`), `src/app.rs:89` and `src/app/folder_navigation.rs:110` (`SiblingFolderListing::from_opened_image` → `src/sibling_folder_listing.rs:45`), `src/app/folder_navigation.rs:169` (the first-image probe → the same constructor). No path defaults to the pure order. The wiring is CLEARED and UNCHANGED by this fix.

## §1 The Fix — Mechanism Decision (junior-encoded; NO alternatives left open)

Replace the Wave-5.1 `CompareStringEx`/`GetUserDefaultLocaleName` implementation in `src/platform/windows_collation.rs` with `StrCmpLogicalW` — the system file manager's logical name comparison (shlwapi): digit-aware natural ordering (`1`, `2`, …, `9`, `10`, `11` …), case-insensitive. Consequences of the API's shape:

- `StrCmpLogicalW` takes TWO null-terminated `PCWSTR` strings and returns `i32` with the same negative/zero/positive convention as `CompareString*` — there is NO error/failure return value. The comparator is therefore TOTAL on Windows: the `Option<Ordering>` + silent-pure-fallback layer of Wave-5.1 is REMOVED (it exists only for fallible APIs).
- Tie convention: result `0` (case-only or identical names) → `Ordering::Equal` — the listings' `sort_by` is Rust's STABLE sort, so equal-comparing names keep `read_dir` order. This matches the currently-landed Wave-5.1 convention (which also returned `Equal` on `CSTR_EQUAL` without a pure tie-break). NO tie-break layer is added.
- The `windows` crate path: `windows::Win32::UI::Shell::StrCmpLogicalW` needing feature `Win32_UI_Shell` — ALREADY in `Cargo.toml`'s feature list since Wave 1 (verified: `Cargo.toml:24`; resolved metadata `windows-0.62.2/Cargo.toml:702` → `Win32_UI_Shell = ["Win32_UI"]`, `:681` → `Win32_UI = ["Win32"]` — transitively satisfied by the existing set). NO feature addition of any kind.
- `Cargo.toml`: REMOVE the now-unused `Win32_Globalization` and `Win32_System_SystemServices` features (the Wave-5.1 additions). `Cargo.lock` is UNTOUCHED (lock entries record package versions/checksums, not features; feature removals are lock-neutral — verified policy precedent from Wave-5.1's additions; the `--locked` harness runs prove it).
- The stub side (`src/platform/stub.rs`) stays byte-identical: the container build's system order remains the pure `compare_entry_names` fallback.

## §2 Resolved-Source Facts (windows crate 0.62.2, container-verified 2026-11-12 — cite in the todo tick)

Verified against `/usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/windows-0.62.2` after `cargo fetch --locked` inside the compose container (the established registry path pattern):

- Binding (`src/Windows/Win32/UI/Shell/mod.rs:4643`):
  `pub unsafe fn StrCmpLogicalW<P0, P1>(psz1: P0, psz2: P1) -> i32 where P0: windows_core::Param<PCWSTR>, P1: windows_core::Param<PCWSTR>` — the `link!` at `:4648` is `windows_core::link!("shlwapi.dll" "system" fn StrCmpLogicalW(psz1: windows_core::PCWSTR, psz2: windows_core::PCWSTR) -> i32)`.
- The strings are NULL-TERMINATED (raw C-string API — unlike Wave-5.1's `CompareStringEx`, which took counted slices WITHOUT a terminator). Each argument must be a UTF-16 encoding WITH the trailing `0` terminator held alive for the call.
- `Param<PCWSTR>` accepts a plain `PCWSTR` value (proven in-project: `src/platform/windows.rs:103` passes `PCWSTR(file_path.as_ptr())` into `CreateDecoderFromFilename`, the same `Param<PCWSTR>` generic shape; windows-core additionally implements it for `&BSTR`/`&HSTRING`/`PWSTR` — `windows-core-0.62.2/src/windows.rs:23/29/35`).
- Feature: `Win32_UI_Shell` exists in the resolved `windows-0.62.2/Cargo.toml:702` and is already enabled in the project (`Cargo.toml:24`).
- `shlwapi.dll` is ALREADY in the recorded 22-DLL import list (`scripts/build-windows.sh:15`, `OBSERVED_DLL_IMPORT_LIST`), so the new import does not change the DLL set; removing `CompareStringEx`/`GetUserDefaultLocaleName` (kernel32 exports) cannot remove `kernel32.dll` (still heavily imported). DLL audit expected SET-EQUAL 22.
- `Cargo.lock` records `windows 0.62.2` at `Cargo.lock:2932-2934` — package entries only; no feature data → lock untouched by the feature removal.

## §3 Code Changes (exact files, exact shapes)

### 3.1 `src/platform/windows_collation.rs` — full rewrite (49 → ~15 lines)

Replace the ENTIRE file content with:

```rust
//! Windows collation helper behind the platform facade's system entry-name
//! order: the system file manager's logical name comparison (`StrCmpLogicalW`,
//! shlwapi) -- digit-aware natural ordering (1, 2, ..., 9, 10, 11 ...) and
//! case-insensitive; the comparison never fails.

use std::cmp::Ordering;

use windows::core::PCWSTR;
use windows::Win32::UI::Shell::StrCmpLogicalW;

pub(super) fn system_collation_order(left_name: &str, right_name: &str) -> Ordering {
    let left_units: Vec<u16> = left_name.encode_utf16().chain([0]).collect();
    let right_units: Vec<u16> = right_name.encode_utf16().chain([0]).collect();
    let result = unsafe {
        StrCmpLogicalW(PCWSTR(left_units.as_ptr()), PCWSTR(right_units.as_ptr()))
    };
    result.cmp(&0)
}
```

Honored constraints: return type is `Ordering` (NOT `Option` — the comparator never fails); the `chain([0])` null terminators are REQUIRED (raw C-string API; the `Vec<u16>` buffers stay alive through the call — local variables, borrowed only inside the unsafe expression); `PCWSTR(units.as_ptr())` mirrors the existing in-project pattern at `windows.rs:100-108`; result mapped by `i32::cmp(&0)` (negative→Less, zero→Equal, positive→Greater); one function, ≤50 lines, ≤2 params, depth ≤2, no comments beyond the module doc.

### 3.2 `src/platform/windows.rs` — facade wrapper simplified (162 → ~159 lines)

- DELETE line 30 (`use crate::name_collation::compare_entry_names;` — becomes unused: the pure fallback is no longer reachable from the Windows side).
- REPLACE lines 156-162 (the doc comment + the `unwrap_or_else` fallback wrapper) with:

```rust
/// The system entry-name order (brief addendum item 8): the system file
/// manager's logical name comparison through windows_collation.
pub(crate) fn compare_entry_names_system(left_name: &str, right_name: &str) -> Ordering {
    super::windows_collation::system_collation_order(left_name, right_name)
}
```

- The public seam signature `pub(crate) fn compare_entry_names_system(&str, &str) -> Ordering` is UNCHANGED (the seam is total on both target sides after this fix). `use std::cmp::Ordering;` (line 7) STAYS (still used).
- Module doc (lines 1-5): replace the third sentence "…and the system entry-name order through windows_collation (any system-call failure falls back to the pure collation)" with "…and the system entry-name order through windows_collation (the system file manager's logical name comparison)". Keep the remaining doc lines verbatim.

### 3.3 `src/platform/mod.rs` and `src/platform/stub.rs` — UNCHANGED

`mod.rs` keeps `#[cfg(target_os = "windows")] mod windows_collation;` and both `compare_entry_names_system` re-exports. `stub.rs` keeps the pure-fallback delegation byte-identical. NO edits to either file.

### 3.4 `Cargo.toml` — feature removal only

In `[target.'cfg(windows)'.dependencies]`, delete the two lines `"Win32_Globalization",` and `"Win32_System_SystemServices",`. The list becomes exactly the original SIX: `Win32_Foundation`, `Win32_Graphics_Imaging`, `Win32_System_Com`, `Win32_UI_Shell`, `Win32_UI_Shell_Common`, `Win32_UI_WindowsAndMessaging`. NO other Cargo.toml change (version stays 0.5.2 — already bumped at Step 3). `Cargo.lock` NOT touched.

## §4 What Does NOT Change (verified; the no-touch list)

- `src/name_collation.rs` — the `EntryNameOrder` seam, `compare_entry_names`, and all 4 inline tests UNCHANGED (the two seam tests inject comparators and stay valid; the two pure-fallback tests stay).
- `src/folder_listing.rs` + `src/sibling_folder_listing.rs` — BOTH listing consumers, their constructors, the stored order, the anchor position (`listed_folder_name_precedes`), and `first_listed_image` UNCHANGED.
- The container tests: `folder_listing/collation_tests.rs` (2), `sibling_folder_listing/collation_tests.rs` (2), `folder_listing/tests.rs`, `sibling_folder_listing/tests.rs`, `neighbor_tests.rs` — all UNCHANGED; verified NO test references any removed `CompareStringEx` fact (grep across `src/`: only `windows_collation.rs` and the two doc lines being rewritten mention it).
- NO-TOUCH (standing-behavior guard): `src/app/zoom_alias.rs`, `src/app/esc_close_confirm.rs`, `src/app/esc_close_expiry_tests.rs`, `src/app/pan.rs`, `src/pan.rs`, `src/app/esc_close_confirm.rs` popup wake, ESC close contract, all pill styling (`pill_style.rs`, `floating_controls.rs`, `button_hint.rs`), `message_text` surfaces, `keyboard_intake.rs`, `fullscreen*.rs`, `zoom.rs`, `scaling.rs`, `last_viewed_images.rs`, `app/folder_navigation*`, `app_display.rs`, `central_surface.rs`, `scripts/`, `docs/`, `CHANGELOG.md` (todo-excluded), existing `.kilo/plans/*` (new plan files only), `.gitignore`, `Dockerfile`, `docker-compose.yml`.
- Census: container test census UNCHANGED at 164 tests across 26 test files (zero test delta — this fix changes NO container-testable surface: the stub path, the seam, and the consumers are identical).

## §5 Module-Doc Updates (exact target texts)

1. `src/name_collation.rs` lines 1-5: replace the trailing clause "…is the pure fallback on any system-collation failure and in container/stub builds" with "…is the pure fallback used in container/stub builds (the Windows side resolves the system comparator)". The rest of the doc stays verbatim.
2. `src/folder_listing.rs` lines 3-4: "…(the system name order; the pure collation fallback in container builds and on system failures)" → "…(the system name order on Windows; the pure fallback in container/stub builds)".
3. `src/sibling_folder_listing.rs` lines 4-6: same clause swap ("the system name order on Windows; the pure fallback in container/stub builds"). The anchor-position sentence stays verbatim.
4. `src/platform/windows.rs` module doc — §3.2 above.
5. `src/platform/windows_collation.rs` module doc — §3.1 above (part of the rewrite).
6. `.agent/project-structure.md`: update the `platform/` line (-23) to "mod.rs (facade re-exports), windows.rs (Windows implementation), windows_collation.rs (the system file manager's logical name comparison via StrCmpLogicalW — digit-aware natural ordering, case-insensitive; shlwapi), stub.rs (non-Windows fallback)"; and trim the "on system failures" clause from the `name_collation.rs` line (-11). This lands with the code commit (project-structure maintenance rule: the map must reflect the current structure at every code landing).
- Test-file references (`folder_listing/collation_tests.rs`, `sibling_folder_listing/collation_tests.rs` lines in project-structure.md) stay valid — no edits.

## §6 Tests (container-testable surface; delta = ZERO)

- The Windows `StrCmpLogicalW` implementation CANNOT run in the container (the stub path compiles) — its correctness stays OPERATOR-RUN verified (Task 4's manual-UX checklist; the operator's confirmed `(1)…(97)` natural order is the acceptance evidence). First-compile validation of the cfg(windows) code is the Task-4 release cross-build.
- The seam-level tests stay valid unchanged: the two `collation_tests.rs` modules inject comparators through `EntryNameOrder::with_comparator` — the seam mechanics are untouched; the `name_collation.rs` `system_order_matches_the_pure_fallback_for_ascii_names` test runs against the stub (pure fallback) in container builds — unchanged. (Documented caveat, NO action: if tests ever ran on a Windows host, `system()` would report `Equal` for case-only-different ASCII names under `StrCmpLogicalW`; container-only test policy is the established project stance.)
- NO new tests are added (there is no container-testable behavior change: the stub, the seam, and the consumers are byte-identical).
- Container census expectation: 164 tests / 26 files, `ALL CHECKS PASSED` — the Task-4 discovery run re-derives and pins it.

## §7 Git Segmentation (this combined cycle's commits, wave precedent)

1. `docs: add wave5.2 tasks 2-3 plan` — THIS plan file only (first commit of the 4.2 step).
2. `fix: follow the system file manager name order` — ONE feature commit: `Cargo.toml` (feature removals) + `src/platform/windows_collation.rs` (rewrite) + `src/platform/windows.rs` (§3.2) + the §5 doc updates (name_collation/folder_listing/sibling_folder_listing module docs + `.agent/project-structure.md` lines).
3. Review-fix commit(s) IF 4.3 finds issues (`fix: wave5.2 tasks 2-3 review fixes` style; max 3 review cycles, then escalate).
4. `task: mark wave5.2 tasks 2-3 done` — the shared 4.6 tick: BOTH `### Task 2:` and `### Task 3:` headings gain `[DONE]`; ALL of Task 2's sub-items (data intake, Suspect-A audit, Suspect-B audit, Suspect-C analysis, and the fallback-stop sub-item — satisfied vacuously by the decided fix, no diagnostic build needed) and Task 3's two sub-items become `[x]`. Preserve the rest of the file byte-identical (Overwrite TODO File Prevention rule). NO Task-4/Task-5 marks in this cycle.
- Gitignore compliance before every commit (`git status`; never stage `logs/`, `target/`, `dist/`); commits on `fix/wave-5-2-name-order`; push restricted to global-plan Step 5 (`main` → `origin` ONLY).

## §8 Implementation Step Order for the 4.2 Implementer

1. Commit this plan (`docs: add wave5.2 tasks 2-3 plan`).
2. Rewrite `src/platform/windows_collation.rs` per §3.1.
3. Edit `src/platform/windows.rs` per §3.2 (delete the unused import; replace the wrapper + doc).
4. Edit `Cargo.toml` per §3.4 (remove the two feature lines).
5. Apply the §5 doc updates (name_collation.rs, folder_listing.rs, sibling_folder_listing.rs module docs; `.agent/project-structure.md`).
6. Verify budgets: `wc -l src/platform/windows_collation.rs` ≈15, `src/platform/windows.rs` ≤200; no commented-out code anywhere.
7. OPTIONAL container sanity run (the AUTHORITATIVE harness run is Task 4): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` via the `alpine-vm` MCP (`vm_status` first; exit code authoritative). Expected: `ALL CHECKS PASSED`, census 164/0/26. If it fails on anything other than a mechanical typo in §3.1/§3.2, STOP and return the failure to the caller.
8. Hand off to 4.3 (code-reviewer + code-simplifier concurrently) → 4.4 (docs-specialist: light — the in-code module docs are already landed in step 5; any residue goes to Task 5) → 4.5b adherence → 4.6 mark (§7 item 4, BOTH headings).

## §9 Review Checklist (4.3)

- `grep -rn "CompareStringEx\|GetUserDefaultLocaleName\|LOCALE_NAME_MAX_LENGTH\|NORM_IGNORECASE\|Globalization\|SystemServices" src/ Cargo.toml` → ZERO hits (the Wave-5.1 mechanism is fully gone from code AND features).
- `grep -n "StrCmpLogicalW" src/platform/windows_collation.rs` → the single import + call; both `chain([0])` terminators present; `result.cmp(&0)` mapping; return type `Ordering` (no `Option`, no fallback call).
- `src/platform/windows.rs` contains NO `compare_entry_names` reference (import deleted); `src/platform/stub.rs` still delegates to `crate::name_collation::compare_entry_names` (byte-identical to before).
- `Cargo.toml` windows features = exactly the six listed in §3.4, in the listed order; `Cargo.lock` diff EMPTY for this cycle's commits (except nothing — the lock must not appear in any diff).
- The two `collation_tests.rs` modules and both listing modules show NO diff (verify with `git diff --stat` — expected files: Cargo.toml, windows_collation.rs, windows.rs, name_collation.rs, folder_listing.rs, sibling_folder_listing.rs, project-structure.md only).
- Rules: no `.gitignore`-matching files staged; single-section boolean conditions; private-by-default members (`system_collation_order` stays `pub(super)`); self-documenting names; ≤200-line files, ≤50-line methods, ≤2 params, depth ≤2.

## §10 Handoff Notes (NOT this cycle's work)

- Task 4 verification plan inputs: census 164/26 (zero delta); the wave-pin sweep = §9's greps + the no-touch list §4; release rebuild 0.5.2 exe (record bytes; no `.part`); DLL audit expected SET-EQUAL 22 (StrCmpLogicalW is a shlwapi.dll import — `shlwapi.dll` is already in `OBSERVED_DLL_IMPORT_LIST` at `scripts/build-windows.sh:15`; kernel32 stays); ANY real DLL-set delta → STOP + caller report; the §-manual-UX checklist copies the todo's Task-4 third sub-item verbatim (the name-order re-test with the operator's REAL `智乃 (1)…(97)` folder on BOTH axes + the standing regressions); the manual-UX sub-item stays `[ ]`.
- Task 5 documentation plan inputs: the root cause (lexical digit-run ordering vs the file manager's natural order), the mechanism swap (CompareStringEx→StrCmpLogicalW, shlwapi), the removed features, the moot Suspect-A line (§0), the 0.5.2 state; `tech.md` §Dependencies (eight→six features) and §Entry-Name System Order (Wave 5.1 section gains the Wave-5.2 mechanism-swap record); `architecture.md` platform/ component line; `context.md` dated entry; `brief.md` verify-only (addendum item 8 already governs — no edit expected); `.agent/project-structure.md` verify-only (updated at 4.2).
- Version: 0.5.2 already bumped (global-plan Step 3, commit `064882a`); CHANGELOG untouched per todo.
