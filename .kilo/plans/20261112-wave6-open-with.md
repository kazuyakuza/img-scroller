# Wave 6 Global Plan — Fast "Open with..." Menu

- Source TODO: `.agent/todos/20261029/20261029-todo-8.md` (Pattern C: `# Title` → `## Tasks` → 5 `###` headings = 5 tasks; each gets its own 4.1–4.6 cycle; Tasks 1+2 are grouped into ONE cycle per the workflow's "extremely short/related" allowance — justification below).
- Requirements source of truth: `.agent/project-info/brief.md` → `### Fast "Open with..." Menu` (+ `## Important Note for AI Agents`). Binding corrections specific to this wave: NONE.
- Current base: `main` == `origin/main` == `f724351`; version 0.5.2; census 164 unit tests / 26 test files; exe `dist/windows-x64/img-scroller.exe` 19,605,828 B; DLL audit set-equal 22.
- Git policy: branch creation ONLY at Step 2; `git push` ONLY at Step 5 (to `origin` ONLY); Gitignore Compliance Rule before every commit. Do NOT touch `CHANGELOG.md`. Do NOT edit existing `.kilo/plans/*` files (new plan files only).
- Build policy (ALL tasks): every Rust/Cargo command runs in the container on the Alpine VM via the `alpine-vm` MCP — `vm_status` FIRST; the command exit code is the authoritative success evidence; if the VM or `/img-scroller` share is unavailable, STOP and return a user question. Host needs NO Rust/Cargo.
- Step 0 (DONE, planner, pre-Step-2): the 0.5.2 acceptance gate was CLOSED (user report "now navigation follows the correc order") — the manual-UX sub-item in `.agent/todos/20261112/20261112-todo-2-DONE.md` marked `[x]` + `context.md` updated. These pending changes are committed by the Step-2 implementer.

## Global Pre-Analysis

- Wave 6 lands brief pillar 3: the "Open with..." menu — enumerate, AT RUNTIME, the programs the Windows system registered for the currently viewed file's type/extension; selecting an entry opens the CURRENT displayed file with that program; NO internal program database.
- Architecture shape: the enumeration + launcher are OS-bound → they live behind the cfg-gated `platform` facade (`src/platform/windows.rs` implementation + `src/platform/stub.rs` container fallback + `src/platform/mod.rs` re-exports), mirroring the landed `decode_image`/`show_open_image_dialog`/`compare_entry_names_system` pattern. Pure logic (parsing/filtering of enumeration results) must be extracted into a container-testable seam (Task 4's testable-seam requirement), mirroring the `EntryNameOrder` comparator-as-data precedent.
- The wave file deliberately does NOT name APIs/crates; the architector steps decide and record them. The architector researches windows-rs 0.62.2 shell/association APIs with source-cited facts before choosing (candidates for research: shell association queries `IQueryAssociations`/`AssocQueryString*` family vs. direct registry enumeration of the OpenWith data; launcher candidates: the shell `ShellExecuteExW` family vs. ProgId command-line composition). The chosen mechanism must minimize new `windows` crate features; adding features is allowed but the Task-4 DLL import audit must account for the delta ("set-equal OR an explained delta") and `docs/build-windows.md`'s OBSERVED list is updated in Task 5 docs if it changed.
- Front-end marks: Task 3 = front-end related (4.1a + 4.5a). Tasks 1+2 = NO (service layer only; UI wiring belongs to Task 3). Task 4 = NO. Task 5 = NO.
- UI placement is propose-and-ask (binding): Task 3's 4.1a produces a CONCRETE layout proposal (recommended option + alternatives + egui 0.37 API facts), and the Planner MUST present it to the user for approval BEFORE 4.1b finalizes the plan. The empty-result behavior decision is bundled into the same user question.
- Version: 0.5.2 → 0.6.0 (minor — new v1 feature wave). `Cargo.lock` carries the package version and must be refreshed in Step 3 (verify with the harness's `--locked` checks; the exit code is the evidence).
- Grouping justification (Tasks 1+2): Task 2's deferred launcher-mechanism decision is explicitly assigned to "THIS wave's architector (4.1b) step" together with the enumeration mechanism; Task 2's implementation surface is the launcher hand-off that only exists once the enumeration result type exists; one 4.x cycle avoids a duplicate architector decision. Task 2's sub-item is marked in the same 4.6 step.

## Step Map (each entry = a separate `task` tool invocation)

- Step 2: Git Feature Branch Setup → implementer
- Step 3: Version Update → implementer
- Tasks 1+2 (grouped cycle A): 4.1b Analysis & Planning → architector
- Tasks 1+2: 4.2 Implementation → implementer
- Tasks 1+2: 4.3 Code Review & Simplification → code-reviewer & code-simplifier (fixes → implementer)
- Tasks 1+2: 4.4 Documentation → docs-specialist
- Tasks 1+2: 4.5b Overall Plan Adherence → architector
- Tasks 1+2: 4.6 Task Completion → implementer
- Task 3 (cycle B): 4.1a Front-end Spec + layout proposal → frontend-specialist
- Task 3: [PLANNER] user approval of layout + empty-result behavior → `question` tool (binding propose-and-ask; re-run 4.1a on feedback/rejection)
- Task 3: 4.1b Implementation Plan → architector
- Task 3: 4.2 Implementation → implementer
- Task 3: 4.3 Code Review & Simplification → code-reviewer & code-simplifier (fixes → implementer)
- Task 3: 4.4 Documentation → docs-specialist
- Task 3: 4.5a Front-end Verification → frontend-specialist
- Task 3: 4.5b Overall Plan Adherence → architector
- Task 3: 4.6 Task Completion → implementer
- Task 4 (cycle C): 4.1b Verification Plan → architector
- Task 4: 4.2 Verification Execution → implementer
- Task 4: 4.3 Code Review & Simplification → code-reviewer & code-simplifier (expected no-op; fixes → implementer if any)
- Task 4: 4.4 Documentation → docs-specialist (expected no-op message acceptable)
- Task 4: 4.5b Overall Plan Adherence → architector
- Task 4: 4.6 Task Completion → implementer (unit-test sub-item `[x]`; manual-UX sub-item STAYS `[ ]`)
- Task 5 (cycle D): 4.1b Documentation Plan → architector
- Task 5: 4.2 Documentation Execution → docs-specialist (markdown-generation rule: implementer is HARD-BLOCKED from doc-file edits; docs-specialist executes)
- Task 5: 4.3 Code Review & Simplification → code-reviewer (doc-accuracy review) & code-simplifier (expected no-op)
- Task 5: 4.4 Documentation → docs-specialist (AI-agent guides/examples check; no-op message acceptable)
- Task 5: 4.5b Overall Plan Adherence → architector
- Task 5: 4.6 Task Completion → implementer
- Step 5: TODO File Completion → implementer (rename todo-8 → `-DONE`; tmp sweep; all-committed check; merge `feat/wave-6-open-with` fast-forward into `main`; delete branch after verified merge; push `main` → `origin` ONLY)
- Step 6: Finish → planner (final `context.md` closing touch + session summary + next-chat handoff text)

## Per-Task Pre-Analysis & Cycle Details

### Cycle A — TODO Tasks 1+2 (runtime enumeration + launcher)

- TODO Task 1 (sub-item 1): enumerate the programs the Windows system registered for the CURRENT file's type/extension, at runtime, from system registration data ONLY — NO internal/hardcoded program database. Sub-item 2: the mechanism is decided in this architector step (the wave file must not name APIs).
- TODO Task 2: selecting an entry opens the CURRENT displayed file with the chosen program (hand-off via the decided launcher mechanism).
- 4.1b architector MUST decide (all four deferred items assigned to this step):
  1. Enumeration mechanism (APIs/crates; source-cited windows-rs 0.62.2 facts; exact `windows` crate features needed — prefer the already-present six; additions allowed).
  2. Launcher mechanism (APIs; how the selected program + current file are handed off; elevation/verb considerations documented).
  3. Refresh/re-open timing (per-invocation fresh enumeration vs session cache — pick ONE and encode it).
  4. The testable seam shape for the pure logic: the enumeration result type (program name + launch data), the parse/filter functions pure and container-testable with injected synthetic data (no registry in the container), the facade function signatures, and the stub behavior (`stub.rs` → empty enumeration / no-op-safe launcher).
- 4.1b plan saved to `.kilo/plans/20261112-wave6-tasks1-2-enumeration-launcher-implementation-plan.md` (auto-approve mode if the user chose "Approve Global and Tasks Plans"; else present).
- 4.2 implementer: implement EXACTLY the plan (platform facade functions, pure seam module + tests, stub path; NO menu UI — that is Task 3; NO app-state wiring beyond what the plan specifies); commits with meaningful messages; harness checks if the plan requires.
- 4.3 reviewers; 4.4 module docs; 4.5b adherence; 4.6 mark Tasks 1+2 `[DONE]` + commit.

### Cycle B — TODO Task 3 (menu entry in the UI) — FRONT-END

- TODO sub-items: the "Open with..." menu entry/portal in the viewer UI; placement follows propose-and-ask (same pattern as Waves 2–4: concrete proposal → ASK → finalize); empty-result behavior decided in the architector step.
- 4.1a frontend-specialist: propose CONCRETE placement options with a RECOMMENDED one (candidates to weigh: a pill-row button opening an in-app popup menu; a right-click context menu on the image surface; a keyboard shortcut + centered list popup) with egui 0.37 API facts (`menu_button`/`popup` mechanics, painted-glyph consistency with the pill's no-font-dependency rule), the menu's interaction with the fullscreen chrome gate, and the 4.5a conformance checklist. Save to `.kilo/plans/20261112-wave6-task3-open-with-menu-frontend-spec.md`.
- PLANNER asks the user: approve the proposed placement (recommended option first) + the proposed empty-result behavior; feedback/rejection → redo 4.1a.
- 4.1b architector: encode the APPROVED layout + empty-result behavior + the enumeration-to-menu wiring (which cycle-A seams the UI consumes; how the pill row/button row changes; how the CURRENT displayed file path is resolved — note the empty-folder browsing mode has NO open file: the entry is hidden/disabled there per the decided empty-result rule) into `.kilo/plans/20261112-wave6-task3-open-with-menu-implementation-plan.md`.
- 4.2 implementer; 4.3 reviewers; 4.4 docs; 4.5a frontend verification (vs the 4.1a spec checklist); 4.5b adherence; 4.6 mark + commit.

### Cycle C — TODO Task 4 (verification)

- TODO sub-items: (1) unit tests where pure logic exists — run via the dev-checks harness in the container per `docs/build.md` (`ALL CHECKS PASSED`: fmt/check/test/clippy; `alpine-vm` MCP; `vm_status` first; exit code authoritative); (2) manual UX check = OPERATOR-RUN on real Windows 10/11 (agent-side GUI/runtime execution is impossible from the Linux container): for a jpg/png file the menu lists at least the system-associated programs; selecting one opens THIS exact file in that program; NOT dependent on any app-maintained list.
- 4.1b architector: verification plan → `.kilo/plans/20261112-wave6-task4-verification-plan.md`: fresh harness dev-checks run; test-census corroboration (expected count vs the cycle-A/B additions); wave-pin sweeps (the new seam/function call sites; the menu render path; feature additions in `Cargo.toml`); release rebuild via `scripts/build-windows.sh` + byte-size record; DLL import audit vs the recorded 22-DLL constant (set-equal OR an explained delta for any new shell DLLs); Git segmentation; the OPERATOR-RUN checklist (todo Task-4 sub-item 2 verbatim + standing regressions: pan/edge-navigate, ←/→/↑/↓ lists, silent edges, buttons, I/O/0/W/H, ESC contract, alias keys, captions, launch state).
- 4.2 implementer: EXECUTE the plan (harness + rebuild + audit; hand the OPERATOR-RUN checklist to the caller; the manual-UX sub-item stays `[ ]`).
- 4.3–4.5 as mapped (no-op messages acceptable where nothing to review/document); 4.6 mark sub-item 1 `[x]` + commit (sub-item 2 stays `[ ]` — the user's gate).

### Cycle D — TODO Task 5 (documentation updates)

- TODO sub-item: `README.md`, `tech.md`, `architecture.md`, `context.md`: record the enumeration mechanism chosen, constraints, remaining open items.
- 4.1b architector: docs plan → `.kilo/plans/20261112-wave6-task5-documentation-plan.md`: README (features/controls: the new menu entry), tech.md (a `## Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)` section: mechanism, seams, refresh timing, empty-result rule, dependency/feature changes if any), architecture.md (Logical Components "Fast Open-with Service" → LANDED; Module Layout additions; Data and State if session state was added; Technical Decisions Summary row), context.md (the wave entry + Next Steps: the pending 0.6.0 OPERATOR-RUN gate), plus `.agent/project-structure.md` verify/update for any new files, and `docs/build-windows.md` OBSERVED-DLL list update IF the Task-4 audit recorded a delta. Constraints: no CHANGELOG.md; no existing-plan edits.
- 4.2 docs-specialist executes; 4.3 review; 4.4 guides check; 4.5b adherence; 4.6 mark + commit.

## Sub-Task Prompt Requirements (applied to EVERY `task` invocation)

- Include the workflow-mandated block (SINGLE DISCRETE STEP; TOP PRIORITY the TODO file; tools/gitignore rules; completion signal; no assumptions).
- Include: TODO path (`.agent/todos/20261029/20261029-todo-8.md`), the task description verbatim, this global plan path, the per-task plan/spec path once produced, the front-end mark, and the VM/branch/push restriction notes (branch = Step 2 only; version = Step 3 only; push = Step 5 only).

## Error Handling

- On sub-agent failure/empty response: resume with a continuation request; split smaller if repeated.
- On harness/VM unavailability: STOP → user question.
- Max 3 review cycles per 4.3; then escalate to the user.
