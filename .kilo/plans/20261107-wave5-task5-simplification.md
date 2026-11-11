# Wave 5 Task 5 — Documentation Simplification Plan

**Review step:** B-Task 5 Step 4.3 (code-simplifier / simplification arm).
**Reviewed commits:** `6ca1450` (C1 plan) + `734f1e0` (C2 docs) on `feat/wave-5-pan-then-navigate`.
**Verdict:** SIMPLIFICATION APPLICABLE — 4 contradiction fixes (6 exact text edits), all junior-safe, text-only, docs-files only. No src touched. No structural restructuring proposed.

## Scope review (what was assessed)

- Only defects in the categories this arm owns: sentences left stale by the commit's OWN edits that now contradict newly added content in the same paragraph/section, and internal cross-reference contradictions introduced by the commit's renumbering.
- The plan file `6ca1450` is a workflow working document — not simplifiable (Markdown Generation Rule + never-edit-plans rule). Its internal inconsistencies (T2 TOC placement vs T4 section placement; E6 scope vs Project Status; A11(a) "Path 5" string) are surfaced here as fixes to the LANDED docs, not to the plan.
- House-style dense restatement between README/tech/architecture (summary sentences repeated per file) is the established pattern — NOT flagged.

## Complementarity — do NOT duplicate the review arm

The documentation-review arm already owns these strings in `20261107-wave5-task5-review-fix.md`: FA-1 (tech.md Status "Wave-5 pan ... are planned" clause), FA-2 (architecture.md Status "Pan-then-navigate ... remain planned"), FA-3 (`ArrowKeyPress` field names), C3(d)-1 (`HEAD = 6ca1450`), C2-1 ("paste the actual output lines here" residue). Execute those first or alongside; the edits below touch ZERO of the same strings/lines.

## S-1 — README.md Project Status: stale "pan-then-navigate open" contradicts the appended Wave-5 landed sentence (same line 68)

The commit updated the Technology Stack intro (line 48) to "the remaining v1 feature ("Open with...") is still open" but appended the "Wave 5 landed" sentence to the Project Status paragraph while leaving the same paragraph claiming pan-then-navigate is still open. Direct self-contradiction on one line.

**File:** `README.md` (line 68)
**Edit:**

```text
oldString:
The remaining v1 features (pan-then-navigate at the image edges while zoomed, "Open with...") are open.

newString:
The remaining v1 feature ("Open with...") is open.
```

## S-2 — tech.md Contents: TOC order contradicts the body section order

The plan (T2 + T4 combination) inserted the Wave-5 Contents item at position 8 while the mechanism section was mandated AFTER the Wave-4.2 section — body order is Wave-4.2 (`tech.md:99`) then Wave-5 (`tech.md:107`); the Contents (`tech.md:16-17`) lists them reversed. Minimal fix = swap the two lines so the TOC mirrors the body (chronological house style preserved).

**File:** `.agent/project-info/tech.md` (lines 16–17)
**Edit:**

```text
oldString:
8. [Wave-5 Pan-then-Navigate Mechanism (Implemented)](#wave-5-pan-then-navigate-mechanism-implemented)
9. [Wave-4.2 Operator-Feedback Mechanisms (Implemented)](#wave-42-operator-feedback-mechanisms-implemented)

newString:
8. [Wave-4.2 Operator-Feedback Mechanisms (Implemented)](#wave-42-operator-feedback-mechanisms-implemented)
9. [Wave-5 Pan-then-Navigate Mechanism (Implemented)](#wave-5-pan-then-navigate-mechanism-implemented)
```

Note for the caller: this deviates from the docs plan's T2 wording ("Contents item 8") — the plan itself was internally inconsistent; the landed body follows the Wave 4 → 4.2 → 5 append order, so the TOC follows the body. The 4.5b coherence check should record this adjudication.

## S-3 — architecture.md Navigation Critical Paths intro: "Path 5" label contradicts the numbered list below

The new intro (plan-mandated A11(a) string) calls the pan-then-navigate interplay "Path 5", but the section's list item 5 is "**Open with**" and the Wave-5 behavior rides on list items 2 and 3 (rewritten in the same commit). Minimal fix removes the phantom numbering.

**File:** `.agent/project-info/architecture.md` (line 82)
**Edit:**

```text
oldString:
) and Path 5's pan-then-navigate interplay (Wave 5);

newString:
) and the Wave-5 pan-then-navigate interplay riding on Paths 2 and 3;
```

## S-4 — context.md Wave-4.2 state bullet: "NOT merged" + "Next-Steps #1" contradict the commit's rewritten Git bullet and renumbered Next Steps

Verified git reality: `main` == `origin/main` == `094980f` (contains the full `fix/wave-4-2-operator-feedback` chain) and the branch is DELETED (`git branch -a` shows no such branch) — yet the untouched Wave-4.2 bullet still says "NOT merged" / "branch HEAD = b2dc0bb", contradicting the same commit's Git bullet (line 72) that records `main == 094980f` and "no other branches". Its "(Next-Steps #1)" pointer also became stale when the same commit renumbered Immediate Next Steps (the 0.4.2 gate is now item 2; the new Wave-5 bullets already use #1/#2 correctly). Fix follows the Wave-4.1 landed bullet's established phrasing.

**File:** `.agent/project-info/context.md` (line 58)
**Edit (a):**

```text
oldString:
(agent-side work complete through Task 6; NOT merged — merge/rename/push are the Workflow-Step-5 mechanics after the Task-6 mark)

newString:
(agent-side work complete through Task 6; landed on `main` — merged fast-forward at `094980f`, pushed to `origin`, branch deleted after the verified merge)
```

**Edit (b):**

```text
oldString:
The pending 0.4.2 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #1); branch HEAD = `b2dc0bb`.

newString:
The pending 0.4.2 OPERATOR-RUN is the wave's acceptance gate (Next-Steps #2).
```

## Verification after fixes

Run before reporting completion (case-sensitive `rg`):

1. `rg -n "pan-then-navigate at the image edges while zoomed, \\"Open with" README.md` → 0 hits.
2. `rg -n "^8\. \[Wave-4\.2|^9\. \[Wave-5" .agent/project-info/tech.md` → 2 hits, in that order.
3. `rg -n "Path 5" .agent/project-info/architecture.md` → 0 hits.
4. `rg -n "NOT merged" .agent/project-info/context.md` → 3 hits ONLY, all Wave-5-related (Current Work Focus, Current Project State Wave-5 bullet, Git bullet); the Wave-4.2 bullet no longer matches.
5. `rg -n "Next-Steps #1" .agent/project-info/context.md` → only the 0.5.0-gate references (Wave-5 bullet + Next Steps item 1 context), none pointing at the 0.4.2 gate.
6. Combined with the review arm's own sweeps; `git status --short` shows only the four owned docs files modified; `src/`, `Cargo.toml`, `Cargo.lock` untouched.
7. Do NOT commit, amend, push, or touch TODO files — review-fix only (4.6 handles the mark).

## Out-of-scope observations (NO action, informational only)

- README feature 6 "— (landed in Wave 5) —" double-dash punctuation and the pill-paragraph keys-only sentence duplicating the table annotations: house style tolerates the restatement; not flagged.
- context.md Wave-4 bullet ("the Wave-4 OPERATOR-RUN is the pending acceptance gate") is pre-existing staleness from an earlier wave (the gate was closed by Wave-4.2 Task 1 per the adjacent Wave-4.1 bullet) — predates these commits, out of this step's mandate; candidate for the 4.5b coherence check only if the caller wants it.
