# Wave 5 Task 4 — Verification Plan 4.1b (FIRST branch harness run — discovery semantics, census 150 corroboration, wave-pin sweeps, 0.5.0 release rebuild + byte-size record, DLL import audit set-equal 22, Git segmentation, 0.5.0 OPERATOR-RUN checklist)

- Date: 2026-11-11 (plan author: architector, workflow step 4.1b; executed by the implementer in 4.2).
- Source TODO: [`.agent/todos/20261029/20261029-todo-7.md`](../../.agent/todos/20261029/20261029-todo-7.md) → `### Task 4: Verification` ONLY (exactly its two sub-items) + the wave's `## Future session execution notes`. NO other task's plans are expanded or executed; the Task-4 `[DONE]` mark is the separate 4.6 step (§10; NOT executed inside 4.2 — NO todo edits in 4.2).
- NOT front-end ⇒ sub-step **4.1b ONLY** (no 4.1a exists for Task 4).
- Workflow slot: Wave-5 Critical Workflow (global plan [`.kilo/plans/20261107-wave42-then-wave5-global-plan.md`](20261107-wave42-then-wave5-global-plan.md) → B-Task 4, step 4.1b) — this PLAN → executed by the implementer in 4.2.
- Build docs (binding usage): [`docs/build.md`](../../docs/build.md), [`docs/build-windows.md`](../../docs/build-windows.md); harness scripts: [`scripts/dev-checks.sh`](../../scripts/dev-checks.sh), [`scripts/build-windows.sh`](../../scripts/build-windows.sh).
- Precedent followed (structure, command table, census rule, mark semantics, checklist block): [`.kilo/plans/20261107-wave4-2-task5-verification-plan.md`](20261107-wave4-2-task5-verification-plan.md) §0–§14 — same house shape, updated to the Wave-5 landed state and the FIRST-run-on-branch discovery semantics.
- **DISCOVERY semantics (binding, caller-provided): NO harness run has EVER happened on this branch.** The B-Group 4.3-fix compile blocker was already corrected at `8058be8`, but NOTHING on this branch has compiled yet — the FIRST §3 step-3 run IS the discovery step: fmt and clippy residuals are EXPECTED possibilities, and §7 sanctions their in-cycle minimal fixes (fmt round FIRST, then compile/clippy residuals). This differs from the Wave-4.2 precedent (where check-fail = immediate STOP) BY DESIGN — the caller pinned the compile-fix escalation rule.
- Approval: the user's global-plan approval makes every decision below BINDING; zero judgment left for the implementer. If any detail conflicts with something ACTUALLY observed during 4.2, STOP and return the question to the caller (§14).
- Target implementer: JUNIOR under 50% restriction. All structural/scope decisions are encoded here; §13 lists the out-of-scope guard verbatim.

---

## §0 Session facts captured at plan time (verified evidence; branch `feat/wave-5-pan-then-navigate` @ HEAD `04f5cbd`)

- Branch: `feat/wave-5-pan-then-navigate`; HEAD **`04f5cbd`** (`task: mark wave5 tasks 1-3 done`); working tree CLEAN at plan time (porcelain empty). The wave chain below the tip is: `4324999` (app.rs relocation-note docs) → `1bfd0cc` (pan-navigate module docs) → `8058be8` (review fixes: copy derives + edge-navigation test assertions — LAST code commit) → `272b98f` (feat: pan then navigate at image edges while zoomed) → `ca8b65f` (spec+plan docs) → `3e2e332` (chore: bump version to 0.5.0) → `094980f` (wave-4.2 todo rename). The implementer ENUMERATES it at pre-flight with host `git log --oneline -16` (§3 step 1) and pastes the lines into the 4.2 report — the plan deliberately pre-records NO further SHAs beyond `04f5cbd` (caller-provided, the only pinned one).
- **NO prior-green context exists (the reason this run is a discovery run): no dev-checks harness execution has EVER been run on this branch.** The FIRST §3 step-3 run is MANDATORY and is the branch's first-ever compile+test evidence.
- Version **0.5.0** (verified: `Cargo.toml` line 3 `version = "0.5.0"`; bumped at Step B.3, commit `3e2e332`). NOTHING touches `Cargo.toml`/`Cargo.lock` in this task (§2).
- Test census verified at plan time by host-side per-file `#[test]` count — **23 files, grand total 150**:

| File | `#[test]` count |
| --- | --- |
| `src/app/tests.rs` | 9 |
| `src/app/button_hint_tests.rs` | 2 |
| `src/app/esc_close_confirm_tests.rs` | 11 |
| `src/app/fit_one_shot_tests.rs` | 12 |
| `src/app/floating_controls_tests.rs` | 1 |
| `src/app/folder_navigation_continuation_tests.rs` | 4 |
| `src/app/folder_navigation_tests.rs` | 5 |
| `src/app/fullscreen_tests.rs` | 9 |
| `src/app/last_viewed_tests.rs` | 5 |
| `src/app/pan_tests.rs` (**Wave-5 B-Group — app-state pan controller**) | 12 |
| `src/app/pill_style_tests.rs` | 3 |
| `src/app/zoom_alias_tests.rs` | 14 |
| `src/app/zoom_tests.rs` | 11 |
| `src/app_display.rs` | 2 |
| `src/folder_listing/tests.rs` | 5 |
| `src/image_loading.rs` | 4 |
| `src/name_collation.rs` | 2 |
| `src/pan/pan_tests.rs` (**Wave-5 B-Group — pure pan math**) | 11 |
| `src/scaling.rs` | 5 |
| `src/scaling/fit_factor_tests.rs` | 7 |
| `src/sibling_folder_listing/neighbor_tests.rs` | 2 |
| `src/sibling_folder_listing/tests.rs` | 6 |
| `src/zoom.rs` | 8 |
| **Total** | **150** |

- No `#[ignore]` / `#[should_panic]` markers anywhere in `src/` (verified by grep at plan time) — nothing skipped, nothing deferred.
- LAST KNOWN EXE size (delta-only reference, never an expectation): 0.4.2 = **19,603,539 bytes**; `dist/windows-x64/img-scroller.exe` on disk currently holds that 0.4.2 artifact and will be ATOMICALLY overwritten (`.part` copy → rename) by the 0.5.0 build. The 0.5.0 exe size WILL differ; any non-empty size is acceptable — the verbatim as-built value is the deliverable. NO target size number is a valid pre-requirement.
- `OBSERVED_DLL_IMPORT_LIST` constant verified present at [`scripts/build-windows.sh`](../../scripts/build-windows.sh) line 15 (identical in [`docs/build-windows.md`](../../docs/build-windows.md) → "Static-linking / import audit"), 22 DLLs — the audit comparator (§6).
- Harness semantics (docs/build.md + `scripts/dev-checks.sh`): fixed order `fmt-check` → `check` → `test` → `clippy`; EVERY check runs even if earlier ones fail; per-check line `PASS|FAIL <name> exit=<code> (<n> s)`; final literal `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1); UTC-timestamped log appended under the gitignored `logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log`, the exact path printed at run start is authoritative; the script never modifies sources and never applies fmt fixes; `FORCE_FAIL` exists but is NOT used this task.
- build-windows semantics (docs/build-windows.md + script): runs `cargo build --release --locked --target x86_64-pc-windows-gnu` with cwd `/img-scroller` and the compose service's `CARGO_TARGET_DIR`; success prints `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, then the OBSERVED-DLL evidence line; install is ATOMIC (`.part` copy → rename over the final name; a failed run leaves the previous artifact byte-for-byte intact and never leaves a surviving `.part`); every failure line starts with the literal prefix `build-windows: error:` and exits 1.
- `.gitignore` contains (verified read) `*.log`, `logs/`, `dist/`, `target/` — artifacts and check logs are NEVER stageable.
- **Wave-5 pins verified at plan time (host reads at HEAD `04f5cbd`; they feed §3's sweep expectations — ALL line numbers below are as-verified NOW):**
  - Pan constants: `PAN_STEP_FRACTION: f32 = 0.125` → exactly `src/pan.rs:12`; `PAN_EDGE_TOLERANCE: f32 = 0.5` → exactly `src/pan.rs:14`.
  - Pure pan-math definitions in `src/pan.rs`: `axis_overflow` :17, `axis_pan_limit` :22, `clamped_axis_offset` :27, `axis_is_at_negative_edge` :33, `axis_is_at_positive_edge` :38, `axis_pan_step` :43, `axis_overflow_pair` :48, `clamped_pan_offset` :56, `arrow_outcome` :121 — nine definitions, all `pub(crate)`.
  - Arrow routing (keyboard_intake.rs): the 4 arrow reads `left/right/up/down_pressed` at :38–:41; the 4 `non_repeat_arrow_press_occurred` reads at :43/:46/:49/:52; the 4 `handle_arrow_key` dispatches at :79/:88/:97/:106 (each fed `screen_context.viewport_rect().size()` and a `pan::ArrowKeyPress` literal at :81/:90/:99/:108); `use super::pan;` at :19.
  - Controller seam (`src/app/pan.rs`): `non_repeat_arrow_press_occurred` :24, `arrow_pan_axis` :41, `arrow_pan_sign` :50, `reset_pan_offset` :64, `handle_arrow_key` :71, `arrow_pan_inputs` :86, `apply_arrow_pan_outcome` :105, `apply_pan_delta` :117 — eight definitions; the Navigate arm calls the shared seam VERBATIM: `pan::ArrowPanOutcome::Navigate => self.navigate(press.direction)` at exactly `:110` (Wave-3 rule untouched; navigation reuses `navigate` unchanged).
  - `viewport_rect().size()` intake: `src/app/keyboard_intake.rs` EXACTLY 6 code sites (:73 fit-width, :76 fit-height, :80/:89/:98/:107 the four arrows); the pre-existing Wave-4.1 BUTTON intake remains exactly `src/app/fit_button.rs:93` (1 site).
  - Resets: `self.pan_offset = egui::Vec2::ZERO;` — EXACTLY 2 code lines: `src/app.rs:95` (inside `accept_open_outcome` — EVERY successful open) and `src/app/pan.rs:65` (inside `reset_pan_offset`); `reset_pan_offset` appears as EXACTLY 2 lines: definition `src/app/pan.rs:64` + the ONE call `src/app/zoom.rs:74` (inside `restore_zoom_to_fit` — the `0` key AND the restore button both funnel there). The empty-state field init `pan_offset: egui::Vec2::ZERO,` sits at `src/app.rs:67` (a struct literal, NOT an assignment — outside the sweep pattern above).
  - Re-clamp (NO direct clamp call exists in the zoom actions BY DESIGN — the offset is re-clamped lazily ON USE): paint-side `src/app_display.rs` — `zoom::zoomed_draw_size` :58, `pan::axis_overflow_pair` :59, `pan::clamped_pan_offset` :60 (the paint clamps with the overflow of the CURRENT zoom-composed display size every frame); the `ImagePainting` field `pan_offset: egui::Vec2` at `app_display.rs:20`; the ONE `ImagePainting` literal gained `pan_offset: self.pan_offset,` at `src/app/central_surface.rs:40`; controller-side write-clamp `pan::clamped_axis_offset(raw + signed_delta, overflow)` at exactly `src/app/pan.rs:126` (the "last step lands exactly on the edge" clamp). Zoom steps do NOT reset the offset — pinned by the `zoom_steps_do_not_reset_the_pan_offset` test (`src/app/pan_tests.rs:62`).
  - `floating_controls.rs` untouched by the wave: `git diff 094980f..HEAD --stat -- src/app/floating_controls.rs` is EMPTY (verified at plan time — the wave's src/ diff touches exactly 12 files: `app.rs`, `app/central_surface.rs`, `app/folder_navigation.rs`, `app/keyboard_intake.rs`, `app/pan.rs` (new), `app/pan_tests.rs` (new), `app/zoom.rs`, `app_display.rs`, `main.rs`, `pan.rs` (new), `pan/pan_tests.rs` (new), `zoom.rs`; floating_controls.rs is NOT among them).
  - `message_text = ` write-sites — EXACTLY 3 (`src/app.rs:93` = `None;`, `src/app.rs:99` = `Some(image_loading::UNSUPPORTED_FILE_MESSAGE);`, `src/app/folder_navigation.rs:108` = `None;` — NONE contains "Zoom"/"Fit"; the folder_navigation site moved 107 → 108 this wave because of the `#[derive(Clone, Copy)]` insert at `:24`).
  - Launch/console/floor regressions: `windows_subsystem` → exactly `src/main.rs:4`; `with_maximized(true)` → exactly ONE code line `src/main.rs:27` (POST-RELOCATION — the window options moved out of `app.rs` into `main.rs`'s `window_options()` at `main.rs:22–30` this wave; ZERO app.rs hits expected); symmetric floor pin `ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM;` → exactly `src/zoom.rs:20`.
- Harness-execution deferral: NONE of §3's VM steps have been run yet; this plan is authored BEFORE any 4.2 execution (the caller forbade running the harness in this planning step).

## §1 Unit-coverage mapping (TODO Task 4 sub-item 1 → the B-Group's landed tests)

The TODO sub-item 1 ("Unit tests: overflow detection per axis, edge detection state machine (pan position vs image bounds), navigation trigger conditions — run via the dev-checks harness in the container per `docs/build.md`") maps onto the landed 150 as:

| TODO wave task | Satisfied by | File(s) |
| --- | --- | --- |
| B-Tasks 1+2+3 combined: per-axis overflow detection | `wide_image_overflows_only_horizontally` / `tall_image_overflows_only_vertically` / `fitting_image_has_zero_overflow_on_both_axes` | `src/pan/pan_tests.rs` |
| Edge-detection state machine (pan position vs image bounds) | the symmetric-clamp tests (`clamped_axis_offset_stays_inside_the_symmetric_bounds`, `clamped_axis_offset_collapses_to_zero_without_overflow`, `an_overshooting_step_is_absorbed_by_the_clamp`) + the edge tests (`arrow_outcome_absorbs_a_repeat_press_at_the_edge`, `arrow_outcome_navigates_on_a_distinct_press_at_the_edge`) | `src/pan/pan_tests.rs` |
| Navigation trigger conditions | `arrow_outcome_navigates_directly_when_the_axis_fits` (fits-entirely direct, repeat included), `arrow_outcome_pans_by_the_pinned_step_when_not_at_the_edge`, the app-routing table (`arrow_pans_the_overflowing_axis_while_not_at_the_edge`, `the_edge_landing_press_clamps_without_navigating`, `a_distinct_press_at_the_right_edge_navigates_to_the_next_image`, `a_repeat_press_at_an_edge_is_absorbed`, `a_distinct_press_at_the_bottom_edge_navigates_to_the_next_sibling_folder` — the vertical Wave-3 trigger, `a_fitting_image_arrow_navigates_directly`, `arrows_without_an_open_image_route_directly_to_navigation`) | `src/pan/pan_tests.rs` + `src/app/pan_tests.rs` |
| Reset policy (deferred-decision pickups: reset-on-open/restore) | `fresh_app_starts_with_zero_pan_offset`, `every_successful_open_resets_the_pan_offset`, `restore_zoom_to_fit_resets_the_pan_offset`, `one_shot_fit_sets_do_not_reset_the_pan_offset`, `zoom_steps_do_not_reset_the_pan_offset` | `src/app/pan_tests.rs` |
| Pinned decisions (pan step 0.125; edge tolerance 0.5) | `pan_constants_are_pinned` | `src/pan/pan_tests.rs` |
| Regressions (navigation, listing, collation, memory, open flow, unsupported message, fullscreen F11/ESC-exit/ESC-close, zoom, fit, pill styling) | the remaining 127 tests (150 − 23 Wave-5) unchanged from the 0.4.2 wave | all files above |

### §1.1 GAP VERDICT

**No gaps.** Every topic nameable from the B-Group (overflow per axis, edge state machine, navigation triggers, resets, pinned step) is pinned by landed tests. **NO test additions are sanctioned in this task; the expected census stays EXACTLY 150.** Any "why not one more test" improvisation in 4.2 → STOP (§14).

### §1.2 Census-failure rule

If the harness `cargo test` summary does not read exactly `test result: ok. 150 passed; 0 failed; ...` (a trailing doc-test `test result: ok. 0 passed` line is expected and normal — NOT a second count): **STOP** and return the full test section to the caller. A census mismatch is a defect, never an ad-hoc-patch opportunity.

## §2 Lockfile status — NO reconcile needed

Version 0.5.0 landed at Step B.3 with the committed lock; the B-Group introduced NO dependency changes. Therefore NO reconcile command, NO lock commit this task. Conditional STOP: if ANY `--locked` invocation (dev-checks steps, or the release build) fails with a "the lock file needs to be updated but --locked was passed"-class error — **STOP immediately** and return the error verbatim to the caller. A lock-drift failure here means a committed-state inconsistency the plan's evidence did not show — caller decision, NOT a junior self-fix.

## §3 Command sequence for 4.2 (exact; per docs/build.md and docs/build-windows.md)

Command channel: the `alpine-vm` MCP tools — call `alpine-vm_vm_status` BEFORE EVERY `alpine-vm_vm_run_command` call; ONE MCP call per row; each row's exact `command` string is passed verbatim; each command gets its own timeoutMs from the table; the `exitCode` field is authoritative; captured MCP stderr may be empty — an absent stderr is NOT a failure signal (documented caveat). Host-side git/grep commands run via the Bash tool ONE command per invocation (never chained, never `&&`; if the host shell rejects `grep`, use the built-in Grep tool per the tool-selection-priority rule).

| # | Step | Command (exact) | timeoutMs | Expected |
| --- | --- | --- | --- | --- |
| 0 | VM gate | `vm_status` (MCP tool) | — | running + SSH reachable; else STOP + user question (binding wave note) |
| P1 | Pre-flight LF guard ×2 | `sh -c "sed 's/\r$//' /img-scroller/scripts/dev-checks.sh \| cmp - /img-scroller/scripts/dev-checks.sh"`, then the IDENTICAL command for `/img-scroller/scripts/build-windows.sh` | 60000 | exit 0, no output, BOTH times. exit 1 (CRLF recurrence) → STOP + report (class outside Task-4 scope; caller owns the fix shape) |
| 1 | Pre-flight git evidence (host, Bash tool) | `git branch --show-current`, then SEPARATE calls: `git log --oneline -16`, then `git status --porcelain` | — | branch = `feat/wave-5-pan-then-navigate`; HEAD = `04f5cbd`; the wave chain visible below the tip (per §0); porcelain EMPTY or showing ONLY this task's permitted residuals (this plan file before C1, and/or a caller-owned modified `.agent/project-info/context.md`). Any OTHER entry → STOP + ask the caller. Paste the `git log` lines into the report (the wave-chain evidence table) |
| 2 | Pre-flight census STRUCTURE check (BEFORE running the harness — caller requirement; report any divergence BEFORE running) | host: `grep -rc "#\[test\]" src` (fallback: the built-in Grep tool) | — | per-file counts COMPLETELY EQUAL to the §0 23-file table; hand-summed grand total **150**. ANY divergence → STOP + report BEFORE any harness run (the caller-projection says 150 expected; the harness figure remains the binding one, but a pre-flight mismatch is reported FIRST, not silently proceeded past) |
| 3 | Pre-flight version check | host: `grep -n "^version" Cargo.toml` | — | exactly 1 line: `3:version = "0.5.0"` |
| C1 | Plan-file commit (wave task-level precedent `ca8b65f`) | host: `git add .kilo/plans/20261107-wave5-task4-verification-plan.md`; then `git commit -m "docs: add wave5 task4 verification plan"` | — | commit succeeds; `git status --porcelain` afterwards shows NO repo-owned file except (possibly) a caller-owned modified `.agent/project-info/context.md` |
| 4 | **FIRST dev-checks harness run at HEAD — the DISCOVERY run (sub-item 1's authoritative evidence; the branch's first-ever compile+test execution)** | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` | 900000 | four `PASS` lines in fixed order + final literal `ALL CHECKS PASSED`, **exit code 0**; harness test block `test result: ok. 150 passed; 0 failed; ...` (+ doc-test `0 passed` line, normal); RECORD the printed log path. On any FAIL → §7 branches (fmt round FIRST; then compile/clippy residual fixes with the one-more-attempt cap) |
| 5 | Census corroboration (AFTER a green or converged run) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rc --include='*.rs' '#\[test\]' /img-scroller/src \| sort"` | 60000 | per-file counts COMPLETELY EQUAL to the §0 23-file table (zero-count rows for the test-free files are normal); implementer sums the non-zero numbers by hand → grand total **150**; corroborates the harness `150 passed` line (the harness number is the binding figure) |
| 6 | Skip/ignore-marker sweep | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn --include='*.rs' -e '#\[ignore\]' -e '#\[should_panic\]' /img-scroller/src \| wc -l"` | 60000 | stdout exactly `0`, exit 0 — no skipped/ignored tests |
| 7a | Sweep: pan constant pins | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'PAN_STEP_FRACTION: f32 = 0.125\|PAN_EDGE_TOLERANCE: f32 = 0.5' /img-scroller/src/pan.rs"` | 60000 | EXACTLY 2 lines: `12` (0.125) and `14` (0.5) |
| 7b | Sweep: pure pan-math definitions | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -n 'pub(crate) fn axis_overflow\|pub(crate) fn axis_pan_limit\|pub(crate) fn clamped_axis_offset\|pub(crate) fn axis_is_at_negative_edge\|pub(crate) fn axis_is_at_positive_edge\|pub(crate) fn axis_pan_step\|pub(crate) fn axis_overflow_pair\|pub(crate) fn clamped_pan_offset\|pub(crate) fn arrow_outcome' /img-scroller/src/pan.rs"` | 60000 | EXACTLY 9 lines: `17`, `22`, `27`, `33`, `38`, `43`, `48`, `56`, `121` (assert the definition shapes by eye vs §0) |
| 7c | Sweep: the 4 arrows route through the pan controller (keyboard_intake.rs) | three greps: `grep -n 'handle_arrow_key' /img-scroller/src/app/keyboard_intake.rs` (EXACTLY 4 lines: `79`, `88`, `97`, `106`); `grep -n 'non_repeat_arrow_press_occurred' /img-scroller/src/app/keyboard_intake.rs` (EXACTLY 4 lines: `43`, `46`, `49`, `52`); `grep -n 'pan::ArrowKeyPress' /img-scroller/src/app/keyboard_intake.rs` (EXACTLY 4 lines: `81`, `90`, `99`, `108`) | 60000 each | every grep exact; NO other line for each pattern |
| 7d | Sweep: controller seam definitions + navigate reuse | two greps: `grep -n 'fn non_repeat_arrow_press_occurred\|fn arrow_pan_axis\|fn arrow_pan_sign\|fn reset_pan_offset\|fn handle_arrow_key\|fn arrow_pan_inputs\|fn apply_arrow_pan_outcome\|fn apply_pan_delta' /img-scroller/src/app/pan.rs` (EXACTLY 8 lines: `24`, `41`, `50`, `64`, `71`, `86`, `105`, `117`); `grep -n 'Navigate => self.navigate(press.direction)' /img-scroller/src/app/pan.rs` (EXACTLY 1 line: `110` — navigation reuses the shared seam VERBATIM) | 60000 each | every grep exact |
| 7e | Sweep: viewport_rect().size() intake | two greps: `grep -n 'viewport_rect().size()' /img-scroller/src/app/keyboard_intake.rs` (EXACTLY 6 lines: `73`, `76`, `80`, `89`, `98`, `107` — 2 fit + 4 arrows); `grep -n 'viewport_rect().size()' /img-scroller/src/app/fit_button.rs` (EXACTLY 1 line: `93` — the pre-existing button intake) | 60000 each | every grep exact |
| 7f | Sweep: open/restore resets | two greps: `grep -rn 'pan_offset = egui::Vec2::ZERO' /img-scroller/src` (EXACTLY 2 lines: `src/app.rs:95`, `src/app/pan.rs:65`); `grep -rn 'reset_pan_offset' /img-scroller/src` (EXACTLY 2 lines: definition `src/app/pan.rs:64`, the ONE call `src/app/zoom.rs:74` inside `restore_zoom_to_fit`) | 60000 each | every grep exact; assert the `app.rs:95` line sits inside `accept_open_outcome` (every successful open) by eye |
| 7g | Sweep: re-clamp composition (paint + controller write) | three greps: `grep -n 'zoomed_draw_size\|axis_overflow_pair\|clamped_pan_offset' /img-scroller/src/app_display.rs` (EXACTLY 3 lines: `58`, `59`, `60`); `grep -n 'pan_offset: self.pan_offset' /img-scroller/src/app/central_surface.rs` (EXACTLY 1 line: `40`); `grep -n 'clamped_axis_offset' /img-scroller/src/app/pan.rs` (EXACTLY 1 line: `126` — the write-clamp) | 60000 each | every grep exact; NOTE the design fact: zoom actions contain NO clamp call — the offset re-clamps lazily ON USE (paint every frame with the current zoom-composed overflow; next arrow write at `app/pan.rs:126`); zoom steps do NOT reset the offset (pinned by the census test) |
| 7h | Sweep: floating_controls untouched | host: `git diff 094980f..HEAD --stat -- src/app/floating_controls.rs` | — | EMPTY output (no diff line at all). ANY output → STOP + report verbatim |
| 7i | Sweep: message ground truth unchanged | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rn 'message_text = ' /img-scroller/src"` | 60000 | EXACTLY 3 lines: `src/app.rs:93` (`= None;`), `src/app.rs:99` (`= Some(image_loading::UNSUPPORTED_FILE_MESSAGE);`), `src/app/folder_navigation.rs:108` (`= None;`). Assert by eye: none of the 3 contains "Zoom" or "Fit" |
| 7j | Sweep: launch/console/floor regression pins | three greps: `grep -rn 'windows_subsystem' /img-scroller/src` (exactly 1 line `main.rs:4`); `grep -rn 'with_maximized' /img-scroller/src` (exactly 1 line `main.rs:27` — POST-RELOCATION; ZERO app.rs hits); `grep -n 'ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM' /img-scroller/src/zoom.rs` (exactly 1 line `zoom.rs:20`) | 60000 each | every grep exact |
| 8 | Release cross-build (0.5.0 exe) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` | 900000 | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; the OBSERVED-DLL evidence line; **exit code 0** |
| 9 | Exe inspection (record the byte size) | `sh -c "ls -l /img-scroller/dist/windows-x64/img-scroller.exe"` | 60000 | non-empty size printed; RECORD the exact as-built byte count verbatim (any non-empty size passes; no pre-requirement) |
| 9b | No dangling `.part` | `sh -c "ls /img-scroller/dist/windows-x64"` | 60000 | the listing shows EXACTLY one entry: `img-scroller.exe` — no `img-scroller.exe.part` (atomic install completed; the part file never survives success) |
| 10 | DLL import audit (per docs/build-windows.md) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe \| grep -i 'DLL Name' \| tr 'A-Z' 'a-z' \| sed 's/.*dll name: //' \| sort -u"` | 180000 | exactly 22 lowercase DLL lines, SET-EQUAL to the recorded constant (§6) |
| 10b | Audit fallback (ONLY if step 10 yields no output — report-format drift) | `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` | 180000 | raw dump; compare manually against §6; large output, still ONE call |
| 11 | Gitignore compliance sweep (host) | re-read `.gitignore`, then `git status --short` | — | NO `dist/`, `logs/`, `target/`, `*.log` entries anywhere in the output; nothing staged; the only allowed residual is a caller-owned modified `.agent/project-info/context.md` |

## §4 Expected outcomes table (authoritative)

| Evidence | Expected |
| --- | --- |
| Pre-flight (steps 1–3) | branch `feat/wave-5-pan-then-navigate`; HEAD `04f5cbd`; wave chain below the tip; census structure = §0 table, hand-sum 150, reported BEFORE the harness run; `Cargo.toml:3` = `0.5.0` |
| Harness log header | `dev-checks starting; log file: logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` — RECORD the exact printed path |
| Harness check lines (fixed order) | `PASS fmt-check exit=0 (<n> s)`; `PASS check exit=0 (<n> s)`; `PASS test exit=0 (<n> s)`; `PASS clippy exit=0 (<n> s)` (clippy runs `-D warnings`, so exit 0 means zero warnings) — after §7's sanctioned convergence (discovery run may fail first; the FINAL run must be all-PASS) |
| Harness test block | `test result: ok. 150 passed; 0 failed; ...` + doc-test `test result: ok. 0 passed` (normal, not a second count) |
| Harness final | literal `ALL CHECKS PASSED`; **exit code 0 = the authoritative success evidence** (sub-item 1's gate; the branch's FIRST-ever green evidence) |
| Census corroboration | per-file grep counts == §0 table; hand-summed total 150; zero `#[ignore]`/`#[should_panic]` |
| §3 sweeps & pins | 7a the 2 lines (`:12`, `:14`); 7b the 9 definition lines (17/22/27/33/38/43/48/56/121); 7c 4+4+4 lines exact; 7d 8 lines + the 1 navigate-reuse line (`:110`); 7e 6 lines (keyboard_intake) + 1 line (fit_button.rs:93); 7f 2+2 lines exact; 7g 3+1+1 lines exact; 7h EMPTY git diff; 7i exactly 3 message sites, none Zoom/Fit; 7j all three regression pins exact (`main.rs:4`, `main.rs:27` ONLY, `zoom.rs:20`) |
| Release build | `build-windows: OK`; `build-windows: artifact: dist/windows-x64/img-scroller.exe`; OBSERVED-DLL evidence line; **exit code 0** |
| Exe | NON-EMPTY gitignored `dist/windows-x64/img-scroller.exe` (0.5.0 wave build); exact as-built size RECORDED (delta vs the 19,603,539 B prior 0.4.2 magnitude is evidence only — any non-empty size passes); NO dangling `.part` (9b: the dist dir lists exactly `img-scroller.exe`) |
| DLL audit | 22 lowercase DLL names, SET-EQUAL to the recorded 2026-10-31 constant (§6) — DLL import list UNCHANGED |
| Git sweep | nothing ignored staged; nothing staged at all after C1 except the 4.6 §10 mark commit later |

## §5 Alpine-VM MCP usage notes (binding, per docs/build.md)

- `vm_status` BEFORE every `vm_run_command` call; only proceed when the VM is running and SSH-reachable; otherwise STOP and return a user question (docs/build.md precondition; wave execution note — harness unavailability = `vm_status` fail OR the `/img-scroller` share missing).
- ONE command per MCP call; pipelines live INSIDE one quoted `sh -c "..."` string exactly as printed in §3.
- The `exitCode` field is the authoritative pass/fail evidence; `null` exitCode + `timedOut: true` → report the timeout with the partial captured output; do NOT re-run silently.
- Absent cargo stderr in captured MCP output is NOT a failure signal (documented caveat).
- Generous timeouts (§3 table) beat aggressive ones.
- EBUSY-on-share-write hazard (vboxsf): retry the IDENTICAL command ONCE; if it fails again → STOP + report with evidence.

## §6 DLL import audit — recorded constant and comparison evidence statement

Recorded `OBSERVED_DLL_IMPORT_LIST` (source of truth: `scripts/build-windows.sh` line 15, identical in `docs/build-windows.md`; audited 2026-10-31 with `x86_64-w64-mingw32-objdump`; docs-split rule: NOT duplicated into tech.md/architecture.md):

```
advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll,
api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll,
gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll,
opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll,
ws2_32.dll
```

Evidence statement to record in the 4.2 report (verbatim shape): "Wave-5 audit: `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` inside the container; output deduplicated + lowercase-folded via the §3 step-10 pipeline; OBSERVED = 22 DLLs; SET-EQUAL to the recorded 2026-10-31 constant — DLL import list UNCHANGED."

On ANY diff (extra, missing, renamed — dependency drift): **STOP + report** the full observed list and the delta. NO edits to `scripts/build-windows.sh` or `docs/build-windows.md` — refreshing the OBSERVED baseline is a Planner decision, never an implementer improvisation (Wave-1/2/3/4/4.1/4.2 precedent).

## §7 Failure/intervention policy for 4.2 — DISCOVERY variant (decision branches; caps PINNED)

This branch has NEVER compiled: the discovery run is EXPECTED to possibly surface fmt/clippy residuals (and, less likely, a compile residual — the known B-Group 4.3 blocker was already fixed at `8058be8`, but nothing proves the branch compiles until this run). The sanctioned convergence sequence is:

- **fmt-check FAIL → fmt round FIRST (sanctioned self-service)**: ONE `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` (timeoutMs 120000) — the documented invocation of the formatter in the container; host-side `git diff` MUST show formatting-only reflow (no semantic content moves); if changed files exist → stage EXACTLY the `src/` paths listed by `git status --short` (never `.agent/`, never `docs/`) and commit EXACTLY `style: apply cargo fmt` (separate commit; Wave-2 precedent). Max 1 fmt round; a SECOND consecutive fmt-check FAIL → STOP + report (a deterministic formatter cannot legitimately fail twice).
- **check FAIL (compile residual) → sanctioned MINIMAL scoped correction**: fix the compile error(s) with the SMALLEST scoped corrections possible (the fewest lines, in exactly the files the compiler names; NO restructuring, NO renames beyond the error, NO drive-by edits); stage ONLY those files; commit EXACTLY `fix: wave5 verification compile fix`; then re-run the §3 step-4 harness. If the harness converges → continue. If check fails AGAIN → ONE more attempt is allowed (one more minimal fix + commit + re-run). If it CANNOT converge in that one more attempt → **STOP and return the verbatim full captured output + log path** (caller decision).
- **clippy FAIL → sanctioned MINIMAL scoped correction**: same shape as the compile branch — fix the named lints with the smallest possible scoped corrections (multi-site lints are fixed together ONLY when each site's fix is trivial and mechanical, e.g. the same lint in sibling lines; any fix requiring a design choice → STOP); commit EXACTLY `fix: wave5 verification clippy fix`; re-run the harness once per round. If it CANNOT converge in one more attempt after the first clippy fix round → **STOP and return the verbatim full captured output + log path**.
- **test FAIL → STOP immediately, NO self-fix.** Any failing named test = a structural/census matter; the §1.2 census rule protects against ad-hoc test invention; report with the failure detail lines and the printed log path. (A red test on this branch is a caller decision — the plan's evidence showed 23 landed Wave-5 tests + 127 regressions; a failure means committed-state divergence from that ground truth.)
- Global cap: at most 2 post-fix harness re-runs in total across ALL sanctioned fix rounds (the discovery run + 2 re-runs = 3 harness invocations max); anything failing after the cap → STOP + report with evidence (four result lines + log path).
- Any §3 sweep (2–3, 5–6, 7a–7j) mismatching its Expected → STOP + report the actual output verbatim. A drift means committed-state deviation from the plan's verified ground truth — caller decision, never a self-fix. (EXCEPTION: the census/structure check of step 2 is reported BEFORE the harness run per the caller requirement — a divergence there still means STOP.)
- Any `build-windows: error:` line of ANY type → STOP + report the typed error verbatim (the docs/build-windows.md failure table is the diagnostic map); never retry beyond the single EBUSY-once rule of §5.
- Every fix round re-reads `.gitignore` before staging (gitignore-compliance rule) and re-runs `git status --short`; `.agent/project-info/context.md` is planner/caller territory — NEVER staged.

## §8 Git plan — which commits this task creates

| When | Commit | Content | Message (EXACT) |
| --- | --- | --- | --- |
| 4.2 start (after §3 step-1 pre-flight) unconditional | C1 | this plan file only | `docs: add wave5 task4 verification plan` |
| Only if fmt drift | C2 | fmt-reflowed `src/` files only | `style: apply cargo fmt` |
| Only if compile residual | C3 | the minimal compile-fixed files only | `fix: wave5 verification compile fix` |
| Only if clippy residual | C4 | the minimal clippy-fixed files only | `fix: wave5 verification clippy fix` |
| **4.6 (the mark step — separate workflow sub-step, §10)** | C5 | the TODO file only (§10 edits) | `task: mark wave5 task4 done` |

- NO commit for build artifacts: `dist/` is gitignored; `logs/checks/` logs are gitignored; NOTHING built is ever staged (§3 step-11 sweep; unstage anything ignore-matching if it appears). `logs/` and `dist/` NEVER staged — if `git status --short` shows either, unstage with `git restore --staged <path>` and report.
- NO `git push` (restricted to workflow Step 5: `main` → `origin` ONLY). NO amend/force. NO `git add -A`/`git add .` (explicit paths only). NO branch creation/switch (Step-2 territory — the branch already exists). NO merge (Step-5 territory). NO `CHANGELOG.md` touches (wave standing ban).
- Commit staging lists are checked with `git status --short` immediately before each commit; the staged list MUST equal the row's content.

## §9 Junior-implementer step-by-step checklist for 4.2

1. Host: `git branch --show-current` → MUST be `feat/wave-5-pan-then-navigate`. Separate calls: `git log --oneline -16` → HEAD `04f5cbd`, the wave chain below the tip (paste into the report); `git status --porcelain` → empty or ONLY the permitted residuals (§3 step 1). Any mismatch → STOP + ask the caller.
2. Host: read `.gitignore` (sweep reference). Read §4 so every comparison is pre-armed. Run the §3 step-2 census STRUCTURE check (host grep; built-in Grep tool fallback) → the §0 table, hand-sum 150; report ANY divergence to the caller BEFORE running the harness (caller requirement). Run the §3 step-3 version check → `0.5.0`.
3. `vm_status` → running + SSH reachable; else STOP + user question.
4. §3 P1 LF guard (both scripts) → exit 0 twice; exit 1 → STOP + report.
5. §3 C1: stage the plan file, commit `docs: add wave5 task4 verification plan`; verify the staged list with `git status --short` before committing.
6. `vm_status`; §3 step 4 FIRST harness run (the DISCOVERY run) at 900000 ms. Compare against §4 line-by-line as output streams; RECORD the printed log path.
   - All four `PASS` lines + literal `ALL CHECKS PASSED` + exit 0 + `test result: ok. 150 passed; 0 failed` → go to 7.
   - `FAIL fmt-check` → §7 fmt branch (1 max) → ONE re-run → back to 6.
   - `FAIL check` → §7 compile branch (max 2 rounds, one-more-attempt rule) → re-run → back to 6.
   - `FAIL clippy` → §7 clippy branch (one-more-attempt rule) → re-run → back to 6.
   - `FAIL test` → STOP + report immediately (§7).
   - Post-cap failures (harness invocation count exceeds the discovery run + 2 re-runs) → STOP + report with the four result lines + log path.
7. `vm_status`; §3 steps 5–6 census + ignore sweep → the §0 table / hand-sum 150 / `0`; mismatch → STOP (§1.2).
8. `vm_status`; §3 steps 7a–7j wave-pin sweeps → EVERY row exact per §4; any mismatch → STOP + report actual output verbatim (§7). The 7h host git diff must be EMPTY.
9. `vm_status`; §3 step 8 release build → expect §4 build rows; any typed error → STOP + report verbatim.
10. `vm_status`; §3 steps 9 + 9b → RECORD the exact as-built byte size verbatim; dist dir must contain exactly `img-scroller.exe` (no `.part`).
11. `vm_status`; §3 step 10 DLL audit → 22 lowercase lines set-equal to §6; any diff → STOP + report (no script/doc edits); no output → §3 step 10b fallback then compare; record the §6 verbatim evidence statement.
12. Host: re-read `.gitignore`; `git status --short` sweep → NOTHING staged, no ignore-matching paths anywhere beyond the permitted caller residual; no unconditional commit remains pending (C1 already made; C2/C3/C4 only if their branches fired).
13. Report to caller (the full evidence set): branch/HEAD + the wave-chain `git log` lines; the pre-flight census result (150/23 files, reported before the run); harness log path; the four PASS lines + `ALL CHECKS PASSED` + exit 0 + the `150 passed; 0 failed` line of the FINAL run; the per-file census split + hand sum + zero-ignore result; every §3 sweep result (7a–7j) with the expected values reached; the discovery-run narrative (what failed, what was fixed, which commits landed — if any fix branch fired); `build-windows: OK`/artifact/exit 0; the as-built exe byte size (verbatim, with the 19,603,539 B 0.4.2 magnitude noted as reference); the §6 verbatim audit evidence statement; git-sweep result; **and the §11 OPERATOR-RUN checklist block copied VERBATIM for handoff to the caller** — plus the explicit note that the Task-4 manual-UX (OPERATOR-RUN) sub-item remains PENDING until the user reports on a real Windows machine.

## §10 4.6 mark semantics (the Task-4 mark step; NOT executed inside 4.2)

When the Critical Workflow reaches 4.6 for Task 4 (AFTER §9 is fully green):

1. Read the TODO file [`.agent/todos/20261029/20261029-todo-7.md`](../../.agent/todos/20261029/20261029-todo-7.md) FIRST; PRESERVE all other content byte-for-byte (overwrite-prevention rule). EXACTLY these edits inside `### Task 4: Verification`:
   - Heading: `### Task 4: Verification` → `### Task 4: Verification [DONE]`.
   - Sub-item 1 (the unit-tests/harness line): `- [ ]` → `- [x]` — box change ONLY; NO wording change (§4's FIRST-branch-green harness evidence makes this check legal).
   - Sub-item 2 (the manual-UX / OPERATOR-RUN line): **STAYS `- [ ]`** — pending the user's report on the real Windows machine (Wave-3/4/4.1/4.2 precedent: heading marked, agent-verifiable sub-item checked, manual-UX sub-item left unchecked until the operator reports). Do NOT annotate it with a status word; just leave the box.
   - No OTHER task heading, sub-item, section, or note is touched; the Future-session-execution-notes and Deferred-decisions sections are NOT edited; the sub-item wording is NEVER reworded (the checklist wording lives in THIS plan §11, referenced by the mark — the todo's own sub-item 2 text stays byte-identical).
2. Stage EXACTLY `.agent/todos/20261029/20261029-todo-7.md`; verify with `git status --short` that the staged list is only this file; commit `task: mark wave5 task4 done`.
3. NO `git push`, NO `-DONE` file rename (workflow Step 5 territory), `.agent/project-info/context.md` untouched by the implementer — the PLANNER copies §11 verbatim into `context.md` → "Immediate Next Steps" (Wave-4.1/4.2 precedent) so the operator pickup uses the same text.

## §11 Wave-5 manual check (OPERATOR-RUN) — verbatim block

(Verbatim from the caller's task source: the todo's `### Task 4: Verification` SECOND sub-item, copied byte-for-byte; handed by the 4.2 report to the CALLER for the user; run with the freshly rebuilt 0.5.0 exe at `dist\windows-x64\img-scroller.exe` on a real Windows 10/11 machine — agent-side GUI/runtime execution is impossible from the Linux container.)

```
Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side
GUI/runtime execution is impossible from the Linux container): zoom wide →
keys pan, navigation only at edge; zoom tall → same vertically; small image →
direct navigation.
```

REGRESSION NOTE (binding, accompanies the verbatim block): the 0.4.2-established checks remain in force as regressions — every behavior the 0.4.2 OPERATOR-RUN covered (captions WHITE + thin black borders; SHIFT+'+' in / SHIFT+'−' out with plain `+`/`=`/`-` inert; `I`/`O`/`0`/`W`/`H` and buttons unchanged; the ESC double-press close while fullscreen OFF; ESC exits fullscreen while ON; launch always maximized, console hidden; tooltips/hotkey captions; one-shot fit; symmetric silent zoom bounds; ←/→ image and ↑/↓ folder navigation; empty-folder mode; last-viewed memory) stays expected-good on top of the NEW Wave-5 pan-then-navigate behavior.

Result handling: recorded by the Planner at wave close; the Task-4 manual-UX (OPERATOR-RUN) sub-item STAYS `[ ]` in the TODO until the user reports (binding TODO structure; §10).

## §12 Divergence note — the wave's Step B.5 merge depends on this harness evidence

Step B.5 of the global plan (TODO-file rename → merge `feat/wave-5-pan-then-navigate` into `main` → push `main` to `origin`) may proceed ONLY after this task's 4.2 green evidence exists (§3 step 4 `ALL CHECKS PASSED` at HEAD + step 8 build exit 0 + step 10 audit set-equal). A STOP anywhere in §3/§7 therefore BLOCKS Step B.5 until the caller resolves it: the merge is never performed on red or missing evidence, and the implementer never performs the merge (Step-5 territory owned by a separate implementer invocation).

## §13 Deliberately out of scope (what 4.2 and this task's mark must NOT do)

- NO feature-code changes anywhere (no pan/zoom/fit/fullscreen/navigation/display/listing/platform edits beyond §7's sanctioned residual-fix rounds) — Task 4 verifies, it does not build.
- NO new tests, NO test additions/renames/skips "while here" (§1.1 gap verdict: none; census pinned at 150).
- NO TODO-file edits inside 4.2 (the Task-4 mark is the 4.6 step, §10; NO other todo file is touched either).
- NO docs updates — B-Task 5 owns README/tech/architecture/context; NO OBSERVED-DLL doc refresh here (a changed list is REPORTED, never recorded, per §6).
- NO `CHANGELOG.md` touches (wave standing ban). NO edits to existing `.kilo/plans/*` files, INCLUDING this plan once it lands (new plan files only, ever).
- NO version changes (0.5.0 landed at Step B.3). NO `Cargo.toml`/`Cargo.lock` touches at all.
- NO x86 target; NO Wave-6/7 work; NO "Open with..."; NO parked future-ideas items (mouse pan/drag is OUT OF SCOPE by the recorded Wave-5 decision — keyboard-only); NO `.agent/project-info/context.md` staging or editing (the PLANNER copies §11 there).
- NO `git push`, NO branch creation/switch, NO merge (Step 2/Step 5 territory). NO `FORCE_FAIL` invocations. NEVER stage `dist/**`, `logs/**`, `target/**`, `*.log`, or `Cargo.*`.

## §14 Stop conditions → return a user question

- `vm_status` not running / `/img-scroller` share unavailable at any VM-gated step (harness unavailability — binding wave note).
- §3 step-1 pre-flight evidence mismatch (branch ≠ `feat/wave-5-pan-then-navigate`, HEAD ≠ `04f5cbd`) or any unexplained porcelain entry.
- Pre-flight census STRUCTURE check ≠ the §0 23-file table / hand-sum ≠ 150 (reported BEFORE the harness run — caller requirement).
- Version check ≠ `0.5.0` at `Cargo.toml:3`.
- P1 LF guard exit 1 (CRLF recurrence — class outside this task).
- Any `--locked` lock-drift error (§2) — STOP; NO reconcile improvisation.
- Any harness `FAIL test` line (no self-fix); any harness FAIL persisting beyond §7's sanctioned rounds/caps (return the verbatim full captured output + log path).
- Harness summary ≠ `150 passed; 0 failed; ...` (§1.2); per-file grep census ≠ the §0 table or hand-sum ≠ 150; any `#[ignore]`/`#[should_panic]` hit.
- Any §3 sweep (7a–7j) mismatching its Expected (report actual output verbatim), including a NON-EMPTY 7h git diff.
- Any `build-windows: error:` typed line (report verbatim; the docs failure table is the diagnostic map).
- A dangling `img-scroller.exe.part` after a successful build (9b).
- DLL audit ≠ the 22-OBSERVED set (report the delta; never record a new baseline).
- Any §13 out-of-scope temptation → STOP; the caller decides.
- Anything ambiguous or outside this plan's assigned scope → return the question to the caller. NEVER make assumptions.
