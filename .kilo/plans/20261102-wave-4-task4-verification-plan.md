# Wave 4 Task 4 — Verification Plan 4.1b (fresh dev-checks, test-census corroboration, exe rebuild, DLL audit, Git hygiene, bundled Wave-4 OPERATOR-RUN checklist)

- Date: 2026-11-02
- Source TODO: [`.agent/todos/20261029/20261029-todo-6.md`](../../.agent/todos/20261029/20261029-todo-6.md) → `### Task 4: Verification` ONLY (both sub-items) + the wave's `## Future session execution notes` constraints. NO other task's plans are expanded.
- NOT front-end ⇒ 4.1b ONLY (no 4.1a exists for Task 4).
- Workflow slot: Wave-4 Critical Workflow, Task 4, sub-step 4.1b (this PLAN) → executed by the implementer in 4.2; the Task-4 `[DONE]` mark is the separate 4.6 step (encoded in §10; NOT executed inside 4.2).
- Global plan: [`.kilo/plans/20261102-wave-4-fullscreen-zoom.md`](20261102-wave-4-fullscreen-zoom.md) → "Task 4 — Verification (NOT front-end)" row.
- Build docs (binding usage): [`docs/build.md`](../../docs/build.md), [`docs/build-windows.md`](../../docs/build-windows.md); harness scripts: [`scripts/dev-checks.sh`](../../scripts/dev-checks.sh), [`scripts/build-windows.sh`](../../scripts/build-windows.sh).
- Precedent plans followed (structure, admin, mark semantics): [`.kilo/plans/20261101-wave3-task4-verification.md`](20261101-wave3-task4-verification.md), [`.kilo/plans/20261031-wave2-task4-verification.md`](20261031-wave2-task4-verification.md).
- Landed-behavior sources for §11's checklist facts: Task-1 plan §0 (F11/fifth-pill/chrome hiding), Task-2 plan §0 (I/O/0 hotkeys, ×1.25 step, [1.0, 8.0], rejected-step-only messages, reset-on-open, button enablement), Task-3 plan §0 (W/H, Option A interplay, `image_is_open` gate, silent FitVisible).
- Approval: per-task plan auto-approved by the user's "_approve Global and Tasks Plans_" choice — every decision below is BINDING; zero judgment left for the implementer.
- Target implementer: JUNIOR under 50% restriction; all structural/scope decisions are encoded here. If a detail conflicts with something observed, STOP and return the question to the caller.

---

## §0 Session facts captured at plan time (verified evidence, branch `feat/wave-4-fullscreen-zoom` @ HEAD `f42e950`)

- `git log --oneline -8` at plan time: HEAD `f42e950` (`task: mark wave-4 task3 fit modes done`) atop the Task-3 chain (`bcc82bd`, `d855223`, `a0fad05`, `4af4c65`, `84aaaec`, `f6facbb`, `f436852`); `git status --porcelain` → CLEAN.
- Version **0.4.0** committed at `4985c6f` — that commit changed BOTH `Cargo.toml` AND `Cargo.lock` (verified: `git show 4985c6f -- Cargo.lock` shows exactly the `img-scroller` version line `0.3.0` → `0.4.0`). **The lockfile is IN SYNC with the manifest — unlike the Wave-3 track, NO lock-reconcile step exists in this task** (§2 records the STOP rule should a `--locked` run still report lock drift).
- Test census verified at HEAD by `rg -c "#\[test\]" src -g "*.rs"` — 17 files, TOTAL **93**, split **76 prior + 17 Task-3 additions**:

| File | `#[test]` count |
| --- | --- |
| `src/app/tests.rs` | 9 |
| `src/app/floating_controls_tests.rs` | 1 |
| `src/app/folder_navigation_tests.rs` | 5 |
| `src/app/folder_navigation_continuation_tests.rs` | 4 |
| `src/app/last_viewed_tests.rs` | 5 |
| `src/app/fullscreen_tests.rs` (Task 1) | 7 |
| `src/app/zoom_tests.rs` (Task 2) | 11 |
| `src/folder_listing/tests.rs` | 5 |
| `src/sibling_folder_listing/tests.rs` | 6 |
| `src/sibling_folder_listing/neighbor_tests.rs` | 2 |
| `src/name_collation.rs` | 2 |
| `src/image_loading.rs` | 4 |
| `src/app_display.rs` | 2 |
| `src/zoom.rs` (Task 2, pure step math) | 8 |
| `src/scaling.rs` (pure fit math, inline) | 5 |
| `src/scaling/fit_mode_tests.rs` (Task 3, math) | 6 |
| `src/app/fit_mode_tests.rs` (Task 3, app policy) | 11 |
| **Total** | **93** |

- No `#[ignore]` / `#[should_panic]` markers anywhere in `src/` (verified by read-side grep at HEAD) — nothing skipped, nothing deferred.
- LAST KNOWN EXE (delta-only reference, never an expectation): on-disk `dist/windows-x64/img-scroller.exe` = **19,502,691 bytes** (0.3.0 Wave-3 rebuild, 2026-11-01). The 0.4.0 exe WILL differ in size (fullscreen + zoom + fit-mode code added); ANY non-empty size is acceptable — the recorded verbatim as-built value is the deliverable. NO target size number is a valid pre-requirement.
- `OBSERVED_DLL_IMPORT_LIST` constant verified present at [`scripts/build-windows.sh`](../../scripts/build-windows.sh) line 15 (identical in [`docs/build-windows.md`](../../docs/build-windows.md) → "Static-linking / import audit"), 22 DLLs — the audit comparator (§6).
- Harness semantics (docs/build.md + script read): fixed order `fmt-check` → `check` → `test` → `clippy`; EVERY check runs even if earlier ones fail; per-check line `PASS|FAIL <name> exit=<code> (<n> s)`; final literal `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1); UTC-timestamped log appended under gitignored `logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log`, the exact path printed at run start is authoritative; the script never modifies sources and never applies fmt fixes; `FORCE_FAIL` exists but is NOT used this task.
- build-windows semantics (docs/build-windows.md + script read): runs `cargo build --release --locked --target x86_64-pc-windows-gnu`; success prints `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, then the OBSERVED-DLL evidence line; install is ATOMIC (`.part` copy → rename over the final name); every failure line starts with the literal prefix `build-windows: error:` and exits 1; **a failed run leaves the previous artifact byte-for-byte intact** (the part file is removed on failure; nothing pre-existing is ever deleted).
- `.gitignore` contains `dist/`, `logs/`, `*.log`, `target/` (verified read) — artifacts and check logs are never stageable. `.gitattributes` contains `*.sh text eol=lf` (verified read).
- Gitignore-compliance rule applies: `.gitignore` is re-read and `git status` re-run before ANY staging.
- Stale-doc note (NOT a blocker; not fixed here): `.agent/project-info/context.md` currently describes the pre-Task-3 state (HEAD `dbdf0c7`). Project-info updates are Planner/Docs flow (Task 5 / workflow docs step) — `context.md` is NEVER staged by this task (Wave-3 precedent: planner file, may sit modified-unstaged; treat as invisible).

### §1 Unit-coverage mapping (TODO Task-4 sub-item (a) → landed tests, name by name)

The TODO sub-item (a) demands unit tests where PURE logic exists: (a1) zoom-step math, (a2) fit-scale computation — run via the dev-checks harness in the container. Mapping onto the landed 93:

| TODO topic | Satisfied by | File |
| --- | --- | --- |
| (a1) zoom-step math (×1.25 multiplicative step, [1.0, 8.0] clamp, composition `zoomed_draw_size`) | the 8 pure-math tests of the `zoom` module — step/clamp/composition vectors plus the constant pins (`RESTORE_TO_FIT_FACTOR == 1.0` assertion among them); zero egui painting, container-runnable | `src/zoom.rs` |
| (a1) app-level zoom policy (step counting, at-bound button enablement, rejected-step `Zoom max`/`Zoom min` messages, reset-on-every-successful-open, factor persistence across fullscreen toggles) | the 11 app-seam tests (Task 2 landing: `zoom_in_once`/`zoom_out_once`/`restore_zoom_to_fit` polices and open-reset) | `src/app/zoom_tests.rs` |
| (a2) fit-scale computation (whole-image fit guards, width-only and height-only scale math, overflow direction, non-positive-dimension rejection, `FitMode::base_size` dispatch, `FitVisible ⇔ fitted_size` agreement) | the 5 inline `scaling` tests (Wave-1 `fitted_size` math + zero/negative guards, unchanged since 0.2.0) + the 6 Task-3 fit-mode math tests (width/height span + vertical/horizontal overflow, square-image agreement across all three modes, wide-image inter-mode difference, `FitVisible == fitted_size`, non-positive rejection for every mode) | `src/scaling.rs`, `src/scaling/fit_mode_tests.rs` |
| (a2) app-level fit policy (reset-on-open returns mode AND factor, activation messages `Fit width`/`Fit height`, silent FitVisible, zoom-keeps-mode, activation-overwrites-blocked-zoom message, W/H key pins, button-label pins, `image_is_open` enablement gate) | the 11 Task-3 app-policy tests | `src/app/fit_mode_tests.rs` |
| Fullscreen policy pins (statically anchored app-side behavior the operator run exercises: F11 pin, chrome gate OFF/ON, ON keeps the image, OFF restores) | the 7 Task-1 tests | `src/app/fullscreen_tests.rs` |

### §1.1 GAP VERDICT

**No gaps.** Every pure-logic topic the TODO bullet names is pinned by landed tests, file-by-file above (zoom-step math: `src/zoom.rs` 8 + `src/app/zoom_tests.rs` 11; fit-scale computation: `src/scaling.rs` 5 + `src/scaling/fit_mode_tests.rs` 6 + `src/app/fit_mode_tests.rs` 11; fullscreen pin-support: 7). **NO test additions are sanctioned in this task; the expected census stays EXACTLY 93.** Any "why not one more test" improvisation in 4.2 → STOP (§13).

### §1.2 Census-failure rule

If the harness `cargo test` summary does not read exactly `test result: ok. 93 passed; 0 failed; ...` (a trailing doc-test `test result: ok. 0 passed` line is expected and normal — not a second count): **STOP** and return the full test section to the caller. A census mismatch is a defect, never an ad-hoc-patch opportunity.

## §2 Lockfile status — NO reconcile needed (deviation from the Wave-3 track, encoded)

`Cargo.lock` and `Cargo.toml` agree on **0.4.0** (verified via `git show 4985c6f` — the bump commit carries the lock's version line). Therefore:

- NO §2-style reconcile command, NO lock commit this task. NOTHING touches `Cargo.toml`/`Cargo.lock` in 4.2.
- Conditional STOP: if ANY `--locked` invocation (dev-checks step 2, or the release build) fails with a "the lock file needs to be updated but --locked was passed"-class error — **STOP immediately** and return the error verbatim to the caller. A lock-drift failure here means a committed-state inconsistency the plan's verified evidence did not show — it is a caller decision, NOT a junior self-fix.

## §3 Container command sequence for 4.2 (exact, per docs/build.md / docs/build-windows.md)

All VM/container commands go through the `alpine-vm` MCP: call `vm_status` BEFORE EVERY `vm_run_command`; ONE command per call; pipelines live INSIDE one quoted `sh -c "<command string>"` (never host-side `&&`/`;` chaining); each command gets its own `timeoutMs` per the table; the `exitCode` field is authoritative; captured MCP stderr may be empty — **an absent stderr is NOT a failure signal** (documented caveat; judge only printed lines, the census, and the exit code); a timeout (`timedOut: true`, `exitCode: null`) is REPORTED, never silently retried.

| # | Step | Command (exact) | timeoutMs | Expected |
| --- | --- | --- | --- | --- |
| 0 | VM gate | `vm_status` (MCP tool) | — | running + SSH reachable; else STOP + user question (binding wave note) |
| P1 | Pre-flight LF guard (cheap; `.gitattributes` makes this exit 0) | `sh -c "sed 's/\r$//' /img-scroller/scripts/dev-checks.sh \| cmp - /img-scroller/scripts/dev-checks.sh"` then the identical command with `scripts/build-windows.sh` | 60000 | exit 0, no output (LF-only) both times. exit 1 (CRLF recurrence) → STOP + report (class outside Task-4 scope; caller owns the fix shape) |
| 1 | Pre-flight git evidence | host: `git branch --show-current`; `git log --oneline -10` | — | branch = `feat/wave-4-fullscreen-zoom`; HEAD `f42e950`; the Task-3 chain below the tip; `git status --porcelain` → empty. Any mismatch → STOP + ask the caller |
| 2 | **Fresh dev-checks harness run** (sub-item (a)'s authoritative evidence) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` | 900000 | §4 harness rows — final literal `ALL CHECKS PASSED`, **exit code 0** |
| 3 | Test-census corroboration (grep-based) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rc --include='*.rs' '#\[test\]' /img-scroller/src \| sort"` | 60000 | per-file `count` lines matching §0's 17-file table; grand total = **93** (76 prior + 17 Task-3: `scaling/fit_mode_tests.rs` 6 + `app/fit_mode_tests.rs` 11). Corroborated against harness step's `93 passed` line — the harness number is the binding figure |
| 4 | Skip/ignore-marker sweep | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn --include='*.rs' -e '#[ignore]' -e '#[should_panic]' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `0`, exit 0 — no skipped/ignored tests |
| 5 | Release cross-build | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` | 900000 | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; OBSERVED-DLL evidence line; **exit code 0** |
| 6 | Exe inspection (record the byte size) | `sh -c "ls -l /img-scroller/dist/windows-x64/img-scroller.exe"` | 60000 | non-empty size printed; RECORD the exact as-built byte count (any non-empty size passes; record it verbatim) |
| 7 | DLL import audit (per docs/build-windows.md) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe \| grep -i 'DLL Name' \| tr 'A-Z' 'a-z' \| sed 's/.*dll name: //' \| sort -u"` | 180000 | exactly 22 lowercase DLL lines, SET-EQUAL to the recorded constant (§6) |
| 8 | Audit fallback (ONLY if step 7 yields no output — report-format drift) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` | 180000 | raw dump; compare manually against §6; large output, still ONE call |
| 9 | Gitignore compliance sweep | host: `git status --short` (after re-reading `.gitignore`) | — | NO `dist/`, `logs/`, `target/`, `*.log` entries; nothing of them staged; nothing staged at all at 4.2 close (conditional commits of §8 are the only exceptions) |

## §4 Expected outcomes table (authoritative)

| Evidence | Expected |
| --- | --- |
| Harness log header | `dev-checks starting; log file: logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` — RECORD the exact printed path |
| Harness check lines (fixed order) | `PASS fmt-check exit=0 (<n> s)` — `cargo fmt --check` validates rustfmt layout only (formatting; the script never rewrites sources); then `PASS check exit=0 (<n> s)` (`cargo check --locked`); then `PASS test exit=0 (<n> s)` (`cargo test --locked`); then `PASS clippy exit=0 (<n> s)` (`cargo clippy --locked -- -D warnings` — every warning is then an error, so exit 0 means zero warnings) |
| Harness test block | `test result: ok. 93 passed; 0 failed; ...` + doc-test line `test result: ok. 0 passed` (normal; not a second count) |
| Census corroboration | grep per-file counts = §0 table; total 93 = 76 prior + 17 Task-3; zero `#[ignore]`/`#[should_panic]` hits |
| Harness final | literal `ALL CHECKS PASSED`; **exit code 0 = the authoritative success evidence** (sub-item (a)'s gate) |
| Release build | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; OBSERVED-DLL evidence line; **exit code 0** |
| Exe | NON-EMPTY gitignored `dist/windows-x64/img-scroller.exe` (0.4.0 wave build); exact as-built size recorded (delta vs the 19,502,691-byte 0.3.0 baseline is evidence only; any non-empty size passes — wave-magnitude expected, no target size pre-requirement) |
| DLL audit | 22 lowercase DLL names, SET-EQUAL to the recorded 2026-10-31 constant (§6) — **DLL import list UNCHANGED** |
| Git sweep | nothing ignored staged; at 4.2 close the tree is clean except nothing (4.2 makes no unconditional commits) |

## §5 Alpine-VM MCP usage notes (binding, per docs/build.md)

- `vm_status` BEFORE **every** `vm_run_command` call; only proceed when the VM is running and SSH-reachable; otherwise STOP and return a user question (docs/build.md precondition; global-plan binding line; wave execution note).
- ONE command per MCP call; pipelines live INSIDE one quoted `sh -c "..."` string.
- The `exitCode` field is the authoritative pass/fail evidence; `null` exitCode + `timedOut: true` → report the timeout with the partial captured output; do NOT re-run silently.
- Absent cargo stderr in captured MCP output is NOT a failure signal (documented caveat, repeated in §0).
- Generous timeouts (§3 table) beat aggressive ones.
- EBUSY-on-share-write hazard (vboxsf): retry the IDENTICAL command ONCE; if it fails again → STOP + report with evidence (no host-side improvisation this task — nothing here legitimately writes to the share).

## §6 DLL import audit — recorded constant and comparison evidence statement

Recorded `OBSERVED_DLL_IMPORT_LIST` (source of truth: `scripts/build-windows.sh` line 15, identical in `docs/build-windows.md`; audited 2026-10-31 with `x86_64-w64-mingw32-objdump`; docs-split rule: NOT duplicated into tech.md/architecture.md):

```
advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll,
api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll,
gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll,
opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll,
ws2_32.dll
```

Evidence statement to record in the 4.2 report (verbatim shape): "GW-4 audit: `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` inside the container; output deduplicated + lowercase-folded via the §3 step-7 pipeline; OBSERVED = 22 DLLs; SET-EQUAL to the recorded 2026-10-31 constant — **DLL import list UNCHANGED**."

On ANY diff (extra, missing, renamed — naming/dependency drift): **STOP + report** the full observed list and the delta. NO edits to `scripts/build-windows.sh` or `docs/build-windows.md` — refreshing the OBSERVED baseline is a Planner/Task-5 decision, never an implementer improvisation (Wave-1/2/3 precedent).

## §7 Failure/intervention policy for 4.2 (decision branches; max iterations PINNED)

- **fmt-check FAIL** → sanctioned minimal self-service fix: ONE `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` (`timeoutMs: 120000`); host-side `git diff` MUST show formatting-only reflow (no semantic content moves); if changed files exist → stage EXACTLY the `src/` paths listed by `git status --short` (never `.agent/`, never `docs/`, never ignored trees) and commit EXACTLY `style: apply cargo fmt` (separate commit; Wave-2 precedent); then re-run the §3 step-2 harness ONCE. Max iterations: 2 fmt-fix → 2 harness re-runs. Second consecutive fmt failure → STOP + report.
- **check FAIL** → STOP immediately. Compile defects are structural; report the full captured output + the printed log path (no self-fix — run-once-then-report).
- **test FAIL** → STOP immediately. Any failing named test = structural; §1.2's census rule protects against ad-hoc test invention; report with the failure detail lines.
- **clippy FAIL** → minor-lint self-service sanctioned ONLY when the failure is a SINGLE lint in a SINGLE file whose fix is ≤ 3 source lines. Fix with the `edit` tool, commit EXACTLY `fix: resolve minor clippy lint (verification round)` staged to that file only, then re-run the harness ONCE. Max 1 such fix round; the second consecutive clippy failure OR any multi-site/structural lint → STOP + report (structural → caller).
- Global cap: at most 2 post-fix harness re-runs in total across all sanctioned fix rounds; anything failing after the cap → STOP + report with evidence (four result lines + log path).
- `build-windows: error:` line of ANY type → STOP + report the typed error verbatim (the docs/build-windows.md failure table is the diagnostic map); never retry beyond the single EBUSY-once rule of §5.
- Every fix round re-reads `.gitignore` before staging (gitignore-compliance rule) and re-runs `git status --short`; `.agent/project-info/context.md` is planner/caller territory — NEVER staged by the implementer.

## §8 Git plan — which commits this task creates

| When | Commit | Content | Message (EXACT) |
| --- | --- | --- | --- |
| 4.2 unconditional | NONE (the lockfile is in sync — §2; no source edits are in scope) | — | — |
| Only if fmt drift | Separate | fmt-reflowed `src/` files only | `style: apply cargo fmt` |
| Only if minor clippy lint | Separate | the one ≤3-line fixed file | `fix: resolve minor clippy lint (verification round)` |
| **4.6 (the mark step — separate from 4.2)** | The Task-4 mark commit | this plan file + the TODO file (§10) | `task: mark wave-4 task4 verification done` |

- NO commit for build artifacts: `dist/` is gitignored; `logs/checks/` logs are gitignored; NOTHING built is ever staged (`git status --short` sweep at §3 step 9; unstage anything ignore-matching if it appears).
- NO `git push` (restricted to workflow Step 5: `main` → `origin` ONLY). NO amend/force. NO `git add -A`/`git add .` (explicit paths only). NO branch creation (Step-2 territory — the branch already exists).
- The 4.6 mark commit's staging list is EXACTLY: `.kilo/plans/20261102-wave-4-task4-verification-plan.md` + `.agent/todos/20261029/20261029-todo-6.md` (§10); verify with `git status --short` before committing that the staged list matches only these two.

## §9 Junior-implementer step-by-step checklist for 4.2

1. Host: `git branch --show-current` → MUST be `feat/wave-4-fullscreen-zoom`; `git status --porcelain` → MUST be empty (or ONLY `.agent/project-info/context.md` modified-unstaged — planner file, never staged); `git log --oneline -3` → HEAD `f42e950`. Any mismatch → STOP + ask the caller.
2. Host: read `.gitignore` (sweep reference). Read §4 (expected outcomes) so every comparison is pre-armed.
3. `vm_status` → must be running + SSH reachable; else STOP + user question.
4. §3 step P1 LF guard (both scripts) → expect exit 0 twice; exit 1 → STOP + report.
5. §3 step 1 pre-flight git evidence → matches, else STOP.
6. §3 step 2 harness run at the given timeout. Compare against §4 line-by-line as output streams; RECORD the printed log path.
   - All four `PASS` lines + literal `ALL CHECKS PASSED` + exit 0 + `test result: ok. 93 passed; 0 failed` → go to 7.
   - `FAIL fmt-check` → §7 fmt branch (once max), then re-run once → 6 again.
   - `FAIL check` / `FAIL test` → STOP + report (§7).
   - `FAIL clippy` → §7 clippy branch (once max) → re-run once → 6 again.
   - Post-cap failures → STOP + report with the four result lines + log path.
7. `vm_status`; §3 steps 3–4 census corroboration + ignore-marker sweep → table/93/`0`; mismatch → STOP + report (census ≠ 93 is a defect, §1.2).
8. `vm_status`; §3 step 5 release build → expect §4 build rows; `build-windows: error:` of any type → STOP + report verbatim.
9. `vm_status`; §3 step 6 `ls -l` → RECORD the exact as-built byte size verbatim in the report.
10. `vm_status`; §3 step 7 DLL audit → 22 lowercase lines set-equal to §6; any diff → STOP + report (no script/doc edits); no output → §3 step 8 fallback then compare.
11. Host: re-read `.gitignore`; `git status --short` sweep → NOTHING staged, no ignore-matching paths anywhere; §8 unconditional-commit row = none.
12. Report to caller (the §0–§8 evidence set): branch/HEAD; harness log path; the four PASS lines + `ALL CHECKS PASSED` + exit 0; the `93 passed; 0 failed` line; the grep census per-file split (76 + 17) and the zero-ignore result; `build-windows: OK`/artifact/exit 0; the as-built exe byte size; the §6 verbatim audit evidence statement; git-sweep result; **and the §11 OPERATOR-RUN checklist block copied VERBATIM for handoff to the user** — plus the explicit note that the OPERATOR-RUN sub-item remains PENDING until the user reports.

## §10 4.6 mark semantics (the Task-4 mark step; NOT executed inside 4.2 — encodes the caller's instruction)

When the Critical Workflow reaches 4.6 for Task 4 (AFTER §9 is fully green):

1. Read the TODO file first; PRESERVE all other content (overwrite-prevention rule). Exactly three edits to `### Task 4: Verification`:
   - Heading: `### Task 4: Verification` → `### Task 4: Verification [DONE]`.
   - Sub-item (a): `- [ ] Unit tests where pure logic exists ...` → `- [x] Unit tests where pre-existing logic exists ...` — NO wording change; ONLY the box `[]` → `[x]` (the harness evidence from §4 is what makes this check legal).
   - Sub-item (b) the OPERATOR-RUN line: **STAYS `- [ ]`** — pending the user's report on the real Windows machine (Wave-3 precedent `20261029-todo-5-DONE.md` line 31–35: heading marked, agent-verifiable sub-items checked, the manual-UX sub-item left unchecked until the operator reports). Do NOT annotate it with a status word; just leave the box.
2. Stage EXACTLY this plan file + the TODO file (§8); commit `task: mark wave-4 task4 verification done`.
3. NO `git push`, NO `-DONE` file rename (that is workflow Step 5), no other task headings touched, `.agent/project-info/context.md` untouched.

## §11 BUNDLED Wave-4 OPERATOR-RUN checklist (verbatim block; 4.2 report hands this to the CALLER for the user — it is NOT delivered by the implementer directly)

```
Wave-4 manual check (OPERATOR-RUN) — run dist\windows-x64\img-scroller.exe (the freshly rebuilt 0.4.0 exe)
on a real Windows 10/11 machine. Prepare: one folder holding differently shaped images — a WIDE/panoramic
image, a TALL portrait image, and a roughly square image — plus sibling folders beside it (some with images,
one WITHOUT images) so horizontal and vertical navigation both work.

1. LAUNCH STATE: launch the exe — the window opens at the normal placement/size it is given (never
   auto-maximized, never auto-fullscreened); fullscreen default is OFF. (Open an image first only when the
   app was installed/launched without one: use the big centered Open… button of the fresh empty state.)
2. OPEN & FIT: open a normal photo — it displays whole-image fit, centered; the bottom pill shows
   [←][→][↑][↓][fullscreen square][+][−][restore-fit square][fit-width glyph][fit-height glyph].
3. FULLSCREEN ON: press F11 — the title bar/borders AND the pill (every control) disappear; ONLY the image
   shows, filling the frame.
4. FULLSCREEN OFF: press F11 again — the standard chrome returns exactly (pill back; image central again).
   While ON the pill is not rendered, so its fullscreen button cannot be clicked then — F11 is the exit
   from ON.
5. FULLSCREEN BUTTON: with chrome visible, click the pill's fullscreen square — same ON behavior as F11.
6. ZOOM IN: with the photo open press I (or Shift+Plus, or click [+]) — the image grows ×1.25 per step.
   Pan is NOT part of this wave (Wave 5): the visible window shows the CENTER of the zoomed image.
7. ZOOM OUT / RESTORE: press O (or Shift+Minus, or click [−]) — shrinks ×1.25 per step and the zoom never
   goes below whole-image fit; press 0 (or click the restore-fit button) — back to whole-image fit ×1.0.
8. ZOOM CAPS: zoom in up to ×8 — additional zoom-in steps are REJECTED with the pill message `Zoom max`
   (a step that merely LANDS on the cap clamps silently, no message); from fit, an extra zoom-out step
   shows `Zoom min`. At ×8 the [+] button paints disabled; at fit the [−] and restore buttons paint disabled.
9. RESET ON OPEN: zoom in a few steps, then arrow-navigate to the NEXT image — the new image appears at
   whole-image fit ×1.0 (every successful open resets zoom AND fit mode).
10. FIT WIDTH: press W (or the fit-width glyph button) with the TALL portrait image open — the image WIDTH
    spans the window width and it overflows/clips vertically; the pill message shows `Fit width`.
11. FIT HEIGHT: press H (or the fit-height glyph button) with the WIDE panoramic image open — the image
    HEIGHT spans the window height and it overflows/clips horizontally; the pill message shows `Fit height`.
12. SQUARE CHECK: on the roughly square image, fit-width / fit-height / default fit all look nearly the
    same (sanity probe); the default whole-image fit itself shows NO message (silent).
13. FIT BUTTONS WITHOUT IMAGE: arrow up/down into a sibling folder WITHOUT images (the `Empty folder`
    surface): the two fit buttons are DISABLED there; the pill is still visible for navigation.
14. FULLSCREEN INTERPLAY: zoom to about ×2, press F11 (ON) — the zoomed image fills the frame per its
    dimensions; W/H/I/O/0 hotkeys still work while ON; press F11 (OFF) — the SAME zoom and fit mode
    persist (fullscreen toggles never touch zoom/mode state).
15. REGRESSION: ←/→ keys and pill buttons still walk the images of the folder exactly as in 0.3.0; ↑/↓
    still walk sibling folders (last-viewed memory and first-image rules unchanged); list edges stay
    silent and their buttons disable.
Report results back into the project (this closes the Task-4 manual-UX sub-item).
```

Result handling: recorded by the Planner at wave close; the Task-4 OPERATOR-RUN sub-item STAYS `[ ]` in the TODO until the user reports (binding TODO file structure; §10).

## §12 Deliberately out of scope (what 4.2 must NOT do)

- NO code-feature changes anywhere (no zoom/fullscreen/fit/navigation/listing/platform/display edits) — Task 4 changes nothing structural; fixes are bounded by §7.
- NO new tests, NO test additions/renames/skips "while here" (gap verdict §1.1: none; census pinned at 93).
- NO docs updates — Task 5 owns README/tech/architecture/structure/context; NO OBSERVED-DLL doc refresh here (a changed list is reported, never recorded, per §6). NO TODO-file 4.6 edits inside 4.2 (the mark is the 4.6 step, §10).
- NO `CHANGELOG.md` touches (wave standing ban). NO edits to existing `.kilo/plans/*` files, INCLUDING this plan once it lands.
- NO version changes (0.4.0 landed at Step 3 together with the lock — §2). NO Cargo.toml/Cargo.lock touches at all.
- NO UI edits; NO x86; NO Wave-5 pan/pan-then-navigate; NO "Open with..."; NO subfolder-picker (parked idea record-only).
- NO `.agent/project-info/context.md` staging or editing.
- NO `git push`, NO branch creation/switch, NO merge (Step 2/Step 5 territory). NO `FORCE_FAIL` invocations. NEVER stage `dist/**`, `logs/**`, `target/**`, `*.log`, or `Cargo.*`.

## §13 Stop conditions → return a user question

- `vm_status` not running / `/img-scroller` share unavailable at any VM-gated step.
- §3 step 1 pre-flight evidence mismatch (branch/HEAD/tree) of any kind.
- P1 LF guard exit 1 (CRLF recurrence — class outside this task).
- Any `--locked` lock-drift error (§2) — STOP; NO reconcile improvisation.
- Any harness `FAIL` line or non-0 exit beyond §7's sanctioned fix rounds; the harness summary ≠ `93 passed; 0 failed; ...` (§1.2); grep census ≠ 93 or ≠ the §0 per-file split; any `#[ignore]`/`#[should_panic]` hit.
- Any `build-windows: error:` typed line (report verbatim; the docs table is the diagnostic map).
- DLL audit ≠ the 22-OBSERVED set (report the delta; never record a new baseline).
- Any out-of-scope temptation in §12 arises again → STOP; the caller decides.
