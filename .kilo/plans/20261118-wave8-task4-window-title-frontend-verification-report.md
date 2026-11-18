# Wave 8 Task 4 — Window Title Path Feature: Front-end Implementation Verification Report (4.5a)

**Date:** 2026-11-18  
**Branch:** `feat/wave-8-wizard-installer`  
**HEAD:** `62a0ec7`  
**Verifier:** front-end sub-agent, static code review only (no `cargo`/docker run)  
**Inputs:**
- Spec: `.kilo/plans/20261118-wave8-task4-window-title-frontend-spec.md`
- Implementation plan: `.kilo/plans/20261118-wave8-task4-window-title-implementation-plan.md`
- TODO: `.agent/todos/20261117/20261117-todo-1.md` Task 4
- Code: `src/window_title.rs`, `src/window_title/segments.rs`, `src/window_title/tests.rs`, `src/app/window_title.rs`, `src/app.rs`, `src/app/central_surface.rs`, `src/app/fullscreen.rs`, `src/main.rs`, `.agent/project-structure.md`

---

## 1. Important planner adjudication (NOT a defect)

Implementation-plan D10 originally pinned popup text as "cached in state; STABLE for the full 3 s even if another navigation occurs." The Planner later adjudicated that clause **superseded**: the TODO's binding rule is that the popup re-arms on **every** navigation and shows the **currently** visible image's name+extension. The implementation rewrites `fullscreen_name_popup_text` in `arm_fullscreen_name_popup` on every re-arm, which matches the adjudicated behavior and is documented in `src/app/window_title.rs` lines 8–15. **This is correct and must not be flagged as a defect.**

---

## 2. Spec conformance checklist

| # | Checklist item (from spec §7 + plan D1–D16) | Status | Evidence |
|---|---------------------------------------------|--------|----------|
| 1 | `src/main.rs` launches with `APP_WINDOW_TITLE` and `with_maximized(true)` unchanged | ✅ | `src/main.rs:27–29`, only `mod window_title;` added |
| 2 | `ImagesScrollerApp` carries the five new fields, initialized in `empty_state()` | ✅ | Fields grouped in `WindowTitleState` (`src/app/window_title.rs:39–55`), initialized at `src/app.rs:69` |
| 3 | New pure module `src/window_title.rs` exists and is unit-tested | ✅ | Module exists; `src/window_title/tests.rs` has the 18-test matrix |
| 4 | Title update called once per frame; `ViewportCommand::Title` only sent when title changed | ✅ | Called at `src/app.rs:145`; change gate at `src/app/window_title.rs:70` and equality guard at `:99–103` |
| 5 | Empty-state title is `"Images Scroller"` | ✅ | `src/window_title.rs:36–38` returns `APP_WINDOW_TITLE` for `None` |
| 6 | Maximized title follows full-path → name → folder → drive → parent-walk algorithm | ✅ | `compose_window_title` maximized branch (`src/window_title.rs:47–62`) and tests 6–12 |
| 7 | Not-maximized title shows only file name+ext, truncated from the left | ✅ | `src/window_title.rs:44–46`; tests 2–5 |
| 8 | Maximize/restore transition re-triggers composition | ✅ | `maximized_changed` gate at `src/app/window_title.rs:69–70` |
| 9 | Text measurement uses egui `layout_no_wrap` + `Galley::size().x` | ✅ | `src/app/window_title.rs:142–153` uses `fonts_mut` + `layout_no_wrap` + `.size().x` (sanctioned D1 fallback) |
| 10 | Fullscreen popup uses `Area` anchored `CENTER_TOP`, `Order::Foreground`, `interactable(false)` | ✅ | `src/app/window_title.rs:159–175` |
| 11 | Popup arms for 3.0 s on every visible-image change while `fullscreen_on` | ✅ | `src/app/window_title.rs:83–85` and `:110–122` |
| 12 | Popup expiry uses `request_repaint_after` | ✅ | `refresh_fullscreen_name_popup_expiry` at `:124–135` |
| 13 | Toggling fullscreen OFF disarms popup | ✅ | `src/app/fullscreen.rs:35–37` |
| 14 | No per-frame texture re-upload/allocation churn introduced | ✅ | Feature touches only title string/popup state; `CurrentImageDisplay` unchanged |
| 15 | Test census matches expected delta (29 test-bearing files) | ✅ | 29 test-bearing files; 213 tests (see §6) |
| 16 | No architectural overstep (no new deps, no unrelated modules changed) | ✅ | Diff stat shows only title-related files + `app.rs` + `central_surface.rs` + `fullscreen.rs` + `main.rs` + structure map |
| 17 | D1: Option A pixel-accurate measurement, `fonts_mut` fallback recorded | ✅ | `measure_text_width` uses `fonts_mut` |
| 18 | D2: 0.85 usable-width fraction, outer-rect fallback shape | ✅ | `src/app/window_title.rs:71–72` |
| 19 | D3–D5: `\` separator, literal `...` ellipsis, shell-parseable display split | ✅ | Constants in `src/window_title.rs:15–19`; split in `src/window_title/segments.rs:44–50` |
| 20 | D6/D7: step order, Interpretation A (truncated base consumes budget) | ✅ | Tests 4, 5, 12; code order in `compose_window_title` |
| 21 | D8: maximized detection with `unwrap_or(false)` | ✅ | `src/app/window_title.rs:61–64` |
| 22 | D9: browsed-empty-folder title stays base | ✅ | Empty path → `APP_WINDOW_TITLE` |
| 23 | D10/D11: popup lifetime/wake/visuals mirror ESC-close precedent | ✅ | Constants and render match `esc_close_confirm.rs` palette |
| 24 | D12: shared `file_name_with_extension` | ✅ | Used for title, not-maximized rule, and popup |
| 25 | D13: exact call sites (after keystrokes, before layout; fullscreen render; OFF disarm) | ✅ | `src/app.rs:142–146`; `src/app/fullscreen.rs:56` and `:35–37` |
| 26 | D14: `app.rs` cap managed via relocated `show_navigation_layout` | ✅ | Method moved to `src/app/central_surface.rs:12–18`; `app.rs` remains under 200 lines |
| 27 | D15: module/test layout (no `app/window_title.rs` tests, child `tests.rs`) | ✅ | Only `src/window_title/tests.rs` added |
| 28 | D16: single-section boolean conditions | ✅ | `title_recomposition_is_due`, `fullscreen_name_popup_rearm_is_due`, `should_add_next_parent`, etc. |

**Overall spec conformance verdict: PASS.**

---

## 3. TODO algorithm end-to-end trace

The TODO's ladder is implemented in `src/window_title.rs::compose_window_title` and `src/window_title/segments.rs`.

1. **Full path → name+ext**: `path_segments` partitions the shell-parseable display path into `drive` / `folders` / `file_name`.
2. **Full path fits?** `composer.full_chain()` builds `drive + all folders + file_name`; if it fits the budget, it is returned verbatim (test 6).
3. **Not-maximized**: regardless of full-path fit, `!is_maximized` returns `fit_or_truncate_last(file_name)` (tests 2–5).
4. **Maximized, name too long**: `fit_or_truncate_last` keeps the last characters that fit, prefixing with `...`; if nothing fits, returns bare `...` (tests 4, 5, 12).
5. **Remaining-space priority**:
   - **Containing folder FIRST**: `containing_folder_fits` checks `candidate(1, false)` (test 7).
   - **Drive SECOND with `...`**: `drive_step_fits` checks `candidate(count, true)`; ellipsis inserted whenever drive is visible and folders are omitted (tests 8, 11).
   - **Parent walk-up**: `while should_add_next_parent` adds nearest parents first and stops at the first failure (tests 9, 10).
6. **Not-maximized name-only rule**: enforced by the early return before any chain logic (tests 2–5).
7. **Fullscreen popup**: `update_window_title_and_popups` detects `path_changed && fullscreen_on` and arms the popup with `file_name_with_extension(path)` for 3.0 s; rendered only in `show_fullscreen_layout`.

### 18-test matrix coverage vs TODO clauses

| TODO clause | Covered by test(s) | Notes |
|-------------|-------------------|-------|
| Title shows path of visible image / updates on navigation | Wiring only; not unit-testable without egui context | Manual/operator-run gate covers live update |
| Full path fits → show it | Test 6 | ✅ |
| Otherwise name+ext completely | Tests 2, 3, 7, 8, 9, 10, 11, 13, 14, 15 | ✅ |
| Name too long → last chars | Tests 4, 5, 12 | ✅ |
| Containing folder FIRST | Test 7 | ✅ |
| Drive SECOND with `...` | Tests 8, 11 | ✅ |
| Parent folders walk up while fit | Tests 9, 10 | ✅ |
| Not-maximized → name only | Tests 2–5 | ✅ |
| Fullscreen popup top-center 3 s | Constants tested (test 18); render/arm behavior is wiring-level | Manual/operator-run gate |

**Uncovered clause:** the pure-test matrix does not exercise the *navigation-triggered wiring* (same-frame title update after a keystroke, or the popup re-arm on navigation). This is consistent with the implementation plan D15, which pinned the wiring to manual verification and forbade adding a test module to `src/app/window_title.rs`. It is not a conformance failure, but it is a coverage gap that remains behind the operator-run gate.

---

## 4. Front-end quality issues

### 4.1 Title width is sampled only on path/maximize transitions

**Severity: Low (known limitation, contract-conformant).**

`available_width` is computed only inside `title_recomposition_is_due` (`src/app/window_title.rs:71–72`). A plain window resize while the maximized state is unchanged does **not** recompose the title. Because the TODO requires updates on navigation only, this matches the spec/plan. The practical impact is minimal on Windows: a maximized window cannot be freely resized; restoring it triggers a maximized-state change and recomposes anyway.

**Fix (optional):** If future UX wants live resize refitting, add a `width_changed` gate comparing the sampled outer width to a cached value. Not required by the current binding.

### 4.2 No automated integration tests for the wiring or popup render

**Severity: Low (by design per D15, but worth recording).**

The 18 tests cover pure composition only. The `update_window_title_and_popups` method, `ViewportCommand::Title` send, and `show_fullscreen_name_popup` render are exercised only by manual/operator-run testing. The plan explicitly left the wiring uncovered to keep the census at 29 files.

**Fix (optional):** Post-wave add an egui-context-less integration seam that records sent viewport commands, or add a snapshot test under `app/window_title_tests.rs` if the team later accepts a census change.

### 4.3 Accessibility of dynamic title changes and popup

**Severity: Low (out of current scope).**

The window title change is a platform-level property; the popup is a non-interactive `Area` with a `label`. Neither is explicitly announced to screen readers. egui's AccessKit path may surface the label automatically, but there is no deliberate accessibility contract.

**Fix (optional):** If accessibility becomes a requirement, add an AccessKit live-region announcement on title/popup change.

### 4.4 Popup `egui::Id::unique` is frame-stable but not content-keyed

**Severity: Low (matches precedent, no functional impact).**

`show_fullscreen_name_popup` uses `egui::Id::unique(FULLSCREEN_NAME_POPUP_AREA_ID)`, identical to `esc_close_confirm.rs`. Because the popup is non-interactive and anchored, this does not cause user-visible bugs.

**Fix:** None required; changing to `Id::new` would be a purely stylistic deviation from the precedent.

---

## 5. Regressions: Waves 1–7

No regressions detected in the static review:

- `app.rs::ui` ordering is **keystrokes → title update → layout**, exactly as pinned by the new comment at `src/app.rs:143–144`.
- `show_navigation_layout` was relocated verbatim to `src/app/central_surface.rs:12–18`; the call site in `show_chrome_layout` (`src/app.rs:132`) is unchanged.
- Fullscreen toggle still sends exactly one `ViewportCommand::Fullscreen` and disarms the ESC-close popup.
- The pill remains hidden while fullscreen is ON via `full_ui_chrome_is_visible`.
- Pan, zoom, fit, open-with menu/modal gating, and ESC-close popup paths are untouched.
- `Cargo.toml`/`Cargo.lock` show no changes (diff stat confirms only source/docs/structure-map files).

---

## 6. Census

| Metric | Expected (plan) | Actual | Verdict |
|--------|-----------------|--------|---------|
| Test-bearing files | 29 | 29 | ✅ |
| Unit tests | 212 (plan arithmetic) | **213** | ✅ — plan's "212" is an arithmetic slip; base 195 + 18 new = 213 |

**Count method:** `rg -l "^\s*#\[test\]" src` → 29 files; per-file `rg ... --count-matches` summed to 213 tests.

---

## 7. Minor implementation-vs-plan deltas (not defects)

| Delta | Plan reference | Implementation | Assessment |
|-------|----------------|----------------|------------|
| `compose_window_title` checks `!is_maximized` **before** the step-0 full-chain check | §4.1 pseudocode put step-0 first | `src/window_title.rs:44–46` returns name-only first | Correct per D7 heading "maximized = true only" and required by pinned tests 2/3/5 |
| `mod window_title;` placement | Plan text said "after `mod zoom;`" | `src/app.rs:165` and `src/main.rs:17` place it alphabetically **before** `mod zoom;` | No behavioral effect; ordering is alphabetically consistent |
| Width read shape | Plan sketched a single tuple closure | `src/app/window_title.rs:61–65` uses three separate `input` closures | Values sampled only at recomposition time; behavior identical |
| Test count | Plan §7 says 212 | Actual 213 | Plan's arithmetic slip (195 + 18 = 213); no code impact |
| `fonts_mut` + `layout_no_wrap` | Plan's primary API was `fonts(...).layout_no_wrap(...)` | Uses `fonts_mut(...).layout_no_wrap(...)` | Sanctioned D1 fallback; Context7 confirms `fonts_mut` provides `FontsView` and `Painter::layout_no_wrap` uses the same layout path |

---

## 8. Simplification-plan alignment

The current HEAD already incorporates the four simplifications from `.kilo/plans/20261118-wave8-task4-window-title-simplification-plan.md`:

- **S1:** `fit_or_truncate_last` no longer has the redundant `ellipsis_does_not_fit` pre-check (`src/window_title.rs:81–92`).
- **S2:** `ChainComposer` is constructed only on the maximized path (`src/window_title.rs:47`).
- **S3:** Popup expiry and wake are folded into one `refresh_fullscreen_name_popup_expiry` pass (`src/app/window_title.rs:124–135`).
- **S4:** UNC drive segment uses `UNC_PATH_MARKER` (`src/window_title/segments.rs:56`).

All simplifications are behavior-preserving and consistent with the pinned test matrix.

---

## 9. Conformance verdict

**PASS.** The Wave 8 Task 4 window-title implementation conforms to the 4.1a front-end technical specification and the 4.1b implementation plan. All pinned contracts (state fields, change-gated title sends, measurement API, composition algorithm, popup lifetime/render/disarm, call-site ordering, and census) are satisfied. The only identified items are low-severity UX/accessibility limitations or optional coverage gaps that are either by design or out of scope.

No source edits were made; no commit was created.
