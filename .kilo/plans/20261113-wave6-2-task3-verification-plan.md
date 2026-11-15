# Wave 6.2 — Task 3 (Cycle C) Verification Plan (Critical Workflow step 4.1b, architector)

- Source TODO: `.agent/todos/20261113/20261113-todo-2.md` → `### Task 3: Verification` (3 sub-items; binding clarity requirement: the OPERATOR-RUN checklist must be presented CLEARLY AND PLAINLY in the chat at the wave close — this plan carries it verbatim, §3, for the Planner to copy).
- Global plan: `.kilo/plans/20261113-wave6-2-open-with-launch.md` → `### Cycle C — Task 3 (verification)` + Step Map line "Task 3 (Verification): 4.1b Verification Plan → architector (PLAIN checklist)".
- Branch: `fix/wave-6-2-open-with-launch` (VERIFIED checked out, working tree clean). NO branch creation, NO switch, NO push in this cycle (`git push` is Step-5-restricted, `main` → `origin` ONLY).
- HEAD at plan time: `0378d6b` (`task: mark wave6.2 task2 done`); version 0.6.2 (`Cargo.toml:3`).
- Executor: JUNIOR developer under 50% restriction — every step, command, and expected outcome below is FINAL; do not redesign, reorder, or extend. Latitude: nothing beyond re-running a failed command once per §1's cap.
- Scope of THIS cycle: verification only. NO new source code, NO tests, NO doc-file edits (Task 4 = Cycle D), NO TODO edit (4.6 does that), NO diagnostic build, NO `--features` invocation of any kind, NO CHANGELOG.md, NO edits to existing `.kilo/plans/*` files (the two NEW files this cycle creates are §2 Step 0's plan commit of THIS file and §2 Step 8's results file).

---

## §0 LANDED FACTS THIS PLAN VERIFIES (all verified at HEAD `0378d6b`, this session)

| # | Fact | Verified by |
| --- | --- | --- |
| 1 | The fix landed: pure `shell_parseable_path(canonical_path: &Path) -> PathBuf` in `src/open_with_listing.rs:93` (constants `VERBATIM_PATH_PREFIX`/`VERBATIM_UNC_PATH_PREFIX` at lines 86–87; 6 new tests in `src/open_with_listing/tests.rs`); the ONE production call site is `src/platform/windows_open_with.rs:146` (`let shell_path = shell_parseable_path(file_path);` as the FIRST statement of `file_data_object`, BEFORE the wide encode and `SHCreateItemFromParsingName`) | `git grep` + file reads |
| 2 | The diagnostic stays: feature `diagnostic-launch-log` in `Cargo.toml:6-8` with `default = []`; module `src/platform/launch_log.rs` (170 lines) declared under `#[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]` at `src/platform/mod.rs:15`; every hook in `windows_open_with.rs` is a cfg'd statement | file reads + grep |
| 3 | `scripts/build-windows.sh` contains NO `--features` anywhere (the release build never enables the feature) | grep of `scripts/` → no matches |
| 4 | Zero Cargo drift since `a048567` (the 0.6.2 bump): `git diff a048567..HEAD -- Cargo.toml` = ONLY the +4-line `[features]` block; `git diff a048567..HEAD -- Cargo.lock` = EMPTY | git diff |
| 5 | Census 195 unit tests / 28 test-bearing files (settled method: `#[test]`-bearing files). Breakdown verified by `git grep`: **19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`) = 28**; `#[test]` occurrences total exactly 195 (Task-2's +6 joined the EXISTING `src/open_with_listing/tests.rs`, so the file count stayed 28). NOTE: the caller sub-task prompt's "23 `*_tests.rs`" is arithmetically inconsistent with 28 and is SUPERSEDED by this direct count — the verification pins are 195 tests / 28 files (see §4) | `git grep -c "#\[test\]" -- src` (sums to 195 over 28 files) |
| 6 | Baseline artifact: `dist/windows-x64/img-scroller.exe` = 19,645,219 bytes (the 0.6.1 exe). Also present in `dist/windows-x64/`: `img-scroller-diagnostic.exe` (225,180,316 B) + `img-scroller-diagnostic.log` (2,768 B) — the Task-1 operator-run artifacts. ALL of `dist/` is gitignored (`.gitignore:31`); `*.log` too (`.gitignore:17`) | host directory listing + `.gitignore` |
| 7 | The recorded 22-DLL import constant lives VERBATIM in `scripts/build-windows.sh:15` (`OBSERVED_DLL_IMPORT_LIST`) and `docs/build-windows.md` (Observed DLL import list) | file reads |
| 8 | Wave-6.2 commit chain `b458897..0378d6b` (11 commits, oldest→newest): `fd7acce` (report+TODO), `c3611ae` (global plan), `a048567` (0.6.2 bump), `02f639a` (T1 plan), `4185d97` (diagnostic feature), `e8c3f0b` (T1 docs), `e1cdfc3` (T1 mark), `b2b47a0` (T2 plan), `2b46a2e` (the fix), `1aac7e1` (T2 docs), `0378d6b` (T2 mark). Source-file footprint `git diff b458897..HEAD --stat -- src Cargo.toml Cargo.lock scripts docs` = exactly 6 paths: `Cargo.toml` (+4), `src/open_with_listing.rs`, `src/open_with_listing/tests.rs`, `src/platform/launch_log.rs`, `src/platform/mod.rs`, `src/platform/windows_open_with.rs`; `scripts/` and `docs/` EMPTY | git log + git diff --stat |
| 9 | Harness: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` (fmt-check → check → test → clippy; final `ALL CHECKS PASSED`/`N CHECK(S) FAILED`; exit code authoritative; logs under gitignored `logs/checks/`); release build `docker compose ... run --rm rust sh scripts/build-windows.sh` (atomic part-file rename install; typed failures prefixed `build-windows: error:`; exit 0) | `docs/build.md` + `docs/build-windows.md` + `scripts/build-windows.sh` |

---

## §1 FAILURE-HANDLING POLICY (applies to EVERY step below)

1. The command EXIT CODE is the authoritative success evidence (alpine-vm MCP; `vm_status` FIRST before every VM command; absent stderr is NOT a failure signal).
2. If a step's pinned expected outcome does not hold: capture the EXACT failing output (the container log path or the diff) and STOP — return the exact failure to the caller. Do NOT improvise fixes outside §1 item 4's single cap.
3. STOP-only failures (NO fix attempt at all): a census mismatch (Step 1); a windows-check error other than the pinned benign warning (Step 2); a `build-windows: error:` typed failure (Step 3); ANY zero-diagnostic-pin failure (Step 3); a DLL delta (Step 4 — STOP + the delta list); a sweep mismatch (Step 5); a git-chain mismatch (Step 6).
4. The ONE minimal in-scope fix attempt cap: if `dev-checks` fails ONLY on `fmt-check` (formatting drift) OR a step fails on a transient VM/harness hiccup (VM unreachable after `vm_status` said OK), the implementer may run the fmt round (`docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt`) and/or re-run the SAME command ONCE more. A second consecutive failure → STOP per item 2. Nothing else is fixable in this cycle.
5. If the VM or the `/img-scroller` share is unavailable: STOP and return a user question (per `docs/build.md`).

---

## §2 VERIFICATION STEPS (4.2 implementer — in order)

### Step 0 — Pre-flight + plan commit (workflow boilerplate, NO verification content)

1. On the host (bash tool, single commands only): `git status` — verify branch `fix/wave-6-2-open-with-launch`, working tree clean except this plan file (untracked or the file the caller staged). Verify HEAD is `0378d6b` (`git log --oneline -1`).
2. Read `.gitignore` (Gitignore Compliance Rule — required before every commit below).
3. Commit THIS plan file: stage ONLY `.kilo/plans/20261113-wave6-2-task3-verification-plan.md`; `git status` check (nothing gitignore-matching staged — `dist/`, `logs/`, `*.log` must not appear); commit message: `docs: add wave6.2 task3 verification plan`.
4. Call `alpine-vm_vm_status` FIRST (before any VM command below). Unavailable → STOP + user question.

### Step 1 — Fresh dev-checks at HEAD (TODO sub-item 1)

1. `alpine-vm_vm_status` immediately before.
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` (MCP timeout 900000 ms; if output is truncated, the final line + exit code remain authoritative — the full record is in the `logs/checks/dev-checks-<timestamp>.log` printed at run start).
3. Expected (ALL pinned): exit code `0`; final line `ALL CHECKS PASSED`; per-check `PASS` for `fmt-check` / `check` / `test` / `clippy`; the cargo-test result line shows **195 passed; 0 failed**.
4. Census corroboration (host, bash tool, two single commands, no chaining):
   - `git grep -l "#\[test\]" -- src` → exactly 28 file lines (the settled method's `#[test]`-bearing files).
   - `git grep -c "#\[test\]" -- src` → 28 file lines whose counts sum to exactly 195.
5. Record in the results file (Step 8): the per-check PASS lines, the census numbers, and the corroboration output.
6. Failure → §1.

### Step 2 — Windows-target compile (the REAL `file_data_object` edit; the container default target compiles the stub path only)

1. `alpine-vm_vm_status` immediately before.
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu` (timeout 600000 ms).
3. Expected: exit code `0`. The ONLY acceptable diagnostic is the pre-existing benign dead-code warning on `compare_entry_names` (the Wave-5.2 recorded note). Any error or any OTHER warning → STOP per §1 item 2.
4. Record in the results file: exit code + the warning (or its absence).

### Step 3 — Release rebuild + the ZERO-DIAGNOSTIC pin (TODO sub-item 2, part 1)

1. `alpine-vm_vm_status` immediately before.
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` (timeout 900000 ms).
3. Expected: exit code `0`; the lines `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, and the OBSERVED-DLL-list note line. The 19,645,219-byte 0.6.1 artifact is overwritten atomically (the part-file rename in `scripts/build-windows.sh:44-59`); a failed run would have left it intact.
4. Record the EXACT byte size of the new artifact — run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust wc -c /img-scroller/dist/windows-x64/img-scroller.exe` (timeout 120000 ms). Expected: one line `<byte count> /img-scroller/dist/windows-x64/img-scroller.exe`, byte count > 0. Record the exact number.
5. **The ZERO-DIAGNOSTIC pin (all three legs; each expected result is pinned; any miss → STOP):**
   - **Pin A (the script never enables the feature):** host grep of `scripts/` for the pattern `--features` → NO matches (`scripts/build-windows.sh` runs plain `cargo build --release --locked --target x86_64-pc-windows-gnu`, `scripts/build-windows.sh:65`).
   - **Pin B (the Cargo feature default):** `Cargo.toml` lines 6–8 read exactly `default = []` / `diagnostic-launch-log = []` under `[features]` — the release build (no `--features`) compiles with the feature OFF, so the cfg-gated module (`src/platform/mod.rs:15`) and every hook compile OUT of the release artifact.
   - **Pin C (the binary carries no diagnostic trace):** run in the container: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -a -c 'img-scroller-diagnostic' /img-scroller/dist/windows-x64/img-scroller.exe"` (timeout 120000 ms). Expected: the output `0` AND exit code `1` — for grep, "no lines matched" prints count 0 and exits 1, which IS the expected evidence here (the string exists only in `src/platform/launch_log.rs`, which is compiled out). Any count > 0 → STOP with the exact output.
6. Record in the results file: the build exit code, the exact byte size, and all three pin outcomes.
7. NOTE (do NOT act): `dist/windows-x64/img-scroller-diagnostic.exe` and `img-scroller-diagnostic.log` are NOT touched, deleted, or rebuilt in this cycle — the diagnostic exe must stay available for a follow-up evidence run if the operator re-test still fails (§3 closing note); their removal belongs to the global plan's Step-5 tmp sweep.

### Step 4 — DLL import audit vs the recorded 22-DLL constant (TODO sub-item 2, part 2; per `docs/build-windows.md`)

1. `alpine-vm_vm_status` immediately before.
2. Run (ONE container command): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | sed -n 's/^.*DLL Name: //p' | tr 'A-Z' 'a-z' | sort -u"` (timeout 120000 ms).
3. Expected: EXACTLY these 22 lines (deduplicated, lowercase), set-equal to the recorded constant (`scripts/build-windows.sh:15` / `docs/build-windows.md`):
   `advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll`
   (presented by the audit as 22 separate lines, one DLL per line.)
4. Rationale (why set-equal is expected): the fix added pure string operations (`shell_parseable_path` — `strip_prefix`/`to_string_lossy`/`PathBuf::from`, std-only) and NO new API imports; the diagnostic module is compiled out (Step 3 Pin C); the shell API surface is unchanged.
5. ANY delta (a DLL added or missing) → STOP and return the delta list (added vs missing) to the caller. NO fix attempt.
6. Record in the results file: the 22 observed lines verbatim + the SET-EQUAL verdict.

### Step 5 — Wave-pin sweep (read-only; host bash/git + reads; every item pinned)

| # | Check | Command (single, host) | Expected |
| --- | --- | --- | --- |
| S1 | The conversion at the ONE call site | `git grep -n "shell_parseable_path" -- src` | EXACTLY 12 matches: 3 in `src/open_with_listing.rs` (2 doc-comment lines + the definition at `:93`), 7 in `src/open_with_listing/tests.rs` (the use line + 6 test assertions), 2 in `src/platform/windows_open_with.rs` (the import at `:46` + the call at `:146`). The ONLY production consumer is `windows_open_with.rs:146`, inside `file_data_object`, as the FIRST statement BEFORE the wide encode / `SHCreateItemFromParsingName`. |
| S2 | The stored canonical path untouched | `git diff a048567..HEAD -- src/app.rs src/app/open_with_button.rs` | EMPTY (the D5 single-normalization point and the launch hand-off are byte-identical to the pre-diagnostic wave state). |
| S3 | The diagnostic module untouched | `git diff 4185d97..HEAD -- src/platform/launch_log.rs src/platform/mod.rs Cargo.toml` | EMPTY (the diagnostic stays exactly as landed; the feature stays; nothing removed). |
| S4 | Zero Cargo drift since `a048567` | `git diff a048567..HEAD -- Cargo.toml` and `git diff a048567..HEAD -- Cargo.lock` | The `Cargo.toml` diff shows ONLY the 4-line `[features]` block (verbatim in §0 fact 4); the `Cargo.lock` diff is EMPTY. |
| S5 | The fix's full source footprint | `git diff b458897..HEAD --stat -- src Cargo.toml Cargo.lock scripts docs` | EXACTLY the 6 paths of §0 fact 8 (Cargo.toml +4; the 5 src paths); `scripts/` and `docs/` show NOTHING. |
| S6 | No stray artifacts | host directory listing of `dist/windows-x64/` | EXACTLY 3 files: `img-scroller.exe` (the new byte size from Step 3), `img-scroller-diagnostic.exe` (225,180,316 B, untouched), `img-scroller-diagnostic.log` (untouched). |

Any mismatch → STOP per §1 item 2 (exact diff/output to the caller). Record all six outcomes in the results file.

### Step 6 — Git segmentation

1. Host `git status` → clean working tree (after Step 0's plan commit; nothing new is written before this step).
2. Host `git log --oneline b458897..HEAD` → the 11-commit wave chain of §0 fact 8 (verbatim, oldest `fd7acce` … newest `0378d6b`) topped by Step 0's plan commit.
3. Host `git log --oneline -1` → HEAD = the plan commit from Step 0 (the results commit of Step 8 comes after; the chain check here pins the pre-results state).
4. Verify NO commit in the chain touched gitignored paths: `git log --stat b458897..HEAD -- dist logs` → EMPTY output.
5. Any mismatch → STOP. Record the chain output in the results file.

### Step 7 — The OPERATOR-RUN checklist (TODO sub-item 3)

The checklist is CARRIED VERBATIM in §3 of this plan. The implementer does NOT present it (the Planner presents it in the chat at the wave close, per the global plan Step Map Step-6 line and the TODO's binding clarity requirement). The implementer's only actions: (a) confirm §3 is intact in the committed plan file; (b) hand the plan path + the §3 section to the caller in the completion summary, stating that the Planner must present §3 verbatim at the wave close.

### Step 8 — Record results + commit protocol

1. Write the NEW file `.kilo/plans/20261113-wave6-2-task3-verification-results.md` (NOT an edit of any existing plan file) containing, per step: Step 1 (per-check PASS lines + 195/0 census + the two corroboration greps), Step 2 (exit code + warning state), Step 3 (build exit code, the EXACT recorded byte size, Pins A/B/C outcomes), Step 4 (the 22 observed DLL lines + the SET-EQUAL verdict), Step 5 (the six sweep outcomes), Step 6 (the git chain), and the verdict line (e.g. `ALL VERIFICATION STEPS PASSED — agent-side Task 3 GREEN; pending the §3 OPERATOR-RUN`).
2. Record the standing notes verbatim in the results file:
   - The release artifact `dist/windows-x64/img-scroller.exe` stays GITIGNORED and UNCOMMITTED (`dist/` never enters git).
   - The diagnostic exe + log stay in `dist/windows-x64/` for a possible follow-up evidence run (the feature stays; already authorized); they are removed ONLY at the global plan's Step-5 tmp sweep.
   - The manual-UX TODO sub-item STAYS `[ ]` — the §3 OPERATOR-RUN is the user's gate; Task 3's 4.6 marks sub-items 1+2 `[x]` only.
3. Commit: read `.gitignore` (already read at Step 0; re-check), run `git status` (only the new results file staged; NO gitignored file staged — `dist/`, `logs/`, `*.log` must not appear), stage ONLY `.kilo/plans/20261113-wave6-2-task3-verification-results.md`, commit message: `docs: add wave6.2 task3 verification results`.
4. Re-run `git status` → clean tree. NO push, NO branch change, NO TODO edit (4.6 does the marking), NO CHANGELOG.md.
5. Signal completion with: what was done (per-step verdicts), what was NOT done (no code changes, no feature-ON runs, no diagnostic rebuild, no TODO marking, no push), the results-file path, and the reminder that the Planner presents §3 verbatim in the chat at the wave close.

---

## §3 THE OPERATOR-RUN CHECKLIST (verbatim — the Planner copies THIS block into the chat, plainly, at the wave close)

```text
0.6.2 re-test — the "Open with..." fix (run dist\windows-x64\img-scroller.exe):

1. Open a png image (the same one as before, e.g. [Image 1]). Click the "Open with..."
   button (or press Q) and select PAINT in the list: the image opens in Paint with NO
   failure message.
2. Open the list again and select PHOTOS: the image opens in Photos with NO failure
   message. (If you select img-scroller itself in the list, a second viewer window
   opens — that is EXPECTED, not a failure.)
3. Press Q: the "Open with..." menu opens. Press Q again: the menu closes.
4. While the menu is open, press other keys and click other buttons (arrows, zoom, fit,
   fullscreen): they all do NOTHING.
5. Press ESC while the menu is open: ONLY the menu closes — the "Press ESC again to
   close" app-exit popup does NOT appear.
6. Click outside the menu: the menu dismisses.
7. The "Q" caption renders under the open-with button symbol (white, thin black outline).
8. If a launch ever fails, the failure message shows in WARNING (orange) color on the
   pill's message line.
9. Regression block — everything that already passed on 0.6.1 still works: arrow
   navigation and folder navigation (including the navigation name order); zoom
   in/out/restore (I / O / 0, Shift+'+' / Shift+'−'); fit width/height (W / H);
   fullscreen (F11, ESC exit); the double-ESC close contract; pan at zoomed image
   edges; opening files; and the open-with launch state (the tooltip "Open with... (Q)";
   Q stays inert when no programs are registered for the file type, in empty-folder
   mode, and in fullscreen; an empty list shows "No programs registered for this file
   type").

If item 1 or 2 STILL fails: do NOT delete anything — the diagnostic build
(dist\windows-x64\img-scroller-diagnostic.exe, writing img-scroller-diagnostic.log
next to it) stays available for a follow-up evidence run; no new authorization is
needed, it is already authorized.
```

Coverage note (for the Planner only, do NOT paste): items 1–2 carry the task prompt's D1 RE-TEST content (a few programs, at least Paint and Photos, the img-scroller second-instance note, NO failure message); items 3–9 carry the prompt's "everything that passed on 0.6.1 re-runs as regressions" list item-by-item; the closing note carries the diagnostic-fallback line.

---

## §4 CENSUS-NOTE (recorded, binding for the implementer)

The caller sub-task prompt's census breakdown "23 `*_tests.rs` + 4 `tests.rs` + 5 in-file" sums to 32 and contradicts its own 28-file total; the recorded project breakdown (`.agent/project-info/context.md`, Wave-6.1 census correction) and this session's direct `git grep` count both give **19 `*_tests.rs` + 4 `tests.rs` + 5 in-file = 28**. The verification pins are the VERIFIED numbers: 195 unit tests, 28 `#[test]`-bearing files. This note is informational — no file outside this plan is edited.

---

## §5 REVIEWER / ADHERENCE HOOKS (4.3 + 4.5b — for the later steps, NOT executed now)

- 4.3 code-reviewer/code-simplifier: expected NO-OP — this cycle writes NO source code; review covers only the results file's factual accuracy (each recorded value matches the actual command outputs).
- 4.5b architector adherence checklist: Steps 1–8 all executed in order; all pinned expected outcomes recorded; the §3 checklist intact and handed to the caller; the results file committed; no source/test/doc drift (`git diff 0378d6b..HEAD --stat -- src Cargo.toml Cargo.lock scripts docs .agent .kilo/commands` shows ONLY the two new `.kilo/plans/` files of Steps 0 and 8).
- 4.6 implementer: mark Task 3 `[DONE]`; sub-items 1+2 `[x]`; the manual-UX sub-item STAYS `[ ]` (preserve all other file content; commit per the workflow).

---

## §6 SCOPE BOUNDARIES (hard)

- NO source-code changes, NO new tests, NO dependency/manifest changes, NO `--features` invocation, NO diagnostic rebuild, NO feature removal.
- NO edits to existing `.kilo/plans/*`, `.agent/todos/*` (before 4.6), `CHANGELOG.md`, `docs/*`, or `.agent/project-info/*` (the wave-level documentation is Task 4 / Cycle D).
- NO push, NO branch creation/switch/merge, NO TODO rename (Step 5 of the global plan).
- NOTHING in `dist/` is deleted or modified except the atomic release-artifact overwrite performed by `scripts/build-windows.sh` itself.

---

## §7 VERBATIM COMPLIANCE vs TODO TASK 3

| TODO Task 3 sub-item | Covered by |
| --- | --- |
| Unit tests / dev-checks harness (`ALL CHECKS PASSED`; `alpine-vm`; `vm_status` first; exit authoritative) | §2 Step 1 (195/0 census + the settled-method corroboration) |
| Release rebuild: non-empty 0.6.2 exe with ZERO diagnostic code; DLL audit set-equal or explained delta per `docs/build-windows.md` | §2 Step 3 (rebuild + byte size + Pins A/B/C) + §2 Step 4 (22-DLL audit) |
| Manual UX check (OPERATOR-RUN) — the checklist CLEARLY AND PLAINLY in the chat; the pending item: selecting a program opens the current file; the standing regressions re-run | §2 Step 7 + §3 (the verbatim PLAIN checklist; the Planner presents it at the wave close) |

## §8 SOURCES (consulted this session)

- Read in full: the source TODO, the global plan, the Task-1 and Task-2 wave plans, `docs/build.md`, `docs/build-windows.md`, `scripts/build-windows.sh`, `Cargo.toml`, `.gitignore`, `.agent/project-structure.md`, `.agent/WORKFLOWS.md`, `.kilo/commands/critical-workflow.md`, `.agent/project-info/instructions.md`, `brief.md`, `context.md`.
- Verified live: `git log`/`git status`/`git grep`/`git diff` at HEAD `0378d6b`; `src/open_with_listing.rs` (the landed fix); the `shell_parseable_path` call-site grep; the `--features`-absence grep of `scripts/`; the `dist/windows-x64/` directory listing.
- Wave-6.1 precedent for the results-file pattern: `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` + `-verification-results.md` (referenced only; NOT edited).
