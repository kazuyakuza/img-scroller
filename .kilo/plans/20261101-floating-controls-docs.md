# Docs-Change Plan — Wave-2 UI Fix Cycle C (TODO Task 5): Landed Floating-Controls Documentation

Date: 2026-11-01
Workflow step: Cycle C → Step 4.1b (architector)
Source TODO: [`.agent/todos/20261101/20261101-todo-1.md`](../../.agent/todos/20261101/20261101-todo-1.md) → **Task 5 (documentation updates)** + Binding notes (no `CHANGELOG.md`; new plan files only)
Global plan: [`.kilo/plans/20261101-wave2-ui-restyle.md`](../../.kilo/plans/20261101-wave2-ui-restyle.md) §2.2 (Task 5 = Cycle C, no 4.1a, 4.5b only), §3 Cycle C step map
Implementation plan (Cycle A): [`.kilo/plans/20261101-floating-controls.md`](../../.kilo/plans/20261101-floating-controls.md) (pinned anatomy facts)
Verification plan (Cycle B): [`.kilo/plans/20261101-floating-controls-verification.md`](../../.kilo/plans/20261101-floating-controls-verification.md) (census 26; exe rebuild; 22-DLL audit)
Approval: user AUTO-APPROVED per-task plans (approval mode 2026-11-01) — every decision below is DECISIVE; zero judgment remains for the docs-specialist/implementer.

## 0. Scope Guard

This step ships EXACTLY: wording updates in `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`, `.agent/project-structure.md` — plus the Task-5 `[DONE]` marks and ONE commit.

- NO `CHANGELOG.md` edits (wave binding). NO `brief.md`/`product.md` edits.
- NO edits to existing `.kilo/plans/*` files (new plan files only).
- NO edits to `docs/build.md` (fine as-is) or `docs/build-windows.md` — verified: it contains NO exe-size claim and NO current-UI claim; its DLL-audit wording is explicitly historical ("Audited 2026-10-31 … Wave 1 verification; first produced exe"). LEAVE UNTOUCHED.
- No source-code edits, no cargo, no TODO content edits beyond the mark-only Task-5 flips (§6).
- `git push` NEVER here (workflow Step 5, `main` → `origin` ONLY).

## 1. Landed state (git- and code-verified 2026-11-01 — every fact below is evidence-backed)

Git: branch `fix/centered-floating-controls`; HEAD `9a70279`; local `main` = `d82ec38`; `origin/main` = `de332fc`; working tree clean EXCEPT ` .agent/project-info/context.md` modified-unstaged (Cycle-A upkeep — planner territory; the closing update in §2.5 supersedes its stale interim lines).

Fix commit chain (top of `main`, i.e. `d82ec38..9a70279`):

| Commit | Role |
| --- | --- |
| `d82ec38` | UI-fix TODO + global plan (operator feedback) |
| `10d0253` | version 0.2.1 |
| `6ef0240` | feature: floating pill + center Open; `src/app.rs` rewritten; `src/app/rails.rs` + `src/app/rails_tests.rs` deleted |
| `f30b802` | review fix: `Area::show(ctx)` receiver, `Color32::from_rgba_unmultiplied_const`, `Id::unique` ids |
| `f7566cb` | TODO Tasks 1–3 marks + 4.5a frontend verification report |
| `9a70279` | TODO Task 4 marks + Cycle-B verification plan |

Landed UI facts (source: `src/app.rs` 192 L, `src/app/floating_controls.rs` 170 L landed code):

- ONE overlay `egui::Area` with id string `"floating_navigation_pill"`, `.anchor(Align2::CENTER_BOTTOM, vec2(0.0, -24.0))`, `.order(Order::Foreground)`; shown via `show_floating_pill` → `show(...ctx(), …)`.
- ONE translucent-dark `Frame`: fill `rgba(20,20,20,180)`, corner radius 10, inner margin 8. Contents, top-down: message label (12 pt, `message.unwrap_or("")` — shows only when `message_text` is `Some`) ABOVE one left-to-right row with order `[←] [→] (16-pt gap) [↑] [↓]`.
- All four arrows are PAINTED vector triangles via `egui::Shape::rotated_triangle` (10×10 inside 36×32 click rects) — zero font dependency; this replaces the Wave-2 (0.2.0) ←/→/↑/↓ glyph labels that rendered as tofu boxes on Windows.
- Enablement (STAY parity, per frame, stateless): `navigation_button_enabled(app, direction)` == `app.neighbor_image_path(direction).is_some()`; ↑/↓ resolve disabled structurally (`navigation_direction()` = `None` — Wave-3 reserves).
- A button's click calls EXACTLY `app.navigate(NavigationDirection::Previous/Next)`; keyboard intake (`ArrowLeft`/`ArrowRight` frame-first in `ui()`) is byte-unchanged.
- Empty state: empty `CentralPanel` containing ONE `egui::Button` "Open…" (text 18 pt, min size 180×48) inside an `Area` with id `"empty_state_open_button"`, `.anchor(Align2::CENTER_CENTER, Vec2::ZERO)`; click → `self.open_via_dialog(frame)`. With an image open there is NO Open control anywhere (top toolbar `open_toolbar`/`open_toolbar_button` retired fully; `grep` over `src/` shows zero `rails`/`open_toolbar` remains).
- Stateless-Area/accessibility fact (VERIFIED REAL — record it, §2.3): both overlay Areas use per-call `egui::Id::unique(...)` ids (landed in `f30b802`; `src/app/floating_controls.rs:34`, `src/app.rs:138`) — stateless overlays with no remembered window/layout state; the painted buttons expose AccessKit info via `WidgetInfo::labeled(Role::Button, enabled, label)` (`floating_controls.rs:124–130`). Disposition DECIDED: encoded as one sentence in the `architecture.md` Viewer-UI bullet; NOT an entry in Open Architectural Decisions (it is a landed fact, not an undecided item).
- Tests: 26 (census unchanged) — `src/app/tests.rs` 9 (git-verified untouched) + `src/app/floating_controls_tests.rs` 1 (the relocated `rails_tests.rs` enablement test; rename verified by git diff) + `src/folder_listing/tests.rs` 5 + `src/image_loading.rs` 4 + `src/app_display.rs` 2 + `src/scaling.rs` 5.
- Exe: `dist/windows-x64/img-scroller.exe` rebuilt at 0.2.1 (Cycle B) = **19,461,520 bytes**; DLL import audit unchanged (22 OBSERVED DLLs).
- Operator report (TODO origin): tofu icons + rails/toolbar layout rejection → this fix; re-test PENDING.

## 2. Per-file edits (docs-specialist applies verbatim; old→new strings are exact)

### 2.1 `README.md` — 3 edits

**Edit 1 — feature bullet 2 (the "v1 Features" list).** REPLACE (old):

> 2. **Horizontal Scroll (Sibling Images)** — left/right arrow keys and the on-screen bottom-rail ←/→ buttons navigate to the previous/next sibling image in the same folder; the scope is strictly the current folder (not recursive); the order is alphanumeric — names compare case-insensitively, with an exact-character tie-break when names differ only by case; at the first/last image the viewer stays on it (no wrap-around). The navigation rails appear while an image is open; the right rail's up/down folder arrows are visible but disabled until folder navigation lands.

WITH (new):

> 2. **Horizontal Scroll (Sibling Images)** — left/right arrow keys and the floating bottom pill's ←/→ buttons navigate to the previous/next sibling image in the same folder; the scope is strictly the current folder (not recursive); the order is alphanumeric — names compare case-insensitively, with an exact-character tie-break when names differ only by case; at the first/last image the viewer stays on it (no wrap-around). The floating bottom pill appears while an image is open; its ←/→/↑/↓ arrows are drawn as painted vector triangles (never font glyphs — they render identically on any Windows system regardless of installed fonts), and its up/down folder arrows are visible but disabled until folder navigation lands.

**Edit 2 — Project Status paragraph.** REPLACE (old):

> Wave 2 landed (branch `feat/wave-2-horizontal-scroll`, version 0.2.0): horizontal sibling-image navigation — Left/Right arrow keys and the on-screen bottom-rail ←/→ buttons move to the previous/next image of the opened image's folder in case-insensitive alphanumeric order.

WITH (new):

> Wave 2 landed (branch `feat/wave-2-horizontal-scroll`, version 0.2.0): horizontal sibling-image navigation — Left/Right arrow keys move to the previous/next image of the opened image's folder in case-insensitive alphanumeric order. A 0.2.1 UI fix (2026-11-01, branch `fix/centered-floating-controls`) rebuilt the on-screen controls per operator feedback: one centered floating bottom pill (translucent dark frame, painted vector-triangle arrows — ←/→ navigate, ↑/↓ reserved and disabled) replaces the Wave-2 corner rails and the Wave-1 top toolbar, and the empty state shows one big centered Open button (no Open control while an image is open); the operator re-test of the rebuilt exe is pending.

(The rest of the Status paragraph — "Wave 1 landed…", "Verification so far: container dev checks (`ALL CHECKS PASSED`, 26 unit tests)…", the brief links — stays byte-identical.)

**Edit 3 — Technology Stack table, Language row.** REPLACE (old):

> | Language | Rust | Implemented: Cargo package `img-scroller` 0.2.0, edition 2024. |

WITH (new):

> | Language | Rust | Implemented: Cargo package `img-scroller` 0.2.1, edition 2024. |

### 2.2 `.agent/project-info/tech.md` — 6 edits

**Edit 1 — Status paragraph.** REPLACE (old, the second sentence's tail into the third sentence — exact span):

> Landed for Wave 2 (branch `feat/wave-2-horizontal-scroll`, version 0.2.0, 2026-11-01): horizontal sibling-image navigation — the current-folder listing module, Left/Right keyboard navigation, and the on-screen corner navigation rails (mechanism: `## Horizontal Navigation Mechanism (Implemented, Wave 2)` below). Both waves are verified through the container harness (dev checks: `ALL CHECKS PASSED`, 26 unit tests; Windows x64 release cross-build: exit 0 → `dist/windows-x64/img-scroller.exe`).

WITH (new):

> Landed for Wave 2 (branch `feat/wave-2-horizontal-scroll`, version 0.2.0, 2026-11-01): horizontal sibling-image navigation — the current-folder listing module and Left/Right keyboard navigation (mechanism: `## Horizontal Navigation Mechanism (Implemented, Wave 2)` below). Landed for the Wave-2 UI follow-up (branch `fix/centered-floating-controls`, version 0.2.1, 2026-11-01): the on-screen controls were rebuilt as ONE centered floating bottom pill with painted vector arrows plus an empty-state center Open button; the 0.2.0 corner rails and the Wave-1 top Open… toolbar were removed. Every landed slice is verified through the container harness (dev checks: `ALL CHECKS PASSED`, 26 unit tests; Windows x64 release cross-build: exit 0 → `dist/windows-x64/img-scroller.exe`).

**Edit 2 — Technology Stack table, Language row.** REPLACE (old):

> | Language | Rust, edition 2024 | Cargo package `img-scroller` version 0.2.0 (single binary; tracked `Cargo.lock`). |

WITH (new):

> | Language | Rust, edition 2024 | Cargo package `img-scroller` version 0.2.1 (single binary; tracked `Cargo.lock`). |

**Edit 3 — WIC mechanism section, failure-classification bullet (message-surface wording).** REPLACE (old tail):

> both surface the single unsupported-file message (constant in `src/image_loading.rs`) in the bottom navigation rail's message area (the rails render only while an image is open — mechanism: `## Horizontal Navigation Mechanism (Implemented, Wave 2)`).

WITH (new tail):

> both surface the single unsupported-file message (constant in `src/image_loading.rs`) in the floating navigation pill's message line (the pill renders only while an image is open — mechanism: `## Horizontal Navigation Mechanism (Implemented, Wave 2)`).

**Edit 4 — Horizontal Navigation Mechanism section, intro line.** REPLACE (old):

> Implemented in `src/folder_listing.rs` (pure listing logic, container-testable) plus `src/app.rs` / `src/app/rails.rs` (intake and on-screen controls):

WITH (new):

> Implemented in `src/folder_listing.rs` (pure listing logic, container-testable) plus `src/app.rs` / `src/app/floating_controls.rs` (intake and on-screen controls):

**Edit 5 — Horizontal Navigation Mechanism section, final bullet (rail → pill wording).** REPLACE (old):

> - On-screen rails (user-approved layout K3, confirmed 2026-10-31): a right rail (40 pt) with ↑/↓ folder arrows — visible but DISABLED in Wave 2 (Wave 3 activates folder navigation) — and a bottom rail (40 pt) with ←/→ image arrows plus the message area (it replaced the Wave-1 message surface). A direction button is enabled exactly when the navigation seam resolves a neighbor. Both rails render ONLY while an image is open, so a failure message on the empty state is invisible by approved design.

WITH (new):

> - On-screen controls (0.2.1 floating layout per operator-run feedback): ONE centered floating bottom pill — an overlay `egui::Area` anchored bottom-center INSIDE the image window (non-space-reserving; it replaced the 0.2.0 corner rails, whose ←/→/↑/↓ glyph labels rendered as tofu boxes on Windows). The pill holds the message line (12 pt, directly above the button row) and one row of four arrow buttons `[←] [→] [↑] [↓]` whose arrows are painted vector triangles via `egui::Shape::rotated_triangle` — no font dependency (guaranteed-visible icons on Windows 10/11). ←/→ navigate previous/next through the same `navigate` seam as the keys; ↑/↓ are visible but DISABLED (Wave-3 reserve). A direction button is enabled exactly when the navigation seam resolves a neighbor. The pill renders ONLY while an image is open, so a failure message on the empty state is invisible by approved design.

(All other mechanism bullets — listing scope/filter/order/state/keyboard/edge/failure — stay byte-identical.)

**Edit 6 — Testing section, first bullet.** REPLACE (old fragment):

> ; `src/app/rails_tests.rs` — 1 (arrow-button enablement follows the navigation edges);

WITH (new fragment):

> ; `src/app/floating_controls_tests.rs` — 1 (arrow-button enablement follows the navigation edges);

Census stays 26 — the surrounding sentence "26 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3)" remains TRUE (9 + 1 + 5 + 4 + 2 + 5 = 26, verified in the Cycle-B verification plan §1.1). NO other Testing-section wording changes.

### 2.3 `.agent/project-info/architecture.md` — 8 edits

**Edit 1 — Status paragraph.** AFTER (keep byte-identical):

> Wave 2 (branch `feat/wave-2-horizontal-scroll`, version 0.2.0, 2026-11-01) landed the horizontal sibling-image navigation slice: the current-folder listing module, Left/Right keyboard navigation, and the on-screen corner navigation rails.

INSERT (new sentence directly after it, before "Vertical navigation (Wave 3), fullscreen, zoom, and "Open with..." remain planned."):

> A follow-up UI-fix slice (branch `fix/centered-floating-controls`, version 0.2.1, 2026-11-01, per operator-run feedback) replaced the corner rails and the top Open… toolbar with one centered floating bottom pill (painted vector-triangle arrows; no font dependency) and an empty-state center Open button.

**Edit 2 — Logical Components, Viewer-UI bullet (full replacement).** REPLACE (old):

> - **Viewer UI (egui/eframe, immediate-mode) — Wave-1 slice LANDED, Wave-2 rails LANDED**: regular resizable window (title `Images Scroller`, not maximized at launch); top "Open…" toolbar button; central image surface painted fit-scaled and centered; while an image is open, two corner navigation rails render around it — a bottom rail (previous/next image arrows + message area) and a right rail (up/down folder arrows, disabled until Wave 3). Remaining UI (activating the folder arrows, fullscreen toggle, zoom controls, "Open with..." menu entry): planned.

WITH (new):

> - **Viewer UI (egui/eframe, immediate-mode) — Wave-1 slice LANDED; Wave-2 navigation slice LANDED; 0.2.1 floating-controls UI-fix LANDED**: regular resizable window (title `Images Scroller`, not maximized at launch); the empty state shows ONE big centered "Open…" button (18 pt text, large 180×48 button) that opens the native dialog; with an image open there is NO Open control anywhere. While an image is open, ONE centered floating bottom pill renders above the central image surface — a non-space-reserving overlay `Area` anchored bottom-center with a translucent dark frame — holding the failure message line and four arrow buttons with PAINTED vector-triangle arrows (no font dependency; ←/→ navigate previous/next, ↑/↓ are visible but disabled until Wave 3). Both overlay Areas are stateless surfaces (per-call `egui::Id::unique(...)` ids — no remembered window/layout state) and the painted buttons expose AccessKit button info (`WidgetInfo::labeled(Role::Button, …)`). Remaining UI (activating the folder arrows, fullscreen toggle, zoom controls, "Open with..." menu entry): planned.

**Edit 3 — Module Layout, app.rs bullet.** REPLACE (old):

> - `src/app.rs` — application shell: `ImagesScrollerApp` state (opened path, in-session folder listing, decoded payload, message text, display), the `open_image` flow (decode → state transition; failure changes only the message), the Left/Right keyboard navigation and the shared `navigate(NavigationDirection)` seam, the top "Open…" button wiring, the `APP_WINDOW_TITLE` constant, and `NativeOptions` (resizable, not maximized). Child modules: `src/app/rails.rs` (the corner navigation rails), `src/app/tests.rs`, `src/app/rails_tests.rs`.

WITH (new):

> - `src/app.rs` — application shell: `ImagesScrollerApp` state (opened path, in-session folder listing, decoded payload, message text, display), the `open_image` flow (decode → state transition; failure changes only the message), the Left/Right keyboard navigation and the shared `navigate(NavigationDirection)` seam, the empty-state centered "Open…" button wiring, the image-open gate that shows the floating pill, the `APP_WINDOW_TITLE` constant, and `NativeOptions` (resizable, not maximized). Child modules: `src/app/arrow_direction.rs` (the painted-arrow control types), `src/app/floating_controls.rs` (the floating pill), `src/app/tests.rs`, `src/app/floating_controls_tests.rs`.

**Edit 4 — Module Layout, replace the rails.rs bullet with two new bullets.** REPLACE (old):

> - `src/app/rails.rs` — on-screen corner navigation rails: right rail (↑/↓ folder arrows, disabled in Wave 2 — Wave-3-reserved), bottom rail (←/→ image arrows + the message area); a direction button is enabled exactly when the navigation seam resolves a neighbor; both rails render only while an image is open.

WITH (new):

> - `src/app/floating_controls.rs` — the floating navigation pill (replaced the deleted `src/app/rails.rs`): one overlay `Area` anchored bottom-center (translucent-dark `Frame`, non-space-reserving, `Order::Foreground`) holding the message line (a 12-pt label above the button row) and one row of four arrow buttons painted via `egui::Shape::rotated_triangle` (no font dependency); a button is enabled exactly when the navigation seam resolves a neighbor (`neighbor_image_path(...).is_some()`), ←/→ drive the `navigate` seam, and ↑/↓ are disabled Wave-3 reserves; the pill renders only while an image is open (caller gate in `app.rs`).
> - `src/app/arrow_direction.rs` — pure child-module types for the pill: `ArrowDirection` (the `rotated_triangle` rotation mapping: left/right/up/down) and `ArrowControl` (which button a widget represents: accessible labels + the optional `NavigationDirection`, `None` for the Wave-3-reserved folder arrows).

**Edit 5 — Open-Image and Display Flow, step 1.** REPLACE (old):

> 1. Launch: CLI argument present → `open_image(path)` runs before the first frame; absent → the user clicks "Open…" → native `IFileOpenDialog` (owner HWND via `raw-window-handle`).

WITH (new):

> 1. Launch: CLI argument present → `open_image(path)` runs before the first frame; absent → the empty state's big centered "Open…" button → native `IFileOpenDialog` (owner HWND via `raw-window-handle`).

**Edit 6 — Open-Image and Display Flow, step 4.** REPLACE (old):

> 4. Failure: `Unsupported`/`Unloadable` → the bottom rail's message area shows the unsupported-file message (the rail renders only while an image is open); a previously loaded image and the rest of the state stay untouched.

WITH (new):

> 4. Failure: `Unsupported`/`Unloadable` → the floating pill's message line shows the unsupported-file message (the pill renders only while an image is open); a previously loaded image and the rest of the state stay untouched.

**Edit 7 — Project Paths, Cargo line.** REPLACE (old):

> - Cargo project at the repository root: `Cargo.toml` (package `img-scroller` 0.2.0, edition 2024) plus the tracked `Cargo.lock` — exists since Wave 1.

WITH (new):

> - Cargo project at the repository root: `Cargo.toml` (package `img-scroller` 0.2.1, edition 2024) plus the tracked `Cargo.lock` — exists since Wave 1.

**Edit 8 — Project Paths, repository-layout line.** REPLACE (old fragment):

> application code in `src/` (the `app` shell with its `rails` child module, `app_display`, `folder_listing`, `image_loading`, `scaling`, plus the `platform/` facade)

WITH (new fragment):

> application code in `src/` (the `app` shell with its `arrow_direction`/`floating_controls` child modules, `app_display`, `folder_listing`, `image_loading`, `scaling`, plus the `platform/` facade)

NOT changed (verified decisions): Navigation Critical Paths path 2 ("arrow key or UI button" — still true), §Open Architectural Decisions (NO addition — the `Id::unique`/a11y fact is landed, recorded in the Viewer-UI bullet instead; the open-decisions list stays untouched).

### 2.4 `.agent/project-structure.md` — 3 edits

**Edit 1 — app.rs bullet.** REPLACE (old):

> - app.rs - eframe application shell: viewer window options, open-flow state, top Open… button, Left/Right keyboard navigation and the shared navigate seam, in-session folder-listing state, window-title constant

WITH (new):

> - app.rs - eframe application shell: viewer window options, open-flow state, the empty-state centered Open button, the image-open gate for the floating pill, Left/Right keyboard navigation and the shared navigate seam, in-session folder-listing state, window-title constant

**Edit 2 — app/ folder bullet.** REPLACE (old):

> - app/ - child modules of the application shell: rails.rs (corner navigation rails: right rail with disabled up/down folder arrows, bottom rail with image arrows + message area), tests.rs (shell/navigation unit tests), rails_tests.rs (rail button enablement tests)

WITH (new):

> - app/ - child modules of the application shell: arrow_direction.rs (painted-arrow control types for the floating pill), floating_controls.rs (centered floating bottom pill: message line + painted vector-triangle arrow buttons), tests.rs (shell/navigation unit tests), floating_controls_tests.rs (pill arrow-button enablement tests)

**Edit 3** — NOTHING else in this file changes. The wave-2 update never touched this file, so `rails` entries exist here to be deleted and NO `tests.rs` additions are missing beyond what Edit 2 covers. Verify: no other `src/` file gained/lost modules this wave (`git diff --stat d82ec38..9a70279` confirms: only `app.rs`, `app/arrow_direction.rs`, `app/floating_controls.rs`, `app/floating_controls_tests.rs`, deleted `app/rails.rs`/`app/rails_tests.rs`, `Cargo.toml/lock`).

### 2.5 `.agent/project-info/context.md` — closing update (structure below is the binding shape)

Markdown edits ordered top-down; the "How to Use This File" section and EVERY aging "Recent Changes" entry (Wave-2-complete entry, 4.1a entry, Task-2/Task-3 spec entries, Wave-1 entries, older wave entries) stay byte-identical except the Top-of-list insertion and the two current-state bullets marked. The stale Cycle-A/interim lines (old Current Work Focus paragraph, stale Current-Project-State TODO/Git bullets, stale Immediate Next Steps 1–5) are REPLACED.

**Edit A — Current Work Focus: replace the ENTIRE paragraph with:**

> Dated 2026-11-01. **Wave-2 UI fix ([`.agent/todos/20261101/20261101-todo-1.md`](../todos/20261101/20261101-todo-1.md)) is COMPLETE through Cycle B and Cycle C documentation is applied; pending: operator re-test + Step-5 merge/push.** Origin: the Wave-2 OPERATOR-RUN reported the 0.2.0 on-screen controls unusable (arrow glyphs rendered as tofu boxes; rails/toolbars rejected) and dictated the replacement layout. Landed fix (version 0.2.1, branch `fix/centered-floating-controls`): one centered floating bottom pill (overlay `Area` anchored bottom-center, translucent dark frame) with painted vector-triangle arrows — ←/→ drive the keyboard `navigate` seam, ↑/↓ are displayed-disabled Wave-3 reserves, enablement per frame from `neighbor_image_path(...).is_some()`; the unsupported-file message renders as a 12-pt line INSIDE the pill above the button row, only while an image is open; the empty state shows ONE big centered Open button (18 pt, 180×48) reusing the native dialog seam, and the Wave-1 top toolbar is gone entirely. `src/app/rails.rs` + `src/app/rails_tests.rs` deleted; keyboard intake unchanged; `src/app/tests.rs` untouched. Verification: 26 unit tests (census unchanged; the enablement test moved to `src/app/floating_controls_tests.rs`), dev-checks `ALL CHECKS PASSED`, exe rebuilt `dist/windows-x64/img-scroller.exe` = 19,461,520 bytes, DLL import audit unchanged (22 OBSERVED DLLs). **OPERATOR-RUN re-test of the 0.2.1 exe is PENDING** (the fix's acceptance gate). Next after the re-test + Step-5 merge/push: Wave 3 ([`.agent/todos/20261029/20261029-todo-5.md`](../todos/20261029/20261029-todo-5.md)) in a NEW session.

**Edit B — Recent Changes: insert ONE new top entry (above the existing 4.1a entry; do not renumber the aging entries — the file's existing numbering style is repeated `1.` items and stays as-is):**

> 1. 2026-11-01: **Wave-2 UI fix landed through Cycle B; Cycle C documentation applied (this session, this cycle).** Commits (chronological on `fix/centered-floating-controls`, above `main`): `d82ec38` (TODO + global plan from the operator report: tofu icons + rails/toolbar layout rejection), `10d0253` (version 0.2.1), `6ef0240` (feature: centered floating pill + empty-state center Open; `src/app/rails.rs`/`rails_tests.rs` deleted; top toolbar removed), `f30b802` (review fix: `Area::show(ctx)` receiver, `Color32::from_rgba_unmultiplied_const`, `Id::unique` ids), `f7566cb` (Tasks 1–3 marks + 4.5a frontend verification report), `9a70279` (Task 4 marks + Cycle-B verification plan). Cycle-B evidence: 26 tests, dev-checks `ALL CHECKS PASSED`, exe rebuilt 19,461,520 B, DLL audit = the recorded 22-DLL constant. Cycle-C docs commit (this one) lands the README/tech/architecture/structure/context wording. **Merged/push hashes and the operator re-test result are NOT recorded yet — the Planner appends them at the workflow Step-6 close.**

**Edit C — Current Project State: add ONE new bullet directly after the Wave-2 bullet:**

> - **Wave-2 UI-fix cycle landed on `fix/centered-floating-controls` (NOT yet merged; version 0.2.1; HEAD `9a70279`)**: `src/app/floating_controls.rs` + `src/app/arrow_direction.rs` (pill: painted vector-triangle arrows, message line above the row, ←/→ via `navigate`, ↑/↓ disabled reserves) + `src/app/floating_controls_tests.rs` (the relocated enablement test); `src/app/rails.rs`/`rails_tests.rs` deleted; empty-state center Open only; keyboard intake and `src/app/tests.rs` untouched; 26 unit tests; dev-checks `ALL CHECKS PASSED`; exe rebuilt `dist/windows-x64/img-scroller.exe` = 19,461,520 bytes; DLL import audit unchanged (22 OBSERVED DLLs); OPERATOR-RUN re-test pending.

**Edit D — Current Project State: amend the Wave-2 bullet** by appending one sentence at its end (nothing else in the bullet changes):

> (its 0.2.0 rails/toolbar UI was superseded by the 0.2.1 floating-controls fix below).

**Edit E — Current Project State, Release-artifact bullet.** REPLACE (old):

> - Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,464,396 bytes, Wave-2 rebuild 2026-11-01); import audit: 22 OBSERVED DLLs recorded in `scripts/build-windows.sh` + `docs/build-windows.md`.

WITH (new):

> - Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,461,520 bytes, 0.2.1 UI-fix rebuild 2026-11-01); import audit: 22 OBSERVED DLLs recorded in `scripts/build-windows.sh` + `docs/build-windows.md` (unchanged).

**Edit F — Current Project State, TODO-state bullet.** REPLACE (old):

> - **TODO state**: `.agent/todos/20261101/20261101-todo-1.md` active (Tasks 1–3 pending implementation); `.agent/todos/20261029/20261029-todo-4-DONE.md` remains done.

WITH (new):

> - **TODO state**: `.agent/todos/20261101/20261101-todo-1.md` active (Tasks 1–4 done; Task 5 documentation is Cycle C, marks land with the docs commit); `.agent/todos/20261029/20261029-todo-4-DONE.md` remains done.

**Edit G — Current Project State, Git bullet.** REPLACE (old):

> - Git: `main` == `origin/main` == `2c9db24`; working tree clean; no other branches (`feat/wave-2-horizontal-scroll` deleted after the verified merge).

WITH (new):

> - Git: branch `fix/centered-floating-controls` at `9a70279`; local `main` at `d82ec38` — AHEAD of `origin/main` (`de332fc`) by that docs commit `d82ec38` plus this fix's branch commits (to merge to `main` and push at workflow Step 5 ONLY, `main` → `origin`); the working tree holds the Cycle-C documentation edits (staged in this cycle's one commit).

(All other Current-Project-State bullets — Wave-1, wave-TODO readiness, binding-facts, CRLF open item — stay byte-identical.)

**Edit H — Immediate Next Steps: replace the WHOLE numbered list with EXACTLY:**

> 1. **Remaining Cycle C steps** — docs review (4.3), architector adherence (4.5b), Task-5 `[DONE]` marks + the single docs commit (4.6).
> 2. **Step 5** — rename `.agent/todos/20261101/20261101-todo-1.md` → `-DONE.md` (content unchanged), merge `fix/centered-floating-controls` to `main` (fast-forward expected; delete the branch after the verified merge), push `main` to `origin` ONLY.
> 3. **Wave-2 UI-fix OPERATOR-RUN re-test (real Windows 10/11 machine, on the rebuilt 0.2.1 exe `dist\windows-x64\img-scroller.exe`)** — checklist: painted arrows VISIBLE (not tofu boxes); pill = one floating translucent-dark pill bottom-center; ←/→ buttons navigate prev/next (keys unchanged; edges stay); ↑/↓ dimmed; NO rails/toolbar remnants anywhere; big centered Open ONLY in the empty state (gone while an image is open); message line only while an image is open. Report results back into the project.
> 4. **Next session: Wave 3** ([`.agent/todos/20261029/20261029-todo-5.md`](../todos/20261029/20261029-todo-5.md)) — vertical scroll (sibling folders); process in a NEW session.
> 5. **Open item (flagged, not a TODO)**: broader line-ending renormalization beyond `*.sh` (e.g. `Dockerfile`, `*.rs` are `i/lf w/crlf` in the Windows working tree) — flagged to the user; a dedicated future plan would own it.

### 2.6 `docs/build-windows.md` — verified NO EDIT

Checked: zero exe-size claims; zero current-UI claims. The only dated wording ("Observed DLL import list … Audited 2026-10-31 … Wave 1 verification; first produced exe") is deliberately historical evidence — still TRUE. Also "22 DLLs unchanged" for 0.2.1 does not require a re-recorded baseline (Cycle-B verified set-equality; recording a new dated audit line is a Planner-only call and NOT part of Task 5). DO NOT TOUCH.

## 3. Docs-specialist application protocol (step Cycle-C 4.2)

1. Verify `git branch --show-current` → `fix/centered-floating-controls`; mismatch → STOP, ask the caller.
2. Apply §2.1–§2.5 edits verbatim with the `edit` tool. If an exact old-string does not match the file content byte-for-byte → STOP, return the mismatch to the caller (no improvised rewording).
3. Post-edit verification (grep, read-only), ALL must hold:
   - `README.md`: `bottom-rail` absent; `navigation rails` absent; `0.2.1` present; `0.2.0` present ONLY in the Wave-2 landed sentence ("version 0.2.0") as historical record.
   - `tech.md`: `rails.rs` absent; `rails_tests` absent; `On-screen rails` absent; `most 26 unit tests` sentence intact; `0.2.1` present in the Stack row; mechanism heading `## Horizontal Navigation Mechanism (Implemented, Wave 2)` unchanged.
   - `architecture.md`: `rails.rs` absent; `rails` present ONLY in the two designated historical spots (Status Wave-2 sentence; nothing else); `0.2.1` in the Cargo path line; no `Id::unique` claim beyond §2.3 Edit 2's sentence.
   - `.agent/project-structure.md`: no `rails` string anywhere; `floating_controls` and `arrow_direction` present.
   - `context.md`: `19,464,396` absent (the old 0.2.0 size); `19,461,520` present; `d82ec38`/`9a70279` present; `origin/main` line encodes the ahead-of state.
   - `docs/build-windows.md` and `docs/build.md`: UNCHANGED (`git status` must not list them).
4. Report: per-file edit count + the grep results.

## 4. Review + adherence (steps Cycle-C 4.3 and 4.5b, recorded for the caller)

- 4.3 (code-reviewer, docs): verify every new fact traces to §1 (landed code/commits) — no invented numbers; verify the Wave-2 historical records were NOT erased (the historical-rail mentions kept at §2.3 Edit 1's Wave-2 sentence and README's Wave-2 sentence are intentional); verify no `CHANGELOG.md` diff.
- 4.5b (architector): READY/NOT-READY scored against this plan's §2 exact strings; flag any drift to the caller (fix cycle only if a genuine wording defect).
- 4.6 (implementer): Task-5 marks — mark-ONLY edits to `.agent/todos/20261101/20261101-todo-1.md`: `### Task 5: Documentation updates` → `### Task 5: Documentation updates [DONE]`, and its `- [ ] ` bullet → `- [x] ` (identical text otherwise; overwrite-todo-prevention rule). NO other TODO content change; Task 4's OPERATOR-RUN sub-item STAYS `[ ]`.

## 5. Git plan (ONE commit)

1. Confirm branch (`fix/centered-floating-controls`); read `.gitignore`; run `git status`.
2. Staging list (EXACT: gitignore-checked; nothing else):

| Staged | Why |
| --- | --- |
| `README.md` | §2.1 feature wording |
| `.agent/project-info/tech.md` | §2.2 mechanism/version/message-surface wording |
| `.agent/project-info/architecture.md` | §2.3 component/module/flow wording |
| `.agent/project-info/context.md` | §2.5 closing update (this file was already modified-unstaged from earlier upkeep — its current content is subsumed by §2.5) |
| `.agent/project-structure.md` | §2.4 map update (rails deleted; new modules) |
| `.agent/todos/20261101/20261101-todo-1.md` | §4 Task-5 `[DONE]`/`[x]` marks (in this same commit, per the caller's one-commit instruction) |
| `.kilo/plans/20261101-floating-controls-docs.md` | this plan (4.1b artifact) |

   NOT staged: `docs/build.md`, `docs/build-windows.md` (untouched), `CHANGELOG.md` (untouched; must NOT exist as modified), `brief.md`/`product.md`, `.agent/todos/20261029/*`, `Cargo.toml`/`Cargo.lock` (unchanged this cycle), `dist/`, `logs/`, `target/` (gitignored — `git check-ignore` if in doubt). Never force-add.
3. Commit message EXACTLY:

```
docs: land centered floating controls documentation (README/tech/architecture/context)
```

4. Post-commit checks: `git status` clean (no gitignore-matching path staged); `git log --oneline -1` shows the hash + exact message. NO amend, NO force, NO push (workflow Step 4.6 boundary; push belongs to Step 5, `main` → `origin` ONLY — verifying the push target against `.kilo/rules/git-remote-safety.md`).

## 6. Boundary with Step 6 (recorded for the caller — NOT this step's work)

At the workflow Step-6 close the Planner: appends the merge hash + push evidence into `context.md` Recent-Changes item 1 (the entry in §2.5 Edit B already carries the note that this happens there) + records the operator re-test result; then hands the Wave-3 session pointer ([`.agent/todos/20261029/20261029-todo-5.md`](../../.agent/todos/20261029/20261029-todo-5.md)). This plan's content is accurate-as-of-now precisely because of that recorded boundary.

## 7. Out of scope (explicit)

- `CHANGELOG.md` (binding: untouched), `brief.md`, `product.md`, `docs/build.md`, `docs/build-windows.md` (verified no false claim), any `.kilo/plans/*` file that already exists.
- Source code, scripts, tests, Cargo files, `.opencode/`/`Dockerfile`/`docker-compose.yml` — untouched.
- Merge/branch operations, push, version bump, exe rebuild, harness runs — other steps own them (Cycle-B evidence is already landed; recorded not redone).
- TODO content besides the two Task-5 mark-only flips.

## 8. Verification mapping (TODO Task 5 → plan)

| TODO Task-5 item | Covered by |
| --- | --- |
| `README.md` features wording (floating bottom navigation; empty-state center Open) + painted-arrow icon note, no toolbar description remains | §2.1 (3 edits) + §3 grep checks |
| `tech.md` Viewer-UI/module rails → floating wording; message-surface wording; version 0.2.1; mechanism wording; census 26 stays true | §2.2 (6 edits) |
| `architecture.md` component bullets + module layout + flow wording (+ landed `Id::unique`/a11y note) | §2.3 (8 edits) |
| `.agent/project-structure.md` full map (rails entries deleted; `arrow_direction.rs`/`floating_controls.rs`/`floating_controls_tests.rs` + `tests.rs` recorded) | §2.4 |
| `context.md` closing update (fix's commits `d82ec38..9a70279` + finalize-at-Step-6 note; accurate git refs; operator re-test pending) | §2.5 (A–H) |
| `CHANGELOG.md` untouched; docs committed by docs-specialist per the markdown rule | §0 + §4/§5 |
