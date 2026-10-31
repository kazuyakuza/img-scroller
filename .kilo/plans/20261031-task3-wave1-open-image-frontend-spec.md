# Task 3 — Front-end Technical Specification: Open Image Flow + In-Window Message Surface

**Date:** 2026-10-31  
**Wave:** Wave 1 — Project Skeleton and Open Image  
**Scope boundary:** Task 3 only (CLI/dialog open flow and the unsupported/unloadable message surface). Image texture rendering and fit-scaling are Task 4 and are NOT decided here.  
**Target implementer restriction:** Junior developer under 50% authority — no architectural or scope decisions are left to the implementer.

## 1. In-app open affordance

**Decision:** a single top-aligned `Open…` text button inside a non-resizable `egui::Panel::top`. A full menu bar is rejected because it adds unused chrome and visually reduces the central image area more than a single thin toolbar.

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 1.1 | Control type and label | The top toolbar contains exactly one `egui::Button` whose label is the exact string `Open…` (U+2026 horizontal ellipsis). |
| 1.2 | Toolbar placement | The button lives inside `egui::Panel::top("open_toolbar")` (not a `Window`, `Area`, or modal). |
| 1.3 | Toolbar sizing | The panel is non-resizable and uses default egui panel styling (no custom `Frame`, no forced height). |
| 1.4 | Toolbar layout | Button is laid out left-to-right via `ui.horizontal(|ui| { ... })`. |
| 1.5 | Click action | When the button is clicked, the app requests the native file dialog; if the user confirms a file selection, the resulting `PathBuf` is passed to `self.open_image(path)`. |
| 1.6 | Dialog cancel | If the dialog is cancelled, no app state changes and no message appears. |
| 1.7 | Persistent visibility | The top toolbar is always shown in Task 3 (there is no fullscreen logic in this task). |

## 2. CLI argument intake

**Decision:** `main` reads the first OS argument with `std::env::args_os().nth(1)`, converts it to `PathBuf`, constructs `ImagesScrollerApp`, calls `app.open_image(path)` once, then runs the event loop. No dialog is shown, no central hint is rendered, and no special success UI appears.

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 2.1 | Argument source | `main` uses `std::env::args_os().nth(1).map(PathBuf::from)` as the optional image path. |
| 2.2 | No dialog on CLI launch | When the first argument is present, the code path that opens the native dialog is not invoked before the window appears. |
| 2.3 | No extra UI on success | A successful CLI load does not add any splash, progress, or "loaded" label to the window; the central panel remains the image surface (blank in Task 3). |
| 2.4 | Empty-state behavior | When no argument is supplied, the window shows the top `Open…` toolbar and a blank central panel. |
| 2.5 | No central hint | The central empty state contains no label, icon, or placeholder text. |

## 3. Message surface

**Decision:** a persistent bottom `egui::Panel` that appears only while a decode-failure message is set. The panel displays the verbatim constant `image_loading::UNSUPPORTED_FILE_MESSAGE` as a wrapped default body-text label. It is not a modal, does not dim the image, and does not require dismissal.

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 3.1 | Location | The message appears inside `egui::Panel::bottom("message_bar")`. |
| 3.2 | Text source | The label text is exactly the value of `image_loading::UNSUPPORTED_FILE_MESSAGE` (`"This file's format is not supported: no Windows image codec is registered for it."`). |
| 3.3 | Typography | The message is rendered with the default egui body text style (no custom font, size, or color). |
| 3.4 | Conditional visibility | The bottom panel is drawn only when `ImagesScrollerApp::message_text` is `Some(...)`; otherwise it is absent. |
| 3.5 | Persistence | The message remains visible until the next successful `open_image` call clears it to `None`. It is not auto-timed or click-to-dismiss. |
| 3.6 | Non-blocking | The message panel does not capture input, does not create a modal overlay, and does not hide the existing loaded image (if any). |
| 3.7 | No OS popup | No `MessageBox`, `msgbox`, or other OS-level dialog is used for this message. |

## 4. Focus / keyboard

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 4.1 | No navigation keys | No arrow-key input handling exists in Task 3 code. |
| 4.2 | No Esc/Enter binding | The `Esc` and `Enter` keys have no Task-3 behavior. |
| 4.3 | Default focus | The `Open…` button relies on egui's default focus/click handling; no custom focus management is added. |

## 5. State integration seams

**Decision:** `ImagesScrollerApp` owns the open-flow state. Two new private fields are added: `message_text: Option<&'static str>` and `decoded_image: Option<DecodedImage>`. The existing `opened_image_path: Option<PathBuf>` becomes the path of the successfully decoded image. A public method `open_image(&mut self, path: PathBuf)` is the only non-UI mutator of this state.

The decoded-pixel payload is owned by a private struct `DecodedImage` in `src/image_loading.rs` with the shape:

```rust
pub(crate) struct DecodedImage {
    pub width: u32,
    pub height: u32,
    /// 4 bytes per pixel, RGBA order.
    pub pixels: Vec<u8>,
}
```

Task 4 will consume this struct; Task 3 only produces and stores it.

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 5.1 | App fields (private) | `ImagesScrollerApp` contains exactly these three open-flow fields: `opened_image_path: Option<PathBuf>`, `message_text: Option<&'static str>`, `decoded_image: Option<DecodedImage>`. |
| 5.2 | Constructor initialization | `new` initializes all three fields to `None`. |
| 5.3 | Single mutator | Only `open_image(&mut self, path: PathBuf)` changes `opened_image_path`, `message_text`, and `decoded_image` from outside the `ui` method. |
| 5.4 | Decode runs outside paint path | `open_image` calls `image_loading::load_image(&path)` synchronously; `ui()` never calls it. |
| 5.5 | Success transition | On `Ok(decoded)`: `opened_image_path = Some(path)`, `decoded_image = Some(decoded)`, `message_text = None`. |
| 5.6 | Failure transition | On `Err(_)` of any kind: `message_text = Some(image_loading::UNSUPPORTED_FILE_MESSAGE)`; `opened_image_path` and `decoded_image` are left unchanged so any previously loaded image remains visible. |
| 5.7 | No async/polling | There is no thread spawn, channel, or per-frame poll for decode completion. |
| 5.8 | Ownership | `DecodedImage` (and therefore its pixel `Vec`) is owned by the app struct, not by `static`, not by the platform module. |

## 6. Render / paint contract

Each frame paints the following surfaces in order:

1. `egui::Panel::top("open_toolbar")` containing the `Open…` button.
2. `egui::Panel::bottom("message_bar")` containing the message label, only if `message_text` is set.
3. `egui::CentralPanel` as the image surface; in Task 3 it adds no widgets.

Illustrative shape (the platform/dialog call is a placeholder; its exact API is decided by the architector):

```rust
fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    egui::Panel::top("open_toolbar").show(ui, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Open…").clicked() {
                if let Some(path) = platform::open_image_dialog() {
                    self.open_image(path);
                }
            }
        });
    });

    if let Some(message) = self.message_text {
        egui::Panel::bottom("message_bar").show(ui, |ui| {
            ui.label(message);
        });
    }

    egui::CentralPanel::default().show(ui, |_ui| {});
}
```

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 6.1 | Panel order | Top panel is added first, bottom panel second, and `CentralPanel` last. |
| 6.2 | Central panel empty | `CentralPanel` adds no labels, images, or placeholders in Task 3. |
| 6.3 | No modal surface | No `egui::Window`, `egui::Area`, or OS dialog hosts the message. |
| 6.4 | No per-frame decode | The `ui` method contains no file I/O, decode, or platform calls. |

## 7. Project-rule constraints that shape the GUI code

| Rule | How it applies to Task 3 |
|------|--------------------------|
| `max-lines-per-file` (≤200 lines; ideally ≤125 substantive) | `app.rs` stays small; new logic is delegated to the `image_loading`/`platform` modules decided by the architector. |
| `max-lines-per-method` (≤50 lines) | `new`, `ui`, and `open_image` bodies must each be ≤50 lines. |
| `max-depth` (≤2 nested blocks) | Avoid third-level nesting; extract any multi-step decode/transition into small private helpers if needed. |
| `prefer-private-members` | The three state fields and any helpers are private; only `new`, `ui`, `open_image`, and the existing `window_options`/`APP_WINDOW_TITLE` are public. |
| `self-documenting-code` | Use names like `message_text`, `decoded_image`, `open_image`. |
| `single-section-boolean-conditions` | Any `if` with more than one clause is extracted into a named helper returning `bool`. |
| `no-commented-code` | No commented-out code is left in source files. |

## 8. Static verification checklist for 4.5a

The following must be verifiable by reading the code only (no build/run):

- [ ] `ImagesScrollerApp` has the three private fields listed in §5.1.
- [ ] `new` initializes those fields to `None`.
- [ ] A public `open_image(&mut self, path: PathBuf)` method exists and is the only non-UI state mutator.
- [ ] `open_image` calls into `image_loading::load_image` and does not itself contain WIC/COM code.
- [ ] `ui()` contains `egui::Panel::top("open_toolbar")` with a button labeled exactly `"Open…"`.
- [ ] `ui()` conditionally shows `egui::Panel::bottom("message_bar")` only when `message_text` is `Some`.
- [ ] The bottom panel label uses exactly `image_loading::UNSUPPORTED_FILE_MESSAGE`.
- [ ] No `egui::Window`, modal overlay, or OS message box is used for the unsupported message.
- [ ] No `TextureHandle`, `Image` widget, or other image-rendering code exists in Task 3.
- [ ] No arrow-key, `Esc`, or `Enter` input handling exists in Task 3.
- [ ] The central empty state contains no hint label or placeholder.
- [ ] `ui()` performs no file I/O, decode, or platform dialog calls.
- [ ] File/method size, depth, private-by-default, and self-documenting rules are respected.

## 9. What this spec does NOT decide (deferred to Task 3 architector 4.1b or Task 4)

- WIC pixel-format conversion, COM object lifetimes, `CreateDecoderFromFilename` call sequence.
- Exact `windows` crate feature set beyond the binding architector decision.
- Signature and HWND-owner plumbing of the platform dialog function.
- Internal error type returned by `image_loading::load_image` (only that `Err` triggers the message).
- Whether the RGBA buffer is premultiplied or straight alpha (only that it is 4 bytes/pixel RGBA).
- Module/file placement of `image_loading` and the `platform` facade.
- egui texture upload, `TextureHandle`, fit-scaling, zoom modes, or navigation keys (Task 4 / later waves).

## 10. Acceptance criteria

1. With a supported image path as the first CLI argument, the app opens without a dialog, shows the top `Open…` toolbar, stores the decoded payload, and shows no message.
2. With an unsupported/unloadable path as the first CLI argument, the app opens without a dialog and shows the verbatim `UNSUPPORTED_FILE_MESSAGE` in the bottom panel (no OS popup).
3. With no CLI argument, the app opens with the top `Open…` toolbar and a blank central panel.
4. Clicking `Open…` opens the native file dialog; selecting a supported file loads it and clears any previous message; selecting an unsupported file shows the message.
5. The message panel is non-modal and does not hide a previously loaded image.
6. `ui()` performs no decode or I/O and introduces no per-frame allocation.
7. The code compiles and container dev-checks pass (evidence comes from the implementer/architector verification step).
