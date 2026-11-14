# Wave 6.2 Global Plan — "Open with..." Launch Still Fails: Diagnostic + Evidence-Driven Fix

- Source TODO: `.agent/todos/20261113/20261113-todo-2.md` (Pattern C: 4 tasks; each gets its own 4.1–4.6 cycle; Task 1 (diagnostic) gates the wave and REQUIRES explicit caller authorization; Task 2 consumes the diagnostic data).
- Requirements source of truth: `.agent/project-info/brief.md` → `### Fast "Open with..." Menu` (binding: selecting a program opens the current file).
- Base state: `main` == `origin/main` == `b458897`; version 0.6.1; census 189/28; exe 19,645,219 B; DLL audit set-equal 22; working tree has the new TODO + context.md report entries (committed at Step 2).
- Git policy: branch `fix/wave-6-2-open-with-launch` created ONLY at Step 2; `git push` ONLY at Step 5 (`main` → `origin` ONLY); Gitignore Compliance Rule before every commit. Do NOT touch `CHANGELOG.md`. Do NOT edit existing `.kilo/plans/*` files.
- Build policy (ALL tasks): Alpine-VM container via the `alpine-vm` MCP — `vm_status` FIRST; exit code authoritative; VM/share unavailable → STOP + user question.
- Version: 0.6.1 → 0.6.2 (patch); Cargo.lock refreshed at Step 3 (Wave-6 precedent: `cargo update -p img-scroller` in the container).

## Global Pre-Analysis

- D1 persists at 100% on the 0.6.1 exe after the Suspect-A fix → the remaining failure lives on a surface the agent-side audit could not observe (the runtime Invoke chain). The diagnostic build converts the invisible failure into evidence: per-selection recorded outcomes for the re-enumeration, the `launch_index` resolution, the data-object construction (`SHCreateItemFromParsingName`/`BindToHandler`), and the `IAssocHandler::Invoke` call.
- Diagnostic build mechanics (pre-analysis for the architector): the debug profile does NOT apply the release `windows_subsystem` attribute (it is `cfg_attr(not(debug_assertions))`), so a debug cross-build (`cargo build --locked --target x86_64-pc-windows-gnu`, no `--release`) produces an exe WITH a console window — a natural diagnostic surface via `eprintln!` lines the operator can copy back. The exe lands in the container's CARGO_TARGET_DIR (`/cargo-target/x86_64-pc-windows-gnu/debug/img-scroller.exe`) and must be copied to the gitignored `dist/` for the operator (a documented manual copy step; dist/ is gitignored, nothing is committed). The diagnostic code (e.g. `#[cfg(debug_assertions)]`-gated or a temporary hardcoded block — the architector pins ONE) is removed before the wave's release rebuild so the 0.6.2 artifact carries zero diagnostic code.
- Front-end marks: Tasks 1–2 = NO (platform/service layer + temporary instrumentation). Task 3 = NO. Task 4 = NO. (If the evidence-decided fix in Task 2 turns out to touch UI gating, the architector re-marks and 4.1a/4.5a run then.)
- Checklist clarity: the Task-3 4.1b verification plan MUST include the OPERATOR-RUN checklist in PLAIN self-contained form, and the Planner presents it in the chat at the wave close.

## Step Map (each entry = a separate `task` tool invocation)

- Step 2: Git Feature Branch Setup → implementer (commit the TODO + context.md report entries; create + switch `fix/wave-6-2-open-with-launch`)
- Step 3: Version Update → implementer (0.6.1 → 0.6.2; `chore: bump version to 0.6.2`)
- Task 1 (diagnostic): 4.1b Diagnostic-Build Plan → architector
- Task 1: 4.2 Implementation → implementer (debug build + the operator handoff instructions; NOT committed permanently — see plan)
- Task 1: 4.3 Code Review & Simplification → code-reviewer & code-simplifier (review the instrumentation; fixes → implementer)
- Task 1: 4.4 Documentation → docs-specialist (module doc note; expected minimal)
- Task 1: 4.5b Overall Plan Adherence → architector
- Task 1: 4.6 Task Completion → implementer (mark; the diagnostic build handed to the operator)
- [OPERATOR RUN: the user runs the diagnostic build and reports the lines — the wave PAUSES for the data]
- Task 2 (root-cause + fix): 4.1b Root-Cause + Fix Plan → architector (with the operator data; any semantics change → caller approval question)
- Task 2: 4.2 Implementation → implementer (the fix + the diagnostic-code REMOVAL)
- Task 2: 4.3 Code Review & Simplification → code-reviewer & code-simplifier (fixes → implementer)
- Task 2: 4.4 Documentation → docs-specialist
- Task 2: 4.5b Overall Plan Adherence → architector
- Task 2: 4.6 Task Completion → implementer
- Task 3 (Verification): 4.1b Verification Plan → architector (PLAIN checklist)
- Task 3: 4.2 Verification Execution → implementer (dev-checks; release rebuild 0.6.2 with ZERO diagnostic code; DLL audit; sweep; checklist)
- Task 3: 4.3 Review & Simplification → code-reviewer & code-simplifier (expected no-op; fixes → implementer)
- Task 3: 4.4 Documentation → docs-specialist (expected no-op)
- Task 3: 4.5b Overall Plan Adherence → architector
- Task 3: 4.6 Task Completion → implementer (sub-items 1+2 `[x]`; manual-UX STAYS `[ ]`)
- Task 4 (Documentation): 4.1b Documentation Plan → architector
- Task 4: 4.2 Documentation Execution → docs-specialist
- Task 4: 4.3 Review & Simplification → code-reviewer & code-simplifier (doc accuracy)
- Task 4: 4.4 Documentation → docs-specialist (guides check)
- Task 4: 4.5b Overall Plan Adherence → architector
- Task 4: 4.6 Task Completion → implementer
- Step 5: TODO File Completion → implementer (rename → `-DONE`; tmp sweep incl. the diagnostic build artifacts in dist/; merge fast-forward into `main`; delete branch; push `main` → `origin` ONLY)
- Step 6: Finish → planner (final `context.md` closing touch; the PLAIN OPERATOR-RUN checklist in the chat; next-chat handoff text)

## Cycle Details

### Cycle A — Task 1 (diagnostic build)

- 4.1b architector: pin the diagnostic shape (the exact temporary instrumentation, its gating mechanism, the recorded fields, the console-line format, the copy-to-dist step for the operator, the removal mechanism for Task 2) into `.kilo/plans/20261113-wave6-2-task1-diagnostic-build-implementation-plan.md`. The diagnostic must be MINIMAL, temporary, and never committed in a form that survives to the release rebuild (the plan pins whether it is `#[cfg(debug_assertions)]`-gated code that may stay in the tree harmlessly, or a hardcoded temporary block that MUST be removed in Task 2 — pick ONE and encode it).
- 4.2 implementer: implement + build the debug exe; verify it compiles for both targets; hand the operator instructions (path + what to do + what to copy back).
- 4.3 reviewers (lightweight — instrumentation only); 4.4 module doc note; 4.5b adherence; 4.6 mark.
- The wave then PAUSES for the operator's diagnostic data.

### Cycle B — Task 2 (root-cause + fix)

- 4.1b architector: intake the diagnostic lines; evidence-decide the failing mechanism; encode the fix into `.kilo/plans/20261113-wave6-2-task2-launcher-fix-implementation-plan.md`; if a semantics change is involved → the Planner asks the caller BEFORE implementation; include the diagnostic-code removal step.
- 4.2–4.6 as mapped.

### Cycle C — Task 3 (verification)

- 4.1b architector: verification plan at `.kilo/plans/20261113-wave6-2-task3-verification-plan.md` — fresh dev-checks (census corroboration with the settled method: `#[test]`-bearing files), windows-target compile, release rebuild (0.6.2; ZERO diagnostic code — a grep pin proves it), DLL audit (set-equal or explained delta), wave-pin sweeps, Git segmentation, the PLAIN OPERATOR-RUN checklist.
- 4.2 implementer executes; 4.3–4.5 as mapped; 4.6 marks sub-items 1+2 `[x]`, manual-UX stays `[ ]`.

### Cycle D — Task 4 (documentation)

- 4.1b architector: docs plan at `.kilo/plans/20261113-wave6-2-task4-documentation-plan.md` (README/tech/architecture/context: the diagnostic evidence, the root cause, the fix, the 0.6.2 state, remaining open items; the corrected census breakdown verbatim if the count changed; verify-only: project-structure.md/brief.md/build-windows.md).
- 4.2–4.6 as mapped.

## Sub-Task Prompt Requirements (applied to EVERY `task` invocation)

- The workflow-mandated block + TODO path + task description verbatim + this global plan path + per-task plan/spec paths + front-end marks + the VM/branch/push restriction notes.

## Error Handling

- Sub-agent failure/empty response → resume/split; harness/VM unavailability → STOP + user question; max 3 review cycles per 4.3 then escalate; if the diagnostic data cannot decide the failure either → return to the caller with the evidence for a joint decision.

## Addendum — 2026-11-13 Caller Decisions (BINDING, supersedes contradicting lines above)

The caller AUTHORIZED the diagnostic build with two modifications that supersede the console-output and must-remove wording above:

1. **Log-file mechanism**: the diagnostic writes a `.log` FILE while the program runs (location/naming = the architector's, easy for the operator to find, e.g. next to the exe); the operator reports the file's content back — no console copying required.
2. **Optional-module preference**: the diagnostic SHOULD be kept as an OPTIONAL build-time module (included or not via a build option, e.g. a Cargo feature), OFF by default — the release rebuild NEVER enables it — but ONLY if not too hard; the architector decides feasibility and falls back to temporary removable code if the optional-gate is complex. Either way the release artifact runs with ZERO diagnostic activity.
3. The global plan's structure, branch/version/push policies, and cycles are unchanged; Cycle A's Task-1 4.1b plan encodes the two modifications.
