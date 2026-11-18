# Wave 8 — Task 5 Verification Plan: Dev-Checks at Wave HEAD, Fresh Installer/Zip Build Chain, DLL Import Audits, Typed-Failure Stance, OPERATOR-RUN Checklist

- Source TODO: `.agent/todos/20261117/20261117-todo-1.md` → `### Task 5: Verification`. Verbatim sub-items: (1) "Unit tests for the new pure logic (title composition/truncation; installer/association-selection pure logic where any exists) — run via the dev-checks harness in the container per `docs/build.md` (`ALL CHECKS PASSED`: fmt/check/test/clippy; `alpine-vm` MCP; `vm_status` first; the command exit code is the authoritative success evidence)."; (2) "The installer build (and the zip build if retained) via the decided mechanism — each command exits `0` and leaves a non-empty artifact; run through the `alpine-vm` MCP (`vm_status` first; exit code authoritative)."; (3) "Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): run the wizard installer → install to a chosen destination; the association page lets the user pick types (e.g. only `.png`) and the registration lands correctly (Open With lists; Settings confirmation); the app launches from the install location and the Waves-1–7 behaviors still work from there; the window title behaves per Task 4 (full path / name / truncation / not-maximized rule / fullscreen 3-s popup); the uninstaller removes files, shortcuts, and associations; the GitHub release is visible and its artifacts download."
- Global plan (approved): `.kilo/plans/20261118-wave8-wizard-installer-global-plan.md` → `## Per-Task Pre-Analysis` → `### Task 5` (NOT edited; this is a NEW file).
- Task plans defining the verification targets: `.kilo/plans/20261118-wave8-task1-wizard-installer-implementation-plan.md` (D1–D19: NSIS technology, `scripts/build-installer.sh`, `dist/ImagesScroller-<version>-windows-installer.exe`, x64-payload-only, zips RETAINED per D12, no-Rust-logic census pin D18, build-order D16) + `20261118-wave8-task1-installer-simplification-plan.md` (S1–S3, behavior-preserving); `.kilo/plans/20261118-wave8-task4-window-title-implementation-plan.md` (D1–D16; the 18-test matrix; census §7) + the 4.3 review-fix (`20261118-wave8-task4-window-title-review-fix.md`) and simplification (`20261118-wave8-task4-window-title-simplification-plan.md`) records.
- Structure precedent followed verbatim: `.kilo/plans/20261114-wave7-task4-verification-plan.md` (formal runs, expected evidence, typed failure handling, §8 PLAIN self-contained OPERATOR-RUN checklist, results-file outline).
- Date: 2026-11-18. Status: FOR PLANNER PRESENTATION (pending user approval before 4.2 runs).
- Branch: `feat/wave-8-wizard-installer`, HEAD `8df8894` at planning time (implementer re-verifies with `git log --oneline -1` at Step 0); Tasks 1–4 `[DONE]`; version **1.1.0**. Front-end related: NO (global-plan flag — no 4.1a, no 4.5a).
- High-level approach: this task RUNS the formal agent-side verification — one dev-checks harness run at wave HEAD with the census corroboration (213/29), then ONE fresh artifact chain (x64 release build → x86 release build → installer build → zip packaging) so EVERY released payload is built from wave-HEAD source, then BOTH DLL import audits re-run on the fresh artifacts, everything recorded in a NEW results file, and the user receives a PLAIN self-contained OPERATOR-RUN checklist (§8). NO source change, NO script edit, NO new test belongs to Task 5.

---

## 0. Verification Design Decisions (BINDING — the implementer must not re-choose)

### V1 — Dev-checks run: the formal wave-HEAD run; census stays 213/29; NO tests added by Task 5

- The formal run is the TODO sub-item 1 evidence: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — expected exit 0; one `PASS <name> exit=0 (<n> s)` line for each of `fmt-check`, `check`, `test`, `clippy`; final line `ALL CHECKS PASSED` (semantics per `docs/build.md`; the printed `logs/checks/dev-checks-<UTC>.log` path is captured for the results file).
- Census statement (explicit, pinned): **Task 5 adds NO tests and NO `src/` change.** The TODO's "unit tests for the new pure logic" sub-item is discharged by the formal wave-HEAD harness run EXERCISING the tests Task 4 already landed: the 18-test composition matrix in `src/window_title/tests.rs` (full-path/name/last-chars fitting, the containing-folder/drive/parent chain, the `...` marker, the constants — the Task-4 delta: 195 → 213 tests, 28 → 29 test-bearing files). The installer/association-selection logic produced **NO Rust pure logic**: the wizard UI, the association page, and the registry writes are the NSIS script `packaging/nsis/installer.nsi` — installer-runtime NSIS instructions executed on Windows, not Rust, not compilable or testable by the dev-checks harness (Task-1 D18 pinned this; zero Rust lines exist for it and zero tests are added).
- Census corroboration (secondary evidence, both legs pinned exactly like the Wave-7 precedent):
  1. The harness output's final cargo `test` summary reads `test result: ok. 213 passed; 0 failed;`.
  2. A container grep counts the test-bearing files: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rl '#\[test\]' src | wc -l"` → expected output exactly `29`.
- Breakdown for the results file (informational): 29 = 19 `*_tests.rs` + 5 `tests.rs` child files (the Wave-8 delta is `src/window_title/tests.rs`) + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`).
- A mismatch vs EITHER number, any `FAIL` line, or a missing `ALL CHECKS PASSED` is a census/harness mismatch → STOP (V7). The pre-existing benign clippy finding (the `compare_entry_names` dead-code warning, recorded since Wave 5.2) may appear; any NEW fmt/clippy finding → STOP.

### V2 — The fresh artifact chain: x64 build → x86 build → installer build → zip packaging; the runs ARE the evidence

Per the TODO sub-item 2 and Task-1 D16's ordering, the formal evidence is EXACTLY this four-command chain (ONE command per call; the exit code is authoritative; each later step consumes the earlier steps' fresh artifacts):

1. **x64 release build** — `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu` → expected exit 0; output contains `build-windows: building target: x86_64-pc-windows-gnu`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, the x64 audit line `build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (dist/windows-x64): advapi32.dll, ... ws2_32.dll` (the recorded constant printed by the script), `build-windows: OK`.
2. **x86 release build** — `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu` → expected exit 0; output contains `build-windows: building target: i686-pc-windows-gnu`, `build-windows: artifact: dist/windows-i686/img-scroller.exe`, the x86 audit line printing `OBSERVED_DLL_IMPORT_LIST_I686` (the recorded constant), `build-windows: OK`.
3. **Installer build** — `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh` → expected exit 0; output contains `build-installer: artifact: dist/ImagesScroller-1.1.0-windows-installer.exe` and `build-installer: OK`. `makensis` warnings are NON-FATAL by design (Task-1 D19): exit 0 + a non-empty artifact is the success evidence; warning 9100 (no `LegalCopyright`) is the documented benign warning if it appears. If the artifact name is NOT exactly `dist/ImagesScroller-1.1.0-windows-installer.exe` → version drift → STOP.
4. **Zip packaging (RETAINED per Task-1 D12)** — `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh` → expected exit 0; output contains `package-windows: zip: dist/ImagesScroller-1.1.0-windows-x64.zip`, `package-windows: zip: dist/ImagesScroller-1.1.0-windows-x86.zip`, `package-windows: OK`.
- A clean-state rebuild is NOT required — the runs are incremental (`--locked`; the compiled dependency tree is reused) and each run IS its own formal evidence.
- The Task-1-era artifacts (the installer at 6,836,150 B and both 1.1.0 zips rebuilt during Task 1's step 6) are SUPERSEDED as the FORMAL record by this fresh chain; the fresh runs overwrite the same paths atomically (`.part`-then-rename — a failed run leaves the previous artifact intact).

### V3 — Artifact-freshness decisions (settled here — both rebuilds REQUIRED)

- **x64 build re-run at wave HEAD: REQUIRED.** The installer embeds the x64 exe (`-DEXE_PAYLOAD`), and both the installer and the x64 zip are Task-7 release artifacts; the payload must be compiled from wave-HEAD source (Task 4 changed `src/`: the `window_title` modules). This also supplies the windows-target compile evidence for the wave (dev-checks compiles the stub path only; the x64+x86 release builds compile the REAL windows path at wave HEAD — no separate cross-compile check is needed).
- **x86 build re-run at wave HEAD: REQUIRED (decided YES).** The x86 exe exists ONLY inside the x86 zip (Task-1 D4 — no x86 installer). Task 1's build matrix ran the x64 build ONLY (its step 6.1), so the x86 exe inside the Task-1-era 1.1.0 x86 zip has UNVERIFIED provenance — it may still be the 1.0.0 binary, i.e. missing the Task-4 window-title behavior and the version-bump-era build. The freshness requirement settles it: both zip payloads must be wave-HEAD exes. The fresh x86 build removes all doubt and re-serves the formal x86 exit-0 evidence with the wave's source.
- **Pinned command order: x64 build → x86 build → installer build → zip packaging** (V2 items 1–4). Rationale: the installer consumes the fresh x64 exe; the zips consume BOTH fresh exes. Do NOT reorder, do NOT skip the x86 build, do NOT use a cached/old artifact for any step.

### V4 — The DLL import audits: BOTH re-run as per-build evidence; SET-EQUAL 22 required; NO script fill

Wave-1 discipline: the audit is OBSERVED per-build evidence, never an assumption — and this wave's `src/` changed (the `window_title` modules, std/egui APIs only), so Task 5 re-runs BOTH audits on the freshly rebuilt artifacts.

- **x64 audit** — tool `x86_64-w64-mingw32-objdump` (in the image), path `dist/windows-x64/img-scroller.exe`.
- **x86 audit** — tool `i686-w64-mingw32-objdump` (in the image), path `dist/windows-i686/img-scroller.exe`.
- **Extraction mechanism (pinned, per audit, TWO commands):**
  1. LIST: `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u` (for the x86 audit: the same pipeline with `i686-w64-mingw32-objdump` and `dist/windows-i686/img-scroller.exe`). Output: the imported-DLL names, one per line, lowercase, deduplicated, sorted.
  2. COUNT: the same pipeline with `| wc -l` appended — expected output exactly `22` (the count guard: an unreadable/empty artifact would list nothing and count 0 ≠ 22; the pipeline's exit code is authoritative for the chain).
- **Expected set — the recorded 22-DLL constant (BOTH `OBSERVED_DLL_IMPORT_LIST` and `OBSERVED_DLL_IMPORT_LIST_I686` in `scripts/build-windows.sh`; the implementer verifies item-by-item, set-equal, order-independent):**
  1. advapi32.dll
  2. api-ms-win-core-path-l1-1-0.dll
  3. api-ms-win-core-synch-l1-2-0.dll
  4. api-ms-win-core-winrt-error-l1-1-0.dll
  5. bcryptprimitives.dll
  6. combase.dll
  7. dwmapi.dll
  8. gdi32.dll
  9. imm32.dll
  10. kernel32.dll
  11. msvcrt.dll
  12. ntdll.dll
  13. ole32.dll
  14. oleaut32.dll
  15. opengl32.dll
  16. rpcrt4.dll
  17. shell32.dll
  18. shlwapi.dll
  19. user32.dll
  20. userenv.dll
  21. uxtheme.dll
  22. ws2_32.dll
- **Adjudication rule (BOTH audits, identical):** the OBSERVED list must be SET-EQUAL to the 22 names above. ANY delta — a NEW name (especially any MinGW runtime DLL `libgcc_s_dw2.dll` / `libstdc++-6.dll` / `libwinpthread-1.dll`) OR a MISSING name — is unexpected → STOP and report to the caller. **NO script fill happens in Task 5** (unlike Wave-7 Task 4): both `OBSERVED_DLL_IMPORT_LIST` constants already exist and the scripts already print them; a delta means the binaries changed imports, which is a deviation to adjudicate, not a list to re-record. The observed lists are recorded verbatim in the results file only.

### V5 — Typed-failure expectations: NO intentionally-expected failures exist in Task 5

- Task 1's smoke probe already exercised the missing-payload typed failure (`build-installer: error: expected payload not found or empty: ...`); NO FORCE_FAIL injection, NO bogus-target check, NO probe of any failure path is run here. Therefore: **ANY nonzero exit on any command of §3 → STOP and report to the caller.** A typed `build-windows: error:` / `build-installer: error:` / `package-windows: error:` line → STOP and report. A census mismatch → STOP. A pinned artifact-name mismatch (anything other than `dist/ImagesScroller-1.1.0-windows-installer.exe` / `dist/ImagesScroller-1.1.0-windows-x64.zip` / `dist/ImagesScroller-1.1.0-windows-x86.zip`) → STOP (version drift — versions are Step-3-owned).
- Absent cargo stderr in captured MCP output is NOT a failure signal (`docs/build.md`) — the exit code is authoritative.

### V6 — OPERATOR-RUN checklist: written PLAIN and SELF-CONTAINED (§8) — the user's gate

- The manual-UX sub-item is agent-impossible (GUI/runtime on real Windows); the checklist in §8 is handed to the caller for the user. The TODO's manual-UX sub-item STAYS `[ ]` until the user reports the run (standing precedent from Waves 4.1–7). The checklist covers EXACTLY what the TODO's third bullet requires: the wizard installer → a chosen destination; the association page picks types (e.g. only `.png`) and the registration lands (Open With lists; the Settings confirmation); the app launches from the install location; the Waves-1–7 behaviors still work from there; the window title per Task 4 (full path / name / truncation / not-maximized rule / fullscreen 3-s popup — with the concrete composition expectations of Task-4's D4–D10: `\`-joined segments, the literal `...` marker, containing-folder-then-drive priority, parents walking up nearest-first, a truncated name consuming the width); the uninstaller removes files, shortcuts, and associations; the GitHub release is visible and its artifacts download.
- **Release-visibility gating (explicit):** the wave's Task-7 (GitHub Releases publishing) comes AFTER Task 5/6 — so §8's GitHub-release item is marked in the checklist as runnable ONLY after Task 7 completes. The user runs the rest of the checklist now; that ONE item is reported "pending Task 7" until the release exists, and it closes with the Task-7 gate. This ordering is recorded in the results file (V8) and in §7's mark semantics.

### V7 — Failure / adjudication rules (consolidated)

- NO intentionally-expected typed failures exist in Task 5 (V5). ANY nonzero exit on any §3 command → STOP and report.
- Census mismatch (not `ALL CHECKS PASSED`, or the test count ≠ 213, or the file count ≠ 29) → STOP and report (V1).
- ANY DLL-audit delta (new OR missing name, either architecture) → STOP and report (V4 — no re-fill, no self-adjudicated "explained delta" in this task).
- Pinned artifact-name mismatch → STOP and report (V5).
- VM or `/img-scroller` share unavailable → STOP, return a user question (wave execution note).
- The OPERATOR-RUN outcome (any §8 item FAIL) → the wave is NOT closed; the caller decides the fix path.

### V8 — Results file: YES (the Wave-6.x/7 precedent) — PINNED NAME

- A NEW file `.kilo/plans/20261118-wave8-task5-verification-results.md` is produced by the implementer (creation authorized; the caller pinned this exact name). Contents (§3 Step 9 pins the outline): the six evidence blocks (dev-checks + census; x64 build; x86 build; installer build; zip packaging + unzip/CRLF checks; both audit lists verbatim + adjudication), the artifact byte-size record, the census statement (213/29, no tests added — the V1 statement), the pending OPERATOR-RUN pointer (§8 of this plan), the Task-7 gating note (the release-visibility item is reported "pending Task 7" until the release exists), and the verdict line.
- Commit (verbatim message): `docs: add wave8 task5 verification results` — stages ONLY the results file.

---

## 1. Environment Facts (verified 2026-11-18)

- Git: on branch `feat/wave-8-wizard-installer`; HEAD `8df8894` at planning time (caller-verified; the implementer re-verifies at Step 0); Tasks 1–4 all `[DONE]`; version 1.1.0 (bumped at `4fb6b94`, Step 3; `Cargo.toml`/`Cargo.lock` are NEVER touched by this task).
- Census NOW: **213 unit tests / 29 test-bearing files** (the Wave-8 Task-4 delta: +18 `window_title` tests in the NEW `src/window_title/tests.rs`; +1 test file). Corroboration history: the simplification-plan baseline verified 213/29 green at HEAD `1234b45`; the review-fix record confirms the observed total 213 (= 195 + 18 — the implementation plan §7's "212" was its own arithmetic slip, already recorded in both 4.3 files); dev-checks `ALL CHECKS PASSED` exit 0 at the 4.3-fix point (`bbe3924`) and again through `62a0ec7` per the 4.5b corroboration.
- Scripts on disk (read during planning): `scripts/build-windows.sh` (121 lines; BOTH `OBSERVED_DLL_IMPORT_LIST` and `OBSERVED_DLL_IMPORT_LIST_I686` constants at lines 21–22, both the SAME 22-name set; per-target audit-note printing at `print_audit_note_for_target`; typed failures; atomic `.part` install; `--locked`); `scripts/build-installer.sh` (98 lines; version extraction from `Cargo.toml` CR-tolerant; `makensis -V2` with six `-D` defines; typed `build-installer: error:` failures; `.part`-then-rename); `scripts/package-windows.sh` (107 lines; the 8-line `package_manifest_entries` manifest — 4 flat entries per architecture; EXIT-trap staging cleanup).
- Alpine VM: RUNNING, SSH-reachable per the wave's standing note (the implementer re-verifies via `vm_status` FIRST at every working session). All runs go through the `alpine-vm` MCP (`vm_status` FIRST; ONE command per call; the exit code is authoritative). If the VM or the `/img-scroller` share is unavailable: STOP, return a user question.
- Artifacts on disk (planning-time, from the caller's verified facts): `dist/ImagesScroller-1.1.0-windows-installer.exe` = 6,836,150 B at the Task-1 4.3-fix rebuild; both 1.1.0 zips rebuilt during Task 1 (the x86 zip's embedded exe has UNVERIFIED provenance — see V3 — which the fresh chain supersedes). The V2 chain overwrites all four artifact paths atomically; sizes are re-recorded after (a byte-size difference vs the planning-time value is NOT a failure — non-empty is the gate; sizes are evidence only).
- `docs/build.md` harness semantics unchanged (fmt-check/check/test/clippy; logs under gitignored `logs/checks/`); `docs/build-windows.md` documents the x64/x86/installer/zip flows and the 22-DLL observed lists (NOT touched by Task 5 — Task 6 owns prose).

## 2. Binding Constraints (apply to every step)

- ALL container runs via the `alpine-vm` MCP: `vm_status` FIRST (re-check before the first command of each working session); ONE command per call; the command exit code is the authoritative success evidence; absent cargo stderr is NOT a failure signal (`docs/build.md`).
- NO `git push` (restricted to Step 5). NO branch creation/switch (restricted to Step 2). NO version edit (`Cargo.toml`/`Cargo.lock` NEVER touched by this task).
- Gitignore Compliance Rule before EVERY commit: read `.gitignore`, run `git status`, ensure NO `dist/`, `logs/`, `target/`, or `*.log` path is staged (the installer, zips, exes, staging tree, and checks logs are all gitignored — never staged). NEVER stage any pre-existing unstaged ride-along file (e.g. `.agent/project-info/context.md` entries) — this task's commits stage ONLY their pinned files.
- Do NOT edit any existing `.kilo/plans/*` file. The NEW files of this task are EXACTLY: this plan file + the results file `.kilo/plans/20261118-wave8-task5-verification-results.md`.
- Do NOT edit `README.md`, `docs/*`, `.agent/project-info/{brief,product,tech,architecture,context}.md`, `CHANGELOG.md`, ANY file under `src/`, `Cargo.toml`, `Cargo.lock`, `Dockerfile`, `docker-compose.yml`, ANY `scripts/*` file, `packaging/*` (including `packaging/nsis/installer.nsi`), `.gitattributes`, or `.agent/project-structure.md` — no source/build-harness/docs change belongs to Task 5; Task 6 owns the wave's prose. The ONLY `.agent/` edit of this task is the 4.6 TODO mark (§7).
- File edits use real newline characters (never literal `\n` escape sequences).
- Max-lines rules: no code file is touched, so no cap applies; the results file is a plan-family markdown file (no cap).
- Sub-step mapping (the caller's assignment): 4.2 implementer runs V1–V4 and commits the results record; 4.3 reviewer+simplifier; 4.4 docs-specialist (results-record review ONLY); 4.5b architector adjudication; 4.6 implementer mark (agent-side sub-items `[x]`; the manual-UX sub-item STAYS `[ ]`).

## 3. Step-by-Step Implementation (workflow step 4.2 — implementer, JUNIOR under 50%)

Execute the steps in order. Check the plan between steps. Stop and ask the caller on ANY ambiguity — never guess. Every `alpine-vm_vm_run_command` call is ONE command; the exit code is authoritative; apply the pinned timeouts.

### Step 0 — Preconditions + commit the plan file (BEFORE implementation)

1. `git status --short --branch` → expect `## feat/wave-8-wizard-installer...` and `git log --oneline -1` → expect HEAD at/after `8df8894` (Tasks 1–4 landed). If the branch or the Task-1..4 marks are not in place → STOP, return a user question.
2. Read `.gitignore`; confirm nothing gitignored is about to be staged (`dist/`, `logs/`, `target/`, `*.log` are ignored — artifacts and check logs are NEVER staged).
3. Stage ONLY the plan file: `git add .kilo/plans/20261118-wave8-task5-verification-plan.md`
4. Commit (verbatim message): `docs: add wave8 task5 verification plan`

### Step 1 — vm_status + the formal dev-checks run + census corroboration (V1)

- Call `vm_status` FIRST. If not running/SSH-unreachable: STOP, return a user question.
- Run the harness (ONE command, timeout 900000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
  - Expected: exit 0; one `PASS <name> exit=0` line for each of `fmt-check`, `check`, `test`, `clippy`; final line `ALL CHECKS PASSED`; the `test` section's final cargo summary contains `test result: ok. 213 passed; 0 failed;`. Capture the printed log path (`logs/checks/dev-checks-<...>.log`) for the results file. The pre-existing benign `compare_entry_names` dead-code clippy warning may appear.
  - Any other outcome → STOP (V7).
- Census file-count corroboration (ONE command, timeout 120000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rl '#\[test\]' src | wc -l"`
  - Expected: exit 0; output exactly `29`. Any other number → census mismatch → STOP (V7).

### Step 2 — x64 single-target release build (fresh at wave HEAD — V2.1/V3)

- `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu` (timeout 3600000 ms).
- Expected: exit 0; output contains `build-windows: building target: x86_64-pc-windows-gnu`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, the audit line `build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (dist/windows-x64): advapi32.dll, ... ws2_32.dll`, `build-windows: OK`.
- Any typed error or nonzero exit → STOP (V7).

### Step 3 — x86 single-target release build (fresh at wave HEAD — V2.2/V3)

- `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu` (timeout 3600000 ms).
- Expected: exit 0; output contains `build-windows: building target: i686-pc-windows-gnu`, `build-windows: artifact: dist/windows-i686/img-scroller.exe`, the x86 audit line `build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (dist/windows-i686): advapi32.dll, ... ws2_32.dll`, `build-windows: OK`.
- Any typed error or nonzero exit → STOP (V7).

### Step 4 — x64 DLL re-audit (on the fresh artifact — V4)

- LIST (ONE command, timeout 180000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u`
  - Expected: exit 0; EXACTLY the 22 names of V4 (one per line, lowercase, sorted).
- COUNT (ONE command, timeout 180000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u | wc -l`
  - Expected: exit 0; output exactly `22`.
- Copy the observed list verbatim for the results file. ANY delta vs V4's list → STOP and report (V4/V7 — NO script fill, NO self-adjudicated delta).

### Step 5 — x86 DLL re-audit (on the fresh artifact — V4)

- LIST (ONE command, timeout 180000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust i686-w64-mingw32-objdump -x dist/windows-i686/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u`
- COUNT (ONE command, timeout 180000 ms): the same pipeline with `| wc -l` appended — expected exactly `22`.
- Adjudication: set-equal to V4's 22 names → proceed to Step 6. ANY delta (new OR missing, esp. a MinGW runtime DLL) → STOP and report to the caller.

### Step 6 — Installer build (V2.3)

- `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh` (timeout 600000 ms).
- Expected: exit 0; output contains `build-installer: artifact: dist/ImagesScroller-1.1.0-windows-installer.exe` and `build-installer: OK`. `makensis` warnings (incl. warning 9100, no `LegalCopyright`) are NON-FATAL (Task-1 D19) — do not treat any warning line as failure; exit 0 + the typed lines + a non-empty artifact is the success evidence.
- If the artifact name is NOT exactly `dist/ImagesScroller-1.1.0-windows-installer.exe` → version drift → STOP (V5). Any typed `build-installer: error:` or nonzero exit → STOP (V7).

### Step 7 — Zip packaging re-run + zip checks + artifact record (V2.4 + the Wave-7 D5 check set)

- Run each as its OWN command; exit code authoritative:
  1. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh` (timeout 600000 ms) — expected exit 0 with `package-windows: zip: dist/ImagesScroller-1.1.0-windows-x64.zip`, `package-windows: zip: dist/ImagesScroller-1.1.0-windows-x86.zip`, `package-windows: OK`.
  2. `unzip -l /img-scroller/dist/ImagesScroller-1.1.0-windows-x64.zip` (timeout 120000 ms) — expected exit 0; EXACTLY 4 flat entries: `img-scroller.exe`, `img-scroller.toml`, `register-default-viewer.cmd`, `DEFAULT-VIEWER-GUIDE.txt` (no directory entries).
  3. `unzip -l /img-scroller/dist/ImagesScroller-1.1.0-windows-x86.zip` (timeout 120000 ms) — expected exit 0; the same EXACTLY 4 flat entries.
  4. Packaged-.cmd CRLF evidence, both zips (the Wave-7 precedent): `sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.1.0-windows-x64.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""` (timeout 120000 ms) — expected exit 0 (the packaged helper copy carries CRLF); the same command against the x86 zip → exit 0.
  5. Artifact record: `ls -l /img-scroller/dist/windows-x64/img-scroller.exe /img-scroller/dist/windows-i686/img-scroller.exe /img-scroller/dist/ImagesScroller-1.1.0-windows-installer.exe /img-scroller/dist/ImagesScroller-1.1.0-windows-x64.zip /img-scroller/dist/ImagesScroller-1.1.0-windows-x86.zip` (timeout 120000 ms) — expected exit 0; all five non-empty; record the byte sizes verbatim for the results file.
- Any typed `package-windows: error:` or nonzero exit → STOP (V7).

### Step 8 — Create the results file (V8)

- Create `.kilo/plans/20261118-wave8-task5-verification-results.md` (NEW file) with EXACTLY these sections:
  1. Header: source TODO Task 5, this plan file's path, date, branch/HEAD commit, version 1.1.0.
  2. Dev-checks evidence: the command, exit code, the four `PASS` lines, the `ALL CHECKS PASSED` line, the log path, and the census corroboration (`213 passed; 0 failed`; file count `29`).
  3. The census statement (V1 verbatim substance): Task 5 adds NO tests and NO `src/` change; the TODO's unit-tests sub-item is discharged by the wave-HEAD run exercising Task 4's landed 18 `window_title` tests (195 → 213, 28 → 29); the installer/association-selection logic is NSIS script code (installer-runtime, not Rust, not harness-testable) — zero Rust pure logic exists for it, hence zero tests.
  4. Build evidence: the four chain commands (x64, x86, installer, packaging) with exit codes and the pinned output lines observed; the V3 freshness decisions recorded (fresh x64 + fresh x86 REQUIRED; the Task-1-era artifacts superseded as the formal record).
  5. Audit evidence: BOTH observed lists verbatim (x64, x86) with counts, the set-equal verdict, and the adjudication (V4).
  6. Packaging evidence: the packaging exit 0 + the three output lines, both `unzip -l` listings (4 flat entries each), both packaged-CRLF exits 0, the five artifact byte sizes.
  7. The pending OPERATOR-RUN gate: the manual-UX sub-item stays `[ ]` until the user reports; the checklist lives at this plan's §8; the GitHub-release item is gated on Task 7 (reported "pending Task 7" until the release exists).
  8. Verdict line: `VERIFIED` (agent-side evidence complete) or the STOP reason if any step halted.

### Step 9 — Gitignore compliance + the results-file commit

- `git status` → confirm NO gitignored path is staged (the installer/zips/exes/staging/checks-logs are under `dist/`+`logs/` — unstaged; any pre-existing unstaged ride-along file stays unstaged).
- Commit (verbatim message): `docs: add wave8 task5 verification results` — stages EXACTLY `git add .kilo/plans/20261118-wave8-task5-verification-results.md`.
- Post-commit `git status`: clean apart from pre-existing unstaged ride-along content (if any).

### Explicitly NOT done in Task 5 (for the implementer)

- NO tests added, NO `src/` change, NO `Cargo.toml`/`Cargo.lock` edit (census stays 213/29). NO clean-state rebuild, NO no-arg both-targets build re-run, NO typed-failure probe (Task 1's smoke evidence; V5). NO script edit of ANY kind (the OBSERVED constants exist; V4 forbids a re-fill). NO README/`docs/`/project-info/CHANGELOG prose edits (Task 6). NO execution of the installer or any `.cmd` anywhere (agent-impossible; §8 is the user's run). NO edits to any existing `.kilo/plans/*` file. NO `git push`; NO branch/version actions. The 4.6 TODO marking is §7 (not §3).

## 4. Workflow step 4.3 — Code Review & Simplification guidance

- code-reviewer checklist (deviation from plan = finding): the results file contains ALL eight sections with the real evidence (exit codes, output lines, byte sizes, BOTH audit lists, census numbers 213/29); the four chain commands ran in the PINNED order (x64 → x86 → installer → packaging) through the `alpine-vm` MCP with `vm_status` first; the two commits contain ONLY their pinned files (plan / results); NO file under `src/`, `Cargo.toml`, `Cargo.lock`, `scripts/*`, `packaging/*`, `docs/`, `.agent/project-info/*`, `README.md`, `CHANGELOG.md`, or `.gitattributes` changed vs the Step-0 baseline; no gitignored path committed; the TODO mark (§7) touches ONLY the Task-5 heading + sub-items 1–2.
- code-simplifier: expected outcome NONE — Task 5 changes NO code (verification-only; the only artifacts are two markdown files). Any proposal to change code in Task 5 is itself a scope deviation.
- Max 3 review cycles, then escalate (workflow rule).

## 5. Workflow step 4.4 — Documentation guidance

- The docs-specialist performs a RESULTS-RECORD REVIEW ONLY (the caller's assignment): read `.kilo/plans/20261118-wave8-task5-verification-results.md` for internal coherence (the evidence sections match the plan's Step-8 outline; the numbers agree across sections; the OPERATOR-RUN gate and the Task-7 gating note are present). Creating or editing ANY documentation file in Task 4/5 (README, docs/, project-info, CHANGELOG) is a plan deviation — ALL wave prose is Task 6's deliverable. Expected outcome: "results record coherent; nothing to document; deferred to Task 6" as a clear message.

## 6. Workflow step 4.5b — Plan-adherence guidance

- Verify §0–§3 against the landed evidence: the Step-1 dev-checks output (`ALL CHECKS PASSED`, `213 passed; 0 failed`, census 29), the four-build chain (exit 0 each + non-empty artifacts + the pinned order), BOTH audit lists (verbatim, set-equal 22, no script fill), the packaging re-run + zip/CRLF/size evidence, the results file (V8 outline, all real values), the two commits (pinned files + verbatim messages), gitignore compliance before each.
- If every element conforms: return a clear "no diffs, VERIFIED" message (no report file needed). If a deviation exists: record it in a NEW plan file `.kilo/plans/20261118-wave8-task5-adherence.md` and propose the disposition — never edit existing plans.

## 7. Workflow step 4.6 — Task completion (implementer)

- Edit `.agent/todos/20261117/20261117-todo-1.md` (preserve ALL other content byte-identical):
  - `### Task 5: Verification` → `### Task 5: Verification [DONE]`
  - Sub-item 1 (unit tests / dev-checks) `- [ ]` → `- [x]`, rest of the line verbatim.
  - Sub-item 2 (installer build + zip build) `- [ ]` → `- [x]`, rest of the line verbatim.
  - Sub-item 3 (manual UX / OPERATOR-RUN) STAYS `- [ ]` — the user's gate (standing precedent; it closes when the user reports the §8 run; its GitHub-release item additionally waits for Task 7).
- Gitignore compliance before the commit (`git status`; the TODO file is tracked — safe).
- Commit (verbatim message): `task: mark wave8 task5 done`

## 8. OPERATOR-RUN Checklist (PLAIN, SELF-CONTAINED — hand verbatim to the user)

Run this on your real Windows 10/11 (64-bit) machine. You need the freshly built installer: `C:\repo\img-scroller\dist\ImagesScroller-1.1.0-windows-installer.exe` (produced by the verification build step; if it is missing, tell the session — do NOT use an old copy). Before you start, prepare a small test setup: one folder with a few images — make at least one of them a `.png` and give one image a LONG file name (longer than ~30 characters) — and a SECOND sibling folder next to it with another image. You will navigate between them later.

1. Double-click `ImagesScroller-1.1.0-windows-installer.exe`: the wizard opens with a Welcome page. No admin/UAC prompt should appear.
2. On the destination page, the suggested folder is `C:\Users\<you>\AppData\Local\Programs\Images Scroller`. You may keep it or type a different folder — pick one you can find easily and REMEMBER it (that is the install folder). Click Next.
3. On the file-association page you see 17 checkboxes (`.avif` `.bmp` `.dib` `.gif` `.heic` `.heif` `.ico` `.jfif` `.jpe` `.jpeg` `.jpg` `.jxr` `.png` `.tif` `.tiff` `.webp` `.wdp`) and NONE of them is pre-ticked. Tick ONLY `.png`. Continue the install.
4. The install finishes. Because you selected a type, the Windows Settings "Default apps" page opens BY ITSELF — that is expected.
5. On that Settings page, find "Images Scroller" and set it as the app for `.png` (Windows shows the list of apps per file type — choose Images Scroller). This is your confirmation step; the app itself never forces a default.
6. Open the install folder and confirm EXACTLY these files are there: `img-scroller.exe`, `img-scroller.toml`, `LICENSE`, and `uninstall.exe`. There must be NO `register-default-viewer.cmd` and NO `DEFAULT-VIEWER-GUIDE.txt`.
7. Open the Start Menu: a "Images Scroller" shortcut exists (and no desktop shortcut was created).
8. In Explorer, right-click a `.png` file → "Open with": "Images Scroller" appears in the list. Then right-click a type you did NOT select (e.g. a `.jpg`): "Images Scroller" does NOT appear in its list (if it does because of earlier testing, report exactly what you see).
9. Double-click `img-scroller.exe` in the install folder (or use the Start Menu shortcut): the app opens MAXIMIZED with no console window, and the title bar reads "Images Scroller" (no image open yet).
10. Click the big centered Open button and open one of your images: the title bar now shows the path of the VISIBLE image. Every time you navigate (see item 11), the title updates to the new image's path.
11. Window title, window MAXIMIZED: (a) with a SHORT path that fits the width, the title is the FULL path (for example `C:\test\images1\img.png`); (b) with a LONGER path, the title shows the image name COMPLETE, then adds the folder containing it, then the drive with `...` between the drive and the folders (for example `C:\...\images1\img.png`), then keeps adding parent folders (nearest first) while space remains; (c) if even the file NAME is too long, the title shows `...` followed by the LAST characters of the name.
12. Window title, window NOT maximized: click the restore button so the window is smaller — the title shows ONLY the file name + extension (for example `img.png`); with a very long name it shows `...` + the last characters. Maximize the window again: the path form comes back.
13. Fullscreen name popup: press F11 (or the fullscreen pill button) — the title bar hides. Press ← or →: a small dark popup appears at the TOP CENTER showing the file name + extension, stays for about 3 seconds, then disappears by itself. Navigate again: it reappears for another ~3 seconds. Leave fullscreen (F11 or ESC): the popup disappears immediately.
14. Waves-1–7 regression block, all from the INSTALLED location: ←/→ step through the images of the folder in the same order Windows Explorer shows them; ↑/↓ move to the sibling folders under the same parent (a folder without images shows "Empty folder"); going back to a folder you already visited this session reopens the image you last viewed there; zoom in with I (or Shift+'+'), out with O (or Shift+'−'), back to fit with 0; fit width with W, fit height with H; zoomed past the window size, the arrows PAN the image and navigation fires only when you reach the image's edge; F11 toggles fullscreen and ESC exits it; with fullscreen OFF, the first ESC shows "Press ESC again to close" for about 2 seconds and a second separate ESC press closes the app; the on-screen pill buttons do the same as the keys; the "Open with..." pill button (or the Q key) lists your registered programs and opening the image with one of them works.
15. Uninstaller: close the app. Open Windows Settings → Apps → Installed apps (or the classic "Programs and Features"), find "Images Scroller", and uninstall it. Afterwards check ALL of: the installed files are gone from the install folder (the folder itself disappears if nothing else is left in it); the Start Menu shortcut is gone; right-click a `.png` → "Open with" no longer lists Images Scroller; "Images Scroller" no longer appears in the installed-apps list.
16. GitHub release — RUN THIS ITEM ONLY AFTER the wave's GitHub-Releases publishing task has been completed (if it has not yet, skip this item and report it as "pending the releases task"): open the project's GitHub page in your browser → the "Releases" section shows a release for version 1.1.0 → it lists the installer `.exe` and both zips (x64 and x86) → download the installer: the downloaded file is non-empty and has the same name as the local one (roughly 6.8 MB).

STANDING NOTE: report the result of this run back to the session (any FAIL means the wave is NOT done). The TODO's manual-UX sub-item stays `[ ]` until your report closes it (item 16 may be reported separately after the releases task completes).

## 9. Risks / stop conditions

- VM or `/img-scroller` share unavailable at any step → STOP, return a user question (wave execution note).
- Any nonzero exit on any §3 command → STOP and report (V7 — NO expected failures exist in this task).
- Census mismatch → STOP and report (V1/V7).
- ANY DLL-audit delta (new OR missing, either architecture) → STOP and report (V4/V7) — no script re-fill, no self-adjudicated delta.
- Pinned artifact-name mismatch (version drift) → STOP and report (V5/V7).
- Packaging fails for a MISSING artifact → the builds of Steps 2–3 precede it, so this should not occur; if it does → STOP and report.
- An unexpected untracked/staged source file at Step 0 → STOP and ask the caller (never stage ride-alongs).
- The OPERATOR-RUN outcome (any §8 item FAIL) → the wave is NOT closed; the caller decides the fix path.
- Any plan ambiguity encountered by a sub-agent → STOP and return the question to the Planner; never guess.
- Review-cycle cap: max 3 (4.3) cycles, then escalate to the user.

## 10. Plan-vs-task compliance check

- TODO Task 5 requires: (1) "Unit tests for the new pure logic (title composition/truncation; installer/association-selection pure logic where any exists) — run via the dev-checks harness ... (`ALL CHECKS PASSED` ...)" ✔ (V1: the formal wave-HEAD run is Step 1; the 18 landed `window_title` composition/truncation tests are exercised; the census statement is explicit — Task 5 adds NO tests because Task 4 already added the pure-logic tests and the installer/association-selection logic is NSIS script code with NO Rust pure logic, per Task-1 D18; the corroboration pins 213/29 with the exact grep/count method); (2) "The installer build (and the zip build if retained) via the decided mechanism — each command exits `0` and leaves a non-empty artifact" ✔ (V2/V3: the fresh chain x64 → x86 → installer → zips, exit-code-authoritative, non-empty artifacts, byte sizes recorded; the zips are RETAINED per Task-1 D12 and both are re-produced; the V3 freshness decisions — fresh x64 REQUIRED for the installer payload, fresh x86 REQUIRED because the x86 exe lives only in the zip and its provenance is unverified; both audits re-run per Wave-1 per-build discipline, set-equal 22 with the delta adjudication rule, NO script fill); (3) "Manual UX check (OPERATOR-RUN ...)" ✔ (V6 + §8: the PLAIN self-contained checklist covering the wizard install to a chosen destination, the association page picking only `.png`, the registration landing (Open With lists + the Settings confirmation), the app launching from the install location, the Waves-1–7 regression block, the window title per Task 4 with the concrete composition expectations (full path / name / truncation / not-maximized rule / fullscreen 3-s popup), the uninstaller removing files/shortcuts/associations, and the GitHub release visibility + artifact downloads — explicitly GATED on Task 7's completion; the sub-item stays `[ ]` as the user's gate). Also encoded: V5 (NO intentionally-expected failures — Task 1's smoke probe already covered the typed missing-payload failure), V7 (typed-failure/STOP rules), V8 (the pinned results file `20261118-wave8-task5-verification-results.md`), the sub-step mapping (4.2 runs V1–V4 and commits the results; 4.3; 4.4 results-record review only; 4.5b adjudication; 4.6 mark with the manual-UX sub-item staying `[ ]`), and the NOT-to-do list (no source changes, no new tests, no version touch, no push, no edits of prior plan files). Nothing from Tasks 6–7 is executed (prose docs = Task 6; releases = Task 7). The plan is complete and correct against the task.
