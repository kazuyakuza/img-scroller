# Task 3 — Front-end Technical Specification: On-Screen Navigation UI Buttons

**Date:** 2026-10-31  
**Wave:** Wave 2 — Horizontal Scroll (Sibling Images)  
**Workflow step:** 4.1a (frontend-specialist)  
**Scope boundary:** On-screen previous/next image buttons. Up/down folder buttons and a menu-area placeholder are included only as future-wave reuse slots; their behavior is not wired in this task. Pan-then-navigate, zoom, fullscreen, and "Open with…" are out of scope.

---

## 1. Reuse contract — buttons are new UI entry points into the existing navigation seam

Task 2 landed a single private navigation method in `src/app.rs`:

- `NavigationDirection` enum (file-private, lines 28–31):
  ```rust
  enum NavigationDirection {
      Previous,
      Next,
  }
  ```
- `ImagesScrollerApp::navigate(&mut self, direction: NavigationDirection)` (lines 90–94):
  ```rust
  fn navigate(&mut self, direction: NavigationDirection) {
      if let Some(neighbor_path) = self.neighbor_image_path(direction) {
          self.open_image(neighbor_path);
      }
  }
  ```

**The UI buttons MUST call exactly this method.** There must be no second handler, no inline decode logic, and no direct mutation of `opened_image_path`, `decoded_image`, `message_text`, or `current_display` from the button code.

Expected call sites (inside whichever panel closure the approved proposal picks):

```rust
if ui.add_enabled(
    self.neighbor_image_path(NavigationDirection::Previous).is_some(),
    egui::Button::new("←").min_size(egui::vec2(28.0, 24.0)),
).clicked() {
    self.navigate(NavigationDirection::Previous);
}

if ui.add_enabled(
    self.neighbor_image_path(NavigationDirection::Next).is_some(),
    egui::Button::new("→").min_size(egui::vec2(28.0, 24.0)),
).clicked() {
    self.navigate(NavigationDirection::Next);
}
```

The enable predicate reuses `neighbor_image_path`, so a button is disabled in exactly the same cases the keyboard path is a no-op: no image open, no folder listing, unresolvable index, or at the first/last list position (Task 2 edge rule: STAY, silent).

---

## 2. egui 0.37 Button API facts

Verified against the landed `src/app.rs` Panel style and the egui 0.37.0 docs:

- `egui::Button::new(text)` constructs a text button. `Button` implements `Widget`, so it is added with `ui.add(...)` or the convenience `ui.button(...)`.
- `Response::clicked()` is the standard click check.
- `Button::min_size(Vec2)` sets a minimum size for the clickable area.
- Disabled buttons are produced with `ui.add_enabled(enabled: bool, widget: impl Widget) -> Response`. When `enabled` is `false` the widget is greyed out and non-interactive. Signature verified in source: `pub fn add_enabled(&mut self, enabled: bool, widget: impl Widget) -> Response` ([`egui::Ui` source, line 1713](https://docs.rs/egui/0.37.0/src/egui/ui.rs.html)).
- Fixed-size allocation is available via `ui.add_sized(max_size: impl Into<Vec2>, widget: impl Widget) -> Response` ([`egui::Ui` source, line 1663](https://docs.rs/egui/0.37.0/src/egui/ui.rs.html)).
- The existing Wave-1 code uses `egui::Panel::top(...)` and `egui::Panel::bottom(...)`. The same Panel API supports `Panel::left(...)`, `Panel::right(...)`, `Panel::top(...)`, and `Panel::bottom(...)` with `.min_size(...)`, `.default_size(...)`, and `.resizable(...)` ([`egui/containers/panel.rs` source](https://docs.rs/egui/0.37.0/src/egui/containers/panel.rs.html)).

Sources:

- `egui::Button` docs: <https://docs.rs/egui/0.37.0/egui/widgets/struct.Button.html>
- `egui::Ui` docs: <https://docs.rs/egui/0.37.0/egui/struct.Ui.html>
- `egui::Panel` source: <https://docs.rs/egui/0.37.0/src/egui/containers/panel.rs.html>
- Task 2 implementation plan (navigation seam): `.kilo/plans/20261031-wave2-task2-keyboard-nav.md`
- Landed code: `src/app.rs` lines 28–31 and 90–117

---

## 3. Concrete UI-layout proposals

These three proposals are self-contained options for the full navigation-control area. **No layout is being decided here; the Planner/user must approve one before 4.1b finalizes the implementation plan.**

Each proposal names the real egui container(s), the button glyphs, the disabled-state rule, and the interaction with the existing bottom message bar and central image area.

### K1 — Bottom navigation bar left-aligned beside message

**Containers:** A single `egui::Panel::bottom("bottom_bar")`, replacing the current conditional message bar. The panel is shown on every frame with a fixed height of about 32 points.

**Compact sketch:**

```text
┌────────────────────────────────────────────────────┐
│ [Open…]                                            │  ← existing top panel
├────────────────────────────────────────────────────┤
│                                                    │
│              central image area                    │
│                                                    │
├────────────────────────────────────────────────────┤
│ [←] [→] [↑] [↓]  <message text or empty>           │  ← K1 bottom bar
└────────────────────────────────────────────────────┘
```

**Button glyphs/labels:** Unicode arrows `←`, `→`, `↑`, `↓`; each button gets `min_size(egui::vec2(28.0, 24.0))`.

**Active/disabled state:**

- When no image is open, all four arrow buttons are disabled.
- `←`/`→` are disabled at the first/last sibling image (Task 2 STAY rule).
- `↑`/`↓` are disabled in Wave 2 (reserved for Wave 3 folder navigation).
- Buttons are **never hidden** so the bar keeps a stable shape.

**Interaction with message bar and central image:** The message text is appended immediately to the right of the buttons in the same horizontal row. If the message is long it wraps or truncates within the remaining bar width. Because the bar is always shown, the empty state displays four disabled buttons instead of a completely blank bottom. The central image area sits above the bar and is unchanged.

**Pros:** Reuses existing bottom-bar real estate; no new panels; keeps top bar minimal.

**Cons:** Message text can be pushed far right and truncated; four disabled buttons in an empty window may look busy; navigation controls are distant from the `Open…` button.

**Classic-gallery readability:** Functional but utilitarian; the bottom status-bar pattern is familiar, though mixing controls and status text is less common in classic viewers.

---

### K2 — Top toolbar extension (after Open…)

**Containers:** The existing `egui::Panel::top("open_toolbar")` is widened into a full control bar. The bottom message bar stays as-is.

**Compact sketch:**

```text
┌────────────────────────────────────────────────────┐
│ [Open…]  [←] [→] [↑] [↓]          [Menu ▾]         │  ← K2 top toolbar
├────────────────────────────────────────────────────┤
│                                                    │
│              central image area                    │
│                                                    │
├────────────────────────────────────────────────────┤
│ <message bar — appears only on failure>            │  ← existing bottom bar
└────────────────────────────────────────────────────┘
```

**Layout construction:** `ui.horizontal(|ui| { self.open_toolbar_button(ui, frame); ui.add_space(12.0); /* nav buttons */; ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { /* menu placeholder */ }) })`.

**Button glyphs/labels:** Same arrow glyphs as K1, same 28×24 minimum size. The menu placeholder is a disabled button or label reading `Menu ▾` (or `⋮`) with the same minimum height.

**Active/disabled state:**

- Same arrow-button rules as K1.
- The menu placeholder is disabled when no image is open ("Open with…" needs a current file).

**Interaction with message bar and central image:** The bottom message bar remains conditional and shows only on decode failure. The central image area is unaffected except for the slightly taller top bar.

**Pros:** All file-level controls live together with `Open…`; the bottom bar remains dedicated to status messages; strongly evokes the Windows Photo Viewer-era toolbar layout.

**Cons:** The top bar becomes more crowded; the menu placeholder is not a real menu until a later wave.

**Classic-gallery readability:** High — a top toolbar with Open + prev/next + a menu is the pattern many classic image viewers use.

---

### K3 — Corner rails (right rail up/down, bottom rail left/right)

**Containers:** Two extra panels plus the unchanged top `Open…` panel:

- `egui::Panel::right("folder_rail")`, width ~40 points, vertical layout, holds `↑`/`↓`.
- `egui::Panel::bottom("image_rail")`, height ~40 points, horizontal layout, holds `←`/`→` and the message area.

**Compact sketch:**

```text
┌─────────────────────────────────────┬──────────────┐
│ [Open…]                             │              │
├─────────────────────────────────────┤     [↑]      │
│                                     │              │
│         central image area          │     [↓]      │  ← right rail
│                                     │              │
├─────────────────────────────────────┴──────────────┤
│      [←] [→]  <message text or empty>              │  ← bottom rail
└────────────────────────────────────────────────────┘
```

**Button glyphs/labels:** Same arrow glyphs and 28×24 minimum size.

**Active/disabled state:**

- Same arrow-button rules as K1.
- The rails are **shown only when an image is open**; in the empty state only the top `Open…` panel is visible, leaving the central area clean.

**Interaction with message bar and central image:** The bottom rail absorbs the message area, so the separate conditional message bar is removed. The central image area is framed by the rails; `CentralPanel` is added last and fills the remaining space.

**Pros:** Strong spatial mapping (left/right below the image, up/down beside it); the top bar stays minimal and clean; visually distinctive.

**Cons:** Two new panels reduce usable image area; panel ordering must be respected (top, right, bottom, then central); the layout is less familiar to users expecting a classic Photo Viewer toolbar.

**Classic-gallery readability:** Lower — this is more media-center than classic gallery viewer.

---

### Recommendation

**RECOMMENDED: K2 — Top toolbar extension (after Open…).**

Reason: it groups the primary file/navigation controls in the most predictable location for a Windows Photo Viewer-era gallery app, leaves the bottom bar exclusively for status messages, and requires only one existing panel to be extended.

---

## 4. Out-of-scope / deferred

The following items are explicitly NOT wired in Task 3:

- **Fullscreen/zoom/fit-mode controls** — only a placeholder area is proposed; wiring is Wave 4+.
- **"Open with…" menu** — only a placeholder control is proposed; wiring is a future wave.
- **Up/down folder navigation** — the `↑`/`↓` buttons may appear as disabled/reserved slots per the approved proposal, but they must call no navigation handler in Wave 2. Wiring is Wave 3.
- **Pan-then-navigate interplay** — Wave 5.
- **Drag-to-pan or mouse-wheel zoom** — not in any current wave plan.
- **Edge-of-list behavior** — already decided by Task 2 (STAY, silent) and reused here.
- **Window-title changes, accessibility announcements, tooltips** — deferred.

---

## 5. Conformance checklist for later 4.5a verification

**[Approved proposal: <filled by caller>]**

Use the checklist below once the user has approved a proposal and the implementer has landed code:

- [ ] The approved proposal's panel(s) and layout order are implemented exactly as specified.
- [ ] Left/right UI buttons call `self.navigate(NavigationDirection::Previous)` and `self.navigate(NavigationDirection::Next)` respectively.
- [ ] There is no separate button handler, no duplicate decode logic, and no direct state mutation outside the existing `open_image` / `accept_open_outcome` seam.
- [ ] Buttons are disabled when `neighbor_image_path(direction).is_none()`; `ui.add_enabled` (or equivalent) is used.
- [ ] Up/down buttons, if shown, are disabled in Wave 2 and call no navigation handler.
- [ ] The menu placeholder, if shown, is disabled when no image is open and performs no action.
- [ ] The texture/display path remains the Wave-1 one-upload-per-decode implementation; buttons do not trigger per-frame texture churn.
- [ ] No new state fields are added solely for button UI.
- [ ] File/method size, depth, private-by-default, and self-documenting rules are respected.

---

## 6. Project-rule constraints that shape the GUI code

| Rule | How it applies to Task 3 |
|------|--------------------------|
| `max-lines-per-file` (≤200 lines; ideally ≤125 substantive) | Button wiring should be a small helper method in `src/app.rs`; if `ui()` grows too large, extract a private `navigation_control_bar` method. |
| `max-lines-per-method` (≤50 lines) | Each panel closure and helper is short; reuse `neighbor_image_path` for enable logic. |
| `max-depth` (≤2 nested blocks) | Multi-section conditions are extracted into named helpers. |
| `prefer-private-members` | Any new layout helper is private by default. |
| `self-documenting-code` | Names like `previous_image_enabled`, `next_image_enabled`, `menu_placeholder_enabled`. |
| `single-section-boolean-conditions` | The enable check for each button is a single call such as `self.neighbor_image_path(direction).is_some()`. |
| `no-commented-code` | No commented-out code is left in source files. |

---

## 7. Acceptance criteria

1. Left and right on-screen buttons perform the same previous/next sibling-image navigation as the Left/Right arrow keys.
2. Buttons are disabled whenever navigation is not possible (no image, no listing, at list edges).
3. The approved layout is implemented exactly; no other layout is silently chosen.
4. Button clicks reuse `ImagesScrollerApp::navigate` and the existing open/decode/state seam.
5. The display/texture path uploads once per successful decode and does not churn per frame.
6. Up/down buttons and the menu placeholder do not perform undeclared actions in Wave 2.
7. Container dev-checks and tests pass (evidence comes from the implementer/architector verification step).
