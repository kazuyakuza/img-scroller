# Wave 6.2 — Task 3 (Cycle C) Verification Results (Critical Workflow step 4.2, implementer)

- Source plan: `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` (committed at `9c8db64`).
- Branch: `fix/wave-6-2-open-with-launch`. HEAD at verification start: `0378d6b`; plan commit `9c8db64`.
- Version: 0.6.2. Harness: Alpine VM `alpine-vm` MCP; `vm_status` called immediately before every VM command; exit codes authoritative.

## VERDICT: STOPPED — two pinned Step-5 sweep expectations missed (S3, S5); no fix attempted (STOP-only per plan §1 item 3)

All actionable verification work (Steps 0–4, 6) passed. Two READ-ONLY sweep pins in Step 5 did not match the plan's pinned expectation. Both deltas are pinned-expectation inaccuracies (no working-tree drift): S3 is a documentation-only module-doc-comment expansion landed by the wave's own `e8c3f0b` commit; S5 is the expected `Cargo.lock` version-line change landed by the wave's own `a048567` version bump, which the plan's "exactly 6 paths" pin omitted from the diff scope. Exact deltas are recorded below for the caller's decision.

---

## Step 1 — Fresh dev-checks at HEAD (TODO sub-item 1) — PASS

Command (exit code authoritative):

```text
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
```

Result: **exit code `0`**. Log: `logs/checks/dev-checks-20261115T000654Z.log`.

```text
==> fmt-check
PASS fmt-check exit=0 (1 s)
==> check
PASS check exit=0 (14 s)
==> test
PASS test exit=0 (16 s)
==> clippy
PASS clippy exit=0 (2 s)
ALL CHECKS PASSED
```

Cargo-test result line (captured from the same pinned test suite in the container):

```text
running 195 tests
test result: ok. 195 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s
```

Census corroboration (host):

`git grep -l "#\[test\]" -- src` → exactly **28** file lines:

```text
src/app/button_hint_tests.rs
src/app/esc_close_confirm_tests.rs
src/app/esc_close_expiry_tests.rs
src/app/fit_one_shot_tests.rs
src/app/floating_controls_tests.rs
src/app/folder_navigation_continuation_tests.rs
src/app/folder_navigation_tests.rs
src/app/fullscreen_tests.rs
src/app/last_viewed_tests.rs
src/app/open_with_button_tests.rs
src/app/pan_tests.rs
src/app/pill_style_tests.rs
src/app/tests.rs
src/app/zoom_alias_tests.rs
src/app/zoom_tests.rs
src/app_display.rs
src/folder_listing/collation_tests.rs
src/folder_listing/tests.rs
src/image_loading.rs
src/name_collation.rs
src/open_with_listing/tests.rs
src/pan/pan_tests.rs
src/scaling.rs
src/scaling/fit_factor_tests.rs
src/sibling_folder_listing/collation_tests.rs
src/sibling_folder_listing/neighbor_tests.rs
src/sibling_folder_listing/tests.rs
src/zoom.rs
```

`git grep -c "#\[test\]" -- src` → 28 file lines whose counts sum to exactly **195**:

```text
src/app/button_hint_tests.rs:2
src/app/esc_close_confirm_tests.rs:12
src/app/esc_close_expiry_tests.rs:4
src/app/fit_one_shot_tests.rs:12
src/app/floating_controls_tests.rs:1
src/app/folder_navigation_continuation_tests.rs:4
src/app/folder_navigation_tests.rs:5
src/app/fullscreen_tests.rs:9
src/app/last_viewed_tests.rs:5
src/app/open_with_button_tests.rs:16
src/app/pan_tests.rs:12
src/app/pill_style_tests.rs:5
src/app/tests.rs:9
src/app/zoom_alias_tests.rs:18
src/app/zoom_tests.rs:11
src/app_display.rs:2
src/folder_listing/collation_tests.rs:2
src/folder_listing/tests.rs:5
src/image_loading.rs:4
src/name_collation.rs:4
src/open_with_listing/tests.rs:12
src/pan/pan_tests.rs:11
src/scaling.rs:5
src/scaling/fit_factor_tests.rs:7
src/sibling_folder_listing/collation_tests.rs:2
src/sibling_folder_listing/neighbor_tests.rs:2
src/sibling_folder_listing/tests.rs:6
src/zoom.rs:8
```

(Sum of the recorded counts = 195 across 28 files.)

## Step 2 — Windows-target compile — PASS

```text
docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu
```

Result: **exit code `0`**. Only diagnostic: the pinned pre-existing benign dead-code warning:

```text
warning: function `compare_entry_names` is never used
 --> src/name_collation.rs:9:15
warning: `img-scroller` (bin "img-scroller") generated 1 warning
Finished `dev` profile [unoptimized + debuginfo] target(s) in 14.85s
```

## Step 3 — Release rebuild + ZERO-DIAGNOSTIC pin (TODO sub-item 2, part 1) — PASS

```text
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh
```

Result: **exit code `0`**; artifact overwritten atomically:

```text
build-windows: OK
build-windows: artifact: dist/windows-x64/img-scroller.exe
build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list: advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll
```

Exact artifact byte size:

```text
19649107 /img-scroller/dist/windows-x64/img-scroller.exe
```

**ZERO-DIAGNOSTIC pin:**

- **Pin A — PASS:** host grep of `scripts/` for `--features` → NO matches.
- **Pin B — PASS:** `Cargo.toml` lines 6–8 read exactly `[features]` / `default = []` / `diagnostic-launch-log = []`; the release build runs with the feature OFF.
- **Pin C — PASS:** container `grep -a -c 'img-scroller-diagnostic' dist/windows-x64/img-scroller.exe` → output `0`, **exit code `1`** (no lines matched — expected evidence; the string is compiled out).

## Step 4 — DLL import audit vs the recorded 22-DLL constant — PASS (set-equal)

```text
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | sed -n 's/^.*DLL Name: //p' | tr 'A-Z' 'a-z' | sort -u"
```

Exit code `0`. Observed lines (22):

```text
advapi32.dll
api-ms-win-core-path-l1-1-0.dll
api-ms-win-core-synch-l1-2-0.dll
api-ms-win-core-winrt-error-l1-1-0.dll
bcryptprimitives.dll
combase.dll
dwmapi.dll
gdi32.dll
imm32.dll
kernel32.dll
msvcrt.dll
ntdll.dll
ole32.dll
oleaut32.dll
opengl32.dll
rpcrt4.dll
shell32.dll
shlwapi.dll
user32.dll
userenv.dll
uxtheme.dll
ws2_32.dll
```

**Verdict: SET-EQUAL** to the recorded 22-DLL constant (`scripts/build-windows.sh:15` / `docs/build-windows.md`). No delta.

## Step 5 — Wave-pin sweep — S1 PASS / S2 PASS / S3 MISMATCH / S4 PASS / S5 MISMATCH / S6 PASS

- **S1 — PASS:** `git grep -n "shell_parseable_path" -- src` → exactly **12** matches: 3 in `src/open_with_listing.rs` (`:17`, `:25` doc-comment lines, definition at `:93`), 7 in `src/open_with_listing/tests.rs` (`:1` use line + 6 test assertions at `:80`, `:89`, `:98`, `:107`, `:116`, `:125`), 2 in `src/platform/windows_open_with.rs` (`:46` import, `:146` the single production call). The only production consumer is `windows_open_with.rs:146`, first statement of `file_data_object` before the wide encode / `SHCreateItemFromParsingName`.
- **S2 — PASS:** `git diff a048567..HEAD -- src/app.rs src/app/open_with_button.rs` → EMPTY.
- **S3 — MISMATCH (expected EMPTY):** `git diff 4185d97..HEAD -- src/platform/launch_log.rs src/platform/mod.rs Cargo.toml` is NOT empty. Only `src/platform/launch_log.rs` differs (module doc-comment block only, no executable code), landed by `e8c3f0b` (`docs: wave6.2 diagnostic module docs`). Exact diff:

```diff
diff --git a/src/platform/launch_log.rs b/src/platform/launch_log.rs
index a930270..9fa063f 100644
--- a/src/platform/launch_log.rs
+++ b/src/platform/launch_log.rs
@@ -1,12 +1,16 @@
 //! OPTIONAL Wave-6.2 Task-1 diagnostic (caller-authorized 2026-11-13): records
 //! the runtime evidence of each "Open with..." launch into a plain `.log` file
-//! so the operator can report it back. Compiled ONLY when BOTH the `windows`
-//! target AND the `diagnostic-launch-log` Cargo feature are enabled; the release
-//! build never enables the feature, so the shipped artifact contains none of
-//! this code. The log file is created lazily on the first diagnostic write and
-//! appended to on every later write; any write failure (or an unresolvable
-//! target) is silently skipped — this module must never alter app behavior or
-//! panic. Task 2's root-cause consumes the resulting log.
+//! (created next to the executable, falling back to the temp directory) so the
+//! operator can report it back. Compiled ONLY when BOTH the `windows` target
+//! AND the `diagnostic-launch-log` Cargo feature are enabled; the feature is
+//! OFF by default, the release build never enables it, and dev-checks run the
+//! default config, so the shipped artifact contains none of this code. The log
+//! file is created lazily on the first diagnostic write and appended to on
+//! every later write; any write failure (or an unresolvable target) is silently
+//! skipped — this module must never alter app behavior or panic. Task 2's
+//! root-cause consumes the resulting log. For future AI agents: the diagnostic's
+//! recorded fields and this optional-feature packaging are pinned in
+//! .kilo/plans/20261113-wave6-2-task1-diagnostic-build-implementation-plan.md.
 
 use std::fs::OpenOptions;
 use std::io::Write;
```

- **S4 — PASS:** `git diff a048567..HEAD -- Cargo.toml` shows ONLY the 4-line `[features]` block (`+[features]`, `+default = []`, `+diagnostic-launch-log = []`, and the blank separator line); `git diff a048567..HEAD -- Cargo.lock` is EMPTY.
- **S5 — MISMATCH (expected EXACTLY 6 paths):** `git diff b458897..HEAD --stat -- src Cargo.toml Cargo.lock scripts docs` shows **7** paths — the pinned 6 plus `Cargo.lock | 2 +-` (the `0.6.1` → `0.6.2` version line changed by the wave's own bump commit `a048567`, inside the `b458897..HEAD` range). Exact output:

```text
 Cargo.lock                        |   2 +-
 Cargo.toml                        |   6 +-
 src/open_with_listing.rs          |  31 ++++++-
 src/open_with_listing/tests.rs    |  57 ++++++++++++-
 src/platform/launch_log.rs        | 170 ++++++++++++++++++++++++++++++++++++++
 src/platform/mod.rs               |   3 +
 src/platform/windows_open_with.rs |  63 +++++++++++---
 7 files changed, 317 insertions(+), 15 deletions(-)
```

  (`scripts/` and `docs/` show NOTHING, as pinned. The only delta vs the pinned list is `Cargo.lock`.)
- **S6 — PASS:** host listing of `dist/windows-x64/` → exactly 3 files: `img-scroller.exe` (19,649,107 B, rebuilt), `img-scroller-diagnostic.exe` (225,180,316 B, untouched), `img-scroller-diagnostic.log` (2,768 B, untouched).

## Step 6 — Git segmentation — PASS

1. `git status` → clean working tree (after `9c8db64`).
2. `git log --oneline b458897..HEAD` → the 11-commit wave chain topped by the plan commit:

```text
9c8db64 docs: add wave6.2 task3 verification plan
0378d6b task: mark wave6.2 task2 done
1aac7e1 docs: wave6.2 launcher-fix module docs
2b46a2e fix: feed the open-with shell item a shell-parseable file path
b2b47a0 docs: add wave6.2 task2 launcher-fix implementation plan
e1cdfc3 task: mark wave6.2 task1 done
e8c3f0b docs: wave6.2 diagnostic module docs
4185d97 feat: add optional diagnostic launch log behind diagnostic-launch-log feature
02f639a docs: add wave6-2 task1 diagnostic-build implementation plan
a048567 chore: bump version to 0.6.2
c3611ae docs: add wave6.2 global plan
fd7acce task: record 0.6.1 operator re-test and create wave6.2 todo
```

3. `git log --oneline -1` → `9c8db64 docs: add wave6.2 task3 verification plan` (the Step 0 plan commit; pre-results state as pinned).
4. `git log --stat b458897..HEAD -- dist logs` → EMPTY output (no commit touched gitignored paths).

---

## Standing notes

- The release artifact `dist/windows-x64/img-scroller.exe` stays GITIGNORED and UNCOMMITTED (`dist/` never enters git).
- The diagnostic exe + log stay in `dist/windows-x64/` for a possible follow-up evidence run (the feature stays; already authorized); they are removed ONLY at the global plan's Step-5 tmp sweep.
- The manual-UX TODO sub-item STAYS `[ ]` — the §3 OPERATOR-RUN is the user's gate; Task 3's 4.6 marks sub-items 1+2 `[x]` only.

## §3 OPERATOR-RUN checklist (verbatim — Planner presents this in the chat at the wave close)

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
