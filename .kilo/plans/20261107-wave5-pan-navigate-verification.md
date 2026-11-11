# Wave 5 — Pan then Navigate at Image Edges (Front-end Implementation Verification Report)

**Report path:** `.kilo/plans/20261107-wave5-pan-navigate-verification.md`  
**Scope:** B-Group Step 4.5a — verify implementation against `.kilo/plans/20261107-wave5-pan-navigate-frontend-spec.md`.  
**Branch:** `feat/wave-5-pan-then-navigate`  
**Commits reviewed:** `3e2e332..8058be8` (`ca8b65f`, `272b98f`, `8058be8`)  
**Head:** `8058be8`

---

## Summary

Static code review against the 4.1a front-end spec and the implementation plan's D1–D6 adjustments shows the Wave 5 implementation is **conformant**. All 15 spec checklist items pass. The review-fix commit `8058be8` (Copy derives + tests 8/10/11 attempt-assertions) is behavior-preserving. One administrative follow-up is noted for the docs-specialist; the OPERATOR-RUN manual gate remains open for B-Task 4.

This verification did **not** run cargo/docker commands (outside the frontend-specialist tooling boundary); build/test evidence belongs to the implementer and B-Task 4 architector.

---

## 1. Spec conformance checklist

| # | Checklist item | Status | Evidence |
|---|----------------|--------|----------|
| 1 | Window shell unchanged | PASS | `src/main.rs:22-30` — `with_title(APP_WINDOW_TITLE)`, `with_resizable(true)`, `with_maximized(true)` (moved verbatim from `src/app.rs`, no behavior change). |
| 2 | App state `pan_offset: egui::Vec2` | PASS | `src/app.rs:44` field declaration; `src/app.rs:67` zero init in `empty_state`; `src/app.rs:95` reset on every successful open. |
| 3 | Pure pan module `src/pan.rs` | PASS | `src/pan.rs:9-150` exposes overflow, clamp, edge predicates, step, `arrow_outcome`; `src/pan/pan_tests.rs` provides 11 pure-math unit tests. |
| 4 | App pan seam `src/app/pan.rs` | PASS | `src/app/pan.rs:17-128` — `ArrowKeyPress`, `non_repeat_arrow_press_occurred`, `reset_pan_offset`, `handle_arrow_key`, axis/sign mapping. |
| 5 | Keyboard intake routing | PASS | `src/app/keyboard_intake.rs:42-113` — arrows no longer call `navigate` directly; routed through `handle_arrow_key` in Left → Right → Up → Down order. |
| 6 | Non-repeat edge navigation | PASS | `src/pan.rs:119-140` — `at_edge_outcome` returns `Navigate` only when `press_is_non_repeat`; repeat returns `Pan(0.0)`. Pan steps use `key_pressed` (accepts OS repeat). |
| 7 | Display integration | PASS | `src/app_display.rs:52-62` — explicit `panel_ui.put(image_rect, ...)` with clamped offset; old centered layout removed. |
| 8 | No gap / no overscan | PASS | `src/app_display.rs:58-61` — `clamped_offset` from `pan::clamped_pan_offset` with symmetric `[-overflow/2, +overflow/2]` bounds per axis. |
| 9 | Button semantics unchanged | PASS | `src/app/floating_controls.rs:37-42` enablement still `neighbor_image_path(...).is_some()`; `src/app/floating_controls.rs:131-144` clicked arrows call `app.navigate(direction)` directly. |
| 10 | Fullscreen ON parity | PASS | `src/app/keyboard_intake.rs:25` runs ungated in every mode; `screen_context.viewport_rect().size()` passed to controller (`src/app/keyboard_intake.rs:79-112`) equals the fullscreen viewport. |
| 11 | Wave-3 folder trigger preserved | PASS | `src/app/keyboard_intake.rs:97-112` maps Up/Down to `PreviousSiblingFolder`/`NextSiblingFolder`; `handle_arrow_key` calls `navigate` verbatim (`src/app/pan.rs:110`). |
| 12 | Reset semantics | PASS | `src/app.rs:95` resets pan on every successful open; `src/app/zoom.rs:74` calls `reset_pan_offset`; one-shot `W`/`H` and zoom steps do not reset. |
| 13 | Texture discipline | PASS | `src/app_display.rs:47-63` recomposes rect only; `ensure_current_image_texture` (`src/app_display.rs:68-81`) uploads only when `texture_upload_pending` is true (one upload per decode). |
| 14 | No mouse pan | PASS | No wheel/drag pan handlers anywhere in `src/`; `src/app/pan.rs:6` doc records mouse pan as out-of-scope. |
| 15 | No fit-mode suppression | PASS | No `FitMode` reference remains; one-shot `W`/`H` only change `zoom_factor`, pan follows the same overflow rule. |

---

## 2. D5 signature adjustment

The implementation uses the adjusted controller signature decided in the implementation plan (D5):

```rust
pub(super) fn handle_arrow_key(&mut self, available_size: egui::Vec2, press: ArrowKeyPress)
```

`src/app/keyboard_intake.rs:79-112` passes `screen_context.viewport_rect().size()` (intake-time, fullscreen-aware size), and `src/app_display.rs:58-59` re-clamps using the central-panel rect. This matches the D5 resolution and the spec's §5 fullscreen note.

---

## 3. Review-fix confirmation (`8058be8`)

| Fix | Files | Behavior change? |
|-----|-------|------------------|
| `#[derive(Clone, Copy)]` on `NavigationDirection` and `ArrowKeyPress` | `src/app/folder_navigation.rs:21`, `src/app/pan.rs:16` | No — fieldless/data-only types; fixes the compile-time double-move without runtime effect. |
| Tests 8/10/11 assert navigation *attempts* via `UNSUPPORTED_FILE_MESSAGE` | `src/app/pan_tests.rs:103-119`, `136-163`, `165-180` | No — adjusted for the Linux container's `ImageLoadFailure::Unloadable` stub; they still prove the edge press routes through `navigate` and resolves the correct target before attempting the open. |

The review fixes are compile/test-harness corrections only and do not alter the pan-or-navigate state machine.

---

## 4. Front-end quality observations (OPERATOR-RUN gate)

These are manual-gate items for B-Task 4; they are not agent-failable:

1. **Pan step feel at 0.125** — roughly 8 presses to cross a fully-overflowing axis; verify it feels controllable on a 1920×1080 window.
2. **Edge tolerance 0.5 px** — confirm float jitter does not make a press at the edge appear "not at edge" or skip the landing-press-only-pan rule.
3. **Navigation-at-edge feel** — confirm the landing press only pans and the **next** distinct press navigates; no accidental double-navigation on a held key.
4. **Behavior at zoom ×8 and at floor (1/8)** — at ×8 both axes may overflow together; at floor the image should fit and arrows navigate directly.
5. **Empty-folder / empty-state arrow behavior** — arrows should route directly to `navigate` (silent no-op when no listing).
6. **Fullscreen ON** — arrows pan/navigate identically; zoom keys (`I`/`O`/aliases/`0`/`W`/`H`) and ESC exit remain unaffected; no ESC-close popup appears.

---

## 5. Non-spec administrative follow-up

- **`.agent/project-structure.md` is stale**: it does not list the new `src/pan.rs` / `src/pan/pan_tests.rs` files or the new `app/pan.rs` child module. The docs-specialist should update it in B-Task 5.

---

## 6. Verdict

**CONFORMANT.** No spec-vs-implementation diffs. No front-end source-code changes are required from this verification step.
