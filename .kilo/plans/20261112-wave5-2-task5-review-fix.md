# Wave 5.2 Task 5 — Documentation Review Fix Plan

Reviewer: code-reviewer (documentation review arm)  
Commits reviewed: `47b4ec0` (C1 plan) + `51d2b27` (C2 docs)  
Range audited: `8f50806..51d2b27`

## Verdict

- **Plan adherence**: C1 lands only the Task-5 documentation plan file. C2 lands exactly the four planned files (`README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`). Every §2–§5 edit group from `.kilo/plans/20261112-wave5-2-task5-documentation-plan.md` is present.
- **Two caller-adjudicated supersession extensions** (context.md §5.3): landed in the Wave-5.1 historical entries and the Current Project State Wave-5.1 bullet.
- **Factual-accuracy sweep**: all mechanism claims match the code (`StrCmpLogicalW` via `Win32_UI_Shell`; digit-aware natural ordering; case-insensitive; no Windows-side failure path; pure fallback = stub/container only; unchanged seam + both consumers + anchor; six Cargo features). Census 164/26 sum checks. Exe 19,605,828 bytes + DLL audit set-equal 22 are correctly presented as current-state claims from Task-4 evidence. The Task-4 plan §11 verbatim block is byte-identical in `context.md` Immediate Next Steps #1. No doc sentence presents `CompareStringEx` as the current mechanism. Architecture line-5 insertion paren balance is verified balanced.
- **Docs-specialist notes adjudicated**:
  - #1 self-corrected paren balance (architecture L5 insertion): committed state is balanced.
  - #2 `[DONE]` typography: matches the plan's specified renderings in each location.
  - #3 stale-by-design observations:
    - Historical section-title pointers: **acceptable-as-historical**.
    - `context.md` Current Project State Release-artifact bullet still reads `19,636,215 bytes, 0.5.1 Wave-5.1 rebuild`: **fix-worthy** — it is a current-state fact that is now wrong after the 0.5.2 rebuild.

## Junior-safe fix (one edit only)

File: `.agent/project-info/context.md`

Replace the stale Release-artifact bullet in `## Current Project State` with the 0.5.2 current fact:

OLD (line 75):

```markdown
- Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,636,215 bytes, 0.5.1 Wave-5.1 rebuild); previous artifacts overwritten by the atomic rebuild; import audit: 22 OBSERVED DLLs recorded in `scripts/build-windows.sh` + `docs/build-windows.md` (unchanged — the Wave-5.1 windows-feature additions are kernel32 exports).
```

NEW:

```markdown
- Release artifact: gitignored `dist/windows-x64/img-scroller.exe` (19,605,828 bytes, 0.5.2 Wave-5.2 rebuild; the 0.5.1 artifact 19,636,215 B overwritten atomically); import audit: 22 OBSERVED DLLs recorded in `scripts/build-windows.sh` + `docs/build-windows.md` (unchanged — `StrCmpLogicalW` is a shlwapi.dll export already in the recorded list; the removed `CompareStringEx`/`GetUserDefaultLocaleName` were kernel32 exports and kernel32.dll remains imported).
```

## Constraints

- Do NOT touch any other file.
- Do NOT edit `.agent/todos/20261112/20261112-todo-2.md`; the Task-5 `[x]` mark is the separate 4.6 step.
- Do NOT commit or push; stage only the one file change and hand off for the 4.5b coherence check / 4.6 mark.

## No other fixes required

All other documentation changes in `51d2b27` are accurate, plan-adherent, and paren-balanced.
