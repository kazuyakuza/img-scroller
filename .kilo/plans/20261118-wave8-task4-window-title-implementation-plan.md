# Wave 8 Task 4 — Window Title Shows the Visible Image's Path: Implementation Plan (4.1b)

## 1. Task, binding source, scope

Binding TODO text (`.agent/todos/20261117/20261117-todo-1.md`, Task 4, VERBATIM — the user's algorithm):

> - The window title shows the path of the VISIBLE image and updates on every navigation (the path of the currently displayed image).
> - If the full path fits completely in the title, show it.
> - Otherwise show the image NAME + extension completely; if the name itself is too long, show only its last characters.
> - With remaining space, add — FIRST the folder containing the opened file, SECOND the drive (the initial path part); between the drive and the visible folder chain show `...`; with still-available space keep adding parent folders (walking up).
> - While the window is NOT maximized: show ONLY the file name (+ extension), last-chars-truncated if too long.
> - While fullscreen is ON: on every visible-image change (navigation), show a popup at the TOP CENTER containing the file name + extension for 3 seconds.
> - Exact measurement/truncation/priority details (pixel measurement method, thresholds, ellipsis rendering, popup visuals) are decided in this wave's architector step.

Front-end input: `.kilo/plans/20261118-wave8-task4-window-title-frontend-spec.md` (4.1a — its pinned contracts bind; its open items are settled in §2 below).

Scope of this step's implementation (4.2): the two new modules, the app-shell wiring, the fullscreen-popup render/disarm hooks, the pure-module tests, the structure-map entry, and the dev-checks verification run. NOT in scope: installer work (Tasks 1/5/7), README/tech/architecture/context/CHANGELOG prose (Task 6), any `git push`, any version/Cargo change.

Environment facts (verified 2026-11-18): branch `feat/wave-8-wizard-installer` exists, HEAD `b1d8e13` (Tasks 1–3 landed), version 1.1.0 already bumped (DO NOT touch), working tree clean except the UNTRACKED 4.1a spec file `.kilo/plans/20261118-wave8-task4-window-title-frontend-spec.md` (it is committed together with this plan — see §5). Census at HEAD: 195 unit tests / 28 test-bearing files.

## 2. Settled architector decisions (ALL open items closed — no open choices remain)

### D1 — Measurement method: Option A, pixel-accurate via egui fonts

- The composition measures candidate strings with the REAL egui font layout: `screen_context.fonts(|fonts| fonts.layout_no_wrap(text.to_owned(), egui::FontId::default(), egui::Color32::WHITE))` and reads the width as the returned `Arc<Galley>`'s `.size().x`.
- Rationale: the task demands pixel fitting; a char-count heuristic (Option B) diverges badly on the operator's real mixed-ASCII/Japanese names (the Wave-5.2 evidence); egui 0.37 provides the API directly (spec §3.3 evidence chain: `Context::fonts` → `FontsView`, `FontsView::layout_no_wrap` → `Arc<Galley>`, `Galley::size() -> Vec2`).
- API-compile fallback (the ONLY sanctioned substitution, decided in advance): if 0.37.0's compiler rejects `Context::fonts(...)`/`FontsView::layout_no_wrap(...)`, substitute `Context::fonts_mut(...)` (same closure shape) and/or the underlying `fonts.layout(text.to_owned(), font_id, color, f32::INFINITY)` (identical galley, identical width). Any substitution used MUST be recorded in the 4.5 adherence report. No other substitution is allowed.
- Font pin: `egui::FontId::default()`. Rationale: measurement is only ever compared against width budgets measured with the SAME font, and the egui default body font renders LARGER than the typical Windows caption font — measurements overestimate, which is conservative on top of D2's margin. The absolute font mismatch with the OS title bar is absorbed by D2.

### D2 — Safety margin: 85 % of the viewport outer width

- Constant `TITLE_BAR_USABLE_WIDTH_FRACTION: f32 = 0.85`, applied by the WIRING (not by the pure module): `available_width = TITLE_BAR_USABLE_WIDTH_FRACTION * outer_width`.
- Width source: `screen_context.input(|i| i.viewport().outer_rect.map(|r| r.width()))`; when `outer_rect` is `None` (early frames), fall back to `screen_context.viewport_rect().width()` — computed OUTSIDE the input closure (no nested context calls inside the closure). Pin the two-read shape:
  `let outer_width = ctx.input(|i| i.viewport().outer_rect.map(|r| r.width()));` then `outer_width.unwrap_or_else(|| ctx.viewport_rect().width())`.
- Rationale: 0.85 leaves ~15 % for the OS caption text, icon, and the min/max/close button cluster (at 1920 px maximized ≈ 288 points reserved — generous, cheap truncation cost); the fraction was the spec's recommended value (§6 item 2).
- Width-sampling policy (recorded limitation, matches spec §2.2's pinned update frequency): the width is sampled ONLY at recomposition time (path change or maximize/restore change). A plain window resize does NOT recompose the title until the next navigation or maximize/restore transition. The TODO requires updates on navigation only — this is contract-conformant.

### D3 — Segment separator: the Windows path separator `\`

- Constant `PATH_SEGMENT_SEPARATOR: &str = "\\"` in the pure module; the composed title is `segments.join(PATH_SEGMENT_SEPARATOR)`.
- Rationale: the title shows a Windows path (the app is Windows-only); `\` matches what the user sees in Explorer.

### D4 — Segmentation: string-based over the shell-parseable display form (NOT `std::path::Components`)

- The pure module converts the stored path through the EXISTING pure `crate::open_with_listing::shell_parseable_path` (Wave 6.2 — `\\?\UNC\…` → `\\…` first, `\\?\` → stripped, pass-through otherwise; string-level ops, OS-independent, already tested) and then splits the lossy display string on BOTH `\` and `/`, dropping empty segments.
- Rationale 1 (verbatim-prefix hygiene): the stored `opened_image_path` is the `fs::canonicalize` output, which on Windows carries the `\\?\` VERBATIM prefix (the Wave-6.2 root-cause evidence); showing `\\?\C:\…` in a user-facing title is noise. The conversion is READ-ONLY display hygiene — the stored path is NEVER rewritten (the D5 single-normalization point stands).
- Rationale 2 (container testability): `std::path::Components` is OS-dependent — in the Linux container a Windows-style literal path never decomposes, so drive/UNC/chain logic would be untestable agent-side. String splitting makes the whole composition container-testable.
- Segment roles pinned (`fn path_segments(path: &Path) -> PathSegments`):
  - UNC (display string starts with `\\` after conversion): the FIRST TWO segments joined by `\` form ONE drive segment (`\\server\share`).
  - Local drive: the FIRST segment when it is exactly 2 characters with `:` as the second character (e.g. `C:`).
  - Everything between the drive (if any) and the LAST segment: the folder chain, ordered OUTERMOST..INNERMOST (path order).
  - The LAST segment: the file name (+ extension). When the path has NO segments (e.g. an empty path), the file name falls back to the ellipsis literal `...`.
- Known note (recorded, acceptable): a drive-less (Unix-style) chain loses its leading `/` in the joined display form (segments start at the first folder). This is cosmetic and container-only — at runtime on Windows the path always carries a drive or UNC prefix. Do NOT add root-marker logic.

### D5 — Ellipsis form: the literal `...` (three ASCII dots)

- Constant `TITLE_ELLIPSIS: &str = "..."`. Used BOTH as the between-drive-and-chain marker and as the last-chars truncation prefix.
- Rationale: the TODO literally writes `...`; the Unicode `…` risks font/glyph inconsistencies; one literal for both roles keeps the composition single-source.

### D6 — Name-only truncation rule (also the "name itself too long" rule): same fraction, same measurement

- `fit_or_truncate_last(text, budget)`: if the text fits the budget → return it COMPLETE (never ellipsized when it fits). Otherwise return `TITLE_ELLIPSIS` + the longest TAIL of the text (walking `char_indices` from the left, first fitting candidate wins) such that ellipsis+tail fits; if even the bare ellipsis does not fit, return the bare ellipsis `...` (the MINIMUM output — never an empty title, never a panic).
- The not-maximized rule uses the SAME budget (`available_width` from D2) and the SAME fitter applied to `file_name_with_extension(path)` — no separate threshold constant (one budget, one rule; the spec's §6 item 5 "safety margin before truncating" is satisfied by D2's fraction).

### D7 — Maximized composition: exact step order, skip-and-continue, termination

Pin the composition as a candidate-driven state machine over `(visible_folder_count, drive_visible)`. `candidate(count, drive_visible)` builds `segments` in display order: `[drive?] + ([TITLE_ELLIPSIS] if drive visible AND any folder is omitted) + [the INNERMOST `count` folders in path order] + [base]`, joined by `PATH_SEGMENT_SEPARATOR`.

Step order (maximized = `true` only):

1. Step 0 — full path: build the FULL chain (drive + ALL folders + complete name, joined). If it fits the budget → return it verbatim (TODO bullet 2). This check happens ONCE, before any truncation.
2. Step 1 — base name: `base = fit_or_truncate_last(file_name_with_extension)` (D6). Under the pinned Interpretation A (below), a truncated base greedily fills the budget.
3. Step 2 — containing folder (FIRST priority): if the candidate with `visible_folder_count = 1`, drive hidden fits → set `visible_folder_count = 1`. If it does not fit, `visible_folder_count` stays 0 (the containing folder is skipped, the loop CONTINUES).
4. Step 3 — drive (SECOND priority): if the drive is `Some` and the candidate with the CURRENT `visible_folder_count`, drive visible fits → set `drive_visible = true`. The `TITLE_ELLIPSIS` segment appears between the drive and the visible chain EXACTLY when at least one folder is omitted (including when ZERO folders are visible — e.g. `C:\...\img.png`). If the drive does not fit → it is SKIPPED (not terminal; the loop continues to the parents).
5. Step 4 — parent walk-up (THIRD priority): for the remaining parents in NEAREST-FIRST order (folders[len-2], folders[len-3], …, folders[0]): if the candidate with `visible_folder_count + 1` (drive visibility unchanged) fits → increment; else STOP the walk immediately (each added parent only ADDS width, so the first failure is final).
6. Return `candidate(visible_folder_count, drive_visible)`.

Termination: every loop is bounded (step 4 by `folders.len()`; the fitter by the char count); no recursion; the minimum output is the bare ellipsis (D6).

Interpretation A (PINNED, decisive): a TRUNCATED base consumes the whole budget, so when the name itself was too long NOTHING else is added — the folders/drive additions apply only to the REMAINING space after a COMPLETE name. Rationale: the TODO states the name rule (complete or last-chars) BEFORE the folder additions and gates them on "remaining space"; a truncated name filling the width leaves no remaining space by construction. The alternative (reserving budget for the chain) would require an unstated reservation policy the TODO does not give. Test 12 below pins this.

### D8 — Maximization detection: `ctx.input(|i| i.viewport().maximized).unwrap_or(false)`

- Exactly the spec §2.5 pin. `None` (unknown/early frames) falls back to `false` → name-only mode; launch is always maximized so the first known state is `Some(true)` and the transition self-corrects (the maximized-state change retriggers composition).

### D9 — Empty-state and browsed-empty-folder titles: `Images Scroller`

- `compose_window_title` with `path == None` returns `crate::app::APP_WINDOW_TITLE` (`"Images Scroller"`) regardless of the maximized flag.
- PINNED and recorded: while an empty sibling folder is browsed (no opened image), the title STAYS `Images Scroller` — the TODO's algorithm covers only images; the simplest consistent choice is to keep the base title (the 4.1a spec pinned the same, §2.2).

### D10 — Fullscreen popup: lifetime, timing constants, disarm

- Constants (homed in the PURE module so tests can pin them; the wiring imports them — see §4.1): `FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS: f64 = 3.0` and `FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS: f64 = 0.05`.
- Arm: EXACTLY when `fullscreen_on == true` AND the opened image path changed this frame → text = `file_name_with_extension(path)` (cached in state; STABLE for the full 3 s even if another navigation occurs), deadline = `now + 3.0` (clock = `ctx.input(|i| i.time)` — the single-clock precedent).
- Expiry: per frame, `now >= deadline` → cleared; while armed, `ctx.request_repaint_after(Duration::from_secs_f64(remaining + 0.05))` re-requested EVERY pass (the Wave-5.1 `esc_close_confirm::request_expiry_wake` precedent — eframe sleeps at `ControlFlow::Wait`).
- Disarm: toggling fullscreen OFF clears the popup immediately (inside `toggle_fullscreen_and_send_command`, OFF branch only). Nothing else disarms (open/navigate/zoom/fit do not).
- Recorded, no special-case: while fullscreen is ON the OS title bar is hidden, so whatever `ViewportCommand::Title` produces during fullscreen is invisible; maximized-state flaps during fullscreen transitions are harmless and self-correct on exit via the change-gated recomposition.

### D11 — Fullscreen popup visuals: full ESC-close precedent mirror

Pin EXACTLY (mirroring `src/app/esc_close_confirm.rs`, own constants — do NOT modify esc_close_confirm.rs visibility):

| Property | Constant | Value |
|---|---|---|
| Area id salt | `FULLSCREEN_NAME_POPUP_AREA_ID` | `"fullscreen_image_name_popup"` (`egui::Id::unique`) |
| Anchor | — | `egui::Align2::CENTER_TOP` with offset `egui::vec2(0.0, 16.0)` (const `FULLSCREEN_NAME_POPUP_TOP_OFFSET`) |
| Order / interactivity | — | `egui::Order::Foreground`, `.interactable(false)` |
| Fill | `FULLSCREEN_NAME_POPUP_FILL_COLOR` | `egui::Color32::from_rgba_unmultiplied_const(20, 20, 20, 220)` |
| Corner radius | `FULLSCREEN_NAME_POPUP_CORNER_RADIUS` | `10` (u8) |
| Inner margin | `FULLSCREEN_NAME_POPUP_INNER_MARGIN_POINTS` | `16.0` |
| Text size | `FULLSCREEN_NAME_POPUP_TEXT_SIZE` | `14.0` |
| Text color | `FULLSCREEN_NAME_POPUP_TEXT_COLOR` | `egui::Color32::WHITE` |
| Caption paint | — | PLAIN `frame_ui.label(RichText::new(...).size(14.0).color(WHITE))` — NOT the outlined-caption paint |

Rationale: the popup sits on a dark filled frame (same as the ESC-close popup) so the pill-style outlined-caption technique (built for transparent pill backgrounds) is unnecessary; mirroring the proven popup palette keeps one visual language. The visual constants are NOT unit-tested (visual-only, manual OPERATOR-RUN territory); the TIMING constants are (D10, homed in the pure module).

### D12 — Popup text source: shared helper

- `pub(crate) fn file_name_with_extension(path: &Path) -> String` in the pure module is the SINGLE source for the name+extension form; the title composition, the not-maximized rule, AND the popup arming all consume it (implemented via `path_segments`, D4 — one segmentation source).

### D13 — Wiring call sites (exact)

- Per-frame method `update_window_title_and_popups(&mut self, screen_context: &egui::Context)` is called in `eframe::App::ui` (`src/app.rs`), AFTER `read_and_dispatch_keystrokes(ui.ctx())` (so a same-frame navigation's new path is already stored) and BEFORE `show_chrome_or_fullscreen_layout(frame, ui)` — once per frame.
- Popup render: END of `show_fullscreen_layout` (`src/app/fullscreen.rs`) — `window_title::show_fullscreen_name_popup(self, ui.ctx());` — so it paints above the central surface (`Order::Foreground`) and only exists in the fullscreen path.
- Disarm: inside `toggle_fullscreen_and_send_command` (`src/app/fullscreen.rs`), after the existing `disarm_esc_close_popup()`, OFF-branch only (D10).

### D14 — app.rs 200-line cap management (decided, mechanical)

`src/app.rs` is EXACTLY at the 200-line cap. The five new session fields are therefore grouped into ONE state struct owned by the new child module, and one existing method is relocated verbatim:

- New field: `window_title_state: window_title::WindowTitleState` (1 doc-comment line + 1 field line; initializer + 1 line).
- `mod window_title;` appended to the mod block (after `mod zoom_button;`, keeping alphabetical order).
- The `ui()` call line (D13).
- RELOCATED verbatim: `show_navigation_layout` (doc comment + 6-line body, app.rs lines 115–121) moves to `src/app/central_surface.rs` as `pub(super) fn show_navigation_layout` on the SAME `impl ImagesScrollerApp` (thematic home — it composes the pill + the navigation central surface), with its `floating_controls::show_floating_pill(self, ui)` call qualified as `super::floating_controls::show_floating_pill(self, ui);`. app.rs's `show_chrome_layout` call site is UNCHANGED (the method keeps its name and receiver).
- Line-budget math (the implementer verifies): 200 − 7 (relocated) + 5 (added) = **198 lines**. `central_surface.rs`: 44 + 7 = 51. `fullscreen.rs`: 58 + 1 use + 3 (OFF-branch disarm) + 1 render call = 63. `main.rs`: 50 + 1 (`mod window_title;` after `mod zoom;`, alphabetical) = 51.

### D15 — Module and test layout (census delta pinned)

- `src/window_title.rs` — NEW top-level pure module (declared in `src/main.rs`): composition logic + task constants + `#[cfg(test)] mod tests;` child declaration.
- `src/window_title/tests.rs` — NEW child test module (the `folder_listing/tests.rs` precedent). This is the census delta: **+1 test-bearing file → 29** (19 `*_tests.rs` + 5 `tests.rs` + 5 in-file test modules).
- `src/app/window_title.rs` — NEW wiring child module (declared in `src/app.rs`): `WindowTitleState`, the per-frame method, the popup render. NO test module (the 4.1a spec §4.4 pin: the wiring is left to manual verification; the pure core carries the full matrix). Adding a test module here would break the pinned census 29 — FORBIDDEN.
- Contingency (ONLY sanctioned split, decided in advance): if the pure file cannot stay ≤ 200 lines with compact doc comments, move `PathSegments` + `path_segments` + the display/split/drive helpers into a child module `src/window_title/segments.rs` (non-test) declared from `src/window_title.rs`. Nothing else may be split.

### D16 — Single-section boolean rule compliance (pre-decided helper names)

Per `.kilo/rules/single-section-boolean-conditions.md`, the two compound conditions the feature would naturally write are pre-extracted:

- `fn title_recomposition_is_due(path_changed: bool, maximized_changed: bool) -> bool` (free fn in the wiring module: `path_changed || maximized_changed`).
- `fn fullscreen_name_popup_rearm_is_due(&self, path_changed: bool) -> bool` (method: `path_changed && self.fullscreen_on`).
- Inside the pure composition, the compound checks fold into named `ChainComposer` methods (`containing_folder_fits`, `drive_step_fits`, `next_parent_fits`) so every `if`/`while` stays single-section (§4.1).

## 3. High-level approach

One pure, container-testable composition module owns the algorithm and the task constants; a thin app child module owns viewport reading, change-gated `ViewportCommand::Title` sending, and the fullscreen popup state/render; two existing files gain minimal hooks (intake call site, popup render + disarm). The composition is fully driven by an injected measure closure (the `EntryNameOrder` comparator-as-data precedent), so the complete behavior matrix is unit-tested in the container without egui. No dependencies change; no stored state changes beyond five RAM-only session fields grouped in one struct.

## 4. File-by-file implementation plan

### 4.1 NEW `src/window_title.rs` (pure module)

Module doc comment (write at implementation): "Window-title composition (Wave-8 Task 4): pure, container-testable fitting of the visible image's path into the window title — full path, name+extension, last-chars truncation, containing-folder/drive/parent-chain priority with the `...` gap marker — plus the shared name extraction and the task's pinned constants (segment separator, ellipsis, usable-width fraction, popup seconds). Text width is supplied by an injected measure closure; no egui context is touched here."

Exact public surface (names, visibility, shapes PINNED):

```rust
use std::path::Path;
use crate::app::APP_WINDOW_TITLE;              // the empty-state title, single source
use crate::open_with_listing::shell_parseable_path; // display-form conversion (read-only)

pub(crate) const PATH_SEGMENT_SEPARATOR: &str = "\\";
pub(crate) const TITLE_ELLIPSIS: &str = "...";
pub(crate) const TITLE_BAR_USABLE_WIDTH_FRACTION: f32 = 0.85;
pub(crate) const FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS: f64 = 3.0;
pub(crate) const FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS: f64 = 0.05;

/// Composition request (param object — the project max-2-args rule; the 4.1a
/// spec's 4-argument sketch is realized as this struct).
pub(crate) struct TitleComposition<'m> {
    pub(crate) path: Option<&'m Path>,
    pub(crate) available_width: f32,   // the margin-adjusted budget (D2 — wiring-applied)
    pub(crate) is_maximized: bool,
    pub(crate) measure: &'m mut dyn FnMut(&str) -> f32,
}

pub(crate) fn compose_window_title(request: TitleComposition<'_>) -> String
pub(crate) fn file_name_with_extension(path: &Path) -> String

#[cfg(test)]
mod tests;
```

Internal shapes (PINNED names/behavior; bodies are the implementer's to write from this spec):

```rust
struct PathSegments {
    drive: Option<String>,    // "C:" or "\\server\share" (D4 rules)
    folders: Vec<String>,     // OUTERMOST..INNERMOST, drive excluded, name excluded
    file_name: String,        // last segment; ELLIPSIS fallback when the path has none
}

fn path_segments(path: &Path) -> PathSegments
// = title_display_path -> split_display_path -> role partition per D4
fn title_display_path(path: &Path) -> String      // shell_parseable_path(...).to_string_lossy().into_owned()
fn split_display_path(display: &str) -> Vec<String> // split on BOTH '\\' and '/', drop empties
fn is_local_drive_segment(segment: &str) -> bool    // exactly 2 chars, second char ':'

/// Width fitter (D6). Holds the budget and the borrowed measure closure.
struct WidthFitter<'m> {
    max_width: f32,
    measure: &'m mut dyn FnMut(&str) -> f32,
}
impl WidthFitter<'_> {
    fn fits(&mut self, text: &str) -> bool            // (self.measure)(text) <= self.max_width
    fn fit_or_truncate_last(&mut self, text: &str) -> String
}

/// Candidate-state machine (D7). Single-section booleans by construction.
struct ChainComposer<'m, 'f> { /* segments, base, visible_count, drive_visible, fitter */ }
impl ChainComposer {
    fn full_chain(&self) -> String                    // drive + ALL folders + base(name), joined
    fn candidate(&self) -> String                     // the D7 candidate() rule
    fn containing_folder_fits(&mut self) -> bool      // folders non-empty AND candidate(1,false) fits
    fn drive_step_fits(&mut self) -> bool             // drive Some AND candidate(count,true) fits
    fn next_parent_fits(&mut self) -> bool            // candidate(count+1, drive) fits
}
```

`compose_window_title` body (PINNED control flow):

```text
let Some(path) = request.path else { return APP_WINDOW_TITLE.to_owned(); };   // D9
let segments = path_segments(path);
let mut fitter = WidthFitter { max_width: request.available_width, measure: request.measure };
let composer = ChainComposer::new(&segments, &mut fitter);
if fitter.fits(&composer.full_chain()) { return composer.full_chain(); }      // step 0 (D7)
if !request.is_maximized { return fitter.fit_or_truncate_last(&segments.file_name); } // D6/D8
let base = fitter.fit_or_truncate_last(&segments.file_name);                  // step 1
if composer.containing_folder_fits() { /* visible_count = 1 */ }              // step 2
if composer.drive_step_fits() { /* drive_visible = true */ }                  // step 3
while composer.next_parent_fits() { /* visible_count += 1 */ }                // step 4 (stop at first failure)
composer.candidate()
```

(ChainComposer owns `base`; the truncated base is set into the composer before step 2. `fit_or_truncate_last`'s char_indices walk and the bare-ellipsis minimum are D6.)

Estimated size ~150 lines with 2–3-line doc comments; the D15 contingency applies only if the cap cannot be met.

### 4.2 NEW `src/window_title/tests.rs` (the FULL test matrix — 18 tests, exactly these)

Harness pin: a char-width stub closure `|text: &str| text.chars().count() as f32 * CHAR_WIDTH_PX` with test-local `const CHAR_WIDTH_PX: f32 = 10.0;`, and a local helper `fn compose(path_str: Option<&str>, budget_chars: usize, maximized: bool) -> String` building `Path::new(...)`/`None` and the `TitleComposition` (budget = `budget_chars as f32 * 10.0`). Path fixtures as `const`/`let` literals: `SHORT = "C:/pix/img.png"`, `DEEP = "C:/photos/album/first/image.png"`, `LONGNAME = "C:/pix/a-very-long-image-file.png"` (name = 26 chars), `QUAD = "C:/aaaa/bbbb/cccc/dddd/image.png"`, `UNIX = "/home/user/pics/img.png"`. All expectations below were computed against the 10 px/char stub — implementer re-verifies each against the algorithm.

| # | Test name | Budget (chars) | Asserts |
|---|---|---|---|
| 1 | `none_path_returns_the_pinned_empty_state_title` | 50 | `None` path → `"Images Scroller"` for BOTH maximized true and false |
| 2 | `not_maximized_shows_only_the_name_with_extension` | 50 | DEEP, not maximized → `"image.png"` (no folders, no drive) |
| 3 | `not_maximized_name_that_fits_is_complete_without_ellipsis` | 9 | DEEP, not maximized → `"image.png"` (exact-fit boundary: complete, no `...`) |
| 4 | `not_maximized_long_name_keeps_last_characters_with_ellipsis_prefix` | 12 | LONGNAME → `"...-file.png"` (starts `...`, keeps the LAST 9 chars, exactly fits 12 chars) |
| 5 | `not_maximized_even_a_truncated_name_cannot_fit_yields_the_bare_ellipsis` | 2 | LONGNAME → `"..."` (bare ellipsis is the minimum output, never empty) |
| 6 | `maximized_full_path_that_fits_is_shown_verbatim` | 31 | DEEP → `"C:\\photos\\album\\first\\image.png"` (Rust literal: `"C:\\\\photos\\\\album\\\\first\\\\image.png"`; the step-0 rule) |
| 7 | `maximized_containing_folder_added_first_when_it_fits` | 16 | DEEP → `"first\\image.png"` (drive skipped, parents stopped) |
| 8 | `maximized_drive_added_with_ellipsis_when_folders_are_omitted` | 23 | DEEP → `"C:\\...\\first\\image.png"` (`...` between drive and the one visible folder) |
| 9 | `maximized_parent_folders_walk_up_while_they_fit` | 29 | DEEP → `"C:\\...\\album\\first\\image.png"` (two folders visible; NOT containing `photos`) |
| 10 | `maximized_parent_walk_stops_at_the_first_parent_that_does_not_fit` | 30 | QUAD → `"C:\\...\\cccc\\dddd\\image.png"`; NOT containing `"bbbb"` or `"aaaa"` (walk stopped at the first failure) |
| 11 | `maximized_drive_skipped_when_it_does_not_fit_but_parents_are_still_added` | 20 | `"C:/ab/album/image.png"` → `"ab\\album\\image.png"`; NOT containing `"C:"` and NOT containing `"..."` (failed drive is skipped, loop continued — D7 step 3/4) |
| 12 | `maximized_truncated_name_consumes_the_width_so_no_folders_are_added` | 12 | LONGNAME, maximized → `"...-file.png"` (Interpretation A pinned: a truncated base leaves no remaining space) |
| 13 | `maximized_unix_style_path_without_drive_composes_the_folder_chain` | 22 | UNIX → `"home\\user\\pics\\img.png"` (drive-less chain; the D4 leading-slash note) |
| 14 | `verbatim_prefix_is_stripped_before_segmentation` | 50 | `Path::new(r"\\?\C:\pix\img.png")` → `"C:\\pix\\img.png"` (D4/D5 display hygiene) |
| 15 | `unc_path_forms_a_single_drive_segment` | 50 | `Path::new(r"\\?\UNC\server\share\pix\img.png")` → `"\\\\server\\share\\pix\\img.png"` |
| 16 | `file_name_with_extension_extracts_the_last_segment` | — | `file_name_with_extension(Path::new(DEEP))` → `"image.png"` |
| 17 | `path_with_no_file_name_falls_back_to_the_ellipsis_name` | — | `file_name_with_extension(Path::new(""))` → `"..."` |
| 18 | `task_constants_are_pinned` | — | `PATH_SEGMENT_SEPARATOR == "\\"`, `TITLE_ELLIPSIS == "..."`, `TITLE_BAR_USABLE_WIDTH_FRACTION == 0.85`, `FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS == 3.0`, `FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS == 0.05` |

(Backslash counts in the Rust literals: each displayed `\` is `\\` in source; e.g. test 6's expected literal is `"C:\\\\photos\\\\album\\\\first\\\\image.png"`.)

### 4.3 NEW `src/app/window_title.rs` (wiring child module)

Module doc comment: "Window-title + fullscreen-name-popup wiring (Wave-8 Task 4): reads the viewport's maximized state and outer width once per frame, recomposes the title through the pure `crate::window_title` module with an egui-font measure closure, sends `ViewportCommand::Title` ONLY when the computed string changed, and arms/expires the 3-second top-center fullscreen name popup (the `esc_close_confirm` wake precedent)."

Contents (PINNED):

```rust
use std::path::PathBuf;
use eframe::egui;
use crate::window_title::{
    self, TitleComposition, FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS,
    FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS, TITLE_BAR_USABLE_WIDTH_FRACTION,
};

/// The five Wave-8 Task-4 session fields (spec §2.1), grouped for the app.rs cap (D14).
/// RAM-only, never persisted.
pub(super) struct WindowTitleState {
    last_title_image_path: Option<PathBuf>,
    last_maximized_state: Option<bool>,
    last_sent_window_title: String,
    fullscreen_name_popup_expires_at: Option<f64>,
    fullscreen_name_popup_text: String,
}
impl WindowTitleState {
    pub(super) fn initial() -> Self {
        Self { last_title_image_path: None, last_maximized_state: None,
               last_sent_window_title: String::new(),
               fullscreen_name_popup_expires_at: None, fullscreen_name_popup_text: String::new() }
    }
}
```

Visual constants (D11 table): `FULLSCREEN_NAME_POPUP_AREA_ID`, `FULLSCREEN_NAME_POPUP_TOP_OFFSET`, `FULLSCREEN_NAME_POPUP_FILL_COLOR`, `FULLSCREEN_NAME_POPUP_CORNER_RADIUS`, `FULLSCREEN_NAME_POPUP_INNER_MARGIN_POINTS`, `FULLSCREEN_NAME_POPUP_TEXT_SIZE`, `FULLSCREEN_NAME_POPUP_TEXT_COLOR` — module-private (`const`), values per D11.

Methods on `ImagesScrollerApp` (impl block in this file):

```rust
pub(super) fn update_window_title_and_popups(&mut self, screen_context: &egui::Context)
pub(super) fn disarm_fullscreen_name_popup(&mut self)   // expires_at = None
pub(super) fn fullscreen_name_popup_is_visible(&self) -> bool  // expires_at.is_some()
fn fullscreen_name_popup_rearm_is_due(&self, path_changed: bool) -> bool  // D16
```

`update_window_title_and_popups` body — EXACT step order (D16 rule compliance marked):

```text
1. let (maximized_flag, outer_width) = screen_context.input(|i|
       (i.viewport().maximized, i.viewport().outer_rect.map(|r| r.width())));
   let is_maximized = maximized_flag.unwrap_or(false);                       // D8
2. let now = screen_context.input(|i| i.time);
3. popup expiry: if now >= deadline (when armed) -> expires_at = None;       // single-section if
4. popup wake: while armed and remaining > 0 ->
   screen_context.request_repaint_after(Duration::from_secs_f64(remaining
       + FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS));                  // D10, single-section if
5. let path_changed = self.window_title_state.last_title_image_path.as_ref()
       != self.opened_image_path.as_ref();                                    // borrow-by-reference
   let maximized_changed = self.window_title_state.last_maximized_state != Some(is_maximized);
6. if title_recomposition_is_due(path_changed, maximized_changed):           // D16 helper
     let title = crate::window_title::compose_window_title(TitleComposition {
         path: self.opened_image_path.as_deref(),
         available_width: TITLE_BAR_USABLE_WIDTH_FRACTION * outer_width
             .unwrap_or_else(|| screen_context.viewport_rect().width()),     // D2
         is_maximized,
         measure: &mut |text| measure_text_width(screen_context, text),
     });
     if title != self.window_title_state.last_sent_window_title {            // single-section if
         screen_context.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
         self.window_title_state.last_sent_window_title = title;
     }
     self.window_title_state.last_title_image_path = self.opened_image_path.clone();
     self.window_title_state.last_maximized_state = Some(is_maximized);
7. if self.fullscreen_name_popup_rearm_is_due(path_changed):                 // D16 helper
     if let Some(path) = self.opened_image_path.as_ref() {                   // single-section ifs
         self.window_title_state.fullscreen_name_popup_text =
             crate::window_title::file_name_with_extension(path);            // D12
         self.window_title_state.fullscreen_name_popup_expires_at =
             Some(now + FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS);              // D10
         screen_context.request_repaint_after(Duration::from_secs_f64(
             FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS
                 + FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS));
     }
```

Helpers (module-private):

```rust
fn measure_text_width(screen_context: &egui::Context, text: &str) -> f32 {
    screen_context.fonts(|fonts| {
        fonts.layout_no_wrap(text.to_owned(), egui::FontId::default(), egui::Color32::WHITE)
    })
    .size()
    .x
}   // D1; the sanctioned fonts_mut/layout(f32::INFINITY) substitution applies here if 0.37 rejects it
```

Free render function (PINNED shape, mirrors `esc_close_confirm::show_esc_close_popup`):

```rust
pub(super) fn show_fullscreen_name_popup(app: &ImagesScrollerApp, screen_context: &egui::Context) {
    if !app.fullscreen_name_popup_is_visible() { return; }
    egui::Area::new(egui::Id::unique(FULLSCREEN_NAME_POPUP_AREA_ID))
        .anchor(egui::Align2::CENTER_TOP, FULLSCREEN_NAME_POPUP_TOP_OFFSET)
        .order(egui::Order::Foreground)
        .interactable(false)
        .show(screen_context, |popup_ui| {
            egui::Frame::new()
                .fill(FULLSCREEN_NAME_POPUP_FILL_COLOR)
                .corner_radius(FULLSCREEN_NAME_POPUP_CORNER_RADIUS)
                .inner_margin(FULLSCREEN_NAME_POPUP_INNER_MARGIN_POINTS)
                .show(popup_ui, |frame_ui| {
                    frame_ui.label(egui::RichText::new(
                            &app.window_title_state.fullscreen_name_popup_text)
                        .size(FULLSCREEN_NAME_POPUP_TEXT_SIZE)
                        .color(FULLSCREEN_NAME_POPUP_TEXT_COLOR));
                });
        });
}
```

(Child-module access to the parent struct's private field is valid Rust — privacy is per module tree.)

### 4.4 EDIT `src/app.rs` (exact edits, cap math per D14)

1. Field: after `open_with_programs` add
   `/// Session window-title + fullscreen-name-popup state (managed by the \`window_title\` child module).` and `window_title_state: window_title::WindowTitleState,`
2. `empty_state()`: add `window_title_state: window_title::WindowTitleState::initial(),` (after `open_with_programs: Vec::new(),`).
3. `eframe::App::ui`: insert `self.update_window_title_and_popups(ui.ctx());` between the intake line and `self.show_chrome_or_fullscreen_layout(frame, ui);`.
4. REMOVE `show_navigation_layout` (doc comment + body, 7 lines) — relocated per D14.
5. Mod block: `mod window_title;` after `mod zoom_button;`.
Result: 198 lines (verify; if the count lands > 200, STOP and ask the caller — do not improvise further removals).

### 4.5 EDIT `src/app/central_surface.rs`

Add the relocated method verbatim (doc comment + signature + body unchanged except the fully-qualified pill call):

```rust
/// Pill layout: the floating pill plus the central surface -- the image paint when one is open, else the empty-folder message.
pub(super) fn show_navigation_layout(&mut self, ui: &mut egui::Ui) {
    super::floating_controls::show_floating_pill(self, ui);
    egui::CentralPanel::default().show(ui, |central_ui| {
        self.show_navigation_central_surface(central_ui);
    });
}
```

### 4.6 EDIT `src/app/fullscreen.rs`

1. Add `use super::window_title;` to the imports.
2. `toggle_fullscreen_and_send_command`: after `self.disarm_esc_close_popup();` add
   `if !self.fullscreen_on { self.disarm_fullscreen_name_popup(); }` (D10/D13).
3. `show_fullscreen_layout`: after the `egui::CentralPanel::default().show(...)` block add
   `window_title::show_fullscreen_name_popup(self, ui.ctx());` (D13).
Doc comment of the module gains one line: "Wave-8 Task 4 addition: toggling fullscreen OFF disarms the top-center name popup, which renders only in the fullscreen layout."

### 4.7 EDIT `src/main.rs`

Add `mod window_title;` after `mod zoom;` (alphabetical). No other change — `window_options()`/`with_maximized(true)`/`APP_WINDOW_TITLE` stay untouched (4.5a checklist item).

### 4.8 EDIT `.agent/project-structure.md`

- `# Files in src/` gains a bullet for `window_title.rs` (pure window-title composition: full-path/name/last-chars fitting, the containing-folder/drive/parent chain with the `...` marker, the shared name extraction, the task constants; tests in the `window_title/` child module).
- `# Folders in src/` gains `window_title/ - child module: tests.rs (the 18-test composition matrix …)`.
- The `app/` bullet gains `window_title.rs (the per-frame title/popup wiring: viewport state reading, change-gated ViewportCommand::Title sends, the 3-second fullscreen name-popup state + render)` — mind the existing bullet's truncation style; append consistently.
- The `app.rs` bullet's summary clause gains the window-title state mention.

## 5. Git discipline and commit sequence (4.2)

Branch `feat/wave-8-wizard-installer` already exists (HEAD `b1d8e13`); the implementer works ON it (branch creation/switch is Step-2 work — DONE). `git push` is FORBIDDEN (workflow Step 5). Version bump is DONE (do not touch `Cargo.toml`). Before EVERY commit: run `git status`, confirm no `.gitignore`-matching paths (`dist/`, `logs/`, `target/`, `*.log`) are staged (Gitignore Compliance Rule).

1. `docs: add wave8 task4 window-title spec and implementation plan` — commits BOTH untracked plan files: `.kilo/plans/20261118-wave8-task4-window-title-frontend-spec.md` (4.1a) and this plan file.
2. `feat: window title shows the visible image path` — the complete feature: `src/window_title.rs`, `src/window_title/tests.rs`, `src/app/window_title.rs`, edits to `src/app.rs`, `src/app/central_surface.rs`, `src/app/fullscreen.rs`, `src/main.rs`.
3. `docs: wave8 task4 structure map entries` — `.agent/project-structure.md` (§4.8). (Structure-map updates are pinned to THIS sub-step, 4.2, as their own commit — 4.4 stays prose-free.)
4. (Conditional) any 4.3 review/simplification fixes → their own commits (`fix: wave8 task4 review fixes` / simplification plan files as required by the workflow).
5. (4.4) doc-comment polish only, if any → `docs: wave8 task4 window-title module docs`.
6. (4.5a/4.5b) report files → their own plan-file commits.
7. (4.6) `task: mark wave8 task4 done` — the TODO mark (`[]` → `[x]` on Task 4's sub-items; preserve all other content).

## 6. Console commands (container harness — the ONLY allowed execution environment)

All commands run through the `alpine-vm` MCP; call `alpine-vm_vm_status` FIRST (if the VM or the `/img-scroller` share is unavailable, STOP and return a user question). The exit code is the authoritative evidence; cargo stderr may be absent in captured MCP output (absent stderr is NOT a failure signal).

1. Dev checks (after commit 2, re-run after any fix commit):
   `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
   Expected: exit `0`, final `ALL CHECKS PASSED`, test summary `212 passed; 0 failed` across **29 test-bearing files** (§7). ANY nonzero exit, census mismatch, or new clippy/fmt finding beyond the pre-existing benign `compare_entry_names` dead-code warning → fix or STOP-and-ask; never ship a red run.
2. NO release build, NO installer build, NO windows-target compile in 4.2 — those belong to Task 5's verification step (the TODO assigns them there; dev-checks already compiles fmt/check/test/clippy).

## 7. Census expectations (PINNED)

- Test-bearing files: 28 → **29** (the single delta: NEW `src/window_title/tests.rs`, a `tests.rs` child file — the in-file test-module count stays 5, the `*_tests.rs` count stays unchanged).
- Unit tests: 195 → **212** (exactly the 18 new tests of §4.2; all other per-file counts unchanged).
- No dependency changes: `Cargo.toml`/`Cargo.lock` are std-only-untouched (git diff against `b1d8e13` must show NO manifest/lock changes).

## 8. NOT-to-do list (hard prohibitions for the implementer)

- Do NOT edit `Cargo.toml`/`Cargo.lock` (no new dependencies — the feature is std + egui only).
- Do NOT touch the version, `main.rs`'s `window_options()`, or `with_maximized(true)`.
- Do NOT write any README/tech/architecture/context/brief/CHANGELOG/docs prose (Task 6's job); 4.4 is module doc-comments only.
- Do NOT modify `esc_close_confirm.rs` (the popup constants are DUPLICATED in the new module per D11 — do not re-export or widen its visibility).
- Do NOT rewrite the stored `opened_image_path` — `shell_parseable_path` is used READ-ONLY at display time (D4).
- Do NOT add a test module to `src/app/window_title.rs` (census 29 pin, D15).
- Do NOT send `ViewportCommand::Title` every frame or on plain window resizes (spec §2.2 + D2 policy).
- Do NOT inline the wiring into `app.rs` or exceed the 200-line cap anywhere; the only sanctioned cap contingencies are D14 (relocation, mandatory) and D15 (segments split, only if needed).
- Do NOT re-inline anything into `keyboard_intake.rs`; the intake file is untouched by this feature.
- Do NOT edit any existing `.kilo/plans/*` file; do NOT mark the TODO done before 4.6; do NOT push.

## 9. Conformance hand-off (for 4.5a/4.5b)

The 4.1a spec's §7 checklist governs; additionally verify: D1's measurement API (or its recorded substitution), D2's 0.85 fraction and two-read width shape, D3–D5 constants, D7's step order (tests 6–12), D9's browsed-folder title, D10's arm/disarm/wake triad, D11's visual table, D13's three call sites, D14's 198-line app.rs, and the §7 census. Any deviation MUST be reported with its rationale, never silently absorbed.
