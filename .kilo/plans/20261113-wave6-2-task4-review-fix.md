# Wave 6.2 Task 4 — Documentation Review Fix Plan

Scope: documentation-only corrections in `.agent/project-info/context.md` and `.agent/project-info/tech.md`. No source/test/Cargo/script changes. No TODO edits, no commit, no push.

---

## F1 — `.agent/project-info/context.md` Recent-Changes: correct the `project-structure.md` stale-map sentence

The entry for 2026-11-13 currently claims the structure map was found stale and left unedited, but the planner adjudicated the one-line fix and it was applied in the Task-4 docs commit (`45c0a52`).

Replace this exact segment in line 21:

```text
`.agent/project-structure.md` verified — FOUND STALE (the `platform/` bullet lacks `launch_log.rs`, the new src file landed at `4185d97`) — flagged to the caller per the binding NO-edit expectation (verify-only, NO edit this cycle)
```

with:

```text
`.agent/project-structure.md` verified — FOUND STALE (the `platform/` bullet lacked `launch_log.rs`); the planner adjudicated the one-line fix and it was applied this cycle
```

---

## F2 — `.agent/project-info/context.md` Current Project State: update the Git branch tip

The Git bullet still records the Wave-6.2 branch tip as `f46d5dd`. After the Task-4 docs commit the branch tip is `45c0a52`.

Replace this exact segment in the Git bullet (line 96):

```text
`main` == `origin/main` == `b458897` (the Wave-6.1 closing state; the Wave-6.2 branch `fix/wave-6-2-open-with-launch`, created from `b458897`, sits at `f46d5dd` and is NOT yet merged — the merge + push belong to the wave's Step 5)
```

with:

```text
`main` == `origin/main` == `b458897` (the Wave-6.1 closing state; the Wave-6.2 branch `fix/wave-6-2-open-with-launch`, created from `b458897`, sits at `45c0a52` after the Task-4 docs commit and is NOT yet merged — the merge + push belong to the wave's Step 5)
```

---

## F3 — `.agent/project-info/tech.md` Testing section: correct the stale grand total

Line 183 was updated to 195 unit tests and to 12 tests in `src/open_with_listing/tests.rs`, but the trailing grand-total sentence still reads 189.

Replace this exact segment:

```text
Grand total 189 across 28 test-bearing files — the CORRECTED breakdown (Wave-6.1, recorded verbatim): **28 test-bearing files = 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `zoom.rs`, `scaling.rs`)** — the 177 pre-Wave-6.1 tests unchanged + 12 Wave-6.1 tests (+9 the open-with modal/Q/warning tests, +2 the pill_style warning-color tests, +1 the generic non-repeat filter test).
```

with:

```text
Grand total 195 across 28 test-bearing files — **28 test-bearing files = 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`)** — the 189 pre-Wave-6.2 tests unchanged + 6 Wave-6.2 tests (the `shell_parseable_path` conversion tests in `src/open_with_listing/tests.rs`).
```

---

## Notes for the implementer

- Apply F1, F2, and F3 as single-section string replacements; preserve every byte outside the prescribed segments.
- After the edits, re-run the coherence greps from the docs plan §4 (e.g. `git grep -n "195 unit tests"`, `git grep -n "19,649,107"`, `git grep -n "shell_parseable_path"`, etc.) to confirm no new drift.
- No build/harness run is required; these are text-only doc corrections.
