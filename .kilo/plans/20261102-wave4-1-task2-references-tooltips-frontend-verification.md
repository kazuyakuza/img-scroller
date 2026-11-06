# Wave 4.1 Task 2 — Front-end Implementation Verification Report

**Date:** 2026-11-05  
**Workflow step:** 4.5a (frontend-specialist)  
**Branch:** `fix/wave-4-operator-fixes`  
**Verified HEAD:** `020c22f`  
**Spec:** `.kilo/plans/20261102-wave4-1-task2-references-tooltips-frontend-spec.md`  
**Plan:** `.kilo/plans/20261102-wave4-1-task2-references-tooltips-plan.md`  

**Verification method:** static code review of the six touched source files + `keyboard_intake.rs` (invariant), plus the Alpine-VM dev-checks harness (`alpine-vm` MCP, `vm_status` first).

---

## 1. Checklist results

| # | Checklist item | Verdict | Evidence |
|---|----------------|---------|----------|
| 1 | **§0 string set EXACT at final HEAD** — captions `F11`/`I`/`O`/`0`/`W`/`H` on six buttons; arrows uncaptioned via `None`; all ten tooltips verbatim; pinned by the two tests. | **CONFORMS** | `src/app/button_hint.rs` lines 57–69 (caption table) and 73–85 (tooltip table). The two inline tests assert every value exactly (`button_hint.rs` lines 151–183). |
| 2 | **Shared column wrapper consumes ALL 10 buttons** — `show_button_column` is called for every pill button (4 arrows + fullscreen + 3 zoom + 2 fit). **Captions never render without a reference** — caption `label` is inside `if let Some(reference) = hotkey_reference(button)`. **Tooltips attach to the SAME Response** — `painted_button.on_hover_text(tooltip).on_disabled_hover_text(tooltip)` on the allocate response. **Captions at 9 pt** — `CAPTION_TEXT_SIZE = 9.0`. **Align::TOP rows** — `floating_controls.rs` line 76 uses `egui::Layout::left_to_right(egui::Align::TOP)`. **Group gaps unchanged** — `FOLDER_GROUP_GAP`, `FULLSCREEN_GROUP_GAP`, `ZOOM_GROUP_GAP`, `FIT_GROUP_GAP` constants and `add_space` calls are untouched. **Bottom-center overlay contract respected statically** — `Area` still anchors `CENTER_BOTTOM` with `Order::Foreground`, frame still non-space-reserving. | **CONFORMS** | `button_hint.rs` lines 122–148; `floating_controls.rs` lines 12, 76, 128–140; `fullscreen_button.rs` lines 18–20; `zoom_button.rs` lines 59–67; `fit_mode_button.rs` lines 60–68. |
| 3 | **Behavior-neutral** — click dispatch (`accepts_click` / `clicked()` / `apply_action`) is unchanged; enablement predicates unchanged; keyboard intake untouched; shape painting unchanged (zoom `paint_glyph`/`zoom_glyph_shapes` merge is mechanical and emits identical shapes); arrow buttons still return the inner button `Response` through the column wrapper, so clicks remain hit-testable. | **CONFORMS** | Diff `86a1c2c..020c22f` shows only the column wrap and the zoom-glyph merge; no logic changes in `accepts_click`, `apply_action`, `navigation_button_enabled`, or `keyboard_intake.rs`. |
| 4 | **Rules at final HEAD** — touched files ≤200 lines (`button_hint.rs` 184, `app.rs` 183, `floating_controls.rs` 191, `fullscreen_button.rs` 55, `zoom_button.rs` 196, `fit_mode_button.rs` 186); methods ≤50 lines; no commented-out code; no magic-number regressions (`9.0`/`2.0` are named constants). | **CONFORMS** | `wc -l` output matches checklist counts exactly. Grep for non-doc `//` or `/*` in touched files found none. `CAPTION_TEXT_SIZE` / `CAPTION_TEXT_GAP` used. |
| 5 | **Tests** — exactly +2 inline tests in `button_hint.rs`; suite census 95; strings pinned. | **CONFORMS** | `cargo test` output: `running 95 tests`; the two new tests (`hotkey_reference_matches_the_spec`, `tooltip_text_matches_the_spec`) both pass. Dev-checks final line: `ALL CHECKS PASSED`. |
| 6 | **Static quality** — remaining docs parse true after `95247dc` trims; tooltips/captions cannot render while chrome hidden because the entire pill path is inside `full_ui_chrome_is_visible()` and no extra gate was added. | **CONFORMS** | `app.rs` lines 119–125 route to `show_fullscreen_layout` when chrome is hidden; `show_floating_pill` is only reached through `show_navigation_layout`. No new fullscreen-specific tooltip/caption gating added. Doc comments in touched files are syntactically valid Rust docs. |

---

## 2. Authorized deviation / review note

- **`hotkey_reference` and `tooltip_text` are module-private**, not `pub(super)`. The spec §9 wording says "exports ... `hotkey_reference`, `tooltip_text`", but the implementation plan §3 design notes explicitly records the `prefer-private` choice and states the 4.5a checklist is satisfied at module scope (`PillButton`, `PillButtonPlacement`, `ButtonColumnSite`, `show_button_column` are `pub(super)`). The inline tests access the private functions via `super`. This is a deliberate, user-approved plan amendment with no functional impact.
- **`src/app.rs` module-block whitespace was collapsed and modules were alphabetized** in commit `95247dc`, going beyond the plan §4 "keep every other row byte-identical" instruction. The project context log records this as "caller-authorized, review-fix commit". No new `ImagesScrollerApp` fields were added and no behavior changed.

---

## 3. Operator-run territory (explicitly not agent-verified)

The following cannot be validated by static review or the Linux container:

- Actual on-screen rendering of the 9 pt captions beneath each glyph (position, clipping, baseline alignment with `Align::TOP`).
- Tooltip popup appearance, position, hover delay, and readability on real Windows DPI/scaling.
- Whether painted-disabled buttons still show their tooltip on hover.
- Mouse click-through of the column wrapper in the Windows runtime.

These remain the OPERATOR-RUN items under Wave 4.1 Task 8.

---

## 4. Verdict

**Implementation CONFORMS to the user-approved front-end specification and plan.** No code changes are required from this verification step.
