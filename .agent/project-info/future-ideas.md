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

## Recording rule

An idea enters scope only when the user schedules it into a future TODO wave via the Critical Workflow ([`../../.kilo/commands/critical-workflow.md`](../../.kilo/commands/critical-workflow.md)). Until then it stays parked in this file and has no effect on the requirements source of truth ([brief.md](brief.md)).
