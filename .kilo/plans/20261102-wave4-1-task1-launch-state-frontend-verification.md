# Wave 4.1 Task 1 — Launch Window State: Front-end Implementation Verification

Date: 2026-11-05
Branch: `fix/wave-4-operator-fixes`
HEAD commits reviewed: `e3873f8` (feature) + `93eb47d` (comment-only docs)
Spec: `.kilo/plans/20261102-wave4-1-task1-launch-state-frontend-spec.md`
Files reviewed: `src/app.rs`, `src/main.rs` (read-only), plus supporting modules `src/app/fullscreen.rs`, `src/app/fullscreen_tests.rs`, `src/app/keyboard_intake.rs`, `src/app_display.rs` for contract cross-checks.

## Checklist verdicts

### 1. Launch maximization

**CONFORMS.**

`src/app.rs` lines 144-152:

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

- `.with_maximized(true)` is present (line 149).
- `.with_title(APP_WINDOW_TITLE)` retained (line 147).
- `.with_resizable(true)` retained (line 148).
- No other `ViewportBuilder` launch-size/mode calls added.
- `NativeOptions` uses `..eframe::NativeOptions::default()` only — no persistence, no decoration, no fullscreen, no forced inner/min/max size changes.

Diff vs. spec: none — matches the spec's chosen mechanism verbatim.

### 2. Console hiding

**CONFORMS.**

`src/main.rs` lines 1-6:

```rust
//! Entry point of the Images Scroller viewer.

// Release runs windowed — no console.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
```

- The exact crate-level attribute `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` is present (line 4).
- Placed at the crate root, before all `mod` declarations and before `main()`.
- Factual comment present on line 3.
- No other crate-level attributes were moved or removed.

Diff vs. spec: none — placement and form match the recommended release-only `cfg_attr`.

### 3. Fullscreen contract unchanged

**CONFORMS.**

- `ImagesScrollerApp::fullscreen_on` initializes to `false` (`src/app.rs` line 59).
- No launch-time fullscreen call exists; `with_fullscreen` is not added to `window_options()`.
- Runtime toggle unchanged: `src/app/fullscreen.rs` line 20 sends `egui::ViewportCommand::Fullscreen(fullscreen_after_toggle)` exactly once per trigger.
- `keyboard_intake.rs` line 34-36 still dispatches F11 to the shared toggle action.
- `fullscreen_tests.rs` line 13 asserts `!app.fullscreen_on` on a fresh app.

The spec's egui 0.37 risk-table claims (ViewportBuilder::with_maximized exists, ViewportCommand::Fullscreen exists, launch fullscreen forbidden, no runtime mechanism change) are consistent with the landed code.

**Note for downstream Task 3:** `fullscreen.rs` module doc currently states F11 is "the only exit while ON". This factually matches the *landed* Task-1 code; it will need updating when ESC exit lands in Task 3, but that is outside this verification scope.

### 4. Module docs match landed launch behavior

**CONFORMS.**

- `src/app.rs` line 1 now reads: `//! Application shell: the resizable viewer window of Images Scroller, maximized at every launch.` (commit `93eb47d`).
- This accurately reflects the launch-maximized behavior and the retained `with_resizable(true)`.
- `src/main.rs` line 1 module doc is unchanged and still describes the entry point accurately.

### 5. Static quality: stale claims and dead code

**CONFORMS in the two target files.**

- `src/app.rs` and `src/main.rs` contain no stale "user placement", "as the user placed it", "opens normal", or "normal or maximized" wording.
- No dead or commented-out code observed in the two files.
- Supporting grep across `src/` found no `println!`, `eprintln!`, `print!`, `eprint!`, `log::`, or `tracing::` usage, confirming the console-hiding attribute does not discard user-visible diagnostics.
- `CurrentImageDisplay` upload discipline remains one upload per successful decode (`src/app_display.rs` lines 36-38, 67-80, tests lines 102-120) — no per-frame re-upload churn introduced.

**Side note:** Other files (`src/app/zoom_button.rs`, `src/app/fit_mode_button.rs`, `src/app/fullscreen_button.rs`) contain "user-approved placement" comments, but these are outside the two target files and refer to button positioning, not launch-window placement. They do not conflict with the Task-1 supersession.

## Diffs vs. spec

None. The implementation matches the front-end technical specification for Task 1.

## Quality notes

1. The attribute choice (`cfg_attr(not(debug_assertions), windows_subsystem = "windows")`) is the recommended release-only form, preserving a debug console on Windows for future diagnostics.
2. `window_options()` remains minimal and free of launch-time sizing beyond maximization.
3. Fullscreen default OFF is preserved; launch maximization and runtime fullscreen remain independent.
4. No state persistence machinery was added; behavior is session-only as required.
5. Texture upload discipline and DPI/scaling recomputation paths are untouched.

## Steps to fix

None required.

## Operator-run lane

The following behaviors cannot be agent-verified from the Linux container and remain **OPERATOR-RUN on Windows**:

- Window actually opens maximized on a real Windows 10/11 desktop.
- Restoring to non-maximized and then re-launching returns to maximized (session-only, no persistence).
- Round-trip behaviors: F11 from maximized → borderless fullscreen → F11 returns to maximized windowed state.
- Console/cmd window is not visible when the release `.exe` runs on Windows.

The static evidence (subsystem attribute in `src/main.rs`, `with_maximized(true)` in `src/app.rs`, unchanged fullscreen toggle, successful cross-build) supports these claims but does not replace a live Windows run.
