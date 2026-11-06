# Wave 4.1 — Operator Fixes and Behavior Corrections: Global Plan

Date: 2026-11-02 · Source TODO: `.agent/todos/20261102/20261102-todo-1.md` (created this session) · Base: `main` @ `4c849fa` (0.4.0, Wave 4 merged; wave-4 acceptance reopened by the operator report)
Related commits already on `main`: `2f2c5c2` (wave-4 log), `381d191`/`4c849fa` (ideas + report records; pushed at Step 5 of this wave together with the new commits).

## Session Inputs (user operator report, 2026-11-02)

1. **0.4.0 OPERATOR-RUN: issues found** — items 1,3,4,5,6,7,10 of the user message became Wave-4.1 tasks (the TODO file); items 2,4-hover,8,9,11 were PARKED in `future-ideas.md` (Ideas 2–6) and are HARD OUT of this wave.
2. The wave-4 TODO (todo-6-DONE) Task-4 OPERATOR-RUN sub-item stays UNCHECKED until Wave 4.1 fixes re-test on a real Windows machine.
3. User did not request auto-approval of plans → present the plan via `question`.

## Global Pre-Analysis

- **Baseline (0.4.0)**: launch honors user placement (`with_maximized(false)`, no auto-fullscreen); `fullscreen_on` + F11 + fifth pill button; zoom `zoom_factor` clamped [1.0, 8.0] (min = fit — the correction REMOVES the lower bound to a symmetric 1/8 floor), step ×1.25, reset-on-every-open, keys I/O/0 + shift aliases (D4 double-resolution, now SUPERSEDED by strict-shift), buttons [+][−][restore-fit]; FitMode state machine (FitVisible/FitWidth/FitHeight, mode×factor composition, W/H set mode + reset factor, active-mode message) — the correction REPLACES it with one-shot size-set semantics; pill is 10 buttons with painted glyphs; console window appears on `cargo run`-built exe (console subsystem binary).
- **Technical anchors (architector verifies against egui/eframe 0.37 docs before encoding)**:
  - Console: `#![windows_subsystem = "windows"]` attribute on `src/main.rs` (release-only via `cfg_attr`? — decided in 4.1b; dev containers keep cargo output in logs anyway since the app runs in the container headless).
  - Maximize: `NativeOptions` maximized=true at startup (`ViewportBuilder::maximized` / `with_maximized(true)`) — the landed `with_maximized(false)` literal flips; verify eframe 0.37 exact member via context7.
  - Tooltips: `egui::Response::on_hover_text` (verify member); hotkey references = small painted/labelled text under each button inside the pill row layout.
  - F11 immediacy: read the intake block + viewport command send; the likely hazards are intake ordering after the zoom chords' reads or a stale-flag frame; ESC = additional `Key::Escape` intake only when fullscreen ON.
  - Stray "zoom in" message: inspect the message-channel writers (`announce_fit_mode`, zoom rejected-step helpers, unsupported constant); the operator paraphrase tolerance is noted — hunt for any message containing "zoom" that fires on non-rejected steps.
  - Symmetric zoom bounds: constants stay [8.0 upper]; add `ZOOM_FACTOR_MINIMUM = 1.0 / ZOOM_FACTOR_MAXIMUM` and adjust predicates, enablement rows, message wording comments; `0` restore returns to 1.0 (the `0` semantic stays "no-zoom baseline" — user: "zoom at 0 means no zoom").
  - Alias strictness: remove the D4 dual-resolution; require `modifiers.shift` true for `Key::Plus`/`Key::Minus`; plain Equals/Minus reject; keep I/O/0. Note the operator layout caveat in the wave file's Task-6 note (operator re-verification needed).
  - Fit one-shot: W/H/buttons compute a NEW factor from the CURRENT image + available area (`fit_width_size`/`fit_height_size` math becomes a factor computation: target_size / fitted whole-image size) and SET `zoom_factor` directly; the `FitMode` enum, mode-composition in `app_display`, the `image_is_open`-mode gating subtleties, and the `Fit width`/`Fit height` ANNOUNCES are removed or re-pointed per the architector's encoding; per-frame recompute of the SET value is NOT automatic after the set (the set is one-shot at press time — user intent: "nothing more"), window resize after a W-set keeps the user's factor (recorded as the architector decision, operator-testable).
- **Version**: 0.4.0 → 0.4.1 (patch — fix wave), Step 3.
- **Harness**: all Rust/Cargo inside the container (alpine-vm MCP, `vm_status` first, exit code authoritative); dev-checks → `ALL CHECKS PASSED`; rebuild exe via `scripts/build-windows.sh`; DLL audit SET-EQUAL to the 22-DLL constant.
- **Binding constraints**: branch creation restricted to Step 2 (`fix/wave-4-operator-fixes`); version bump restricted to Step 3; push restricted to Step 5 (`origin` only); no `CHANGELOG.md`; NO edits to existing `.kilo/plans/*` (new files only); TODO marks preserve content; code rules (≤200-line files, ≤50-line methods, ≤2 args, depth ≤2, private-by-default, single-section booleans, no commented-out code); markdown doc edits executed by docs-specialist; parked items stay untouched.

## Execution Order (each step = ONE `task` invocation)

- **Step 2 — Git Feature Branch Setup** ⇒ `implementer`: commit the untracked new TODO `.agent/todos/20261102/20261102-todo-1.md` first (`docs: add wave 4.1 operator fixes todo`), then create + switch to `fix/wave-4-operator-fixes` from `main`.
- **Step 3 — Version Update** ⇒ `implementer`: `Cargo.toml`/`Cargo.lock` 0.4.0 → 0.4.1 inside the container; commit `chore: bump version to 0.4.1`.
- **Task 1 — Launch window state** (front-end related):
  - 4.1a ⇒ `frontend-specialist`: spec = always-maximized launch + console hiding + their egui/eframe mechanisms + interrupted-contract notes (window placement respect superseded) + 4.5a checklist.
  - 4.1b ⇒ `architector` → new plan; 4.2 ⇒ `implementer`; 4.3 ⇒ `code-reviewer` + `code-simplifier` (concurrent) then fixes ⇒ `implementer`; 4.4 ⇒ `docs-specialist`; 4.5a ⇒ `frontend-specialist`; 4.5b ⇒ `architector`; 4.6 ⇒ `implementer`.
- **Task 2 — Button hotkey references + tooltips** (front-end related): same cycle; UI copy set decided at 4.1a proposals → user question if placement conflicts arise, else architector encodes within the pill layout.
- **Task 3 — Fullscreen tuning (F11 immediacy + ESC)** (front-end related): same cycle; 4.1a documents the intake-order investigation; 4.1b encodes ESC + any intake re-order fix.
- **Task 4 — Stray zoom message removal** (front-end related, small): same cycle; investigation-first plan (grep seams → fix → test).
- **Task 5 — Symmetric zoom range** (front-end related): same cycle; pure-math change + predicate/enablement/message updates + tests.
- **Task 6 — Alias strict-shift correction** (front-end related): same cycle; intake-only change + pin tests (plain keys reject; shifted aliases pass on both logical key resolutions caused by shifted `=`? — architector encodes the strict rule with the operator-layout caveat).
- **Task 7 — One-shot fit semantics** (front-end related, largest): same cycle; 4.1a re-specs the fit UX (no mode state), 4.1b plans the seam removals (FitMode enum, app_display composition, zoom-button gating, messages) + new factor math + tests.
- **Task 8 — Verification**: 4.1b-only plan ⇒ 4.2 implementer: dev-checks, test census, rebuild exe, DLL audit, assembled OPERATOR-RUN checklist into the context log; OPERATOR-RUN sub-item stays unchecked pending user.
- **Task 9 — Documentation**: 4.1b-only plan ⇒ docs-specialist executes (markdown rule): README/tech/architecture/context + brief/product correction addenda.
- **Step 5 — TODO File Completion** ⇒ `implementer`: rename to `20261102-todo-1-DONE.md` (content untouched), tmp-file check, ensure committed, switch `main`, ff-merge branch, delete branch, push `main` → `origin` ONLY.
- **Step 6 — Finish** ⇒ Planner: summary + next-TODO handoff text.
