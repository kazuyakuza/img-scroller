# Wave 4.1 — Task 9, Step 4.1b: Documentation Plan (Operator-Fix Wave Reference Update)

- Date: 2026-11-06
- Source TODO: [`.agent/todos/20261102/20261102-todo-1.md`](../../.agent/todos/20261102/20261102-todo-1.md) → `### Task 9: Documentation updates` ONLY (exactly its two sub-items). NO other task's plans are expanded or executed.
- NOT front-end ⇒ sub-step **4.1b ONLY** (no 4.1a exists for Task 9; no 4.5a).
- Workflow slot: Wave-4.1 Critical Workflow, Task 9, step 4.1b (this PLAN, created by the architector) → the doc-content edits are executed by the **docs-specialist** in the 4.2-equivalent step (markdown-generation rule), reviewed at 4.3, adherence-checked at 4.5b, and the `[DONE]` mark is the separate 4.6 step (§7). NOT executed inside 4.2: the mark; nothing else.
- Precedent followed (per-file BEFORE→AFTER anchors + budget rule): [`.kilo/plans/20261101-wave3-task5-documentation.md`](20261101-wave3-task5-documentation.md) (the Wave-3 Task-5 documentation plan — same shape, updated to the Wave-4.1 landed state).
- Approval: the caller's wave-plan approval makes every decision below BINDING; zero judgment left for the executor. If any BEFORE anchor in §3 is NOT found verbatim (files drifted since this plan's writing), STOP and return the question to the caller (§9).
- Target executor: junior under 50% restriction. All structural/scope decisions are encoded here; §8 lists the out-of-scope guard.
- Plan-code cross-references used throughout: the Git plan is §6, the 4.6 mark semantics are §7, the out-of-scope guard is §8, and the executor step order is §9.

---

## §0 Session facts captured at plan time (verified evidence at `fix/wave-4-operator-fixes` @ HEAD `37df6eb`)

- Branch: `fix/wave-4-operator-fixes`; HEAD `37df6eb` (verified via `git log --oneline -30`); working tree clean (`git status`).
- Version **0.4.1** (verified: `Cargo.toml` `[package] version = "0.4.1"` — landed at workflow Step 3, commit `53c2e60`). NOTHING touches `Cargo.toml`/`Cargo.lock` in this task.
- Test census **104 in 18 files** (verified at plan time by host-side `rg -c "#\[test\]" src`; identical to the Task-8 verification record §0 table):

  | File | `#[test]` count |
  | --- | --- |
  | `src/app/tests.rs` | 9 |
  | `src/app/floating_controls_tests.rs` | 1 |
  | `src/app/folder_navigation_tests.rs` | 5 |
  | `src/app/folder_navigation_continuation_tests.rs` | 4 |
  | `src/app/last_viewed_tests.rs` | 5 |
  | `src/app/fullscreen_tests.rs` (Wave-4 Task 1 + Wave-4.1-Task-3 ESC tests) | 9 |
  | `src/app/zoom_tests.rs` (Wave-4 Task 2 + Wave-4.1 Tasks 4/5/6) | 16 |
  | `src/app/fit_one_shot_tests.rs` (Task 7 app policy) | 12 |
  | `src/app/button_hint.rs` (Task 2 inline caption/tooltip pins) | 2 |
  | `src/folder_listing/tests.rs` | 5 |
  | `src/sibling_folder_listing/tests.rs` | 6 |
  | `src/sibling_folder_listing/neighbor_tests.rs` | 2 |
  | `src/name_collation.rs` | 2 |
  | `src/image_loading.rs` | 4 |
  | `src/app_display.rs` | 2 |
  | `src/zoom.rs` (pure zoom math; symmetric-bounds pins) | 8 |
  | `src/scaling.rs` (pure fit math, inline) | 5 |
  | `src/scaling/fit_factor_tests.rs` (Task 7 fit-factor math) | 7 |
  | **Total** | **104** |

- Exe artifact verified present: `dist/windows-x64/img-scroller.exe` = **19,593,177 bytes** (host file listing dated 2026-11-06; the Task-8 0.4.1 rebuild). DLL import list SET-EQUAL 22 (unchanged; the recorded constant lives ONLY in `scripts/build-windows.sh` + `docs/build-windows.md`).
- Wave-4.1 commit chain (verified `git log --oneline --reverse main..HEAD`; the branch sits on `main` == `19aa9a6`):
  `2fd26c8` (todo) → `7df1ef3` (global plan) → `53c2e60` (bump 0.4.1) → Task 1: `e1092f8` (spec+plan), `e3873f8` (`feat: window opens maximized and release console hidden`), `93eb47d` (module docs), `4052c5a` (verification+adherence), `86a1c2c` (mark) → Task 2: `fa8ec7e` (spec+plan), `4a2e7f0` (`feat: button hotkey references and hover tooltips`), `95247dc` (review fix + whitespace), `19ff32c`/`020c22f` (context logs), `4833576` (review+verification+adherence), `892efc1` (mark) → Task 3: `02e0870` (spec+plan), `9c46fa3` (`feat: escape exits fullscreen and f11 note`), `db83dd4` (module docs), `c135ede` (verification+log), `9c0625a` (mark) → Task 4: `f2aa371` (plan), `943d868` (`fix: remove zoom bound messages`), `72a0de7` (adherence), `f543da9` (mark) → Task 5: `761ac50` (plan), `d4c6870` (`fix: allow zoom out below fit with symmetric floor`), `990006c` (review fix), `2483f76` (simplification plan), `c19ff78` (adherence), `daf0e2f` (mark) → Task 6: `fc2a6f4` (plan), `e6ddb7b` (`fix: require shift for zoom plus minus aliases`), `92b86f7` (adherence), `aa2175b` (mark) → Task 7: `8e51cbf` (spec+plan), `ae4b0f3` (`feat: fit keys and buttons set zoom one shot`), `4e44674` (review fix), `2aecf5a` (review-fix plan), `5c7a067` (module docs), `f45c7ea` (adherence), `5579cfb` (mark) → Task 8: `3f74a3f` (verification plan), `37df6eb` (mark; the Task-8 OPERATOR-RUN sub-item stays `[ ]`).
- Landed-code ground truth (verified at plan time by `rg` over `src/`):
  - `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` — exactly `src/main.rs:4` (console hidden in RELEASE builds only; a dev `cargo run` console is expected, NOT a defect).
  - `.with_maximized(true),` — exactly `src/app.rs:146` (the window opens ALWAYS MAXIMIZED at every launch; fullscreen default stays OFF).
  - `ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM` — exactly `src/zoom.rs:20` (the symmetric ÷8–×8 range).
  - `FULLSCREEN_EXIT_KEY = egui::Key::Escape` + `escape_exits_fullscreen` — `src/app/fullscreen.rs:16`/`:20`, wired in `src/app/keyboard_intake.rs:23–25`.
  - `shift_alias_is_pressed` — 4 occurrences, all in `src/app/zoom.rs` (definition 139; uses 151/154/174) — the strict shift-only alias chord (Shift+'+'/Shift+'=' in; Shift+'−' out; plain +/=/− inert).
  - ZERO occurrences of `Zoom max`/`Zoom min` and of the fit-activation messages in `src/` — the only remaining app message is the unsupported-file failure text. `Fit width`/`Fit height` survive ONLY as UI labels/tooltips (`button_hint.rs:83–84` = `"Fit width (W)"`/`"Fit height (H)"`; `fit_button.rs:9–10` = `"Fit width"`/`"Fit height"`).
  - One-shot seams: `set_zoom_to_fit_width`/`set_zoom_to_fit_height` in `src/app/zoom.rs:77`/`:92`; funneled from `keyboard_intake.rs:52`/`:55` (W/H keys) and `fit_button.rs:92–93` (buttons); the intake area is `viewport_rect().size()` (egui 0.37 removed `screen_rect`; ZERO `screen_rect` occurrences).
  - Dimming predicates `zoom_can_grow`/`zoom_can_shrink`/`zoomed_off_fit` — `src/app/zoom.rs:107/:114/:121`, consumed by `zoom_button.rs:47–49`.
  - Renames/removals landed: `src/app/fit_mode_button.rs` → `src/app/fit_button.rs`; `src/app/fit_mode_tests.rs` REMOVED → `src/app/fit_one_shot_tests.rs`; `src/scaling/fit_mode_tests.rs` REMOVED → `src/scaling/fit_factor_tests.rs`. ZERO occurrences of `fit_mode`/`FitMode` anywhere in `src/`.
  - `ImagePainting` is the 2-field `{ decoded_image, zoom_factor }` (`src/app_display.rs:15–18`); paint recomposes `fitted_size(available) × zoomed_draw_size` per frame, so a window resize after a one-shot set does NOT re-fit.
  - Pill tooltips (all 10, `src/app/button_hint.rs:75–85`): `Previous image (Left Arrow)` / `Next image (Right Arrow)` / `Previous sibling folder (Up Arrow)` / `Next sibling folder (Down Arrow)` / `Fullscreen (F11)` / `Zoom in (I)` / `Zoom out (O)` / `Restore fit (0)` / `Fit width (W)` / `Fit height (H)`. Captions under the SIX keyboard-distinct buttons: `F11` / `I` / `O` / `0` / `W` / `H` (the four arrows carry NO caption).
- Doc-state ground truth: all seven target docs read in full at 4.1b; their current wave markers read 0.4.0 / 93 tests / 19,535,645 bytes / the FitMode model / `fit_mode_button` / "F11 is the only exit" — every such stale marker has an edit assigned in §3.
- The **Wave-4.1 OPERATOR-RUN checklist** source of truth: [`.kilo/plans/20261102-wave-4-1-task8-verification-plan.md`](20261102-wave-4-1-task8-verification-plan.md) §11 (15 items; imported byte-for-byte into context.md per §4/§5).

## §1 Verified NO-CHANGE files (do NOT open/edit; any urge to touch must STOP)

- `CHANGELOG.md` — wave standing ban; entries begin at MVP/v1.0.0 only.
- `docs/build.md`, `docs/build-windows.md`, `scripts/*` — no version/test/size/status content there; the 22-DLL OBSERVED constant stays only where recorded.
- `src/**`, `Cargo.toml`, `Cargo.lock`, `LICENSE`, `.gitignore`, `.gitattributes` — Task 9 is documentation-only.
- ALL existing `.kilo/plans/*` files (never edited; this plan file is brand-new and, once committed, likewise frozen).
- `.agent/project-info/future-ideas.md`, `.agent/project-info/instructions.md`, other TODO files (the Task-9 mark edits of §7 are the ONLY TODO-file touch, made at 4.6).
- `.kilo/rules/*`, `.kilo/commands/*`, `.opencode/*`.

## §2 Budget rule

Every change is a small local rewrite anchored by exact FROM/TO strings; the three rewrites marked as WHOLE-REWRITES (tech.md §7 body, context.md work-focus paragraph, context.md wave-state bullet) replace exactly their bounded regions. NO section reordering, NO restructuring, NO new sections except: the tech.md §7 body is rewritten in place (heading KEPT, no Contents/anchor churn), and the brief/product addenda are the only inserted sections. Rationale comments in this plan are for the executor's understanding only — never copy them into the docs.

## §3 Per-file edit plan (anchors verbatim from the current files)

### 3.1 `README.md` (5 edits)

**R-1** · Feature 4 bullet — WHOLE-REWRITE (this removes the stale always-placement wording):
- FROM (the whole bullet): `4. **Fullscreen Mode** — default OFF at launch: the window opens normal or maximized exactly at the user's own placement/size (the feature never changes the launch behavior). When OFF, the floating bottom pill and all other UI controls are visible and the image takes almost all of the central space. Toggling shares ONE action between the F11 keyboard shortcut and the pill's painted fullscreen-square button; the first press turns fullscreen ON — the window header/borders and ALL UI controls (the pill included) disappear and only the image shows, while the arrow keys keep navigating images/folders; F11 is the ONLY exit while ON (the pill is not rendered then). Fullscreen toggling never touches the zoom or fit state.`
- TO: `4. **Fullscreen Mode** — default OFF at launch: the window opens ALWAYS MAXIMIZED at every launch (binding 2026-11-02 operator correction; never auto-fullscreened), and while the release build runs no console window is visible. When OFF, the floating bottom pill and all other UI controls are visible and the image takes almost all of the central space. Toggling shares ONE action between the F11 keyboard shortcut and the pill's painted fullscreen-square button; the first press turns fullscreen ON — the window header/borders and ALL UI controls (the pill included) disappear and only the image shows, while the arrow keys keep navigating images/folders; F11 — or ESC — exits fullscreen while ON (outside fullscreen, ESC does nothing). Fullscreen toggling never touches the zoom state.`

**R-2** · Feature 5 bullet — WHOLE-REWRITE:
- FROM: the entire bullet starting `5. **Zoom In/Out and Fit Modes** — zoom in/out via the keyboard (`I`, alias Shift+`+`; `O`, alias Shift+`−`)` and ending `...Zoom and fit state are session-only (no persistence in v1).`
- TO: `5. **Zoom In/Out and Fit Levels** — zoom in/out via the keyboard (`I` zooms in, `O` zooms out; strict shifted aliases: Shift+`+` or Shift+`=` zooms in, Shift+`−` zooms out — the plain unshifted `+`, `=`, and `-` keys do nothing) and via the pill's [+][−] buttons; every step multiplies/divides by 1.25 within the SYMMETRIC fixed bounds — from ×8 of the whole-image fit at the ceiling down to ÷8 of it at the floor (binding 2026-11-02 correction: zoom-out is NOT blocked at the neutral fit level and keeps shrinking below it). Steps at a bound clamp SILENTLY — every zoom-bound message was removed; zoom actions never announce a message. The [+]/[−] buttons paint dimmed exactly at their bounds. Zooming enlarges/shrinks the image's center — the overflow is not drawn (pan is Wave 5). The `0` key and the pill's [restore-fit] button return to whole-image fit at ×1.0, and EVERY successful image open resets to whole-image fit at ×1.0 as well. "Fit width" (`W` key or the pill's fit-width button) sets the zoom level ONE-SHOT so the image's width fills the window (it may overflow vertically); "fit height" (`H` key or the pill's fit-height button) sets it so its height fills the window (it may overflow horizontally) — the set is a one-shot SIZE SET, not a mode: normal zoom in/out keeps working FROM the set level, a window resize does not re-fit, the action is silent, and both fit buttons are disabled while an empty folder is browsed. Zoom and fit state are session-only (no persistence in v1).`

**R-3** · Controls table — three row edits (the `Restore whole-image fit | 0 |` row stays UNCHANGED):
- `| Toggle fullscreen | F11 (only exit while ON) | fullscreen square button |` → `| Toggle fullscreen | F11 (ESC also exits while fullscreen is ON) | fullscreen square button |`
- `| Zoom in / zoom out | I (alias Shift+Plus) / O (alias Shift+Minus) | [+] / [−] buttons |` → `| Zoom in / zoom out | I / O (strict shifted aliases: Shift+'+' or Shift+'=' in, Shift+'−' out; unshifted +/=/− inert) | [+] / [−] buttons (dimmed at their bounds) |`
- `| Fit width / fit height | W / H | fit-width / fit-height buttons |` → `| Fit width / fit height (one-shot size set) | W / H | fit-width / fit-height buttons |`

**R-4** · Controls paragraph:
- FROM: `The pill renders while an image is open or while an empty sibling folder is browsed; every button is enabled exactly when its action resolves a destination (at the zoom bounds and without an open image the respective buttons paint disabled).`
- TO: `The pill renders while an image is open or while an empty sibling folder is browsed (on the fresh empty state the pill itself is absent); every button is enabled exactly when its action resolves a destination (each zoom button paints disabled exactly at its bound — [+] at ×8, [−] at the ÷8 floor, restore-fit only at ×1.0 — and the two fit buttons also stay disabled while an empty folder is browsed). The row shows 10 buttons; since Wave 4.1 a small hotkey caption is painted beneath the glyph of the six keyboard-distinct buttons — `F11` (fullscreen square), `I` (+), `O` (−), `0` (restore-fit), `W` (fit-width), `H` (fit-height) — while the four arrows carry no caption, and every one of the 10 buttons shows a hover tooltip in BOTH its enabled and disabled state: "Previous image (Left Arrow)", "Next image (Right Arrow)", "Previous sibling folder (Up Arrow)", "Next sibling folder (Down Arrow)", "Fullscreen (F11)", "Zoom in (I)", "Zoom out (O)", "Restore fit (0)", "Fit width (W)", "Fit height (H)" (data: `src/app/button_hint.rs`).`

**R-5** · Technology Stack + Project Status:
- R-5a (intro sentence): FROM `...the Wave-2 horizontal-navigation slice, the Wave-3 vertical-navigation slice, and the Wave-4 fullscreen/zoom/fit slice; the remaining v1 features` TO `...the Wave-2 horizontal-navigation slice, the Wave-3 vertical-navigation slice, the Wave-4 fullscreen/zoom/fit slice, and the Wave-4.1 operator-fix corrections; the remaining v1 features`.
- R-5b (Language row): FROM `| Language | Rust | Implemented: Cargo package `img-scroller` 0.4.0, edition 2024. |` TO `| Language | Rust | Implemented: Cargo package `img-scroller` 0.4.1, edition 2024. |`
- R-5c (Project Status paragraph, three components):
  - Insert the Wave-4.1 sentence AFTER the Wave-4 historical sentence (which ends `...zoom and fit state are session-only.`) and BEFORE `Verification so far:` — exactly: `Wave 4.1 landed (branch `fix/wave-4-operator-fixes`, version 0.4.1): the operator-fix wave — the launch window opens ALWAYS MAXIMIZED with the console hidden in release builds (fullscreen default stays OFF); the six keyboard-distinct pill buttons show hotkey captions and all ten buttons show hover tooltips; ESC exits fullscreen ON in addition to F11 (the operator-perceived F11 delay is the OS key-repeat timing — operator note); every zoom-bound message was removed (silent actions); the zoom range became symmetric — zoom-out passes whole-image fit and keeps shrinking to the ÷8 floor (×1.25 steps; the buttons dim at the bounds); Shift+'+' / Shift+'−' are the strict zoom aliases (plain +/=/− inert); and W/H plus both fit buttons set the zoom level ONE-SHOT to width/height fit — zoom keeps working from that level, no window resize re-fits, and a successful open returns whole-image fit ×1.0.`
  - Date the superseded historical fragment: FROM `(default OFF at launch; F11 and the pill's fullscreen square toggle it; ON hides the header/borders and all controls with only the image, and F11 is the only exit)` TO `(default OFF at launch; F11 and the pill's fullscreen square toggle it; ON hides the header/borders and all controls with only the image, F11 its only exit at 0.4.0)`.
  - Fix the verification sentence: FROM `Verification so far: container dev checks (`ALL CHECKS PASSED`, 93 unit tests), Windows x64 release cross-build (exit 0 → `dist/windows-x64/img-scroller.exe` = 19,535,645 bytes), import audit; running the `.exe`` TO `Verification so far: container dev checks (`ALL CHECKS PASSED`, 104 unit tests), Windows x64 release cross-build (exit 0 → `dist/windows-x64/img-scroller.exe` = 19,593,177 bytes), import audit; running the `.exe`` — COUNTS AND SIZE ONLY; everything after that anchor phrase stays byte-identical.

### 3.2 `.agent/project-info/tech.md` (4 edits)

**T-1** · Status paragraph:
- T-1a: FROM `Every landed slice is verified through the container harness (dev checks: `ALL CHECKS PASSED`, 93 unit tests; Windows x64 release cross-build: exit 0 → `dist/windows-x64/img-scroller.exe` = 19,535,645 bytes at 0.4.0; the DLL import audit is UNCHANGED` TO `Every landed slice is verified through the container harness (dev checks: `ALL CHECKS PASSED`, 104 unit tests; Windows x64 release cross-build: exit 0 → `dist/windows-x64/img-scroller.exe` = 19,593,177 bytes at 0.4.1; the DLL import audit is UNCHANGED` (only the numbers/only this anchor phrase).
- T-1b: Insert the Wave-4.1 sentence immediately AFTER the Wave-4 Landed-sentence (which ends `(mechanism: `## Fullscreen, Zoom and Fit Modes (Implemented, Wave 4)` below)`) — exactly: ` **Wave 4.1 CORRECTED those 0.4.0 facts** (branch `fix/wave-4-operator-fixes`, version 0.4.1, 2026-11-05/06; binding operator corrections 2026-11-02): launch ALWAYS MAXIMIZED + release console hidden; pill hotkey captions + hover tooltips; ESC exits fullscreen ON; ALL zoom/fit bound-and-activation messages removed (silent actions); zoom range symmetric [1/8, 8] with zoom-out passing fit; strict Shift+'+' / Shift+'−' aliases (plain +/=/− inert); W/H one-shot fit sets — the FitMode state machine removed. The 0.4.0 facts above are superseded where they contradict these (mechanism: `## Fullscreen, Zoom and Fit Modes (Implemented, Wave 4)` below, rewritten to the final model).`

**T-2** · Technology-Stack Language row: FROM `| Language | Rust, edition 2024 | Cargo package `img-scroller` version 0.4.0 (single binary; tracked `Cargo.lock`). |` TO `| Language | Rust, edition 2024 | Cargo package `img-scroller` version 0.4.1 (single binary; tracked `Cargo.lock`). |`

**T-3** · `## Fullscreen, Zoom and Fit Modes (Implemented, Wave 4)` — the HEADING is KEPT EXACTLY (the body explains the correction layer; no Contents/anchor churn). The BODY — everything from the paragraph starting `Implemented in `src/zoom.rs`` through the end of the last Display bullet (ending `...one upload per decode).`) — is replaced by:

```markdown
Landed in Wave 4; the model recorded here carries the binding 2026-11-02 Wave-4.1 operator corrections — the 0.4.0 fit-mode×factor composition, the asymmetric [1.0, 8.0] zoom range, the loose alias chord, and every rejected-step/bound/activation message they contained are SUPERSEDED: `Zoom max`/`Zoom min`/`Fit width`/`Fit height` messages no longer exist anywhere in `src/`. Implemented in `src/zoom.rs` (pure zoom math), `src/scaling.rs` (the one-shot fit factors), `src/app/zoom.rs` (the app seams incl. the alias chord and the one-shot fit sets), `src/app/fullscreen.rs` (the fullscreen flag/gate/layout + ESC exit), `src/app/keyboard_intake.rs` (all per-frame key intake), `src/app/zoom_button.rs`/`src/app/fit_button.rs`/`src/app/fullscreen_button.rs` (the pill groups), `src/app/button_hint.rs` (captions/tooltips), `src/app/central_surface.rs` (the single paint seam), and `src/app_display.rs` (the paint math):

- Launch state (Wave 4.1 Task 1, superseding "launch options untouched"): `src/app.rs` builds `NativeOptions` with `.with_maximized(true)` — the window opens ALWAYS MAXIMIZED at EVERY launch; `src/main.rs` carries `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` — the console window is hidden in RELEASE builds only (a dev `cargo run` console is expected, not a defect). The fullscreen default stays OFF and nothing ever auto-fullscreens.
- Session state (RAM in `ImagesScrollerApp`, never persisted): `fullscreen_on` (default OFF) and `zoom_factor` — ONE multiplicative factor with SYMMETRIC bounds: ceiling 8.0, floor `ZOOM_FACTOR_MINIMUM = 1.0 / ZOOM_FACTOR_MAXIMUM` (≈ ÷8, `src/zoom.rs`), step ×1.25, restore target 1.0. NO mode field of any kind exists (the former `FitMode` tri-state and `set_fit_mode` seam were removed; `ImagePainting` is the 2-field `{ decoded_image, zoom_factor }` request).
- Fullscreen: ONE shared toggle action (`toggle_fullscreen_and_send_command`) flips the flag and sends `egui::ViewportCommand::Fullscreen(bool)` exactly once; the F11 intake (`FULLSCREEN_TOGGLE_KEY`) and the pill's painted-square button both use it; the ONE chrome gate `full_ui_chrome_is_visible()` hides pill/Open-button/empty-folder-message while ON. Wave 4.1 added the ESC exit (`FULLSCREEN_EXIT_KEY = Key::Escape`, pure predicate `escape_exits_fullscreen`): ESC exits ONLY while fullscreen is ON; otherwise it is a silent no-op colliding with no other key. The operator's F11 non-immediacy was root-caused agent-side as the OS key-repeat delay with NO timing change (recorded as the OPERATOR-RUN note). Arrow navigation and zoom/fit hotkeys keep working while ON; successful opens keep fullscreen ON; toggling never touches `zoom_factor`.
- Zoom: `I` zooms in, `O` zooms out; the alias layer is strict (`shift_alias_is_pressed` — `src/app/zoom.rs`): Shift+'+' OR Shift+'=' zooms in, Shift+'−' zooms out; the PLAIN (unshifted) `+`, `=`, and `-` keys are inert. `0` (`Key::Num0`) restores. Every step multiplies/divides by 1.25 between the symmetric bounds; zoom-out PASSES the whole-image fit level and keeps shrinking to the ÷8 floor (binding correction); a step past a bound clamps SILENTLY — no message is ever announced for zoom/fit actions (the ONLY remaining pill message is the unsupported-file failure text). Button enablement: [+] dimmed exactly while `zoom_can_grow()` is false (at the ceiling), [−] dimmed exactly while `zoom_can_shrink()` is false (at the floor), restore-fit dimmed exactly while `zoomed_off_fit()` is false (factor == 1.0).
- One-shot fit (Wave 4.1 Task 7, replacing the Wave-4 mode model): `W`/`H` keys AND both fit buttons funnel through the ONE seams `set_zoom_to_fit_width`/`set_zoom_to_fit_height` (`src/app/zoom.rs`) — each computes the axis-fit factor AT INTAKE from the current available size (`egui::viewport_rect().size()`; egui 0.37 removed `screen_rect`, so the intake decision reads the full viewport rect) as an axis-ratio over the whole-image fit via `scaling::fit_width_factor`/`fit_height_factor`, clamped INSIDE the symmetric bounds through the ONE shared clamp `crate::zoom::clamped_zoom_factor`, and sets `zoom_factor` ONLY. The set is silent; it no-ops without an open image; afterward normal zoom in/out works FROM that level (nothing blocked); a repeated set recomputes; `0`/restore and EVERY successful open reset to 1.0; and the display keeps composing `fitted_size(current available) × zoom_factor` per frame, so a window resize after a set does NOT re-fit.
- Intake: `src/app/keyboard_intake.rs` runs the one per-frame read in fixed order (F11 → ESC-exit → zoom-in → zoom-out → restore `0` → `W` → `H` → the four arrows), dispatch mirroring the reads, in EVERY mode and always BEFORE the layout dispatch.
- Display: `ImagePainting { decoded_image, zoom_factor }` (2 fields, `src/app_display.rs`) → `fitted_size` scaled by `zoomed_draw_size` every frame; zoomed overflow clips so the CENTER stays visible (no pan — Wave 5); BOTH paint surfaces route through the single `src/app/central_surface.rs` seam; texture discipline unchanged (one upload per decode).
```

**T-4** · Testing — the census bullet is REPLACED whole (the following `- Scope: pure logic only...` and `- Windows runtime/GUI...` bullets stay byte-identical):
- FROM: the entire bullet starting `- 93 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3):` and ending `...non-positive rejection).`
- TO:

```markdown
- 104 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3; census pinned per file by the Task-8 verification): `src/sibling_folder_listing/tests.rs` — 6 (sibling-folder collation sort with exact-name tie-break; directories-only enumeration with the synthetic scope; current-folder exclusion; non-recursive immediate-siblings-only; no-siblings yields an empty listing; unreadable scope yields an empty listing and an unanchorable anchor yields no listing); `src/sibling_folder_listing/neighbor_tests.rs` — 2 (±1 folder stepping with scope-edge reports; symlinked scope entries are never listed as directories); `src/name_collation.rs` — 2 (lowercase form decides before exact characters, exact tie-break when folded forms match); `src/folder_listing/tests.rs` — 5 (unchanged horizontal listing census); `src/app/tests.rs` — 9 (unchanged shell/open-flow census); `src/app/floating_controls_tests.rs` — 1 (arrow-button enablement follows the horizontal navigation edges); `src/app/folder_navigation_tests.rs` — 5 (adjacent sibling folders resolve in sort order; a visited folder resurfaces its last-viewed image; a new folder opens its first image; entering an image-less folder clears the image and re-anchors the vertical listing; empty-folder mode continues vertically into both folder kinds); `src/app/folder_navigation_continuation_tests.rs` — 4 (sibling-list edges stay silent in image and empty-folder modes; a vanished last-viewed target reports the standard failure and keeps the displayed image; button enablement matches neighbor-resolution semantics; a vertical target open records the new folder's last-viewed memory); `src/app/last_viewed_tests.rs` — 5 (fresh empty memory; success records the folder's image; every open updates the entry; a failed open keeps it; folders are scoped independently); `src/image_loading.rs` — 4 (message wording; per-failure message mapping; pixel-buffer sizing; empty/overflow rejection); `src/app_display.rs` — 2 (initial texture state; upload happens exactly once per decode); `src/app/button_hint.rs` — 2 (per-button hotkey-caption and hover-tooltip strings pinned); `src/scaling.rs` — 5 (fit vectors ×4; zero-dimension rejection); `src/zoom.rs` — 8 (symmetric-bound step/clamp/composition math — the floor pins as 1.0/ceiling — plus the fit-size × factor composition); `src/app/zoom_tests.rs` — 16 (app zoom policy: symmetric bounds with zoom-out PAST fit and SILENT clamps at both bounds, restore, open-reset, fullscreen-toggle neutrality, strict-shift alias behavior — Shift+'+' / Shift+'=' in and Shift+'−' out, plain +/=/− inert — and at-bound/off-fit button enablement); `src/app/fullscreen_tests.rs` — 9 (launch OFF; F11 pin; one-inversion-per-trigger; chrome gate ON/OFF; navigation-context persistence; open keeps fullscreen; plus the Wave-4.1-Task-3 ESC pins: the `FULLSCREEN_EXIT_KEY` constant and the `escape_exits_fullscreen` predicate matrix); `src/app/fit_one_shot_tests.rs` — 12 (one-shot fit app policy: W/H keys and both buttons set the factor ONLY; silent; I/O work FROM the set level; `0`/successful-open reset to 1.0; no-op without an image; label pins); `src/scaling/fit_factor_tests.rs` — 7 (width/height factor ratios over the whole-image fit, clamping INSIDE the symmetric bounds, non-positive-size rejection). The Wave-4.1 corrections replaced the removed former `src/app/fit_mode_tests.rs` (11) and `src/scaling/fit_mode_tests.rs` (6); their successors are the fit-factor and one-shot census lines above.
```

### 3.3 `.agent/project-info/architecture.md` (10 edits)

**A-1** · Status paragraph:
- A-1a: FROM `Dev checks ended `ALL CHECKS PASSED` (93 unit tests), the exe was rebuilt at `dist/windows-x64/img-scroller.exe` (19,535,645 bytes), and the DLL import audit is unchanged` TO `Dev checks ended `ALL CHECKS PASSED` (104 unit tests), the exe was rebuilt at `dist/windows-x64/img-scroller.exe` (19,593,177 bytes at 0.4.1), and the DLL import audit is unchanged`.
- A-1b: insert immediately BEFORE `Pan-then-navigate while zoomed (Wave 5)` — exactly: `Wave 4.1 (branch `fix/wave-4-operator-fixes`, version 0.4.1) corrected the operator-reported behaviors on the same architecture: launch ALWAYS MAXIMIZED with the release console hidden; pill hotkey captions + hover tooltips; ESC exits fullscreen ON; silent zoom/fit actions (every bound message removed); symmetric zoom range [1/8, 8]; strict shift-only zoom aliases; and the fit-mode tri-state replaced by ONE-SHOT fit size sets (W/H keys + both buttons set the zoom factor only). `

**A-2** · Logical Components:
- A-2a (Viewer UI bullet): FROM `regular resizable window (title `Images Scroller`, not maximized at launch)` TO `regular resizable window (title `Images Scroller`, opened ALWAYS MAXIMIZED at launch)`; and FROM `(10 buttons in the row) — are LANDED;` TO `(10 buttons in the row) — are LANDED, and since Wave 4.1 the six keyboard-distinct buttons carry hotkey captions while all ten carry hover tooltips (`src/app/button_hint.rs`);`
- A-2b (Full-Screen View bullet): FROM `**Full-Screen View — LANDED (Wave 4)**: session `fullscreen_on` flag (default OFF at launch; launch options untouched); toggle via F11` TO `**Full-Screen View — LANDED (Wave 4 + Wave-4.1 corrections)**: session `fullscreen_on` flag (default OFF at launch; the window itself opens ALWAYS MAXIMIZED and the release console stays hidden); toggle via F11`. In the same bullet: FROM `F11 is the only exit; arrow navigation and zoom/fit hotkeys remain active while ON (intake is ungated), and successful opens keep fullscreen ON.` TO `F11 or ESC exits while ON (ESC no-op otherwise; the F11-delay observation is the OS key-repeat — operator note); arrow navigation and zoom/fit hotkeys remain active while ON (intake is ungated), and successful opens keep fullscreen ON.`
- A-2c (Manual Zoom + Fit Modes bullet) — WHOLE-REWRITE to: `**Manual Zoom + Fit — LANDED (Wave 4 + Wave-4.1 corrections)**: ONE session `zoom_factor` with the SYMMETRIC bounds [1/8, 8] (floor = 1/ceiling; ×1.25 multiplicative steps; pure math `src/zoom.rs`; app seam `src/app/zoom.rs`) — zoom-out passes whole-image fit and keeps shrinking; steps at a bound clamp SILENTLY (all bound messages removed; the only remaining app message is the unsupported-file failure); the buttons dim via `zoom_can_grow`/`zoom_can_shrink`/`zoomed_off_fit`. Fit is NO MODE: `W`/`H` keys and both fit buttons (`src/app/fit_button.rs`, renamed from `fit_mode_button`) funnel through `set_zoom_to_fit_width`/`set_zoom_to_fit_height` and set the factor ONLY (axis-ratio over the whole-image fit via `scaling::fit_width_factor`/`fit_height_factor`, clamped INSIDE the bounds; intake at `viewport_rect().size()`); zoom keeps working from the set level; `0`/restore and every successful open return to fit ×1.0; the per-frame draw size composes `fitted_size × factor` in `src/app_display.rs` (a window resize recomputes the base, NOT the factor — no re-fit).`

**A-3** · Module Layout:
- A-3a (`src/app.rs` bullet): the executor greps the bullet for the exact substring containing `fullscreen_on`/`zoom_factor`/`fit_mode` in the state enumeration and replaces that substring with ``fullscreen_on`/`zoom_factor`` (surrounding text byte-identical). Then FROM `and `NativeOptions` (resizable, not maximized).` TO `and `NativeOptions` (resizable, always maximized at launch).` Then TO the whole Child-modules list: FROM `src/app/arrow_direction.rs`, `src/app/central_surface.rs`, `src/app/empty_folder_view.rs`, `src/app/empty_state_view.rs`, `src/app/fit_mode_button.rs`, `src/app/floating_controls.rs`, `src/app/folder_navigation.rs`, `src/app/fullscreen.rs`, `src/app/fullscreen_button.rs`, `src/app/keyboard_intake.rs`, `src/app/zoom.rs`, `src/app/zoom_button.rs`, plus the test modules `src/app/tests.rs`, `src/app/floating_controls_tests.rs`, `src/app/folder_navigation_tests.rs`, `src/app/folder_navigation_continuation_tests.rs`, `src/app/last_viewed_tests.rs`, `src/app/fullscreen_tests.rs`, `src/app/zoom_tests.rs`, `src/app/fit_mode_tests.rs` TO `src/app/arrow_direction.rs`, `src/app/button_hint.rs`, `src/app/central_surface.rs`, `src/app/empty_folder_view.rs`, `src/app/empty_state_view.rs`, `src/app/fit_button.rs`, `src/app/floating_controls.rs`, `src/app/folder_navigation.rs`, `src/app/fullscreen.rs`, `src/app/fullscreen_button.rs`, `src/app/keyboard_intake.rs`, `src/app/zoom.rs`, `src/app/zoom_button.rs`, plus the test modules `src/app/tests.rs`, `src/app/floating_controls_tests.rs`, `src/app/folder_navigation_tests.rs`, `src/app/folder_navigation_continuation_tests.rs`, `src/app/last_viewed_tests.rs`, `src/app/fullscreen_tests.rs`, `src/app/zoom_tests.rs`, `src/app/fit_one_shot_tests.rs`
- A-3b: NEW bullet inserted right after the `arrow_direction.rs` bullet — exactly: `- `src/app/button_hint.rs` — the pill-button caption/tooltip data (Wave 4.1 Task 2): the per-button hotkey captions (six keyboard-distinct buttons ONLY — the four arrows are uncaptioned) and the per-button hover tooltips for all ten buttons (in BOTH enabled and disabled states), plus the shared top-aligned vertical button-column wrapper the pill uses; two inline tests pinned.`
- A-3c (fit-mode-button bullet): the bullet beginning `- `src/app/fit_mode_button.rs` — the pill's two fit-mode buttons` is REPLACED by: `- `src/app/fit_button.rs` — the pill's two one-shot fit buttons (renamed from `src/app/fit_mode_button.rs` in Wave 4.1 Task 7; the labels, painted line-plus-triangle glyphs, sizes and gaps carried over verbatim; the mode state was removed): enabled exactly while an image is open (`image_is_open` gate) — disabled while an empty folder is browsed; clicks drive the one-shot `set_zoom_to_fit_width`/`set_zoom_to_fit_height` seams (factor set ONLY, silent), with the available size observed at `viewport_rect().size()`.`
- A-3d (`keyboard_intake.rs` bullet): FROM `one read pass in fixed order (F11 → zoom-in → zoom-out → restore `0` → `W` → `H` → the four arrows)` TO `one read pass in fixed order (F11 → ESC-exit-fullscreen → zoom-in → zoom-out → restore `0` → `W` → `H` → the four arrows)`.
- A-3e (`zoom.rs` app bullet) — WHOLE-REWRITE to: `- `src/app/zoom.rs` — the zoom/fit app seam: the `I`/`O`/`0` hotkey constants and the STRICT alias chord (`shift_alias_is_pressed`: Shift+'+'/Shift+'=' in, Shift+'−' out; plain +/=/− inert), `zoom_in_once`/`zoom_out_once` with SILENT symmetric-bound clamping (bounds [1/8, 8] — no messages), the at-bound/off-fit predicates (`zoom_can_grow`/`zoom_can_shrink`/`zoomed_off_fit`) the zoom buttons dim from, and the ONE-SHOT seams `set_zoom_to_fit_width`/`set_zoom_to_fit_height` (the fit keys and both buttons funnel here; factor-only set, silent; no-op without an open image; the restore/`0` path and open-success reuse the plain reset to 1.0).`
- A-3f (`zoom_button.rs` bullet): FROM `with at-bound dimming (zoom-in disabled exactly at ×8.0; zoom-out and restore disabled exactly at fit) and the click seams.` TO `with at-bound dimming from the predicated state: zoom-in disabled exactly at the ×8 ceiling; zoom-out disabled exactly at the ÷8 floor (zoom-out WORKS at the fit level itself since Wave 4.1 and dims only AT the floor); restore-fit disabled exactly while the factor is 1.0 (`zoomed_off_fit` false); and the click seams.`
- A-3g (`central_surface.rs` bullet): FROM `the one `ImagePainting` owner (decoded image + session fit mode + session zoom factor).` TO `the one `ImagePainting` owner (the 2-field Wave-4.1 form: decoded image + session zoom factor).`
- A-3h (`scaling.rs` bullet): FROM `Wave 4 adds the `FitMode` tri-state and `fit_width_size`/`fit_height_size`, dispatched by `FitMode::base_size`, with the fit-mode math tests split into `src/scaling/fit_mode_tests.rs`.` TO `Wave 4.1 replaced the former `FitMode` dispatch with the ONE-SHOT factors `fit_width_factor`/`fit_height_factor` (each an axis-span size ratio over the whole-image `fitted_size`, clamped INSIDE the symmetric zoom bounds via `crate::zoom::clamped_zoom_factor` — the ONE shared clamp), with the factor tests split into `src/scaling/fit_factor_tests.rs`.`
- A-3i (`app_display.rs` bullet): FROM `Wave 4 adds the `ImagePainting { decoded_image, fit_mode, zoom_factor }` request object — `paint` computes the active mode's base size then scales it by the zoom factor per frame.` TO `Wave 4.1 reduced `ImagePainting` to the 2-field `{ decoded_image, zoom_factor }` request object — `paint` recomputes `fitted_size` (per-frame available size) then scales it by the zoom factor (`zoomed_draw_size`) per frame, so a window resize changes the whole-image base, NOT the stored factor (no re-fit).`

**A-4** · Navigation Critical Paths:
- A-4a (intro sentence): FROM `and Path 4's fullscreen/zoom/fit toggle slice (Wave 4);` TO `and Path 4's fullscreen/zoom/fit toggle slice (Wave 4, as corrected by Wave 4.1);`
- A-4b (path-4 bullet) — WHOLE-REWRITE to: `4. **Fullscreen / zoom / fit toggles (Wave-4 slice LANDED, corrected by Wave 4.1)**: the window opens ALWAYS MAXIMIZED at launch with the release console hidden; fullscreen OFF default — F11 or the pill's square toggles ON (header/borders + every control hidden — only the image; F11 or ESC exits while ON, ESC no-op otherwise), OFF restores the standard chrome; zoom in/out steps ×1.25 within the SYMMETRIC [1/8, 8] bounds via `I`/`O` (strict Shift+'+' / Shift+'−' aliases; plain +/=/− inert) or the `[+][−]` buttons, clamping SILENTLY at both bounds (zoom-out passes fit and keeps shrinking; the buttons dim at their bounds); `W`/`H` keys and both fit buttons set the zoom factor ONE-SHOT to width/height fit (zoom continues from that level; no resize re-fit); restore/`0` and every successful open return whole-image fit ×1.0.`

**A-5** · Data and State (per-session bullet): FROM `the session `fullscreen_on` flag (default OFF), the session `zoom_factor` ([1.0, 8.0] bounds), and the session `fit_mode` (`FitVisible`/`FitWidth`/`FitHeight`)` TO `the session `fullscreen_on` flag (default OFF; the launch window itself is always-maximized with the release console hidden) and the session `zoom_factor` (symmetric [1/8, 8] bounds — NO separate fit-mode field exists since Wave 4.1; every fit key/button mutates this one factor only)`.

**A-6** · Project Paths:
- FROM `Cargo project at the repository root: `Cargo.toml` (package `img-scroller` 0.4.0, edition 2024)` TO `Cargo project at the repository root: `Cargo.toml` (package `img-scroller` 0.4.1, edition 2024)`.
- In the repository-layout enumeration: FROM `with its `arrow_direction`/`central_surface`/`empty_folder_view`/`empty_state_view`/`fit_mode_button`/`floating_controls`/`folder_navigation`/`fullscreen`/`fullscreen_button`/`keyboard_intake`/`zoom`/`zoom_button` child modules` TO `with its `arrow_direction`/`button_hint`/`central_surface`/`empty_folder_view`/`empty_state_view`/`fit_button`/`floating_controls`/`folder_navigation`/`fullscreen`/`fullscreen_button`/`keyboard_intake`/`zoom`/`zoom_button` child modules`; and FROM `` `scaling` (+ `scaling/fit_mode_tests`) `` TO `` `scaling` (+ `scaling/fit_factor_tests`) `` (both inside the same body line; the surrounding text stays byte-identical).

**A-7** · Technical Decisions Summary — edit 2 rows, add 2 rows (immediately after the zoom row):
- Fullscreen row: FROM `| Fullscreen mechanism: one shared toggle action sending `ViewportCommand::Fullscreen`; hidden chrome while ON | landed (Wave 4) | `### Fullscreen Mode` |` TO `| Fullscreen mechanism: one shared toggle action sending `ViewportCommand::Fullscreen`; hidden chrome while ON; F11 OR ESC exits while ON (ESC no-op otherwise); launch ALWAYS MAXIMIZED + release console hidden | landed (Wave 4; ESC + launch state = Wave-4.1 operator corrections) | `### Fullscreen Mode` + the 2026-11-02 brief addendum |`
- Zoom row: FROM `| Zoom policy: ×1.25 steps clamped [1.0, 8.0]; fit-mode coupling via one `set_fit_mode` seam; reset-on-open from the user-approved binding blocks | landed (Wave 4) | `### Zoom In/Out and Fit Modes` |` TO `| Zoom policy: ×1.25 steps clamped to the SYMMETRIC [1/8, 8] bounds (floor = 1/ceiling; zoom-out passes fit); silent bounds (all bound/fit messages removed); strict Shift+'+' / Shift+'−' aliases; one-shot W/H fit sets replacing `set_fit_mode`; reset-on-open | landed (Wave 4; range/aliases/messages/fit semantics corrected by Wave 4.1) | `### Zoom In/Out and Fit Modes` + the 2026-11-02 brief addendum |`
- ADD row: `| Pill affordances: hotkey captions under the six keyboard-distinct buttons; hover tooltips on all ten buttons (enabled + disabled) | landed (Wave 4.1) | 2026-11-02 operator correction (brief addendum) |`
- ADD row: `| Launch state: ALWAYS-MAXIMIZED window (`with_maximized(true)`); no console in release builds (`windows_subsystem = "windows"`) | landed (Wave 4.1) | 2026-11-02 operator correction (brief addendum) |`

**A-8** · Open Architectural Decisions — verified NO-CHANGE: the current list (Pan-then-navigate details while zoomed (Wave 5); Distribution-helper mechanism; Config file contents/format; Design patterns beyond immediate-mode) already contains exactly the keep-set the wave requires. The executor greps the section for `FitMode|fit_mode|zoom|fullscreen` case-insensitively — ZERO hits are expected; ANY hit means the section drifted since this plan → STOP and return to the caller instead of editing.

### 3.4 `.agent/project-info/context.md` (6 edits — the Closing-Step file)

**C-1** · Current Work Focus — WHOLE-REWRITE of the paragraph under the heading to:

```markdown
Dated 2026-11-06 (sessions 2026-11-05/06). **Wave 4.1 operator fixes are COMPLETE agent-side on branch `fix/wave-4-operator-fixes`** (wave tasks 1–9 all landed; Task 9 = the wave documentation, this step). Task 1 always-maximized launch + release-console hiding (`src/app.rs:146` / `src/main.rs:4`); Task 2 pill hotkey captions + hover tooltips (`src/app/button_hint.rs`); Task 3 F11/ESC tuning (F11 delay = OS key-repeat — operator note; ESC exits fullscreen ON, no-op otherwise); Task 4 ALL zoom-bound messages removed (silent clamp); Task 5 symmetric zoom ÷8–×8 (zoom-out passes fit; buttons dim at the bounds); Task 6 strict shift-only aliases (plain +/=/− inert); Task 7 one-shot W/H fit sets (the FitMode model removed; `fit_button.rs` rename; factor intake at `viewport_rect()`); Task 8 fresh container verification: dev-checks `ALL CHECKS PASSED` (104 unit tests in 18 files), the 0.4.1 exe rebuilt at 19,593,177 bytes, DLL import audit SET-EQUAL to the recorded 22-DLL constant; Task 9 documentation landed (this step) — README/tech/architecture/structure/brief/product/context. **The wave's ONLY remaining gate is the OPERATOR-RUN in Next-Step #1** (the verbatim 15-item checklist from `.kilo/plans/20261102-wave-4-1-task8-verification-plan.md` §11; the Task-8 OPERATOR-RUN sub-item stays `[ ]` until the user reports). After the re-test closes the wave (merge/rename/push are the Workflow-Step-5 mechanics), Wave 5 (`20261029-todo-7.md`) runs in a NEW chat. Wave 4 itself remains landed on `main` at `19aa9a6`; the Wave-4 OPERATOR-RUN gate closes with this Wave-4.1 re-test.
```

**C-2** · Recent Changes — insert ONE new entry at the top (before the current first entry); ALL other entries stay byte-identical:

```markdown
1. 2026-11-06: **Wave 4.1 Task 9 documentation landed — the wave's agent-side work is COMPLETE.** Branch `fix/wave-4-operator-fixes` (whole wave chain, HEAD moving past `37df6eb`): `2fd26c8` (todo) → `7df1ef3` (global plan) → `53c2e60` (bump 0.4.1) → Task 1 `e1092f8`/`e3873f8`/`93eb47d`/`4052c5a`/`86a1c2c` (always-maximized launch + console hidden; `app.rs:146`, `main.rs:4`) → Task 2 `fa8ec7e`/`4a2e7f0`/`95247dc`/`4833576`/`892efc1` (hotkey captions + tooltips; `src/app/button_hint.rs`) → Task 3 `02e0870`/`9c46fa3`/`db83dd4`/`c135ede`/`9c0625a` (ESC exits fullscreen ON; F11 delay = OS key-repeat note) → Task 4 `f2aa371`/`943d868`/`72a0de7`/`f543da9` (ALL zoom-bound messages removed) → Task 5 `761ac50`/`d4c6870`/`990006c`/`2483f76`/`c19ff78`/`daf0e2f` (symmetric ÷8 floor = 1/×8 ceiling; silent bounds) → Task 6 `fc2a6f4`/`e6ddb7b`/`92b86f7`/`aa2175b` (strict Shift+'+' / Shift+'−' aliases; plain +/=/− inert) → Task 7 `8e51cbf`/`ae4b0f3`/`4e44674`/`2aecf5a`/`5c7a067`/`f45c7ea`/`5579cfb` (one-shot W/H fit sets; FitMode removed; `fit_button` rename; factor intake at `viewport_rect`) → Task 8 `3f74a3f`/`37df6eb` (fresh dev-checks `ALL CHECKS PASSED` 104/0; exe rebuilt 19,593,177 B; DLL audit set-equal 22; the OPERATOR-RUN sub-item stays `[ ]`) → Task 9 (this step: wave reference docs + the 15-item OPERATOR-RUN checklist copied into Immediate Next Steps #1). Awaiting ONLY the user's OPERATOR-RUN on Windows (gate #1).
```

**C-3** · Current Project State:
- C-3a: REPLACE the first bullet (starting `- **Wave 4.1 branch `fix/wave-4-operator-fixes` (HEAD `aa2175b`)**` and ending `...no code written.`) with:

```markdown
- **Wave 4.1 branch `fix/wave-4-operator-fixes` (agent-side COMPLETE; HEAD at the Task-9 docs commits)**: Task 1 launch ALWAYS MAXIMIZED (`with_maximized(true)` at `src/app.rs:146`) + console hidden in release builds (`windows_subsystem` at `src/main.rs:4`); Task 2 hotkey captions under the six keyboard-distinct pill buttons + hover tooltips on all 10 (`src/app/button_hint.rs`; tooltip strings pinned); Task 3 ESC exits fullscreen ON (`FULLSCREEN_EXIT_KEY`/`escape_exits_fullscreen`; no-op otherwise) — the F11-immediacy observation is the OS key-repeat (operator note); Task 4 ALL zoom-bound messages removed (silent clamping; zero `Zoom max`/`Zoom min` in `src/`); Task 5 symmetric zoom range (`ZOOM_FACTOR_MINIMUM = 1.0 / ZOOM_FACTOR_MAXIMUM` at `src/zoom.rs:20`; the buttons dim at the bounds); Task 6 strict shift aliases (`shift_alias_is_pressed`); Task 7 one-shot fit (`set_zoom_to_fit_width`/`set_zoom_to_fit_height` seams; factors from axis-ratios clamped inside via `scaling::fit_width_factor`/`fit_height_factor`; `viewport_rect()` intake; window resize does NOT re-fit; `fit_button.rs` rename; `fit_one_shot_tests`/`fit_factor_tests` replace the `fit_mode_tests` files); Task 8 dev-checks `ALL CHECKS PASSED` (104 unit tests in 18 files), exe rebuilt 19,593,177 B, DLL import audit unchanged (22 OBSERVED DLLs); Task 9 wave documentation landed (this step). The Wave-4.1 OPERATOR-RUN (Next-Step #1) is the wave's pending acceptance gate; the Task-8 OPERATOR-RUN sub-item stays `[ ]` until the user reports.
```

- C-3b: release-artifact bullet: FROM `- Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,535,645 bytes, 0.4.0 Wave-4 rebuild);` TO `- Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,593,177 bytes, 0.4.1 Wave-4.1 rebuild);` (the remainder of that bullet stays byte-identical).
- C-3c: TODO-state bullet: the executor PREPENDS, before the bullet's `Wave TODO files ready:` sentence — exactly: `.agent/todos/20261102/20261102-todo-1.md` (Wave 4.1) — Tasks 1–8 marked `[DONE]`; Task 9 gains its `[DONE]` at this workflow's 4.6; the Task-8 OPERATOR-RUN sub-item stays `[ ]` until the user reports.` (as a normal inline sentence inside that same bullet; every other word of the bullet stays byte-identical).
- C-3d: git bullet: FROM `- Git: `main` == `origin/main` == `19aa9a6`; working tree clean; no other branches (`feat/wave-4-fullscreen-zoom` deleted after the verified merge).` TO `- Git: `main` == `origin/main` == `19aa9a6`; the Wave-4.1 wave lives on `fix/wave-4-operator-fixes` on top of it (NOT merged — merge/rename/push are the Workflow-Step-5 mechanics after Task 9's mark lands).`

**C-4** · Immediate Next Steps:
- DELETE the stale first item (the line starting `1. **Task 4 — stray zoom message removal**` and ending `...always-maximized launch, ESC exit, symmetric zoom range, and one-shot fit semantics).`) — its content is superseded by C-1/C-2.
- DELETE the whole current Wave-4 gate block: its intro line (`1. **Wave-4 acceptance gate — user OPERATOR-RUN** ... the freshly rebuilt 0.4.0 exe...`), the fenced 15-item 0.4.0 checklist, and its trailing sentence (`The Task-4 OPERATOR-RUN sub-item stays `[ ]` ... closes the wave's manual-UX gate.`).
- INSERT in their place the Wave-4.1 gate of §4 (framing + verbatim 15-item fence), numbered with the file's house style `1. ` (it becomes the new #1).
- REPLACE item #2's paragraph with — exactly: `2. **Wave 5 (Pan and Navigate at Image Edges While Zoomed) runs after this wave's acceptance in a NEW chat/session** (TODO `20261029-todo-7.md`); the Wave-4 draw-size composition is the pan anchor. **Parked future ideas stay OUT of scope**: the parked list (drag & drop onto the app; fullscreen bottom-hover reveal; normal-mode auto-hide chrome; the Wave-5 pan viewport-edge/opposite-start refinement; per-folder session zoom memory; the subfolder picker) is recorded in [`.agent/project-info/future-ideas.md`](future-ideas.md)` until scheduled (NO scope change).`
- KEEP item #3 byte-identical (`3. **Open item (flagged, not a TODO)**: broader line-ending renormalization beyond `*.sh` ...`).

### 3.5 `.agent/project-info/brief.md` (ONE insertion; everything else byte-identical)

Insert immediately BEFORE the `<!-- DO NOT DELETE NEXT SECTION -->` comment (i.e., after `## Out of Scope for v1 (Deferred)`), exactly:

```markdown
## Addendum — Operator Corrections (2026-11-02, BINDING)

The 2026-11-02 operator chat corrections supersede the contradicting sentences of this brief wherever they appear (this addendum governs); everything not contradicted below stays byte-identical. Source: [`.agent/todos/20261102/20261102-todo-1.md`](../todos/20261102/20261102-todo-1.md) (Wave 4.1).

1. **Launch window state** (supersedes the `### Open Image with System-Registered Formats` sentence "The image opens in a regular resizable viewer window consistent with the classic viewer experience (not maximized, not a modal dialog)" AND the `### Fullscreen Mode` sentence "Default fullscreen state at launch: OFF — the window opens normal or maximized at the user's own placement/size"): the application window opens **ALWAYS MAXIMIZED at EVERY launch** (never auto-fullscreened; the fullscreen default stays OFF), and while the release build runs, **no console window is visible**. The viewing window remains a regular RESIZABLE window — the maximized state is just its launch size.
2. **Zoom range is symmetric** (supersedes any reading of `### Zoom In/Out and Fit Modes` that blocks zoom-out at whole-image fit): zoom-out is NOT blocked at the neutral fit level; the zoom-out lower limit is SYMMETRIC to the zoom-in upper limit (same magnitude reversed — when the zoom-in max is ×8, the zoom-out floor is ÷8 of the fit size).
3. **Zoom/fit actions are silent** (supersedes the Wave-4 message scheme): the bound-step and fit-activation messages are removed — zoom/fit actions do not announce anything (the only app message remains the unsupported-file failure text).
4. **Fit width/height are ONE-SHOT size sets** (supersedes fit-mode persistence in `### Zoom In/Out and Fit Modes`): pressing `W`/`H` or their buttons ONLY sets the current zoom level so the current image's width/height fills the window — nothing else; normal zoom in/out works from that level afterward (no persistent mode may block zoom-out).
5. **ESC exits fullscreen** (additional to F11; the exact interplay was decided in the Wave-4.1 architector step): ESC exits fullscreen ON; otherwise ESC does nothing.
6. **Affordance add-on** (new requirement from the 2026-11-02 operator chat): each keyboard-distinct UI button displays a small reference of its related hotkey beneath it, and hovering any button shows a short tooltip of what it does.
```

### 3.6 `.agent/project-info/product.md` (ONE insertion; everything else byte-identical)

Append at the END of the file (after `## Explicit Non-Goals (Deferred)`), exactly:

```markdown
## Addendum — Operator Corrections (2026-11-02, BINDING)

User-facing corrections superseding the contradicting sentences of this product overview; source: the binding addendum in [brief.md](brief.md) (Wave 4.1 — [`.agent/todos/20261102/20261102-todo-1.md`](todos/20261102/20261102-todo-1.md)).

1. **Launch window state**: the window opens **ALWAYS MAXIMIZED at EVERY launch** (never auto-fullscreened; fullscreen default OFF; the viewing window stays a regular resizable window), and the release build shows **no console window**.
2. **Zoom range symmetric**: zoom-out is NOT blocked at the neutral fit level and keeps shrinking to the symmetric floor (÷8 of fit when the ceiling is ×8).
3. **Zoom/fit actions silent**: no bound-step or fit-activation messages (the only app message is the unsupported-file failure text).
4. **Fit width/height are one-shot size sets**: `W`/`H` or their buttons set the zoom level so width/height fills the window — zoom continues from that level (no persistent mode, nothing blocked).
5. **ESC exits fullscreen** while ON (F11 unchanged; ESC a no-op otherwise).
6. **Affordance add-on**: each keyboard-distinct UI button shows a small hotkey reference beneath it; every button shows a hover tooltip.
```

### 3.7 `.agent/project-structure.md` (5 edits — the rename/test records per the current `git ls-files src` tree)

**S-1** · `app.rs` line: FROM `- app.rs - eframe application shell: viewer window options,` TO `- app.rs - eframe application shell: viewer window options (always maximized at launch),` — only this phrase; everything else byte-identical.

**S-2** · `main.rs` line: FROM `- main.rs - entry point: CLI image-path intake, module registration, starts the eframe native app` TO `- main.rs - entry point: CLI image-path intake, module registration, starts the eframe native app with the console hidden in release builds`.

**S-3** · `scaling.rs` line: FROM `- scaling.rs - pure fit-scaling math: largest uniform factor for the available area (no egui painting)` TO `- scaling.rs - pure fit-scaling math: largest uniform factor for the available area, plus the Wave-4.1 one-shot fit-width/fit-height factors (axis-ratio over the whole-image fit, clamped into the symmetric zoom bounds; no egui painting)`.

**S-4** · `zoom.rs` line: FROM `- zoom.rs - pure zoom math: the ×1.25 step clamped to the [1.0, 8.0] bounds, at-bound predicates, the restore-to-fit constant, and the fit-size × factor draw-size composition (with its inline unit tests)` TO `- zoom.rs - pure zoom math: the ×1.25 step clamped to the SYMMETRIC [1/8, 8] bounds (floor = 1.0/ceiling; the ÷8 floor lets zoom-out pass fit), at-bound predicates, the restore-to-fit constant, and the fit-size × factor draw-size composition (with its inline unit tests)`.

**S-5** · `app/` enumeration line, four sub-edits:
- FROM `keyboard_intake.rs (the one per-frame keyboard intake: F11, zoom I/O/0 chords, fit W/H, the four arrows)` TO `keyboard_intake.rs (the one per-frame keyboard intake: F11, ESC-exit-fullscreen, zoom I/O/0 with the strict shift aliases, one-shot fit W/H, the four arrows)`.
- FROM `zoom.rs (zoom/fit-mode app seam: factor steps and bounds, set_fit_mode with mode-relative reset and activation messages)` TO `zoom.rs (zoom app seam: symmetric-bound factor steps with silent clamping, the strict shift-alias predicate, the at-bound/off-fit dimming predicates, and the one-shot set_zoom_to_fit_width/fit_height seams)`.
- FROM `fit_mode_button.rs (the two fit pill buttons gated on an open image)` TO `fit_button.rs (the two one-shot fit pill buttons gated on an open image; renamed from fit_mode_button in Wave 4.1 Task 7)`.
- FROM the test-modules tail `..., zoom_tests.rs, fit_mode_tests.rs` TO `..., zoom_tests.rs, fit_one_shot_tests.rs`.

## §4 Verbatim import — the 15-item Wave-4.1 OPERATOR-RUN checklist

The following block is byte-identical to [`.kilo/plans/20261102-wave-4-1-task8-verification-plan.md`](20261102-wave-4-1-task8-verification-plan.md) §11 — the executor copies it INCLUDING the fence lines into context.md as the new `#1` gate item, preceded by the framing paragraph and followed by the closing sentence, exactly as shown (this is the final assembled region; the indented presentation below exists only so the inner fences survive in THIS plan file):

    1. **Wave-4.1 acceptance gate — user OPERATOR-RUN** on a real Windows 10/11 machine, running `dist\windows-x64\img-scroller.exe` (the freshly rebuilt 0.4.1 exe). The verbatim 15-item checklist is recorded in [`.kilo/plans/20261102-wave-4-1-task8-verification-plan.md`](../../.kilo/plans/20261102-wave-4-1-task8-verification-plan.md) §11 and copied here byte-for-byte:

    ```
    Wave-4.1 manual check (OPERATOR-RUN) — run dist\windows-x64\img-scroller.exe (the freshly rebuilt 0.4.1 exe)
    on a real Windows 10/11 machine. Prepare: one folder holding differently shaped images — a WIDE/panoramic
    image, a TALL portrait image, and a roughly square image — plus sibling folders beside it (some with images,
    one WITHOUT images) so horizontal and vertical navigation both work. (Task-4 re-test items 3.0-below supersede
    the Wave-4 checklist: launch state, console hiding, captions/tooltips, F11/ESC tuning, symmetric zoom range,
    strict aliases, and one-shot fit are the NEW expectations; the silent-behavior lines replace the old
    `Zoom max`/`Zoom min`/`Fit width`/`Fit height` messages.)

    1. LAUNCH STATE: launch the exe — the window opens MAXIMIZED at every launch (never auto-fullscreened;
       fullscreen default is OFF). Open an image first when the app launched without one: use the big centered
       Open… button of the fresh empty state.
    2. CONSOLE HIDDEN: while the release exe runs, NO console/cmd window is visible behind or beside the app.
    3. OPEN & PILL: open a normal photo — whole-image fit, centered; the bottom pill shows the row
       [←][→][↑][↓][fullscreen square][+][−][restore-fit square][fit-width glyph][fit-height glyph].
    4. CAPTIONS + TOOLTIPS: under the SIX keyboard-distinct buttons the small 9 pt hotkey captions render —
       "F11" (fullscreen), "I" (+), "O" (−), "0" (restore), "W" (fit-width), "H" (fit-height); the four arrow
       buttons carry NO caption. Hover EVERY one of the 10 buttons — each shows its tooltip
       ("Previous image (Left Arrow)", "Next image (Right Arrow)", "Previous sibling folder (Up Arrow)",
       "Next sibling folder (Down Arrow)", "Fullscreen (F11)", "Zoom in (I)", "Zoom out (O)", "Restore fit (0)",
       "Fit width (W)", "Fit height (H)"); tooltips also appear over DISABLED buttons.
    5. FULLSCREEN ON: press F11 — immediate, no lag class to watch: title bar/borders AND the pill (every
       control) disappear; ONLY the image shows.
    6. FULLSCREEN OFF: press F11 again — the standard chrome returns exactly (pill back; image central again).
    7. ESC EXIT: while fullscreen ON, press ESC — fullscreen exits immediately (same effect as F11). While
       fullscreen OFF, ESC does nothing visible.
    8. ZOOM IN: press I (or click [+], or Shift+'+') — the image grows ×1.25 per step; the CENTER of the zoomed
       image stays visible (no pan yet — Wave 5); NO message appears.
    9. ZOOM CEILING: zoom in until two more I steps change nothing — the ceiling is ×8 and further steps are
       SILENT (no `Zoom max` message, ever); the [+] button paints dimmed at the ceiling.
    10. ZOOM OUT BELOW FIT: press O (or click [−], or Shift+'-') — the image shrinks ×1.25 per step, PASSES the
        original whole-image fit level and keeps shrinking to the symmetric floor ≈ ÷8 of fit; steps past the
        floor change nothing SILENTLY (no `Zoom min` message, ever); the [−] button paints dimmed at the floor;
        zoom-out is NOT blocked AT the fit level itself.
    11. ALIASES STRICT: Shift+'+' zooms IN; Shift+'-' zooms OUT; pressing plain '=' , '+' or '-' WITHOUT Shift
        does NOTHING (all three inert).
    12. RESTORE: press 0 (or click the restore-fit square) — always returns exactly to the whole-image fit
        level ×1.0 silently, from any factor including the boundaries above.
    13. ONE-SHOT W/H: with the TALL portrait open, press W (or the fit-width button) — the image WIDTH spans the
        window (clipped top/bottom); afterward I/O zoom IN AND OUT normally FROM that level (nothing blocked);
        pressing W AGAIN on the same window recomputes the same span (no surprise jump); RESIZE the window —
        the image scales with the window per its stored factor and does NOT re-snap to full-image fit. The H
        analog with the WIDE panoramic: HEIGHT spans the window; zoom works from it the same way.
    14. FIT BUTTONS WITHOUT IMAGE: arrow up/down into a sibling folder WITHOUT images (the `Empty folder`
        surface): the two fit buttons are DIMMED and non-clicking there (they stay hover-tooltip-able); the pill
        remains visible for navigation.
    15. FULLSCREEN INTERPLAY + RESET & REGRESSION: zoom to about ×2, press F11 (ON) — the zoom level persists
        and I/O/0/W/H keep working while ON; F11 (OFF) — the SAME factor persists (fullscreen toggles never
        touch zoom). Then arrow-navigate to the NEXT image (or ↑/↓ to a sibling) — the new image opens at
        whole-image fit ×1.0 silently (every successful open resets the factor). ←/→ keys and pill buttons still
        walk the folder's images as in 0.4.0; ↑/↓ still walk sibling folders (last-viewed memory and first-image
        rules unchanged); list edges stay silent and their buttons disable. Through ALL zoom/fit actions of this
        run the pill's message line stays EMPTY — the ONLY message that may ever appear is the unsupported-file
        failure text on a failed open (unchanged by design).
    Report results back into the project (this closes the Task-8 manual-UX sub-item).
    ```

    The Task-8 OPERATOR-RUN sub-item stays `[ ]` in the TODO until the user reports; a PASS closes the wave's manual-UX gate.

In the ACTUAL context.md the block is NOT indented — it renders as normal list content: the `1. ` intro paragraph, the blank line, the ``` fence, the checklist, the closing fence, the blank line, and the closing gate sentence. The executor's fidelity check: the fence content must equal the source plan §11 block minus ITS two fence lines, byte-for-byte (spot-grep items `1.`, `7.`, `13.` start-phrases in §5's G-8).

## §5 Verification & consistency gates (run by the docs executor before committing; re-run by the code-reviewer at 4.3 and the architector at 4.5b)

- **G-1 version**: EVERY new/edited line asserting a CURRENT version states `0.4.1`; `0.4.0` survives ONLY inside dated-history sentences (the Wave-4-landed fragments) and inside the verbatim §4 fence (item 15's "as in 0.4.0" regression clause). Any other survivor is a defect.
- **G-2 census**: `104` stated in README verification + tech.md Status + tech.md Testing intro + architecture.md Status; the per-file numbers in tech.md's census bullet sum to 104 by hand and equal §0's table exactly.
- **G-3 exe size**: `19,593,177` replaces `19,535,645` in README (verification sentence), tech.md, architecture.md, and context.md's release-artifact bullet. The old number survives ONLY in dated-history bullets (context.md's Wave-4 landed bullets — verified all present there and left untouched).
- **G-4 no-contradiction sweep** (grep the final docs for `FitMode`, `fit_mode`, `set_fit_mode`, `Zoom max`, `Zoom min`, `fit_mode_button`, `fit_mode_tests`, `F11 is the only exit`, `not maximized at launch`, `opens normal or maximized`, `[1.0, 8.0]`, `fit-mode coupling`): expected survivors are ONLY (a) dated-history Wave-4 fragments explicitly marked as 0.4.0 (including the R-5c "F11 its only exit at 0.4.0" dating), and (b) the §4 checklist fence (verbatim import). Every other hit is a defect to fix.
- **G-5 landed-name anchoring**: the changed docs reference exactly `src/app/button_hint.rs`, `fit_button.rs`, `fit_one_shot_tests.rs`, `fit_factor_tests.rs`, `set_zoom_to_fit_width`/`set_zoom_to_fit_height`, `viewport_rect()`, `ZOOM_FACTOR_MINIMUM`, `windows_subsystem`, `.with_maximized(true)`.
- **G-6 addendum shape**: brief.md and product.md each contain EXACTLY ONE new `## Addendum — Operator Corrections (2026-11-02, BINDING)` section at the §3.5/§3.6 slots; `git diff` on both files must show INSERTIONS ONLY.
- **G-7 bans**: `CHANGELOG.md` untouched; no existing `.kilo/plans/*` modified; the only plan-addition is this Task-9 plan.
- **G-8 checklist fidelity**: the context.md fence content equals the Task-8 plan §11 block byte-for-byte (visual line-by-line check + the §4 spot-greps).
- **G-9 no invented behavior**: every behavioral sentence in the docs must map to a §0 verified pin (silent messages; symmetric floor; strict aliases; ESC-under-ON-only; one-shot set; resize-no-refit; always-maximized; release-only console; captions/tooltips; dimming predicates).

## §6 Git plan (branch `fix/wave-4-operator-fixes`; push FORBIDDEN — workflow Step 5)

| When | Commit | Content (staging list; verify via `git status --short` before each commit; explicit paths only) | Message (EXACT) |
| --- | --- | --- | --- |
| 4.2-equivalent start, unconditional | C1 | this plan file only | `docs: add wave4.1 task9 documentation plan` |
| 4.2-equivalent completion | C2 — the ONE wave-docs commit | `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`, `.agent/project-info/brief.md`, `.agent/project-info/product.md`, `.agent/project-structure.md` — ALL SEVEN staged in ONE commit (decision: one commit; the brief/product corrections are not separable from the wave-reference update without a mid-state contradiction) | `docs: land wave-4.1 operator-fixes documentation (README/tech/architecture/context/structure/brief/product)` |
| 4.3 fixes, ONLY if the reviewer finds doc defects | C3 (conditional; ≤ 2 review cycles total) | only the doc files the fix touches | `docs: finalize wave-4.1 task9 documentation coherence` (Wave-3 precedent `8e30582`) |
| 4.6 (separate mark step, §7) | C4 | the TODO file only | `task: mark wave-4.1 task9 documentation done` |

- NO `git push` (restricted to workflow Step 5, `main` → `origin` ONLY; the branch is NOT merged in this task). NO amend/force. NO `git add -A`/`git add .`. NO branch creation/switch (Step-2 territory). NO merge; NO TODO-file `-DONE` rename (Step-5).
- Every staged list is verified with `git status --short` immediately before its commit and MUST equal the row's content; `.gitignore` is re-read before staging (gitignore-compliance rule) — `dist/**`, `logs/**`, `target/**`, `*.log`, `Cargo.*` are never stageable.

## §7 4.6 mark semantics (a SEPARATE sub-task strictly after adherence passes; NEVER inside the docs step)

1. `git branch --show-current` → `fix/wave-4-operator-fixes`. In `.agent/todos/20261102/20261102-todo-1.md` make EXACTLY these edits inside `### Task 9: Documentation updates`: the heading gains ` [DONE]` (→ `### Task 9: Documentation updates [DONE]`); BOTH sub-items flip `- [ ]` → `- [x]` (the README/tech/... line and the brief/product line). EVERY other byte of the file is PRESERVED (overwrite-prevention rule) — Task 8's unchecked OPERATOR-RUN box and all other tasks/sections stay untouched, and the Future-session-execution-notes section is not edited.
2. Stage exactly `.agent/todos/20261102/20261102-todo-1.md`; commit `task: mark wave-4.1 task9 documentation done`.

## §8 Deliberately out of scope (what the executor must NOT do)

- NO application code, tests, harness scripts, build-guide edits, or Cargo changes (verification is the Task-8 record's business).
- NO changes to `future-ideas.md`, `instructions.md`, other TODO files, rule/workflow files, `.opencode/*`, `.kilo/rules/*`, `LICENSE`.
- NO new features restated as planned/landed (Wave-5 pan, drag & drop, pill auto-hide, per-folder zoom memory, subfolder picker, "Open with...", packaging) — any mention stays record-only in its current planned/parked form.
- NO CHANGELOG entries. NO edits to ANY existing `.kilo/plans/*` (including this file after its C1 commit). NO push/merge/TODO-rename work (Step 5).
- NO context.md edits beyond §3.4's scoped list (the `How to Use This File` block, historical Recent-Changes entries, and unaffected Current-Project-State bullets are never reordered or reworded).
- If any §3 FROM anchor is not found verbatim, or any doc fact contradicts a §0 pin, STOP and return the question to the caller — never improvise a semantics change.

## §9 Executor checklist for the 4.2-equivalent docs step (junior-safe order)

1. Read this plan fully; read the seven target docs so every anchor is pre-armed; re-read `.gitignore`; `git status --porcelain` must show (at most) ONLY this plan file as untracked/plan-stageable — anything else unexplained → STOP and ask the caller.
2. Commit C1 (§6) if it has not landed yet.
3. Edit `README.md` (§3.1, R-1 → R-5 in order).
4. Edit `tech.md` (§3.2, T-1 → T-4 in order; T-3 is the whole-body section rewrite with the heading KEPT; T-4 replaces exactly the one census bullet).
5. Edit `architecture.md` (§3.3, A-1 → A-7 in order; A-8 is a verify-only grep).
6. Edit `context.md` (§3.4, C-1 → C-4 in order; the §4 checklist block is transplanted inside C-4; then re-read `## Immediate Next Steps` and confirm the numbering symmetry `1.`-gate → `2.`-Wave 5 → `3.`-line-endings).
7. Insert the brief.md addendum (§3.5) and the product.md addendum (§3.6) — verify via `git diff` that both files show INSERTIONS ONLY.
8. Edit `.agent/project-structure.md` (§3.7, S-1 → S-5).
9. Run the §5 gates G-1 → G-9 (grep sweeps + hand-sum census + byte-diff checks).
10. Stage the SEVEN files; verify the staged list equals C2's content row; commit C2.
11. Report to the caller: the commit sha, the seven files changed, the §5 gate results, and the explicit note that NO push/merge happened (Step-5 territory).
