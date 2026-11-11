# Wave 5 Task 5 — Documentation Plan 4.1b (README.md + tech.md + architecture.md + context.md → the 0.5.0 pan-then-navigate facts; verify-only structure/brief checks; two commits; NO harness work)

- Date: 2026-11-11 (plan author: architector, workflow step 4.1b; executed by the implementer under the docs-specialist file-authority in 4.2).
- Source TODO: [`.agent/todos/20261029/20261029-todo-7.md`](../../.agent/todos/20261029/20261029-todo-7.md) → `### Task 5: Documentation updates` ONLY (exactly its one sub-item: "`README.md`, `tech.md`, `architecture.md`, `context.md`: record the pan/navigate controller, remaining open items"). NO other task is expanded; the Task-5 `[DONE]` mark is the separate 4.6 step — NOT executed inside 4.2.
- Workflow slot: Wave-5 Critical Workflow (global plan [`.kilo/plans/20261107-wave42-then-wave5-global-plan.md`](20261107-wave42-then-wave5-global-plan.md) → B-Task 5, step 4.1b) — this PLAN → executed in 4.2.
- Docs-only task: NO `alpine-vm` calls, NO harness runs, NO code edits, NO `Cargo.*` touches. Host-side git + file edits only.
- Facts source: caller-verified evidence at branch `feat/wave-5-pan-then-navigate` HEAD `e0332fb` (B-Task-4 verification GREEN) + the B-Task-4 plan §0 pins ([`.kilo/plans/20261107-wave5-task4-verification-plan.md`](20261107-wave5-task4-verification-plan.md)).
- Approval: the user's global-plan approval makes every decision below BINDING; zero judgment left for the implementer. If any detail conflicts with something ACTUALLY observed during 4.2 → STOP and return the question to the caller (§9).
- Target implementer: JUNIOR under 50% restriction. Every edit is pinned with an exact anchor and exact replacement text; §8 lists the out-of-scope guards verbatim.

---

## §0 Session facts captured at plan time (caller-verified evidence at HEAD `e0332fb`)

- Branch `feat/wave-5-pan-then-navigate`; version **0.5.0** (bumped at Step B.3, commit `3e2e332`). Wave chain pinned up to `04f5cbd` (`task: mark wave5 tasks 1-3 done`), per B-Task-4 plan §0: `094980f` (wave-4.2 todo rename) → `3e2e332` (bump 0.5.0) → `ca8b65f` (spec+plan docs) → `272b98f` (`feat: pan then navigate at image edges while zoomed`) → `8058be8` (review fixes) → `1bfd0cc` (pan-navigate module docs) → `4324999` (app.rs relocation-note docs) → `04f5cbd`. The B-Task-4 tail (verification plan-file commit, any sanctioned fix commits, the Task-4 mark) is NOT pre-pinned — the implementer captures it VERBATIM at pre-flight (§5 step 1); the caller-provided HEAD is `e0332fb` (pre-flight mismatch → STOP).
- **B-Task-4 verification evidence (GREEN, caller-verified)**: dev-checks `ALL CHECKS PASSED` — **150 unit tests, 0 failed, across 23 test files**; exe rebuilt `dist/windows-x64/img-scroller.exe` = **19,606,124 bytes** (0.5.0); DLL import audit **SET-EQUAL to the recorded 22-DLL constant** (the list lives ONLY in `scripts/build-windows.sh` + `docs/build-windows.md` — the docs-split rule forbids duplicating it anywhere else).
- Wave-5 landed content (the facts the docs must record):
  - Pure math `src/pan.rs`: per-axis overflow = `max(0, zoomed_draw_size − available)`; symmetric [-limit, +limit] clamped offsets; at-edge predicates with `PAN_EDGE_TOLERANCE = 0.5`; pan step `PAN_STEP_FRACTION = 0.125` (a window-fraction pan step — 1/8 of the available axis span per press); the pure `arrow_outcome` pan-or-navigate decision.
  - App seam `src/app/pan.rs`: `ArrowKeyPress` (direction + repeat), `non_repeat_arrow_press_occurred` (the repeat-flag filter — the Wave-4.2 ESC precedent), `handle_arrow_key`, `reset_pan_offset`; per-axis routing: an overflowing axis pans on EVERY press (auto-repeats included) until its edge; at-edge navigation requires a DISTINCT non-repeat press (held-at-edge repeats are absorbed); a fits-entirely axis navigates directly on every press. The Navigate arm calls the shared seam VERBATIM (`pan::ArrowPanOutcome::Navigate => self.navigate(press.direction)` at `src/app/pan.rs:110`) — the Wave-3 folder-target rule is referenced unchanged.
  - Reset/re-clamp contracts: `pan_offset: egui::Vec2` (ONE session field carrying both per-axis components) is zeroed on EVERY successful open (`src/app.rs:95`, inside `accept_open_outcome`) and on restore-to-fit (`reset_pan_offset` at `src/app/pan.rs:65`, called from `restore_zoom_to_fit` at `src/app/zoom.rs:74` — both the `0` key and the pill's restore-fit button). NO clamp call exists in the zoom actions BY DESIGN — the offset re-clamps lazily ON USE: the paint re-clamps every frame against the current zoom-composed overflow (`src/app_display.rs:58–60`), and the next arrow write re-clamps at `src/app/pan.rs:126`. Zoom steps and one-shot W/H fit sets do NOT reset the offset (test-pinned).
  - Intake: `src/app/keyboard_intake.rs` routes the four arrows through `handle_arrow_key`, each fed `screen_context.viewport_rect().size()` (6 intake sites total: 2 fit + 4 arrows).
  - Paint: `src/app/central_surface.rs` passes the offset into `ImagePainting` (now the 3-field `{ decoded_image, zoom_factor, pan_offset }` request at `src/app/central_surface.rs:40`); `src/app_display.rs` composes the clamped pan offset into the paint via explicit `ui.put` repositioning — no gaps, no overscan; texture discipline unchanged.
  - `src/app/floating_controls.rs` UNTOUCHED — the pill's four arrow buttons keep the PRE-EXISTING navigate-on-click semantics + neighbor-based enablement (keys-only pan).
  - `window_options` relocated `app.rs` → `main.rs` (the launch-state semantics — resizable, always maximized — are unchanged).
  - Deferred-decision outcomes (ALL settled by the Wave-5 architector; the docs record them): pan step = 0.125 window fraction; auto-repeat contract = pan accepts repeats / edge-navigation needs a distinct non-repeat press / held-at-edge absorbed (OS-repeat timing is OS-owned — the Wave-4.1 F11 precedent note); mouse pan/wheel/drag = NOT in v1 scope (keys-only v1 per the brief — OUT-OF-SCOPE note); fit-mode suppression = MOOT (no `FitMode` exists since Wave 4.1 — one-shot W/H only sets the factor and pan follows the same overflow rule).
- Pending acceptance gates (BOTH to be recorded): the **0.5.0 Wave-5 OPERATOR-RUN** (the Task-4 manual-UX sub-item stays `[ ]`) AND the still-pending **0.4.2 Wave-4.2 3-item OPERATOR-RUN**.
- Test census (reused VERBATIM from B-Task-4 plan §0 — the source for the tech.md Testing replacement):

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
| `src/app/pan_tests.rs` (Wave-5 B-Group — app-state pan controller) | 12 |
| `src/app/pill_style_tests.rs` | 3 |
| `src/app/zoom_alias_tests.rs` | 14 |
| `src/app/zoom_tests.rs` | 11 |
| `src/app_display.rs` | 2 |
| `src/folder_listing/tests.rs` | 5 |
| `src/image_loading.rs` | 4 |
| `src/name_collation.rs` | 2 |
| `src/pan/pan_tests.rs` (Wave-5 B-Group — pure pan math) | 11 |
| `src/scaling.rs` | 5 |
| `src/scaling/fit_factor_tests.rs` | 7 |
| `src/sibling_folder_listing/neighbor_tests.rs` | 2 |
| `src/sibling_folder_listing/tests.rs` | 6 |
| `src/zoom.rs` | 8 |
| **Total** | **150** |

- Verify-only verdict #1 (verified by the architector at plan time): [`.agent/project-structure.md`](../../.agent/project-structure.md) is ALREADY CURRENT — it carries the Wave-5 entries (`src/pan.rs`, `src/app/pan.rs`, `src/pan/pan_tests.rs`, the `main.rs` window-options relocation note, the `keyboard_intake.rs` arrow routing). **NO edit is made to it in 4.2.** If the implementer's read shows a MISSING Wave-5 entry → STOP + ask the caller (never self-fix a verify-only file).
- Verify-only verdict #2 (verified by the architector at plan time): [`.agent/project-info/brief.md`](../../.agent/project-info/brief.md) needs **NO addendum** — the wave-mandated behavior is ALREADY in the brief's `### Pan and Navigate at Image Edges While Zoomed` (plus the `### Horizontal Scroll` / `### Vertical Scroll` pan-first sentences). **NO edit is made to it in 4.2.** Record this verdict in the 4.2 report only.
- Current docs state (post-Wave-4.2): README.md / tech.md / architecture.md / context.md all describe 0.4.2 with pan as "Wave 5 planned"; every stale marker to remove is pinned in §1–§4 below.

## §1 README.md edits (repo root `README.md`)

Execute E1→E8 in order. Every edit uses the edit tool with the exact anchor/oldString shown.

- **E1 — zoom bullet pan-reference removal** (line 24): oldString `Zooming enlarges/shrinks the image's center — the overflow is not drawn (pan is Wave 5).` → newString `Zooming scales the image around the window's center; while the zoomed image overflows the window, the overflowing region is drawn and the arrow keys pan it (feature 6 below).`
- **E2 — feature 6 bullet becomes the landed description** (line 25): replace the ENTIRE bullet text after `6. **Pan and Navigate at Image Edges While Zoomed** — ` with EXACTLY:

```text
(landed in Wave 5) — while the zoomed image is WIDER than the window, the left/right arrow keys pan the image horizontally, and previous/next sibling-image navigation triggers only when panning reaches the image's left or right edge; while the zoomed image is TALLER than the window, the up/down arrow keys pan the image vertically, and sibling-folder navigation triggers only when panning reaches the image's top or bottom edge (the Wave-3 folder-target rule is unchanged). The two axes are independent and may overflow together. On an overflowing axis every press pans — held/auto-repeated presses included — moving the image by 1/8 of the window's size on that axis per press; at the image's edge a DISTINCT press navigates (holding the key against the edge never navigates); when the image fits entirely on an axis, that axis's arrows navigate directly, as before Wave 5. The pan offset resets on every successful image open and on restore-to-fit (the `0` key or the pill's restore-fit button) and re-clamps itself after zoom changes. Keyboard-only: mouse-wheel and mouse-drag panning are out of scope in v1.
```

- **E3 — controls table, image-nav row** (line 30): oldString `| Navigate previous/next image | Left / Right arrow | [←] / [→] arrow buttons |` → newString `| Navigate previous/next image | Left / Right arrow — pans first while the zoomed image overflows the window, navigates only at the image's left/right edge, direct when it fits | [←] / [→] arrow buttons (always navigate directly) |`
- **E4 — controls table, folder-nav row** (line 31): oldString `| Navigate previous/next sibling folder | Up / Down arrow | [↑] / [↓] arrow buttons |` → newString `| Navigate previous/next sibling folder | Up / Down arrow — the same pan-then-navigate rule on the vertical axis | [↑] / [↓] arrow buttons (always navigate directly) |`
- **E5 — pill paragraph, keys-only note** (line 38): APPEND this sentence at the END of the paragraph (after the `pill_style.rs` sentence): `The pill's four arrow buttons always navigate directly (click = navigate); the pan-then-navigate behavior applies to the arrow KEYS only.`
- **E6 — Technology Stack intro** (line 48): oldString `the remaining v1 features (pan-then-navigate at the image edges while zoomed, "Open with...") are still open` → newString `the remaining v1 feature ("Open with...") is still open`
- **E7 — version row** (line 52): oldString `Cargo package \`img-scroller\` 0.4.2, edition 2024` → newString `Cargo package \`img-scroller\` 0.5.0, edition 2024`
- **E8 — Project Status append** (line 68): APPEND this sentence at the END of the `## Project Status` paragraph (before the `## Build and Run` heading), with one space separating it from the existing final sentence:

```text
Wave 5 landed (branch `feat/wave-5-pan-then-navigate`, version 0.5.0): the pan-then-navigate controller — while the zoomed image overflows the window, the arrow keys pan the overflowing axis (1/8 of the window per press) and navigation triggers only at the image's edge on a distinct press, while an axis that fits entirely navigates directly as before; the pill's arrow buttons keep their navigate-on-click behavior. Verification: dev checks `ALL CHECKS PASSED` (150 unit tests, 0 failed, across 23 test files), the release executable rebuilt at `dist/windows-x64/img-scroller.exe` (19,606,124 bytes), and the DLL import audit unchanged (the recorded 22-DLL set). Pending operator acceptance: the 0.5.0 manual UX run (the pan-then-navigate checklist) and the still-pending 0.4.2 checks.
```

## §2 tech.md edits (`.agent/project-info/tech.md`)

Execute T1→T6 in order.

- **T1 — Status append** (line 5, the `## Status` paragraph): APPEND this sentence at the very END of the paragraph (the line before `## Contents`), with one space separating it from the existing final sentence:

```text
Landed for Wave 5 (branch `feat/wave-5-pan-then-navigate`, version 0.5.0, 2026-11-11): the pan-then-navigate controller — the four arrow keys route through a per-axis pan-or-navigate decision (pure math `src/pan.rs` + app seam `src/app/pan.rs`), the overflowing axis pans on every press until the image's edge where a distinct non-repeat press navigates, a fits-entirely axis navigates directly, offsets reset on every successful open and on restore-to-fit and re-clamp lazily on use, and the paint composes the clamped offset (mechanism: `## Wave-5 Pan-then-Navigate Mechanism (Implemented)` below). Verification: dev checks `ALL CHECKS PASSED` (150 unit tests), the 0.5.0 exe rebuilt at `dist/windows-x64/img-scroller.exe` (19,606,124 bytes), and the DLL import audit set-equal to the recorded 22-DLL constant.
```

- **T2 — Contents list**: after item `7. [Fullscreen, Zoom and Fit Modes (Implemented, Wave 4)](#fullscreen-zoom-and-fit-modes-implemented-wave-4)` insert a new item `8. [Wave-5 Pan-then-Navigate Mechanism (Implemented)](#wave-5-pan-then-navigate-mechanism-implemented)` and RENUMBER the existing items 8–16 to 9–17 (mechanical: add 1 to each number; no wording change).
- **T3 — Technology Stack version row** (the Language row): oldString `Cargo package \`img-scroller\` version 0.4.2 (single binary; tracked \`Cargo.lock\`)` → newString `Cargo package \`img-scroller\` version 0.5.0 (single binary; tracked \`Cargo.lock\`)`
- **T4 — NEW mechanism section**: insert EXACTLY this section between the end of `## Wave-4.2 Operator-Feedback Mechanisms (Implemented)` and the `## Development Environment and Setup` heading:

```markdown
## Wave-5 Pan-then-Navigate Mechanism (Implemented)

Landed in Wave 5 (branch `feat/wave-5-pan-then-navigate`, version 0.5.0). Implemented in `src/pan.rs` (pure math), `src/app/pan.rs` (the controller seam), `src/app/keyboard_intake.rs` (arrow routing), `src/app/zoom.rs` (the restore-path reset), `src/app.rs` (the session state + the open-path reset), `src/app/central_surface.rs` + `src/app_display.rs` (the paint composition); `src/app/floating_controls.rs` is UNTOUCHED — the pill's arrow buttons keep the pre-existing navigate-on-click semantics and neighbor-based enablement (keys-only pan):

- Pure math (`src/pan.rs`): per-axis overflow = `max(0, zoomed_draw_size − available)`; the symmetric [-limit, +limit] offset space around center (`axis_pan_limit` = half the overflow; `clamped_axis_offset` collapses to 0 without overflow); at-edge predicates `axis_is_at_negative_edge`/`axis_is_at_positive_edge` with tolerance `PAN_EDGE_TOLERANCE = 0.5`; the pan step `PAN_STEP_FRACTION = 0.125` × the available axis span per press (a window-fraction step); and the pure `arrow_outcome` decision — a fits-entirely axis navigates directly on EVERY press (repeat included); an overflowing axis pans on EVERY press until its edge; AT the edge a DISTINCT non-repeat press navigates and a held auto-repeat is absorbed.
- Controller seam (`src/app/pan.rs`): `ArrowKeyPress { direction, repeat }`; `non_repeat_arrow_press_occurred` (the repeat-flag filter — the Wave-4.2 ESC non-repeat precedent); `arrow_pan_axis`/`arrow_pan_sign` map the direction to its axis/sign; `handle_arrow_key` evaluates the axis overflow from `zoomed_draw_size` vs the `viewport_rect().size()` intake and applies the outcome — `Pan` writes the offset through `apply_pan_delta` with the WRITE-side clamp (`clamped_axis_offset(raw + signed_delta, overflow)` at `src/app/pan.rs:126` — the last step lands exactly on the edge); `Navigate` calls `self.navigate(press.direction)` VERBATIM (the Wave-3 folder-target rule referenced unchanged).
- Intake routing (`src/app/keyboard_intake.rs`): the four arrow reads and dispatches route through `handle_arrow_key`; the available-size intake is `screen_context.viewport_rect().size()` (6 intake sites total: 2 fit + 4 arrows).
- Reset contracts: `pan_offset: egui::Vec2` (ONE session-RAM field carrying both per-axis components, never persisted) is zeroed on EVERY successful open (`accept_open_outcome`, `src/app.rs:95`) and on restore-to-fit (`reset_pan_offset` at `src/app/pan.rs:65`, called from `restore_zoom_to_fit` at `src/app/zoom.rs:74` — both the `0` key and the pill's restore-fit button funnel there). Zoom steps and one-shot W/H fit sets do NOT reset the offset (pinned by tests).
- Re-clamp contract (lazy-on-use): NO clamp call exists in the zoom actions BY DESIGN — the paint re-clamps the offset EVERY frame against the CURRENT zoom-composed overflow (`src/app_display.rs`: `zoomed_draw_size` → `axis_overflow_pair` → `clamped_pan_offset`), and the next arrow write re-clamps at `src/app/pan.rs:126`; a zoom/factor change that grows or shrinks the overflow is therefore absorbed without a reset.
- Paint composition (`src/app/central_surface.rs` + `src/app_display.rs`): `ImagePainting` gained the `pan_offset` field (the 3-field `{ decoded_image, zoom_factor, pan_offset }` request); the paint positions the zoomed draw size with an explicit `ui.put` repositioning derived from the clamped offset — no gaps, no overscan; texture discipline unchanged (one upload per decode).
- Deferred-decision outcomes (the todo's deferred list, ALL settled this wave): pan step = 0.125 window fraction; auto-repeat contract = pan accepts repeats, edge-navigation needs a distinct non-repeat press, held-at-edge repeats are absorbed (the OS-repeat timing caveat is the Wave-4.1 F11 precedent note — repeat timing is OS-owned); mouse pan/wheel/drag = NOT in v1 scope (keys-only v1 per the brief — OUT-OF-SCOPE note); fit-mode suppression = MOOT (no `FitMode` exists since Wave 4.1 — one-shot W/H only sets the factor and pan follows the same overflow rule).
- Verification (Wave-5 B-Task 4): dev checks `ALL CHECKS PASSED` — 150 unit tests, 0 failed, across 23 test files (census: `## Testing`); the 0.5.0 exe rebuilt at `dist/windows-x64/img-scroller.exe` = 19,606,124 bytes; the DLL import audit SET-EQUAL to the recorded 22-DLL constant (the list is NOT duplicated here — docs-split rule); NO dependency changes (`Cargo.toml`/`Cargo.lock` untouched by the wave).
```

- **T5 — Testing census replacement** (the FIRST bullet of `## Testing`, the one beginning `- 127 unit tests, all container-runnable`): replace that ENTIRE bullet with EXACTLY:

```text
- 150 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3; census pinned per file by the Wave-5 B-Task-4 verification): `src/app/tests.rs` — 9 (unchanged shell/open-flow census); `src/app/button_hint_tests.rs` — 2; `src/app/esc_close_confirm_tests.rs` — 11; `src/app/fit_one_shot_tests.rs` — 12; `src/app/floating_controls_tests.rs` — 1; `src/app/folder_navigation_continuation_tests.rs` — 4; `src/app/folder_navigation_tests.rs` — 5; `src/app/fullscreen_tests.rs` — 9; `src/app/last_viewed_tests.rs` — 5; `src/app/pan_tests.rs` — 12 (Wave-5: the app-state pan controller — the routing table, the open/restore resets, the zoom/fit non-resets); `src/app/pill_style_tests.rs` — 3; `src/app/zoom_alias_tests.rs` — 14; `src/app/zoom_tests.rs` — 11; `src/app_display.rs` — 2; `src/folder_listing/tests.rs` — 5; `src/image_loading.rs` — 4; `src/name_collation.rs` — 2; `src/pan/pan_tests.rs` — 11 (Wave-5: the pure pan math — per-axis overflow, the symmetric clamp, the pan-step sign/size, the at-edge decision, repeat absorption, the pinned constants); `src/scaling.rs` — 5; `src/scaling/fit_factor_tests.rs` — 7; `src/sibling_folder_listing/neighbor_tests.rs` — 2; `src/sibling_folder_listing/tests.rs` — 6; `src/zoom.rs` — 8. Grand total 150 across 23 test files (the 127 pre-Wave-5 tests unchanged + 23 Wave-5 tests).
```

- **T6 — Technical Constraints extension** (the bullet beginning `- Fullscreen, zoom, and fit-mode keybindings`): APPEND this sentence at the end of that bullet: `The pan-then-navigate policy (per-axis overflow, the 0.125 window-fraction step, the distinct-press edge navigation, the reset/re-clamp contracts, and the keys-only scope) is recorded in \`## Wave-5 Pan-then-Navigate Mechanism (Implemented)\` above.`

## §3 architecture.md edits (`.agent/project-info/architecture.md`)

Execute A1→A15 in order.

- **A1 — Status append** (line 5, the `## Status` paragraph): APPEND this sentence at the very END of the paragraph (the line before `## Contents`), with one space separating it from the existing final sentence:

```text
Wave 5 (branch `feat/wave-5-pan-then-navigate`, version 0.5.0, 2026-11-11) landed the pan-then-navigate slice: the pure per-axis pan math (`src/pan.rs`), the keyboard controller seam (`src/app/pan.rs` routing the four arrows between a pan step and edge navigation), the clamped-offset paint composition (`src/app/central_surface.rs` + `src/app_display.rs`), and the reset/re-clamp contracts, with the pill's arrow buttons unchanged. Dev checks ended `ALL CHECKS PASSED` (150 unit tests), the exe was rebuilt at `dist/windows-x64/img-scroller.exe` (19,606,124 bytes at 0.5.0), and the DLL import audit is unchanged (22).
```

- **A2 — Logical Components: the planned bullet becomes LANDED** (line 37): replace the ENTIRE bullet `- **Pan-then-navigate — planned (Wave 5)**: pan until the zoomed image's edge, then navigate; the draw-size composition is the Wave-5 anchor.` with EXACTLY:

```text
- **Pan-then-Navigate — LANDED (Wave 5)**: while the zoomed image overflows the window on an axis, that axis's arrow keys PAN on every press (auto-repeats included) by 0.125 of the available axis span (`PAN_STEP_FRACTION`) until the image's edge; AT the edge only a DISTINCT non-repeat press navigates (held keys are absorbed); an axis that fits entirely navigates directly on every press. Pure math `src/pan.rs` (per-axis overflow `max(0, zoomed_draw_size − available)`, the symmetric clamp, at-edge predicates with `PAN_EDGE_TOLERANCE = 0.5`, the pure `arrow_outcome` decision); the controller seam `src/app/pan.rs` (`handle_arrow_key` — the Navigate arm calls the shared `navigate` seam verbatim, so the Wave-3 folder-target rule is unchanged); intake routing in `src/app/keyboard_intake.rs` with the `viewport_rect().size()` available-size intake; the paint consumes the re-clamped offset (`src/app/central_surface.rs` + `src/app_display.rs` — explicit `ui.put` repositioning, no gaps/overscan); the offset resets on EVERY successful open and on restore-to-fit and re-clamps lazily on use; keyboard-only (mouse pan/wheel/drag out of scope in v1); the pill's arrow buttons keep navigate-on-click (`src/app/floating_controls.rs` untouched).
```

- **A3 — main.rs bullet** (line 45): oldString `starts the eframe native app with the \`Images Scroller\` title/window options, and opens the CLI image before the first frame when present.` → newString `builds the viewer window options (resizable, always maximized at launch — \`window_options()\`, relocated here from \`app.rs\` in Wave 5 with the launch-state semantics unchanged), starts the eframe native app with the \`Images Scroller\` title, and opens the CLI image before the first frame when present.`
- **A4 — app.rs bullet** (line 46): two edits inside the one bullet:
  - (a) oldString `the \`APP_WINDOW_TITLE\` constant, and \`NativeOptions\` (resizable, always maximized at launch).` → newString `the \`APP_WINDOW_TITLE\` constant (the \`NativeOptions\`/window-options construction moved to \`src/main.rs\` in Wave 5), and the session pan offset (\`pan_offset\`, reset on every successful open and on restore-to-fit).`
  - (b) In the child-module list, oldString `\`src/app/keyboard_intake.rs\`, \`src/app/pill_style.rs\`` → newString `\`src/app/keyboard_intake.rs\`, \`src/app/pan.rs\`, \`src/app/pill_style.rs\``; in the test-module list, oldString `\`src/app/fit_one_shot_tests.rs\`.` → newString `\`src/app/fit_one_shot_tests.rs\`, \`src/app/pan_tests.rs\`.`
- **A5 — keyboard_intake bullet** (line 61): oldString `→ \`W\` → \`H\` → the four arrows), the per-frame ESC-close popup-expiry update` → newString `→ \`W\` → \`H\` → the four arrows routed through the Wave-5 \`handle_arrow_key\` pan-or-navigate controller, each fed \`screen_context.viewport_rect().size()\`), the per-frame ESC-close popup-expiry update`
- **A6 — NEW app-seam bullet**: insert EXACTLY this bullet immediately AFTER the `src/app/keyboard_intake.rs` bullet (before the `src/app/zoom.rs` bullet):

```text
- `src/app/pan.rs` — the keyboard-only pan-or-navigate controller (Wave 5): maps each arrow press (`ArrowKeyPress`: direction + repeat flag) to its axis/sign, routes through `crate::pan`'s pure `arrow_outcome` between a pan step and edge navigation, gates at-edge navigation on a distinct non-repeat press (`non_repeat_arrow_press_occurred`), applies the pan delta with the write-side clamp, and calls the shared `navigate` seam verbatim on the Navigate arm; owns the pan-offset reset-on-restore plus the re-clamped writes to app state; 12 tests in `src/app/pan_tests.rs`.
```

- **A7 — central_surface bullet** (line 54): oldString `both layouts route through the one \`ImagePainting\` owner (the 2-field Wave-4.1 form: decoded image + session zoom factor).` → newString `both layouts route through the one \`ImagePainting\` owner (the 3-field Wave-5 form: decoded image + session zoom factor + the clamped pan offset).`
- **A8 — app zoom seam bullet** (line 62): oldString `the restore/\`0\` path and open-success reuse the plain reset to 1.0).` → newString `the restore/\`0\` path and open-success reuse the plain reset to 1.0, and the restore path also resets the Wave-5 pan offset through \`reset_pan_offset\`).`
- **A9 — NEW pure-math bullet**: insert EXACTLY this bullet immediately AFTER the `src/zoom.rs` bullet (before the `src/app_display.rs` bullet):

```text
- `src/pan.rs` — pure Wave-5 pan math: the per-axis overflow (`max(0, zoomed_draw_size − available)`), the symmetric [-limit, +limit] clamp, the at-edge predicates (`PAN_EDGE_TOLERANCE = 0.5`), the 0.125-of-available-axis pan step (`PAN_STEP_FRACTION`), and the arrow pan-or-navigate decision (a fitting axis navigates directly; an overflowing axis pans on every press including auto-repeats until its edge, where only a distinct non-repeat press navigates); no egui painting; 11 tests in `src/pan/pan_tests.rs`.
```

- **A10 — app_display bullet** (line 69): two edits inside the one bullet:
  - (a) oldString `Wave 4.1 reduced \`ImagePainting\` to the 2-field \`{ decoded_image, zoom_factor }\` request object` → newString `Wave 4.1 reduced \`ImagePainting\` to the \`{ decoded_image, zoom_factor }\` request object and Wave 5 added \`pan_offset\` (the 3-field \`{ decoded_image, zoom_factor, pan_offset }\` request)`
  - (b) oldString `so a window resize changes the whole-image base, NOT the stored factor (no re-fit).` → newString `so a window resize changes the whole-image base, NOT the stored factor (no re-fit); the paint re-clamps the pan offset EVERY frame against the current zoom-composed overflow (\`axis_overflow_pair\` → \`clamped_pan_offset\`) and positions the draw with an explicit \`ui.put\` repositioning — no gaps, no overscan.`
- **A11 — Navigation Critical Paths**: three edits:
  - (a) intro (line 80): oldString `as is Path 4's fullscreen/zoom/fit toggle slice (Wave 4, as corrected by Wave 4.1, with the Wave-4.2 pill-styling/alias/ESC-close corrections riding on the same paths); the pan-then-navigate interplay and "Open with..." remain planned.` → newString `as is Path 4's fullscreen/zoom/fit toggle slice (Wave 4, as corrected by Wave 4.1, with the Wave-4.2 pill-styling/alias/ESC-close corrections riding on the same paths) and Path 5's pan-then-navigate interplay (Wave 5); "Open with..." remains planned.`
  - (b) Path 2 (line 83): oldString `arrows navigate directly at this wave — pan-then-navigate (navigation only at the zoomed image's left/right edge) lands in Wave 5.` → newString `since Wave 5 the KEYS pan first while the zoomed image overflows the window and navigate only at the image's left/right edge (a distinct press); the pill's ←/→ buttons keep the direct navigate-on-click semantics.`
  - (c) Path 3 (line 84): oldString `The pan-first interplay — navigation only at the zoomed image's top/bottom edge — remains Wave-5 planned; arrows navigate directly now.` → newString `Since Wave 5 the Up/Down KEYS pan the overflowing vertical axis first and navigate (the Wave-3 target rule) only at the image's top/bottom edge; the pill's ↑/↓ buttons keep the direct navigate-on-click semantics.`
- **A12 — Data and State** (line 91): oldString `otherwise the absolute input time when it dismisses) — all RAM-only, destroyed on close; never persisted in v1.` → newString `otherwise the absolute input time when it dismisses) and the session \`pan_offset\` (Wave 5: ONE \`egui::Vec2\` field carrying the per-axis clamped pan offsets; zeroed on every successful open and on restore-to-fit; re-clamped lazily on use against the current zoom-composed overflow; zoom steps do NOT reset it) — all RAM-only, destroyed on close; never persisted in v1.`
- **A13 — Technical Decisions Summary**: insert EXACTLY this table row immediately AFTER the ESC-double-press row (before the Launch-state row):

```text
| Pan-then-navigate policy: per-axis overflow controller — an overflowing axis pans on EVERY press (auto-repeats included) by `PAN_STEP_FRACTION = 0.125` of the available axis until its edge; at-edge navigation needs a DISTINCT non-repeat press; a fits-entirely axis navigates directly; offsets reset on every successful open and on restore-to-fit and re-clamp lazily on use; keyboard-only (mouse pan/wheel/drag out of scope in v1); the pill's arrow buttons keep navigate-on-click | landed (Wave 5) | `### Pan and Navigate at Image Edges While Zoomed` |
```

- **A14 — Project Paths**: two edits:
  - (a) line 108: oldString `package \`img-scroller\` 0.4.2, edition 2024` → newString `package \`img-scroller\` 0.5.0, edition 2024`
  - (b) line 109: in the app child-modules list, oldString `/\`keyboard_intake\`/\`pill_style\`` → newString `/\`keyboard_intake\`/\`pan\`/\`pill_style\``; in the top-level list, oldString `\`name_collation\`, \`sibling_folder_listing\`` → newString `\`name_collation\`, \`pan\` (+ \`pan/pan_tests\`), \`sibling_folder_listing\``
- **A15 — Open Architectural Decisions** (line 141): DELETE the line `- Pan-then-navigate details while zoomed (Wave 5).` (decided and landed — the section keeps its other three items).

## §4 context.md edits (`.agent/project-info/context.md`)

Execute C1→C4 in order. PRESERVE every other entry byte-for-byte (overwrite-prevention rule). All entries in `## Recent Changes` use the literal `1.` numbering — keep that pattern.

- **C1 — Current Work Focus**: REPLACE the ENTIRE paragraph under `## Current Work Focus` (the one beginning `Dated 2026-11-11 (sessions 2026-11-07/10/11)`) with EXACTLY:

```text
Dated 2026-11-11. **Wave 5 (pan then navigate) agent-side work is COMPLETE on branch `feat/wave-5-pan-then-navigate` (version 0.5.0; NOT merged — the merge/push are Step-B.5 mechanics after the Task-5 mark)**: the B-Group (Tasks 1+2+3 combined) landed the pan-then-navigate controller — pure `src/pan.rs` (per-axis overflow `max(0, zoomed_draw_size − available)`, symmetric clamped offsets, at-edge predicates, `PAN_STEP_FRACTION = 0.125`, `PAN_EDGE_TOLERANCE = 0.5`) + app seam `src/app/pan.rs` (`ArrowKeyPress`, `non_repeat_arrow_press_occurred`, `handle_arrow_key`, `reset_pan_offset` — an overflowing axis pans on EVERY press including auto-repeats until its edge; at-edge navigation needs a DISTINCT non-repeat press; a fits-entirely axis navigates directly; the Wave-3 folder-target rule referenced unchanged) with arrow routing in `keyboard_intake.rs` (`viewport_rect().size()` intake), resets on EVERY successful open and on restore/`0`, lazy-on-use re-clamp (`app/pan.rs:126` + paint write-backs), the clamped-offset paint composition in `app_display.rs`/`central_surface.rs` (explicit `ui.put`), `floating_controls.rs` UNTOUCHED, and `window_options` relocated `app.rs` → `main.rs`; B-Task 4 verification is GREEN (dev-checks `ALL CHECKS PASSED` — 150 tests, 0 failed, 23 files; exe rebuilt `dist/windows-x64/img-scroller.exe` = 19,606,124 bytes at 0.5.0; DLL audit set-equal 22); B-Task 5 documentation landed (this step: README/tech/architecture/context updated to the 0.5.0 facts; `.agent/project-structure.md` and `brief.md` verified current — NO edits). REMAINING: B-Task-5 4.5b coherence check + 4.6 mark, then global-plan Step B.5 (todo-7 rename to `-DONE`, merge the branch into `main`, push `main` → `origin` ONLY). PENDING ACCEPTANCE GATES: the 0.5.0 Wave-5 OPERATOR-RUN (the Task-4 manual-UX sub-item stays `[ ]`) AND the still-pending 0.4.2 Wave-4.2 3-item OPERATOR-RUN. Wave 6 = `.agent/todos/20261029/20261029-todo-8.md` follows after both waves close.
```

- **C2 — Recent Changes entry**: INSERT EXACTLY this new FIRST entry line above the existing `1. 2026-11-11: **Wave 5 B-Group landed; B-Task 4 step 4.1b verification plan produced (this step).**` entry (the existing entries stay below, unchanged):

```text
1. 2026-11-11: **Wave 5 B-Tasks 4–5 complete — verification green + wave documentation landed (this step).** Branch `feat/wave-5-pan-then-navigate` (version 0.5.0). B-Task-4 verification (implementer): dev-checks `ALL CHECKS PASSED` — 150 unit tests, 0 failed, across 23 test files (the 127 pre-Wave-5 tests unchanged + 23 Wave-5: 11 pure-math in `src/pan/pan_tests.rs` + 12 app-seam in `src/app/pan_tests.rs`); the 0.5.0 exe rebuilt at `dist/windows-x64/img-scroller.exe` = 19,606,124 bytes (the previous 0.4.2 artifact of 19,603,539 bytes overwritten atomically); DLL import audit SET-EQUAL to the recorded 22-DLL constant. B-Task-5 documentation (this step): `README.md`, `tech.md`, `architecture.md`, `context.md` updated to the 0.5.0 facts — the landed pan-then-navigate behavior in the README features/controls, the new `## Wave-5 Pan-then-Navigate Mechanism (Implemented)` section in tech.md (math, seam, intake routing, paint composition, reset/re-clamp contracts, the deferred-decision outcomes incl. the mouse OUT-OF-SCOPE note and the MOOT fit-suppression resolution), the Logical-Components/Module-Layout/Data-and-State/decisions-table updates + the `window_options` relocation note in architecture.md, the 150-test census, the exe/audit lines, and BOTH pending OPERATOR-RUN gates (0.5.0 + 0.4.2); `.agent/project-structure.md` verified current (verify-only, NO edit); `brief.md` verified — the wave's behavior was ALREADY mandated by the brief's `### Pan and Navigate at Image Edges While Zoomed`, so NO addendum was made (verify-only). Wave commit chain (verbatim from the pre-flight `git log --oneline -20`; paste the actual output lines here): <PASTE>.
```

- **C3 — Current Project State**: four edits:
  - (a) INSERT EXACTLY this new FIRST bullet above the existing `- **Wave 4.2 branch ...` bullet:

```text
- **Wave 5 branch `feat/wave-5-pan-then-navigate` (agent-side COMPLETE through B-Task 5 docs; NOT merged — the merge/rename/push are Step-B.5 mechanics after the Task-5 mark)**: version 0.5.0; the pan-then-navigate controller landed (the mechanism: tech.md → `## Wave-5 Pan-then-Navigate Mechanism (Implemented)`); census 150 unit tests across 23 test files; exe rebuilt 19,606,124 bytes (0.5.0); DLL import audit unchanged (22 OBSERVED DLLs); the pending 0.5.0 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1), and the 0.4.2 OPERATOR-RUN stays pending in parallel (Next-Steps #2).
```

  - (b) Release-artifact line: oldString `(19,603,539 bytes, 0.4.2 Wave-4.2 rebuild)` → newString `(19,606,124 bytes, 0.5.0 Wave-5 rebuild)`
  - (c) TODO-state bullet: after the existing `.agent/todos/20261107/20261107-todo-1.md` sentence, INSERT this sentence: `.agent/todos/20261029/20261029-todo-7.md` (Wave 5) — Tasks 1–4 marked `[DONE]` (the Task-4 manual-UX OPERATOR-RUN sub-item stays `[ ]` until the user reports); Task 5 gains its `[DONE]` at this workflow's 4.6.`
  - (d) Git line: oldString `Git: \`main\` == \`origin/main\` == \`88c45f7\`; branch \`fix/wave-4-2-operator-feedback\` (Wave 4.2 — the wave lives on it, NOT merged; merge/rename/push are the Workflow-Step-5 mechanics after the Task-6 mark; HEAD = \`b2dc0bb\`)` → newString `Git: \`main\` == \`origin/main\` == <MAIN-SHA captured verbatim at pre-flight (§5 step 1 — do NOT guess)>; branch \`feat/wave-5-pan-then-navigate\` (Wave 5 — the wave lives on it, NOT merged; the merge lands at Step B.5 AFTER this docs task's 4.6; HEAD = <BRANCH-TIP-SHA captured verbatim at pre-flight>)`

- **C4 — Immediate Next Steps**: RESTRUCTURE to EXACTLY these four items (the existing items #2/#3 fold in; the 0.4.2 checklist fenced block is PRESERVED byte-for-byte under item 2):
  1. EXACTLY this item (the 0.5.0 gate; the checklist is verbatim from B-Task-4 plan §11; the exe path uses the Windows form exactly as the todo wrote it):

```text
1. **Pending 0.5.0 Wave-5 OPERATOR-RUN gate — user run on a real Windows 10/11 machine**, running `dist\windows-x64\img-scroller.exe` (the freshly rebuilt 0.5.0 exe). The manual-UX checklist is preserved verbatim at [`.kilo/plans/20261107-wave5-task4-verification-plan.md`](../../.kilo/plans/20261107-wave5-task4-verification-plan.md) §11:

Manual UX check (OPERATOR-RUN on the real Windows 10/11 machine — agent-side
GUI/runtime execution is impossible from the Linux container): zoom wide →
keys pan, navigation only at edge; zoom tall → same vertically; small image →
direct navigation.

The Task-4 manual-UX (OPERATOR-RUN) sub-item stays `[ ]` in the TODO until the user reports; a PASS closes the wave's gate. Regression note (binding, from the verification plan §11): every 0.4.2-established behavior stays expected-good on top of the NEW pan-then-navigate behavior.
```

  2. EXACTLY this item: `2. **Still-pending 0.4.2 Wave-4.2 OPERATOR-RUN gate — user run on a real Windows 10/11 machine**, running the 0.4.2 exe; the 3-item checklist is preserved verbatim at [.kilo/plans/20261107-wave4-2-task5-verification-plan.md](../../.kilo/plans/20261107-wave4-2-task5-verification-plan.md) §11 — KEEP the existing fenced checklist block from the current context.md under this item, byte-for-byte (the implementer MOVES it from the old item 1).`
  3. EXACTLY this item: `3. **Wave-5 Step B.5 close (agent-side, after this task's 4.5b + 4.6)**: rename \`.agent/todos/20261029/20261029-todo-7.md\` → \`20261029-todo-7-DONE.md\` (content preserved; the Task-4 OPERATOR-RUN sub-item STAYS \`[ ]\`), merge \`feat/wave-5-pan-then-navigate\` into \`main\` (fast-forward expected), delete the branch, push \`main\` → \`origin\` ONLY.`
  4. EXACTLY this item: `4. **Wave 6 (\`.agent/todos/20261029/20261029-todo-8.md\`) runs after both waves complete in a NEW chat/session** per the global plan's Session-Closing; the parked future-ideas list (incl. the recorded mouse-pan out-of-scope note) stays OUT of scope until scheduled.`

## §5 Execution sequence for 4.2 (exact; host-side only — NO alpine-vm calls)

| # | Step | Command / action | Expected |
| --- | --- | --- | --- |
| 0 | Pre-flight git evidence (host, ONE command per invocation, never chained) | `git branch --show-current`; then `git log --oneline -20`; then `git status --porcelain` | branch = `feat/wave-5-pan-then-navigate`; branch tip = `e0332fb` (mismatch → STOP + ask the caller); the wave chain visible below the tip (§0); porcelain shows ONLY the untracked plan file (this plan, pre-C1) and/or permitted docs residuals — any OTHER entry → STOP + ask. PASTE the `git log` lines into the C2 entry (the `<PASTE>` slot) and capture `main`'s SHA for the C3(d) slot (e.g. from `git rev-parse main`) |
| 1 | Verify-only reads (NO edits) | read `.agent/project-structure.md` and `.agent/project-info/brief.md` | structure: all Wave-5 entries present (§0 verdict #1); brief: `### Pan and Navigate at Image Edges While Zoomed` already mandates the behavior (§0 verdict #2). ANY missing entry → STOP + ask the caller (never self-fix a verify-only file) |
| 2 | Plan-file commit (wave precedent `ca8b65f`) | stage EXACTLY `.kilo/plans/20261107-wave5-task5-documentation-plan.md`; commit `docs: add wave5 task5 documentation plan` | commit succeeds; verify the staged list with `git status --short` BEFORE committing |
| 3 | README.md edits | §1 E1→E8, in order | every anchor found exactly once; no other README content changed |
| 4 | tech.md edits | §2 T1→T6, in order | every anchor found exactly once; Contents renumbered 1–17 |
| 5 | architecture.md edits | §3 A1→A15, in order | every anchor found exactly once |
| 6 | context.md edits | §4 C1→C4, in order | all other entries/sections preserved byte-for-byte |
| 7 | Coherence sweeps (host grep, ONE pattern per call) | see §6 | every sweep exact; ANY mismatch → fix ONLY the corresponding §1–§4 edit, never improvise |
| 8 | Docs commit | stage EXACTLY `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md` (verify with `git status --short` first — nothing else staged); commit `docs: wave5 task5 documentation` | commit succeeds |
| 9 | Gitignore sweep | re-read `.gitignore`, then `git status --short` | NO `dist/`, `logs/`, `target/`, `*.log` entries; nothing staged after step 8 |
| 10 | Report to caller | the commit SHAs of C1/C2; the verify-only verdicts; any sweep corrections made; the note that the Task-5 `[DONE]` mark (4.6), the todo rename, the merge, and the push are NOT this task's scope | — |

## §6 Post-edit coherence sweeps (§5 step 7; host grep, one pattern per invocation; the Grep tool is the fallback)

| Pattern | File | Expected |
| --- | --- | --- |
| `pan is Wave 5` | README.md | 0 hits (E1 removed the stale reference) |
| `lands in Wave 5` / `remains Wave-5 planned` | architecture.md | 0 hits (A11 replaced both) |
| `planned (Wave 5)` | architecture.md | 0 hits (A2 replaced the planned bullet) |
| `0.4.2, edition 2024` | README.md, tech.md, architecture.md | 0 hits (E7/T3/A14(a) bumped to 0.5.0) |
| `Wave-5 Pan-then-Navigate Mechanism` | tech.md | exactly 2 hits (Contents item 8 + the §-heading) |
| `src/pan.rs` | architecture.md | at least 2 hits (A2 + A9) |
| `src/app/pan.rs` | architecture.md | at least 2 hits (A2/A4 + A6) |
| `127 unit tests, all container-runnable` | tech.md | 0 hits (T5 replaced the census bullet) |
| `19,606,124` | context.md | at least 2 hits (C1 + C2/C3) |
| `feat/wave-5-pan-then-navigate` | context.md | at least 3 hits (C1, C2, C3(a)) |
| `19,603,539 bytes, 0.4.2 Wave-4.2 rebuild` | context.md | 0 hits (C3(b) updated the artifact line; the C2 historical mention of the OLD artifact size is permitted and expected) |
| `88c45f7`; branch `fix/wave-4-2-operator-feedback`; HEAD = `b2dc0bb` | context.md | 0 hits in the Git bullet (C3(d) rewrote it) |

NOTE: historical `127`/`19,603,539`/`0.4.2` mentions inside OLDER entries of context.md's `## Recent Changes` and inside the Status paragraphs' historical wave narratives of README/tech/architecture are PERMITTED (chronological record) — the sweeps above target only the pinned stale markers.

## §7 Junior-implementer checklist for 4.2 (condensed)

1. Host: `git branch --show-current` → `feat/wave-5-pan-then-navigate`; `git log --oneline -20` → tip `e0332fb` + the wave chain (paste into the C2 `<PASTE>` slot); `git status --porcelain` → only permitted residuals. Any mismatch → STOP + ask.
2. Host: read `.gitignore` (sweep reference) + the two verify-only files (§5 step 1).
3. Host: commit the plan file (§5 step 2, message EXACT).
4. Edit README.md (§1), tech.md (§2), architecture.md (§3), context.md (§4) — in that order, every anchor EXACT.
5. Host: run the §6 sweeps; fix ONLY the corresponding pinned edit on any mismatch.
6. Host: stage the four docs files; verify with `git status --short`; commit `docs: wave5 task5 documentation` (message EXACT).
7. Host: `git status --short` → nothing staged, no ignore-matching paths.
8. Report to caller (§5 step 10).

## §8 Git plan — which commits this task creates

| When | Commit | Content | Message (EXACT) |
| --- | --- | --- | --- |
| 4.2 start | C1 | this plan file only | `docs: add wave5 task5 documentation plan` |
| 4.2 end | C2 | `README.md` + `.agent/project-info/tech.md` + `.agent/project-info/architecture.md` + `.agent/project-info/context.md` ONLY | `docs: wave5 task5 documentation` |

- NO `git push` (restricted to workflow Step 5: `main` → `origin` ONLY). NO amend/force. NO `git add -A`/`git add .` (explicit paths only). NO branch creation/switch/merge (Step 2/Step 5 territory — Step B.5 is a separate implementer invocation).
- The Task-5 `[DONE]` mark (4.6) and the `20261029-todo-7.md` → `-DONE` rename are NOT this task's commits (4.6 + Step B.5 territory).
- Note (contrast with the Task-4 plan): in THIS docs task, `.agent/project-info/context.md` IS an owned, staged file — the Task-4 "never stage context.md" rule applied to Task 4 only.
- Commit staging lists are checked with `git status --short` immediately before each commit; the staged list MUST equal the row's content.

## §9 Deliberately out of scope (what 4.2 must NOT do)

- NO code changes anywhere in `src/`; NO `Cargo.toml`/`Cargo.lock` touches (0.5.0 already landed); NO version bumps.
- NO `.agent/project-structure.md` edits (verify-only — already current). NO `.agent/project-info/brief.md` edits (verify-only — the addendum already exists in the brief's feature text; NO new addendum needed). NO `product.md` edits (not in this task's file list).
- NO `CHANGELOG.md` touches (wave standing ban). NO edits to existing `.kilo/plans/*` files, INCLUDING this plan once it lands (new plan files only, ever).
- NO `docs/build-windows.md` or `docs/build.md` edits (the OBSERVED-DLL list stays where it is — docs-split rule; this task records only the audit-unchanged fact).
- NO todo-file edits (the Task-5 mark is 4.6; the rename is Step B.5). NO future-ideas.md edits.
- NO `git push`, NO branch operations, NO merge. NO harness/VM work of any kind (docs-only).
- NO new Wave-6/7 planning, NO "Open with..." implementation notes beyond the existing planned bullets.
- Any §6 sweep mismatch not resolvable by the corresponding pinned edit → STOP + report; anything ambiguous or outside this plan's assigned scope → return the question to the caller. NEVER make assumptions.

## §10 Stop conditions → return a user question

- §5 step-0 pre-flight mismatch: branch ≠ `feat/wave-5-pan-then-navigate`, branch tip ≠ `e0332fb`, or any unexplained porcelain entry.
- A verify-only file (`.agent/project-structure.md` or `.agent/project-info/brief.md`) failing its §0 verdict.
- ANY §1–§4 anchor not found exactly once in its target file (stale-marker drift = committed-state divergence from the plan's verified ground truth — report, never improvise).
- A §6 sweep mismatch not resolvable by the corresponding pinned edit.
- Anything ambiguous or outside this plan's assigned scope → return the question to the caller. NEVER make assumptions.
