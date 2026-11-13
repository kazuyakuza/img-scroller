# Wave 6 — Tasks 1+2 (Cycle A) Implementation Plan: Runtime Open-with Enumeration + Selected-Program Launcher

- Source TODO: `.agent/todos/20261029/20261029-todo-8.md` → `### Task 1: Runtime program enumeration` + `### Task 2: Open the current file with the selected program` (grouped cycle A per the global plan).
- Global plan: `.kilo/plans/20261112-wave6-open-with.md` → `### Cycle A — TODO Tasks 1+2`.
- Scope of THIS cycle: the OS-bound enumeration service, the launcher hand-off, the pure testable seam, and the platform facade/stub wiring. **NO menu UI, NO app-state wiring, NO docs files** (Task 3 = UI cycle B; Task 4 = verification cycle C; Task 5 = docs cycle D).
- Branch: `feat/wave-6-open-with` (already created at workflow Step 2; verify with `git status` — currently clean, HEAD on the branch).
- Version: 0.6.0 (already bumped at Step 3; `Cargo.toml` line 3 — untouched by this cycle).
- Baseline census: 164 unit tests across 26 test files (`tech.md` § Testing). Expected census after this cycle: **170 tests across 27 test files** (+6 new tests in one new test module).
- Executor: JUNIOR developer under 50% restriction — every structural choice below is FINAL; do not redesign, rename, or relocate anything. Minor local details (exact variable spellings not pinned here) are the only latitude.

---

## §0 Landed Facts This Plan Builds On (verified at HEAD)

| Fact | Source (verified) |
| --- | --- |
| cfg-gated `platform` facade: `mod.rs` re-exports `decode_image`, `show_open_image_dialog`, `compare_entry_names_system` from `windows`/`stub` per target | `src/platform/mod.rs` (24 lines) |
| Real Windows impl file + the sibling-file precedent (`windows_collation.rs` added in Wave 5.2 rather than growing `windows.rs`) | `src/platform/windows.rs` (159 lines), `src/platform/windows_collation.rs` |
| Stub fallback pattern: dialog → `None`, decode → `Err(Unloadable)`, name order → pure fallback | `src/platform/stub.rs` (21 lines) |
| COM init helper reused by both facade entries: `ensure_com_initialized()` (`CoInitializeEx` + `S_OK/S_FALSE/RPC_E_CHANGED_MODE`) — private fn at `windows.rs:45` | `src/platform/windows.rs:45–51` |
| Comparator-as-data seam precedent (`EntryNameOrder::system()` resolves the platform fn; `with_comparator` injects synthetic comparators in container tests) | `src/name_collation.rs` |
| Pure-listing precedent with a child test module | `src/folder_listing.rs` + `src/folder_listing/tests.rs` |
| Failure-enum + single static message precedent (`ImageLoadFailure`, `UNSUPPORTED_FILE_MESSAGE`, `user_message()`) | `src/image_loading.rs` |
| Targeted `#[allow(dead_code)]` precedent for `pub(crate)` items not yet consumed | `src/image_loading.rs:28,35,47`, `src/name_collation.rs:34` |
| The canonical open path: `open_image` canonicalizes once (`fs::canonicalize`) → `opened_image_path` | `src/app.rs:72–76` |
| The app's ONLY message surface: `message_text: Option<&'static str>` (static strings only) | `src/app.rs:29`, `src/image_loading.rs:10` |
| `windows` crate =0.62.2 with EXACTLY SIX features (`Win32_Foundation`, `Win32_Graphics_Imaging`, `Win32_System_Com`, `Win32_UI_Shell`, `Win32_UI_Shell_Common`, `Win32_UI_WindowsAndMessaging`) | `Cargo.toml:17–25`, `tech.md` § Dependencies |
| Module registration point | `src/main.rs` (`mod` declarations) |
| Harness: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` on the Alpine VM (`vm_status` FIRST; exit code authoritative) | `docs/build.md`, `tech.md` § Build and Run |

---

## §1 THE FOUR DEFERRED DECISIONS (all settled here — research-backed)

### Decision 1 — Enumeration mechanism: `SHAssocEnumHandlers` → `IEnumAssocHandlers` → `IAssocHandler::GetUIName`

**Chosen**: the shell association-handler enumeration API family from `windows::Win32::UI::Shell`.

At runtime, for the CURRENT displayed file's extension, the app calls `SHAssocEnumHandlers` with the extension string (`".jpg"`, `".png"`, …) and `ASSOC_FILTER_NONE`. The returned `IEnumAssocHandlers` yields `IAssocHandler` COM objects — the exact handlers the Windows system has registered for that extension (the same data Explorer's "Open with" list shows, drawn from the system's association registration: default associations, `OpenWithList`/`OpenWithProgids` registration data, and Store-app package associations). Each handler's display name is read via `IAssocHandler::GetUIName` (MSDN: "Retrieves the display name of an application"). **The menu list is built from that data ONLY — zero internal/hardcoded program database, zero ProgId table, zero command-line storage.**

**Exact APIs used** (windows crate `=0.62.2`; all already behind the enabled `Win32_UI_Shell` feature):

```rust
// windows::Win32::UI::Shell  (verified against windows-docs-rs for the windows crate)
pub unsafe fn SHAssocEnumHandlers<P0>(pszextra: P0, affilter: ASSOC_FILTER) -> Result<IEnumAssocHandlers>
    where P0: Param<PCWSTR>;                 // pszextra = ".ext"; affilter = ASSOC_FILTER_NONE

impl IEnumAssocHandlers {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IAssocHandler>], pceltfetched: Option<*mut u32>) -> Result<()>;
    // celt is implicit in the slice length; with a 1-element slice pceltfetched may be None (MSDN: allowed when celt == 1)
}

impl IAssocHandler {
    pub unsafe fn GetUIName(&self) -> Result<PWSTR>;   // display name; CoTaskMemAlloc'd → must CoTaskMemFree
}
```

Constant: `ASSOC_FILTER_NONE` (enum `ASSOC_FILTER`, `windows::Win32::UI::Shell`). String memory: the `PWSTR` returned by `GetUIName` is freed with `CoTaskMemFree(Some(ptr.cast::<core::ffi::c_void>()))` — identical to the landed `selected_dialog_path` pattern (`windows.rs:72–82`). COM initialization reuses `ensure_com_initialized()` (§3 step 3). Result handling uses `windows::core::Result`/`Error` exactly like the landed WIC code.

**Crate features ADDED: NONE.** `SHAssocEnumHandlers`, `IEnumAssocHandlers`, `IAssocHandler`, `ASSOC_FILTER`, `SHCreateItemFromParsingName`, `SHCreateShellItemArrayFromShellItem` all live in `Win32_UI_Shell` (enabled since Wave 1); `IDataObject` and `CoTaskMemFree` live in `Win32_System_Com` (enabled since Wave 1). `Cargo.toml` is NOT edited. Consequence for the Task-4 DLL audit: the new calls add direct `shell32.dll` imports (the previously landed shell COM usage resolved everything through `CoCreateInstance`, so `shell32.dll` was not necessarily a direct import — the audit is expected to record a delta; Task 4 accounts for it as an "explained delta" and Task 5 updates the OBSERVED list in `docs/build-windows.md`). This note is repeated in §5.

**Rejected candidates** (with reasons — do not revisit):
- *Registry enumeration* of `HKCR\.<ext>\OpenWithList`/`OpenWithProgids` + `HKCU\...\Explorer\FileExts\.<ext>` (needs the new `Win32_System_Registry` feature): requires manual ProgId→friendly-name/command resolution (MUIVerb indirection, `%1` substitution), NEVER sees UWP/Store-app registrations (they live outside HKCR OpenWith data), and would push the binary toward storing launch commands — against the "no internal program database" requirement. Rejected.
- *`AssocQueryStringW`/`IQueryAssociations*`*: resolves only the DEFAULT association's data — a single program, not the registered set. Cannot build a list. Rejected.

**Sources cited** (consulted for this decision):
- `https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.IAssocHandler.html` — exact 0.62 projections: `GetName() -> Result<PWSTR>`, `GetUIName() -> Result<PWSTR>`, `Invoke(pdo: Param<IDataObject>) -> Result<()>`, `IsRecommended() -> HRESULT`.
- `https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/fn.SHAssocEnumHandlers.html` — exact signature above.
- `https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.IEnumAssocHandlers.html` — exact `Next` projection.
- `https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/fn.SHCreateItemFromParsingName.html` and `.../fn.SHCreateShellItemArrayFromShellItem.html` — exact signatures (used by Decision 2).
- `https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-iassochandler` — method semantics: `GetUIName` = display name, `Invoke` = "Directly invokes the associated handler".
- `https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shassocenumhandlers` — shell32, `shobjidl_core.h`, Windows Vista+ (Windows 10/11 covered).

**Ambiguity settled (per instruction 5 — DECIDED, not punted): UWP/Store apps are INCLUDED.** `ASSOC_FILTER_NONE` means "no filter" — the enumeration returns everything the system registered, including Store-app handlers on Windows 10/11. Rationale: the brief says the list is built from system registration data ONLY; applying an app-side filter would be the app maintaining opinion about the system's data. If a Store app is registered for `.png` (e.g. Photos), it appears in the list with its system display name, and `IAssocHandler::Invoke` launches it correctly (the shell composes the launch — including Store-app activation — internally; the app never composes anything).

### Decision 2 — Launcher mechanism: `IAssocHandler::Invoke` with a file-path `IDataObject`

**Chosen**: the system-blessed handler invocation. The launcher re-runs the SAME fresh enumeration, selects the handler at the given index, builds an `IDataObject` for the CURRENT displayed file, and calls `handler.Invoke(&data_object)` — the shell then composes and executes the launch (classic ProgId command lines with `%1`, DDE, and Store-app activation all handled by the system; the app hands off a file path, never a command).

**Exact API chain** (windows crate `=0.62.2`):

```rust
// 1. Shell item for the CURRENT file path (the canonical `opened_image_path`):
pub unsafe fn SHCreateItemFromParsingName<P0, P1, T>(pszpath: P0, pbc: P1) -> Result<T>
    where P0: Param<PCWSTR>, P1: Param<IBindCtx>, T: Interface;
//    call: SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None)  → T = IShellItem
//    (landed code already passes bare `None` for optional shell params — windows.rs:101; if inference
//     demands an annotation, the compiler-suggested `None::<&IBindCtx>` is the same decision, not a new one)

// 2. Wrap the single shell item into an item array exposed as IDataObject:
pub unsafe fn SHCreateShellItemArrayFromShellItem<P0, T>(psi: P0) -> Result<T>
    where P0: Param<IShellItem>, T: Interface;
//    call: SHCreateShellItemArrayFromShellItem(&shell_item)  → T = IDataObject

// 3. Invoke the SELECTED handler with the file's data object:
pub unsafe fn Invoke(&self, pdo: P0) -> Result<()> where P0: Param<IDataObject>;
//    call: handler.Invoke(&data_object)
```

`IDataObject` is imported from `windows::Win32::System::Com` (feature enabled). `IShellItem` is already imported by `windows.rs`.

**Hand-off contract (Task 2 verbatim compliance)**: the launcher receives the CURRENT displayed file's path — on the app side that is the canonical `opened_image_path` (`app.rs:74` single-canonicalization point). Task 3's menu wiring will pass `opened_image_path` verbatim; the facade itself takes any `&Path` and derives the extension internally.

**Selection payload**: the pure data type carries the handler's ordinal position (`launch_index`, §3 step 1) in the RAW system enumeration. The launcher re-enumerates fresh and indexes the RAW handler list, so the pair (enumerate → list the names, click → launch by index) is internally consistent. Accepted edge (documented, same class as any snapshot-then-act UI): if registration data changes between menu-open and click, the click either launches the handler now at that index or resolves out of range → failure value. Display names are NOT matched (duplicate display names across distinct handlers would make name-matching ambiguous; ordinals cannot collide).

**Error surfacing**: launcher failures (COM init failure, enumeration failure, index out of range, `Invoke` error) collapse into the new failure enum `OpenWithLaunchFailure::LaunchFailed`, whose `user_message()` returns the static constant `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` (§3 step 1) — mirroring `ImageLoadFailure` exactly, because the app's ONLY message surface is `message_text: Option<&'static str>` (static strings only). **Wiring that message into the message line happens in Task 3 (UI cycle); this cycle defines the constant + enum only.** Enumeration failures are NOT surfaced as errors: `enumerate_open_with_programs` returns an EMPTY `Vec` on any failure (no registered programs, no extension, COM failure) — the empty-result UX decision belongs to Task 3.

**Rejected candidates** (with reasons — do not revisit):
- *`ShellExecuteExW` with the file path*: opens the file with the DEFAULT handler — it cannot honor a user-selected OTHER program. Rejected.
- *Composing the ProgId command line* (exe path + quoted file arg): would require storing/composing launch commands (against "no internal program database"), breaks UWP handlers (no exe path), and diverges from what the system itself does for DDE/verb-based registrations. Rejected.
- *`IAssocHandler::GetName`* (the full exe path): NOT used anywhere — the app never composes commands; only `GetUIName` (display) and `Invoke` (launch) are consumed.

### Decision 3 — Refresh/re-open timing: PER-INVOCATION FRESH ENUMERATION (no session cache)

Every call to `enumerate_open_with_programs` queries the shell fresh; the launcher ALSO re-enumerates fresh internally at launch time. No cached program list is stored in app state (no new session field), no cache invalidation exists, and nothing persists.

Rationale (encoded, final):
- Registrations stay current within a session: a program installed while the app runs appears on the next menu open; a program uninstalled disappears.
- Zero new app-state fields — the smallest possible Task-3 wiring surface (the global plan pins "NO app-state wiring beyond what the plan specifies").
- Shell enumeration cost is negligible at menu-open frequency; freshness is a feature, not a risk.
- The launcher's internal re-enumeration guarantees the launch index maps onto a list produced by the SAME query, so no stale-handler COM objects are ever held across frames.

### Decision 4 — Testable seam: pure `open_with_listing` module fed by raw handler names

**Raw-result representation** (the injection seam — synthetic fixtures replace the OS query, mirroring the `EntryNameOrder::with_comparator` precedent where container tests inject synthetic comparators):

- The OS-bound query yields `Vec<String>` — the handlers' UI display names in the SYSTEM's enumeration order (order preserved verbatim; the system order is the Explorer-consistent order, and no re-sorting is defined).
- The PURE function `programs_from_handler_names(handler_names: &[String]) -> Vec<RegisteredProgram>` turns raw names into the menu-list data. Container tests feed synthetic `Vec<String>` fixtures directly — no registry, no shell, no COM in the container.

**Pure rules implemented by the seam (pinned, testable)**:
1. **Filter**: a raw name that is blank (`trim().is_empty()`) is dropped (a handler with an empty UI name would render a blank menu row).
2. **Order**: the system enumeration order is preserved verbatim — no re-sort.
3. **Dedup**: names are deduped by CASE-INSENSITIVE exact display-name match (`to_lowercase` fold), keeping the FIRST occurrence (its `launch_index` — the raw enumeration position — is what survives).
4. **Wrap**: every surviving raw name becomes `RegisteredProgram { display_name, launch_index }` where `launch_index` is the raw index in the input slice (`enumerate()` position) — the launch payload consumed by Decision 2's launcher.

**Data type** (owned by the pure module; plain `pub(crate)` fields — the `DecodedImage` field precedent):

```rust
pub(crate) struct RegisteredProgram {
    pub display_name: String,  // what the Task-3 menu shows
    pub launch_index: usize,   // the handler's position in the raw system enumeration (launch payload)
}
```

**Stub behavior** (`src/platform/stub.rs`, container/non-Windows target — safe no-ops, NEVER a panic):
- `enumerate_open_with_programs(_file_path: &Path) -> Vec<RegisteredProgram>` → returns `Vec::new()` (empty enumeration).
- `open_file_with_selected_program(_file_path: &Path, _program_index: usize) -> Result<(), OpenWithLaunchFailure>` → returns `Err(OpenWithLaunchFailure::LaunchFailed)` (a failure value, not a panic).

**OS-bound code remains untested by design** (`windows_open_with.rs` cannot execute in the Linux container; dev-checks compile the stub path only). Its first Windows-target compile validation is the target-scoped `cargo check` step in §5.

---

## §2 High-Level Data Flow (what the landed tree looks like after this cycle)

```
Task 3 (later): menu UI ──calls──▶ crate::platform::{enumerate_open_with_programs, open_file_with_selected_program}
                                        │ (cfg windows)                     │ (cfg not windows)
                                        ▼                                   ▼
                        src/platform/windows_open_with.rs        src/platform/stub.rs
                        SHAssocEnumHandlers → GetUIName →        Vec::new() / Err(LaunchFailed)
                        programs_from_handler_names  │
                                                     ▼
                        src/open_with_listing.rs (PURE, container-tested)
                        filter blank → dedup case-insensitive keep-first →
                        RegisteredProgram { display_name, launch_index }
                                                     ▲
Task 3 (later): menu list consumes these; on click passes launch_index to the launcher
```

New files: `src/open_with_listing.rs`, `src/open_with_listing/tests.rs`, `src/platform/windows_open_with.rs`. Edited files: `src/main.rs` (one `mod` line), `src/platform/mod.rs` (mod decl + re-exports), `src/platform/stub.rs` (two fns + imports), `src/platform/windows.rs` (ONE word: `pub(super)` visibility on `ensure_com_initialized`). NOT edited: `Cargo.toml`, `Cargo.lock`, `app.rs`, any UI/keyboard/pill module, any doc file.

---

## §3 Implementation Steps (tiny, ordered, verifiable — execute in this exact order)

### Step 3.1 — Pure seam module `src/open_with_listing.rs` (NEW)

Create the file with EXACTLY this shape (bodies may be spelled out by you, but every name, signature, visibility, allow, rule, and constant below is pinned):

```rust
//! Pure open-with program listing: turns raw system handler names into the
//! menu-list data. The OS-bound enumeration (platform facade) feeds it the
//! handlers' display names in the system's enumeration order; container tests
//! feed synthetic fixtures through the same function.

pub(crate) const OPEN_WITH_LAUNCH_FAILURE_MESSAGE: &str =
    "Windows could not open this file with the selected program.";

/// One system-registered program for the current file's type: the name the
/// menu shows plus the launch payload (the handler's position in the raw
/// system enumeration, consumed by the launcher's fresh re-enumeration).
#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) struct RegisteredProgram {
    pub display_name: String,
    pub launch_index: usize,
}

/// Launcher-hand-off failure (Task 2): collapses every launch error into the
/// single static message (the app's only message surface takes &'static str).
#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) enum OpenWithLaunchFailure {
    LaunchFailed,
}

impl OpenWithLaunchFailure {
    #[allow(dead_code)]
    pub(crate) fn user_message(self) -> &'static str {
        OPEN_WITH_LAUNCH_FAILURE_MESSAGE
    }
}

/// Filters blank handler names, dedups display names case-insensitively
/// (first occurrence kept, its raw index survives as launch_index), and
/// preserves the system enumeration order verbatim.
#[allow(dead_code)] // consumed by the Windows-side enumeration (this target: stub path)
pub(crate) fn programs_from_handler_names(handler_names: &[String]) -> Vec<RegisteredProgram> {
    let mut folded_names_seen: Vec<String> = Vec::new();
    let mut programs = Vec::new();
    for (raw_index, raw_name) in handler_names.iter().enumerate() {
        if handler_name_is_blank(raw_name) {
            continue;
        }
        let folded_name = raw_name.to_lowercase();
        if folded_names_seen.contains(&folded_name) {
            continue;
        }
        folded_names_seen.push(folded_name);
        programs.push(RegisteredProgram {
            display_name: raw_name.clone(),
            launch_index: raw_index,
        });
    }
    programs
}

fn handler_name_is_blank(raw_name: &str) -> bool {
    raw_name.trim().is_empty()
}

#[cfg(test)]
mod tests;
```

Compliance notes (binding): method bodies ≤ 50 lines; nesting ≤ 2; the dedup match is a simple two-expression predicate inside the loop (single-section boolean rule respected — `contains` alone); no magic numbers; no comments beyond the module/rationale doc lines shown (they document non-obvious seam intent — allowed as minimal high-level comments).

### Step 3.2 — Test module `src/open_with_listing/tests.rs` (NEW)

Create with EXACTLY these six tests (names pinned verbatim; bodies may be spelled out by you; fixtures are plain `Vec<String>`/`&[String]` literals):

1. `empty_input_yields_no_programs` — `programs_from_handler_names(&[])` (via `&Vec::new()`) → empty `Vec`.
2. `handler_names_map_to_programs_in_system_order` — fixture `["Zed Viewer", "Alpha Paint", "Photos"]` → output display names equal the input order verbatim (no sorting).
3. `launch_index_records_the_raw_enumeration_position` — fixture `["B", "x", "C"]` → the surviving programs' `launch_index` values are `0` and `2` (raw positions, unaffected by any filtering before them in the input).
4. `blank_handler_names_are_dropped` — fixture `["Real", "", "   ", "\t"]` → exactly one program, display name `Real`.
5. `duplicate_display_names_dedup_case_insensitively_keeping_first` — fixture `["Paint", "Viewer", "PAINT", "paint"]` → two programs: (`Paint`, `launch_index 0`), (`Viewer`, `launch_index 1`).
6. `non_ascii_display_names_are_preserved_verbatim` — fixture `["写真ビューア", "Photos"]` → both kept, names byte-identical to the fixtures (consistent with the project's name-collation test style).

Assert display names AND `launch_index` values where the test pins them. No other tests in this file (the OS-bound platform side is untestable by design; the stub path compiles it).

### Step 3.3 — Register the module: `src/main.rs` (EDIT, one line)

Add `mod open_with_listing;` to the module declarations, alphabetically among the existing top-level `mod` lines (next to `mod name_collation;`).

### Step 3.4 — Reuse the COM init: `src/platform/windows.rs` (EDIT, one word)

Change line 45 from `fn ensure_com_initialized() -> bool` to `pub(super) fn ensure_com_initialized() -> bool`. No other change to this file. (Both existing callers are inside this file — unaffected.)

### Step 3.5 — Windows implementation `src/platform/windows_open_with.rs` (NEW)

Create the file with EXACTLY this shape (module doc line 1 mirrors `windows.rs:1–5`'s cross-target note):

```rust
//! Windows implementation of the open-with enumeration and launcher (Tasks 1+2):
//! SHAssocEnumHandlers queries the system's registered handlers for the current
//! file's extension; IAssocHandler::GetUIName yields display names (fed to the
//! pure open_with_listing seam) and IAssocHandler::Invoke launches the selected
//! handler with the current file's data object. Compiled only on Windows
//! targets; first-compile validation is the cross-target check step.

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use windows::core::PCWSTR;
use windows::Win32::System::Com::{CoTaskMemFree, IDataObject};
use windows::Win32::UI::Shell::{
    ASSOC_FILTER_NONE, IAssocHandler, IEnumAssocHandlers, IShellItem, SHAssocEnumHandlers,
    SHCreateItemFromParsingName, SHCreateShellItemArrayFromShellItem,
};

use crate::open_with_listing::{
    programs_from_handler_names, OpenWithLaunchFailure, RegisteredProgram,
};

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn enumerate_open_with_programs(file_path: &Path) -> Vec<RegisteredProgram> {
    raw_handlers(file_path)
        .map(|handlers| programs_from_handler_names(&handler_ui_names(&handlers)))
        .unwrap_or_default()
}

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn open_file_with_selected_program(
    file_path: &Path,
    program_index: usize,
) -> Result<(), OpenWithLaunchFailure> {
    let handlers = raw_handlers(file_path).ok_or(OpenWithLaunchFailure::LaunchFailed)?;
    let handler = handlers
        .get(program_index)
        .ok_or(OpenWithLaunchFailure::LaunchFailed)?;
    let launch_outcome = unsafe {
        file_data_object(file_path).and_then(|data_object| handler.Invoke(&data_object))
    };
    launch_outcome.map_err(|_| OpenWithLaunchFailure::LaunchFailed)
}

fn raw_handlers(file_path: &Path) -> Option<Vec<IAssocHandler>> {
    if !super::windows::ensure_com_initialized() {
        return None;
    }
    let query = extension_query_string(file_path)?;
    let enum_handlers =
        unsafe { SHAssocEnumHandlers(PCWSTR(query.as_ptr()), ASSOC_FILTER_NONE) }.ok()?;
    Some(unsafe { drained_handlers(&enum_handlers) })
}

unsafe fn drained_handlers(enum_handlers: &IEnumAssocHandlers) -> Vec<IAssocHandler> {
    let mut handlers = Vec::new();
    loop {
        let Some(handler) = next_handler(enum_handlers) else {
            break;
        };
        handlers.push(handler);
    }
    handlers
}

unsafe fn next_handler(enum_handlers: &IEnumAssocHandlers) -> Option<IAssocHandler> {
    let mut buffer = [None::<IAssocHandler>];
    enum_handlers.Next(&mut buffer, None).ok()?; // pceltfetched None is legal when celt == 1
    buffer[0].take()
}

fn handler_ui_names(handlers: &[IAssocHandler]) -> Vec<String> {
    handlers
        .iter()
        .filter_map(|handler| unsafe { handler_ui_name(handler) })
        .collect()
}

unsafe fn handler_ui_name(handler: &IAssocHandler) -> Option<String> {
    let ui_name = handler.GetUIName().ok()?;
    let text = ui_name.to_string().ok();
    CoTaskMemFree(Some(ui_name.as_ptr().cast::<core::ffi::c_void>()));
    text
}

fn extension_query_string(file_path: &Path) -> Option<Vec<u16>> {
    let extension = file_path.extension()?;
    let mut query: Vec<u16> = Vec::with_capacity(extension.len() + 2);
    query.push(u16::from(b'.'));
    query.extend(extension.encode_wide());
    query.push(0);
    Some(query)
}

unsafe fn file_data_object(file_path: &Path) -> windows::core::Result<IDataObject> {
    let wide_path: Vec<u16> = file_path.as_os_str().encode_wide().chain([0]).collect();
    let shell_item: IShellItem =
        SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None)?;
    let data_object: IDataObject = SHCreateShellItemArrayFromShellItem(&shell_item)?;
    Ok(data_object)
}
```

Compliance notes (binding):
- File stays under 200 lines (~120). Each fn body ≤ 50 lines; nesting ≤ 2 (the `raw_handlers` guards are flat early returns; the `drained_handlers` loop body is 1 level; the `let-else` breaks keep the loop flat).
- `handler_ui_name` frees the `PWSTR` unconditionally AFTER the conversion attempt is captured (the `text` binding before `CoTaskMemFree` is load-bearing — keep that order).
- `buffer[0].take()` (not `first_mut`): a 1-element array literal; the slice coercion satisfies `&mut [Option<IAssocHandler>]`.
- Every failure path returns the empty/default/failure value (never panics) — same stance as the landed facade.
- If the compiler requires an explicit annotation for the bare `None` bind-context argument, use the compiler-suggested `None::<&IBindCtx>` — same decision, only the spelling changes.
- Clippy `-D warnings` must pass: the `#[allow(dead_code)]` lines are pinned above exactly where nothing on that target consumes the item yet (Task 3 will; the 4.3 reviewers may audit the placements, not remove them prematurely).

### Step 3.6 — Stub fallback: `src/platform/stub.rs` (EDIT)

Append the two stub functions and their import. The file gains:

```rust
use crate::open_with_listing::{OpenWithLaunchFailure, RegisteredProgram};

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn enumerate_open_with_programs(_file_path: &Path) -> Vec<RegisteredProgram> {
    Vec::new()
}

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn open_file_with_selected_program(
    _file_path: &Path,
    _program_index: usize,
) -> Result<(), OpenWithLaunchFailure> {
    Err(OpenWithLaunchFailure::LaunchFailed)
}
```

(`Path`/`PathBuf` are already imported in `stub.rs`; extend the existing `use std::path::...` only if needed.) Update the module doc header's summary sentence to mention the open-with stub behavior (empty enumeration / failed launcher) — one sentence, keep the rest verbatim.

### Step 3.7 — Facade re-exports: `src/platform/mod.rs` (EDIT)

Add (keeping the existing lines untouched):

```rust
#[cfg(target_os = "windows")]
mod windows_open_with;

// Consumed by the Task-3 menu UI; the allow silences the unused re-export until then.
#[cfg(target_os = "windows")]
#[allow(unused_imports)]
pub(crate) use windows_open_with::{enumerate_open_with_programs, open_file_with_selected_program};

#[cfg(not(target_os = "windows"))]
#[allow(unused_imports)]
pub(crate) use stub::{enumerate_open_with_programs, open_file_with_selected_program};
```

Also extend the module doc header (lines 1–3) with one sentence: the facade additionally re-exports the open-with enumeration/launcher pair. Nothing else changes in `mod.rs`.

### Step 3.8 — Compile both target paths

1. Container (stub path) quick compile: via the Alpine VM (`vm_status` FIRST) run
   `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "cargo check --locked"`
   — exit 0 expected (stub path).
2. Windows-target compile (first validation of `windows_open_with.rs` — dev-checks do NOT compile it):
   `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "cargo check --locked --target x86_64-pc-windows-gnu"`
   — exit 0 expected. If this exposes a genuine API-projection mismatch (name/module path differing from §1's cited signatures), FIX the import path to what the compiler says (the cited docs are authoritative for signatures, the compiler for module paths), re-run, and record the correction in the completion summary. Everything else (signatures, allow placements, structure) is NOT latitude.

### Step 3.9 — Full harness run (dev-checks)

Via the Alpine VM (`vm_status` FIRST):

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
```

Expected: `ALL CHECKS PASSED` (exit 0) — `cargo fmt --check`, `cargo check --locked`, `cargo test --locked` (**170 passed; 0 failed**), `cargo clippy --locked -- -D warnings`. If `cargo fmt --check` fails, run the fmt command via the harness (`docker compose ... run --rm rust sh -c "cargo fmt"`), re-run dev-checks, and commit the fmt result as a separate `style: apply cargo fmt` commit (the Wave-5 precedent).

---

## §4 Verification Evidence (what 4.2 must report back)

- Both §3.8 exit codes are 0.
- Dev-checks output ends `ALL CHECKS PASSED` with the test line showing **170 passed; 0 failed** across **27 test files** (the 164 pre-existing tests are UNCHANGED — zero test delta elsewhere; new file: `src/open_with_listing/tests.rs`, 6 tests).
- `git status` clean after the commit; `Cargo.toml`/`Cargo.lock` untouched in the diff (NO dependency change — `git diff --stat` must show only the 7 files listed in §6).

---

## §5 Notes Handed to Later Steps (do NOT act on them in 4.2 — record only)

- **Task 4 (verification cycle C)**: the DLL import audit is expected to show a DELTA from the recorded 22-DLL set — the new direct `shell32.dll` exports (`SHAssocEnumHandlers`, `SHCreateItemFromParsingName`, `SHCreateShellItemArrayFromShellItem`; `IAssocHandler`/`IEnumAssocHandlers` are COM-resolved via `CoCreateInstance`). The audit verdict is "set-equal OR an explained delta" per the global plan; the release rebuild (`scripts/build-windows.sh`) supplies the fresh OBSERVED list.
- **Task 5 (docs cycle D)**: `tech.md` (mechanism section), `architecture.md` (Logical Components "Fast Open-with Service" → LANDED + Module Layout + decisions table), `README.md`, `context.md`, `.agent/project-structure.md` (the three new files), and — if the Task-4 audit recorded a delta — the `docs/build-windows.md` OBSERVED list.
- **Task 3 (UI cycle B)**: consumes `crate::platform::{enumerate_open_with_programs, open_file_with_selected_program}` and `crate::open_with_listing::{RegisteredProgram, OpenWithLaunchFailure, OPEN_WITH_LAUNCH_FAILURE_MESSAGE}`; passes the canonical `opened_image_path`; decides the empty-list UX (user-approved propose-and-ask).

---

## §6 Git Steps (this cycle)

1. BEFORE any change: `git status` (expect clean, on `feat/wave-6-open-with`). If dirty, STOP and return the state to the caller (never commit unrelated changes).
2. Read `.gitignore` before staging (Gitignore Compliance Rule). The staged set must be EXACTLY these 7 files:
   - `src/main.rs` (one `mod` line)
   - `src/open_with_listing.rs` (NEW)
   - `src/open_with_listing/tests.rs` (NEW)
   - `src/platform/windows.rs` (one-word visibility change)
   - `src/platform/windows_open_with.rs` (NEW)
   - `src/platform/stub.rs` (two fns + import)
   - `src/platform/mod.rs` (mod decl + re-exports)
   Nothing else. `Cargo.lock`/`Cargo.toml` are NOT modified. Harness logs (`logs/`, gitignored) are never staged. Dev-check log paths stay gitignored.
3. Commit message (conventional, single implementation commit — a partial commit would break clippy's dead-code lints):
   `feat: runtime open-with enumeration and selected-program launcher`
4. Optional fmt-fix commit ONLY if §3.9's fmt round produced changes: `style: apply cargo fmt`.
5. NO branch creation (restricted to Step 2), NO version edit (restricted to Step 3), NO `git push` (restricted to Step 5), NO merge, NO `CHANGELOG.md`.

---

## §7 Scope Boundaries (explicit NOT-done list for 4.2)

- NO menu entry, NO pill/button/keyboard changes, NO egui UI code of any kind (Task 3).
- NO edits to `src/app.rs`, `src/app/*`, `src/app_display.rs`, `src/keyboard_intake` — zero app-state wiring; nothing in the app calls the new facade functions yet.
- NO `Cargo.toml`/`Cargo.lock` changes (zero new features/dependencies).
- NO documentation-file edits (Task 5; module doc comments inside the new files are part of the code and ARE in scope).
- NO `CHANGELOG.md` touch; NO edits to existing `.kilo/plans/*` files.
- No manual-UX testing (Task 4's OPERATOR-RUN owns it).

---

## §8 Plan-vs-TODO Cross-Check (verbatim compliance)

TODO Task 1 sub-item 1: "Query the Windows system registration data at runtime for the programs associated with the currently displayed file's type/extension; build the menu list from that data only. NO internal/hardcoded program database." → **§1 Decision 1**: `SHAssocEnumHandlers` queries the system's registration data at runtime per extension; the list is built exclusively from returned handlers; nothing program-related is hardcoded or stored (§3.1/§3.5).

TODO Task 1 sub-item 2: "The enumeration mechanism … is decided in this wave's architector step — this wave file must not name implementation APIs or crates." → **§1 Decision 1** (decided here; the wave file remains API-free).

TODO Task 2 sub-item: "Selecting an entry opens the CURRENT displayed file with that program (hand-off to the system launcher mechanism per the architector decision)." → **§1 Decision 2**: `IAssocHandler::Invoke` with the current file's `IDataObject`; hand-off by the launch index into the fresh re-enumeration; the current canonical path is the input (Task 3 passes `opened_image_path`).

Deferred item "Refresh/re-open timing of the enumeration (per-open vs cached — architector decision)" → **§1 Decision 3**: per-invocation fresh, no cache.

Deferred item "Empty-result behavior" → owned by Task 3 (global plan binds it to the Task-3 user question); this cycle only defines the empty-`Vec` data shape the decision will consume. NOT decided here (outside the four items assigned to this step).

TODO Task 4 prerequisite (testable seam) → **§1 Decision 4** + §3.1/§3.2 (pure module, synthetic-fixture seam, 6 named container tests).

Global plan Cycle-A 4.1b checklist (4 decisions + plan path) → all four settled in §1; plan saved at the mandated path; no existing plan file touched.

---

## §9 4.3 Reviewer Handoff Notes

- The `#[allow(dead_code)]`/`#[allow(unused_imports)]` annotations are INTENTIONAL and pinned (pre-Task-3 consumption; `image_loading.rs`/`name_collation.rs` precedent). Reviewers audit placements but do not delete them while Task 3 is pending.
- The dead-code-on-container-target situation is the inverse of the recorded Wave-5.2 note (there: pure fallback dead on Windows; here: the service dead everywhere until Task 3) — acceptable, recorded, self-resolving at Task 3.
- `windows.rs` was NOT grown (the `windows_collation.rs` sibling-file precedent); the new Windows code lives in `windows_open_with.rs`.
- Behavior changes visible to the user in this cycle: NONE (no caller exists yet). Manual UX belongs to Task 4.
