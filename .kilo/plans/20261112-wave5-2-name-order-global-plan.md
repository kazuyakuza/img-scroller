# Global Plan — Session 2026-11-12 (round 2): Wave 5.2 (Name-Order Round 2)

Session request: process the second 2026-11-12 operator report as TODO file `.agent/todos/20261112/20261112-todo-2.md` — Wave 5.2, version 0.5.1 → 0.5.2 (patch), branch `fix/wave-5-2-name-order`. Two of three Wave-5.1 gate items PASS (Shift+'+' alias; popup expiry); the name-order defect PERSISTS (the `CompareStringEx` fix did not take effect on real Windows). The operator's diagnostic data (the file-name list as text + observed order + expected order) is requested in parallel with the approval — Task 2 consumes it.

Pre-analysis: base = `main` == `origin/main` == `a11c080` (clean). Three suspect classes (TODO Task 2): **A — the silent fallback** (most probable: `compare_entry_names_system` → `None` on ANY runtime failure → silently falls back to the OLD code-point order; the Windows-side code was compile-verified only — never runtime-verified agent-side); **B — wiring** (any production construction path of the two listings still defaulting to the pure order); **C — expectation gap** (the operator's expected order vs user-default-locale collation: digit-in-bracket lexical-vs-natural ordering, Japanese block order, punctuation). The container harness CANNOT execute the Windows-side code (stub path) — runtime verification is OPERATOR-RUN only; the release cross-build compile-checks it.

## Workflow steps

### Step 2 — Git Feature Branch Setup → implementer
- Clean-tree check; verify `main` @ `a11c080`; create + switch to `fix/wave-5-2-name-order`; commit the untracked Wave-5.2 todo + global plan (two `docs:` commits per precedent).

### Step 3 — Version Update → implementer
- `Cargo.toml` + `Cargo.lock` `0.5.1` → `0.5.2` (hand-aligned lock per precedent); commit `chore: bump version to 0.5.2`.

### Task 1 — Record the second report (NOT front-end; light cycle)
- 4.1b architector plan → 4.2 implementer (context.md dated entry: the two PASSes; the persisting name-order defect; the gate sub-item stays `[ ]`; the data request) → 4.3 reviewers (no-change expected) → 4.4 N/A → 4.5b → 4.6 mark.

### Task 2 — Root-cause (NOT front-end → 4.1b only; the OPERATOR DATA + code audit cycle)
- 4.1b architector: consumes the operator's text data (names/observed/expected) + performs the three-suspect audit per the TODO (source-cited windows-0.62.2 API facts for the runtime-failure paths; the wiring audit across ALL production construction paths; the expectation analysis with the real names). DECIDES the fix class:
  - (a) runtime-bug repair (Suspect A) → encode the exact repair in the plan;
  - (b) wiring repair (Suspect B) → encode it;
  - (c) mechanism/semantics adjustment (Suspect C) → **STOP + caller approval** before encoding a semantics change;
  - (d) undecidable runtime-only → **STOP + diagnostic-build question** (caller authorization).
- The plan encodes the fix + its tests + commit segmentation; harness deferred to Task 4.
- 4.2 implementer → 4.3 reviewers (+fix) → 4.4 docs-specialist → 4.5b → 4.6 mark.

### Task 3 — Fix implementation (NOT front-end → 4.1b only; per the Task-2 decision)
- 4.1b architector plan (if Task 2's cycle did not already land the fix — the grouping decision: if the fix class was decided AND encoded in Task 2's plan, Tasks 2–3 MAY run as one combined cycle per the Critical Workflow's grouping clause; the todo keeps two headings and both get marked at the shared 4.6) → 4.2 implementer → 4.3 reviewers (+fix) → 4.4 docs-specialist → 4.5b → 4.6 mark (both tasks).

### Task 4 — Verification (NOT front-end; light cycle)
- 4.1b architector verification plan (house format): preflight census (164 + the fix's test delta); discovery dev-checks with conditional branches; wave-pin sweeps (the fix's surface + the standing no-touch list); release rebuild (0.5.2 exe; record bytes; no `.part`); DLL audit (set-equal 22 or explained delta → STOP + caller report on any real delta); gitignore sweep; commit segmentation; the §-manual UX checklist copied verbatim from the todo's Task 4 third sub-item (the name-order re-test with the operator's REAL folder + the standing regressions); manual-UX sub-item stays `[ ]`.
- 4.2 implementer → 4.3 reviewers (light) → 4.4 N/A → 4.5b → 4.6 mark.

### Task 5 — Documentation updates (NOT front-end; light cycle)
- 4.1b architector docs plan → 4.2 docs-specialist writes: README/tech/architecture/context (root cause + fix + 0.5.2 state; any staleness the docs review flags), brief verify-only → 4.3 reviewers (light) → 4.5b → 4.6 mark.

### Step 5 — TODO File Completion → implementer
- Rename `20261112-todo-2.md` → `-DONE`; tmp sweep; checkout `main`; fast-forward merge; verify + delete branch; push `main` → `origin` ONLY; notify on failure.

## Session closing
- context.md final update (docs-specialist in Task 5's write covers the wave content; one closing touch after Step 5 for the merged/pushed state) → Workflow Step 6 finish message: resume + the next-session prompt (Wave 6 = `.agent/todos/20261029/20261029-todo-8.md`, "Open with..." menu).

## Error handling
- Sub-agent failure/empty response → resume the same step; repeated failure → split smaller; endless failure → escalate to the user.
- Harness/VM/share unavailability → STOP + user question.
- Suspect-C semantics changes and the diagnostic build → STOP + question tool (caller authorization).
- Max 3 review cycles per task, then escalate.
