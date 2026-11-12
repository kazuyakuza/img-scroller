# Wave 5.1 Task 5 — Verification Plan 4.1b (FIRST branch harness run — discovery semantics, census 164 corroboration across 26 files, wave-pin sweeps incl. the round-2 alias accept-set / expiry wake / collation seam + the no-touch list, 0.5.1 release rebuild + byte-size record + the Windows-collation compile evidence, DLL import audit set-equal 22 as the deciding evidence, Git segmentation, 0.5.1 OPERATOR-RUN checklist)

- Date: 2026-11-12 (plan author: architector, workflow step 4.1b; executed by the implementer in 4.2).
- Source TODO: [`.agent/todos/20261112/20261112-todo-1.md`](../../.agent/todos/20261112/20261112-todo-1.md) → `### Task 5: Verification` ONLY (exactly its THREE sub-items) + the wave's `## Future session execution notes`. NO other task's plans are expanded or executed; the Task-5 `[DONE]` mark is the separate 4.6 step (§10; NOT executed inside 4.2 — NO todo edits in 4.2).
- NOT front-end ⇒ sub-step **4.1b ONLY** (no 4.1a exists for Task 5).
- Workflow slot: Wave-5.1 Critical Workflow (global plan [`.kilo/plans/20261112-wave5-1-operator-feedback-global-plan.md`](20261112-wave5-1-operator-feedback-global-plan.md) → Task 5, step 4.1b) — this PLAN → executed by the implementer in 4.2.
- Build docs (binding usage): [`docs/build.md`](../../docs/build.md), [`docs/build-windows.md`](../../docs/build-windows.md); harness scripts: [`scripts/dev-checks.sh`](../../scripts/dev-checks.sh), [`scripts/build-windows.sh`](../../scripts/build-windows.sh).
- Precedent followed (structure, command table, census rule, mark semantics, checklist block): [`.kilo/plans/20261107-wave5-task4-verification-plan.md`](20261107-wave5-task4-verification-plan.md) §0–§14 — same house shape, updated to the Wave-5.1 landed state and the FIRST-run-on-branch discovery semantics.
- **DISCOVERY semantics (binding, caller-provided): NO harness run has EVER happened on this branch.** The Tasks 2–4 fixes (`0d15cc0`, `481af1e`, `788b312`) have NEVER compiled — the FIRST §3 step-4 run IS the discovery step: fmt and clippy residuals are EXPECTED possibilities, and §7 sanctions their in-cycle minimal fixes (fmt round FIRST, then compile/clippy residuals). This differs from the Wave-4.2 precedent (where check-fail = immediate STOP) BY DESIGN — the caller pinned the compile-fix escalation rule.
- **Windows-target compile expectation (binding, caller-provided — pinned as a fact of the harness, NOT a defect):** the container dev-checks build compiles the NON-Windows (stub) path ONLY — `src/platform/windows.rs` + `src/platform/windows_collation.rs` are cfg-gated behind `#[cfg(target_os = "windows")]` (`src/platform/mod.rs:5–9`) and are NOT part of the container compile. The Windows-collation code's compile-correctness is therefore NOT harness-verifiable in the dev-checks; its AUTHORITATIVE compile evidence is §3 step 8 — the release cross-build (`cargo build --release --locked --target x86_64-pc-windows-gnu`) DOES compile the windows target — plus the OPERATOR-RUN (runtime evidence). §4 records this expectation explicitly; a green dev-checks harness says NOTHING about the windows-target code.
- Approval: the user's global-plan approval makes every decision below BINDING; zero judgment left for the implementer. If any detail conflicts with something ACTUALLY observed during 4.2, STOP and return the question to the caller (§14).
- Target implementer: JUNIOR under 50% restriction. All structural/scope decisions are encoded here; §13 lists the out-of-scope guard verbatim.

---

## §0 Session facts captured at plan time (verified evidence; branch `fix/wave-5-1-operator-feedback` @ HEAD `b25677d`)

- Branch: `fix/wave-5-1-operator-feedback`; HEAD **`b25677d`** (`task: mark wave5.1 task4 done`); working tree CLEAN at plan time (porcelain empty). The wave chain below the tip (verified by host `git log --oneline -20`): `e96f4a1` (task4 module docs) → `645cea3` (task4 review fixes: test expectation and structure map) → `788b312` (**Task 4 feat: follow the system name order for entry navigation** — LAST code commit) → `68f3b6c` (task4 plan) → `c38a8df` (task3 mark) → `38ac100` (task3 module docs) → `481af1e` (**Task 3 fix: wake the event loop so the esc-close popup expires after two seconds**) → `140404d` (task3 plan) → `5b0071d` (task2 mark) → `0d15cc0` (**Task 2 fix: accept the shifted plus variant for the zoom-in alias**) → `1eec11d` (task2 plan) → `88ef3d8` (task1 mark) → `8a8d395` (report recording) → `8b9894a` (**chore: bump version to 0.5.1**) → `cb1b174` (global plan) → `718d3a6` (todo) → `b1435e7` (wave base). The implementer ENUMERATES the chain at pre-flight with host `git log --oneline -20` (§3 step 1) and pastes the lines into the 4.2 report — the plan deliberately pre-records NO further SHAs beyond `b25677d` (caller-provided, the only pinned one).
- **NO prior-green context exists (the reason this run is a discovery run): no dev-checks harness execution has EVER been run on this branch.** The FIRST §3 step-4 run is MANDATORY and is the branch's first-ever compile+test evidence.
- Version **0.5.1** (verified: `Cargo.toml` line 3 `version = "0.5.1"`; bumped at Step 3, commit `8b9894a`). NOTHING touches `Cargo.toml`/`Cargo.lock` in this task (§2; the wave's windows-feature additions `Win32_Globalization` + `Win32_System_SystemServices` and the version bump landed WITH the committed lock at `8b9894a`/`788b312`).
- Test census verified at plan time by host-side per-file `#[test]` count — **26 files, grand total 164** (the 150 baseline of the 0.5.0 census + Task-2's +4 in `zoom_alias_tests` + Task-3's +4 in the new `esc_close_expiry_tests` + Task-4's +6 in `name_collation` (+2) and the two new collation test files (+2 each)):

| File | `#[test]` count |
| --- | --- |
| `src/app/tests.rs` | 9 |
| `src/app/button_hint_tests.rs` | 2 |
| `src/app/esc_close_confirm_tests.rs` | 11 |
| `src/app/esc_close_expiry_tests.rs` (**Task-3 new — the ctx-level expiry tests**) | 4 |
| `src/app/fit_one_shot_tests.rs` | 12 |
| `src/app/floating_controls_tests.rs` | 1 |
| `src/app/folder_navigation_continuation_tests.rs` | 4 |
| `src/app/folder_navigation_tests.rs` | 5 |
| `src/app/fullscreen_tests.rs` | 9 |
| `src/app/last_viewed_tests.rs` | 5 |
| `src/app/pan_tests.rs` | 12 |
| `src/app/pill_style_tests.rs` | 3 |
| `src/app/zoom_alias_tests.rs` (**Task-2: 14 → 18, the round-2 accept-set tests**) | 18 |
| `src/app/zoom_tests.rs` | 11 |
| `src/app_display.rs` | 2 |
| `src/folder_listing/collation_tests.rs` (**Task-4 new — the horizontal seam tests**) | 2 |
| `src/folder_listing/tests.rs` | 5 |
| `src/image_loading.rs` | 4 |
| `src/name_collation.rs` (**Task-4: 2 → 4 — the seam + ASCII-fallback tests**) | 4 |
| `src/pan/pan_tests.rs` | 11 |
| `src/scaling.rs` | 5 |
| `src/scaling/fit_factor_tests.rs` | 7 |
| `src/sibling_folder_listing/collation_tests.rs` (**Task-4 new — the vertical seam tests**) | 2 |
| `src/sibling_folder_listing/neighbor_tests.rs` | 2 |
| `src/sibling_folder_listing/tests.rs` | 6 |
| `src/zoom.rs` | 8 |
| **Total** | **164** |

- Zero-count rows for the test-free `src/` files (`platform/` facade files, `app/` non-test modules, etc.) are normal in the per-file grep and NOT census failures.
- No `#[ignore]` / `#[should_panic]` markers anywhere in `src/` (verified by grep at plan time) — nothing skipped, nothing deferred.
- LAST KNOWN EXE size (delta-only reference, never an expectation): 0.5.0 = **19,606,124 bytes**; `dist/windows-x64/img-scroller.exe` on disk currently holds that 0.5.0 artifact and will be ATOMICALLY overwritten (`.part` copy → rename) by the 0.5.1 build. The 0.5.1 exe size WILL differ; any non-empty size is acceptable — the verbatim as-built value is the deliverable. NO target size number is a valid pre-requirement.
- `OBSERVED_DLL_IMPORT_LIST` constant verified present at [`scripts/build-windows.sh`](../../scripts/build-windows.sh) line 15 (identical in [`docs/build-windows.md`](../../docs/build-windows.md) → "Static-linking / import audit", observed list line 60), 22 DLLs — the audit comparator (§6).
- Harness semantics (docs/build.md + `scripts/dev-checks.sh`): fixed order `fmt-check` → `check` → `test` → `clippy`; EVERY check runs even if earlier ones fail; per-check line `PASS|FAIL <name> exit=<code> (<n> s)`; final literal `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1); UTC-timestamped log appended under the gitignored `logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log`, the exact path printed at run start is authoritative; the script never modifies sources and never applies fmt fixes; `FORCE_FAIL` exists but is NOT used this task.
- build-windows semantics (docs/build-windows.md + script): runs `cargo build --release --locked --target x86_64-pc-windows-gnu` with cwd `/img-scroller` and the compose service's `CARGO_TARGET_DIR`; success prints `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, then the OBSERVED-DLL evidence line; install is ATOMIC (`.part` copy → rename over the final name; a failed run leaves the previous artifact byte-for-byte intact and never leaves a surviving `.part`); every failure line starts with the literal prefix `build-windows: error:` and exits 1.
- `.gitignore` contains (verified read) `*.log`, `logs/`, `dist/`, `target/` (plus generic/temp/IDE/env patterns) — artifacts and check logs are NEVER stageable.
- **Wave-5.1 pins verified at plan time (host reads at HEAD `b25677d`; they feed §3's sweep expectations — ALL line numbers below are as-verified NOW):**
  - **Task-2 round-2 alias accept-set** (`src/app/zoom_alias.rs`): `ZOOM_IN_ALIAS_CHARACTERS: [char; 3] = ['+', '=', '*'];` → exactly `:25` (the `'*'` entry IS the round-2 addition); `ZOOM_IN_ALIAS_KEYS: [egui::Key; 2] = [egui::Key::Equals, egui::Key::Plus];` → `:23`; `ZOOM_OUT_ALIAS_KEYS: [egui::Key; 1] = [egui::Key::Minus];` → `:24`; `ZOOM_OUT_ALIAS_CHARACTERS: [char; 3] = ['-', TRUE_MINUS_SIGN, '_'];` → `:26` (the UNPROVEN text-member entries from round 1 are KEPT — deferred-decision outcome).
  - **Shift gate untouched (round-1 semantics preserved)**: `fn shift_alias_is_pressed` → exactly `src/app/zoom_alias.rs:28` (the per-member wrappers `alias_member_was_pressed` :54 and `alias_character_was_pressed` :60 call it at :57/:63 — every member individually shift-gated); the app seam's chord delegates are unchanged in the intake: `zoom::zoom_in_chord_was_pressed` / `zoom::zoom_out_chord_was_pressed` → exactly `src/app/keyboard_intake.rs:33`/:34.
  - **Task-2 residual scope**: the wave's only behavioral change in the alias layer is the `zoom_alias.rs` accept-set (`['+', '=', '*']` text members + the module doc); `src/app/zoom.rs` changed DOC-ONLY (verified diff: 3 doc-comment lines) — the `zoom.rs`-file sweep is the no-touch diff (§7h) which EXPECTS output for `src/app/zoom.rs` (doc-only) and is NOT a no-touch file.
  - **Round-2 test additions** (`src/app/zoom_alias_tests.rs`, 14 → 18): `shift_star_text_zooms_in` :157, `plain_star_text_is_inert` :165, `spanish_shift_plus_delivery_zooms_in` :173, `spanish_plus_key_member_alone_is_inert` :180 (the 4 new tests; the 14 pre-round-2 tests unchanged).
  - **Task-3 expiry wake** (`src/app/esc_close_confirm.rs`): `POPUP_EXPIRY_WAKE_MARGIN_SECONDS: f64 = 0.05;` → exactly `:37`; `fn request_expiry_wake` → `:141` (the `request_repaint_after(Duration::from_secs_f64(remaining_seconds + POPUP_EXPIRY_WAKE_MARGIN_SECONDS))` wake at :149–:151); the per-frame `update_esc_close_popup` (:114) calls `self.request_expiry_wake(screen_context, now);` → exactly `:116`. Unchanged round-1 shape: `POPUP_VISIBLE_SECONDS: f64 = 2.0` → `:32`; the pure `update_esc_close_popup_at` :121; `current_time` = `input.time` :155–:157 (the SAME clock arms and expires); the arm/disarm helpers :98/:104/:109.
  - **Intake expiry call untouched (no-touch)**: `self.update_esc_close_popup(screen_context);` → exactly `src/app/keyboard_intake.rs:53` (between the reads and the dispatch — the round-1 wiring shape preserved).
  - **Task-3 new ctx-level tests** (`src/app/esc_close_expiry_tests.rs`, 4): `expiry_clears_through_real_intake_after_deadline` :88, `expiry_before_deadline_keeps_popup_visible` :106, `armed_popup_schedules_expiry_wake` :118, `wake_requests_stop_after_expiry` :138 — driven through the PRODUCTION intake (`read_and_dispatch_keystrokes`) with ADVANCING `RawInput.time` on ONE long-lived egui `Context` (the `TimedIntakeHarness`), the round-1 gap closed.
  - **Task-4 collation seam** (`src/name_collation.rs`): `pub(crate) struct EntryNameOrder` → `:24`; `pub(crate) fn system()` → `:29` (resolves `crate::platform::compare_entry_names_system` at :31); `pub(crate) fn with_comparator` → `:35`; `pub(crate) fn compare` → `:38`; `pub(crate) type EntryNameCompare` → `:17`; the pure fallback `compare_entry_names` :9–:15 UNCHANGED (lowercase fold first, exact-character tie-break — the container/stub/system-failure order).
  - **Platform facade re-exports incl. windows_collation** (`src/platform/mod.rs`): `mod windows_collation` (cfg-gated) → `:8–:9`; `pub(crate) use windows::compare_entry_names_system` → `:20–:21`; `pub(crate) use stub::compare_entry_names_system` → `:23–:24`; the stub's `compare_entry_names_system` DELEGATES to the pure fallback (`src/platform/stub.rs:19–:21`) — the container build's system order IS the fallback order.
  - **Windows-collation implementation (NOT container-compilable — the §-fact above)**: `src/platform/windows_collation.rs` — `system_collation_order` :15, the `CompareStringEx` call with `NORM_IGNORECASE` :20–:22, `GetUserDefaultLocaleName` :35, failure → `None` :42–:49; `src/platform/windows.rs` `compare_entry_names_system` :159–:162 falls back to the pure collation on `None`. Compiled ONLY by the release cross-build (§3 step 8) — the authoritative compile evidence.
  - **Task-4 consumers — BOTH axes through the seam**: `src/folder_listing.rs` — `from_opened_image` :32 resolves `EntryNameOrder::system()` at :33 and delegates to `from_opened_image_with_order` :38; the listing sort calls `entry_order.compare` at :56–:58; `first_listed_image` :69 (the vertical first-image priority probe reads the SAME sorted list). `src/sibling_folder_listing.rs` — `from_opened_image` :44 resolves `EntryNameOrder::system()` at :45 and delegates to `from_opened_image_with_order` :47; the folder sort calls `entry_order.compare` at :108–:112; the anchor position reads the SAME stored order through `listed_folder_name_precedes` :131–:137 (which uses `self.entry_order.compare` at :136).
  - **The Cargo.toml windows-feature additions** (Task-4 dependency change): `Win32_Globalization` + `Win32_System_SystemServices` added under `[target.'cfg(windows)'.dependencies]` — verified diff; these additions must NOT change the DLL import list (§3 step 10; `CompareStringEx`/`GetUserDefaultLocaleName` are kernel32 exports — the AUDIT is the deciding evidence, never an assumption; §6).
  - **No-touch list (files that MUST show EMPTY wave diffs — §7h)**: `src/app/keyboard_intake.rs` (the ESC/pan wiring + the expiry call), `src/app/fullscreen.rs` (the disarm contract), `src/app/pan.rs` + `src/pan.rs` + `src/app/pan_tests.rs` + `src/pan/pan_tests.rs` (the passed Wave-5 pan controller), `src/main.rs`, `src/zoom.rs` (pure), `src/app/floating_controls.rs`, `src/app/pill_style.rs`, `src/app/button_hint.rs`, `src/app/zoom_button.rs`, `src/app/fit_button.rs`, `src/app/fullscreen_button.rs`, `src/app/folder_navigation.rs`, `src/app/esc_close_confirm_tests.rs`, `src/last_viewed_images.rs`, `src/app_display.rs`, `src/app/central_surface.rs`, `src/scaling.rs`.
  - **message_text write-sites** — EXACTLY 3 (`src/app.rs:93` = `None;`, `src/app.rs:99` = `Some(image_loading::UNSUPPORTED_FILE_MESSAGE);`, `src/app/folder_navigation.rs:108` = `None;` — the app.rs sites did NOT move; the wave's app.rs diff is the +2 test-module declaration lines only; NONE of the 3 sites contains "Zoom"/"Fit").
  - **Fullscreen disarm untouched**: `self.disarm_esc_close_popup();` call → EXACTLY 1 code site `src/app/fullscreen.rs:31` (definition `src/app/esc_close_confirm.rs:109`); every fullscreen toggle still disarms.
  - **Launch/console/floor regression pins**: `windows_subsystem` → exactly `src/main.rs:4`; `with_maximized(true)` → exactly ONE code line `src/main.rs:27`; symmetric floor pin `ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM;` → exactly `src/zoom.rs:20`.
- Harness-execution deferral: NONE of §3's VM steps have been run yet; this plan is authored BEFORE any 4.2 execution (the caller forbade running the harness in this planning step).

## §1 Unit-coverage mapping (TODO Task 5 sub-item 1 → the three fix tasks' landed tests)

The TODO sub-item 1 ("the round-2 alias accept-path covers the real delivery resolution researched in Task 2; the ctx-level expiry test of Task 3; the collation tests of Task 4 — run via the dev-checks harness in the container per `docs/build.md`") maps onto the landed 164 as:

| TODO wave task | Satisfied by | File(s) |
| --- | --- | --- |
| Task 2 — the round-2 alias accept-path (the real delivery: Shift+'+' types `Event::Text("*")` on Spanish-class layouts; the key member falls back to the physical `CloseBracket`) | `shift_star_text_zooms_in`, `spanish_shift_plus_delivery_zooms_in` (the accepted deliveries) + `plain_star_text_is_inert`, `spanish_plus_key_member_alone_is_inert` (the strictness regressions) + the pinned sets `alias_key_sets_are_pinned`/`alias_characters_are_pinned` | `src/app/zoom_alias_tests.rs` |
| Task 2 — round-1 strictness preserved (Shift+'−' out; plain `+`/`=`/`-` inert) | the 14 pre-round-2 tests unchanged (`shift_minus_key_event_zooms_out`, `plain_equals_key_is_inert`, `plain_plus_key_is_inert`, `plain_minus_key_is_inert`, `shift_only_frame_is_inert`, `typed_*_with_shift_*`, `typed_alias_chars_without_shift_are_inert`, `shift_alias_is_pressed_matrix`, …) | `src/app/zoom_alias_tests.rs` + `src/app/zoom_tests.rs` |
| Task 3 — the ctx-level expiry test through the REAL intake wiring | `expiry_clears_through_real_intake_after_deadline` + `expiry_before_deadline_keeps_popup_visible` (advancing `RawInput.time` on ONE long-lived Context through `read_and_dispatch_keystrokes`) | `src/app/esc_close_expiry_tests.rs` |
| Task 3 — the wake schedule + the no-spam shape | `armed_popup_schedules_expiry_wake`, `wake_requests_stop_after_expiry` | `src/app/esc_close_expiry_tests.rs` |
| Task 3 — the round-1 pure cores + close contract regressions | the 11 `esc_close_confirm_tests.rs` tests unchanged | `src/app/esc_close_confirm_tests.rs` |
| Task 4 — the collation seam (both listings sort through `EntryNameOrder`) | `listed_images_sort_through_the_injected_order_seam`, `sibling_folders_sort_through_the_injected_order_seam`, `anchor_position_follows_the_injected_order`, `injected_comparator_decides_the_entry_order` | `src/folder_listing/collation_tests.rs` + `src/sibling_folder_listing/collation_tests.rs` + `src/name_collation.rs` |
| Task 4 — the fallback order (ASCII behavior + Japanese-mixed names through the pure fallback; the system() resolution matches it for ASCII) | `listed_images_fallback_order_places_ascii_before_japanese`, `system_order_matches_the_pure_fallback_for_ascii_names` + the pre-existing lowercase/tie-break tests unchanged | `src/folder_listing/collation_tests.rs` + `src/name_collation.rs` + `src/folder_listing/tests.rs` + `src/sibling_folder_listing/tests.rs` |
| Task 4 — the first-image/last-viewed priority (reads the same sorted listings — no separate change) | `vertical_navigation_opens_the_first_image_of_a_new_folder`, `vertical_navigation_reopens_the_visited_folders_last_viewed_image`, `vertical_navigation_resolves_the_adjacent_sibling_folders_in_sort_order` + the last_viewed memory tests | `src/app/folder_navigation_tests.rs` + `src/app/folder_navigation_continuation_tests.rs` + `src/app/last_viewed_tests.rs` |
| Standing regressions (pan controller, fullscreen F11/ESC-exit/ESC-close, zoom/fit, pill styling, open flow, unsupported message, shell) | the remaining tests unchanged from the 0.5.0 census | all files above |

### §1.1 GAP VERDICT

**No gaps.** Every topic nameable from the three fix tasks (the round-2 accept path + strictness, the ctx-level expiry + wake shape, the seam + fallback order on both axes) is pinned by landed tests. **NO test additions are sanctioned in this task; the expected census stays EXACTLY 164.** Any "why not one more test" improvisation in 4.2 → STOP (§14).

### §1.2 Census-failure rule

If the harness `cargo test` summary does not read exactly `test result: ok. 164 passed; 0 failed; ...` (a trailing doc-test `test result: ok. 0 passed` line is expected and normal — NOT a second count): **STOP** and return the full test section to the caller. A census mismatch is a defect, never an ad-hoc-patch opportunity.

## §2 Lockfile status — NO reconcile needed

Version 0.5.1 and the windows-feature additions landed WITH the committed `Cargo.lock` (Step 3 `8b9894a` + Task 4 `788b312`); the fixes introduced NO further dependency changes. Therefore NO reconcile command, NO lock commit this task. Conditional STOP: if ANY `--locked` invocation (dev-checks steps, or the release build) fails with a "the lock file needs to be updated but --locked was passed"-class error — **STOP immediately** and return the error verbatim to the caller. A lock-drift failure here means a committed-state inconsistency the plan's evidence did not show — caller decision, NOT a junior self-fix.

## §3 Command sequence for 4.2 (exact; per docs/build.md and docs/build-windows.md)

Command channel: the `alpine-vm` MCP tools — call `alpine-vm_vm_status` BEFORE EVERY `alpine-vm_vm_run_command` call; ONE MCP call per row; each row's exact `command` string is passed verbatim; each command gets its own timeoutMs from the table; the `exitCode` field is authoritative; captured MCP stderr may be empty — an absent stderr is NOT a failure signal (documented caveat). Host-side git/grep commands run via the Bash tool ONE command per invocation (never chained, never `&&`; if the host shell rejects `grep`, use the built-in Grep tool per the tool-selection-priority rule).

| # | Step | Command (exact) | timeoutMs | Expected |
| --- | --- | --- | --- | --- |
| 0 | VM gate | `vm_status` (MCP tool) | — | running + SSH reachable; else STOP + user question (binding wave note) |
| P1 | Pre-flight LF guard ×2 | `sh -c "sed 's/\r$//' /img-scroller/scripts/dev-checks.sh \| cmp - /img-scroller/scripts/dev-checks.sh"`, then the IDENTICAL command for `/img-scroller/scripts/build-windows.sh` | 60000 | exit 0, no output, BOTH times. exit 1 (CRLF recurrence) → STOP + report (class outside Task-5 scope; caller owns the fix shape) |
| 1 | Pre-flight git evidence (host, Bash tool) | `git branch --show-current`, then SEPARATE calls: `git log --oneline -20`, then `git status --porcelain` | — | branch = `fix/wave-5-1-operator-feedback`; HEAD = `b25677d`; the wave chain visible below the tip (per §0); porcelain EMPTY or showing ONLY this task's permitted residuals (this plan file before C1, and/or a caller-owned modified `.agent/project-info/context.md`). Any OTHER entry → STOP + ask the caller. Paste the `git log` lines into the report (the wave-chain evidence table) |
| 2 | Pre-flight census STRUCTURE check (BEFORE running the harness — caller requirement; report any divergence BEFORE running) | host: `git grep -c "#\[test\]" -- src` (fallback: the built-in Grep tool) | — | per-file counts COMPLETELY EQUAL to the §0 26-file table; hand-summed grand total **164**. ANY divergence → STOP + report BEFORE any harness run (the caller-projection says 164 expected; the harness figure remains the binding one, but a pre-flight mismatch is reported FIRST, not silently proceeded past) |
| 3 | Pre-flight version check | host: `git grep -n "^version" -- Cargo.toml` | — | exactly 1 line: `3:version = "0.5.1"` |
| C1 | Plan-file commit (wave task-level precedent `e28f33d`/`6621c4b`) | host: `git add .kilo/plans/20261112-wave5-1-task5-verification-plan.md`; then `git commit -m "docs: add wave5.1 task5 verification plan"` | — | commit succeeds; `git status --porcelain` afterwards shows NO repo-owned file except (possibly) a caller-owned modified `.agent/project-info/context.md` |
| 4 | **FIRST dev-checks harness run at HEAD — the DISCOVERY run (sub-item 1's authoritative evidence; the branch's first-ever compile+test execution — NON-Windows/stub path ONLY, see §0)** | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` | 900000 | four `PASS` lines in fixed order + final literal `ALL CHECKS PASSED`, **exit code 0**; harness test block `test result: ok. 164 passed; 0 failed; ...` (+ doc-test `0 passed` line, normal); RECORD the printed log path. On any FAIL → §7 branches (fmt round FIRST; then compile/clippy residual fixes with the one-more-attempt cap). NOTE: this run compiles the STUB path only — it is NOT evidence for the windows-target code (§0, §4) |
| 5 | Census corroboration (AFTER a green or converged run) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rc --include='*.rs' '#\[test\]' /img-scroller/src \| sort"` | 60000 | per-file counts COMPLETELY EQUAL to the §0 26-file table (zero-count rows for the test-free files are normal); implementer sums the non-zero numbers by hand → grand total **164**; corroborates the harness `164 passed` line (the harness number is the binding figure) |
| 6 | Skip/ignore-marker sweep | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn --include='*.rs' -e '#\[ignore\]' -e '#\[should_panic\]' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `0`, exit 0 — no skipped/ignored tests |
| 7a | Sweep: round-2 alias accept-set pins | two greps: `grep -n "ZOOM_IN_ALIAS_CHARACTERS" /img-scroller/src/app/zoom_alias.rs` (EXACTLY 1 line: `25`, the line reads `['+', '=', '*']`); `grep -n "ZOOM_IN_ALIAS_KEYS\|ZOOM_OUT_ALIAS_KEYS\|ZOOM_OUT_ALIAS_CHARACTERS" /img-scroller/src/app/zoom_alias.rs` (EXACTLY 3 lines: `23`, `24`, `26`) | 60000 each | every grep exact; assert the `:25` line contains the three-member `['+', '=', '*']` set by eye (the round-2 addition); the round-1 text-member entries (`'+'`/`'='` in; `'-'`/U+2212/'_' out) are KEPT |
| 7b | Sweep: shift gate untouched + intake delegates | three greps: `grep -n "fn shift_alias_is_pressed" /img-scroller/src/app/zoom_alias.rs` (EXACTLY 1 line: `28`); `grep -n "shift_alias_is_pressed(" /img-scroller/src/app/zoom_alias.rs` (EXACTLY 3 lines: `28`, `57`, `63` — the per-member wrappers call it); `grep -n "zoom_in_chord_was_pressed\|zoom_out_chord_was_pressed" /img-scroller/src/app/keyboard_intake.rs` (EXACTLY 2 lines: `33`, `34`) | 60000 each | every grep exact |
| 7c | Sweep: expiry wake constant + wiring | two greps: `grep -n "POPUP_EXPIRY_WAKE_MARGIN_SECONDS: f64 = 0.05\|fn request_expiry_wake\|self.request_expiry_wake" /img-scroller/src/app/esc_close_confirm.rs` (EXACTLY 3 lines: `37`, `141`, `116`); `grep -n "POPUP_VISIBLE_SECONDS: f64 = 2.0" /img-scroller/src/app/esc_close_confirm.rs` (EXACTLY 1 line: `32`) | 60000 each | every grep exact; assert by eye that the `:116` call sits inside `update_esc_close_popup` and the `:149–:151` wake request composes `remaining_seconds + POPUP_EXPIRY_WAKE_MARGIN_SECONDS` |
| 7d | Sweep: intake expiry call untouched | `grep -n "update_esc_close_popup" /img-scroller/src/app/keyboard_intake.rs` | 60000 | EXACTLY 1 line: `53` (between the reads and the dispatch) |
| 7e | Sweep: EntryNameOrder seam | `grep -n "pub(crate) struct EntryNameOrder\|pub(crate) fn system()\|pub(crate) fn with_comparator\|pub(crate) fn compare" /img-scroller/src/name_collation.rs` | 60000 | EXACTLY 4 lines: `24`, `29`, `35`, `38` (assert the `:31` body resolves `crate::platform::compare_entry_names_system` by eye) |
| 7f | Sweep: platform facade re-exports incl. windows_collation | `grep -n "mod windows_collation\|compare_entry_names_system" /img-scroller/src/platform/mod.rs` | 60000 | EXACTLY 3 lines: `9`, `21`, `24` (cfg-gated windows side :20–:21; stub side :23–:24; assert the stub delegation to the pure fallback by eye in `src/platform/stub.rs:19–:21`) |
| 7g | Sweep: both from_opened_image_with_order consumers | four greps: `grep -n "fn from_opened_image" /img-scroller/src/folder_listing.rs` (EXACTLY 2 lines: `32`, `38`); `grep -n "EntryNameOrder::system()" /img-scroller/src/folder_listing.rs` (EXACTLY 1 line: `33`); `grep -n "fn from_opened_image" /img-scroller/src/sibling_folder_listing.rs` (EXACTLY 2 lines: `44`, `47`); `grep -n "EntryNameOrder::system()" /img-scroller/src/sibling_folder_listing.rs` (EXACTLY 1 line: `45`) | 60000 each | every grep exact; assert the sort sites use `entry_order.compare` (`folder_listing.rs:56–:58`; `sibling_folder_listing.rs:108–:112`) and the anchor position reads the SAME stored order (`sibling_folder_listing.rs:136`) by eye |
| 7h | Sweep: the no-touch list (host git diffs) | host, ONE call per file: `git diff b1435e7..HEAD --stat -- <FILE>` for EACH of: `src/app/keyboard_intake.rs`, `src/app/fullscreen.rs`, `src/app/pan.rs`, `src/pan.rs`, `src/app/pan_tests.rs`, `src/pan/pan_tests.rs`, `src/main.rs`, `src/zoom.rs`, `src/app/floating_controls.rs`, `src/app/pill_style.rs`, `src/app/button_hint.rs`, `src/app/zoom_button.rs`, `src/app/fit_button.rs`, `src/app/fullscreen_button.rs`, `src/app/folder_navigation.rs`, `src/app/esc_close_confirm_tests.rs`, `src/last_viewed_images.rs`, `src/app_display.rs`, `src/app/central_surface.rs`, `src/scaling.rs` | — | EVERY diff EMPTY (no output at all). ANY output → STOP + report verbatim |
| 7i | Sweep: message ground truth unchanged | `grep -rn "message_text = " /img-scroller/src` | 60000 | EXACTLY 3 lines: `src/app.rs:93` (`= None;`), `src/app.rs:99` (`= Some(image_loading::UNSUPPORTED_FILE_MESSAGE);`), `src/app/folder_navigation.rs:108` (`= None;`). Assert by eye: none of the 3 contains "Zoom" or "Fit" |
| 7j | Sweep: launch/console/floor regression pins | three greps: `grep -rn "windows_subsystem" /img-scroller/src` (exactly 1 line `main.rs:4`); `grep -rn "with_maximized" /img-scroller/src` (exactly 1 line `main.rs:27`); `grep -n "ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM" /img-scroller/src/zoom.rs` (exactly 1 line `zoom.rs:20`) | 60000 each | every grep exact |
| 7k | Sweep: fullscreen disarm unchanged | `grep -rn "self.disarm_esc_close_popup()" /img-scroller/src` | 60000 | EXACTLY 1 line: `src/app/fullscreen.rs:31` (the definition at `esc_close_confirm.rs:109` is a separate pattern-free fact) |
| 8 | Release cross-build (0.5.1 exe) — **THE WINDOWS-CODE COMPILE EVIDENCE** (this build compiles the cfg(windows) target: `windows.rs` + `windows_collation.rs` — the authoritative compile check the dev-checks harness cannot provide) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` | 900000 | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; the OBSERVED-DLL evidence line; **exit code 0**. Exit 0 on this step IS the windows-collation code's compile-correctness evidence; a compile error here names the windows-target code — report verbatim (§7 build branch) |
| 9 | Exe inspection (record the byte size) | `sh -c "ls -l /img-scroller/dist/windows-x64/img-scroller.exe"` | 60000 | non-empty size printed; RECORD the exact as-built byte count verbatim (any non-empty size passes; no pre-requirement) |
| 9b | No dangling `.part` | `sh -c "ls /img-scroller/dist/windows-x64"` | 60000 | the listing shows EXACTLY one entry: `img-scroller.exe` — no `img-scroller.exe.part` (atomic install completed; the part file never survives success) |
| 10 | DLL import audit (per docs/build-windows.md) — **the DECIDING evidence for the feature additions** | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe \| grep -i 'DLL Name' \| tr 'A-Z' 'a-z' \| sed 's/.*dll name: //' \| sort -u"` | 180000 | exactly 22 lowercase DLL lines, SET-EQUAL to the recorded constant (§6). If the set CHANGES (extra/missing/renamed): **STOP + return the EXACT delta to the caller** (a real audit delta is caller-reportable — never recorded as a new baseline, never treated as silent pass/fail; §6) |
| 10b | Audit fallback (ONLY if step 10 yields no output — report-format drift) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` | 180000 | raw dump; compare manually against §6; large output, still ONE call |
| 11 | Gitignore compliance sweep (host) | re-read `.gitignore`, then `git status --short` | — | NO `dist/`, `logs/`, `target/`, `*.log` entries anywhere in the output; nothing staged; the only allowed residual is a caller-owned modified `.agent/project-info/context.md` |

## §4 Expected outcomes table (authoritative)

| Evidence | Expected |
| --- | --- |
| Pre-flight (steps 1–3) | branch `fix/wave-5-1-operator-feedback`; HEAD `b25677d`; wave chain below the tip; census structure = §0 table, hand-sum 164, reported BEFORE the harness run; `Cargo.toml:3` = `0.5.1` |
| Harness log header | `dev-checks starting; log file: logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` — RECORD the exact printed path |
| Harness check lines (fixed order) | `PASS fmt-check exit=0 (<n> s)`; `PASS check exit=0 (<n> s)`; `PASS test exit=0 (<n> s)`; `PASS clippy exit=0 (<n> s)` (clippy runs `-D warnings`, so exit 0 means zero warnings) — after §7's sanctioned convergence (discovery run may fail first; the FINAL run must be all-PASS) |
| Harness test block | `test result: ok. 164 passed; 0 failed; ...` + doc-test `test result: ok. 0 passed` (normal, not a second count) |
| Harness final | literal `ALL CHECKS PASSED`; **exit code 0 = the authoritative success evidence** (sub-item 1's gate; the branch's FIRST-ever green evidence) — **SCOPE NOTE (binding): the dev-checks build compiles the NON-Windows (stub) path ONLY; the cfg(windows) code (`windows.rs`, `windows_collation.rs`) is NOT compiled by it; its compile evidence is step 8's exit 0 + the OPERATOR-RUN** |
| Census corroboration | per-file grep counts == §0 table; hand-summed total 164; zero `#[ignore]`/`#[should_panic]` |
| §3 sweeps & pins | 7a `:25` with the `['+', '=', '*']` set + the 3 lines (`:23`/:24/:26); 7b `:28` + 3 shift-gate lines (28/57/63) + the 2 intake delegate lines (`:33`/:34); 7c the 3 wake lines (37/141/116) + `:32`; 7d `:53` single intake expiry call; 7e the 4 seam lines (24/29/35/38); 7f the 3 facade lines (9/21/24); 7g 2+1+2+1 consumer lines (32/38/33, 44/47/45); 7h EVERY no-touch diff EMPTY; 7i exactly 3 message sites, none Zoom/Fit; 7j all three regression pins exact (`main.rs:4`, `main.rs:27` ONLY, `zoom.rs:20`); 7k exactly the 1 disarm call (`fullscreen.rs:31`) |
| Release build | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; OBSERVED-DLL evidence line; **exit code 0 = the WINDOWS-CODE compile evidence (the cfg(windows) target compiles: `windows_collation.rs` + `windows.rs`)** |
| Exe | NON-EMPTY gitignored `dist/windows-x64/img-scroller.exe` (0.5.1 wave build); exact as-built size RECORDED (delta vs the 19,606,124 B prior 0.5.0 magnitude is evidence only — any non-empty size passes); NO dangling `.part` (9b: the dist dir lists exactly `img-scroller.exe`) |
| DLL audit | 22 lowercase DLL names, SET-EQUAL to the recorded 2026-10-31 constant (§6) — the audit is the DECIDING evidence that the windows-feature additions (`CompareStringEx`/`GetUserDefaultLocaleName`, kernel32 exports) did not change the import set; on ANY delta: STOP + report the exact delta (never an assumption, never a silent decision) |
| Git sweep | nothing ignored staged; nothing staged at all after C1 except the 4.6 §10 mark commit later |

## §5 Alpine-VM MCP usage notes (binding, per docs/build.md)

- `vm_status` BEFORE every `vm_run_command` call; only proceed when the VM is running and SSH-reachable; otherwise STOP and return a user question (docs/build.md precondition; wave execution note — harness unavailability = `vm_status` fail OR the `/img-scroller` share missing).
- ONE command per MCP call; pipelines live INSIDE one quoted `sh -c "..."` string exactly as printed in §3.
- The `exitCode` field is the authoritative pass/fail evidence; `null` exitCode + `timedOut: true` → report the timeout with the partial captured output; do NOT re-run silently.
- Absent cargo stderr in captured MCP output is NOT a failure signal (documented caveat).
- Generous timeouts (§3 table) beat aggressive ones.
- EBUSY-on-share-write hazard (vboxsf): retry the IDENTICAL command ONCE; if it fails again → STOP + report with evidence.

## §6 DLL import audit — recorded constant and comparison evidence statement

Recorded `OBSERVED_DLL_IMPORT_LIST` (source of truth: `scripts/build-windows.sh` line 15, identical in `docs/build-windows.md` → "Static-linking / import audit"; audited 2026-10-31 with `x86_64-w64-mingw32-objdump`; docs-split rule: NOT duplicated into tech.md/architecture.md):

```
advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll,
api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll,
gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll,
opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll,
ws2_32.dll
```

Evidence statement to record in the 4.2 report (verbatim shape): "Wave-5.1 audit: `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` inside the container; output deduplicated + lowercase-folded via the §3 step-10 pipeline; OBSERVED = 22 DLLs; SET-EQUAL to the recorded 2026-10-31 constant — DLL import list UNCHANGED by the Wave-5.1 windows-feature additions (`Win32_Globalization`, `Win32_System_SystemServices` → `CompareStringEx`/`GetUserDefaultLocaleName`, kernel32 exports)."

On ANY diff (extra, missing, renamed — dependency drift): **STOP + report** the full observed list and the EXACT delta (added/removed names). The audit result is the DECIDING evidence — it is never pre-assumed unchanged and never decided by reading the feature list alone. NO edits to `scripts/build-windows.sh` or `docs/build-windows.md` — refreshing the OBSERVED baseline is a Planner decision, never an implementer improvisation (Wave-1/2/3/4/4.1/4.2/5 precedent).

## §7 Failure/intervention policy for 4.2 — DISCOVERY variant (decision branches; caps PINNED)

This branch has NEVER compiled: the discovery run is EXPECTED to possibly surface fmt/clippy residuals (and, less likely, a compile residual in the NON-Windows code — nothing proves the Tasks 2–4 code compiles until this run; the WINDOWS-target code is NOT covered by the dev-checks and belongs to step 8). The sanctioned convergence sequence is:

- **fmt-check FAIL → fmt round FIRST (sanctioned self-service)**: ONE `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` (timeoutMs 120000) — the documented invocation of the formatter in the container; host-side `git diff` MUST show formatting-only reflow (no semantic content moves); if changed files exist → stage EXACTLY the `src/` paths listed by `git status --short` (never `.agent/`, never `docs/`) and commit EXACTLY `style: apply cargo fmt` (separate commit; Wave-2 precedent). Max 1 fmt round; a SECOND consecutive fmt-check FAIL → STOP + report (a deterministic formatter cannot legitimately fail twice).
- **check FAIL (compile residual) → sanctioned MINIMAL scoped correction**: fix the compile error(s) with the SMALLEST scoped corrections possible (the fewest lines, in exactly the files the compiler names; NO restructuring, NO renames beyond the error, NO drive-by edits); stage ONLY those files; commit EXACTLY `fix: wave5.1 verification compile fix`; then re-run the §3 step-4 harness. If the harness converges → continue. If check fails AGAIN → ONE more attempt is allowed (one more minimal fix + commit + re-run). If it CANNOT converge in that one more attempt → **STOP and return the verbatim full captured output + log path** (caller decision).
- **clippy FAIL → sanctioned MINIMAL scoped correction**: same shape as the compile branch — fix the named lints with the smallest possible scoped corrections (multi-site lints are fixed together ONLY when each site's fix is trivial and mechanical, e.g. the same lint in sibling lines; any fix requiring a design choice → STOP); commit EXACTLY `fix: wave5.1 verification clippy fix`; re-run the harness once per round. If it CANNOT converge in one more attempt after the first clippy fix round → **STOP and return the verbatim full captured output + log path**.
- **test FAIL → STOP immediately, NO self-fix.** Any failing named test = a structural/census matter; the §1.2 census rule protects against ad-hoc test invention; report with the failure detail lines and the printed log path. (A red test on this branch is a caller decision — the plan's evidence showed the landed census of 164; a failure means committed-state divergence from that ground truth.)
- **Windows-target compile failure at step 8 (the cross-build compiles the cfg(windows) code the dev-checks never saw)**: a `build-windows: error:` line naming a windows-target file (`windows.rs`, `windows_collation.rs`, or a windows-feature mismatch) → STOP + report the typed error verbatim; the docs/build-windows.md failure table is the diagnostic map. A WINDOWS-TARGET code fix is NOT a sanctioned §7 residual-fix class (it was never container-compile-checkable) — caller decision on the fix shape, even if the error looks trivial. NEVER retry beyond the single EBUSY-once rule of §5.
- Global cap: at most 2 post-fix harness re-runs in total across ALL sanctioned fix rounds (the discovery run + 2 re-runs = 3 harness invocations max); anything failing after the cap → STOP + report with evidence (four result lines + log path).
- Any §3 sweep (2–3, 5–6, 7a–7k) mismatching its Expected → STOP + report the actual output verbatim. A drift means committed-state deviation from the plan's verified ground truth — caller decision, never a self-fix. (EXCEPTION: the census/structure check of step 2 is reported BEFORE the harness run per the caller requirement — a divergence there still means STOP.)
- Any `build-windows: error:` line of ANY type → STOP + report the typed error verbatim (the docs/build-windows.md failure table is the diagnostic map); never retry beyond the single EBUSY-once rule of §5.
- Every fix round re-reads `.gitignore` before staging (gitignore-compliance rule) and re-runs `git status --short`; `.agent/project-info/context.md` is planner/caller territory — NEVER staged.

## §8 Git plan — which commits this task creates

| When | Commit | Content | Message (EXACT) |
| --- | --- | --- | --- |
| 4.2 start (after §3 step-1 pre-flight) unconditional | C1 | this plan file only | `docs: add wave5.1 task5 verification plan` |
| Only if fmt drift | C2 | fmt-reflowed `src/` files only | `style: apply cargo fmt` |
| Only if compile residual | C3 | the minimal compile-fixed files only (NON-Windows code named by the dev-checks compiler) | `fix: wave5.1 verification compile fix` |
| Only if clippy residual | C4 | the minimal clippy-fixed files only | `fix: wave5.1 verification clippy fix` |
| **4.6 (the mark step — separate workflow sub-step, §10)** | C5 | the TODO file only (§10 edits) | `task: mark wave5.1 task5 done` |

- NO commit for build artifacts: `dist/` is gitignored; `logs/checks/` logs are gitignored; NOTHING built is ever staged (§3 step-11 sweep; unstage anything ignore-matching if it appears). `logs/` and `dist/` NEVER staged — if `git status --short` shows either, unstage with `git restore --staged <path>` and report.
- NO `git push` (restricted to workflow Step 5: `main` → `origin` ONLY). NO amend/force. NO `git add -A`/`git add .` (explicit paths only). NO branch creation/switch (Step-2 territory — the branch already exists). NO merge (Step-5 territory). NO `CHANGELOG.md` touches (wave standing ban).
- Commit staging lists are checked with `git status --short` immediately before each commit; the staged list MUST equal the row's content.

## §9 Junior-implementer step-by-step checklist for 4.2

1. Host: `git branch --show-current` → MUST be `fix/wave-5-1-operator-feedback`. Separate calls: `git log --oneline -20` → HEAD `b25677d`, the wave chain below the tip (paste into the report); `git status --porcelain` → empty or ONLY the permitted residuals (§3 step 1). Any mismatch → STOP + ask the caller.
2. Host: read `.gitignore` (sweep reference). Read §4 so every comparison is pre-armed. Run the §3 step-2 census STRUCTURE check (host grep; built-in Grep tool fallback) → the §0 table, hand-sum 164; report ANY divergence to the caller BEFORE running the harness (caller requirement). Run the §3 step-3 version check → `0.5.1`.
3. `vm_status` → running + SSH reachable; else STOP + user question.
4. §3 P1 LF guard (both scripts) → exit 0 twice; exit 1 → STOP + report.
5. §3 C1: stage the plan file, commit `docs: add wave5.1 task5 verification plan`; verify the staged list with `git status --short` before committing.
6. `vm_status`; §3 step 4 FIRST harness run (the DISCOVERY run) at 900000 ms. Compare against §4 line-by-line as output streams; RECORD the printed log path. REMEMBER: this run covers the stub path only — it can NEVER green-light or red-flag the windows-target code (§0).
   - All four `PASS` lines + literal `ALL CHECKS PASSED` + exit 0 + `test result: ok. 164 passed; 0 failed` → go to 7.
   - `FAIL fmt-check` → §7 fmt branch (1 max) → ONE re-run → back to 6.
   - `FAIL check` → §7 compile branch (max 2 rounds, one-more-attempt rule) → re-run → back to 6.
   - `FAIL clippy` → §7 clippy branch (one-more-attempt rule) → re-run → back to 6.
   - `FAIL test` → STOP + report immediately (§7).
   - Post-cap failures (harness invocation count exceeds the discovery run + 2 re-runs) → STOP + report with the four result lines + log path.
7. `vm_status`; §3 steps 5–6 census + ignore sweep → the §0 table / hand-sum 164 / `0`; mismatch → STOP (§1.2).
8. `vm_status`; §3 steps 7a–7k wave-pin sweeps → EVERY row exact per §4; any mismatch → STOP + report actual output verbatim (§7). The 7h host git diffs must ALL be EMPTY.
9. `vm_status`; §3 step 8 release build → expect §4 build rows; any typed error → STOP + report verbatim (§7: a windows-target compile error is NOT self-fixable under the residual-fix class). Exit 0 here IS the windows-collation compile evidence — record it as such.
10. `vm_status`; §3 steps 9 + 9b → RECORD the exact as-built byte size verbatim; dist dir must contain exactly `img-scroller.exe` (no `.part`).
11. `vm_status`; §3 step 10 DLL audit → 22 lowercase lines set-equal to §6; ANY delta → STOP + return the exact delta to the caller (§6); no output → §3 step 10b fallback then compare; record the §6 verbatim evidence statement.
12. Host: re-read `.gitignore`; `git status --short` sweep → NOTHING staged, no ignore-matching paths anywhere beyond the permitted caller residual; no unconditional commit remains pending (C1 already made; C2/C3/C4 only if their branches fired).
13. Report to caller (the full evidence set): branch/HEAD + the wave-chain `git log` lines; the pre-flight census result (164/26 files, reported before the run); harness log path; the four PASS lines + `ALL CHECKS PASSED` + exit 0 + the `164 passed; 0 failed` line of the FINAL run; the per-file census split + hand sum + zero-ignore result; every §3 sweep result (7a–7k) with the expected values reached; the discovery-run narrative (what failed, what was fixed, which commits landed — if any fix branch fired); the windows-code compile narrative (step 8 exit 0 = the cfg(windows) compile evidence; the dev-checks harness did NOT compile that target — §0/§4); `build-windows: OK`/artifact/exit 0; the as-built exe byte size (verbatim, with the 19,606,124 B 0.5.0 magnitude noted as reference); the §6 verbatim audit evidence statement (or the STOP + exact delta); git-sweep result; **and the §11 OPERATOR-RUN checklist block copied VERBATIM for handoff to the caller** — plus the explicit note that the Task-5 manual-UX (OPERATOR-RUN) sub-item remains PENDING until the user reports on a real Windows machine.

## §10 4.6 mark semantics (the Task-5 mark step; NOT executed inside 4.2)

When the Critical Workflow reaches 4.6 for Task 5 (AFTER §9 is fully green):

1. Read the TODO file [`.agent/todos/20261112/20261112-todo-1.md`](../../.agent/todos/20261112/20261112-todo-1.md) FIRST; PRESERVE all other content byte-for-byte (overwrite-prevention rule). EXACTLY these edits inside `### Task 5: Verification`:
   - Heading: `### Task 5: Verification` → `### Task 5: Verification [DONE]`.
   - Sub-item 1 (the unit-tests/harness line): `- [ ]` → `- [x]` — box change ONLY; NO wording change (§4's FIRST-branch-green harness evidence makes this check legal).
   - Sub-item 2 (the release-rebuild + DLL-audit line): `- [ ]` → `- [x]` — box change ONLY; NO wording change (§4's step-8/9/10 evidence makes this check legal).
   - Sub-item 3 (the manual-UX / OPERATOR-RUN line): **STAYS `- [ ]`** — pending the user's report on the real Windows machine (Wave-3/4/4.1/4.2/5 precedent: heading marked, agent-verifiable sub-items checked, manual-UX sub-item left unchecked until the operator reports). Do NOT annotate it with a status word; just leave the box.
   - No OTHER task heading, sub-item, section, or note is touched; the Future-session-execution-notes and Deferred-decisions sections are NOT edited; the sub-item wording is NEVER reworded (the checklist wording lives in THIS plan §11, referenced by the mark — the todo's own sub-item 3 text stays byte-identical).
2. Stage EXACTLY `.agent/todos/20261112/20261112-todo-1.md`; verify with `git status --short` that the staged list is only this file; commit `task: mark wave5.1 task5 done`.
3. NO `git push`, NO `-DONE` file rename (workflow Step 5 territory), `.agent/project-info/context.md` untouched by the implementer — the PLANNER copies §11 verbatim into `context.md` → "Immediate Next Steps" (Wave-4.2/5 precedent) so the operator pickup uses the same text.

## §11 Wave-5.1 manual check (OPERATOR-RUN) — verbatim block

(Verbatim from the caller's task source: the todo's `### Task 5: Verification` THIRD sub-item, copied byte-for-byte; handed by the 4.2 report to the CALLER for the user; run with the freshly rebuilt 0.5.1 exe at `dist\windows-x64\img-scroller.exe` on a real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container.)

```
Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): the SINGLE re-test runs the 0.5.1 exe against — (a) Shift+'+' zooms IN (Shift+'−' zooms out; plain keys inert); (b) the popup disappears by itself after ~2 s (then all the close-contract behaviors again); (c) the navigation NAME ORDER with Japanese-mixed names matches the expected order on BOTH axes (images in the folder; sibling folders) — the operator compares against the order they expect (e.g., the system file manager's order); plus the standing regressions (←/→/↑/↓ lists, silent edges, buttons, I/O/0/W/H, pan/edge-navigate behaviors from the passed Wave-5 gate).
```

REGRESSION NOTE (binding, accompanies the verbatim block): every 0.4.2/0.5.0-established behavior stays expected-good on top of the three fixes — captions WHITE + thin black borders; `I`/`O`/`0`/`W`/`H` and buttons unchanged; ESC exits fullscreen while ON; launch always maximized, console hidden; tooltips/hotkey captions; one-shot fit; symmetric silent zoom bounds; empty-folder mode; last-viewed memory; the Wave-5 pan/edge-navigate behaviors (zoom wide → keys pan, navigation only at edge; zoom tall → same vertically; small image → direct navigation — the PASSED Wave-5 gate).

Result handling: recorded by the Planner at wave close; the Task-5 manual-UX (OPERATOR-RUN) sub-item STAYS `[ ]` in the TODO until the user reports (binding TODO structure; §10). The re-test is the SINGLE 0.5.1 gate — it supersedes the still-open 0.4.2 checklist items (alias + expiry) in one run; the name-order check (c) is NEW on this exe.

## §12 Divergence note — the wave's Step 5 merge depends on this harness evidence

Step 5 of the global plan (TODO-file rename → merge `fix/wave-5-1-operator-feedback` into `main` → push `main` to `origin`) may proceed ONLY after this task's 4.2 green evidence exists (§3 step 4 `ALL CHECKS PASSED` at HEAD + step 8 build exit 0 + step 10 audit set-equal). A STOP anywhere in §3/§7 therefore BLOCKS Step 5 until the caller resolves it: the merge is never performed on red or missing evidence, and the implementer never performs the merge (Step-5 territory owned by a separate implementer invocation).

## §13 Deliberately out of scope (what 4.2 and this task's mark must NOT do)

- NO feature-code changes anywhere (no alias/expiry/collation/platform/listing/navigation/pan/zoom/fit/fullscreen/display edits beyond §7's sanctioned residual-fix rounds) — Task 5 verifies, it does not build.
- NO new tests, NO test additions/renames/skips "while here" (§1.1 gap verdict: none; census pinned at 164).
- NO TODO-file edits inside 4.2 (the Task-5 mark is the 4.6 step, §10; NO other todo file is touched either — including the already-done Tasks 1–4 headings and the `20261029-todo-7-DONE.md` file).
- NO docs updates — Task 6 owns README/tech/architecture/context/brief.md; NO OBSERVED-DLL doc refresh here (a changed list is REPORTED, never recorded, per §6).
- NO `CHANGELOG.md` touches (wave standing ban). NO edits to existing `.kilo/plans/*` files, INCLUDING this plan once it lands (new plan files only, ever).
- NO version changes (0.5.1 landed at Step 3). NO `Cargo.toml`/`Cargo.lock` touches at all — including no feature additions/removals (the `Win32_Globalization`/`Win32_System_SystemServices` additions are DONE and committed).
- NO windows-target code fixes under §7's residual-fix class (the cfg(windows) code is not container-compile-checkable; a step-8 windows compile error → STOP + caller decision, §7).
- NO x86 target; NO Wave-6/7 work; NO "Open with..."; NO parked future-ideas items (the recorded pan-enhancements idea is OUT of scope — record-only); NO `.agent/project-info/context.md` staging or editing (the PLANNER copies §11 there).
- NO `git push`, NO branch creation/switch, NO merge (Step 2/Step 5 territory). NO `FORCE_FAIL` invocations. NEVER stage `dist/**`, `logs/**`, `target/**`, `*.log`, or `Cargo.*`.

## §14 Stop conditions → return a user question

- `vm_status` not running / `/img-scroller` share unavailable at any VM-gated step (harness unavailability — binding wave note).
- §3 step-1 pre-flight evidence mismatch (branch ≠ `fix/wave-5-1-operator-feedback`, HEAD ≠ `b25677d`) or any unexplained porcelain entry.
- Pre-flight census STRUCTURE check ≠ the §0 26-file table / hand-sum ≠ 164 (reported BEFORE the harness run — caller requirement).
- Version check ≠ `0.5.1` at `Cargo.toml:3`.
- P1 LF guard exit 1 (CRLF recurrence — class outside this task).
- Any `--locked` lock-drift error (§2) — STOP; NO reconcile improvisation.
- Any harness `FAIL test` line (no self-fix); any harness FAIL persisting beyond §7's sanctioned rounds/caps (return the verbatim full captured output + log path).
- Harness summary ≠ `164 passed; 0 failed; ...` (§1.2); per-file grep census ≠ the §0 table or hand-sum ≠ 164; any `#[ignore]`/`#[should_panic]` hit.
- Any §3 sweep (7a–7k) mismatching its Expected (report actual output verbatim), including ANY non-empty 7h git diff.
- Any `build-windows: error:` typed line (report verbatim; the docs failure table is the diagnostic map); a WINDOWS-TARGET compile error is caller-decision territory (§7).
- A dangling `img-scroller.exe.part` after a successful build (9b).
- DLL audit ≠ the 22-OBSERVED set (STOP + report the EXACT delta; never record a new baseline; the audit decides, not an assumption).
- Any §13 out-of-scope temptation → STOP; the caller decides.
- Anything ambiguous or outside this plan's assigned scope → return the question to the caller. NEVER make assumptions.
