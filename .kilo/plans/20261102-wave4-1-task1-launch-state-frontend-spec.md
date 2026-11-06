# Wave 4.1 Task 1 — Launch Window State: Front-end Technical Specification

Date: 2026-11-05  
Source TODO: `.agent/todos/20261102/20261102-todo-1.md`  
Base: `fix/wave-4-operator-fixes` @ `53c2e60` (0.4.0 → 0.4.1)  
Spec file: `.kilo/plans/20261102-wave4-1-task1-launch-state-frontend-spec.md`

## 1. Binding behavior (source of truth)

- The application window opens **ALWAYS MAXIMIZED** at every launch.
- The window is **never auto-fullscreened** at launch.
- The session fullscreen flag (`fullscreen_on`) default stays **OFF**; the maximize change does not alter the fullscreen default.
- This is **session-only** behavior: no window-state persistence to disk is introduced by this task.
- The Wave-1 / Wave-4 wording "window opens normal or maximized exactly as the user placed it" is **SUPERSEDED** by the 2026-11-02 operator correction. `brief.md` / `product.md` updates are Task 9's duty; this spec notes the supersession but does not implement documentation.

## 2. Verified egui/eframe 0.37 API facts

### 2.1 Launch maximization

- `egui::ViewportBuilder::with_maximized(maximized: bool)` exists in egui 0.37.0 and is documented as: **"Request that the window is maximized upon creation. The default is `false`."**
- The struct field is `maximized: Option<bool>`.
- The current `src/app.rs` `window_options()` already chains `.with_maximized(false)`. The required change is flipping that literal to `true`.
- `egui::ViewportBuilder::with_fullscreen(fullscreen: bool)` exists but is for **launch-time fullscreen only** ("Sets whether the window should be put into fullscreen upon creation. The default is `None."`). It must **not** be added; launch fullscreen remains forbidden.
- `egui::ViewportCommand::Maximized(bool)` exists for runtime maximize/unmaximize, but it is **not** needed here because the launch state is set through `NativeOptions`.

### 2.2 Runtime fullscreen (unchanged contract)

- `egui::ViewportCommand::Fullscreen(bool)` is unchanged from the Wave-4 Task-1 spec: **"Turn borderless fullscreen on/off."**
- `ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen_on))` remains the single runtime fullscreen mechanism.
- Launch maximization and runtime fullscreen are independent: a maximized window can later enter borderless fullscreen via F11, and exiting fullscreen returns to the maximized windowed state.

### 2.3 Console hiding

- The Rust `#![windows_subsystem = "windows"]` crate attribute sets the Windows PE subsystem at link time (Rust Reference, "The `windows_subsystem` attribute").
- Accepted values are `"console"` (default) and `"windows"`.
- The attribute is **ignored on non-Windows targets and non-`bin` crate types**.
- For this project (Windows-only `bin`, cross-compiled from Linux), it has no effect inside the Linux container; it only affects the produced `.exe` when run on Windows.
- It does not change the glow/eframe build logic, dependency set, or DLL import audit.

## 3. Chosen mechanisms

### 3.1 Launch-maximized

Change the single existing builder call in `src/app.rs`:

```rust
pub fn window_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_WINDOW_TITLE)
            .with_resizable(true)
            .with_maximized(true),
        ..eframe::NativeOptions::default()
    }
}
```

No new `NativeOptions` fields are introduced. `with_inner_size`, `with_min_inner_size`, `with_max_inner_size`, `with_fullscreen`, `with_decorations(false)`, or any other launch-time sizing/mode option must **not** be added.

### 3.2 Console hiding

Add at the top of `src/main.rs`, before any other crate-level attributes or `mod` declarations:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

**Recommendation: release-only `cfg_attr`.**

Tradeoffs:

| Form | Dev (`cargo run` / debug) | Release `.exe` | Rationale |
|---|---|---|---|
| `#![windows_subsystem = "windows"]` | No console on Windows | No console on Windows | Simpler, but hides `eprintln!`/`println!` even during local Windows dev runs; not needed because the app has no stdout/stderr output (verified below). |
| `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` | Console present on Windows | No console on Windows | Keeps a console in debug builds for any future printf-style debugging on Windows; still satisfies the operator requirement for the shipped release `.exe`. |

**Recommended form: `cfg_attr(not(debug_assertions), windows_subsystem = "windows")`.** It satisfies the operator requirement with the smallest behavioral change and leaves the debug path open for future diagnostics.

### 3.3 Verified stdout/stderr usage

A grep of `src/` found **no** uses of `println!`, `eprintln!`, `print!`, `eprint!`, `log::`, or `tracing::`. Therefore the console-hiding attribute will not discard any user-visible diagnostic output on Windows release builds. (The container harness and `log::` dependency are unaffected.)

## 4. State model

No new app state fields are introduced. The relevant state is:

- `ImagesScrollerApp.fullscreen_on: bool` — default `false` (unchanged). Launch maximization does not change this flag.
- Launch window state is expressed only through `NativeOptions::viewport` and is therefore not part of `ImagesScrollerApp` RAM state.

## 5. Interplay checks

### 5.1 Fullscreen ON from a maximized launch

- Pressing F11 while the window is maximized sends `ViewportCommand::Fullscreen(true)`.
- The window transitions from maximized windowed mode to borderless fullscreen on the current monitor.
- Exiting fullscreen (F11 / ESC after Task 3) sends `ViewportCommand::Fullscreen(false)`.
- The expected post-exit state is the previous maximized windowed state. The exact restore size is winit/Windows-defined; any deviation is an operator-run defect, not a spec deviation.

### 5.2 Closing and restoring

- Whether Windows remembers the maximized state across process restarts is OS-defined and **not** implemented by this task.
- This task is session-only: every fresh process launches maximized regardless of how the previous session ended.

### 5.3 DPI / scale factor

- Launch maximization is handled by winit/Windows; egui's DPI scaling continues to apply automatically.
- The image fit-scaling pipeline (`scaling::fitted_size` / `FitMode::base_size`) already recomputes per frame from `panel_ui.available_size()`, so DPI changes do not require extra code.

### 5.4 No other `NativeOptions` change

- `with_resizable(true)` stays.
- `with_title(APP_WINDOW_TITLE)` stays.
- No `with_inner_size`, `with_min_inner_size`, `with_max_inner_size`, `with_fullscreen`, `with_decorations(false)`, `with_position`, `with_active`, or `with_visible` additions.
- `NativeOptions::persist_window` stays at its default (`true` if the persistence feature is enabled, which it is not in this project). This task does not add persistence machinery.

## 6. 4.5a conformance checklist

For later front-end verification against this spec:

1. `src/app.rs` `window_options()` contains `.with_maximized(true)` and no other launch-size/mode changes.
2. `window_options()` still calls `.with_resizable(true)` and `.with_title(APP_WINDOW_TITLE)`.
3. `window_options()` does **not** add `with_fullscreen`, `with_decorations(false)`, forced inner size, or position.
4. `ImagesScrollerApp::fullscreen_on` still initializes to `false` and is never persisted.
5. The F11 fullscreen toggle still sends `egui::ViewportCommand::Fullscreen(bool)` exactly on the toggle frame.
6. The runtime fullscreen mechanism is unchanged; fullscreen ON and OFF still work from a maximized launch.
7. `src/main.rs` contains the crate-level console-subsystem attribute in the recommended form.
8. The console-subsystem attribute is placed at the crate root and does not interfere with module declarations or `main()`.
9. The app has no `println!` / `eprintln!` / `log::` / `tracing::` output that would be lost by hiding the console.
10. No per-frame texture re-upload churn: `CurrentImageDisplay` upload discipline remains one upload per successful decode.

## 7. Explicitly out of scope

- Task 2: button hotkey references and hover tooltips.
- Task 3: F11 immediacy tuning and ESC fullscreen exit.
- Task 4: stray zoom message removal.
- Task 5: symmetric zoom-out range.
- Task 6: strict-shift zoom alias correction.
- Task 7: one-shot fit width/height semantics.
- Task 8: verification harness execution.
- Task 9: documentation updates (`brief.md`, `product.md`, etc.).
- Parked future ideas (drag & drop, fullscreen bottom-hover reveal, normal-mode auto-hide chrome, Wave 5 pan-then-navigate, per-folder zoom memory).

## 8. Operator-run verification note

- The "window opens maximized" fact can only be fully verified by running the produced `.exe` on a real Windows 10/11 machine.
- The "console is hidden" fact is likewise operator-run: agent-side evidence is limited to the compiled subsystem attribute in `src/main.rs` plus a successful cross-build exit.

## 9. Risks / open notes for the architector

- **Risk: low** — flipping `with_maximized(false)` to `true` is a one-literal change with no runtime state interactions.
- **Risk: low** — `cfg_attr(not(debug_assertions), windows_subsystem = "windows")` is ignored on non-Windows targets, so container builds/tests are unaffected.
- **Note** — If a future task adds stdout/stderr diagnostics on Windows, the release-only subsystem attribute means those diagnostics will not be visible to a Windows release user unless a console is explicitly allocated.
