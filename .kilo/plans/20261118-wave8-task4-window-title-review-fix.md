# Wave 8 Task 4 — Window Title Review Fix Plan

## Issue 1: Fullscreen name-popup text is overwritten on every navigation

**Location:** `src/app/window_title.rs`, `fn arm_fullscreen_name_popup`

**Deviation:** D10 pins the popup text as "cached in state; STABLE for the full 3 s even if another navigation occurs". The current implementation unconditionally reassigns `fullscreen_name_popup_text` whenever `fullscreen_name_popup_rearm_is_due` is true, so rapid navigation while fullscreen is ON updates the visible text instead of keeping it stable for the current 3-second lifetime.

**Fix:** Only set the popup text when the popup is not already armed. The deadline and wake request are still refreshed on every navigation.

```rust
fn arm_fullscreen_name_popup(&mut self, screen_context: &egui::Context, now: f64) {
    let Some(path) = self.opened_image_path.as_ref() else {
        return;
    };
    if self.window_title_state.fullscreen_name_popup_expires_at.is_none() {
        self.window_title_state.fullscreen_name_popup_text =
            window_title::file_name_with_extension(path);
    }
    self.window_title_state.fullscreen_name_popup_expires_at =
        Some(now + FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS);
    screen_context.request_repaint_after(Duration::from_secs_f64(
        FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS
            + FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS,
    ));
}
```

**Rationale:** This preserves D10's stability guarantee while still rearming the deadline on each navigation. Because `expire_fullscreen_name_popup_if_due` runs earlier in the same frame, a popup that just expired will have `expires_at == None`, so the text correctly updates on the next arming event.

---

## Non-fix observations (explained deltas)

1. **D1 measurement substitution:** The implementation uses `ctx.fonts_mut(...).layout_no_wrap(...)`. This is the plan's sanctioned API fallback (D1: "substitute `Context::fonts_mut(...)` and/or the underlying `fonts.layout(...)`"), so it is conformant.
2. **Census count:** The test matrix is exactly the pinned 18 tests. The observed total is 213 passed, i.e. 195 + 18, not 212; this is the plan's own arithmetic slip (195 + 18 = 213). Test-bearing file delta is the single new `src/window_title/tests.rs`.
3. **Module declaration order:** `src/app.rs` declares `mod window_title;` before `mod zoom;` (and `src/main.rs` does likewise). The plan text said "after `mod zoom;`" but also required alphabetical order; the implementation's ordering is alphabetically correct and has no behavioral effect.
4. **Width read split:** `update_window_title_and_popups` reads `maximized` and `outer_rect` in two separate `input` closures instead of the plan's sketched tuple closure. The values are sampled only at recomposition time and the behavior is identical.
5. **Line-count targets:** `src/app.rs` lands at 197 lines (target 198), `src/app/central_surface.rs` at 52 (target 51), and `src/app/fullscreen.rs` at 65 (target 63). All remain well under the 200-line hard cap.
