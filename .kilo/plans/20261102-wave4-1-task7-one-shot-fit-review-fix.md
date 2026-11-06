# Wave 4.1 Task 7 — One-Shot Fit Width/Height: Review Fix Plan

**Date:** 2026-11-06 (critical-workflow step 4.3 review findings)
**Branch:** `fix/wave-4-operator-fixes`
**Scope:** comment-only cleanup in files already touched by Task 7; no behavior change, no test change, no harness run required.

## Issues found in review of commit `ae4b0f3`

Three doc comments still refer to a "fit mode" state that no longer exists after Task 7 (`FitMode` enum deleted, no persistent mode anywhere). The code is correct; the comments are stale and misleading.

### 1. `src/app/central_surface.rs:29-30` — stale "session fit mode's base size"

Current:

```rust
    /// The single paint request: the decoded image at the session fit mode's
    /// base size, scaled by the zoom factor (inside `CurrentImageDisplay`).
```

Replace with:

```rust
    /// The single paint request: the decoded image at the whole-image fitted
    /// size, scaled by the session zoom factor (inside `CurrentImageDisplay`).
```

### 2. `src/app/fit_button.rs:30` — stale "fit-mode group"

Current:

```rust
/// Appends the fit-mode group to the pill row, separated by the standard
/// group gap after the zoom buttons (user-approved placement).
```

Replace with:

```rust
/// Appends the one-shot fit button group to the pill row, separated by the
/// standard group gap after the zoom buttons (user-approved placement).
```

### 3. `src/app/fit_button.rs:53-54` — stale "sets the fit mode"

Current:

```rust
    /// Paints the glyph and, when clicked while enabled, sets the fit mode
    /// (the app seam resets the factor and gives the feedback).
```

Replace with:

```rust
    /// Paints the glyph and, when clicked while enabled, applies the one-shot
    /// fit action (sets the zoom factor only; no message, no persistent mode).
```

## Verification

- `rg 'fit.mode|fit_mode' src` after the edit must return no matches (underscore and hyphen/space variants both gone).
- No `#[test]` count change (comment-only).
- No source-code behavior change.

## Git action

After applying the three edits:

1. `git add src/app/central_surface.rs src/app/fit_button.rs`
2. `git commit --amend --no-edit` (the fixes belong inside the existing `feat: fit keys and buttons set zoom one shot` commit; no new commit hash is required by the workflow).
3. Do NOT push.

## Out-of-scope guard

- No source-code logic changes.
- No new files, no test files, no docs/project-info/TODO/CHANGELOG/plan/structure-map edits beyond this plan file.
- No harness run needed.
