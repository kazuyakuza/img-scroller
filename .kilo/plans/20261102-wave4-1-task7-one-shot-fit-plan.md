# Wave 4.1 Task 7 — One-Shot Fit Width/Height: Implementation Plan

**Date:** 2026-11-06 (critical-workflow step 4.1b)
**Branch:** `fix/wave-4-operator-fixes` — starting HEAD `aa2175b` (task: mark wave4.1 task6 aliases done)
**Spec (binding):** [`.kilo/plans/20261102-wave4-1-task7-one-shot-fit-frontend-spec.md`](20261102-wave4-1-task7-one-shot-fit-frontend-spec.md)
**TODO source:** `.agent/todos/20261102/20261102-todo-1.md` Task 7
**Implementer:** junior (≤50% restriction) — execute ONLY the numbered steps; every decision in this file is final; do not redesign anything. If a file/line does not match, STOP and return a question to the caller.

---

## 0. Pre-implementation state (verified 2026-11-06 at HEAD `aa2175b`)

- Package version is already `0.4.1` (`Cargo.toml` line 3) — the wave Step-3 bump landed. **No version change in this task.**
- Untracked/modified artifacts from step 4.1a (recorded state, to be committed first):
  - `.agent/project-info/context.md` — **modified** (staged nothing yet); its `## Current Work Focus` / `## Recent Changes` already carries the Task-7 spec entry (dated 2026-11-06).
  - `.kilo/plans/20261102-wave4-1-task7-one-shot-fit-frontend-spec.md` — **untracked**.
  - `.kilo/plans/20261102-wave4-1-task7-one-shot-fit-plan.md` — **untracked** (this file).
- Verified census at HEAD: **102** `#[test]` fns, per-file: `image_loading.rs` 4, `app_display.rs` 2, `folder_listing/tests.rs` 5, `scaling.rs` 5, `zoom.rs` 8, `name_collation.rs` 2, `scaling/fit_mode_tests.rs` 6, `sibling_folder_listing/tests.rs` 6, `sibling_folder_listing/neighbor_tests.rs` 2, `app/button_hint.rs` 2, `app/floating_controls_tests.rs` 1, `app/fit_mode_tests.rs` 11, `app/folder_navigation_tests.rs` 5, `app/folder_navigation_continuation_tests.rs` 4, `app/fullscreen_tests.rs` 9, `app/last_viewed_tests.rs` 5, `app/zoom_tests.rs` 16, `app/tests.rs` 9.

## 1. Binding decisions encoded (charts of every thorny point — NO alternatives)

| # | Decision | Content |
|---|----------|---------|
| D1 | Factor fns live in **`src/scaling.rs`** (`fitted_size`'s home — pure display math cohesion), NOT `src/zoom.rs`. |
| D2 | Derivation **via the landed ratios**: `fit_width_factor = fit_width_size(…)?.x / fitted_size(…)?.x` (height analogue with `.y`). The spec §4 snippet bodies (`available.x / fitted.x`) are superseded by this ratio derivation (mathematically identical; keeps `fit_width_size`/`fit_height_size` alive as factor inputs so they are not dead code). |
| D3 | **Clamp INSIDE** the factor fns via `crate::zoom::clamped_zoom_factor` (made `pub(crate)` in `src/zoom.rs`). One clamp source only — call it **fully-qualified as `crate::zoom::clamped_zoom_factor(...)` inside `scaling.rs`** (twin-zoom trap: there are two `zoom` modules, the pure-math `crate::zoom` and the app seam `super::zoom` inside `src/app/*.rs`; in `scaling.rs` only `crate::zoom::…` is valid). No clamp constant is duplicated in `scaling.rs`. The floor clamp is a latent safety net: the ratio is mathematically always ≥ 1.0, so only the ceiling is operator/test-reachable — this fact goes into the ceiling test's doc comment. |
| D4 | **Available size at intake = `ctx.screen_rect().size()`**, read **at the two callsites** (keyboard intake + fit button), NOT cached. Trade-offs resolved: (a) vs. caching the last painted `panel_ui.available_size()` in app state: no new app field, no first-frame-after-CLI-open gap, and resize-then-press-`W` always uses the CURRENT window size (a cache is up to 1 frame stale during a resize drag); (b) cost: `screen_rect` is the full window client rect while the paint base uses the central panel's available size (panel frame margins, pill is a non-space-reserving overlay) — the one-shot display lands within a few px of a full-window span; overflow clips at the panel rect (already the design). Accepted per spec §5. **Seam signature takes the pre-read `egui::Vec2`** (not `&egui::Context`): `set_zoom_to_fit_width(&mut self, available_size: egui::Vec2)` — identical runtime semantics, and it makes the new app tests deterministic (a bare `egui::Context::default()` has no pinned screen rect; unit tests then inject the exact `Vec2`). |
| D5 | One-shot semantics: `W`/`H`/buttons assign `self.zoom_factor` only. **No persistent mode exists anywhere.** `0`/restore and every successful open reset `zoom_factor` to `1.0`. Fit actions are SILENT (user-approved: the `Fit width`/`Fit height` messages are removed entirely); `message_text` is never written by a fit action. |
| D6 | Button module: **Option A** — rename `src/app/fit_mode_button.rs` → `src/app/fit_button.rs`; `FitModeButtonAction` → `FitButtonAction { FitWidth, FitHeight }`; `FitModeButtonSpec` → `FitButtonSpec`; `show_fit_mode_pill_buttons` → `show_fit_pill_buttons`; `fit_mode_button_specs` → `fit_button_specs`. Painted glyphs, sizes, gaps, and the `image_is_open()` enablement gate are unchanged. |
| D7 | `ImagePainting` becomes `{ decoded_image, zoom_factor }`; the paint base is ALWAYS `scaling::fitted_size(panel_ui.available_size(), image.size())` (whole-image fit) at both paint sites (chrome + fullscreen route through the single `central_surface.rs` seam unchanged). |
| D8 | `scaling::fit_width_size`/`fit_height_size` survive (as factor inputs, D2) but are demoted from `pub(crate)` to private `fn` — their only remaining callers are the same-module factor fns (private-by-default rule). `FitMode` + `FitMode::base_size` (± the `#[allow(clippy::enum_variant_names)]` line) are DELETED. |
| D9 | Test census: **104** final (see §6 name-by-name fate table). The spec's ~98 estimate counted the five re-homed hotkey/label/enablement pins as zero-net; under Option A they reappear as distinct `#[test]` fns in the new files, so the exact delta is −17 +19 = +2 → **102 → 104**. This plan's 104 is authoritative; the spec §7 census checklist item is satisfied by 104. |

## 2. Ordered implementation steps

Run NO cargo command between steps S1–S12 (the container harness runs once at S13). Use built-in file tools for edits; `git` for the rename in S8.

### S0 — Docs/artifacts commit (pre-implementation)

1. `git branch --show-current` → must print `fix/wave-4-operator-fixes`.
2. `git add .agent/project-info/context.md .kilo/plans/20261102-wave4-1-task7-one-shot-fit-frontend-spec.md .kilo/plans/20261102-wave4-1-task7-one-shot-fit-plan.md`
3. `git status --short` → exactly these three entries staged (`M` context.md + `A` two plans); nothing else staged (`dist/`, `logs/`, `target/` are gitignored — verify none appear).
4. Commit: `docs: add wave4.1 task7 spec and plan`

### S1 — Expose the clamp (`src/zoom.rs`, then 128 lines)

- Rename `fn clamped_zoom_factor` (line 39) → `pub(crate) fn clamped_zoom_factor`.
- Add one doc line above it: `/// Shared with the fit-factor math in `src/scaling.rs` (the ONE clamp source).`
- Nothing else in this file changes (bounds already symmetric `[1/8, 8]`; `RESTORE_TO_FIT_FACTOR = 1.0` already exists).

### S2 — Pure math (`src/scaling.rs`) and its tests

In `src/scaling.rs`:

1. DELETE lines 39–69: the `FitMode` enum (incl. the `#[allow(clippy::enum_variant_names)]` attribute and all doc comments), the `impl FitMode` block, and `base_size`.
2. DELETE the trailing `#[cfg(test)] mod fit_mode_tests;` (lines 160–161) and the file `src/scaling/fit_mode_tests.rs` (the whole file).
3. Demote visibility: `pub(crate) fn fit_width_size` → `fn fit_width_size`; `pub(crate) fn fit_height_size` → `fn fit_height_size` (same-module callers only now, D8). Keep their doc comments verbatim.
4. ADD below `fit_height_size` (before `#[cfg(test)] mod tests;` stays untouched — it is inline in `mod tests`):

```rust
/// One-shot fit factors (Wave 4.1 Task 7): the zoom factor whose paint
/// produces the landed fit outcome. Each factor derives as the ratio of the
/// axis-fit size over the whole-image fitted size, then clamps INSIDE to the
/// symmetric zoom bounds via `crate::zoom::clamped_zoom_factor` (one clamp
/// source; no duplicate bound constants here). The caller supplies the
/// available area observed AT INTAKE (window client rect via
/// `screen_rect().size()`); the display keeps composing
/// `fitted_size(current_available) * zoom_factor` per frame, so a window
/// resize after a one-shot set does NOT re-fit.
pub(crate) fn fit_width_factor(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<f32> {
    let fitted = fitted_size(available_size, image_size)?;
    let width_fitted = fit_width_size(available_size, image_size)?;
    Some(crate::zoom::clamped_zoom_factor(width_fitted.x / fitted.x))
}

/// The height-fit twin of `fit_width_factor`: the zoom factor that makes the
/// image HEIGHT span the available height (width overflow allowed).
pub(crate) fn fit_height_factor(
    available_size: egui::Vec2,
    image_size: egui::Vec2,
) -> Option<f32> {
    let fitted = fitted_size(available_size, image_size)?;
    let height_fitted = fit_height_size(available_size, image_size)?;
    Some(crate::zoom::clamped_zoom_factor(height_fitted.y / fitted.y))
}
```

(4-arg limit check: exactly 2 params each; bodies ≤ 4 lines; max depth 1.)

5. ADD at the bottom: `#[cfg(test)] mod fit_factor_tests;`

6. CREATE `src/scaling/fit_factor_tests.rs` — exact content skeleton (tests 1–6 are the spec §6.2 set; test 7 is the D3/D9 ceiling case):

```rust
//! One-shot fit-factor tests: exact-in-f32 vectors (the `fitted_size`
//! conventions) plus the clamp-at-ceiling case. The zoom-out FLOOR clamp is
//! NOT testable here: every factor is mathematically `>= 1.0` (the fitted
//! size never exceeds the available area), so the floor arm of
//! `clamped_zoom_factor` is a latent safety net with no reachable input.

use super::{fit_height_factor, fit_width_factor};
use eframe::egui;

#[test]
fn fit_width_factor_is_one_for_a_wide_image() {
    let available_size = egui::Vec2::new(800.0, 300.0); // width constrains
    let image_size = egui::Vec2::new(400.0, 100.0);
    assert_eq!(fit_width_factor(available_size, image_size), Some(1.0));
}

#[test]
fn fit_width_factor_exceeds_one_for_a_tall_image() {
    let available_size = egui::Vec2::new(300.0, 800.0);
    let image_size = egui::Vec2::new(100.0, 400.0); // fitted (200, 800)
    assert_eq!(fit_width_factor(available_size, image_size), Some(1.5));
}

#[test]
fn fit_height_factor_is_one_for_a_tall_image() {
    let available_size = egui::Vec2::new(300.0, 800.0); // height constrains
    let image_size = egui::Vec2::new(100.0, 400.0);
    assert_eq!(fit_height_factor(available_size, image_size), Some(1.0));
}

#[test]
fn fit_height_factor_exceeds_one_for_a_wide_image() {
    let available_size = egui::Vec2::new(800.0, 300.0);
    let image_size = egui::Vec2::new(400.0, 100.0); // fitted (800, 200)
    assert_eq!(fit_height_factor(available_size, image_size), Some(1.5));
}

#[test]
fn square_image_fit_factors_are_one() {
    let available_size = egui::Vec2::new(400.0, 400.0);
    let image_size = egui::Vec2::new(100.0, 100.0);
    assert_eq!(fit_width_factor(available_size, image_size), Some(1.0));
    assert_eq!(fit_height_factor(available_size, image_size), Some(1.0));
}

#[test]
fn fit_factors_reject_non_positive_dimensions() {
    let usable = egui::Vec2::new(100.0, 100.0);
    assert_eq!(fit_width_factor(usable, egui::Vec2::new(0.0, 10.0)), None);
    assert_eq!(fit_width_factor(usable, egui::Vec2::new(10.0, 0.0)), None);
    assert_eq!(fit_width_factor(egui::Vec2::new(0.0, 100.0), usable), None);
    assert_eq!(fit_width_factor(egui::Vec2::new(100.0, -1.0), usable), None);
    assert_eq!(fit_height_factor(usable, egui::Vec2::new(0.0, 10.0)), None);
    assert_eq!(fit_height_factor(usable, egui::Vec2::new(10.0, 0.0)), None);
    assert_eq!(fit_height_factor(egui::Vec2::new(0.0, 100.0), usable), None);
    assert_eq!(fit_height_factor(egui::Vec2::new(100.0, -1.0), usable), None);
}

#[test]
fn fit_width_factor_clamps_at_the_zoom_ceiling() {
    let available_size = egui::Vec2::new(10.0, 10.0);
    let image_size = egui::Vec2::new(10.0, 1000.0); // raw ratio ~100 >> 8
    assert_eq!(fit_width_factor(available_size, image_size), Some(8.0));
}
```

### S3 — Shared image dimensions (`src/image_loading.rs`, 86 → ~93)

1. Add `use eframe::egui;` after the `use std::path::Path;` line.
2. Add after the `DecodedImage` struct definition:

```rust
impl DecodedImage {
    /// Image dimensions as `egui::Vec2`, shared by the display-paint base and
    /// the one-shot fit-factor seams (no duplicated u32→f32 casts).
    pub(crate) fn size(&self) -> egui::Vec2 {
        egui::Vec2::new(self.width as f32, self.height as f32)
    }
}
```

### S4 — Display composition (`src/app_display.rs`, 121 → ~113)

1. Add `use crate::scaling;` after `use crate::zoom;` (line 7 area).
2. Struct doc (lines 11–13) reword to: `/// The 2-argument paint request (project max-args rule): WHICH decoded image is on screen and WHAT zoom factor scales the whole-image fit base into the draw size.`
3. DELETE the `fit_mode` field (line 16). `ImagePainting` = `{ decoded_image, zoom_factor }` in declaration order.
4. In `paint`, replace lines 49–54 (`painting.fit_mode.base_size(...)`) with:

```rust
        let Some(fit_size) = scaling::fitted_size(
            panel_ui.available_size(),
            painting.decoded_image.size(),
        ) else {
            return;
        };
```

5. DELETE the private `image_dimensions` fn and its doc comment (lines 83–86) — replaced by `DecodedImage::size()` (S3).
6. The doc comment on `paint` (lines 40–43) stays as-is (it already describes fit×zoom composition; no fit-mode reference there).

### S5 — Paint seam literal (`src/app/central_surface.rs`, 44 → 43)

1. DELETE line 40 (`fit_mode: self.fit_mode,`).
2. Header doc lines 2–3: reword `WHICH session fit mode and zoom factor paints` → `at WHICH session zoom factor paints`.

### S6 — App seam (`src/app/zoom.rs`, 168 → ~155)

1. REWORDED the module-doc fit paragraph (lines 13–15) to:

```text
//! One-shot fit semantics (Wave 4.1 Task 7): the two `set_zoom_to_fit_width`/
//! `set_zoom_to_fit_height` seams set the zoom factor ONLY; the `W`/`H` keys
//! and both fit buttons funnel through them. No mode state exists: after a
//! one-shot set the zoom in/out steps run from that level, `0`/restore and
//! every successful open reset to 1.0, and a window resize does NOT re-fit
//! (the paint multiplies the stored factor by the current whole-image fit
//! base every frame). The intake available size is the window client rect
//! (`screen_rect().size()`; spec §5: one-shot display lands within the
//! central-panel margins of a full-window span). Fit actions are SILENT.
```

2. DELETE `FIT_WIDTH_MESSAGE` + `FIT_HEIGHT_MESSAGE` consts (lines 43–46).
3. `restore_zoom_to_fit` body becomes ONLY:
   `self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;`
   and its doc comment becomes: `/// Hotkey `0` and the restore button: back to the neutral fit factor (1.0). Silent.`
4. DELETE `set_fit_mode` (lines 72–80), `announce_fit_mode` (lines 82–84), and the free fn `fit_mode_activation_label` (lines 160–168) entirely.
5. ADD the two seams inside `impl ImagesScrollerApp` (after `restore_zoom_to_fit`):

```rust
    /// One-shot fit-width set (Wave 4.1 Task 7): the zoom factor ONLY -- the
    /// width-fit factor of the CURRENT image against the CURRENT available
    /// size (clamped inside by the pure math). No `message_text` write: fit
    /// actions are silent. No-op while no image is open (covers the fresh
    /// empty state AND the browsed empty-folder state -- one `image_is_open`
    /// gate) and on a non-positive decoded payload (unreachable in practice).
    pub(super) fn set_zoom_to_fit_width(&mut self, available_size: egui::Vec2) {
        if !self.image_is_open() {
            return;
        }
        let Some(decoded_image) = self.decoded_image.as_ref() else {
            return;
        };
        let Some(factor) = crate::scaling::fit_width_factor(available_size, decoded_image.size())
        else {
            return;
        };
        self.zoom_factor = factor;
    }

    /// The height one-shot twin of `set_zoom_to_fit_width` (``H``/button).
    pub(super) fn set_zoom_to_fit_height(&mut self, available_size: egui::Vec2) {
        if !self.image_is_open() {
            return;
        }
        let Some(decoded_image) = self.decoded_image.as_ref() else {
            return;
        };
        let Some(factor) = crate::scaling::fit_height_factor(available_size, decoded_image.size())
        else {
            return;
        };
        self.zoom_factor = factor;
    }
```

(2 params each; single-section guards: `!self.image_is_open()` is the only boolean condition per `if`.)

6. KEEP `FIT_WIDTH_KEY`/`FIT_HEIGHT_KEY` consts, `zoom_can_grow`/`zoom_can_shrink`/`zoomed_off_fit`, all chords, `zoom_in_once`/`zoom_out_once` EXACTLY as landed.

### S7 — Keyboard intake (`src/app/keyboard_intake.rs`, 75 → 75)

1. Replace the two dispatch bodies (lines 51–56):

```rust
        if fit_width_pressed {
            self.set_zoom_to_fit_width(screen_context.screen_rect().size());
        }
        if fit_height_pressed {
            self.set_zoom_to_fit_height(screen_context.screen_rect().size());
        }
```

2. Header doc line 3: `"...and the fit-mode `W`/`H` keys..."` → `"...and the one-shot fit `W`/`H` keys..."`.
3. Reads (`fit_width_pressed`/`fit_height_pressed`) and read order UNCHANGED.

### S8 — Button module rename (`src/app/fit_mode_button.rs` → `src/app/fit_button.rs`, Option A)

Run: `git mv src/app/fit_mode_button.rs src/app/fit_button.rs`

Then inside `src/app/fit_button.rs`:

1. Header doc (lines 1–5) reword to: `//! The floating pill's one-shot fit buttons (Wave 4.1 Task 7): fit-width [W] and fit-height [H]. Painted line-and-triangle glyphs of the SAME vector family as the arrows -- no font glyphs. Clicking set the zoom factor ONLY (one-shot, silent). Enabled exactly while an image is open; rendered DISABLED while an empty folder is browsed (fresh empty state: the pill is absent anyway).`
2. DELETE `use crate::scaling::FitMode;` (line 12).
3. Rename types/methods (D6): `FitModeButtonAction` → `FitButtonAction` (3 occurrences incl. the enum + impl block line 131), `FitModeButtonSpec` → `FitButtonSpec`, `fit_mode_button_specs` → `fit_button_specs`, `show_fit_mode_pill_buttons` → `show_fit_pill_buttons`. KEEP the consts `FIT_WIDTH_BUTTON_LABEL`/`FIT_HEIGHT_BUTTON_LABEL` and ALL glyph/size/gap constants verbatim.
4. `show_and_apply_action` click branch becomes: `self.apply_action(app, row_ui);`
5. `apply_action` becomes (the row lets the button read the CURRENT window rect; D4):

```rust
    fn apply_action(&self, app: &mut ImagesScrollerApp, row_ui: &egui::Ui) {
        let available_size = row_ui.ctx().screen_rect().size();
        match self.action {
            FitButtonAction::FitWidth => app.set_zoom_to_fit_width(available_size),
            FitButtonAction::FitHeight => app.set_zoom_to_fit_height(available_size),
        }
    }
```

6. Inside `show_and_apply_action`, `PillButton::from_fit_mode_action(self.action)` → `PillButton::from_fit_action(self.action)` (S9 keeps the enum variant names).
7. `(matches!(self, Self::FitWidth))` in `glyph_shapes` — unchanged (now matches `FitButtonAction`).

### S9 — Button-hint plumbing (`src/app/button_hint.rs`, 184 → 184)

1. Line 9: `use super::fit_mode_button::FitModeButtonAction;` → `use super::fit_button::FitButtonAction;`
2. Lines 47–52: `from_fit_mode_action` → `from_fit_action`; types renamed to `FitButtonAction`; variant arms VERBATIM.
3. `hotkey_reference` and `tooltip_text` — UNTOUCHED (`Fit width (W)` / `Fit height (H)` stay exact).

### S10 — Pill wiring (`src/app/floating_controls.rs`, 191 → 191)

1. Line 13: `use super::fit_mode_button;` → `use super::fit_button;`
2. Line 84: `fit_mode_button::show_fit_mode_pill_buttons(app, row_ui);` → `fit_button::show_fit_pill_buttons(app, row_ui);`
3. Header doc line 4: `"...plus the fit-width/fit-height buttons (Wave-4 Task 3)..."` → `"...plus the one-shot fit-width/fit-height buttons..."`.

### S11 — Shell (`src/app.rs`, 183 → 179)

1. DELETE lines 38–39 (`/// Session fit mode ...` doc comment + `fit_mode: crate::scaling::FitMode,` field).
2. DELETE line 61 (`fit_mode: crate::scaling::FitMode::FitVisible,` in `empty_state`).
3. Line 88: `self.set_fit_mode(crate::scaling::FitMode::FitVisible);` → `self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;` (open-reset stays FACTOR-ONLY and silent; position within `accept_open_outcome` unchanged).
4. Line 159: `mod fit_mode_button;` → `mod fit_button;`
5. Line 169: `#[cfg(test)] mod fit_mode_tests;` → `#[cfg(test)] mod fit_one_shot_tests;`

### S12 — Test relocations

1. `src/app/zoom_tests.rs`: DELETE the single line 44 (`assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);`) inside `zoom_out_from_fit_shrinks_below_fit`. Every other test in this file is KEPT verbatim.
2. DELETE `src/app/fit_mode_tests.rs` (all 11 tests — fate in §6).
3. CREATE `src/app/fit_one_shot_tests.rs` — exact content skeleton:

```rust
//! One-shot fit app-state tests (Wave 4.1 Task 7): the factor-only set, its
//! silence, the no-op gates, the reset-to-1.0 policy, and the pinned hotkey /
//! button-label literals with the open-image enablement gate.

use eframe::egui;
use std::path::PathBuf;

use super::tests::sample_decoded_image;
use super::{ImagesScrollerApp, fit_button, zoom};
use crate::image_loading::DecodedImage;

const ONE_SHOT_AVAILABLE_SIZE: egui::Vec2 = egui::Vec2::new(400.0, 400.0);

fn tall_decoded_image() -> DecodedImage {
    // fitted at ONE_SHOT_AVAILABLE_SIZE = (200, 400): the WIDTH-first zoom
    // factor is 400/200 = 2.0, the HEIGHT-fit factor is 1.0 (height constrains).
    DecodedImage {
        width: 100,
        height: 200,
        pixels: vec![0; 100 * 200 * 4],
    }
}

fn wide_decoded_image() -> DecodedImage {
    // Mirror image: the HEIGHT one-shot factor is 2.0, the WIDTH one-shot 1.0.
    DecodedImage {
        width: 200,
        height: 100,
        pixels: vec![0; 200 * 100 * 4],
    }
}

fn open_decoded_image(image: DecodedImage) -> ImagesScrollerApp {
    let mut app = ImagesScrollerApp::empty_state();
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(image));
    app
}

#[test]
fn fit_width_sets_zoom_factor_to_width_fit_ratio() {
    let mut app = open_decoded_image(tall_decoded_image());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, 2.0); // avail.x / whole-image-fitted-width, exact
}

#[test]
fn fit_height_sets_zoom_factor_to_height_fit_ratio() {
    let mut app = open_decoded_image(wide_decoded_image());
    app.set_zoom_to_fit_height(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, 2.0); // avail.y / whole-image-fitted-height, exact
}

#[test]
fn fit_width_is_silent() {
    let mut app = open_decoded_image(tall_decoded_image());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    assert!(app.message_text.is_none());
}

#[test]
fn fit_height_is_silent() {
    let mut app = open_decoded_image(wide_decoded_image());
    app.set_zoom_to_fit_height(ONE_SHOT_AVAILABLE_SIZE);
    assert!(app.message_text.is_none());
}

#[test]
fn pressing_fit_width_twice_recomputes_the_same_factor() {
    let mut app = open_decoded_image(tall_decoded_image());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, 2.0);
    // No persistent mode may block zoom: steps run normal from the set level.
    app.zoom_out_once();
    assert_eq!(app.zoom_factor, 1.6); // 2.0 / 1.25, exact
    app.zoom_in_once();
    assert_eq!(app.zoom_factor, 2.0);
}

#[test]
fn fit_action_is_no_op_when_no_image_is_open() {
    let mut app = ImagesScrollerApp::empty_state();
    // The one `image_is_open` gate covers the fresh empty state AND the
    // browsed empty-folder state (both keep `opened_image_path` None).
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    app.set_zoom_to_fit_height(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, crate::zoom::RESTORE_TO_FIT_FACTOR);
    assert!(app.message_text.is_none());
}

#[test]
fn restore_fit_and_open_reset_zoom_factor_to_one() {
    let mut app = open_decoded_image(tall_decoded_image());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    assert_eq!(app.zoom_factor, 2.0);
    app.restore_zoom_to_fit();
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
    app.set_zoom_to_fit_width(ONE_SHOT_AVAILABLE_SIZE);
    app.accept_open_outcome(PathBuf::from("other.png"), Ok(wide_decoded_image()));
    assert_eq!(app.zoom_factor, 1.0);
    assert!(app.message_text.is_none());
}

#[test]
fn fit_width_hotkey_is_w() {
    assert_eq!(zoom::FIT_WIDTH_KEY, egui::Key::W);
}

#[test]
fn fit_height_hotkey_is_h() {
    assert_eq!(zoom::FIT_HEIGHT_KEY, egui::Key::H);
}

#[test]
fn fit_width_button_label_is_pinned() {
    assert_eq!(fit_button::FIT_WIDTH_BUTTON_LABEL, "Fit width");
}

#[test]
fn fit_height_button_label_is_pinned() {
    assert_eq!(fit_button::FIT_HEIGHT_BUTTON_LABEL, "Fit height");
}

#[test]
fn fit_buttons_enabled_only_when_image_is_open() {
    let mut app = ImagesScrollerApp::empty_state();
    assert!(!app.image_is_open());
    app.accept_open_outcome(PathBuf::from("photo.png"), Ok(sample_decoded_image()));
    assert!(app.image_is_open());
}
```

(`fit_buttons_enabled_only_when_image_is_open` consumes `sample_decoded_image` from `super::tests` — the import list above is complete.)

### S13 — Dev-checks (container, via `alpine-vm` MCP)

1. `vm_status` first; if the VM or `/img-scroller` share is unavailable, STOP and return a user question.
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
3. Contract: exit code authoritative; expected final `ALL CHECKS PASSED` and `104 passed; 0 failed` (fmt → check → test → clippy). cargo stderr absence is NOT a failure signal (documented MCP caveat).
4. If clippy/fmt fails: fix ONLY the mechanical issue it names in the files this plan touched; NO structural redesigns; re-run S13.

## 3. Test fate table (name-by-name; census arithmetic)

### Deleted outright (17)

`src/scaling/fit_mode_tests.rs` (whole file, 6):
| # | test name | why |
|---|-----------|-----|
| 1 | `fit_width_spans_the_available_width_and_overflows_vertically` | helper-geometry coverage transfers into `fit_factor_tests.rs` (tests 1,2,4,7 exercise the surviving size helpers transitively) |
| 2 | `fit_height_spans_the_available_height_and_overflows_horizontally` | same |
| 3 | `square_image_width_height_and_visible_fits_agree` | `FitMode::base_size` deleted |
| 4 | `fit_mode_base_sizes_difference_is_exact_on_a_wide_image` | same |
| 5 | `fit_visible_base_size_matches_the_whole_image_fitted_size` | same (paint base now calls `fitted_size` directly) |
| 6 | `non_positive_dimensions_reject_every_fit_mode` | replaced by `fit_factors_reject_non_positive_dimensions` |

`src/app/fit_mode_tests.rs` (whole file, 11):
| # | test name | why |
|---|-----------|-----|
| 1 | `fresh_app_starts_in_the_whole_image_fit_mode` | `fit_mode` field gone; `zoom_tests.rs::fresh_app_starts_zoomed_to_fit` already pins zoom 1.0 |
| 2 | `fit_width_activation_resets_the_factor_and_announces_fit_width` | superseded by `fit_width_sets_zoom_factor_to_width_fit_ratio` + `fit_width_is_silent` (no mode, no message) |
| 3 | `fit_height_activation_resets_the_factor_and_announces_fit_height` | superseded likewise |
| 4 | `fit_visible_activation_is_silent_and_resets_the_mode_and_factor` | `set_fit_mode` deleted; `zoom_tests.rs::restore_zoom_to_fit_returns_to_fit` covers the factor path |
| 5 | `zoom_steps_keep_the_active_fit_mode` | inversion kept by `pressing_fit_width_twice_recomputes_the_same_factor` (steps run AFTER the set) |
| 6 | `restore_fit_returns_to_the_whole_image_mode_at_the_fit_factor` | covered by `zoom_tests.rs::restore_zoom_to_fit_returns_to_fit` + new `restore_fit_and_open_reset_zoom_factor_to_one` |
| 7 | `zoom_out_below_fit_is_silent_then_fit_width_announces` | fit announces REMOVED (binding); `zoom_tests.rs::zoom_out_from_fit_shrinks_below_fit` already pins silence |
| 8 | `every_successful_open_returns_to_the_whole_image_fit` | covered by `zoom_tests.rs::every_successful_open_resets_the_zoom_to_fit` + new reset test |
| 9 | `fit_mode_hotkeys_are_pinned_to_the_w_and_h_keys` | re-homed (see "moved" table) |
| 10 | `fit_mode_button_labels_are_pinned` | re-homed (see "moved" table) |
| 11 | `fit_mode_button_enablement_gate_is_the_open_image_state` | re-homed (see "moved" table) |

### Added (19)

`src/scaling/fit_factor_tests.rs` (+7): (already listed in S2.6, spec §6.2 set + ceiling)
`src/app/fit_one_shot_tests.rs` (+12): (already listed in S12.3)
→ The five hotkey/label/enablement pins (spec §6.2 "relabels") reappear as five distinct `#[test]` fns: `fit_width_hotkey_is_w`, `fit_height_hotkey_is_h`, `fit_width_button_label_is_pinned`, `fit_height_button_label_is_pinned`, `fit_buttons_enabled_only_when_image_is_open`.

### Edits only (count unchanged)

- `src/app/zoom_tests.rs` (16 → 16): one assertion line out of `zoom_out_from_fit_shrinks_below_fit`.
- All other untouched test modules keep their counts.

### Final census: **102 − 17 + 19 = 104** (D9).

### Kept-verbatim pure-math modules

- `src/scaling.rs::mod tests` (5 `fitted_size` tests) and `src/zoom.rs::mod tests` (8) — unchanged.

## 4. Git actions

1. S0 docs/artifacts commit (first, before any code change): `docs: add wave4.1 task7 spec and plan`
2. After S13 passes: `git status --short` — expect exactly the changed `src/` files + the two new test files + the `fit_mode_button.rs` rename (shown delete/add) and NOTHING else. Verify the Gitignore Compliance checklist: no `dist/`, `logs/`, `*.log`, `target/` entries staged; no dependency dirs exist to stage.
3. Stage all: `git add src .agent` (the TODO file is NOT touched in this plan's execution — see §7).
4. Commit: `feat: fit keys and buttons set zoom one shot`
5. DO NOT push (git push is Wave-4.1 workflow Step 5 only). DO NOT touch `CHANGELOG.md`. DO NOT edit any existing `.kilo/plans/*` file.

## 5. Purity / budget verification (post-edit quick audit before S13)

| File | land | after | rule checks |
|------|------|-------|-------------|
| `src/scaling.rs` | 161 | ~152 | factor fns 2-arg, bodies ≤ 6 lines, depth ≤ 1; no comments-on-code; enum gone |
| `src/zoom.rs` | 128 | 129 | one doc line + `pub(crate)` keyword |
| `src/app/zoom.rs` | 168 | ~155 | seams 2 params, bodies ≤ 10 lines, single-section guards |
| `src/app/fit_button.rs` | 186 | ~185 | rename-only + apply_action rewire |
| `src/app/button_hint.rs` | 184 | 184 | import+method rename only |
| `src/app/floating_controls.rs` | 191 | 191 | import+call+doc line only |
| `src/app/keyboard_intake.rs` | 75 | 75 | dispatch swap only |
| `src/app.rs` | 183 | 179 | field/init/line swaps |
| `src/app/central_surface.rs` | 44 | 43 | one field out |
| `src/app_display.rs` | 121 | ~113 | base swap, field out, helper out |
| `src/image_loading.rs` | 86 | ~93 | +accessor impl |
| `src/app/zoom_tests.rs` | 158 | 157 | one assert line out |

Verify every file with the line-count command of your choice staying ≤ 200; report the observed figures in the completion summary.

## 6. 4.5a conformance mapping (next workflow step consumes spec §7)

- Spec §7 items about "clamp with `zoom::clamped_zoom_factor`" and the `(&mut self, ctx: &egui::Context)` signature are SATISFIED TRANSITIVELY/by D3+D4 (clamp moved inside the scaling fns; ctx rect read at the two intake callsites, seam takes `egui::Vec2`). Record these two as spec-deviation dispositions, both pre-approved by this plan.
- Everything else in spec §7 checks verbatim against the landed code.

## 7. Out-of-scope guard (hard)

- Tasks 4/5/6 landings: untouched (their behavior is consumed as-is at HEAD).
- Task 8 (verification cycle) and Task 9 (documentation cycle): NOT executed by this plan.
- The TODO file: no `[DONE]` marks in this execution (the Critical Workflow 4.6 mark belongs to a later step).
- Parked future-ideas, `CHANGELOG.md`, `dist/`, `logs/`: untouched.
- `.agent/project-structure.md` intentionally NOT updated here (rename belongs to the Task-9 docs cycle; see the touchpoint list below).

## 8. Docs-touchpoint list handed to Task 9

- `.agent/project-structure.md` — rename record (`fit_mode_button.rs` → `fit_button.rs`, `fit_mode_tests.rs` → `fit_one_shot_tests.rs` + `scaling/fit_mode_tests.rs` → `scaling/fit_factor_tests.rs`).
- `README.md` — `W`/`H` semantics (one-shot zoom-set, silent).
- `.agent/project-info/tech.md` — §7 mechanism rewrite (factor-only composition; `set_fit_mode` gone; silence).
- `.agent/project-info/architecture.md` — Module Layout members (`fit_button.rs`), the Manual-Zoom/fit-mode component paragraph, `ImagePainting` shape.
- `.agent/project-info/brief.md` + `.agent/project-info/product.md` — the brief's "Zoom In/Out and Fit Modes" gains the one-shot correction addendum (fits the Task-8 correction bundle).
- `.agent/project-info/context.md` — the Task-7 landing entry.
- Note: Task 3's `23611...`-era `fit_mode` landing story in `tech.md`/`architecture.md` is superseded by these updates (same file-set as the wave's Task-9 plan will enumerate).

## 9. Completion signals for this execution

- S13 `ALL CHECKS PASSED` with `104 passed; 0 failed`.
- Both commits exist on `fix/wave-4-operator-fixes` (docs commit, then `feat: fit keys and buttons set zoom one shot`), `git status --short` clean.
- Report: final per-file line counts (§5), final census printout (`rg -c "#\[test\]"` = the §3 table), and the two commit hashes.
