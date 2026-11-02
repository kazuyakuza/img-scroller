# Wave 3 Task 3 — 4.5a Front-end Implementation Verification

**Spec:** `.kilo/plans/20261101-wave3-task3-vertical-nav-frontend-spec.md`  
**Implementation range:** `36be11b..51c1b08` on `feat/wave-3-vertical-scroll`  
**Adjudicated behavior source:** `.kilo/plans/20261101-wave3-task3-vertical-nav-navigation-v2.md` §11  
**Date:** 2026-11-01

---

## 1. Spec §7 Conformance Checklist

| # | Spec item | Status | In-code evidence |
|---|-----------|--------|------------------|
| 1 | `ArrowUp`/`ArrowDown` read with `ui.ctx().input(\|i\| i.key_pressed(...))` | **CONFORMS** | `src/app.rs:143-144` |
| 2 | New keys evaluated before the layout branch (spec's `image_is_open()` branch maps to the renamed `navigation_context_is_active()` gate) | **CONFORMS** | Intake at `src/app.rs:141-156`; branch at `src/app.rs:157` |
| 3 | Up key routes through same `navigate` seam as up button | **CONFORMS** | `src/app.rs:151-152`; `ArrowControl::FolderUp` → `PreviousSiblingFolder` at `src/app/arrow_direction.rs:62` |
| 4 | Down key routes through same `navigate` seam as down button | **CONFORMS** | `src/app.rs:154-155`; `ArrowControl::FolderDown` → `NextSiblingFolder` at `src/app/arrow_direction.rs:63` |
| 5 | Left/Right key behavior unchanged | **CONFORMS** | `src/app.rs:145-150` still maps to `Previous`/`Next` |
| 6 | Pan interaction absent; arrows navigate directly | **CONFORMS** | No pan code added anywhere in range |
| 7 | Floating pill layout unchanged: same `Area` id, anchor, frame, four buttons, group gap, message line | **CONFORMS** | `src/app/floating_controls.rs:11-21`, `34-43`, `64-78`, `57-61` |
| 8 | ↑/↓ buttons remain in existing physical slots; no new widget/panel introduced | **CONFORMS** | `src/app/floating_controls.rs:80-87`; `empty_folder_view.rs` renders only in central surface |
| 9 | `FolderUp`/`FolderDown` return a concrete navigation direction instead of `None` | **CONFORMS** | `src/app/arrow_direction.rs:58-64` |
| 10 | Up button maps to "previous sibling folder"; down button maps to "next sibling folder" | **CONFORMS** | `src/app/arrow_direction.rs:62-63` |
| 11 | Enablement derived per-frame from vertical navigation seam, mirroring horizontal `neighbor_image_path` approach (adjudicated semantics: enabled for any adjacent folder, regardless of image content) | **CONFORMS** | `src/app/floating_controls.rs:24-29`; vertical seam returns adjacent folder path at `src/app/folder_navigation.rs:139-154` |
| 12 | Disabled folder arrows paint dimmed via existing `arrow_color` logic | **CONFORMS** | `src/app/floating_controls.rs:161-169` |
| 13 | Accessible labels for ↑/↓ are `"Previous sibling folder"` / `"Next sibling folder"` | **CONFORMS** | `src/app/arrow_direction.rs:48-54`; `WidgetInfo::labeled(Role::Button, ...)` preserved at `src/app/floating_controls.rs:124-130` |
| 14 | Message line remains the only status surface **above the button row** and unchanged in placement/formatting; adjudicated empty-folder mode adds a separate centered central-surface label | **SUPERSEDED-BY-ADJUDICATION** | Pill message line unchanged at `src/app/floating_controls.rs:57-61`; central label added at `src/app/empty_folder_view.rs:12-18`, called from `src/app.rs:103` |
| 15 | No per-frame texture re-upload or new allocation churn introduced | **CONFORMS** | `src/app_display.rs` untouched in range; empty mode skips paint via `decoded_image == None` at `src/app.rs:100-104` |
| 16 | No UI code directly mutates `opened_image_path`, `decoded_image`, `message_text`, or `last_viewed_images` | **CONFORMS** | UI code (`app.rs::ui`, `floating_controls.rs`) only calls `navigate`/`open_via_dialog`; mutations live in `folder_navigation.rs` seam and `accept_open_outcome` |
| 17 | New/modified source files respect project rules (≤200 lines, ≤50-line methods, ≤2-level nesting, private-by-default, self-documenting names, no commented-out code) | **CONFORMS** | All source files ≤200 lines; no method exceeds 50 body lines; nesting ≤2 levels; no commented-out code; no dead_code marker census regressions (see audit) |

**Counts:** 15 CONFORMS, 1 SUPERSEDED-BY-ADJUDICATION, 0 DIVERGES.

---

## 2. Adjudicated Empty-Folder Mode Verification

| Check | Status | Evidence |
|-------|--------|----------|
| 1. Keyboard intake per-frame, before layout branch; Left/Right unchanged; no pan interplay; vertical keys act in BOTH image-mode and empty-folder mode (gate does not swallow them) | **CONFORMS** | `src/app.rs:141-156` reads all four arrows before the `navigation_context_is_active()` branch at line 157; intake runs identically in both modes |
| 2. Pill: same Area/anchor/frame; message line placement unchanged; ↑/↓ enabled per folder adjacency per frame; ←/→ unchanged (dimmed in empty-folder mode naturally) | **CONFORMS** | Pill unchanged at `src/app/floating_controls.rs:34-43`, `57-61`, `80-87`; vertical enablement from folder adjacency at `src/app/folder_navigation.rs:139-154`; horizontal seam returns `None` in empty mode because `opened_image_path`/`current_folder_listing` are cleared at `src/app/folder_navigation.rs:106-109` |
| 3. Labels + accessible info: "Previous sibling folder"/"Next sibling folder"; WidgetInfo button roles preserved | **CONFORMS** | `src/app/arrow_direction.rs:52-53`; `widget_info` call preserved at `src/app/floating_controls.rs:124-130` |
| 4. Empty-folder central surface: centered "Empty folder" via `show_empty_folder_message`; NOT in the pill line; image paint skipped when `decoded` is `None`; no texture-lifecycle churn (`app_display.rs` untouched) | **CONFORMS** | `src/app/empty_folder_view.rs:8,12-18`; paint skip at `src/app.rs:100-104`; `git diff --name-only 36be11b..51c1b08` does not list `src/app_display.rs` |
| 5. Center Open button: shows only in fresh state (no image AND no context) | **CONFORMS** | Gate is `navigation_context_is_active()` at `src/app.rs:157`; `false` only when both `image_is_open()` and `browsed_folder_context` are `None`; center Open reached only in that branch at `src/app.rs:159-160` |
| 6. No visual drift beyond the adjudicated additions (no new widgets/panels), no font-glyph dependency | **CONFORMS** | Only new rendered element is the central-surface label; no new font/glyph imports or custom glyph rendering |

---

## 3. dead_code Marker / Census Audit

- `src/sibling_folder_listing.rs`: removed markers from `ListedFolderNeighbor` enum, `from_opened_image`, `previous_neighbor_of`, `next_neighbor_of`; kept on `listed_index` and `listed_folder_count`.
- `src/last_viewed_images.rs`: removed marker from `last_displayed_image`.
- `src/folder_listing.rs`: new `first_listed_image` has an immediate caller; no marker needed.
- New files (`folder_navigation.rs`, `empty_folder_view.rs`, `folder_navigation_tests.rs`, `folder_navigation_continuation_tests.rs`) contain no `#[allow(dead_code)]` markers.

Result: no census regressions; marker removals match the v2 plan §4 ledger.

---

## 4. Quality Issues (non-blocking)

| # | Issue | Location | Concrete fix |
|---|-------|----------|--------------|
| Q1 | Missing one-line doc comments on the two new state fields (`current_sibling_folders`, `browsed_folder_context`). The v2 plan §3.2 items 3-4 specified them. | `src/app.rs:25-26` | Add `/// Sibling folders under the opened image's folder (rebuilt on every successful open).` above `current_sibling_folders`, and `/// Folder being browsed without an opened image (empty-folder mode).` above `browsed_folder_context`. This adds 2 lines; file is currently 199 lines, still ≤200. |
| Q2 | `app.rs` finished at 199 lines vs the v2 plan ledger expectation of 197. Within the hard 200-line cap, but 2 lines over the planned target. | `src/app.rs` overall | No functional fix required. If planners want to hit the 197 target, apply the §3.2 contingency: drop the two new field doc lines (but Q1 recommends adding them) or remove blank lines around `mod empty_folder_view;`. |

No divergences from the binding spec/adjudicated behavior were found.

---

## 5. Verdict Summary

- **CONFORMS:** 15 checklist items
- **SUPERSEDED-BY-ADJUDICATION (implementation matches adjudicated behavior):** 1 item (§7.14 — message-line / central-surface status split)
- **DIVERGES:** 0 items
- **Quality issues:** 2 minor doc/line-count notes, no behavioral fixes required.
- **Recommendation:** Address Q1 field doc comments, then proceed to Task 4 dev checks / Task 5 docs.
