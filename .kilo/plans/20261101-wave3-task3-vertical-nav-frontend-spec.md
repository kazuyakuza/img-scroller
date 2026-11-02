# Wave 3 Task 3 — Vertical scroll (sibling folders) keyboard and buttons

**Date:** 2026-11-01  
**Workflow step:** 4.1a (frontend-specialist)  
**Source TODO:** [`.agent/todos/20261029/20261029-todo-5.md`](../../.agent/todos/20261029/20261029-todo-5.md)  
**Branch:** `feat/wave-3-vertical-scroll`  
**Status:** Front-end technical specification — user directives only; no implementation plan.

---

## 1. Directives (verbatim from TODO Task 3)

- Up arrow key AND up UI button → previous sibling folder; Down arrow key AND down UI button → next sibling folder.
- Target image priority (binding correction): (a) if the target folder was already visited this session → its session last-viewed image; (b) otherwise → ALWAYS the target folder's FIRST image in alphanumeric order. NO same-filename attempt, NO other fallback.
- The newly shown image updates the session last-viewed map for the target folder.
- Behavior for a sibling folder containing no displayable image (skip / disable / error) is decided in this wave's architector step.
- Pan interaction: out of scope here — arrows navigate directly until Wave 5 implements pan-then-navigate.

---

## 2. Keyboard intake

### 2.1 Intake pattern

Use the same frame-first egui 0.37 input pattern already in `src/app.rs`:

```rust
let up_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowUp));
let down_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowDown));
```

Both are evaluated in `ImagesScrollerApp::ui()` **before** the `image_is_open()` branch, exactly like `ArrowLeft` / `ArrowRight` today.

### 2.2 Continuation of existing gating

`app.rs` currently does **not** gate `ArrowLeft`/`ArrowRight` behind `image_is_open()` explicitly; it calls `self.navigate(...)` unconditionally and the underlying `neighbor_image_path(...)` returns `None` when no image is open, at a list edge, or the listing is missing. The Wave 3 intake must follow the same contract:

```rust
if up_pressed {
    self.navigate(/* previous sibling folder direction */);
}
if down_pressed {
    self.navigate(/* next sibling folder direction */);
}
```

The vertical resolution seam (provided by the architector's back-end plan) returns `None` for the no-image-open case, so the UI produces no change and no message.

### 2.3 No pan interplay

`ArrowUp`/`ArrowDown` are navigation keys only. There is no pan step, no directional scrolling, and no key-repeat special handling. Wave 5 owns pan-then-navigate.

---

## 3. Button activation contract

### 3.1 Current state

`src/app/arrow_direction.rs` defines:

```rust
pub(super) fn navigation_direction(self) -> Option<NavigationDirection> {
    match self {
        Self::ImagePrevious => Some(NavigationDirection::Previous),
        Self::ImageNext => Some(NavigationDirection::Next),
        Self::FolderUp | Self::FolderDown => None,
    }
}
```

### 3.2 Activation change

Wave 3 activates the existing ↑/↓ slots **in place**. The only code change on the UI side is the mapping inside `ArrowControl::navigation_direction`:

- `FolderUp` must return a direction that means "previous sibling folder under the same parent".
- `FolderDown` must return a direction that means "next sibling folder under the same parent".

The exact Rust type/variant names are a 4.1b architector decision (for example: extending `NavigationDirection` with `SiblingFolderPrevious` / `SiblingFolderNext`, or introducing a separate enum that `navigate` accepts). The UI contract is fixed:

| Button | Action | Must call |
| --- | --- | --- |
| ↑ | Previous sibling folder | the same `app.navigate(...)` seam as keyboard/←/→ |
| ↓ | Next sibling folder | the same `app.navigate(...)` seam as keyboard/←/→ |

`src/app/floating_controls.rs` must not gain a second navigation handler, inline decode logic, or direct mutation of `opened_image_path`/`decoded_image`/`message_text`.

### 3.3 Target-image priority is not a UI concern

The "visited → last-viewed; new folder → first image" rule is resolved inside the vertical navigation seam. The UI receives only the final target image path (or `None`) and calls `app.open_image(target_path)`. `accept_open_outcome` already records the displayed image in `last_viewed_images`, so the map update requires no additional UI code.

---

## 4. Enablement semantics

### 4.1 Per-frame mirroring of ←/→

A folder arrow button is enabled exactly when the vertical navigation seam resolves a displayable target image for that direction in the current frame. The existing helper pattern is:

```rust
pub(super) fn navigation_button_enabled(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> bool {
    app.neighbor_image_path(direction).is_some()
}
```

Wave 3 introduces an equivalent vertical-resolution helper (name and signature decided by 4.1b). `arrow_button_is_enabled` in `floating_controls.rs` will then use that helper for the folder arrows:

```rust
fn arrow_button_is_enabled(arrow_control: ArrowControl, app: &ImagesScrollerApp) -> bool {
    let Some(direction) = arrow_control.navigation_direction() else {
        return false;
    };
    vertical_navigation_button_enabled(app, direction) // or equivalent seam
}
```

### 4.2 UI-visible states

| Situation | Required enablement | Visual |
| --- | --- | --- |
| No image open | Disabled (pill is not rendered anyway) | N/A |
| Folder at list edge (no previous/next sibling under same parent) | Disabled | Dimmed triangle |
| Adjacent folder exists and contains a displayable image | Enabled | Normal/hover triangle |
| Adjacent folder exists but has no displayable image | Depends on 4.1b decision (see §4.3) | See §4.3 |

### 4.3 Empty / no-image sibling-folder options (4.1b decision)

The TODO explicitly defers this behavior to the architector. The front-end consequences of each option are:

1. **Disable**
   - The button for that direction is dimmed.
   - The user cannot move to a folder that has no displayable image.
   - No message appears; the current image stays visible.
   - Consistent with the current horizontal edge-of-list behavior.

2. **Skip**
   - The vertical seam searches past the empty folder (in the same direction) until it finds a folder with a displayable image.
   - The button is enabled whenever any folder in that direction has a displayable image.
   - Edge-of-list applies only after the last non-empty folder.
   - The UI may jump multiple folder names in one key/button press; this is invisible to the front end except through enablement.

3. **Error**
   - The button is enabled whenever an adjacent sibling folder exists.
   - Pressing it attempts to open the folder, fails because there is no displayable image, and surfaces the unsupported-file message (or a new message decided by 4.1b) in the pill's message line.
   - `accept_open_outcome` keeps the current image displayed per the Wave-1 failure rule.

The spec does not pick one; the 4.1b plan must encode a single choice and the corresponding enablement rule.

---

## 5. Accessible labels

`ArrowControl::accessible_label` currently returns:

- `FolderUp` → `"Up folder"`
- `FolderDown` → `"Down folder"`

After activation these must describe the action, not just the direction:

- `FolderUp` → `"Previous sibling folder"`
- `FolderDown` → `"Next sibling folder"`

The label is exposed through the existing `widget_info` call:

```rust
button_response.widget_info(|| {
    egui::WidgetInfo::labeled(
        egui::Role::Button,
        self.enabled,
        self.control.accessible_label(),
    )
});
```

The ←/→ labels remain `"Previous image"` and `"Next image"`.

---

## 6. egui 0.37 API facts

- **Input:** `ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowUp))` is the existing egui 0.37 per-frame key-press API; it returns `bool` and is already used for `ArrowLeft`/`ArrowRight`.
- **Area/Frame:** `show_floating_pill` already uses `egui::Area` anchored `CENTER_BOTTOM` with `Order::Foreground` and an `egui::Frame` for the translucent pill background. No structural change is required.
- **Button hit-space:** buttons allocate `ARROW_BUTTON_SIZE` via `ui.allocate_exact_size(..., Sense::click())`; activation does not change allocation or paint.
- **Triangle paint:** `egui::Shape::rotated_triangle` is already used; `FolderUp`/`FolderDown` map to `ArrowDirection::Up`/`Down` with the existing rotations. No visual change.
- **Accessibility:** `Response::widget_info` with `WidgetInfo::labeled(Role::Button, enabled, label)` is unchanged.
- **Version compatibility:** All of the above APIs are stable in egui 0.37. Activation only requires new enum-variant mapping; no API migration is needed.

---

## 7. 4.5a conformance checklist

- [ ] `ArrowUp` and `ArrowDown` are read with `ui.ctx().input(|i| i.key_pressed(...))` in `ImagesScrollerApp::ui()`.
- [ ] The new keys are evaluated before the `image_is_open()` branch, matching the existing `ArrowLeft`/`ArrowRight` placement.
- [ ] Up key routes through the same `navigate` seam as the up button.
- [ ] Down key routes through the same `navigate` seam as the down button.
- [ ] Left/Right key behavior is unchanged.
- [ ] Pan interaction is absent; arrows navigate directly.
- [ ] The floating pill layout is unchanged: same `Area` id, anchor, frame, four buttons, group gap, and message line.
- [ ] The ↑/↓ buttons remain in their existing physical slots; no new widget or panel is introduced.
- [ ] `FolderUp`/`FolderDown` return a concrete navigation direction instead of `None`.
- [ ] Up button maps to "previous sibling folder"; down button maps to "next sibling folder".
- [ ] Enablement is derived per-frame from the vertical navigation seam, mirroring the horizontal `neighbor_image_path` approach.
- [ ] Disabled folder arrows paint dimmed via the existing `arrow_color` logic.
- [ ] Accessible labels for ↑/↓ are `"Previous sibling folder"` / `"Next sibling folder"`.
- [ ] The message line remains the only status surface above the button row and is unchanged in placement/formatting.
- [ ] No per-frame texture re-upload or new allocation churn is introduced.
- [ ] No UI code directly mutates `opened_image_path`, `decoded_image`, `message_text`, or `last_viewed_images`.
- [ ] New/modified source files respect project rules (≤200 lines, ≤50-line methods, ≤2-level nesting, private-by-default, self-documenting names, no commented-out code).

---

## 8. Pending-decision handoff for 4.1b

The architector must settle the following before implementation. Each item includes the UI consequence.

1. **Vertical navigation seam / target resolution state**
   - How is the previous/next sibling folder enumerated and resolved to a target image path?
   - Does the `NavigationDirection` enum gain new variants (`SiblingFolderPrevious`/`SiblingFolderNext`), or does `navigate` accept a richer argument?
   - UI consequence: determines the exact return type of `ArrowControl::navigation_direction` and the signature of the enablement helper called by `floating_controls.rs`.

2. **Empty / no-image sibling-folder handling**
   - Choose **disable**, **skip**, or **error**.
   - UI consequence: directly controls whether the ↑/↓ button is enabled when the adjacent folder has no displayable image, and whether a message appears.

3. **Edge-of-list behavior for folder navigation**
   - At the first/last sibling under the same parent, is the movement silently blocked (disabled), wrapped, or handled another way?
   - UI consequence: controls enablement at the top/bottom of the sibling list.

4. **Unit-test seams around folder I/O**
   - Does the implementation expose test doubles or an in-memory seam for folder listing?
   - UI consequence: determines how `floating_controls_tests.rs` can verify ↑/↓ enablement without relying on the real file system.

---

## 9. Sources / API references

- `src/app.rs` — existing `ArrowLeft`/`ArrowRight` intake, `navigate`, `neighbor_image_path`, and `image_is_open` gate.
- `src/app/floating_controls.rs` — pill layout, enablement helper, custom painted arrow buttons.
- `src/app/arrow_direction.rs` — `ArrowControl`, `ArrowDirection`, and `navigation_direction` mapping.
- `src/last_viewed_images.rs` — session last-viewed memory (already present; no UI change needed).
- `egui` 0.37 docs: `Context::input`, `Key::ArrowUp`/`Key::ArrowDown`, `Area`, `Frame`, `Shape::rotated_triangle`, `Response::widget_info`.
