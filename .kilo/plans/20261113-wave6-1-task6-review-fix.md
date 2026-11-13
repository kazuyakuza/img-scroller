# Wave 6.1 — Task 6 Documentation Review Fix Plan

- Source review: `.kilo/plans/20261113-wave6-1-task6-documentation-plan.md` applied at commit `63ad5f4`.
- Reviewer verdict: **FIX REQUIRED** — two tiny current-state inconsistencies in `.agent/project-info/tech.md`; all other prescriptions are accurate and accepted as-applied.
- Scope: docs-only Markdown edits; no application code, no Cargo changes, no TODO marks, no merge/push.

---

## Fix 1 — tech.md `## Testing` lead count (current-state inconsistency)

**File:** `.agent/project-info/tech.md`  
**Line:** 180 (as of HEAD `63ad5f4`)

**Current (wrong — contradicts the 189 grand total on the same line):**

```markdown
- 177 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3; census pinned per file by the Wave-5.1 Task-5 verification): `src/app/tests.rs` — 9 (unchanged shell/open-flow census); ...
```

**Prescribed replacement (only the lead number changes):**

```markdown
- 189 unit tests, all container-runnable through `cargo test --locked` (dev-checks step 3; census pinned per file by the Wave-5.1 Task-5 verification): `src/app/tests.rs` — 9 (unchanged shell/open-flow census); ...
```

**Rationale:** the per-file counts and the grand total on the same line already read 189; the lead must match the current census. The historical Wave-6 verification bullet (`177 unit tests, 0 failed, across 28 test files`) on line 142 correctly stays unchanged as a historical record per the plan.

---

## Fix 2 — tech.md Wave-6.1 pointer direction (misleading)

**File:** `.agent/project-info/tech.md`  
**Line:** 142 (as of HEAD `63ad5f4`)

**Current (wrong — the Wave-6.1 bullets are physically below this sentence):**

```markdown
The pending 0.6.0 OPERATOR-RUN checklist lives verbatim in `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11 (SUPERSEDED by the single 0.6.1 OPERATOR-RUN — the Wave-6.1 bullet above).
```

**Prescribed replacement (one-word correction):**

```markdown
The pending 0.6.0 OPERATOR-RUN checklist lives verbatim in `.kilo/plans/20261112-wave6-task4-verification-plan.md` §11 (SUPERSEDED by the single 0.6.1 OPERATOR-RUN — the Wave-6.1 bullet below).
```

**Rationale:** the four Wave-6.1 bullets (T5) are appended immediately after the Wave-6 verification bullet (T6) in the file; "above" sends the reader in the wrong direction. The original plan text contained this direction error; it was applied verbatim and now needs correction.

---

## Adjudicated non-fixes

- **`.agent/project-structure.md` was edited** despite the plan's "verify-only / NO edit" expectation. The changes ("two-point" → "three-point" refresh contract; "no hotkey in v1" → "the Q hotkey with the key caption under the symbol") are factually correct and were planner-adjudicated as acceptable. No reversion needed.
- **README.md Project Status historical Wave-6 paragraph** retains lowercase "no hotkey in v1" and "two-point enumeration contract" as dated narration of the 0.6.0 state. This is acceptable and planner-waived; it is not a current-state statement.
- **All other prescriptions** (README R1–R6, tech.md T1–T5/T7–T8, architecture.md A1–A9, context.md C1–C4, the §9 checklist verbatim in Next-Steps #1, the census correction 19+4+5, the BindToHandler fix, Q/modal/warning facts, the 0.6.1 state, and the Wave 7 next pointer) were verified and match the plan.

---

## Post-fix verification

After applying the two edits, re-run the plan's §7 coherence sweep:

```bash
rg -n "no hotkey in v1" README.md .agent/project-info/tech.md .agent/project-info/architecture.md
rg -n "two points|two decided points|Two-point" README.md .agent/project-info/tech.md .agent/project-info/architecture.md
rg -n "0\.6\.0, edition 2024" README.md .agent/project-info/tech.md .agent/project-info/architecture.md
rg -n "version 0\.6\.0 \(" .agent/project-info/tech.md
rg -n "BindToHandler" README.md .agent/project-info/tech.md .agent/project-info/architecture.md
rg -n "WARNING_MESSAGE_COLOR|255, 165, 0" README.md .agent/project-info/tech.md .agent/project-info/architecture.md
rg -n "19,645,219" .agent/project-info/tech.md .agent/project-info/architecture.md
rg -n "OPEN_WITH_MENU_KEY|handle_modal_open_with_menu_keys|open_with_menu_id" .agent/project-info/tech.md
```

Expect: the same results as before (the two fixes do not affect these checks).
