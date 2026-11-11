# Wave 5 — Pan then Navigate at Image Edges While Zoomed (Implementation Plan)

**Plan path:** `.kilo/plans/20261107-wave5-pan-navigate-implementation-plan.md`
**Step:** B-Group (Wave-5 Tasks 1+2+3 combined cycle), Critical Workflow step **4.1b** of the global plan `.kilo/plans/20261107-wave42-then-wave5-global-plan.md`.
**Branch for implementation:** `feat/wave-5-pan-then-navigate` (branch creation belongs to Step B.2; this step's implementer VERIFIES it is the current branch — see Step 0).
**Version:** 0.5.0 (bumped in Step B.3; this step must NOT edit `Cargo.toml`).
**Inputs:**
- Front-end spec (4.1a): `.kilo/plans/20261107-wave5-pan-navigate-frontend-spec.md` — pass-through input; where it left a decision open, THIS plan picks one and records it (§1).
- TODO: `.agent/todos/20261029/20261029-todo-7.md` — Tasks 1, 2, 3 + the deferred-decisions list.
- Brief: `.agent/project-info/brief.md` § `Pan and Navigate at Image Edges While Zoomed`.

---

## 0. Deferred-decision outcomes (SETTLED here — binding for the implementer)

| # | Deferred item | Outcome | Recorded as |
|---|---------------|---------|-------------|
| D1 | Pan step size | **PINNED:** `PAN_STEP_FRACTION = 0.125` — one arrow press pans `0.125 × available_axis_size` logical pixels (1/8 of the viewport axis). Consistent with the zoom ×1.25 precedent: both are exact binary fractions (`0.125` and `1.25` are exact in f32, so all test `assert_eq!` comparisons stay exact). ~8 presses cross an axis; the final step clamps to the remaining distance (never overshoots). | plan decision |
| D2 | Keyboard auto-repeat at edges | **PINNED (spec §1.4 contract):** panning ACCEPTS OS auto-repeat — the `key_pressed` read fires on repeats (Wave-4.1 F11 observation) so a held arrow pans smoothly. Navigation at an edge requires a **distinct non-repeat** `egui::Event::Key { pressed: true, repeat: false }` event (same spirit as the Wave-4.2 ESC-close repeat filter). A held key already at the edge is absorbed as `Pan(0.0)` — it must never navigate repeatedly. **OS-repeat note** (recorded like the Wave-4.1 F11 precedent): repeat delay/rate is OS-configured; agent-side code adds no timing logic; the OPERATOR-RUN (B-Task 4) validates the feel on real Windows. | plan decision |
| D3 | Mouse-wheel / mouse-drag pan | **OUT-OF-SCOPE for v1.** The brief specifies KEYS; the pan controller is keyboard-only. No mouse-wheel or drag panning is added; any future mouse pan belongs to a future wave (future-ideas owns it). The pill arrow buttons are navigation affordances and keep direct navigation (spec §4.3 pin). | out-of-scope note |
| D4 | Do fit modes suppress panning? | **MOOT.** `FitMode` no longer exists (removed in Wave 4.1). One-shot `W`/`H` sets only change `zoom_factor`; pan follows the SAME overflow rule with no suppression logic — nothing to suppress. No code and no test reference a "mode". | moot resolution |
| D5 | Which available size does the CONTROLLER use? (resolves the spec's §2.2-vs-§5 tension) | **PINNED:** the pan-or-navigate controller computes overflow/clamp/edge with `ctx.viewport_rect().size()` read at INTAKE (exactly the precedent of the one-shot fit keys at `src/app/keyboard_intake.rs:60,63`; fullscreen §5: equals the fullscreen viewport). The PAINT re-clamps with the central-panel rect (`available_rect_before_wrap()`), which is the authoritative no-gap guarantee (spec §3.3). The few-px viewport-vs-panel difference is accepted and recorded exactly like the Wave-4.1 one-shot-fit note ("lands within a few px of a full-window span"). The stored offset may therefore be a few px outside the panel's own clamp range; the paint clamp absorbs it silently. | plan decision |
| D6 | Reset semantics (spec §2.5 pins; restated as implementation facts) | (a) `empty_state()` initializes `pan_offset = egui::Vec2::ZERO`. (b) EVERY successful open zeroes it in `accept_open_outcome` (mirrors the `zoom_factor` reset line). (c) Restore-to-fit (`0` key AND restore button, both funnel through `restore_zoom_to_fit`) zeroes it via the new `reset_pan_offset()` seam. (d) One-shot `W`/`H` does NOT reset. (e) Zoom-in/out steps do NOT reset. (f) Entering empty-folder mode does NOT reset (the controller is inactive without an open image; the next successful open zeroes the stale value anyway). | spec pin |

---

## 1. Architecture overview

One pure math module + one app seam + two consumption sites, reusing every existing seam unchanged:

```
keyboard_intake.rs  (reads: was_pressed ×4 + non-repeat scan ×4)
      │  ArrowKeyPress { direction, non_repeat }  +  viewport_rect().size()
      ▼
src/app/pan.rs  (app seam: handle_arrow_key, reset_pan_offset,
                 direction→(PanAxis, PanSign) mapping, non-repeat arrow filter)
      │  pan::ArrowPanInputs
      ▼
src/pan.rs  (PURE: per-axis overflow, clamp, edge predicates, step, arrow_outcome)
      ▲                        │
      │ re-clamp at paint      │ ArrowPanOutcome::{Pan(delta), Navigate}
      │                        ▼
src/app_display.rs (+ central_surface.rs passes pan_offset)     navigate seam
(pan_offset composed into the image rect)                      (UNCHANGED:
                                                    src/app/folder_navigation.rs)
```

- State: ONE session-RAM field `pan_offset: egui::Vec2` on `ImagesScrollerApp` (spec §2.1). No new struct on the app; the param-object types live in the modules that own their signatures (≤2-params rule, §3).
- Stored-value discipline: the stored `pan_offset` may briefly hold a value outside the current valid range (after zoom/resize changes); EVERY consumer clamps through the pure math before use ("re-clamp on consumption"), and every pan action writes the clamped result back. This makes the "zoom change re-clamps" requirement automatic with zero extra code paths in `src/app/zoom.rs` beyond the restore reset.
- Per-axis independence: axes are stored in one `egui::Vec2` and clamped independently; arrow keys are axis-aligned so no diagonal pan is possible.
- Wave-3 rule: `navigate(direction)` is called verbatim for every Navigate outcome — the Wave-3 target priority (session last-viewed → first image), silent list edges, and empty-folder mode are NOT duplicated or restated.

---

## 2. Work breakdown (ordered, tiny steps)

All paths relative to the repository root. Implementer executes steps in order; each step ends with the pinned verification.

### Step 0 — Git state and the plan-docs commit (C1)

1. Run `git status`. Expected working tree: at most the two new/modified plan documents below. STOP and return a question to the caller if the tree contains ANY other modification (do not touch unrelated changes).
2. Verify the current branch is `feat/wave-5-pan-then-navigate` (`git branch --show-current`). If NOT, STOP and return a question to the caller — branch creation is Step B.2's action and must never happen here.
3. Stage and commit (if not already committed) exactly these two files with the pinned message:
   - `.kilo/plans/20261107-wave5-pan-navigate-frontend-spec.md` (produced in 4.1a)
   - `.kilo/plans/20261107-wave5-pan-navigate-implementation-plan.md` (this file)
   - Message: `docs: add wave5 pan-navigate spec and implementation plan`
   - If both are already committed on the branch, skip (record the skip in the completion summary).
4. Gitignore compliance: run `git status` after staging; assert nothing matching `.gitignore` (e.g. `target/`, `dist/`, `logs/`) is staged. Unstage if found and re-check.

### Step 1 — Pure math module `src/pan.rs` (+ `src/pan/pan_tests.rs`)

New file `src/pan.rs` (sibling of `src/zoom.rs` / `src/scaling.rs`; pure calculations only — no egui painting, no app state, no texture code). Register in `src/main.rs`: add `mod pan;` to the module block, alphabetically between `mod name_collation;` and `mod platform;`.

Module doc header (first lines of the file): states it is the Wave-5 pan math — per-axis overflow, clamped offsets, at-edge predicates, the pan step, and the arrow pan-or-navigate decision; authoritative for the MATH only; the reset policy lives in the app (`src/app.rs` / `src/app/pan.rs`); the paint consumes it in `src/app_display.rs`.

Constants (both `pub(crate)`, both exact in f32):

```rust
pub(crate) const PAN_STEP_FRACTION: f32 = 0.125;
pub(crate) const PAN_EDGE_TOLERANCE: f32 = 0.5;
```

Per-axis scalar primitives (all `pub(crate)` free functions, ≤2 visible params each):

```rust
/// overflow on one axis: max(0, draw - available)
pub(crate) fn axis_overflow(draw_axis: f32, available_axis: f32) -> f32

/// half-width of the valid pan range on one axis: [−limit, +limit]
pub(crate) fn axis_pan_limit(overflow: f32) -> f32          // = overflow * 0.5

/// clamp one axis of a raw offset into [−limit, +limit] (collapses to 0 at no overflow)
pub(crate) fn clamped_axis_offset(raw_offset: f32, overflow: f32) -> f32

/// at the negative (left/top) edge: clamped <= -limit + tolerance
pub(crate) fn axis_is_at_negative_edge(clamped_offset: f32, overflow: f32) -> bool

/// at the positive (right/bottom) edge: clamped >= +limit - tolerance
pub(crate) fn axis_is_at_positive_edge(clamped_offset: f32, overflow: f32) -> bool

/// one pan step on one axis: PAN_STEP_FRACTION * available_axis
pub(crate) fn axis_pan_step(available_axis: f32) -> f32
```

Vec2-level helpers (2 params each) used by the paint and the seam:

```rust
/// per-axis overflow pair
pub(crate) fn axis_overflow_pair(draw_size: egui::Vec2, available_size: egui::Vec2) -> egui::Vec2

/// per-axis clamped offset pair
pub(crate) fn clamped_pan_offset(raw_offset: egui::Vec2, overflow: egui::Vec2) -> egui::Vec2
```

Axis selection types + the decision entry point:

```rust
pub(crate) enum PanAxis { Horizontal, Vertical }
impl PanAxis {
    pub(crate) fn component(self, size: egui::Vec2) -> f32;              // x or y
    pub(crate) fn with_component(self, size: egui::Vec2, value: f32) -> egui::Vec2;
}

pub(crate) enum PanSign { Negative, Positive }   // Negative = left/top; Positive = right/bottom

/// Everything one arrow press needs (≤2-params rule: ONE param object).
#[derive(Clone, Copy)]
pub(crate) struct ArrowPanInputs {
    pub(crate) raw_offset: egui::Vec2,
    pub(crate) draw_size: egui::Vec2,
    pub(crate) available_size: egui::Vec2,
    pub(crate) axis: PanAxis,
    pub(crate) sign: PanSign,
    pub(crate) press_is_non_repeat: bool,
}

pub(crate) enum ArrowPanOutcome {
    /// Signed delta for the axis offset (the caller re-clamps; 0.0 = a repeat
    /// press absorbed at an edge).
    Pan(f32),
    Navigate,
}

pub(crate) fn arrow_outcome(inputs: ArrowPanInputs) -> ArrowPanOutcome
```

`arrow_outcome` decision table (exact behavior; flat early-return structure, ≤2 nesting depth, single-section boolean conditions only):

1. `overflow = axis_overflow_pair(inputs.draw_size, inputs.available_size)`; take the `inputs.axis` component.
2. If that overflow `<= 0.0` → `ArrowPanOutcome::Navigate` (Task 3: the axis fits entirely — direct navigation for EVERY press, repeat or not).
3. Else `clamped = clamped_axis_offset(<inputs.raw_offset axis component>, overflow)`.
4. At the REQUESTED edge? `PanSign::Negative` → `axis_is_at_negative_edge`, `PanSign::Positive` → `axis_is_at_positive_edge`.
5. If at the requested edge:
   - `press_is_non_repeat == true` → `ArrowPanOutcome::Navigate` (the second distinct press rule, spec §1.3).
   - otherwise → `ArrowPanOutcome::Pan(0.0)` (held key absorbed; never navigates on repeat).
6. Else → `ArrowPanOutcome::Pan(signed step)`: `+axis_pan_step(available)` for `Positive`, `−axis_pan_step(available)` for `Negative`.

Pinned sign/axis convention (matches spec §2.2): positive offset = viewport panned right/down; the painted top-left is `centered_min − pan_offset`; a Right/Down arrow increases the offset (reveals the right/bottom edge); Left/Up decreases it.

New test file `src/pan/pan_tests.rs`, declared at the bottom of `src/pan.rs` as `#[cfg(test)] mod pan_tests;` (exact precedent: `src/scaling.rs` + `src/scaling/fit_factor_tests.rs`). All test vectors use binary-exact f32 values so `assert_eq!` is exact. EXACTLY these 11 tests, in this order, with these names:

1. `wide_image_overflows_only_horizontally` — draw 1600×300 vs available 800×300: overflow x = 800, y = 0 (`axis_overflow_pair`).
2. `tall_image_overflows_only_vertically` — draw 300×1600 vs available 300×800: overflow x = 0, y = 800.
3. `fitting_image_has_zero_overflow_on_both_axes` — draw 400×200 vs available 800×300: both 0.
4. `clamped_axis_offset_stays_inside_the_symmetric_bounds` — raw +1000 with overflow 800 → +400; raw −1000 → −400; raw 0 → 0.
5. `clamped_axis_offset_collapses_to_zero_without_overflow` — any raw with overflow 0 → 0.
6. `arrow_outcome_navigates_directly_when_the_axis_fits` — zero overflow → `Navigate` for BOTH `press_is_non_repeat` values (Task 3).
7. `arrow_outcome_pans_by_the_pinned_step_when_not_at_the_edge` — raw 0, overflow (800,·): Right-style Positive → `Pan(+100.0)` (= 0.125 × 800); Left-style Negative → `Pan(−100.0)`.
8. `arrow_outcome_absorbs_a_repeat_press_at_the_edge` — raw at the positive limit, Positive, `press_is_non_repeat: false` → `Pan(0.0)`; same for the negative edge (assert NOT `Navigate`).
9. `arrow_outcome_navigates_on_a_distinct_press_at_the_edge` — same states with `press_is_non_repeat: true` → `Navigate`.
10. `an_overshooting_step_is_absorbed_by_the_clamp` — raw = +limit − 40 with step 100: `clamped_axis_offset(raw + 100, overflow)` equals exactly `+limit` (the landing press never overshoots).
11. `pan_constants_are_pinned` — `PAN_STEP_FRACTION == 0.125`, `PAN_EDGE_TOLERANCE == 0.5`.

Verification: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked` passes (per-check fallback per `docs/build.md`).

### Step 2 — App state in `src/app.rs` (+ the pinned line-budget move)

`src/app.rs` is at the 200-line cap (197 lines). The seven lines this step adds would push it to 204, so the plan MOVES `window_options()` to `src/main.rs` (the entry point already owns `run_native` and the only call site; `grep` confirms no other reference, and no test references it). This is a pinned, in-plan structural action — do it EXACTLY as specified, nothing else.

2a. **Move `window_options` to `src/main.rs`:**
- Delete from `src/app.rs` the whole `pub fn window_options() -> eframe::NativeOptions { ... }` block (lines 147–155) plus the blank line between it and the `impl eframe::App` block (the file keeps exactly one blank line there).
- Paste it into `src/main.rs` ABOVE `fn main()` as a PRIVATE `fn window_options() -> eframe::NativeOptions` with the body byte-identical (same `ViewportBuilder` chain: `with_title(APP_WINDOW_TITLE)`, `with_resizable(true)`, `with_maximized(true)`, `..eframe::NativeOptions::default()`).
- Change the call in `main()` from `app::window_options()` to `window_options()`.
- Add `use eframe::egui;` to `src/main.rs` (needed by `ViewportBuilder`).
- `APP_WINDOW_TITLE` stays in `src/app.rs` (also used by `run_native` and the title test).

2b. **Add the pan state (7 lines net after 2a; app.rs ends ≈194):**
- Struct field after `esc_close_popup_expires_at` (one doc line + field):

```rust
    /// Pan offset of the visible viewport from the centered position (Wave 5);
    /// zeroed on every successful open and on restore-to-fit; never persisted.
    pan_offset: egui::Vec2,
```

- In `empty_state()`: `pan_offset: egui::Vec2::ZERO,` (after `esc_close_popup_expires_at: None,`).
- In `accept_open_outcome`, Ok branch, directly after the existing `self.zoom_factor = crate::zoom::RESTORE_TO_FIT_FACTOR;` line: `self.pan_offset = egui::Vec2::ZERO;` (direct assignment, mirroring the adjacent zoom-reset line style).
- Module block: `mod pan;` (alphabetical, after `mod keyboard_intake;` — the app child modules are `arrow_direction`…`zoom_button`, so `pan` goes between `keyboard_intake` and `pill_style`).
- Test block: `#[cfg(test)]` + `mod pan_tests;` (alphabetical, after `mod last_viewed_tests;`).
- Do NOT reset pan anywhere else in `app.rs` (D6: empty-folder mode deliberately does not reset).

Verification: `cargo check --locked` passes.

### Step 3 — App seam `src/app/pan.rs` (+ `src/app/pan_tests.rs`)

New file `src/app/pan.rs` (child module of the app shell; the ≤2-params rule is respected by the request objects; max method length ≤50 lines; single-section conditions). Module doc header pins: the keyboard-only pan-or-navigate controller; `crate::pan` (pure math) referenced FULLY-qualified because `mod pan;` in `src/app.rs` shadows it inside app children (same note style as `src/app/zoom.rs` doc lines 31–34); mouse pan out of scope (D3); navigation reuses `navigate` unchanged.

```rust
use eframe::egui;

use super::ImagesScrollerApp;
use super::NavigationDirection;
use crate::pan;

/// One arrow-key press handed to the controller: the navigation direction the
/// key maps to and whether the event is a DISTINCT (non-repeat) press.
pub(super) struct ArrowKeyPress {
    pub(super) direction: NavigationDirection,
    pub(super) non_repeat: bool,
}

/// True exactly for a non-repeat pressed `arrow_key` event in this frame's
/// events (the at-edge navigation gate; pan steps do NOT require it).
pub(super) fn non_repeat_arrow_press_occurred(
    events: &[egui::Event],
    arrow_key: egui::Key,
) -> bool
```

(The filter body mirrors `esc_close_confirm::non_repeat_escape_press_occurred` — a `matches!` over `egui::Event::Key { key, pressed: true, repeat: false, .. }` with the key compared to `arrow_key`.)

Direction mapping (spec §4.2, exact):

```rust
fn arrow_pan_axis(direction: NavigationDirection) -> pan::PanAxis
// Previous | Next => Horizontal; PreviousSiblingFolder | NextSiblingFolder => Vertical

fn arrow_pan_sign(direction: NavigationDirection) -> pan::PanSign
// Previous | PreviousSiblingFolder => Negative; Next | NextSiblingFolder => Positive
```

App methods:

```rust
impl ImagesScrollerApp {
    /// Zeroes the pan offset (restore-to-fit path: the `0` key AND the
    /// restore button both funnel through `restore_zoom_to_fit`).
    pub(super) fn reset_pan_offset(&mut self) {
        self.pan_offset = egui::Vec2::ZERO;
    }

    /// The pan-or-navigate controller: no image open → direct navigation
    /// (also the degenerate-payload guard); an image open → the pure
    /// `pan::arrow_outcome` decision per the press's axis.
    pub(super) fn handle_arrow_key(&mut self, available_size: egui::Vec2, press: ArrowKeyPress) {
        if !self.image_is_open() {
            self.navigate(press.direction);
            return;
        }
        let Some(pan_inputs) = self.arrow_pan_inputs(available_size, press) else {
            self.navigate(press.direction);
            return;
        };
        self.apply_arrow_pan_outcome(pan_inputs, press);
    }
}
```

Two private helpers (each ≤2 visible params):

```rust
/// Builds the pure request: draw size from the current payload and zoom
/// factor over the intake available size; axis/sign from the direction.
/// None when the payload or its fit is degenerate (unreachable in practice).
fn arrow_pan_inputs(&self, available_size: egui::Vec2, press: ArrowKeyPress)
    -> Option<pan::ArrowPanInputs>
{
    let decoded_image = self.decoded_image.as_ref()?;
    let fit_size = crate::scaling::fitted_size(available_size, decoded_image.size())?;
    Some(pan::ArrowPanInputs {
        raw_offset: self.pan_offset,
        draw_size: crate::zoom::zoomed_draw_size(fit_size, self.zoom_factor),
        available_size,
        axis: arrow_pan_axis(press.direction),
        sign: arrow_pan_sign(press.direction),
        press_is_non_repeat: press.non_repeat,
    })
}

/// Applies the pure outcome: Pan(delta) writes the clamped result back;
/// Navigate calls the shared seam verbatim (Wave-3 rule untouched).
fn apply_arrow_pan_outcome(&mut self, pan_inputs: pan::ArrowPanInputs, press: ArrowKeyPress) {
    match pan::arrow_outcome(pan_inputs) {
        pan::ArrowPanOutcome::Pan(signed_delta) => self.apply_pan_delta(pan_inputs, signed_delta),
        pan::ArrowPanOutcome::Navigate => self.navigate(press.direction),
    }
}

/// Adds the signed delta on the outcome's axis and stores the CLAMPED value
/// (the last step lands exactly on the edge; a stale out-of-range stored
/// value is normalized here — the re-clamp-on-use rule).
fn apply_pan_delta(&mut self, pan_inputs: pan::ArrowPanInputs, signed_delta: f32) {
    let axis = pan_inputs.axis;
    let overflow = pan::axis_overflow(
        axis.component(pan_inputs.draw_size),
        axis.component(pan_inputs.available_size),
    );
    let raw = axis.component(self.pan_offset);
    self.pan_offset = axis.with_component(
        self.pan_offset,
        pan::clamped_axis_offset(raw + signed_delta, overflow),
    );
}
```

`ArrowPanInputs` is `Copy`, so passing it by value to multiple helpers is fine.

New test file `src/app/pan_tests.rs` (declared in `src/app.rs` per Step 2b). Reuse the existing fixtures: `super::tests::{TempImageFolder, sample_decoded_image}` and, for the vertical navigation test, `super::folder_navigation_tests::{TempParentScope, app_opened_in}` (both already `pub(super)`; the continuation tests import them exactly this way). Tests set `app.pan_offset` directly where a specific pan state is needed (same private-field access pattern as `zoom_tests.rs` reading `app.zoom_factor`). Every `handle_arrow_key` call passes an injected `available_size: egui::Vec2` — no egui `Context` input is required. EXACTLY these 12 tests, in this order:

1. `fresh_app_starts_with_zero_pan_offset` — `empty_state()`; `app.pan_offset == egui::Vec2::ZERO`.
2. `every_successful_open_resets_the_pan_offset` — TempImageFolder with a.png/b.png; open b; set a non-zero `pan_offset` + zoom in twice; `accept_open_outcome(a.png, Ok(sample))`; assert offset zero AND `zoom_factor == 1.0`.
3. `restore_zoom_to_fit_resets_the_pan_offset` — set non-zero offset; `app.restore_zoom_to_fit()`; assert zero.
4. `one_shot_fit_sets_do_not_reset_the_pan_offset` — set non-zero offset; `app.set_zoom_to_fit_width(egui::vec2(800.0, 300.0))`; assert offset unchanged (D6d).
5. `zoom_steps_do_not_reset_the_pan_offset` — set non-zero offset; `app.zoom_in_once()`; assert offset unchanged (D6e).
6. `arrow_pans_the_overflowing_axis_while_not_at_the_edge` — two-image folder, open a.png, `available_size = egui::vec2(800.0, 300.0)`; press Right (`ArrowKeyPress { direction: Next, non_repeat: true }`): the 2×2 payload at zoom 8 draws 2400×2400 → overflow x 1600 → offset x == +200.0 (0.125 × 1600); press Left: offset x returns to 0.0 (exact f32).
7. `the_edge_landing_press_clamps_without_navigating` — same setup; set `pan_offset.x = pan::axis_pan_limit(1600.0) − 40.0`; press Right: assert offset x == `axis_pan_limit(1600.0)` exactly AND `opened_image_path` STILL a.png (landing press only pans).
8. `a_distinct_press_at_the_right_edge_navigates_to_the_next_image` — offset.x == +limit; press Right with `non_repeat: true`: assert `opened_image_path` == b.png (navigation fired through the normal seam) and the NEW image's pan offset is zero (the open reset, D6b).
9. `a_repeat_press_at_an_edge_is_absorbed` — two-image folder open a.png; offset.x == −limit; press Left with `non_repeat: false`: assert offset x unchanged AND `opened_image_path` still a.png (held key never navigates; D2).
10. `a_distinct_press_at_the_bottom_edge_navigates_to_the_next_sibling_folder` — `TempParentScope` with `folder1`/`folder2`, one image each (fixture pattern per `folder_navigation_continuation_tests.rs`); open folder1's image; set `pan_offset.y = +axis_pan_limit(<vertical overflow>)` with `available_size = egui::vec2(300.0, 800.0)` (2×2 payload at zoom 8 → overflow y = 2100 → limit 1050); press Down (`NextSiblingFolder`, `non_repeat: true`): assert the app now shows folder2's target image (Wave-3 rule verdict, unchanged).
11. `a_fitting_image_arrow_navigates_directly` — two-image folder open a.png at zoom 1.0 (`zoom_factor` untouched): draw 300×300 fits 800×300 vertically and NOT horizontally… use `available_size = egui::vec2(800.0, 300.0)` with `zoom_factor = 0.125` (draw = 37.5×37.5 — fits both axes); press Right (`non_repeat: false` — repeat value irrelevant on a fitting axis): assert opened == b.png AND offset still zero.
12. `arrows_without_an_open_image_route_directly_to_navigation` — fresh `empty_state()`; press Left (`Previous`, `non_repeat: true`): no-op (no listing), `pan_offset` stays zero, `message_text` stays None.

If the file would exceed 200 lines, move tests 10 (the vertical one) into a new `src/app/pan_navigation_tests.rs` (declared in `src/app.rs` after `mod pan_tests;`) — the same tests+neighbor split precedent as `folder_navigation_tests`/`folder_navigation_continuation_tests`. Record whichever shape was used in the completion summary.

Verification: `cargo check --locked` and `cargo test --locked` (per-check fallbacks) pass.

### Step 4 — Keyboard intake routing `src/app/keyboard_intake.rs`

Edits inside `read_and_dispatch_keystrokes` ONLY (file stays ≤200 lines; projected ≈125):

1. Add `use super::pan;` to the imports.
2. KEEP the four `was_pressed` arrow reads (lines 37–40) unchanged and in order (Left, Right, Up, Down) — they accept repeats (pan path).
3. ADD four non-repeat reads immediately after them, one line each, in the same order:

```rust
let left_non_repeat = screen_context
    .input(|input| pan::non_repeat_arrow_press_occurred(&input.events, egui::Key::ArrowLeft));
let right_non_repeat = screen_context
    .input(|input| pan::non_repeat_arrow_press_occurred(&input.events, egui::Key::ArrowRight));
let up_non_repeat = screen_context
    .input(|input| pan::non_repeat_arrow_press_occurred(&input.events, egui::Key::ArrowUp));
let down_non_repeat = screen_context
    .input(|input| pan::non_repeat_arrow_press_occurred(&input.events, egui::Key::ArrowDown));
```

(If rustfmt prefers single lines, keep them single-line — `cargo fmt` decides; both shapes are acceptable, the content is pinned.)

4. REPLACE the four dispatch blocks (lines 65–76) — reads-before-dispatch is preserved because all reads stay above the dispatch chain; the dispatch ORDER stays Left → Right → Up → Down:

```rust
if left_pressed {
    self.handle_arrow_key(
        screen_context.viewport_rect().size(),
        pan::ArrowKeyPress {
            direction: NavigationDirection::Previous,
            non_repeat: left_non_repeat,
        },
    );
}
```

- Right → `NavigationDirection::Next` + `right_non_repeat`
- Up → `NavigationDirection::PreviousSiblingFolder` + `up_non_repeat`
- Down → `NavigationDirection::NextSiblingFolder` + `down_non_repeat`

5. NOTHING else in the file changes: the F11/ESC/zoom/fit chords, the ESC-close branch, and the module doc note ("keep all reads collected before the dispatch chain") stay byte-identical. Update ONLY the module doc's first paragraph to add "the four pan-or-navigate arrows" to the chord list (doc touch in this step's own file).

Verification: `cargo check --locked` passes.

### Step 5 — Display composition `src/app_display.rs` (+ `src/app/central_surface.rs`)

5a. `src/app_display.rs`:
- `ImagePainting` gains the third field (spec §3.1) and its doc comment updates from "2-argument" to "3-argument":

```rust
pub(crate) struct ImagePainting<'a> {
    pub(crate) decoded_image: &'a DecodedImage,
    pub(crate) zoom_factor: f32,
    pub(crate) pan_offset: egui::Vec2,
}
```

- Replace the `paint` body's centered-layout block (`panel_ui.with_layout(egui::Layout::centered_and_justified(...)...)`, lines 55–60) with the explicit positioned rect (spec §3.2, exact):

```rust
let panel_rect = panel_ui.available_rect_before_wrap();
let overflow = pan::axis_overflow_pair(display_size, panel_rect.size());
let clamped_offset = pan::clamped_pan_offset(painting.pan_offset, overflow);
let centered_min = panel_rect.center() - display_size * 0.5;
let image_rect = egui::Rect::from_min_size(centered_min - clamped_offset, display_size);
panel_ui.put(image_rect, egui::Image::new(image_texture).fit_to_exact_size(display_size));
```

  (`display_size` is the existing `zoom::zoomed_draw_size(fit_size, painting.zoom_factor)` value; `fitted_size` keeps using `panel_rect.size()` instead of `panel_ui.available_size()` — same value at first allocation, one rect read for size AND center.)
- Add `use crate::pan;` to the imports.
- Update the stale doc text: the `paint` doc and the "No pan or scroll this wave -- Wave 5 composes its offsets onto this same fit-scaled size" sentence (lines 40–43) become the pinned paint contract: the offset is clamped per axis so an overflowing axis never shows a gap or overscan; a fitting axis stays centered; the panel clip rect hides anything beyond; NO texture re-upload — the offset only repositions the rect; one upload per successful decode remains (spec §3.3/§3.4).
- Texture lifecycle code (`ensure_current_image_texture`, `accept_decoded_image`, `initial`) stays byte-identical.

5b. `src/app/central_surface.rs`:
- In `paint_open_image`, add one field to the `ImagePainting` literal: `pan_offset: self.pan_offset,` (after `zoom_factor`).
- Update the module doc's parenthetical ("(which decoded image at WHICH session zoom factor paints)") to include the pan offset.

Verification: `cargo check --locked` passes.

### Step 6 — Restore seam `src/app/zoom.rs`

- In `restore_zoom_to_fit` (line 72–74), add ONE line: `self.reset_pan_offset();` after the factor assignment. This covers BOTH the `0` key and the restore button (both call `restore_zoom_to_fit`; the button path is `zoom_button.rs:96`).
- One-shot `set_zoom_to_fit_width`/`set_zoom_to_fit_height`: NO change (D6d — no reset; consumption-time re-clamp covers them).
- `zoom_in_once`/`zoom_out_once`: NO change.
- Update the stale pure-math doc line in `src/zoom.rs` (line 63: "Wave-5 pan composes its offsets on this size; no scroll/pan this wave") to the present tense: "Wave-5 pan composes its clamped offsets on this size".
- `src/zoom.rs`'s other code: untouched.

### Step 7 — Verify-no-change pass over the pinned-untouched modules

Read-only verification (report the results in the completion summary; NO edits):
- `src/app/floating_controls.rs`: arrow enablement is still `navigation_button_enabled` = `neighbor_image_path(direction).is_some()` (spec §4.3 pin: buttons never disable because of zoom/pan; clicks call `app.navigate(direction)` directly). Expected finding: zero changes required.
- `src/app/esc_close_confirm.rs`: the popup Area is `interactable(false)` — it cannot intercept arrow keys; nothing required (global plan B-Group: "no interplay").
- `src/app/fullscreen.rs`: arrow intake is ungated (`read_and_dispatch_keystrokes` runs in `ui()` in every mode, `src/app.rs:141-144`); the controller therefore works identically fullscreen ON, with `viewport_rect().size()` equal to the fullscreen viewport (spec §5). Nothing required.
- `src/app/folder_navigation.rs`: `navigate`/`neighbor_image_path` are reused as-is; the Wave-3 vertical rule is referenced, never restated. Nothing required.

### Step 8 — Full dev-checks harness (mandatory before commit C2)

1. `alpine-vm_vm_status` FIRST — only proceed when the VM is running and SSH-reachable; if the VM or `/img-scroller` is unavailable, STOP and return a user question (bound by the todo's execution notes).
2. Run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
3. Expected final line: `ALL CHECKS PASSED` (exit code 0 — the authoritative evidence). All four checks pass: fmt-check, check, test, clippy.
4. Census expectation: **150** unit tests total (127 current + 11 `pan_tests` + 12 app `pan_tests`). If the count differs, the ACTUAL count from the harness output is recorded verbatim in the completion summary and becomes the B-Task 4 census pin (the harness is authoritative; a reviewer-cycle test adjustment is recorded the same way).
5. On any failure: fix within this plan's scope (the failing check's own output is the guide), re-run the FULL harness, and record the fix in the summary. Repeated harness failure → STOP, return to caller.

### Step 9 — Feature commit (C2)

1. `git status`: stage ONLY the files this plan touched:
   - `src/pan.rs` (new), `src/pan/pan_tests.rs` (new)
   - `src/app/pan.rs` (new), `src/app/pan_tests.rs` (new; plus the optional `src/app/pan_navigation_tests.rs` only if Step 3's split fired)
   - `src/main.rs`, `src/app.rs`, `src/app/keyboard_intake.rs`, `src/app_display.rs`, `src/app/central_surface.rs`, `src/app/zoom.rs`, `src/zoom.rs`
2. Assert NOTHING matching `.gitignore` is staged (Gitignore Compliance Rule) — no `target/`, `dist/`, `logs/`.
3. Commit message: `feat: pan then navigate at image edges while zoomed`
4. NOT in this step (later workflow steps): review-fix commits (4.3), module-docs commit (4.4), verification plan/rebuild/audit (B-Task 4), documentation commits (B-Task 5), branch merge/push (Step B.5).

---

## 3. Rules conformance encoded in the design

- **≤200 lines/file:** `src/pan.rs` ≈ 120 (+ separate test file), `src/pan/pan_tests.rs` ≈ 160, `src/app/pan.rs` ≈ 120, `src/app/pan_tests.rs` ≈ 175 (with the pinned split escape-hatch), `keyboard_intake.rs` ≈ 125, `app_display.rs` ≈ 125, `central_surface.rs` ≈ 44, `app/zoom.rs` ≈ 150, `src/zoom.rs` ≈ 129, `app.rs` ≈ 194, `main.rs` ≈ 47.
- **≤50 lines/method:** every method above is sketched under 30 lines.
- **≤2 params/method:** request objects (`ArrowKeyPress`, `pan::ArrowPanInputs`) carry the multi-value signatures; visible non-self params stay ≤2 everywhere.
- **≤2 nesting depth + single-section booleans:** the decision function uses flat early returns; edge predicates are single comparisons in dedicated functions.
- **Private by default:** everything new is `pub(crate)` (pure math, cross-module) or `pub(super)` (app seam) — nothing `pub`.
- **Self-documenting, minimal comments:** doc headers state contracts; no commented-out code anywhere.
- **No magic numbers:** `PAN_STEP_FRACTION`, `PAN_EDGE_TOLERANCE` are named constants; `0.5` in `axis_pan_limit` is the definition of the symmetric range (documented once).
- **Preserve existing code:** no navigation/listing/collation/ESC/fullscreen/button logic changes; the only existing-code moves are the pinned `window_options` relocation and the four arrow dispatch blocks.

## 4. Explicit no-touch list

- `message_text` — pan and edge behavior are SILENT; no new message constant may appear anywhere (zoom-bound-message removal precedent stays in force).
- `src/folder_listing.rs`, `src/sibling_folder_listing.rs`, `src/name_collation.rs`, `src/last_viewed_images.rs`, `src/image_loading.rs`, `src/scaling.rs` (code), `src/platform/**` — untouched.
- `src/app/folder_navigation.rs`, `src/app/esc_close_confirm.rs`, `src/app/fullscreen.rs`, `src/app/floating_controls.rs`, `src/app/fit_button.rs`, `src/app/fullscreen_button.rs`, `src/app/button_hint.rs`, `src/app/pill_style.rs`, `src/app/zoom_alias.rs`, `src/app/arrow_direction.rs`, `src/app/empty_folder_view.rs`, `src/app/empty_state_view.rs`, `src/app/zoom_button.rs` — untouched (Step 7 verifies floating_controls/esc/fullscreen/folder_navigation explicitly).
- Window shell options (`with_title`/`with_resizable`/`with_maximized`) — moved verbatim, never altered.
- No mouse-wheel/drag pan, no persistence, no disk writes, no new dependencies (no `Cargo.toml`/`Cargo.lock` change).
- `CHANGELOG.md` — not touched in this wave (todo rule).
- Existing `.kilo/plans/*` files — never edited (new plan files only).

## 5. Test-plan summary and census pin (for B-Task 4)

- Pure math: 11 tests (`src/pan/pan_tests.rs`) — overflow per axis (wide/tall/fit), symmetric clamp, collapse-to-zero, step sign+size, overshoot absorption, repeat-at-edge absorption, distinct-at-edge navigation, pinned constants.
- App seam: 12 tests (`src/app/pan_tests.rs`) — per-axis routing table (overflow+not-at-edge → pan; overflow+at-edge → navigate on distinct press only; no-overflow → direct navigate regardless of repeat), reset semantics (open/restore reset; W/H and zoom steps do not), landing-press clamp, repeat absorption, vertical Wave-3 trigger, no-image direct routing.
- Census: 127 → **150 projected** (+23); the harness's actual count is authoritative and is recorded verbatim in the completion summary and in B-Task 4's verification plan.
- B-Task 4 (NOT this step) will additionally pin: the exe rebuild via `scripts/build-windows.sh` (0.5.0), the 22-DLL import audit constant, and the OPERATOR-RUN checklist verbatim from the todo's Task 4 sub-item (zoom wide → keys pan, navigation only at edge; zoom tall → same vertically; small image → direct navigation).

## 6. 4.5a conformance anchors (what the front-end verifier will check against the 4.1a spec)

Single `pan_offset: egui::Vec2` field; `src/pan.rs` pure math with tests; `src/app/pan.rs` seam (`handle_arrow_key`, `reset_pan_offset`, axis/sign mapping, `ArrowKeyPress`); arrows routed through the seam in `keyboard_intake.rs` (no direct `navigate` for arrows); non-repeat gate at edges with repeat-accepting pan; explicit `ui.put(image_rect, ...)` paint with the clamped offset (centered layout gone); no-gap/no-overscan clamp guarantee; arrow buttons unchanged (enablement + direct navigation); fullscreen-ON parity; Wave-3 vertical trigger untouched; reset semantics per D6; texture discipline (one upload per decode); mouse pan absent (D3); no fit-mode suppression (D4).

## 7. Original-task comparison (self-check)

- TODO Task 1 (horizontal pan-then-navigate): Steps 1/3/4/5 — state machine row "overflow + not-at-edge → pan, at-edge → navigate on the next distinct press" covers it. ✔
- TODO Task 2 (vertical pan-then-navigate + Wave-3 rule referenced): same controller on the vertical axis; test 10 reuses the Wave-3 fixtures; `navigate` untouched. ✔
- TODO Task 3 (fits entirely → direct navigation): decision-table row 2 + tests 6/11. ✔
- Deferred decisions: D1–D6 all settled explicitly (§0), including the mouse OUT-OF-SCOPE note and the fit-suppression MOOT resolution. ✔
- The plan is scoped to ONE 4.1b step: no code was written by the architector; the implementer executes §2. ✔
