# Wave 3 Task 4 — Verification Plan 4.1b (dev-checks, lockfile reconcile, exe rebuild, DLL audit, coverage mapping)

- Date: 2026-11-01
- Source TODO: [`.agent/todos/20261029/20261029-todo-5.md`](../../.agent/todos/20261029/20261029-todo-5.md) → `### Task 4: Verification` ONLY (all three sub-items). NOT front-end; no other task's plans are expanded.
- Workflow slot: Wave-3 Critical Workflow, Task 4, sub-step 4.1b → executed by the implementer in 4.2.
- Global plan: [`.kilo/plans/20261101-wave3-vertical-scroll.md`](20261101-wave3-vertical-scroll.md) → "### Task 4 — Verification (NOT front-end)" + Risk/Escalation Notes (+ the Task-4 entry in Execution Order).
- Build docs: [`docs/build.md`](../../docs/build.md), [`docs/build-windows.md`](../../docs/build-windows.md); harness: [`scripts/dev-checks.sh`](../../scripts/dev-checks.sh), [`scripts/build-windows.sh`](../../scripts/build-windows.sh).
- Precedent plans followed (structure, decision shapes, admin): [`.kilo/plans/20261031-wave2-task4-verification.md`](20261031-wave2-task4-verification.md), [`.kilo/plans/20261101-floating-controls-verification.md`](20261101-floating-controls-verification.md).
- Approval: per-task plan auto-approved by the user's "Approve Global and Tasks Plans" choice — every decision below is BINDING; zero judgment left for the implementer.
- Target implementer: JUNIOR under 50% restriction; all structural/scope decisions are encoded here.

## 0. Session facts captured at plan time (verified evidence)

- Branch `feat/wave-3-vertical-scroll`; HEAD `9c2c94a` (`task: mark wave-3 vertical navigation done`); `git status --porcelain` → CLEAN (no output). Task-1/2/3 code chain present below the tip: `bc7261a`, `c2dcfba`, `1428c4b`, `5d503a2`, `7657f32` (the known clippy ptr_arg risk already fixed), `51c1b08`, `36be11b`, `4864c3c`, `e2845c1`.
- Version `0.3.0` committed at `89e1db4` — **that commit changed ONLY `Cargo.toml`** (`git show --stat 89e1db4`: 1 file, 1 insertion). The 0.2.1 precedent `10d0253` changed BOTH Cargo.toml and Cargo.lock. `git log 89e1db4..HEAD -- Cargo.lock` → EMPTY: **no commit after the bump touched Cargo.lock**, and the clean tree ⇒ the lock on disk still pins `img-scroller` `0.2.1`. **LOCKFILE IS STALE — the first `--locked` cargo invocation (dev-checks step 2, or the release build) WILL FAIL** with a "the lock file needs to be updated but --locked was passed"-class error. §2 encodes the sanctioned reconcile BEFORE any `--locked` run. (Cargo.lock is read-blocked for host-side read tools by the ignore configuration — verification of the lock content goes through git diff/`git show`, never a direct read tool; this is why §0 and §2 verify by diff-shape, not content reads.)
- Test census (grep `#[test]` over `src/`, at HEAD): app/tests.rs 9 + app/folder_navigation_tests.rs 5 + app/folder_navigation_continuation_tests.rs 4 + app/floating_controls_tests.rs 1 + app/last_viewed_tests.rs 5 + folder_listing/tests.rs 5 + sibling_folder_listing/tests.rs 8 + name_collation.rs 2 + image_loading.rs 4 + app_display.rs 2 + scaling.rs 5 = **50 tests** (matches the Task-4 expected census "50 after Task 3").
- `.gitignore` contains `dist/`, `logs/`, `*.log`, `target/` (verified read) — artifacts and check logs are never stageable. `.gitattributes` contains `*.sh text eol=lf` (verified read) — harness scripts materialize LF at checkout; the Wave-2 CRLF defect class is structurally prevented.
- LAST KNOWN EXE (delta-only reference, never an expectation): `dist/windows-x64/img-scroller.exe` = 19,461,520 bytes (0.2.1 build, 2026-11-01). The Wave-3 exe will differ in size (Wave-3 added sibling_navigation/last-viewed/navigation code and, at build time, the 0.3.0 section) — ANY non-empty size is acceptable; the recorded verbatim value is the deliverable.
- Harness semantics (docs/build.md): fixed order fmt-check → check → test → clippy; every check runs even if earlier ones fail; per-check line `PASS|FAIL <name> exit=<code> (<n> s)`; final `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1); UTC-timestamped log appended under gitignored `logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` (path printed at run start = authoritative); the script never modifies sources and never applies fmt fixes; `FORCE_FAIL` is documentation-only (NOT used this task).
- build-windows semantics (docs/build-windows.md + script read): runs `cargo build --release --locked --target x86_64-pc-windows-gnu`; success prints `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, then the OBSERVED-DLL static-linking evidence line; atomic `.part` → rename; failures all start `build-windows: error:` and exit 1; a failed run leaves any previous artifact byte-intact.

## 1. Unit-coverage mapping (TODO Task-4 bullet 1 → landed tests, name by name)

The TODO bullet demands coverage of: (a) same-parent restriction (no grandparent/deeper jumps), (b) sort order, (c) target-image priority (visited → last-viewed; new folder → first image), (d) last-viewed map set/get, (e) per-session lifetime. Mapping onto the landed 50 tests:

| TODO topic | Satisfied by (tests) | File |
| --- | --- | --- |
| (a) same-parent restriction / no grandparent / no deeper traversal | `listing_covers_only_immediate_scope_child_folders` (creates a grandchild folder `inner` under `folder_a` AND a `leaf.txt` inside it; asserts the listing is exactly `folder_a, folder_b, folder_c` — the grandchild/deeper level is NEVER enumerated); `listing_enumerates_only_scope_child_directories` (directories-only: stray files and dot-concealed entries excluded); `current_folder_is_excluded_from_the_enumerated_siblings` (the anchor's own folder name never listed — no self jump); `symlinked_scope_entries_are_not_listed_as_directories` (symlinked entries stay out); `missing_or_unreadable_scope_folder_yields_empty_listing_and_none_when_unanchorable` (unanchorable parent → `None`) | `src/sibling_folder_listing/tests.rs` |
| (a) app-level restriction | `vertical_navigation_resolves_the_adjacent_sibling_folders_in_sort_order` (from `folder2` the neighbors are ONLY `folder1`/`folder3` — same parent, ±1); `empty_folder_mode_continues_vertically_in_both_folder_kinds` (sibling-only continuation in empty-folder mode) | `src/app/folder_navigation_tests.rs` |
| (b) sort order (folder axis) | `sibling_folders_sort_by_lowercase_form_then_exact_name_tie_break` (folders ALPHA/Alpha/alpha/Bombay/ZULU pinned to the Wave-2 collation order); app-level `vertical_navigation_resolves_the_adjacent_sibling_folders_in_sort_order` | `src/sibling_folder_listing/tests.rs`, `src/app/folder_navigation_tests.rs` |
| (b) sort order (shared collation, both axes) | `lowercase_form_decides_order_before_exact_characters` + `exact_characters_tie_break_when_lowercase_forms_match` (the extracted collation both axes consume); `listed_images_sort_case_insensitively_then_by_exact_name_tie_break` + `navigation_resolves_the_previous_and_next_sibling_in_sort_order` (image axis unchanged — guards no Wave-2 behavior drift) | `src/name_collation.rs`, `src/folder_listing/tests.rs`, `src/app/tests.rs` |
| (c) priority — visited → last-viewed | `vertical_navigation_reopens_the_visited_folders_last_viewed_image` (opened `folder1/name2.png`, then opened `folder2`; PreviousSiblingFolder resolves `folder1` and the target is EXACTLY `folder1/name2.png` — the last-viewed entry, not `name1.png`); `vertical_navigation_to_a_folder_whose_last_viewed_image_vanished_reports_the_standard_failure` (priority edge: vanished entry → standard failure); `vertical_target_open_records_the_new_folders_last_viewed_memory` (navigation into a NEW folder records ITS last-viewed entry — priority state stays fresh after the jump) | `src/app/folder_navigation_tests.rs`, `src/app/folder_navigation_continuation_tests.rs` |
| (c) priority — new folder → FIRST image (no same-filename, no other fallback) | `vertical_navigation_opens_the_first_image_of_a_new_folder` (folder2 holds `b.png` + `a.png`; target resolves to EXACTLY `a.png` = first in the collation — proves the no-same-filename rule: no `b.png` attempt); `empty_folder_mode_continues_vertically_in_both_folder_kinds` (fresh `folder3` → target `c.png`, its only/first image) | `src/app/folder_navigation_tests.rs` |
| (d) last-viewed map set/get | `successful_open_records_the_folders_last_viewed_image` (set on success); `every_subsequent_open_in_a_folder_updates_the_last_viewed_entry` (overwrite updates); `failed_open_keeps_the_folders_last_viewed_image` (failure does NOT clobber); `each_folder_keeps_its_own_last_viewed_entry` (per-folder isolation + failure leaves both entries intact) | `src/app/last_viewed_tests.rs` |
| (e) per-session lifetime | `fresh_app_starts_with_an_empty_last_viewed_memory` (a FRESH `ImagesScrollerApp` = a new session starts with an EMPTY map — struct-scoped evidence); the map is a plain field on the app struct (`app.last_viewed_images`); a new app instance = fresh memory ⇒ it cannot survive a relaunch; destroyed when the app struct drops. There is NO drop-hook/static/globals test — there is nothing to hook: no statics, no singletons, no disk-write path exists for this map. | `src/app/last_viewed_tests.rs` + structural evidence |

### 1.1 GAP VERDICT

**No gaps.** All five TODO-required topics are pinned by landed tests, name-by-name above. No test additions are sanctioned in this task; the expected census stays EXACTLY 50. If a reviewer disputes topic (e)'s structural pin: the add-a-test decision belongs to the CALLER as a follow-up plan (never self-service in 4.2) — but the Wave-2 precedent (`tests census: structurally covered — NOT a gap`) applies as recorded above.

### 1.2 Census failure rule

If the harness `cargo test` summary does not read exactly `50 passed; 0 failed` (a trailing doc-test `test result: ok. 0 passed` line is expected and normal): **STOP** and return the full test section to the caller. A census mismatch is a defect, never an ad-hoc-patch opportunity.

## 2. Lockfile reconcile — SANCTIONED RESOLUTION (before any `--locked` run)

Decision (encoded, per repo precedent `10d0253` and the caller's instruction): a manifest-VERSION-ONLY change (89e1db4) reconciles by materializing the lock's own package-version entry via ONE plain (NO `--locked`) `cargo check`, staged/committed as its OWN commit BEFORE the dev-checks run.

- **L-FLOW.1** — `vm_status` first. Then run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check` — `timeoutMs: 600000`. Expect exit 0. This updates `/img-scroller/Cargo.lock`'s img-scroller version entry (0.2.1 → 0.3.0) WITHOUT touching dependency entries (plain `check` skips `--locked`; dependencies are all exact-pinned).
  - EBUSY hazard (vboxsf share; documented pattern: global plan Risk note line 111; primary citation `.kilo/plans/20261031-task3-wave1-open-image-implementation.md` "EBUSY failure handling" — retry the IDENTICAL command ONCE. If it fails again → the authorized host-side fallback (same citation): host-side one-line edit of `Cargo.lock` restoring the landed diffshape of `10d0253` — the img-scroller package entry's `version` line only, `0.2.1` → `0.3.0` — via the dedicated `edit` tool (real newline rule; no vault content reformatting), then ONE `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked` to PROVE manifest/lock consistency (expect exit 0). Apply the fallback ONLY if triggered.
- **L-FLOW.2** — host-side `git diff Cargo.lock` (`git diff -- Cargo.lock`): the diff MUST be exactly one hunk flipping the img-scroller `version` line (`-version = "0.2.1"` / `+version = "0.3.0"` equivalent, 2 changed lines total, matching the diffshape `10d0253` recorded: 1 file, 2 insertions). **Extra hunks of ANY kind (dependency entries, resolution tree) → STOP and report; do not commit a churned lock.**
- **L-FLOW.3** — stage and commit EXACTLY: `git add Cargo.lock` → `git status --short` (staged list = ONLY `Cargo.lock`) → `git commit -m "chore: update cargo lock for 0.3.0"`. This is the commit that holds the lockfile update. NO other file may ride in this commit.

Expected outcome: the reconcile commit exists and `cargo check` exit 0 (or, fallback path, `cargo check --locked` exit 0) — manifest and lock agree on 0.3.0 before any `--locked` harness/build run.

## 3. Container command sequence for 4.2 (exact, per docs/build.md / docs/build-windows.md)

All VM/container commands go through the `alpine-vm` MCP: call `vm_status` BEFORE EVERY command; ONE command per call; multi-step composition only as a single quoted `sh -c "<command string>"` (never host-side `&&`/`;` chaining); the command `exitCode` is authoritative; captured MCP stderr may be empty (absence is NOT a failure signal); a timeout (`timedOut: true`, `exitCode: null`) is REPORTED, never silently retried.

| # | Step | Command (exact) | timeoutMs | Expected |
| --- | --- | --- | --- | --- |
| 0 | VM gate | `vm_status` (MCP tool) | — | running + SSH reachable; else STOP + user question |
| P1 | Pre-flight LF guard (cheap; .gitattributes should already make both exit 0) | `sh -c "sed 's/\r$//' /img-scroller/scripts/dev-checks.sh \| cmp - /img-scroller/scripts/dev-checks.sh"` then the identical command with `scripts/build-windows.sh` | 60000 | exit 0, no output (LF-only). exit 1 (CRLF present) → STOP + report (recurrence of a class outside Task-4 scope; the caller owns the fix shape) |
| 1 | LOCK reconcile | §2 L-FLOW.1 + L-FLOW.2 + L-FLOW.3 | 600000 | §2 expected outcomes |
| 2 | Pre-flight git evidence | host: `git branch --show-current`; `git log --oneline -10` | — | branch = `feat/wave-3-vertical-scroll`; HEAD = the lock commit; Task-1/2/3 chain + `89e1db4` below it |
| 3 | Dev-checks harness | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` | 900000 | see §4 table |
| 4 | Release cross-build | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` | 900000 | `build-windows: OK`; artifact line; OBSERVED-DLL evidence line; exit 0 |
| 5 | Exe inspection | `sh -c "ls -l /img-scroller/dist/windows-x64/img-scroller.exe"` | 60000 | non-empty size printed; RECORD the exact byte count (recorded as-built value = deliverable; the script already asserts `-s` non-empty post-install) |
| 6 | DLL import audit re-check | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe \| grep -i 'DLL Name' \| tr 'A-Z' 'a-z' \| sed 's/.*dll name: //' \| sort -u"` | 180000 | exactly 22 lowercase DLL lines, set-equal to the OBSERVED constant (§6) |
| 7 | Gitignore compliance sweep | host: `git status --short` | — | NO `dist/`, `logs/`, `target/`, `*.log` entries; nothing of them staged; tree matches the landed commits only |

Fallback for step 6: if the filtered pipeline yields no output (report-format drift), run the raw dump ONCE: `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` and compare manually (large output; still ONE call).

## 4. Expected outcomes table (authoritative)

| Evidence | Expected |
| --- | --- |
| Lock reconcile | `cargo check` exit 0; `git diff Cargo.lock` = version-line hunk ONLY; commit `chore: update cargo lock for 0.3.0` holding ONLY Cargo.lock |
| Harness log header | `dev-checks starting; log file: logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` — RECORD the exact printed path |
| Harness check lines (fixed order) | `PASS fmt-check exit=0 (<n> s)`; `PASS check exit=0 (<n> s)`; `PASS test exit=0 (<n> s)`; `PASS clippy exit=0 (<n> s)` |
| Harness test block | `test result: ok. 50 passed; 0 failed; …` + doc-test line `test result: ok. 0 passed` (normal; not a second count) |
| Harness final | `ALL CHECKS PASSED`; **exit code 0** = the authoritative success evidence |
| Printf census identity | app 9 + folder_navigation 5 + folder_navigation_continuation 4 + floating_controls 1 + folder_listing 5 + sibling 8 + last_viewed 5 + name_collation 2 + image_loading 4 + app_display 2 + scaling 5 = 50 |
| Release build | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; OBSERVED-DLL line; **exit code 0** |
| Exe | NON-EMPTY gitignored `dist/windows-x64/img-scroller.exe` at the 0.3.0 build; exact size = the recorded as-built value (delta vs the 19,461,520-byte 0.2.1 baseline is evidence only; any non-empty size passes) |
| DLL audit | 22 lowercase DLL names, SET-EQUAL to the recorded constant (§6): unchanged OBSERVED baseline |
| Git sweep | only the Task-4 commits landed in this task present; nothing ignored staged |

## 5. Alpine-VM MCP usage notes (binding, per docs/build.md)

- `vm_status` BEFORE **every** `vm_run_command` call; only proceed when the VM is running and SSH-reachable; otherwise STOP and return a user question (docs/build.md precondition; global plan binding line).
- ONE command per MCP call; pipelines live INSIDE one quoted `sh -c "..."` string.
- The `exitCode` field is the authoritative pass/fail evidence; `null` exitCode + `timedOut: true` → report the timeout with the partial captured output; do not re-run silently.
- Absent cargo stderr in captured MCP output is NOT a failure signal (documented caveat).
- Generous timeouts (§3 table) beat aggressive ones.
- EBUSY-on-manifest-write (vboxsf share): documented pattern = retry the IDENTICAL command ONCE, then the authorized host-side-write fallback (citation: `.kilo/plans/20261031-task3-wave1-open-image-implementation.md` — "EBUSY failure handling: ... apply the CALLER-AUTHORIZED FALLBACK ... then run `cargo check` ONCE (NO `--locked`) to materialize the `Cargo.lock` entries"; echoed as "authorized host-side write fallback is the documented pattern" in the global and Wave-2 plan risk notes). In THIS task the fallback applies ONLY inside §2 L-FLOW.1, in the shape encoded there. No other EBUSY improvisation: elsewhere on the share, EBUSY → retry once → STOP + report.

## 6. DLL import audit — recorded constant and comparison evidence statement

Recorded OBSERVED_DLL_IMPORT_LIST (source of truth: `scripts/build-windows.sh` line 15, identical in `docs/build-windows.md` §Static-linking — audited 2026-10-31 with `x86_64-w64-mingw32-objdump`; docs-split rule: NOT duplicated into tech.md/architecture.md):

```
advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll,
api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll,
gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll,
opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll,
ws2_32.dll
```

Evidence statement to record in the 4.2 report (verbatim shape): "GW-3 audit: `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` inside the container; output deduplicated + lowercase-folded via the §3 step-6 pipeline; OBSERVED = 22 DLLs; SET-EQUAL to the recorded 2026-10-31 constant — **DLL import list UNCHANGED**."
On ANY diff (extra, missing, renamed): STOP + report the full observed list and the delta; NO edits to `scripts/build-windows.sh` or `docs/build-windows.md` — refreshing the OBSERVED baseline is a Planner/Task-5 decision, never an implementer improvisation (Wave-1/2 precedent).

## 7. Failure/intervention policy for 4.2 (decision branches; max iterations PINNED)

- **fmt-check FAIL** → sanctioned minimal self-service fix: ONE `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` (`timeoutMs: 120000`); host-side `git diff` MUST show formatting-only reflow (no semantic content moves); if changed files exist → commit EXACTLY `git add -u` limited to `src/` ref paths listed by `git status --short` (never `.agent/`, never `docs/`, never ignored trees) with message `style: apply cargo fmt` (separate commit, Wave-2 precedent); then re-run the harness ONCE. Max iterations: 2 fmt-fix → 2 harness runs [total 3 runs, including the initial 0.3.0 lock-check run count: one fmt repair round + one re-verify]. Second consecutive fmt failure → STOP + report.
- **check FAIL** → STOP immediately. Compile defects are structural; report the full captured output + the printed log path (no self-fix — run-once-then-report).
- **test FAIL** → STOP immediately. Any failing named test = structural (§1.2 census rule protects against ad-hoc test invention); report with the failure detail lines.
- **clippy FAIL** → minor-lint self-service is sanctioned ONLY when the failure is a SINGLE lint in a SINGLE file whose fix is ≤ 3 source lines and matches an already-recorded lint class (ptr_arg was the branch's one such risk — already fixed at `7657f32`; expect zero clippy residue). Fix with the `edit` tool, commit EXACTLY `fix: resolve minor clippy lint (verification round)` staged to that file only, then re-run the harness ONCE. Max 1 such fix round; the second consecutive clippy failure OR any multi-site/structural lint → STOP + report (structural → caller).
- Global cap: at most 2 post-fix harness re-runs in total across all sanctioned fix rounds; anything failing after the cap → STOP + report with evidence.
- Every fix round re-reads `.gitignore` before staging (gitignore-compliance rule) and re-runs `git status --short`; `.agent/project-info/context.md` is planner/caller territory — NEVER staged by the implementer.

## 8. Git plan — which commits this task creates

| When | Commit | Content | Message (EXACT) |
| --- | --- | --- | --- |
| Before dev-checks | ALWAYS (lock proven stale) | Cargo.lock ONLY (§2) | `chore: update cargo lock for 0.3.0` |
| Only if fmt drift | Separate | fmt-reflowed `src/` files only | `style: apply cargo fmt` |
| Only if minor clippy lint | Separate | the one ≤3-line fixed file | `fix: resolve minor clippy lint (verification round)` |

- NO commit for build artifacts: `dist/` is gitignored; `logs/checks/` logs are gitignored; NOTHING built is ever staged (`git status --short` sweep at §3 step 7; unstage anything ignore-matching if it appears).
- Handoff note for 4.6 (not executed by 4.2): the Wave-2/2-fix precedent commits the per-task plan file together with the `[DONE]` marks — 4.6's staging list for this task = this plan file (`.kilo/plans/20261101-wave3-task4-verification.md`) + the TODO file. 4.2 itself commits NOTHING except the three table rows above.
- NO `git push` (restricted to workflow Step 5: `main` → `origin` ONLY). NO amend/force. NO `git add -A`/`git add .` (explicit paths only).

## 9. Junior-implementer step-by-step checklist for 4.2

1. Host: `git branch --show-current` → MUST be `feat/wave-3-vertical-scroll`; `git status --porcelain` → MUST be empty (or ONLY `.agent/project-info/context.md` modified-unstaged — planner file, never staged); `git log --oneline -10` → HEAD `9c2c94a` + chain incl. `89e1db4`. Any mismatch → STOP + ask the caller.
2. Host: read `.gitignore` (sweep reference). Read this plan §4 (expected outcomes) so every comparison is pre-armed.
3. `vm_status` → must be running + SSH reachable; else STOP + user question.
4. §3 P1 LF guard (both scripts) → expect exit 0 twice; exit 1 → STOP + report.
5. §2 L-FLOW.1 → L-FLOW.2 → L-FLOW.3 (lock reconcile + verify + own commit). L-FLOW.2 extra hunks → STOP.
6. §3 step 3 harness run at the given timeout. Compare against §4 table line-by-line as output streams.
   - ALL four PASS + `ALL CHECKS PASSED` + exit 0 + test line `50 passed; 0 failed` → record the printed log path → go to 7.
   - `FAIL fmt-check` → §7 fmt branch (once max), then re-run once → 6 again.
   - `FAIL check` / `FAIL test` → STOP + report (§7).
   - `FAIL clippy` → §7 clippy branch (once max) → re-run once → 6 again.
   - Post-cap failures → STOP + report with the four result lines + log path.
7. `vm_status`; §3 step 4 release build → expect §4 build rows; `build-windows: error:` line of any type → STOP + report the typed error (docs/build-windows.md failure table is the diagnostic map).
8. `vm_status`; §3 step 5 `ls -l` → record the exact as-built size.
9. `vm_status`; §3 step 6 DLL audit → 22 lines set-equal to §6; diff → STOP + report (no script/doc edits).
10. Host: `git status --short` sweep; confirm only this task's commits landed; nothing ignored staged.
11. Report to caller (\S0–§8 evidence set): branch/HEAD; lock commit message+`git show --stat`; harness log path; the four PASS lines; `ALL CHECKS PASSED` + exit 0; the `50 passed` line; `build-windows: OK`/artifact/exit 0; as-built exe size; the 22-DLL audit evidence statement; git sweep result; **and the explicit note that the OPERATOR-RUN sub-item remains PENDING** (user-side, next section).

## 10. OPERATOR-RUN handoff (Task 4 bullet 3 — recorded as PENDING; text placed at 4.6/wave close, VERBATIM)

```
Wave-3 manual check (OPERATOR-RUN) — run dist\windows-x64\img-scroller.exe (the freshly rebuilt 0.3.0 exe) on a real Windows 10/11 machine:
1. Prepare: a parent folder containing exactly three child folders — folder1 with img1.png, folder2 with img2.png,
   folder3 with img3.png (child folders may hold several images each; add folder3b.png etc. for step 6).
2. START: launch the exe and open folder2\img2.png.
3. SAME-PARENT MOVE: press the UP arrow key AND click the pill's up button — both must go to the PREVIOUS sibling
   folder (folder1, showing its image); press the DOWN arrow key AND the down button — both must go to the NEXT
   sibling folder (folder3). The viewer never leaves the same parent (no grandparent folder is ever offered).
4. LAST-VIEWED MEMORY: from folder2 move to folder1 and open its OTHER image (folder1b.png), then move to folder3
   and back to folder1 — it must reopen folder1b.png (the folder's LAST VIEWED image this session), not its first.
5. FIRST-IMAGE RULE: continue to a folder NOT visited this session — it must open that folder's FIRST image in
   alphanumeric order; no same-named file attempt happens.
6. SORT ORDER: travel order follows the alphanumeric folder order (lowercase-folded, case tie-break as Wave 2).
7. EDGES: at the FIRST/LAST sibling folder the corresponding arrow shows disabled and the key does nothing.
8. EMPTY FOLDER: if a prepared sibling folder holds no image, entering it clears the canvas to "Empty folder" and
   going up/down from there still walks the sibling list (record what you see — important evidence).
9. SESSION LIFETIME: CLOSE the app and relaunch on the same folder — the previous session's last-viewed memory is
   GONE: each folder opens its FIRST image again (no persistence in v1).
10. REGRESSION: Left/Right keys and pill buttons still navigate images inside a folder exactly as in 0.2.1.
Report results back into the project (this closes the manual-UX sub-item).
```

Result handling: recorded by the Planner at 4.6/wave close; the Task-4 manual-UX sub-item STAYS `[ ]` in the TODO until the user reports (binding TODO note; Wave-1/2 precedent).

## 11. Deliberately out of scope (what 4.2 must NOT do)

- NO code-feature changes anywhere (no navigation/listing/memory/platform/display edits) — Task-4 changes nothing structural; fixes are bounded by §7.
- NO new tests (gap verdict §1.1: none; census fixed at 50), NO test additions/renames "while here".
- NO docs edits (Task 5 owns README/tech/architecture/structure/context; NO OBSERVED-DLL doc refresh here — a changed list is reported, not recorded, per §6).
- NO `CHANGELOG.md` touches (wave binding). NO TODO-file edits (4.6 marks `[x]`; the OPERATOR-RUN sub-item stays `[ ]` even then). NO edits to pre-existing `.kilo/plans/*` files, including this plan once it lands.
- NO version changes (0.3.0 landed at Step 3/Task-4 lock commit carries only the lock).
- NO UI edits; NO pan/zoom/fullscreen/OpenWith; NO x86; NO same-filename-rule experiments.
- NO `git push`, NO branch creation/switch (Step 2/Step 5 territory). NO `FORCE_FAIL` invocations. NEVER stage `dist/**`, `logs/**`, `target/**`, `Cargo.*` beyond §8's lock commit, or `.agent/project-info/context.md`.

## 12. Stop conditions → return a user question

- `vm_status` not running / `/img-scroller` share unavailable at any VM-gated step.
- L-FLOW.2 diff shows any hunk beyond the version line; lock reconcile fails twice (EBUSY included).
- Any harness `FAIL` line or non-0 exit beyond §7's sanctioned fix rounds; census ≠ `50 passed`.
- P1 LF guard exit 1 (CRLF recurrence — class outside this task).
- Any `build-windows: error:` typed line (report it verbatim; docs table is the diagnostic map).
- DLL audit ≠ 22 OBSERVED (report the delta; never record a new baseline).
- Any out-of-scope temptation in §11 arises again → STOP; the caller decides.
