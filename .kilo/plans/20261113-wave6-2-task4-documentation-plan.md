# Wave 6.2 Task 4 — Documentation Plan (Critical Workflow step 4.1b, architector)

- Source TODO: `.agent/todos/20261113/20261113-todo-2.md` → `### Task 4: Documentation updates` (1 sub-item): "`README.md`, `tech.md`, `architecture.md`, `context.md`: record the diagnostic evidence, the root cause, the fix, the 0.6.2 state, and remaining open items."
- Global plan: `.kilo/plans/20261113-wave6-2-open-with-launch.md` → Cycle D (Task 4).
- Branch: `fix/wave-6-2-open-with-launch` (VERIFIED checked out, working tree clean). HEAD at plan time: `f46d5dd` (`task: mark wave6.2 task3 done`). Version 0.6.2 (`Cargo.toml:3`).
- Executor: the docs-specialist (4.2 of Cycle D) — a JUNIOR developer under 50% restriction. Every edit below is FINAL and text-pinned: do not redesign, reword beyond the pinned text, reorder, or extend. Latitude: real newlines (never literal `\n`), markdown list numbering already used by each file, and correcting an accidental keystroke in the pinned text — nothing else.
- Scope of THIS cycle: documentation only. NO source-code change, NO test change, NO Cargo change, NO TODO edit (4.6 does that), NO `CHANGELOG.md`, NO edit to any existing `.kilo/plans/*` file (the ONE new plan file this cycle creates is §5 Step 0's commit of THIS file), NO push (Step-5-restricted, `main` → `origin` ONLY).
- Binding constraints honored: the wave's Gitignore Compliance Rule before every commit; `docs/` and `brief.md`/`project-structure.md` verify-only verdicts in §3; the markdown-generation rule (this plan file is created by the architector; the doc files are edited by the docs-specialist).

---

## §0 LANDED FACTS THIS PLAN RECORDS (all verified at HEAD `f46d5dd`, this session)

| # | Fact | Verified by |
| --- | --- | --- |
| 1 | Branch `fix/wave-6-2-open-with-launch`; HEAD `f46d5dd`; working tree clean; Task 4 is the ONLY unchecked TODO sub-item | `git status` + `git log --oneline -1` + TODO read |
| 2 | Wave-6.2 commit chain (verbatim, chronological, 15 commits): `fd7acce` (report+TODO), `c3611ae` (global plan), `a048567` (0.6.2 bump), `02f639a` (T1 plan), `4185d97` (diagnostic feature `feat: add optional diagnostic launch log behind diagnostic-launch-log feature`), `e8c3f0b` (T1 docs `docs: wave6.2 diagnostic module docs`), `e1cdfc3` (T1 mark), `b2b47a0` (T2 plan), `2b46a2e` (the fix `fix: feed the open-with shell item a shell-parseable file path`), `1aac7e1` (T2 docs `docs: wave6.2 launcher-fix module docs`), `0378d6b` (T2 mark), `9c8db64` (T3 plan), `4d87e75` (T3 results), `685e113` (T3 adjudication), `f46d5dd` (T3 mark) | `git log --oneline b458897..HEAD` |
| 3 | THE DIAGNOSTIC: the caller-authorized OPTIONAL build-time module — Cargo feature `diagnostic-launch-log` (`Cargo.toml:6-8`, `default = []`), module `src/platform/launch_log.rs` (170 lines) declared under `#[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]` (`src/platform/mod.rs:15`); when enabled (with the windows target) it lazily appends `img-scroller-diagnostic.log` NEXT TO the executable (temp-dir fallback) recording the stored/fresh enumerations, the resolved `launch_index`, and the `SHCreateItemFromParsingName` / `BindToHandler` / `Invoke` outcomes with result codes; write failures silently skipped, never alters behavior. The release NEVER enables it: `scripts/build-windows.sh` contains NO `--features`; the zero-diagnostic proof legs (Pin A script / Pin B feature default / Pin C binary grep count `0`) ALL PASSED in Task 3 | `Cargo.toml` + `src/platform/launch_log.rs` + T3 plan §2 Step 3 + T3 results |
| 4 | THE OPERATOR'S DIAGNOSTIC EVIDENCE: `img-scroller-diagnostic.log` from the real run — enumeration/pairing PERFECT (12 handlers: Photos, Snipping Tool, Paint, img-scroller, Firefox, Notepad, VS Code, DB Browser, MPC-HC, Notepad++, VS Version Selector, WMP Legacy; the stored list and the fresh re-enumeration agree; `launch_index` resolves every attempt; suspects B/C/D eliminated); the failure line `shcreateitemfromparsingname: created=false code=0x80070057` (E_INVALIDARG) on EVERY attempt, BEFORE `BindToHandler`/`Invoke` are ever reached; the attempted file logged as `\\?\C:\_kazuya\imgs\Chino_Kafuu__BOOTH_\Chino Kafuu 1.png` | `.agent/project-info/context.md` (the landed T1 entry) + T2 plan §0 |
| 5 | THE ROOT CAUSE: the app's stored `opened_image_path` is the `fs::canonicalize` output (`src/app.rs:71-75`, the D5 single-normalization point) which on Windows carries the `\\?\` VERBATIM (extended-length) prefix; `SHCreateItemFromParsingName` (a shell parsing-name API — its input is a display/parsing name routed through `SHParseDisplayName`) rejects `\\?\`-prefixed paths with E_INVALIDARG. The Wave-6.1 `IShellItem::BindToHandler(BHID_DataObject)` fix was CORRECT but upstream-blocked. CORRECTED WIC FACT: the decode is invoked with the PRE-canonicalization path (`src/app.rs:72` calls `image_loading::load_image(&path)` BEFORE `fs::canonicalize` on `:73`) — opening NEVER depended on the canonical form | T2 plan §0.1–§0.3 |
| 6 | THE FIX (commit `2b46a2e`): the pure `shell_parseable_path(canonical_path: &Path) -> PathBuf` in `src/open_with_listing.rs:93` (constants `VERBATIM_PATH_PREFIX` = `\\?\` and `VERBATIM_UNC_PATH_PREFIX` = `\\?\UNC\` at `:86-87`): `\\?\UNC\…` → `\\…` FIRST, then `\\?\…` → strip, everything else passes through unchanged; the ONE production call site is `src/platform/windows_open_with.rs:146` — `let shell_path = shell_parseable_path(file_path);` as the FIRST statement of `file_data_object`, BEFORE the wide encode and `SHCreateItemFromParsingName`; the enumeration (`raw_handlers` → `extension_query_string`) keeps receiving the canonical path (it only reads `.extension()` and the log proved it works); the stored canonical `opened_image_path` is NEVER rewritten (all other consumers unchanged); NO semantics change; 6 new container tests joined the EXISTING `src/open_with_listing/tests.rs` (6 → 12); census 189 → 195 unit tests / 28 test-bearing files | T2 plan §1–§3 + `git grep` |
| 7 | THE 0.6.2 STATE (Task 3, VERIFIED — the two read-only sweep pin mismatches S3/S5 adjudicated as explained deltas, verdict VERIFIED at `685e113`): dev-checks `ALL CHECKS PASSED` — 195 unit tests, 0 failed, 28 test-bearing files (the verified breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`) = 28); the windows-target compile (`cargo check --locked --target x86_64-pc-windows-gnu`) exit 0 carrying ONLY the pre-existing benign dead-code warning on `compare_entry_names`; the release rebuild → `dist/windows-x64/img-scroller.exe` = 19,649,107 bytes (the 0.6.1 artifact 19,645,219 B overwritten atomically) with ZERO diagnostic code; the DLL import audit SET-EQUAL to the recorded 22-DLL constant (`shell_parseable_path` is std-only string operations — no new API imports); `Cargo.toml` diff since `a048567` = ONLY the 4-line `[features]` block, `Cargo.lock` diff EMPTY | T3 plan §0 + T3 results (VERDICT: VERIFIED) |
| 8 | THE PENDING GATE: the Task-3 manual-UX sub-item STAYS `[ ]` — the 0.6.2 OPERATOR-RUN is the user's gate; the PLAIN checklist lives VERBATIM at `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` §3 (reproduced in §2.4 E4 of this plan); it supersedes the pending 0.6.1 checklist in one run; the closing note keeps the diagnostic exe (`img-scroller-diagnostic.exe`, 225,180,316 B) + log (`img-scroller-diagnostic.log`, 2,768 B) available in `dist/windows-x64/` for a follow-up evidence run (they are removed ONLY at the global plan's Step-5 tmp sweep) | T3 plan §3 + T3 results standing notes |
| 9 | Docs-target current state: README.md ends its feature-7 bullet at the Wave-6.1 sentence, its Technology Stack intro at the Wave-6.1 clause, its Language row at "0.6.1", and its Project Status paragraph at the 0.6.1 pending-gate sentence; tech.md's `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` section ends at the Wave-6.1 census/verification bullet, its Testing section leads with "189 unit tests" and pins `src/open_with_listing/tests.rs — 6`, its Dependencies section has NO `diagnostic-launch-log` mention, its Status paragraph ends at the Wave-6.1 sentence, its Technology Stack row says 0.6.1, and its Technical Constraints open-with bullet has NO conversion/diagnostic mention; architecture.md's Status paragraph ends at the Wave-6.1 sentence, its Fast Open-with Service bullet ends at the Wave-6.1 modal sentence, its `src/open_with_listing.rs` module bullet says "6 tests" with NO `shell_parseable_path`, its `src/platform/` module bullet has NO `launch_log.rs` and its `windows_open_with.rs` clause has NO conversion, its Critical-Path 5 says "the current canonical file's `IDataObject`", and its decisions-table Open-with row ends at the Wave-6.1 corrections; context.md's Current-Work-Focus still opens with "Wave 6.2 … is PENDING user authorization + global-plan approval" (stale), its Recent Changes carries ONLY the T1 diagnostic entry (the T2/T3 facts and the wave chain are NOT present), its Current Project State has NO Wave-6.2 bullet and its Wave-6.1 bullet still claims the 0.6.1 gate as Next-Steps #1, its release-artifact bullet still says 19,645,219 B (0.6.1), its TODO-state bullet has NO Wave-6.2 clause, its Git bullet is stale (`main == origin/main == 5f4f8ec`), and its Immediate Next Steps still carry the 0.6.1 checklist | full file reads + targeted `findstr` extracts |
| 10 | VERIFY-ONLY FINDINGS: (a) `brief.md` `### Fast "Open with..." Menu` requires "Selecting a program opens the current file with it" — the fix REALIZES the existing requirement; NO addendum is needed (NO edit). (b) `docs/build-windows.md` — the audit stayed SET-EQUAL and the doc carries NO diagnostic references (the only "diagnostic" match is the unrelated cargo-build-failure wording); NO edit. (c) **`agent/project-structure.md` is STALE**: the `platform/` bullet (line 25) lists `mod.rs`/`windows.rs`/`windows_collation.rs`/`windows_open_with.rs`/`stub.rs` but NOT `launch_log.rs` — a NEW src file landed THIS wave at `4185d97` (`git log b458897..HEAD -- .agent/project-structure.md` is EMPTY; the file's `git diff b458897..HEAD` shows no change). The caller's premise "no new src files this wave" is factually contradicted. Per the caller's binding "NO edit expected", this cycle does NOT edit the map; the implementer records the finding and flags it to the caller (§3 V1 protocol) | file reads + git log/diff |

---

## §1 FAILURE-HANDLING POLICY (applies to EVERY step below)

1. If a pinned edit's anchor text is NOT found exactly in the target file: STOP that edit, capture the surrounding lines, and return the mismatch to the caller. Do NOT improvise a different anchor.
2. If the live re-verification of a §0 fact contradicts the fact as pinned here: STOP and return the exact contradiction to the caller (the plan's facts were verified at `f46d5dd`; anything else is drift).
3. NO fix attempts exist in this cycle (documentation only); the single allowed retry is re-reading the file after a transient tool error.
4. All commits follow §5; before EVERY commit: read `.gitignore`, run `git status`, ensure NO gitignore-matching file is staged (`dist/`, `logs/`, `*.log`, `target/`); never force-add ignored files.

---

## §2 THE FOUR DOC-FILE EDIT PRESCRIPTIONS (docs-specialist, 4.2 — in order)

Apply the edits in this order: README.md → tech.md → architecture.md → context.md. All texts below are FINAL. When a prescription says "append after sentence X", the appended text starts directly after X's closing punctuation on the SAME line (these files write long single-line paragraphs) — EXCEPT the tech.md and architecture.md bullets and the context.md list entries, which are separate list items on their own lines. Preserve every byte outside the prescribed edits.

### §2.1 README.md — 4 edits

**E1 — Feature-7 bullet (line 26), append after the current final sentence** ("...requested `IID_IDataObject` off-contract and failed on real Windows before the launch hand-off (the D1 root cause, fixed in 0.6.1)."):

```markdown
Since Wave 6.2 the launch works on real Windows: the path handed to the shell item builder is first converted to its shell-parseable form, because the app's stored canonical path is the `fs::canonicalize` output carrying Windows' `\\?\` extended-length (verbatim) prefix, which the shell parsing-name API `SHCreateItemFromParsingName` rejects with E_INVALIDARG (0x80070057) — root-caused 100% on every selection with a caller-authorized diagnostic build and fixed by converting ONLY at the launch hand-off (the stored path and every other consumer are untouched; the image OPENING never depended on the canonical form — the decode runs with the pre-canonicalization path). The diagnostic remains an OPTIONAL build-time module (`diagnostic-launch-log`, OFF by default — the release build never enables it) that writes `img-scroller-diagnostic.log` next to the executable only when explicitly enabled; the released 0.6.2 build runs with zero diagnostic activity.
```

**E2 — Technology Stack intro (line 49), replace the tail**: the current text ends with `` ...and the Wave-6.1 operator-feedback corrections (the launcher root-cause fix, the `Q` hotkey, the modal menu, and the warning-colored failure message) — all seven v1 features are landed; ``. Replace the segment `` failure message) — all seven v1 features are landed `` with:

```markdown
failure message), and the Wave-6.2 shell-parseable launch-path fix (the diagnostic-decided launcher correction) — all seven v1 features are landed
```

**E3 — Technology Stack table (line 53), the Language row**: replace `` Implemented: Cargo package `img-scroller` 0.6.1, edition 2024. `` with `` Implemented: Cargo package `img-scroller` 0.6.2, edition 2024. `` (the ONLY version change in README; historical status mentions of 0.6.1 stay).

**E4 — Project Status (line 69), append after the current final sentence** ("...the next step is the DIAGNOSTIC debug-build fallback requiring explicit caller authorization."):

```markdown
Wave 6.2 landed (branch `fix/wave-6-2-open-with-launch`, version 0.6.2): the caller-authorized diagnostic build — an OPTIONAL, OFF-by-default `diagnostic-launch-log` Cargo feature writing `img-scroller-diagnostic.log` next to the executable — provided the runtime evidence: the enumeration and the stored-list pairing were PERFECT (12 handlers), but `SHCreateItemFromParsingName` failed with E_INVALIDARG (0x80070057) on EVERY selection attempt — the app's stored canonical image path carries the `\\?\` extended-length prefix that `fs::canonicalize` produces on Windows, which the shell parsing-name API rejects (the Wave-6.1 `BindToHandler` fix was correct but upstream-blocked). The fix (commit `2b46a2e`) derives the shell-parseable hand-off form — `\\?\UNC\` → `\\`, `\\?\` → stripped, everything else unchanged — with the pure `shell_parseable_path` helper at the ONE launch hand-off point; the stored canonical path is untouched and nothing else changes. Verification: dev checks `ALL CHECKS PASSED` (195 unit tests, 0 failed, across 28 test-bearing files — the verified breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules), the release executable rebuilt at `dist/windows-x64/img-scroller.exe` (19,649,107 bytes at 0.6.2) with ZERO diagnostic code (the feature stays OFF by default; the three zero-diagnostic proof legs passed), and the DLL import audit SET-EQUAL (the recorded 22-DLL set). Pending operator acceptance: the SINGLE 0.6.2 OPERATOR-RUN — re-test that selecting programs (Paint, Photos) in the "Open with..." list opens the file, plus the standing regressions — it supersedes the pending 0.6.1 checklist in one run (the plain checklist at `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` §3); if items 1–2 still fail, the diagnostic build remains available for a follow-up evidence run (already authorized).
```

NO other README edit: the controls table, the pill paragraph (line 39), and the Additional scope notes carry NO Wave-6.2-relevant change (no behavior/control change landed).

### §2.2 tech.md — 6 edits

**E1 — Status (line 5), append after the current final sentence** ("...(mechanism: `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` below, extended with the Wave-6.1 bullets)."):

```markdown
 Landed for Wave 6.2 (branch `fix/wave-6-2-open-with-launch`, version 0.6.2, 2026-11-13): the still-100%-failing launcher was root-caused with the caller-authorized optional diagnostic build and fixed with the shell-parseable launch-path conversion (mechanism: `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` below, extended with the Wave-6.2 bullets).
```

**E2 — Technology Stack table (line 33), the Language row**: replace `` Cargo package `img-scroller` version 0.6.1 (single binary; tracked `Cargo.lock`). `` with `` Cargo package `img-scroller` version 0.6.2 (single binary; tracked `Cargo.lock`). `` (the ONLY version change in tech.md).

**E3 — Dependencies (Landed, Wave 1) section**: insert a NEW bullet AFTER the `windows =0.62.2` bullet (line 51) and BEFORE the `No other dependencies` bullet (line 52):

```markdown
- Cargo feature (Wave 6.2): `diagnostic-launch-log` — an empty feature with `default = []` gating the OPTIONAL diagnostic launch-log module `src/platform/launch_log.rs`; it adds NO dependency and NO library feature and is NEVER enabled by release builds (the shipped artifact runs with zero diagnostic activity); recorded here because it is the only manifest feature beyond the pinned dependency feature lists.
```

(The `No other dependencies` bullet stays, unchanged.)

**E4 — `` ## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6) `` section, launcher bullet (line 139), append after the current final sentence** (`` ...the module doc at `src/platform/windows_open_with.rs:9–15` records the root cause. ``):

```markdown
 Wave 6.2 FIXED the REMAINING upstream blocker (commit `2b46a2e`): the operator's diagnostic log proved `SHCreateItemFromParsingName` rejected the app's stored canonical path on EVERY attempt (`shcreateitemfromparsingname: created=false code=0x80070057` — E_INVALIDARG) BEFORE `BindToHandler`/`Invoke` were ever reached — `fs::canonicalize` returns Windows extended-length paths carrying the `\\?\` VERBATIM prefix, which is NOT a shell parsing name (the Wave-6.1 `BindToHandler` route was correct but upstream-blocked). The fix derives the shell-parseable hand-off form with the pure `shell_parseable_path` (`src/open_with_listing.rs:93`; constants `VERBATIM_PATH_PREFIX`/`VERBATIM_UNC_PATH_PREFIX` at `:86-87`: `\\?\UNC\…` → `\\…` FIRST, then `\\?\…` → strip, everything else passes through unchanged) and converts at the ONE call site `src/platform/windows_open_with.rs:146` — the FIRST statement of `file_data_object`, BEFORE the wide encode and `SHCreateItemFromParsingName`; the enumeration keeps receiving the canonical path (it only reads `.extension()` and the log proved it works); the stored canonical `opened_image_path` is NEVER rewritten (the D5 single-normalization point and every other consumer unchanged); NO semantics change. CORRECTED WIC FACT (recorded): the decode is invoked with the PRE-canonicalization path (`src/app.rs:72` before `fs::canonicalize` at `:73`) — image OPENING never depended on the canonical form. Test delta: 6 new container tests joined the EXISTING `src/open_with_listing/tests.rs` (6 → 12; the verbatim-prefix strip with the operator's real path, the UNC device → network-share mapping, the local/plain-UNC/relative pass-through cases, and the case/non-ASCII preservation).
```

**E5 — same section, TWO new bullets appended at the section end (after the Wave-6.1 census/verification bullet, line 146), each on its own line:**

Bullet 1 (the diagnostic module):

```markdown
- Wave-6.2 OPTIONAL diagnostic launch log (Task 1, caller-authorized 2026-11-13): the Cargo feature `diagnostic-launch-log` (`Cargo.toml:6-8`, `default = []`) gates `src/platform/launch_log.rs` (declared under `#[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]`, `src/platform/mod.rs:15`; every hook in `windows_open_with.rs` is a cfg-gated statement). When enabled (with the windows target) it lazily appends `img-scroller-diagnostic.log` NEXT TO the executable (temp-dir fallback), with a one-time header line (version/profile/started-unix), recording per selection attempt: the stored and fresh enumerations (`pos`/`launch_index`/name), the resolved `launch_index`, the `SHCreateItemFromParsingName` outcome, the `BindToHandler(BHID_DataObject)` outcome, and the final `Invoke` classification with result codes; any write failure is silently skipped and the module NEVER alters app behavior or panics. The release build NEVER enables it (`scripts/build-windows.sh` contains NO `--features`); the zero-diagnostic proof legs ALL PASSED at Task 3 (Pin A script / Pin B feature default / Pin C binary grep of `img-scroller-diagnostic` → count `0`), so the shipped 0.6.2 artifact runs with ZERO diagnostic activity. The operator's diagnostic run produced the D1 root-cause evidence (§0 of this section's launcher bullet above).
```

Bullet 2 (the Wave-6.2 verification record):

```markdown
- Wave-6.2 census + verification (Task 3, VERIFIED): the census rose 189 → 195 unit tests across the SAME 28 test-bearing files (the 6 new `shell_parseable_path` tests joined the EXISTING `src/open_with_listing/tests.rs` — the file count stayed 28). The verified breakdown (verbatim): **28 test-bearing files = 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`)**. Verification: dev checks `ALL CHECKS PASSED` — 195 unit tests, 0 failed, across 28 test-bearing files (census: `## Testing`); the windows-target compile (`cargo check --locked --target x86_64-pc-windows-gnu`) exit 0 carrying ONLY the pre-existing benign dead-code warning on `compare_entry_names`; the 0.6.2 exe rebuilt at `dist/windows-x64/img-scroller.exe` = 19,649,107 bytes (the 0.6.1 artifact 19,645,219 B overwritten atomically) with ZERO diagnostic code; the DLL import audit SET-EQUAL to the recorded 22-DLL constant (`shell_parseable_path` is std-only string operations — `strip_prefix`/`to_string_lossy`/`PathBuf::from` — no new API imports; the deciding evidence, never an assumption); zero Cargo drift since the bump (`git diff a048567..HEAD -- Cargo.toml` = ONLY the 4-line `[features]` block; `Cargo.lock` EMPTY). The two read-only sweep pin mismatches of the Task-3 run were adjudicated as explained deltas (the `launch_log.rs` doc-comment change and the `Cargo.lock` version line) — verdict VERIFIED (`.kilo/plans/20261113-wave6-2-task3-verification-results.md`). The pending 0.6.2 OPERATOR-RUN checklist lives verbatim (PLAIN form) in `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` §3.
```

**E6 — `` ## Testing `` (line 180), two pinned replacements inside the SAME line:**
1. Replace the leading `` - 189 unit tests, all container-runnable `` with `` - 195 unit tests, all container-runnable ``.
2. Replace the segment `` src/open_with_listing/tests.rs` — 6 (Wave-6 Tasks 1–2: the blank-name filter, system-order preservation, and the launch_index/dedup contract) `` with `` src/open_with_listing/tests.rs` — 12 (Wave-6 Tasks 1–2: the blank-name filter, system-order preservation, and the launch_index/dedup contract; Wave 6.2: +6 — the `shell_parseable_path` conversion tests: the verbatim-prefix strip, the UNC device → network-share mapping, the pass-through cases, and the case/non-ASCII preservation) ``.
No other per-file count changes (the wave added tests to an EXISTING file only).

**E7 — `## Technical Constraints` (line 191), append to the open-with bullet after** "...are recorded in `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` above.":

```markdown
 The launcher hand-off converts the stored canonical path to its shell-parseable form ONLY at that hand-off (`shell_parseable_path` — `\\?\UNC\` → `\\`, `\\?\` → stripped, pass-through otherwise); the stored canonical path itself is never rewritten. The OPTIONAL `diagnostic-launch-log` diagnostic module exists OFF-by-default (never enabled in release builds — zero diagnostic activity in shipped artifacts).
```

### §2.3 architecture.md — 6 edits

**E1 — Status (line 5), append after the current final sentence** (`` ...the exe was rebuilt at `dist/windows-x64/img-scroller.exe` (19,645,219 bytes at 0.6.1), and the DLL import audit stayed SET-EQUAL 22. ``):

```markdown
 Wave 6.2 (branch `fix/wave-6-2-open-with-launch`, version 0.6.2, 2026-11-13) fixed the STILL-100%-failing launcher with the EVIDENCE-DECIDED shell-parseable launch-path conversion (the diagnostic-log root cause: `SHCreateItemFromParsingName` rejects the `fs::canonicalize` `\\?\` verbatim prefix with E_INVALIDARG — the Fast Open-with bullet, the module bullets, and the decisions table below; the OPTIONAL OFF-by-default `diagnostic-launch-log` module provided the evidence); dev checks ended `ALL CHECKS PASSED` (195 unit tests), the exe was rebuilt at `dist/windows-x64/img-scroller.exe` (19,649,107 bytes at 0.6.2), and the DLL import audit stayed SET-EQUAL 22; the pending acceptance is the 0.6.2 OPERATOR-RUN.
```

**E2 — Fast "Open with..." Service bullet (line 39), append after the current final sentence** (`` ...via the pure predicate `message_line_is_warning` — the ONLY warning-colored message. ``):

```markdown
 Wave 6.2 (evidence-decided, commit `2b46a2e`): the operator's diagnostic log proved `SHCreateItemFromParsingName` rejected the stored canonical path on EVERY selection (E_INVALIDARG 0x80070057 — `fs::canonicalize`'s `\\?\` VERBATIM prefix is not a shell parsing name; the Wave-6.1 `BindToHandler` route was CORRECT but upstream-blocked), so `file_data_object` now converts the path to its shell-parseable form with the pure `shell_parseable_path` seam (`src/open_with_listing.rs` — `\\?\UNC\` → `\\` FIRST, `\\?\` → stripped, pass-through otherwise) at the ONE hand-off point BEFORE the shell item build; the stored canonical path and every other consumer are untouched, NO semantics change. The OPTIONAL `diagnostic-launch-log` Cargo feature (OFF by default; the release never enables it — the shipped artifact runs with zero diagnostic activity) provides the per-selection evidence log (`img-scroller-diagnostic.log` next to the exe) when explicitly enabled.
```

**E3 — `` src/open_with_listing.rs `` module bullet (line 52), two replacements inside the SAME line:**
1. After the segment `` ...and `OPEN_WITH_LAUNCH_FAILURE_MESSAGE` (the single static launch-failure message) live here `` insert EXACTLY (plain backticks, real backslashes — `\\?\UNC\` and `\\?\` and `\\` are the literal characters to write):

```markdown
; since Wave 6.2 the file also carries the pure `shell_parseable_path(canonical_path) -> PathBuf` conversion (the launcher's shell hand-off form: `\\?\UNC\` → `\\` first, `\\?\` → stripped, pass-through otherwise; constants `VERBATIM_PATH_PREFIX`/`VERBATIM_UNC_PATH_PREFIX`), consumed ONLY by `windows_open_with.rs::file_data_object` BEFORE `SHCreateItemFromParsingName`
```

2. Replace the segment `` ; 6 tests in `src/open_with_listing/tests.rs`. `` with:

```markdown
; 12 tests in `src/open_with_listing/tests.rs` (6 Wave-6 + 6 Wave-6.2 the conversion tests)
```

**E4 — `` src/platform/ `` module bullet (line 70), two pinned edits inside the SAME line:**
1. In the `windows_open_with.rs` clause, replace the segment `` and the launcher (`IAssocHandler::Invoke` over the `IDataObject` built from the current canonical file) `` with EXACTLY:

```markdown
and the launcher (`IAssocHandler::Invoke` over the `IDataObject` built from the current file's shell-parseable form — since Wave 6.2 `file_data_object` converts the stored canonical path via `open_with_listing::shell_parseable_path` BEFORE `SHCreateItemFromParsingName`, because the shell parsing-name API rejects the `\\?\` verbatim prefix with E_INVALIDARG)
```

2. Insert a NEW clause between the `windows_open_with.rs` clause and the `stub.rs` clause — EXACTLY (keep the existing single spaces around it):

```markdown
`launch_log.rs` (Wave 6.2, OPTIONAL) is the caller-authorized diagnostic launch-log module compiled only when BOTH the windows target AND the OFF-by-default `diagnostic-launch-log` feature are enabled — it appends per-selection evidence to `img-scroller-diagnostic.log` next to the executable and never alters app behavior;
```

**E5 — Navigation Critical Paths, path 5 (line 91)**: replace the segment `` selecting an entry re-enumerates fresh and invokes the selected handler (`IAssocHandler::Invoke` with the current canonical file's `IDataObject`) `` with EXACTLY:

```markdown
selecting an entry re-enumerates fresh and invokes the selected handler (`IAssocHandler::Invoke` with the current file's `IDataObject` built from its shell-parseable form — the Wave-6.2 conversion at the hand-off)
```

**E6 — Technical Decisions Summary, the Open-with mechanism row (line 142), two pinned edits:**
1. Append to the Decision cell after the segment `` ...and the warning-orange failure message (`WARNING_MESSAGE_COLOR`, predicate-only) `` EXACTLY:

```markdown
; Wave 6.2: the launcher converts the stored canonical path to its shell-parseable form (`shell_parseable_path` — the diagnostic-log D1 root cause: `SHCreateItemFromParsingName` rejects the `fs::canonicalize` `\\?\` verbatim prefix with E_INVALIDARG; commit `2b46a2e`; the stored path untouched; NO semantics change) and the OPTIONAL `diagnostic-launch-log` diagnostic module (OFF by default; the release never enables it) stays
```

2. Replace the Status cell `` landed (Wave 6; the Q/modal/warning + launcher-route corrections Wave 6.1) `` with EXACTLY:

```markdown
landed (Wave 6; the Q/modal/warning + launcher-route corrections Wave 6.1; the shell-parseable launch-path conversion + the optional diagnostic module Wave 6.2)
```

The Source cell stays `` `### Fast "Open with..." Menu` `` (unchanged).

### §2.4 context.md — 4 edits

**E1 — Current Work Focus (line 15), REPLACE the ENTIRE first paragraph** (the paragraph beginning `Dated 2026-11-13. **Wave 6.2 ...**` and ending before the `Dated 2026-11-12.` paragraph) with:

```markdown
Dated 2026-11-13. **Wave 6.2 (`.agent/todos/20261113/20261113-todo-2.md`) is COMPLETE on the agent side through Task 3 + Task 4 documentation, PENDING the 0.6.2 OPERATOR-RUN gate and global-plan Step 5 (the branch is NOT yet merged)** — branch `fix/wave-6-2-open-with-launch`, version 0.6.2. What landed: (Task 1) the caller-AUTHORIZED diagnostic (2026-11-13, with two binding modifications: a `.log` FILE the operator reports back, and keep-as-optional build-time gating) — the OPTIONAL OFF-by-default `diagnostic-launch-log` Cargo feature gating `src/platform/launch_log.rs` (appends `img-scroller-diagnostic.log` next to the exe; the release NEVER enables it; the zero-diagnostic proof legs passed); the operator's diagnostic run gave the D1 root-cause evidence: the enumeration + pairing are PERFECT (12 handlers), but `shcreateitemfromparsingname: created=false code=0x80070057` (E_INVALIDARG) on EVERY attempt BEFORE `BindToHandler`/`Invoke` — the app's stored `opened_image_path` is the `fs::canonicalize` output carrying the `\\?\` VERBATIM (extended-length) prefix, which the shell parsing-name API `SHCreateItemFromParsingName` rejects; the Wave-6.1 `BindToHandler` fix was correct but upstream-blocked (and the corrected WIC fact: the decode is invoked with the PRE-canonicalization path — opening never depended on the canonical form). (Task 2, commit `2b46a2e`) the evidence-decided fix: the pure `shell_parseable_path` (`src/open_with_listing.rs:93`; `\\?\UNC\` → `\\` first, `\\?\` → strip, pass-through otherwise; constants `VERBATIM_PATH_PREFIX`/`VERBATIM_UNC_PATH_PREFIX`) + the ONE call-site conversion in `windows_open_with.rs::file_data_object` BEFORE `SHCreateItemFromParsingName`; the stored canonical path untouched; NO semantics change; census 189 → 195 (+6 container tests joining the existing `src/open_with_listing/tests.rs`, 6 → 12). (Task 3, VERIFIED) dev-checks `ALL CHECKS PASSED` (195/0, 28 test-bearing files — the verified breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file); the windows-target compile exit 0 (the pre-existing benign `compare_entry_names` warning only); the release rebuild → exe 19,649,107 bytes with ZERO diagnostic code; the DLL audit SET-EQUAL 22; the two read-only sweep pin mismatches adjudicated as explained deltas (verdict VERIFIED). (Task 4) the wave documentation landed in `README.md`, `tech.md`, `architecture.md`, and this file (this step). REMAINING: the SINGLE 0.6.2 OPERATOR-RUN gate (Immediate Next Steps #1 — the PLAIN checklist at `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` §3) and global-plan Step 5 (Task-4 mark at 4.6, the TODO renamed `-DONE`, the tmp sweep incl. the diagnostic exe/log, the merge + push). Wave 6.1 (below) is COMPLETE — merged to `main` at `b458897` and pushed to `origin`; its 0.6.1 OPERATOR-RUN gate was SUPERSEDED by this single 0.6.2 run.
```

**E2 — Recent Changes: insert ONE new entry at the TOP of the numbered list (before the current `1. 2026-11-13: **Wave 6.2 Task 1 diagnostic run COMPLETE...**` entry):**

```markdown
1. 2026-11-13: **Wave 6.2 Tasks 1–3 landed + Task 4 documentation (this step) — the D1 launch defect is EVIDENCE-DECIDED and FIXED; census 195/28; exe 19,649,107 B with ZERO diagnostic code; the pending acceptance is the 0.6.2 OPERATOR-RUN (the PLAIN checklist at `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` §3).** Branch `fix/wave-6-2-open-with-launch`, version 0.6.2. Wave commit chain (verbatim, chronological, 15 commits): `fd7acce` (report+TODO), `c3611ae` (global plan), `a048567` (0.6.2 bump), `02f639a` (T1 plan), `4185d97` (diagnostic feature), `e8c3f0b` (T1 docs), `e1cdfc3` (T1 mark), `b2b47a0` (T2 plan), `2b46a2e` (the fix), `1aac7e1` (T2 docs), `0378d6b` (T2 mark), `9c8db64` (T3 plan), `4d87e75` (T3 results), `685e113` (T3 adjudication — the two read-only sweep pin mismatches adjudicated as explained deltas, verdict VERIFIED), `f46d5dd` (T3 mark). (Task 1) the caller-AUTHORIZED OPTIONAL diagnostic: the OFF-by-default `diagnostic-launch-log` Cargo feature (`Cargo.toml:6-8`, `default = []`) gating `src/platform/launch_log.rs` (`src/platform/mod.rs:15`; appends `img-scroller-diagnostic.log` next to the exe with the per-selection evidence: stored/fresh enumerations, the resolved `launch_index`, and the `SHCreateItemFromParsingName`/`BindToHandler`/`Invoke` outcomes with result codes; silent write-failure skip; never alters behavior); the release build NEVER enables it (`scripts/build-windows.sh` has NO `--features`) — the Task-1 facts are in the pre-existing entry below. (Task 2, commit `2b46a2e`) the evidence-decided fix: the pure `shell_parseable_path(canonical_path) -> PathBuf` in `src/open_with_listing.rs:93` (constants `VERBATIM_PATH_PREFIX`/`VERBATIM_UNC_PATH_PREFIX` at `:86-87`; `\\?\UNC\` → `\\` FIRST, then `\\?\` → strip, pass-through otherwise) + the ONE call-site conversion `let shell_path = shell_parseable_path(file_path);` as the FIRST statement of `file_data_object` (`src/platform/windows_open_with.rs:146`) BEFORE the wide encode and `SHCreateItemFromParsingName`; the enumeration keeps the canonical path (`.extension()` only, proven working); the stored canonical `opened_image_path` is NEVER rewritten (the D5 single-normalization point and every other consumer unchanged); NO semantics change (no handler filtering, no enumeration change — caller approval NOT needed); the corrected WIC fact recorded: the decode is invoked with the PRE-canonicalization path (`src/app.rs:72` before `fs::canonicalize` at `:73`) — opening never depended on the canonical form; 6 new container tests joined the existing `src/open_with_listing/tests.rs` (6 → 12; census 189 → 195). (Task 3, VERIFIED) dev-checks `ALL CHECKS PASSED` — 195 unit tests, 0 failed, across 28 test-bearing files (the verified breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules — `app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`); the windows-target compile exit 0 (ONLY the pre-existing benign dead-code warning on `compare_entry_names`); the release rebuild → `dist/windows-x64/img-scroller.exe` = 19,649,107 bytes (the 0.6.1 artifact 19,645,219 B overwritten atomically) with ZERO diagnostic code (Pins A/B/C all passed — the binary carries no `img-scroller-diagnostic` string); the DLL import audit SET-EQUAL 22 (std-only string operations, no new API imports); zero Cargo drift since `a048567` (the `[features]` block only). (Task 4, this step) `README.md`, `tech.md`, `architecture.md`, and this file updated to the Wave-6.2 facts (the feature-7 record + the 0.6.2 status paragraph; the launcher-bullet conversion + the diagnostic-module bullet + the Wave-6.2 verification bullet + the 195/28 Testing census + the Technical Constraints clauses; the Fast Open-with bullet + the module bullets incl. `launch_log.rs` + path-5 + the decisions-table row + the Status paragraph; the Current-Work-Focus rewrite, this entry, the Current-Project-State Wave-6.2 bullet with the micro-edits, and the Immediate-Next-Steps rewrite with the 0.6.2 gate); `brief.md` verified — `### Fast "Open with..." Menu` already mandates the fixed behavior, NO addendum (verify-only, NO edit); `.agent/project-structure.md` verified — FOUND STALE (the `platform/` bullet lacks `launch_log.rs`, the new src file landed at `4185d97`) — flagged to the caller per the binding NO-edit expectation (verify-only, NO edit this cycle); `docs/build-windows.md` verified — the audit SET-EQUAL, NO OBSERVED-list edit (verify-only, NO edit). ARTIFACT NOTE (recorded, do not act): the diagnostic exe (`img-scroller-diagnostic.exe`, 225,180,316 B) + log (`img-scroller-diagnostic.log`, 2,768 B) stay in `dist/windows-x64/` for a possible follow-up evidence run (already authorized) and are removed ONLY at the global plan's Step-5 tmp sweep. PENDING ACCEPTANCE GATE: the SINGLE 0.6.2 OPERATOR-RUN (Next-Steps #1); it supersedes the pending 0.6.1 checklist in one run.
```

**E3 — Current Project State, FOUR pinned edits:**

1. INSERT a new bullet at the TOP of the bullet list (before the current `` - **Wave 6.1 LANDED on `main`**... `` bullet) — EXACTLY:

```markdown
- **Wave 6.2 LANDED on the branch `fix/wave-6-2-open-with-launch` (agent-side COMPLETE through Task 4 docs; NOT yet merged — Step 5 pending; TODO `.agent/todos/20261113/20261113-todo-2.md` has Task 4's mark at 4.6 pending, then the `-DONE` rename at Step 5)**: version 0.6.2; the D1 launch defect EVIDENCE-DECIDED and FIXED — the optional caller-authorized `diagnostic-launch-log` feature (OFF by default; the release never enables it) produced the root-cause evidence (`SHCreateItemFromParsingName` rejects the `fs::canonicalize` `\\?\` VERBATIM prefix with E_INVALIDARG on every attempt; the Wave-6.1 `BindToHandler` fix was correct but upstream-blocked), and the fix (commit `2b46a2e`) is the pure `shell_parseable_path` conversion at the ONE launch hand-off (`windows_open_with.rs:146` — `\\?\UNC\` → `\\` first, `\\?\` → strip, pass-through otherwise; the stored canonical path untouched; NO semantics change; the corrected WIC fact: the decode uses the PRE-canonicalization path); census 195 unit tests / 28 test-bearing files (19 `*_tests.rs` + 4 `tests.rs` + 5 in-file); dev-checks `ALL CHECKS PASSED` (195/0); exe rebuilt 19,649,107 bytes (0.6.2; the 0.6.1 artifact 19,645,219 B overwritten atomically) with ZERO diagnostic code; DLL audit set-equal 22; the pending 0.6.2 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1 — the PLAIN checklist at `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` §3).
```

2. In the EXISTING Wave-6.1 bullet (line 75), replace the final clause `` the pending 0.6.1 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1). `` with:

```markdown
the 0.6.1 OPERATOR-RUN gate was SUPERSEDED by the Wave-6.2 0.6.2 OPERATOR-RUN (Next-Steps #1).
```

3. In the release-artifact bullet (line 90), replace the parenthetical `` (19,645,219 bytes, 0.6.1 Wave-6.1 rebuild; the 0.6.0 artifact 19,647,219 B overwritten atomically) `` with EXACTLY:

```markdown
(19,649,107 bytes, 0.6.2 Wave-6.2 rebuild; the 0.6.1 artifact 19,645,219 B overwritten atomically; the diagnostic exe + log also in `dist/windows-x64/` are gitignored, stay available for a follow-up evidence run, and are removed at the wave's Step-5 tmp sweep)
```

4. TWO pinned edits in TWO bullets:
   a. In the TODO-state bullet (line 91), insert EXACTLY between `` - **TODO state**: `` and `` **Wave 6.1** `` (i.e. the insert goes immediately after the colon-space and before the bolded `**Wave 6.1**`):

```markdown
**Wave 6.2 (IN PROGRESS — the branch is NOT merged)**: `.agent/todos/20261113/20261113-todo-2.md` — Tasks 1–3 marked `[DONE]` (the Task-3 manual-UX OPERATOR-RUN sub-item STAYS `[ ]` — the user's 0.6.2 gate, Next-Steps #1); Task 4's mark is the wave's 4.6 step; the rename to `-DONE` happens at global-plan Step 5. 
```

   (Note the trailing space after `Step 5.` — the insert ends with ONE space before the existing `**Wave 6.1**`.)
   b. In the Git bullet (line 94), replace the leading segment `` Git: `main` == `origin/main` == `5f4f8ec` (working tree clean at this closing step); `` with EXACTLY:

```markdown
Git: `main` == `origin/main` == `b458897` (the Wave-6.1 closing state; the Wave-6.2 branch `fix/wave-6-2-open-with-launch`, created from `b458897`, sits at `f46d5dd` and is NOT yet merged — the merge + push belong to the wave's Step 5);
```

**E4 — Immediate Next Steps: REPLACE the ENTIRE numbered list (items 1–3) with the following three items (item 1 embeds the §3 checklist of `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` VERBATIM — copy it exactly, line breaks included):**

Item 1:

```markdown
1. **Pending 0.6.2 Wave-6.2 OPERATOR-RUN gate — the user's run on a real Windows 10/11 machine**, running the freshly rebuilt 0.6.2 exe at `dist\windows-x64\img-scroller.exe`, with a png image open (the same one as before, e.g. `[Image 1]`) — the PLAIN checklist, verbatim from `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` §3:
   (1) Open a png image. Click the "Open with..." button (or press Q) and select PAINT in the list: the image opens in Paint with NO failure message.
   (2) Open the list again and select PHOTOS: the image opens in Photos with NO failure message. (If you select img-scroller itself in the list, a second viewer window opens — that is EXPECTED, not a failure.)
   (3) Press Q: the "Open with..." menu opens. Press Q again: the menu closes.
   (4) While the menu is open, press other keys and click other buttons (arrows, zoom, fit, fullscreen): they all do NOTHING.
   (5) Press ESC while the menu is open: ONLY the menu closes — the "Press ESC again to close" app-exit popup does NOT appear.
   (6) Click outside the menu: the menu dismisses.
   (7) The "Q" caption renders under the open-with button symbol (white, thin black outline).
   (8) If a launch ever fails, the failure message shows in WARNING (orange) color on the pill's message line.
   (9) Regression block — everything that already passed on 0.6.1 still works: arrow navigation and folder navigation (including the navigation name order); zoom in/out/restore (I / O / 0, Shift+'+' / Shift+'−'); fit width/height (W / H); fullscreen (F11, ESC exit); the double-ESC close contract; pan at zoomed image edges; opening files; and the open-with launch state (the tooltip "Open with... (Q)"; Q stays inert when no programs are registered for the file type, in empty-folder mode, and in fullscreen; an empty list shows "No programs registered for this file type").
   STANDING NOTE (do not skip): if item 1 or 2 STILL fails, do NOT delete anything — the diagnostic build (`dist\windows-x64\img-scroller-diagnostic.exe`, writing `img-scroller-diagnostic.log` next to it) stays available for a follow-up evidence run; no new authorization is needed, it is already authorized. The 0.6.1 gate is SUPERSEDED by this single 0.6.2 run; one run, one gate.
```

Item 2:

```markdown
2. **After the 0.6.2 gate closes — the wave's global-plan Step 5**: mark Task 4 `[DONE]` at 4.6 (preserve file content), rename the TODO `.agent/todos/20261113/20261113-todo-2.md` → `20261113-todo-2-DONE.md`, run the tmp sweep (NO tmp files expected; the diagnostic exe + log in `dist/windows-x64/` are REMOVED here per the T3 results standing note — `dist/` is gitignored), check out `main`, fast-forward merge `fix/wave-6-2-open-with-launch` into `main`, push `main` → `origin` ONLY, delete the branch, and land the FINAL closing context.md touch (the Wave-6.2 closing entry in Current Work Focus / Current Project State / Git). Task 3's manual-UX sub-item is marked `[x]` with the dated PASS report at this point.
```

Item 3:

```markdown
3. **NEXT SESSION — Wave 7 (`.agent/todos/20261029/20261029-todo-9.md`, the x64 AND x86 packaging matrix) runs in a NEW chat after Wave 6.2 closes at Step 5**; the 0.6.2 gate (item 1) stays the user's responsibility and closes when the user reports PASS; the parked future-ideas list stays OUT of scope until scheduled.
```

---

## §3 VERIFY-ONLY ITEMS (expected verdicts; read + record, NO edit)

- **V1 — `.agent/project-structure.md` — the STALE-MAP finding (record-and-flag protocol):**
  1. Read the file; re-verify live that the `platform/` bullet (line 25) does NOT mention `launch_log.rs` while `git log b458897..HEAD -- .agent/project-structure.md` is EMPTY (the map was never updated after `4185d97` added `src/platform/launch_log.rs` THIS wave — §0 fact 10c).
  2. The caller's binding expectation is "confirm accurate, NO edit expected". The map is NOT accurate. RESOLUTION PINNED BY THE CALLER'S CONSTRAINT: do NOT edit the map in this cycle; do NOT stop the cycle. Record the finding (the exact missing entry) in the completion summary — the §2.4 E2 context.md entry already carries the sentence (verbatim, plain backticks) `` `.agent/project-structure.md` verified — FOUND STALE (the `platform/` bullet lacks `launch_log.rs`, the new src file landed at `4185d97`) — flagged to the caller per the binding NO-edit expectation (verify-only, NO edit this cycle) `` — no additional edit exists for the map.
  3. Any further resolution (a one-line structure-map micro-edit) belongs to the caller, NOT this cycle.
- **V2 — `brief.md` — expected verdict NO ADDENDUM, NO edit**: `` ### Fast "Open with..." Menu `` already mandates "Selecting a program opens the current file with it"; the fix realizes the existing requirement with NO semantics change; the diagnostic is an agent-side build option, not a user-facing requirement. Record the verdict in the completion summary (the §2.4 E2 context.md entry already carries "`brief.md` verified — `### Fast "Open with..." Menu` already mandates the fixed behavior, NO addendum (verify-only, NO edit)").
- **V3 — `docs/build-windows.md` — expected verdict NO edit**: the DLL audit stayed SET-EQUAL (T3 results Step 4); the doc's OBSERVED-DLL list is unchanged; the doc carries NO diagnostic references. Record the verdict (the E2 context.md entry carries it).

---

## §4 COHERENCE + VERIFICATION (after all edits, before the commit)

1. Read each edited file's edited regions in full: real newlines everywhere (never literal `\n`), the long paragraphs stay single-line, the markdown lists stay valid.
2. Grep verifications (host, one command each, exit/output recorded in the completion summary):
   - `git grep -n "0.6.2" README.md` → the Language row + the Wave-6.2 Project-Status record present.
   - `git grep -n "shell_parseable_path" -- .agent/project-info` → hits in tech.md (the launcher bullet) and architecture.md (the module bullets + the decisions row).
   - `git grep -n "diagnostic-launch-log" -- .agent/project-info README.md` → hits in tech.md (Dependencies + the diagnostic bullet + the launcher bullet), architecture.md (the Fast Open-with bullet + the platform bullet + the decisions row), README.md (feature 7 + the status record), context.md.
   - `git grep -n "19,649,107" -- .agent/project-info README.md` → hits in all four files.
   - `git grep -n "launch_log.rs" -- .agent/project-info` → hits in tech.md (the diagnostic bullet), architecture.md (the platform bullet), context.md (the wave records).
   - `git grep -n "195 unit tests" -- .agent/project-info/tech.md` → the Testing section lead.
   - `git grep -n "0.6.1" README.md` → ONLY historical mentions (the Wave-6.1 status record, the feature-7 Wave-6.1 sentence, the 0.6.2 record's "supersedes" clause); the Technology Stack table row must read 0.6.2.
3. NO build/harness run exists in this cycle (docs only; no `dist/`, `logs/`, or `target/` interaction).
4. 4.5b adherence hooks (for the later step): every §2 edit landed verbatim; §3 verdicts recorded; `git diff f46d5dd..HEAD --stat` shows EXACTLY: `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`, and THIS plan file — nothing else; no `.kilo/plans/*` existing file touched; no `CHANGELOG.md`.

---

## §5 COMMIT PROTOCOL (docs-specialist)

1. **Step 0 (at 4.2 start):** host `git status` (clean tree expected except THIS plan file); read `.gitignore`; stage ONLY `.kilo/plans/20261113-wave6-2-task4-documentation-plan.md`; `git status` check (NO gitignore-matching file staged); commit with the message EXACTLY: `docs: add wave6.2 task4 documentation plan`.
2. **After §2 + §3 + §4 all pass:** read `.gitignore` (re-check), run `git status`, stage ONLY the four doc files: `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`; `git status` check; commit with the message EXACTLY: `docs: wave6.2 task4 documentation`.
3. Re-run `git status` → clean tree. NO push, NO branch creation/switch/merge, NO TODO edit (4.6 does the Task-4 mark), NO `CHANGELOG.md`, NO edit to any existing `.kilo/plans/*` file, NO `dist/`/`logs/` interaction (the diagnostic exe + log stay untouched — their removal is the Step-5 tmp sweep).

---

## §6 SCOPE BOUNDARIES (hard)

- Documentation ONLY: the four §2 files + THIS plan file. NOTHING else is written.
- NO source code, NO tests, NO Cargo manifest/lock change, NO script change, NO harness run, NO diagnostic rebuild, NO feature change.
- NO edits to: `.agent/todos/*` (4.6 does the marking), existing `.kilo/plans/*` files, `CHANGELOG.md`, `docs/*`, `brief.md`, `.agent/project-structure.md` (the V1 finding is recorded-and-flagged, NOT edited), `.agent/project-info/product.md`, `future-ideas.md`.
- NO push, NO merge, NO branch operation, NO TODO rename.
- NOTHING in `dist/` or `logs/` is created, deleted, or modified.

---

## §7 VERBATIM COMPLIANCE vs TODO TASK 4

| TODO Task 4 sub-item | Covered by |
| --- | --- |
| `README.md`, `tech.md`, `architecture.md`, `context.md`: record the diagnostic evidence | §2.1 E1/E4, §2.2 E4/E5, §2.3 E2/E4/E6, §2.4 E1/E2 |
| ...the root cause | §2.1 E1/E4, §2.2 E4 (the verbatim-prefix/E_INVALIDARG root cause + the corrected WIC fact), §2.3 E2/E6, §2.4 E1/E2 |
| ...the fix | §2.1 E1/E4 (`2b46a2e`), §2.2 E4, §2.3 E3/E4/E5/E6, §2.4 E1/E2 |
| ...the 0.6.2 state | §2.1 E3/E4, §2.2 E2/E5/E6, §2.3 E1/E6, §2.4 E1/E2/E3 |
| ...and remaining open items | §2.4 E4 (the 0.6.2 OPERATOR-RUN gate + Step 5 + Wave 7) and the E1/E2 "REMAINING" clauses |

## §8 SOURCES (consulted this session)

- Read in full: the source TODO, `.agent/WORKFLOWS.md`, `AGENTS.md`, `.agent/project-info/instructions.md`, the four doc targets (`README.md`, `tech.md`, `architecture.md`, `context.md`), `brief.md`, `.agent/project-structure.md`, the T2 fix plan, the T3 verification plan (+ §3 checklist) and results, `src/open_with_listing.rs` + its tests, `src/platform/launch_log.rs`, `src/platform/windows_open_with.rs`, `Cargo.toml`.
- Verified live: `git status`, `git log --oneline b458897..HEAD` (the 15-commit chain), `git diff b458897..HEAD --stat`, the project-structure staleness (`git log` EMPTY for the map), and the targeted `findstr` extracts of the truncated long lines (README 26/49/69; tech.md 5; architecture.md 5/30/39/63/70/91/142).
- Wave precedents for the docs-step shape: `.kilo/plans/20261113-wave6-1-task6-documentation-plan.md` (referenced only; NOT edited) and the Wave-6/6.1 documentation records in `context.md`.
