# Wave 6 — Task 5 Documentation Adherence (4.5b) Corrective Plan: context.md Current-Project-State Coherence

**Source:** Architector step 4.5b (Overall Plan Adherence) for TODO Task 5 (Documentation) of Wave 6, run at HEAD `1ae15b3` on `feat/wave-6-open-with`.
**Verdict of the 4.5b pass:** docs-plan prescription list (README R1–R6, tech T1–T7, architecture A1–A8, context C1–C3) FULLY applied; all six review-fix coherence corrections (commit `1ae15b3`) landed correctly; cross-doc facts consistent. **ONE DEVIATION FOUND** (below) → this corrective plan.
**Scope of this fix:** `.agent/project-info/context.md` — the `## Current Project State` section ONLY. No other file, no other section, no code, no TODO, no commit of anything else.

---

## The deviation (why this plan exists)

The Task-5 docs plan (`.kilo/plans/20261112-wave6-task5-documentation-plan.md` §4) scoped context.md to C1 (Current Work Focus) / C2 (Recent Changes) / C3 (Immediate Next Steps) ONLY, and its §8 forbade any other edit. The executor conformed. But EVERY precedent docs plan (Waves 2, 3, 4, 4.1, 4.2, 5, 5.1, 5.2) also refreshed the `## Current Project State` trailing current-state trackers at the docs step, and the Wave-5.2 review-fix explicitly ruled an un-refreshed release-artifact bullet "**fix-worthy — it is a current-state fact that is now wrong after the rebuild**". The Wave-6 4.3 reviewer's sweep did not flag the resulting staleness. Result: context.md now contradicts ITSELF in its two current-state sections.

The five stale current-state facts (all in `## Current Project State`):

1. **Release-artifact bullet** — still reads `19,605,828 bytes, 0.5.2 Wave-5.2 rebuild` while C1/C2 (same file) record the current 0.6.0 artifact at **19,647,219 bytes**.
2. **TODO-state bullet tail** — still says `Remaining TODO files: `20261029-todo-8.md` (Wave 6, the "Open with..." menu — NEXT session)` while Wave 6 is the ACTIVE wave (Tasks 1–4 `[DONE]`, Task 5 landed); no Wave-6 TODO-state sentence was inserted (precedent: Waves 5.1/5.2 inserted theirs at the docs step). Contradicts C1 ("Tasks 1–5 all `[DONE]` except the Task-4 manual-UX sub-item").
3. **Git bullet** — still leads with ``Git: `main` == `origin/main` == `f724351` (working tree clean at this closing step)`` — the Wave-5.2 CLOSING state, not the current `feat/wave-6-open-with` state (precedent: the Git bullet is refreshed at every docs step).
4. **Wave-5.2 bullet gate pointer** — still claims "the pending 0.5.2 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1)" while the 0.5.2 gate is CLOSED (C1/C2 #2 record the PASSED re-test) and Next-Steps #1 is now the 0.6.0 gate. Precedent: the Wave-5.2 docs step added a "SUPERSEDED" note to the Wave-5.1 bullet (caller-adjudicated extension, `.kilo/plans/20261112-wave5-2-task5-review-fix.md` §"Two caller-adjudicated supersession extensions").
5. **No Wave-6 branch bullet** — precedent (Wave-5.1 plan C3b, Wave-5.2 plan 5.4a) inserts the wave's branch bullet as the NEW FIRST bullet of Current Project State at the docs step; the newest bullet is now Wave 5.2, misrepresenting the latest state.

**Explicitly NOT fixed (adjudicated acceptable-as-recorded — do NOT touch):**
- README Project Status's dated Wave-5.2 pending-acceptance sentence (reviewer-passed dated narration; the appended Wave-6 sentence supersedes it in reading order, and the reviewer deliberately left dated wave records).
- The dated 0.5.2 mentions in the tech.md/architecture.md Status paragraphs and per-wave bullets (dated narration).
- The historical Wave-4.1 README sentence ("all ten buttons") — reviewer's recorded deliberate leave.
- context.md's repeated `1.` list-marker style in Recent Changes (the file's own established style, prescribed by the docs plan C2).

---

## Fixes (execute in order; every oldString must be UNIQUE — if not, STOP and return to the caller)

### Fix 1 — Release-artifact bullet (current-state fact)

oldString (exact):

> `- Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,605,828 bytes, 0.5.2 Wave-5.2 rebuild; the 0.5.1 artifact 19,636,215 B overwritten atomically); import audit:`

newString (exact; the rest of the bullet stays byte-identical — Wave-4.1 C-3b precedent):

> `- Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,647,219 bytes, 0.6.0 Wave-6 rebuild; the 0.5.2 artifact 19,605,828 B overwritten atomically); import audit:`

### Fix 2 — TODO-state bullet (two sub-edits)

**Fix 2a — insert the Wave-6 sentence at the very START of the bullet**, immediately after the literal `` - **TODO state**: `` and BEFORE the existing Wave-5.2 sentence (Wave-5.2 plan 5.4b precedent). Insert EXACTLY:

> `.agent/todos/20261029/20261029-todo-8.md` (Wave 6) — Tasks 1–4 marked `[DONE]`; Task 5 documentation landed (the mark is the wave's 4.6 step); the Task-4 manual-UX (OPERATOR-RUN) sub-item STAYS `[ ]` (the user's 0.6.0 gate, Next-Steps #1). 

(single trailing space separates it from the existing Wave-5.2 sentence; keep every existing sentence byte-identical).

**Fix 2b — replace the bullet's tail fragment:**

oldString:

> `Remaining TODO files: `20261029-todo-8.md` (Wave 6, the "Open with..." menu — NEXT session) and `20261029-todo-9.md` (Wave 7); process ONE wave per session, in order.`

newString:

> `Remaining TODO files: `20261029-todo-9.md` (Wave 7 — NEXT session, after Wave 6 closes at global-plan Step 5); process ONE wave per session, in order.`

### Fix 3 — Git bullet (current-state fact)

Replace ONLY the leading clause:

oldString:

> `- Git: `main` == `origin/main` == `f724351` (working tree clean at this closing step); the Wave-5.2 branch`

newString:

> `- Git: branch `feat/wave-6-open-with` at HEAD `1ae15b3` (working tree clean; NOT merged — the rename/merge/push are global-plan Step-5 mechanics); `main` == `origin/main` == `f724351` (the Wave-5.2 closing state); the Wave-5.2 branch`

(the remainder of the bullet — the Wave-5.2 merge history and the earlier-branches sentence — stays byte-identical).

### Fix 4 — Wave-5.2 bullet gate pointer (supersession note, caller-adjudicated precedent)

oldString:

> `the pending 0.5.2 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1).`

newString:

> `the 0.5.2 OPERATOR-RUN gate is CLOSED — PASSED at the Wave-6 session start (see Current Work Focus); the standing regressions re-run inside the single 0.6.0 OPERATOR-RUN (Next-Steps #1).`

### Fix 5 — NEW Wave-6 branch bullet (precedent: Wave-5.1 C3b / Wave-5.2 5.4a)

Insert as the NEW FIRST bullet of `## Current Project State`, directly ABOVE the existing `- **Wave 5.2 landed on `main`...` bullet. Insert EXACTLY:

> - **Wave 6 branch `feat/wave-6-open-with` (agent-side COMPLETE through Task 5 docs; NOT merged — Step 5 pending)**: version 0.6.0; the "Open with..." menu landed (runtime `SHAssocEnumHandlers` enumeration with `ASSOC_FILTER_NONE` — UWP/Store apps included by design; NO internal program database; the `IAssocHandler::Invoke` launcher with fresh per-call re-enumeration; the pill's 11th button + the upward `egui::Popup::menu`; the two-point refresh contract and the state gates as recorded in Current Work Focus); census 177 unit tests / 28 test files; dev-checks `ALL CHECKS PASSED` (177/0); exe rebuilt 19,647,219 bytes (0.6.0; the 0.5.2 artifact 19,605,828 B overwritten atomically); DLL audit set-equal 22; the pending 0.6.0 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1).

---

## Scope boundaries (binding)

- EDIT exactly ONE file: `.agent/project-info/context.md` — ONLY the `## Current Project State` section, ONLY the five fixes above.
- NO edits to README.md, tech.md, architecture.md, the TODO file, brief.md, `.agent/project-structure.md`, docs/build-windows.md, or any `.kilo/plans/*` file.
- NO code changes; NO Cargo edits; NO harness runs (markdown-only cycle).
- Preserve every other line/section byte-for-byte (overwrite-prevention rule); real newline characters only.

## Verification (static greps, no cargo)

1. `grep -n "0.5.2 Wave-5.2 rebuild" .agent/project-info/context.md` → **0 hits**.
2. `grep -n "19,647,219" .agent/project-info/context.md` → hits include C1, C2, the release-artifact bullet, and the Wave-6 branch bullet (≥4).
3. `grep -n "NEXT session" .agent/project-info/context.md` → the todo-8 "NEXT session" fragment GONE; remaining hits refer to Wave 7 / new-chat notes only.
4. `grep -n "feat/wave-6-open-with" .agent/project-info/context.md` → hits include the Git bullet and the Wave-6 branch bullet.
5. `grep -n "is the wave's acceptance gate (Next-Steps #1)" .agent/project-info/context.md` → only the NEW Wave-6 branch bullet matches (the Wave-5.2 bullet now reads "gate is CLOSED").
6. `git status` → after commit, clean working tree on `feat/wave-6-open-with`.

## Commit protocol (Gitignore Compliance Rule before commit)

1. Read `.gitignore`; run `git status`; stage ONLY `.agent/project-info/context.md` (never `git add -A`; nothing under `dist/`, `logs/`, `target/`).
2. Commit message: `docs: wave6 task5 coherence fixes` (the Task-5 docs plan §7.4 follow-up-commit convention).
3. Final `git status` → clean tree on `feat/wave-6-open-with`. NO push (Step-5 restricted).

## Notes for the executor

- The Task-5 docs-specialist conformed to its binding plan; this gap is the PLAN's §4 scope, recorded here — not an executor fault. No TODO-file action arises from this fix (the 4.6 step owns all `[x]` marks).
- After landing, the 4.5b adherence verdict for the fixed items becomes CONFORMS; the global-plan Step-5 closing touch then amends the Wave-6 branch bullet and Git bullet to the merged/pushed state (Wave-5.2 closing precedent).
