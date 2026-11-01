# Verification Plan — Wave-2 UI Fixes Cycle B (TODO Task 4): Container Harness + Exe Rebuild + OPERATOR-RUN Handoff

Date: 2026-11-01
Workflow step: Cycle B → Step 4.1b (architector)
Source TODO: [`.agent/todos/20261101/20261101-todo-1.md`](../../.agent/todos/20261101/20261101-todo-1.md) → Task 4 (verification + exe refresh + operator re-test item)
Global plan: [`.kilo/plans/20261101-wave2-ui-restyle.md`](../../.kilo/plans/20261101-wave2-ui-restyle.md) (§2.2 grouping: Task 4 = Cycle B, no 4.1a, 4.5b only)
Implementation plan (Cycle A): [`.kilo/plans/20261101-floating-controls.md`](../../.kilo/plans/20261101-floating-controls.md) (§6 test arithmetic, §7 line counts, §8.3 hazards)
Front-end verification input: [`.kilo/plans/20261101-floating-controls-frontend-verification.md`](../../.kilo/plans/20261101-floating-controls-frontend-verification.md) (19/19 PASS; 3 recorded compile-correct plan diffs)
Build docs: [`docs/build.md`](../../docs/build.md), [`docs/build-windows.md`](../../docs/build-windows.md); harness: [`scripts/dev-checks.sh`](../../scripts/dev-checks.sh), [`scripts/build-windows.sh`](../../scripts/build-windows.sh)

**Landed state at planning time (git-verified):**
- Branch `fix/centered-floating-controls`; HEAD `f7566cb` (`chore: mark UI-fix Tasks 1-3 done (floating controls) + frontend verification report`); version `0.2.1` (`10d0253`).
- Cycle A chain: `6ef0240` (feature) → `f30b802` (review fix: `Area::show(ctx)` receiver, `Color32::from_rgba_unmultiplied_const`, `Id::unique`) → `f7566cb` (TODO Tasks 1–3 marks + 4.5a report committed).
- Working tree: ONLY `.agent/project-info/context.md` modified (unstaged; planner/caller territory — NOT staged in Cycle B).
- Previous exe on disk: `dist\windows-x64\img-scroller.exe` = **19,464,396 bytes** (the 0.2.0 Wave-2 build; this cycle's rebuild overwrites it — old size recorded above for the diff evidence).
- Wave-2 landed mega-facts: `.gitattributes` pins `*.sh text eol=lf` (`cf244d5`) — harness reaches the container clean; container quartet green 26/26 at `f30b802` (implementer-provided evidence, re-proven authoritatively by the run below).

Approval: user AUTO-APPROVED per-task plans (approval mode 2026-11-01) — every decision below is DECISIVE; zero judgment remains for the implementer.
Status: PLAN — Cycle B step 4.2 (implementer) executes §2–§6 in order; no code changes anywhere in this cycle.

---

## 0. Scope Guard

This cycle ships EXACTLY: (a) one authoritative container dev-checks run, (b) one Windows-x64 exe rebuild + DLL import audit, (c) a gitignore sweep, (d) the OPERATOR-RUN re-test handoff text (§7). NOTHING else:

- NO source-code edits of any kind. NO script edits (`scripts/*.sh` read-only — a defect there is REPORTED, never patched here).
- NO new unit tests — decision PINNED in §1.2 (census stays 26; no file arithmetic).
- NO commits in step 4.2 (§5: no new tests ⇒ no test commit). The only commit of this cycle is the step-4.6 `[x]` marks commit, whose staging list §5.2 fixes exactly.
- TODO marks happen ONLY in step 4.6, per overwrite-todo-prevention (mark-only edits; Task 4's OPERATOR-RUN sub-item stays `[ ]`).
- NO documentation edits (Cycle C owns `README.md`/`tech.md`/`architecture.md`/`context.md`/`.agent/project-structure.md`/`docs/*`, incl. any OBSERVED-DLL refresh). NO `CHANGELOG.md`. NO edits to existing `.kilo/plans/*` files.
- OFF-LIMITS: `.agent/project-info/context.md` remains unstaged in every cycle-B command (planner upkeep).

## 1. Test-census expectations (TODO Task 4 bullet 1)

### 1.1 Bullet → test mapping (grep-verified at HEAD `f7566cb`; `#[test]` census over `src/`)

**Final census: 26 tests — the line the harness MUST print, arithmetic:**

| Test file | Tests | Covers |
| --- | --- | --- |
| `src/app/tests.rs` | 9 | State level: `navigate`/`neighbor_image_path`/`accept_open_outcome`/<br>`empty_state`/`APP_WINDOW_TITLE` — all STATE assertions, zero renderer references (verified Cycle-A impl plan §6.1 while planning; 4.5a item 19 re-verified). |
| `src/app/floating_controls_tests.rs` | 1 | `arrow_button_enablement_follows_the_navigation_edges` — the relocated rails enablement test (`git mv`-equivalent rename `rails_tests.rs` → `floating_controls_tests.rs`, `R055` in 4.5a evidence §4); SAME semantics: empty state → both disabled; first sibling → ← disabled/→ enabled; last sibling → ← enabled/→ disabled. This is the TODO's "state-level enablement/edge" test. |
| `src/folder_listing/tests.rs` | 5 | 17-extension whitelist + collation ordering (untouched by this TODO). |
| `src/image_loading.rs` | 4 | Decode path (untouched by this TODO). |
| `src/app_display.rs` | 2 | Fit-display paint state (untouched by this TODO). |
| `src/scaling.rs` | 5 | Fit-scale computation (untouched by this TODO). |
| **Total** | **26** | **9 + 1 + 5 + 4 + 2 + 5** |

TODO Task-4 bullet mapping verdicts:

| Task-4 bullet | Maps to |
| --- | --- |
| "update/extend where the state/logic/UI wiring changes touch them" | SATISFIED BY THE LANDING: the only test-touching change was the relocation in `floating_controls_tests.rs` (already landed, committed `6ef0240`); `app/tests.rs` verified untouched. |
| icons visible / floating pill / center Open | UI-visual paint-plane properties — NOT unit-testable in this codebase (no egui headless rig exists; adding one = new dependency = scope violation). PINNED as covered by (a) the 4.5a static verification (19/19 PASS, commit-recorded) and (b) the OPERATOR-RUN re-test (§7 handoff). |
| keys unchanged; edge-stay-silently | EXISTING state tests in `src/app/tests.rs` (9) assert `navigate`/edge behavior on the unchanged seam. |

### 1.2 NEW-tests decision — PINNED: NONE

The Cycle-A change added NO state, NO state transition, and NO new logic seam (struct fields byte-identical per 4.5a item 2; `navigate`/`neighbor_image_path`/`message_text` untouched; UI-click is the only new entry point and it drives the already-tested `navigate` seam). The single testable state-level area (enablement edges) is already covered by the relocated test. **Decision: add NOTHING. Default invoked exactly as directed.** Test count stays 26; no file arithmetic changes; §5.1 or-statement resolves to the "no new tests ⇒ NO new commits" branch.

**Census failure rule:** if the harness `cargo test` summary does not read exactly `26 passed; 0 failed` (doc-tests line `0` is expected and fine), STOP and report the full test section to the caller — a census mismatch is a defect, never a local patch.

## 2. Order of operations (execute top-down; one command per call)

Pre-VM host checks (host bash tool):

1. `git branch --show-current` → MUST print `fix/centered-floating-controls`; mismatch → STOP, ask the caller.
2. `git status` → MUST show only `modified: .agent/project-info/context.md` (unstaged). Anything else unexpected → STOP, ask the caller. Read `.gitignore` (commit-time sweep reference for §5).
3. `git log --oneline -1` → confirm HEAD `f7566cb`.

Then, in the Alpine VM via the `alpine-vm` MCP (rules per `docs/build.md` §alpine-vm MCP usage notes; see §6 gate notes):

4. **Call `vm_status` first** → proceed only when running + SSH-reachable; if not running/SSH-reachable → STOP and return a user question.
5. Harness run (§3).
6. `vm_status` again (before every subsequent VM call — re-verify each time).
7. Pre-build size record: `ls -l /img-scroller/dist/windows-x64/img-scroller.exe` → expect `19464396` (the 0.2.0 artifact; recorded so the post-build delta is evidence, not assumption).
8. `vm_status` again.
9. Exe rebuild (§4.1).
10. `vm_status` again.
11. Post-build artifact check: `ls -l /img-scroller/dist/windows-x64/img-scroller.exe` → non-empty size recorded (the authoritative size evidence; the exe lives on the VM share `/img-scroller/dist/...`).
12. `vm_status` again.
13. DLL import audit (§4.2).
14. `vm_status` again.
15. Gitignore sweep host-side: `git status` → tree must be unchanged from step 2 (dist/, logs/, target/ all gitignored; `context.md` still modified-unstaged). Verify no ignore-matching path is staged anywhere (there is nothing to stage in 4.2).
16. Report back: harness exit 0 + `ALL CHECKS PASSED` + exact log path + the 26-test line; old + new exe sizes; DLL audit result vs the 22-DLL constant; `git status` screenshot-equivalent text.

## 3. Step 4.2-A — container dev-checks harness (TODO Task 4 bullet 2)

### 3.1 Command (exact, single call)

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
```

Set tool `timeoutMs: 900000` (full check+test+clippy chain on a shared volume historically runs well under this; the contract converts a timeout into `timedOut` + partial output, which is REPORTED, never retried silently).

### 3.2 Expected output semantics (authoritative per `docs/build.md` §Expected output semantics)

- First line: `dev-checks starting; log file: logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` — **RECORD the exact path** (UTC timestamp; the tree is gitignored `logs/`).
- Four check lines in this fixed order (all four always run):
  1. `PASS fmt-check exit=0 (<n> s)`
  2. `PASS check exit=0 (<n> s)`
  3. `PASS test exit=0 (<n> s)` — with the cargo test output above it showing a `test result: ok. 26 passed; 0 failed; 0 ignored; ...` line (§1.1 census; a doc-tests `test result: ok. 0 passed` line is expected and normal for a binary crate).
  4. `PASS clippy exit=0 (<n> s)`
- Final line: `ALL CHECKS PASSED`; **script exit code 0** is the authoritative success evidence (captured MCP stderr may be missing/empty cargo noise — per §6 stderr absence is NOT a failure signal).

### 3.3 Log verification (cheap, optional but recommended)

The harness streams its full output to stdout, so the stdout capture is normally ALL the evidence needed. Only consult the log file if that stdout evidence is ambiguous: read it HOST-side from `C:\repo\img-scroller\logs\checks\dev-checks-<...>.log` with the read tool (the VM share persists it; never stage it — gitignored `logs/`). NEVER read the log with a VM `cat` call — the container working directory is NOT the project root and the log already exists host-side.

### 3.4 Failure handling

Any `FAIL <name> exit=<n>` or `N CHECK(S) FAILED` (exit 1) → STOP. Report: the full four result lines + the recorded log path + the failing check's tail output to the caller. Expected remediation shapes (for the caller's 4.3 fix cycle — NOT executed here): fmt drift → container `cargo fmt` (hazard §8.3-i of the Cycle-A plan: never manual reflow); check/test/clippy compile or lint defects → defect fix plan; then re-run §3 once. `FORCE_FAIL` injection is NOT used in this cycle (fault-injection was proven in Wave 1).

## 4. Step 4.2-B — Windows x64 exe rebuild + DLL audit (TODO Task 4 bullet 3)

### 4.1 Rebuild command (exact, single call)

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh
```

Set `timeoutMs: 900000`.

Expected (per `docs/build-windows.md`):

- `cargo build --release --locked --target x86_64-pc-windows-gnu` passes (reusing the named target volume).
- Output lines: `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, plus the script's static-linking OBSERVED-DLL evidence line.
- **Exit code 0** = authoritative.
- The script atomically overwrites the on-disk exe (`.part` → rename); a failed run leaves the previous artifact byte-intact (failure lines all start `build-windows: error:` — the typed table in `docs/build-windows.md` is the diagnostic map; any failure → STOP and report the exact typed error).

Then step §2-11 records the NEW size via `ls -l` (expected: different from 19,464,396 B because `src/app.rs`/`floating_controls.rs`/`arrow_direction.rs` changed since the 0.2.0 build; ANY non-empty size is acceptable — record it verbatim).

### 4.2 DLL import audit (compare vs the recorded 22-DLL constant)

The build script does NOT run the audit itself (`docs/build-windows.md` §Static-linking). Run ONE audit command:

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep -i 'DLL Name' | tr 'A-Z' 'a-z' | sed 's/.*dll name: //' | sort -u"
```

(single composed VM command; allowed `docker` prefix; set `timeoutMs: 180000`).

**Expected: exactly 22 lowercase DLL names, matching SET-EQUAL the recorded constant** (`OBSERVED_DLL_IMPORT_LIST` in `scripts/build-windows.sh` line 15; identical list in `docs/build-windows.md`):

```
advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll,
api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll,
gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll,
opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll,
ws2_32.dll
```

Verification method: count the output lines (must be 22) and compare name-by-name against the list above (order-insensitive; the `sort -u` already dedups + lowercase-folds).

**On ANY diff (extra, missing, or renamed DLL):** STOP. Report the full observed list + the delta vs the constant to the caller. NO edits to `scripts/build-windows.sh` or `docs/build-windows.md` — recording a new OBSERVED-DLL baseline is a Planner/Cycle-C decision, never an implementer improvisation (Wave-1 verification precedent).

If the filtered pipeline yields no output at all (report-format drift), fall back to the raw dump for manual reading: `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` (large output — read with care; still ONE call).

## 5. Git plan for Cycle B

### 5.1 Decision — PINNED from §1.2: NO new unit tests ⇒ NO new commits in step 4.2

Nothing is committed, staged, force-added, or amended during §2 steps 4–14. Specifically NEVER staged: `logs/**` (gitignored `logs/`), `dist/**` (gitignored `dist/`), `target/**` (gitignored `target/`), `Cargo.lock`/`Cargo.toml` (unchanged this cycle), `.agent/project-info/context.md` (upkeep = caller/planner territory), any plan/spec/verification report other than §5.2's two files.

### 5.2 Step 4.6 marks commit (the ONLY commit of Cycle B)

In step 4.6 (after 4.3 review + 4.5b adherence, per the global plan Cycle-B step map), the implementer commits the Cycle-B evidence into the marks commit:

- TODO edits (mark-only, nothing else touched):
  - `- [ ] Unit tests: ...` line of Task 4 → `- [x] Unit tests: ...` with the census evidence appended in the line's own scope as the call for the record: final census **26** unchanged (9 app-tests + 1 floating-controls + 5 folder_listing + 4 image_loading + 2 app_display + 5 scaling); `ALL CHECKS PASSED` + exact log path; harness exit 0.
  - `- [ ] Rebuild the Windows exe ...` line of Task 4 → `- [x]` with: rebuilt exit 0; new exe size in bytes; DLL audit == the recorded 22-DLL constant.
  - `- [ ] Manual UX re-check (OPERATOR-RUN by the user ...)`: **STAYS `[ ]`** until the user reports back (binding TODO note).
- Staging list (EXACT): `.agent/todos/20261101/20261101-todo-1.md` + `.kilo/plans/20261101-floating-controls-verification.md` (this plan — same Wave-2 marks-commit precedent as `f7566cb`). Nothing else.
- Commit message EXACTLY:

```
chore: record floating-controls cycle B verification evidence (harness + exe rebuild)
```

- Commit protocol: re-read `.gitignore`, run `git status` before committing; confirm no gitignore-matching path staged; watch that `context.md` stays unstaged. NO push (push restricted to workflow Step 5, `main` → `origin` ONLY). NO amend/force.

## 6. Alpine-VM MCP notes (binding for every VM call in §2)

- `vm_status` BEFORE **every** `vm_run_command` call (docs/build.md: "Call vm_status BEFORE any command"); only proceed when running + SSH-reachable; otherwise STOP and return a user question.
- ONE command per `vm_run_command` call; multi-step composition only as ONE `sh -c "<pipeline>"` string (§4.2 audit), never host-side chaining.
- The command **EXIT CODE is the authoritative** success/failure evidence (`exitCode` field; `null` exitCode + `timedOut` = report the timeout, do not silently retry).
- Captured MCP output **may lack cargo stderr — an absent stderr is NOT a failure signal** (Wave-1 documented MCP caveat).
- Prefer a generous `timeoutMs` (§3.1/§4.1: 900000) over aggressive timeouts; a timeout is REPORTED, never assumed recovered.
- Never attempt VM-side writes to `dist/`/source (vboxsf EBUSY hazard is read-safe here: the rebuild performs container-side writes through its own mounted worker, and §2's reads are pure `ls`).

## 7. OPERATOR-RUN re-test handoff text (for the Step-6 close; pass to the user VERBATIM)

```
OPERATOR-RUN RE-TEST — img-scroller v0.2.1 (floating-controls rebuild)
Machine: a real Windows 10/11 computer. App: dist\windows-x64\img-scroller.exe (the freshly rebuilt one).

1. START: launch the exe with no file argument. Verify the empty state:
   - NO toolbar, NO corner rails anywhere;
   - ONE big centered "Open…" button (18 pt label, large 180×48 button);
   - NO arrow buttons visible in the empty state.
2. OPEN: click the big centered Open… — the native Windows file dialog opens.
   Pick an image whose folder contains at least one other whitelisted image (png/jpg/gif/webp/bmp/ico/tiff...).
3. ICONS: with the image open, the bottom-center pill shows four PAINTED triangles
   (left / right / up / down arrows). Verify all four are VISIBLE FILLED ARROWS —
   NOT empty/tofu boxes — the fix for the Wave-2 defect.
4. PILL: verify it is one floating translucent-dark pill at the bottom-center
   (not a space-reserving rail/toolbar); the two folder arrows (up/down) are dimmed
   (Wave-3 reserve); left/right are bright.
5. NAVIGATE (buttons): click ← and → — they go to the previous/next sibling image;
   at the first/last image the edge arrow is dimmed and clicking it does nothing (no wrap);
   open-flow failures (e.g. a corrupt/unsupported sibling) keep the displayed image and show
   a small message line ABOVE the arrows, while an image stays open.
6. MESSAGE: verify the message appears ONLY while an image is open — it never shows in the
   empty state.
7. OPEN VISIBILITY: verify the big centered Open… is GONE while an image is open
   (no Open control anywhere).
8. KEYS: verify Left/Right arrow keys still navigate exactly as before; no other key behavior changed.
9. CLOSE the image (open the native dialog and press Cancel to reach the empty state) and confirm the empty state returns exactly as in step 1.
10. EXTRA: note the pill's readability over a bright/light image (recorded risk in the frontend
    verification report). Report pass/fail per item and any visual issue verbatim.
```

## 8. Verification mapping (Task 4 bullets → this plan)

| TODO Task-4 bullet | Covered by |
| --- | --- |
| Unit tests updated/extended where the state/logic/UI wiring changes touch them; census recorded; container dev-checks; `alpine-vm` rules | §1.1 mapping + §1.2 pinned no-new-tests decision (census 26) + §3 harness protocol + §3.2 26-test line + §5.2 marks evidence format |
| Rebuild the Windows exe for the operator re-test | §4.1 (exact command; exit 0 authority; size recorded pre/post) |
| DLL expectations vs the recorded constant | §4.2 (22-DLL set equality; diff → caller report, no edits) |
| gitignore compliance + git plan | §5 (no 4.2 commits; 4.6 marks-only commit; sweep in §2 step 15) |
| Manual UX re-check (OPERATOR-RUN) | §7 handoff text (sub-item stays `[ ]` in 4.6) |

Cycle-A hazard leftovers resolved HERE authoritatively (Cycle-A §8.3): (g) const-Color32 — resolved by the landed `from_rgba_unmultiplied_const` (`f30b802`); (h) `&mut app` capture — resolved (container builds green); (i) rustfmt drift — §3.4 remediation path pinned if `fmt-check` fails; (j) clippy — `-D warnings` gate runs in §3.

## 9. Out of scope (explicit)

- Any source, script, docs, CHANGELOG, plan-edit, TODO-content, or `.agent/project-info/context.md` write.
- Wave-3 Up/Down wiring, pan, zoom, menu placeholder — every Wave-2 out-of-scope list stays binding.
- New tests / new dependencies / headless egui rig (§1.2 pinned NONE).
- Any commit during step 4.2 (§5.1); any push (workflow Step 5 owns the only one).
- No retry loops: a failing run is reported, the caller decides (fix cycles are 4.3 territory).
