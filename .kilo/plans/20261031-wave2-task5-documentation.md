# Wave 2 Task 5 — Documentation Plan (exact doc edits + git plan)

Date: 2026-11-01
Source TODO: [`.agent/todos/20261029/20261029-todo-4.md`](../../.agent/todos/20261029/20261029-todo-4.md) → `### Task 5: Documentation updates` + §"Deferred decisions" + §"Future session execution notes"
Workflow slot: Wave-2 Critical Workflow, Task 5, sub-step 4.1b (this plan) → applied in 4.2.
Branch: `feat/wave-2-horizontal-scroll` (Tasks 1–4 landed; HEAD at plan time `85007ce`; version `0.2.0`).
Status: APPROVED (the user auto-approves per-task plans; every edit below is binding — no judgment left).
Executor split (global plan §3 Task 5): **docs-specialist applies the edits in 4.2; the implementer ONLY commits afterward** (markdown-generation rule). The implementer never edits the doc files in this step.

## 0. Session facts captured at plan time (all verified against the landed code/files — no invented facts)

- Landed Wave-2 code (verified by direct read):
  - `src/folder_listing.rs` — `IMAGE_FILE_EXTENSIONS: [&str; 17]` = `avif, bmp, dib, gif, heic, heif, ico, jfif, jpe, jpeg, jpg, jxr, png, tif, tiff, webp, wdp` (lines 10–13); extension match ignores ASCII case (`eq_ignore_ascii_case`, lines 110–114); collation = `to_lowercase()` primary compare, exact-name `cmp` tie-break (`compare_image_file_names`, lines 116–122); non-recursive parent-folder listing (`from_opened_image` → `fs::read_dir` of the parent only, lines 31–51); `previous_neighbor`/`next_neighbor` report `ListBeginningReached`/`ListEndReached` (lines 65–79).
  - `src/app.rs` — `current_folder_listing: Option<FolderListing>` rebuilt on every successful open (lines 19–20, 62); keyboard intake `ArrowLeft`→`navigate(Previous)`, `ArrowRight`→`navigate(Next)` per frame (lines 121–128); shared `navigate` seam reuses `open_image` (lines 88–94); `neighbor_image_path` returns `None` for every "nothing to navigate to" case (lines 96–111) → edges STAY silently, no wrap, no message; a failed neighbor open keeps the displayed image and shows the message (lines 68–71).
  - `src/app/rails.rs` — approved layout K3: `RAIL_SIZE = 40.0` (line 9); right rail `Panel::right("folder_rail")` with ↑/↓ rendered `add_enabled(false)` — disabled until Wave 3 (lines 27–41); bottom rail `Panel::bottom("image_rail")` with ←/→ buttons enabled exactly when the navigation seam resolves a neighbor, plus the message label (lines 44–73); BOTH rails render only inside `if self.image_is_open()` (`src/app.rs` lines 134–137) — with no image open there are no rails and no message surface (empty-state failure message invisible by approved design).
  - Test census (grep `#[test]` over `src/`): `src/app/tests.rs` 9 + `src/app/rails_tests.rs` 1 + `src/folder_listing/tests.rs` 5 + `src/image_loading.rs` 4 + `src/app_display.rs` 2 + `src/scaling.rs` 5 = **26 tests**.
- `Cargo.toml` line 3: `version = "0.2.0"` (bumped at Step 3, commit `69e8925`).
- Harness CRLF repair (commit `cf244d5` "fix(harness): normalize shell scripts to LF (repair container dev-checks)"): added `.gitattributes` with the single line `*.sh text eol=lf` (verified: file exists with exactly that line; commit stat = `.gitattributes | 1 +`). Root cause per `.kilo/plans/20261031-wave2-task4-verification.md` §2: both `scripts/*.sh` were `i/lf w/crlf` (CRLF Windows-checkout smudging); the container `sh` aborted (`set: Illegal option -`); both scripts were converted to LF on the VM; dev-checks then ended `ALL CHECKS PASSED` exit 0.
- Exe rebuilt in Task 4: `dist/windows-x64/img-scroller.exe` = **19,464,396 bytes** (verified host-side `ls dist\windows-x64`); DLL import audit unchanged: **22 OBSERVED DLLs** (list recorded in `docs/build-windows.md` lines 58–62 + `scripts/build-windows.sh`).
- Wave-1 exe confirmation: the user reported (session-start note of the Wave-2 global plan §1; reconfirmed 2026-11-01) that the Wave-1 exe OPERATOR-RUN **worked as expected** — Wave 1's pending manual-UX item is closed.
- Dockerfile-CRLF open item: `git ls-files --eol` shows `Dockerfile`, `docker-compose.yml`, `Cargo.toml` are `i/lf w/crlf` — only `*.sh` is LF-enforced by `.gitattributes`. Broader renormalization was deliberately NOT done in Task 4 (plan §2.4 forbids it there) — it is **flagged to the user as a possible dedicated future plan, NOT a TODO now**.
- Current branch/working tree: `git status --short` → exactly ` M .agent/project-info/context.md` (pre-existing unstaged session notes; absorbed by this step's context.md rewrite — see §2.6).

## 1. Scope — files changed and files NOT changed

Changed in this step (7 files):

1. `README.md`
2. `.agent/project-info/tech.md`
3. `.agent/project-info/architecture.md`
4. `.agent/project-structure.md`
5. `docs/build.md`
6. `.agent/project-info/context.md`
7. `.kilo/plans/20261031-wave2-task5-documentation.md` (this plan file — new, committed with the docs)

NOT changed (hard exclusions):

- `CHANGELOG.md` — wave binding (TODO §"Future session execution notes"; global plan §2.2 binding 4). NEVER staged.
- `.agent/project-info/brief.md` — requirements source of truth; Task 5 does not edit it.
- `.agent/project-info/product.md` — not in the Task-5 assignment; its feature list is brief-derived and stays.
- `docs/build-windows.md` — **verified: no truly wrong line exists.** Its audit paragraph (lines 53–62) records the 2026-10-31 audit of the first produced exe — a true historical observation; the 22-DLL list is unchanged in the Wave-2 rebuild. NO edit.
- Any pre-existing `.kilo/plans/*` file (TODO binding: never edit existing plan files).
- Any `src/` code, `Cargo.toml`, `Cargo.lock`, harness scripts, TODO files (4.6 marks `[DONE]`, not this step).

## 2. Exact edits per file

Apply with the dedicated read/edit tools (never PowerShell). Each "current" quote is exact — match it exactly.

### 2.1 `README.md` — 3 edits

**R1 — line 21 (feature 2: horizontal scroll behavior + current-folder ordering + K3 buttons phrase).**
Current:

```
2. **Horizontal Scroll (Sibling Images)** — left/right arrow keys and left/right UI buttons navigate to the previous/next sibling image in the same folder; the scope is strictly the current folder (not recursive) and the order is alphanumeric.
```

Replacement:

```
2. **Horizontal Scroll (Sibling Images)** — left/right arrow keys and the on-screen bottom-rail ←/→ buttons navigate to the previous/next sibling image in the same folder; the scope is strictly the current folder (not recursive); the order is alphanumeric — names compare case-insensitively, with an exact-character tie-break when names differ only by case; at the first/last image the viewer stays on it (no wrap-around). The navigation rails appear while an image is open; the right rail's up/down folder arrows are visible but disabled until folder navigation lands.
```

(Facts: keyboard+buttons `src/app.rs` 121–128 / `src/app/rails.rs` 59–70; current-folder scope `src/folder_listing.rs` 31–51; collation `src/folder_listing.rs` 116–122; edge stay `src/app.rs` 96–111; rails visible only with an image open `src/app.rs` 134–137; right rail disabled `src/app/rails.rs` 37–39.)

**R2 — line 35 (Technology Stack intro — removes the now-false "navigation still open").**
Current:

```
The choices below are confirmed and implemented for the Wave-1 open-image slice; the remaining v1 features (navigation, fullscreen, zoom, "Open with...") are still open — see [Project Status](#project-status) and [.agent/project-info/architecture.md](.agent/project-info/architecture.md).
```

Replacement:

```
The choices below are confirmed and implemented for the Wave-1 open-image slice and the Wave-2 horizontal-navigation slice; the remaining v1 features (vertical navigation, fullscreen, zoom, "Open with...") are still open — see [Project Status](#project-status) and [.agent/project-info/architecture.md](.agent/project-info/architecture.md).
```

**R3 — line 55 (Project Status paragraph — Wave-1-only status is stale).**
Current (single paragraph):

```
Wave 1 landed (2026-10-31, branch `feat/wave-1-skeleton-open-image`): the Cargo project exists, and the viewer opens images via a CLI argument or the in-app open dialog, decodes them through the WIC system codecs, and displays them fit-scaled in the regular resizable window. Verification so far: container dev checks (`ALL CHECKS PASSED`), Windows x64 release cross-build (exit 0), import audit; running the `.exe` on Windows is operator-run. The remaining v1 features (horizontal/vertical navigation, fullscreen, zoom, "Open with...") are open. The full requirements, decisions, and scope boundaries live in the project brief: [.agent/project-info/brief.md](.agent/project-info/brief.md).
```

Replacement:

```
Wave 1 landed (2026-10-31, merged to `main`): the Cargo project exists, and the viewer opens images via a CLI argument or the in-app open dialog, decodes them through the WIC system codecs, and displays them fit-scaled in the regular resizable window. Wave 2 landed (branch `feat/wave-2-horizontal-scroll`, version 0.2.0): horizontal sibling-image navigation — Left/Right arrow keys and the on-screen bottom-rail ←/→ buttons move to the previous/next image of the opened image's folder in case-insensitive alphanumeric order. Verification so far: container dev checks (`ALL CHECKS PASSED`, 26 unit tests), Windows x64 release cross-build (exit 0), import audit; running the `.exe` on Windows is operator-run (the Wave-1 build passed the operator's manual check). The remaining v1 features (vertical navigation, fullscreen, zoom, "Open with...") are open. The full requirements, decisions, and scope boundaries live in the project brief: [.agent/project-info/brief.md](.agent/project-info/brief.md).
```

(Do NOT touch README features 1, 3–7, the Overview, or the Build-and-Run section — no line there is false.)

### 2.2 `.agent/project-info/tech.md` — 5 edits + 1 new section + TOC renumber (file >100 lines → TOC rule applies)

**T1 — line 5 (Status paragraph).** Replace the whole paragraph:
Current:

```
Landed for Wave 1 (branch `feat/wave-1-skeleton-open-image`, 2026-10-31): the Cargo project, the egui/eframe window shell, the open-image flow (CLI argument + native open dialog + WIC decode), and the fit-scaled image display are implemented and verified through the container harness (dev checks: `ALL CHECKS PASSED`; Windows x64 release cross-build: exit 0 → `dist/windows-x64/img-scroller.exe`). The remaining v1 features (navigation, fullscreen, zoom, "Open with...", packaging) are planned. Facts are sourced from [brief.md](brief.md) (source of truth), the landed code, and the build guides ([docs/build.md](../../docs/build.md), [docs/build-windows.md](../../docs/build-windows.md)).
```

Replacement:

```
Landed for Wave 1 (branch `feat/wave-1-skeleton-open-image`, 2026-10-31): the Cargo project, the egui/eframe window shell, the open-image flow (CLI argument + native open dialog + WIC decode), and the fit-scaled image display. Landed for Wave 2 (branch `feat/wave-2-horizontal-scroll`, version 0.2.0, 2026-11-01): horizontal sibling-image navigation — the current-folder listing module, Left/Right keyboard navigation, and the on-screen corner navigation rails (mechanism: `## Horizontal Navigation Mechanism (Implemented, Wave 2)` below). Both waves are verified through the container harness (dev checks: `ALL CHECKS PASSED`, 26 unit tests; Windows x64 release cross-build: exit 0 → `dist/windows-x64/img-scroller.exe`). The remaining v1 features (vertical navigation, fullscreen, zoom, "Open with...", packaging) are planned. Facts are sourced from [brief.md](brief.md) (source of truth), the landed code, and the build guides ([docs/build.md](../../docs/build.md), [docs/build-windows.md](../../docs/build-windows.md)).
```

**T2 — line 26 (stack table version cell — stale `0.1.0`).**
Current: `| Language | Rust, edition 2024 | Cargo package `img-scroller` version 0.1.0 (single binary; tracked `Cargo.lock`). |`
Replacement: `| Language | Rust, edition 2024 | Cargo package `img-scroller` version 0.2.0 (single binary; tracked `Cargo.lock`). |`

**T3 — line 54 (failure-surface wording — the "bottom message bar" became the bottom rail's message area).**
Current:

```
- Failure classification: `WINCODEC_ERR_COMPONENTNOTFOUND` → `Unsupported`; every other error → `Unloadable`; both surface the single unsupported-file message (constant in `src/image_loading.rs`) in the window's bottom message bar.
```

Replacement:

```
- Failure classification: `WINCODEC_ERR_COMPONENTNOTFOUND` → `Unsupported`; every other error → `Unloadable`; both surface the single unsupported-file message (constant in `src/image_loading.rs`) in the bottom navigation rail's message area (the rails render only while an image is open — mechanism: `## Horizontal Navigation Mechanism (Implemented, Wave 2)`).
```

**T4 — new section, inserted BETWEEN the end of `## WIC Decode and Open-Dialog Mechanism (Implemented)` (after current line 54) and `## Development Environment and Setup` (current line 56). Insert EXACTLY (with a blank line before and after the heading block):**

```
## Horizontal Navigation Mechanism (Implemented, Wave 2)

Implemented in `src/folder_listing.rs` (pure listing logic, container-testable) plus `src/app.rs` / `src/app/rails.rs` (intake and on-screen controls):

- Listing scope: the sibling images of the opened image's folder ONLY — one non-recursive directory read of the opened image's parent; directories are never listed (nested subfolders are not entered).
- Image filter: a fixed 17-extension whitelist (`avif`, `bmp`, `dib`, `gif`, `heic`, `heif`, `ico`, `jfif`, `jpe`, `jpeg`, `jpg`, `jxr`, `png`, `tif`, `tiff`, `webp`, `wdp`), matched ignoring ASCII case. WIC codec availability is NOT probed — an extension-listed file the system cannot decode fails at open time through the normal failure path.
- Order (v1 fixed alphanumeric, no settings UI): file names compare by lowercase form first; names equal ignoring case tie-break by exact characters (e.g. `alpha.gif` < `ALPHA.jpg` < `Alpha.jpg` < `alpha.jpg`).
- Listing state: `ImagesScrollerApp.current_folder_listing` (RAM-only) — rebuilt from the new folder on EVERY successful open; `None` until the first open succeeds.
- Keyboard: `ArrowLeft` → previous, `ArrowRight` → next, evaluated per frame; the arrows navigate DIRECTLY — pan-then-navigate interplay is Wave-5 scope, not implemented here.
- Edge behavior: at the list's first/last position navigation stays silently (no wrap, no message); every "nothing to navigate to" case (no image open, no listing, unresolvable index, list beginning/end) resolves to no neighbor.
- Failure rule: opening a neighbor reuses the Wave-1 `open_image` seam — a failed decode keeps the currently displayed image and shows the unsupported-file message.
- On-screen rails (user-approved layout K3, confirmed 2026-10-31): a right rail (40 pt) with ↑/↓ folder arrows — visible but DISABLED in Wave 2 (Wave 3 activates folder navigation) — and a bottom rail (40 pt) with ←/→ image arrows plus the message area (it replaced the Wave-1 bottom message bar). A direction button is enabled exactly when the navigation seam resolves a neighbor. Both rails render ONLY while an image is open, so a failure message on the empty state is invisible by approved design.
```

**T5 — Contents TOC (lines 9–20): insert the new entry as item 5 and renumber the old items 5–12 to 6–13.** The TOC list becomes exactly:

```
1. [Technology Stack](#technology-stack)
2. [Image Format Support](#image-format-support)
3. [Dependencies (Landed, Wave 1)](#dependencies-landed-wave-1)
4. [WIC Decode and Open-Dialog Mechanism (Implemented)](#wic-decode-and-open-dialog-mechanism-implemented)
5. [Horizontal Navigation Mechanism (Implemented, Wave 2)](#horizontal-navigation-mechanism-implemented-wave-2)
6. [Development Environment and Setup](#development-environment-and-setup)
7. [Build Environment (Container on the Alpine VM)](#build-environment-container-on-the-alpine-vm)
8. [Build and Run](#build-and-run)
9. [Testing](#testing)
10. [Technical Constraints](#technical-constraints)
11. [Distribution and Packaging (Planned)](#distribution-and-packaging-planned)
12. [Tool Usage Patterns for AI Agents](#tool-usage-patterns-for-ai-agents)
13. [References](#references)
```

**T6 — line 88 (Testing census — stale 15-test breakdown).** Replace the whole bullet:
Current:

```
- 15 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3): `src/app.rs` — 4 (window-title constant; failed CLI open keeps no path and shows the message; successful open stores path/payload and clears the message; failure after success keeps the previous image); `src/image_loading.rs` — 4 (message wording; per-failure message mapping; pixel-buffer sizing; empty/overflow rejection); `src/app_display.rs` — 2 (initial texture state; upload happens exactly once per decode); `src/scaling.rs` — 5 (fit vectors ×4; zero-dimension rejection).
```

Replacement:

```
- 26 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3): `src/folder_listing/tests.rs` — 5 (case-insensitive sort with exact-name tie-break; folder-only enumeration incl. nested-subfolder exclusion; unlisted/missing index resolution; ±1 stepping with list-edge reports; empty/missing folder); `src/app/tests.rs` — 9 (window-title constant; failed CLI open keeps no path and shows the message; successful open stores path/payload and clears the message; failure after success keeps the previous image; empty-state navigation changes nothing; previous/next resolve in sort order; first/last position stays silent; failed neighbor open keeps the image + message; every successful open rebuilds the listing); `src/app/rails_tests.rs` — 1 (arrow-button enablement follows the navigation edges); `src/image_loading.rs` — 4 (message wording; per-failure message mapping; pixel-buffer sizing; empty/overflow rejection); `src/app_display.rs` — 2 (initial texture state; upload happens exactly once per decode); `src/scaling.rs` — 5 (fit vectors ×4; zero-dimension rejection).
```

(Testing bullets at lines 89–90 stay — still true. Do NOT add anything about `.gitattributes`/line endings to tech.md — the docs-split rule puts that in `docs/build.md` only, see §3.)

### 2.3 `.agent/project-info/architecture.md` — 11 edits (file >100 lines → TOC rule applies)

**A1 — line 5 (Status paragraph).** Replace the whole paragraph:
Current:

```
The planned v1 architecture below is partially **implemented** (Wave 1, branch `feat/wave-1-skeleton-open-image`, 2026-10-31): the window shell, the open-image flow (CLI argument + native dialog + WIC decode), and the fit-scaled display exist in `src/` and are verified through the container harness. Navigation, fullscreen, zoom, and "Open with..." remain planned. Facts are sourced from [brief.md](brief.md) (source of truth), the landed code, and [tech.md](tech.md).
```

Replacement:

```
The planned v1 architecture below is partially **implemented** (Wave 1, branch `feat/wave-1-skeleton-open-image`, 2026-10-31): the window shell, the open-image flow (CLI argument + native dialog + WIC decode), and the fit-scaled display exist in `src/` and are verified through the container harness. Wave 2 (branch `feat/wave-2-horizontal-scroll`, version 0.2.0, 2026-11-01) landed the horizontal sibling-image navigation slice: the current-folder listing module, Left/Right keyboard navigation, and the on-screen corner navigation rails. Vertical navigation (Wave 3), fullscreen, zoom, and "Open with..." remain planned. Facts are sourced from [brief.md](brief.md) (source of truth), the landed code, and [tech.md](tech.md).
```

**A2 — line 11 (Contents TOC entry 3 — heading is renamed in A5).**
Current: `3. [Module Layout (Implemented, Wave 1)](#module-layout-implemented-wave-1)`
Replacement: `3. [Module Layout (Implemented)](#module-layout-implemented)`

**A3 — line 28 (cross-reference before the component list).**
Current: `The decomposition below is derived from the v1 features; each component carries a landed/planned annotation. The realized module layout is in `## Module Layout (Implemented, Wave 1)`.`
Replacement: `The decomposition below is derived from the v1 features; each component carries a landed/planned annotation. The realized module layout is in `## Module Layout (Implemented)`.`

**A4 — line 30 (Viewer UI component bullet — message bar → corner rails).** Replace the whole bullet:
Current:

```
- **Viewer UI (egui/eframe, immediate-mode) — Wave-1 slice LANDED**: regular resizable window (title `Images Scroller`, not maximized at launch); top "Open…" toolbar button; bottom message bar; central image surface painted fit-scaled and centered. Remaining UI (navigation buttons, fullscreen toggle, zoom controls, "Open with..." menu entry): planned.
```

Replacement:

```
- **Viewer UI (egui/eframe, immediate-mode) — Wave-1 slice LANDED, Wave-2 rails LANDED**: regular resizable window (title `Images Scroller`, not maximized at launch); top "Open…" toolbar button; central image surface painted fit-scaled and centered; while an image is open, two corner navigation rails render around it — a bottom rail (previous/next image arrows + message area) and a right rail (up/down folder arrows, disabled until Wave 3). Remaining UI (activating the folder arrows, fullscreen toggle, zoom controls, "Open with..." menu entry): planned.
```

**A5 — line 32 (Navigation Engine component bullet — planned → landed slice).** Replace the whole bullet:
Current:

```
- **Navigation Engine — planned**: horizontal — previous/next sibling image in the current folder (alphanumeric order); vertical — previous/next sibling folder under the SAME parent (target image priority: (a) session last-viewed image for folders already visited this session, (b) otherwise ALWAYS the target folder's first image in alphanumeric order).
```

Replacement:

```
- **Navigation Engine — horizontal slice LANDED (Wave 2); vertical slice planned (Wave 3)**: horizontal — previous/next sibling image in the current folder (listing: non-recursive parent-folder read, 17-extension whitelist; order: lowercase-folded alphanumeric with exact-name tie-break; list edges stay silently); vertical — previous/next sibling folder under the SAME parent (target image priority: (a) session last-viewed image for folders already visited this session, (b) otherwise ALWAYS the target folder's first image in alphanumeric order).
```

**A6 — line 38 (Module Layout heading).**
Current: `## Module Layout (Implemented, Wave 1)`
Replacement: `## Module Layout (Implemented)`

**A7 — line 43 (`src/app.rs` module bullet) — update + append the Wave-2 wiring and child modules.** Replace the whole bullet:
Current:

```
- `src/app.rs` — application shell: `ImagesScrollerApp` state (opened path, decoded payload, message text, display), the `open_image` flow (decode → state transition; failure changes only the message), the top "Open…" button wiring, the `APP_WINDOW_TITLE` constant, and `NativeOptions` (resizable, not maximized).
```

Replacement:

```
- `src/app.rs` — application shell: `ImagesScrollerApp` state (opened path, in-session folder listing, decoded payload, message text, display), the `open_image` flow (decode → state transition; failure changes only the message), the Left/Right keyboard navigation and the shared `navigate(NavigationDirection)` seam, the top "Open…" button wiring, the `APP_WINDOW_TITLE` constant, and `NativeOptions` (resizable, not maximized). Child modules: `src/app/rails.rs` (the corner navigation rails), `src/app/tests.rs`, `src/app/rails_tests.rs`.
```

**A8 — insert two NEW module bullets immediately after the (replaced) `src/app.rs` bullet, before the `src/image_loading.rs` bullet:**

```
- `src/folder_listing.rs` — pure-logic current-folder listing (child module `src/folder_listing/tests.rs`): one non-recursive read of the opened image's parent folder, a fixed 17-extension image whitelist (case-insensitive match), lowercase-folded name sort with exact-name tie-break, and ±1 neighbor resolution that reports the list's beginning/end (mechanism details: [tech.md](tech.md) → `## Horizontal Navigation Mechanism (Implemented, Wave 2)`).
- `src/app/rails.rs` — on-screen corner navigation rails: right rail (↑/↓ folder arrows, disabled in Wave 2 — Wave-3-reserved), bottom rail (←/→ image arrows + the message area); a direction button is enabled exactly when the navigation seam resolves a neighbor; both rails render only while an image is open.
```

**A9 — line 54 (open/display flow step 4 — message surface wording).** Replace the whole numbered item:
Current:

```
4. Failure: `Unsupported`/`Unloadable` → the bottom message bar shows the unsupported-file message; a previously loaded image and the rest of the state stay untouched.
```

Replacement:

```
4. Failure: `Unsupported`/`Unloadable` → the bottom rail's message area shows the unsupported-file message (the rail renders only while an image is open); a previously loaded image and the rest of the state stay untouched.
```

**A10 — lines 58 and 61 (Navigation Critical Paths intro + Path 2).**
Line 58 current: `Path 1's open/decode/display slice is implemented (Wave 1); the navigation, pan/zoom, and "Open with..." parts of all paths remain planned.`
Line 58 replacement: `Path 1's open/decode/display slice (Wave 1) and Path 2's horizontal sibling-image navigation (Wave 2) are implemented; the vertical-navigation, pan/zoom, and "Open with..." parts remain planned.`

Line 61 current:

```
2. **Horizontal navigation**: arrow key or UI button → previous/next sibling image in the current folder (alphanumeric order) → decode → display; while the image is wider than the window, pan first; navigation only at the image's left/right edge.
```

Line 61 replacement:

```
2. **Horizontal navigation (Wave-2 slice LANDED)**: arrow key or UI button → previous/next sibling image in the current folder (listing: 17-extension whitelist; order: lowercase-folded alphanumeric with exact-name tie-break; list edges stay silently) → decode → display; arrows navigate directly at this wave — pan-then-navigate (navigation only at the zoomed image's left/right edge) lands in Wave 5.
```

**A11 — lines 87–88 (Project Paths bullets — module census + cross-reference).**
Line 87 current: `- Repository layout: application code in `src/` (five modules plus the `platform/` facade), harness in `Dockerfile`/`docker-compose.yml`/`scripts/`, guides in `docs/`, agent scaffolding in `.agent/`/`.kilo/`/`.opencode/`.`
Line 87 replacement: `- Repository layout: application code in `src/` (the `app` shell with its `rails` child module, `app_display`, `folder_listing`, `image_loading`, `scaling`, plus the `platform/` facade), harness in `Dockerfile`/`docker-compose.yml`/`scripts/`, guides in `docs/`, agent scaffolding in `.agent/`/`.kilo/`/`.opencode/`.`

Line 88 current: `- Module layout: decided and implemented — see `## Module Layout (Implemented, Wave 1)`.`
Line 88 replacement: `- Module layout: decided and implemented — see `## Module Layout (Implemented)`.`

**A12 — line 113 (Open Architectural Decisions first bullet — the horizontal design is no longer open).** Replace the whole bullet:
Current: `- Navigation-engine design: horizontal sibling-image and vertical sibling-folder flow, scan/ordering and state transitions (Wave 2).`
Replacement: `- Vertical-navigation engine design: sibling-folder flow, target-image priority state, and the session last-viewed memory (Wave 3). The horizontal sibling-image slice is decided and landed (Wave 2).`

(architecture.md "Data and State" line 68 is deliberately NOT changed: it is not false — the RAM-only state list is Wave-1-scoped and the listing state shape is documented in tech.md's new section. Line 75 "Build and Release Pipeline" stays: it is a true Wave-1 historical record.)

### 2.4 `.agent/project-structure.md` — full replacement (stale file map: missing 5 files, wrong app.rs description)

Replace the ENTIRE file content with EXACTLY:

```
# Project Structure

# Files in src/

- app.rs - eframe application shell: viewer window options, open-flow state, top Open… button, Left/Right keyboard navigation and the shared navigate seam, in-session folder-listing state, window-title constant
- app_display.rs - central image display: texture lifecycle wrapper (one upload per decode), fit-scaled paint, pending-upload flag
- folder_listing.rs - current-folder image listing for horizontal navigation: one non-recursive parent-folder read, 17-extension image whitelist, lowercase-folded sort with exact-name tie-break, ±1 neighbor stepping with list-edge reports
- image_loading.rs - open-image seam: load_image entry, DecodedImage payload shape, failure classification, unsupported-file message constant
- main.rs - entry point: CLI image-path intake, module registration, starts the eframe native app
- scaling.rs - pure fit-scaling math: largest uniform factor for the available area (no egui painting)

# Folders in src/

- app/ - child modules of the application shell: rails.rs (corner navigation rails: right rail with disabled up/down folder arrows, bottom rail with image arrows + message area), tests.rs (shell/navigation unit tests), rails_tests.rs (rail button enablement tests)
- folder_listing/ - child module: tests.rs (listing/collation/neighbor unit tests)
- platform/ - OS-bound facade for the native open dialog and WIC decode: mod.rs (facade re-exports), windows.rs (Windows implementation), stub.rs (non-Windows fallback)

# Other folders

- .agent/ - agent context: project-info/, todos/, rules/workflow indexes and the structure map
- .kilo/ - Kilo Code integration: agents/, rules/, commands/ and plans/
- .opencode/ - opencode integration: agents/, commands/ and opencode.json
- docs/ - Documentation files
- scripts/ - container harness scripts: dev-checks and Windows x64 cross-build (POSIX sh)

# Other files (repository root)

- .gitattributes - line-ending policy: `*.sh text eol=lf` forces LF working-tree checkouts for the shell scripts (added after a CRLF Windows checkout broke the container dev-checks)
```

(The `scripts/` row text itself is unchanged — the LF enforcement is explained once, in the new `.gitattributes` line here and operationally in `docs/build.md`; no duplication.)

### 2.5 `docs/build.md` — 1 insertion (the routed `.gitattributes`/CRLF note, 2–4 lines, no duplication)

Append at the END of the file (after the `## Lockfile policy` section, as a new final section):

```
## Shell-script line endings

The harness scripts must reach the container with LF line endings: the repository pins `*.sh text eol=lf` in `.gitattributes`, so every checkout materializes them with LF. (Added after a CRLF Windows checkout made `sh` abort with `set: Illegal option -`; if a working tree regains CRLF endings, renormalize the two scripts before running the harness.)
```

(Nothing else in docs/build.md changes.)

### 2.6 `.agent/project-info/context.md` — closing update (4 section operations; TODO Task-5 content deliverable)

Apply in this order. The pre-existing unstaged ` M` content is the current working-tree text quoted below — it is absorbed by these operations and committed in this step (caller-approved: context.md IS staged in Task 5).

**C1 — replace the ENTIRE "## Current Work Focus" paragraph (current line 15) with:**

```
Dated 2026-11-01. **Wave 2 (`.agent/todos/20261029/20261029-todo-4.md`) horizontal scroll is landing on `feat/wave-2-horizontal-scroll`**: Tasks 1–4 are COMPLETE (commits `6615d0a` + review fix `43b8c44`, version bump `69e8925` to 0.2.0, keyboard nav `d1a85d5`, corner rails `4b45ca7` + fixes `6f00e6b`/`f4317c4`, harness CRLF repair `cf244d5`, verification `85007ce`), and Task 5 (documentation updates) is the current step. Landed capability: the viewer lists the opened image's sibling images (same folder only, non-recursive, 17-extension whitelist, lowercase-folded alphanumeric order with exact-name tie-break) and navigates previous/next via the Left/Right arrow keys and the on-screen corner rails (user-approved layout K3, confirmed 2026-10-31: bottom rail 40 pt with ←/→ image arrows + message area; right rail 40 pt with ↑/↓ folder arrows disabled until Wave 3; rails hidden while no image is open). List edges stay silently (no wrap); a failed neighbor open keeps the displayed image and shows the message. Verification: 26 unit tests, dev-checks `ALL CHECKS PASSED`, exe rebuilt (`dist/windows-x64/img-scroller.exe`, 19,464,396 bytes), DLL import audit unchanged (22 OBSERVED DLLs); `.gitattributes` now enforces `*.sh text eol=lf` (commit `cf244d5`) after the CRLF Windows checkout broke the container dev-checks. **The user confirmed (reported 2026-11-01) that the Wave-1 exe OPERATOR-RUN worked as expected** — Wave 1's manual-UX item is closed. Remaining: the Wave-2 OPERATOR-RUN (below), then the Step-5 merge/push.
```

**C2 — insert ONE new entry as the FIRST item of "## Recent Changes" (immediately after the `## Recent Changes` heading line), with the literal number `1.`; change NOTHING else in the section (existing entries keep their text; their printed numbers need no edits — markdown renders ordered lists sequentially):**

```
1. 2026-11-01: **Wave 2 Tasks 1–4 landed on `feat/wave-2-horizontal-scroll`** (commits: `6615d0a` folder-listing module + `43b8c44` review fix; `69e8925` version 0.2.0; `d1a85d5` Left/Right keyboard navigation; `4b45ca7` corner navigation rails per user-approved layout K3 (confirmed 2026-10-31) + `6f00e6b`/`f4317c4` review fixes; `cf244d5` `.gitattributes` `*.sh text eol=lf` after the CRLF Windows checkout broke the container dev-checks (both scripts repaired to LF on the VM); `85007ce` Task-4 verification + plan artifact). Dev checks `ALL CHECKS PASSED` (26 unit tests: folder_listing 5, app 9, rails 1, image_loading 4, app_display 2, scaling 5); Windows x64 exe rebuilt (`dist/windows-x64/img-scroller.exe`, 19,464,396 bytes); DLL import audit unchanged (22 OBSERVED DLLs). **User confirmed the Wave-1 exe OPERATOR-RUN worked as expected** (reported 2026-11-01). Task 5 (documentation: README/tech/architecture/structure/build-docs/context) is in progress this session.
```

**C3 — replace the ENTIRE "## Current Project State" bullet list (current lines 34–42, all 9 bullets) with EXACTLY these 7 bullets:**

```
- **Wave 1 landed on `main`** (`92dc22c`, pushed to `origin`): container harness + application skeleton (open flow, WIC decode, fit display). **The user confirmed the Wave-1 exe OPERATOR-RUN worked as expected (reported 2026-11-01).**
- **Wave 2 landed on `feat/wave-2-horizontal-scroll` (version 0.2.0)**: `src/folder_listing.rs` + `src/folder_listing/tests.rs`; Left/Right keyboard navigation + shared `navigate` seam in `src/app.rs`; corner rails `src/app/rails.rs` (+ `src/app/tests.rs`, `src/app/rails_tests.rs`); 26 unit tests; dev-checks `ALL CHECKS PASSED`; `.gitattributes` enforces `*.sh text eol=lf` (`cf244d5`).
- Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,464,396 bytes, Wave-2 rebuild 2026-11-01); import audit: 22 OBSERVED DLLs recorded in `scripts/build-windows.sh` + `docs/build-windows.md`.
- **TODO state**: `.agent/todos/20261029/20261029-todo-4.md` — Tasks 1–4 `[DONE]`/`[x]` (Task 4's OPERATOR-RUN sub-item stays unchecked until the user reports); Task 5 (documentation) in progress; then Step 5 (rename to `-DONE`, merge to `main`, push `origin` ONLY).
- Wave TODO files ready: `20261029-todo-5.md` (Wave 3) … `todo-9` (Wave 7); process ONE wave per session, in order.
- Binding facts unchanged: all Rust/Cargo inside the container on the Alpine VM (`alpine-vm` MCP: `vm_status` first; exit code authoritative; vboxsf EBUSY on container manifest writes → authorized host-side write fallback is the documented pattern); host needs NO Rust/Cargo; GUI/runtime validation is OPERATOR-RUN on real Windows 10/11.
- Open item (flagged to the user, NOT a TODO now): non-script text files (`Dockerfile`, `docker-compose.yml`, `Cargo.toml`, `*.rs`) are still `i/lf w/crlf` in the Windows working tree — only `*.sh` is LF-enforced by `.gitattributes`; broader renormalization would need a dedicated future plan.
- Git: branch `feat/wave-2-horizontal-scroll` (Tasks 1–4 through `85007ce`, plus this step's docs commit); `main` == `origin/main` == `92dc22c`; merge/push happen at Step 5.
```

**C4 — replace the ENTIRE "## Immediate Next Steps" numbered list (current lines 46–50, all 5 items) with EXACTLY these 4 items:**

```
1. **Wave-2 OPERATOR-RUN (real Windows 10/11 machine)** — run `dist\windows-x64\img-scroller.exe` with the test folder from `.kilo/plans/20261031-wave2-task4-verification.md` §5 (`alpha.gif`, `ALPHA.jpg`, `Alpha.jpg`, `alpha.jpg`, `beta.jpg`, `Beta.png`, `zeta.webp`, `notes.txt`, subfolder `sub` containing `inner.png`): (1) start the exe, click `Open…`, open `beta.jpg`; (2) the LEFT arrow key AND the bottom-rail `←` button go to the PREVIOUS image, the RIGHT arrow key AND `→` go to the NEXT image; (3) travel order matches the collation (`alpha.gif` → `ALPHA.jpg` → `Alpha.jpg` → `alpha.jpg` → `beta.jpg` → `Beta.png` → `zeta.webp`); (4) `sub\inner.png` never appears (nested subfolders NOT entered); (5) at the first/last image navigation is inert and the respective rail button shows disabled (stay, silent, no wrap); (6) the rails and the message area exist only while an image is open. Report results back into the project (Context or TODO).
2. **Finish the Wave-2 Critical Workflow** — Step 5: rename the TODO to `20261029-todo-4-DONE.md` (content unchanged), merge `feat/wave-2-horizontal-scroll` to `main`, push `main` to `origin` ONLY; then the Planner's Step-6 closing (final `context.md` polish: merge/push hashes + the OPERATOR-RUN result).
3. **Next session: Wave 3** ([`.agent/todos/20261029/20261029-todo-5.md`](../todos/20261029/20261029-todo-5.md)) — vertical scroll (sibling folders); the right rail's ↑/↓ buttons are already in place, disabled until then; process in a NEW session.
4. **Open item (flagged, not a TODO)**: broader line-ending renormalization beyond `*.sh` (e.g. `Dockerfile`, `*.rs` are `i/lf w/crlf` in the Windows working tree) — flagged to the user; a dedicated future plan would own it.
```

(Keep the "## How to Use This File" section and everything above "## Current Work Focus" untouched. Historical "Recent Changes" entries keep their text — including their historical "bottom message bar" wording, which is a true record of Wave-1-era state.)

### 2.7 `docs/build-windows.md` — verified NO edit

Checked every line against the landed Wave-2 state: invocation, path, overwrite behavior, failure table, toolchain notes, the 22-DLL observed list, and both policy sections remain true. The audit paragraph is a correctly-scoped historical record ("first produced exe"). No edit — do not touch this file.

## 3. Docs-split / TOC / style compliance (encoded decisions)

- **Docs-split rule**: the `.gitattributes`/CRLF note lives ONLY in `docs/build.md` (§2.5); `tech.md` gets NO line-endings content. The DLL list stays ONLY in `docs/build-windows.md` + the script (tech.md/architecture.md keep their existing no-duplication references). The navigation mechanism details live in tech.md; architecture.md links to them instead of restating them.
- **TOC/index rule**: files >100 lines (`tech.md`, `architecture.md`) keep their `## Contents` indexes consistent with the renamed/new headings (T5, A2). `README.md` (<100 lines, inline anchors only) — the `[Project Status](#project-status)` anchor target heading `## Project Status` is unchanged by R3.
- No commented-out content, no duplication of brief.md text — docs reference the brief instead of restating requirements.

## 4. Order of operations for 4.2

1. **docs-specialist** applies edits in this order, verifying each against §2: README (R1→R3) → tech.md (T1→T6) → architecture.md (A1→A12) → project-structure.md (full replacement) → docs/build.md (append) → context.md (C1→C4). Use the dedicated read/edit tools only; never PowerShell; no other files.
2. **docs-specialist** runs the §6 verification greps and reports the result with the completion summary (what was done / what was NOT done).
3. **implementer** performs the §5 git plan (staging + ONE commit). The implementer does NOT edit any doc file.
4. **4.3 code-reviewer** reviews the commit against this plan (docs-adherence review). If findings require fixes: a NEW fix plan file, then docs-specialist applies + implementer commits with message `docs: address Task 5 review findings` (only if required; max 3 cycles per workflow).
5. **4.5b architector** adherence report; **4.6 implementer** marks Task 5 `[DONE]` in the TODO and commits (TODO-file commit is separate and later).

## 5. Git plan (implementer, after step 4.2 edits + §6 checks pass)

Read `.gitignore` first (gitignore-compliance rule) and confirm none of the staged paths match ignore patterns. Single commands, in order (no `&&` chains, no `git add -A`/`git add .`):

1. `git status --short` — expect exactly the 7 paths of §1 as `M`/`??` (plus nothing else; `dist/`, `logs/`, `target/`, `*.log` must not appear).
2. `git add README.md`
3. `git add docs/build.md`
4. `git add .agent/project-info/tech.md`
5. `git add .agent/project-info/architecture.md`
6. `git add .agent/project-info/context.md` — YES, staged in this step (TODO Task-5 content deliverable; caller-approved override of the Task-4-era "never stage context.md" note, which applied to Task 4's commits only).
7. `git add .agent/project-structure.md`
8. `git add .kilo/plans/20261031-wave2-task5-documentation.md`
9. `git diff --cached --name-only` — must equal EXACTLY the 7 paths; `CHANGELOG.md` must NOT appear; no `.gitignore`-matching path may appear (unstage if one does).
10. `git commit -m "docs: land Wave 2 horizontal scroll documentation (README/tech/architecture/context)"`
11. `git status --short` — expect empty/clean for tracked content.

NO `git push` (restricted to Step 5, `main` → `origin` ONLY). NO branch creation/switch. NO TODO-file edits in this commit.

## 6. Post-edit verification checklist (docs-specialist, before the commit)

- `git grep -n "bottom message bar" -- README.md .agent/project-structure.md .agent/project-info/tech.md .agent/project-info/architecture.md` → **0 matches** (the only allowed remaining occurrences are historical records inside `context.md` Recent Changes).
- `git grep -n "0.1.0" -- .agent/project-info` → **0 matches**.
- `git grep -n "15 unit tests" -- .agent/project-info` → **0 matches**.
- `git grep -n "Module Layout (Implemented, Wave 1)" -- .agent/project-info` → **0 matches**.
- tech.md TOC link `#horizontal-navigation-mechanism-implemented-wave-2` and architecture.md link `#module-layout-implemented` match their headings exactly (visual check).
- The §2.4 replacement left project-structure.md with exactly 5 files, 3 src folders, 5 other folders, 1 other file.

## 7. Note for 4.5b / Step 6 (context.md stays "accurate-but-finalizable")

This step's context.md update is complete and accurate for the current state, but is deliberately NOT final: at Step 6 the **Planner** performs the final polish (adds the merge/push hashes, the Wave-2 OPERATOR-RUN result, and re-checks "Current Work Focus"). The docs-specialist/implementer must NOT invent merge/push hashes, must not pre-mark Task 5 `[DONE]` in context.md, and must not rename the TODO file (Step 5 duty).

## 8. Hard exclusions

- NO `CHANGELOG.md` touch (wave binding). NO `brief.md`/`product.md` edits. NO `docs/build-windows.md` edit (§2.7). NO pre-existing `.kilo/plans/*` edits.
- NO `src/`, `Cargo.toml`, `Cargo.lock`, script, or Dockerfile/compose edits.
- NO `git push`, no branch operations, no version bump (`0.2.0` already landed).
- NO TODO-file `[DONE]` marks in this step (4.6 duty).
- NO invented facts: every doc statement in §2 traces to a verified file/line in §0; anything unverifiable was left out.

## 9. Stop conditions → user question

- Any §2 "current" quote does not match the file exactly (file drifted since plan time).
- §6 grep results differ from expectations.
- `git diff --cached --name-only` shows anything outside the 7-path list, or `CHANGELOG.md`/gitignored paths appear.
- Working tree contains unexpected entries in `git status --short` beyond the 7 paths.
