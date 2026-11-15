# Wave 7 — Task 4 Verification Plan: Dev-Checks, Dual-Architecture Release Builds, x86 DLL Import Audit, Packaging Re-Verification, OPERATOR-RUN Checklist

- Source TODO: `.agent/todos/20261029/20261029-todo-9.md` → `### Task 4: Verification`. Verbatim sub-items: (1) "Unit tests for any pure packaging/config logic that exists — run via the dev-checks harness in the container per `docs/build.md` (`ALL CHECKS PASSED`: fmt/check/test/clippy; `alpine-vm` MCP; `vm_status` first; the command exit code is the authoritative success evidence)."; (2) "x64 AND x86 release builds via the extended Wave-1 build script inside the container (per Task 2) — each command exits `0` and leaves a non-empty artifact at the gitignored `dist/` locations; run through the `alpine-vm` MCP (call `vm_status` first; the command exit code is the authoritative success evidence)."; (3) "Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): run the installer flow on a clean destination dir → binary + config file present and app launches; default-viewer helper/guidance performs or guides correctly; packaged app navigations from Waves 1–6 still work from the installed location."
- Global plan (approved): `.kilo/plans/20261114-wave7-distribution-mvp.md` → `## Task 4 — Verification` + Wave-level decisions (#2 version 1.0.0, #3 no-frontend, #7 house rules).
- Task plans (landed, defining the verification targets): `.kilo/plans/20261114-wave7-task1-config-plan.md` (config = header-only TOML; NO tests), `20261114-wave7-task2-installer-packaging-plan.md` (D4 build matrix; D6 smoke evidence; D2's x86 audit note — the OBSERVED x86 list is recorded HERE, the Wave-1 fill-at-verification precedent), `20261114-wave7-task3-viewer-registration-plan.md` (D1b registry surface; D7: NO Rust/tests; real registration = THIS task's OPERATOR-RUN).
- Date: 2026-11-15. Status: FOR PLANNER PRESENTATION (pending user approval before 4.2 runs).
- Branch: `feat/wave-7-distribution` (Tasks 1–3 landed: commits `fae5cf7`, `f38dece`, `1fe2844`; version 1.0.0). Front-end related: NO (Wave-level decision #3 — no 4.1a, no 4.5a).
- High-level approach: this task RUNS the formal verification — one dev-checks harness run, two single-target release builds, TWO DLL import audits (x64 re-audit + the first x86 audit), one packaging re-run with zip/CRLF checks — records everything in a NEW results file, fills the OBSERVED x86 DLL list into the build script (Wave-1 precedent), and hands the user a PLAIN self-contained OPERATOR-RUN checklist (§8).

## 0. Verification Design Decisions (BINDING — the implementer must not re-choose)

### D1 — Dev-checks run: the formal wave-HEAD run; census stays 195/28; NO tests added

- The formal run is the TODO sub-item 1 evidence: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — expected final line `ALL CHECKS PASSED`, exit 0 (per `docs/build.md`: one `PASS <name> exit=0` line per check `fmt-check`/`check`/`test`/`clippy`).
- Census statement (explicit, pinned): **Task 4 does NOT add tests — the only trigger would be a pure packaging/config function landing in `src/`, and NONE exists.** Tasks 1–3 changed NO file under `src/` and NO Cargo manifest (Task 1 = tracked `packaging/img-scroller.toml` header-only; Task 2 = Dockerfile + 2 shell scripts; Task 3 = 2 `packaging/` files + `.gitattributes` + manifest lines). The census stays **195 unit tests / 28 test-bearing files** (breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules — `app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`).
- Corroboration (secondary evidence, both pinned below): the harness output's final `cargo test` summary reads `test result: ok. 195 passed; 0 failed;`; a container grep counts the test-bearing files (expect `28`). A mismatch vs either number is a census mismatch → STOP (D7).

### D2 — Dual-architecture release builds: two single-target commands; the run IS the evidence

- Per the TODO and the Task-2 D4 build matrix, the evidence is EXACTLY two single-target invocations (ONE command per call; the exit code is authoritative):
  1. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu` → expected exit 0; output contains `build-windows: building target: x86_64-pc-windows-gnu`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, the x64 OBSERVED 22-DLL audit line, `build-windows: OK`; the artifact at the gitignored `dist/windows-x64/img-scroller.exe` is non-empty.
  2. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu` → expected exit 0; output contains `build-windows: building target: i686-pc-windows-gnu`, `build-windows: artifact: dist/windows-i686/img-scroller.exe`, the x86 audit note (pre-fill wording: `... the OBSERVED list is recorded by the Wave-7 verification task ...`), `build-windows: OK`; the artifact at the gitignored `dist/windows-i686/img-scroller.exe` is non-empty.
- A clean-state rebuild is NOT required — the runs are incremental (`--locked`, cargo reuses the compiled dependency tree from Task 2; both were exit-0 smoke-verified there) and each run IS its own formal evidence.
- The no-argument both-targets run and the bogus-target typed-failure check are NOT repeated here (they were Task 2's D6 smoke evidence; the TODO requires only the two per-target builds). NO intentionally-expected typed failures exist in Task 4 (D7).

### D3 — The DLL import audits: BOTH re-run as this task's per-build evidence

Wave-1 discipline: the audit is OBSERVED per-build evidence, never an assumption — so Task 4 re-runs BOTH audits on the freshly rebuilt 1.0.0 artifacts and adjudicates any delta.

- **x64 re-audit** (corroborates the recorded 22-DLL constant against the fresh artifact): audit tool `x86_64-w64-mingw32-objdump` (in the image).
- **x86 first audit** (the Task-2 D2 pointer resolves here): audit tool `i686-w64-mingw32-objdump` (in the image, verified 2026-11-14 in Task 2's D1).
- **Extraction mechanism (pinned, per audit, TWO commands):**
  1. LIST: `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u` (for the x86 audit, the same pipeline with `i686-w64-mingw32-objdump` and `dist/windows-i686/img-scroller.exe`). Output: the imported-DLL names, one per line, lowercase, deduplicated, sorted.
  2. COUNT: the same pipeline with `| wc -l` appended — expected output exactly `22` (the count guard: if the artifact were unreadable/empty the list would be empty and the count 0 ≠ 22 — the count expectation is the authoritative guard; the pipeline's exit code is authoritative for the grep/sort chain).
- **Expected set — the recorded 22-DLL constant (the implementer verifies item-by-item, set-equal, order-independent):**
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
- **Adjudication rules:** x64 re-audit — ANY delta vs the recorded constant (new OR missing name) is unexpected and → STOP and report to the caller. x86 first audit — (a) set-equal to the 22 names → proceed to the fill (D4); (b) a NEW name not in the list — especially any MinGW runtime DLL (`libgcc_s_dw2.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`) — means the x86 build is NOT portable → STOP and ask the caller (global-plan risk §); (c) MISSING names only (the x86 build imports fewer system DLLs) → recorded as an explained delta, the OBSERVED x86 list IS what was printed, and the fill (D4) uses the observed list — proceed.

### D4 — Audit recording decision: BOTH mechanisms (the Wave-1 fill-at-verification precedent)

- **(a) Results-file recording (always):** the OBSERVED x86 list is recorded in the NEW results file `.kilo/plans/20261114-wave7-task4-verification-results.md` (D8), and it lands in `docs/build-windows.md` during Task 5 (docs step) — a binding Task-5 carry-forward recorded in the results file.
- **(b) Script fill (CHOSEN — the Wave-1 precedent, exactly as Wave 1 filled `OBSERVED_DLL_IMPORT_LIST`):** `scripts/build-windows.sh` gains a SECOND constant `OBSERVED_DLL_IMPORT_LIST_I686` and the x86 branch prints it. Justification: the constant next to the build keeps the observed evidence with the producer of the artifact (the same reason the x64 list lives in the script); the script never prints an unobserved list, so the fill happens ONLY after the D3 adjudication passes.
- **EXACT script diff (the implementer makes EXACTLY these two edits, nothing else):**

  Edit 1 — after the existing line 21 (`readonly OBSERVED_DLL_IMPORT_LIST="advapi32.dll, ... ws2_32.dll"`), insert ONE new line:
  ```sh
  readonly OBSERVED_DLL_IMPORT_LIST_I686="<the observed x86 list, comma-separated lowercase, same format as the x64 constant>"
  ```
  If the x86 audit adjudicated set-equal (expected), the string is IDENTICAL to the x64 constant's value.

  Edit 2 — in `print_audit_note_for_target`, replace the x86 case branch:
  ```sh
    "$X86_TARGET")
      printf 'build-windows: %s DLL import audit: the OBSERVED list is recorded by the Wave-7 verification task (audit tool: i686-w64-mingw32-objdump)\n' "$X86_DIST_DIR"
      ;;
  ```
  with:
  ```sh
    "$X86_TARGET")
      printf 'build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (%s): %s\n' "$X86_DIST_DIR" "$OBSERVED_DLL_IMPORT_LIST_I686"
      ;;
  ```
  (the x64 branch and EVERY other line of the script stay byte-identical).

- Commit (verbatim message): `feat: record observed x86 dll import list` — stages ONLY `scripts/build-windows.sh`.

### D5 — Packaging + zip re-verification: ONE full re-run as the formal record

- The Task-2/Task-3 smoke evidence is superseded as the FORMAL record by ONE fresh re-run in this task (the formal record belongs here; the re-run consumes the artifacts produced by D2's two builds):
  1. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh` → exit 0; output contains `package-windows: zip: dist/ImagesScroller-1.0.0-windows-x64.zip`, `package-windows: zip: dist/ImagesScroller-1.0.0-windows-x86.zip`, `package-windows: OK`.
  2. `unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip` → exit 0; EXACTLY 4 flat entries: `img-scroller.exe`, `img-scroller.toml`, `register-default-viewer.cmd`, `DEFAULT-VIEWER-GUIDE.txt` (no directory entries).
  3. `unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip` → exit 0; the same EXACTLY 4 flat entries.
  4. Packaged-.cmd CRLF evidence, both zips: `sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""` → exit 0 (the packaged helper copy carries CRLF); the same command against the x86 zip → exit 0.
- Artifact record (the results file's byte-size table): `ls -l /img-scroller/dist/windows-x64/img-scroller.exe /img-scroller/dist/windows-i686/img-scroller.exe /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip` → exit 0; all four non-empty; sizes recorded verbatim.
- The pre-existing gitignored `dist/windows-x64/img-scroller-diagnostic.exe` + `.log` are UNTOUCHED (never packaged — the manifest is explicit; removal belongs to the wave's Step-5 tmp sweep).

### D6 — OPERATOR-RUN checklist: written PLAIN and SELF-CONTAINED (§8) — the user's gate

- The manual-UX sub-item is agent-impossible (GUI/runtime on real Windows); the checklist in §8 is handed to the caller for the user. The TODO's manual-UX sub-item STAYS `[ ]` until the user reports the run (standing pattern from Waves 4.1–6.2). The checklist covers EXACTLY what the TODO requires: the installer flow on a clean destination dir → the 4 flat files present and the app launches; the default-viewer helper/guidance performs or guides correctly; the packaged app's Waves-1–6 navigations still work from the installed location.

### D7 — Failure / adjudication rules

- NO intentionally-expected typed failures exist in Task 4 (no FORCE_FAIL injection, no bogus-target check — those were Task 2's design). Therefore: **ANY nonzero exit on any command of §3 → STOP and report to the caller.**
- Census mismatch (not `ALL CHECKS PASSED`, or the test count ≠ 195, or the file count ≠ 28) → STOP and report.
- x64 audit delta → STOP and report. x86 audit NEW DLL → STOP and ask the caller (not portable). x86 audit missing-only names → explained delta, fill with the observed list, proceed (D3).
- A typed `build-windows: error:` / `package-windows: error:` line → STOP and report.
- VM or `/img-scroller` share unavailable → STOP, return a user question (wave execution note).

### D8 — Results file: YES (the Wave-6.x precedent)

- A NEW file `.kilo/plans/20261114-wave7-task4-verification-results.md` is produced by the implementer (creation authorized; matches the Wave-6.x precedent, e.g. `20261113-wave6-2-task3-verification-results.md`). Contents (§3 Step 9 pins the outline): the five evidence blocks (dev-checks + census; the two builds; both audit lists verbatim + adjudication; packaging + zip/CRLF checks; the artifact byte-size record), the census statement (195/28, no tests added), the script-fill record, the pending OPERATOR-RUN pointer (§8), and the Task-5 carry-forward (the x86 OBSERVED list → `docs/build-windows.md`; also the x86/build-matrix rows for the typed-failure table).
- Commit (verbatim message): `docs: add wave7 task4 verification results` — stages ONLY the results file.

## 1. Environment Facts (verified 2026-11-15)

- Git: on branch `feat/wave-7-distribution`; Tasks 1–3 all `[DONE]` (commits `fae5cf7`, `7384a24`/`1050785`, `f38dece`, `6708461`, `a00f5f6`, `2d0bcab`/`3405b80`, `494177d`, `5279531`, `1fe2844`); version 1.0.0; working tree has exactly ONE unstaged modification — `.agent/project-info/context.md` (the Tasks 1–3 4.1b ride-along; NEVER staged by this task's commits).
- `scripts/build-windows.sh` (120 lines) carries the Task-2 pinned shape: the 22-DLL `OBSERVED_DLL_IMPORT_LIST` constant (line 21), the x86 audit-note pointer branch (lines 91–93), the typed-failure discipline, atomic `.part` install, `--locked`.
- `scripts/package-windows.sh` (107 lines) carries the Task-2/3 shape: the 8-line manifest (both archs × 4 files), dynamic version extraction (CR-tolerant), flat zips, EXIT-trap staging cleanup.
- Alpine VM: RUNNING, SSH reachable (verified via `vm_status` 2026-11-15). All runs go through the `alpine-vm` MCP (`vm_status` FIRST; exit code authoritative; ONE command per call). If the VM or the `/img-scroller` share is unavailable: STOP, return a user question.
- Artifacts on disk (planning-time `ls -l`, exit 0): `dist/windows-x64/img-scroller.exe` = 19,649,092 bytes; `dist/windows-i686/img-scroller.exe` = 19,802,972 bytes; `dist/ImagesScroller-1.0.0-windows-x64.zip` = 6,711,458 bytes; `dist/ImagesScroller-1.0.0-windows-x86.zip` = 7,160,311 bytes; the gitignored diagnostic exe + log remain in `dist/windows-x64/` (untouched). The D2 builds overwrite the exes atomically; sizes are re-recorded after.
- Census pinned: 195 unit tests / 28 test-bearing files (no `src/` change since the 195-pin; corroboration in Step 1).
- `docs/build.md` harness semantics unchanged (fmt/check/test/clippy; logs under gitignored `logs/checks/`); `docs/build-windows.md` still documents the x64-only flow with the OBSERVED 22-DLL list (the x86 rows + the x86 OBSERVED list are Task 5's).

## 2. Binding Constraints (apply to every step)

- ALL container runs via the `alpine-vm` MCP: `vm_status` FIRST (re-check before the first command of each working session); ONE command per call; the command exit code is the authoritative success evidence; absent cargo stderr is NOT a failure signal (`docs/build.md`).
- NO `git push` (restricted to Step 5). NO branch creation/switch (restricted to Step 2). NO version edit (`Cargo.toml`/`Cargo.lock` NEVER touched by this task).
- Gitignore Compliance Rule before EVERY commit: read `.gitignore`, run `git status`, ensure NO `dist/`, `logs/`, `target/`, or `*.log` path is staged (the zips, staging tree, checks logs, and diagnostic exe/log are all gitignored — never staged). NEVER stage `.agent/project-info/context.md` (the ride-along).
- Do NOT edit any existing `.kilo/plans/*` file. The NEW files of this task are EXACTLY: this plan file + the results file.
- Do NOT edit `README.md`, `docs/*` (`build.md`, `build-windows.md`), `.agent/project-info/{brief,product,tech,architecture}.md`, `CHANGELOG.md`, `Dockerfile`, `docker-compose.yml`, `scripts/dev-checks.sh`, `scripts/package-windows.sh`, `packaging/*`, or ANYTHING under `src/` — Task 5 owns the wave's prose; no source/build-harness change belongs to Task 4. The ONLY script edit of this task is the D4b fill in `scripts/build-windows.sh`.
- File edits use real newline characters (never literal `\n` escape sequences); `scripts/build-windows.sh` MUST stay LF (the `*.sh text eol=lf` pin; the Step-7 CR check proves it).
- Max-lines rules: the edited script stays 121 lines (one added constant line) — under the scripts' ~line discipline; no `src/` rule applies.

## 3. Step-by-Step Implementation (workflow step 4.2 — implementer, JUNIOR under 50%)

Execute the steps in order. Check the plan between steps. Stop and ask the caller on ANY ambiguity — never guess. Every `alpine-vm_vm_run_command` call is ONE command; the exit code is authoritative; apply the pinned timeouts.

### Step 0 — Commit the plan file (BEFORE implementation)

- `git status` → on branch `feat/wave-7-distribution`; the tree contains (at least) the new file `.kilo/plans/20261114-wave7-task4-verification-plan.md` plus the pre-existing unstaged `context.md` ride-along (NOT this commit).
- Gitignore compliance: read `.gitignore`; confirm nothing gitignored is about to be staged.
- Stage ONLY the plan file: `git add .kilo/plans/20261114-wave7-task4-verification-plan.md`
- Commit (verbatim message): `docs: add wave7 task4 verification plan`

### Step 1 — vm_status + the formal dev-checks run + census corroboration

- Call `vm_status` FIRST. If not running/SSH-unreachable: STOP, return a user question.
- Run the harness (ONE command, timeout 900000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
  - Expected: exit 0; one `PASS <name> exit=0` line for each of `fmt-check`, `check`, `test`, `clippy`; final line `ALL CHECKS PASSED`; the `test` section's final cargo summary contains `test result: ok. 195 passed; 0 failed;`. Capture the printed log path (`logs/checks/dev-checks-<...>.log`) for the results file.
  - Any other outcome → STOP (D7).
- Census file-count corroboration (ONE command, timeout 120000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rl '#\[test\]' src | wc -l"`
  - Expected: exit 0; output exactly `28`. Any other number → census mismatch → STOP (D7).

### Step 2 — x64 single-target release build

- `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu` (timeout 3600000 ms).
- Expected: exit 0; output contains `build-windows: building target: x86_64-pc-windows-gnu`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, the line `build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (dist/windows-x64): advapi32.dll, ... ws2_32.dll`, `build-windows: OK`.
- Any typed error or nonzero exit → STOP (D7).

### Step 3 — x86 single-target release build

- `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu` (timeout 3600000 ms).
- Expected: exit 0; output contains `build-windows: building target: i686-pc-windows-gnu`, `build-windows: artifact: dist/windows-i686/img-scroller.exe`, the pre-fill x86 audit-note line (`... the OBSERVED list is recorded by the Wave-7 verification task ...`), `build-windows: OK`.
- Any typed error or nonzero exit → STOP (D7).

### Step 4 — x64 DLL re-audit (on the fresh artifact)

- LIST (ONE command, timeout 180000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u`
  - Expected: exit 0; EXACTLY the 22 names of D3 (one per line, lowercase, sorted).
- COUNT (ONE command, timeout 180000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u | wc -l`
  - Expected: exit 0; output exactly `22`.
- Copy the observed list verbatim for the results file. ANY delta vs D3's list → STOP and report (D3/D7).

### Step 5 — x86 DLL first audit (on the fresh artifact)

- LIST (ONE command, timeout 180000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust i686-w64-mingw32-objdump -x dist/windows-i686/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u`
  - Expected: exit 0; the observed x86 imported-DLL list (one per line, lowercase, sorted).
- COUNT (ONE command, timeout 180000 ms): the same pipeline with `| wc -l` appended.
  - Expected: exit 0; output exactly `22` (per D3's expectation; if the observed list legitimately differs — see adjudication below — the count equals the OBSERVED list's length and the adjudication decides).
- Adjudication (D3 rules, applied NOW by the implementer before anything else):
  - set-equal to D3's 22 names → proceed to Step 6.
  - any NEW name (esp. `libgcc_s_dw2.dll` / `libstdc++-6.dll` / `libwinpthread-1.dll`) → STOP and ask the caller (NOT portable).
  - missing-only names → record as an explained delta; the OBSERVED list is what was printed; proceed to Step 6 with the observed list.

### Step 6 — Fill the OBSERVED x86 list into the build script (D4b)

- Edit `C:\repo\img-scroller\scripts\build-windows.sh`: make EXACTLY the two edits of D4 (the new `OBSERVED_DLL_IMPORT_LIST_I686` constant after line 21 — value = the adjudicated observed x86 list in the comma-separated lowercase format; the x86 printf branch swapped to print it). NOTHING else changes; the file stays LF with real newlines.
- CR check (ONE command, timeout 120000 ms): `sh -c "! grep -q \"$(printf '\r')\" /img-scroller/scripts/build-windows.sh"` — expected exit 0 (LF-only). Exit 1 → fix the file's line endings and re-run.
- Confirmation re-run (ONE command, timeout 3600000 ms): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu`
  - Expected: exit 0; the x86 audit line NOW prints `build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (dist/windows-i686): <the observed list>`; `build-windows: OK`. (The cargo build is an incremental no-op — this run re-serves the formal x86 exit-0 evidence with the final script.)

### Step 7 — Packaging re-run + zip checks + artifact record (D5)

- Run each as its OWN command; exit code authoritative:
  1. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh` (timeout 600000 ms) — expected exit 0 with the three pinned output lines (D5).
  2. `unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip` (timeout 120000 ms) — expected exit 0; EXACTLY the 4 flat entries (D5).
  3. `unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip` (timeout 120000 ms) — expected exit 0; the same 4 flat entries.
  4. `sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""` (timeout 120000 ms) — expected exit 0 (packaged CRLF).
  5. `sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""` (timeout 120000 ms) — expected exit 0.
  6. `ls -l /img-scroller/dist/windows-x64/img-scroller.exe /img-scroller/dist/windows-i686/img-scroller.exe /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip` (timeout 120000 ms) — expected exit 0; all four non-empty; record the byte sizes verbatim for the results file.
- Any typed `package-windows: error:` or nonzero exit → STOP (D7).

### Step 8 — Create the results file (D8)

- Create `.kilo/plans/20261114-wave7-task4-verification-results.md` (NEW file) with EXACTLY these sections:
  1. Header: source TODO Task 4, this plan file's path, date, branch/HEAD commit, version 1.0.0.
  2. Dev-checks evidence: the command, exit code, the four `PASS` lines, the `ALL CHECKS PASSED` line, the log path, and the census corroboration (`195 passed; 0 failed`; file count `28`) + the census statement (195/28 pinned; NO tests added — no pure packaging/config logic exists).
  3. Build evidence: both commands with exit codes, the pinned output lines observed, `build-windows: OK`.
  4. Audit evidence: BOTH observed lists verbatim (x64 re-audit, x86 first audit) with counts, the comparison verdict (set-equal / explained-delta), and the adjudication.
  5. Packaging evidence: the re-run exit 0 + the three output lines, both `unzip -l` listings (4 flat entries each), both packaged-CRLF exits 0, the four artifact byte sizes.
  6. The script-fill record: the D4b edits landed, the CR check exit 0, the confirmation re-run exit 0 with the printed x86 OBSERVED list.
  7. The pending OPERATOR-RUN gate: the manual-UX sub-item stays `[ ]` until the user reports; the checklist lives at this plan's §8.
  8. The Task-5 carry-forward: (a) the x86 OBSERVED list lands in `docs/build-windows.md` (the docs step); (b) `docs/build-windows.md` gains the x86/build-matrix rows for its typed-failure and invocation sections; (c) `docs/build.md` needs NO change (harness semantics untouched).
  9. Verdict line: `VERIFIED` (agent-side evidence complete) or the STOP reason if any step halted.

### Step 9 — Gitignore compliance + the two remaining commits

- `git status` → confirm NO gitignored path is staged (the zips/staging/checks-logs/diagnostic files are under `dist/`+`logs/` — unstaged; `context.md` stays unstaged).
- Commit B (the script fill): stage EXACTLY `git add scripts/build-windows.sh` → commit (verbatim message): `feat: record observed x86 dll import list`
- Commit C (the results file): stage EXACTLY `git add .kilo/plans/20261114-wave7-task4-verification-results.md` → commit (verbatim message): `docs: add wave7 task4 verification results`
- Post-commit `git status`: clean apart from the pre-existing unstaged `context.md` ride-along.

### Explicitly NOT done in Task 4 (for the implementer)

- NO tests added, NO `src/` change, NO `Cargo.toml`/`Cargo.lock` edit (census stays 195/28). NO clean-state rebuild, NO no-arg build re-run, NO typed-failure spot check (Task 2's smoke evidence; D2/D7). NO README/`docs/`/project-info prose edits (Task 5; the results file's carry-forward governs). NO `CHANGELOG.md` edit (Task 6). NO execution of the `.cmd` helper anywhere (agent-impossible; §8 is the user's run). NO edits to any existing `.kilo/plans/*` file. NO `git push`; NO branch/version actions. The 4.6 TODO marking is §7 (not §3).

## 4. Workflow step 4.3 — Code Review & Simplification guidance

- code-reviewer checklist (deviation from plan = finding): the results file contains ALL nine sections with the real evidence (exit codes, output lines, byte sizes, both audit lists, census numbers); `scripts/build-windows.sh` matches D4 EXACTLY (one new constant line after line 21; ONLY the x86 printf branch changed; x64 branch + everything else byte-identical; LF; the constant's value == the adjudicated OBSERVED x86 list); the three commits contain ONLY their pinned files (plan / script / results); NO file under `src/`, `Cargo.toml`, `Cargo.lock`, `docs/`, `.agent/project-info/*`, `README.md`, `packaging/`, `Dockerfile`, `docker-compose.yml`, or `CHANGELOG.md` changed vs the Step-0 baseline; no gitignored path committed; the TODO mark (§7) touches ONLY the Task-4 heading + sub-items 1–2.
- code-simplifier: review the script edit for simplification — expected outcome NONE (a constant + a printf swap; nothing to simplify; the x86 branch now mirrors the x64 branch exactly). Any change must NOT alter the pinned constant name, note format, or script contract without a plan amendment.
- Max 3 review cycles, then escalate (workflow rule).

## 5. Workflow step 4.4 — Documentation guidance

- The docs-specialist adds NOTHING in Task 4: the recording mechanisms of this task are the results file (D8) + the script constant (D4b); ALL wave prose — README distribution/installation/registration section, `tech.md`/`architecture.md`/`context.md` records, `docs/build-windows.md` (the x86 OBSERVED list + the x86/build-matrix rows), `docs/build.md` (verify-only — harness semantics unchanged, expected NO edit) — is Task 5's deliverable, governed by the results file's carry-forward. Expected outcome: "nothing to document; deferred to Task 5" as a clear message. Creating any documentation file in Task 4 (beyond the results file) is a plan deviation.

## 6. Workflow step 4.5b — Plan-adherence guidance

- Verify §0–§3 against the landed evidence: the Step-1 dev-checks output (`ALL CHECKS PASSED`, 195/0, census 28), the two build runs (exit 0 + non-empty artifacts), BOTH audit lists (verbatim, adjudicated per D3), the script fill (exact D4 diff, LF, confirmation re-run), the packaging re-run + zip/CRLF/size evidence (D5), the results file (D8 outline, all real values), the three commits (pinned files + verbatim messages), gitignore compliance before each.
- If every element conforms: return a clear "no diffs, VERIFIED" message (no report file needed). If a deviation exists: record it in a NEW plan file `.kilo/plans/20261115-wave7-task4-adherence.md` and propose the disposition — never edit existing plans.

## 7. Workflow step 4.6 — Task completion (implementer)

- Edit `.agent/todos/20261029/20261029-todo-9.md` (preserve ALL other content byte-identical):
  - `### Task 4: Verification` → `### Task 4: Verification [DONE]`
  - Sub-item 1 (dev-checks) `- [ ]` → `- [x]`, rest of the line verbatim.
  - Sub-item 2 (x64 AND x86 builds) `- [ ]` → `- [x]`, rest of the line verbatim.
  - Sub-item 3 (manual UX / OPERATOR-RUN) STAYS `- [ ]` — the user's gate (standing pattern; it closes when the user reports the §8 run).
- Gitignore compliance before the commit (`git status`; the TODO file is tracked — safe).
- Commit (verbatim message): `task: mark wave7 task4 done`

## 8. OPERATOR-RUN Checklist (PLAIN, SELF-CONTAINED — hand verbatim to the user)

Run this on your real Windows 10/11 machine. You need the freshly built zip: `C:\repo\img-scroller\dist\ImagesScroller-1.0.0-windows-x64.zip` (the x64 package; the x86 zip `...-x86.zip` works identically if you prefer to test the 32-bit build). The zip was produced by the packaging step and contains exactly 4 files.

1. Extract `ImagesScroller-1.0.0-windows-x64.zip` into a NEW, EMPTY folder of your choice (right-click the zip → "Extract All..." → pick a clean destination folder). That folder is the install folder.
2. Open the extracted folder and confirm ALL FOUR files are present at its root (no subfolders): `img-scroller.exe`, `img-scroller.toml`, `register-default-viewer.cmd`, `DEFAULT-VIEWER-GUIDE.txt`.
3. Double-click `img-scroller.exe`: the app launches (window opens maximized, no console window appears).
4. Regression block — from THIS installed location, verify the Waves-1–6 navigations and behaviors still work: open an image (the big center Open button or drag not needed — use Open); arrow LEFT/RIGHT move through sibling images in the folder in the same order Windows Explorer shows them; arrow UP/DOWN move to sibling folders (an image-less folder shows "Empty folder"); zoom IN with I or Shift+'+' and OUT with O or Shift+'−', restore with 0 (zoom out passes the fit level down to tiny); fit-width W and fit-height H work; F11 toggles fullscreen and ESC exits it; when zoomed past the window size, the arrows PAN the image and navigation fires only at the image edge (press-and-hold at the edge does nothing; a fresh press navigates); the double-ESC close contract ("Press ESC again to close" popup appears for ~2 seconds and auto-dismisses); the on-screen pill buttons do the same as the keys.
5. Open an image, then click the "Open with..." pill button (rightmost; or press Q): the menu lists the programs registered for that file type; select Paint or Photos — the image opens in that program with NO failure message.
6. Default-viewer helper: from the extracted install folder, double-click `register-default-viewer.cmd`: a console window shows the registration message, then the Windows Settings "Default apps" page opens.
7. On the Settings "Default apps" page, find "Images Scroller" in the app list (Windows 11: search or scroll; Windows 10: "Set defaults by app") and set it as the default for ONE type, e.g. `.png`. Then, in Explorer, double-click a `.png` file: it opens in the INSTALLED Images Scroller (from your extracted folder).
8. Right-click a `.png` (or any covered type) in Explorer → "Open with": Images Scroller appears in the list.
9. Open `DEFAULT-VIEWER-GUIDE.txt` (double-click): it opens in Notepad and reads as clear guidance (what the helper does, the Settings steps, the 17 covered types, unregister).
10. OPTIONAL cleanup: open a command prompt IN the extracted folder and run `register-default-viewer.cmd --unregister`: the console confirms removal; Images Scroller disappears from the "Open with" list (your file-type DEFAULT choices, if any, stay yours to change in Settings).

STANDING NOTE: report the result of this run back to the session (any FAIL means the wave is NOT done). The TODO's manual-UX sub-item stays `[ ]` until your report closes it.

## 9. Risks / stop conditions

- VM or `/img-scroller` share unavailable at any step → STOP, return a user question (wave execution note).
- Any nonzero exit on any §3 command → STOP and report (D7 — NO expected failures exist in this task).
- Census mismatch → STOP and report (D1/D7).
- x64 audit delta → STOP and report (the recorded constant is the wave-validated baseline).
- x86 audit NEW DLL (esp. a MinGW runtime DLL) → the x86 build is NOT portable → STOP and ask the caller (global-plan risk; do NOT "fix" by adding link flags or editing Cargo files).
- Packaging fails for a MISSING artifact → the builds of Steps 2–3 precede it, so this should not occur; if it does → STOP and report.
- The OPERATOR-RUN outcome (any §8 item FAIL) → the wave is NOT closed; the caller decides the fix path.
- Any plan ambiguity encountered by a sub-agent → STOP and return the question to the Planner; never guess.
- Review-cycle cap: max 3 (4.3) cycles, then escalate to the user.

## 10. Plan-vs-task compliance check

- TODO Task 4 requires: (1) "Unit tests for any pure packaging/config logic that exists — run via the dev-checks harness ... (`ALL CHECKS PASSED` ...)" ✔ (D1: the formal run is Step 1; the census statement is explicit — Task 4 adds NO tests because NO pure packaging/config function exists in `src/` after Tasks 1–3; the corroboration pins 195/28); (2) "x64 AND x86 release builds via the extended Wave-1 build script inside the container — each command exits `0` and leaves a non-empty artifact at the gitignored `dist/` locations" ✔ (D2: the two single-target commands of the Task-2 D4 matrix; exit-code-authoritative; non-empty artifacts re-recorded in Step 7.6); (3) "Manual UX check (OPERATOR-RUN ...)" ✔ (D6 + §8: the PLAIN self-contained checklist covering the installer flow on a clean destination dir, the 4 flat files, the app launch, the default-viewer helper/guidance, and the Waves-1–6 navigation regression — the sub-item stays `[ ]` as the user's gate). Additionally resolved from the wave plan's Task-4 pre-analysis: the census re-pin (stays 195), the DLL audit for the NEW x86 artifact (D3/D4 — BOTH audits re-run per the Wave-1 per-build discipline; the x86 OBSERVED list is recorded in the results file + filled into the script + handed to `docs/build-windows.md` in Task 5), the packaging runs recorded (D5), and the results file (D8). Nothing from Tasks 5–6 is executed (prose docs = Task 5; CHANGELOG = Task 6). The plan is complete and correct against the task.
