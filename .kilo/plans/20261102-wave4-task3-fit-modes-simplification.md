# Wave 4 Task 3 — Fit Width/Height Modes: 4.3 Code Simplification Plan

Date: 2026-11-02 · Critical Workflow step 4.3 (code-simplifier) for Wave-4 Task 3
Branch: `feat/wave-4-fullscreen-zoom` · HEAD reviewed: `f6facbb` (feature commit; `git show --stat` file set = the 11 reviewed files)
Implementation plan reviewed: `.kilo/plans/20261102-wave-4-task3-fit-modes-plan.md` (§0 + §0a bindings respected — nothing below contradicts them)
Scope reviewed: `src/app/central_surface.rs`, `src/app/fit_mode_button.rs`, `src/app/fit_mode_tests.rs`, `src/scaling/fit_mode_tests.rs`, `src/app.rs`, `src/app_display.rs`, `src/scaling.rs`, `src/app/floating_controls.rs`, `src/app/keyboard_intake.rs`, `src/app/zoom.rs`, `src/app/fullscreen.rs`

## Verdict summary

The Task-3 implementation is clean and faithful to the plan: the fit math split + `base_size` dispatch, the single-source `ImagePainting` literal (`central_surface.rs`), the one reset point at the Ok arm, the intake ordering (fit dispatches after zoom), and the key/label pins all landed as encoded; 93/93 test census grep-verified (76 + 6 + 11). All files ≤200, bodies ≤50, depth ≤2. Three behavior-neutral trims apply: S1/S2 are the Task-2-S1/S3-precedent duplicate-doc removals, S3 merges the two positional-duplicate glyph twin builders in the 193-line cap-pressure file (−25 lines). Expected end state: `src/app.rs` 199→198, `src/app/fit_mode_button.rs` 193→166, every other file unchanged. Everything else considered was REJECTED with reasons below. The 3-arg helpers / two-section boolean in `fit_mode_button.rs` are NOT patched here (pending review-side adjudication — see the coordination section). No new TODO file needed — all three steps fit one small fix commit.

## S1 — `src/app/fit_mode_button.rs`: remove the palette doc duplicated from the `floating_controls::arrow_color` canonical

Justification: lines 90–91 are a paraphrase-copy of the canonical palette wording that lives once in `floating_controls.rs` `arrow_color` (the Task-2 simplification S1 removed the identical copy from `zoom_button.rs` — same rule, same fix). The three-return ladder is self-evident from the code (`noninteractive`/`hovered`/`inactive` appear in each return expression). Keeps exactly one canonical instance of the wording.

Location: `impl FitModeButtonSpec`, method `fit_glyph_color` — doc comment only, not the signature or body.

Change (exact; delete these two lines):

```rust
// BEFORE (lines 90–91, immediately above the fn signature):
    /// Default egui button-text palette: idle in `inactive`, brightened on
    /// hover, dimmed in `noninteractive` while disabled (the arrow palette).

// AFTER: (no doc line; `fn fit_glyph_color(` follows the previous method's closing `}` + blank line directly)
```

Notes for the implementer: delete ONLY the two `///` lines; do not touch the function, its multi-line signature, or any other file. Diff: 2 lines deleted.

Verification claim (behavior-neutral): comment-only hunk; `wc -l src/app/fit_mode_button.rs` == 191; `grep -c "Default egui button-text palette" src/app/fit_mode_button.rs` == 0 and still exactly 1 hit repo-wide (in `floating_controls.rs`); `cargo fmt --check` clean (fmt never rewrites comments); tests stay 93/93 (no test references a doc line).

## S2 — `src/app.rs`: merge the Task-3 `fit_mode` field doc from 2 to 1 physical line

Justification: line-budget pressure (the caller-designated simplification motive; Task-2 simplification S3 set the precedent on the adjacent `zoom_factor` doc): `app.rs` sits at 199/200 with zero headroom. The dropped clause "set by the fit seam; reset on every successful open; never persisted" is a verbatim-policy restatement of the `zoom_factor` doc two lines above (identical session policy, both owned by the same `set_fit_mode` seam, visible in `accept_open_outcome`). The facts unique to this field — the Wave-4 Task-3 attribution and the `FitVisible` = whole-image-fit meaning — are preserved.

Location: `pub struct ImagesScrollerApp`, the last field's doc comment (lines 38–39).

Change (exact; replace these two lines with this one — same surviving words):

```rust
// BEFORE (lines 38–39):
    /// Session fit mode (Wave-4 Task 3): `FitVisible` = whole-image fit;
    /// set by the fit seam; reset on every successful open; never persisted.

// AFTER (one line, 96 chars incl. indent — inside rustfmt's 100-col limit and `cargo fmt` never rewraps comments):
    /// Session fit mode (Wave-4 Task 3): `FitVisible` = whole-image fit; reset on every open.
```

Notes for the implementer: the field line `fit_mode: crate::scaling::FitMode,` stays byte-identical; do not touch the `zoom_factor` doc above it. Diff: 1 line deleted.

Verification claim (behavior-neutral): comment-only hunk; `wc -l src/app.rs` == 198; `grep -n "fit_mode: crate::scaling::FitMode," src/app.rs` unchanged; tests stay 93/93 (`fresh_app_starts_in_the_whole_image_fit_mode` asserts the code, not the doc).

## S3 — `src/app/fit_mode_button.rs`: merge the twin glyph builders + their dispatch into one `FitModeButtonAction` method

Justification (per the caller's "delete redundant duplicate helpers" directive): `fit_width_glyph_shapes` and `fit_height_glyph_shapes` (37 lines) are pure positional duplicates — identical 3-shape assembly differing only in which literal (`true`/`false`) they pass as the third `axis_line_shape` argument and which same-family triangle pair they place at `center ∓ (SPACING, 0)` vs `center ∓ (0, SPACING)`. The `horizontal` discriminator is already the plan-encoded concept (the `axis_line_shape(…, horizontal: bool)` param). The dispatch method `fit_glyph_shapes` (10 lines) exists only to pick the twins. One 24-line `&self` method replaces all three (−25 net lines — the real headroom the 193/200 file needs). This is NOT the Task-2 REJECTED #5 case: the trap twins encode distinct D2 semantics (two different bound constants), sit in a file with 91 lines of headroom, and had no shared discriminator; these builders share everything. The two 3-arg helpers the merged method calls (`axis_line_shape`, `fit_arrow_triangle_shape`) are left byte-identical (review-side pending — coordination section).

Location: `impl FitModeButtonSpec` (loop line + `fit_glyph_shapes` removal) and the free-function block after it (`fit_width_glyph_shapes`/`fit_height_glyph_shapes` replacement). Line references are HEAD `f6facbb` (before S1); S1's hunk (lines 90–91) does not overlap any S3 hunk.

Change (three exact edits):

```rust
// EDIT (a) — line 109, inside paint_glyph, one line changed:
// BEFORE:
        for glyph_shape in self.fit_glyph_shapes(glyph_center, glyph_color) {
// AFTER:
        for glyph_shape in self.action.glyph_shapes(glyph_center, glyph_color) {

// EDIT (b) — delete lines 114–125 (the doc + the whole fit_glyph_shapes method;
// the blank line 113 and the impl closing `}` line 126 stay):
    /// Per-action glyph geometry: one axis line plus the two opposing
    /// triangles of the painted-arrow family (`Shape::rotated_triangle`).
    fn fit_glyph_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        match self.action {
            FitModeButtonAction::FitWidth => fit_width_glyph_shapes(glyph_center, glyph_color),
            FitModeButtonAction::FitHeight => fit_height_glyph_shapes(glyph_center, glyph_color),
        }
    }

// EDIT (c) — replace lines 128–164 (the two twin free functions with their docs)
// with this impl block (the blank line 127 before it and the const-free tail after
// it — axis_line_shape + fit_arrow_triangle_shape — stay untouched):

impl FitModeButtonAction {
    /// Per-action glyph geometry: one axis line plus the two opposing
    /// triangles of the painted-arrow family (`Shape::rotated_triangle`).
    fn glyph_shapes(
        &self,
        glyph_center: egui::Pos2,
        glyph_color: egui::Color32,
    ) -> Vec<egui::Shape> {
        let horizontal = matches!(self, Self::FitWidth);
        let axis_offset = if horizontal {
            egui::vec2(FIT_ARROW_SPACING, 0.0)
        } else {
            egui::vec2(0.0, FIT_ARROW_SPACING)
        };
        let (minus_facing, plus_facing) = if horizontal {
            (ArrowDirection::Left, ArrowDirection::Right)
        } else {
            (ArrowDirection::Up, ArrowDirection::Down)
        };
        vec![
            axis_line_shape(glyph_center, glyph_color, horizontal),
            fit_arrow_triangle_shape(glyph_center - axis_offset, minus_facing, glyph_color),
            fit_arrow_triangle_shape(glyph_center + axis_offset, plus_facing, glyph_color),
        ]
    }
}
```

Notes for the implementer: no derive needed — the method takes `&self` and `matches!(self, Self::FitWidth)` works through match ergonomics (precedent: `ArrowDirection::paint_rotation` unit-variant matching). `FitModeButtonAction` stays file-private with its two variants unchanged. Every other item in the file keeps its bytes. Diff: −25 lines net.

Verification claim (behavior-neutral): for `FitWidth` the merged path yields `horizontal = true`, offset `(7.5, 0)`, facings Left/Right → `vec![axis_line, left, right]`; for `FitHeight` → `horizontal = false`, offset `(0, 7.5)`, facings Up/Down → `vec![axis_line, up, down]` — identical shape lists, identical positions, identical paint order (axis first) as the twins today; glyph painting is test-exempt by plan §4 ("NO tests for button glyph geometry"). Post-S3: `wc -l src/app/fit_mode_button.rs` == 166 (after S1+S2+S3 with S1 applied: 191−25); `grep -c "fn fit_width_glyph_shapes\|fn fit_height_glyph_shapes\|fn fit_glyph_shapes" src/app/fit_mode_button.rs` == 0; longest body 21 lines (≤50 ✓), max depth 1 (≤2 ✓), all methods ≤2 non-self args ✓; tests stay 93/93; operator eyeball during Task-4 UX check confirms the W/H pills render unchanged.

## Pending review-side adjudication (NO patch from this side — coordination)

1. `fit_mode_button.rs::accepts_click` — `self.enabled && button_response.clicked()` (two-section boolean rule). Identical landed pattern in `floating_controls.rs`/`zoom_button.rs`; the code-reviewer is adjudicating it in parallel. Deliberately untouched here; S3 does not modify the method.
2. `fit_mode_button.rs` 3-arg free helpers `axis_line_shape(center, color, horizontal)` / `fit_arrow_triangle_shape(center, facing, color)` (≤2-non-self-args rule) — code-reviewer adjudicating in parallel. S3 keeps both byte-identical; IF the reviewer's fix converts them to param-struct calls, S3's three `vec![…]` call lines mechanically adopt the reviewer's final call form (apply reviewer-fix commits first, then rebase the S3 hunk onto the post-fix signatures — no structural decision changes).

## Considered and REJECTED (do not re-open at 4.5a without a new reason)

1. `scaling.rs`: share the duplicated `is_positive_size` double-guard across `fitted_size`/`fit_width_size`/`fit_height_size` — the plan binds "guards identical to `fitted_size`" (§3.5) as the reuse stance; a shared predicate either becomes a 3-arg helper (rule break) or hides the guard order; file sits 161/200 (ample room — no-churn rule). Kept.
2. `app/zoom.rs`: inline the 1-line `announce_fit_mode` into `set_fit_mode`, and/or shorten `crate::zoom::RESTORE_TO_FIT_FACTOR` to the file-local `zoom::RESTORE_TO_FIT_FACTOR` — zero line pressure (157/200); the named announce step keeps the seam doc's Option-A story readable, and the fully-qualified spelling is the deliberate guard the module doc lines 14–17 documents for the twin `zoom` names. Kept.
3. `app.rs`: collapse the blank lines between the `mod` declarations (would free ~16 lines) — Task-2 simplification REJECTED #7 decided this exact trim (pre-existing pattern, Preserve-Existing-Code); S2 alone recovers headroom to 198/200, and the remaining Wave-4 work (Task 4 verification, Task 5 docs) adds no `app.rs` lines. Kept unless the caller explicitly decides otherwise.
4. `central_surface.rs`: merge `show_navigation_central_surface` + `show_central_image_while_open` behind one flag — they differ by the §0-bound behavior (chrome shows the empty-folder message, fullscreen shows blank); the shared core is already single-sourced in `paint_open_image`. Same adjudication as Task-1 REJECTED #2 / Task-2 REJECTED #6. Kept.
5. `fit_mode_button.rs`: replace `FitModeButtonAction` with `crate::scaling::FitMode` directly (spec holds the mode) — forces an unreachable `FitVisible` arm (wildcard or `unreachable!()`) in `accessible_label`, weakening the compile-time exactness that the 2-variant enum buys; line delta ≈ 0. Kept.
6. `fit_mode_button.rs`: share `accepts_click`/the color ladder with `floating_controls`/`zoom_button` — cross-module sharing widens private-by-default surfaces (Task-1 REJECTED #3, Task-2 REJECTED #1). Kept per-file.
7. `scaling.rs`: rename the `Fit*` variants to clear the `#[allow(clippy::enum_variant_names)]` — variant names are plan-bound (§3.5 exact code) and used across dispatch, tests and the §0a crosswalk; renaming is semantic reinterpretation, not simplification. The allow attribute stays (clippy `-D warnings` requires it). Kept.
8. `keyboard_intake.rs`: fold the `fit_width_pressed`/`fit_height_pressed` reads directly into their `if` blocks — breaks the plan-bound "all reads collected before the dispatch chain" invariant (Task-2 REJECTED #2 same conclusion; the file's AI-Agents note forbids reshaping the block). Kept.
9. Tests: no change in either `fit_mode_tests.rs` (the 17-test addition is binding; 93 total pinned; bodies are plan-§3.10/§3.11 verbatim modulo fmt wraps).
10. Glyph builders → direct painter calls (dropping the `Vec<egui::Shape>`): rejected already at Task-2 REJECTED #3 against the identical plan-encoded design; S3 keeps the Vec-returning shape.

## Informational (no patch — caller/4.5a awareness)

1. `app/zoom.rs::announce_fit_mode` landed as the UNCONDITIONAL `self.message_text = fit_mode_activation_label(fit_mode);`. Plan §3.6.3's `if let Some(...)` wording would keep a stale "Fit width" label after a FitVisible activation and FAIL plan §3.10's pinned test `fit_visible_activation_is_silent_and_resets_the_mode_and_factor` — the implementation resolved the plan's internal wording/test conflict in favor of the binding test. Not a defect; 4.5a should not "restore" the conditional form.
2. `scaling/fit_mode_tests.rs` asserts `FitWidth` on (800×300)/(400×100) == `(800.0, 200.0)`; the plan §3.11 snippet printed `(800.0, 100.0)` — the PLAN snippet is arithmetically wrong (`image × available.x/image.x` = 400×(800/400) = 800×200); the landed value is correct. Do NOT "fix" the test toward the snippet.
3. Post-step line budget: `app.rs` 198/200, `fit_mode_button.rs` 166/200, `scaling.rs` 161, `app/zoom.rs` 157, `floating_controls.rs` 181 (unchanged), rest ≥ 40 headroom.

## Per-file verdicts

- `src/app/central_surface.rs` (44): plan §3.1 verbatim, single-source literal, no duplication → NOTHING.
- `src/app/fit_mode_button.rs` (193): S1 + S3 (→166); 3-arg/boolean items pending review-side.
- `src/app/fit_mode_tests.rs` (109) / `src/scaling/fit_mode_tests.rs` (91): binding census, plan-verbatim → NOTHING.
- `src/app.rs` (199): S2 (→198).
- `src/app_display.rs` (121): clean 3-field request + mode dispatch exactly per §3.4 → NOTHING.
- `src/scaling.rs` (161): math split exactly per §3.5; guard duplication REJECTED #1 → NOTHING.
- `src/app/floating_controls.rs` (181): Task-3 delta is 1 import + 1 call + doc phrase → NOTHING.
- `src/app/keyboard_intake.rs` (68): ordering is §0a-2 binding; fold REJECTED #8 → NOTHING.
- `src/app/zoom.rs` (157): seams minimal; inlines REJECTED #2 → NOTHING.
- `src/app/fullscreen.rs` (45): delegation exactly per §3.3 → NOTHING.

## Verification (owner: caller / implementer — NOT run by this step)

After S1+S2+S3, rerun the container harness (`alpine-vm_vm_status` first, then `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`): expect literal `ALL CHECKS PASSED` and 93/93 tests. S1/S2 are comment-only; S3 re-plumbs test-exempt paint code with byte-preserved geometry (any harness failure means a typo when applying, not a design issue). Expected line counts: `src/app.rs` 198, `src/app/fit_mode_button.rs` 166, all other files unchanged. Commit form per the implementation plan §6 style, one small fix commit, sequenced per the coordination section if review-fix commits land in the same file: `fix: fit-mode review fixes (trim duplicate palette docs, rewrap fit field doc, merge glyph twin builders)`.
