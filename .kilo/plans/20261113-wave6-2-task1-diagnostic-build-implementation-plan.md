# Wave 6.2 — Task 1 (Cycle A) Implementation Plan: Diagnostic Build (open-with launch evidence → `.log` file)

- Source TODO: `.agent/todos/20261113/20261113-todo-2.md` → `### Task 1: Diagnostic build (CALLER-AUTHORIZED 2026-11-13 — log-file mechanism, optional-module preference)` + the `## Deferred decisions` section.
- Global plan: `.kilo/plans/20261113-wave6-2-open-with-launch.md` → `### Cycle A — Task 1 (diagnostic build)` + the `## Addendum — 2026-11-13 Caller Decisions` (BINDING, supersedes contradicting lines).
- Branch: `fix/wave-6-2-open-with-launch` (created at workflow Step 2 — VERIFIED checked out, working tree clean).
- Version: 0.6.2 (already bumped at Step 3 — VERIFIED, `Cargo.toml:3`). NOT touched by this cycle.
- Baseline census: 189 unit tests across 28 test-bearing files (Wave-6.1 close). Expected census after this cycle: **UNCHANGED — 189 tests / 28 test files** (the diagnostic adds NO tests — §8).
- Executor: JUNIOR developer under 50% restriction — every structural choice below is FINAL; do not redesign, rename, or relocate anything. Latitude: exact comment/doc-line wrapping and rustfmt-mandated formatting only.
- Scope of THIS cycle: `Cargo.toml` (one `[features]` section), `src/platform/mod.rs` (one cfg'd mod declaration), ONE new file `src/platform/launch_log.rs`, ONE instrumented file `src/platform/windows_open_with.rs`. **NO Tasks 2/3/4 work** (no fix, no verification matrix, no wave docs), NO enumeration-list changes, NO app/seam/stub changes, NO behavioral changes when the feature is OFF.

---

## §0 PACKAGING DECISION (the caller's binding preference, feasibility decided HERE)

**DECISION: the diagnostic is an OPTIONAL BUILD-TIME MODULE — a crate-level Cargo feature `diagnostic-launch-log`, OFF by default.** The fallback (temporary removable code) is NOT needed.

Feasibility assessment (honest):

| Concern | Verdict |
| --- | --- |
| Cargo feature on the local crate | Trivial: a `[features]` section with `default = []` and one empty feature. Adds NO dependencies; the dependency graph is unchanged → **`Cargo.lock` is byte-UNAFFECTED and every `--locked` invocation keeps passing** (own-crate features are not recorded in the lock; Wave-5.2 lock-neutrality precedent). |
| Harness (`--locked` runs) under the DEFAULT config (feature OFF) | The diagnostic module and every hook statement are `#[cfg(...)]`-STRIPPED — the default configuration compiles exactly the pre-diagnostic code. The harness MUST keep passing without the feature (§6 Step 3). |
| `--all-features` accidental enablement | The module gate is `all(target_os = "windows", feature = "diagnostic-launch-log")` — on the host Linux target the module NEVER exists regardless of features; on the Windows target it exists only with the feature. Clippy/fmt run on the host target WITHOUT the feature in the harness → never see the module. |
| Unused-code warnings when the feature is OFF | NONE: the module is removed from the module tree entirely (cfg'd mod declaration), and every hook call site is a cfg'd STATEMENT — cfg-stripping removes the whole statement including its argument expressions, so no code, no allocation, no warning survives in the default config. NO `#[allow(dead_code)]` patterns are needed. |
| Release artifact carries ZERO diagnostic activity | `scripts/build-windows.sh` is UNTOUCHED and never passes `--features` → the release build compiles with the feature OFF → the diagnostic module is not even compiled into the 0.6.2 artifact. Removal before the release rebuild is therefore NOT REQUIRED. |

Per the caller's binding modification (2), the diagnostic code **IS committed** (feature-gated OFF by default). Record for the Task-2 architector: the global plan's "diagnostic-code REMOVAL in Task 2" line is SUPERSEDED by the Addendum's optional-module decision — whether Task 2 removes the module is that cycle's own decision; keeping it is harmless and the release path is already clean.

Pinned `Cargo.toml` edit (insert BETWEEN the `[package]` section and `[dependencies]`):

```toml
[features]
default = []
diagnostic-launch-log = []
```

Pinned cfg condition used EVERYWHERE in this plan (the exact string, verbatim):

```rust
all(target_os = "windows", feature = "diagnostic-launch-log")
```

---

## §1 LOG-FILE MECHANISM (caller's binding modification 1)

| Decision point | Pinned choice |
| --- | --- |
| File name | `img-scroller-diagnostic.log` |
| Primary location | The directory of the RUNNING exe (`std::env::current_exe()` → `.parent()`); the operator runs the exe from `dist\windows-x64\`, so the log lands NEXT TO the exe — easy to find. |
| Fallback location | `std::env::temp_dir()` — used ONLY when the exe-directory append fails (current_exe failure, missing parent, or an open/write error). |
| Mode | APPEND (`OpenOptions::new().create(true).append(true)`); created lazily on the FIRST diagnostic write; never truncated. Multiple selection attempts and multiple app runs ACCUMULATE in one file the operator sends back once. |
| Header | Written ONCE PER PROCESS RUN (before the first line of that run) via a process-lifetime `AtomicBool`: `=== img-scroller-diagnostic v0.6.2 profile=debug started_unix=<secs> ===` (version from `env!("CARGO_PKG_VERSION")`; profile from `cfg!(debug_assertions)`; unix-epoch seconds — std-only, NO chrono dependency). |
| Timestamps | `SystemTime::now().duration_since(UNIX_EPOCH)` → seconds (`unwrap_or(0)` on error); formatted as `unix=<secs>`. |
| Failure handling | A failed log write (or an entirely unresolvable log target) is SILENTLY SKIPPED — never a panic, never an app-behavior change, never a message. All file errors are swallowed. |
| Console | NOT used (the mechanism is the file). NOTE for the operator: the diagnostic exe is a DEBUG build (`debug_assertions` ON → the release `windows_subsystem` attribute is not applied) so a console window appears alongside the viewer — EXPECTED, harmless, ignore it. |

---

## §2 RECORDED FIELDS (exactly what Task 2 needs)

One header line per process run (§1), then per selection attempt — the lines IN ORDER:

| # | Line | Format (verbatim) | Feeds Task 2 |
| --- | --- | --- | --- |
| 1 | stored enumeration (logged at EVERY `enumerate_open_with_programs` call — every successful image open AND every menu refresh) | `stored-enumeration: count=<n> entries=[pos=<i> launch_index=<j> "<name>"; ...]` | The STORED menu list: each entry's list position AND its recorded raw `launch_index` (pairing-B suspect, checked directly). |
| 2 | attempt start (logged at the launcher's entry, before any OS work) | `--- attempt: file="<path>" menu_position=<n> unix=<secs>` | WHICH file was handed off and WHICH menu entry was clicked (the display name is derivable from the most recent stored-enumeration line via `pos`). |
| 3 | fresh enumeration failure (ONLY when `raw_handlers` returns `None`) | `fresh-enumeration: FAILED (raw_handlers returned None)` | COM/query failure at launch time vs later failures. |
| 4 | fresh enumeration drained | `fresh-enumeration: handler_count=<n> names=["<name>", ...]` | The launch-time re-enumeration's handler count AND its raw handler names (pairing-B check against line 1). NOTE: names may be FEWER than handlers (`GetUIName` failures are filtered) — both counts are on the line. |
| 5 | `launch_index` resolution | `launch-index: menu_position=<n> resolved=<bool>` | Whether the value the launcher received resolved to a handler in the fresh list. |
| 6 | `SHCreateItemFromParsingName` outcome | `shcreateitemfromparsingname: created=<bool> code=<0x........|none>` | Data-object construction step 1. |
| 7 | `BindToHandler` (BHID_DataObject) outcome | `bindtohandler-bhid_dataobject: bound=<bool> code=<0x........|none>` | Data-object construction step 2. |
| 8 | final classified outcome (the value the app produced) | `launch-final: succeeded=<bool> code=<0x........|none> classified=<Success\|LaunchFailed>` | The overall result. When lines 6–7 are both `ok`, line 8's `code` IS the `IAssocHandler::Invoke` HRESULT; when a sub-step failed, its own line carries the code and line 8 repeats it. |

HRESULT formatting (PINNED): the raw `i32` via `windows_core::Error::code()` (the `windows`-crate error carries the HRESULT), rendered as `0x{:08X}` of the u32 bit pattern (`code as u32`) — e.g. `code=0x80004005`; `code=none` when `Ok`. The `{:?}` debug form is NOT used. If the compiler rejects `error.code()` on `windows-core 0.62.2` (NOT expected — the caller's task text itself cites `error.code()`), STOP and return the exact compiler output to the caller; do NOT improvise a different accessor.

Field-coverage map (every field the TODO + the caller's step text demand → where it is in the log):

| Required field | Recorded as |
| --- | --- |
| Build profile, timestamp (header) | Header line, once per process run (§1) |
| The selected program's display name | The most recent `stored-enumeration` line + the attempt's `menu_position` (the stored line lists `pos=<i> ... "<name>"` for every entry; the click selects by list position, so `pos = menu_position` names the selection) |
| The stored `launch_index` of the selection | Same correlation: the stored line's `launch_index=<j>` for `pos = menu_position` |
| The launch-time re-enumeration's handler count + raw names | `fresh-enumeration:` line |
| Whether the stored index resolves | `launch-index:` line |
| `SHCreateItemFromParsingName` outcome | `shcreateitemfromparsingname:` line |
| `BindToHandler` outcome | `bindtohandler-bhid_dataobject:` line |
| `IAssocHandler::Invoke` outcome (result code) | `launch-final:` line's `code` when lines 6–7 are `ok` (that code IS Invoke's HRESULT) |
| The final classified outcome | `launch-final:` line's `classified=<Success\|LaunchFailed>` |

Rationale for the position-based correlation (pinned): the launch hand-off receives ONLY the list position (`open_file_with_selected_program(file_path, program_index)`), so the display name is NOT available inside the platform seam; recording the full stored list (name + recorded launch_index per position) plus the attempt's position reconstructs the selection's name + launch_index per attempt from the SAME single log file, without touching the app layer. TODO line 18 delegates the exact recorded-field design to this architector step — this mapping is that decision.

DESIGN NOTE (recorded for Task 2's evidence review — NOT a fix, NOT a conclusion, NO behavior change in this task): in the landed call chain, `src/app/open_with_button.rs` `show_menu_entry_buttons` passes the menu-list POSITION to `launch_open_with_program`, which forwards it to `open_file_with_selected_program`; the stored `launch_index` field is not read at the launch site. The diagnostic logs BOTH the `menu_position` (line 2/5) and every stored entry's recorded `launch_index` (line 1), so Task 2 can verify the pairing directly from the log. This task makes NO judgment and NO change about it.

---

## §3 INSTRUMENTATION POINTS (all inside the platform seam; the app layer stays byte-identical)

The diagnostic hooks live INSIDE `src/platform/windows_open_with.rs` — the platform seam — because the app-side error type collapses every failure into `OpenWithLaunchFailure::LaunchFailed` (the step-level evidence would be lost at the app seam). `src/app/open_with_button.rs` is NOT edited at all. Production behavior is BYTE-IDENTICAL when the feature is OFF: every hook is a `#[cfg(...)]`-stripped STATEMENT; the two production functions keep their exact control flow and failure semantics (every failure still collapses into `Err(OpenWithLaunchFailure::LaunchFailed)`).

New module `src/platform/launch_log.rs` (std-only; NO new dependency; NO egui/windows imports except the `RegisteredProgram` type from the crate):

- Declared in `src/platform/mod.rs` (append after the existing mod declarations):
  ```rust
  #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
  pub(crate) mod launch_log;
  ```
- Constants/statics: `LOG_FILE_NAME = "img-scroller-diagnostic.log"`; `static HEADER_WRITTEN: AtomicBool = AtomicBool::new(false);`
- Public hook functions (ALL `pub(crate)`, ALL ≤2 params — max-arguments rule):
  1. `pub(crate) fn enumeration_stored(programs: &[RegisteredProgram])`
  2. `pub(crate) fn attempt_started(file_path: &Path, menu_position: usize)`
  3. `pub(crate) fn fresh_enumeration_failed()`
  4. `pub(crate) fn fresh_enumeration_drained(handler_count: usize, handler_names: &[String])`
  5. `pub(crate) fn launch_index_resolved(menu_position: usize, resolved: bool)`
  6. `pub(crate) fn shell_item_created(created: bool, error_code: Option<i32>)`
  7. `pub(crate) fn data_object_bound(bound: bool, error_code: Option<i32>)`
  8. `pub(crate) fn launch_finished(succeeded: bool, error_code: Option<i32>)`
- Private plumbing (the exact shape the implementer writes):
  ```rust
  fn emit(line: &str) {
      if let Some(header) = header_line_if_first_write() {
          append_line(&header);
      }
      append_line(line);
  }

  fn append_line(line: &str) {
      if append_in_directory(current_exe_directory(), line) {
          return;
      }
      append_in_directory(Some(std::env::temp_dir()), line);
  }

  fn append_in_directory(directory: Option<PathBuf>, line: &str) -> bool {
      let Some(directory) = directory else { return false; };
      let Ok(mut log_file) =
          OpenOptions::new().create(true).append(true).open(directory.join(LOG_FILE_NAME))
      else {
          return false;
      };
      writeln!(log_file, "{line}").is_ok()
  }

  fn current_exe_directory() -> Option<PathBuf> {
      std::env::current_exe().ok()?.parent().map(Path::to_path_buf)
  }

  fn header_line_if_first_write() -> Option<String> {
      if HEADER_WRITTEN
          .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
          .is_err()
      {
          return None;
      }
      Some(format!(
          "=== img-scroller-diagnostic v{} profile={} started_unix={} ===",
          env!("CARGO_PKG_VERSION"),
          profile_name(),
          unix_epoch_seconds()
      ))
  }

  fn profile_name() -> &'static str {
      if cfg!(debug_assertions) { "debug" } else { "release" }
  }

  fn unix_epoch_seconds() -> u64 {
      SystemTime::now()
          .duration_since(UNIX_EPOCH)
          .map(|elapsed| elapsed.as_secs())
          .unwrap_or(0)
  }

  fn error_code_text(error_code: Option<i32>) -> String {
      match error_code {
          Some(code) => format!("0x{:08X}", code as u32),
          None => "none".to_string(),
      }
  }

  fn quoted_names(names: &[String]) -> String {
      names.iter().map(|name| format!("\"{name}\"")).collect::<Vec<_>>().join(", ")
  }
  ```
  Hook bodies follow §2's formats verbatim (each hook = one `emit(&format!(...))`); `enumeration_stored` additionally joins `pos=<i> launch_index=<j> "<name>"` entries with `"; "` and wraps them in `entries=[...]`.
- Imports: `std::fs::OpenOptions`, `std::io::Write`, `std::path::{Path, PathBuf}`, `std::sync::atomic::{AtomicBool, Ordering}`, `std::time::{SystemTime, UNIX_EPOCH}`, `crate::open_with_listing::RegisteredProgram`. Module doc comment: state that this is the OPTIONAL Wave-6.2 Task-1 diagnostic (caller-authorized), compiled ONLY when BOTH the `windows` target AND the `diagnostic-launch-log` feature are on, that the release build never enables the feature so the shipped artifact contains none of it, that every write failure is silently skipped, and that Task 2's root-cause consumes the log.
- `launch_log.rs` target size ≈ 100–120 lines (well under the 200-line file cap).

---

## §4 LANDED FACTS THIS PLAN BUILDS ON (verified at HEAD, branch `fix/wave-6-2-open-with-launch`)

| Fact | Source (verified) |
| --- | --- |
| Launcher shape: `raw_handlers(...).ok_or(LaunchFailed)?` → `handlers.get(program_index).ok_or(LaunchFailed)?` → `unsafe { file_data_object(file_path).and_then(\|d\| handler.Invoke(&d)) }` → `map_err(\|_\| LaunchFailed)` | `src/platform/windows_open_with.rs:46–57` |
| Enumeration shape: `raw_handlers` → `handler_ui_names` → `programs_from_handler_names` → `unwrap_or_default` (expression form) | `src/platform/windows_open_with.rs:39–43` |
| `file_data_object`: `SHCreateItemFromParsingName(PCWSTR(...), None)?` → `shell_item.BindToHandler(None::<&IBindCtx>, &BHID_DataObject)?` | `src/platform/windows_open_with.rs:115–123` |
| `handler_ui_names(&[IAssocHandler]) -> Vec<String>` exists and is safe to call again on fresh handlers (GetUIName is a read; it does not consume the handler) | `src/platform/windows_open_with.rs:90–104` |
| The launch call site passes the menu-list POSITION; the stored `launch_index` field is not read there | `src/app/open_with_button.rs:190–198` (grep-verified: `.launch_index` has NO non-test consumer in `src/app`) |
| The crate has NO `[features]` section; `windows = "=0.62.2"` with exactly the six features | `Cargo.toml` (full file read) |
| Version 0.6.2 already bumped; tree clean on `fix/wave-6-2-open-with-launch` | `Cargo.toml:3` + `git status` |
| Harness: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` (fmt-check → check → test → clippy; final `ALL CHECKS PASSED`/`N CHECK(S) FAILED`; exit code authoritative; logs under gitignored `logs/checks/`) | `docs/build.md` |
| Windows cross-build script (release, atomic install to `dist/windows-x64/img-scroller.exe`) — UNTOUCHED by this cycle; it never passes `--features` | `scripts/build-windows.sh` + `docs/build-windows.md` |
| `CARGO_TARGET_DIR=/cargo-target` (named volume, persists between runs) → the DEBUG cross-build exe lands at `/cargo-target/x86_64-pc-windows-gnu/debug/img-scroller.exe`; the project share is mounted at `/img-scroller` inside the container | `docs/build.md` + tech.md `## Build Environment` |
| `.gitignore` covers `dist/`, `target/`, `logs/`, `*.log` | `.gitignore:17,18,31,32` |
| `windows_core::Error` carries the HRESULT (`code()` → `HRESULT(pub i32)`) | caller task text (authoritative for this plan); unexpected compile rejection → STOP + report (§2) |

---

## §5 IMPLEMENTATION STEPS (4.2 implementer — tiny steps, in order)

### Step 0 — Pre-flight (NO file changes)

1. Call `alpine-vm_vm_status` FIRST. If the VM or the `/img-scroller` share is unavailable → STOP and return a user question (per `docs/build.md`).
2. On the host: `git status` (bash tool; single commands only) — verify: branch `fix/wave-6-2-open-with-launch`, working tree clean, `Cargo.toml` version 0.6.2. Otherwise STOP and return a question to the caller (branch creation is Step-2-restricted).
3. Read `.gitignore` (listed in §4) — required by the Gitignore Compliance Rule before every commit below.

### Step 1 — Commit the plan file (workflow-precedent plan commit)

- Stage ONLY `.kilo/plans/20261113-wave6-2-task1-diagnostic-build-implementation-plan.md` (this file).
- `git status` check: nothing gitignore-matching staged; no other files staged.
- Commit message: `docs: add wave6-2 task1 diagnostic-build implementation plan`

### Step 2 — `Cargo.toml`: add the feature section

Insert EXACTLY the §0 three-line `[features]` block between the `[package]` section and `[dependencies]`. Nothing else changes in the file.

### Step 3 — `src/platform/mod.rs`: declare the module

Append after the existing mod declarations (after the `mod windows_open_with;` cfg block):

```rust
#[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
pub(crate) mod launch_log;
```

Nothing else changes in the file.

### Step 4 — NEW file `src/platform/launch_log.rs`

Write the module per §3 (header doc, constants/statics, the eight `pub(crate)` hooks with §2's verbatim formats, the private plumbing). Module doc comment MUST state: the optional Wave-6.2 Task-1 diagnostic (caller-authorized), the compound cfg gate, the release build never enabling the feature, silent-skip on write failure, and Task-2 consumption. Keep the file ≤ 200 lines (target ~100–120).

### Step 5 — `src/platform/windows_open_with.rs`: the instrumentation (the ONLY instrumented file)

Exactly these sub-edits; every cfg'd line uses the §0 pinned condition string; every hook call is fully qualified as `super::launch_log::<hook>(...)` (NO `use` statement — avoids unused-import risk in both cfg states).

**5a. `enumerate_open_with_programs`** — replace the expression-form body (lines 39–43) with the let-form + ONE hook (behavior-identical):

```rust
pub(crate) fn enumerate_open_with_programs(file_path: &Path) -> Vec<RegisteredProgram> {
    let programs = raw_handlers(file_path)
        .map(|handlers| programs_from_handler_names(&handler_ui_names(&handlers)))
        .unwrap_or_default();
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::enumeration_stored(&programs);
    programs
}
```

**5b. `open_file_with_selected_program`** — replace lines 46–57 with:

```rust
pub(crate) fn open_file_with_selected_program(
    file_path: &Path,
    program_index: usize,
) -> Result<(), OpenWithLaunchFailure> {
    let handlers = match raw_handlers(file_path) {
        Some(handlers) => handlers,
        None => {
            #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
            super::launch_log::fresh_enumeration_failed();
            return Err(OpenWithLaunchFailure::LaunchFailed);
        }
    };
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::attempt_started(file_path, program_index);
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::fresh_enumeration_drained(handlers.len(), handler_ui_names(&handlers));
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::launch_index_resolved(program_index, handlers.get(program_index).is_some());
    let handler = handlers
        .get(program_index)
        .ok_or(OpenWithLaunchFailure::LaunchFailed)?;
    let launch_outcome =
        unsafe { file_data_object(file_path).and_then(|data_object| handler.Invoke(&data_object)) };
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::launch_finished(launch_outcome.is_ok(), error_code_of(&launch_outcome));
    launch_outcome.map_err(|_| OpenWithLaunchFailure::LaunchFailed)
}
```

Line order inside the function: `fresh_enumeration_failed` inside the `None` arm; then `attempt_started` → `fresh_enumeration_drained` → `launch_index_resolved` → the UNCHANGED `let handler` lines → the UNCHANGED `launch_outcome` line → `launch_finished` → the UNCHANGED `map_err` line. The failure semantics are unchanged (`ok_or` replaced by the behavior-identical `match`; every failure still `Err(OpenWithLaunchFailure::LaunchFailed)`).

**5c. `file_data_object`** — replace lines 115–123 with (behavior-identical; named intermediate results so each sub-step can be logged):

```rust
unsafe fn file_data_object(file_path: &Path) -> windows::core::Result<IDataObject> {
    unsafe {
        let wide_path: Vec<u16> = file_path.as_os_str().encode_wide().chain([0]).collect();
        let shell_item: windows::core::Result<IShellItem> =
            SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None);
        #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
        super::launch_log::shell_item_created(shell_item.is_ok(), error_code_of(&shell_item));
        let shell_item = shell_item?;
        let bind_outcome: windows::core::Result<IDataObject> =
            shell_item.BindToHandler(None::<&IBindCtx>, &BHID_DataObject);
        #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
        super::launch_log::data_object_bound(bind_outcome.is_ok(), error_code_of(&bind_outcome));
        Ok(bind_outcome?)
    }
}
```

**5d. NEW private helper** (append near the bottom of the file; cfg-gated so it vanishes entirely when OFF):

```rust
#[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
fn error_code_of<T>(outcome: &windows::core::Result<T>) -> Option<i32> {
    outcome.as_ref().err().map(|error| error.code().0)
}
```

Expected file size after the edits: ~150 lines (under the 200-line cap); every edited function stays ≤ 50 body lines; ≤ 2 nesting levels.

### Step 6 — Harness round 1: dev-checks WITHOUT the feature (the default config)

1. `alpine-vm_vm_status` first (immediately before running).
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` (use a generous MCP timeout, e.g. 900000 ms; cargo resumes incrementally if re-run).
3. Authoritative evidence: EXIT CODE `0`; final line `ALL CHECKS PASSED`; per-check `PASS` for `fmt-check`/`check`/`test`/`clippy`; test census corroborates **189 passed; 0 failed** (UNCHANGED).
4. If `fmt-check` fails on the new/edited files: apply `cargo fmt` once (`docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt`), then re-run the SAME dev-checks command. Any OTHER failure: fix per this plan (minimal compile/clippy residual fixes, one-more-attempt cap) and re-run.
5. This step proves the DEFAULT configuration (feature OFF) is unaffected — the caller's zero-diagnostic-activity guarantee for dev builds.

### Step 7 — Windows-target compile WITHOUT the feature

Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu`
Expected: exit `0` (only the pre-existing benign dead-code warning on `compare_entry_names` is acceptable).

### Step 8 — Windows-target compile WITH the feature (the diagnostic code path)

Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu --features diagnostic-launch-log`
Expected: exit `0`. A compile error around `error.code()` / `BindToHandler` / cfg inference → capture the FULL diagnostics and STOP with a caller question; do NOT redesign.

### Step 9 — Build the diagnostic exe (DEBUG cross-build with the feature ON)

Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo build --locked --target x86_64-pc-windows-gnu --features diagnostic-launch-log`
Expected: exit `0`. The exe lands at `/cargo-target/x86_64-pc-windows-gnu/debug/img-scroller.exe` (inside the named volume; NOT `--release` — `debug_assertions` ON, so the operator's run shows a console window alongside the viewer; §1).

### Step 10 — Copy the diagnostic exe to the gitignored `dist/` for the operator

1. Run (one docker command; the share is mounted at `/img-scroller` inside the container):
   `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "mkdir -p /img-scroller/dist/windows-x64 && cp /cargo-target/x86_64-pc-windows-gnu/debug/img-scroller.exe /img-scroller/dist/windows-x64/img-scroller-diagnostic.exe"`
   Expected: exit `0`.
2. Verify non-empty: `docker compose -f /img-scroller/docker-compose.yml run --rm rust ls -l /img-scroller/dist/windows-x64/` — `img-scroller-diagnostic.exe` present and size > 0.
3. The 0.6.2 RELEASE artifact `dist/windows-x64/img-scroller.exe` (if present) is NOT touched — the diagnostic exe uses the DISTINCT name. Nothing in `dist/` is ever committed (gitignored).

### Step 11 — Gitignore compliance + commit the diagnostic code

1. `git status` — verify ONLY these four files are modified/added: `Cargo.toml`, `src/platform/mod.rs`, `src/platform/launch_log.rs` (new), `src/platform/windows_open_with.rs`. **`Cargo.lock` must NOT appear** (§0 lock-neutrality — if it appears, STOP and return the diff to the caller). `dist/`, `logs/`, `*.log` must NOT appear (gitignored).
2. Stage ONLY those four files.
3. Commit message: `feat: add optional diagnostic launch log behind diagnostic-launch-log feature`
4. Re-run `git status` to confirm a clean tree afterwards.

### Step 12 — Scope-boundary self-check (all must hold before declaring the cycle implemented)

- `git diff` against the pre-cycle state shows EXACTLY the four files of Step 11; `src/app/open_with_button.rs`, `src/app/open_with_button_tests.rs`, `src/open_with_listing.rs`, `src/open_with_listing/tests.rs`, `src/platform/stub.rs`, `src/platform/windows.rs`, `src/platform/windows_collation.rs`, `src/app.rs`, `src/main.rs`, `Cargo.lock`, `scripts/build-windows.sh` — ALL byte-identical.
- No new tests anywhere (census 189/28 pinned); no UI/intake/styling/state changes; no messages; no app-state fields.
- Enumeration list content, `ASSOC_FILTER_NONE`, the three-point refresh contract, the state gates, `OPEN_WITH_LAUNCH_FAILURE_MESSAGE`, and the failure semantics — ALL unchanged.
- Feature OFF everywhere by default; `scripts/build-windows.sh` untouched (release NEVER enables the feature).

---

## §6 VERIFICATION MATRIX (the implementer's evidence, §5 Steps 6–10)

| Config | Command | Expected evidence |
| --- | --- | --- |
| Host default (feature OFF) | `sh scripts/dev-checks.sh` in the container | exit 0; `ALL CHECKS PASSED`; census 189/0 across 28 files |
| Windows target, feature OFF | `cargo check --locked --target x86_64-pc-windows-gnu` | exit 0 (pre-existing benign warning allowed) |
| Windows target, feature ON | `cargo check --locked --target x86_64-pc-windows-gnu --features diagnostic-launch-log` | exit 0 |
| Diagnostic exe | `cargo build --locked --target x86_64-pc-windows-gnu --features diagnostic-launch-log` | exit 0; exe non-empty at `/cargo-target/x86_64-pc-windows-gnu/debug/img-scroller.exe`; copied to `dist/windows-x64/img-scroller-diagnostic.exe` (Step 10) |
| Release path | NOT run in this cycle (Task 3 scope); `build-windows.sh` untouched — the feature is never enabled there → ZERO diagnostic activity guaranteed by construction |

---

## §7 OPERATOR INSTRUCTIONS (the implementer hands this text to the caller at completion; the Planner presents it at 4.6 — PLAIN, self-contained, per the operator's clarity requirement)

```text
Diagnostic run — "Open with..." (the viewer will look exactly like before; a black console
window may appear next to it — that is expected for this special build, ignore it):

1. Double-click: dist\windows-x64\img-scroller-diagnostic.exe
2. Open a png image (the same one as before, e.g. [Image 1]).
3. Click the "Open with..." button in the bottom pill (or press Q) and select the FIRST
   program in the list. Let it show the failure message.
4. Open the list once more and select a DIFFERENT program (any). Let it fail too.
5. Close the app (press ESC twice, or the window's X).
6. Send back the file img-scroller-diagnostic.log — it is in the SAME folder as the exe
   (dist\windows-x64\). If you cannot find it there, look in %TEMP% for a file with the
   same name.

The log accumulates both selection attempts (and a header line per app run) — one file
is enough. Nothing needs to be copied from the console.
```

The diagnostic exe is gitignored and lives ONLY in `dist/` on the operator machine; the wave's Step-5 tmp sweep removes it (the global plan's Step-5 tmp-sweep note already covers diagnostic build artifacts in `dist/`).

---

## §8 TESTS

- **Zero new unit tests; census stays pinned at 189/28.** Rationale: the diagnostic module is std file-I/O + format strings, compiled ONLY under a feature the harness never enables — a test inside it would never run in dev-checks; the harness's census corroboration (Step 6) is the container-testable evidence; the REAL verification is the operator's diagnostic run (§7), whose log feeds Task 2. Wave-5.2/6.1 zero-test-delta precedent (OS-bound diagnostic surface).

---

## §9 REVIEWER / SIMPLIFIER / ADHERENCE HOOKS (4.3 + 4.5b)

- Code-reviewer verifies: (1) the diff touches ONLY the four files of Step 11; (2) the cfg condition string is byte-identical everywhere (`all(target_os = "windows", feature = "diagnostic-launch-log")`); (3) the hook order matches §5b exactly (attempt-start BEFORE the drained/resolved lines; launch-final BEFORE `map_err`); (4) every hook is a cfg'd STATEMENT (no cfg'd fn wrappers wrapping production code, no `#[allow(dead_code)]` added anywhere); (5) `open_with_button.rs`/`stub.rs`/`open_with_listing.rs` are byte-identical; (6) no commented-out code, no new tests, `Cargo.lock` untouched.
- Code-simplifier expected verdict: NO-CHANGE (the instrumentation is already minimal — 9 cfg'd statements + one helper + one new module).
- 4.5b adherence checklist: §0/§1/§2/§3 decisions all present in the code; the §6 matrix evidence recorded; the §7 operator text handed back verbatim; the §12 self-check items.
- 4.4 docs-specialist: module-header coherence in `launch_log.rs` and a one-sentence pointer in `windows_open_with.rs`'s header ONLY (the wave-level tech.md/architecture.md/context.md documentation is Task 4 / Cycle D scope).

---

## §10 SCOPE BOUNDARIES (hard)

- NO Task 2/3/4 work: no root-cause conclusions, no fix, no verification matrix beyond §6, no wave documentation.
- NO behavioral changes: with the feature OFF every production path is byte-identical; with the feature ON the ONLY difference is log writes (which never alter control flow, never panic, never change messages).
- NO dependency changes; `Cargo.lock` byte-identical; no `CHANGELOG.md` edits; no edits to existing `.kilo/plans/*` files.
- NO removal decision for the diagnostic module in this cycle (recorded for Task 2's architector in §0).
- The diagnostic exe is never committed and never overwrites the release artifact.

---

## §11 SOURCES (consulted this session)

- Landed sources read in full: `src/platform/windows_open_with.rs`, `src/open_with_listing.rs`, `src/app/open_with_button.rs`, `src/platform/mod.rs`, `src/platform/stub.rs`, `src/main.rs`, `Cargo.toml`, `.gitignore`.
- `.kilo/plans/20261113-wave6-1-task1-launcher-fix-implementation-plan.md` §6 (the armed diagnostic fallback this task executes — NOT edited, referenced only).
- `docs/build.md` (harness commands, alpine-vm MCP usage, lockfile policy, validation stance).
- `windows-core 0.62.2` `Error::code()` → `HRESULT` (`HRESULT(pub i32)`) — pinned by the caller's task text; compile rejection → STOP + report (§2).
- Verified facts via grep/git this session: the `launch_index` consumption audit (§2 DESIGN NOTE) and the branch/ cleanliness state (§5 Step 0).
