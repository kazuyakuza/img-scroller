# Wave 4 Task 3 — Fit Width / Fit Height Modes: Front-end Implementation Verification

Date: 2026-11-02
Branch: `feat/wave-4-fullscreen-zoom` (HEAD `d855223`)
Spec: `.kilo/plans/20261102-wave4-task3-fit-modes-frontend-spec.md`
Plan: `.kilo/plans/20261102-wave-4-task3-fit-modes-plan.md`

## Method

Static READ-only review of the final committed source files listed in the task prompt, plus a `#[test]` census via `rg -c "#\[test\]"`. No code changes, no commits, no live GUI run.

## Per-item verdicts

| # | Checklist item | Verdict | Evidence |
|---|----------------|---------|----------|
| 1 | Mode math: fit-width scale = avail_w/img_w, fit-height analog, FitVisible = whole-image inside, zero/positive rejection, per-frame recompute | **CONFORMS** | `src/scaling.rs:74-100` implements `fit_width_size`/`fit_height_size` with the same `is_positive_size` guards as `fitted_size`; `FitMode::base_size` dispatches to them at `src/scaling.rs:58-68`. `src/app_display.rs:49-55` recomposes `base_size(...) * zoom_factor` every frame via `zoom::zoomed_draw_size`. Math tests cover overflow directions (`src/scaling/fit_mode_tests.rs:7-25`) and the corrected wide-image literal (`src/scaling/fit_mode_tests.rs:46-61`). |
| 2 | Single `fit_mode` field; `set_fit_mode` resets mode + factor through one seam; open-success reset at Ok arm only | **CONFORMS** | Field added at `src/app.rs:39`; initialized at `src/app.rs:61`; reset at `src/app.rs:88` via `self.set_fit_mode(FitVisible)`. `set_fit_mode` is the only seam that mutates `fit_mode` and `zoom_factor` together (`src/app/zoom.rs:75-79`). No other assignments to `fit_mode` exist in the codebase. |
| 3 | `W`/`H` edge-triggered in `keyboard_intake`, dispatched after zoom chords, active in every mode, no collisions | **CONFORMS** | `src/app/keyboard_intake.rs:28-29` reads `FIT_WIDTH_KEY`/`FIT_HEIGHT_KEY` via `was_pressed` (edge-triggered). Dispatches at lines 46-51, after F11/zoom/restore and before arrows. No mode gating; fullscreen ON still runs this intake. Keys are plain `egui::Key::W`/`H`, unbound elsewhere. |
| 4 | Pill group after zoom buttons; labels `Fit width`/`Fit height`; painted axis-line + rotated triangles; `image_is_open` gate; 10-button row | **CONFORMS** | `src/app/floating_controls.rs:81` appends `fit_mode_button::show_fit_mode_pill_buttons`. `src/app/fit_mode_button.rs:35-41` adds the 16.0 group gap and renders two buttons. Accessible labels at `src/app/fit_mode_button.rs:87-92`. Glyphs are `Shape::rect_filled` axis lines + `Shape::rotated_triangle` arrows (`src/app/fit_mode_button.rs:117-172`), no font glyphs. Enablement is `app.image_is_open()` (`src/app/fit_mode_button.rs:43-48`). Row order: 4 arrows + 1 fullscreen + 3 zoom + 2 fit = 10. |
| 5 | Single `ImagePainting` literal; `base_size × factor` per frame in chrome and fullscreen; no texture/pipeline change; no pan/scroll | **CONFORMS** | `src/app/central_surface.rs:31-43` is the only `ImagePainting { ... }` literal. Both `show_navigation_central_surface` and `show_central_image_while_open` route through it. `src/app_display.rs:49-55` computes the fit base and applies `zoom_factor`. Texture upload remains gated by `texture_upload_pending` (`src/app_display.rs:67-80`); fit/zoom never touch it. No pan offset or scroll logic is present. |
| 6 | Activation announces `Fit width`/`Fit height` once; `FitVisible` clears; zoom-bound and fit messages share one channel with event-chronology precedence | **CONFORMS** | `src/app/zoom.rs:75-83` sets the message from `fit_mode_activation_label`, which returns `Some` for W/H and `None` for Visible (`src/app/zoom.rs:151-157`). Zoom-bound traps assign `ZOOM_AT_MINIMUM_MESSAGE`/`ZOOM_AT_MAXIMUM_MESSAGE` (`src/app/zoom.rs:99-114`). `src/app/keyboard_intake.rs:21-22` documents and encodes the dispatch order so a same-frame fit activation overwrites a zoom-bound message. |
| 7 | Fullscreen hides fit buttons automatically; hotkeys stay active; mode survives toggles | **CONFORMS** | Fit buttons are part of the pill, which is only rendered when `full_ui_chrome_is_visible()` is true (`src/app/fullscreen.rs:26-28`). Fullscreen layout (`src/app/fullscreen.rs:33-37`) uses only the central surface. Keyboard intake runs unconditionally before layout. `toggle_fullscreen` does not mutate `fit_mode` or `zoom_factor`. |
| 8 | 17 new tests (6 math + 11 policy); suite total 93; no OS-bound tests; key pins exact | **CONFORMS** | `rg -c "#\[test\]"` counts 93 across `src/`. New files: `src/scaling/fit_mode_tests.rs` has 6 tests; `src/app/fit_mode_tests.rs` has 11 tests. Hotkey pins assert `zoom::FIT_WIDTH_KEY == egui::Key::W` and `FIT_HEIGHT_KEY == egui::Key::H` (`src/app/fit_mode_tests.rs:90-92`). All tests are pure in-memory/unit; no OS-bound frame/GUI tests. |
| 9 | Static quality: stale docs, dead code, packaging/clip anomalies | **MISMATCH / QUALITY NOTE** | Project-info docs (`README.md`, `tech.md`, `architecture.md`, `context.md`) still describe Wave 4 Task 3 as a front-end spec awaiting approval and list fullscreen/zoom keybindings as undecided, even though the code is landed. `.agent/project-structure.md` omits the new modules (`central_surface.rs`, `fit_mode_button.rs`, `fit_mode_tests.rs`, `scaling/fit_mode_tests.rs`) and still describes `scaling.rs` as only whole-image fit. No dead code that breaks functionality, but `src/app/fit_mode_button.rs:72-78` keeps a thin `accepts_click` wrapper around `enabled_click_would_fire` that could be inlined. No packaging/clip anomalies detected statically. |
| 10 | Scope guard: no Wave-5 pan, no persistence, no CHANGELOG/existing-plan edits, preserves Task-1/2 behaviors | **CONFORMS** | No `pan_offset`, no config/persistence write, no `CHANGELOG.md` changes, no edits to existing `.kilo/plans/*` files. Zoom factor range/clamp/step unchanged; restore-fit returns to `FitVisible` as encoded. Task-1 fullscreen toggle and Task-2 zoom buttons are untouched except for the new fit group appended after the zoom group. |

## Diffs vs binding spec / adjudicated deviations

These are intentional encodings from the 4.1b plan, present in the final code:

1. **3-arg `fit_mode_size` → `FitMode::base_size` method + axis helpers**
   - Spec §4 recommended `fit_mode_size(available, image, mode)`.
   - Implementation: `FitMode::base_size(self, available, image)` at `src/scaling.rs:58-68` plus `fit_width_size`/`fit_height_size` (`src/scaling.rs:74-100`). Functionally equivalent and respects the ≤2-non-self-args rule.
2. **Active-mode feedback: once-at-activation standing message**
   - Spec §7/F1 described a per-frame message-line label with precedence `unsupported file > zoom bound > fit mode`.
   - Implementation assigns the fit label once inside `set_fit_mode` and lets the next event overwrite it. `keyboard_intake.rs` documents the encoded chronology: fit dispatch follows zoom, so a same-frame fit activation overwrites a zoom-bound message. This matches the adjudicated plan §0a.
3. **Fit-button enablement adds an explicit `image_is_open` gate**
   - Zoom buttons remain gated only by factor predicates (Task-2 behavior preserved). Fit buttons introduce `image_is_open` enablement (`src/app/fit_mode_button.rs:44`), making them disabled in empty-folder mode as required.

## Quality notes (not blocking, junior-executable fixes)

1. **Stale project-info documentation**
   - `README.md` line 25 lists "fit width / fit height modes" but the Project Status paragraph still says Task 3 spec is produced/awaiting approval.
   - `tech.md` line 127 still says "Fullscreen/zoom keybindings and zoom step sizes are implementation details, not fixed here."
   - `architecture.md` lines 34, 70, 119 still describe fullscreen/zoom/fit modes as planned or undecided.
   - `context.md` Current Work Focus and Current Project State still say Task 3 "front-end spec produced; awaiting user/planner approval."
   - `.agent/project-structure.md` does not list the new modules and still describes `scaling.rs` as only whole-image fit.
   - **Fix:** update these files in the Task-5 documentation cycle (out of scope for this verification step, but should be tracked).
2. **Thin wrapper in fit-mode button**
   - `FitModeButtonSpec::accepts_click` (`src/app/fit_mode_button.rs:72-74`) just calls `self.enabled_click_would_fire(button_response)`. It can be removed and `show_and_apply_action` can call `enabled_click_would_fire` directly, matching the post-4.3 simplification intent.
3. **No operator-run rendering verification**
   - Live check of glyph orientation, W/H overflow behavior, and pill layout on a real Windows window remains OPERATOR-RUN and is not covered by this static report.

## Steps to fix the one code-quality item

If the architector wants the wrapper removed:

1. In `src/app/fit_mode_button.rs`, replace the call in `show_and_apply_action`:
   ```rust
   if self.enabled_click_would_fire(&button_response) {
       self.apply_action(app);
   }
   ```
2. Delete the `accepts_click` method (`lines 72-74`).
3. Re-run container dev checks (`scripts/dev-checks.sh`) to confirm fmt/clippy/test still pass.

## Summary verdict

**CONFORMS** to the binding spec and implementation plan for all functional front-end requirements. One **MISMATCH/QUALITY NOTE** for stale project-info documentation that should be refreshed in the pending Task-5 docs cycle. Live rendering verification is OPERATOR-PENDING.
