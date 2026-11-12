# Implementation Plan — Wave 5.2, Task 5: Documentation Updates (the Wave-5.2 mechanism-swap record)

Branch: `fix/wave-5-2-name-order` (already exists; NO branch/version actions in this cycle) · Version: 0.5.2 (already bumped at Step 3, commit `064882a`) · TODO source: [`.agent/todos/20261112/20261112-todo-2.md`](../../.agent/todos/20261112/20261112-todo-2.md) → `### Task 5: Documentation updates` ONLY · Global plan: [`.kilo/plans/20261112-wave5-2-name-order-global-plan.md`](20261112-wave5-2-name-order-global-plan.md) → Task 5 · This is the 4.1b architector output for Task 5 (the 4.2 docs-specialist executes it). NOT front-end — no 4.1a exists or is needed.

- Docs to edit: `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`.
- Verify-only (NO edit expected): `.agent/project-info/brief.md` (addendum item 8 already governs), `.agent/project-structure.md` (already updated at the Tasks-2+3 landing — verified at HEAD: the `platform/` line already says "the system file manager's logical name comparison via StrCmpLogicalW — digit-aware natural ordering, case-insensitive; shlwapi" and the `name_collation.rs` line already says "the pure fallback ... used in container/stub builds").
- NO `CHANGELOG.md` touches (wave standing ban). NO edits to existing `.kilo/plans/*` files (new plan files only). NO code-file touches of any kind.
- Target implementer: JUNIOR under 50% restriction. Every edit below is exact old→new or exact-append text; zero judgment left. If any detail conflicts with something ACTUALLY observed during 4.2, STOP and return the question to the caller.

---

## §0 Session facts captured at plan time (verified evidence; branch `fix/wave-5-2-name-order` @ HEAD `8f50806`, clean tree)

- The wave commit chain below the tip (host `git log --oneline -20` verified at plan time): `8f50806` (task: mark wave5.2 task4 done) ← `ae7d3f9` (docs: add wave5.2 task4 verification plan) ← `5dfb85b` (task: mark wave5.2 tasks 2-3 done) ← `f3ddef2` (fix: wave5.2 review fix — windows_collation formatting) ← `d188b0b` (fix: follow the system file manager name order) ← `437640a` (docs: add wave5.2 tasks 2-3 plan) ← `064882a` (chore: bump version to 0.5.2) ← `a10c8ab` (docs: add wave5.2 global plan) ← `55af213` (docs: add wave5.2 todo) ← `a11c080` (wave base — docs: record wave5.1 completion in context log). `main` == `origin/main` == `6c498c4` (the branch is NOT merged; Step 5 pending).
- Verified Wave-5.2 ground truth (caller-verified at HEAD, cross-checked against the Tasks-2+3 plan and the Task-4 verification plan §0):
  - Root cause: the operator's real folder `智乃 (1).jpg … 智乃 (97).jpg` proved the expectation is the SYSTEM FILE MANAGER's exact order — Explorer-exact `StrCmpLogicalW` (digit-aware NATURAL ordering: `(1)`, `(2)`, …, `(9)`, `(10)`, …; case-insensitive; shlwapi). BOTH the Wave-5.1 `CompareStringEx` user-default-locale mechanism AND the original code-point order sort digit runs LEXICALLY (`(1)`, `(10)`, `(11)`, …, `(19)`, `(2)`, …) — the operator correctly reported that as wrong. Verdict: Suspect C (expectation/mechanism gap), pre-approved by brief addendum item 8; the Suspect-B wiring audit cleared every production path.
  - The fix (commits `d188b0b` + review-fix `f3ddef2`): `src/platform/windows_collation.rs` is now the 17-line `StrCmpLogicalW` comparator (`system_collation_order` :11; null-terminated UTF-16 buffers :12–:13; the single unsafe call :14–:15; `result.cmp(&0)` :16) — NO failure path (the Wave-5.1 `Option<Ordering>`/silent-fallback layer REMOVED); `src/platform/windows.rs`'s `compare_entry_names_system` wrapper (:157–:159) delegates DIRECTLY (no fallback call); the two Wave-5.1 windows features `Win32_Globalization` + `Win32_System_SystemServices` REMOVED from `Cargo.toml` (the original SIX remain; `Cargo.lock` untouched).
  - UNCHANGED surface: the `EntryNameOrder` seam (`src/name_collation.rs`), BOTH listing consumers (`folder_listing.rs`, `sibling_folder_listing.rs`), the anchor position (`listed_folder_name_precedes`), `first_listed_image`, the stub delegation (`stub.rs:19–:20`), and ALL container tests. The pure `compare_entry_names` fallback remains the stub/container path — DEAD CODE on the Windows release target (acceptable-as-recorded; stays container-tested; any cfg-gate cleanup is deferred to a maintenance wave — this note MUST be recorded in the docs).
  - Verification (Task 4 GREEN): census UNCHANGED 164 tests / 26 files (zero test delta); dev-checks `ALL CHECKS PASSED` (164/0); the 0.5.2 exe rebuilt at `dist/windows-x64/img-scroller.exe` = **19,605,828 bytes** (the 0.5.1 artifact 19,636,215 B overwritten atomically); DLL import audit SET-EQUAL 22 (`StrCmpLogicalW` is a shlwapi.dll import and `shlwapi.dll` is ALREADY in the recorded `OBSERVED_DLL_IMPORT_LIST`; the removed `CompareStringEx`/`GetUserDefaultLocaleName` were kernel32 exports and kernel32.dll remains imported).
  - The Wave-5.1 gate results (recorded with the wave): Shift+'+' zoom-in PASS (round-2 alias fix confirmed); popup auto-dismiss after ~2 s PASS (wake fix confirmed); "all the rest OK" — the name-order DEFECT persisted (the Wave-5.1 fix did not take effect on real Windows), which is THIS wave's origin.
  - Pending: the SINGLE 0.5.2 OPERATOR-RUN re-test — the Task-4 plan §11 checklist (the natural-order re-test with the 智乃 folder + standing regressions); it SUPERSEDES the still-open 0.5.1 checklist items in one run.
- **CALLER-VISIBLE DISCREPANCY (recorded, NOT assumed away): the working-tree `context.md` at HEAD does NOT yet contain a Wave-5.2 dated entry** — the todo's Task 1 ("Record the second 2026-11-12 report") is UNCHECKED and no separate context.md record exists (verified: the `## Current Work Focus` and `## Recent Changes` sections still carry only the Wave-5.1 closing state). Resolution encoded by THIS plan (deterministic, no assumption): the Task-5 landing entry written into `context.md` (§5.2) INCLUDES the second-report facts (the two PASSes + the persisting defect + the diagnostic data), so nothing is lost whether or not Task 1 is later executed separately. Whether to retroactively execute/mark Task 1 is the CALLER's decision — NOT this cycle's work.
- The Task-1 `20261112-wave5-1-task1-record-report-plan.md` precedent shape is the model for the §5 context.md entry, adapted to the Wave-5.2 landing facts.

---

## §1 Editing conventions for the 4.2 docs-specialist (binding)

- All four docs keep their current structure; edits are surgical — exact old→new replacements or exact appends as specified in §2–§5. NO restructuring, NO renumbering of lists/tables, NO drive-by rewording beyond the specified fragments.
- The docs are Markdown; several target lines are longer than the reader's display width — locate fragments by the UNIQUE quoted search string, never by column position. After each edit, re-read the touched line to confirm the rest of the line is byte-identical.
- Every quoted old string in §2–§5 is verified verbatim at HEAD `8f50806`. If a quoted old string is NOT found in the file, STOP and return the question to the caller (do NOT improvise a different anchor).
- Fact discipline: every fact written into the docs comes from §0 or from the binding plans named in §0 (the Tasks-2+3 plan, the Task-4 verification plan). NO new claims (no new numbers, no new file/line references beyond those recorded here).

## §2 README.md edits (root: `README.md`)

### 2.1 Feature-2 name-order sentence (line 21)

Replace the parenthetical clause:

OLD (exact fragment inside line 21):
`(the binding 2026-11-12 operator correction: the order is the names' expected Unicode/system order, not raw character codes; the pure lowercase-form alphanumeric fallback is kept for system-collation failures)`

NEW (exact):
`(the binding 2026-11-12 operator corrections: the order is the SYSTEM FILE MANAGER's exact order — digit-aware NATURAL/numeric ordering (the operator's real \`智乃 (1).jpg … 智乃 (97).jpg\` folder navigates \`(1)\`, \`(2)\`, …, \`(9)\`, \`(10)\`, …), case-insensitive, the \`StrCmpLogicalW\` comparator since Wave 5.2 — not raw character codes and not lexical digit runs; the pure lowercase-form alphanumeric fallback is kept for container/stub builds only)`

### 2.2 Technology-Stack intro wave list (line 48)

Replace:
OLD: `and the Wave-4.2 operator-feedback corrections; the remaining v1 feature`
NEW: `and the Wave-4.2 operator-feedback corrections, the Wave-5 pan-then-navigate slice, the Wave-5.1 operator-feedback corrections, and the Wave-5.2 name-order mechanism swap; the remaining v1 feature`

### 2.3 Version row (line 52)

Replace: OLD `Implemented: Cargo package \`img-scroller\` 0.5.1, edition 2024.` → NEW `Implemented: Cargo package \`img-scroller\` 0.5.2, edition 2024.`

### 2.4 Project-Status paragraph (line 68) — append the Wave-5.2 record

The paragraph currently ends (verified tail) with:
`... Pending operator acceptance: the SINGLE 0.5.1 re-test (Shift+'+' in; the popup auto-dismiss; the name order on both axes; the standing regressions).`

APPEND directly after that final period, as new sentences in the SAME paragraph/line:

` Wave 5.2 landed (branch \`fix/wave-5-2-name-order\`, version 0.5.2): the name-order mechanism was SWAPPED to the system file manager's exact comparator \`StrCmpLogicalW\` (shlwapi) — digit-aware NATURAL/numeric ordering: the operator's real \`智乃 (1).jpg … 智乃 (97).jpg\` folder navigates \`(1)\`, \`(2)\`, …, \`(9)\`, \`(10)\`, … exactly as Explorer shows, fixing the lexical digit-run order (\`(1)\`, \`(10)\`, …, \`(19)\`, \`(2)\`, …) that BOTH the original code-point order and the Wave-5.1 \`CompareStringEx\` collation produced; the two Wave-5.1 windows-crate features were removed and the original six remain. Verification: dev checks \`ALL CHECKS PASSED\` (164 unit tests, 0 failed, across 26 test files), the release executable rebuilt at \`dist/windows-x64/img-scroller.exe\` (19,605,828 bytes), and the DLL import audit unchanged (the recorded 22-DLL set). Pending operator acceptance: the SINGLE 0.5.2 re-test — the natural/numeric name order with the real Japanese-mixed folder on BOTH axes (images in the folder; sibling folders), plus the standing regressions — it supersedes the still-open 0.5.1 checklist items in one run.`

## §3 tech.md edits (root: `.agent/project-info/tech.md`)

### 3.1 Status paragraph (line 5) — append the Wave-5.2 sentence

The paragraph currently ends (verified tail) with:
`... the 0.5.1 exe rebuilt at \`dist/windows-x64/img-scroller.exe\` (19,636,215 bytes), and the DLL import audit set-equal to the recorded 22-DLL constant.`

APPEND directly after it:
` Wave 5.2 (branch \`fix/wave-5-2-name-order\`, version 0.5.2, 2026-11-12): the name-order mechanism was SWAPPED to the system file manager's \`StrCmpLogicalW\` comparator (shlwapi; digit-aware natural ordering; NO failure path — mechanism: \`## Entry-Name System Order (Implemented, Wave 5.1; Mechanism Wave 5.2)\` below); the two Wave-5.1 windows features were removed (the original six remain). Verification: dev checks \`ALL CHECKS PASSED\` (164 unit tests), the 0.5.2 exe rebuilt at \`dist/windows-x64/img-scroller.exe\` (19,605,828 bytes), and the DLL import audit set-equal to the recorded 22-DLL constant.`

### 3.2 Contents item 10 (line 18)

Replace: OLD `10. [Entry-Name System Order (Implemented, Wave 5.1)](#entry-name-system-order-implemented-wave-51)`
NEW: `10. [Entry-Name System Order (Implemented, Wave 5.1; Mechanism Wave 5.2)](#entry-name-system-order-implemented-wave-51-mechanism-wave-52)`

### 3.3 Technology-Stack version (line 32)

Replace: OLD `Cargo package \`img-scroller\` version 0.5.1 (single binary; tracked \`Cargo.lock\`).` → NEW `Cargo package \`img-scroller\` version 0.5.2 (single binary; tracked \`Cargo.lock\`).`

### 3.4 Dependencies bullet (line 50) — the six-feature shape

Replace the ENTIRE bullet:
OLD (exact):
`- Windows-target-scoped (under \`[target.'cfg(windows)'.dependencies]\`): \`windows\` \`=0.62.2\` with exactly EIGHT features — \`Win32_Foundation\`, \`Win32_Graphics_Imaging\`, \`Win32_Globalization\` (Wave 5.1: the system entry-name collation — \`CompareStringEx\`/\`GetUserDefaultLocaleName\`), \`Win32_System_Com\`, \`Win32_System_SystemServices\` (Wave 5.1: \`LOCALE_NAME_MAX_LENGTH\`), \`Win32_UI_Shell\`, \`Win32_UI_Shell_Common\`, \`Win32_UI_WindowsAndMessaging\`. The two Wave-5.1 additions compile only into the Windows target; both added functions are kernel32 exports, so the DLL import audit stayed SET-EQUAL to the recorded 22-DLL constant (the deciding evidence — mechanism: \`## Entry-Name System Order (Implemented, Wave 5.1)\` below).`

NEW (exact):
`- Windows-target-scoped (under \`[target.'cfg(windows)'.dependencies]\`): \`windows\` \`=0.62.2\` with exactly SIX features — \`Win32_Foundation\`, \`Win32_Graphics_Imaging\`, \`Win32_System_Com\`, \`Win32_UI_Shell\`, \`Win32_UI_Shell_Common\`, \`Win32_UI_WindowsAndMessaging\`. The two Wave-5.1 additions (\`Win32_Globalization\`, \`Win32_System_SystemServices\`) were REMOVED in Wave 5.2 when the entry-name mechanism swapped to \`StrCmpLogicalW\` (a shlwapi export needing only \`Win32_UI_Shell\`, already present since Wave 1); the DLL import audit stayed SET-EQUAL to the recorded 22-DLL constant (the deciding evidence — mechanism: \`## Entry-Name System Order (Implemented, Wave 5.1; Mechanism Wave 5.2)\` below).`

### 3.5 Horizontal-mechanism order bullet (line 68)

Replace:
OLD: `- Order (no settings UI): since Wave 5.1 the SYSTEM entry-name order (mechanism: \`## Entry-Name System Order (Implemented, Wave 5.1)\` below); the RETAINED pure fallback compares by lowercase form first with an exact-character tie-break (e.g. \`alpha.gif\` < \`ALPHA.jpg\` < \`Alpha.jpg\` < \`alpha.jpg\`) and is the order used on system-collation failures and in container/stub builds.`
NEW: `- Order (no settings UI): the SYSTEM entry-name order — the system file manager's exact order (mechanism: \`## Entry-Name System Order (Implemented, Wave 5.1; Mechanism Wave 5.2)\` below); the RETAINED pure fallback compares by lowercase form first with an exact-character tie-break (e.g. \`alpha.gif\` < \`ALPHA.jpg\` < \`Alpha.jpg\` < \`alpha.jpg\`) and is the order used in container/stub builds (the Windows comparator never fails).`

### 3.6 Vertical-mechanism order bullet (line 80)

Replace:
OLD: `- Order: the SAME entry-name order as the image listing, shared via \`src/name_collation.rs\` — since Wave 5.1 the SYSTEM name order through the \`EntryNameOrder\` seam, with the pure fallback collation (lowercase form first, exact-character tie-break) on system failures and stub builds (mechanism: \`## Entry-Name System Order (Implemented, Wave 5.1)\` below) — one order for files AND folders.`
NEW: `- Order: the SAME entry-name order as the image listing, shared via \`src/name_collation.rs\` — the SYSTEM name order through the \`EntryNameOrder\` seam, with the pure fallback collation (lowercase form first, exact-character tie-break) in container/stub builds (mechanism: \`## Entry-Name System Order (Implemented, Wave 5.1; Mechanism Wave 5.2)\` below) — one order for files AND folders.`

### 3.7 Entry-Name System Order section — retitle + rewrite (lines 121–130)

3.7a Heading (line 121). Replace:
OLD: `## Entry-Name System Order (Implemented, Wave 5.1)`
NEW: `## Entry-Name System Order (Implemented, Wave 5.1; Mechanism Wave 5.2)`
(the new GitHub anchor is `#entry-name-system-order-implemented-wave-51-mechanism-wave-52` — already wired in §3.2)

3.7b Section intro (line 123). Replace:
OLD: `Landed in Wave 5.1 (branch \`fix/wave-5-1-operator-feedback\`, version 0.5.1; the 2026-11-12 operator defect: navigation with Japanese-mixed names diverged from the expected system/Unicode order on BOTH axes). Implemented in \`src/name_collation.rs\` (the seam + the fallback), \`src/platform/windows_collation.rs\` + \`src/platform/windows.rs\` (the Windows collation), and \`src/platform/stub.rs\` (the container fallback), consumed by \`src/folder_listing.rs\` + \`src/sibling_folder_listing.rs\`:`
NEW: `Landed in Wave 5.1 (branch \`fix/wave-5-1-operator-feedback\`, version 0.5.1; the 2026-11-12 operator defect: navigation with Japanese-mixed names diverged from the expected system/Unicode order on BOTH axes) and MECHANISM-SWAPPED in Wave 5.2 (branch \`fix/wave-5-2-name-order\`, version 0.5.2). The root cause (caller-confirmed with the operator data): the operator's real folder \`智乃 (1).jpg … 智乃 (97).jpg\` must navigate in the SYSTEM FILE MANAGER's order — Explorer-exact \`StrCmpLogicalW\` — while BOTH the Wave-5.1 user-default-locale collation and the original code-point order sort digit runs LEXICALLY (\`(1)\`, \`(10)\`, …, \`(19)\`, \`(2)\`, …), which the operator correctly reported as wrong. Implemented in \`src/name_collation.rs\` (the seam + the fallback), \`src/platform/windows_collation.rs\` + \`src/platform/windows.rs\` (the Windows collation), and \`src/platform/stub.rs\` (the container fallback), consumed by \`src/folder_listing.rs\` + \`src/sibling_folder_listing.rs\`:`

3.7c The order bullet (line 125). Replace:
OLD: `- The order (the 2026-11-12 binding correction — brief addendum item 8): BOTH navigation axes order entry names by the SYSTEM name order — the user-default locale's \`CompareStringEx\` with \`NORM_IGNORECASE\` (\`src/platform/windows_collation.rs\`: \`system_collation_order\` :15, the locale via \`GetUserDefaultLocaleName\` :35; ANY system-call failure surfaces as \`None\` :42–:49). Pure-ASCII names keep the identical order they had under the fallback (test-pinned).`
NEW: `- The order (the 2026-11-12 binding corrections — brief addendum item 8): BOTH navigation axes order entry names by the SYSTEM FILE MANAGER's exact order — \`StrCmpLogicalW\` (shlwapi), digit-aware NATURAL/numeric ordering and case-insensitive (\`src/platform/windows_collation.rs\` — 17 lines total: \`system_collation_order\` :11; each name encoded to a null-terminated UTF-16 buffer :12–:13; the single unsafe call :14–:15; the \`i32\` result mapped by \`result.cmp(&0)\` :16). The comparison NEVER FAILS — the Wave-5.1 \`Option<Ordering>\`/silent-fallback layer was REMOVED with the swap, and the Wave-5.1 \`CompareStringEx\`/\`GetUserDefaultLocaleName\`/\`NORM_IGNORECASE\` mechanism is GONE (zero residue in \`src/\` + \`Cargo.toml\` — verified at HEAD).`

3.7d The seam bullet (line 126) — VERIFY ONLY (still accurate at HEAD: `EntryNameOrder` :24, `system()` :29, the resolution :31, `with_comparator` :35). NO edit.

3.7e The fallback bullet (line 127). Replace:
OLD: `- The fallback RETAINED: the pure \`compare_entry_names\` (\`src/name_collation.rs:9–:15\` — lowercase form first, exact-character tie-break, the fixed v1 alphanumeric rule) is the order used (a) when the Windows collation fails (\`src/platform/windows.rs\` \`compare_entry_names_system\` :159–:162 falls back on \`None\`) and (b) in container/stub builds (the stub's \`compare_entry_names_system\` at \`src/platform/stub.rs:19–:21\` delegates to it — the container build's system order IS the fallback order).`
NEW: `- The fallback RETAINED as the container/stub order ONLY: the pure \`compare_entry_names\` (\`src/name_collation.rs:9–:15\` — lowercase form first, exact-character tie-break, the fixed v1 alphanumeric rule) is the order used in container/stub builds (the stub's \`compare_entry_names_system\` at \`src/platform/stub.rs:19–:20\` delegates to it — the container build's system order IS the fallback order). The Windows side has NO failure path and NO fallback: \`src/platform/windows.rs\` \`compare_entry_names_system\` :157–:159 delegates DIRECTLY to \`super::windows_collation::system_collation_order\`. KNOWN NOTE (recorded, acceptable-as-is): on the Windows release target the pure \`compare_entry_names\` is DEAD CODE (only the stub path calls it); it stays container-tested; any cfg-gate cleanup is deferred to a maintenance wave.`

3.7f The consumers bullet (line 128) — VERIFY ONLY (line refs still valid: `folder_listing.rs` :33/:56–:58/:69; `sibling_folder_listing.rs` :45/:108–:112/:131–:137). NO edit.

3.7g The dependency+audit bullet (line 129). Replace:
OLD: `- Dependency + audit: \`Win32_Globalization\` + \`Win32_System_SystemServices\` added to the \`windows\` crate features (the wave's ONLY dependency change); \`CompareStringEx\`/\`GetUserDefaultLocaleName\` are kernel32 exports — the DLL import audit stayed SET-EQUAL to the recorded 22-DLL constant (the deciding evidence, never an assumption).`
NEW: `- Dependency + audit (Wave 5.2): the two Wave-5.1 windows-crate features \`Win32_Globalization\` + \`Win32_System_SystemServices\` were REMOVED (the original six remain — \`## Dependencies\`); \`StrCmpLogicalW\` needs only \`Win32_UI_Shell\`, already present since Wave 1; \`Cargo.lock\` untouched (feature removals are lock-neutral). \`StrCmpLogicalW\` is a shlwapi.dll export and \`shlwapi.dll\` is ALREADY in the recorded 22-DLL import list (\`scripts/build-windows.sh\` line 15) — the Wave-5.2 DLL import audit stayed SET-EQUAL to the recorded 22-DLL constant (kernel32 remains imported; the deciding evidence, never an assumption).`

3.7h The tests bullet (line 130) — VERIFY ONLY (accurate: the seam/collation tests unchanged; census ref). NO edit.

### 3.8 Testing census (line 164) — numbers UNCHANGED + zero-delta sentence

The census numbers (164 / 26 files / every per-file count) are UNCHANGED by Wave 5.2 — verify NO number edit is needed (the per-file annotations referencing Wave-5.1 remain historically accurate). APPEND ONE sentence at the very end of the line (after `... +6 the collation seam).`):
` Wave 5.2 added ZERO tests (the mechanism swap changed no container-testable surface — the stub path, the seam, and the consumers are identical; the census was re-pinned unchanged by the Wave-5.2 Task-4 verification).`

### 3.9 Technical Constraints (line 174)

Replace:
OLD: `- v1 sort order follows the system name order (the 2026-11-12 correction refining the alphanumeric rule for non-ASCII names; the retained pure fallback is the alphanumeric rule on system-collation failures/stub builds) — no settings UI.`
NEW: `- v1 sort order follows the system name order — the system file manager's exact order (\`StrCmpLogicalW\`, digit-aware natural ordering; the 2026-11-12 corrections refining the alphanumeric rule for non-ASCII names; the retained pure fallback is the alphanumeric rule in container/stub builds) — no settings UI.`

### 3.10 Verify-only tech.md rows

- Lines 105–106 (the Wave-4.2 alias/expiry mechanism bullets): their Wave-5.1 references are alias/expiry facts — still accurate. NO edit.
- Line 119 (Wave-5 verification), 152 (import audit), 174 (done above): no other mechanism residue. Sweep-check at §9: `CompareStringEx`, `GetUserDefaultLocaleName`, `NORM_IGNORECASE`, `Win32_Globalization`, `Win32_System_SystemServices`, `system-collation failures`, `on system failures` must be ZERO hits in tech.md after §3 (the only surviving mentions, if any, are inside §3.4/§3.7's own "were REMOVED/GONE" sentences — those are the Wave-5.2 record and are EXPECTED).

## §4 architecture.md edits (root: `.agent/project-info/architecture.md`)

### 4.1 Status paragraph (line 5) — append the Wave-5.2 sentence

The paragraph currently ends (verified tail) with:
`... the exe was rebuilt at \`dist/windows-x64/img-scroller.exe\` (19,636,215 bytes at 0.5.1), and the DLL import audit is unchanged (22).`

APPEND directly after it:
` Wave 5.2 (branch \`fix/wave-5-2-name-order\`, version 0.5.2, 2026-11-12) swapped the entry-name-order mechanism to the system file manager's \`StrCmpLogicalW\` comparator (the dedicated bullet below); dev checks ended \`ALL CHECKS PASSED\` (164 unit tests), the exe was rebuilt at \`dist/windows-x64/img-scroller.exe\` (19,605,828 bytes at 0.5.2), and the DLL import audit is unchanged (22).`

### 4.2 Navigation Engine bullet (line 32)

Replace:
OLD: `order: the SYSTEM entry-name order (Wave 5.1 — the \`EntryNameOrder\` seam through the platform facade; the dedicated bullet below); list edges stay silently)`
NEW: `order: the SYSTEM entry-name order (Wave 5.2 — the system file manager's \`StrCmpLogicalW\` comparator through the \`EntryNameOrder\` seam; the dedicated bullet below); list edges stay silently)`

### 4.3 System Entry-Name Order component bullet (line 33) — full rewrite

Replace the ENTIRE bullet:
OLD: `- **System Entry-Name Order — LANDED (Wave 5.1)**: BOTH navigation axes order entry names by the SYSTEM name order — the user-default locale's \`CompareStringEx\` with \`NORM_IGNORECASE\` through the cfg-gated \`platform\` facade (\`src/platform/windows_collation.rs\`; any system-call failure surfaces as \`None\`) — via the \`EntryNameOrder\` comparator-as-data seam (\`src/name_collation.rs\`: \`system()\` resolves the platform's \`compare_entry_names_system\` at listing-build time; \`with_comparator\` injects synthetic comparators in container tests); the pure alphanumeric fallback (lowercase form first, exact-character tie-break) is RETAINED for system-collation failures and container/stub builds (pure-ASCII names keep the identical order); BOTH listings consume the seam (the horizontal sort + \`first_listed_image\`; the vertical sort + the anchor position reading the SAME stored order), so the first-image and last-viewed rules are unchanged.`

NEW: `- **System Entry-Name Order — LANDED (Wave 5.1; mechanism Wave 5.2)**: BOTH navigation axes order entry names by the SYSTEM FILE MANAGER's exact order — Explorer-exact \`StrCmpLogicalW\` (shlwapi): digit-aware NATURAL/numeric ordering (\`(1)\`, \`(2)\`, …, \`(9)\`, \`(10)\`, …), case-insensitive, through the cfg-gated \`platform\` facade (\`src/platform/windows_collation.rs\` — a 17-line TOTAL comparator: two null-terminated UTF-16 buffers, the \`i32\` result mapped by \`i32::cmp(&0)\`; NO failure path — the Wave-5.1 \`Option\`/silent-fallback layer was REMOVED and \`windows.rs\`'s wrapper delegates directly) — via the UNCHANGED \`EntryNameOrder\` comparator-as-data seam (\`src/name_collation.rs\`: \`system()\` resolves the platform's \`compare_entry_names_system\` at listing-build time; \`with_comparator\` injects synthetic comparators in container tests); the pure alphanumeric fallback (lowercase form first, exact-character tie-break) remains the container/stub order ONLY (the stub delegates to it; on the Windows release target it is dead code — recorded as acceptable, cfg-gate cleanup deferred to a maintenance wave); BOTH listings consume the seam (the horizontal sort + \`first_listed_image\`; the vertical sort + the anchor position reading the SAME stored order), so the first-image and last-viewed rules are unchanged. Root cause (Wave 5.2): BOTH the original code-point order and the Wave-5.1 user-default-locale \`CompareStringEx\` collation sort digit runs LEXICALLY (\`(1)\`, \`(10)\`, \`(11)\`, …, \`(19)\`, \`(2)\`, …) — the operator's real \`智乃 (1).jpg … 智乃 (97).jpg\` folder proved the expectation is the file manager's natural order.`

### 4.4 folder_listing.rs module bullet (line 48)

Replace:
OLD: `EntryNameOrder system-order sort with the retained pure fallback (the Wave-5.1 collation seam), and ±1 neighbor resolution`
NEW: `EntryNameOrder system-order sort with the retained pure fallback (the Wave-5.1 seam; the Windows comparator is \`StrCmpLogicalW\` since Wave 5.2), and ±1 neighbor resolution`

Replace:
OLD: `(mechanism details: [tech.md](tech.md) → \`## Entry-Name System Order (Implemented, Wave 5.1)\`)`
NEW: `(mechanism details: [tech.md](tech.md) → \`## Entry-Name System Order (Implemented, Wave 5.1; Mechanism Wave 5.2)\`)`

### 4.5 name_collation.rs module bullet (line 51)

Replace:
OLD: `the entry-name ordering seam shared by the image listing and the sibling-folder listing (Wave 5.1):`
NEW: `the entry-name ordering seam shared by the image listing and the sibling-folder listing (Wave 5.1; the Windows comparator is \`StrCmpLogicalW\` since Wave 5.2):`

Replace:
OLD: `is the pure fallback used on any system-collation failure and in container/stub builds; 4 inline tests.`
NEW: `is the pure fallback used in container/stub builds (the Windows side resolves the system comparator — \`StrCmpLogicalW\` since Wave 5.2); 4 inline tests.`

### 4.6 platform/ module-layout bullet (line 68) — full rewrite

Replace the ENTIRE bullet:
OLD: `- \`src/platform/\` — OS-bound facade behind cfg-gating: \`mod.rs\` re-exports \`decode_image\`, \`show_open_image_dialog\` and \`compare_entry_names_system\`; \`windows.rs\` implements the native \`IFileOpenDialog\` and the WIC decode (mechanism: [tech.md](tech.md)); \`windows_collation.rs\` (Wave 5.1) implements the system entry-name order — the user-default-locale \`CompareStringEx\` with \`NORM_IGNORECASE\`, failures surfaced as \`None\` so \`windows.rs\` falls back to the pure collation; \`stub.rs\` is the non-Windows fallback (dialog → \`None\`, decode → \`Unloadable\`, the name order → the pure fallback) that keeps container checks/tests compiling.`

NEW: `- \`src/platform/\` — OS-bound facade behind cfg-gating: \`mod.rs\` re-exports \`decode_image\`, \`show_open_image_dialog\` and \`compare_entry_names_system\`; \`windows.rs\` implements the native \`IFileOpenDialog\` and the WIC decode (mechanism: [tech.md](tech.md)) and since Wave 5.2 its \`compare_entry_names_system\` wrapper delegates DIRECTLY to \`windows_collation.rs\` (no fallback call); \`windows_collation.rs\` (Wave 5.2) implements the system entry-name order — the system file manager's logical name comparison via \`StrCmpLogicalW\` (shlwapi): digit-aware natural ordering, case-insensitive, total (NO failure path); \`stub.rs\` is the non-Windows fallback (dialog → \`None\`, decode → \`Unloadable\`, the name order → the pure fallback) that keeps container checks/tests compiling.`

### 4.7 Horizontal critical path (line 86)

Replace: OLD `order: the system entry-name order (Wave 5.1); list edges stay silently)` → NEW `order: the system entry-name order (Wave 5.2 — the file manager's natural order); list edges stay silently)`

### 4.8 Project Paths version row (line 111)

Replace: OLD `Cargo.toml\` (package \`img-scroller\` 0.5.1, edition 2024)` → NEW `Cargo.toml\` (package \`img-scroller\` 0.5.2, edition 2024)`

### 4.9 Decisions-table Sorting row (line 130)

Replace:
OLD: `| Sorting: the system entry-name order (the 2026-11-12 correction refining the alphanumeric rule for non-ASCII names; the pure alphanumeric fallback retained on failures/stub builds) | landed (Wave 5.1) | \`## Sorting Order\` + the 2026-11-12 brief addendum |`
NEW: `| Sorting: the system entry-name order — the system file manager's exact order (the 2026-11-12 corrections refining the alphanumeric rule for non-ASCII names; the pure alphanumeric fallback retained as the stub-build order) | landed (Wave 5.1; mechanism Wave 5.2) | \`## Sorting Order\` + the 2026-11-12 brief addendum |`

### 4.10 Decisions-table entry-name-order mechanism row (line 138) — full rewrite

Replace the ENTIRE row:
OLD: `| Entry-name order mechanism: the platform-facade system collation (user-default-locale \`CompareStringEx\`, \`NORM_IGNORECASE\`) behind the \`EntryNameOrder\` comparator-as-data seam; BOTH listings consume it (the anchor position reads the stored order); the pure alphanumeric fallback retained on failures/stub builds; \`Win32_Globalization\` + \`Win32_System_SystemServices\` added | landed (Wave 5.1) | 2026-11-12 operator feedback (brief addendum item 8) |`
NEW: `| Entry-name order mechanism: the system file manager's \`StrCmpLogicalW\` (shlwapi — digit-aware natural ordering, case-insensitive; NO failure path) behind the \`EntryNameOrder\` comparator-as-data seam; BOTH listings consume it (the anchor position reads the stored order); the pure alphanumeric fallback retained as the stub-build order (dead code on the Windows release target — recorded note); the two Wave-5.1 \`windows\` features REMOVED (the original six remain) | landed (Wave 5.2; the Wave-5.1 \`CompareStringEx\` mechanism superseded) | 2026-11-12 operator feedback (brief addendum item 8) |`

## §5 context.md edits (root: `.agent/project-info/context.md`)

Order of application: §5.1 → §5.2 → §5.3 → §5.4 (the insert positions are relative to the CURRENT file; applying in this order keeps every anchor valid — §5.2 inserts at the TOP of Recent Changes, pushing existing entries down; all old-string matches below are unique and unaffected).

### 5.1 Current Work Focus (line 15) — full replacement

Replace the ENTIRE paragraph with:
`Dated 2026-11-12. **Wave 5.2 (name-order round 2) agent-side work is COMPLETE on branch \`fix/wave-5-2-name-order\` (version 0.5.2; NOT merged — the rename/merge/push are global-plan Step 5 mechanics after the Task-5 mark)**: the name-order DEFECT is ROOT-CAUSED and FIXED — the operator's real folder \`智乃 (1).jpg … 智乃 (97).jpg\` proved the expectation is the SYSTEM FILE MANAGER's exact order — Explorer-exact \`StrCmpLogicalW\` (shlwapi): digit-aware NATURAL/numeric ordering (\`(1)\`, \`(2)\`, …, \`(9)\`, \`(10)\`, …), case-insensitive — while BOTH the Wave-5.1 \`CompareStringEx\` user-default-locale mechanism and the original code-point order sort digit runs LEXICALLY (\`(1)\`, \`(10)\`, \`(11)\`, …, \`(19)\`, \`(2)\`, …), which the operator correctly reported as wrong. The mechanism was SWAPPED: \`src/platform/windows_collation.rs\` is now the 17-line \`StrCmpLogicalW\` comparator (NO failure path — the Wave-5.1 \`Option\`/silent-fallback layer removed; \`windows.rs\`'s wrapper delegates directly), the two Wave-5.1 windows features were REMOVED from \`Cargo.toml\` (the original six remain), and the \`EntryNameOrder\` seam + both listing consumers + the anchor position + container tests are UNCHANGED (census 164/26, zero test delta; the pure \`compare_entry_names\` fallback remains the stub/container path — dead code on the Windows release target, recorded as acceptable, cfg-gate cleanup deferred to a maintenance wave). Task-4 verification is GREEN (dev-checks \`ALL CHECKS PASSED\` — 164 tests, 0 failed, 26 files; the 0.5.2 exe rebuilt \`dist/windows-x64/img-scroller.exe\` = 19,605,828 bytes; DLL audit set-equal 22). Task 5 documentation landed (this step: README/tech/architecture/context updated to the 0.5.2 facts; brief.md verified — addendum item 8 already governs, NO edit; \`.agent/project-structure.md\` verified current — NO edit). REMAINING: Task-5 4.5b coherence check + 4.6 mark, then global-plan Step 5 (todo rename to \`-DONE\`, merge the branch into \`main\`, push \`main\` → \`origin\` ONLY). PENDING ACCEPTANCE GATE: the SINGLE 0.5.2 OPERATOR-RUN (Next-Steps #1 — it supersedes the still-open 0.5.1 checklist items in one run). Wave 6 = \`.agent/todos/20261029/20261029-todo-8.md\` next.`

### 5.2 Recent Changes — insert the Wave-5.2 landing entry as the NEW first entry

Insert immediately BELOW the `## Recent Changes` heading line (becoming the first `1.` entry, pushing all existing entries down):

`1. 2026-11-12: **Wave 5.2 Tasks 2–5 landed — the name-order mechanism swapped to the system file manager's \`StrCmpLogicalW\` (this step: Task 5 documentation).** Branch \`fix/wave-5-2-name-order\` (version 0.5.2). The SECOND 2026-11-12 operator report (0.5.1 exe run) opened this wave: Shift+'+' zoom-in PASS (the round-2 alias fix confirmed) and the popup auto-dismiss after ~2 s PASS (the wake fix confirmed), "all the rest OK" — but the navigation NAME ORDER was STILL WRONG with Japanese-mixed names (the Wave-5.1 fix did not take effect on real Windows). Task 2 root cause (caller-confirmed with the operator data; Tasks-2+3 plan §0): the operator's real folder \`智乃 (1).jpg … 智乃 (97).jpg\` must navigate in the SYSTEM FILE MANAGER's order — Explorer-exact \`StrCmpLogicalW\` (digit-aware natural ordering, case-insensitive, shlwapi) — while BOTH the Wave-5.1 \`CompareStringEx\` user-default-locale collation and the original code-point order sort digit runs LEXICALLY (\`(1)\`, \`(10)\`, …, \`(19)\`, \`(2)\`, …); verdict Suspect C (mechanism gap), pre-approved by brief addendum item 8; the Suspect-B wiring audit cleared every production path (\`EntryNameOrder::system()\` at \`folder_listing.rs:33\` and \`sibling_folder_listing.rs:45\`/\`:97\`). Task 3 fix (commit \`d188b0b\` + review-fix \`f3ddef2\`): \`windows_collation.rs\` rewritten to the 17-line \`StrCmpLogicalW\` comparator (null-terminated UTF-16 buffers; \`result.cmp(&0)\`; NO failure path — the Wave-5.1 \`Option\`/silent-fallback layer REMOVED; \`windows.rs\`'s \`compare_entry_names_system\` wrapper delegates directly); \`Cargo.toml\` lost the two Wave-5.1 features \`Win32_Globalization\` + \`Win32_System_SystemServices\` (the original six remain; \`Cargo.lock\` untouched); the seam, both listing consumers, the anchor position, and all container tests UNCHANGED. Task 4 verification GREEN: dev-checks \`ALL CHECKS PASSED\` (164 tests, 0 failed, 26 files — zero test delta; census re-pinned); the 0.5.2 exe rebuilt \`dist/windows-x64/img-scroller.exe\` = 19,605,828 bytes (the 0.5.1 artifact 19,636,215 B overwritten atomically); DLL audit SET-EQUAL 22 (\`StrCmpLogicalW\` = shlwapi.dll, already in the recorded list; the removed \`CompareStringEx\`/\`GetUserDefaultLocaleName\` were kernel32 exports and kernel32 stays). Task 5 documentation (this step): \`README.md\`, \`tech.md\`, \`architecture.md\`, \`context.md\` updated to the 0.5.2 facts (the 智乃 root cause, the \`StrCmpLogicalW\` mechanism, the six-feature Cargo.toml, the dead-code-on-Windows-release note, the census, the exe/audit lines, the pending 0.5.2 gate); \`brief.md\` verified — addendum item 8 already governs (verify-only, NO edit); \`.agent/project-structure.md\` verified current (already updated at the Tasks-2+3 landing). PENDING ACCEPTANCE GATE: the single 0.5.2 OPERATOR-RUN re-test — the natural-order re-test with the 智乃 folder on BOTH axes + the standing regressions — checklist verbatim at \`.kilo/plans/20261112-wave5-2-task4-verification-plan.md\` §11 (also Next-Steps #1); it SUPERSEDES the still-open 0.5.1 checklist items in one run. REMAINING agent-side: the Task-5 4.6 mark, then Step 5 (todo rename, merge, push).`

### 5.3 Surgical supersession notes on the historical Wave-5.1 entries (per the caller's flagged staleness)

5.3a Entry `1. 2026-11-12: **Wave 5.1 COMPLETE — ...` — replace BOTH fragments:
- OLD: `(3) the system entry-name order (\`CompareStringEx\` user-default-locale via the platform facade; \`EntryNameOrder\` seam both listings; pure fallback retained).`
  NEW: `(3) the system entry-name order (\`EntryNameOrder\` seam both listings; pure fallback retained — the Wave-5.1 \`CompareStringEx\` mechanism was SUPERSEDED in Wave 5.2 by the system file manager's \`StrCmpLogicalW\`; see the Wave-5.2 entry above).`
- OLD: `PENDING ACCEPTANCE GATE: the single 0.5.1 OPERATOR-RUN re-test on the 0.5.1 exe —`
  NEW: `PENDING ACCEPTANCE GATE (SUPERSEDED by the single 0.5.2 re-test — see the Wave-5.2 entry above): the single 0.5.1 OPERATOR-RUN re-test on the 0.5.1 exe —`

5.3b Entry `1. 2026-11-12: **Wave 5.1 Tasks 2–6 landed — ...` — replace ONE fragment:
- OLD: `(\`CompareStringEx\`, user-default locale via \`GetUserDefaultLocaleName\`, \`NORM_IGNORECASE\`; failures → \`None\` → the RETAINED pure fallback, also the container/stub order);`
  NEW: `(\`CompareStringEx\`, user-default locale via \`GetUserDefaultLocaleName\`, \`NORM_IGNORECASE\`; failures → \`None\` → the RETAINED pure fallback, also the container/stub order — this mechanism was SUPERSEDED in Wave 5.2 by \`StrCmpLogicalW\`; see the Wave-5.2 entry above);`

5.3c Current Project State, the Wave-5.1 bullet — replace BOTH fragments:
- OLD: `the system name order via the \`EntryNameOrder\` seam + \`src/platform/windows_collation.rs\` with the retained fallback); census 164 unit tests`
  NEW: `the system name order via the \`EntryNameOrder\` seam + \`src/platform/windows_collation.rs\` with the retained fallback — the Wave-5.1 \`CompareStringEx\` mechanism was SUPERSEDED in Wave 5.2 by \`StrCmpLogicalW\`); census 164 unit tests`
- OLD: `the pending 0.5.1 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1).`
  NEW: `the pending 0.5.1 OPERATOR-RUN was the wave's acceptance gate (SUPERSEDED by the single 0.5.2 re-test — Next-Steps #1).`

### 5.4 Current Project State — insert the Wave-5.2 bullet + refresh the TODO-state and Git bullets

5.4a Insert as the NEW FIRST bullet of `## Current Project State` (above the Wave-5.1 bullet):
`- **Wave 5.2 branch \`fix/wave-5-2-name-order\` (agent-side COMPLETE through Task 5 docs; NOT merged — Step 5 pending)**: version 0.5.2; the name-order mechanism swapped to the system file manager's \`StrCmpLogicalW\` (shlwapi; digit-aware natural ordering, case-insensitive; NO failure path) in \`src/platform/windows_collation.rs\` (17 lines) + the simplified \`windows.rs\` wrapper (direct delegation, no fallback); the two Wave-5.1 windows features REMOVED (the original six remain); the \`EntryNameOrder\` seam, both listing consumers, the anchor position, and the container tests UNCHANGED (census 164/26, zero test delta); the pure \`compare_entry_names\` fallback = the stub/container order only (dead code on the Windows release target — recorded note, cfg-gate cleanup deferred to a maintenance wave); dev-checks \`ALL CHECKS PASSED\` (164/0); exe rebuilt 19,605,828 bytes (0.5.2; the 0.5.1 artifact 19,636,215 B overwritten atomically); DLL audit set-equal 22; the pending 0.5.2 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1).`

5.4b TODO-state bullet — insert at the very START of the bullet (right after `- **TODO state**: `), before the existing Wave-5.1 sentence:
`\`.agent/todos/20261112/20261112-todo-2.md\` (Wave 5.2) — Tasks 2–4 marked \`[DONE]\`; Task 1's record sub-item is UNCHECKED (its context.md dated entry was never separately landed — the second-report facts are recorded in the Wave-5.2 Recent-Changes entry; flagged to the caller); Task 5's mark is the wave's 4.6 step; the Task-4 manual-UX (OPERATOR-RUN) sub-item STAYS \`[ ]\` (the user's 0.5.2 gate, Next-Steps #1). `

AND replace the Wave-5.1 sentence's gate clause:
- OLD: `the Task-5 manual-UX (OPERATOR-RUN) sub-item STAYS \`[ ]\` (the user's 0.5.1 gate, Next-Steps #1). \`.agent/todos/20261107/20261107-todo-1-DONE.md\``
  NEW: `the Task-5 manual-UX (OPERATOR-RUN) sub-item STAYS \`[ ]\` (SUPERSEDED — the single 0.5.2 re-test re-checks these as standing regressions, Next-Steps #1). \`.agent/todos/20261107/20261107-todo-1-DONE.md\``

5.4c Git bullet — replace the ENTIRE bullet:
OLD: `- Git: \`main\` == \`origin/main\` == \`6c498c4\`; the working tree is clean; no branches other than \`main\` remain (\`fix/wave-5-1-operator-feedback\` merged fast-forward \`b1435e7..6c498c4\` and deleted — after the earlier \`fix/wave-4-2-operator-feedback\` merged at \`094980f\` and \`feat/wave-5-pan-then-navigate\` merged \`094980f..9413f5a\` — all deleted after their verified merges, as was the earlier \`fix/wave-4-operator-fixes\`).`
NEW: `- Git: branch \`fix/wave-5-2-name-order\` is ACTIVE (HEAD \`8f50806\` at the Task-5 plan time; moving with the Task-5 commits); \`main\` == \`origin/main\` == \`6c498c4\` (the wave base \`a11c080\` is an ancestor of BOTH); Step 5 will fast-forward merge the branch into \`main\` and push \`main\` → \`origin\` ONLY (the earlier \`fix/wave-5-1-operator-feedback\` merged \`b1435e7..6c498c4\`, \`fix/wave-4-2-operator-feedback\` at \`094980f\`, and \`feat/wave-5-pan-then-navigate\` \`094980f..9413f5a\` — all deleted after their verified merges, as was the earlier \`fix/wave-4-operator-fixes\`).`

### 5.5 Immediate Next Steps — full rewrite (replace ALL THREE items, including the old fenced block and the Wave-5.1 regression note)

Item 1 (the 0.5.2 gate — the Task-4 plan §11 block copied VERBATIM):

`1. **Pending 0.5.2 Wave-5.2 OPERATOR-RUN gate — the SINGLE re-test, user run on a real Windows 10/11 machine**, running the freshly rebuilt 0.5.2 exe at \`dist\windows-x64\img-scroller.exe\`. The manual-UX checklist is preserved verbatim at [\`.kilo/plans/20261112-wave5-2-task4-verification-plan.md\`](../../.kilo/plans/20261112-wave5-2-task4-verification-plan.md) §11:`

Fenced block (verbatim from the Task-4 plan §11):
```
Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container): the name order with the operator's REAL Japanese-mixed folder matches the expected order on BOTH axes (images in the folder; sibling folders); plus the standing regressions (Shift+'+' in / Shift+'−' out / plain keys inert; popup expiry + close contract; pan/edge-navigate; ←/→/↑/↓ lists; silent edges; buttons; I/O/0/W/H).
```

`CONCRETE EXPECTATION for the name-order item (binding): the operator's real folder \`智乃 (1).jpg … 智乃 (97).jpg\` must navigate in NATURAL/numeric order — \`(1)\`, \`(2)\`, …, \`(9)\`, \`(10)\`, \`(11)\`, … — matching what the system file manager (Explorer) shows in the same folder; the LEXICAL order (\`(1)\`, \`(10)\`, \`(11)\`, …, \`(19)\`, \`(2)\`, …) is the defect signature (the pre-fix behavior and the Wave-5.1 behavior alike).`

`REGRESSION NOTE (binding, accompanies the verbatim block): every 0.4.2/0.5.0/0.5.1-established behavior stays expected-good on top of the mechanism swap — captions WHITE + thin black borders; \`I\`/\`O\`/\`0\`/\`W\`/\`H\` and buttons unchanged; Shift+'+' zooms in (the round-2 \`'*'\` alias), Shift+'−' out, plain keys inert; the popup auto-dismisses after ~2 s then all the close-contract behaviors; ESC exits fullscreen while ON; launch always maximized, console hidden; tooltips/hotkey captions; one-shot fit; symmetric silent zoom bounds; empty-folder mode; last-viewed memory; the Wave-5 pan/edge-navigate behaviors (zoom wide → keys pan, navigation only at edge; zoom tall → same vertically; small image → direct navigation — the PASSED Wave-5 gate). This SINGLE 0.5.2 run SUPERSEDES the still-open 0.5.1 checklist items (the name-order defect is the only open item; the alias + expiry items PASSED on 0.5.1 and are re-run as standing regressions in one run) — one run, one gate.`

Item 2:
`2. **Wave-5.2 global-plan Step 5 is PENDING**: rename \`.agent/todos/20261112/20261112-todo-2.md\` → \`-DONE\` (Tasks 2–4 already \[DONE\]; the Task-5 mark is the 4.6 step; the Task-4 manual-UX sub-item stays \`[ ]\`), tmp sweep, checkout \`main\`, fast-forward merge \`fix/wave-5-2-name-order\`, verify + delete the branch, push \`main\` → \`origin\` ONLY; notify on failure. Then ONE closing context.md touch for the merged/pushed state (Workflow Step 6 finish message).`

Item 3:
`3. **NEXT SESSION — Wave 6 (\`.agent/todos/20261029/20261029-todo-8.md\`, the "Open with..." menu)** runs in a NEW chat after Wave 5.2 closes at Step 5; the 0.5.2 gate (item 1) stays the user's responsibility and closes when the user reports PASS; the parked future-ideas list stays OUT of scope until scheduled.`

## §6 Verify-only files (NO edit in either)

- `brief.md`: READ the 2026-11-12 Addendum item 8 and CONFIRM it already governs the Wave-5.2 mechanism — item 8 says the order is "the order the Windows system (e.g. the file manager) shows for those names", which is exactly the file-manager mechanism the swap implements. NO edit. Report the verify-only result in the 4.2 report.
- `.agent/project-structure.md`: READ and CONFIRM it is already current for Wave 5.2 (verified at HEAD: the `platform/` line already records `StrCmpLogicalW`; the `name_collation.rs` line already records the container/stub-only fallback). NO edit.
- Hard bans: NO `CHANGELOG.md`; NO edits to any existing `.kilo/plans/*` file; NO `docs/build*.md` touches (the DLL list lives only in `scripts/build-windows.sh` + `docs/build-windows.md` and is UNCHANGED — set-equal 22); NO code-file touches; NO `Cargo.toml`/`Cargo.lock` touches.

## §7 Git plan — which commits this task creates (all on `fix/wave-5-2-name-order`; NO push — Step 5 territory)

| When | Commit | Content | Message (EXACT) |
| --- | --- | --- | --- |
| 4.2 start (after §8 step-1 pre-flight) | C1 | THIS plan file only | `docs: add wave5.2 task5 documentation plan` |
| 4.2 end (all §2–§5 edits applied + §9 checks green) | C2 | `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md` | `docs: wave5.2 task5 documentation` |
| 4.6 (the mark step — §10) | C3 | the TODO file only | `task: mark wave5.2 task5 done` |

- Gitignore compliance before EVERY commit: read `.gitignore`, run `git status --short`; NOTHING ignore-matching (`logs/`, `dist/`, `target/`, `*.log`) may be staged; stage ONLY the file list of the row (explicit paths; NO `git add -A`/`git add .`); verify the staged list equals the row's content immediately before committing.
- NO amend/force; NO merge; NO branch creation/switch; NO `git push` (restricted to Step 5: `main` → `origin` ONLY).

## §8 Implementation step order for the 4.2 docs-specialist

1. Host pre-flight: `git branch --show-current` → MUST be `fix/wave-5-2-name-order`; separate calls `git log --oneline -20` (HEAD `8f50806`, the §0 chain visible) and `git status --porcelain` (empty or ONLY this plan file before C1). Any mismatch → STOP + ask the caller.
2. Read `.gitignore` (sweep reference). Read §1–§6 so every edit is pre-armed.
3. Commit THIS plan file: stage `.kilo/plans/20261112-wave5-2-task5-documentation-plan.md`, verify the staged list, commit C1.
4. Apply the §2 README.md edits (2.1 → 2.4, in order).
5. Apply the §3 tech.md edits (3.1 → 3.9, in order; 3.10 verify-only).
6. Apply the §4 architecture.md edits (4.1 → 4.10, in order).
7. Apply the §5 context.md edits (5.1 → 5.5, in order — the order matters: §5.2 inserts at the TOP of Recent Changes before the §5.3 fragment replacements are anchored against the existing entries; all quoted old-strings remain unique regardless, but apply in the stated order).
8. Apply the §6 verify-only reads (brief.md, project-structure.md); record both results.
9. Residue sweep (host or built-in Grep tool): across `README.md`, `tech.md`, `architecture.md` — `CompareStringEx`, `GetUserDefaultLocaleName`, `NORM_IGNORECASE`, `Win32_Globalization`, `Win32_System_SystemServices` may appear ONLY inside the explicit Wave-5.2 supersession sentences listed in §3.4/§3.7/§4.10 (each such mention must sit inside a "REMOVED/GONE/SUPERSEDED" sentence); `system-collation failures` and `on system failures` must be ZERO hits; `0.5.1` in README/tech/architecture may appear ONLY in the historical Wave-5.1 narrative sentences and the supersession notes (the version rows and any "current state" phrasing must read 0.5.2). Any other hit → fix it only if it is one of the listed fragments; otherwise STOP + report.
10. Budget checks: every edited file stays a Markdown doc (no line-count rule applies to docs); confirm NO file outside the four was touched (`git status --short`).
11. Commit C2 with the §7 staged list. Verify the commit shows EXACTLY the four files (`git show --stat HEAD`).
12. Hand off to 4.3 (code-reviewer + code-simplifier, light) → 4.5b adherence → 4.6 mark (§10). The Critical Workflow's 4.4 docs-specialist step is N/A (this task IS the docs task).

## §9 Review checklist (4.3, light)

- `grep -n "StrCmpLogicalW" README.md .agent/project-info/tech.md .agent/project-info/architecture.md` → present in README:21/68 (§2.1/§2.4), tech.md §3.1/§3.4? (no — §3.4 mentions it once), §3.7c/§3.7e/§3.7g/§3.9, architecture.md §4.1/§4.3/§4.5/§4.6/§4.10 — no count pin beyond "present in every mechanism section"; assert by eye that every mention is the Wave-5.2 record.
- `grep -rn "CompareStringEx" .agent/project-info/tech.md .agent/project-info/architecture.md README.md .agent/project-info/context.md` → EVERY hit must be inside a Wave-5.2 supersession sentence (the §5.2/§5.3 records and §3.4/§3.7c/§4.10's "were REMOVED/GONE/SUPERSEDED" phrasing). No hit may present CompareStringEx as the CURRENT mechanism.
- `grep -rn "system-collation failures\|on system failures" .agent/project-info/tech.md .agent/project-info/architecture.md README.md` → ZERO hits.
- Version rows: `grep -n "0\.5\.2" README.md .agent/project-info/tech.md .agent/project-info/architecture.md` → README:52, tech.md:32, architecture.md:111 present; `grep -n "19,605,828" README.md .agent/project-info/tech.md .agent/project-info/architecture.md` → exactly one hit per file (README §2.4, tech §3.1, architecture §4.1).
- context.md: the Wave-5.2 entry is the FIRST Recent-Changes entry; Current Work Focus starts `Dated 2026-11-12. **Wave 5.2`; Next-Steps #1 carries the verbatim §11 block; the file's pre-existing entries (below the new ones) are byte-identical except the §5.3 supersession fragments.
- brief.md and project-structure.md show NO diff (`git diff --stat` on the C2 commit lists EXACTLY the four files).
- Rules: single-section boolean conditions N/A (docs); no commented-out code N/A; no `.gitignore`-matching files staged; markdown files were edited only by the Planner/Docs-specialist role (markdown-generation rule).

## §10 4.6 mark semantics (the Task-5 mark step; NOT executed inside 4.2)

When the Critical Workflow reaches 4.6 for Task 5 (AFTER §8 is fully green):

1. Read the TODO file [`.agent/todos/20261112/20261112-todo-2.md`](../../.agent/todos/20261112/20261112-todo-2.md) FIRST; PRESERVE all other content byte-for-byte (overwrite-prevention rule). EXACTLY these edits inside `### Task 5: Documentation updates`:
   - Heading: `### Task 5: Documentation updates` → `### Task 5: Documentation updates [DONE]`.
   - Sub-item 1 (the README/tech/architecture/context line): `- [ ]` → `- [x]` — box change ONLY; NO wording change.
   - Sub-item 2 (the brief.md verify-only line): `- [ ]` → `- [x]` — box change ONLY; NO wording change.
   - NO other heading, sub-item, section, or note is touched — in particular the UNCHECKED Task 1 heading and its sub-items are NOT this step's territory (flagged to the caller, §0).
2. Stage EXACTLY `.agent/todos/20261112/20261112-todo-2.md`; verify with `git status --short`; commit `task: mark wave5.2 task5 done`.
3. NO `git push`, NO `-DONE` rename (Step 5 territory). The Step-5 merge/push and the ONE closing context.md touch belong to the separate Step-5 implementer invocation.

## §11 Deliberately out of scope (what 4.2 and this task's mark must NOT do)

- NO code-file edits of any kind (`src/**`, `Cargo.toml`, `Cargo.lock`, `scripts/**`, `docs/build*.md`, `Dockerfile`, `docker-compose.yml`).
- NO CHANGELOG.md; NO edits to existing `.kilo/plans/*` files (including this plan once landed); NO todo-file edits beyond the §10 mark; NO `.agent/todos/**` renames.
- NO brief.md / project-structure.md edits (verify-only, §6).
- NO Task-1 execution or marking (the caller decides; §0 discrepancy note).
- NO version changes, NO new dependencies, NO harness runs (docs task — no VM/dev-checks step exists; the Task-4 green evidence is already recorded and is the citation source).
- NO x86 target; NO Wave-6/7 work; NO "Open with..."; NO parked future-ideas items.
- NO `git push`, NO branch operations, NO merge (Step 2/Step 5 territory). NEVER stage `dist/**`, `logs/**`, `target/**`, `*.log`, `Cargo.*`.

## §12 Stop conditions → return a question to the caller

- Pre-flight mismatch (branch ≠ `fix/wave-5-2-name-order`, HEAD ≠ `8f50806`, or an unexplained porcelain entry).
- Any quoted old-string from §2–§5 NOT found in its file (do NOT improvise a different anchor — STOP + report).
- Any residue-sweep hit that is not one of the listed expected supersession sentences.
- Any file diff beyond the four docs + this plan file at commit time.
- Anything ambiguous or outside this plan's assigned scope → return the question to the caller. NEVER make assumptions.
