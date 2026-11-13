# Wave 6.1 — Task 1 (Cycle A) Implementation Plan: Launcher Root Cause + Fix (D1)

- Source TODO: `.agent/todos/20261113/20261113-todo-1.md` → `### Task 1: Root-cause why the launcher never opens the file (D1)` + the `## Deferred decisions` section.
- Global plan: `.kilo/plans/20261113-wave6-1-open-with-feedback.md` → `### Cycle A — Task 1 (D1 root-cause + fix)`.
- Branch: `fix/wave-6-1-open-with-feedback` (created at workflow Step 2; Step 0 below verifies it).
- Version: 0.6.1 (already bumped at Step 3; `Cargo.toml` line 3 — NOT touched by this cycle).
- Baseline census: 177 unit tests across 28 test files (Wave-6 close, unchanged at the 0.6.1 bump). Expected census after this cycle: **UNCHANGED — 177 tests / 28 test files** (the fix is OS-bound Windows-only; the pure seam is untouched; zero test delta, Wave-5.2 precedent).
- Executor: JUNIOR developer under 50% restriction — every structural choice below is FINAL; do not redesign, rename, or relocate anything. Minor local details (exact comment-line wrapping) are the only latitude.
- Scope of THIS cycle: ONE file (`src/platform/windows_open_with.rs`). **NO Tasks 2/3/4 work** (Q hotkey, modal menu, warning color), NO enumeration-list changes, NO app/seam changes, NO docs files (Task 6).

---

## §0 SUSPECT AUDIT (the audit trail — evidence-decided, prioritized)

Operator report (0.6.0 exe, real Windows machine): selecting ANY program in the "Open with..." menu NEVER opens the file; EVERY selection shows only `OPEN_WITH_LAUNCH_FAILURE_MESSAGE`. The enumeration works (the menu lists system-associated programs). The failure is therefore in the shared LAUNCH hand-off path, common to every handler.

### Verdict summary

| Suspect | Verdict | One-line reason |
| --- | --- | --- |
| (A) `IDataObject` shape/creation | **ROOT CAUSE (primary hypothesis)** | The data object is built by requesting `IID_IDataObject` from a SHELL ITEM ARRAY object — an off-contract interface request on an object whose documented interface set is `IUnknown` + `IShellItemArray` only; it fails before `Invoke` is ever reached → `LaunchFailed` for EVERY selection, all handler kinds. |
| (B) `launch_index` pairing | Downgraded — NOT the root cause | Stored list and fresh re-enumeration come from the SAME call chain on the SAME thread; divergence requires a mid-session registration change and could only mis-launch/fail a SPECIFIC selection — it cannot produce a 100% failure across all handler kinds. |
| (C) COM state / swallowed HRESULTs | Downgraded — NOT the root cause | Same `ensure_com_initialized` helper, same UI thread as the working enumeration; windows-core 0.62.2 classifies ALL failure codes as `Err` and ALL success codes (incl. `S_FALSE`) as `Ok` — no misclassification; `Invoke` is documented to return `S_OK` on success. |
| (D) Association kinds (UWP etc.) | Downgraded as root cause — residual-failure class to watch | `Invoke` is the same method the OS Open With dialog uses for ALL handler kinds; a kind-specific limitation cannot fail 100% of selections (desktop handlers accept a single-file data object). Filtering UWP handlers would be a semantics change regressing enumeration content — forbidden. |

### Suspect (A) — THE EVIDENCE CHAIN (decisive)

1. **Landed code**: `src/platform/windows_open_with.rs:113` — `let data_object: IDataObject = SHCreateShellItemArrayFromShellItem(&shell_item)?;`
2. **windows-0.62.2 binding (verified)**: `pub unsafe fn SHCreateShellItemArrayFromShellItem<P0, T>(psi: P0) -> Result<T> where P0: Param<IShellItem>, T: Interface` (microsoft.github.io/windows-docs-rs, `windows::Win32::UI::Shell::SHCreateShellItemArrayFromShellItem`). The `IDataObject` type annotation pins `T = IDataObject` → the generated call passes `riid = IID_IDataObject`.
3. **What the function creates**: a SHELL ITEM ARRAY — MSDN (`SHCreateShellItemArrayFromShellItem`): `riid` is "typically IID_IShellItemArray"; `ppv` "typically a pointer to an IShellItemArray".
4. **The array's documented interface set**: MSDN (`IShellItemArray` interface page): "The IShellItemArray interface inherits from the IUnknown interface." Requesting `IID_IDataObject` from that object is OFF-CONTRACT. The Microsoft-documented ways to obtain an item's data object are `IShellFolder::GetUIObjectOf` and `IShellItem::BindToHandler` (MSDN "Data scenarios": "Data objects for items can be acquired through either the IShellFolder::GetUIObjectOf or IShellItem::BindToHandler methods").
5. **The canonical pattern for EXACTLY this scenario** (single file → invoke a chosen handler via `IAssocHandler::Invoke`) — verified working in multiple independent sources:
   - Simon Mourier's sample (StackOverflow 48459953 "Passing IDataObject to IAssocHandler::Invoke in C#", and simonmourier.com blog): `SHCreateItemFromParsingName(path)` → `item.BindToHandler(null, BHID_DataObject, IID_IDataObject)` → `handler.Invoke(dao)` — launches Firefox on a test file. `[STAThread]` noted — our app runs its UI thread as STA via `COINIT_APARTMENTTHREADED` (same discipline).
   - Chromium `command_execute_impl.cc`: `shell_item->BindToHandler(NULL, BHID_DataObject, IID_IDataObject, ...)` on a single item.
   - Wine shell32 test suite (`shlfolder.c`): `IShellItem_BindToHandler(psi, NULL, &BHID_DataObject, &IID_IDataObject)` → expected `S_OK` on Vista+.
   - Microsoft's own PowerToys (`PowerRenameExt.cpp`): when it needs a data object for item(s), it calls `...BindToHandler(NULL, BHID_DataObject, IID_IDataObject, ...)` on the item array — i.e., BindToHandler, never a direct array→`IDataObject` request.
6. **The failure mode matches the operator report EXACTLY**: if the off-contract `IID_IDataObject` request fails (typically `E_NOINTERFACE`), `file_data_object` returns `Err` at `windows_open_with.rs:113` BEFORE `handler.Invoke` is reached (the `and_then` at lines 48–49 short-circuits) → `OpenWithLaunchFailure::LaunchFailed` for EVERY selection, regardless of handler kind, while the enumeration (which never touches the data object) keeps working → the menu lists programs but every selection fails. 100% failure, all programs, every file type — the D1 report.

Even under the alternative branch (the array object unexpectedly answering `IID_IDataObject` via an undocumented implementation detail), the current mechanism has no documented support and has already failed at runtime on the operator machine — the replacement below is the canonical fix in BOTH branches.

### Suspect (B) — `launch_index` pairing (evidence: downgraded)

- The stored menu list is produced at the popup-opening click frame by `platform::enumerate_open_with_programs(&opened_path)` (`src/app/open_with_button.rs:97–109`).
- The launcher calls `platform::open_file_with_selected_program(&opened_path, program_launch_index)` (`src/app/open_with_button.rs:174–181`), which re-enumerates via the SAME `raw_handlers(file_path)` helper (`src/platform/windows_open_with.rs:34` and `:44`), the SAME extension query, on the SAME egui UI thread.
- Both lists therefore derive from the same system query in the same session; the shell association enumeration order is stable within a session. Divergence requires the system's registration set to change between menu-open and selection-click (an installer/Store update mid-menu); that could mis-launch or fail ONE selection — it cannot explain EVERY selection failing across ALL handler kinds. NOT the root cause; no fix candidate (the pairing stays as-is).

### Suspect (C) — COM state / HRESULT classification (evidence: downgraded)

- `ensure_com_initialized` (`src/platform/windows.rs:45–51`): `CoInitializeEx(None, COINIT_APARTMENTTHREADED)` → `S_OK | S_FALSE | RPC_E_CHANGED_MODE` all count as initialized. The invoke path runs on the egui UI thread (the click handler executes inside the frame update) — the SAME thread where the enumeration already succeeded via the same helper (the menu lists programs, which requires COM init success). COM state at invoke time is therefore healthy; not the failure.
- windows-core 0.62.2 `HRESULT` semantics (docs.rs `windows-result 0.4.1` `hresult.rs`): `ok()` converts to `Err` ONLY for failure codes (`is_err`); success codes (`S_OK`, `S_FALSE`, any non-negative) classify as `Ok` — NO success-code misclassification. `IAssocHandler::Invoke` is documented to return `S_OK` on success (MSDN). NOT the root cause.

### Suspect (D) — Association kinds / UWP handlers (evidence: downgraded as root cause)

- `SHAssocEnumHandlers` with `ASSOC_FILTER_NONE` may return UWP/Store handlers — IN BY DESIGN (the Wave-6 Decision 1 semantics; the TODO forbids regressing enumeration list content).
- `IAssocHandler::Invoke` is the SAME launch method the OS Open With dialog itself uses for all handler kinds (MSDN Invoke remarks: "IAssocHandler objects are typically used to populate an Open With menu. When one of those menu items is selected, this method is called to launch the chosen application."; "It is generally safe to assume that an application will support invocation on a single item").
- A handler-kind-specific limitation cannot produce a 100% failure across every kind (desktop handlers such as Paint/Notepad accept a single-file data object). The universal failure implicates the shared construction step — suspect (A). NO filtering change (a semantics change needing caller approval AND a forbidden enumeration regression). UWP-handler behavior is a residual class to observe in the Task-5 operator run, not a fix here.

### Research question settled (caller's key question)

**What data object do registered handlers accept on `IAssocHandler::Invoke`, and what is the canonical Windows way to invoke a handler on ONE file?** Answer, with cited evidence: an `IDataObject` representing the item(s) (MSDN Invoke parameter doc: "A pointer to an IDataObject that represents the selected item on which to invoke the handler"). For a single file the canonical, documented acquisition is `IShellItem::BindToHandler(NULL, BHID_DataObject, IID_IDataObject)` on the `IShellItem` created from the path — verified working end-to-end for handler invocation by the cited sample, Chromium, Wine tests, and Microsoft's own code. `SHCreateDataObject` (pidl-folder route, `shlobj_core.h`) is an alternative but requires parent-folder pidls — more plumbing for the identical result; rejected as non-minimal.

---

## §1 THE FIX (Decision 1 — settled here; NO caller approval required)

**Chosen fix**: build the launch data object with the documented bind-handler route — drop the shell-item-array step entirely and bind `BHID_DataObject` directly on the single-file `IShellItem`:

```rust
let data_object: IDataObject = shell_item.BindToHandler(None::<&IBindCtx>, &BHID_DataObject)?;
```

- This is an API-USAGE change ONLY — **no semantics change**: the enumeration list content (incl. UWP inclusion via `ASSOC_FILTER_NONE`), the two-point refresh contract, the `launch_index` pairing, the state gates, the failure message constant, and every standing behavior are UNCHANGED. Therefore **NO caller approval is needed** and **NO diagnostic debug build is needed** (the audit DECIDED the failure; §6 encodes the fallback trigger for the only residual uncertainty, the operator-run confirmation, which is the standing Task-5 gate — the Wave 5.1/5.2 precedent).
- `Invoke`'s own doc warns against multi-item selections; a ONE-item data object is the supported case — the single-item bind route is also the semantically right shape, not just the working one.
- The removed `SHCreateShellItemArrayFromShellItem` import leaves NO unused-import residue (clippy `-D warnings` would fail otherwise).

**Exact windows-0.62.2 bindings the fix relies on (verified this session)**:

```rust
// windows::Win32::UI::Shell — constant exists, behind the ALREADY-ENABLED Win32_UI_Shell feature:
pub const BHID_DataObject: GUID;

// windows::Win32::UI::Shell — method (microsoft.github.io/windows-docs-rs, struct.IShellItem.html):
impl IShellItem {
    pub unsafe fn BindToHandler<P0, T>(&self, pbc: P0, bhid: *const GUID) -> Result<T>
    where P0: Param<IBindCtx>, T: Interface,
}
//    call: shell_item.BindToHandler(None::<&IBindCtx>, &BHID_DataObject)
//    T inferred = IDataObject (the annotated let binding) → riid = IID_IDataObject. Correct cast path.

// windows::Win32::System::Com::IBindCtx — behind the ALREADY-ENABLED Win32_System_Com feature.

// windows::Win32::UI::Shell — invoke (UNCHANGED landed call):
impl IAssocHandler {
    pub unsafe fn Invoke<P0>(&self, pdo: P0) -> Result<()> where P0: Param<IDataObject>,
}
```

- `SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None)` is UNCHANGED landed, compile-verified code (0.6.0 build exit 0; binding pinned in the Wave-6 plan Decision 2).
- **Crate features ADDED: NONE. `Cargo.toml`/`Cargo.lock` NOT edited.** (`BHID_DataObject` and `SHCreateShellItemArrayFromShellItem` are both in `Win32_UI_Shell`; `IBindCtx` in `Win32_System_Com` — all already enabled.)
- **DLL audit expectation**: SET-EQUAL 22 — `BindToHandler` is a virtual call through the already-imported shell COM surface; the removed `SHCreateShellItemArrayFromShellItem` and the kept `SHAssocEnumHandlers`/`SHCreateItemFromParsingName` are all `shell32.dll`. The full audit re-check stays Task-5 (Cycle C) scope.

**Rejected candidates (do not revisit)**:
- *Keep the array, add `array.BindToHandler(BHID_DataObject)`*: works for flat arrays but the array page carries a usage caveat for `BHID_DataObject`, while the single-item route is the verified end-to-end pattern for exactly one file; the array step is now dead weight. Rejected.
- *`SHCreateDataObject` (pidlFolder + child pidls)*: needs parent-folder pidl plumbing for the identical result. Rejected (non-minimal).
- *`ShellExecuteExW` fallback / `IApplicationActivationManager` for UWP*: composes launch knowledge app-side (against the no-internal-database contract) and cannot honor a user-SELECTED non-default program. Rejected.
- *Filter UWP handlers out of the list*: semantics change + forbidden enumeration regression. Rejected.
- *Diagnostic debug build now*: the audit decided the failure from documented evidence; the fallback stays encoded (§6) with a caller-authorization trigger. Not implemented in this cycle.

---

## §2 LANDED FACTS THIS PLAN BUILDS ON (verified at HEAD)

| Fact | Source (verified) |
| --- | --- |
| Landed launcher: `raw_handlers` fresh re-enumeration → `handlers.get(program_index)` → `file_data_object(...).and_then(\|d\| handler.Invoke(&d))` → `Err → LaunchFailed` | `src/platform/windows_open_with.rs:39–51` |
| The off-contract construction: `let data_object: IDataObject = SHCreateShellItemArrayFromShellItem(&shell_item)?;` | `src/platform/windows_open_with.rs:109–116` |
| Enumerated-only call sites of the facade (menu wiring, unchanged by this fix) | `src/app/open_with_button.rs:97–109` (enumerate on popup-open), `:174–181` (launch by `launch_index`) |
| Pure seam UNTOUCHED: `RegisteredProgram { display_name, launch_index }`, dedup/filter, failure enum + constant | `src/open_with_listing.rs` |
| COM helper (shared, unchanged) | `src/platform/windows.rs:45–51` |
| Facade re-exports (unchanged) | `src/platform/mod.rs:30–35` |
| Stub path (unchanged; covers the failure transition in container tests) | `src/platform/stub.rs:30–36` |
| `windows = "=0.62.2"` with EXACTLY the six features incl. `Win32_System_Com` + `Win32_UI_Shell` | `Cargo.toml:17–25` |
| Version 0.6.1 already bumped | `Cargo.toml:3` |
| Harness: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` (fixed order fmt-check/check/test/clippy; final line `ALL CHECKS PASSED` or `N CHECK(S) FAILED`; logs under gitignored `logs/checks/`) | `docs/build.md` |
| Windows cross-build: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` → `dist/windows-x64/img-scroller.exe` (atomic overwrite) | `docs/build-windows.md` |
| `.gitignore` covers `dist/`, `target/`, `logs/`, `*.log` | `.gitignore:17,31,32` |

---

## §3 IMPLEMENTATION STEPS (4.2 implementer — tiny steps, in order)

### Step 0 — Pre-flight (NO file changes)

1. Call `alpine-vm_vm_status` FIRST. If the VM or the `/img-scroller` share is unavailable → STOP and return a user question (per `docs/build.md`).
2. On the host: `git status` (bash tool, single commands only) — verify: current branch `fix/wave-6-1-open-with-feedback`, working tree clean, version 0.6.1 in `Cargo.toml`. If the branch is not checked out → STOP and return a question to the caller (branch creation is Step-2-restricted).
3. Read `.gitignore` (already listed in §2) — required by the Gitignore Compliance Rule before every commit below.

### Step 1 — Commit the plan file (workflow-precedent plan commit)

- Stage ONLY `.kilo/plans/20261113-wave6-1-task1-launcher-fix-implementation-plan.md` (this file).
- `git status` check: nothing gitignore-matching staged; no other files staged.
- Commit message: `docs: add wave6-1 task1 launcher-fix implementation plan`

### Step 2 — The code edit (EXACTLY these three sub-edits in `src/platform/windows_open_with.rs`; NO other file)

**2a. Import block swap** — replace the current import lines (file lines 21–25):

```rust
use windows::Win32::System::Com::{CoTaskMemFree, IDataObject};
use windows::Win32::UI::Shell::{
    ASSOC_FILTER_NONE, IAssocHandler, IEnumAssocHandlers, IShellItem, SHAssocEnumHandlers,
    SHCreateItemFromParsingName, SHCreateShellItemArrayFromShellItem,
};
```

with:

```rust
use windows::Win32::System::Com::{CoTaskMemFree, IBindCtx, IDataObject};
use windows::Win32::UI::Shell::{
    ASSOC_FILTER_NONE, BHID_DataObject, IAssocHandler, IEnumAssocHandlers, IShellItem,
    SHAssocEnumHandlers, SHCreateItemFromParsingName,
};
```

(`SHCreateShellItemArrayFromShellItem` is REMOVED — an unused import fails clippy `-D warnings`.)

**2b. `file_data_object` body swap** — replace (file lines 109–116):

```rust
unsafe fn file_data_object(file_path: &Path) -> windows::core::Result<IDataObject> {
    unsafe {
        let wide_path: Vec<u16> = file_path.as_os_str().encode_wide().chain([0]).collect();
        let shell_item: IShellItem = SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None)?;
        let data_object: IDataObject = SHCreateShellItemArrayFromShellItem(&shell_item)?;
        Ok(data_object)
    }
}
```

with:

```rust
unsafe fn file_data_object(file_path: &Path) -> windows::core::Result<IDataObject> {
    unsafe {
        let wide_path: Vec<u16> = file_path.as_os_str().encode_wide().chain([0]).collect();
        let shell_item: IShellItem = SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None)?;
        let data_object: IDataObject =
            shell_item.BindToHandler(None::<&IBindCtx>, &BHID_DataObject)?;
        Ok(data_object)
    }
}
```

Everything else in the function stays byte-identical (the `wide_path` build, the `SHCreateItemFromParsingName` call with bare `None`, the `Ok(data_object)` return). The invoke call site at lines 48–49 (`handler.Invoke(&data_object)`) stays byte-identical.

**2c. Module header doc sentence** — the target header line is file line 9 (line 8 shown as context):

```
//! across calls — and launches the selected handler via IAssocHandler::Invoke
//! with an IDataObject built from the current canonical file. COM init reuses
```

Replace the line `//! with an IDataObject built from the current canonical file. COM init reuses` with EXACTLY this block (the following line `//! the shared ensure_com_initialized discipline from windows.rs; every failure` stays untouched, with its leading `the` reading naturally after `COM init reuses`):

```
//! with an IDataObject built from the current canonical file via
//! IShellItem::BindToHandler(BHID_DataObject) — the documented way to obtain
//! an item's data object; the removed SHCreateShellItemArrayFromShellItem
//! route requested IID_IDataObject off-contract from a shell item array and
//! failed on real Windows (Wave 6.1 D1 root cause — see
//! .kilo/plans/20261113-wave6-1-task1-launcher-fix-implementation-plan.md
//! §0/§1). COM init reuses
```

(Keep the existing final header sentences about `ensure_com_initialized` and the 20261112 plan reference untouched.)

Expected file size after the edit: ~119 lines (well under the 200-line file cap; `file_data_object` keeps 1 param and stays short — max-arguments/max-lines rules hold).

### Step 3 — Harness round 1: fmt/check/test/clippy (dev-checks)

1. `vm_status` first (again, immediately before running).
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
3. Authoritative evidence: the command EXIT CODE. Expected: exit `0`, final line `ALL CHECKS PASSED`, per-check `PASS` lines for `fmt-check`, `check`, `test`, `clippy`; test census line corroborates **177 passed; 0 failed** (census UNCHANGED).
4. If `fmt-check` FAILS on the import reordering/wrapping: apply `cargo fmt` (the one allowed fix), then re-run the SAME dev-checks command. Any OTHER failure: fix per this plan (compile/clippy residual minimal fixes, one-more-attempt cap per harness precedent) and re-run.
5. If a compile error appears around `BindToHandler`/`None::<&IBindCtx>` inference (NOT expected — both bound shapes verified): STOP and return the exact compiler output to the caller; do NOT redesign the call.

### Step 4 — Windows-target compile evidence (BOTH-target requirement)

1. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh`
2. Authoritative evidence: exit `0`; output states the artifact path `dist/windows-x64/img-scroller.exe` and that it is non-empty. The rebuild atomically overwrites the 0.6.0-era artifact — expected; Task 5 (Cycle C) re-records the byte size.
3. DLL audit: NOT re-run in this cycle (Task-5 scope). Record only the expectation: no new DLL class (`BindToHandler` = vtable call; shell COM surface unchanged — see §1).
4. A windows-target build failure here (e.g. a mingw-side symbol/link issue with `BHID_DataObject`) → capture the full diagnostics and STOP with a caller question; do not improvise alternatives.

### Step 5 — Commit the fix

1. `git status` — verify ONLY `src/platform/windows_open_with.rs` is modified (`git diff --stat` must show exactly that one file). `logs/`, `dist/`, `target/` are gitignored and must NOT appear staged.
2. Stage ONLY `src/platform/windows_open_with.rs`.
3. Commit message: `fix: build the open-with launch data object with the documented bind handler`
4. Re-run `git status` to confirm a clean tree afterwards.

### Step 6 — Scope-boundary self-check (all must hold before declaring the cycle implemented)

- `Cargo.toml` and `Cargo.lock`: byte-identical to the wave Step-3 version-bump state (0.6.1, no new features, no lock changes).
- `src/open_with_listing.rs`, `src/open_with_listing/tests.rs`, `src/app/open_with_button.rs`, `src/platform/mod.rs`, `src/platform/windows.rs`, `src/platform/stub.rs`: byte-identical.
- No new app-state fields; no new messages; no UI/intake/styling changes (Tasks 2/3/4 untouched).
- Enumeration list content, `ASSOC_FILTER_NONE`, UWP inclusion, the two-point refresh contract, the `launch_index` pairing, the state gates, and `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` — ALL unchanged.
- No diagnostic/debug build exists in the tree.

---

## §4 TESTS

- **Zero new unit tests; census stays pinned at 177/28.** The change is Windows-OS-bound; the pure seam (`open_with_listing.rs`) is untouched; the stub path (`stub.rs:30–36`) already drives the failure transition in container tests. Wave-5.2 precedent (mechanism swap, zero test delta).
- Container-testable surface verification = the dev-checks census corroboration in Step 3 (`177 passed; 0 failed`).
- The REAL verification of D1 is the Task-5 OPERATOR-RUN on the rebuilt 0.6.1 exe (standing gate; checklist lands in Cycle C, presented plainly per the operator's clarity requirement).

---

## §5 REVIEWER / SIMPLIFIER / ADHERENCE HOOKS (4.3 + 4.5b)

- Reviewer verifies: (1) the diff touches ONLY `windows_open_with.rs`; (2) the two code swaps match §3 exactly (imports + `BindToHandler` body); (3) `SHCreateShellItemArrayFromShellItem` has NO remaining reference in the file; (4) no comments were left commented-out code (no-commented-code rule); (5) census 177/28 unchanged in the dev-checks log.
- Simplifier expected verdict: NO-CHANGE (the fix is already minimal — one call replacing one call).
- 4.5b adherence checklist: the §6 scope-boundary self-check items + the evidence citations (§0/§1) reflected in the module header doc (Step 2c).
- 4.4 docs-specialist: module-header coherence only; tech.md/architecture.md/context.md wave-level documentation is Task 6 (Cycle D).

---

## §6 RUNTIME-VERIFICATION BOUNDARY + DIAGNOSTIC FALLBACK TRIGGER

- This fix is compile-verified agent-side (both targets, Step 3 + Step 4). Runtime confirmation happens ONLY in the Task-5 OPERATOR-RUN on the real Windows machine (the standing Wave-6.1 acceptance gate) — NOT in this cycle.
- **FALLBACK TRIGGER (caller authorization required)**: if the operator reports on the 0.6.1 exe that selections STILL never open the file (the same 100% failure), the next step is the TODO's DIAGNOSTIC fallback — a temporary DEBUG-ONLY build displaying the per-selection invoke outcomes (per-selection result codes on the pill's message line). This plan does NOT implement it. The implementer/architector of that follow-up MUST obtain explicit caller authorization BEFORE writing any diagnostic build; it is never an assumption and never ships in a release artifact.

---

## §7 SOURCES CITED (consulted + verified 2026-11-13)

- windows-rs 0.62.2 projections (canonical docs):
  - `https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.IShellItem.html` — `BindToHandler<P0, T>(&self, pbc: P0, bhid: *const GUID) -> Result<T> where P0: Param<IBindCtx>, T: Interface`
  - `https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/constant.BHID_DataObject.html` — `pub const BHID_DataObject: GUID` in `windows::Win32::UI::Shell`
  - `https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.IAssocHandler.html` — `Invoke<P0>(&self, pdo: P0) -> Result<()> where P0: Param<IDataObject>`
  - `https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/fn.SHCreateShellItemArrayFromShellItem.html` — `SHCreateShellItemArrayFromShellItem<P0, T>(psi: P0) -> Result<T> where P0: Param<IShellItem>, T: Interface` (proves the landed call passes `riid = IID_IDataObject`)
- Windows API contracts (Microsoft Learn):
  - `IAssocHandler::Invoke` (`shobjidl_core.h`): `pdo` = "an IDataObject that represents the selected item"; S_OK on success; single-item invocation is the safe assumption; the OS Open With menu launches via this method.
  - `SHCreateShellItemArrayFromShellItem`: `riid` "typically IID_IShellItemArray"; creates a one-element array.
  - `IShellItemArray` interface: "inherits from the IUnknown interface" (IDataObject is NOT part of the documented contract).
  - `IShellItem::BindToHandler` / `IShellItemArray::BindToHandler`: `BHID_DataObject` = "Gets an IDataObject object for use with an item or an array of items."
  - "Data scenarios" shell topic: "Data objects for items can be acquired through either the IShellFolder::GetUIObjectOf or IShellItem::BindToHandler methods."
  - `SHCreateDataObject` (`shlobj_core.h`): the pidl-folder alternative (rejected as non-minimal).
- Verified working pattern for this exact scenario:
  - StackOverflow 48459953 / simonmourier.com — "Passing IDataObject to IAssocHandler::Invoke" (SHCreateItemFromParsingName → `BindToHandler(null, BHID_DataObject, IID_IDataObject)` → `handler.Invoke(dao)`; `[STAThread]` = the STA discipline our app already keeps).
  - Chromium `win8/delegate_execute/command_execute_impl.cc` — `shell_item->BindToHandler(NULL, BHID_DataObject, IID_IDataObject, ...)` on a single item.
  - Microsoft PowerToys `PowerRenameExt.cpp` — data object obtained via `BindToHandler(NULL, BHID_DataObject, IID_IDataObject, ...)` (never a direct array→`IDataObject` request).
  - Wine shell32 tests (`shlfolder.c`) — `IShellItem_BindToHandler(psi, NULL, &BHID_DataObject, &IID_IDataObject)` expected `S_OK` on Vista+ (XP-broken `MK_E_NOOBJECT`).
- windows-core 0.62.2 `HRESULT` semantics: `https://docs.rs/windows-core/0.62.2/windows_core/struct.HRESULT.html` (backed by `windows-result 0.4.1` `hresult.rs`) — `ok()` errs ONLY on failure codes; success codes (incl. `S_FALSE`) classify as `Ok`.
