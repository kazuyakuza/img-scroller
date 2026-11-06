# Wave 4.1 Task 8 — Verification Plan 4.1b (fresh dev-checks, census corroboration, removed-behavior grep sweep, 0.4.1 exe rebuild + byte-size record, DLL audit, Git hygiene, updated Wave-4.1 OPERATOR-RUN checklist)

- Date: 2026-11-06
- Source TODO: [`.agent/todos/20261102/20261102-todo-1.md`](../../.agent/todos/20261102/20261102-todo-1.md) → `### Task 8: Verification` ONLY (exactly its two sub-items) + the wave's `## Future session execution notes`. NO other task's plans are expanded or executed.
- NOT front-end ⇒ sub-step **4.1b ONLY** (no 4.1a exists for Task 8).
- Workflow slot: Wave-4.1 Critical Workflow, Task 8, step 4.1b (this PLAN, created by the architector in 4.1b) → executed by the implementer in 4.2; the Task-8 `[DONE]` mark is the separate 4.6 step (encoded in §10; NOT executed inside 4.2).
- Build docs (binding usage): [`docs/build.md`](../../docs/build.md), [`docs/build-windows.md`](../../docs/build-windows.md); harness scripts: [`scripts/dev-checks.sh`](../../scripts/dev-checks.sh), [`scripts/build-windows.sh`](../../scripts/build-windows.sh).
- Precedent followed (structure, command table, mark semantics): [`.kilo/plans/20261102-wave-4-task4-verification-plan.md`](20261102-wave-4-task4-verification-plan.md) (the Wave-4 Task-4 verification plan — same shape, updated to the Wave-4.1 landed state).
- Approval: user's "Approve Global and Tasks Plans" choice makes every decision below BINDING; zero judgment left for the implementer. If any detail conflicts with something ACTUALLY observed during 4.2, STOP and return the question to the caller (§13).
- Target implementer: JUNIOR under 50% restriction. All structural/scope decisions are encoded here; §12 lists the out-of-scope guard verbatim.

---

## §0 Session facts captured at plan time (verified evidence; branch `fix/wave-4-operator-fixes` @ HEAD `5579cfb`, per caller)

- Branch: `fix/wave-4-operator-fixes`; HEAD `5579cfb`; the chain below the tip is the Tasks-1–7 landing chain (Tasks 1 launch-state, 2 captions/tooltips, 3 F11+ESC tuning, 4 stray-message removal, 5 symmetric zoom range, 6 strict-shift aliases, 7 one-shot fit) plus their docs/spec commit. The implementer ENUMERATES it at pre-flight with `git log --oneline -12` (§3 step 1) and pastes the lines into the 4.2 report — the plan deliberately pre-records NO SHAs beyond `5579cfb` (caller-provided, the only pinned one).
- Version **0.4.1** (verified: `Cargo.toml` `[package] version = "0.4.1"`). The 0.4.1 bump landed at workflow Step 3 with the lockfile; NOTHING touches `Cargo.toml`/`Cargo.lock` in this task (§2).
- Test census verified at plan time by host-side `rg -c "#\[test\]" src` — **18 files, grand total 104**:

| File | `#[test]` count |
| --- | --- |
| `src/app/tests.rs` | 9 |
| `src/app/floating_controls_tests.rs` | 1 |
| `src/app/folder_navigation_tests.rs` | 5 |
| `src/app/folder_navigation_continuation_tests.rs` | 4 |
| `src/app/last_viewed_tests.rs` | 5 |
| `src/app/fullscreen_tests.rs` (Tasks 1+3; incl. the 2 ESC tests) | 9 |
| `src/app/zoom_tests.rs` (Tasks 5+6 additions included) | 16 |
| `src/app/fit_one_shot_tests.rs` (Task 7 app policy) | 12 |
| `src/app/button_hint.rs` (Task 2 inline caption/tooltip pins) | 2 |
| `src/folder_listing/tests.rs` | 5 |
| `src/sibling_folder_listing/tests.rs` | 6 |
| `src/sibling_folder_listing/neighbor_tests.rs` | 2 |
| `src/name_collation.rs` | 2 |
| `src/image_loading.rs` | 4 |
| `src/app_display.rs` | 2 |
| `src/zoom.rs` (pure zoom math; symmetric-bounds pins) | 8 |
| `src/scaling.rs` (pure fit math, inline) | 5 |
| `src/scaling/fit_factor_tests.rs` (Task 7 fit-factor math) | 7 |
| **Total** | **104** |

- No `#[ignore]` / `#[should_panic]` markers anywhere in `src/` (verified by grep at plan time) — nothing skipped, nothing deferred.
- Removed-behavior ground truth (parsed at plan time, feeds §3 sweep expectations):
  - `Zoom min` / `Zoom max`: ZERO occurrences in `src/` (the Wave-4-Task-4 rejected-step removal landed).
  - `message_text = ` writes: EXACTLY 3 code sites — `src/app.rs:84` (`None` on successful open), `src/app.rs:89` (`Some(UNSUPPORTED_FILE_MESSAGE)` on failed open), `src/app/folder_navigation.rs:107` (`None` on entering empty-folder mode). NONE of the three contains "Zoom" or "Fit".
  - `Fit width` / `Fit height` surviving strings are UI LABELS/TOOLTIPS only, not pill-message writes: `src/app/button_hint.rs:83,84` (`"Fit width (W)"` / `"Fit height (H)"` tooltips), `src/app/fit_button.rs:9,10` (`FIT_WIDTH_BUTTON_LABEL`/`FIT_HEIGHT_BUTTON_LABEL` accessible labels), plus their test pins (`button_hint.rs` tests, `fit_one_shot_tests.rs:117,122`).
  - `windows_subsystem`: exactly `src/main.rs:4` — `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` (console hidden in RELEASE builds only; a dev `cargo run` console is expected and NOT a defect).
  - `with_maximized`: exactly `src/app.rs:146` — `.with_maximized(true),` (launch window state).
  - `viewport_rect`: 5 lines total — 3 CODE call sites (`src/app/keyboard_intake.rs:52` and `:55` — the W/H key dispatch; `src/app/fit_button.rs:90` — the button click path) + 2 doc-comment mentions (`src/scaling.rs:70`, `src/app/zoom.rs:20`).
  - `screen_rect`: ZERO occurrences in `src/`.
  - Symmetric-bounds pin: `src/zoom.rs:20` — `ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM` (the ÷8 floor inverse-encoding of the ×8 ceiling).
  - Strict-shift alias pin: `shift_alias_is_pressed` — 4 occurrences, all in `src/app/zoom.rs` (line 139 pure predicate definition; uses at 151, 154, 174).
  - One-shot seam pin: `set_zoom_to_fit` call sites — `src/app/keyboard_intake.rs:52,55` and `src/app/fit_button.rs:92,94` (both fit keys AND both buttons funnel through `set_zoom_to_fit_width`/`set_zoom_to_fit_height` in `src/app/zoom.rs:77,92`); the seams write NO message and no-op while no image is open.
  - ESC pins: `src/app/fullscreen.rs:16` (`FULLSCREEN_EXIT_KEY = Key::Escape`), `:20` (`escape_exits_fullscreen` predicate), wired in `src/app/keyboard_intake.rs:23–25`.
- LAST KNOWN EXE sizes (delta-only references, never expectations): 0.3.0 = **19,502,691 bytes** (Wave-3 rebuild); 0.4.0 = **19,535,645 bytes** (Wave-4 rebuild). The 0.4.1 exe WILL differ; any non-empty size is acceptable — the recorded verbatim as-built value is the deliverable. NO target size number is a valid pre-requirement (wave precedent: exe byte size only RECORDED).
- `OBSERVED_DLL_IMPORT_LIST` constant verified present at [`scripts/build-windows.sh`](../../scripts/build-windows.sh) line 15 (identical in [`docs/build-windows.md`](../../docs/build-windows.md) → "Static-linking / import audit"), 22 DLLs — the audit comparator (§6).
- Harness semantics (docs/build.md + `scripts/dev-checks.sh` read): fixed order `fmt-check` → `check` → `test` → `clippy`; EVERY check runs even if earlier ones fail; per-check line `PASS|FAIL <name> exit=<code> (<n> s)`; final literal `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1); UTC-timestamped log appended under gitignored `logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log`, the exact path printed at run start is authoritative; the script never modifies sources and never applies fmt fixes; `FORCE_FAIL` exists but is NOT used this task.
- build-windows semantics (docs/build-windows.md + script read): runs `cargo build --release --locked --target x86_64-pc-windows-gnu`; success prints `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, then the OBSERVED-DLL evidence line; install is ATOMIC (`.part` copy → rename over the final name); every failure line starts with the literal prefix `build-windows: error:` and exits 1; a failed run leaves the previous artifact byte-for-byte intact.
- `.gitignore` contains `dist/`, `logs/`, `*.log`, `target/` (verified read) — artifacts and check logs are never stageable.
- `.agent/project-info/context.md` is Planner/caller territory (Closing-Step habit): it may sit modified-unstaged at any point; it is NEVER staged or edited by this task.

### §1 Unit-coverage mapping (TODO Task-8 sub-item (a) → landed tests, name by name)

The TODO sub-item (a) demands unit tests where PURE logic exists (symmetric zoom-bound math, one-shot fit-size computation), run via the dev-checks harness in the container. Mapping onto the landed 104:

| TODO topic | Satisfied by | File |
| --- | --- | --- |
| Symmetric zoom-bound math (×1.25 step in/out, clamps at ×8 ceiling AND the divisible-by-step ÷8 floor, `ZOOM_FACTOR_MINIMUM = 1.0/ZOOM_FACTOR_MAXIMUM` pin, symmetric-inverse assertion, at-bound predicates) | the 8 pure-math tests of the `zoom` module (incl. `zoom_bounds_and_restore_constant_are_pinned` asserting `ZOOM_FACTOR_MINIMUM * ZOOM_FACTOR_MAXIMUM == 1.0`) | `src/zoom.rs` |
| Symmetric-range app policy (zoom-out WORKS below fit; silent no-ops at both bounds — no messages ever; restore target stays 1.0 mid-range; reset-on-every-successful-open; button enablement: `[+]` dim only at ceiling, `[−]` dim only at floor, restore dim only at 1.0) | the 16 app-seam tests (Wave-4 Task-2 base + Wave-4.1 Tasks 4/5/6 additions) | `src/app/zoom_tests.rs` |
| One-shot fit-size computation (fit-width/fit-height factors vs the current available size, clamped into `[1/8, 8]` by the ONE shared clamp source, non-positive-size rejection; whole-image `fitted_size` guards) | the 5 inline `scaling` tests + the 7 `fit_factor_tests` | `src/scaling.rs`, `src/scaling/fit_factor_tests.rs` |
| One-shot app policy (W/H keys AND both buttons set the factor ONLY; silent; no-op without an image; I/O work FROM the set level; `0`/successful-open reset to 1.0; W re-set recomputes; no persistent mode exists) | the 12 `fit_one_shot_tests` | `src/app/fit_one_shot_tests.rs` |
| Fullscreen F11 toggle + ESC-while-ON exit (predicate matrix `escape_exits_fullscreen`: only ON+ESC fires) | 9 `fullscreen_tests` (7 Wave-4 Task-1 + 2 Wave-4.1 Task-3 ESC additions) | `src/app/fullscreen_tests.rs` |
| Strict-shift aliases (Shift+'+' / Shift+'-' fire; plain '='/'+'/'-' never) | predicate + zoom_tests pins | `src/app/zoom.rs` (used), `src/app/zoom_tests.rs` |
| Regressions implicitly exercised by the operator run unchanged (arrows, sibling memory, empty-folder mode, collation, open flow, unsupported message) | the remaining 47 tests (9+1+5+4+5+2+5+6+2+2+4+2) — unchanged since Wave 4 except counts above | see §0 table |

### §1.1 GAP VERDICT

**No gaps.** Every pure-logic topic the TODO Task-8 bullet names is pinned by landed tests (symmetric bounds: `src/zoom.rs` 8 + `src/app/zoom_tests.rs` 16; one-shot fit computation: `src/scaling.rs` 5 + `src/scaling/fit_factor_tests.rs` 7 + `src/app/fit_one_shot_tests.rs` 12; F11/ESC support: 9). **NO test additions are sanctioned in this task; the expected census stays EXACTLY 104.** Any "why not one more test" improvisation in 4.2 → STOP (§13).

### §1.2 Census-failure rule

If the harness `cargo test` summary does not read exactly `test result: ok. 104 passed; 0 failed; ...` (a trailing doc-test `test result: ok. 0 passed` line is expected and normal — not a second count): **STOP** and return the full test section to the caller. A census mismatch is a defect, never an ad-hoc-patch opportunity.

## §2 Lockfile status — NO reconcile needed

Version 0.4.1 landed at Step 3 with the committed lock (wave execution note). Therefore NO reconcile command, NO lock commit this task. Conditional STOP: if ANY `--locked` invocation (dev-checks step 2, or the release build) fails with a "the lock file needs to be updated but --locked was passed"-class error — **STOP immediately** and return the error verbatim to the caller. A lock-drift failure here means a committed-state inconsistency the plan's evidence did not show — it is a caller decision, NOT a junior self-fix.

## §3 Command sequence for 4.2 (exact; per docs/build.md and docs/build-windows.md)

Command channel: the `alpine-vm` MCP tools — call `alpine-vm_vm_status` BEFORE EVERY `alpine-vm_vm_run_command` call; ONE MCP call per row; each row's exact `command` string is passed verbatim; each command gets its own timeout from the table; the `exitCode` field is authoritative; captured MCP stderr may be empty — an absent stderr is NOT a failure signal (documented caveat); host-side git commands run via the Bash tool ONE command per invocation (never chained, never `&&`).

| # | Step | Command (exact) | timeoutMs | Expected |
| --- | --- | --- | --- | --- |
| 0 | VM gate | `vm_status` (MCP tool) | — | running + SSH reachable; else STOP + user question (binding wave note) |
| P1 | Pre-flight LF guard ×2 | `sh -c "sed 's/\r$//' /img-scroller/scripts/dev-checks.sh \| cmp - /img-scroller/scripts/dev-checks.sh"`, then the IDENTICAL command for `/img-scroller/scripts/build-windows.sh` | 60000 | exit 0, no output, BOTH times. exit 1 (CRLF recurrence) → STOP + report (class outside Task-8 scope; caller owns the fix shape) |
| 1 | Pre-flight git evidence (host, Bash tool) | `git branch --show-current`, then SEPARATE calls: `git log --oneline -12`, then `git status --porcelain` | — | branch = `fix/wave-4-operator-fixes`; HEAD = `5579cfb`; the Tasks-1–7 chain visible below the tip; porcelain EMPTY or showing ONLY `?? .kilo/plans/20261102-wave-4-1-task8-verification-plan.md` (this plan file, created in 4.1b) and/or `M .agent/project-info/context.md` (planner file, invisible — never staged). Any OTHER entry → STOP + ask the caller. Paste the `git log` lines into the report (the Tasks-1–7 evidence table) |
| C1 | Plan-file commit (Wave-4.1 task-level precedent `02e0870`) | host: `git add .kilo/plans/20261102-wave-4-1-task8-verification-plan.md`; then `git commit -m "docs: add wave4.1 task8 verification plan"` | — | commit succeeds; `git status --porcelain` afterwards shows NO repo file except (possibly) `M .agent/project-info/context.md` |
| 2 | **Fresh dev-checks harness run** (sub-item (a)'s authoritative evidence) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` | 900000 | four `PASS` lines in fixed order + final literal `ALL CHECKS PASSED`, **exit code 0**; harness test block `test result: ok. 104 passed; 0 failed; ...` (+ doc-test `0 passed` line, normal); RECORD the printed log path |
| 3 | Census corroboration | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rc --include='*.rs' '#\[test\]' /img-scroller/src \| sort"` | 60000 | per-file counts COMPLETELY EQUAL to the §0 18-file table; implementer sums the 18 numbers by hand → grand total **104**; corroborates the harness `104 passed` line (the harness number is the binding figure) |
| 4 | Skip/ignore-marker sweep | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn --include='*.rs' -e '#[ignore]' -e '#[should_panic]' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `0`, exit 0 — no skipped/ignored tests |
| 5a | Sweep: removed rejected-step messages gone | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn -e 'Zoom min' -e 'Zoom max' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `0` — zero occurrences in all of `src/` |
| 5b | Sweep: the ONLY message writes left | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn 'message_text = ' /img-scroller/src"` | 60000 | EXACTLY 3 lines: `src/app.rs:84` (`= None;`), `src/app.rs:89` (`= Some(image_loading::UNSUPPORTED_FILE_MESSAGE);`), `src/app/folder_navigation.rs:107` (`= None;`). Assert by eye: none of the 3 contains "Zoom" or "Fit" |
| 5c | Sweep: surviving 'Fit width'/'Fit height' strings are labels/tooltips only | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'Fit width\|Fit height' /img-scroller/src/app/button_hint.rs /img-scroller/src/app/fit_button.rs"` | 60000 | EXACTLY 4 lines: `button_hint.rs:83` `"Fit width (W)"`, `:84` `"Fit height (H)"` (tooltips), `fit_button.rs:9` `"Fit width"`, `:10` `"Fit height"` (accessible labels). NO message-channel write among them |
| 5d | Sweep: windows_subsystem pin | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn 'windows_subsystem' /img-scroller/src"` | 60000 | EXACTLY 1 line: `src/main.rs:4` `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` |
| 5e | Sweep: maximized launch pin | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn 'with_maximized' /img-scroller/src"` | 60000 | EXACTLY 1 line: `src/app.rs:146` `.with_maximized(true),` |
| 5f | Sweep: viewport_rect code call sites | first `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'viewport_rect' /img-scroller/src/app/keyboard_intake.rs"`; then the same for `/img-scroller/src/app/fit_button.rs`; then `sh -c "grep -c 'viewport_rect' /img-scroller/src/app/keyboard_intake.rs /img-scroller/src/app/fit_button.rs \| wc -l"` (three MCP calls, 60000 ms each) | 60000 each | keyboard_intake.rs grep shows EXACTLY lines **52 and 55**; fit_button.rs grep shows EXACTLY line **90** — the 3 code call sites; the pair `wc -l` prints exactly `3` |
| 5g | Sweep: viewport_rect total occurrences | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn 'viewport_rect' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `5` (the 3 call sites + the 2 doc-comment mentions) |
| 5h | Sweep: screen_rect = 0 | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn 'screen_rect' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `0` |
| 5i | Sweep: symmetric-floor encoding pin | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM' /img-scroller/src/zoom.rs"` | 60000 | EXACTLY 1 line: `src/zoom.rs:20` |
| 5j | Sweep: strict-shift predicate pin | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -c 'shift_alias_is_pressed' /img-scroller/src/app/zoom.rs"` | 60000 | stdout exactly `4` (definition line 139 + uses 151, 154, 174) |
| 5k | Sweep: one-shot seam funnel pin | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'set_zoom_to_fit' /img-scroller/src/app/keyboard_intake.rs /img-scroller/src/app/fit_button.rs"` | 60000 | EXACTLY 4 lines: `keyboard_intake.rs:52`, `:55` (W/H key dispatch) and `fit_button.rs:92`, `:94` (button clicks) — both entry kinds funnel through the same two one-shot seams |
| 5l | Sweep: ESC-exit pins | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'FULLSCREEN_EXIT_KEY\|escape_exits_fullscreen' /img-scroller/src/app/fullscreen.rs /img-scroller/src/app/keyboard_intake.rs"` | 60000 | EXACTLY 4 lines: `fullscreen.rs:16`, `fullscreen.rs:20`, `keyboard_intake.rs:23`, `keyboard_intake.rs:25` |
| 6 | Release cross-build (0.4.1 exe) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` | 900000 | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; the OBSERVED-DLL evidence line; **exit code 0** |
| 7 | Exe inspection (record the byte size) | `sh -c "ls -l /img-scroller/dist/windows-x64/img-scroller.exe"` | 60000 | non-empty size printed; RECORD the exact as-built byte count verbatim (any non-empty size passes; no pre-requirement) |
| 7b | No dangling `.part` | `sh -c "ls /img-scroller/dist/windows-x64"` | 60000 | the listing shows EXACTLY one entry: `img-scroller.exe` — no `img-scroller.exe.part` (atomic install completed; the part file never survives success) |
| 8 | DLL import audit (per docs/build-windows.md) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe \| grep -i 'DLL Name' \| tr 'A-Z' 'a-z' \| sed 's/.*dll name: //' \| sort -u"` | 180000 | exactly 22 lowercase DLL lines, SET-EQUAL to the recorded constant (§6) |
| 8b | Audit fallback (ONLY if step 8 yields no output — report-format drift) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` | 180000 | raw dump; compare manually against §6; large output, still ONE call |
| 9 | Gitignore compliance sweep (host) | re-read `.gitignore`, then `git status --short` | — | NO `dist/`, `logs/`, `target/`, `*.log` entries anywhere in the output; nothing staged; the only allowed residual is `M .agent/project-info/context.md` (planner file, invisible) |

## §4 Expected outcomes table (authoritative)

| Evidence | Expected |
| --- | --- |
| Harness log header | `dev-checks starting; log file: logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` — RECORD the exact printed path |
| Harness check lines (fixed order) | `PASS fmt-check exit=0 (<n> s)`; `PASS check exit=0 (<n> s)`; `PASS test exit=0 (<n> s)`; `PASS clippy exit=0 (<n> s)` (clippy runs `-D warnings`, so exit 0 means zero warnings) |
| Harness test block | `test result: ok. 104 passed; 0 failed; ...` + doc-test `test result: ok. 0 passed` (normal, not a second count) |
| Harness final | literal `ALL CHECKS PASSED`; **exit code 0 = the authoritative success evidence** (sub-item (a)'s gate) |
| Census corroboration | per-file grep counts == §0 table; hand-summed total 104; zero `#[ignore]`/`#[should_panic]` |
| §5 sweeps & pins | 5a `0`; 5b exactly 3 sites (none Zoom/Fit); 5c exactly 4 label/tooltip lines; 5d `main.rs:4` only; 5e `app.rs:146` only; 5f/5g the 3 call sites + total 5; 5h `0`; 5i `zoom.rs:20`; 5j `4`; 5k the 4 funnel lines; 5l the 4 ESC lines |
| Release build | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; OBSERVED-DLL evidence line; **exit code 0** |
| Exe | NON-EMPTY gitignored `dist/windows-x64/img-scroller.exe` (0.4.1 wave build); exact as-built size RECORDED (delta vs the 19,502,691 B / 19,535,645 B prior magnitudes is evidence only — any non-empty size passes); NO dangling `.part` (7b: the dist dir lists exactly `img-scroller.exe`) |
| DLL audit | 22 lowercase DLL names, SET-EQUAL to the recorded 2026-10-31 constant (§6) — DLL import list UNCHANGED |
| Git sweep | nothing ignored staged; nothing staged at all after C1 except the 4.6 §10 mark commit later |

## §5 Alpine-VM MCP usage notes (binding, per docs/build.md)

- `vm_status` BEFORE every `vm_run_command` call; only proceed when the VM is running and SSH-reachable; otherwise STOP and return a user question (docs/build.md precondition; wave execution note).
- ONE command per MCP call; pipelines live INSIDE one quoted `sh -c "..."` string exactly as printed in §3.
- The `exitCode` field is the authoritative pass/fail evidence; `null` exitCode + `timedOut: true` → report the timeout with the partial captured output; do NOT re-run silently.
- Absent cargo stderr in captured MCP output is NOT a failure signal (documented caveat).
- Generous timeouts (§3 table) beat aggressive ones.
- EBUSY-on-share-write hazard (vboxsf): retry the IDENTICAL command ONCE; if it fails again → STOP + report with evidence (nothing here legitimately writes to the share).

## §6 DLL import audit — recorded constant and comparison evidence statement

Recorded `OBSERVED_DLL_IMPORT_LIST` (source of truth: `scripts/build-windows.sh` line 15, identical in `docs/build-windows.md`; audited 2026-10-31 with `x86_64-w64-mingw32-objdump`; docs-split rule: NOT duplicated into tech.md/architecture.md):

```
advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll,
api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll,
gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll,
opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll,
ws2_32.dll
```

Evidence statement to record in the 4.2 report (verbatim shape): "Wave-4.1 audit: `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` inside the container; output deduplicated + lowercase-folded via the §3 step-8 pipeline; OBSERVED = 22 DLLs; SET-EQUAL to the recorded 2026-10-31 constant — DLL import list UNCHANGED."

On ANY diff (extra, missing, renamed — dependency drift): **STOP + report** the full observed list and the delta. NO edits to `scripts/build-windows.sh` or `docs/build-windows.md` — refreshing the OBSERVED baseline is a Planner decision, never an implementer improvisation (Wave-1/2/3/4 precedent).

## §7 Failure/intervention policy for 4.2 (decision branches; max iterations PINNED)

- **fmt-check FAIL** → sanctioned minimal self-service fix: ONE `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` (timeoutMs 120000); host-side `git diff` MUST show formatting-only reflow (no semantic content moves); if changed files exist → stage EXACTLY the `src/` paths listed by `git status --short` (never `.agent/`, never `docs/`) and commit EXACTLY `style: apply cargo fmt` (separate commit; Wave-2 precedent); then re-run the §3 step-2 harness ONCE. Max 2 fmt-fix → 2 harness re-runs. Second consecutive fmt failure → STOP + report.
- **check FAIL** → STOP immediately. Compile defects are structural; report the full captured output + the printed log path (no self-fix — run-once-then-report).
- **test FAIL** → STOP immediately. Any failing named test = structural; §1.2's census rule protects against ad-hoc test invention; report with the failure detail lines. Census ≠ `104 passed` by ANY means → §1.2 STOP.
- **clippy FAIL** → minor-lint self-service sanctioned ONLY when the failure is a SINGLE lint in a SINGLE file whose fix is ≤ 3 source lines. Fix with the `edit` tool, commit EXACTLY `fix: resolve minor clippy lint (verification round)` staged to that file only, then re-run the harness ONCE. Max 1 such fix round; the second consecutive clippy failure OR any multi-site/structural lint → STOP + report.
- Global cap: at most 2 post-fix harness re-runs in total across all sanctioned fix rounds; anything failing after the cap → STOP + report with evidence (four result lines + log path).
- Any §3 grep sweep mismatching its Expected (5a–5l counts/lines off) → STOP + report the actual output verbatim. A drift means committed-state deviation from the plan's verified ground truth — caller decision, never a self-fix.
- Any `build-windows: error:` line of ANY type → STOP + report the typed error verbatim (the docs/build-windows.md failure table is the diagnostic map); never retry beyond the single EBUSY-once rule of §5.
- Every fix round re-reads `.gitignore` before staging (gitignore-compliance rule) and re-runs `git status --short`; `.agent/project-info/context.md` is planner/caller territory — NEVER staged.

## §8 Git plan — which commits this task creates

| When | Commit | Content | Message (EXACT) |
| --- | --- | --- | --- |
| 4.2 start (after §3 step-1 pre-flight) unconditional | C1 | this plan file only | `docs: add wave4.1 task8 verification plan` |
| Only if fmt drift | C2 | fmt-reflowed `src/` files only | `style: apply cargo fmt` |
| Only if single minor clippy lint | C3 | the one ≤3-line fixed file | `fix: resolve minor clippy lint (verification round)` |
| **4.6 (the mark step — separate workflow sub-step, §10)** | C4 | the TODO file only (§10 edits) | `task: mark wave-4.1 task8 verification done` |

- NO commit for build artifacts: `dist/` is gitignored; `logs/checks/` logs are gitignored; NOTHING built is ever staged (§3 step-9 sweep; unstage anything ignore-matching if it appears).
- NO `git push` (restricted to workflow Step 5: `main` → `origin` ONLY). NO amend/force. NO `git add -A`/`git add .` (explicit paths only). NO branch creation/switch (Step-2 territory — the branch already exists). NO merge (Step-5 territory).
- Commit staging lists are checked with `git status --short` immediately before each commit; the staged list MUST equal the row's content.

## §9 Junior-implementer step-by-step checklist for 4.2

1. Host: `git branch --show-current` → MUST be `fix/wave-4-operator-fixes`. Separate calls: `git log --oneline -12` → HEAD `5579cfb`, the Tasks-1–7 chain below the tip (paste into the report); `git status --porcelain` → empty or ONLY the two permitted entries (§3 step 1). Any mismatch → STOP + ask the caller.
2. Host: read `.gitignore` (sweep reference). Read §4 so every comparison is pre-armed.
3. `vm_status` → running + SSH reachable; else STOP + user question.
4. §3 P1 LF guard (both scripts) → exit 0 twice; exit 1 → STOP + report.
5. §3 C1: stage the plan file, commit `docs: add wave4.1 task8 verification plan`; verify the staged list with `git status --short` before committing.
6. `vm_status`; §3 step 2 harness run at 900000 ms. Compare against §4 line-by-line as output streams; RECORD the printed log path.
   - All four `PASS` lines + literal `ALL CHECKS PASSED` + exit 0 + `test result: ok. 104 passed; 0 failed` → go to 7.
   - `FAIL fmt-check` → §7 fmt branch (2 max), then ONE re-run → back to 6.
   - `FAIL check` / `FAIL test` → STOP + report (§7).
   - `FAIL clippy` → §7 clippy branch (1 max) → ONE re-run → back to 6.
   - Post-cap failures → STOP + report with the four result lines + log path.
7. `vm_status`; §3 steps 3–4 census + ignore sweep → the §0 table / hand-sum 104 / `0`; mismatch → STOP (§1.2).
8. `vm_status`; §3 steps 5a–5l evidence sweeps → EVERY row exact per §4; any mismatch → STOP + report actual output verbatim (§7).
9. `vm_status`; §3 step 6 release build → expect §4 build rows; any typed error → STOP + report verbatim.
10. `vm_status`; §3 steps 7 + 7b → RECORD the exact as-built byte size verbatim; dist dir must contain exactly `img-scroller.exe` (no `.part`).
11. `vm_status`; §3 step 8 DLL audit → 22 lowercase lines set-equal to §6; any diff → STOP + report (no script/doc edits); no output → §3 step 8b fallback then compare; record the §6 verbatim evidence statement.
12. Host: re-read `.gitignore`; `git status --short` sweep → NOTHING staged, no ignore-matching paths anywhere beyond the permitted planner residual; no unconditional commit remains pending (C1 already made; C2/C3 only if their branches fired).
13. Report to caller (the full evidence set): branch/HEAD + the Tasks-1–7 `git log` lines; harness log path; the four PASS lines + `ALL CHECKS PASSED` + exit 0 + the `104 passed; 0 failed` line; the per-file census split + hand sum + zero-ignore result; every §5 sweep result (5a–5l) with the expected counts reached; `build-windows: OK`/artifact/exit 0; the as-built exe byte size (verbatim, with the 0.3.0/0.4.0 magnitudes noted as reference); the §6 verbatim audit evidence statement; git-sweep result; **and the §11 OPERATOR-RUN checklist block copied VERBATIM for handoff to the caller** — plus the explicit note that the Task-8 OPERATOR-RUN sub-item remains PENDING until the user reports on a real Windows machine.

## §10 4.6 mark semantics (the Task-8 mark step; NOT executed inside 4.2 — encodes the workflow's 4.6 + the caller's instruction)

When the Critical Workflow reaches 4.6 for Task 8 (AFTER §9 is fully green):

1. Read the TODO file [`.agent/todos/20261102/20261102-todo-1.md`](../../agent/todos/20261102/20261102-todo-1.md) FIRST; PRESERVE all other content byte-for-byte (overwrite-prevention rule). EXACTLY two edits inside `### Task 8: Verification`:
   - Heading: `### Task 8: Verification` → `### Task 8: Verification [DONE]`.
   - Sub-item (a) (the unit-tests/harness line): `- [ ]` → `- [x]` — box change ONLY; NO wording change (the §4 harness evidence makes this check legal).
   - Sub-item (b) (the OPERATOR-RUN / manual-UX line): **STAYS `- [ ]`** — pending the user's report on the real Windows machine (Wave-3/4 precedent: heading marked, agent-verifiable sub-item checked, manual-UX sub-item left unchecked until the operator reports). Do NOT annotate it with a status word; just leave the box.
   - No OTHER task heading, sub-item, section, or note is touched; the Future-session-execution-notes section is NOT edited.
2. Stage EXACTLY `.agent/todos/20261102/20261102-todo-1.md`; verify with `git status --short` that the staged list is only this file; commit `task: mark wave-4.1 task8 verification done`.
3. NO `git push`, NO `-DONE` file rename (workflow Step 5 territory), `.agent/project-info/context.md` untouched.

## §11 Wave-4.1 OPERATOR-RUN checklist (verbatim block; the 4.2 report hands it to the CALLER for the user; the Planner copies this block VERBATIM into `.agent/project-info/context.md` → "Immediate Next Steps" so the upcoming docs Task 9 and the operator pickup use the same text; the Task-8 OPERATOR-RUN sub-item stays `[ ]` until the user reports)

```
Wave-4.1 manual check (OPERATOR-RUN) — run dist\windows-x64\img-scroller.exe (the freshly rebuilt 0.4.1 exe)
on a real Windows 10/11 machine. Prepare: one folder holding differently shaped images — a WIDE/panoramic
image, a TALL portrait image, and a roughly square image — plus sibling folders beside it (some with images,
one WITHOUT images) so horizontal and vertical navigation both work. (Task-4 re-test items 3.0-below supersede
the Wave-4 checklist: launch state, console hiding, captions/tooltips, F11/ESC tuning, symmetric zoom range,
strict aliases, and one-shot fit are the NEW expectations; the silent-behavior lines replace the old
`Zoom max`/`Zoom min`/`Fit width`/`Fit height` messages.)

1. LAUNCH STATE: launch the exe — the window opens MAXIMIZED at every launch (never auto-fullscreened;
   fullscreen default is OFF). Open an image first when the app launched without one: use the big centered
   Open… button of the fresh empty state.
2. CONSOLE HIDDEN: while the release exe runs, NO console/cmd window is visible behind or beside the app.
3. OPEN & PILL: open a normal photo — whole-image fit, centered; the bottom pill shows the row
   [←][→][↑][↓][fullscreen square][+][−][restore-fit square][fit-width glyph][fit-height glyph].
4. CAPTIONS + TOOLTIPS: under the SIX keyboard-distinct buttons the small 9 pt hotkey captions render —
   "F11" (fullscreen), "I" (+), "O" (−), "0" (restore), "W" (fit-width), "H" (fit-height); the four arrow
   buttons carry NO caption. Hover EVERY one of the 10 buttons — each shows its tooltip
   ("Previous image (Left Arrow)", "Next image (Right Arrow)", "Previous sibling folder (Up Arrow)",
   "Next sibling folder (Down Arrow)", "Fullscreen (F11)", "Zoom in (I)", "Zoom out (O)", "Restore fit (0)",
   "Fit width (W)", "Fit height (H)"); tooltips also appear over DISABLED buttons.
5. FULLSCREEN ON: press F11 — immediate, no lag class to watch: title bar/borders AND the pill (every
   control) disappear; ONLY the image shows.
6. FULLSCREEN OFF: press F11 again — the standard chrome returns exactly (pill back; image central again).
7. ESC EXIT: while fullscreen ON, press ESC — fullscreen exits immediately (same effect as F11). While
   fullscreen OFF, ESC does nothing visible.
8. ZOOM IN: press I (or click [+], or Shift+'+') — the image grows ×1.25 per step; the CENTER of the zoomed
   image stays visible (no pan yet — Wave 5); NO message appears.
9. ZOOM CEILING: zoom in until two more I steps change nothing — the ceiling is ×8 and further steps are
   SILENT (no `Zoom max` message, ever); the [+] button paints dimmed at the ceiling.
10. ZOOM OUT BELOW FIT: press O (or click [−], or Shift+'-') — the image shrinks ×1.25 per step, PASSES the
    original whole-image fit level and keeps shrinking to the symmetric floor ≈ ÷8 of fit; steps past the
    floor change nothing SILENTLY (no `Zoom min` message, ever); the [−] button paints dimmed at the floor;
    zoom-out is NOT blocked AT the fit level itself.
11. ALIASES STRICT: Shift+'+' zooms IN; Shift+'-' zooms OUT; pressing plain '=' , '+' or '-' WITHOUT Shift
    does NOTHING (all three inert).
12. RESTORE: press 0 (or click the restore-fit square) — always returns exactly to the whole-image fit
    level ×1.0 silently, from any factor including the boundaries above.
13. ONE-SHOT W/H: with the TALL portrait open, press W (or the fit-width button) — the image WIDTH spans the
    window (clipped top/bottom); afterward I/O zoom IN AND OUT normally FROM that level (nothing blocked);
    pressing W AGAIN on the same window recomputes the same span (no surprise jump); RESIZE the window —
    the image scales with the window per its stored factor and does NOT re-snap to full-image fit. The H
    analog with the WIDE panoramic: HEIGHT spans the window; zoom works from it the same way.
14. FIT BUTTONS WITHOUT IMAGE: arrow up/down into a sibling folder WITHOUT images (the `Empty folder`
    surface): the two fit buttons are DIMMED and non-clicking there (they stay hover-tooltip-able); the pill
    remains visible for navigation.
15. FULLSCREEN INTERPLAY + RESET & REGRESSION: zoom to about ×2, press F11 (ON) — the zoom level persists
    and I/O/0/W/H keep working while ON; F11 (OFF) — the SAME factor persists (fullscreen toggles never
    touch zoom). Then arrow-navigate to the NEXT image (or ↑/↓ to a sibling) — the new image opens at
    whole-image fit ×1.0 silently (every successful open resets the factor). ←/→ keys and pill buttons still
    walk the folder's images as in 0.4.0; ↑/↓ still walk sibling folders (last-viewed memory and first-image
    rules unchanged); list edges stay silent and their buttons disable. Through ALL zoom/fit actions of this
    run the pill's message line stays EMPTY — the ONLY message that may ever appear is the unsupported-file
    failure text on a failed open (unchanged by design).
Report results back into the project (this closes the Task-8 manual-UX sub-item).
```

Result handling: recorded by the Planner at wave close; the Task-8 OPERATOR-RUN sub-item STAYS `[ ]` in the TODO until the user reports (binding TODO structure; §10).

## §12 Deliberately out of scope (what 4.2 must NOT do)

- NO source-code changes anywhere (no zoom/fullscreen/fit/navigation/listing/platform/display edits) — Task 8 verifies, it does not build; the only sanctioned rewrites are §7's bounded fmt/clippy rounds.
- NO new tests, NO test additions/renames/skips "while here" (§1.1 gap verdict: none; census pinned at 104).
- NO docs updates — Task 9 owns README/tech/architecture/context/brief/product (and the OPERATOR-RUN pickup); NO OBSERVED-DLL doc refresh here (a changed list is REPORTED, never recorded, per §6). NO TODO-file 4.6 edits inside 4.2 (the mark is the 4.6 step, §10).
- NO `CHANGELOG.md` touches (wave standing ban; TODO execution note). NO edits to existing `.kilo/plans/*` files, INCLUDING this plan once it lands (new plan files only, ever).
- NO version changes (0.4.1 landed at Step 3). NO `Cargo.toml`/`Cargo.lock` touches at all.
- NO UI edits; NO x86; NO Wave-5 pan; NO "Open with..."; NO parked future-ideas items (drag&drop, pill hide on idle, etc.); NO `.agent/project-info/context.md` staging or editing (Planner copies §11 there).
- NO `git push`, NO branch creation/switch, NO merge (Step 2/§10/Step 5 territory). NO `FORCE_FAIL` invocations. NEVER stage `dist/**`, `logs/**`, `target/**`, `*.log`, or `Cargo.*`.

## §13 Stop conditions → return a user question

- `vm_status` not running / `/img-scroller` share unavailable at any VM-gated step.
- §3 step-1 pre-flight evidence mismatch (branch ≠ `fix/wave-4-operator-fixes`, HEAD ≠ `5579cfb`) or any unexplained porcelain entry.
- P1 LF guard exit 1 (CRLF recurrence — class outside this task).
- Any `--locked` lock-drift error (§2) — STOP; NO reconcile improvisation.
- Any harness `FAIL` line or non-0 exit beyond §7's sanctioned fix rounds; the harness summary ≠ `104 passed; 0 failed; ...` (§1.2); per-file grep census ≠ the §0 table or hand-sum ≠ 104; any `#[ignore]`/`#[should_panic]` hit.
- Any §5 sweep (5a–5l) mismatching its Expected (report actual output verbatim).
- Any `build-windows: error:` typed line (report verbatim; the docs failure table is the diagnostic map).
- A dangling `img-scroller.exe.part` after a successful build (7b).
- DLL audit ≠ the 22-OBSERVED set (report the delta; never record a new baseline).
- Any §12 out-of-scope temptation → STOP; the caller decides.
