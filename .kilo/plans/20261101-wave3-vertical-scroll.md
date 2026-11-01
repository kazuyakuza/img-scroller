# Global Plan — Wave 3: Vertical Scroll (Sibling Folders)

- Date: 2026-11-01
- Source TODO: [`.agent/todos/20261029/20261029-todo-5.md`](../.agent/todos/20261029/20261029-todo-5.md) (Pattern C: five `###` tasks → 5 tasks below)
- Workflow: [`.kilo/commands/critical-workflow.md`](../.kilo/commands/critical-workflow.md)
- Session start state verified 2026-11-01: branch `main` == `origin/main` == `990285f`; working tree clean; version **0.2.1**; 26 unit tests; `src/app/floating_controls.rs` pill has painted ↑/↓ buttons present but disabled (Wave-3 reserves wired to `NavigationDirection: None`).

## Binding Constraint Sources (all sub-agents MUST honor)

- Requirements: `brief.md` → `### Vertical Scroll (Sibling Folders)`, `## Sorting Order`; the TODO file's Task 3 text is the BINDING correction (Addendum v2 item 3): **NO same-filename rule** (the stale brief same-filename text is superseded — do not implement it).
- Build environment: ALL Rust/Cargo runs in the container on the Alpine VM through `alpine-vm` MCP — call `vm_status` FIRST; command exit code is authoritative; if the VM or `/img-scroller` share is unavailable, STOP and ask the user. Host needs NO Rust/Cargo.
- TODO execution notes: fresh branch `feat/wave-3-vertical-scroll` (branch creation restricted to Step 2); `git push` restricted to Step 5 (`main` → `origin` ONLY); commit per step with meaningful messages; mark tasks `[DONE]` at 4.6 (preserve file content, `[]` → `[x]`); do NOT touch `CHANGELOG.md`; do NOT edit existing `.kilo/plans/*` files (new plan files only).
- Code rules (apply during implementation): all source in `src/`; files ≤ 200 lines; methods ≤ 50 lines; ≤ 2 arguments per method (extract param structs into new files when needed); max nesting depth 2; private-by-default members; self-documenting code (no comments unless truly warranted); no commented-out code; single-section boolean conditions.
- Deferred decisions that MUST be settled in Task 3's 4.1b (architector) step and proposed to the Planner/user (mandatory propose-and-ask): UI placement for up/down buttons, empty/no-image sibling-folder handling, edge-of-list behavior, unit-test seams around folder I/O. Pre-analysis positions are given per task below; the architector formalizes them and the Planner presents them to the user.

## Global Pre-Analysis (technical & architecture)

1. **Landed baseline to build on (0.2.1)**: `src/app.rs` holds the `ImagesScrollerApp` state, the shared `navigate(NavigationDirection)` seam, `current_folder_listing`, and the image-open gate; `src/folder_listing.rs` holds the pure horizontal listing (17-extension whitelist, lowercase-folded alphanumeric sort with exact-name tie-break, ±1 neighbor resolution with list-edge reports); `src/app/floating_controls.rs` renders ONE centered floating bottom pill with painted vector-triangle arrows ←/→/↑/↓ (message line above; ←/→ drive `navigate`; ↑/↓ disabled).
2. **Vertical engine shape (Wave-3 slice)**: a sibling-folder listing (non-recursive read of the parent of the current folder, directories only, NO `.` entries, NO grandparent/deeper traversal) + a folder-neighbor resolver (±1 over the folder list) + the target-image priority rule — (a) visited this session → its session last-viewed image; (b) else ALWAYS the target folder's first image in alphanumeric order; NO same-filename attempt; NO other fallback.
   - Wave-3 architector consumes the collation decision recorded in `tech.md` → `## Horizontal Navigation Mechanism (Implemented, Wave 2)` (lowercase-folded name sort with exact-name tie-break) and applies the SAME collation to folder names — one fixed order for both axes (brief `## Sorting Order`).
   - Reuse pattern follows the Wave-2 precedent: pure-logic modules under `src/` that are container-testable with injected directory-read seams (like `folder_listing`'s design), OS-cfg untouched.
3. **Session last-viewed memory (RAM-only)**: a `HashMap<folder_path, image_path>`-style field on `ImagesScrollerApp` updated every time an image is successfully displayed; destroyed when the app struct drops (no disk writes anywhere — persistence absence in v1 is satisfied by simply never writing anything).
4. **Intake & UI (Task 3)**: `ArrowUp`/`ArrowDown` keyboard handling joins the existing per-frame arrow-key intake (arrows navigate DIRECTLY — pan-then-navigate is Wave 5); the pill's existing ↑/↓ buttons get `NavigationDirection::PrevSiblingFolder`/`NextSiblingFolder`-style wiring through the same `navigate` seam; per-frame enablement of ↑/↓ comes from folder-neighbor resolution (`is_some()`), mirroring how ←/→ enable from image-neighbor resolution.
5. **Version bump (Step 3)**: Wave 3 adds a feature → semver **minor**: 0.2.1 → **0.3.0** (`Cargo.toml`, tracked lock flow respected).
6. **Verification stance (Task 4)**: container dev-checks via harness (`ALL CHECKS PASSED`: fmt/check/test/clippy); expanded unit-test census; the manual UX checklist (OPERATOR-RUN on real Windows) is USER-side and stays unchecked until the user reports; the release exe SHOULD be rebuilt (`scripts/build-windows.sh`) so the operator can execute the checklist on the 0.3.0 build.
7. **Non-goals for this wave**: NO pan interplay, NO zoom/fullscreen/OpenWith, NO `CHANGELOG.md` edits, NO x86, NO same-filename rule, NO persistence of the last-viewed map.

## Execution Order (each entry = separate `task` invocation)

- Step 2: Git Feature Branch Setup => `implementer` (commit any stray state → none expected; verify on `main`; create+switch `feat/wave-3-vertical-scroll`)
- Step 3: Version Update => `implementer` (0.2.1 → 0.3.0; commit `chore: bump version to 0.3.0`)
- Task 1: 4.1b => `architector` (NOT front-end; sibling-folder enumeration plan)
- Task 1: 4.2 => `implementer`
- Task 1: 4.3 => `code-reviewer` + `code-simplifier`; 4.3-fix => `implementer`
- Task 1: 4.4 => `docs-specialist`
- Task 1: 4.5b => `architector`
- Task 1: 4.6 => `implementer` (commit marks)
- Task 2: 4.1b => `architector` (NOT front-end; session last-viewed map plan)
- Task 2: 4.2 => `implementer`
- Task 2: 4.3 => `code-reviewer` + `code-simplifier`; 4.3-fix => `implementer`
- Task 2: 4.4 => `docs-specialist`
- Task 2: 4.5b => `architector`
- Task 2: 4.6 => `implementer`
- Task 3: 4.1a => `frontend-specialist` (FRONT-END task: keys + pill buttons)
- Task 3: 4.1b => `architector` (uses 4.1a spec; settles deferred decisions; propose-and-ask to Planner/user)
- Task 3: 4.2 => `implementer`
- Task 3: 4.3 => `code-reviewer` + `code-simplifier`; 4.3-fix => `implementer`
- Task 3: 4.4 => `docs-specialist`
- Task 3: 4.5a => `frontend-specialist` (FRONT-END verification vs 4.1a spec)
- Task 3: 4.5b => `architector` (incorporates 4.5a report)
- Task 3: 4.6 => `implementer`
- Task 4: 4.1b => `architector` (verification + exe rebuild plan)
- Task 4: 4.2 => `implementer`
- Task 4: 4.3 => `code-reviewer` + `code-simplifier`; 4.3-fix => `implementer`
- Task 4: 4.4 => `docs-specialist` (minimal/none — Task 5 owns docs)
- Task 4: 4.5b => `architector`
- Task 4: 4.6 => `implementer`
- Task 5: 4.1b => `architector` (documentation plan; NOT front-end)
- Task 5: 4.2 => `implementer`
- Task 5: 4.3 => `code-reviewer` + `code-simplifier`; 4.3-fix => `implementer`
- Task 5: 4.4 => `docs-specialist`
- Task 5: 4.5b => `architector`
- Task 5: 4.6 => `implementer`
- Step 5: TODO File Completion => `implementer` (rename `-DONE`; clean tmp artifacts; ensure branch committed; merge to `main`; push `main` → `origin` ONLY)
- Step 6: Finish (Planner — context.md closing update verified done via Task 5/Step 5; short resume + next-TODO text to user)

## Per-Task Pre-Analysis

### Task 1 — Sibling-folder enumeration (NOT front-end)

- New pure-logic module (architector names it, precedent `folder_listing.rs` / `sibling_folder_listing`): one non-recursive read of the parent directory of the current image's folder; directories ONLY (exclusive listing — regular files excluded); exclusion of self (`.`/current folder name) resolved by the architector; NEVER grandparent-level or deeper entries.
- Sort: the EXACT Wave-2 collation (lowercase-folded alphanumeric with exact-name tie-break; reference `tech.md` mechanism section) — reuse/extract shared collation where sensible WITHOUT touching Wave-2 behavior (architector decides extraction vs duplication with rationale; no existing-plan edits).
- Expected tests (Task 4 locks them, but module ships with its own tests per Wave-2 precedent): sort order, folder-only enumeration, sibling-scoping.
- 4.2 lands module + tests; 4.3 review/simplify; 4.5b adherence vs the 4.1b plan.

### Task 2 — Session last-viewed memory (NOT front-end; the wave's deliverable)

- RAM-only map field(s) on `ImagesScrollerApp`: folder path → last viewed image path for every folder visited this session; updated whenever an image becomes the displayed image (successful `open_image`, including the initial CLI/open-dialog open).
- No persistence: no file writes, no settings storage — verified by ABSENCE of any write path + test seam if the architector deems it valuable.
- The map update point is decided in 4.1b (likely inside the success transition of the open flow), keeping the update single-sourced.

### Task 3 — Target image priority rule: keyboard + buttons (FRONT-END → 4.1a before 4.1b; 4.5a after 4.3)

- 4.1a frontend-specialist produces the front-end spec: `ArrowUp`/`ArrowDown` intake (egui 0.37 `key_pressed`, per-frame, direct navigation), pill ↑/↓ button wiring (`ArrowControl` gains folder-direction wiring; accessible button labels for folder navigation), enablement semantics (folder-neighbor resolution per frame), and the 4.5a conformance checklist.
- 4.1b architector settles ALL TODO "Deferred decisions" and MUST encode the proposals for the Planner/user:
  - UI layout placement: PRE-SETTLED by the 0.2.1 operator-directed pill — ↑/↓ already exist as disabled reserves in the floating bottom pill; Wave 3 activates them in place (no new layout); confirm with user via per-task approval if needed.
  - Empty/no-image sibling-folder handling — pre-analysis position: DISABLE the direction button when the adjacent folder has no displayable first image (list edges and no-image folders both resolve to "no neighbor"); keyboard equally does nothing. The architector confirms/revises and states it.
  - Edge-of-list behavior — pre-analysis position: first/last sibling folder stays silently (could be the Wave-2 precedent).
  - Unit-test seams around folder I/O — pre-analysis position: pure-logic modules with injectable directory-read seam (Wave-2 pattern), no direct std::fs calls in tested logic.
- Priority rule implementation: `navigate` extended with folder directions; target resolution = (a) session last-viewed image of the target folder if visited this session; (b) else the target folder's FIRST image in the Wave-2 collation; displayed image update immediately refreshes the last-viewed map entry for the NEW folder.
- NO same-filename attempt, NO other fallback (binding Addendum v2 item 3).

### Task 4 — Verification (NOT front-end)

- Unit tests (container-runnable) covering, at minimum: same-parent restriction (no grandparent/deeper), folder sort order, target-image priority (visited → last-viewed; new folder → first image), last-viewed map set/get + session lifetime (map dies with app — struct-scoped evidence), folder-neighbor edges.
- Dev checks through the harness per `docs/build.md` via `alpine-vm` MCP (`vm_status` first; exit code authoritative) ending `ALL CHECKS PASSED`.
- Release exe rebuild (`scripts/build-windows.sh`) so the operator can run the manual UX checklist on a real Windows 10/11 machine; DLL import audit re-checked against the recorded 22-DLL OBSERVED list if the tooling makes it cheap (guidance: follow `docs/build-windows.md`).
- OPERATOR-RUN (user-side): up/down keys/buttons move folders under the same parent; revisiting a folder this session reopens its last viewed image; closing and relaunching loses the memory. Stays unchecked in the TODO until the user reports.

### Task 5 — Documentation updates (NOT front-end)

- `README.md`: vertical scroll feature + the corrected target rule (no same-filename rule).
- `tech.md`: session-memory component record + corrected navigation rule + updated test census/mechanism sections.
- `architecture.md`: Status line + Navigation Engine vertical slice moved to LANDED + module layout entry for the new module(s).
- `.agent/project-structure.md`: add the new module file(s).
- `context.md`: closing update (recent changes, current state, next steps = operator re-test of 0.2.1 carried + Wave-3 operator validation + next wave pointer).
- If doc files exceed 100 lines keep TOCs accurate; do not duplicate the OBSERVED DLL list (docs-split rule).

## Risk / Escalation Notes

- vboxsf EBUSY on container manifest writes → authorized host-side write fallback is the documented pattern (from prior waves).
- Any ambiguity about deferred decisions UNRESOLVED by the user answer at task-plan approval => implementer STOPS and asks the caller instead of guessing.
- On any sub-agent failure: resume/retry in smaller steps per workflow; escalate to user on repeat failures.
