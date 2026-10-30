# Plan — Task 1 (Planning Wave): Specification for the 7 Phased Implementation Wave TODO Files

- Date: 2026-10-29
- Source TODO: [`.agent/todos/20261029/20261029-todo-2.md`](../../.agent/todos/20261029/20261029-todo-2.md) — **Task 1 ONLY** (Tasks 2–5 belong to the grouped doc cycle, not this plan).
- Global plan: [`.kilo/plans/20261029-planning-waves.md`](20261029-planning-waves.md) — **`## Addendum v2 — User Decisions` (2026-10-29) is BINDING** and is encoded in this spec.
- Requirements source of truth: [`.agent/project-info/brief.md`](../../.agent/project-info/brief.md).
- Scope: this plan is a SPECIFICATION for the Planner (creator of markdown TODO files only). No code, no `Cargo.toml`, no dependency installs, no git operations are performed by the implementer producing the 7 files — only new markdown TODO files are created.
- Existing-plan protection: NO existing `.kilo/plans/*.md` file may be modified; only the two new files named in this spec may be created.

---

## 1. Binding User Corrections (Addendum v2, 2026-10-29) — encode in the wave files

The wave files MUST state these as the CURRENT requirements, overriding any older text they may find in live docs:

1. **MVP checkpoint = end of Wave 7 (full v1)**. Waves 1–6 must NOT touch `CHANGELOG.md`. Wave 7 only writes the first changelog entry as its final task (no entry content pre-written in this spec or the wave files).
2. **Vertical navigation rule (Wave 3)**: the same-filename rule is REMOVED. Target image priority when navigating to a sibling folder: (a) if the target folder was already visited this session → reopen its session last-viewed image (RAM-only memory, destroyed on app close, no disk persistence); (b) otherwise → ALWAYS the target folder's FIRST image in alphanumeric order. No other fallback.
3. **Fullscreen rule (Wave 4)**: default OFF at launch; the window opens normal or maximized at the user's own placement/size; fullscreen OFF → UI controls visible and the image takes almost all of the central space (UI layout details deferred to a later definition by the user); fullscreen ON → hide the window header and ALL UI controls, showing only the image.

### 1.1 Conflicts detected (record once, in a short note inside each affected wave file)

- `brief.md` `### Vertical Scroll (Sibling Folders)` still contains the same-filename rule and `### Fullscreen Mode` still lacks the OFF-launch/OFF-UI/ON-minimal behavior — the corrections above are NEWER and binding; the live docs are updated by planning-wave Tasks 2 and 5 (executed in the same session in the grouped doc cycle). Wave files must not depend on the stale brief text for these two topics.
- The global plan's main section (line for Wave 3) still mentions the same-filename rule; Addendum v2 supersedes it. Do not reference the stale wording.

---

## 2. Files to create (exactly 7, Pattern C)

| Wave | File (exact path) | Title line (`# Wave N — …`) | Suggested feature branch |
| --- | --- | --- | --- |
| 1 | `.agent/todos/20261029/20261029-todo-3.md` | `# Wave 1 — Project Skeleton and Open Image` | `feat/wave-1-skeleton-open-image` |
| 2 | `.agent/todos/20261029/20261029-todo-4.md` | `# Wave 2 — Horizontal Scroll (Sibling Images)` | `feat/wave-2-horizontal-scroll` |
| 3 | `.agent/todos/20261029/20261029-todo-5.md` | `# Wave 3 — Vertical Scroll (Sibling Folders)` | `feat/wave-3-vertical-scroll` |
| 4 | `.agent/todos/20261029/20261029-todo-6.md` | `# Wave 4 — Fullscreen Mode and Zoom (Fit Width/Height)` | `feat/wave-4-fullscreen-zoom` |
| 5 | `.agent/todos/20261029/20261029-todo-7.md` | `# Wave 5 — Pan then Navigate at Image Edges While Zoomed` | `feat/wave-5-pan-then-navigate` |
| 6 | `.agent/todos/20261029/20261029-todo-8.md` | `# Wave 6 — Fast "Open with..." Menu` | `feat/wave-6-open-with` |
| 7 | `.agent/todos/20261029/20261029-todo-9.md` | `# Wave 7 — Distribution, Packaging, MVP Checkpoint` | `feat/wave-7-distribution` |

TODO-file naming complies with [`.kilo/rules/important-paths.md`](../../.kilo/rules/important-paths.md): `.agent/todos/<YYYYMMDD>/<YYYYMMDD>-todo-<number>.md`.

---

## 3. Mandatory standard structure for EVERY wave TODO file (deterministic template)

Pattern C per the [Critical Workflow](../../.kilo/commands/critical-workflow.md): `# Title` → `## Tasks` → `### Heading` (each `###` inside `## Tasks` is ONE task; sub-items under a heading belong to that task and do NOT spawn new tasks). Trailing `##` sections after `## Tasks` are allowed (precedent: `20261029-todo-2.md`).

Exact skeleton per file:

```markdown
# Wave N — <Title from Section 2, exactly>

<Intro paragraph: 1–3 sentences wave description, then the exact
"Requirements source:" bullet list given per wave in Section 4.>

## Tasks

### Task 1: <per wave, Section 4>
- [ ] <sub-items per wave>

<…additional ### Task headings per wave…>

## Deferred decisions — settle in THIS wave's architector (4.1b) step

- [informational list, never edit into tasks; never blank — per wave, Section 4]

## Future session execution notes

- Process this wave standalone in a future session via the Critical Workflow
  ([`.kilo/commands/critical-workflow.md`]); process tasks ONE 4.x cycle at a time.
- Create a fresh feature branch at workflow Step 2: `<branch from Section 2>`
  (branch creation is restricted to Step 2; `git push` restricted to Step 5,
  push `main` to `origin` ONLY).
- Commit per step with meaningful messages; follow the Gitignore Compliance Rule
  (`.kilo/rules/gitignore-compliance.md`). Mark each task `[DONE]` at 4.6
  (preserve file content, `[]` → `[x]` on sub-items).
- Requirements source of truth: `.agent/project-info/brief.md`, with the binding
  2026-10-29 user corrections applied (list correction numbers from Section 1
  of this spec that apply to this wave).
- Do NOT touch `CHANGELOG.md` in this wave <omit for Wave 7, replaced by: "CHANGELOG.md
  may be edited ONLY for the first MVP entry in its dedicated final task">.
- Do NOT edit existing `.kilo/plans/*` files (new plan files only).

<!-- DO NOT DELETE NEXT SECTION -->

## Important Note for AI Agents

All agents working on this project MUST adhere to the workflows and rules outlined
in the root [AGENTS.md](../../../AGENTS.md). Follow the Critical Workflow
([`.kilo/commands/critical-workflow.md`](../../../.kilo/commands/critical-workflow.md)).

<!-- END DO NOT DELETE -->
```

Notes for the Planner:
- Relative-link depths above are illustrative; compute links correctly from `.agent/todos/20261029/` (e.g., brief = `../../../.agent/project-info/brief.md`).
- All task sub-items use `- [ ]` checkboxes (GitHub task-list style, same as `20261029-todo-2.md`).
- Review/fix loops (workflow steps 4.3/4.5) are PART OF THE WORKFLOW and must NOT be written as wave tasks. None of the `###` tasks below may reference 4.3/4.5.

---

## 4. Per-wave content specification

### 4.1 Wave 1 → `20261029-todo-3.md` — Project Skeleton and Open Image

Intro + `Requirements source:` list:
- `.agent/project-info/brief.md` → `## Technology Stack` (Rust; egui/eframe; WIC system codecs; Windows 10/11; x64/x86), `## v1 Feature Scope` → `### Open Image with System-Registered Formats`, `## Important Note for AI Agents`. Binding correction: none (Section 1 items that apply: none, except general rules).

Wave description sentence (write verbatim): "Wave 1 boots the greenfield application: a Cargo project with an egui/eframe window that can open an image (from a CLI argument or an in-app open dialog), decode it via the Windows system codec layer (WIC), and display it in a regular resizable viewer window."

Tasks (`###` headings, in this order):

- ### Task 1: Cargo project scaffold and window shell
  - [ ] Create `Cargo.toml` at the repository root and the first `src/` application code (all source in `src/` per the project-structure rule; keep the existing empty `src/.gitkeep` situation: remove `.gitkeep` once source files exist).
  - [ ] egui/eframe dependency: add via `cargo add` in this wave's implementation step; the exact crate identifiers and versions are decided in this wave's architector step (see Deferred list) — the TODO file must NOT name or pin crate versions.
  - [ ] App opens a regular, resizable viewer window (not maximized at launch, not a modal dialog). Window title = `Images Scroller` (final app name, user-decided; the repository/`img-scroller` directory name is unchanged and this does not depend on planning-wave Task 2 landing first).
  - [ ] `cargo build` succeeds; `cargo run` shows the empty window.
- ### Task 2: Open Image — CLI argument and in-app open dialog
  - [ ] If launched with an image-path CLI argument, load that image; otherwise the user opens via an in-app open dialog (native Windows open dialog approach decided in this wave's architector step).
  - [ ] Decode via Windows Imaging Component (WIC) system codecs — no bundled decoders; formats not registered in the system are unsupported in v1. The WIC binding approach is decided in this wave's architector step (CRITICAL: the wave file must state verbatim: "WIC binding approach and src/ module layout are deferred to this wave's architector step; they are decided before code writing begins").
  - [ ] On unsupported/unloadable file: show an informational message in the window (exact wording = minor, one sentence chosen during this wave's architector step for determinism — Planner leaves a bullet "unsupported-file message text: decided in the wave's architector step").
- ### Task 3: Display the image
  - [ ] Render the decoded image inside the window, scaled to fit the initial view (initial fit behavior = simple whole-image fit; zoom modes arrive in Wave 4).
  - [ ] Resizing the window keeps the display sane (no cropped/clipped render beyond acceptable fit scaling).
- ### Task 4: Verification
  - [ ] `cargo build` and `cargo run` succeed on a Windows 10/11 machine.
  - [ ] `cargo test` passes (add first unit tests only if the wave's architector step defines a test suite; a minimal smoke test of the decoding-selection logic is acceptable — the architector decides what is unit-testable given WIC is an OS-boundary).
  - [ ] Manual UX check (during the future manual-run verification of this wave): open jpg/png from CLI argument and from dialog; try a file with a format not registered (e.g., intentionally obscure extension) → message shown; window resize keeps the image visible.
- ### Task 5: Documentation updates
  - [ ] `README.md`: replace the build/run placeholder with real commands (`cargo run`, release build), plus the "any system-registered format" constraint.
  - [ ] `.agent/project-info/tech.md`: fill "Development Environment and Setup", "Build and Run", "Testing" placeholders with what this wave landed; record the WIC mechanism and dependencies chosen.
  - [ ] `.agent/project-info/architecture.md`: replace the "planned/no code" status lines that this wave invalidated (module layout, WIC mechanism); keep undecided items listed as open.
  - [ ] `.agent/project-info/context.md`: critical closing step — record wave outcome in the context log.

`## Deferred decisions — settle in THIS wave's architector (4.1b) step`:
- WIC binding approach (which mechanism/crates — no crate names in the TODO file).
- `src/` module layout (first module structure; not pre-invented anywhere).
- Crate selection/versions for the GUI stack (no versions pinned in the TODO file).
- Open-dialog mechanism (native common dialog approach).
- Unsupported-file message wording.
- Whether/how unit tests can cover OS-bound WIC logic vs pure logic.

Notes line (write verbatim in the file): "Wave 1 scope notes: WIC binding approach and src/ module layout are DEFERRED to this wave's architector step. Build/run documentation updates are part of Task 5."

### 4.2 Wave 2 → `20261029-todo-4.md` — Horizontal Scroll (Sibling Images)

Requirements source:
- `brief.md` → `### Horizontal Scroll (Sibling Images)`, `## Sorting Order`, `## v1 Feature Scope` → `### Open Image with System-Registered Formats` (listing awareness). Binding correction: NONE apply verbatim, but note: pan-then-navigate interplay is Wave 5 scope.

Wave description sentence: "Wave 2 adds horizontal navigation: the viewer lists the images of the image's current folder (non-recursive) in alphanumeric order and moves previous/next via the Left/Right arrow keys and on-screen left/right UI buttons — strictly within the current folder."

Tasks:
- ### Task 1: Current-folder image listing
  - [ ] From the currently displayed image's path, enumerate image files of the SAME folder only (strictly not recursive; no grandparent/child traversal).
  - [ ] Which files count as "images" (extension/codec-availability filter) = decided in this wave's architector step; the TODO file must not hardcode a filter list.
  - [ ] Sort the list alphanumeric (the exact collation definition — case handling method — is decided in this wave's architector step; v1 fixed, no settings UI).
- ### Task 2: Previous/next image navigation — keyboard
  - [ ] Left arrow key → previous sibling image; Right arrow key → next sibling image in the sorted order.
  - [ ] At this wave, arrows navigate DIRECTLY (pan-then-navigate interplay is implemented in Wave 5 — state this explicitly in the task).
  - [ ] Behavior at the first/last list position (stay? wrap?) = decided in this wave's architector step.
- ### Task 3: Previous/next image navigation — on-screen UI buttons
  - [ ] Left and right UI buttons perform the same navigation as the keys.
  - [ ] UI layout: the definitive placement/design of the navigation controls is PENDING USER DEFINITION (planning-wave fullscreen correction defers UI layout). The wave file MUST instruct its implementing session to: propose a concrete UI layout (buttons for left/right/up/down, menu area) and ASK THE PLANNER/user for approval before finalizing layout code — no layout is invented silently.
- ### Task 4: Verification
  - [ ] Unit tests (suite exists since Wave 1): listing enumeration (folder-only scope), alphanumeric sort correctness, previous/next index stepping, first/last edge behavior.
  - [ ] `cargo build`, `cargo test`, `cargo run`; manual UX check: keys and buttons move prev/next; nested subfolders are NOT entered; order matches the decided collation.
- ### Task 5: Documentation updates
  - [ ] `README.md` features section: horizontal scroll behavior. `tech.md`/`architecture.md`: record the navigation-engine implementation decisions landed and remaining open items. `context.md`: closing update.

Deferred list: pan-then-navigate interplay (→ Wave 5); UI layout (propose-and-ask mandatory); image-file detection filter; exact alphanumeric collation; edge-of-list behavior.

### 4.3 Wave 3 → `20261029-todo-5.md` — Vertical Scroll (Sibling Folders)

Requirements source:
- `brief.md` → `### Vertical Scroll (Sibling Folders)`, `## Sorting Order`. Binding correction: **Section 1 item 2 applies — NO same-filename rule**. The wave file must include, verbatim: "Requirements note (correction 2026-10-29, Addendum v2 item 3): the same-filename rule present in the brief's current text is REMOVED from v1; the corrected rule below is binding."

Wave description sentence: "Wave 3 adds vertical navigation between sibling folders under the SAME parent folder, with a session-only RAM map of the last viewed image per visited folder, and up/down arrow keys plus on-screen up/down UI buttons."

Tasks:
- ### Task 1: Sibling-folder enumeration
  - [ ] From the current image's folder, enumerate sibling folders under the SAME parent (e.g., from `parent\folder2\img1` → `parent\folder1`, `parent\folder3`). NEVER folders of `parent` (no grandparent jumping); NEVER recursive/deeper traversal.
  - [ ] Sort folders alphanumeric (same collation as Wave 2 — one fixed order; exact definition already decided in Wave 2 and recorded in tech.md; the wave file references that decision).
- ### Task 2: Session last-viewed memory (deliverable of this wave)
  - [ ] RAM-only map: current folder → last viewed image path (per folder visited this session). Update it whenever an image is displayed.
  - [ ] Destroyed when the application closes; NO persistence to disk in v1 (no files written, no settings storage).
- ### Task 3: Target image priority rule — keyboard and buttons
  - [ ] Up arrow key AND up UI button → previous sibling folder; Down arrow key AND down UI button → next sibling folder.
  - [ ] Target image priority (correction 2 above): (a) folder already visited this session → its session last-viewed image; (b) otherwise → ALWAYS the target folder's FIRST image in alphanumeric order. NO same-filename attempt, NO other fallback. (Encode the priority as two bullets verbatim.)
  - [ ] The newly shown image updates the session last-viewed map for the target folder.
  - [ ] Behavior for a sibling folder containing no displayable image (skip it? disable? error?) = decided in this wave's architector step.
  - [ ] Pan interaction: out of scope here — arrows navigate directly until Wave 5 implements pan-then-navigate.
- ### Task 4: Verification
  - [ ] Unit tests: same-parent restriction (no grandparent/deeper jumps), sort order, target-image priority (visited-first-image paths), last-viewed map set/get and per-session lifetime.
  - [ ] `cargo build`, `cargo test`, `cargo run`; manual UX check: up/down keys and buttons move folders under the same parent; revisiting a folder this session reopens its last viewed image; closing and relaunching loses the memory.
- ### Task 5: Documentation updates
  - [ ] `README.md`: vertical scroll + corrected target rule (no same-filename rule — the version corrected per planning-wave Task 5). `tech.md`/`architecture.md`: record session-memory component, corrected navigation rule, remaining open items. `context.md`: closing update.

Deferred list: UI layout placement for the up/down buttons (propose-and-ask, same as Wave 2); empty/no-image sibling-folder handling; edge-of-list behavior for folder navigation (first/last sibling); whether unit-test seams are needed around folder I/O (architector design decision).

### 4.4 Wave 4 → `20261029-todo-6.md` — Fullscreen Mode and Zoom (Fit Width/Height)

Requirements source:
- `brief.md` → `### Fullscreen Mode`, `### Zoom In/Out and Fit Modes`, `## Important Note for AI Agents`. Binding correction: **Section 1 item 3 (fullscreen behavior) applies**.

Wave description sentence: "Wave 4 adds fullscreen mode and zooming: a toggled fullscreen (default OFF at launch) via keyboard shortcut and UI control, plus zoom in/out and fit width/fit height modes via keyboard shortcuts and UI controls; exact bindings and steps are decided in this wave's architector step."

Tasks:
- ### Task 1: Fullscreen toggle — full behavior (correction encoded)
  - [ ] Default fullscreen state at launch: OFF. The window opens normal or maximized exactly as the USER placed/sized it (no forced size; never auto-maximized or auto-fullscreened).
  - [ ] Fullscreen ON: hide the native window title bar (and borders) and ALL UI controls — only the image is shown.
  - [ ] Fullscreen OFF: all UI controls visible again; the image takes almost all of the central space.
  - [ ] Toggleable via BOTH a keyboard shortcut AND a UI control (exact keybinding deferred to this wave's architector step; state it).
- ### Task 2: Zoom in/out
  - [ ] Zoom in / zoom out via keyboard shortcut AND UI controls (which inputs exist, and the zoom step sizes, are decided in this wave's architector step — the wave file must not invent them).
  - [ ] Zoom state is session state (no persistence in v1).
- ### Task 3: Fit width / fit height modes
  - [ ] "Fit width" scales the image so its width fits the window; "Fit height" scales it so its height fits the window; both are UI-selectable modes (keyboard binding for the fit modes, if any, is decided in this wave's architector step).
  - [ ] Mode interactions with manual zoom (e.g., leaving fit mode when zooming) = decided in this wave's architector step.
- ### Task 4: Verification
  - [ ] Unit tests where pure logic exists (zoom-step math, fit-scale computation). `cargo build`, `cargo test`, `cargo run`.
  - [ ] Manual UX check: launch never opens fullscreen; toggling ON hides header and all controls leaving only the image; OFF restores controls with the image central; fit width/height computations correct on differently shaped images.
- ### Task 5: Documentation updates
  - [ ] `README.md` + `tech.md` + `architecture.md` + `context.md`: record landed fullscreen/zoom behavior, keybindings and steps as decided, remaining open items (UI layout pending user definition).

Deferred list: exact fullscreen keyboard shortcut; eframe fullscreen mechanism (how the header/control hiding is achieved); exact zoom shortcuts and zoom step sizes; fit-mode keyboard bindings and zoom↔fit interaction; UI control placement (propose-and-ask to the Planner/user — mandatory instruction in the file).

### 4.5 Wave 5 → `20261029-todo-7.md` — Pan then Navigate at Image Edges While Zoomed

Requirements source:
- `brief.md` → `### Pan and Navigate at Image Edges While Zoomed`. Binding corrections: none specific; note it REPLACES the Wave 2/3 "arrows navigate directly" interim behavior for zoomed images (those stay until this wave).

Wave description sentence: "Wave 5 incorporates pan into navigation: while the image is zoomed beyond the window, arrow keys pan first and navigation triggers only at the image edges; when the image fits entirely, arrows navigate directly as before."

Tasks:
- ### Task 1: Horizontal — pan then previous/next image
  - [ ] While the zoomed image is WIDER than the window: Left/Right arrow keys pan the image horizontally; previous/next sibling image navigation triggers ONLY when panning reaches the image's left or right edge.
- ### Task 2: Vertical — pan then previous/next folder
  - [ ] While the zoomed image is TALLER than the window: Up/Down arrow keys pan vertically; previous/next sibling-folder navigation triggers ONLY when panning reaches the image's top or bottom edge.
  - [ ] Target selection for the triggered folder navigation is the Wave 3 rule (session last-viewed → first image) — unchanged; reference it, do not re-state the full algorithm.
- ### Task 3: Fits entirely → navigate directly
  - [ ] When the image fits the window with no overflow on an axis, arrows navigate directly on that axis (pre-Wave-5 behavior restored for that axis).
- ### Task 4: Verification
  - [ ] Unit tests: overflow detection per axis, edge detection state machine (pan position vs image bounds), navigation trigger conditions. `cargo build`, `cargo test`, `cargo run`.
  - [ ] Manual UX check: zoom wide → keys pan, navigation only at edge; zoom tall → same vertically; small image → direct navigation.
- ### Task 5: Documentation updates
  - [ ] `README.md`, `tech.md`, `architecture.md`, `context.md`: record the pan/navigate controller, remaining open items.

Deferred list: pan step size / scroll increment; keyboard auto-repeat handling at edges; whether mouse-wheel or mouse-drag pan participates (brief specifies KEYS — v1 is keyboard pan; the wave's architector step confirms and may propose mouse support later as out-of-scope note); whether fit modes (Wave 4) suppress panning (architector decision).

### 4.6 Wave 6 → `20261029-todo-8.md` — Fast "Open with..." Menu

Requirements source:
- `brief.md` → `### Fast "Open with..." Menu`, `## Important Note for AI Agents`. Corrections: none specific.

Wave description sentence: "Wave 6 adds a fast 'Open with...' menu entry that enumerates, at runtime, the programs the Windows system registered for the currently viewed file's type/extension and opens the current file with the chosen program — with no internal program database."

Tasks:
- ### Task 1: Runtime program enumeration
  - [ ] Query the Windows system registration data at runtime for programs associated with the currently displayed file's type/extension; build the menu list from that data only. NO internal/hardcoded program database.
  - [ ] The enumeration mechanism (which system APIs / how) is decided in this wave's architector step — the wave file must not name implementation APIs or crates.
- ### Task 2: Open the current file with the selected program
  - [ ] Selecting an entry opens the CURRENT displayed file with that program (hand-off to the system launcher mechanism per the architector decision).
- ### Task 3: Menu entry in the UI
  - [ ] Add the "Open with..." menu entry/portal in the viewer UI; placement follows the wave's propose-and-ask instruction (same UI-layout-pending-user-definition instruction as Waves 2–4: propose, then ask the Planner/user).
  - [ ] Behavior when NO programs are registered for the type = decided in this wave's architector step.
- ### Task 4: Verification
  - [ ] Unit tests where pure logic exists (e.g., parsing/filtering of enumeration results represented in a testable seam). `cargo build`, `cargo test`, `cargo run`.
  - [ ] Manual UX check: for a jpg/png file, the menu lists at least the system-associated programs; selecting one opens this exact file in that program; NOT dependent on any app-maintained list.
- ### Task 5: Documentation updates
  - [ ] `README.md`, `tech.md`, `architecture.md`, `context.md`: record the enumeration mechanism chosen, constraints, remaining open items.

Deferred list: enumeration mechanism and launcher mechanism (APIs/crates — decided, never named in the wave file); menu placement in the UI (propose-and-ask); empty-result behavior; refresh/re-open timing of the enumeration (per-open vs cached — architector decision).

### 4.7 Wave 7 → `20261029-todo-9.md` — Distribution, Packaging, MVP Checkpoint

Requirements source:
- `brief.md` → `## Distribution and Default App Registration`, `## Technology Stack` (x64 + x86), `## Out of Scope for v1 (Deferred)`. Corrections: **Section 1 item 1 (MVP checkpoint) applies**.

Wave description sentence: "Wave 7 completes v1: a zip-style portable installer (user picks the destination; the package unzips the application binary AND its config file there), default-viewer registration helper/guidance, release packaging for x64 and x86 — and it carries the MVP CHECKPOINT (MVP = end of Wave 7 = full v1, per user decision 2026-10-29). Only THIS wave writes the first `CHANGELOG.md` entry, as its final task."

Tasks:
- ### Task 1: Config file (existence + format decision)
  - [ ] Introduce the application config file shipped alongside the binary; its contents/format are decided in this wave's architector step (contents must remain minimal and consistent with architecture.md's "On disk" note). No last-viewed persistence in the config (v1 constraint — session memory stays RAM-only).
- ### Task 2: Zip-style portable installer
  - [ ] The user picks a destination folder; the package unzips the application binary AND the config file into that destination.
  - [ ] Release builds for x64 AND x86 Windows targets; exact target triples and build commands are decided in this wave's architector step.
  - [ ] The packaging mechanism (script vs program, archive tooling) is decided in this wave's architector step.
- ### Task 3: Default-viewer registration helper/guidance
  - [ ] The package includes a helper and/or guidance that helps the user register the app as the default image viewer (file associations) in Windows.
  - [ ] The helper mechanism (script vs executable, exact mechanism) is decided in this wave's architector step — the wave file must not pre-select it.
- ### Task 4: Verification
  - [ ] Unit tests for any pure packaging/config logic that exists. `cargo build --release` (and the x64/x86 target build matrix decided in this wave), `cargo test`.
  - [ ] Manual UX check: run the installer flow on a clean destination dir → binary + config file present and app launches; default-viewer helper/guidance performs or guides correctly; packaged app navigations from Waves 1–6 still work from the installed location.
- ### Task 5: Documentation updates
  - [ ] `README.md`: distribution/installation/registration instructions. `tech.md`, `architecture.md`, `context.md`: record config format, packaging mechanism, helper mechanism, build matrix.
- ### Task 6: MVP checkpoint — first `CHANGELOG.md` entry (FINAL task of the wave)
  - [ ] Per user decision 2026-10-29: MVP = end of Wave 7 = full v1. This is the ONLY wave allowed to write the first `CHANGELOG.md` entry, and it does so only after all prior tasks of this wave are `[DONE]`.
  - [ ] Write the first entry in the existing Keep-a-Changelog structure created in the planning wave (SemVer; the file already contains the empty structure and an "entries begin at MVP" banner). The entry summarizes v1.0.0 content (the 7 features + distribution) — actual entry text is written by the implementing session, NOT pre-drafted in the wave file or this spec.

Deferred list: config file contents/format; default-viewer helper mechanism; packaging mechanism/tooling; release target triples and build matrix; CHANGELOG entry wording (written at execution, not in the wave file).

---

## 5. Banned content (Planner checklist before saving each file)

- No changelog entries anywhere; no version-number brainstorming. Waves 1–6 files must contain the "Do NOT touch CHANGELOG.md in this wave" execution note; Wave 7 uses the Wave-7-specific note.
- No invented keybindings, zoom steps, UI placements, collation details, or mechanism/crate names — every such item appears ONLY in the wave's deferred-decisions list.
- No crate versions, no registry URLs, no `Cargo.toml` content beyond the fact that Wave 1 creates the project.
- No code snippets beyond illustrative TODO guidance (the required verbatim strings in Sections 3 and 4 are text, not code).
- No modification of existing `.kilo/plans/*.md`; never batch all waves' tasks into one cycle within the wave files (each wave file must itself say "one task cycle at a time").

## 6. Planner execution steps for this task (caller-side: Task 1, step 4.2 of the planning wave)

1. Create the 7 files by Section 2 (paths/titles exact) applying Section 3 skeleton + Section 4 content, computing correct relative links from `.agent/todos/20261029/`.
2. Self-check each file against Section 5 (banned content) and against the per-wave Deferred list being non-empty.
3. Mark Task 1 sub-items and completion marking/commit are governed by the global plan steps (4.6 commit message: `docs: planning wave task 1 done (7 wave todo files defined)`), not this spec.

## 7. Verification of this spec against the source task

- Matches todo-2 Task 1 (all 7 waves, in order, with corrections baked in): ✔ Waves 1–7 mapping identical to Sections 4.1–4.7; corrections 2/3/4 (Addendum v2) encoded; Wave 3 and Wave 4 wave-file conflict notes included (Section 1.1).
- Matches caller requirements: Deferred-decisions list per wave, never blank ✔; Future-session execution notes per file ✔; Pattern C ✔; build/run + verification + docs tasks per wave ✔; no CHANGELOG.md before Wave 7 ✔; MVP checkpoint in Wave 7 only ✔.

Return the path to the Planner Agent: `C:\repo\img-scroller\.kilo\plans\20261029-task1-wave-files-spec.md`.
