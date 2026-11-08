# Global Plan — Session 2026-11-07: Wave 4.2 (Operator Feedback) then Wave 5 (Pan then Navigate)

Session request: process BOTH pending TODO files in the user-chosen order.

- Part A — Wave 4.2: `.agent/todos/20261107/20261107-todo-1.md` (created this session from the 2026-11-07 chat: acceptance recording + item-4 styling + item-11 Shift-alias defect + ESC double-press close feature). Branch `fix/wave-4-2-operator-feedback`, version 0.4.1 → 0.4.2 (patch, corrective-wave precedent of 0.2.1/0.4.1).
- Part B — Wave 5: `.agent/todos/20261029/20261029-todo-7.md` (pan then navigate at image edges while zoomed). Branch `feat/wave-5-pan-then-navigate`, version 0.4.2 → 0.5.0 (minor — feature wave precedent of 0.3.0/0.4.0).

Rules honored every step: Critical Workflow `.kilo/commands/critical-workflow.md`; sub-agent prompt template (single-discrete-step preamble) in EVERY `task` invocation; Alpine-VM harness (`vm_status` first; exit code authoritative; share `/img-scroller`; `docs/build.md`); Gitignore Compliance Rule; branch creation restricted to Step 2; `git push` restricted to Step 5 (main → origin ONLY, never other remotes); version update restricted to Step 3; no `CHANGELOG.md` edits in either wave; new plan files only (never edit existing `.kilo/plans/*`); max 3 review cycles per task then escalate; Markdown Generation Rule (plans/docs authored by planner or the assigned specialists).

Pre-analysis note (both parts): the 0.4.1 codebase is the base (`main` == `origin/main` == `88c45f7`, working tree clean). Key modules: `src/app/zoom.rs` (alias chords `zoom_in_shift_alias_was_pressed` / `zoom_out_shift_alias_was_pressed` — today `Key::Equals`|`Key::Plus` shift-gated in, `Key::Minus` shift-gated out), `src/app/keyboard_intake.rs` (fixed-order edge-trigger intake, `key_pressed` reads — repeats DO fire `key_pressed` per the Wave-4.1 F11 hold observation), `src/app/fullscreen.rs` (`escape_exits_fullscreen` = fullscreen_on && escape_pressed), `src/app/button_hint.rs` + `src/app/floating_controls.rs` (pill: painted vector shapes, six 9 pt captions, hover tooltips), `src/app_display.rs` (per-frame `fitted_size × zoom_factor` composition, center-anchored clipped paint — the Wave-5 pan anchor), `src/app/folder_navigation.rs` (vertical Wave-3 target rule referenced, not restated).

Approval: presented via `question` tool (the request/TODO files contain no "Don't request me to approve plans" string).

---

## Part A — Wave 4.2

### Step A.2 — Git Feature Branch Setup → implementer
- `git status` (clean expected → nothing to commit); verify on `main`; create + switch to `fix/wave-4-2-operator-feedback`.

### Step A.3 — Version Update → implementer
- `Cargo.toml` version = "0.4.2"; `Cargo.lock` stays consistent (`--locked` policy irrelevant to the manual version field edit; commit `chore: bump version to 0.4.2`).

### A-Task 1 — Record the Wave-4.1 acceptance + close the superseded Wave-4 gate
Front-end flag: NO (bookkeeping file edits only).
- 4.1b → architector (light plan: two checkbox marks exactly as the todo sub-items state, one commit; no code).
- 4.2 → implementer (marks `[] → [x]` ONLY on the two OPERATOR-RUN sub-items in `20261102-todo-1-DONE.md` and `20261029-todo-6-DONE.md`; preserve all other content; commit).
- 4.3 → code-reviewer + code-simplifier (expected: no-change report).
- 4.4 → docs-specialist returns "not required" (no documentation target).
- 4.5b → architector (adherence report).
- 4.6 → implementer marks A-Task 1 `[DONE]` in the Wave-4.2 todo + commit.

### A-Task 2 — Pill captions white + thin black border on icons and captions
Front-end flag: YES (pill visual styling) → A cycle: 4.1a THEN 4.1b.
- 4.1a → frontend-specialist: Front-end Technical Specification to `.kilo/plans/20261107-wave4-2-task2-pill-styling-frontend-spec.md`: (i) caption text color = WHITE (the icons' color family today), (ii) thin black outline on EVERY painted pill shape (4 arrows, `+` cross bars, `−` bar, fullscreen square, restore-fit square, fit line-plus-triangle glyphs), (iii) thin black FONT border around all caption glyphs — research the egui 0.37 idiomatic text-outline mechanism (painted offset black copies vs any stroke support), with the stroke/offset constants proposed; constraint: captions stay 9 pt and all captions/tooltips/enablement behavior unchanged.
- 4.1b → architector (plan from the spec, files: `src/app/floating_controls.rs`, `src/app/button_hint.rs` consumers, possibly the paint modules it routes through).
- 4.2 → implementer; 4.3 → reviewers; 4.4 → docs-specialist (module doc bullets only); 4.5a → frontend-specialist verification vs the 4.1a spec; 4.5b → architector adherence; 4.6 → mark + commit.

### A-Task 3 — Fix the strict zoom Shift-aliases (item 11 defect)
Front-end flag: NO (keyboard intake logic; Wave-4.1 Task-6 precedent had no 4.1a).
- 4.1b → architector: root-cause the real-Windows failure of `zoom_in_shift_alias_was_pressed` (`Key::Equals`/`Key::Plus`) and `zoom_out_shift_alias_was_pressed` (`Key::Minus`) against egui 0.37 / egui-winit Windows key translation (research: context7 egui docs; candidates — logical-key '+' gap on Windows, numpad `Key::NumpadAdd`/`Key::NumpadSubtract` absent from the current chord set, physical-key resolutions, `Event::Key` raw path). Decide the fix combination so Shift+'+' zooms in and Shift+'−' zooms out on real Windows WITHOUT regressing the strict rule (plain `+`/`=`/`-` inert) and WITHOUT messages; add synthetic-input unit tests reproducing each accepted resolution (reuse the existing egui-Context test harness pattern in `src/app/zoom_tests.rs`).
- 4.2 → implementer (edits scoped to `src/app/zoom.rs` + test files); 4.3 → reviewers; 4.4 → docs-specialist; 4.5b → architector; 4.6 → mark + commit.

### A-Task 4 — ESC double-press close while fullscreen OFF
Front-end flag: YES (new centered popup surface) → 4.1a THEN 4.1b.
- 4.1a → frontend-specialist: spec to `.kilo/plans/20261107-wave4-2-task4-esc-close-frontend-spec.md`: centered popup (plain central overlay, egui 0.37 mechanism `Area`/lower-layer route confirmed), message text exactly "Press ESC again to close", 2-second lifetime then self-dismiss, re-arm on later presses; the two-distinct-presses requirement (NOT held/autorepeat — research `egui::Event::Key` `repeat` flag or the pressed-released-pressed state machine; the `key_pressed`-fires-on-repeat fact is bound by the Wave-4.1 F11-observation); fullscreen-ON interplay (ESC exits fullscreen only, popup must not show or arm; if fullscreen toggles ON while the popup is visible it hides and the close disarms); close mechanism (`egui::ViewportCommand::Close` or the eframe 0.37 equivalent); popup visible in ALL fullscreen-OFF states (image open, empty-folder browsed, fresh empty state) and must not disturb the chrome.
- 4.1b → architector: plan — new module (e.g. `src/app/esc_close_confirm.rs`) + session RAM state (armed/expires-at), intake order wired so fullscreen-ON ESC consumption keeps priority, unit tests for the state machine (show → close-within-window → expiry → re-arm; repeat-filter; fullscreen interplay).
- 4.2 → implementer; 4.3 → reviewers; 4.4 → docs-specialist; 4.5a → frontend-specialist; 4.5b → architector; 4.6 → mark + commit.

### A-Task 5 — Verification
Front-end flag: NO.
- 4.1b → architector verification plan (unit-test census expectations per the three code tasks; release rebuild via `scripts/build-windows.sh`; DLL import audit = the recorded 22-DLL constant; OPERATOR-RUN checklist for 0.4.2 copying the manual-UX sub-items verbatim; container harness, `vm_status` first, exit-code evidence).
- 4.2 → implementer: dev checks `ALL CHECKS PASSED`; rebuild → non-empty `dist/windows-x64/img-scroller.exe`; audit; the OPERATOR-RUN sub-item stays `[ ]` until the user reports.
- 4.3 → reviewers (light); 4.4 → not required; 4.5b → architector; 4.6 → mark + commit.

### A-Task 6 — Documentation updates
Front-end flag: NO.
- 4.1b → architector docs plan; 4.2 → implementer executes docs work under the docs-specialist's file-authority — per the Markdown Generation Rule the docs-specialist WRITES: `README.md`, `tech.md`, `architecture.md`, `context.md` (styling + alias fix + ESC close behavior; context.md gains the session's acceptance-report entry + the pending 0.4.2 OPERATOR-RUN gate); `.agent/project-info/brief.md` addendum line 7 (ESC double-press close affordance, fullscreen-ON exception); `.agent/project-info/future-ideas.md` ESC-close idea closed as scheduled; 4.5b → architector coherence check; 4.6 → mark + commit.

### Step A.5 — TODO File Completion → implementer
- Rename `20261107-todo-1.md` → `20261107-todo-1-DONE.md` (content preserved); tmp-file sweep; all work committed; switch to `main`; merge `fix/wave-4-2-operator-feedback` (fast-forward expected); verify then delete the branch; push `main` → `origin` ONLY; notify on failure.

---

## Part B — Wave 5

### Step B.2 — Git Feature Branch Setup → implementer
- From the just-updated `main`: create + switch to `feat/wave-5-pan-then-navigate` (per the Wave-5 todo's branch instruction).

### Step B.3 — Version Update → implementer
- `Cargo.toml` version = "0.5.0"; commit `chore: bump version to 0.5.0`.

### B-Group — Tasks 1+2+3 combined cycle (pan then navigate, both axes) → marked "front-end related" (interaction + display interplay; Wave-4 interaction precedent)
GROUPING JUSTIFICATION (Critical Workflow Step 1.2 grouping clause): Tasks 1–3 are ONE per-axis overflow/edge controller — horizontal (Task 1), vertical (Task 2, Wave-3 target rule referenced), fits-entirely direct navigation (Task 3). Implementing them as three sequential cycles would churn the same new state machine three times; the cycle is shared, the 4.6 mark covers all three task headings.
- 4.1a → frontend-specialist: spec to `.kilo/plans/20261107-wave5-pan-navigate-frontend-spec.md`: while zoomed, arrows act on the OVERFLOWING axis only; pan increments move the visible window; navigation triggers ONLY at the image's left/right (wider-than-window) or top/bottom (taller-than-window) edge; a non-overflowing axis keeps direct navigation; both zoomed axes may overflow together (per-axis independence); pan state is session-RAM in the app struct, reset semantics decided (todo-deferred: reset-on-new-open/restore — architector decides consistently with the fit-reset rule: reset on EVERY successful open and on restore/`0`); interaction with fullscreen-ON (pan must work while ON) and with the A-T4 popup (no interplay).
- 4.1b → architector: plan — pure math in `src/zoom.rs` or a new pure module (overflow per axis, clamped pan offset, at-edge predicate per direction), app seam (new `src/app/pan.rs` or widened keyboard_intake routing: arrow reads stop navigating directly and route through the pan-or-navigate controller), `src/app_display.rs` paint composition consumes the offset (draw position = `fitted_size × factor`, non-centered translation), deferred-decision pickups from the Wave-5 todo (pan step size; auto-repeat at edges; mouse pan/ OUT-OF-SCOPE note; the "fit modes suppress panning" item recorded MOOT — `FitMode` no longer exists post-Wave-4.1); navigation targets reuse `NavigationDirection` unchanged; unit tests for the state machine and per-axis routing.
- 4.2 → implementer; 4.3 → reviewers; 4.4 → docs-specialist; 4.5a → frontend-specialist vs the spec; 4.5b → architector; 4.6 → implementer marks B-Tasks 1, 2, 3 `[DONE]` (all sub-items) + commit.

### B-Task 4 — Verification
Front-end flag: NO.
- 4.1b → architector verification plan (completed-state machine tests via the harness; exe rebuild; DLL audit = 22-DLL constant; OPERATOR-RUN checklist verbatim: zoom wide → keys pan, navigation only at edge; zoom tall → same vertically; fits-entirely/small image → direct navigation).
- 4.2 → implementer (dev checks `ALL CHECKS PASSED`; rebuild `dist/windows-x64/img-scroller.exe` 0.5.0; audit; OPERATOR-RUN sub-item stays `[ ]` until the user reports).
- 4.3 → reviewers (light); 4.4 → not required; 4.5b → architector; 4.6 → mark + commit.

### B-Task 5 — Documentation updates
Front-end flag: NO.
- 4.1b → architector docs plan; 4.2 executed by docs-specialist: `README.md`, `tech.md`, `architecture.md`, `context.md` — record the pan/navigate controller, the deferred-decision outcomes, the remaining open items, and the pending 0.5.0 OPERATOR-RUN gate; 4.5b → architector; 4.6 → mark + commit.

### Step B.5 — TODO File Completion → implementer
- Rename `20261029-todo-7.md` → `20261029-todo-7-DONE.md` (content preserved; renames after B-Task 5's 4.6); tmp sweep; commit; switch to `main`; merge `feat/wave-5-pan-then-navigate`; verify then delete; push `main` → `origin` ONLY; notify on failure.

---

## Session-Closing

- `context.md` final update (docs-specialist within B-Task 5's write: session state, final merged refs, next steps = the pending 0.5.0 + the already-pending 0.4.2 OPERATOR-RUN gates; Wave 6 = todo-8 next).
- Workflow Step 6 finish message: short resume + the next-session prompt block (per Critical Workflow: the first unfinished TODO = Wave 6 `.agent/todos/20261029/20261029-todo-8.md` after both waves complete).

## Error handling (both parts)

- On sub-agent failure/empty response: resume the same step (fresh `task` to the same subagent_type with the context + "continue" instruction), or split smaller; on repeated failure escalate to the user.
- On any harness/VMS share unavailability (`vm_status` fail, `/img-scroller` missing): STOP and return a user question (bound by the Wave-5 todo execution notes).
- On 4.1a/4.1b/review plan rejection by the user: re-do and re-present (approval required every time).
