# Wave 6.1 — Task 5 (Cycle C) Verification Plan (Critical Workflow step 4.1b, architector)

- Source TODO: `.agent/todos/20261113/20261113-todo-1.md` → `### Task 5: Verification` (3 sub-items).
- Global plan: `.kilo/plans/20261113-wave6-1-open-with-feedback.md` → `### Cycle C — Task 5 (verification)`.
- Executor: JUNIOR developer under 50% restriction — every step below is FINAL; no redesign, no extra checks, no skipped steps. Minor latitude: none beyond recording observed values in the results report.
- Scope of THIS cycle: VERIFICATION ONLY. **NO application-code edits** (the single allowed exception is the mechanical `cargo fmt` re-run in Step 1's failure handling). NO Task 6 documentation work. NO edits to existing `.kilo/plans/*` files. No merge, no push (Step 5 of the global plan is NOT this cycle).
- Requirements binding: the OPERATOR-RUN checklist MUST be presented CLEARLY AND PLAINLY in the chat at the wave close (short self-contained numbered items, NOT plan-file-only references) — the Planner copies §9 VERBATIM into the chat. This plan carries the plain checklist for that purpose.

## §0 BASELINE FACTS (verified 2026-11-13 at HEAD `c51315a`)

| Fact | Value | Source |
| --- | --- | --- |
| Branch / HEAD | `fix/wave-6-1-open-with-feedback` at `c51315a` | caller context; Step 0 re-verifies |
| Version | 0.6.1 (`Cargo.toml` line 3) | verified this session |
| Census (SETTLED by 4.5b adjudication) | **189 unit tests / 28 test-bearing files** = 23 `*_tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `zoom.rs`, `scaling.rs`); settled method: `rg -l "#[test]" src` → 28 files | verified this session (`rg -c "#[test]" src` sums to exactly 189) |
| Prior exe (0.6.0 artifact to be overwritten) | `dist/windows-x64/img-scroller.exe` = 19,647,219 bytes | docs/build-windows.md + context |
| Recorded DLL audit constant | 22 DLLs (§5 lists them verbatim) | docs/build-windows.md |
| D1 fix state | `windows_open_with.rs` uses `IShellItem::BindToHandler(BHID_DataObject)`; the array route (`SHCreateShellItemArrayFromShellItem`) removed | verified this session (file lines 9–15, 27–31) |
| Tasks 2–4 state | Q key constant, `Some("Q")` caption, modal guard, 5 D2 gates, warning predicate — all landed (line anchors in §6) | verified this session |
| Harness | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` (fmt-check/check/test/clippy; `ALL CHECKS PASSED` / `N CHECK(S) FAILED`; logs under gitignored `logs/checks/`) | docs/build.md |
| Wave commits so far (chronological) | `097d2e4` (report+TODO), `5a6ed36` (global plan), `46d1d22` (0.6.1 bump), `18ed719` (T1 plan), `befcf9e` (D1 fix), `ad73b4e` (T1 mark), `decc537` (T2-4 spec+plan), `3e74a2f` (feature), `85fe6b0` (simplification), `c7fad26` (T2-4 docs), `c51315a` (T2-4 marks) | caller context |
| Untracked wave artifact | `.kilo/plans/20261113-wave6-1-tasks2-4-frontend-verification.md` (the 4.5a report) — committed in Step 0 | caller context |

## §1 FAILURE HANDLING (applies to EVERY step)

- Expected outcomes are pinned per step. On ANY miss: **STOP** and return the EXACT failure to the caller (command + exit code + the relevant output lines). NO silent retries.
- The ONE exception: a `fmt-check` FAIL from mechanical rustfmt wrapping → apply `cargo fmt` (the only allowed code-touching action), re-run the SAME dev-checks command ONCE. A second failure → STOP.
- A compile/clippy/test failure in Step 1 or 2 is a WAVE DEFECT, not a verification issue → STOP; do NOT fix code.
- VM/`/img-scroller` share unavailable at any harness step → STOP + user question (docs/build.md).
- Every harness step: call `alpine-vm_vm_status` FIRST (immediately before the command); the command EXIT CODE is the authoritative evidence; absent stderr is not a failure signal (docs/build.md).

## §2 STEP 0 — PRE-FLIGHT + PLAN COMMIT (no verification yet)

1. `alpine-vm_vm_status` — VM running + SSH-reachable. Else STOP + user question.
2. Host (bash tool, single commands): `git rev-parse --abbrev-ref HEAD` → `fix/wave-6-1-open-with-feedback`; `git status --porcelain` → exactly one entry: `?? .kilo/plans/20261113-wave6-1-tasks2-4-frontend-verification.md` (plus this plan file, also untracked, once created). Branch or extra entries → STOP.
3. Read `.gitignore` (covers `dist/`, `target/`, `logs/`, `*.log`) — required by the Gitignore Compliance Rule before every commit below.
4. Commit 1 — stage ONLY `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` (this file) and `.kilo/plans/20261113-wave6-1-tasks2-4-frontend-verification.md` (the untracked 4.5a report). Message: `docs: add wave6.1 task5 verification plan and the tasks2-4 frontend verification report`. Re-run `git status` → clean tree. (Decision pinned: the 4.5a report rides the PLAN commit, so Steps 1–7 run on a clean tree; the results commit in Step 8 stays single-purpose.)

## §3 STEP 1 — FRESH DEV-CHECKS AT HEAD + CENSUS CORROBORATION

Expected: exit `0`, final line `ALL CHECKS PASSED`, per-check `PASS` lines for `fmt-check`, `check`, `test`, `clippy`, and the test output containing **`189 passed; 0 failed`**.

1. `vm_status` first.
2. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
3. Census corroboration (the SETTLED method — file set via the `#[test]`-bearing-file list; count via the cargo test line):
   - `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "cd /img-scroller && grep -rl '#\[test\]' src | wc -l"` → **28**.
   - The same command without `| wc -l` (record the listing in the results): exactly 23 paths matching `*_tests.rs` and exactly 5 others (`src/app_display.rs`, `src/image_loading.rs`, `src/name_collation.rs`, `src/zoom.rs`, `src/scaling.rs`).
   - If `rg` is available the equivalent settled form `rg -l "#\[test\]" src` may be used instead of grep; expectations identical.
   - The 189 count itself = the cargo test line in the dev-checks output (already captured in this step's output).
4. Any deviation → §1 failure handling.

## §4 STEP 2 — WINDOWS-TARGET COMPILE

Expected: exit `0`. The pre-existing benign dead-code warning on `compare_entry_names` MAY appear (the Wave-5.2 recorded note) — benign. **NO NEW warnings** beyond that one.

1. `vm_status` first.
2. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "cd /img-scroller && cargo check --locked --target x86_64-pc-windows-gnu"`
3. This is the step that compiles the real `platform/windows_*` modules (`windows_open_with.rs`, `windows_collation.rs`) — the dev-checks `cargo check` covers the stub path only.
4. A compile error (e.g. around `BindToHandler`) → §1: STOP with the full diagnostics; do NOT redesign anything.

## §5 STEP 3+4 — RELEASE REBUILD + DLL IMPORT AUDIT

### Step 3 — release rebuild (sub-item 2 of TODO Task 5)

1. `vm_status` first.
2. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` → expected exit `0`; the typed-failure table in docs/build-windows.md lists the `build-windows: error:` lines (any of them → STOP with the exact line).
3. Record the byte size: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "wc -c < /img-scroller/dist/windows-x64/img-scroller.exe"` → record the OBSERVED byte count in the results. Expectations: non-empty, and DIFFERENT from the 0.6.0 artifact's 19,647,219 bytes (the 0.6.1 rebuild overwrites it atomically — a byte-identical result would be a defect signal → STOP).
4. On the host the artifact appears at `dist\windows-x64\img-scroller.exe` (gitignored — never staged).

### Step 4 — DLL import audit (sub-item 2 of TODO Task 5)

1. `vm_status` first.
2. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "cd /img-scroller && x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep 'DLL Name' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u"` (per docs/build-windows.md; record the OBSERVED list in the results).
3. Expected: **SET-EQUAL** to the recorded 22-DLL constant, verbatim:
   `advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll`
4. Rationale for set-equal (the D1 fix): `BindToHandler` is a virtual call through the already-imported shell COM surface; the removed `SHCreateShellItemArrayFromShellItem` and the kept `SHAssocEnumHandlers`/`SHCreateItemFromParsingName` are all `shell32.dll` exports (T1 plan §1). The audit MUST still verify — any delta → STOP and return the exact observed-vs-recorded diff; an "explained delta" requires the CALLER's acceptance, never the implementer's.

## §6 STEP 5 — WAVE-PIN SWEEP (host, bash tool; `rg` single commands)

Every check below must MATCH; any miss → STOP with the exact evidence.

1. **D1 fix site**: `rg -n "BindToHandler" src` → present ONLY in `src/platform/windows_open_with.rs` (module doc + `file_data_object`). `rg -n "SHCreateShellItemArrayFromShellItem" src` → **zero matches** in the whole tree (no residue — the import was removed).
2. **Q key constant + caption**: `rg -n "OPEN_WITH_MENU_KEY" src` → `pub(super) const OPEN_WITH_MENU_KEY: egui::Key = egui::Key::Q;` at `src/app/open_with_button.rs:33`. `rg -n 'Some\("Q"\)' src/app` → the mapping arm at `src/app/button_hint.rs:66` + the test assertion in `src/app/button_hint_tests.rs:18`. `rg -n "Open with\.\.\. \(Q\)" src/app` → the enabled tooltip (`button_hint.rs`) + its test.
3. **Modal guard placement**: `src/app/keyboard_intake.rs` — `update_esc_close_popup(screen_context);` is the FIRST statement of `read_and_dispatch_keystrokes` (line 35); the early-return guard `if self.handle_modal_open_with_menu_keys(screen_context) { return; }` (lines 36–38); guard body at lines 136–144 queries `open_with_button::open_with_menu_is_open` and closes via `Popup::close_id` for Q or ESC — no `ViewportCommand::Close` and no ESC-close arming inside the guard.
4. **D2 gates — five sites** (each contains `!open_with_button::open_with_menu_is_open(row_ui.ctx())` or the local equivalent): arrows `src/app/floating_controls.rs:186`; fullscreen `src/app/fullscreen_button.rs:32`; zoom `src/app/zoom_button.rs:93`; fit `src/app/fit_button.rs:89`; open-with refresh `src/app/open_with_button.rs:91`. (`rg -n "open_with_menu_is_open" src` → exactly these sites plus the definition/open_with_button internals/guard.)
5. **Warning color predicate**: `src/app/open_with_button.rs:135–137` — `message_line_is_warning(message)` equals `message == Some(OPEN_WITH_LAUNCH_FAILURE_MESSAGE)`; `src/app/pill_style.rs:38` — `pub(super) const WARNING_MESSAGE_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 165, 0);`.
6. **Zero Cargo drift since the 0.6.1 bump**: `git diff 46d1d22..HEAD -- Cargo.toml Cargo.lock` → EMPTY output (exit 0, no hunks).

## §7 STEP 6 — GIT SEGMENTATION

1. `git status --porcelain` → EMPTY (clean tree; `dist/`, `logs/`, `target/` are gitignored and must not appear).
2. `git log --oneline -15` → the wave chain present, HEAD on top (the Step-0 plan commit), then exactly: `c51315a`, `c7fad26`, `85fe6b0`, `3e74a2f`, `decc537`, `ad73b4e`, `befcf9e`, `18ed719`, `46d1d22`, `5a6ed36`, `097d2e4` (newest→oldest; then pre-wave commits). Commit-message wording is not the check; SHA presence + order is.
3. Still on branch `fix/wave-6-1-open-with-feedback` (no merge, no push — Step 5 of the global plan is NOT this cycle).

## §8 STEP 7 — THE OPERATOR-RUN CHECKLIST (PLAIN FORM — §9 carries it verbatim)

- Manual UX is OPERATOR-RUN ONLY (agent-side GUI/runtime execution is impossible from the Linux container). This cycle PRODUCES the checklist; the PLANNER presents §9 verbatim in the chat at the wave close (the operator's clarity requirement). The TODO Task-5 manual-UX sub-item STAYS `[ ]` until the user reports.

## §9 OPERATOR-RUN CHECKLIST — COPY VERBATIM INTO THE CHAT

Run on a real Windows 10/11 machine, using the freshly rebuilt 0.6.1 exe at `dist\windows-x64\img-scroller.exe`, with an image open in a folder whose file type has registered programs:

1. D1 FIXED — select a program in the "Open with..." list: the CURRENT file opens in that program (no failure message).
2. Press `Q`: the "Open with..." menu opens (when programs exist for the file type).
3. While the menu is open, press other keys and click other buttons (arrows, zoom, fit, fullscreen): they all do NOTHING.
4. Press `ESC` while the menu is open: the menu closes, and the "Press ESC again to close" app-exit popup does NOT appear.
5. Press `Q` again: the menu closes.
6. Click outside the menu: the menu dismisses.
7. `Q` stays inert when: no programs are registered for the type, in empty-folder mode, and in fullscreen.
8. If a failed launch is reproducible: the failure message shows in WARNING (orange) color on the pill's message line.
9. The `Q` caption renders under the open-with button symbol (white, thin black outline).
10. Regression block — previous functionalities still work: arrow navigation and folder navigation; zoom in/out/restore (`I`/`O`/`0`, Shift+'+' / Shift+'−'); fit width/height (`W`/`H`); fullscreen (`F11`, ESC exit); double-ESC close contract; pan at zoomed image edges; navigation name order; opening files.

STANDING NOTE (do not skip): if item 1 STILL fails 100% of selections on the 0.6.1 exe → the next step is the DIAGNOSTIC debug-build fallback (a temporary DEBUG-ONLY build showing per-selection invoke result codes), which REQUIRES explicit caller authorization before any work — plan `.kilo/plans/20261113-wave6-1-task1-launcher-fix-implementation-plan.md` §6; it is never an assumption and never ships in a release artifact.

## §10 STEP 8 — RESULTS RECORDING + COMMIT PROTOCOL

1. Create the results report `.kilo/plans/20261113-wave6-1-task5-verification-results.md` (NEW file; this cycle's only other artifact) recording, per step: the command, the exit code, the observed values — dev-checks verdict + `189 passed; 0 failed` + the 28-file listing (23 `*_tests.rs` / 5 in-file), windows-target compile result + any warnings seen, the OBSERVED exe byte count, the OBSERVED 22-DLL audit list + set-equal verdict, the six sweep checks PASS/FAIL, and the git segmentation output. Verdict line: VERIFIED or STOPPED (with the exact failure).
2. Read `.gitignore` + `git status` (Gitignore Compliance Rule). Stage ONLY the results file. NO `dist/`, `logs/`, `target/`, `*.log` staged (gitignored — never force-add).
3. Commit 2 message: `docs: add wave6.1 task5 verification results`.
4. Re-run `git status` → clean tree.

## §11 FOLLOW-THROUGH (NOT executed by this plan)

- Task 5 4.6 step (separate): mark TODO sub-items 1 (dev-checks/verification) and 2 (rebuild + DLL audit) `[x]`; the manual-UX sub-item STAYS `[ ]` (the user's OPERATOR-RUN gate).
- Wave close: the Planner presents §9 verbatim in the chat; Task 6 (documentation, Cycle D) follows.

## §12 SCOPE BOUNDARIES (HARD)

- NO application-code changes (except the single mechanical `cargo fmt` in §1's fmt failure path). NO Cargo.toml/Cargo.lock changes. NO `CHANGELOG.md`. NO edits to existing `.kilo/plans/*` files. NO merge/push. NO diagnostic build. NO Task 6 documentation edits.
- TODO Task-5 sub-item mapping: sub-item 1 = §3 (dev-checks + census); sub-item 2 = §4 + §5 (windows compile, rebuild, byte record, DLL audit); sub-item 3 = §8/§9 (the plain OPERATOR-RUN checklist, presented in chat by the Planner).

## §13 SOURCES

- `.agent/todos/20261113/20261113-todo-1.md` (Task 5 verbatim + execution notes).
- `.kilo/plans/20261113-wave6-1-open-with-feedback.md` (Cycle C).
- `.kilo/plans/20261113-wave6-1-task1-launcher-fix-implementation-plan.md` (§1 DLL-audit rationale; §6 diagnostic-fallback trigger).
- `.kilo/plans/20261113-wave6-1-tasks2-4-ui-feedback-implementation-plan.md` (D1–D12; census 189/28 pin; sweep anchors).
- `.kilo/plans/20261113-wave6-1-tasks2-4-frontend-verification.md` (the 4.5a report — committed in Step 0).
- `docs/build.md`, `docs/build-windows.md` (harness, rebuild, audit).
- Landed code verified this session: `src/platform/windows_open_with.rs`, `src/app/open_with_button.rs`, `src/app/keyboard_intake.rs`, `src/app/button_hint.rs`, `src/app/pill_style.rs`, `src/app/floating_controls.rs`, `src/app/zoom_button.rs`, `src/app/fit_button.rs`, `src/app/fullscreen_button.rs`, `Cargo.toml`.
