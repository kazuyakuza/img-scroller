# Wave 4.1 Task 7 — One-Shot Fit Width/Height Adherence Report

**Branch:** `fix/wave-4-operator-fixes`  
**HEAD:** `5c7a067` (`docs: fit one-shot module docs`)  
**Date:** 2026-11-06  
**Spec:** `.kilo/plans/20261102-wave4-1-task7-one-shot-fit-frontend-spec.md`  
**Plan:** `.kilo/plans/20261102-wave4-1-task7-one-shot-fit-plan.md` + caller amendment D4a  

---

## (A) 4.5a Front-End Verification Checklist

| # | Item | Verdict | Evidence |
|---|------|---------|----------|
| 1 | **UX contract:** `W`/`H` set `zoom_factor` to the current image's width-fit/height-fit ratio, clamped to `[1/8, 8]`; no other state changes; `I`/`O` compose from the set level; `0`/restore + every open reset → `1.0`; repeats recompute; window resize does **not** re-fit. | **CONFORMS** | `src/scaling.rs:73-85` (`fit_width_factor`/`fit_height_factor` ratio derivation + clamp); `src/app/zoom.rs:77-104` (seams assign `zoom_factor` only); `src/app.rs:85` (open reset factor-only); `src/app/zoom.rs:67-69` (`0`→`1.0`); `src/app/fit_one_shot_tests.rs:68-78` (repeat + step composition); `src/app_display.rs:49-54` (per-frame `fitted_size(current_available) * zoom_factor`). |
| 2 | **Fullscreen interplay:** `W`/`H` keys remain active in every mode (intake runs pre-dispatch); buttons hidden when chrome hidden. | **CONFORMS** | `src/app/keyboard_intake.rs:1-5` (intake runs in every mode before layout), `:51-55` (`W`/`H` dispatch); `src/app.rs:117` (`full_ui_chrome_is_visible` gate). |
| 3 | **Empty-state/empty-folder:** buttons unrenderable or disabled; seams no-op silently when no image. | **CONFORMS** | `src/app/fit_button.rs:40-46` (`image_is_open` enablement gate); `src/app.rs:126-131` (pill only in navigation context). `src/app/fit_one_shot_tests.rs:81-89` asserts no-op silence. |
| 4 | **Static UI:** captions, tooltips, pill layout, glyphs unchanged. | **CONFORMS** | `src/app/button_hint.rs:57-69` (captions `"W"`/`"H"`), `:73-85` (tooltips `"Fit width (W)"`/`"Fit height (H)"`); `src/app/fit_button.rs:9-10` (labels); `src/app/floating_controls.rs:83-84` (pill group order unchanged). |
| — | **Spec §7 state/composition:** no `fit_mode` field; `ImagePainting` has `decoded_image` + `zoom_factor`; paint base is `fitted_size`; no mode passed to `ImagePainting`. | **CONFORMS** | `src/app.rs:17-38` (no `fit_mode`); `src/app_display.rs:15-18` (`ImagePainting` 2-field); `src/app_display.rs:49-54` (`fitted_size` base); `src/app/central_surface.rs:31-41` (no mode field). |
| — | **Spec §7 new app seam:** `set_zoom_to_fit_width`/`height` guard on `image_is_open`, compute via `scaling::fit_*_factor`, clamp inside math, do not touch `message_text`. | **CONFORMS** | `src/app/zoom.rs:77-104`; `src/scaling.rs:73-85` (clamps via `crate::zoom::clamped_zoom_factor`). |
| — | **Spec §7 removal verification:** no `FitMode`, `set_fit_mode`, fit-mode messages, or `fit_mode_button.rs`. | **CONFORMS** | `rg -n "FitMode\|set_fit_mode\|announce_fit_mode\|fit_mode_activation_label\|FIT_WIDTH_MESSAGE\|FIT_HEIGHT_MESSAGE" src` → zero matches; `src/app/fit_mode_button.rs` deleted, `src/app/fit_button.rs` created. |
| — | **Spec §7 behavior/performance:** texture upload once per decode, no per-frame allocations from fit logic. | **CONFORMS** | `src/app_display.rs:66-79` (`texture_upload_pending` gate); fit one-shot logic does not allocate per frame. |

**Live GUI fit behavior:** agent-side GUI/runtime execution is impossible from the Linux container; the operator-run confirmation on real Windows remains outstanding and is explicitly recorded here.

---

## (B) 4.5b Implementation Adherence Matrix

### Plan §2 S1–S12 §-by-§

| Step | Requirement | Verdict | Evidence |
|------|-------------|---------|----------|
| S1 | Expose `pub(crate) fn clamped_zoom_factor` in `src/zoom.rs` with one clamp source. | **CONFORMS** | `src/zoom.rs:39-42`. |
| S2 | Add ratio-derived `fit_width_factor`/`fit_height_factor` in `src/scaling.rs`, clamp inside via `crate::zoom::clamped_zoom_factor`; delete `FitMode`; demote `fit_width_size`/`fit_height_size`; add `fit_factor_tests.rs`. | **CONFORMS** | `src/scaling.rs:64-85`; `src/scaling/fit_factor_tests.rs` (7 tests). |
| S3 | Add `DecodedImage::size()` accessor. | **CONFORMS** | `src/image_loading.rs:19-25`. |
| S4 | `ImagePainting` becomes 2-field; paint base always `fitted_size`; remove helper. | **CONFORMS** | `src/app_display.rs:15-18`, `:49-54`. |
| S5 | `central_surface.rs` no longer passes a mode into `ImagePainting`. | **CONFORMS** | `src/app/central_surface.rs:31-41`. |
| S6 | Remove fit-mode messages/`set_fit_mode`; add `set_zoom_to_fit_width`/`height`; `restore_zoom_to_fit` factor-only. | **CONFORMS** | `src/app/zoom.rs:1-104`. |
| S7 | Keyboard `W`/`H` dispatch to the new seams using current viewport rect. | **CONFORMS** | `src/app/keyboard_intake.rs:51-55` (uses `viewport_rect().size()` per D4a). |
| S8 | Rename `fit_mode_button.rs` → `fit_button.rs`; rewire click action to viewport-rect factor set. | **CONFORMS** | `src/app/fit_button.rs:89-95` (uses `viewport_rect().size()`). |
| S9 | `button_hint.rs` import/method rename; labels/tooltips verbatim. | **CONFORMS** | `src/app/button_hint.rs:9`, `:47-52`, `:67-68`, `:83-84`. |
| S10 | `floating_controls.rs` import/call/doc update. | **CONFORMS** | `src/app/floating_controls.rs:13`, `:84`, header doc. |
| S11 | `app.rs`: remove `fit_mode` field/init; open-reset factor-only; update `mod` declarations. | **CONFORMS** | `src/app.rs:17-38`, `:85`, `:156`, `:166`. |
| S12 | Delete `fit_mode_tests.rs`; create `fit_one_shot_tests.rs`; remove one assertion from `zoom_tests.rs`. | **CONFORMS** | `src/app/fit_mode_tests.rs` deleted; `src/app/fit_one_shot_tests.rs` created; `src/app/zoom_tests.rs:40-49` no `fit_mode` assertion. |
| S13 | Dev-checks pass: fmt/check/test/clippy, 104 tests green. | **CONFORMS** | `alpine-vm` harness: `ALL CHECKS PASSED`; test exit=0 with 104 tests. |

### Ledger

| Item | Status | Notes |
|------|--------|-------|
| **D4a amendment** (`screen_rect` → `viewport_rect().size()`) | **VERIFIED** | `rg "screen_rect" src` = 0 matches; `viewport_rect().size()` at exactly 3 intake sites: `src/app/keyboard_intake.rs:52`, `:55`; `src/app/fit_button.rs:90`; plus doc references in `src/scaling.rs:70` and `src/app/zoom.rs:20`. |
| **Line-count drifts** | **ACCEPTED** | All source files ≤200 lines. Observed vs plan estimates: `src/app/zoom.rs` 178 (plan ~155), `src/scaling.rs` 146 (plan ~152), `src/app/fit_button.rs` 182 (plan ~185), `src/app_display.rs` 115 (plan ~113), `src/image_loading.rs` 96 (plan ~93). Difference is doc/comment/guard expansion; within rule. |
| **D4-variant errors** | **NONE** | No stray `screen_rect` or alternative rect usage found. |
| **Review-fix chain** | **APPLIED** | `4e44674` comment-only stale-mode prose fixes in `central_surface.rs` + `fit_button.rs`; `5c7a067` comment-only module-doc tweak; no behavior change. |
| **Simplifier pass** | **NONE** | No simplifier commit in the chain. |
| **104 census** | **VERIFIED** | `rg -c "#\[test\]" src` totals **104**; harness reports `running 104 tests` / PASS. |

### Skeptical AI-REALITY test of the §3 fate table

- **Deleted test names grep-zero:** `rg` for all 17 deleted names across `src` returned no matches.
- **19 new test names present:** 7 in `src/scaling/fit_factor_tests.rs` + 12 in `src/app/fit_one_shot_tests.rs` all found.
- **"Edits only" rows:** `src/app/zoom_tests.rs` lost exactly one `fit_mode` assertion line; `src/app/floating_controls_tests.rs` remains untouched (only arrow-enablement test). Both match the plan.

### Out-of-scope guard

- No Task 8/9 implementation work in `src` commits.
- No edits to `.agent/project-info/*`, TODO, `CHANGELOG.md`, existing plans, or `.agent/project-structure.md` in the source commits.
- `git diff --name-status 8e51cbf..HEAD` shows only one new plan file (`.kilo/plans/20261102-wave4-1-task7-one-shot-fit-review-fix.md`) plus `src/` changes.
- Gitignore compliance: `git status --short` is clean; no `dist/`, `logs/`, `target/`, or `*.log` files staged.

### Harness evidence

- Command: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
- Result: `fmt-check PASS`, `check PASS`, `test PASS`, `clippy PASS`, `ALL CHECKS PASSED`.
- Test count: `running 104 tests` with exit code 0.

---

## VERDICT: CONFORMS

The implementation matches the Task 7 spec and plan (including the D4a `viewport_rect()` amendment). All 104 tests pass; fmt/check/clippy are green; no out-of-scope edits; no gitignore violations. The only remaining work is the **operator-run live GUI verification** on Windows, which cannot be executed from the container.
