# Wave 5 Task 5 — Documentation Review Fix Plan

**Review step:** B-Task 5 Step 4.3 (code-reviewer / documentation-review arm).  
**Reviewed commits:** `6ca1450` (C1 plan) + `734f1e0` (C2 docs).  
**Branch:** `feat/wave-5-pan-then-navigate`.  
**Actual branch tip at review time:** `734f1e0dbaf41a02b780777c8325ef4be40ec989`.  
**Actual `main`/`origin/main`:** `094980f`.

## Overall verdict

The four owned docs files (`README.md`, `tech.md`, `architecture.md`, `context.md`) landed the intended §-groups (README E1–E8; tech T1–T6; architecture A1–A15; context C1–C4).  The Wave-5 mechanism facts, test census (150/23), exe size (`19,606,124`), DLL audit (set-equal 22), reset/re-clamp contracts, `window_options` relocation, and keys-only/mouse-out-of-scope notes all match the code.

A small number of **junior-safe text fixes** are required before the docs can be considered fully coherent with the actual branch state and the source code.

---

## Adjudication of the docs-specialist's deviations #1–#5

| # | Deviation | Disposition | Rationale |
|---|-----------|-------------|-----------|
| 1 | **Sweep-intent hits** — `Wave-5 Pan-then-Navigate Mechanism` appears 4× in `tech.md` instead of the §6 sweep's stated 2×. | **Acceptable** | The extra two hits come from the plan-mandated T1 status sentence and T6 constraints sentence, both of which intentionally reference the section heading. The sweep was meant to catch stale duplicates, not plan-mandated cross-references. |
| 2 | **`HEAD = 6ca1450` real-capture rule** — `context.md` records the pre-C2 tip. | **Fix-worthy** | At execution/review time the branch tip is `734f1e0`. The git-state bullet must record the current tip. |
| 3 | **"Paste" residue phrase** — `context.md` C2 still contains `; paste the actual output lines here` after the commit chain was pasted. | **Fix-worthy** | The placeholder instruction should be removed now that the actual log lines are present. |
| 4 | **C4 restructure** — Immediate Next Steps were restructured to four items. | **Acceptable** | The four items match the plan verbatim, the 0.4.2 checklist block was preserved byte-for-byte under item 2, and the 0.5.0 checklist is present under item 1. |
| 5 | **A11 label phrasing** — Path 2/Path 3 navigation phrasing. | **Acceptable** | The landed text matches the plan's A11(b) and A11(c) strings exactly. |

---

## Additional factual-accuracy fixes found during review

### FA-1 — Stale "Wave 5 pan planned" clause in `tech.md` Status

The T1 append left the pre-Wave-5 closing sentence intact, creating a contradiction: the paragraph now says the Wave-5 pan behavior is **planned** and then immediately says it **landed**.

**File:** `.agent/project-info/tech.md`  
**Edit:**

```text
oldString:
The remaining v1 features ("Open with..." and packaging; plus the Wave-5 pan-then-navigate behavior while zoomed) are planned.

newString:
The remaining v1 features ("Open with..." and packaging) are planned.
```

### FA-2 — Stale "Wave 5 pan planned" clause in `architecture.md` Status

Same pattern as FA-1: A1 appended the landed Wave-5 sentence while the older sentence still said "Pan-then-navigate while zoomed (Wave 5) ... remain planned."

**File:** `.agent/project-info/architecture.md`  
**Edit:**

```text
oldString:
Pan-then-navigate while zoomed (Wave 5) and "Open with..." remain planned.

newString:
"Open with..." remains planned.
```

### FA-3 — `ArrowKeyPress` field name mismatch in `tech.md` and `architecture.md`

The plan used `ArrowKeyPress { direction, repeat }` and `direction + repeat flag`, but the source code defines the field as `non_repeat: bool` (`src/app/pan.rs:19`). The behavior description is correct, but the field literal is wrong.

**File:** `.agent/project-info/tech.md` (Wave-5 mechanism bullet)  
**Edit:**

```text
oldString:
`ArrowKeyPress { direction, repeat }`

newString:
`ArrowKeyPress { direction, non_repeat }`
```

**File:** `.agent/project-info/architecture.md` (`src/app/pan.rs` module bullet)  
**Edit:**

```text
oldString:
(`ArrowKeyPress`: direction + repeat flag)

newString:
(`ArrowKeyPress`: direction + non_repeat flag)
```

---

## Required context.md git-state fixes

### C3(d)-1 — Update branch HEAD

**File:** `.agent/project-info/context.md`  
**Edit:**

```text
oldString:
HEAD = `6ca1450`); at edit time the working tree carries ONLY this task's docs changes (committed as §6 C2)

newString:
HEAD = `734f1e0`); at edit time the working tree is clean after the C2 docs commit
```

### C2-1 — Remove paste placeholder residue

**File:** `.agent/project-info/context.md`  
**Edit:**

```text
oldString:
Wave commit chain (verbatim from the pre-flight `git log --oneline -20`; paste the actual output lines here):

newString:
Wave commit chain (verbatim from the pre-flight `git log --oneline -20`):
```

---

## Verification after fixes

Run these checks before reporting completion:

1. `git status --short` shows no unexpected changes beyond the three docs files.
2. `git diff -- .agent/project-info/tech.md .agent/project-info/architecture.md .agent/project-info/context.md` contains **only** the five edits above.
3. Grep sweeps:
   - `grep -n "Wave-5 pan-then-navigate behavior while zoomed" .agent/project-info/tech.md` → 0 hits.
   - `grep -n "Pan-then-navigate while zoomed (Wave 5)" .agent/project-info/architecture.md` → 0 hits.
   - `grep -n "ArrowKeyPress { direction, repeat }" .agent/project-info/tech.md` → 0 hits.
   - `grep -n "direction + repeat flag" .agent/project-info/architecture.md` → 0 hits.
   - `grep -n "paste the actual output lines here" .agent/project-info/context.md` → 0 hits.
   - `grep -n "HEAD = \`6ca1450\`" .agent/project-info/context.md` → 0 hits.
   - `grep -n "HEAD = \`734f1e0\`" .agent/project-info/context.md` → 1 hit.
4. No code files are touched (`src/`, `Cargo.toml`, `Cargo.lock` unchanged).
5. Do **not** commit, amend, push, or touch the TODO file — this is a review fix only.

---

## Notes for the caller

- The only deviations from the original documentation plan are the three factual-accuracy fixes (FA-1, FA-2, FA-3) plus the two context.md state cleanups (C3(d)-1, C2-1). All are text-only and confined to the owned docs files.
- The plan's §6 sweep for `Wave-5 Pan-then-Navigate Mechanism` intentionally exceeds its own "exactly 2 hits" target because T1 and T6 mandate cross-references to that heading. This was adjudicated acceptable above.
