# Wave 6 — Task 4 (Cycle C) Verification Plan: "Open with..." Menu

- Source TODO (scope of THIS step): `.agent/todos/20261029/20261029-todo-8.md` → `### Task 4: Verification` (2 sub-items, verbatim below).
- Global plan: `.kilo/plans/20261112-wave6-open-with.md` → `### Cycle C — TODO Task 4 (verification)`.
- Wave artifacts verified against:
  - `.kilo/plans/20261112-wave6-tasks1-2-enumeration-launcher-implementation-plan.md` (Cycle A: enumeration + launcher)
  - `.kilo/plans/20261112-wave6-task3-open-with-menu-implementation-plan.md` (Cycle B: menu UI)
  - `.kilo/plans/20261112-wave6-task3-frontend-verification.md` (10 OPERATOR-RUN items)
  - `.kilo/plans/20261112-wave6-task3-adherence.md` (structure-map corrective step — landed at `caa1aa0`)
- Branch: `feat/wave-6-open-with`; branch HEAD at plan time: `49b3b01` (working tree clean); last code commit `c00d427`+`fa0a6ac` (Task 3 feature+simplification; everything after is docs/marks only).
- Version: 0.6.0 (bumped at `f0b2274`).
- Harness: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` on the Alpine VM via the `alpine-vm` MCP — `vm_status` FIRST; the command EXIT CODE is the authoritative success evidence; absent cargo stderr is NOT a failure signal (`docs/build.md`).
- Executor: JUNIOR developer under 50% restriction — every step below is FINAL; execute in this exact order; no redesign, no extra sweeps, no out-of-scope fixes.

## TODO Task 4 verbatim (scope of this verification)

1. "Unit tests where pure logic exists (e.g., parsing/filtering of enumeration results represented in a testable seam) — run via the dev-checks harness in the container per `docs/build.md` (`ALL CHECKS PASSED`: fmt/check/test/clippy; `alpine-vm` MCP; `vm_status` first; the command exit code is the authoritative success evidence)."
2. "Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): for a jpg/png file, the menu lists at least the system-associated programs; selecting one opens this exact file in that program; NOT dependent on any app-maintained list."

Sub-item 1 is agent-executable (Steps 1–6). Sub-item 2 is the USER's gate: the OPERATOR-RUN checklist (§11) is ASSEMBLED by this step and HANDED to the caller; the TODO sub-item stays `[ ]` (the 4.6 step marks ONLY sub-item 1).

---

## Baseline facts (pinned; all verified at plan time)

| Fact | Value | Source |
| --- | --- | --- |
| Branch / HEAD | `feat/wave-6-open-with` @ `49b3b01`, tree clean | `git status`, `git log --oneline -20` |
| Version | 0.6.0 (`Cargo.toml` line 3) | `Cargo.toml` |
| Pre-wave census | 164 unit tests / 26 test files | `tech.md` § Testing |
| Census after Cycle A | 170 / 27 (+6: `src/open_with_listing/tests.rs`) | Cycle-A plan §4 |
| Census after Cycle B (current, expected) | **177 unit tests / 28 test files** (+7: `src/app/open_with_button_tests.rs`) | Cycle-B plan §2 D12, §4 |
| Last dev-checks run | `ALL CHECKS PASSED` (177/0) | Task-3 verification context |
| Exe baseline (pre-wave, 0.5.2) | `dist/windows-x64/img-scroller.exe` = 19,605,828 bytes | context.md; gitignored `dist/` |
| DLL audit baseline | recorded 22-DLL constant (`scripts/build-windows.sh:15` `OBSERVED_DLL_IMPORT_LIST` + `docs/build-windows.md` Observed list) | both files |
| Audit-relevant note | the 22-DLL set ALREADY includes `shell32.dll` and `shlwapi.dll` — the new Wave-6 shell COM calls (`SHAssocEnumHandlers`, `SHCreateItemFromParsingName`, `SHCreateShellItemArrayFromShellItem`) are shell32 EXPORTS, so the likely verdict is SET-EQUAL; a delta is still possible and handled by Step 4's decision table | `docs/build-windows.md` line 60 |
| `windows` crate features | EXACTLY six, unchanged since Wave 5.2: `Win32_Foundation`, `Win32_Graphics_Imaging`, `Win32_System_Com`, `Win32_UI_Shell`, `Win32_UI_Shell_Common`, `Win32_UI_WindowsAndMessaging` | `Cargo.toml:18–25` |
| Dependency drift this wave | `git diff f0b2274..HEAD -- Cargo.toml Cargo.lock` is EMPTY (the only wave change to both files was the Step-3 version bump inside `f0b2274`) | verified at plan time |

---

## Execution steps (tiny, ordered, verifiable — execute in this exact order)

### Step 0 — Pre-flight (STOP conditions)

1. Call `alpine-vm_vm_status`. If the VM is not running / SSH not reachable → STOP and return a user question (never retry past the single re-check; `docs/build.md`).
2. On the HOST (no cargo involved): run `git status` → expect `On branch feat/wave-6-open-with` and `nothing to commit, working tree clean`. If anything is staged/unstaged → STOP and return the exact state to the caller (never commit unrelated changes; branch actions are Step-2-restricted).
3. Run `git log --oneline -5` → expect the head of the chain to read `49b3b01`, `caa1aa0`, `c5cb56b`, `61cc1c8`, `fa0a6ac` (top-to-bottom). If HEAD differs → STOP and report.
4. Read `.gitignore` (expect `dist/`, `logs/`, `*.log`, `target/` among its entries). No `.gitignore`-matching file may EVER be staged in this whole task.

### Step 1 — Fresh dev-checks harness run (TODO sub-item 1's direct evidence)

Via the Alpine VM (`vm_status` already done in Step 0):

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
```

Expected:
- Final line `ALL CHECKS PASSED`; script exit code **0** (authoritative).
- Per-check lines all `PASS` for `fmt-check`, `check`, `test`, `clippy`.
- The `test` check reports **177 passed; 0 failed**.
- The run's log lands under the gitignored `logs/checks/` tree — nothing to commit.

Census corroboration (host-side, AFTER the harness run — the harness result stays authoritative): with the built-in Grep tool, search `src` (include `*.rs`) for the pattern `#\[test\]` and count the matches and the files containing ≥1 match. Expected: **177 matches across 28 files** (the two new Wave-6 files are `src/open_with_listing/tests.rs` with 6 and `src/app/open_with_button_tests.rs` with 7; the pre-wave 164 tests are unchanged). If the harness test line and the grep census disagree → STOP and return both numbers.

### Step 2 — Windows-target compile evidence (the Windows path compiles)

Via the Alpine VM:

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "cargo check --locked --target x86_64-pc-windows-gnu"
```

Expected: exit code **0**. This is the FIRST full target-scoped compile evidence of `src/platform/windows_open_with.rs` (dev-checks compile the stub path only). Failure classes:
- An API-projection/import-path mismatch against the real windows 0.62.2 crate → ONE minimal in-scope fix attempt is allowed: correct ONLY the import path/name to what the compiler reports (signatures/structure/allow placements are NOT latitude), then re-run this step once. If it still fails → STOP and return the exact compiler error.
- Anything else → STOP and return the exact failure (no retries).

### Step 3 — Release rebuild (0.6.0 exe; atomic overwrite)

Via the Alpine VM:

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh
```

Expected:
- Exit code **0**; the success note prints the OBSERVED DLL import list (the script's `OBSERVED_DLL_IMPORT_LIST` constant — recorded evidence only; the REAL audit is Step 4).
- The artifact `dist/windows-x64/img-scroller.exe` (gitignored) is re-written ATOMICALLY (`.part` → rename): non-empty, 0.6.0 build. It OVERWRITES the 0.5.2 artifact (19,605,828 bytes); a failed run would leave the previous artifact byte-for-byte intact (`docs/build-windows.md` § Overwrite/rebuild behavior).
- RECORD the exact byte size of the new exe (host-side directory listing of `dist\windows-x64\`). Any non-zero size is acceptable — 0.6.0 adds code, so a size DIFFERENT from 19,605,828 is expected and must simply be recorded verbatim; report the number to the caller.
- Typed failure lines (`build-windows: error: ...`) → STOP and return the exact line (`docs/build-windows.md` failure table; a lockfile/manifest mismatch class must not be self-fixed here).

### Step 4 — DLL import audit vs the recorded 22-DLL constant

Via the Alpine VM (audit tool per `docs/build-windows.md`):

```
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep 'DLL Name' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u"
```

This prints the fresh exe's deduplicated, lowercase DLL import SET. Compare it against the recorded 22-name constant (verbatim, from `scripts/build-windows.sh:15` / `docs/build-windows.md`):

```
advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll
```

Decision table (binding):
- **Set-equal (22 == 22, same names)** → record "DLL import audit SET-EQUAL to the recorded 22-DLL constant". This is the EXPECTED outcome: `shell32.dll` was already in the set; the new Wave-6 entry points are shell32 exports and the COM interfaces resolve through `combase`/`ole32` (already present). Do NOT edit any docs file (the OBSERVED list update belongs to Task 5 ONLY if a delta was recorded).
- **Delta (names added and/or removed)** → record the EXACT delta list (added names; removed names) and mark it "explained delta pending Task-5 docs update of the OBSERVED list" — an explained delta is a PASSING audit verdict per the global plan Cycle C. Explanation template: added = new system DLLs pulled in by the Wave-6 shell COM surface (`SHAssocEnumHandlers`/`IAssocHandler`/`SHCreateItemFromParsingName`/`SHCreateShellItemArrayFromShellItem`); removed = DLLs no longer imported. Do NOT edit `docs/build-windows.md` or `scripts/build-windows.sh` (Task 5 owns that).
- Any objdump execution error → STOP and return the exact failure.

### Step 5 — Wave-pin sweep (fast greps; host-side built-in Grep tool)

Each check is a search + expected result. A MISS on any pinned expectation → STOP and return the miss.

**S1 — Enumeration/launcher call sites (facade re-exports consumed by `open_with_button.rs`).** Grep `src` (include `*.rs`) for `enumerate_open_with_programs|open_file_with_selected_program`. Expected EXACTLY 9 matches at these sites (line numbers as of `49b3b01`):
- `src/app.rs:90` — `self.open_with_programs = platform::enumerate_open_with_programs(&path);` (inside `accept_open_outcome`'s success arm, BEFORE the `self.opened_image_path = Some(path);` move)
- `src/platform/mod.rs:32` and `:35` — the cfg-gated re-export pair (windows / stub)
- `src/platform/windows_open_with.rs:33` and `:40` — the Windows implementations
- `src/platform/stub.rs:26` and `:31` — the stub fallbacks
- `src/app/open_with_button.rs:108` — the popup-opening re-enumeration; `:178` — the launch hand-off

**S2 — Two-point enumeration contract.** Confirm by reading the two sites: (a) `src/app.rs` `accept_open_outcome` success arm contains the S1 line and the FAILURE arm does NOT touch `open_with_programs`; (b) `src/app/open_with_button.rs` contains `fn enumerate_when_menu_opens` (line 97), called once (line 70) before the popup attachment, and the popup contents closure calls ONLY `show_open_with_menu_entries` — no facade call inside the closure (no per-frame OS enumeration anywhere).

**S3 — Zero internal program database.** Grep `src` (include `*.rs`, EXCLUDING files matching `*tests.rs` and the `#[cfg(test)]` content of production files) for the pattern `Paint|Photos|ProgId|OpenWithList|OpenWithProgids|ShellExecute`. Expected: ZERO matches in production code. (The strings `Paint`/`Photos` exist ONLY in test fixtures: `src/app/open_with_button_tests.rs`, `src/open_with_listing/tests.rs`.) The app's program data comes exclusively from the runtime enumeration (`RegisteredProgram { display_name, launch_index }` from `programs_from_handler_names`).

**S4 — Zero Cargo.toml dependency change this wave.** On the HOST run `git diff f0b2274..HEAD -- Cargo.toml Cargo.lock`. Expected: EMPTY output (both files were last touched by the Step-3 version bump commit itself). Additionally read `Cargo.toml:18–25` and confirm the six features are byte-identical to the baseline list above.

**S5 — Census lines.** Restate from Step 1: harness test line **177 passed; 0 failed**; grep census **177 `#[test]` across 28 files**.

### Step 6 — Git segmentation check

1. `git status` → expect clean (`nothing to commit, working tree clean`), on `feat/wave-6-open-with`, nothing staged, nothing unstaged.
2. `git log --oneline -16` → expect the wave chain to be EXACTLY (top-to-bottom): `49b3b01` (verification report + wording), `c5cb56b` (Task-3 mark), `caa1aa0` (structure map), `61cc1c8` (docs), `fa0a6ac` (simplification), `c00d427` (menu feature), `3f43439` (Task-3 plan+spec), `84613cc` (Tasks 1+2 marks), `7d0a4a3` (module docs), `ba66ce2` (test dedup), `bfa20e1` (enumeration+launcher), `d9dffe9` (Cycle-A plan), `f0b2274` (version 0.6.0), `42d6237` (global plan), `59068cf` (gate closure), `fea72c2` (Wave-5.2 closing docs). No unexpected commits.
3. Confirm no gitignored path (`dist/`, `logs/`, `target/`, `*.log`) appears in any tracked/staged state (`git status --short` empty is the sufficient evidence).

### Step 7 — Assemble the OPERATOR-RUN checklist (§11 of this plan)

Assemble = copy §11 of this plan into the step's completion summary verbatim and hand it to the caller. It is: TODO Task-4 sub-item 2 VERBATIM + the 10 items from the Task-3 4.5a frontend verification report + the standing regressions. NO GUI work is attempted agent-side (impossible from the Linux container). The checklist text is FINAL in §11 — no rewording by the executor.

### Step 8 — Record results + commit protocol

1. Results are recorded in the step's returned completion summary ONLY (what passed, exit codes, the exact exe byte size, the DLL audit verdict — set-equal or the delta list). NO results report file is created (previous-wave precedent: results ride in the summary; narrative docs land in Task 5).
2. Commit protocol (Gitignore Compliance Rule before EVERY commit):
   - Commit 1: stage ONLY this plan file (`.kilo/plans/20261112-wave6-task4-verification-plan.md`) → commit `docs: add wave6 task4 verification plan`.
   - NOTHING else is committed by the 4.2 verification execution: no source changes (Step 2's allowed import-path fix, if used, is committed separately as `fix: correct windows open-with import path` — expected to be UNNECESSARY), logs stay under gitignored `logs/`, the exe stays under gitignored `dist/`.
   - Read `.gitignore` and run `git status` before staging; verify no `dist/`/`logs/`/`*.log`/`target/` file is staged; never `git add -A` blindly; never force-add tracked files matching ignore patterns.
   - NO branch creation, NO version edit, NO `git push` (Step-5 restricted), NO merge, NO `CHANGELOG.md`, NO edits to existing `.kilo/plans/*` files.
3. Final `git status` → clean.

---

## Expected outcomes summary (pinned per step)

| Step | Authoritative evidence | Expected |
| --- | --- | --- |
| 1 | dev-checks exit code + final line + test line | exit 0; `ALL CHECKS PASSED`; **177 passed; 0 failed**; census 177/28 |
| 2 | `cargo check --locked --target x86_64-pc-windows-gnu` exit code | 0 |
| 3 | build script exit code + artifact size | 0; non-empty `dist/windows-x64/img-scroller.exe` (0.6.0); exact byte size recorded (≠ 19,605,828 expected) |
| 4 | objdump DLL-name set vs the 22-name constant | SET-EQUAL 22 (expected); a delta = explained-delta record, still a PASS |
| 5 | the five sweep greps | all pinned expectations hold |
| 6 | git status / log | clean; chain exactly as listed |
| 7 | §11 handed to caller verbatim | done; TODO sub-item 2 stays `[ ]` |
| 8 | commit + final git status | 1 docs commit; tree clean |

## Failure handling (binding)

- A failing check (any step's expected outcome missed) → STOP and return the EXACT failure (command, exit code, output lines) to the caller. NO silent retries.
- The ONLY allowed minimal in-scope fix attempt: Step 2's import-path correction (one attempt, then STOP on repeat failure).
- VM / `/img-scroller` share unavailable → STOP and return a user question.
- Anything discovered that seems to need code/architecture changes → STOP; that is outside Task 4's scope (report to caller).

---

## §11 OPERATOR-RUN checklist (hand to caller VERBATIM — Task 4 sub-item 2 + 4.5a items + standing regressions)

**Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): for a jpg/png file, the menu lists at least the system-associated programs; selecting one opens this exact file in that program; NOT dependent on any app-maintained list.**

Run the freshly rebuilt 0.6.0 exe at `dist\windows-x64\img-scroller.exe`.

### Wave-6 "Open with..." menu items (from the Task-3 4.5a frontend verification report, verbatim)

1. **Basic menu content** — Open a `.jpg` or `.png`; click the new (rightmost) pill button; confirm the popup lists at least the system-registered programs (e.g., Paint, Photos) and that names match the system "Open with" dialog.
2. **Launch hand-off** — Select a listed program; confirm the *same* file opens in that program and not a different file.
3. **No internal database** — Temporarily change the default program for the file type or register/unregister a handler; confirm the menu reflects the live system state on the next popup-open click.
4. **Empty-enumeration tooltip** — Open a file type with no registered handlers (or use a custom extension); hover the disabled button and confirm `"No programs registered for this file type"`.
5. **Disabled in empty-folder mode** — Navigate into an empty sibling folder; confirm the open-with button is disabled and its tooltip is `"Open with..."` (not the no-programs tooltip).
6. **Fullscreen hiding** — Enter fullscreen; confirm the whole pill (including the new button) disappears.
7. **Popup interaction** — With a populated menu, verify outside-click and `Esc` dismiss the popup; verify arrow-key navigation and Enter selection inside the popup.
8. **Long-list scrolling** — If more than ~12 programs are registered, confirm the popup caps height and scrolls vertically.
9. **Failure message line** — If a launch can be made to fail (e.g., handler executable removed), confirm the pill shows `"Windows could not open this file with the selected program."`.
10. **Visual/DPI sanity** — Check that the glyph aligns with adjacent pill glyphs, the popup opens above the button, and scaling on high-DPI displays is sane.

### Standing regressions (every previously-established behavior stays expected-good on top of Wave 6)

11. **Pan/edge-navigate** — zoom wider than the window → left/right keys PAN; horizontal navigation fires only at the image's left/right edge; zoom taller → same vertically for folders; small/fits-entirely image → arrows navigate directly.
12. **←/→/↑/↓ lists** — horizontal moves across sibling images in the folder; vertical moves across sibling folders under the SAME parent (no grandparent jumps, no recursive deeper); navigation follows the system (natural, Explorer-like) name order — e.g. `智乃 (1).jpg … (9).jpg, (10).jpg …` — NOT lexical (`(1), (10), (11), …`).
13. **Silent edges** — no wrap-around and no error message at either list edge (silent stop).
14. **Buttons** — all pill buttons work as labeled: `[←][→][↑][↓]` navigation; `[+][−][restore-fit]` zoom/restore; `[W]`/`[H]` one-shot fit-width/fit-height; fullscreen square.
15. **I/O/0/W/H keys** — `I` zoom in, `O` zoom out, `0` restore fit (silent actions, no messages), symmetric zoom bounds (÷8 floor, ×8 ceiling).
16. **ESC contract incl. popup expiry** — fullscreen ON: ESC exits fullscreen; fullscreen OFF: first ESC shows the centered "Press ESC again to close" popup for ~2 s (it auto-dismisses after expiry), a second DISTINCT press closes the app, two presses never held/autorepeated.
17. **Shift+'+' / Shift+'−' aliases** — Shift+'+' zooms in, Shift+'−' zooms out; plain (unshifted) `+`/`=`/`-` keys are inert.
18. **Captions/tooltips** — hotkey captions under the six keyboard-distinct pill buttons (`F11`/`I`/`O`/`0`/`W`/`H`); hover tooltips on all 11 buttons, white captions + thin black outlines on glyphs and captions.
19. **Always-maximized launch + hidden console** — the window opens ALWAYS MAXIMIZED at every launch and no console window is visible while the release build runs.
20. **Empty-folder mode** — an image-less sibling folder browses in empty-folder mode (centered `Empty folder`, pill visible, message line empty, ↑/↓ continue from it); the open-with button is disabled there (item 5).
21. **Last-viewed memory** — revisiting a folder this session reopens its last-viewed image; a first visit opens the folder's FIRST image in system order; memory is session-only (closing and relaunching loses it).

---

## Cross-check against TODO Task 4 verbatim

- Sub-item 1 (unit tests via the dev-checks harness, `ALL CHECKS PASSED`: fmt/check/test/clippy; `alpine-vm` MCP; `vm_status` first; exit code authoritative) → **Step 0 + Step 1** (fresh harness run at the current branch HEAD, 177/0, census corroboration via grep). The unit tests themselves landed in Cycles A (`open_with_listing/tests.rs`, 6 tests) and B (`open_with_button_tests.rs`, 7 tests) — this task verifies they run green. ✓
- Sub-item 2 (manual UX OPERATOR-RUN on real Windows) → **Step 7 + §11**: the checklist is assembled verbatim and handed to the caller; the TODO sub-item stays `[ ]` (the user's gate; the 4.6 step marks ONLY sub-item 1). ✓
- Global-plan Cycle-C 4.1b checklist → fresh harness run (Step 1), test-census corroboration (Step 1), wave-pin sweeps incl. the new seam call sites / menu render path / feature list in `Cargo.toml` (Step 5), release rebuild + byte-size record (Step 3), DLL import audit vs the 22-DLL constant with the set-equal-OR-explained-delta verdict (Step 4), Git segmentation (Step 6), OPERATOR-RUN checklist with standing regressions (§11). ✓

**Plan file:** `.kilo/plans/20261112-wave6-task4-verification-plan.md`
