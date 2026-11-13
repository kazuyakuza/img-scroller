# Wave 6.1 Global Plan — "Open with..." Operator Feedback

- Source TODO: `.agent/todos/20261113/20261113-todo-1.md` (Pattern C: 6 tasks; each gets its own 4.1–4.6 cycle; Tasks 2+3+4 MAY be grouped into ONE cycle — related UI intake/styling riding on the same surfaces; Task 1 (D1) is the correctness core and runs FIRST).
- Requirements source of truth: `.agent/project-info/brief.md` → `### Fast "Open with..." Menu` (binding: selecting a program opens the current file).
- Base state: `main` == `origin/main` == `5f4f8ec`; version 0.6.0; census 177/28; exe 19,647,219 B; DLL audit set-equal 22; working tree has the new TODO + context.md report entries (committed at Step 2).
- Git policy: branch `fix/wave-6-1-open-with-feedback` created ONLY at Step 2; `git push` ONLY at Step 5 (`main` → `origin` ONLY); Gitignore Compliance Rule before every commit. Do NOT touch `CHANGELOG.md`. Do NOT edit existing `.kilo/plans/*` files.
- Build policy (ALL tasks): Alpine-VM container via the `alpine-vm` MCP — `vm_status` FIRST; exit code authoritative; VM/share unavailable → STOP + user question.
- Version: 0.6.0 → 0.6.1 (patch — defect-fix round on the 0.6.x line, Wave-4.1/4.2/5.1/5.2 precedent); Cargo.lock refreshed at Step 3 like Wave 6's Step 3.

## Global Pre-Analysis

- D1 is a runtime-verification defect class (Wave 5.1/5.2 precedent): the launcher was compile-verified but never ran on real Windows. The root-cause audit (Task 1 4.1b) prioritizes: (A) the `IDataObject` construction the handlers reject; (B) the `launch_index` pairing between the stored menu list and the launch-time re-enumeration; (C) COM apartment/swallowed HRESULTs; (D) association kinds that cannot launch. The architector researches the exact windows-0.62.2 `IAssocHandler::Invoke` contract (what registered handlers expect from the passed data object) with source-cited facts before picking the fix; a semantics change needs caller approval; a diagnostic debug build needs caller authorization.
- Tasks 2/3/4 group into one cycle: the `Q` hotkey + caption, the modal gating (intake gated while the menu is open; ESC/Q closes; outside-click behavior decided), and the warning-colored failure message all touch `open_with_button.rs`/`keyboard_intake.rs`/`button_hint.rs`/the message-line presentation. Grouping avoids three conflicting edit rounds over the same files.
- Front-end marks: Tasks 2, 3, 4 = front-end related (4.1a + 4.5a for the grouped cycle). Task 1 = NO (platform/service layer; the fix's user-visible surface is the launch hand-off). Task 5 = NO. Task 6 = NO.
- Checklist clarity (operator item 6): the Task-5 4.1b verification plan MUST include the OPERATOR-RUN checklist in plain self-contained form, and the Planner presents it in the chat at the wave close (not only inside a plan file).

## Step Map (each entry = a separate `task` tool invocation)

- Step 2: Git Feature Branch Setup → implementer (commit the TODO + context.md report entries; create + switch `fix/wave-6-1-open-with-feedback`)
- Step 3: Version Update → implementer (0.6.0 → 0.6.1; Cargo.lock refresh; `chore: bump version to 0.6.1`)
- Task 1 (D1): 4.1b root-cause + fix plan → architector
- Task 1: 4.2 Implementation → implementer
- Task 1: 4.3 Code Review & Simplification → code-reviewer & code-simplifier (fixes → implementer)
- Task 1: 4.4 Documentation → docs-specialist
- Task 1: 4.5b Overall Plan Adherence → architector
- Task 1: 4.6 Task Completion → implementer
- Tasks 2+3+4 (grouped cycle): 4.1a Front-end Spec → frontend-specialist
- Tasks 2+3+4: 4.1b Implementation Plan → architector
- Tasks 2+3+4: 4.2 Implementation → implementer
- Tasks 2+3+4: 4.3 Code Review & Simplification → code-reviewer & code-simplifier (fixes → implementer)
- Tasks 2+3+4: 4.4 Documentation → docs-specialist
- Tasks 2+3+4: 4.5a Front-end Verification → frontend-specialist
- Tasks 2+3+4: 4.5b Overall Plan Adherence → architector
- Tasks 2+3+4: 4.6 Task Completion → implementer
- Task 5 (Verification): 4.1b Verification Plan → architector (checklist in PLAIN form)
- Task 5: 4.2 Verification Execution → implementer
- Task 5: 4.3 Review & Simplification → code-reviewer & code-simplifier (expected no-op; fixes → implementer)
- Task 5: 4.4 Documentation → docs-specialist (expected no-op)
- Task 5: 4.5b Overall Plan Adherence → architector
- Task 5: 4.6 Task Completion → implementer (sub-item 1 + rebuild sub-item `[x]`; manual-UX STAYS `[ ]`)
- Task 6 (Documentation): 4.1b Documentation Plan → architector
- Task 6: 4.2 Documentation Execution → docs-specialist
- Task 6: 4.3 Review & Simplification → code-reviewer & code-simplifier (doc accuracy)
- Task 6: 4.4 Documentation → docs-specialist (guides check)
- Task 6: 4.5b Overall Plan Adherence → architector
- Task 6: 4.6 Task Completion → implementer
- Step 5: TODO File Completion → implementer (rename → `-DONE`; tmp sweep; merge fast-forward into `main`; delete branch; push `main` → `origin` ONLY)
- Step 6: Finish → planner (final `context.md` closing touch; present the PLAIN OPERATOR-RUN checklist in the chat; next-chat handoff text)

## Cycle Details

### Cycle A — Task 1 (D1 root-cause + fix)

- 4.1b architector: evidence-driven suspect audit (A–D above) against source-cited windows-0.62.2 facts; picks ONE fix; encodes it into `.kilo/plans/20261113-wave6-1-task1-launcher-fix-implementation-plan.md` (tiny steps, exact files/signatures/tests, harness runs, commit messages). Scope boundaries: enumeration list content, refresh contract, state gates, and standing behaviors preserved.
- 4.2 implementer; 4.3 reviewers; 4.4 module docs; 4.5b adherence; 4.6 mark.
- If the audit cannot decide a runtime failure → STOP + DIAGNOSTIC fallback question to the caller (user).

### Cycle B — Tasks 2+3+4 (Q hotkey + modal menu + warning message)

- 4.1a frontend-specialist: spec for the `Q` caption under the symbol (the established 9-pt caption pattern), the modal intake-gating UX, the ESC/Q-close interactions, the outside-click behavior options, and the warning-color options for the pill's message line; save to `.kilo/plans/20261113-wave6-1-tasks2-4-ui-feedback-frontend-spec.md`. Binding user decisions are already fixed (Q; all-functionalities-OFF; ESC/Q closes; warning color) — the spec encodes HOW, not WHETHER.
- 4.1b architector: encode the spec (intake gating mechanism composing with `keyboard_intake`; popup input-capture facts for egui 0.37; exact color constants; exact tooltip wording) into `.kilo/plans/20261113-wave6-1-tasks2-4-ui-feedback-implementation-plan.md`; expected test census pinned.
- 4.2–4.6 as mapped.

### Cycle C — Task 5 (verification)

- 4.1b architector: verification plan at `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` — fresh dev-checks run (census corroboration), windows-target compile, release rebuild + byte record (0.6.1), DLL audit (set-equal or explained delta), wave-pin sweeps, Git segmentation, and the OPERATOR-RUN checklist in PLAIN self-contained numbered form (the user's clarity requirement).
- 4.2 implementer executes; 4.3–4.5 as mapped; 4.6 marks sub-items 1+2 `[x]`, manual-UX stays `[ ]`.

### Cycle D — Task 6 (documentation)

- 4.1b architector: docs plan at `.kilo/plans/20261113-wave6-1-task6-documentation-plan.md` (README/tech/architecture/context: the D1 root cause + fix mechanism, Q hotkey, modal behavior, warning color, 0.6.1 state, remaining open items; verify-only: project-structure.md/brief.md/build-windows.md).
- 4.2 docs-specialist executes; 4.3–4.6 as mapped.

## Sub-Task Prompt Requirements (applied to EVERY `task` invocation)

- The workflow-mandated block + TODO path + task description verbatim + this global plan path + per-task plan/spec paths + front-end marks + the VM/branch/push restriction notes.

## Error Handling

- Sub-agent failure/empty response → resume/split; harness/VM unavailability → STOP + user question; max 3 review cycles per 4.3 then escalate; runtime-only root-cause dead-ends → the TODO's DIAGNOSTIC fallback (caller authorization).
