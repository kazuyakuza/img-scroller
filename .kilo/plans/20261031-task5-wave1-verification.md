# Implementation Plan — Wave 1, Task 5: Verification

- Date: 2026-10-31
- Workflow step: Critical Workflow §4.1b (Analysis & Planning) — Task 5 of TODO
  [`.agent/todos/20261029/20261029-todo-3.md`](../../.agent/todos/20261029/20261029-todo-3.md) ("### Task 5: Verification")
- Role: Architector. Plan ONLY — this file is the deliverable of this step. NO code writing, NO cargo/docker runs by the architector (research reads already done: `vm_status`, `df -h`, host `git status`, `src/` tree, `Cargo.toml`, both fill files).
- Executing branch (already created in Step 2): `feat/wave-1-skeleton-open-image` @ `99c4321` (Tasks 1–4 landed). No branch creation, no push, no branch ops in this task.
- Plan approval: per-task plans AUTO-APPROVED ("Approve Global and Tasks Plans", 2026-10-30). Save + return path; do NOT ask for approval.
- Front-end marking: Task 5 is verification of landed code — NOT front-end related → 4.1b only (no 4.1a, no 4.5a).
- Global plan in force (do NOT modify): [`.kilo/plans/20261030-wave1-skeleton-open-image.md`](20261030-wave1-skeleton-open-image.md) — §0.2 (binding build environment), §2 Task 5, §3 (risks).
- Task 5 is VERIFICATION of Tasks 1–4 output: a failure is a REGRESSION owned by Tasks 3/4 review — handled by STOP + report, never by in-draft fixes (see each step's failure policy).
- Target implementer: JUNIOR developer, 50% restriction — every threshold, timeout, format, and decision is encoded here; zero judgment calls remain.

## 0. Scope

### 0.1 In scope (TODO Task 5 sub-items + Task-1 retained-scope binding, verbatim mapping)

| TODO Task 5 sub-item (abbreviated) | Delivered by |
| --- | --- |
| Container dev-checks pass via the harness (`ALL CHECKS PASSED`, through the `alpine-vm` MCP, `vm_status` first, exit code authoritative) | §3 MS-2 |
| `cargo test --locked` covers the container-runnable unit tests (15 tests, from the Tasks 2–4 architector plans) | §3 MS-2 (+ §3 MS-3 decision record) |
| Windows release cross-build exits `0` with non-empty `.exe` at `dist/windows-x64/` | §3 MS-4 |
| Manual UX check — OPERATOR-RUN on real Windows; recorded as operator-pending, NEVER executed by agents | §3 MS-7 (+ §6 mapping) |
| Task-1 retained scope: import audit ONCE via `x86_64-w64-mingw32-objdump` on OUR produced exe; OBSERVED DLL list recorded in `scripts/build-windows.sh` success-note constant + `docs/build-windows.md` | §3 MS-5 + §3 MS-6 (edits in §3) |

### 0.2 Out of scope (explicit)

- NO Task-6 doc updates (README.md, tech.md, architecture.md) — Task 6 consumes this step's outcomes.
- NO changes to `docs/build.md` (its operator/validation stance already covers the operator-pending recording; adding anything there would duplicate it).
- NO changes to any `src/` file, `Cargo.toml`, `Cargo.lock`, `Dockerfile`, `docker-compose.yml`, `scripts/dev-checks.sh` — Task 5 never modifies landed code.
- NO `.agent/todos/*` edits — checkbox marking at 4.6 is the implementer's follow-up step, untouched here.
- NO cleanup of the `/cargo-target` named volume (its GNU release artifacts remain for future waves; see §8).
- NO objdump run BEFORE MS-4 succeeds (the audit runs exactly once, on the actually produced exe).
- NO rerun of `scripts/build-windows.sh` after the OBSERVED-DLL fill (decision D6, §2).
- NO `FORCE_FAIL` usage — the hook is NOT for masking failures.
- NO logging/evidence files created inside the repo (evidence = captured MCP output + the 4.2 summary; `logs/checks/*.log` from the harness run stays gitignored and untracked).

### 0.3 Binding constraints (global plan §0.2 + tech.md + docs/build.md)

- ALL Rust/Cargo execution happens inside the Linux Docker container on the Alpine VM; host needs NO Rust/Cargo.
- `alpine-vm` MCP discipline — call `vm_status` BEFORE **EVERY** VM command (MS-1, MS-2, MS-4, MS-5, MS-6 below; never batch a command before its own `vm_status`). The command EXIT CODE is the authoritative success signal. Absent cargo stderr in captured MCP output is NOT a failure signal.
- Project shared on the VM at `/img-scroller`; Docker + Compose plugin exist ONLY on the VM.
- If the VM or the `/img-scroller` share is unavailable at any point → STOP and return a user question (binding, do not retry VM startup yourself).
- Container validation stance: container-only; Windows GUI/runtime validation is OPERATOR-RUN on a real Windows 10/11 machine and is recorded as operator-pending.
- The root disk `/dev/sda3` hosts BOTH the container image layers AND the `img-scroller-cargo-target` named volume (Docker volume storage). Disk availability on `/dev/sda3` is therefore the binding constraint for ALL builds. df-gate discipline: reading below threshold → STOP + report (full `df -h` output); the USER manages the space — the implementer never deletes anything.
- Current baseline (research reading 2026-10-31): `/dev/sda3` = 16 GB, used 11 GB, **avail 3.8 GiB (75%)**. This reading is recorded in the MS-1 summary; the MS-4 gate re-reads it.
- Task-1 bindings consumed here: `scripts/dev-checks.sh` semantics (4 PASS/FAIL lines + summary + exit 0/1), `scripts/build-windows.sh` behavior contract (exit 0 → `build-windows: OK` + artifact line + OBSERVED DLL success-note line; typed failures `build-windows: error:`; `.part`-rename; failed run leaves `dist/` untouched; script does NOT run the audit), and the PENDING placeholder locations (script line 15 constant; `docs/build-windows.md` "Observed DLL import list:" paragraph).
- Task-2/3/4 bindings consumed here: binary name `img-scroller` → exe `img-scroller.exe`; deliverable path `dist/windows-x64/img-scroller.exe`; `CARGO_TARGET_DIR=/cargo-target` named volume; 15 container-runnable unit tests already landed.

## 1. Architector decisions (settled NOW; nothing left open)

Reference for D1 sizing: the release GNU build compiles the full eframe dependency tree (eframe 0.37.0 glow + `windows` 0.62.2 + transitive crates) for a NEW target triple; `--release` intermediates (rlibs/rmeta) plus the final exe realistically add ~1.5–2.5 GB into `/cargo-target/x86_64-pc-windows-gnu/release`. The debug dev-check artifacts are already warm (Tasks 2–4 ran `cargo test/check`).

| # | Decision | Value | Rationale |
| --- | --- | --- | --- |
| D1 | df-gate count + thresholds | Exactly TWO gates: **G1** before MS-2 (dev-checks): `/dev/sda3` Avail **≥ 1.5 GiB**. **G2** before MS-4 (release cross-build): `/dev/sda3` Avail **≥ 2.5 GiB** | dev-checks consume only warm-incremental debug bytes (low delta) → 1.5 GiB floor is ample. The GNU release tree adds an estimated 1.5–2.5 GB; a 2.5 GiB floor leaves ≥ ~1 GiB headroom on the 16 GB root disk after the build (keep `df -h` coherent: never so low the container/Docker itself misbehaves). Current 3.8 GiB passes both. Below a threshold → STOP + report with full `df -h` (user manages space). No gate after the build (build already happened; nothing for the gate to prevent). |
| D2 | Separate `cargo test --locked` invocation in MS-3 | **NO** — MS-2's dev-checks output already contains the `cargo test --locked` run and its `test result: ok. 15 passed` line; that line IS the Task-5 evidence for the cargo-test sub-item. A separate invocation adds zero information and spends container time. Recorded as a decision, no extra command | Redundant with MS-2; evidence-clarity argument does not outweigh a duplicate container run. |
| D3 | Timeouts (timeoutMs per VM call) | MS-1 `df -h`: 30,000. MS-2 dev-checks: **1,200,000** (fmt+check+test+clippy on warm incremental). MS-4 build-windows: **3,600,000** (cold release cross-build of the full eframe tree for a fresh target triple — generous by design; slower is expected, never a failure signal while running). MS-5 objdump audit: **300,000** (single static parse of one exe — fast; the margin absorbs container startup). MS-6 sh -n syntax: 60,000. | Sized to the heaviest legitimate step per mandate; `timedOut` (exit null, partial output) = treat the invoking step as indeterminate → rerun that step ONCE, and if it times out again → STOP + report. |
| D4 | OBSERVED DLL list format | **Comma-separated, case-insensitively deduplicated, all-lowercase, alphabetically ascending (ASCII), no wildcards** — e.g. a value shaped like `kernel32.dll, msvcrt.dll, ole32.dll, user32.dll` joined with a comma+space (`, `). Recorded EXACTLY as observed from OUR exe (typical windows-gnu shape: `kernel32.dll` plus CRT entries (`msvcrt.dll` and/or `api-ms-win-crt-<name>.dll` forwarders) plus GUI/com DLLs (`user32.dll`, `gdi32.dll`, `opengl32.dll`, `ole32.dll`, `shell32.dll`, …) — illustrative ONLY; the implementer records the ACTUAL `x86_64-w64-mingw32-objdump -x` output, never this illustration, never any reference project's list) | Deterministic, compact, fits the one-line script constant and the doc code block; lowercasing normalizes objdump's case-preserving `DLL Name:` values. |
| D5 | Commit structure | **ONE commit**: message `Record Wave 1 verification: Windows x64 release build and import audit`; files EXACTLY `scripts/build-windows.sh` + `docs/build-windows.md`. NO push (push is restricted to Step 5, `origin` ONLY). No branch ops | Both fill edits are one logical outcome (the audit result); the verified build/test evidence produces no staged file (exe + logs are gitignored). Matches the repo's plain-descriptive message style. |
| D6 | NO rerun of `build-windows.sh` after the OBSERVED-DLL fill | Decided: **NO rerun**. The exe is already produced and audited; rerunning would trigger a full rebuild just to reprint the success-note line. The filled constant exists for FUTURE runs. `sh -n` syntax check (MS-6) validates the edited script | Saves ~up-to-an-hour of rebuild time; zero verification loss. |
| D7 | Operator-pending recording location | Operator list lives ONLY in this plan (§6 table) + the 4.2/4.5b summaries. **NO repo file is created** for operator items; `docs/build.md` is NOT touched (its Validation-stance section already records the operator-run rule generally) | Avoids a redundant artifacts file and an unnecessary docs edit. |
| D8 | MS-2 failure policy | ANY dev-checks failure (any `FAIL` line, summary `N CHECK(S) FAILED`, exit 1, or missing expected evidence) = a Task 3/4 REGRESSION → capture the FULL output, STOP, report to the caller with the full combined output. NO hotfix, NO source edits, NO authoritative in-draft fix. The caller (Planner) decides the remediation path | Task 5 verifies LANDED code; verification steps do not author fixes. |
| D9 | MS-4 failure policy | Typed `build-windows: error:` output → capture FULL output; classify: (a) lockfile/manifest mismatch (`cargo build failed` with manifest/lock inconsistency in diagnostics) → STOP + report, do NOT regenerate `Cargo.lock`; (b) transient/network-class transient errors (e.g., network hiccups mid-cargo-fetch) → ONE identical retry of MS-4 (same command, same timeout) after re-running `vm_status` + G2; (c) anything else persistent → STOP + report with full output. A failed run leaves `dist/` untouched — NO cleanup by us, NO retry beyond (b's single retry) | Mirrors docs/build-windows.md failure table; retry budget is exactly one identical run. |
| D10 | Audit outcome semantics | The audit records what the exe ACTUALLY imports ("OBSERVED"). Static-linking is evidenced by the ABSENCE of extra runtime DLLs the docs warn about: expected OBSERVED set contains NO `libgcc_*_sjlj*.dll`/`libgcc_dw2*.dll` style entries and NO standalone `libstdc++*.dll`/`libwinpthread-1.dll`-distribution requirement beyond what is observed. If the OBSERVED list shows GNU-runtime DLLs that would need distribution, record them anyway (OBSERVED is binding) and STOP + report the discrepancy before the fill | The docs claim "GNU runtime components are statically linked (no extra runtime DLLs distributed with the app)"; the audit is the evidence. Recording contradictions rather than normalizing them keeps the record honest. |

## 2. Preconditions snapshot (verified during 4.1b; re-checked in MS-1)

- Branch `feat/wave-1-skeleton-open-image` @ `99c4321` (`git log --oneline -8` shows the Tasks 1–4 chain ending at `chore: mark Wave 1 Task 4 done and commit planning artifacts`).
- Host tree CLEAN at plan time (`git status` → `nothing to commit, working tree clean`).
- `src/` tree exactly: `main.rs`, `app.rs`, `app_display.rs`, `image_loading.rs`, `scaling.rs`, `platform/mod.rs`, `platform/windows.rs`, `platform/stub.rs` (matches `.agent/project-structure.md`).
- `Cargo.toml` tracked: `package.name = "img-scroller"`, `version = "0.1.0"`, edition 2024; deps `eframe =0.37.0` (glow), `raw-window-handle =0.6.2`; `[target.'cfg(windows)'.dependencies] windows =0.62.2`; `Cargo.lock` tracked.
- Harness files tracked: `Dockerfile`, `docker-compose.yml`, `scripts/dev-checks.sh`, `scripts/build-windows.sh` (PENDING constant at line 15), `docs/build.md`, `docs/build-windows.md` (PENDING paragraph at lines 58–60).
- VM: running, SSH reachable (`alpine-vm_vm_status`); `/dev/sda3` Avail 3.8 GiB.
- 15 container-runnable unit tests landed by Tasks 2–4 (dev-checks ALL CHECKS PASSED as of Task 4; GNU cross-target `cargo check` green; `/cargo-target` volume warm with debug artifacts).

## 3. Implementation steps (for 4.2 — ordered, atomic, verifiable)

Command envelope (all VM commands are the MCP `alpine-vm_vm_run_command` tool): allowlisted prefixes are `docker`, `sh`, `apk`, `ls`, `cat`, `ps`, `df`, `free`, `uname`, `pwd`, `whoami` — every command below starts with one. Sequence discipline: `vm_status` immediately before EVERY VM command; `exitCode` authoritative; `timedOut: true` (exit null) → rerun the step ONCE, then STOP + report; `stdoutTruncated`/`stderrTruncated` true → capture is partial — note it in the summary and, for MS-2/MS-4 evidence lines, re-derive the specific evidence line with a targeted follow-up read (e.g. `cat` of the dev-checks log path printed at run start; `logs/checks/…` is gitignored, untracked, safe to `cat`).

### MS-1 — Preconditions (single gate block, do first)

1. `vm_status` → expect running + SSH reachable. Unavailable → STOP + user question (binding).
2. Host (local terminal, NOT the VM): `git status` → accept ONLY these: `context.md` (or any `.agent/project-info/context.md`) shown modified, and/or untracked `.kilo/plans/…task5…` artifacts (this plan file). Anything else unexplained (modified `src/`, scripts, docs, manifests) → STOP + report; do NOT stash/checkout fixes yourself.
3. Host: `git log --oneline -1` → expect HEAD `99c4321`… if the caller moved HEAD forward (e.g. planning artifacts commit), accept a NEWER commit on the same branch only if `git status` output remains per step 2; do NOT switch branches.
4. VM: `vm_status`, then `df -h`, `timeoutMs: 30000` → record the `/dev/sda3` Avail reading in the summary as the pre-MS-2 baseline. If Avail < 1.5 GiB (G1) → STOP + report (user manages space).
5. VM share sanity: `ls -la /img-scroller/scripts`, `timeoutMs: 30000` → expect exit 0 listing `build-windows.sh` and `dev-checks.sh`. Absent → the share has not synced; STOP + user question.
6. No unexplained drift: the `src/` list from §2 must hold on the host (verified in step 2 via the clean `git status` — a clean tree at the expected HEAD implies it).

### MS-2 — Dev-checks harness run (verification gate for the dev-checks TODO sub-item)

1. `vm_status`, then re-check G1: `df -h` must show `/dev/sda3` Avail ≥ 1.5 GiB (below → STOP + report).
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`, `timeoutMs: 1200000`.
3. Expected output contract (ALL of these must appear; exit code MUST be 0):
   - One PASS line per check in order: `PASS fmt-check exit=0 (<n> s)`, `PASS check exit=0 (<n> s)`, `PASS test exit=0 (<n> s)`, `PASS clippy exit=0 (<n> s)`.
   - Exactly the summary line `ALL CHECKS PASSED` as the last line.
   - A `test result: ok. 15 passed` line (with `0 failed`); typically a single such line because the package is a binary crate, but if a second `test result: ok.` line appears (doc-tests), each MUST be `ok` and NO `test result: FAILED` may exist anywhere.
   - A log path line `dev-checks starting; log file: logs/checks/dev-checks-<UTC>.log` printed at run start.
4. On ANY dev-checks failure: D8 applies — capture the full combined output (and `cat` the printed log path if capture was truncated), STOP, report to caller with the full output; do NOT fix, do NOT rerun as a workaround, do NOT touch source files.
5. Record in the summary: exit code, the four PASS lines verbatim, the `test result: ok. 15 passed` line verbatim, the `ALL CHECKS PASSED` line, the log path. Evidence lives in the summary + MCP capture — do NOT create evidence files in the repo.

### MS-3 — Test-evidence decision record (no command)

- Decision D2 applies: the `cargo test --locked` line inside MS-2 IS the Task-5 test evidence. NO separate `cargo test --locked` invocation is run. The implementer records this decision verbatim in the summary ("MS-3: single dev-checks invocation is the cargo-test evidence; no separate test call").

### MS-4 — Windows x64 release cross-build (disk-gated)

1. `vm_status`, then re-check G2: `df -h` must show `/dev/sda3` Avail ≥ 2.5 GiB (below → STOP + report with full `df -h`; user manages space — the implementer NEVER deletes anything, including `/cargo-target` content).
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh`, `timeoutMs: 3600000`.
3. Expected output contract (ALL of these; exit code MUST be 0):
   - Exit 0.
   - `build-windows: OK` line.
   - `build-windows: artifact: dist/windows-x64/img-scroller.exe` line.
   - `build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list: PENDING — filled once in Wave 1 verification (Task 5) from the x86_64-w64-mingw32-objdump import audit of our produced exe` — the PENDING placeholder MUST STILL BE VISIBLE here (the fill happens in MS-6 AFTER the audit; cargo diagnostics above may be present and are expected on a warm-but-new-target build).
4. Artifact check: `vm_status`, then `ls -la /img-scroller/dist/windows-x64`, `timeoutMs: 30000` → exit 0 and an `img-scroller.exe` entry with size **> 0 bytes** (the script itself also `-s`-asserted non-empty). A `.part` leftover file would indicate a failed run — its presence in a reported-success run is a STOP + report condition (do not delete it).
5. Volume-redirect verification (Task-1 R3 closure duty): `vm_status`, then `ls /img-scroller/target`, `timeoutMs: 30000` → expect exit 0 BUT the listing should be empty if the dir exists OR the dir absent entirely — EITHER counts as PASS ONLY IF no build artifacts appear in the repo root; there must be NO `target/` tree with contents in the repo root. (The named volume redirect means cargo state lives at `/cargo-target`, NOT `/img-scroller/target`.)
6. Failure handling: D9 applies exactly (one identical retry max after G2 re-check; typed failure capture; STOP on lockfile/manifest mismatch; mid-build disk exhaustion appears as a `cargo build failed` diagnostic → treat as STOP + report class, never as a rebuild trigger while space is not confirmed).

### MS-5 — Import audit (Task-1 retained scope) — runs exactly ONCE, only after MS-4 succeeded

1. `vm_status`, then run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x /img-scroller/dist/windows-x64/img-scroller.exe"`, `timeoutMs: 300000` — the FULL audit output (sections + import tables). This is the single, authoritative audit invocation for Wave 1 (it must NOT be re-run; if capture is truncated, do NOT re-run objdump — instead run the targeted follow-up read in step 2 and note the truncation in the summary).
2. If the full capture is usable, parse the `DLL Name:` lines from the "The Import Tables / Import Address Table" region (objdump prints `        DLL Name: <name>` per import entry). If capture was truncated (or for a clean always-readable record), ALSO run the targeted follow-up read: `vm_status`, then `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x /img-scroller/dist/windows-x64/img-scroller.exe | grep 'DLL Name:'"`, `timeoutMs: 300000`, expect exit 0 — this follow-up read is NOT a second audit of record; it is a filtered view of the same single audit.
3. Materialize the OBSERVED list per D4: strip the `DLL Name:` prefix, trim, lowercase, deduplicate case-insensitively, sort alphabetically ascending, join with `, `. Record the raw `DLL Name:` lines verbatim in the summary, then the derived list (the derived list is what MS-6 writes into the repo files).
4. Audit-verification semantics (D10): the OBSERVED set is the exe's actual imports. Expected static-linking evidence = `kernel32.dll` + CRT entries (`msvcrt.dll` and/or `api-ms-win-crt-*.dll` forwarders) + GUI/com DLLs (e.g. `user32.dll`, `gdi32.dll`, `opengl32.dll`, `ole32.dll`, `shell32.dll`) AND NOTHING that indicates the GNU runtime was dynamically distributed (check: no `libgcc*sjlj*`/`libgcc*dw2*`-style DLL, no standalone `libstdc++*.dll` or `libwinpthread-1.dll`-distribution pattern). Whatever is ACTUALLY observed is what gets recorded — the expected shape is illustrative, never authoritative.
5. Empty/absent `DLL Name:` output (grep follow-up exit 1 with empty stdout) = impossible for a GUI exe → STOP + report (do not fabricate).
6. If D10's discrepancy condition fires → STOP + report BEFORE MS-6 (the fill must not record a contradictory claim silently).

### MS-6 — OBSERVED-DLL placeholder fill (EXACTLY 2 files, then syntax re-check)

Precondition: MS-4 exit 0 + non-empty exe + MS-5 OBSERVED list derived (name it `$OBSERVED_LIST` below = the D4-format comma-joined lowercase list from MS-5 step 3).

Edit 1 — `scripts/build-windows.sh` (host file, `edit` tool, ONE line, rest of file byte-exact):
- Find (line 15, exact current text):
  `readonly OBSERVED_DLL_IMPORT_LIST="PENDING — filled once in Wave 1 verification (Task 5) from the x86_64-w64-mingw32-objdump import audit of our produced exe"`
- Replace with (TEMPLATE — `$OBSERVED_LIST` is materialized from MS-5 evidence):
  `readonly OBSERVED_DLL_IMPORT_LIST="$OBSERVED_LIST"`
- Example of a correctly materialized line (ILLUSTRATIVE, not prescriptive): `readonly OBSERVED_DLL_IMPORT_LIST="kernel32.dll, msvcrt.dll, ole32.dll, user32.dll"`
- Rules: no other line changes; quote characters unchanged (the new string is plain double-quoted); lowercase; `, ` separators; no trailing period inside the value; the em-dash placeholder wording disappears entirely.

Edit 2 — `docs/build-windows.md` (host file, `edit` tool, ONE paragraph):
- Find (exact current text, lines 58–60):
  `Observed DLL import list:`
  (blank line)
  `    PENDING — recorded during Wave 1 verification (first produced exe, audited with x86_64-w64-mingw32-objdump).`
- Replace the PENDING line ONLY with this TEMPLATE (indented = code block, real newlines):
      $OBSERVED_LIST

      Audited 2026-10-31 with `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` inside the container (Wave 1 verification; first produced exe). List is deduplicated and lowercase.
  — where `$OBSERVED_LIST` is the identical literal string used in Edit 1, and the audit date is THE ACTUAL IMPLEMENTATION DATE (use the real date of the 4.2 run; 2026-10-31 stands unless the run happens on another day — then use that day's date).
- The `Observed DLL import list:` header line stays verbatim; the file's `## Toolchain / target notes` static-linking bullet and every other section stay byte-exact; NO other doc changes anywhere.

Then syntax re-check ONLY (D6: NO script rerun, NO build rerun):
1. `vm_status`, then `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -n scripts/build-windows.sh`, `timeoutMs: 60000` → expect exit 0. Non-zero → re-check the edit for quoting/line-ending corruption (LF, no BOM rules from Task 1), fix ONLY the edited line, re-verify once; persistent → STOP + report.
2. Re-read both edited files and confirm: script line count unchanged, the ONLY script diff is the one constant line; the ONLY doc diff is the replaced paragraph (host `git diff` must show exactly those hunks — anything else → undo the extra change before committing).
3. The audit evidence feeding `$OBSERVED_LIST` comes from MS-5 ONLY — never re-run objdump to "confirm" the fill (single audit of record, D6/D10).

### MS-7 — Operator-pending record (no file)

- Per D7: record in the 4.2 summary (and this plan's §6 table) that the OPERATOR-RUN manual UX items are PENDING and were NOT executed by agents. The operator list for THIS wave (real Windows 10/11 machine, runs `dist\windows-x64\img-scroller.exe`): (1) open jpg/png from CLI argument; (2) open jpg/png from the in-app dialog; (3) try a file with a format not registered in the system (e.g., an obscure extension) → in-window message appears; (4) window resize keeps the image visible; (5) Task-4 items: fit display correct on open, resize sanity, message-after-a-failed-open does not lose a previously displayed image. NOT recorded into any repo file.

## 4. Commit procedure (per D5) — exact steps with gitignore compliance

1. Read `.gitignore`; confirm `dist/`, `logs/`, `*.log`, `target/` patterns still cover everything produced this step (the exe, `logs/checks/*.log`, any `target/` leak — none may be staged).
2. Host `git status` → expect modifications ONLY: `scripts/build-windows.sh`, `docs/build-windows.md`, plus possibly `.agent/project-info/context.md` (caller/planner-owned; if IT is the only other change, leave it UNSTAGED — commit only the two files below; if anything else appears, STOP + report).
3. Stage ONLY: `git add scripts/build-windows.sh docs/build-windows.md`.
4. Re-run `git status` → staged list exactly the two files; no `dist/`, `logs/`, `*.log`, `target/`, no untracked leftovers staged. Explicitly verify the built exe and the dev-checks log are NOT staged (gitignored).
5. Commit with EXACTLY this message (one commit, D5):
   `Record Wave 1 verification: Windows x64 release build and import audit`
6. `git status` confirms clean tree afterward. NO push (Step-5 restricted, `origin` ONLY). No branch ops. Record the commit hash in the summary.

## 5. Alpine-VM operational intent (encoded)

- Command order in 4.2 is MS-1 → MS-2 → (MS-3 decision only) → MS-4 → MS-5 → MS-6 → MS-7 → (§4 commit); the audit (MS-5) NEVER precedes a successful build (MS-4), and the fill (MS-6) NEVER precedes the audit.
- Every VM command: `vm_status` first; exit code authoritative; absent cargo stderr ≠ failure; truncated capture ≠ failure (follow-up `cat` of the printed dev-checks log path is the sanctioned re-derivation route).
- `FORCE_FAIL` is NOT used in any MS step at any time.
- The named volume `img-scroller-cargo-target` ends this task holding BOTH the warm debug dev-check state AND the new `x86_64-pc-windows-gnu/release` artifacts — retained for future waves (§8).

## 6. Verification mapping (for 4.5b — Overall Plan Adherence)

| TODO Task 5 sub-item / Task-1 binding | Verify in 4.5b by | Pass state |
| --- | --- | --- |
| Dev-checks via harness (`ALL CHECKS PASSED`) | 4.2 summary shows MS-2 exit 0 + 4 PASS lines + `test result: ok. 15 passed` + `ALL CHECKS PASSED` verbatim | All present, exit 0 |
| `cargo test --locked` covers the container-runnable unit tests | Same MS-2 evidence line (D2 recorded separately in summary) | `test result: ok. 15 passed` present |
| Windows release cross-build exits 0, non-empty exe at `dist/windows-x64/img-scroller.exe` | MS-4 exit 0 + `build-windows: OK` + artifact line + `ls -la` showing non-zero size exe | Exit 0 + artifact listed |
| Import audit ONCE (`x86_64-w64-mingw32-objdump` on OUR exe) | Summary contains the raw `DLL Name:` lines (MS-5) and the derived D4-format list; EXACTLY one audit invocation (full read counts as the audit; grep view is a filtered read, not a second audit) | Present + single-invocation confirmed |
| OBSERVED DLL list recorded in script success-note constant | `scripts/build-windows.sh` line 15 = `readonly OBSERVED_DLL_IMPORT_LIST="<materialized list>"`; byte-exact elsewhere; `sh -n` exit 0 after edit | Content match |
| OBSERVED DLL list recorded in `docs/build-windows.md` | PENDING paragraph replaced per §3 MS-6 template; header + static-linking bullet untouched; no other diff | Content match + `git diff` scope |
| NO build-script rerun after fill (D6) | Summary states no re-invocation occurred; `git log` shows no later build commit noise | Summary + log check |
| Operator-run manual UX item | MS-7 summary text lists the 5 operator items as PENDING; NO operator file created; `docs/build.md` diff-free | Summary check |
| Volume-redirect verification (Task-1 R3) | `ls /img-scroller/target` shows no populated target tree in the repo root | No `target/` contents in repo root |
| Commit hygiene | ONE commit (D5 message), staged = exactly 2 files, nothing gitignored staged, nothing pushed, clean tree after | Git log/status check |
| Disk-gate compliance (D1) | Summary records both `df -h` readings (G1 pre-MS-2, G2 pre-MS-4) and confirms thresholds held (or the STOP precedes the run) | Readings recorded |

## 7. Risks and mitigations

| # | Risk | Mitigation / handling |
| --- | --- | --- |
| R1 | Disk exhaustion on `/dev/sda3` during the GNU release build (root disk hosts image layers + the named volume) | D1 df-gates (G1 1.5 GiB / G2 2.5 GiB) BEFORE the runs, STOP discipline, user manages space; volume consumption stays in the named volume (root disk), repo root stays out of the build path; mid-build exhaustion surfaces as `cargo build failed` → STOP + report class per D9; implementer NEVER deletes `/cargo-target` or anything else |
| R2 | MS-4 first-release-compile duration (full eframe tree, fresh target triple) | `timeoutMs 3600000` (D3); `timedOut` = indeterminate, not failure → rerun once, then STOP + report |
| R3 | Typed failures misread as recoverable | D9 classes: lockfile/manifest mismatch → STOP (never regenerate the lock); transient → exactly ONE identical retry; anything else persistent → STOP + full output; failed run leaves `dist/` untouched — NO cleanup by us |
| R4 | objdump parse pitfalls (forwarders, casing, duplicates) | D4 deterministic format (comma-separated, deduped case-insensitively, lowercase, alphabetical); the `grep 'DLL Name:'` filtered follow-up read keeps evidence extractable without re-running the audit; `api-ms-win-crt-*` forwarders and `kernel32`/`msvcrt`-style entries are all recorded as OBSERVED — never normalized away or copied from a reference list |
| R5 | Static-linking claim contradicted by the OBSERVED list (GNU runtime DLLs appearing) | D10: record OBSERVED as-is, STOP + report BEFORE the fill — the fill never silently records a contradictory claim |
| R6 | Script edit corrupts quoting/line endings on the Windows host → VM share | LF/no-BOM rules from Task 1 still apply; `sh -n` syntax check after Edit 1 (MS-6) is the deterministic catcher; only the one line may be fixed on a failure |
| R7 | Accidental build-script rerun after the fill (would trigger a needless full rebuild) | D6 explicitly forbids reruns; `sh -n` is the only post-fill execution |
| R8 | MCP capture drops cargo stderr / truncates long cargo output | Exit code authoritative; truncated captures re-derive via `cat logs/checks/dev-checks-<UTC>.log` (gitignored, untracked, safe to read); absent stderr is never a failure signal |
| R9 | A dev-checks failure now (regression from Task 4 landing) | D8: STOP + full output to caller; no hotfix — Task 5 is verification of LANDED code |
| R10 | `.part` leftover or a stale exe from a previous partial run confusing artifact verification | MS-4 step 4 treats unexpected `.part` presence in a success run as STOP + report; `dist/` state is never "cleaned up" by the implementer |

## 8. Cross-task bindings (carry forward)

- Task 6 (4.1b) consumes: the recorded verification outcomes (MS-2 evidence, MS-4 exit 0 + artifact path, MS-5 OBSERVED DLL list + audit date) to update README.md / tech.md / architecture.md, and the final context.md closing write by the Planner. The OBSERVED list in the doc IS the materialized `$OBSERVED_LIST` — Task 6 copies from it, never re-audits.
- The `/cargo-target` named volume state (debug warm-up + GNU release artifacts) REMAINS after this task — it is an asset for future waves (faster rebuilds); NO cleanup unless a task fails for disk reasons and the caller/user authorizes it.
- The operator-pending list (MS-7) stays unexecuted until operator input; Task 6/Step-5 flows must NOT tick those items as agent-verified.

## 9. Junior-proof command annex (copy-exact)

| Step | VM command | timeoutMs | Expected |
| --- | --- | --- | --- |
| MS-1 G1 | `df -h` | 30000 | `/dev/sda3` Avail ≥ 1.5 GiB |
| MS-1 | `ls -la /img-scroller/scripts` | 30000 | exit 0; both .sh files listed |
| MS-2 | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` | 1200000 | exit 0; `PASS fmt-check/check/test/clippy` lines; `test result: ok. 15 passed`; `ALL CHECKS PASSED` |
| MS-4 G2 | `df -h` | 30000 | `/dev/sda3` Avail ≥ 2.5 GiB |
| MS-4 | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` | 3600000 | exit 0; `build-windows: OK`; artifact line; PENDING still in DLL line |
| MS-4 check | `ls -la /img-scroller/dist/windows-x64` | 30000 | exit 0; non-empty `img-scroller.exe` |
| MS-4 check | `ls /img-scroller/target` | 30000 | no populated target tree in repo root |
| MS-5 audit | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x /img-scroller/dist/windows-x64/img-scroller.exe"` | 300000 | exit 0; full objdump output |
| MS-5 filtered read (1) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x /img-scroller/dist/windows-x64/img-scroller.exe \| grep 'DLL Name:'"` | 300000 | exit 0; ≥ 2 DLL Name lines |
| MS-6 | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -n scripts/build-windows.sh` | 60000 | exit 0 |

(1) The grep pipe is INSIDE `sh -c` (container-side); the MCP command still starts with the allowlisted `docker` prefix. The two MS-5 commands are one audit of record + one filtered read of that SAME audit — never two audits.


## 10. Completion report shape (4.2 return to caller)

- Plan path (this file).
- Decisions honored: D1 thresholds held (both `df -h` readings), D2 no-separate-test-call recorded, D3 timeouts as listed, D4 OBSERVED list format, D5 single commit + hash, D6 no rerun recorded, D7 operator-record decision, D8–D10 not fired (or fired with full evidence).
- Evidence quotes (verbatim): 4 PASS lines; `test result: ok. 15 passed`; `ALL CHECKS PASSED`; `build-windows: OK`; artifact line; raw `DLL Name:` lines; the final `ls -la` artifact listing.
- Files changed: exactly the 2 fill files; commit hash.
- Operator items: PENDING list (MS-7) restated.
- Any STOP encountered: full captured output + the gate/step that fired.
