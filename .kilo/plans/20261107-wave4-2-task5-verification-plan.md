# Wave 4.2 Task 5 — Verification Plan 4.1b (fresh HEAD dev-checks, census 127 corroboration, 0.4.2 release rebuild + byte-size record, DLL import audit set-equal 22, wave-pin sweeps, Git segmentation, 0.4.2 OPERATOR-RUN checklist)

- Date: 2026-11-07 (plan author: architector, workflow step 4.1b; executed by the implementer in 4.2).
- Source TODO: [`.agent/todos/20261107/20261107-todo-1.md`](../../.agent/todos/20261107/20261107-todo-1.md) → `### Task 5: Verification` ONLY (exactly its three sub-items) + the wave's `## Future session execution notes`. NO other task's plans are expanded or executed.
- NOT front-end ⇒ sub-step **4.1b ONLY** (no 4.1a exists for Task 5).
- Workflow slot: Wave-4.2 Critical Workflow (global plan [`.kilo/plans/20261107-wave42-then-wave5-global-plan.md`](20261107-wave42-then-wave5-global-plan.md) → A-Task 5, step 4.1b) — this PLAN → executed by the implementer in 4.2; the Task-5 `[DONE]` mark is the separate 4.6 step (encoded in §10; NOT executed inside 4.2).
- Build docs (binding usage): [`docs/build.md`](../../docs/build.md), [`docs/build-windows.md`](../../docs/build-windows.md); harness scripts: [`scripts/dev-checks.sh`](../../scripts/dev-checks.sh), [`scripts/build-windows.sh`](../../scripts/build-windows.sh).
- Precedent followed (structure, command table, census rule, mark semantics, checklist block): [`.kilo/plans/20261102-wave-4-1-task8-verification-plan.md`](20261102-wave-4-1-task8-verification-plan.md) — same shape, updated to the Wave-4.2 landed state.
- Approval: the user's global-plan approval makes every decision below BINDING; zero judgment left for the implementer. If any detail conflicts with something ACTUALLY observed during 4.2, STOP and return the question to the caller (§14).
- Target implementer: JUNIOR under 50% restriction. All structural/scope decisions are encoded here; §13 lists the out-of-scope guard verbatim.

---

## §0 Session facts captured at plan time (verified evidence; branch `fix/wave-4-2-operator-feedback` @ HEAD `f604051`)

- Branch: `fix/wave-4-2-operator-feedback`; HEAD **`f604051`** (`task: mark wave4.2 task4 done`); working tree clean at plan time. The chain below the tip is the wave's Tasks 1–4 landing chain: `f2857c5` (Task-2 spec+plan docs) → `4c3c959` (Task-2 feat) → `c53fbf0`/`d90e9ee`/`f2e560d`/`a7ddc60` (Task-2 docs/fix/mark) → `80b442f` (Task-3 plan) → `5ec7574` (Task-3 feat) → `5e3827d`/`5edea1c` (Task-3 docs/mark) → `f4e9a55` (Task-4 spec+plan docs) → `7e235fa` (Task-4 feat) → `f72d076` (Task-4 review fix — LAST code commit) → `a7e8be9`/`f604051` (Task-4 docs + mark; docs-only). The implementer ENUMERATES it at pre-flight with host `git log --oneline -16` (§3 step 1) and pastes the lines into the 4.2 report — the plan deliberately pre-records NO further SHAs beyond `f604051` (caller-provided, the only pinned one).
- **Prior-green context (caller-provided; the reason for step 2's FRESH run): dev checks ended `ALL CHECKS PASSED` earlier in the wave (127 tests) at the `f72d076`-era code; the commits after it are docs/mark-only.** The wave still requires HEAD-level stamped evidence — a FRESH §3 step-2 run at `f604051` is MANDATORY. Skipped/deferred/reused prior evidence is NOT acceptable.
- Version **0.4.2** (verified: `Cargo.toml` `[package] version = "0.4.2"`; bumped at Step A.3). NOTHING touches `Cargo.toml`/`Cargo.lock` in this task (§2).
- Test census verified at plan time by host-side `rg -c "#\[test\]" src` — **21 files, grand total 127**:

| File | `#[test]` count |
| --- | --- |
| `src/sibling_folder_listing/tests.rs` | 6 |
| `src/sibling_folder_listing/neighbor_tests.rs` | 2 |
| `src/name_collation.rs` | 2 |
| `src/folder_listing/tests.rs` | 5 |
| `src/app/tests.rs` | 9 |
| `src/app/floating_controls_tests.rs` | 1 |
| `src/app/button_hint_tests.rs` (Task-2 simplification relocated the 2 inline pins) | 2 |
| `src/app/pill_style_tests.rs` (**Task 2 — white caption + outlined glyphs**) | 3 |
| `src/app/folder_navigation_tests.rs` | 5 |
| `src/app/folder_navigation_continuation_tests.rs` | 4 |
| `src/app/last_viewed_tests.rs` | 5 |
| `src/app/fullscreen_tests.rs` | 9 |
| `src/app/zoom_tests.rs` (Task-3 refactor delegated aliases out; 16 → 11) | 11 |
| `src/app/zoom_alias_tests.rs` (**Task 3 — accepted real-Windows input resolutions**) | 14 |
| `src/app/esc_close_confirm_tests.rs` (**Task 4 — popup state machine**) | 11 |
| `src/app/fit_one_shot_tests.rs` | 12 |
| `src/app_display.rs` | 2 |
| `src/image_loading.rs` | 4 |
| `src/zoom.rs` | 8 |
| `src/scaling.rs` | 5 |
| `src/scaling/fit_factor_tests.rs` | 7 |
| **Total** | **127** |

- No `#[ignore]` / `#[should_panic]` markers anywhere in `src/` (verified by grep at plan time) — nothing skipped, nothing deferred.
- LAST KNOWN EXE size (delta-only reference, never an expectation): 0.4.1 = **19,593,177 bytes**; `dist/windows-x64/img-scroller.exe` on disk currently holds that 0.4.1 artifact and will be ATOMICALLY overwritten (`.part` copy → rename) by the 0.4.2 build. The 0.4.2 exe size WILL differ; any non-empty size is acceptable — the verbatim as-built value is the deliverable. NO target size number is a valid pre-requirement.
- `OBSERVED_DLL_IMPORT_LIST` constant verified present at [`scripts/build-windows.sh`](../../scripts/build-windows.sh) line 15 (identical in [`docs/build-windows.md`](../../docs/build-windows.md) → "Static-linking / import audit"), 22 DLLs — the audit comparator (§6).
- Harness semantics (docs/build.md + `scripts/dev-checks.sh`): fixed order `fmt-check` → `check` → `test` → `clippy`; EVERY check runs even if earlier ones fail; per-check line `PASS|FAIL <name> exit=<code> (<n> s)`; final literal `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1); UTC-timestamped log appended under the gitignored `logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log`, the exact path printed at run start is authoritative; the script never modifies sources and never applies fmt fixes; `FORCE_FAIL` exists but is NOT used this task.
- build-windows semantics (docs/build-windows.md + script): runs `cargo build --release --locked --target x86_64-pc-windows-gnu` with cwd `/img-scroller` and the compose service's `CARGO_TARGET_DIR`; success prints `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, then the OBSERVED-DLL evidence line; install is ATOMIC (`.part` copy → rename over the final name; a failed run leaves the previous artifact byte-for-byte intact and never leaves a surviving `.part`); every failure line starts with the literal prefix `build-windows: error:` and exits 1.
- `.gitignore` contains (verified read) `*.log`, `logs/`, `dist/`, `target/` — artifacts and check logs are NEVER stageable.
- 0.4.2 wave pins verified at plan time (feed §5's sweep expectations):
  - ESC-close popup constants: `POPUP_MESSAGE_TEXT: &str = "Press ESC again to close"` — exactly `src/app/esc_close_confirm.rs:21`; `POPUP_VISIBLE_SECONDS: f64 = 2.0` — exactly `esc_close_confirm.rs:24`.
  - Close command: `ViewportCommand::Close` — exactly ONE code line: `src/app/keyboard_intake.rs:91`.
  - Non-repeat ESC filter wired: `non_repeat_escape_press_occurred` — exactly ONE code call line: `keyboard_intake.rs:31` (definition `esc_close_confirm.rs:64`; tests use it legitimately).
  - Alias accept-set (Task 3): `src/app/zoom_alias.rs` lines 12/14/15/16/17 — `TRUE_MINUS_SIGN: char = '\u{2212}';`, `ZOOM_IN_ALIAS_KEYS: [egui::Key; 2] = [egui::Key::Equals, egui::Key::Plus]`, `ZOOM_OUT_ALIAS_KEYS: [egui::Key; 1] = [egui::Key::Minus]`, `ZOOM_IN_ALIAS_CHARACTERS: [char; 2] = ['+', '=']`, `ZOOM_OUT_ALIAS_CHARACTERS: [char; 3] = ['-', TRUE_MINUS_SIGN, '_']`.
  - Alias delegation: `zoom_alias::zoom_in_alias_was_pressed` / `zoom_out_alias_was_pressed` used exactly at `src/app/zoom.rs:138` / `:147`; the consumer chords `zoom_in_chord_was_pressed` / `zoom_out_chord_was_pressed` used exactly at `src/app/keyboard_intake.rs:32` / `:33`.
  - Pill styling (Task 2): `PILL_GLYPH_COLOR: egui::Color32 = egui::Color32::WHITE` — `src/app/pill_style.rs:12`; `OUTLINE_COLOR: egui::Color32 = egui::Color32::BLACK` — `pill_style.rs:15`; `OUTLINE_STROKE_WIDTH: f32 = 1.0` — `pill_style.rs:18`.
  - Fullscreen-toggle disarm: `self.disarm_esc_close_popup()` — exactly ONE code call: `src/app/fullscreen.rs:31` (every fullscreen toggle disarms the popup).
  - Popup render: `esc_close_confirm::show_esc_close_popup(self, ui.ctx());` — exactly ONE code call: `src/app.rs:136`.
  - Regressions carried from the 0.4.1 plan, line numbers as-verified NOW: `windows_subsystem` → exactly `src/main.rs:4`; `with_maximized(true)` → exactly ONE code line `src/app.rs:152`; `message_text = ` writes → EXACTLY 3 sites (`src/app.rs:89` = `None;`, `src/app.rs:94` = `Some(image_loading::UNSUPPORTED_FILE_MESSAGE);`, `src/app/folder_navigation.rs:107` = `None;` — NONE contains "Zoom"/"Fit"); symmetric floor pin `ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM;` → exactly `src/zoom.rs:20` (definition line).
- Harness-execution deferral: NONE of §3's VM steps have been run yet; this plan is authored BEFORE any 4.2 execution (the caller forbade running the harness in this planning step).

## §1 Unit-coverage mapping (TODO Task 5 sub-item 1 → the three wave code tasks' landed tests)

The TODO sub-item 1 ("Unit tests … run via the dev-checks harness in the container per `docs/build.md`") maps onto the landed 127 as:

| TODO wave task | Satisfied by | File(s) |
| --- | --- | --- |
| Task 2: caption color/outline composition "covered where pure logic exists" | the 3 `pill_style_tests` (palette/negative-outline/outlined-caption pins) + the 2 relocated `button_hint_tests` pins + the pill-census test | `src/app/pill_style_tests.rs`, `src/app/button_hint_tests.rs`, `src/app/floating_controls_tests.rs` |
| Task 3: alias-detection fix "covers the real-input resolutions researched in Task 3 … with synthetic input" | the 14 `zoom_alias_tests` (KEY-family Equals/Plus/Minus, typed-character '+'/='/'−'/'U+2212'/'_' members, Shift-gating, strict plain-key inertness, chord-delegation pins) + the 11 `zoom_tests` (app policy incl. silent bounds, restore, one-shot fit interplay) | `src/app/zoom_alias_tests.rs`, `src/app/zoom_tests.rs` |
| Task 4: ESC popup state machine (show → within-window close → expiry → re-arm), repeat filter, fullscreen-ON interplay | the 11 `esc_close_confirm_tests` (arm/deadline visibility, expiry at/before deadline, non-repeat press filter incl. repeat + non-Escape, branch matrix visible/invisible × pressed/idle, re-arm, fullscreen-toggle disarm) | `src/app/esc_close_confirm_tests.rs` |
| Regressions (navigation, listing, collation, memory, open flow, unsupported message, fullscreen F11/ESC-exit, fit) | the remaining 85 tests (127 − 6 Task-2-related − 25 Task-3-related − 11 Task-4) unchanged in this wave | all files above |

### §1.1 GAP VERDICT

**No gaps.** Every topic nameable from the wave's Tasks 2/3/4 is pinned by landed tests. **NO test additions are sanctioned in this task; the expected census stays EXACTLY 127.** Any "why not one more test" improvisation in 4.2 → STOP (§14).

### §1.2 Census-failure rule

If the harness `cargo test` summary does not read exactly `test result: ok. 127 passed; 0 failed; ...` (a trailing doc-test `test result: ok. 0 passed` line is expected and normal — NOT a second count): **STOP** and return the full test section to the caller. A census mismatch is a defect, never an ad-hoc-patch opportunity.

## §2 Lockfile status — NO reconcile needed

Version 0.4.2 landed at Step A.3 with the committed lock; the wave's Tasks 1–4 introduced NO dependency changes. Therefore NO reconcile command, NO lock commit this task. Conditional STOP: if ANY `--locked` invocation (dev-checks step 2, or the release build) fails with a "the lock file needs to be updated but --locked was passed"-class error — **STOP immediately** and return the error verbatim to the caller. A lock-drift failure here means a committed-state inconsistency the plan's evidence did not show — caller decision, NOT a junior self-fix.

## §3 Command sequence for 4.2 (exact; per docs/build.md and docs/build-windows.md)

Command channel: the `alpine-vm` MCP tools — call `alpine-vm_vm_status` BEFORE EVERY `alpine-vm_vm_run_command` call; ONE MCP call per row; each row's exact `command` string is passed verbatim; each command gets its own timeoutMs from the table; the `exitCode` field is authoritative; captured MCP stderr may be empty — an absent stderr is NOT a failure signal (documented caveat). Host-side git commands run via the Bash tool ONE command per invocation (never chained, never `&&`).

| # | Step | Command (exact) | timeoutMs | Expected |
| --- | --- | --- | --- | --- |
| 0 | VM gate | `vm_status` (MCP tool) | — | running + SSH reachable; else STOP + user question (binding wave note) |
| P1 | Pre-flight LF guard ×2 | `sh -c "sed 's/\r$//' /img-scroller/scripts/dev-checks.sh \| cmp - /img-scroller/scripts/dev-checks.sh"`, then the IDENTICAL command for `/img-scroller/scripts/build-windows.sh` | 60000 | exit 0, no output, BOTH times. exit 1 (CRLF recurrence) → STOP + report (class outside Task-5 scope; caller owns the fix shape) |
| 1 | Pre-flight git evidence (host, Bash tool) | `git branch --show-current`, then SEPARATE calls: `git log --oneline -16`, then `git status --porcelain` | — | branch = `fix/wave-4-2-operator-feedback`; HEAD = `f604051`; the Tasks-1–4 chain visible below the tip (per §0); porcelain EMPTY or showing ONLY this task's permitted residuals (this plan file before C1, and/or a caller-owned modified `.agent/project-info/context.md`). Any OTHER entry → STOP + ask the caller. Paste the `git log` lines into the report (the Tasks-1–4 evidence table) |
| C1 | Plan-file commit (wave task-level precedent `80b442f`/`f4e9a55`) | host: `git add .kilo/plans/20261107-wave4-2-task5-verification-plan.md`; then `git commit -m "docs: add wave4.2 task5 verification plan"` | — | commit succeeds; `git status --porcelain` afterwards shows NO repo-owned file except (possibly) a caller-owned modified `.agent/project-info/context.md` |
| 2 | **Fresh dev-checks harness run at HEAD** (sub-item 1's authoritative HEAD-level evidence; MANDATORY even though prior green evidence exists earlier in the wave) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` | 900000 | four `PASS` lines in fixed order + final literal `ALL CHECKS PASSED`, **exit code 0**; harness test block `test result: ok. 127 passed; 0 failed; ...` (+ doc-test `0 passed` line, normal); RECORD the printed log path |
| 3 | Census corroboration | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rc --include='*.rs' '#\[test\]' /img-scroller/src \| sort"` | 60000 | per-file counts COMPLETELY EQUAL to the §0 21-file table; implementer sums the 21 numbers by hand → grand total **127**; corroborates the harness `127 passed` line (the harness number is the binding figure) |
| 4 | Skip/ignore-marker sweep | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn --include='*.rs' -e '#\[ignore\]' -e '#\[should_panic\]' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `0`, exit 0 — no skipped/ignored tests |
| 5a | Sweep: popup sentence + lifetime pins | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'POPUP_MESSAGE_TEXT: &str = \"Press ESC again to close\"\|POPUP_VISIBLE_SECONDS: f64 = 2.0' /img-scroller/src/app/esc_close_confirm.rs"` | 60000 | EXACTLY 2 lines: `21` (the exact `"Press ESC again to close"` sentence) and `24` (`2.0`) |
| 5b | Sweep: the ONE app-close send | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn 'ViewportCommand::Close' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `1` (code line only: `keyboard_intake.rs:91`) |
| 5c | Sweep: non-repeat ESC filter wired once | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'non_repeat_escape_press_occurred' /img-scroller/src/app/keyboard_intake.rs"` | 60000 | EXACTLY 1 line: `31` |
| 5d | Sweep: alias accept-set pins (Task-3 fix shape) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'ZOOM_IN_ALIAS_KEYS\|ZOOM_OUT_ALIAS_KEYS\|ZOOM_IN_ALIAS_CHARACTERS\|ZOOM_OUT_ALIAS_CHARACTERS\|TRUE_MINUS_SIGN: char' /img-scroller/src/app/zoom_alias.rs"` | 60000 | EXACTLY 9 lines: the DEFINITIONS at `12` (`TRUE_MINUS_SIGN: char = '\u{2212}';`), `14` (in-keys Equals+Plus), `15` (out-keys Minus), `16` (in-chars '+'+'='), `17` (out-chars '-'+TRUE_MINUS_SIGN+'_'), each reading the §0 shape — assert by eye; PLUS the 4 use lines `24`, `25`, `29`, `30` inside `zoom_in_alias_was_pressed`/`zoom_out_alias_was_pressed`. NO other line |
| 5e | Sweep: alias delegation + consumer chords | first `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'zoom_in_alias_was_pressed\|zoom_out_alias_was_pressed' /img-scroller/src/app/zoom.rs"` (expect EXACTLY 2 lines: `138` zoom-in and `147` zoom-out delegate into `zoom_alias`; the `use super::zoom_alias;` import at line 39 must NOT match this pattern); then `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'zoom_in_chord_was_pressed\|zoom_out_chord_was_pressed' /img-scroller/src/app/keyboard_intake.rs"` (expect EXACTLY 2 lines: `32` and `33`) | 60000 each | both greps 2 lines each, exact per text |
| 5f | Sweep: pill-styling pins (Task-2 shape) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'PILL_GLYPH_COLOR\|OUTLINE_COLOR\|OUTLINE_STROKE_WIDTH' /img-scroller/src/app/pill_style.rs"` | 60000 | EXACTLY 6 lines: the three DEFINITIONS at `12` (`PILL_GLYPH_COLOR: egui::Color32 = egui::Color32::WHITE`), `15` (`OUTLINE_COLOR: egui::Color32 = egui::Color32::BLACK`), `18` (`OUTLINE_STROKE_WIDTH: f32 = 1.0`) — assert the shapes by eye — PLUS the three USE lines `90`, `104`, `138`. NO other line |
| 5g | Sweep: fullscreen-toggle disarms the popup | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn 'self.disarm_esc_close_popup()' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `1` (`src/app/fullscreen.rs:31`) |
| 5h | Sweep: popup render call site | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'show_esc_close_popup' /img-scroller/src/app.rs"` | 60000 | EXACTLY 1 line: `136` |
| 5i | Sweep: removed-behavior message ground truth | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn 'message_text = ' /img-scroller/src"` | 60000 | EXACTLY 3 lines: `src/app.rs:89` (`= None;`), `src/app.rs:94` (`= Some(image_loading::UNSUPPORTED_FILE_MESSAGE);`), `src/app/folder_navigation.rs:107` (`= None;`). Assert by eye: none of the 3 contains "Zoom" or "Fit" |
| 5j | Sweep: launch/console/floor regression pins | three greps: `'windows_subsystem' /img-scroller/src` (exactly 1 line `main.rs:4`); `'with_maximized' /img-scroller/src` (exactly 1 code line `app.rs:152`); `'ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM' /img-scroller/src/zoom.rs` (exactly 1 line `zoom.rs:20`) | 60000 each | every grep exact |
| 6 | Release cross-build (0.4.2 exe) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` | 900000 | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; the OBSERVED-DLL evidence line; **exit code 0** |
| 7 | Exe inspection (record the byte size) | `sh -c "ls -l /img-scroller/dist/windows-x64/img-scroller.exe"` | 60000 | non-empty size printed; RECORD the exact as-built byte count verbatim (any non-empty size passes; no pre-requirement) |
| 7b | No dangling `.part` | `sh -c "ls /img-scroller/dist/windows-x64"` | 60000 | the listing shows EXACTLY one entry: `img-scroller.exe` — no `img-scroller.exe.part` (atomic install completed; the part file never survives success) |
| 8 | DLL import audit (per docs/build-windows.md) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe \| grep -i 'DLL Name' \| tr 'A-Z' 'a-z' \| sed 's/.*dll name: //' \| sort -u"` | 180000 | exactly 22 lowercase DLL lines, SET-EQUAL to the recorded constant (§6) |
| 8b | Audit fallback (ONLY if step 8 yields no output — report-format drift) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` | 180000 | raw dump; compare manually against §6; large output, still ONE call |
| 9 | Gitignore compliance sweep (host) | re-read `.gitignore`, then `git status --short` | — | NO `dist/`, `logs/`, `target/`, `*.log` entries anywhere in the output; nothing staged; the only allowed residual is a caller-owned modified `.agent/project-info/context.md` |

## §4 Expected outcomes table (authoritative)

| Evidence | Expected |
| --- | --- |
| Harness log header | `dev-checks starting; log file: logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` — RECORD the exact printed path |
| Harness check lines (fixed order) | `PASS fmt-check exit=0 (<n> s)`; `PASS check exit=0 (<n> s)`; `PASS test exit=0 (<n> s)`; `PASS clippy exit=0 (<n> s)` (clippy runs `-D warnings`, so exit 0 means zero warnings) |
| Harness test block | `test result: ok. 127 passed; 0 failed; ...` + doc-test `test result: ok. 0 passed` (normal, not a second count) |
| Harness final | literal `ALL CHECKS PASSED`; **exit code 0 = the authoritative success evidence** (sub-item 1's gate; HEAD-level stamped evidence for the wave) |
| Census corroboration | per-file grep counts == §0 table; hand-summed total 127; zero `#[ignore]`/`#[should_panic]` |
| §5 sweeps & pins | 5a the 2 lines (`:21`, `:24`); 5b `1` (keyboard_intake.rs:91); 5c 1 line (`:31`); 5d exactly 9 lines (definitions 12/14/15/16/17 + uses 24/25/29/30), shapes asserted by eye vs §0; 5e zoom.rs 2 lines (138+147), keyboard_intake.rs 2 lines (32+33); 5f exactly 6 lines (defs 12/15/18 + uses 90/104/138); 5g `1` (fullscreen.rs:31); 5h 1 line (`app.rs:136`); 5i exactly 3 message sites, none Zoom/Fit; 5j all three regression pins exact |
| Release build | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; OBSERVED-DLL evidence line; **exit code 0** |
| Exe | NON-EMPTY gitignored `dist/windows-x64/img-scroller.exe` (0.4.2 wave build); exact as-built size RECORDED (delta vs the 19,593,177 B prior 0.4.1 magnitude is evidence only — any non-empty size passes); NO dangling `.part` (7b: the dist dir lists exactly `img-scroller.exe`) |
| DLL audit | 22 lowercase DLL names, SET-EQUAL to the recorded 2026-10-31 constant (§6) — DLL import list UNCHANGED |
| Git sweep | nothing ignored staged; nothing staged at all after C1 except the 4.6 §10 mark commit later |

## §5 Alpine-VM MCP usage notes (binding, per docs/build.md)

- `vm_status` BEFORE every `vm_run_command` call; only proceed when the VM is running and SSH-reachable; otherwise STOP and return a user question (docs/build.md precondition; wave execution note).
- ONE command per MCP call; pipelines live INSIDE one quoted `sh -c "..."` string exactly as printed in §3.
- The `exitCode` field is the authoritative pass/fail evidence; `null` exitCode + `timedOut: true` → report the timeout with the partial captured output; do NOT re-run silently.
- Absent cargo stderr in captured MCP output is NOT a failure signal (documented caveat).
- Generous timeouts (§3 table) beat aggressive ones.
- EBUSY-on-share-write hazard (vboxsf): retry the IDENTICAL command ONCE; if it fails again → STOP + report with evidence.

## §6 DLL import audit — recorded constant and comparison evidence statement

Recorded `OBSERVED_DLL_IMPORT_LIST` (source of truth: `scripts/build-windows.sh` line 15, identical in `docs/build-windows.md`; audited 2026-10-31 with `x86_64-w64-mingw32-objdump`; docs-split rule: NOT duplicated into tech.md/architecture.md):

```
advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll,
api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll,
gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll,
opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll,
ws2_32.dll
```

Evidence statement to record in the 4.2 report (verbatim shape): "Wave-4.2 audit: `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` inside the container; output deduplicated + lowercase-folded via the §3 step-8 pipeline; OBSERVED = 22 DLLs; SET-EQUAL to the recorded 2026-10-31 constant — DLL import list UNCHANGED."

On ANY diff (extra, missing, renamed — dependency drift): **STOP + report** the full observed list and the delta. NO edits to `scripts/build-windows.sh` or `docs/build-windows.md` — refreshing the OBSERVED baseline is a Planner decision, never an implementer improvisation (Wave-1/2/3/4/4.1 precedent).

## §7 Failure/intervention policy for 4.2 (decision branches; max iterations PINNED)

- **fmt-check FAIL** → sanctioned minimal self-service fix: ONE `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` (timeoutMs 120000); host-side `git diff` MUST show formatting-only reflow (no semantic content moves); if changed files exist → stage EXACTLY the `src/` paths listed by `git status --short` (never `.agent/`, never `docs/`) and commit EXACTLY `style: apply cargo fmt` (separate commit; Wave-2 precedent); then re-run the §3 step-2 harness ONCE. Max 2 fmt-fix → 2 harness re-runs. Second consecutive fmt failure → STOP + report.
- **check FAIL** → STOP immediately. Compile defects are structural; report the full captured output + the printed log path (no self-fix — run-once-then-report).
- **test FAIL** → STOP immediately. Any failing named test = structural; §1.2's census rule protects against ad-hoc test invention; report with the failure detail lines. Census ≠ `127 passed` by ANY means → §1.2 STOP.
- **clippy FAIL** → minor-lint self-service sanctioned ONLY when the failure is a SINGLE lint in a SINGLE file whose fix is ≤ 3 source lines. Fix with the `edit` tool, commit EXACTLY `fix: resolve minor clippy lint (verification round)` staged to that file only, then re-run the harness ONCE. Max 1 such fix round; the second consecutive clippy failure OR any multi-site/structural lint → STOP + report.
- Global cap: at most 2 post-fix harness re-runs in total across all sanctioned fix rounds; anything failing after the cap → STOP + report with evidence (four result lines + log path).
- Any §3 sweep (3–4, 5a–5j) mismatching its Expected → STOP + report the actual output verbatim. A drift means committed-state deviation from the plan's verified ground truth — caller decision, never a self-fix.
- Any `build-windows: error:` line of ANY type → STOP + report the typed error verbatim (the docs/build-windows.md failure table is the diagnostic map); never retry beyond the single EBUSY-once rule of §5.
- Every fix round re-reads `.gitignore` before staging (gitignore-compliance rule) and re-runs `git status --short`; `.agent/project-info/context.md` is planner/caller territory — NEVER staged.

## §8 Git plan — which commits this task creates

| When | Commit | Content | Message (EXACT) |
| --- | --- | --- | --- |
| 4.2 start (after §3 step-1 pre-flight) unconditional | C1 | this plan file only | `docs: add wave4.2 task5 verification plan` |
| Only if fmt drift | C2 | fmt-reflowed `src/` files only | `style: apply cargo fmt` |
| Only if single minor clippy lint | C3 | the one ≤3-line fixed file | `fix: resolve minor clippy lint (verification round)` |
| **4.6 (the mark step — separate workflow sub-step, §10)** | C4 | the TODO file only (§10 edits) | `task: mark wave4.2 task5 verification done` |

- NO commit for build artifacts: `dist/` is gitignored; `logs/checks/` logs are gitignored; NOTHING built is ever staged (§3 step-9 sweep; unstage anything ignore-matching if it appears).
- NO `git push` (restricted to workflow Step 5: `main` → `origin` ONLY). NO amend/force. NO `git add -A`/`git add .` (explicit paths only). NO branch creation/switch (Step-2 territory — the branch already exists). NO merge (Step-5 territory). NO `CHANGELOG.md` touches (wave standing ban).
- Commit staging lists are checked with `git status --short` immediately before each commit; the staged list MUST equal the row's content.

## §9 Junior-implementer step-by-step checklist for 4.2

1. Host: `git branch --show-current` → MUST be `fix/wave-4-2-operator-feedback`. Separate calls: `git log --oneline -16` → HEAD `f604051`, the Tasks-1–4 chain below the tip (paste into the report); `git status --porcelain` → empty or ONLY the permitted residuals (§3 step 1). Any mismatch → STOP + ask the caller.
2. Host: read `.gitignore` (sweep reference). Read §4 so every comparison is pre-armed.
3. `vm_status` → running + SSH reachable; else STOP + user question.
4. §3 P1 LF guard (both scripts) → exit 0 twice; exit 1 → STOP + report.
5. §3 C1: stage the plan file, commit `docs: add wave4.2 task5 verification plan`; verify the staged list with `git status --short` before committing.
6. `vm_status`; §3 step 2 FRESH harness run at 900000 ms. Compare against §4 line-by-line as output streams; RECORD the printed log path.
   - All four `PASS` lines + literal `ALL CHECKS PASSED` + exit 0 + `test result: ok. 127 passed; 0 failed` → go to 7.
   - `FAIL fmt-check` → §7 fmt branch (2 max), then ONE re-run → back to 6.
   - `FAIL check` / `FAIL test` → STOP + report (§7).
   - `FAIL clippy` → §7 clippy branch (1 max) → ONE re-run → back to 6.
   - Post-cap failures → STOP + report with the four result lines + log path.
7. `vm_status`; §3 steps 3–4 census + ignore sweep → the §0 table / hand-sum 127 / `0`; mismatch → STOP (§1.2).
8. `vm_status`; §3 steps 5a–5j wave-pin sweeps → EVERY row exact per §4; any mismatch → STOP + report actual output verbatim (§7).
9. `vm_status`; §3 step 6 release build → expect §4 build rows; any typed error → STOP + report verbatim.
10. `vm_status`; §3 steps 7 + 7b → RECORD the exact as-built byte size verbatim; dist dir must contain exactly `img-scroller.exe` (no `.part`).
11. `vm_status`; §3 step 8 DLL audit → 22 lowercase lines set-equal to §6; any diff → STOP + report (no script/doc edits); no output → §3 step 8b fallback then compare; record the §6 verbatim evidence statement.
12. Host: re-read `.gitignore`; `git status --short` sweep → NOTHING staged, no ignore-matching paths anywhere beyond the permitted caller residual; no unconditional commit remains pending (C1 already made; C2/C3 only if their branches fired).
13. Report to caller (the full evidence set): branch/HEAD + the Tasks-1–4 `git log` lines; harness log path; the four PASS lines + `ALL CHECKS PASSED` + exit 0 + the `127 passed; 0 failed` line; the per-file census split + hand sum + zero-ignore result; every §5 sweep result (5a–5j) with the expected values reached; `build-windows: OK`/artifact/exit 0; the as-built exe byte size (verbatim, with the 19,593,177 B 0.4.1 magnitude noted as reference); the §6 verbatim audit evidence statement; git-sweep result; **and the §11 OPERATOR-RUN checklist block copied VERBATIM for handoff to the caller** — plus the explicit note that the Task-5 manual-UX (OPERATOR-RUN) sub-item remains PENDING until the user reports on a real Windows machine.

## §10 4.6 mark semantics (the Task-5 mark step; NOT executed inside 4.2)

When the Critical Workflow reaches 4.6 for Task 5 (AFTER §9 is fully green):

1. Read the TODO file [`.agent/todos/20261107/20261107-todo-1.md`](../../agent/todos/20261107/20261107-todo-1.md) FIRST; PRESERVE all other content byte-for-byte (overwrite-prevention rule). EXACTLY these edits inside `### Task 5: Verification`:
   - Heading: `### Task 5: Verification` → `### Task 5: Verification [DONE]`.
   - Sub-item 1 (the unit-tests/harness line): `- [ ]` → `- [x]` — box change ONLY; NO wording change (§4's HEAD-level harness evidence makes this check legal).
   - Sub-item 2 (the release-rebuild line): `- [ ]` → `- [x]` — box change ONLY; NO wording change (the rebuild + audit evidence makes this check legal).
   - Sub-item 3 (the manual-UX / OPERATOR-RUN line): **STAYS `- [ ]`** — pending the user's report on the real Windows machine (Wave-3/4/4.1 precedent: heading marked, agent-verifiable sub-items checked, manual-UX sub-item left unchecked until the operator reports). Do NOT annotate it with a status word; just leave the box.
   - No OTHER task heading, sub-item, section, or note is touched; the Future-session-execution-notes and Deferred-decisions sections are NOT edited; the sub-item wording is NEVER reworded (the checklist wording lives in THIS plan §11, referenced by the mark — the todo's own sub-item 3 text stays byte-identical).
2. Stage EXACTLY `.agent/todos/20261107/20261107-todo-1.md`; verify with `git status --short` that the staged list is only this file; commit `task: mark wave4.2 task5 verification done`.
3. NO `git push`, NO `-DONE` file rename (workflow Step 5 territory), `.agent/project-info/context.md` untouched by the implementer — the PLANNER copies §11 verbatim into `context.md` → "Immediate Next Steps" (Wave-4.1 precedent) so the operator pickup uses the same text.

## §11 Wave-4.2 manual check (OPERATOR-RUN) — verbatim block

(Verbatim from the caller's task; handed by the 4.2 report to the CALLER for the user; run with the freshly rebuilt 0.4.2 exe at `dist\windows-x64\img-scroller.exe` on a real Windows 10/11 machine. Wave-4.2 re-test scope: items 1–3 below are the NEW/restated expectations — the styling adjustment of item 1 supersedes the Wave-4.1 item-4 styling verdict, item 2 is the item-11 alias-defect fix, and item 3 adds the ESC double-press close feature on top of the Wave-4.1 ESC-exits-fullscreen rule. The non-regressed 0.4.1 behaviors (launch maximized, console hidden, captions/tooltips, F11/ESC tuning, symmetric silent zoom bounds, one-shot fit, navigation, empty-folder mode) remain expected-good and are additionally exercised implicitly by items 2–3.)

```
1. CAPTIONS: captions are WHITE and BOTH icons and captions show a thin black
   border, legible over light and dark content.
2. ZOOM ALIASES: SHIFT+'+' zooms IN and SHIFT+'−' zooms OUT (test main-row AND
   numpad if available); plain unshifted `+`/`=`/`-` still do nothing;
   `I`/`O`/`0`/`W`/`H` and buttons unchanged.
3. ESC DOUBLE-PRESS CLOSE (fullscreen OFF): first ESC shows the centered popup
   for ~2 s with "Press ESC again to close"; pressing ESC again while visible
   closes the app; holding ESC never closes; letting it expire then pressing
   ESC shows it again; the popup appears with image open, empty-folder browsed
   AND the fresh empty state; when fullscreen ON, ESC exits fullscreen only
   (no popup); returning to fullscreen OFF restores the feature.
Report results back into the project (this closes the Task-5 manual-UX sub-item).
```

Result handling: recorded by the Planner at wave close; the Task-5 manual-UX sub-item STAYS `[ ]` in the TODO until the user reports (binding TODO structure; §10).

## §12 Divergence note — the wave's Step A.5 merge depends on this harness evidence

Step A.5 of the global plan (TODO-file rename → merge `fix/wave-4-2-operator-feedback` into `main` → push `main` to `origin`) may proceed ONLY after this task's 4.2 green evidence exists (§3 step 2 `ALL CHECKS PASSED` at HEAD + step 6 build exit 0 + step 8 audit set-equal). A STOP anywhere in §3/§7 therefore BLOCKS Step A.5 until the caller resolves it: the merge is never performed on red or missing evidence, and the implementer never performs the merge (Step-5 territory owned by a separate implementer invocation).

## §13 Deliberately out of scope (what 4.2 and this task's mark must NOT do)

- NO source-code changes anywhere (no zoom/alias/esc-close/pill-styling/fullscreen/fit/navigation/listing/platform/display edits) — Task 5 verifies, it does not build; the only sanctioned rewrites are §7's bounded fmt/clippy rounds.
- NO new tests, NO test additions/renames/skips "while here" (§1.1 gap verdict: none; census pinned at 127).
- NO docs updates — Task 6 owns README/tech/architecture/context/brief/future-ideas; NO OBSERVED-DLL doc refresh here (a changed list is REPORTED, never recorded, per §6). NO TODO-file 4.6 edits inside 4.2 (the mark is the 4.6 step, §10).
- NO `CHANGELOG.md` touches (wave standing ban). NO edits to existing `.kilo/plans/*` files, INCLUDING this plan once it lands (new plan files only, ever).
- NO version changes (0.4.2 landed at Step A.3). NO `Cargo.toml`/`Cargo.lock` touches at all.
- NO x86 target; NO Wave-5 pan work; NO "Open with..."; NO parked future-ideas items; NO `.agent/project-info/context.md` staging or editing (the PLANNER copies §11 there).
- NO `git push`, NO branch creation/switch, NO merge (Step 2/Step 5 territory). NO `FORCE_FAIL` invocations. NEVER stage `dist/**`, `logs/**`, `target/**`, `*.log`, or `Cargo.*`.

## §14 Stop conditions → return a user question

- `vm_status` not running / `/img-scroller` share unavailable at any VM-gated step.
- §3 step-1 pre-flight evidence mismatch (branch ≠ `fix/wave-4-2-operator-feedback`, HEAD ≠ `f604051`) or any unexplained porcelain entry.
- P1 LF guard exit 1 (CRLF recurrence — class outside this task).
- Any `--locked` lock-drift error (§2) — STOP; NO reconcile improvisation.
- Any harness `FAIL` line or non-0 exit beyond §7's sanctioned fix rounds; the harness summary ≠ `127 passed; 0 failed; ...` (§1.2); per-file grep census ≠ the §0 table or hand-sum ≠ 127; any `#[ignore]`/`#[should_panic]` hit.
- Any §5 sweep (5a–5j) mismatching its Expected (report actual output verbatim).
- Any `build-windows: error:` typed line (report verbatim; the docs failure table is the diagnostic map).
- A dangling `img-scroller.exe.part` after a successful build (7b).
- DLL audit ≠ the 22-OBSERVED set (report the delta; never record a new baseline).
- Any §13 out-of-scope temptation → STOP; the caller decides.
- Anything ambiguous or outside this plan's assigned scope → return the question to the caller. NEVER make assumptions.
