# Front-end Verification Report — Wave 2 centered floating controls

**Date:** 2026-11-01  
**Branch:** `fix/centered-floating-controls`  
**Commits reviewed:** `6ef0240` + `f30b802`  
**Spec:** `.kilo/plans/20261101-floating-controls-frontend-spec.md`  
**Implementation plan:** `.kilo/plans/20261101-floating-controls.md`  
**Container/test status:** quartet green at `f30b802` (provided by implementer).  

## Executive summary

All three binding user directives are satisfied. The 19 spec §6 conformance assertions all pass. The implementation deviates from the exact plan text in three minor, compile-correct ways (`egui::Id::unique` instead of `Id::new` for the two Areas, `Area::show(..., ctx)` instead of `show(..., Ui)`, and `Color32::from_rgba_unmultiplied_const` instead of `from_rgba_unmultiplied`). None of these change operator-visible behavior, but they are recorded as spec/plan diffs below.

**No MUST-FIX front-end items.**

## §6 conformance checklist

| # | Assertion | Status | Evidence |
|---|-----------|--------|----------|
| 1 | Window shell unchanged: title `Images Scroller`, resizable, not maximized. | PASS | `src/app.rs:174-181` `window_options()` keeps `.with_title(APP_WINDOW_TITLE)`, `.with_resizable(true)`, `.with_maximized(false)`. |
| 2 | `ImagesScrollerApp` state fields are not expanded solely for UI surface needs. | PASS | `src/app.rs:15-26` fields are byte-identical to the landed struct. |
| 3 | Top `open_toolbar` panel and `open_toolbar_button` helper are removed entirely. | PASS | Grep `src/` for `open_toolbar` returns no matches; `src/app.rs` has no top-panel block. |
| 4 | `src/app/rails.rs` deleted; `mod rails;` removed; no commented-out rail code remains. | PASS | `src/app/*.rs` glob has no `rails.rs`; git diff `HEAD~2..HEAD` shows `D src/app/rails.rs` and `R055 src/app/rails_tests.rs → src/app/floating_controls_tests.rs`; grep `src/` for `rails::\|folder_rail\|image_rail\|RAIL_SIZE` returns nothing. |
| 5 | Floating controls are shown only when `image_is_open()`. | PASS | `src/app.rs:161-162` branches on `image_is_open()` and calls `show_image_layout`, which invokes `floating_controls::show_floating_pill` at `src/app.rs:112`. The empty-state branch (`src/app.rs:164`) never shows the pill. |
| 6 | The floating row is centered at the bottom of the window and is not a space-reserving panel (uses `Area` or equivalent overlay pattern). | PASS | `src/app/floating_controls.rs:34-42` creates `egui::Area` anchored `Align2::CENTER_BOTTOM` with offset `(0.0, -24.0)` and `Order::Foreground`. It is drawn above the `CentralPanel` image surface. |
| 7 | Previous button calls `app.navigate(NavigationDirection::Previous)`. | PASS | `ArrowControl::ImagePrevious` maps to `NavigationDirection::Previous` (`src/app/arrow_direction.rs:60`); click arm at `src/app/floating_controls.rs:116-118` calls `app.navigate(direction)`. |
| 8 | Next button calls `app.navigate(NavigationDirection::Next)`. | PASS | `ArrowControl::ImageNext` maps to `NavigationDirection::Next` (`src/app/arrow_direction.rs:61`); same click arm as above. |
| 9 | Previous/Next enablement equals `neighbor_image_path(direction).is_some()`. | PASS | `src/app/floating_controls.rs:24-29` `navigation_button_enabled` returns exactly that; `src/app/floating_controls.rs:96-101` uses it for image arrows. |
| 10 | Up/Down buttons are visible but disabled in Wave 2 (`add_enabled(false, …)`) and call no navigation handler. | PASS | `ArrowControl::FolderUp/FolderDown` return `None` from `navigation_direction()` (`src/app/arrow_direction.rs:62`); `arrow_button_is_enabled` returns `false` (`src/app/floating_controls.rs:97-99`); `accepts_click` is `false` when disabled (`src/app/floating_controls.rs:144-146`), so `app.navigate` is never reached for them. |
| 11 | No second button handler, no inline decode logic, and no direct mutation of `opened_image_path`/`decoded_image`/`message_text` from the UI code. | PASS | The only UI-initiated state change is `app.navigate(direction)` (`src/app/floating_controls.rs:117`). |
| 12 | The failure message is displayed only while an image is open; empty-state failures show no message. | PASS | `show_message_line` lives inside the pill (`src/app/floating_controls.rs:58-62`), which only exists in the `image_is_open()` branch. |
| 13 | Empty state shows one big centered `Open…` button that calls `open_via_dialog(frame)`. | PASS | `src/app.rs:133-147` creates an `Area` anchored `CENTER_CENTER` with a `Button::new(RichText::new("Open…").size(18.0)).min_size(Vec2::new(180.0, 48.0))`; click calls `self.open_via_dialog(open_dialog_frame)` at `src/app.rs:145`. |
| 14 | No `Open…` control appears while an image is open. | PASS | The only render site is the `else` branch of the `image_is_open()` split (`src/app.rs:163-165`); the constant `OPEN_LABEL_TEXT` is defined once at `src/app.rs:170` and used only in `show_centered_open_button`. |
| 15 | Arrow-key intake is checked before the overlay is drawn and remains unchanged. | PASS | `src/app.rs:153-160` reads `ArrowLeft`/`ArrowRight` at the top of `ui()` before any panels/areas are shown. |
| 16 | The overlay does not capture or consume arrow-key events. | PASS | The floating pill is an `egui::Area` with default `Sense::click()` buttons; there is no focusable widget, so arrow keys fall through to the frame-first key intake. |
| 17 | Texture upload remains gated by `texture_upload_pending`; no per-frame re-upload or allocation churn. | PASS | `src/app_display.rs:53-66` is unchanged; `ensure_current_image_texture` returns early when `!texture_upload_pending` and uploads exactly once per decode. |
| 18 | All new/modified source files respect project rules (≤200 lines, ≤50-line methods, ≤2 param preference, private-by-default, single-section booleans, self-documenting names, no commented-out code). | PASS | Line counts: `src/app.rs` 192 L, `src/app/floating_controls.rs` 170 L, `src/app/arrow_direction.rs` 65 L, `src/app/floating_controls_tests.rs` 32 L, `src/app/tests.rs` 187 L — all ≤200. Method bodies are ≤50 lines; no function has more than 2 non-self params; items are `pub(super)` only where needed; compound conditions are isolated in `accepts_click` and `arrow_button_is_enabled`; no commented-out code. |
| 19 | Unit tests are updated to reflect the retired rails module and final enablement helper location; test count is recorded. | PASS | `src/app/rails_tests.rs` renamed to `src/app/floating_controls_tests.rs`; test semantics unchanged. Total `#[test]` count across `src/` is 26 (`app/tests.rs` 9, `floating_controls_tests.rs` 1, `app_display.rs` 2, `scaling.rs` 5, `image_loading.rs` 4, `folder_listing/tests.rs` 5). |

## User directives verification

### (a) No font-dependent glyph remains in any button

- Grep for the Unicode arrows `← → ↑ ↓` in `src/app/**` returns no matches.
- Grep for ASCII arrow-like labels `"<"`, `">"`, `"^"`, `"v"` inside button constructors returns no matches.
- All four arrow buttons are painted with `egui::Shape::rotated_triangle` at `src/app/floating_controls.rs:151-156`.
- The only text rendered on buttons is the empty-state `"Open…"` label (`src/app.rs:142`).

**Verdict:** PASS.

### (b) Floating centered bottom pill replaces rails and toolbar

- No top toolbar panel exists in `src/app.rs`.
- `src/app/rails.rs` is deleted; `mod rails;` is gone.
- Controls appear only when an image is open and are rendered as a single `Area` anchored `CENTER_BOTTOM` (`src/app/floating_controls.rs:34-42`), not as a `Panel` that reserves layout space.
- The pill is raised with `Order::Foreground` and sits over the `CentralPanel` image surface.

**Verdict:** PASS.

### (c) Open visible ONLY in the empty state

- `src/app.rs:161-165` branch-structurally decides: image-open branch shows only the pill + image; empty-state branch shows only the centered `Open…` button.
- There is exactly one `Open…` control in the codebase (`src/app.rs:170` constant, rendered at `src/app.rs:141`).
- With an image open, the empty-state branch never executes, so zero Open controls are rendered.

**Verdict:** PASS.

## Cross-checks

- **Key intake:** unchanged and frame-first (`src/app.rs:153-160`).
- **Area arrow-key consumption:** the floating Areas contain only clickable arrow regions; no focusable widget intercepts `ArrowLeft`/`ArrowRight`.
- **Message placement:** the unsupported-file label is rendered above the button row inside the same pill `Frame` (`src/app/floating_controls.rs:58-62`), and only while an image is open.

## Diffs between spec/plan and implementation

The following are compile-correct deviations from the exact spec/plan wording. They do not alter operator-visible behavior, but are recorded for the architector's 4.5b consolidation:

1. **Area IDs:** spec/plan specified `egui::Id::new(FLOATING_PILL_AREA_ID)` and `egui::Id::new(EMPTY_STATE_OPEN_AREA_ID)`; implementation uses `egui::Id::unique(...)` at `src/app/floating_controls.rs:34` and `src/app.rs:138`. This makes the IDs unstable frame-to-frame; with anchored, stateless Areas it does not break clicks, but it is not the spec's intended stable ID and may churn the accessibility tree.
2. **`Area::show` receiver:** spec/plan wrote `.show(host_ui, |area_ui| …)`; implementation calls `.show(host_ui.ctx(), |area_ui| …)` at `src/app/floating_controls.rs:40` and `.show(central_ui.ctx(), …)` at `src/app.rs:140`. The container build is green, so this matches the 0.37 API overload that accepts a `Context`.
3. **Color constructor:** spec/plan used `Color32::from_rgba_unmultiplied`; implementation uses `Color32::from_rgba_unmultiplied_const` at `src/app/floating_controls.rs:13-14` to make the initializer a `const`. Both produce the same `rgba(20,20,20,180)` value.
4. **Line count:** `src/app.rs` is 192 L (plan estimated ~183 L); still well within the 200-line rule.
5. **Project structure doc:** `.agent/project-structure.md` still lists `rails.rs`, `rails_tests.rs`, and the retired top Open button. This is expected to be refreshed in Cycle C docs task; it is not a code defect.

## Quality issues (operator-perceivable)

| Issue | Severity | Notes |
|-------|----------|-------|
| Pill legibility over bright images | LOW / informational | Fill is `rgba(20,20,20,180)` (`src/app/floating_controls.rs:13-14`). Under egui's default dark theme the arrow icons use the light default `fg_stroke` colors and remain readable. If the app is ever switched to a light theme, the dark pill + dark default foreground would reduce contrast; consider pinning dark visuals or tinting arrows light. |
| Button hit sizes | ACCEPTABLE | Hit rect is `36×32` logical points (`src/app/floating_controls.rs:17`), above the retired `28×24` rail buttons and meeting the spec minimum. |
| Disabled visual clarity of Wave-3 reserves | ACCEPTABLE | Disabled arrows use `widgets.noninteractive.fg_stroke.color` (`src/app/floating_controls.rs:163`), which is the standard dimmed palette. They are visually distinct from enabled arrows. |
| Center-Open prominence | ACCEPTABLE | `180×48` logical points, `18 pt` label, anchored `CENTER_CENTER` (`src/app.rs:138-145`). It dominates the empty state as required. |

## Must-fix items

None.

## Sign-off

Front-end surface matches the binding spec and user directives. Safe to proceed to architector 4.5b / Cycle B build verification.
