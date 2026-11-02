# Wave 3 — Task 5, Step 4.1b: Documentation Analysis & Plan (Vertical-Scroll Wave Closing)

Plan id: `20261101-wave3-task5-documentation` · Author: architector (4.1b) · Environment snapshot read: 2026-11-02
Branch at analysis time: `feat/wave-3-vertical-scroll` (work tree clean; last commit `6d243fb`). Implementation is junior-executable in the 4.2 step; this file is the ONLY artifact of 4.1b.

---

## 0. Binding task recap (source: `.agent/todos/20261029/20261029-todo-5.md` § Task 5 + wave-closing rules)

- `README.md`: vertical scroll + the corrected target rule (NO same-filename rule — binding Addendum v2 item 3). The empty-folder behavior (user-adjudicated D2-final) must be described AS-SHIPPED, wording matching landed code exactly.
- `tech.md` / `architecture.md`: record the session-memory component, the corrected navigation rule, remaining open items, test-census refresh (stale "26" → actual 50 with file-split accounting; tech.md `## Testing` rewrite), Status lines → Wave 3 landed at 0.3.0 (dev-checks `ALL CHECKS PASSED`; exe rebuilt `dist/windows-x64/img-scroller.exe` = 19,502,691 bytes — verified present on disk 2026-11-02; DLL import audit UNCHANGED, 22 OBSERVED DLLs restated WITHOUT duplicating the list — cross-reference only, docs-split rule).
- `context.md`: closing update (recent changes, current state, immediate next steps = Wave-3 OPERATOR-RUN checklist user-side + Wave-4 pointer).
- `.agent/project-structure.md`: add the missing `neighbor_tests.rs` entry (flagged by the Task-4 docs check) and ALL files the wave added.
- `docs/build.md` + `docs/build-windows.md`: verified NO change needed (see § 2).
- Wave rules honored: do NOT touch `CHANGELOG.md`; do NOT edit existing `.kilo/plans/*`; the TODO file is marked by 4.6 (after review), NOT by 4.2.

Landed-code facts verified this session (source of every claim used below):

| Fact | Source (read in full at 4.1b) |
| --- | --- |
| Sibling-folder enumeration: one non-recursive read of the current folder's PARENT; only immediate child directories (via `DirEntry::file_type`, which leaves symlinks unlisted); current folder excluded; unreadable scope parent → empty listing; unanchorable → `None`; ±1 with `ListBeginningReached`/`ListEndReached`, edges → `None` | `src/sibling_folder_listing.rs` (doc comments + impl) |
| Fixed v1 collation now shared via `src/name_collation.rs` (`compare_entry_names`: lowercase form first, exact-character tie-break), used by BOTH listings | `src/name_collation.rs`, `src/folder_listing.rs` |
| Keyboard: `ArrowUp` → `PreviousSiblingFolder`, `ArrowDown` → `NextSiblingFolder`, evaluated per frame in `ui()`; `NavigationDirection` has four arms | `src/app.rs` (lines 140–162), `src/app/folder_navigation.rs` |
| Pill: ALL FOUR arrows navigate (`ArrowControl::navigation_direction` returns a direction for FolderUp/FolderDown); disabled arrows paint dimmed and stay inert; accessible labels `Previous sibling folder` / `Next sibling folder`; per-frame enablement = `neighbor_image_path(direction).is_some()` | `src/app/arrow_direction.rs`, `src/app/floating_controls.rs` |
| Target priority (corrected rule): visited this session → recorded last-viewed image; otherwise ALWAYS the folder's FIRST whitelisted image (first-listed probe via `FolderListing::first_listed_image`); NO same-filename attempt, NO other fallback; image-less folder → `None` → empty-folder mode; target opens through the NORMAL `open_image` flow (failed decode keeps previous image + unsupported message — e.g. vanished last-viewed path) | `src/app/folder_navigation.rs` (`sibling_folder_target_image_path`, `enter_sibling_folder`), `src/folder_listing.rs` (`first_listed_image`) |
| D2-final empty-folder mode: image-less sibling folder is ENTERED, not skipped — open path/payload/horizontal listing/message released, `browsed_folder_context = Some(folder)`, NO last-viewed entry, vertical listing re-anchors ON the browsed folder (placeholder anchor `listing_anchor.bin`); central surface shows centered literal `Empty folder` (`EMPTY_FOLDER_MESSAGE`, `src/app/empty_folder_view.rs`) NOT in the pill's line; pill renders (`navigation_context_is_active()` = image open OR browsed context); mode ends at the next successful open (browsed context cleared); vanished browsed folder → empty vertical listing (both folder arrows disable, no message) | `src/app/folder_navigation.rs` (`enter_empty_folder_mode`, `navigation_context_is_active`), `src/app/empty_folder_view.rs`, `src/app.rs` (`show_navigation_layout`), tests `folder_navigation_tests.rs` + `folder_navigation_continuation_tests.rs` |
| D5 canonicalization: single normalization point in `open_image` (`fs::canonicalize(&path)` with raw-path fallback before `accept_open_outcome`); last-viewed KEY = resolved open path's parent (plain `parent()`, never re-canonicalized — per `src/last_viewed_images.rs` doc comment), and the vertical listing's folders derive from the same canonicalized resolution → same-folder visits hit the same map entry | `src/app.rs` (`open_image`), `src/last_viewed_images.rs` doc comment |
| Session memory: `LastViewedImages` — RAM-only parent→image map, created empty with the app, updated at EVERY open-success (`record_displayed_image` in the `Ok` arm), destroyed with the app struct, no persistence | `src/last_viewed_images.rs`, `src/app.rs` (`accept_open_outcome` Ok arm also rebuilds the vertical listing and clears `browsed_folder_context`) |
| Test census = 50 in 12 files (file-split accounting): `src/sibling_folder_listing/tests.rs` 6 + `src/sibling_folder_listing/neighbor_tests.rs` 2 + `src/name_collation.rs` 2 + `src/folder_listing/tests.rs` 5 + `src/app/tests.rs` 9 + `src/app/floating_controls_tests.rs` 1 + `src/app/folder_navigation_tests.rs` 5 + `src/app/folder_navigation_continuation_tests.rs` 4 + `src/app/last_viewed_tests.rs` 5 + `src/image_loading.rs` 4 + `src/app_display.rs` 2 + `src/scaling.rs` 5 | `rg -c "#\[test\]" src -g "*.rs"` run at 4.1b (sum = 50) |
| Version 0.3.0; branch `feat/wave-3-vertical-scroll`; wave commits `253d4db`…`6d243fb` (dates 2026-11-01/02, chronological: `253d4db` global plan, `89e1db4` version bump, `bc7261a`+`c2dcfba`+`e2845c1`+`4864c3c` Task 1, `1428c4b`+`bc4456d`+`36be11b` Task 2, `5d503a2`+`7657f32`+`51c1b08`+`9c2c94a` Task 3, `bde6b7f`+`7f2afd9`+`4b79e5c`+`05f8a2e`+`6d243fb` fmt/review-fixes/test-split + Task 4 mark) | `git log --format="%h %cs %s" -25` at 4.1b; `Cargo.toml` = `version = "0.3.0"` |
| Wave-4 pointer file: `.agent/todos/20261029/20261029-todo-6.md` ("Wave 4 — Fullscreen Mode and Zoom (Fit Width/Height)") | directory listing of `.agent/todos/20261029/` |

---

## 1. Verified NO-CHANGE files

- `docs/build.md`: contains no version numbers, no test counts, no wave-status text, no DLL list. Nothing in Task 5's scope lands there.
- `docs/build-windows.md`: same; its "Audited 2026-10-31 …" line is dated history that remains true (the audit was first run then; Wave 3 did not re-run it because the 22-DLL constant is unchanged). The OBSERVED 22-DLL list stays ONLY here + in `scripts/build-windows.sh`; every other doc below CITES it, never copies it.
- Instruction to 4.2: do not open/change either file; if any edit urges (it must not), STOP and return a question to the caller.

---

## 2. Per-file edit plan (exact BEFORE→AFTER anchors)

Budget rule of this plan: every change is a small local rewrite — no reordering of sections, no restructure, except the two Contents-list inserts (tech.md) and appends explicitly marked. Each change rationale ≤ 2 lines.

### 2.1 `README.md` (5 changes; README has no Contents/nav-section block — vertical scroll lives in the `## v1 Features` list and `## Project Status` paragraph; edits follow the existing list-prose style)

**R-1** · Feature 2 bullet, final clause (line 21 — pill appearance sentence).
- BEFORE: `…The floating bottom pill appears while an image is open; its ←/→/↑/↓ arrows are drawn as painted vector triangles (never font glyphs — they render identically on any Windows system regardless of installed fonts), and its up/down folder arrows are visible but disabled until folder navigation lands.`
- AFTER: `…The floating bottom pill appears while an image is open or while an empty sibling folder is being browsed; its ←/→/↑/↓ arrows are drawn as painted vector triangles (never font glyphs — they render identically on any Windows system regardless of installed fonts), and the up/down folder arrows navigate to the previous/next sibling folder under the same parent (each arrow is enabled exactly when its direction resolves a destination).`
- Why: removes the stale "disabled until folder navigation lands" fact (arrows are enabled as-shipped) and the pill-visibility fact now covers empty-folder mode.

**R-2** · Feature 3 bullet (line 22 — the corrected rule is ALREADY correct here; append the as-shipped empty-folder sentence at the bullet's end).
- BEFORE: `…the app also remembers the last viewed image per visited folder for the current session (RAM-only, destroyed on app close — no persistence to disk in v1).`
- AFTER: `…the app also remembers the last viewed image per visited folder for the current session (RAM-only, destroyed on app close — no persistence to disk in v1). An image-less sibling folder is NOT skipped: navigation enters it in an empty-folder browsing state — the open image is released, the blank viewer surface shows the centered "Empty folder" message, and the up/down controls keep moving between sibling folders from there (no wrap-around; at the first/last sibling folder the viewer stays).`
- Why: D2-final described as-shipped; message wording `"Empty folder"` matches the literal in `src/app/empty_folder_view.rs`.

**R-3** · `## Technology Stack` intro (line 35).
- BEFORE: `The choices below are confirmed and implemented for the Wave-1 open-image slice and the Wave-2 horizontal-navigation slice; the remaining v1 features (vertical navigation, fullscreen, zoom, "Open with...") are still open — see [Project Status](#project-status)…`
- AFTER: `The choices below are confirmed and implemented for the Wave-1 open-image slice, the Wave-2 horizontal-navigation slice, and the Wave-3 vertical-navigation slice; the remaining v1 features (fullscreen, zoom, "Open with...") are still open — see [Project Status](#project-status)…`
- Why: vertical navigation left the "remaining open" list.

**R-4** · Technology Stack table, Language row (line 39).
- BEFORE: `| Language | Rust | Implemented: Cargo package `img-scroller` 0.2.1, edition 2024. |`
- AFTER: `| Language | Rust | Implemented: Cargo package `img-scroller` 0.3.0, edition 2024. |`
- Why: version fact.

**R-5** · `## Project Status` paragraph (line 55) — two edits inside one rewrite:
- R-5a BEFORE: `…one centered floating bottom pill (translucent dark frame, painted vector-triangle arrows — ←/→ navigate, ↑/↓ reserved and disabled) replaces the Wave-2 corner rails…`
  AFTER: `…one centered floating bottom pill (translucent dark frame, painted vector-triangle arrows — ←/→ navigate) replaces the Wave-2 corner rails…`
  (The "↑/↓ reserved and disabled" clause is deleted: it states the pill's CURRENT arrow state, which Wave 3 changed; the pill's arrow state is now stated in R-5b.)
- R-5b BEFORE: `…the operator re-test of the rebuilt exe is pending. Verification so far: container dev checks (`ALL CHECKS PASSED`, 26 unit tests), Windows x64 release cross-build (exit 0), import audit; running the `.exe` on Windows is operator-run (the Wave-1 build passed the operator's manual check). The remaining v1 features (vertical navigation, fullscreen, zoom, "Open with...") are open.`
  AFTER: `…the operator re-test of that rebuilt exe is pending. Wave 3 landed (branch `feat/wave-3-vertical-scroll`, version 0.3.0): vertical sibling-folder navigation — the Up/Down arrow keys and the pill's ↑/↓ buttons move to the previous/next sibling folder under the same parent; a folder visited this session reopens its last-viewed image and a new folder opens its first image in alphanumeric order (NO same-filename attempt, no wrap-around); an image-less sibling folder is entered in an empty-folder browsing state (the blank viewer surface shows "Empty folder", the pill stays, and the folder arrows keep working from it); the per-folder last-viewed memory is RAM-only for the current session (destroyed on app close; no persistence in v1). Verification so far: container dev checks (`ALL CHECKS PASSED`, 50 unit tests), Windows x64 release cross-build (exit 0), import audit; running the `.exe` — including up/down folder movement, last-viewed reuse, and the empty-folder state — is operator-run on a real Windows machine (the Wave-1 build passed the operator's manual check; the 0.3.0 run is pending). The remaining v1 features (fullscreen, zoom, "Open with...") are open.`
- Why: wave-status refresh to 0.3.0/50 tests; README keeps its no-byte-sizes convention → the exe byte size is stated only in tech.md/context.md (docs-split).
- Note: the dated history fragments inside this paragraph (`Wave 2 landed (…, 0.2.0)`, `A 0.2.1 UI fix (2026-11-01…)`) stay untouched.

### 2.2 `.agent/project-info/tech.md` (8 changes; file has a `## Contents` — one new section slotted)

**T-1** · `## Contents` — insert item 6 after item 5, renumber the rest (6→7 … 13→14):
- BEFORE (items 5–13): `5. [Horizontal Navigation Mechanism (Implemented, Wave 2)]…` … `13. [References](#references)`
- AFTER:
  ```
  5. [Horizontal Navigation Mechanism (Implemented, Wave 2)](#horizontal-navigation-mechanism-implemented-wave-2)
  6. [Vertical Navigation and Session Last-Viewed Memory (Implemented, Wave 3)](#vertical-navigation-and-session-last-viewed-memory-implemented-wave-3)
  7. [Development Environment and Setup]… (renumbered)
  8. [Build Environment (Container on the Alpine VM)]… (renumbered)
  9. [Build and Run]… (renumbered)
  10. [Testing](#testing) (renumbered)
  11. [Technical Constraints]… (renumbered)
  12. [Distribution and Packaging (Planned)]… (renumbered)
  13. [Tool Usage Patterns for AI Agents]… (renumbered)
  14. [References](#references) (renumbered)
  ```
- Design decision (minimal): ONE combined section for both the vertical navigation mechanism and the session memory — they are the same landed slice and the memory's only reader is the vertical priority chain.

**T-2** · `## Status` paragraph — append the Wave-3 sentence after the Wave-2-UI-follow-up sentence, and refresh the verification/rest-of-scope sentence.
- BEFORE: `…the 0.2.0 corner rails and the Wave-1 top Open… toolbar were removed. Every landed slice is verified through the container harness (dev checks: `ALL CHECKS PASSED`, 26 unit tests; Windows x64 release cross-build: exit 0 → `dist/windows-x64/img-scroller.exe`). The remaining v1 features (vertical navigation, fullscreen, zoom, "Open with...", packaging) are planned. Facts are sourced from…`
- AFTER: `…the 0.2.0 corner rails and the Wave-1 top Open… toolbar were removed. Landed for Wave 3 (branch `feat/wave-3-vertical-scroll`, version 0.3.0, 2026-11-01/02): vertical sibling-folder navigation — the sibling-folder listing module, the Up/Down keys and pill buttons, the vertical target-image priority, the empty-folder browsing mode, and the session last-viewed memory (mechanism: `## Vertical Navigation and Session Last-Viewed Memory (Implemented, Wave 3)` below). Every landed slice is verified through the container harness (dev checks: `ALL CHECKS PASSED`, 50 unit tests; Windows x64 release cross-build: exit 0 → `dist/windows-x64/img-scroller.exe` = 19,502,691 bytes at 0.3.0; the DLL import audit is UNCHANGED — the OBSERVED 22-DLL list stays recorded ONLY in `scripts/build-windows.sh` + [docs/build-windows.md](../../docs/build-windows.md), deliberately not duplicated here). The remaining v1 features (fullscreen, zoom, "Open with...", packaging) are planned. Facts are sourced from…`
- Why: Status → Wave 3 landed at 0.3.0 with the audit cross-reference (docs-split rule).

**T-3** · `## Technology Stack` table, Language row (line 27).
- BEFORE: `Cargo package `img-scroller` version 0.2.1 (single binary; tracked `Cargo.lock`).`
- AFTER: `Cargo package `img-scroller` version 0.3.0 (single binary; tracked `Cargo.lock`).`

**T-4** · `## WIC Decode and Open-Dialog Mechanism` — failure-classification bullet tail (line 55).
- BEFORE: `…both surface the single unsupported-file message (constant in `src/image_loading.rs`) in the floating navigation pill's message line (the pill renders only while an image is open — mechanism: `## Horizontal Navigation Mechanism (Implemented, Wave 2)`).`
- AFTER: `…both surface the single unsupported-file message (constant in `src/image_loading.rs`) in the floating navigation pill's message line (the pill renders while an image is open or an empty-folder context is browsed — mechanism: `## Vertical Navigation and Session Last-Viewed Memory (Implemented, Wave 3)`).`
- Why: the pill-visibility fact changed with D2-final.

**T-5** · `## Horizontal Navigation Mechanism (Implemented, Wave 2)` — two clause updates inside the section's keyboard bullet (line 65) and pill bullet (line 68):
- T-5a (keyboard bullet, line 65) BEFORE: `- Keyboard: `ArrowLeft` → previous, `ArrowRight` → next, evaluated per frame; the arrows navigate DIRECTLY — pan-then-navigate interplay is Wave-5 scope, not implemented here.`
  AFTER: `- Keyboard: `ArrowLeft` → previous, `ArrowRight` → next — and, since Wave 3, `ArrowUp`/`ArrowDown` (vertical axis: the section below) — all evaluated per frame; the arrows navigate DIRECTLY — pan-then-navigate interplay is Wave-5 scope, not implemented here.`
- T-5b (pill bullet, line 68) BEFORE: `…←/→ navigate previous/next through the same `navigate` seam as the keys; ↑/↓ are visible but DISABLED (Wave-3 reserve). A direction button is enabled exactly when the navigation seam resolves a neighbor. The pill renders ONLY while an image is open, so a failure message on the empty state is invisible by approved design.`
  AFTER: `…←/→ and ↑/↓ all navigate through the same `navigate` seam as the keys (the vertical axis: the section below). A direction button is enabled exactly when the navigation seam resolves a neighbor. The pill renders while an image is open or while an empty-folder context is browsed; on the fresh empty state it is absent by approved design, so a failure message there is never shown.`
- Why: removes the last "disabled reserve" assertions from current-state text; the horizontal section keeps its own history (0.2.0 rails replacement sentence stays).

**T-6** · NEW SECTION inserted between the Horizontal section and `## Development Environment and Setup` — exact text to insert verbatim:

```markdown
## Vertical Navigation and Session Last-Viewed Memory (Implemented, Wave 3)

Implemented in `src/sibling_folder_listing.rs` (pure sibling-folder listing, container-testable), `src/app/folder_navigation.rs` (the vertical arm of the shared `navigate` seam), and `src/last_viewed_images.rs` (the memory):

- Enumerated scope: the sibling folders of the opened image's folder — ONE non-recursive directory read of the current folder's PARENT; only immediate child DIRECTORIES of that scope parent are listed (deeper folders are never entered, the current folder itself is excluded, and directory-ness comes from each entry's `file_type`, which leaves symlinked folders unlisted — the sibling listing's symlink-policy test); an unreadable scope parent yields an empty listing, an unresolvable anchor (no parent / missing current folder) yields no listing.
- Order (v1 fixed): the SAME fixed alphanumeric collation as the image listing, now shared via `src/name_collation.rs` (`compare_entry_names`: lowercase form first, exact-character tie-break) — one fixed order for files AND folders.
- Anchor + resolution: the OPENED IMAGE (or, in empty-folder mode, the browsed folder with a placeholder name) is passed verbatim and the module canonicalizes its paths internally (`fs::canonicalize`); resolution is ±1 per direction with `ListBeginningReached`/`ListEndReached` at the list edges — an edge resolves to no neighbor (no wrap, no skip, no message).
- Intake + buttons: `ArrowUp` → previous sibling folder, `ArrowDown` → next, evaluated per frame in `src/app.rs`; the pill's ↑/↓ buttons carry the same directions (accessible labels `Previous sibling folder` / `Next sibling folder`); all four arrows share the one `navigate` seam, and each is enabled exactly when `neighbor_image_path(direction)` resolves Something (per frame; in empty-folder mode the ←/→ image arrows resolve to no neighbor).
- Target-image priority (binding correction — NO same-filename attempt): (a) if the target folder was visited this session → its recorded last-viewed image; (b) otherwise ALWAYS the target folder's FIRST whitelisted image in the fixed collation (the first-listed probe through `FolderListing::first_listed_image`); NO other fallback; an image-less folder resolves to no target → empty-folder mode below; the chosen target always opens through the NORMAL `open_image` flow (a decode failure — e.g. the recorded last-viewed path vanished — keeps the previously displayed image and shows the unsupported-file message).
- Canonicalization (D5): the single normalization point of the open flow is `open_image` — it resolves the opened path through `fs::canonicalize` (raw-path fallback) before the open outcome applies; the last-viewed KEY is that resolved path's parent folder (plain `parent()`, never re-canonicalized) and the vertical listing's folders come from the same canonicalized resolution, so same-folder visits meet the same map entry regardless of path spelling.
- Empty-folder browsing mode (D2-final, as shipped): an image-less sibling folder is ENTERED, NOT skipped — the browsed folder replaces the open image (opened path, decoded payload, horizontal listing and message released; NO last-viewed entry is recorded); the vertical listing re-anchors ON the browsed folder, so ↑/↓ continue from it across both folder kinds while the list edges stay silent; the blank central surface shows the centered `Empty folder` constant (`EMPTY_FOLDER_MESSAGE` in `src/app/empty_folder_view.rs`) — NOT in the pill's message line, which stays empty in this mode; the pill renders while a folder is browsed (visibility gate: image open OR browsed-folder context), a failed open in this mode only updates the pill's message line, and the mode ends at the next successful open (the browsed-folder context is cleared); a vanished browsed folder yields an empty vertical listing (both folder arrows disable, no message).
- Session last-viewed memory: `LastViewedImages` — a RAM-only folder→image map created empty with the application, updated at EVERY successful open, read for the visited-folder priority lookup, and destroyed with the application struct; NO persistence to disk in v1.
```

- Why/attribution: every clause is lifted from the landed code's own doc comments and tests (verified 4.1b); title chosen to keep the Contents-link style; slotted right after the Horizontal section it extends.

**T-7** · `## Testing` — REPLACE the first bullet (the census bullet, line 102) with the file-split census; leave the other two bullets (lines 103–104) untouched.
- BEFORE (line 102): the bullet starting `- 26 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3): `src/folder_listing/tests.rs` — 5 …` (one long bullet).
- AFTER (one bullet, same style, verbatim):
  ```
  - 50 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3; the Wave-3 test files were split across modules to respect the per-file size cap): `src/sibling_folder_listing/tests.rs` — 6 (sibling-folder collation sort with exact-name tie-break; directories-only enumeration with the synthetic scope; current-folder exclusion; non-recursive immediate-siblings-only; no-siblings yields an empty listing; unreadable scope yields an empty listing and an unanchorable anchor yields no listing); `src/sibling_folder_listing/neighbor_tests.rs` — 2 (±1 folder stepping with scope-edge reports; symlinked scope entries are never listed as directories); `src/name_collation.rs` — 2 (lowercase form decides before exact characters, exact tie-break when folded forms match); `src/folder_listing/tests.rs` — 5 (unchanged horizontal listing census); `src/app/tests.rs` — 9 (unchanged shell/open-flow census); `src/app/floating_controls_tests.rs` — 1 (arrow-button enablement follows the horizontal navigation edges); `src/app/folder_navigation_tests.rs` — 5 (adjacent sibling folders resolve in sort order; a visited folder resurfaces its last-viewed image; a new folder opens its first image; entering an image-less folder clears the image and re-anchors the vertical listing; empty-folder mode continues vertically into both folder kinds); `src/app/folder_navigation_continuation_tests.rs` — 4 (sibling-list edges stay silent in image and empty-folder modes; a vanished last-viewed target reports the standard failure and keeps the displayed image; button enablement matches neighbor-resolution semantics; a vertical target open records the new folder's last-viewed memory); `src/app/last_viewed_tests.rs` — 5 (fresh empty memory; success records the folder's image; every open updates the entry; a failed open keeps it; folders are scoped independently); `src/image_loading.rs` — 4 (message wording; per-failure message mapping; pixel-buffer sizing; empty/overflow rejection); `src/app_display.rs` — 2 (initial texture state; upload happens exactly once per decode); `src/scaling.rs` — 5 (fit vectors ×4; zero-dimension rejection).
  ```
- Arithmetic for the junior's cross-check: 6+2+2+5+9+1+5+4+5+4+2+5 = **50**.

**T-8** · no other tech.md edits. Stale-sweep leftovers retained ON PURPOSE (dated history still true): Status line's "the 0.2.0 corner rails … were removed" sentence; the `rust:1.99.1-bookworm` pin facts; nothing else.

### 2.3 `.agent/project-info/architecture.md` (11 changes; Architectural Contents stays AS-IS — no new section: the wave's facts slot into existing Logical-Components / Module-Layout / Critical-Paths / Data-and-State / Decisions structures)

**A-1** · `## Status` final sentences (line 5).
- BEFORE: `…and an empty-state center Open button. Vertical navigation (Wave 3), fullscreen, zoom, and "Open with..." remain planned. Facts are sourced from [brief.md](brief.md)…`
- AFTER: `…and an empty-state center Open button. Wave 3 (branch `feat/wave-3-vertical-scroll`, version 0.3.0, 2026-11-01/02) landed the vertical sibling-folder navigation slice: the sibling-folder listing module, the Up/Down keys and pill buttons, the vertical target-image priority, the empty-folder browsing mode, and the session last-viewed memory. Dev checks ended `ALL CHECKS PASSED` (50 unit tests), the exe was rebuilt at `dist/windows-x64/img-scroller.exe` (19,502,691 bytes), and the DLL import audit is unchanged (the OBSERVED 22-DLL list lives only in `scripts/build-windows.sh` + [docs/build-windows.md](../../docs/build-windows.md)). Fullscreen, zoom, and "Open with..." remain planned. Facts are sourced from [brief.md](brief.md)…`
- Why: wave-closing status; docs-split honored.

**A-2** · Logical Components — Viewer UI bullet (line 30), two edits inside it:
- A-2a BEFORE: `While an image is open, ONE centered floating bottom pill renders above the central image surface — a non-space-reserving overlay `Area` anchored bottom-center with a translucent dark frame — holding the failure message line and four arrow buttons with PAINTED vector-triangle arrows (no font dependency; ←/→ navigate previous/next, ↑/↓ are visible but disabled until Wave 3).`
  AFTER: `While an image is open — or an empty sibling folder is being browsed — ONE centered floating bottom pill renders above the central surface — a non-space-reserving overlay `Area` anchored bottom-center with a translucent dark frame — holding the message line and four arrow buttons with PAINTED vector-triangle arrows (no font dependency; ←/→ navigate previous/next image, ↑/↓ navigate previous/next sibling folder, each enabled exactly when its navigation seam resolves a destination). In empty-folder browsing mode the blank central surface shows the centered `Empty folder` message while the pill's message line stays empty.`
- A-2b BEFORE: `Remaining UI (activating the folder arrows, fullscreen toggle, zoom controls, "Open with..." menu entry): planned.`
  AFTER: `Remaining UI (fullscreen toggle, zoom controls, "Open with..." menu entry): planned.`

**A-3** · Logical Components — Navigation Engine bullet (line 32).
- BEFORE: `- **Navigation Engine — horizontal slice LANDED (Wave 2); vertical slice planned (Wave 3)**: horizontal — previous/next sibling image in the current folder (listing: non-recursive parent-folder read, 17-extension whitelist; order: lowercase-folded alphanumeric with exact-name tie-break; list edges stay silently); vertical — previous/next sibling folder under the SAME parent (target image priority: (a) session last-viewed image for folders already visited this session, (b) otherwise ALWAYS the target folder's first image in alphanumeric order).`
- AFTER: `- **Navigation Engine — both slices LANDED (horizontal Wave 2; vertical Wave 3)**: horizontal — previous/next sibling image in the current folder (listing: non-recursive parent-folder read, 17-extension whitelist; order: lowercase-folded alphanumeric with exact-name tie-break; list edges stay silently); vertical — the ADJACENT sibling folder under the SAME parent (target image priority: (a) session last-viewed image for folders already visited this session, (b) otherwise ALWAYS the target folder's first image in alphanumeric order — NO same-filename attempt, NO other fallback; no skip, no wrap; an image-less sibling folder is entered in empty-folder browsing mode, not skipped).`

**A-4** · Logical Components — Session Last-Viewed Memory bullet (line 33).
- BEFORE: `- **Session Last-Viewed Memory — planned**: RAM-only map of the last viewed image per visited folder; destroyed on app close; no disk persistence in v1.`
- AFTER: `- **Session Last-Viewed Memory — LANDED (Wave 3)**: `src/last_viewed_images.rs` holds the RAM-only map of the last viewed image per visited folder — created empty with the app, updated at every successful open (the key is the RESOLVED open path's parent folder — D5 canonicalization fact), read for the visited-folder priority, destroyed on app close; no disk persistence in v1.`

**A-5** · Module Layout — update the `src/app.rs` bullet (line 43).
- BEFORE: `- `src/app.rs` — application shell: `ImagesScrollerApp` state (opened path, in-session folder listing, decoded payload, message text, display), the `open_image` flow (decode → state transition; failure changes only the message), the Left/Right keyboard navigation and the shared `navigate(NavigationDirection)` seam, the empty-state centered "Open…" button wiring, the image-open gate that shows the floating pill, the `APP_WINDOW_TITLE` constant, and `NativeOptions` (resizable, not maximized). Child modules: `src/app/arrow_direction.rs` (the painted-arrow control types), `src/app/floating_controls.rs` (the floating pill), `src/app/tests.rs`, `src/app/floating_controls_tests.rs`.`
- AFTER: `- `src/app.rs` — application shell: `ImagesScrollerApp` state (opened path, in-session folder listing, session last-viewed memory, vertical sibling-folder listing, browsed empty-folder context, decoded payload, message text, display), the `open_image` flow (canonicalizes the path once — the D5 single-normalization point — then decode → state transition; failure changes only the message), the four-direction arrow-key intake and the shared `navigate(NavigationDirection)` seam wiring, the empty-state centered "Open…" button wiring, the navigation-context gate that shows the pill (image open OR empty-folder context), the `APP_WINDOW_TITLE` constant, and `NativeOptions` (resizable, not maximized). Child modules: `src/app/arrow_direction.rs`, `src/app/floating_controls.rs`, `src/app/folder_navigation.rs`, `src/app/empty_folder_view.rs`, `src/app/tests.rs`, `src/app/floating_controls_tests.rs`, `src/app/folder_navigation_tests.rs`, `src/app/folder_navigation_continuation_tests.rs`, `src/app/last_viewed_tests.rs`.`

**A-6** · Module Layout — insert three NEW bullets after the `src/folder_listing.rs` bullet (line 44) — verbatim:
```markdown
- `src/sibling_folder_listing.rs` — pure sibling-folder listing for vertical navigation (split test modules `src/sibling_folder_listing/tests.rs` and `src/sibling_folder_listing/neighbor_tests.rs` over one TempScope fixture): one non-recursive read of the current folder's parent, immediate child directories only (current folder excluded; symlinked entries unlisted — the `file_type` policy), the shared fixed collation sort, and ±1 neighbor resolution reporting the list edges; an unreadable/missing scope yields an empty listing, an unanchorable anchor no listing.
- `src/last_viewed_images.rs` — session last-viewed memory: RAM-only folder→last-displayed-image map, updated at every open-success, read for the visited-folder target priority, destroyed with the app; no persistence.
- `src/name_collation.rs` — the fixed v1 alphanumeric entry-name collation (`compare_entry_names`: lowercase form first, exact-character tie-break) shared by the image listing and the sibling-folder listing.
```
(Also acceptable alternative for the junior: group the SAME three bullets as one insert if context-diff noise matters — but the three-bullet form is the binding one.)

**A-7** · Module Layout — update the `src/app/floating_controls.rs` bullet (line 45).
- BEFORE: `…a button is enabled exactly when the navigation seam resolves a neighbor (`neighbor_image_path(...).is_some()`), ←/→ drive the `navigate` seam, and ↑/↓ are disabled Wave-3 reserves; the pill renders only while an image is open (caller gate in `app.rs`).`
- AFTER: `…a button is enabled exactly when the navigation seam resolves a neighbor (`neighbor_image_path(...).is_some()`), and ALL FOUR arrows drive the `navigate` seam — the folder ones mapped to `PreviousSiblingFolder`/`NextSiblingFolder`; the pill renders while an image is open or an empty-folder context is browsed (gate `navigation_context_is_active()` in `src/app/folder_navigation.rs`, called from `app.rs`).`

**A-8** · Module Layout — update the `src/app/arrow_direction.rs` bullet (line 46).
- BEFORE: `- `src/app/arrow_direction.rs` — pure child-module types for the pill: `ArrowDirection` (the `rotated_triangle` rotation mapping: left/right/up/down) and `ArrowControl` (which button a widget represents: accessible labels + the optional `NavigationDirection`, `None` for the Wave-3-reserved folder arrows).`
- AFTER: `- `src/app/arrow_direction.rs` — pure child-module types for the pill: `ArrowDirection` (the `rotated_triangle` rotation mapping: left/right/up/down) and `ArrowControl` (which button a widget represents: accessible labels + a concrete `NavigationDirection` for EVERY arrow — the folder arrows map to `PreviousSiblingFolder`/`NextSiblingFolder`).`

**A-9** · Module Layout — update the `src/folder_listing.rs` bullet tail (line 44).
- BEFORE: `…and ±1 neighbor resolution that reports the list's beginning/end (mechanism details: [tech.md](tech.md) → `## Horizontal Navigation Mechanism (Implemented, Wave 2)`).`
- AFTER: `…and ±1 neighbor resolution that reports the list's beginning/end, plus `first_listed_image` (the vertical navigation's new-folder first-image probe); its sort is the shared collation from `src/name_collation.rs` (mechanism details: [tech.md](tech.md) → the horizontal and Wave-3 mechanism sections).`

**A-10** · Navigation Critical Paths — intro (line 61) and Path 3 (line 65).
- A-10a BEFORE: `Path 1's open/decode/display slice (Wave 1) and Path 2's horizontal sibling-image navigation (Wave 2) are implemented; the vertical-navigation, pan/zoom, and "Open with..." parts remain planned.`
  AFTER: `Path 1's open/decode/display slice (Wave 1), Path 2's horizontal sibling-image navigation (Wave 2), and Path 3's vertical sibling-folder navigation (Wave 3) are implemented; the pan/zoom and "Open with..." parts remain planned.`
- A-10b BEFORE: `3. **Vertical navigation**: arrow key or UI button → previous/next sibling folder under the same parent → target image chosen by priority: (a) the folder's session last-viewed image if the folder was already visited this session (RAM-only), else (b) ALWAYS the target folder's first image (alphanumeric order) → decode → display; while the image is taller than the window, pan first; navigation only at the image's top/bottom edge.`
  AFTER: `3. **Vertical navigation (Wave-3 slice LANDED)**: arrow key or UI button → the ADJACENT previous/next sibling folder under the same parent (no skip, no wrap; silent list edges) → target image chosen by priority: (a) the folder's session last-viewed image if the folder was already visited this session (RAM-only), else (b) ALWAYS the target folder's first image (alphanumeric order); NO same-filename attempt → decode → display through the normal open flow; an image-less sibling folder is entered in empty-folder browsing mode (the blank surface shows `Empty folder`; the vertical listing re-anchors on the browsed folder). The pan-first interplay — navigation only at the zoomed image's top/bottom edge — remains Wave-5 planned; arrows navigate directly now.`

**A-11** · Four remaining stale facts:
- A-11a Data and State (line 72) BEFORE `- **In memory (per session, planned)**: folder context while navigating; the per-folder last-viewed map (RAM-only, destroyed on close — no persistence in v1); window/zoom state for the session.`
  AFTER `- **In memory (per session; Wave-3 landed)**: the horizontal folder listing, the vertical sibling-folder listing, the browsed empty-folder context, and the per-folder last-viewed map (RAM-only, destroyed on close — no persistence in v1); window/zoom state for the session stays planned.`
- A-11b Project Paths (line 89) BEFORE `…Cargo.toml` (package `img-scroller` 0.2.1, edition 2024)…` AFTER `…Cargo.toml` (package `img-scroller` 0.3.0, edition 2024)…`
- A-11c Project Paths (line 90) BEFORE `…application code in `src/` (the `app` shell with its `arrow_direction`/`floating_controls` child modules, `app_display`, `folder_listing`, `image_loading`, `scaling`, plus the `platform/` facade)…` AFTER `…application code in `src/` (the `app` shell with its `arrow_direction`/`floating_controls`/`folder_navigation`/`empty_folder_view` child modules, `app_display`, `folder_listing`, `last_viewed_images`, `name_collation`, `sibling_folder_listing`, `image_loading`, `scaling`, plus the `platform/` facade)…`
- A-11d Technical Decisions Summary row (line 109) BEFORE `| Last-viewed memory: session-only, RAM | v1 constraint | …` AFTER `| Last-viewed memory: session-only, RAM | landed (Wave 3) | …`
- A-11e Open Architectural Decisions (line 116) — delete the bullet `- Vertical-navigation engine design: sibling-folder flow, target-image priority state, and the session last-viewed memory (Wave 3). The horizontal sibling-image slice is decided and landed (Wave 2).` entirely (decided + landed); leave the rest of the list untouched.

### 2.4 `.agent/project-structure.md` (4 changes; append-style edits only)

**S-1** · `Files in src/` — update the `folder_listing.rs` entry tail.
- BEFORE: `- folder_listing.rs - current-folder image listing for horizontal navigation: one non-recursive parent-folder read, 17-extension image whitelist, lowercase-folded sort with exact-name tie-break, ±1 neighbor stepping with list-edge reports`
- AFTER: `- folder_listing.rs - current-folder image listing for horizontal navigation: one non-recursive parent-folder read, 17-extension image whitelist, ±1 neighbor stepping with list-edge reports, plus first_listed_image (the vertical first-image priority probe); sorts with the shared name_collation`
- Insert two NEW entries immediately after the `main.rs` line (keep the file's one-line-per-module style):
  - `- last_viewed_images.rs - session RAM map of the last displayed image per visited folder: created empty with the app, updated at every open-success, read for the visited-folder target priority, destroyed with the app (no persistence)`
  - `- name_collation.rs - fixed v1 alphanumeric entry-name collation shared by the image and folder listings (lowercase form first, exact-character tie-break) with its inline unit tests`
  - `- sibling_folder_listing.rs - sibling-folder listing for vertical navigation: one non-recursive read of the current folder's parent, immediate child directories only (current folder excluded; symlinked entries unlisted), shared-collation sort, ±1 folder stepping with scope-edge reports`

**S-2** · `Folders in src/` — insert a new bullet after the `folder_listing/` bullet:
- `- sibling_folder_listing/ - child modules: tests.rs (listing/collation/self-exclusion/non-recursive/empty-scope unit tests), neighbor_tests.rs (neighbor-stepping unit tests + the symlink policy tests, reusing the sibling tests module's TempScope fixture)`
  (This is THE entry the Task-4 docs check flagged as missing.)

**S-3** · `Folders in src/` — rewrite the `app/` bullet to cover every child module.
- BEFORE: `- app/ - child modules of the application shell: arrow_direction.rs (painted-arrow control types for the floating pill), floating_controls.rs (centered floating bottom pill: message line + painted vector-triangle arrow buttons), tests.rs (shell/navigation unit tests), floating_controls_tests.rs (pill arrow-button enablement tests)`
- AFTER: `- app/ - child modules of the application shell: arrow_direction.rs (painted-arrow control types for the floating pill), floating_controls.rs (centered floating bottom pill: message line + painted vector-triangle arrow buttons), folder_navigation.rs (shared navigate seam: the four directions' ±1 neighbor resolution on both axes, the vertical target-image priority, and empty-folder mode), empty_folder_view.rs (the empty-folder browsing message), tests.rs (shell/navigation unit tests), floating_controls_tests.rs, folder_navigation_tests.rs, folder_navigation_continuation_tests.rs, last_viewed_tests.rs (pill enablement and vertical navigation / last-viewed / empty-folder unit tests)`

**S-4** · NO other entries change (`platform/`, other folders, root files are unaffected by this wave).

### 2.5 `.agent/project-info/context.md` (4 changes) — DEPENDS on NOTHING above except the final wording; do LAST

**C-1** · `## Current Work Focus` — REPLACE the whole paragraph (the one starting `Dated 2026-11-01. **Wave-2 UI fix…`) with this verbatim closing paragraph:
```markdown
Dated 2026-11-02. **Wave 3 ([`.agent/todos/20261029/20261029-todo-5.md`](../todos/20261029/20261029-todo-5.md)) implementation is LANDED on `feat/wave-3-vertical-scroll` — Tasks 1–5 processed through the Critical Workflow; NOT yet merged (Task-5 review/closing marks and the Step-5 merge/push to `main` remain) — at version `0.3.0`.** Landed: vertical sibling-folder navigation — `src/sibling_folder_listing.rs` (+ its split test modules) enumerates the immediate child DIRECTORIES of the current folder's parent (current folder excluded, sorted by the fixed collation now shared from the new `src/name_collation.rs`); Up/Down arrow keys and the pill's ↑/↓ buttons (now ENABLED) reach the ADJACENT sibling folder through the one `navigate` seam in `src/app/folder_navigation.rs` (no skip, no wrap, silent list edges); target priority: a folder visited this session reopens its recorded last-viewed image, a new folder opens its FIRST whitelisted image (NO same-filename rule, no other fallback); an image-less sibling folder is ENTERED in empty-folder browsing mode (D2-final): the open image is released, the blank central surface shows the centered `Empty folder` message, the pill stays visible with its message line empty, no last-viewed entry is recorded, and ↑/↓ continue from the browsed folder; `open_image` canonicalizes the opened path once (D5) so the `src/last_viewed_images.rs` RAM map (updated at every open-success, destroyed on close, no persistence) keys consistently. Verification: 50 unit tests (census + file-split accounting in `tech.md` → `## Testing`), dev-checks `ALL CHECKS PASSED`, exe rebuilt `dist/windows-x64/img-scroller.exe` = 19,502,691 bytes, DLL import audit unchanged (22 OBSERVED DLLs — recorded ONLY in `scripts/build-windows.sh` + `docs/build-windows.md`). **The 0.3.0 OPERATOR-RUN on a real Windows machine is the wave's pending acceptance gate.**
```

**C-2** · `## Recent Changes` — INSERT one new entry ABOVE the current first entry (keep the file's repeated `1.` numbering style; keep ALL existing entries untouched — they are history):
```markdown
1. 2026-11-02: **Wave 3 implementation landed on `feat/wave-3-vertical-scroll` (not yet merged) at 0.3.0; Task-5 documentation landed in this commit.** Commits (chronological): `253d4db` (Wave-3 global plan), `89e1db4` (version 0.3.0; `6a3ca30` Cargo.lock refresh), `bc7261a`/`c2dcfba`/`e2845c1`/`4864c3c` (Task 1 sibling-folder enumeration + review fix + module docs), `1428c4b`/`bc4456d`/`36be11b` (Task 2 session last-viewed memory + review simplifications), `5d503a2`/`7657f32`/`51c1b08`/`9c2c94a` (Task 3 up/down keys + pill buttons, review fixes, module docs), `bde6b7f`/`7f2afd9`/`4b79e5c`/`05f8a2e`/`6d243fb` (fmt round, scoped-reader lifetime fix, test-file cap conformance, module-split docs, Task 4 mark; dev-checks `ALL CHECKS PASSED` 50/50; exe rebuilt 19,502,691 B; DLL audit = the recorded 22-DLL constant), and this documentation commit (Task 5 step 4.2: README/tech/architecture/structure/context).
```

**C-3** · Current-state section updates (three small edits in `## Current Project State` + one in its Wave-2-UI-fix bullet):
- C-3a INSERT a new Wave-3 bullet as the FIRST bullet of the section:
  `- **Wave 3 landed on `feat/wave-3-vertical-scroll` (NOT yet merged to `main`; version 0.3.0; Tasks 1–5 done, Task-5 review + Step-5 merge/push pending)**: `src/sibling_folder_listing.rs` (+ `tests.rs`/`neighbor_tests.rs`), shared collation `src/name_collation.rs`, session memory `src/last_viewed_images.rs`, `src/app/folder_navigation.rs` (+ `folder_navigation_tests.rs`, `folder_navigation_continuation_tests.rs`, `last_viewed_tests.rs`, `empty_folder_view.rs`); Up/Down keys and pill ↑/↓ enabled; target priority last-viewed → first image (NO same-filename rule); empty-folder browsing mode (D2-final); 50 unit tests; dev-checks `ALL CHECKS PASSED`; exe rebuilt 19,502,691 bytes; DLL import audit unchanged.`
- C-3b in the Wave-2-UI-fix bullet, tail BEFORE `…DLL import audit unchanged (22 OBSERVED DLLs); OPERATOR-RUN re-test pending.` AFTER `…DLL import audit unchanged (22 OBSERVED DLLs); its operator re-test is folded into the upcoming 0.3.0 run (below).`
- C-3c Release-artifact bullet BEFORE `- Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,461,520 bytes, 0.2.1 UI-fix rebuild 2026-11-01); import audit: …` AFTER `- Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,502,691 bytes, 0.3.0 Wave-3 rebuild); previous 0.2.1 artifact (19,461,520 bytes) was overwritten by the atomic rebuild; import audit: 22 OBSERVED DLLs recorded in `scripts/build-windows.sh` + `docs/build-windows.md` (unchanged).`
- C-3d TODO-state bullet BEFORE `- **TODO state**: `.agent/todos/20261101/20261101-todo-1-DONE.md` (renamed at `2b73a5f`; Tasks 1–5 all `[DONE]`; the Task-4 OPERATOR-RUN sub-item remains unchecked until the user reports). `.agent/todos/20261029/20261029-todo-4-DONE.md` remains done.` — keep that sentence, then REPLACE the next sentence `- Wave TODO files ready: `20261029-todo-5.md` (Wave 3) … `todo-9` (Wave 7); process ONE wave per session, in order.` WITH `- `20261029-todo-5.md` (Wave 3): Tasks 1–4 marked `[DONE]` on the branch (the Task-4 OPERATOR-RUN sub-item remains unchecked until the user reports); Task 5 in progress — documentation landed this commit, marks due at 4.6. `20261029-todo-6.md` … `todo-9` ready; process ONE wave per session, in order.`
- C-3e Git bullet BEFORE `- Git: `main` == `origin/main` == `2b73a5f`; working tree clean; no other branches (`fix/centered-floating-controls` deleted after the verified merge).` AFTER `- Git: `main` == `origin/main` == `2b73a5f` (unchanged since the UI-fix merge); implementation branch `feat/wave-3-vertical-scroll` holds the Wave-3 commits through the Task-5 documentation commit; push remains restricted to Step 5 (`main` → `origin` ONLY).`

**C-4** · `## Immediate Next Steps` — REPLACE the whole numbered list (the three current items) with this verbatim list (binds the caller's requirement: OPERATOR-RUN checklist user-side + Wave-4 pointer; the list-ending renormalization flag is preserved):
```markdown
1. **Finish the Task-5 process on the branch, then Step 5**: review of this documentation change and the Task-5 marks (4.6) will land as governed by the Critical Workflow; afterwards fast-forward merge `feat/wave-3-vertical-scroll` to `main` and push `main` to `origin` ONLY.
2. **Wave-3 OPERATOR-RUN (user-side, real Windows 10/11, on the rebuilt 0.3.0 exe `dist\windows-x64\img-scroller.exe` — this same run closes out the pending 0.2.1 pill re-test)** — checklist: (a) ↑/↓ arrow keys and the pill's ↑/↓ buttons move to the adjacent previous/next sibling folder under the same parent — no grandparent/deeper jumps, no wrap; at the first/last sibling folder the viewer stays; (b) revisiting a folder this session reopens its last-viewed image; a first visit opens the folder's FIRST image in alphanumeric order (no same-filename behavior may be observed); (c) an image-less sibling folder shows the centered `Empty folder` message on the blank surface with the pill visible and its message line empty; ↑/↓ keep working from it and the next successful open returns to image mode; (d) closing and relaunching the app loses the memory (a revisit stops reusing last-viewed); (e) 0.2.1-era checks still hold: painted arrow triangles (no tofu), ONE bottom-center pill, big centered Open ONLY in the fresh empty state, ←/→ image navigation unchanged. Report results back into the project.
3. **Next session: Wave 4** ([`.agent/todos/20261029/20261029-todo-6.md`](../todos/20261029/20261029-todo-6.md) — fullscreen mode + zoom/fit modes) — process in a NEW session.
4. **Open item (flagged, not a TODO)**: broader line-ending renormalization beyond `*.sh` (e.g. `Dockerfile`, `*.rs` are `i/lf w/crlf` in the Windows working tree) — a dedicated future plan would own it.
```

---

## 3. Legacy-fact sweep (mandatory; grep-driven inventory with dispositions)

Run per-command (single, unchained) AFTER all edits, against the five changed files:

1. `rg -n "26 test|26/26|50 test" README.md .agent/project-info/tech.md .agent/project-info/architecture.md .agent/project-structure.md .agent/project-info/context.md`
2. `rg -n "0\.2\.1|0\.2\.0|0\.3\.0" <the five files>`
3. `rg -n "reserv|visible but disabled|disabled until|reserved and disabled|Wave-3" <the five files>`
4. `rg -n "corner rail|rails" <the five files>`
5. `rg -n "vertical navigation, fullscreen|vertical navigation.*planned|planned \(Wave 3\)" README.md .agent/project-info/tech.md .agent/project-info/architecture.md`

Expected state after edits (disposition table — from the 4.1b grep, every hit is accounted):

| File | Hit (pattern from the task list) | Disposition |
| --- | --- | --- |
| README L21 | "its up/down folder arrows are visible but disabled until folder navigation lands" | UPDATE → R-1 (zero remaining) |
| README L35 | "remaining v1 features (vertical navigation, …)" | UPDATE → R-3 |
| README L39 | "0.2.1" | UPDATE → R-4 |
| README L55 | "26 unit tests", "↑/↓ reserved and disabled", "remaining v1 features (vertical navigation, …)" | UPDATE → R-5; the dated "0.2.1 UI fix (2026-11-01…)" opening stays (history) — exactly ONE `0.2.1` hit may remain in README, none in tech/architecture |
| tech L5 | "26 unit tests", "remaining v1 features (vertical navigation…)" | UPDATE → T-2 (the "0.2.0 corner rails … were removed" fragment of the Dated UI-follow-up sentence STAYS — dated history) |
| tech L27 | "version 0.2.1" | UPDATE → T-3 |
| tech L65/L68 | "Wave-5 scope" (stays, still true), "↑/↓ … DISABLED (Wave-3 reserve)", "only while an image is open" | UPDATE → T-5 ("Wave-5 scope" clause is retained deliberately — still the current scope) |
| tech L55 | "the pill renders only while an image is open" | UPDATE → T-4 |
| tech L102 | "26 unit tests …" census | REPLACE → T-7 |
| arch L5 | "Vertical navigation (Wave 3) … remain planned" | UPDATE → A-1 (the "replaced the corner rails and the top Open… toolbar" fragment STAYS — dated history) |
| arch L30 | "↑/↓ are visible but disabled until Wave 3", "activating the folder arrows" | UPDATE → A-2 |
| arch L32/L33 | "vertical slice planned (Wave 3)"; "Session Last-Viewed Memory — planned" | UPDATE → A-3, A-4 |
| arch L45/L46 | "↑/↓ are disabled Wave-3 reserves"; "`None` for the Wave-3-reserved folder arrows" | UPDATE → A-7, A-8 |
| arch L61/L65 | "vertical-navigation … remain planned"; "while the image is taller than the window, pan first" | UPDATE → A-10 (Path 2's "pan-then-navigate … lands in Wave 5" clause STAYS — still true) |
| arch L72/L89/L90/L109/L116 | "planned" data-state; "0.2.1"; missing new modules; constraint row; vertical open-decision bullet | UPDATE → A-11a…e |
| structure | "neighbor_tests.rs" absent; app/ bullet missing five files; folder_listing sort now shared | UPDATE → S-1…S-3 |
| context L15 (Current Work Focus) | whole current-state paragraph ("26 unit tests", "↑/↓ … Wave-3 reserves", "0.2.1", "Next: Wave 3 … NEW session") | REPLACE → C-1 |
| context L19–L25 (Recent Changes) | "26/26", "rails", "0.2.1", "Wave-3 reserves" inside dated wave-2 entries | KEEP — dated history, untouched (this is the only file where such fragments remain and it is correct) |
| context L38–L46 (Current Project State) | "26 unit tests", "0.2.1", "Wave TODO files ready: todo-5…" | UPDATE → C-3a…e (Wave-1/Wave-2 bullets' histories stay) |
| context L50–51 (Immediate Next Steps) | stale 0.2.1 re-test + "disabled until then" checklist | REPLACE → C-4 |

Success criteria: after edits, commands 1 and 3 return ZERO hits outside the protected history spots listed above; command 2 returns exactly the allowed `0.3.0` usages + README's single dated `0.2.1` + context.md's dated-history hits; command 5 zero.

---

## 4. Budget notes (≤ 2 lines rationale per change; additions vs the doc files' current line counts)

| File | Changes | Approx net line delta (add/−del within rewrites) | Note |
| --- | --- | --- | --- |
| README.md | R-1…R-5 (5) | +4 / −2 | All within existing paragraphs; no new sections |
| tech.md | T-1…T-7 (7) | +9 (new section = 8 lines + link) / contents renumber ±0 | T-7 replaces 1 long bullet with 1 long bullet |
| architecture.md | A-1…A-11 (11 small) | +10 / −3 | Bullet appends/rewrites only |
| project-structure.md | S-1…S-3 (3) | +4 | One-line-per-entry style preserved |
| context.md | C-1…C-4 (4) | +6 / −0 net (paragraph/list replacements) | New Recent-Changes entry is the largest block |

Nothing is reorganized beyond the T-1 Contents insert and the A-5/A-9/S-bullet rewrites. If any edit cannot stay within "replace phrase inside the existing sentence", STOP that edit and reuse the AFTER text verbatim as given.

---

## 5. Junior-implementer (4.2) execution checklist — IN ORDER

Before anything: confirm you are the 4.2 implementer for Wave-3 Task 5; no builds, no cargo, no VM, no other file may be touched. The plan is authorized as-is (per-task plan auto-approved).

1. `git status --short` — expect clean tree on `feat/wave-3-vertical-scroll`; `git branch --show-current` must print `feat/wave-3-vertical-scroll`. If not clean: STOP, return a question.
2. Read `.gitignore` (rule requirement) and re-read each of the five target files before editing it (fresh content; anchors below assume it).
3. Edit `README.md`: apply R-1 → R-5 in order.
4. Edit `.agent/project-info/tech.md`: apply T-1 (Contents insert incl. renumbering items 6–13 → 7–14) → T-2 → T-3 → T-4 → T-5a → T-5b → T-6 (insert the new section verbatim) → T-7 (census bullet swap).
5. Edit `.agent/project-info/architecture.md`: apply A-1 → A-2a → A-2b → A-3 → A-4 → A-5 → A-6 (insert three bullets) → A-7 → A-8 → A-9 → A-10a → A-10b → A-11a → A-11b → A-11c → A-11d → A-11e.
6. Edit `.agent/project-structure.md`: apply S-1 (with the three new file entries) → S-2 → S-3.
7. Edit `.agent/project-info/context.md`: apply C-1 → C-2 → C-3a…e → C-4. (Last, so the "Recent Changes" entry states the docs commit as "this commit".)
8. Self-verification pass (commands from § 3; plus the per-file factual cross-checks below). Fix any mismatch by re-applying the exact AFTER text; re-run until green. NO builds/tests — none exist for docs.
9. Stage + commit (§ 6), then re-run `git log -1 --stat` to confirm the staging list and the exact message; verify `CHANGELOG.md` and `.agent/todos/*` and `src/*` are absent from the diff.
10. Return: signal completion + the path of this plan file (no TODO-file edits — that is 4.6 after review).

### 5.1 Per-file factual cross-check lists (junior runs these BEFORE staging)

README.md:
- `rg -n "0\.3\.0"` → hits at the Language row and Project Status Wave-3 sentence (2+), `rg -n "0\.2\.1"` → exactly 1 (dated history sentence).
- `rg -c "disabled"` → 0 (the only old "disabled" clause was R-1/R-5a).
- `rg -n "NO same-filename"` → ≥ 1 (corrected rule remains explicit).
- `rg -n "Empty folder"` → 2 (R-2 + R-5b) — exact capitalization `Empty folder` everywhere.
- `rg -c "\[^0-9\]50 unit tests"` → 1.

tech.md:
- `rg -c "^[0-9]+\." README-like Contents` — count Contents entries = 14, and item 6 links `#vertical-navigation-and-session-last-viewed-memory-implemented-wave-3` EXACTLY matching the inserted heading text (title-case → lowercase kebab, no parentheses/commas).
- `rg -n "50 unit tests"` → 2 (Status + Testing).
- census sum check: run `rg -c "#\[test\]" src -g "*.rs"` and verify the per-file numbers in the T-7 bullet still match the output exactly (6/2/2/5/9/1/5/4/5/4/2/5 = 50); any mismatch → STOP and return a question.
- `rg -n "disables|disabled|reserv"` in tech.md → only D2-final-mode arrow-disable sentence in the new section (folder arrows disable for a vanished browsed folder) and no "Wave-3 reserve".
- `rg -n "19,502,691"` → 1 (Status). `rg -n "22-DLL`|22 OBSERVED"` → 1 (Status cross-reference — NO enumerated DLL names anywhere in tech.md).

architecture.md:
- `rg -n "vertical slice planned|— planned\*\*|planned (Wave 3)\*\*"` → 0.
- `rg -n "Empty folder"` → 1 (Viewer-UI bullet; Path 3 says `Empty folder` too → expected total 2 if A-10b applied).
- `rg -n "Navigation Engine"` constains `both slices LANDED`; `rg -n "Session Last-Viewed Memory"` contains `LANDED (Wave 3)`.
- `rg -n "sibling_folder_listing|last_viewed_images|name_collation"` → ≥ 3 module bullets present (A-6, A-11c).
- `rg -c "0\.2\.1"` → 0; `rg -c "0\.3\.0"` → 2 (Status + Project Paths).

project-structure.md:
- `rg -n "neighbor_tests.rs"` → exactly 1 (S-2 — the flagged missing entry now exists).
- `rg -n "folder_navigation.rs|last_viewed_tests.rs|folder_navigation_continuation_tests.rs|empty_folder_view.rs"` → each 1 inside the S-3/S-1 additions.
- File stays ≤ 40 lines (was 28; additions +4/~10 words per bullet).

context.md:
- `## Current Work Focus` starts with `Dated 2026-11-02. **Wave 3` and contains `50 unit tests`, `19,502,691`, `ALL CHECKS PASSED`, `NOT yet merged`, `2b73a5f` remains in `## Current Project State`.
- `## Recent Changes` STILL contains the untouched 2026-10-31/11-01 entries (history preserved), plus the new 2026-11-02 Wave-3 entry (9 commits + `this documentation commit` wording, NO commit hash for the docs commit).
- `## Immediate Next Steps` item 2 contains all five checklist letters (a)…(e) and `dist\windows-x64\img-scroller.exe`; item 3 links `20261029-todo-6.md`; item 4 keeps the line-ending flag verbatim.
- The section order and the `## How to Use This File` block stay untouched.

Global "-count of empty spots" sanity: the instruction "count of `[x]` vs empty checkboxes" applies to the TODO file, which 4.2 must NOT touch — record in the completion signal: "`20261029-todo-5.md` untouched at 4.2; Task-5 sub-items remain unchecked pending 4.6."

---

## 6. Git actions (single, unchained commands; PowerShell-safe)

1. `git status --short` → verify ONLY the five files appear as modified.
2. `.gitignore` compliance: none of the five paths is ignored (analysis-time check: `git check-ignore README.md .agent/project-info/tech.md .agent/project-info/architecture.md .agent/project-info/context.md .agent/project-structure.md` → no output). If output appears → STOP and question.
3. Stage (one command): `git add README.md .agent/project-info/tech.md .agent/project-info/architecture.md .agent/project-info/context.md .agent/project-structure.md`
4. `git status --short` → exactly five staged lines, nothing else staged.
5. Commit — message EXACTLY (verbatim, no co-author/footer): `git commit -m "docs: land vertical-scroll wave documentation"`
6. Post-commit read-only: `git log -1 --stat` → verify commit touches the five files, no CHANGELOG.md, no src, no plans, no todos. `git status --short` → clean.
7. NO push, NO merge, NO branch ops (Step-5 restricted). Verify rule compliance per `.kilo/rules/git-remote-safety.md`.

Allowed remaining legacy check on the branch before the 4.3 review hand-off: re-run `§ 3` command 2 — `0.3.0` must NOT appear in `Cargo.toml`-independent docs inconsistently (README/tech/architecture all `0.3.0`; version already bumped at `89e1db4`, NO further version bump here — deliberately out of scope).

---

## 7. Verification matrix (plan ↔ Task-5 requirements + wave-closing rules)

| Requirement (TODO Task 5 + caller) | Covered by |
| --- | --- |
| README vertical scroll + corrected rule (no same-filename) | R-1, R-2, R-5b |
| README empty-folder behavior as-shipped, wording = landed code | R-2, R-5b ("Empty folder" literal from `empty_folder_view.rs`) |
| tech/architecture session-memory component | A-4, T-6 (memory bullets), T-7 census |
| tech/architecture corrected navigation rule | A-3, A-10, T-6 (priority bullet, "NO same-filename attempt") |
| Remaining open items + census refresh 26→50 with file-split accounting | T-2, T-7, A-1 |
| tech.md Testing section rewrite | T-7 |
| Status → Wave 3 landed at 0.3.0 | A-1, T-2, R-3/R-4/R-5, C-1 |
| dev-checks ALL CHECKS PASSED + exe 19,502,691 + DLL audit unchanged (cross-reference only) | A-1, T-2, C-1 (docs-split honored — no copies) |
| context.md closing update (recent changes / current state / next steps = OPERATOR-RUN checklist + Wave-4 pointer) | C-1, C-2, C-3, C-4 |
| .agent/project-structure.md updated incl. neighbor_tests.rs | S-1…S-3 |
| Legacy-fact sweep in touched docs | § 3 dispositions per hit |
| CHANGELOG.md untouched (wave rule) | § 5 step 9 + § 6 step 6 |
| Do NOT mark Task-5 `[DONE]` (4.6 does it) | § 5 step 10 note |
| No code/harness/plan/TODO edits; versions untouched beyond facts stated | § 8 |

## 8. Deliberately Out of Scope (4.1b + 4.2)

- NO source-code changes of any kind; no harness/script changes; no rebuilds of `dist/*` (the shipped 19,502,691-byte exe is the fact being documented); no container/VM runs; no builds/tests.
- NO edits to `.agent/todos/**` (Task-5 marks happen at 4.6, after review); no edits to `CHANGELOG.md`; NO edits to any existing `.kilo/plans/*` file (this plan is a new file); NO edits to `docs/build*` (verified no-change); NO version bump (`0.3.0` landed at `89e1db4`; docs merely restate it).
- NO edits to `.agent/project-info/brief.md` or `product.md` (their same-filename fragment is superseded by the Addendum-v2 note machinery; not a Task-5 file); NO merge/push (Step 5); NO Wave-4 planning.

## 9. Decision summary (5 lines, for the return)

1. tech.md gains ONE new section `## Vertical Navigation and Session Last-Viewed Memory (Implemented, Wave 3)` (after the Horizontal section; Contents item 6, list renumbered to 14 items); architecture.md takes NO new section — its facts land in existing bullets; README gets list-prose edits only (no contents/nav block).
2. All five doc edits are bounded replacements/appends keyed to verbatim BEFORE→AFTER anchors; the D2-final empty-folder behavior and the D5 canonicalization are restated in the code's own wording (message literal `Empty folder`; `parent()`-derived keys; placeholder re-anchor; no last-viewed entry).
3. Legacy sweep: every stale current-state fact (26 tests, ↑/↓ disabled/reserve phrases, "planned Wave 3", 0.2.1) has an enumerated edit site; dated history sentences (context Recent Changes, README's dated 0.2.1 opening, corner-rails-removal sentences, "Wave-5 scope") are protected and stay.
4. Testing census rewritten to the verified 50/12-file split (6+2+2+5+9+1+5+4+5+4+2+5); docs/build.md + docs/build-windows.md verified NO-CHANGE (version-agnostic; 22-DLL list stays solely in scripts/build-windows.sh + docs/build-windows.md, cross-referenced everywhere else).
5. Git: one commit `docs: land vertical-scroll wave documentation` staging exactly the five files after a gitignore check; no push; CHANGELOG/todos/plans untouched; TODO marking deferred to 4.6.
