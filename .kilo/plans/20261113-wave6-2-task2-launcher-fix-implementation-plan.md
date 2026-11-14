# Wave 6.2 Task 2 — Launcher Fix Implementation Plan (Critical Workflow step 4.1b, architector)

- Source TODO: `.agent/todos/20261113/20261113-todo-2.md` → Task 2 (root-cause with the diagnostic data + implement the evidence-decided fix).
- Global plan: `.kilo/plans/20261113-wave6-2-open-with-launch.md` → Cycle B.
- Operator evidence (verbatim, recorded in `.agent/project-info/context.md`): `dist/windows-x64/img-scroller-diagnostic.log` v0.6.2 profile=debug.
- Branch state at plan time: `fix/wave-6-2-open-with-launch`, HEAD `e1cdfc3`, version 0.6.2, census 189 unit tests / 28 test-bearing files, dev-checks green.
- This plan is NEW (no existing `.kilo/plans/*` file is edited).

## §0 Verdict verification — CONFIRMED (independent code trace)

The caller's preliminary verdict is independently verified against the landed code. The failure is `SHCreateItemFromParsingName` receiving the `\\?\`-prefixed canonical path → `E_INVALIDARG` (0x80070057).

### §0.1 The path's journey (exact lines)

1. `src/app.rs:71-75` (`open_image`) — the D5 single-normalization point:
   ```rust
   let outcome = image_loading::load_image(&path);
   let opened_path = std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
   ```
   Rust std documents (doc.rust-lang.org/std/fs/fn.canonicalize.html, §Platform-specific behavior): *"On Windows, this converts the path to use extended length path syntax"* — i.e. the returned `PathBuf` carries the `\\?\` VERBATIM prefix (corroborated by rust-lang/rust#42869, "std::fs::canonicalize returns UNC paths on Windows"). The `unwrap_or_else` fallback keeps the raw path only when canonicalization fails (non-existent file — the launch path is irrelevant there since the open already failed).
2. `src/app.rs:91` — `self.opened_image_path = Some(path);` stores the CANONICAL result. The diagnostic proves the prefix is present: the log's `--- attempt: file="\\?\C:\_kazuya\imgs\Chino_Kafuu__BOOTH_\Chino Kafuu 1.png"` is produced by `src/platform/launch_log.rs:45-51` printing `file_path.display()` of the exact `opened_image_path` value handed to the launcher. CONFIRMED: `opened_image_path` carries `\\?\`.
3. `src/app/open_with_button.rs:174-181` (`launch_open_with_program`) — clones `opened_image_path` and passes it VERBATIM:
   ```rust
   let Some(opened_path) = self.opened_image_path.clone() else { return; };
   if platform::open_file_with_selected_program(&opened_path, program_launch_index).is_err() { ... }
   ```
4. `src/platform/windows_open_with.rs:137-151` (`file_data_object`) — encodes the path wide VERBATIM and feeds `SHCreateItemFromParsingName` with NO conversion:
   ```rust
   let wide_path: Vec<u16> = file_path.as_os_str().encode_wide().chain([0]).collect();
   let shell_item: windows::core::Result<IShellItem> =
       SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None);
   ```
5. API contract: MSDN (learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname): `[in] pszPath — A pointer to a display name` — a shell PARSING name, not a verbatim device path. The function routes the name through the desktop folder's parse (the ReactOS reference implementation, `dll/win32/shell32/CShellItem.cpp`, calls `SHParseDisplayName(pszPath, ...)`). A `\\?\`-prefixed verbatim path is not a filesystem parsing name → `E_INVALIDARG`. Runtime confirmation: the operator log shows `shcreateitemfromparsingname: created=false code=0x80070057` (E_INVALIDARG) on EVERY attempt, before `bindtohandler-bhid_dataobject` is ever reached — the Wave-6.1 `BindToHandler(BHID_DataObject)` fix was correct but unreachable.

Conclusion: `open_image`'s `fs::canonicalize` output → `opened_image_path` → verbatim into `SHCreateItemFromParsingName` = the 100% failure. Suspects B (pairing), C (Invoke contract), D (COM state) are ELIMINATED by the same log: the enumeration/pairing lines are perfect (12 handlers, `launch-index: resolved=true` on all three attempts).

### §0.2 Why opening works but launching does not (correction to the caller's wording)

The WIC decode is invoked with the PRE-canonicalization path — `src/app.rs:72` calls `image_loading::load_image(&path)` BEFORE `fs::canonicalize` runs on line 73. So the decode never even sees the canonical path in the open-image flow; and WIC's `CreateDecoderFromFilename` (`src/platform/windows.rs:94-110`) performs file-handle access that tolerates verbatim paths anyway. The navigation listings also re-canonicalize internally (`src/folder_listing.rs:43`/`:74`, `src/sibling_folder_listing.rs:120`/`:148`) and std `fs` APIs accept `\\?\`. ONLY the shell parsing-name hand-off rejects the prefix.

### §0.3 Other shell-API consumers of the canonical path — grep audit (COMPLETE)

- `SHCreateItemFromParsingName`: ONE call site only — `src/platform/windows_open_with.rs:141`. No other consumer in the codebase.
- `show_open_image_dialog` (`src/platform/windows.rs:31-82`): the owner-HWND path takes NO file path; its `IShellItem` comes from `dialog.GetResult()` and it only reads a normal path OUT via `GetDisplayName(SIGDN_FILESYSPATH)`. Unaffected.
- `SHAssocEnumHandlers` (`windows_open_with.rs:87`): consumes the extension query string from `Path::extension()` (`extension_query_string`, lines 128-135) — tolerant of the `\\?\` prefix, and the diagnostic proves the enumeration succeeds (12 handlers). Unaffected.
- `windows_collation` (`StrCmpLogicalW`): name strings only, no paths. Unaffected.

→ The fix needs exactly ONE call-site edit.

## §1 The evidence-decided fix (decisions, pinned)

A LOCALIZED INPUT-FORM CONVERSION at the shell hand-off — NOT a semantics change (no handler filtering, no enumeration change, no behavior change anywhere else). Therefore, per the TODO's binding gate, NO caller approval is required.

- **D1 — Function location**: pure function inside `src/open_with_listing.rs` (NOT a new `src/shell_paths.rs`). Rationale: it is the established pure seam of the open-with launcher chain, already registered in `src/main.rs:12`, already has the container-testable tests child module (`src/open_with_listing/tests.rs`); a new module would add a `main.rs` mod line + structure-map churn for a single-function concern.
- **D2 — Name and signature** (self-documenting, 1 param, private-by-default pub(crate) consistent with the file's existing items):
  ```rust
  #[allow(dead_code)] // consumed by the Windows-side launcher (this target: stub path)
  pub(crate) fn shell_parseable_path(canonical_path: &Path) -> PathBuf
  ```
  The `#[allow(dead_code)]` + comment mirrors the existing pattern (e.g. `open_with_listing.rs:50`) because the only production consumer is the Windows-side launcher and container builds compile the stub path.
- **D3 — Conversion rule** (only when the prefix exists; case/unicode/name bytes preserved as-is otherwise):
  - `\\?\C:\x.png` → `C:\x.png` (strip the verbatim prefix);
  - `\\?\UNC\server\share\f.png` → `\\server\share\f.png` (UNC device path → network-share form);
  - a path WITHOUT the prefix passes through unchanged (same `PathBuf` content);
  - the UNC check runs FIRST (a `\\?\UNC\...` path also starts with `\\?\`).
- **D4 — Named constants** (no magic strings):
  ```rust
  const VERBATIM_PATH_PREFIX: &str = r"\\?\";
  const VERBATIM_UNC_PATH_PREFIX: &str = r"\\?\UNC\";
  ```
  (raw strings are legal with a trailing backslash — no escaping exists inside `r"..."`).
- **D5 — String representation**: the conversion operates on `as_os_str().to_string_lossy()` (one deterministic code path). Accepted edge (recorded): on exotic names containing invalid Unicode (unpaired surrogates, WTF-8), the hand-off COPY may replace those characters with U+FFFD — the shell then fails with the existing single failure message; the STORED canonical `opened_image_path` and every other consumer are NEVER touched.
- **D6 — The ONE call-site edit**: inside `file_data_object` (`src/platform/windows_open_with.rs`), convert BEFORE `SHCreateItemFromParsingName`. The enumeration call (`raw_handlers` → `extension_query_string`) keeps receiving the canonical path — it only reads `.extension()` and is proven working.
- **D7 — Diagnostic module UNTOUCHED**: `src/platform/launch_log.rs` and every `#[cfg(... feature = "diagnostic-launch-log")]` hook stay exactly as landed (the caller's keep-as-optional decision). NO removal step in this task — this SUPERSEDES the global plan Cycle-B line "the diagnostic-code REMOVAL" (the 2026-11-13 binding modification made the diagnostic an OFF-by-default optional module). The release rebuild never enables the feature; the diagnostic simply remains OFF by default. With the fix in place, a future diagnostic run would log `shcreateitemfromparsingname: created=true`.
- **D8 — No Cargo/feature/lockfile changes**: `Cargo.toml` (six windows features + `diagnostic-launch-log`) and `Cargo.lock` untouched.

## §2 Exact code edits (file-by-file)

### §2.1 `src/open_with_listing.rs` (76 → ~103 lines; ≤200 cap holds)

1. Add the import after the header comment block (the file currently has NO `use` statements):
   ```rust
   use std::path::{Path, PathBuf};
   ```
2. Extend the module header doc with ONE sentence after the launch-index-contract paragraph (self-documenting module purpose):
   `The launcher's shell hand-off additionally converts the app's canonical opened-image path to its shell-parseable form (shell_parseable_path): SHCreateItemFromParsingName requires a shell parsing name and rejects the Windows extended-length \?\ verbatim prefix.`
3. Add the two constants (§1 D4) immediately above the function, and the function placed after `handler_name_is_blank` (the last private fn) and before `#[cfg(test)] mod tests;`:
   ```rust
   pub(crate) fn shell_parseable_path(canonical_path: &Path) -> PathBuf {
       let path_text = canonical_path.as_os_str().to_string_lossy();
       if let Some(network_path) = path_text.strip_prefix(VERBATIM_UNC_PATH_PREFIX) {
           return PathBuf::from(format!(r"\\{network_path}"));
       }
       if let Some(local_path) = path_text.strip_prefix(VERBATIM_PATH_PREFIX) {
           return PathBuf::from(local_path);
       }
       canonical_path.to_path_buf()
   }
   ```
   (Body ~9 lines — ≤50 ✓; max nesting depth 2 ✓; 1 param ✓.)

### §2.2 `src/platform/windows_open_with.rs` (156 → ~157 lines; ≤200 cap holds)

1. Extend the existing import (lines 38-40) with the new function:
   ```rust
   use crate::open_with_listing::{
       OpenWithLaunchFailure, RegisteredProgram, programs_from_handler_names, shell_parseable_path,
   };
   ```
2. In `file_data_object` (line 137), insert the conversion as the FIRST statement and consume it (the ONLY production edit; everything below — the wide encode, `SHCreateItemFromParsingName`, the `shell_item_created` diagnostic hook, `BindToHandler` — is unchanged):
   ```rust
   unsafe fn file_data_object(file_path: &Path) -> windows::core::Result<IDataObject> {
       unsafe {
           let shell_path = shell_parseable_path(file_path);
           let wide_path: Vec<u16> = shell_path.as_os_str().encode_wide().chain([0]).collect();
           // ... rest of the existing body unchanged ...
   ```
3. Extend the module header doc's mechanism sentence (lines 9-11) with ONE clause (accuracy of the recorded mechanism): after "...an IDataObject built from the current canonical file", insert "— converted first to its shell-parseable form (`fs::canonicalize`'s `\?\` verbatim prefix is not a shell parsing name; `SHCreateItemFromParsingName` rejects it with E_INVALIDARG — the Wave 6.2 D1 root cause)" — then continue "...via IShellItem::BindToHandler(BHID_DataObject)".

NO other file changes. `src/app.rs`, `src/app/open_with_button.rs`, `src/platform/windows.rs`, `src/platform/launch_log.rs`, `src/main.rs`, `Cargo.toml`, `Cargo.lock` — ALL untouched.

## §3 Tests (pure logic — container-testable)

Append to `src/open_with_listing/tests.rs` (73 → ~135 lines; ≤200 cap holds). Extend the use line:
```rust
use super::{RegisteredProgram, programs_from_handler_names, shell_parseable_path};
use std::path::{Path, PathBuf};
```

PINNED test names (6 new tests, appended after the existing six; raw-string literals; the first uses the operator's EXACT real path as the diagnostic tie-in):

1. `verbatim_prefixed_local_path_loses_the_prefix` — input `Path::new(r"\\?\C:\_kazuya\imgs\Chino_Kafuu__BOOTH_\Chino Kafuu 1.png")` → `PathBuf::from(r"C:\_kazuya\imgs\Chino_Kafuu__BOOTH_\Chino Kafuu 1.png")` (spaces preserved).
2. `verbatim_unc_path_maps_to_the_network_share_form` — `r"\\?\UNC\server\share\folder\f.png"` → `r"\\server\share\folder\f.png"`.
3. `unprefixed_local_path_passes_through_unchanged` — `r"C:\x.png"` → `r"C:\x.png"`.
4. `plain_unc_path_passes_through_unchanged` — `r"\\server\share\f.png"` → unchanged (a plain UNC path does NOT carry the verbatim prefix).
5. `relative_path_passes_through_unchanged` — `"photo.png"` → unchanged.
6. `verbatim_prefix_conversion_preserves_case_and_non_ascii_names` — `r"\\?\C:\智乃\Chino Kafuu 1.PNG"` → `r"C:\智乃\Chino Kafuu 1.PNG"` (case and non-ASCII preserved verbatim; assertions compare whole `PathBuf` values with `assert_eq!`).

The pure function is fully container-testable: it performs lexical string operations only, so the Linux container exercises it identically to Windows (the prefix bytes are plain ASCII).

**CENSUS PIN: 189 → 195 unit tests / 0 failed across 28 test-bearing files (delta +6; the tests join the EXISTING `src/open_with_listing/tests.rs` — the test-bearing-file count stays 28).**

## §4 Harness verification (implementer, Alpine-VM container)

1. Call `vm_status` FIRST (alpine-vm MCP; only proceed when the VM is running and SSH-reachable; exit code is the authoritative evidence; VM/share unavailable → STOP + user question).
2. Dev-checks:
   ```
   docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
   ```
   Expected: `ALL CHECKS PASSED` (exit 0) with `cargo test` showing **195 passed; 0 failed** (the §3 census pin).
3. Both-target compile (validates the REAL `file_data_object` edit — the container default target compiles the stub path only):
   ```
   docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu
   ```
   Expected: exit 0 (the pre-existing benign dead-code warning on `compare_entry_names` may appear — recorded note).
4. NO diagnostic build and NO operator handoff in this task: the diagnostic stays the OFF-by-default feature; no `--features diagnostic-launch-log` run is needed (the diagnostic module and its hooks are untouched by this fix). The release rebuild belongs to Task 3, NOT here.
5. Minor residual fixes (fmt/clippy drift): minimal local fixes allowed; if a fix cannot land in one more attempt, STOP and report.

## §5 Commits and Git compliance

- Commit 0 (this plan, implementer at 4.2 start): `docs: add wave6.2 task2 launcher fix implementation plan`
- Commit 1 (the implementation, after §2+§3+§4 all pass): `fix: feed the open-with shell item a shell-parseable file path`
- Before EVERY commit: read `.gitignore`, run `git status`; ensure NO gitignored file is staged (`target/`, `dist/`, `logs/checks/`); never force-add ignored files. Commit ONLY the intended files (`src/open_with_listing.rs`, `src/open_with_listing/tests.rs`, `src/platform/windows_open_with.rs` for Commit 1).
- Push restrictions unchanged: NO push in this cycle (`git push` is Step 5 only, `main` → `origin` ONLY).
- Branch: stay on `fix/wave-6-2-open-with-launch`; do NOT create branches or touch `CHANGELOG.md`.

## §6 Review & documentation expectations (subsequent cycle steps — NOT executed by the 4.2 implementer)

- 4.3 code review/simplification: verify the §1 decisions (D1-D8) landed verbatim; verify no other file drifted; expect no simplification beyond formatting.
- 4.4 docs-specialist: the module docs were updated in the implementation itself (§2.1.2, §2.2.3); expect a minimal/no-op documentation step — wave-level documentation (README/tech/architecture/context) belongs to Task 4.
- `.agent/project-structure.md`: NO edit needed (no new module/file; the `open_with_listing.rs` entry already covers the launcher seam) — verify-only at Task 4.

## §7 Scope boundaries (binding for the implementer)

- NO Task 3 (verification/rebuild/checklist) and NO Task 4 (documentation) work.
- The diagnostic module (`src/platform/launch_log.rs`) and all its cfg hooks: UNTOUCHED (no removal, no edits).
- NO Cargo.toml / Cargo.lock / feature-flag changes; NO new source files; NO new module declarations.
- NO changes to: the enumeration list content, the three-point refresh contract, the state gates, the `Q` hotkey, the modal menu, the warning color, or any standing behavior (the TODO's regression list).
- NO editing of existing `.kilo/plans/*` files; NO `CHANGELOG.md`.

## §8 Verbatim compliance vs TODO Task 2

| TODO Task 2 sub-item | Covered by |
|---|---|
| Operator data intake | §0 (the diagnostic lines consumed; recorded in context.md) |
| Root-cause with the evidence | §0.1-§0.3 (the mechanism DECIDED: `SHCreateItemFromParsingName` rejects the `\\?\` verbatim prefix → E_INVALIDARG; suspects B/C/D eliminated by the log's enumeration/pairing lines) |
| Implement the evidence-decided fix | §1-§4 (the localized input-form conversion; the pure function + tests + the one call-site edit) |
| No SEMANTICS change → no caller approval | §1 (no handler filtering, no enumeration change — a localized input-form conversion is NOT a semantics change → **caller approval NOT needed**) |
| WITHOUT regressing: enumeration content, refresh contract, state gates, Q hotkey, modal menu, warning color, standing behaviors | §7 (all untouched; the failure message path keeps its existing single-message collapse) |

## §9 Caller-approval gate status

**NOT REQUIRED.** The fix is a localized input-form conversion at the shell hand-off (derive the shell-parseable path from the canonical one, ONLY when the verbatim prefix exists). It does NOT filter handler kinds, does NOT change the enumeration, and does NOT alter any user-visible behavior other than making the launch succeed — which is exactly the binding operator decision ("Selecting a program MUST open the current file — D1 is a correctness bug, not a behavior change"). The plan may proceed directly to the 4.2 implementer step.
