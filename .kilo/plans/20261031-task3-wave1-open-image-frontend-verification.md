# Task 3 — Front-end Implementation Verification

- **Task:** Wave 1, Task 3 — Open Image Flow + In-Window Message Surface
- **Date:** 2026-10-31
- **Branch:** `feat/wave-1-skeleton-open-image`
- **Commit verified:** `0eacf61`
- **Spec:** `.kilo/plans/20261031-task3-wave1-open-image-frontend-spec.md`
- **Implementation plan mapping:** `.kilo/plans/20261031-task3-wave1-open-image-implementation.md` §5

## §8 static checklist results

| # | Checklist item | Verdict | Evidence |
|---|----------------|---------|----------|
| 1 | `ImagesScrollerApp` has the three private fields listed in §5.1 | PASS | `src/app.rs:13-21` (`opened_image_path`, `decoded_image`, `message_text`) |
| 2 | `new` initializes those fields to `None` | PASS | `src/app.rs:24-26` → `empty_state()` sets all three to `None` |
| 3 | Public `open_image(&mut self, path: PathBuf)` exists and is the only non-UI state mutator | PASS | `src/app.rs:37-40`; helper `accept_open_outcome` is private |
| 4 | `open_image` calls `image_loading::load_image` and contains no WIC/COM code | PASS | `src/app.rs:38` calls `image_loading::load_image`; no platform/WIC code in `app.rs` |
| 5 | `ui()` contains `egui::Panel::top("open_toolbar")` with button labeled exactly `"Open…"` | PASS | `src/app.rs:77` (`egui::Panel::top("open_toolbar")`); `src/app.rs:62` (`ui.button("Open…")`) |
| 6 | `ui()` conditionally shows `egui::Panel::bottom("message_bar")` only when `message_text` is `Some` | PASS | `src/app.rs:81-85` (`if let Some(message) = self.message_text`) |
| 7 | Bottom panel label uses exactly `image_loading::UNSUPPORTED_FILE_MESSAGE` | PASS | `src/app.rs:55` stores `Some(image_loading::UNSUPPORTED_FILE_MESSAGE)`; `src/app.rs:83` renders the stored message |
| 8 | No `egui::Window`, modal overlay, or OS message box for the unsupported message | PASS | No `Window`, `Area`, or `MessageBox` usage in `src/app.rs` or `src/platform/stub.rs` |
| 9 | No `TextureHandle`, `Image` widget, or other image-rendering code exists in Task 3 | PASS | `src/app.rs:87` CentralPanel empty; no texture code anywhere |
| 10 | No arrow-key, `Esc`, or `Enter` input handling exists in Task 3 | PASS | No input handling code in `src/app.rs` |
| 11 | Central empty state contains no hint label or placeholder | PASS | `src/app.rs:87` `CentralPanel` closure adds no widgets |
| 12 | `ui()` performs no file I/O, decode, or platform dialog calls | PASS (adapted) | The platform dialog call is inside the user-triggered button-click branch (`src/app.rs:62-64` → `open_via_dialog`), not on the per-frame paint path; `load_image` is never called from `ui()` (only from `open_image`) |
| 13 | File/method size, depth, private-by-default, and self-documenting rules respected | PASS | `src/app.rs` 150 lines (≤200); largest method bodies ≤50 lines; fields/helpers private; descriptive names used |

## §1/§2/§3/§5/§6 table verification

| Section | Verdict | Notes |
|---------|---------|-------|
| §1 Open affordance | PASS | Single top `Open…` button in horizontal layout; synchronous dialog → `open_image`; cancel maps to `None` and leaves state unchanged |
| §2 CLI intake UX | PASS | `src/main.rs:24-25` uses `std::env::args_os().nth(1).map(PathBuf::from)` verbatim; no dialog on CLI; no success UI; blank central panel |
| §3 Message surface | PASS | Bottom `message_bar` panel conditional on `Some`; verbatim `UNSUPPORTED_FILE_MESSAGE`; persists until next successful open; no OS popup; failure keeps prior image state |
| §5 State shape | PASS | Three private fields; `new` = empty state; single mutator `open_image`; decode via `image_loading::load_image`; no async/channels/polls; payload owned by app struct |
| §6 Paint contract | PASS | Order: top panel → conditional bottom panel → `CentralPanel` last; central panel empty; no modal surfaces; no per-frame decode/platform I/O |

## Sanctioned deviations / encoded adaptations read-through

The following deviations from the literal frontend-spec sketch are encoded in the implementation plan §5 and are accepted as the canonical Task 3 surface:

1. **A1 struck — panel type:** The code uses `egui::Panel::top(...)` / `egui::Panel::bottom(...)` (`src/app.rs:77`, `src/app.rs:82`), which is the correct egui 0.37 API. The implementation plan's earlier `TopBottomPanel` mention was superseded.
2. **A2 — dialog signature:** The platform facade is `platform::show_open_image_dialog(frame: &eframe::Frame) -> Option<PathBuf>` (`src/app.rs:69`, `src/platform/mod.rs:11`, `src/platform/stub.rs:9`, `src/platform/windows.rs:28`), satisfying spec §9's deferred HWND-owner plumbing.
3. **A3 — dialog call location:** The native dialog is invoked synchronously inside the `Open…` button click branch (`src/app.rs:62` → `open_via_dialog`). This is the spec's own §1.5/§6 sketch behavior and is not a per-frame paint-path call.
4. **Module names:** The platform modules are named `windows` and `stub` (`src/platform/mod.rs:5-8`), not `windows_impl`/`stub_impl`.
5. **Target-scoped `windows` dependency:** `Cargo.toml` declares `windows` for the Windows target path; the Linux container sees an empty crate, consistent with Task 2 bindings.
6. **Canonical `windows.rs` content deviations (fix3 + fix4):**
   - `WINCODEC_ERR_COMPONENTNOTFOUND` imported from `Win32::Foundation` (`src/platform/windows.rs:10`).
   - Owner extraction uses `frame.window_handle().ok()?` (`src/platform/windows.rs:51`).
   - Dialog path conversion uses `Error::from_hresult(windows::Win32::Foundation::E_FAIL)` (`src/platform/windows.rs:75`).
   - `classify_failure` takes `Error` by value (`src/platform/windows.rs:145`).
   - `CopyPixels` is called with the 3-argument form (`src/platform/windows.rs:135`).
   - Nested `unsafe {}` blocks used for local COM pointer scopes.

## Diffs: spec vs implementation

No front-end structural diffs were found beyond the sanctioned deviations listed above. Minor local details within implementer latitude:

- `src/image_loading.rs:11`, `:19`, `:26`, `:38` add `#[allow(dead_code)]` attributes on items that are dead on the Linux container target; these are clippy-hygiene choices and do not change semantics.
- `src/app.rs:109` accesses `APP_WINDOW_TITLE` via `super::APP_WINDOW_TITLE` in the inline test module, equivalent to the planned `ImagesScrollerApp::APP_WINDOW_TITLE` assertion.

Neither affects the front-end contract.

## Front-end quality issues

None identified. No texture/fitting/zoom code, no navigation key handling, and no sibling-list logic leaked in. The central panel remains intentionally blank for Task 4 to fill.

## Operator-run pending items (Task 5)

The following are recorded as operator-run manual checks and are not executed by this agent:

- CLI open of a supported JPG/PNG on Windows.
- In-app `Open…` dialog open, file selection, and cancel behavior on Windows.
- Unregistered-format file selection shows the verbatim `UNSUPPORTED_FILE_MESSAGE` in the bottom panel.
- Message bar remains visible while a previously loaded image is retained.

## Final verdict

**VERIFICATION PASSED (6 sanctioned adapted items).**
