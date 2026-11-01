# Task 3 — Front-end Implementation Verification Report

**Date:** 2026-11-01  
**Wave:** Wave 2 — Horizontal Scroll (Sibling Images)  
**Workflow step:** 4.5a (frontend-specialist)  
**Branch:** `feat/wave-2-horizontal-scroll`  
**Commits reviewed:** `4b45ca7`, `6f00e6b`  
**Spec:** [`.kilo/plans/20261031-wave2-task3-ui-buttons-frontend-spec.md`](./20261031-wave2-task3-ui-buttons-frontend-spec.md) — **approved layout §K3 (Corner Rails)**  
**Implementation files:** `src/app.rs`, `src/app/rails.rs`, `src/app/tests.rs`, `src/app_display.rs`

---

## 1. §K3 + conformance-checklist assertions → evidence → verdict

| # | Assertion | Evidence (file:line) | Verdict |
|---|-----------|----------------------|---------|
| 1 | Right rail container `Panel::right("folder_rail")`, ~40 pt width, vertical layout | `src/app/rails.rs:28-33` (`default_size(RAIL_SIZE)` with `RAIL_SIZE = 40.0`, `top_down` layout) | **PASS** |
| 2 | Bottom rail container `Panel::bottom("image_rail")`, ~40 pt height, horizontal layout | `src/app/rails.rs:45-50` (`default_size(RAIL_SIZE)`, `left_to_right` layout) | **PASS** |
| 3 | Arrow glyphs `←`, `→`, `↑`, `↓` at exact positions (left/right in bottom rail, up/down in right rail) | `src/app/rails.rs:37` (`↑`), `39` (`↓`), `60` (`←`), `66` (`→`) | **PASS** |
| 4 | Every arrow button uses `min_size(egui::vec2(28.0, 24.0))` | `src/app/rails.rs:10` (`ARROW_BUTTON_SIZE`), `13-15` (`arrow_button`) | **PASS** |
| 5 | Rails rendered only while an image is open (`image_is_open` gate) | `src/app.rs:114-116` (`image_is_open`), `134-137` (`if self.image_is_open()`) | **PASS** |
| 6 | Panel order is top → right → bottom → `CentralPanel` last | `src/app.rs:130-143` (top panel, then rail block, then `CentralPanel`) | **PASS** |
| 7 | `↑` / `↓` are statically disabled with `add_enabled(false, …)` and call no handler | `src/app/rails.rs:36-40` (`add_enabled(false, …)` only; no `.clicked()` arms) | **PASS** |
| 8 | `←` click calls exactly `app.navigate(NavigationDirection::Previous)`; no other behavior | `src/app/rails.rs:59-64` | **PASS** |
| 9 | `→` click calls exactly `app.navigate(NavigationDirection::Next)`; no other behavior | `src/app/rails.rs:65-70` | **PASS** |
| 10 | Message area renders `Some(message)` as label and `None` as empty label; state untouched | `src/app/rails.rs:55` (read `message_text`), `71` (`label(message.unwrap_or(""))`); no assignment to `message_text` in rails code | **PASS** |
| 11 | Old conditional `message_bar` panel fully removed | No `message_bar` string in `src/` (grep returned no matches) | **PASS** |
| 12 | Menu placeholder | **K3 spec defines no menu placeholder**; implementation also contains none. Spec-vs-impl diff: **none**. | **N/A** |
| 13 | Buttons disabled exactly when `neighbor_image_path(direction).is_none()` via `add_enabled` | `src/app/rails.rs:19-24` (`navigation_button_enabled`), `53-54`, `59`, `66` | **PASS** |
| 14 | No duplicate navigation handler / no direct mutation of `opened_image_path`, `decoded_image`, `message_text`, `current_display` from button code | `navigate(` appears only at definition and the two click arms + keyboard arms; grep for `message_text =` / `opened_image_path =` / `decoded_image =` in `src/app` found no assignments in rails code | **PASS** |
| 15 | Texture/display path unchanged; no per-frame texture churn | `src/app_display.rs` untouched; `CurrentImageDisplay::paint` still uploads only when `texture_upload_pending` is true (`src/app_display.rs:54-66`) | **PASS** |
| 16 | No new UI-only state fields | `ImagesScrollerApp` struct in `src/app.rs:15-26` unchanged except `current_display` from Wave 1; rail code derives enablement per frame | **PASS** |
| 17 | Keyboard arrow handling unchanged at top of `ui()` | `src/app.rs:121-128` (identical to Task 2 seam) | **PASS** |

---

## 2. egui API fact cross-check

All API claims in the spec and implementation were verified against the landed egui 0.37.0 docs.

| Claim | Source | Evidence in code |
|-------|--------|------------------|
| `egui::Button::new(text)` constructs a text button; `Button` implements `Widget`; `Button::min_size(Vec2)` sets the minimum button size | docs.rs `Button`: `pub fn new(atoms: impl IntoAtoms<'a>) -> Self`, `pub fn min_size(self, min_size: Vec2) -> Self` ([Button docs](https://docs.rs/egui/0.37.0/egui/widgets/struct.Button.html)) | `src/app/rails.rs:13-15` |
| `ui.add_enabled(enabled, widget) -> Response` produces a greyed-out, non-interactive widget when `enabled` is `false` | docs.rs `Ui::add_enabled`: `pub fn add_enabled(&mut self, enabled: bool, widget: impl Widget) -> Response` ([Ui docs](https://docs.rs/egui/0.37.0/egui/struct.Ui.html)) | `src/app/rails.rs:37, 39, 60, 66` |
| `Panel::right` / `Panel::bottom` exist and take an ID salt; `default_size(f32)` clamps the outer size on the panel axis | Context7 `/emilk/egui` panel builder snippet (`default_size`, `min_size`, `Panel::right`, `Panel::bottom`) | `src/app/rails.rs:28-30, 45-47` |
| `CentralPanel` must be added last after top/left/right/bottom panels | Context7 canonical `Panels` demo: "Always add any CentralPanel last" | `src/app.rs:139-143` |

---

## 3. Front-end quality (operator-perceivable)

- **Button hit targets:** `min_size(28.0, 24.0)` is applied to each arrow button. The actual clickable area is at least that size plus the default button frame/padding, so the 28 × 24 pt minimum is met.
- **Rail spacing:** The right rail uses `add_space(ARROW_BUTTON_SPACING)` (`8.0` pt) between `↑` and `↓`; the bottom rail relies on egui's default `item_spacing` between `←`, `→`, and the message label, giving a compact but not crowded bar.
- **Message truncation/wrapping:** The message label uses `ui.label(message.unwrap_or(""))` with the default `TextWrapMode`, so long messages wrap within the remaining bottom-rail width rather than being truncated with an ellipsis. This matches the spec's "wraps or truncates" wording.
- **Edge-disabled consistency:** `←` is disabled at the first sibling and `→` at the last sibling, exactly mirroring the keyboard STAY semantics (a key press at the edge is a silent no-op; a click cannot occur because the button is disabled).
- **Wave-3 reserve visibility:** `↑` and `↓` are visible only when an image is open and are always disabled in Wave 2, clearly signalling future folder navigation without performing any action.
- **Known K3 consequence (user-confirmed, NOT a defect):** Because both rails are gated on `image_is_open()`, a failed open from the empty state sets `message_text` but has no visible surface. This is the literal, user-approved K3 layout behavior; `message_text` state semantics are untouched.

---

## 4. Spec-vs-implementation diffs

- **Menu placeholder:** K3 spec does not define a menu placeholder, and none is present in the landed rails. Diff: none.
- **Exact line counts vs. plan prediction:** The implementation landed at `src/app.rs` 160 lines (plan predicted 166), `src/app/rails.rs` 74 lines (plan predicted ~70), and `src/app/tests.rs` 222 lines (plan predicted 198). Only `tests.rs` crosses the project hard limit (see §5).
- **Lifetime annotation on `arrow_button`:** The implementation returns `egui::Button<'_>` (clippy lifetime-elision fix in `6f00e6b`) rather than the spec's bare `egui::Button`. This is a minor, compile-driven local detail and does not change behavior.

---

## 5. Must-fix items

1. **`src/app/tests.rs` exceeds the `max-lines-per-file` hard limit.**
   - Project rule: source files in `src/` must not exceed 200 lines.
   - Current count: **222 lines**.
   - **Required action:** Reduce the file to ≤200 lines. Options: split the new button-enablement test into a child module (e.g. `src/app/tests/buttons.rs`) and declare it from `src/app/tests.rs`, or refactor the new test to fewer lines without losing coverage. Do not manually reflow purely to evade the rule; follow the existing module pattern used by `src/app/tests.rs`.

No other front-end defects were found.

---

## 6. Summary

- **PASS** on all layout, wiring, enablement, and state-contract assertions.
- **PASS** on egui API usage and panel-order constraints.
- **One FAIL:** `src/app/tests.rs` is 222 lines, violating the 200-line source-file cap.
- The user-approved K3 empty-state message-visibility consequence is present as specified and is **not** reported as a defect.
