# Future Ideas (Parked)

This file records **parked, NOT-yet-scheduled feature ideas** for `Images Scroller`. Each entry captures a commit point only. Nothing here changes the v1 scope: the requirements source of truth remains [brief.md](brief.md) until the user schedules an idea into a TODO wave (see `## Recording rule` below).

## Idea 1 — Subfolder picker on image-less root folder

Date recorded: 2026-11-02 · Reporter: user

**Status:** Initial idea — details pending from the user. NOT scheduled into any wave (Wave 4/5/6/7 scope unchanged).

Behavior as given by the user (2026-11-02):

- When the current folder has **NO** root-level images but **HAS** subfolders that contain images, the app shows the message `no root images` INSTEAD of the current `Empty folder` message.
- The subfolders are listed on screen beside/next to that message.
- The user presses `Tab` to select the first subfolder and can switch the selection between subfolders.
- Pressing `ENTER` changes the open path to a folder inside the selected subfolder (re-anchor: open that folder's content).

**Open details (PENDING — supplied by the user later):** the user stated further details remain to be provided. The exact message copy, the subfolder list format, and the selection UI are OPEN and NOT decided here. Do NOT invent behavior beyond the points above. This record is a commit point only and changes no wave's scope.

## Idea 2 — Drag & drop file open

**Status:** Parked idea — NOT scheduled into any wave (Wave 4.1 fix-wave scope at `.agent/todos/20261102/20261102-todo-1.md` is unaffected). Date recorded: 2026-11-02, reporter: user.

Behavior as given by the user (2026-11-02):

- The user drags a file from another Windows app (file explorer or similar) and drops it over the Images Scroller window → the app opens it.
- If the type is incompatible, the app shows a message.

**Open details (PENDING user details):** drop-target surface decisions, message wording/placement, whether multi-file drops are supported. This record is a commit point only and changes no wave's scope.

## Idea 3 — Fullscreen bottom-hover reveal

**Status:** Parked idea — NOT scheduled into any wave (Wave 4.1 fix-wave scope at `.agent/todos/20261102/20261102-todo-1.md` is unaffected). Date recorded: 2026-11-02, reporter: user.

Behavior as given by the user (2026-11-02):

- While fullscreen is ON, the UI buttons are shown when the user moves the cursor to the center-bottom area.
- Recorded together with Idea 5 — the user framed them as the same reveal mechanic under two states.

## Idea 4 — Wave-5 pan refinement

**Status:** Parked idea — NOT scheduled into any wave (Wave 4.1 fix-wave scope at `.agent/todos/20261102/20261102-todo-1.md` is unaffected). Date recorded: 2026-11-02, reporter: user. Refines the Wave-5 scope; the Wave-5 architector MUST read this idea and encode it (user: feature very related to the pan item). To be carried INTO Wave 5 `20261029-todo-7.md` when that wave runs — per the user: "maybe it is already included in another TODO file".

Behavior as given by the user (2026-11-02):

- When the zoomed image exceeds the available space, the arrow keys pan the image.
- Navigation triggers ONLY when the user presses the arrow key AGAIN while the view is already at that side's edge (left/right/top/bottom limit).
- When navigation fires to the next image IN THE SAME FOLDER, the view port of the new image starts at the OPPOSITE position of the previous view (example: viewport at right-bottom, press right → next image opens with viewport starting at left-bottom).

## Idea 5 — Normal-mode auto-hide chrome + bottom-hover reveal

**Status:** Parked idea — NOT scheduled into any wave (Wave 4.1 fix-wave scope at `.agent/todos/20261102/20261102-todo-1.md` is unaffected). Date recorded: 2026-11-02, reporter: user.

Behavior as given by the user (2026-11-02):

- When fullscreen is OFF, the UI buttons are shown only for a short amount of seconds after an image is loaded, and are shown again when the cursor moves to the bottom area (same reveal pattern as fullscreen mode — Idea 3).

**Open details (PENDING user details):** the exact delay/dwell, target area, and whether the pill behaves identically. This record is a commit point only and changes no wave's scope.

## Idea 6 — Per-folder session zoom memory

**Status:** Parked idea — NOT scheduled into any wave (Wave 4.1 fix-wave scope at `.agent/todos/20261102/20261102-todo-1.md` is unaffected). Date recorded: 2026-11-02, reporter: user. New state/model feature; verdict interactions (fit one-shot from Wave 4.1 Task 7) are open — PENDING user details.

Behavior as given by the user (2026-11-02):

- Navigating between images of the SAME folder keeps, for each folder, ONE zoom level for the session: all images of that folder render at the folder's current zoom; changing zoom in image 3 updates the folder level.
- The ZOOM level is remembered BY FOLDER for the current app session only (RAM).
- Arriving at a NEVER-visited folder opens its image at fit size.

## Idea 7 — Pan-related enhancements beyond the current keyboard pan

**Status:** Parked idea — NOT scheduled into any wave (Wave 5.1 fix-wave scope at `.agent/todos/20261112/20261112-todo-1.md` is unaffected). Date recorded: 2026-11-12, reporter: user. Recorded AFTER the Wave-5 pan gate FULLY PASSED on the 0.5.0 exe; the user chose to leave the working keyboard pan as-is for now. Related earlier record: Idea 4 (the pre-Wave-5 pan refinement) — left untouched.

Behavior as given by the user (2026-11-12):

- Pan-related enhancements (e.g., beyond the current keyboard pan) are a FUTURE idea, NOT to implement now — the user's wording: "now that it's working, just let it be".

**Open details (PENDING user details):** none supplied — the user asked to record the idea only. This record is a commit point only and changes no wave's scope.

## Idea 8 — Close the app after a successful "Open with..." launch (optional via a future settings checkbox)

**Status:** Parked idea — NOT scheduled into any wave (Wave 6.2 fix-wave scope at `.agent/todos/20261113/20261113-todo-2-DONE.md` is unaffected; the "Open with..." feature is confirmed WORKING on the 0.6.2 exe). Date recorded: 2026-11-13, reporter: user. Recorded per the user's explicit instruction ("register this, without following the critical workflow").

Behavior as given by the user (2026-11-13):

- When an "Open with..." selection successfully opens the current file in the chosen program, the application CLOSES itself.
- The user suggests implementing this later, TOGETHER with the future config/settings place: a CHECKBOX in the (future) configuration to enable/disable this close-after-launch behavior.

**Open details (PENDING user details):** the exact close semantics (immediate close vs a short delay; what happens if the launched program fails to start — presumably stay open; interaction with the session last-viewed memory on close) and the settings-UI placement are OPEN and NOT decided here. This record is a commit point only and changes no wave's scope.

## Idea 9 — In-app "About" section

**Status:** Parked idea — NOT scheduled into any wave (the Wave-8 scope at `.agent/todos/20261117/20261117-todo-1.md` is unaffected). Date recorded: 2026-11-17, reporter: user.

Behavior as given by the user (2026-11-17):

- Somewhere in the app, include an "About" section that shows things like the GitHub repository URL, the application version number, the author, etc.

**Open details (PENDING user details):** the exact placement (menu entry, pill button, dialog), the exact content fields, and the styling are OPEN and NOT decided here. This record is a commit point only and changes no wave's scope.

## Idea 10 — In-app per-type association link/unlink

**Status:** Parked idea — NOT scheduled into any wave (the Wave-8 scope at `.agent/todos/20261117/20261117-todo-1.md` is unaffected). Date recorded: 2026-11-17, reporter: user. Related binding facts: the Wave-7 helper registry mechanism (`packaging/register-default-viewer.cmd` — HKCU-only, OpenWithProgids + Capabilities/RegisteredApplications, never writes the hash-protected `UserChoice`) and the Wave-8 installer's association-selection page are the installed-side counterparts; the in-app version would be the runtime-side one.

Behavior as given by the user (2026-11-17):

- The app should include an option where the user can LINK/UNLINK image file types to the app — i.e. set or unset `img-scroller.exe` as the default app for that file type. The user chooses types while installing the app (the Wave-8 installer's association page), then can change them later in a section of the app.

**Open details (PENDING user details):** the UI placement, the exact mechanism (whether it reuses the Wave-7 helper registry surface at runtime), and the Windows constraint handling (the per-type DEFAULT is user-mediated in Settings on Windows 10/11 — the hash-protected `UserChoice` key must not be written programmatically; the in-app option would register/unregister the app into the choice lists and open Settings for the final choice) are OPEN and NOT decided here. This record is a commit point only and changes no wave's scope.

## Idea 11 — GitHub update checker + in-app updater

**Status:** Parked idea — NOT scheduled into any wave (the Wave-8 scope at `.agent/todos/20261117/20261117-todo-1.md` is unaffected; the Wave-8 Task-7 GitHub Releases publishing is the release side this idea would consume). Date recorded: 2026-11-17, reporter: user.

Behavior as given by the user (2026-11-17):

- Every time the app opens, it verifies whether a new release exists on GitHub.
- If a new release exists, show a pop-up asking to update or not.
- If the user chooses update, download the new release installer in the background.
- When the download finishes, a new pop-up asks "update now or later".
- If the user chooses "now", the application closes and the downloaded installer executes.

**Open details (PENDING user details):** the release-check mechanism (GitHub API endpoint/queries), the network-failure and offline behavior, the privacy consideration (a network call on every launch), the per-architecture installer URL resolution, and any authenticity/safety checks on the downloaded installer are OPEN and NOT decided here. This record is a commit point only and changes no wave's scope.

## Idea 12 — Screenshot automation

**Status:** Parked idea — NOT scheduled into any wave (the Wave-8 scope at `.agent/todos/20261117/20261117-todo-1.md` is unaffected). Date recorded: 2026-11-17, reporter: user (originating from the user's 2026-11-17 chat question "is there a way to automatize screenshots of the app?"; the Planner's answer was recorded and the user chose to park it).

The two viable paths recorded by the Planner (2026-11-17):

- **Agent-side (fully automatable in the container):** egui's official `kittest` snapshot-testing harness renders the real egui UI offscreen (software renderer) inside `cargo test` and can save PNG screenshots of UI states (the pill, the open-with menu, the empty state, the zoomed paint) — automatable inside the existing dev-checks container, no operator needed. Limit: it captures the app's widget surface, NOT the OS window frame (no title bar / maximize chrome).
- **Real-window screenshots (operator-assisted but scriptable):** a small PowerShell capture script on the real Windows machine (System.Drawing `PrintWindow`/`CopyFromScreen` bound to the "Images Scroller" window by title), optionally with a dev-only CLI flag that drives the app into deterministic states (fullscreen, zoomed, menu open) so captures are reproducible.
- (Considered, not recommended: in-app GL frame readback — eframe/glow has no built-in screenshot API; reading back the GL framebuffer is nontrivial.)

**Open details (PENDING user details):** the goal (README/docs screenshots, test snapshots, or both), which UI states, where the PNGs land (committed vs gitignored), and whether a dev-only CLI state flag is acceptable are OPEN and NOT decided here. This record is a commit point only and changes no wave's scope.

## Recording rule

An idea enters scope only when the user schedules it into a future TODO wave via the Critical Workflow ([`../../.kilo/commands/critical-workflow.md`](../../.kilo/commands/critical-workflow.md)). Until then it stays parked in this file and has no effect on the requirements source of truth ([brief.md](brief.md)).
