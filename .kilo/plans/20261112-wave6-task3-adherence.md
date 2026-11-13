# Wave 6 Task 3 — Adherence Corrective Plan (4.5b)

**Date:** 2026-11-13
**Branch:** `feat/wave-6-open-with` (HEAD `61cc1c8`; NO branch/commit/push actions in the plan-authoring step itself)
**Source TODO:** `.agent/todos/20261029/20261029-todo-8.md` → Task 3
**Origin:** 4.5b Overall Plan Adherence for the implementation plan `.kilo/plans/20261112-wave6-task3-open-with-menu-implementation-plan.md`
**Verdict basis:** all code-level adherence points CONFORM (see 4.5b report to the Planner). Exactly ONE deviation is NOT acceptable-as-recorded:

## The single deviation

`.agent/project-structure.md` was NOT updated. The implementation plan §8 flagged: "*.agent/project-structure.md* gains the two new files in the 4.4 docs step — flagged here so 4.4 knows (`src/app/open_with_button.rs`, `src/app/open_with_button_tests.rs`)." The landed 4.4 docs step (commit `61cc1c8`) edited only source doc-comments and did NOT touch the structure map. The file was last updated in Wave 5.1 (`d188b0b`) and now omits ALL Wave-6 modules (Cycle A and Cycle B), violating the project-structure rule (`.kilo/rules/project-structure.md`: the file "must exist and accurately reflect the current project structure").

## Corrective step (docs-only; NO source-code changes)

Assign to a docs-capable sub-agent (docs-specialist or implementer with explicit Planner authorization for `.md` edits — markdown-generation rule). Execute in ONE commit on `feat/wave-6-open-with`:

1. Read `.agent/project-structure.md` (current full content) and `.gitignore`.
2. Update the `src/` file list: add `open_with_listing.rs` (runtime open-with enumeration + launcher seam + failure-message constant, with its `tests.rs` child module) after `name_collation.rs`, keeping the existing one-line-per-file style.
3. Update the `src/app/` entry: append to the child-module description the two Wave-6 Task-3 modules — `open_with_button.rs` (the pill's 11th "Open with..." button: painted square-with-emerging-arrow glyph, upward `egui::Popup::menu` listing the system-registered programs verbatim in a 300-point scroll area, the two-point display-list refresh contract, the disabled + "No programs registered for this file type" empty-result behavior, no hotkey in v1) and `open_with_button_tests.rs` (7 container-deterministic tests of the availability/tooltip/launch-failure seams).
4. Update the `src/platform/` entry: add `windows_open_with.rs` (Windows enumeration of the programs the system registered for a file's type and the selected-program launcher) and note `mod.rs` re-exports the enumerate/launch pair consumed by `src/app/open_with_button.rs`.
5. Also reflect the existing Wave-6 cycle-A test module already missed: `open_with_listing/tests.rs` under the `src/` folders list (same style as the `folder_listing/` and `sibling_folder_listing/` entries).
6. Edit NOTHING else in the file; preserve all existing wording except the entries extended above; no line-number references.
7. Commit: `docs: update project structure map for wave6 open-with modules` — stage ONLY `.agent/project-structure.md`. Before committing: read `.gitignore`, run `git status`, verify nothing ignored is staged.

## Explicitly OUT of scope

- NO Task-4 work (no release rebuild, no OPERATOR-RUN execution).
- NO Task-5 work (README/tech/architecture/context edits stay with Task 5).
- NO source-code edits; NO harness run needed (docs-only change does not affect the 177/28 census or any check).
- NO edits to existing `.kilo/plans/*` files (this is a NEW plan file).

**Plan file:** `.kilo/plans/20261112-wave6-task3-adherence.md`
