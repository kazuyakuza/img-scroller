# Task 2 — Front-end Technical Specification: Previous/Next Image Keyboard Navigation

**Date:** 2026-10-31  
**Wave:** Wave 2 — Horizontal Scroll (Sibling Images)  
**Workflow step:** 4.1a (frontend-specialist)  
**Scope boundary:** Keyboard-driven previous/next image navigation only. On-screen UI buttons are Task 3; pan-then-navigate is Wave 5.  
**Target implementer restriction:** Junior developer under 50% authority — no architectural or scope decisions are left to the implementer.

## 1. egui 0.37 input API analysis

### 1.1 Verified sources

- `eframe::App::ui` override shape in the landed code: `src/app.rs` lines 79–96.
  ```rust
  impl eframe::App for ImagesScrollerApp {
      fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
          egui::Panel::top("open_toolbar").show(ui, |ui| { … });
          if let Some(message) = self.message_text { … }
          egui::CentralPanel::default().show(ui, |panel_ui| { … });
      }
  }
  ```
- egui 0.37.0 `Context::input` API: read-only access to `InputState` via a closure.
  - Signature: `pub fn input<R>(&self, reader: impl FnOnce(&InputState) -> R) -> R`
  - Source: <https://docs.rs/egui/0.37.0/egui/struct.Context.html#method.input>
- `InputState::key_pressed` API:
  - Signature: `pub fn key_pressed(&self, desired_key: Key) -> bool`
  - Documentation: "Was the given key pressed this frame? **Includes key-repeat events.**"
  - Source: <https://docs.rs/egui/0.37.0/egui/struct.InputState.html#method.key_pressed>
- `InputState::key_down` API:
  - Signature: `pub fn key_down(&self, desired_key: Key) -> bool`
  - Documentation: "Is the given key currently held down? Keys released this frame are NOT considered down."
  - Source: <https://docs.rs/egui/0.37.0/egui/struct.InputState.html#method.key_down>
- `Key` enum variants for the arrow keys: `Key::ArrowLeft`, `Key::ArrowRight`.
  - Source: <https://docs.rs/egui/0.37.0/egui/enum.Key.html>

### 1.2 Pinned API call pattern

Inside the existing `ImagesScrollerApp::ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)` override, the implementation must read arrow-key presses exactly like this:

```rust
let left_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowLeft));
let right_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowRight));
```

Equivalently, because `Ui` dereferences to `Context`, `ui.input(|i| i.key_pressed(egui::Key::ArrowLeft))` is also valid; the pinned semantic is: read through the closure-based `input` API and use `key_pressed` for the logical arrow keys.

### 1.3 Release-repeat semantics (mandated)

- Use **`key_pressed`**, not `key_down`.
- Reason: `key_pressed` fires on the initial press and on OS key-repeat events, but it does **not** fire every frame while the key is held. `key_down` would return `true` on every frame during a hold, causing uncontrolled multi-step navigation per second.
- The Task 2 behavior is therefore a **single step per press/repeat event**, which matches the classic arrow-key navigation model and avoids frame-rate-dependent stepping.

## 2. Interaction contract

### 2.1 Key-to-direction mapping

| Key | Action |
|-----|--------|
| `ArrowLeft` | Move to the **previous** sibling image in the current folder's sorted listing. |
| `ArrowRight` | Move to the **next** sibling image in the current folder's sorted listing. |

- The sort order and file filter are exactly those decided in Task 1 (`src/folder_listing.rs`): alphanumeric, case-insensitive fold with exact-name tie-break, 17-entry extension whitelist.
- Navigation is **strictly within the current folder**; subfolders are never entered by the arrow keys.

### 2.2 Direct navigation (no pan interplay)

- At this wave, arrow keys **navigate directly** to the neighbor image.
- **No pan-then-navigate logic is implemented.** There is no zoom/pan state to consult because zoom and pan are not implemented until Wave 5.
- This direct behavior must remain correct even after later waves add pan logic: the Wave 5 implementation will gate navigation on pan-edge conditions, but that gating is out of scope here.

### 2.3 Behavior per state

| Situation | Required behavior |
|-----------|-------------------|
| No image is currently open (`self.opened_image_path` is `None`) | Arrow keys are a **no-op**. No navigation request is made; no message is shown. |
| Current image has no valid folder listing (`FolderListing::from_opened_image` returns `None`, or `listed_index_of` returns `None`) | Arrow keys are a **no-op**. The current image stays visible. |
| Neighbor resolves to a valid path (`ListedNeighbor::At`) | The implementation calls the existing `open_image(PathBuf)` seam with the neighbor's `image_path`. |
| Neighbor decode fails (e.g., unsupported/locked file) | Reuse `accept_open_outcome` semantics from `src/app.rs` lines 46–62: the **current image and path stay unchanged**, the bottom message bar shows `UNSUPPORTED_FILE_MESSAGE`, and the previous image remains visible behind the message. |
| At first/last list position (`ListedNeighbor::ListBeginningReached` / `ListEndReached`) | Behavior is **deliberately NOT decided in this spec**. Task 2's architector (step 4.1b) will choose between stay, wrap, or another policy. The spec records this as a pending decision. |

## 3. UI lifecycle placement

### 3.1 Where key intake happens

- Key intake must happen **inside `ImagesScrollerApp::ui`**, at the very beginning of the method, **before any panel is created**.
- Exact ordering:
  1. Read arrow-key input (`key_pressed` calls).
  2. If a navigation request is valid, call the shared navigation path (which ultimately calls `open_image`).
  3. Then paint the top `Open…` toolbar, the conditional bottom message bar, and the central image surface.

### 3.2 Every-frame requirement

- The input reads must run **every frame**, regardless of whether an image is loaded, whether the message bar is shown, or whether the central panel will paint.
- This is required because egui's `InputState` is reset each frame; missing a frame would miss a key event.

### 3.3 Ordering with the Open dialog and message bar

- The `Open…` button triggers a native dialog through `platform::show_open_image_dialog`. That dialog is modal and blocks the event loop; no `ui` frames run while it is open, so key intake cannot conflict with it.
- Key intake itself must not allocate or paint any widget. It only decides whether to invoke the navigation seam; the resulting state change (new path/message) is reflected naturally by the existing panel closures.

### 3.4 No visual state changes from key intake itself

- The arrow-key handling code must not directly mutate `message_text`, `decoded_image`, or `current_display` except through the existing `open_image` / `accept_open_outcome` seam.
- No new on-screen feedback (cursor changes, overlays, toasts) is added for key presses in Task 2.

## 4. Anti-pattern guardrails

| Anti-pattern | Required avoidance |
|--------------|--------------------|
| Polling loops | Do not write `while key_pressed { … }`, `loop` over events, or any per-frame busy-waiting. A single boolean read per key per frame is sufficient. |
| Manual key-state cache | Do not add fields like `was_left_down_last_frame`, `key_repeat_timer`, or `last_navigated_key`. egui's `InputState` already tracks press/repeat/release state. |
| Per-frame texture churn | The display/texture path must remain exactly the Wave 1 implementation: `current_display.accept_decoded_image()` flags a pending upload, and `paint` uploads once per successful decode. Keyboard navigation must not add any new per-frame texture or pixel operation. |
| Duplicate open logic | Navigation must reuse `ImagesScrollerApp::open_image(path)` or an equivalent helper that calls `accept_open_outcome`; do not inline decode/texture/state mutation. |
| `key_down` for stepping | `key_down` must not be used as the trigger for navigation. It is only appropriate for continuous hold behaviors (not in scope). |

## 5. Deferred scope

### 5.1 Wave 5 interplay deferred (pan-then-navigate)

- The brief's "Pan and Navigate at Image Edges While Zoomed" behavior is **explicitly deferred to Wave 5**.
- For Task 2, the arrow keys always navigate directly; there is no zoom/pan state and no edge-gating logic.
- This spec records the deferral so that the Wave 5 review can confirm no premature pan logic leaked into the keyboard handler.

### 5.2 Out-of-scope list

The following are NOT part of this front-end spec and must not be implemented in Task 2:

- **Up/Down arrow keys** — sibling-folder navigation (Wave 3 / vertical scroll).
- **On-screen left/right UI buttons** — Task 3 of this wave.
- **Zoom in/out, fit-width, fit-height** — Wave 4.
- **Fullscreen mode** — later wave.
- **Edge-of-list behavior** — stay vs. wrap; owned by Task 2's 4.1b architector.
- **In-session listing-cache state layout** — owned by Task 2's 4.1b architector.
- **Shared navigation handler signature** — owned by Task 2's 4.1b architector.
- **Changing the window title to the filename** — deferred.
- **Any accessibility/narrator announcement for navigation** — deferred.

## 6. Project-rule constraints that shape the GUI code

| Rule | How it applies to Task 2 |
|------|--------------------------|
| `max-lines-per-file` (≤200 lines; ideally ≤125 substantive) | Keyboard wiring in `app.rs` must be short; navigation helper logic should live in a dedicated small module or be folded into an existing seam helper. |
| `max-lines-per-method` (≤50 lines) | `ui` may grow slightly but must delegate navigation decisions to private helpers. |
| `max-depth` (≤2 nested blocks) | Extract any nested key-handling/edge logic into named helpers returning simple values. |
| `prefer-private-members` | Any new navigation state/helpers are private by default. |
| `self-documenting-code` | Names like `navigate_to_previous_image`, `navigate_to_next_image`, `current_listing`. |
| `single-section-boolean-conditions` | Any multi-section `if` guarding navigation is extracted into a named predicate. |
| `no-commented-code` | No commented-out code is left in source files. |

## 7. Static verification checklist for 4.5a

The following must be verifiable by reading the code only (no build/run):

- [ ] `ImagesScrollerApp::ui` reads `ArrowLeft`/`ArrowRight` using `ui.ctx().input(|i| i.key_pressed(egui::Key::Arrow...))` (or the equivalent `ui.input(...)` deref form).
- [ ] `key_pressed` is used; `key_down` is not used for navigation stepping.
- [ ] Key intake happens at the top of `ui`, before panel creation.
- [ ] No manual key-state cache fields are added to `ImagesScrollerApp`.
- [ ] Navigation invokes the existing `open_image` / `accept_open_outcome` seam.
- [ ] When no image is open or the current image has no listing index, arrow keys are a no-op.
- [ ] On neighbor decode failure, the current image and path remain unchanged and `UNSUPPORTED_FILE_MESSAGE` is shown.
- [ ] No pan/zoom logic is present in the key handler.
- [ ] The texture/display path remains the Wave 1 one-upload-per-decode implementation.
- [ ] No per-frame texture upload, pixel copy, or image-size allocation is added.
- [ ] Up/Down arrows, UI buttons, zoom, and fullscreen are absent from Task 2 code.
- [ ] File/method size, depth, private-by-default, and self-documenting rules are respected.

## 8. Acceptance criteria

1. With an image open and sibling images in the same folder, pressing `ArrowLeft` moves to the previous image and pressing `ArrowRight` moves to the next image in the Task 1 sort order.
2. With no image open, arrow keys do nothing.
3. If the target neighbor fails to decode, the previously displayed image stays visible and the bottom message bar shows the unsupported-file message.
4. Navigation uses the existing open/decode/state seam; no duplicate decode or texture logic is introduced.
5. The display/texture path uploads once per successful decode and does not churn per frame.
6. Container dev-checks and tests pass (evidence comes from the implementer/architector verification step).

## 9. Pending decisions for Task 2's 4.1b architector

The front-end layer is intentionally agnostic on the following; the architector must decide them in the implementation plan:

- Edge-of-list behavior (`ListBeginningReached` / `ListEndReached` → stay, wrap, or other).
- How the in-session folder listing is cached and rebuilt (state field shape, rebuild triggers, invalidation on folder change).
- The exact signature of the shared navigation helper that both keyboard and later UI buttons will call.
