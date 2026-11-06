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

## Recording rule

An idea enters scope only when the user schedules it into a future TODO wave via the Critical Workflow ([`../../.kilo/commands/critical-workflow.md`](../../.kilo/commands/critical-workflow.md)). Until then it stays parked in this file and has no effect on the requirements source of truth ([brief.md](brief.md)).
