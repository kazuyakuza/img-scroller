# Amendment 2 — Task 3 Implementation Plan: platform module/file-name alignment + egui Panel API correction

- Date: 2026-10-31
- Parent plan (read together with this file; do NOT edit it): [`.kilo/plans/20261031-task3-wave1-open-image-implementation.md`](20261031-task3-wave1-open-image-implementation.md). First amendment in force: [`20261031-task3-wave1-open-image-fix.md`](20261031-task3-wave1-open-image-fix.md) (target-scoped `windows` manifest — already applied to the tree).
- Amends: parent §2.1.4 (`src/platform/mod.rs` contents) + §2.1.3 `ui()` (two call sites) + §5 adaptation A1 (struck, replaced) + §6 one reference row (A1–A3 → A2–A3).
- Workflow step: Critical Workflow 4.1b MINI AMENDMENT #2 (Task 3), resume 4.2. Implementer was HARD-STOPPED at parent S4/S5 by two defects with captured compiler output (deterministic).
- Plan approval: AUTO-APPROVED ("Approve Global and Tasks Plans", 2026-10-30). Save + return path; no user approval round.
- Scope: ONLY the two defect fixes + their textual consequences. Nothing else changes.

## 1. Verified defect evidence

### D1 — module/file-name mismatch (blocks BOTH dev-checks and cross-target builds)

Parent §2.1.4 declared `mod windows_impl;` / `mod stub_impl;` (as did the §1.2-c wording and the §1.3 cfg-pattern restatement) while §2.1.5/§2.1.6/§2.1.7 and the §4 commit list mandate the FILES `src/platform/windows.rs` / `src/platform/stub.rs`. Rust resolves `mod windows_impl;` → `src/platform/windows_impl.rs` (E0583 file not found). Verified implementer compiler output: `error E0583: file not found for module windows_impl` / `stub_impl` on both S5 (Linux target) and S6 (GNU target) directions. The tree currently holds the S3 draft of `src/platform/mod.rs` with this exact defect.

### D2 — adaptation A1 is INVERTED (blocks S5, compile of `src/app.rs`)

Parent §2.1.3 wrote `egui::TopBottomPanel::top/bottom(...)` and §5 A1 claimed "no `egui::Panel` type exists in egui 0.37". Independently re-verified 2026-10-31 against the resolved egui 0.37.0 (published 2026-10-15) sources/docs:

- `egui::Panel` IS re-exported at the crate root (`src/lib.rs` `pub use self::{containers::{… CentralPanel, … Panel, PanelState, …}}`).
- `src/containers/panel.rs` + docs: `pub struct Panel` with `pub fn top(id_salt: impl AsIdSalt) -> Self`, `pub fn bottom(id_salt: impl AsIdSalt) -> Self` (both "By default this is NOT resizable" — matches frontend-spec §1.3), and `pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R>`. `show_inside` is deprecated (renamed to `show`).
- `TopBottomPanel` is ABSENT from the `panel` module struct list (`Panel`, `CentralPanel`, `PanelState` only) — the parent API does not exist in 0.37.

Conclusion: the 4.1a frontend-spec §4.1a sketch (`egui::Panel::top/bottom(...)`) was the CORRECT API; A1 was wrong and is strike-retracted.

## 2. Decision — option (A), malformed-declaration fix in `mod.rs` only

Keep the mandated file names `src/platform/windows.rs` / `src/platform/stub.rs`; change ONLY `src/platform/mod.rs` to declare `mod windows;` / `mod stub;` with matching `pub(crate) use windows::{…}` / `pub(crate) use stub::{…}` re-exports (cfg gates unchanged as written below in §3).

Why (A) wins:
1. Task-2 plan §1.1-b BINDS the exact file set `src/platform/mod.rs`, `src/platform/windows.rs`, `src/platform/stub.rs` — the file names are the binding artifact; the §1.1-h pattern text ("`mod windows_impl;`") was a pattern description using placeholder aliases, not a file-name mandate. Task-2 §1.1-b is the more specific authority.
2. Parent §2.1.5/§2.1.6/§2.1.7 (file headers), the §4 commit list (names `platform/windows.rs`/`platform/stub.rs`), and the already-written `.agent/project-structure.md` entry all name `windows.rs`/`stub.rs` — option (A) keeps EVERY other deliverable consistent with a single-file delta (`mod.rs` only).
3. Rust 2018+ module resolution decouples module name from file name freely: `mod windows;` resolves `src/platform/windows.rs`; inside `platform/windows.rs` a `use windows::Win32::…` path still resolves to the EXTERN crate `windows` (the local module is `platform::windows`, a child of `platform`, never in the extern-prelude comparison set), and the crate root declares no `mod windows`. No ambiguity, no shadowing.
4. Option (B) (rename files to `windows_impl.rs`/`stub_impl.rs`) is REJECTED: it violates the Task-2 §1.1-b binding file set, contradicts §2.1.5–§2.1.7 headers, the §4 commit list and `project-structure.md`, and forces a three-file delta + doc churn for zero benefit.

## 3. EXACT final `src/platform/mod.rs` (full file; replaces parent §2.1.4 verbatim; LF endings, trailing newline)

```rust
//! OS-bound facade for the native open dialog and image decoding.
//! Both target sides expose the same signatures; see the Task-2 plan §1.1-h.

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
mod stub;

#[cfg(target_os = "windows")]
pub(crate) use windows::{decode_image, show_open_image_dialog};

#[cfg(not(target_os = "windows"))]
pub(crate) use stub::{decode_image, show_open_image_dialog};
```

Semantic content identical to parent §2.1.4; ONLY the module identifiers changed (`windows_impl` → `windows`, `stub_impl` → `stub`) plus declaration grouping. The cfg pattern (paired arms, same signatures both sides, NO third arm per Task-2 §1.1-h) is unchanged.

## 4. EXACT final `ui()` in `src/app.rs` (only the two call sites change; nothing else in app.rs moves)

The final `impl eframe::App` block must read EXACTLY:

```rust
impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::Panel::top("open_toolbar").show(ui, |ui| {
            self.open_toolbar_button(ui, frame);
        });

        if let Some(message) = self.message_text {
            egui::Panel::bottom("message_bar").show(ui, |ui| {
                ui.label(message);
            });
        }

        egui::CentralPanel::default().show(ui, |_ui| {});
    }
}
```

- BOTH changed lines are the `egui::TopBottomPanel::top("open_toolbar")` / `::bottom("message_bar")` calls → `egui::Panel::top("open_toolbar")` / `egui::Panel::bottom("message_bar")`. ids, closure bodies, conditional, comment-free structure: all unchanged.
- The REST of `src/app.rs` stands per parent §2.1.3 AS RESUMABLY WRITTEN (the current tree file matches parent §2.1.3 verbatim today; verified 2026-10-31). Do NOT touch any other line. `cargo fmt` (S4) may re-shape whitespace/lines — whitespace deviations after fmt are NOT defects; semantic content is fixed by this amendment.

## 5. Updated frontend-spec mapping (FULL table; 4.5a reads THIS table as authority — no A1-tolerance note anywhere)

| Frontend-spec item | Location in drafts |
| --- | --- |
| §1.1 button labeled exactly `Open…` (U+2026) | §2.1.3 `open_toolbar_button` |
| §1.2 top toolbar (not Window/Area/modal) | §2.1.3 `ui()` first surface — `egui::Panel::top("open_toolbar")` (spec sketch API verbatim; NO adaptation) |
| §1.3 non-resizable default panel styling | same calls (no custom Frame/height; `Panel::top/bottom` are "By default NOT resizable" in egui 0.37 — default panel styling) |
| §1.4 horizontal layout | §2.1.3 `ui.horizontal` |
| §1.5 click → dialog → `open_image(path)` | §2.1.3 `open_via_dialog` + §2.1.5 `show_open_image_dialog` (naming ADAPTATION A2) |
| §1.6 cancel → no state change, no message | §2.1.5 dialog `Ok`/`Err` → `None` → nothing happens |
| §1.7 toolbar always visible | §1.2-g paint order (no fullscreen logic in Task 3) |
| §2.1 args source line | §2.1.1 `cli_image_path` |
| §2.2 no dialog on CLI launch | §2.1.1 (no dialog call in the CLI path) |
| §2.3 no extra success UI | §2.1.1 + §2.1.3 `ui()` (central surface stays blank on success) |
| §2.4 empty-state = toolbar + blank center | §2.1.3 |
| §2.5 no central hint | §2.1.3 `CentralPanel` no widgets |
| §3.1 message in bottom panel `message_bar` | §2.1.3 `egui::Panel::bottom("message_bar")` (spec sketch API verbatim; NO adaptation) |
| §3.2 label text = verbatim constant | §2.1.2 constant + §2.1.3 `accept_open_outcome` uses `image_loading::UNSUPPORTED_FILE_MESSAGE` directly |
| §3.3 default body text style | §2.1.3 plain `ui.label(message)` |
| §3.4 shown only when `Some` | §2.1.3 `if let Some(message)` |
| §3.5 persists until next successful open | §2.1.3 transition (`Ok` → `message_text = None`) |
| §3.6 non-blocking/non-modal/keeps image | bottom `Panel` only; no dimming; decode failure never clears state |
| §3.7 no OS popup for the message | constant rendered in-window; no MessageBox in any draft |
| §4.1–4.3 no keys, no Esc/Enter, default focus | no input handling anywhere in the drafts |
| §5.1–5.4 three private fields; `new` = all None; single mutator `open_image`; decode via `image_loading::load_image` outside `ui()` | §2.1.3 struct + `empty_state` + `open_image` (`ui()` calls `open_image`, never `load_image` — spec §5.4 literal) |
| §5.5 success transition | §2.1.3 `accept_open_outcome` Ok arm |
| §5.6 failure transition (message set; image state untouched) | §2.1.3 Err arm + tests (`failure_after_success…`) |
| §5.7 no threads/channels/polls | §1.2-g (synchronous pattern) |
| §5.8 payload owned by app struct | `decoded_image: Option<DecodedImage>` field |
| §6.1–6.4 paint order; empty central panel; no modal surface; no per-frame decode/platform calls | §2.1.3 `ui()` — see ADAPTATION A3 for the per-frame interpretation |
| §7 rules table | §2.1.3 note C6 compliance list |
| §8 static checklist | this table = item-by-item mapping; 4.5a reads the checklist against the drafts via this table |
| §9 deferrals | all resolved in §1.2 (c: signatures/home; d: pixel format; e: WIC sequence; f: dialog+COM; message error type = `ImageLoadFailure`) |

Encoded adaptations (REVISED; 4.5a reads WITHOUT any A1 tolerance):
- A1: **STRUCK (retracted).** The spec sketch `egui::Panel::top/bottom(...)` is the correct egui 0.37 API — verified in the resolved egui-0.37.0 source (`src/containers/panel.rs`) and crate-root re-exports; `TopBottomPanel` no longer exists. The corrected §2.1.3 `ui()` matches the spec sketch EXACTLY — there is NO adaptation for the panel-surface rows.
- A2: stands VERBATIM from the parent plan (signature `show_open_image_dialog(frame: &eframe::Frame) -> Option<PathBuf>`).
- A3: stands VERBATIM from the parent plan (per-frame interpretation of the §8 no-platform-calls checklist item).

## 6. Corrected §6 verification row (parent §6 otherwise unchanged)

Parent §6 row "Frontend-spec §8 checklist | §5 mapping table read-through against drafts (with A1–A3) | All rows pass" reads with THIS amendment as: "…against the amended §5 mapping table (adaptations A2–A3 only; A1 struck) | All rows pass". No other §6 row references the panel API or the module names.

## 7. Unchanged-content confirmation (explicit)

1. Fix-amendment-1 stands in full: final `Cargo.toml` = its §1 target-scoped manifest, verbatim (tree already holds it).
2. Parent §2.1.1 `src/main.rs`, §2.1.2 `src/image_loading.rs`, §2.1.5 `src/platform/windows.rs` (with its binding draft notes), §2.1.6 `src/platform/stub.rs`, §2.1.7 `.agent/project-structure.md`: stand VERBATIM. Only `src/platform/mod.rs` (§3 here supersedes §2.1.4) and the two `ui()` call sites (§4 here supersedes that fragment of §2.1.3) change.
3. Parent §1.2-g / §1.3 cfg-pattern wording: read as corrected by §2–§4 of this amendment (`mod windows;`/`mod stub;`, `egui::Panel`). The pattern itself (paired cfg arms, identical signatures, no third arm) is untouched.
4. Parent steps S1–S3: DONE (S3 drafts landed). S4/S5/S6/S7/S8 commands, timeouts, semantics: unchanged except as §8 here sequences the resume.
5. Parent §7 risks: stand (S6 micro-fix list category 6 import-path corrections unchanged). New amendment-local risks are in §9.
6. §4 commit: file list UNCHANGED — `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/app.rs`, `src/image_loading.rs`, `src/platform/mod.rs`, `src/platform/windows.rs`, `src/platform/stub.rs`, `.agent/project-structure.md` — NO new files, NO removed files. Message unchanged: `Open images via CLI argument or native dialog with WIC decode`.

## 8. Resume instructions for 4.2 (work-tree holds the S3 drafts)

Current observed tree (verified 2026-10-31): `M Cargo.toml` (fix-amendment-1 §1 manifest, applied), `M Cargo.lock`, `M src/main.rs`, `M src/app.rs`, `M .agent/project-structure.md`, `M .agent/project-info/context.md`, `?? src/image_loading.rs`, `?? src/platform/`, plus the four `?? .kilo/plans/20261031-task3*` plan/spec files (workflow artifacts — never staged by the Task-3 commit list).

- R0. Preconditions (re-run): `alpine-vm_vm_status` first; host `git status` → expect exactly the state above (plus plan artifacts); any OTHER drift → STOP + report.
- R1. Edit `src/platform/mod.rs` (host file tools) to the EXACT §3 content full-file (replace all 14 current lines).
- R2. Edit `src/app.rs`: apply ONLY the two call-site replacements of §4 (TopBottomPanel → Panel in `ui()`). No other line.
- R3. Parent S4 VERBATIM: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` — `timeoutMs: 120000`; EBUSY → retry identical once, then fmt-check-output-guided host whitespace fixes (parent rule). Whitespace-only reshaping of the §4 closure by fmt is expected, not a defect.
- R4. Parent S5 VERBATIM: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — `timeoutMs: 1200000`. Expected: exit 0; per-check `PASS fmt-check exit=0`, `PASS check exit=0`, `PASS test exit=0`, `PASS clippy exit=0`; `test result: ok. 8 passed`; final `ALL CHECKS PASSED`. Failure handling = parent S5 categories verbatim (trivial in-draft fixes allowed; structural → STOP with captured output).
- R5. Parent S6, DF-GATE THEN COMMAND: first a disk-space gate on the VM — `vm_status`, then `df -h` via `alpine-vm_vm_run_command`; if the filesystem hosting the cargo target volume/share shows LESS than 4 GiB available → STOP + report before the compile. Then the parent S6 command VERBATIM: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu` — `timeoutMs: 1800000`; expected exit 0; the parent §3-S6 six-category micro-fix list binds EXACTLY as written; unused windows features NEVER warn and stay binding-pinned (do not remove); anything beyond the six categories → STOP + report with full output.
- R6. Parent S7 commit per §4 + amendment §7 item 6: read `.gitignore`; `git status` hygiene; stage exactly the 9-file list (NO `git add .`, NO plan/TODO/context files); single commit with the exact message; `git status` clean after; NO push, NO branch ops, NO `CHANGELOG.md`, NO edits to existing `.kilo/plans/*`.
- R7. Parent S8 summary VERBATIM, plus: record that fix-amendment-1 (manifest fallback) and THIS amendment (mod.rs declarations + two `ui()` call sites + A1 strike) were applied; operator-run items stay OPERATOR-PENDING (Task 5).

## 9. Risks unique to this amendment

| # | Risk | Handling |
| --- | --- | --- |
| F1 | Implementer "helpfully" renames files to `windows_impl.rs`/`stub_impl.rs` or renames the module identifiers in any other file | FORBIDDEN: file names are the Task-2 §1.1-b binding; the ONLY name changes are inside `src/platform/mod.rs` per §3. Any other rename = STOP. |
| F2 | Name-conflict worry: `mod windows;` vs the extern `windows` crate inside `platform/windows.rs` | NOT a conflict (option-A rationale item 3): `use windows::…` resolves to the extern crate; the local module is `platform::windows`, and the crate root declares no `mod windows`. S5/S6 will prove it. |
| F3 | `cargo fmt` reshapes the §4 block (e.g. joins/wraps the `Panel::top(...).show(...)` chain) and the implementer suspects drift | Whitespace-only reflow is EXPECTED; only semantic content (ids, method names, closures, condition) is fixed. Do not "fix back" fmt output. |
| F4 | egui 0.37 `Panel::show` vs deprecated `show_inside` | Use ONLY `.show(ui, …)` (as §4 does). `show_inside` is deprecated in 0.37 and would trip clippy `-D warnings`. |
