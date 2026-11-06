# Wave 4.1 Task 2 — Button Hotkey References + Hover Tooltips: Front-end Technical Specification

**Date:** 2026-11-05  
**Workflow step:** 4.1a (frontend-specialist)  
**Source TODO:** [`.agent/todos/20261102/20261102-todo-1.md`](../../.agent/todos/20261102/20261102-todo-1.md) §Task 2  
**Branch:** `fix/wave-4-operator-fixes`  
**Base commit:** `86a1c2c` (Task 1 launch-state mark done)  
**Spec file:** `.kilo/plans/20261102-wave4-1-task2-references-tooltips-frontend-spec.md`

---

## 1. Binding directives

1. Each UI button displays, beneath it, a small reference of its related hotkey (keyboard-distinct buttons only).
2. Hovering the cursor over a button shows a tooltip with a short description of what that button does.
3. The no-font-glyph constraint applies **only to the painted icon glyphs** (arrow triangles, fullscreen square, zoom bars, fit-mode glyphs). Small caption text rendered through `egui::Label`/`RichText` is explicitly allowed; the message line already proves the default font renders correctly.

---

## 2. Reference map (proposed exact strings)

The four arrow buttons are already painted as the arrow keys themselves, so a duplicate arrow glyph caption adds noise and no new information. The six view-control buttons have distinct one-key hotkeys that are not visible in their painted glyphs.

| Pill button | Show caption? | Caption reference |
|-------------|---------------|-------------------|
| `ImagePrevious` (left arrow) | **No** | — |
| `ImageNext` (right arrow) | **No** | — |
| `FolderUp` (up arrow) | **No** | — |
| `FolderDown` (down arrow) | **No** | — |
| `Fullscreen` (filled square) | Yes | `F11` |
| `ZoomIn` (`+`) | Yes | `I` |
| `ZoomOut` (`−`) | Yes | `O` |
| `RestoreFit` (stroked square) | Yes | `0` |
| `FitWidth` | Yes | `W` |
| `FitHeight` | Yes | `H` |

If the architector decides to caption the arrow buttons, the constants can be changed to `Some("←")`, `Some("→")`, `Some("↑")`, `Some("↓")`; the layout mechanism stays identical.

---

## 3. Tooltip map (proposed exact strings, all ≤ 60 chars)

Tooltips mention the primary hotkey so the hover surface doubles as discoverability text. Disabled buttons still show the tooltip (via `Response::on_disabled_hover_text`).

| Pill button | Tooltip text |
|-------------|--------------|
| `ImagePrevious` | `Previous image (Left Arrow)` |
| `ImageNext` | `Next image (Right Arrow)` |
| `FolderUp` | `Previous sibling folder (Up Arrow)` |
| `FolderDown` | `Next sibling folder (Down Arrow)` |
| `Fullscreen` | `Fullscreen (F11)` |
| `ZoomIn` | `Zoom in (I)` |
| `ZoomOut` | `Zoom out (O)` |
| `RestoreFit` | `Restore fit (0)` |
| `FitWidth` | `Fit width (W)` |
| `FitHeight` | `Fit height (H)` |

---

## 4. Verified egui 0.37 API facts

- `Response::on_hover_text(text: impl Into<WidgetText>) -> Self` exists and returns the same `Response` for chaining. It shows the text when the widget is hovered and **enabled**.
- `Response::on_disabled_hover_text(text: impl Into<WidgetText>) -> Self` exists for the disabled case; `on_hover_text` is suppressed while disabled.
- Calling both methods on the same `Response` is safe: the enabled tooltip fires when enabled, the disabled tooltip fires when disabled.
- Tooltips are pointer-per-frame stateless popups; they need no stored state and render in egui's tooltip layer above the pill's `Order::Foreground` Area.
- `egui::TextStyle` has no `Tiny` variant in 0.37 (only `Small`, `Body`, `Button`, `Heading`, `Monospace`, `Name`). For a reliably tiny caption, use `RichText::new(text).size(9.0)` instead of a non-existent style.
- `egui::Layout::top_down(Align::Center)` creates a vertical child column; `Ui::allocate_ui_with_layout(desired_size, layout, |child_ui| ...)` returns an `InnerResponse` and lets us fix the column width to the button width while letting the height grow.
- `Ui::allocate_exact_size(size, Sense::click()) -> (Rect, Response)` works inside such a child `Ui` and produces the same click-response shape used today.

---

## 5. Layout shape: per-button vertical stack

### 5.1 Recommended shape (binding for the implementer)

Inside the existing pill row, each button becomes a vertical column:

```text
        column width = button width (36 px)
        ┌─────────────────┐
        │   painted glyph │  ← allocate_exact_size(36×32), Sense::click()
        │   (32 px tall)  │
        │   F11           │  ← optional 9 pt caption label (centered)
        └─────────────────┘
```

- Row layout changes from `egui::Layout::left_to_right(egui::Align::Center)` to **`egui::Layout::left_to_right(egui::Align::TOP)`**. Top alignment keeps the painted glyphs of captioned and uncaptioned columns on the same horizontal line; center alignment would lift the shorter arrow columns above the captioned ones.
- Each button column is created with `row_ui.allocate_ui_with_layout(egui::Vec2::new(BUTTON_WIDTH, 0.0), egui::Layout::top_down(egui::Align::Center), |column_ui| { ... })`.
- Inside the child column:
  1. Allocate the exact-size click area (`36.0 × 32.0`).
  2. Paint the glyph using the existing shape code.
  3. For buttons with a reference (`Some`), add a centered `RichText::new(reference).size(9.0).color(column_ui.visuals().text_color())` label.
  4. Chain tooltip methods on the returned `Response`.
- The column's `item_spacing` should be tightened to about `2.0` points between glyph and caption so the caption sits close under the glyph.
- Group gaps (`FULLSCREEN_GROUP_GAP`, `ZOOM_GROUP_GAP`, `FIT_GROUP_GAP`, `FOLDER_GROUP_GAP`) stay exactly as today; they are added between columns in the row.
- The pill frame grows taller by roughly the caption height (~10–12 px). The existing bottom-center overlay contract still holds: the pill remains an `Area` anchored above the image center and does not reserve layout space from the central image.

### 5.2 Helper module (binding location)

Create `src/app/button_hint.rs` as a pure, OS-free module owned by the pill UI. It contains:

- A `PillButton` enum covering all ten pill buttons.
- `hotkey_reference(button: PillButton) -> Option<&'static str>` returning the exact table from §2.
- `tooltip_text(button: PillButton) -> &'static str` returning the exact table from §3.
- A shared layout helper `show_button_column(...)` that creates the vertical column, renders the optional caption, and attaches the tooltip pair.
- Inline unit tests for the two pure maps.

`src/app.rs` adds `mod button_hint;`. The existing button modules (`floating_controls`, `fullscreen_button`, `zoom_button`, `fit_mode_button`) import from `button_hint` instead of duplicating strings or layout math.

### 5.3 Placement/style options for captions

| Option | Placement / style | Pros | Cons | Verdict |
|--------|-------------------|------|------|---------|
| **A — Below glyph, 9 pt, text color (RECOMMENDED)** | Caption sits 2 px under the painted glyph; uses `RichText::new(...).size(9.0).color(text_color())`. | Compact; keeps icon family visually dominant; caption text color matches the message line for consistency; top-aligned row keeps glyph baselines even. | Slightly increases pill height. | **Recommended.** |
| **B — Below glyph, 10 pt, weak text color** | Same position, slightly larger and dimmer (`weak()` or `text_color().gamma_multiply(0.7)`). | More readable for low-DPI screens. | Takes more vertical space; dimming may reduce contrast on dark pill background. | Fallback if operator-run reports readability issues. |
| **C — Above glyph** | Caption rendered between the message line and the button glyph. | Does not extend the pill downward toward the image. | Draws the eye away from the glyph; may feel like a header row; duplicates the message-line area visually. | Not recommended. |

---

## 6. Button-module change contract

For each existing pill button module, the change is limited to:

1. Replace the raw `allocate_exact_size(...)` + paint sequence with a call to `button_hint::show_button_column(...)` (or an equivalent local wrapper that follows §5.1).
2. Move the existing `widget_info(...)` and `on_hover_cursor(...)` calls onto the response produced inside the column.
3. Keep the existing click/action dispatch unchanged.
4. Do **not** add new state fields to `ImagesScrollerApp`.

Specifically:

- `floating_controls.rs`: the four arrow buttons get the tooltip pair; they do **not** render a caption.
- `fullscreen_button.rs`: renders the "F11" caption and the `Fullscreen (F11)` tooltip.
- `zoom_button.rs`: renders captions `I`, `O`, `0` and tooltips `Zoom in (I)`, `Zoom out (O)`, `Restore fit (0)`.
- `fit_mode_button.rs`: renders captions `W`, `H` and tooltips `Fit width (W)`, `Fit height (H)`.

---

## 7. Fullscreen interplay

- All pill UI is gated by `full_ui_chrome_is_visible()`.
- While fullscreen is ON the buttons are not rendered, so no references or tooltips are rendered either.
- No extra fullscreen-specific tooltip gating is required.

---

## 8. Unit-test surface

The renderable caption/tooltip behavior is OS/GUI-bound and not unit-testable in the Alpine container. The pure data shape is testable.

In `src/app/button_hint.rs` add an inline `#[cfg(test)] mod tests` with two tests:

1. `hotkey_reference_matches_the_spec` — asserts the `Option<&str>` value for every `PillButton` variant (10 assertions).
2. `tooltip_text_matches_the_spec` — asserts the exact tooltip string for every `PillButton` variant (10 assertions).

**Expected test census increase:** +2 `#[test]` items, 20 assertion-level expectations.

No tests need to mock `egui::Response`; the functions take only the enum and return `&str`.

---

## 9. 4.5a conformance checklist

- [ ] `src/app/button_hint.rs` exists and exports `PillButton`, `hotkey_reference`, `tooltip_text`, plus the shared column helper.
- [ ] No new fields are added to `ImagesScrollerApp`.
- [ ] The pill row layout uses `left_to_right(Align::TOP)`.
- [ ] Every button is wrapped in a vertical column of width equal to its button width.
- [ ] Arrow buttons show **no** caption; the six view-control buttons show the exact reference strings from §2.
- [ ] Captions use `RichText::new(...).size(9.0)` and the current visuals' text color.
- [ ] Tooltips are attached with `on_hover_text` **and** `on_disabled_hover_text` on the same `Response` used for clicks.
- [ ] Tooltip strings match §3 exactly.
- [ ] Group gaps (`FOLDER_GROUP_GAP`, `FULLSCREEN_GROUP_GAP`, `ZOOM_GROUP_GAP`, `FIT_GROUP_GAP`) are unchanged.
- [ ] Fullscreen ON hides all buttons and therefore all references/tooltips (no extra gate).
- [ ] The inline tests in `button_hint.rs` pass.
- [ ] All modified files remain under the 200-line / 50-line-method project limits.
- [ ] No per-frame texture re-upload or allocation churn is introduced (tooltips are stateless; captions use stack string literals).

---

## 10. Out of scope

- ESC / F11 immediacy fixes (Task 3).
- Zoom semantics, bounds, and alias corrections (Tasks 5–6).
- One-shot fit-mode rework (Task 7).
- Parked future ideas: fullscreen bottom-hover reveal, auto-hide chrome, drag-and-drop.
- `CHANGELOG.md` edits.

---

**Digest for Planner/architector:**
- **Recommended captions:** `F11`, `I`, `O`, `0`, `W`, `H` under their respective buttons; arrow buttons uncaptioned.
- **Tooltip list:** `Previous image (Left Arrow)`, `Next image (Right Arrow)`, `Previous sibling folder (Up Arrow)`, `Next sibling folder (Down Arrow)`, `Fullscreen (F11)`, `Zoom in (I)`, `Zoom out (O)`, `Restore fit (0)`, `Fit width (W)`, `Fit height (H)`.
- **Recommended layout:** per-button vertical column, row top-aligned, 9 pt caption below glyph.
- **Test count expected:** +2 unit tests (20 assertions).
