# Wave 2 Task 4 — Verification Plan (test-coverage audit, harness CRLF repair, exe refresh, OPERATOR handoff)

Date: 2026-11-01
Source TODO: [`.agent/todos/20261029/20261029-todo-4.md`](../../.agent/todos/20261029/20261029-todo-4.md) → `### Task 4: Verification` (all three sub-items + "Future session execution notes").
Workflow slot: Wave-2 Critical Workflow, Task 4, sub-step 4.1b → executed by the implementer in 4.2.
Branch: `feat/wave-2-horizontal-scroll` (Tasks 1–3 landed; code commits `43b8c44`, `d1a85d5`, `4b45ca7`; HEAD `6378753`; fmt/check/test/clippy all exit 0 last verified at `f4317c4`; version `0.2.0`).
Status: APPROVED (the user auto-approves per-task plans; every decision below is binding — no judgment left).
Target implementer: JUNIOR under 50% restriction; all structural/scope decisions encoded.

## 0. Session facts captured at plan time (verified evidence)

- `git status --short` → exactly one entry: ` M .agent/project-info/context.md` (Planner's file; NEVER staged by the implementer). No untracked non-ignored files; `dist/`, `logs/`, `target/` are gitignored (`logs/` and `*.log` and `dist/` lines in `.gitignore`).
- `git ls-files --eol` (host-side, single command) → for BOTH harness scripts:
  - `scripts/dev-checks.sh`  : `i/lf` (index blob = LF)  `w/crlf` (working tree = CRLF)
  - `scripts/build-windows.sh`: `i/lf` (index blob = LF)  `w/crlf` (working tree = CRLF)
- **`.gitattributes` does NOT exist** (no repo-root file; `git ls-files --eol` shows an empty attr/ column for every tracked file).
- Test census (grep `#[test]` over `src/`): `folder_listing/tests.rs` 5 + `scaling.rs` 5 + `app/tests.rs` 9 + `app_display.rs` 2 + `image_loading.rs` 4 + `app/rails_tests.rs` 1 = **26 tests**. Line budgets: `folder_listing/tests.rs` 174/200, `app/tests.rs` 187/200, `app/rails_tests.rs` 40/200.
- Harness semantics (docs/build.md, full): fixed order fmt-check → check → test → clippy; every check runs even if earlier ones fail; per-check line `PASS|FAIL <name> exit=<code> (<n> s)`; final `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1); UTC-timestamped log appended under gitignored `logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` (path printed at run start; authoritative); `FORCE_FAIL` is a fault-injection hook (exactly one of `fmt-check|check|test|clippy`, synthetic exit 7) — NOT used in normal verification; script never modifies source files and never applies fmt fixes.
- build-windows semantics (docs/build-windows.md, full): runs `cargo build --release --locked --target x86_64-pc-windows-gnu` in the container; on success prints `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, then the OBSERVED-DLL static-linking note line; atomic `.part` → rename install; a failed run never deletes or modifies a previous artifact; every failure line starts with `build-windows: error:` and exits 1.

## 1. Test-coverage mapping (TODO Task 4 bullets → landed tests, name by name)

The TODO bullet list demands: (a) listing enumeration (folder-only scope), (b) alphanumeric sort correctness, (c) previous/next index stepping, (d) first/last edge behavior. Mapping onto the landed 26 tests:

| Bullet | Satisfied by (test) | File (lines) | Pinned evidence |
| --- | --- | --- | --- |
| (a) folder-only scope, non-recursive | `listing_excludes_directories_and_files_without_listed_extensions` | `src/folder_listing/tests.rs` 78–110 | Creates non-image files (`notes.txt`, extensionless, `.concealed.png`), **AND a real subfolder `sibling_subfolder` containing a nested image `inner.png` (lines 86–94), then asserts `listed_image_count() == 3`** with only `.concealed.png` / `LOWER.PNG` / `upper.jpg`. Answer to the audit question: **YES** — an existing test already demonstrates that nested-subfolder IMAGES are not listed even when a subfolder exists beside listed files. |
| (a) scope completion | `unlisted_and_missing_images_resolve_to_no_listing_index`; `empty_folder_yields_empty_listing_and_missing_folder_yields_none` | `src/folder_listing/tests.rs` 112–135; 166–174 | Index resolution for excluded/missing paths; empty existing folder → count 0; missing folder → `None`. |
| (a) app-level rebuild per folder | `navigation_on_the_empty_state_does_not_change_any_state`; `every_successful_open_rebuilds_the_listing_from_the_new_folder` | `src/app/tests.rs` 87–96; 163–187 | Empty state navigation inert; listing rebuilt from the new folder on each successful open (two distinct temp folders). |
| (b) alphanumeric sort incl. case handling | `listed_images_sort_case_insensitively_then_by_exact_name_tie_break` | `src/folder_listing/tests.rs` 35–76 | 7 files (`beta.jpg, Beta.png, ALPHA.jpg, Alpha.jpg, alpha.jpg, alpha.gif, zeta.webp`) pinned to indices: `alpha.gif`=0, `ALPHA.jpg`=1, `Alpha.jpg`=2, `alpha.jpg`=3, `beta.jpg`=4, `Beta.png`=5, `zeta.webp`=6 (case-insensitive primary, exact-name tie-break; decided Task-1 collation). |
| (c) previous/next index stepping | `neighbors_step_by_one_and_report_list_edges`; `navigation_resolves_the_previous_and_next_sibling_in_sort_order` | `src/folder_listing/tests.rs` 137–164; `src/app/tests.rs` 98–114 | Unit: index 1 steps to 0/2 with exact paths. App: from `b.png`, Previous → `a.png`, Next → `c.png` via the shared `NavigationDirection` seam. |
| (d) first/last edge behavior (decision: STAY, silent, no wrap) | `neighbors_step_by_one_and_report_list_edges` edges; `navigation_at_the_first_and_last_list_positions_stays_silent`; `arrow_button_enablement_follows_the_navigation_edges`; `failed_neighbor_open_keeps_the_current_image_and_shows_the_message` | `src/folder_listing/tests.rs` 157–163; `src/app/tests.rs` 116–144; `src/app/rails_tests.rs` 7–40; `src/app/tests.rs` 146–161 | Unit: `previous_neighbor(0)` → `ListBeginningReached`, `next_neighbor(2)` → `ListEndReached`. App: Previous at first → `None`, Next at last → `None`, single-image folder → both `None`, no message. Rails: ←/→ buttons disabled exactly at the resolved-list edges. Failure during step keeps the current image + message. |

Grandparent-traversal exclusion: structurally guaranteed by `FolderListing::from_opened_image` resolving only the opened image's parent directory (single `read_dir`; no ancestor walk — `src/folder_listing.rs` 31–42). Not unit-pinned with a parent-vs-grandparent fixture; recorded as **structurally covered — NOT a gap** (bullet (a) "folder-only scope" is already pinned by the nested-subfolder test above; adding an ancestor fixture test is FORBIDDEN here).

### 1.1 GAP VERDICT

**No gaps.** All four bullets are fully pinned by existing landed tests. **No test additions; final expected test count stays 26** (expect the harness test line `test result: ok. 26 passed; 0 failed; …`; a trailing doc-test result line `test result: ok. 0 passed` is normal and not a second count).

File-budget arithmetic (recomputed as instructed; no additions will occur): `folder_listing/tests.rs` headroom 26 lines, `app/tests.rs` headroom 13, `app/rails_tests.rs` headroom 160 — all within the 200-line rule as landed.

If at 4.2 the harness test line shows anything other than `26 passed` → **STOP** and return a user question (do not invent ad-hoc tests; this plan encodes the no-gap verdict as a design decision).

Commit (b) "any new tests" → **NOT APPLICABLE — skipped** (gaps verdict: none).

## 2. Known defect: harness CRLF → root-cause verdict + fix decision

### 2.1 Root-cause verdict (Windows checkout, NOT the VM share)

Evidence: `git ls-files --eol` shows `i/lf w/crlf` for both scripts — the git index/blob content is LF; the Windows-host working tree is CRLF. The `/img-scroller` vboxsf share mirrors the Windows working tree, so the container read CRLF lines and `sh` aborted at the shebang-adjacent `set` lines (`sh: set: Illegal option -`).

**Verdict: the defect was introduced by the Windows checkout (git autocrlf-style line-ending smudging at checkout time), not by the dev VM share.** The VM share faithfully mirrored the host tree it serves.

### 2.2 build-windows.sh has the SAME defect

`git ls-files --eol` → `scripts/build-windows.sh` is also `i/lf w/crlf`. Encoded consequence: **both** scripts are converted in the SAME conversion step (§4 OV.3), BEFORE any harness or build run; no second conversion later.

### 2.3 Fix mechanism — DECIDED

**Chosen: VM-side in-place conversion of the tracked working-tree files via the `alpine-vm` MCP**, using busybox `sed -i 's/\r$//'`, one file per command, each wrapped as `sh -c "…"` (the `alpine-vm` MCP allowlist requires commands to start with an allowed prefix: `docker, sh, apk, ls, cat, ps, df, free, uname, pwd, whoami`; raw `sed` starts with an unallowed prefix, hence the `sh -c` wrapper).

Rejected candidates, with reasons (recorded so the junior cannot re-litigate):
- Host-side `tr -d '\r' < file > file.tmp`: rejected — the host shell is PowerShell 5.1 (no `tr`), and the tool-selection rules forbid PowerShell-driven file edits and forbid chained `&&` single-liners.
- Container-side `sed -i` via `docker compose run … sed`: rejected — a container spin-up for a two-file edit is needless churn, and Wave-1 recorded vboxsf EBUSY on container writes through the share (authorized fallback pattern: the VM, as the share's server side, is the most direct write path).

The end state of either route is identical (LF working tree under a tracked path); the VM-side route is the one encoded.

### 2.4 Recurrence prevention — `.gitattributes` — DECIDED

**Yes, add it.** `.gitattributes` does NOT exist (verified); create it containing EXACTLY one line (LF, nothing after):

```
*.sh text eol=lf
```

Justification: the stored blobs are already LF (`i/lf`), so this attribute changes nothing in git history; it forces every future Windows checkout to materialize `.sh` working-tree files with LF regardless of the host's `core.autocrlf` — which is the verified recurrence vector (§2.1). Broader renormalization lines (`* text=auto`, `*.rs text eol=lf`, etc.) are **FORBIDDEN in this step** (out-of-scope churn: rustfmt/cargo tolerate the current `.rs` CRLF state — 26 tests + fmt/check/clippy all pass branch-wide — so no other defect class is observed or repaired today; any broad normalization belongs to a dedicated future plan).

### 2.5 Commit-content contingency (encoded in advance)

After the VM-side conversion, the Windows host's `git status --short` may or may not list the two scripts as modified (an LF-only working file round-trips clean against the LF index under autocrlf). Therefore §4 OV.8 stages EXACTLY what git actually lists among `{.gitattributes, scripts/dev-checks.sh, scripts/build-windows.sh}`. `.gitattributes` is a brand-new untracked file — it is always present, so commit (a) always has content acceptable as:
`fix(harness): normalize shell scripts to LF (repair container dev-checks)`
(when the scripts show no diff, the diff is `.gitattributes` alone — still truthful: it repairs/normalizes the harness plumbing and enforces LF going forward; the most-recent-verified shared-tree state after OV.3/OV.4 is what actually proves the runtime repair).

## 3. Exe refresh — decided procedure

- Preconditions (all binding): OV.7 harness green (`ALL CHECKS PASSED`, exit 0), commit (a) made, and `git log --oneline -5` showing the branch tip carries Tasks 1–3 code commits (`43b8c44`, `d1a85d5`, `4b45ca7` present). The exe is therefore rebuilt strictly AFTER all Wave-2 code landed — the cross-build consumes the checked-out working tree of `feat/wave-2-horizontal-scroll`, which is this branch.
- Command: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh`
- Expected stdout (in order): `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; `build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list: advapi32.dll, api-ms-win-core-path-l1-1-0.dll, …` (22-DLL list, unchanged); **exit code 0** (authoritative).
- Expected artifact: NON-EMPTY gitignored `dist/windows-x64/img-scroller.exe`. Size evidence: `sh -c "ls -l /img-scroller/dist/windows-x64/img-scroller.exe"` → record exact bytes (>0; compared against the Wave-1 baseline 19,428,795 bytes — Wave-2 added navigation/rails code, so a size change in either direction is acceptable; the recorded observed value is the deliverable).
- `dist/` stays gitignored: the post-build `git status --short` sweep (OV.12) must show NO `dist/`, `logs/`, `target/`, or `*.log` entries and nothing of them staged.

## 4. Order of operations for 4.2 (implementer)

All VM/container commands go through the `alpine-vm` MCP: call `vm_status` FIRST; the command EXIT CODE is authoritative; captured MCP output may lack cargo stderr (absent stderr is NOT a failure signal). One MCP call per step; multiple sub-commands inside ONE call must be a single quoted `sh -c "<command string>"`; never chain host-side commands with `&&` (one bash command per step). Host-side file edits use the dedicated write/edit tools, never PowerShell.

### OV.1 — VM gate
1. `vm_status` → must report running + SSH reachable. If unavailable → **STOP**, return a user question.

### OV.2 — Defect evidence (pre-fix; expect exit 1 = files differ = CRLF present)
1. `sh -c "sed 's/\r$//' /img-scroller/scripts/dev-checks.sh | cmp - /img-scroller/scripts/dev-checks.sh"` → expect cmp prints a `differ: byte…` style line and exit 1.
2. `sh -c "sed 's/\r$//' /img-scroller/scripts/build-windows.sh | cmp - /img-scroller/scripts/build-windows.sh"` → expect exit 1.
(The single quoted `sh -c` contains a `|` pipeline — that is ONE command string, allowed; do NOT add `&&`/`;` between steps.)

### OV.3 — Conversion (one command per file; exit 0 expected)
1. `sh -c "sed -i 's/\r$//' /img-scroller/scripts/dev-checks.sh"`
2. `sh -c "sed -i 's/\r$//' /img-scroller/scripts/build-windows.sh"`

### OV.4 — Post-fix proof (same two cmp commands from OV.2 → now SILENT + exit 0 = LF-only)
1. `sh -c "sed 's/\r$//' /img-scroller/scripts/dev-checks.sh | cmp - /img-scroller/scripts/dev-checks.sh"` → expect exit 0, no output.
2. `sh -c "sed 's/\r$//' /img-scroller/scripts/build-windows.sh | cmp - /img-scroller/scripts/build-windows.sh"` → expect exit 0, no output.

### OV.5 — Create `.gitattributes` (host-side, dedicated write tool)
Content EXACTLY (one line, LF):

```
*.sh text eol=lf
```

### OV.6 — Record host state: `git status --short`
Expected (autocrlf-dependent): `?? .gitattributes`, and either nothing more, or ` M scripts/dev-checks.sh` and ` M scripts/build-windows.sh`. **`.agent/project-info/context.md` may appear as ` M` — it is NEVER staged.**

### OV.7 — FULL harness run (the authoritative runtime proof):
`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
Expected (in order): header line `dev-checks starting; log file: logs/checks/dev-checks-<UTC>.log`; then `==> fmt-check` + `PASS fmt-check exit=0 (… s)`; `PASS check exit=0 (… s)`; `PASS test exit=0 (… s)` (test block shows `test result: ok. 26 passed; 0 failed`); `PASS clippy exit=0 (… s)`; final `ALL CHECKS PASSED`; **exit code 0**. NO `-e FORCE_FAIL=…` env may be set (plain run; the hook stays documentation-only this wave). Record the printed log path for the final report.

### OV.8 — Commit (a) (single commands, in order)
1. `git add .gitattributes`
2. `git add scripts/dev-checks.sh` — ONLY IF OV.6 listed it as modified (else skip silently)
3. `git add scripts/build-windows.sh` — same condition
4. `git diff --cached --name-only` — verify the staged list is a subset of exactly `{.gitattributes, scripts/dev-checks.sh, scripts/build-windows.sh}`; NOT `.agent/project-info/context.md`; NEVER `git add -A`/`git add .`
5. `git commit -m "fix(harness): normalize shell scripts to LF (repair container dev-checks)"`

### OV.9 — Commit (b): SKIPPED (no test gaps; §1.1). Do NOT create tests; do NOT commit "tests".

### OV.10 — Exe refresh (preconditions per §3; build-windows.sh was already converted in OV.3)
1. `git log --oneline -5` → confirm tasks-1–3 code commits present at the tip.
2. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` → expected per §3; exit 0.

### OV.11 — Artifact evidence
1. `sh -c "ls -l /img-scroller/dist/windows-x64/img-scroller.exe"` → record exact bytes (§3).

### OV.12 — Gitignore compliance sweep
1. Host: `git status --short` → NO `dist/`, `logs/`, `target/`, `*.log` entries; nothing besides the allowed tracked changes (` M .agent/project-info/context.md` remains unstaged). If any ignored artifact appears staged → unstage it (gitignore-compliance rule).

### OV.13 — Completion report to caller
Report: the VM `vm_status` result; the pre-fix cmp evidence (exit 1) and post-fix evidence (exit 0); the harness log path as printed; the four `PASS …` lines + `ALL CHECKS PASSED` + exit 0; the test line (`26 passed`); commit (a) hash and staged file list; `build-windows: OK` + artifact line + exit 0; the exe path `dist/windows-x64/img-scroller.exe` + exact recorded size; confirmation that `dist/`/`logs/` appeared in no staging.

## 5. OPERATOR-RUN handoff (verbatim insert for the 4.6/close user question)

Text to place in the user question at 4.6/wave close (executed on the real Windows 10/11 machine; agent-side GUI execution is impossible):

> **Wave-2 manual check (OPERATOR-RUN)** — run `dist\windows-x64\img-scroller.exe`:
> 1. Prepare a test folder (e.g. `C:\Users\<you>\Pictures\scroller-check`) containing EXACTLY: `alpha.gif`, `ALPHA.jpg`, `Alpha.jpg`, `alpha.jpg`, `beta.jpg`, `Beta.png`, `zeta.webp`, `notes.txt`, and a subfolder `sub` containing `inner.png`.
> 2. Start the exe; click `Open…`; open `beta.jpg`.
> 3. **Keys and buttons move prev/next**: press the LEFT arrow key AND click the on-screen bottom-rail `←` button — both must go to the PREVIOUS image; press the RIGHT arrow key AND click the bottom-rail `→` button — both must go to the NEXT image.
> 4. **Order matches the decided collation** (case-insensitive alphanumeric, exact-name tie-break): travel order must be `alpha.gif` → `ALPHA.jpg` → `Alpha.jpg` → `alpha.jpg` → `beta.jpg` → `Beta.png` → `zeta.webp` (and back).
> 5. **Nested subfolders are NOT entered**: `sub\inner.png` never appears while navigating; navigation stays strictly within the current folder.
> 6. Edge behavior: at the FIRST image `←` is inert and the bottom-rail `←` button shows disabled; at the LAST image `→`/`→`-button likewise (stay, silent, no wrap).
> 7. **Wave-2 note (approved layout K3 as landed)**: the message surface lives in the bottom rail and appears ONLY while an image is open — with no image open there are no rails and no message bar; the right rail's ↑/↓ folder arrows are visible but disabled (Wave-3 reserved).
> 8. Optional Wave-1 regression: resize the window → the image stays fit-scaled and centered.
> Report the results back into the project (Context or TODO); this closes the manual-UX bullet.

Result handling: operator results are recorded by the Planner at close; the manual-UX sub-item stays unchecked in the TODO until the user reports (Wave-1 precedent).

## 6. Hard exclusions (what 4.2 must NOT do)

- NO code-feature changes (no navigation, listing, rails, display, WIC, `platform/` edits).
- NO edits to `ListedNeighbor` edge/wrap semantics, no navigation-semantics experiments (the stay-silent behavior is decided and landed).
- NO docs edits (README/tech/architecture/product are Task 5 or the Planner's duty; no doc notes about `.gitattributes` either — docs touch = Task 5).
- NO `CHANGELOG.md` touches (wave binding). NO version bump (`0.2.0` landed at Step 3; `Cargo.toml`/`Cargo.lock` untouched — `--locked` guarantees no lockfile writes).
- NO TODO-file edits (4.6 marks `[DONE]`/`[]`→`[x]`, never 4.2). NO edits to pre-existing `.kilo/plans/*` files (including this plan file once it lands).
- NO `git push` (restricted to Step 5: `main` → `origin` ONLY) and NO branch creation/switch (restricted to Step 2).
- NO `FORCE_FAIL` invocations, NO `cargo fmt` fix applications, NO ad-hoc test invention (gap verdict: none), NO `git add -A`/`git add .`, `.agent/project-info/context.md` NEVER staged by the implementer.

## 7. Stop conditions → user question

- `vm_status` not running / `/img-scroller` share unavailable at any VM-gated step.
- Any `FAIL` line in the harness or a non-0 harness exit → return the FAIL lines + the printed log path; do not self-repair beyond executing this plan verbatim.
- The harness test line showing ≠ 26 passed (plan census mismatch).
- `sh: set: Illegal option -` recurring AFTER OV.4 proved LF-only (would indicate a defect class outside line endings, e.g. compose/sh resolution) → STOP + user question.
- `build-windows: error:` lines (per docs/build-windows.md table) → STOP + user question with the typed error line.

## 8. Final expected verification matrix

| Evidence | Expected |
| --- | --- |
| `vm_status` | running + SSH reachable |
| Pre-fix cmp ×2 | exit 1 (CRLF present) |
| `sed -i` ×2 | exit 0 |
| Post-fix cmp ×2 | exit 0, no output (LF-only) |
| Full harness | `PASS fmt-check/check/test/clippy` ×4 + `ALL CHECKS PASSED`, exit 0, log under `logs/checks/` |
| Test line | `test result: ok. 26 passed; 0 failed` (final count 26 — unchanged) |
| Commit (a) | `fix(harness): normalize shell scripts to LF (repair container dev-checks)`; staged ⊆ {`.gitattributes`, + scripts if git lists them}; context.md unstaged |
| Commit (b) | NOT APPLICABLE (no gaps) |
| Exe rebuild | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; exit 0 |
| Artifact | non-empty exe; exact size recorded (baseline 19,428,795 bytes) |
| `git status` sweep | no `dist/`, `logs/`, `target/`, `*.log` entries staged or listed as to-stage |
| Manual UX | OPERATOR-RUN handoff (§5) placed in the 4.6/close user question |
