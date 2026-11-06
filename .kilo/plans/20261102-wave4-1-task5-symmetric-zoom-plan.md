# Wave 4.1 — Task 5 Implementation Plan: Zoom-out below the neutral fit level (symmetric floor)

- Source task: `.agent/todos/20261102/20261102-todo-1.md` → `### Task 5: Zoom-out below the neutral fit level`
- Branch: `fix/wave-4-operator-fixes` — HEAD at planning time: `f543da9` (`task: mark wave4.1 task4 message removal done`)
- Pre-state verified at planning time: `git status --short` EMPTY (no stray/untracked files); branch = `fix/wave-4-operator-fixes`; test census = **96 unit tests** (grep `#[test]` across `src`, all indentation styles).
- No new files are created by this task (no module additions); all edits are to existing files. No existing `.kilo/plans/*` file is edited.
- Commit message (caller-approved): `fix: allow zoom out below fit with symmetric floor`

---

## §0 User-approved bindings (verbatim restate — binding source for this plan)

Operator-reported user words, verbatim (chat 2026-11-02, operator report):

1. "the user should be able to zoom out even when that means to show the image at a size inferior that its original size. I mean, zoom at 0 means no zoom, but user can still zoom out, which makes the image to reduce its size."
2. "The zoom out limit from zoom 0 should be the same as zoom in max, but in reverse."

### §0.1 Derived encoding (decided, binding)

- The factor model is UNCHANGED in its meaning: `1.0` = the neutral fit / "0" no-zoom baseline.
- Zoom-in ceiling: `ZOOM_FACTOR_MAXIMUM = 8.0` — UNCHANGED.
- NEW floor: `ZOOM_FACTOR_MINIMUM = 1.0 / ZOOM_FACTOR_MAXIMUM` (= 0.125, the ÷8 of the fit size when the ceiling is ×8). This is the literal encoding of binding 2 ("same as zoom in max, but in reverse").
- Zoom-out steps divide by `ZOOM_STEP_MULTIPLIER` (1.25) as today, and CLAMP at the new floor — silently. NOTE: `1.0 / 1.25 = 0.8` and `1.0 / 8.0 = 0.125` are exact in f32, so test assertions with `assert_eq!` on these values are exact.

### §0.2 Supersession record (IMPORTANT — resolve the TODO wording conflict)

- The TODO Task-5 sub-item 3 reads: "Message/enablement schemes reflect the new range consistently (the landed rejected-step `Zoom max`/`Zoom min` design is reused, adjusted to the new bounds)."
- This wording is **SUPERSEDED** by Task 4 of this same wave, LANDED at `943d868` (`fix: remove zoom bound messages`): ALL zoom-step messages were REMOVED; zoom/fit actions are silent-by-default; the ONLY remaining messages are the fit activations (`Fit width` / `Fit height`) and the unsupported-file message. There is therefore NO message design to adapt — the new zoom-out range is SILENT at the floor (clamped steps write nothing to `message_text`), exactly like the ceiling behaves today.
- This plan encodes silence. The implementer must NOT reintroduce any `Zoom min` / `Zoom max` / `Zoom floor` message string.

### §0.3 All other landed zoom contracts UNCHANGED

- Hotkey chord sets: `I` (zoom in) / `O` (zoom out) / `0` (`Key::Num0`, restore fit) — unchanged in this task. The alias family correctness (Shift+`+` / Shift+`-` only; plain `+`/`=`/`-` inert) is **Task 6 — OUT OF SCOPE here**; `src/app/zoom.rs` chord functions (`zoom_in_shift_alias_was_pressed`, `zoom_out_chord_was_pressed`) and `src/app/keyboard_intake.rs` are NOT touched by this plan.
- Buttons `[+][−][restore-fit]` — unchanged labels, glyphs, placement, click seams.
- Step model: ×1.25 multiplicative (in), ÷1.25 multiplicative (out) — unchanged function shapes.
- Per-successful-open reset to fit (mode `FitVisible`, factor 1.0) via `set_fit_mode` in `accept_open_outcome` — unchanged.
- Restore-to-fit (`0` key / restore-fit button) → factor 1.0 + `FitVisible` — unchanged.
- Session-only state, no persistence — unchanged.
- `ZOOM_BOUND_TOLERANCE = 1.0e-4` — unchanged value and semantics (absorption guard for the at-bound detections).
- `zoomed_draw_size` — unchanged (already a plain multiplication; a factor of 0.8 paints BELOW fit naturally).

### §0.4 Button enablement (binding)

- zoom-in enabled ⇔ factor < 8.0 (unchanged semantics; predicate `zoom_can_grow` stays).
- zoom-out enabled ⇔ factor > 1/8 (NEW — today zoom-out is disabled exactly at fit 1.0; it must now be enabled there and everywhere above the floor).
- restore-fit enabled ⇔ factor ≠ 1.0 (unchanged semantics — note this is a DIFFERENT predicate from the zoom-out one now: at the floor 0.125 the zoom-out button is DISABLED while the restore-fit button is ENABLED).
- The at-bound dimming path is verified in research: `src/app/zoom_button.rs` derives `enabled` per frame in `zoom_button_specs` (reads the app predicates), and `zoom_glyph_color` paints the disabled palette (`widgets.noninteractive.fg_stroke.color`) whenever `!self.enabled` — the dimming follows the predicates automatically. Only the predicate names/semantics change; the dimming path itself needs NO rework.

### §0.5 Task-7 forward-compat note (record only — NO implementation here)

Task 7 (one-shot fit semantics) will recomputes the FACTOR directly from fit-size ratios (e.g. `fit_width_size / fitted_size`). Under this plan a factor set that way is just an in-range f32 value inside `[1/8, 8]`; nothing in the floor/ceiling clamp, the predicates, or the buttons special-cases where the factor came from. Composition is automatic: any W/H-resolved factor zooms in/out normally and clamps at the same bounds. No hook, no flag, no special case is added now.

---

## §1 Ordered implementation steps (junior-executable; exact snippets)

Steps are in dependency order. After every step, re-check the file obeys: ≤ 200 lines total, methods ≤ 50 lines, max nesting depth 2, self-documenting names, no comments explaining the obvious (the doc comments listed below are CONTRACT statements, keep them).

### Step 1 — `src/zoom.rs`: constants + pure predicates + doc truth-up

File: `C:\repo\img-scroller\src\zoom.rs` (currently 113 lines).

1.1. NOT touched (verify only): `ZOOM_STEP_MULTIPLIER`, `zoom_in_factor`, `zoom_out_factor` bodies, `clamped_zoom_factor`, `ZOOM_BOUND_TOLERANCE`, `zoom_factor_is_at_maximum`, `zoomed_draw_size`, and all inline tests except as listed in §2. `clamped_zoom_factor` already clamps via the two constants, so changing the constant re-clamps zoom-out automatically — confirm `zoom_out_factor`'s clamp target is `clamped_zoom_factor` (it is; no edit).

1.2. REORDER + REPLACE the two bound constants (current lines 14–17) to:

```rust
/// Upper zoom bound: the largest supported zoom-in.
pub(crate) const ZOOM_FACTOR_MAXIMUM: f32 = 8.0;
/// Lower zoom bound: the SYMMETRIC zoom-out floor -- the exact inverse of
/// the ceiling (when the ceiling is ×8, zoom-out reaches ÷8 of the fit size).
pub(crate) const ZOOM_FACTOR_MINIMUM: f32 = 1.0 / ZOOM_FACTOR_MAXIMUM;
```

(Reordering puts `ZOOM_FACTOR_MAXIMUM` first so the floor's const expression reads top-down; Rust would also accept the forward reference, but the reorder removes all doubt for a junior editor.)

1.3. REPLACE `RESTORE_TO_FIT_FACTOR` (current line 18–19) — it must STOP aliasing `ZOOM_FACTOR_MINIMUM` (the floor is no longer the fit level):

```rust
/// The restore-to-fit target: the neutral fit baseline ("0" / no-zoom),
/// NOT the zoom-out floor -- fit is the zero point INSIDE the symmetric
/// range [ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM].
pub(crate) const RESTORE_TO_FIT_FACTOR: f32 = 1.0;
```

1.4. UPDATE the doc line on `zoom_factor_is_at_minimum` (current line 37): old text is "Whether `factor` has reached the fit floor (zoom-out/restore are inert there)." — becomes:

```rust
/// Whether `factor` has reached the symmetric zoom-out floor (zoom-out is
/// inert there; the restore-fit button is NOT — it re-anchors to 1.0).
```

(Function body unchanged: `factor <= ZOOM_FACTOR_MINIMUM + ZOOM_BOUND_TOLERANCE`.)

1.5. ADD one new pure predicate right after `zoom_factor_is_at_maximum` (needed by the new restore-fit enablement, see Step 2.4):

```rust
/// Whether `factor` sits AWAY from the neutral fit baseline (1.0) outside
/// the bound tolerance — the restore-fit-button enablement. True at both
/// bounds and everywhere between except fit itself.
pub(crate) fn zoom_factor_is_off_restore_target(factor: f32) -> bool {
    (factor - RESTORE_TO_FIT_FACTOR).abs() > ZOOM_BOUND_TOLERANCE
}
```

1.6. UPDATE the module doc (`//!` block, current lines 1–8): replace the words that say the floor is fit / zoom-out below fit is rejected. The rewritten module doc must state, exactly:
- the multiplicative step scaling within fixed SYMMETRIC bounds `[ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM]` around the neutral fit factor `1.0` (the "0" no-zoom baseline);
- zoom-out BELOW fit is allowed and reaches ÷8 (`ZOOM_FACTOR_MINIMUM`) when the ceiling is ×8;
- the clamp is silent and zoom actions write no messages (the silent at-bound no-op and the reset-on-every-successful-open policy live in the app seam — keep those two existing sentences).

1.7. Line-count check: the file must stay ≤ 200 lines (expected ≈ 125).

### Step 2 — `src/app/zoom.rs`: rename the bound predicate, add the restore one, doc rewrite

File: `C:\repo\img-scroller\src\app\zoom.rs` (currently 132 lines).

2.1. PREDICATE NAMING DECISION (encoded, binding): `zoomed_beyond_fit` is RENAMED and RETARGETED to **`zoom_can_shrink`**.
- Why: `zoomed_beyond_fit` asserted `factor > 1.0` ("factor left the fit level"), and it fed BOTH the zoom-out and restore-fit enablement. Under the new range its old name is a LIE — at fit (factor 1.0) zoom-out now SUCCEEDS, and "beyond fit" is no longer what gates zoom-out; what gates it is the floor. Keeping an inverted-meaning name misleads every future reader. `zoom_can_shrink` states the new truth: enabled while the factor is above the floor.
- Body change: from `!zoom::zoom_factor_is_at_minimum(self.zoom_factor)` (unchanged expression, but now it means "above the floor" instead of "off the fit level").

Replace the whole tail predicate block (current lines 80–89) with:

```rust
    /// Zoom-in-button enablement: enabled until the ×8 ceiling is reached.
    pub(super) fn zoom_can_grow(&self) -> bool {
        !zoom::zoom_factor_is_at_maximum(self.zoom_factor)
    }

    /// Zoom-out-button enablement: enabled while the factor is above the
    /// symmetric floor (`ZOOM_FACTOR_MINIMUM`) -- zoom-out now works BELOW
    /// fit, so the fit level itself does NOT disable it (only the floor does).
    pub(super) fn zoom_can_shrink(&self) -> bool {
        !zoom::zoom_factor_is_at_minimum(self.zoom_factor)
    }

    /// Restore-fit-button enablement: enabled whenever the factor is away
    /// from the neutral fit value 1.0 (including at the zoom-out floor,
    /// where the zoom-out button is already disabled).
    pub(super) fn zoomed_off_fit(&self) -> bool {
        zoom::zoom_factor_is_off_restore_target(self.zoom_factor)
    }
```

(Satisfies the naming/enablement split safety: the zoom-out button reads `zoom_can_shrink`, the restore-fit button reads `zoomed_off_fit` — the restore button stays ENABLED at the floor per §0.4.)

2.2. `zoom_in_once` — NO change (guard reads `zoom_factor_is_at_maximum`, ceiling semantics unchanged). Verify only.

2.3. `zoom_out_once` — NO body change; guard already returns when `zoom_factor_is_at_minimum` (now the floor) and otherwise steps via `zoom_out_factor`. Update ONLY its doc sentence: old "a step attempted at the fit floor is a silent no-op" → "a step attempted at the symmetric floor is a silent no-op".

2.4. `restore_zoom_to_fit` and `set_fit_mode` — NO change (`set_fit_mode` still assigns `crate::zoom::RESTORE_TO_FIT_FACTOR`, which now unhappily reads 1.0 as intended — this is exactly why Step 1.3 re-pins the constant).

2.5. MODULE DOC rewrite (the `//!` block, current lines 1–18): keep the D4-chord and the module-separation paragraphs; rewrite the RANGE statements to:
- the session factor lives in the symmetric range `[ZOOM_FACTOR_MINIMUM, ZOOM_FACTOR_MAXIMUM]` = `[1/8, 8]` with `1.0` the neutral fit / no-zoom baseline;
- zoom-out steps divide by 1.25 and keep going BELOW fit down to the floor; a step attempted at this floor is a silent no-op; a step landing on any bound clamps silently;
- zoom actions write NOTHING to `message_text` (wave 4.1 Task 4 binding);
- `zoomed_off_fit` drives ONLY the restore-fit button; `zoom_can_shrink` drives ONLY the zoom-out button (they diverge at the floor).

2.6. Line-count check: ≤ 200 lines (expected ≈ 145).

### Step 3 — `src/app/zoom_button.rs`: propagate the predicate split

File: `C:\repo\img-scroller\src\app\zoom_button.rs` (196 lines — the leakiest file; touch ONLY the two spec lines).

3.1. In `zoom_button_specs` (current lines 45–51) replace the enablement arguments:

```rust
        ZoomButtonSpec::row_button(ZoomButtonAction::ZoomOut, app.zoom_can_shrink()),
        ZoomButtonSpec::row_button(ZoomButtonAction::RestoreFit, app.zoomed_off_fit()),
```

(ZoomIn line unchanged: `app.zoom_can_grow()`.)

3.2. NOTHING ELSE changes in this file: glyphs, palette, click seam, labels, sizes all stay. The dimming path was verified (per-frame predicate read; `!enabled` → `noninteractive` palette; `accepts_click` gate) — it already paints disabled exactly when the (renamed/fresh) predicates say so. No behavior code is added.

3.3. Line-count check: file stays at 196 lines; do NOT reformat siblings.

### Step 4 — NO other production seams: verify-only checklist

- `src/app/keyboard_intake.rs` — untouched; its doc's "silent" wording stays true (zoom steps are silent; the intake dispatches the same `zoom_out_once`).
- `src/app_display.rs` / `src/app/central_surface.rs` — untouched; `zoomed_draw_size(fit, 0.8)` < fit draws smaller than fit naturally, no guard exists to remove.
- `src/app/floating_controls.rs` message line — untouched (zoom stays silent; only fit activations write).
- `src/scaling.rs` — untouched (Task 7 territory; see §0.5 note).
- Grid-check: run `grep -n "ZOOM_FACTOR_MINIMUM\|RESTORE_TO_FIT_FACTOR\|zoomed_beyond_fit\|zoom_can_shrink\|zoom_factor_is_at_minimum" src` after Steps 1–3 and confirm NO occurrence of `zoomed_beyond_fit` remains anywhere, and `RESTORE_TO_FIT_FACTOR` references still resolve to the re-pinned 1.0.

---

## §2 Test plan (exact fates; exact census)

Baseline verified at HEAD `f543da9`: **96 tests**. Final planned census: **97** (+1, exactly three files touched, all deltas accounted below).

### §2.1 `src/zoom.rs` inline tests (8 → 8; all in-place reworks, net 0)

1. `zoom_out_step_divides_by_the_step_multiplier` — UNCHANGED (10→8 and 5→4 are still in-range divisions).
2. `zoom_out_never_goes_below_fit` — DIES as named; same slot becomes `zoom_out_clamps_at_the_symmetric_floor`:

```rust
    #[test]
    fn zoom_out_clamps_at_the_symmetric_floor() {
        assert_eq!(zoom_out_factor(1.0), 0.8); // below fit is now REACHED
        assert_eq!(zoom_out_factor(0.1), ZOOM_FACTOR_MINIMUM); // 0.1/1.25 clamps
        assert_eq!(zoom_out_factor(ZOOM_FACTOR_MINIMUM), ZOOM_FACTOR_MINIMUM);
    }
```

3. `zoom_factor_is_at_minimum_is_exact_at_the_fit_bound` — renamed `zoom_factor_is_at_minimum_is_exact_at_the_symmetric_floor`; bodies become:

```rust
        assert!(zoom_factor_is_at_minimum(ZOOM_FACTOR_MINIMUM));
        assert!(!zoom_factor_is_at_minimum(1.0)); // the fit level is mid-range now
```

(keep the second old-style line pattern; do not test 1.25 against the floor).
4. `zoom_factor_is_at_maximum_is_exact_at_the_ceiling` — UNCHANGED.
5. `zoom_in_step_multiplies_by_the_step_multiplier`, `zoom_in_clamps_at_the_maximum`, `zoomed_draw_size_multiplies_the_fit_size_by_the_zoom_factor` — UNCHANGED.
6. `zoom_bounds_and_restore_constant_are_pinned` — UPDATED in place (no new test added; the symmetric relation assert is FOLDED IN):

```rust
        assert_eq!(ZOOM_STEP_MULTIPLIER, 1.25);
        assert_eq!(ZOOM_FACTOR_MAXIMUM, 8.0);
        assert_eq!(ZOOM_FACTOR_MINIMUM, 0.125);
        assert_eq!(ZOOM_FACTOR_MINIMUM * ZOOM_FACTOR_MAXIMUM, 1.0); // symmetric inverse
        assert_eq!(RESTORE_TO_FIT_FACTOR, 1.0);
```

7. NEW pure-math adjacency is NOT added (the relation assert in item 6 covers the symmetry pin; census held at +1 app-level only).

### §2.2 `src/app/zoom_tests.rs` (10 → 11; net +1)

1. `fresh_app_starts_zoomed_to_fit` — UPDATED in place: the `!app.zoomed_beyond_fit()` line becomes BOTH:

```rust
    assert!(!app.zoom_can_shrink()); // fit is mid-range; shrink is gated by the FLOOR… 
```

NO — careful: at a fresh start the factor IS 1.0 (mid-range), so the fresh app CAN shrink. The test body becomes:

```rust
    #[test]
    fn fresh_app_starts_zoomed_to_fit() {
        let app = ImagesScrollerApp::empty_state();
        assert_eq!(app.zoom_factor, crate::zoom::RESTORE_TO_FIT_FACTOR);
        assert!(!app.zoomed_off_fit()); // the restore-fit button is DISABLED at fit
        assert!(app.zoom_can_shrink()); // zoom-out is now ENABLED at fit (goes below)
        assert!(app.zoom_can_grow());
    }
```

2. `zoom_in_once_zooms_beyond_fit` — UPDATED in place: `assert!(app.zoomed_beyond_fit());` → `assert!(app.zoomed_off_fit());` plus keep `assert!(app.zoom_can_shrink());` and `assert!(app.zoom_can_grow());`.
3. `restore_zoom_to_fit_returns_to_fit` — UNCHANGED.
4. `zoom_out_at_fit_is_rejected_silently` — **DIES** (the behavior is REMOVED: zoom-out at fit no longer stops; it now steps DOWN below fit). Same slot (1:1 replacement, census-neutral):

```rust
    #[test]
    fn zoom_out_from_fit_shrinks_below_fit() {
        let mut app = ImagesScrollerApp::empty_state();
        app.zoom_out_once();
        assert_eq!(app.zoom_factor, 0.8); // 1.0 / 1.25, exact in f32
        assert_eq!(app.fit_mode, crate::scaling::FitMode::FitVisible);
        assert!(app.zoom_can_shrink());
        assert!(app.message_text.is_none()); // silent — Task-4 binding in force
        app.zoom_out_once();
        assert_eq!(app.zoom_factor, 0.64);
        assert!(app.message_text.is_none());
    }
```

5. NEW test (+1, the census delta):

```rust
    #[test]
    fn repeated_zoom_out_steps_reach_the_floor_then_stay() {
        let mut app = ImagesScrollerApp::empty_state();
        for _ in 0..10 {
            app.zoom_out_once(); //            10 × ÷1.25 from 1.0 ≈ 0.107 < 0.125
        }
        assert_eq!(app.zoom_factor, crate::zoom::ZOOM_FACTOR_MINIMUM);
        assert!(!app.zoom_can_shrink()); //    disabled at the floor
        assert!(app.zoomed_off_fit()); //      restore-fit still ENABLED at the floor
        app.zoom_out_once(); //                inert silent no-op at the floor
        assert!(app.message_text.is_none());
        assert_eq!(app.zoom_factor, crate::zoom::ZOOM_FACTOR_MINIMUM);
    }
```

(Math note for the implementer: step 9 of the loop is the first crossing below 0.125 → clamps exactly to `ZOOM_FACTOR_MINIMUM`; the 10th and extra steps stake out the silent no-op. The clamp makes the final value EXACT, so `assert_eq!` is safe.)

6. `repeated_zoom_in_reaches_the_ceiling_then_stays_there` — UNCHANGED (silent at-ceiling assertion stays).
7. `restore_fit_hotkey_is_num0` — UNCHANGED.
8. `fullscreen_toggles_do_not_touch_the_zoom_state` — UNCHANGED.
9. `every_successful_open_resets_the_zoom_to_fit` — UNCHANGED.
10. `failed_open_keeps_the_zoom_factor` — UNCHANGED.
11. `zoom_button_accessible_labels_are_pinned` — UNCHANGED.

### §2.3 `src/app/fit_mode_tests.rs` (11 → 11; rework in place, net 0)

1. `zoom_out_at_fit_is_silent_then_fit_width_announces` — DIES as named (its precondition — zoom-out REJECTED at fit — no longer exists). REWORK, same slot, renamed `zoom_out_below_fit_is_silent_then_fit_width_announces`:

```rust
    #[test]
    fn zoom_out_below_fit_is_silent_then_fit_width_announces() {
        let mut app = ImagesScrollerApp::empty_state();
        app.zoom_out_once();
        assert_eq!(app.zoom_factor, 0.8); // zoom-out now SUCCEEDS below fit
        assert!(app.message_text.is_none());
        app.set_fit_mode(crate::scaling::FitMode::FitWidth);
        assert_eq!(app.message_text, Some(zoom::FIT_WIDTH_MESSAGE));
    }
```

(the fit-width-announce assertion is KEPT — it was the point of the second half).
2. EVERY other fit-mode test — UNCHANGED (`fit_visible_activation…` still asserts message cleared + factor 1.0; `restore_fit_returns_to_the_whole_image_mode_at_the_fit_factor` unchanged; `zoom_steps_keep_the_active_fit_mode` unchanged; reset-on-open unchanged).

### §2.4 Coverage map (what each dying/changed behavior proves)

- Died: "zoom-out at fit is a no-op" → replaced by "zoom-out at fit steps to 0.8".
- Died: "fit is the lower clamp edge" → replaced by "0.125 is the lower clamp edge, symmetric to 8.0".
- Survived/unchanged: silent-clamp semantics, ceiling behavior, restore semantics, reset-on-open, fullscreen interplay, undo/failure-message interplay.

### §2.5 Exact census statement

96 → **97 unit tests** (deltas: zoom_tests.rs 10→11 = +1; zoom.rs 8→8 and fit_mode_tests.rs 11→11 net 0; every other test file byte-identical). The implementer's harness run MUST print `97 passed; 0 failed` — any other total means a step was skipped or a stranger test appeared: STOP and report.

---

## §3 Harness, commit, pre-commit checklist

3.1. Harness (per `docs/build.md`; ALL cargo in the container on the Alpine VM):

1. `vm_status` FIRST (alpine-vm MCP); only proceed when running + SSH-reachable. If the VM or `/img-scroller` share is unavailable → STOP and return a user question.
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
3. Expected per-check PASS lines: `fmt-check`, `check`, `test`, `clippy`; final line `ALL CHECKS PASSED`; test line census `97 passed; 0 failed`. The command EXIT CODE 0 is the authoritative evidence.
4. On fmt drift ONLY: apply `cargo fmt` per-file for the three touched files inside the container and re-run the full harness (never partial re-runs).

3.2. Pre-commit checklist (Gitignore Compliance Rule):

1. `git status` — confirm ONLY these three files modified: `src/zoom.rs`, `src/app/zoom.rs`, `src/app/zoom_button.rs`, plus the two test files if the reworks landed in the same pass: `src/app/zoom_tests.rs`, `src/app/fit_mode_tests.rs`. NO other file staged or unstaged.
2. Re-read `.gitignore`; assert no staged path matches (`dist/`, `logs/`, `target/` are gitignored — none of our files are under them).
3. `git diff --stat` sanity: the 5 files, all within `src/`.

3.3. Commit (single commit, meaningful message, exact):

```
git add src/zoom.rs src/app/zoom.rs src/app/zoom_button.rs src/app/zoom_tests.rs src/app/fit_mode_tests.rs
git commit -m "fix: allow zoom out below fit with symmetric floor"
```

- NO push in this task (push is Step-5 restricted). NO merge. NO version bump (Step-3 restricted; already at 0.4.1 path set by the global plan).
- TODO file marking `[DONE]`/`[x]` belongs to step 4.6, NOT this step — do not touch `20261102-todo-1.md`.

---

## §4 Out-of-scope guard (hard blocks for the implementer)

1. NO alias/chord changes to `zoom_in_shift_alias_was_pressed` / `zoom_out_chord_was_pressed` / `keyboard_intake.rs` — Task 6 owns the strict-shift correction.
2. NO fit-model changes (`src/scaling.rs`, `set_fit_mode` semantics, FitMode seams) — Task 7 owns the one-shot remap. The only allowed delta there is the §0.5 forward-compat NOTE (no code).
3. NO message strings introduced anywhere for zoom steps — the Task-4 REMOVE-ALL-zoom-messages binding is IN FORCE (§0.2).
4. NO `CHANGELOG.md` edit; NO edits to existing `.kilo/plans/*` files; NO parked future-ideas implementation; NO unrelated Task items (1/2/3/4/6/7/8/9).
5. NO button-glyph/layout/tooltip changes; `src/app/button_hint.rs` tooltips ("Zoom out (O)" etc.) stay as landed.
6. NO persistence, session state shape changes, or new app fields.

## §5 Documentation touchpoints for Task 9 (LIST ONLY — no edits in this step)

These carry the old [1.0, 8.0] / "Zoom min" wording and must be updated by Task 9 with the symmetric range (silent floor, restore-vs-zoomout button split at the floor):

1. `README.md` — lines 24 (feature blurb: bounds 1.0–8.0, `Zoom max`/`Zoom min` message sentence) and 67 (wave-4 state line).
2. `.agent/project-info/tech.md` — lines 5 (status paragraph), 89 (state bounds), 91 (zoom mechanism: bounds, disabled-exactly-at-fit wording, `Zoom max`/`Zoom min` sentence), 92 (fit seam paragraph's stale same-frame message sentence).
3. `.agent/project-info/architecture.md` — lines 5, 35 (`zoom_factor` [1.0, 8.0]), 58 (`src/app/zoom.rs` module description: rejected-step messages), 63 (`src/zoom.rs` description), 80 (path 4: bounds + `Zoom max`/`Zoom min`), 86 (state bounds), 125 (decision table "Zoom policy" row).
4. `.agent/project-structure.md` — line 14 (zoom.rs blurb names the [1.0, 8.0] bounds).
5. `.agent/project-info/context.md` — the caller's closing step adds the Task-5 entry (NOT this step).
6. `brief.md` + `product.md` — Task 9's binding addendum (symmetric-fit-baseline note) — out of this step.

## §6 Verification of this plan against the original Task-5 block

| TODO sub-item | Plan coverage |
| --- | --- |
| Zoom-out may reduce the image BELOW the neutral fit size (fit = the "0" baseline) | §0.1 + Steps 1.3/1.4/2.1 + tests §2.2-4/§2.3-1 |
| Zoom-out lower limit SYMMETRIC to the zoom-in upper limit (×8 → ÷8), exact numeric encoding decided here | §0.1 (`1.0/8.0 = 0.125`), Step 1.2, folded predicate in seq. test §2.1-6 |
| Message/enablement schemes reflect the new range consistently | Messages: silence per §0.2 supersession (Task-4 binding). Enablement: §0.4 + Steps 2.1/3.1 |
| Wave-4.1 execution notes (harness, commit style, no CHANGELOG/plans edits) | §3, §4, §5 |

The TODO's "exact numeric encoding decided in this wave's architector step" is hereby DECIDED: floor = `1.0 / ZOOM_FACTOR_MAXIMUM` (0.125), restore target re-pinned independently at 1.0, enablement split encoded, silent clamping everywhere, census 96 → 97 exactly.

---

*Plan saved by architector (4.1b) — implementation NOT performed here.*
