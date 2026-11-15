# Wave 6.2 Task 4 — 4.5b Adherence Corrective Plan (stale-statement residue, docs-only)

- Produced by the architector step 4.5b (Overall Plan Adherence) for TODO Task 4 of Wave 6.2, checked at HEAD `1643ea2` (clean tree).
- Docs plan under review: `.kilo/plans/20261113-wave6-2-task4-documentation-plan.md`; review-fix plan: `.kilo/plans/20261113-wave6-2-task4-review-fix.md`.
- Verdict basis: every §2 edit of the docs plan landed verbatim; the three §3 verify-only verdicts are recorded; the review-fix F1/F2/F3 corrections are applied. The ONLY deviations found are the three stale current-state statements below — none was prescribed by the docs plan (a plan-detection gap), and none was introduced by a wrong edit.
- Scope: documentation-only, the THREE single-segment replacements below. NO source/test/Cargo/script change, NO TODO edit, NO commit/push decision here, NO `CHANGELOG.md`, NO edit to any existing `.kilo/plans/*` file (this file is the ONE new plan file).

---

## §0 FINDINGS (each verified live at HEAD `1643ea2`)

| # | File : line | Stale text (current-state, NOT historical/dated) | Why stale |
| --- | --- | --- | --- |
| A1 | `.agent/project-info/architecture.md` : 113 | `` (package `img-scroller` 0.6.1, edition 2024) `` in the Project Paths bullet | Version is 0.6.2 (`Cargo.toml:3`; tech.md:33 and README.md:53 both say 0.6.2). This is a standing, undated current-state path bullet, not a wave narration. |
| A2 | `.agent/project-info/tech.md` : 142 | `` (16 tests in `src/app/open_with_button_tests.rs` since Wave 6.1, 6 in `src/open_with_listing/tests.rs`). `` at the end of the Wave-6 Stub-path bullet | `src/open_with_listing/tests.rs` has 12 tests since Wave 6.2 (tech.md:183 + tech.md:149 + architecture.md:52 all say 12). The unqualified `6` reads as the current count → an internal tech.md inconsistency. |
| A3 | `.agent/project-info/context.md` : 96 | `` sits at `45c0a52` after the Task-4 docs commit `` in the Git bullet (Current Project State) | Present-tense branch-tip claim. The branch tip is `1643ea2` (the Task-4 coherence-fix commit) — the value is one commit behind because a commit cannot record its own hash. The rewording below removes the self-referential absolute-tip claim. |

## §1 THE THREE EDITS (docs-specialist / implementer, in order; exact single-segment replacements; preserve every byte outside the segment)

### E1 — `.agent/project-info/architecture.md` line 113 (Project Paths bullet)

Replace EXACTLY:

```text
(package `img-scroller` 0.6.1, edition 2024)
```

with:

```text
(package `img-scroller` 0.6.2, edition 2024)
```

### E2 — `.agent/project-info/tech.md` line 142 (Wave-6 Stub-path bullet, final parenthetical)

Replace EXACTLY:

```text
(16 tests in `src/app/open_with_button_tests.rs` since Wave 6.1, 6 in `src/open_with_listing/tests.rs`).
```

with:

```text
(16 tests in `src/app/open_with_button_tests.rs` since Wave 6.1; 12 in `src/open_with_listing/tests.rs` since Wave 6.2 — 6 Wave-6 + 6 Wave-6.2 the conversion tests).
```

(The wording mirrors architecture.md:52's established form "12 tests in `src/open_with_listing/tests.rs` (6 Wave-6 + 6 Wave-6.2 the conversion tests)".)

### E3 — `.agent/project-info/context.md` line 96 (Git bullet, Current Project State)

Replace EXACTLY:

```text
, created from `b458897`, sits at `45c0a52` after the Task-4 docs commit and is NOT yet merged
```

with:

```text
, created from `b458897`, landed its Task-4 documentation at `45c0a52` and its coherence fixes at `1643ea2` (later commits supersede the recorded hashes — read `git log --oneline -1` for the live tip) and is NOT yet merged
```

(The absolute present-tense tip claim is replaced by dated landed-commit facts that stay true regardless of later commits; the not-yet-merged clause is preserved. The Step-5 final closing touch will rewrite this bullet again with the merged state, as already prescribed in Immediate Next Steps #2.)

## §2 FAILURE-HANDLING POLICY

1. If an anchor segment is NOT found exactly in the target file: STOP that edit, capture the surrounding lines, and return the mismatch to the caller. Do NOT improvise a different anchor.
2. NO fix attempts beyond the three replacements; the single allowed retry is re-reading the file after a transient tool error.
3. Before the commit: read `.gitignore`, run `git status`, ensure NO gitignore-matching file is staged; stage ONLY the two doc files (`architecture.md`, `tech.md`) and `context.md` per the landed edits; commit with the message EXACTLY: `docs: wave6.2 task4 adherence stale-statement fixes`. NO push, NO branch operation, NO TODO edit.

## §3 VERIFICATION (after the edits, before the commit)

1. Re-run the coherence greps from the docs plan §4.2 (host, one command each) — expect NO new drift: `git grep -n "0.6.2"` hits in README.md/tech.md/architecture.md (architecture now in the Project Paths bullet too); `git grep -n "0.6.1" .agent/project-info/architecture.md` → ONLY historical/dated narrations remain; `git grep -n "Grand total 195" .agent/project-info/tech.md`; `git grep -n "12 in \`src/open_with_listing/tests.rs\`" .agent/project-info/tech.md`.
2. NO build/harness run (docs only; markdown-only cycle).

## §4 SCOPE BOUNDARIES (hard)

- The THREE files' THREE segments ONLY. NO other edit anywhere; no existing `.kilo/plans/*` file touched; NO `CHANGELOG.md`; NO `dist/`/`logs/` interaction.
