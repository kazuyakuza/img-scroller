# Wave 6 — Task 5 Documentation Review Fix Plan

**Source review:** code-REVIEWER half of Critical Workflow step 4.3 for TODO Task 5 (Documentation) of Wave 6.
**Scope:** doc-accuracy fixes only; no code changes, no TODO edits, no commit/merge/push.
**Verdict:** six stale factual lines found in the four docs (the five flagged by the docs-specialist plus one additional current-state table line). All are one-line/minimal word substitutions.

---

## Stale line corrections

### 1. `architecture.md` — Status paragraph (line 5)

**Current stale clause (mid-paragraph, after the Wave 4.1 description):**

> ...set the zoom factor only). "Open with..." remains planned. Facts are sourced from...

**Correction:** delete the stale sentence (the Wave 6 end-of-paragraph append already records that the "Open with..." service is landed).

**Result:**

> ...set the zoom factor only). Facts are sourced from [brief.md](brief.md)...

---

### 2. `architecture.md` — Viewer-UI bullet (line 30)

**Current stale clauses:**

> The pill's Wave-4 control groups — the fullscreen toggle (painted square), the zoom `[+][−][restore-fit]` group, and the fit `[fit-width][fit-height]` group **(10 buttons in the row)** — are LANDED, and since Wave 4.1 the six keyboard-distinct buttons carry hotkey captions while **all ten** carry hover tooltips (`src/app/button_hint.rs`); ...

**Correction:** update both counts to 11 to match the Wave-6 pill row.

**Result:**

> The pill's Wave-4 control groups — the fullscreen toggle (painted square), the zoom `[+][−][restore-fit]` group, and the fit `[fit-width][fit-height]` group **(11 buttons in the row)** — are LANDED, and since Wave 4.1 the six keyboard-distinct buttons carry hotkey captions while **all eleven** carry hover tooltips (`src/app/button_hint.rs`); ...

---

### 3. `architecture.md` — `button_hint.rs` bullet (line 55)

**Current stale clause:**

> - `src/app/button_hint.rs` — the pill-button caption/tooltip data (Wave 4.1 Task 2): the per-button hotkey captions (six keyboard-distinct buttons ONLY — the four arrows are uncaptioned) and the per-button hover tooltips for **all ten buttons** (in BOTH enabled and disabled states), plus the shared top-aligned vertical button-column wrapper the pill uses; ...

**Correction:**

> - `src/app/button_hint.rs` — the pill-button caption/tooltip data (Wave 4.1 Task 2): the per-button hotkey captions (six keyboard-distinct buttons ONLY — the four arrows are uncaptioned) and the per-button hover tooltips for **all eleven buttons** (in BOTH enabled and disabled states), plus the shared top-aligned vertical button-column wrapper the pill uses; ...

---

### 4. `architecture.md` — Project Paths (line 113)

**Current stale clause:**

> - Cargo project at the repository root: `Cargo.toml` (package `img-scroller` **0.5.2**, edition 2024) plus the tracked `Cargo.lock` — exists since Wave 1.

**Correction:**

> - Cargo project at the repository root: `Cargo.toml` (package `img-scroller` **0.6.0**, edition 2024) plus the tracked `Cargo.lock` — exists since Wave 1.

---

### 5. `tech.md` — Status paragraph (line 5)

**Current stale clause (mid-paragraph, after the Wave 4.2 verification sentence):**

> ...deliberately not duplicated here). **The remaining v1 features ("Open with..." and packaging) are planned.** Facts are sourced from [brief.md](brief.md) ...

**Correction:** keep packaging as the only remaining planned v1 item (Open with is landed in Wave 6).

**Result:**

> ...deliberately not duplicated here). **The remaining v1 feature (packaging) is planned.** Facts are sourced from [brief.md](brief.md) ...

---

## Additional finding

### 6. `architecture.md` — Technical Decisions Summary table (line 136)

**Current stale row:**

> | Pill affordances: hotkey captions under the six keyboard-distinct buttons; hover tooltips on **all ten buttons** (enabled + disabled) | landed (Wave 4.1) | 2026-11-02 operator correction (brief addendum) |

**Correction:** update the tooltip count to the current 11-button pill.

**Result:**

> | Pill affordances: hotkey captions under the six keyboard-distinct buttons; hover tooltips on **all eleven buttons** (enabled + disabled) | landed (Wave 4.1) | 2026-11-02 operator correction (brief addendum) |

---

## Verification notes (prescription pass/fail)

- **README.md R1–R6, Technology Stack, Project Status:** applied correctly. Current pill paragraph says 11 buttons / 11 tooltips; Technology Stack shows 0.6.0; Project Status records Open with as landed. PASS.
- **tech.md T1 (Status append), T2 (Contents item 11 + renumber), T3 (0.6.0), T5 (new mechanism section), T6 (177/28 census), T7 (Constraints bullet):** applied correctly. FAIL only the stale mid-paragraph "remaining v1 features (\"Open with...\" and packaging) are planned" clause (item 5 above).
- **architecture.md A1 (Status append), A2 (Overview), A3 (Open-with service bullet), A4 (module bullets incl. 11th button), A5 (Critical Path 5), A6 (Data and State), A7 (new decisions row), A8 (Project Paths layout):** applied correctly. FAIL the five stale count/status clauses (items 1–4 and 6 above).
- **context.md C1, C2, C3:** applied correctly; 17-commit chain, 177/28 census, 19,647,219 bytes, SET-EQUAL 22, artifact note all present. PASS.

---

## What was NOT reviewed/changed

- No source code (`src/`) edits.
- No `Cargo.toml`/`Cargo.lock` edits.
- No TODO file edits.
- No changes to `brief.md`, `.agent/project-structure.md`, or `docs/build-windows.md` (verify-only per plan §5).
- No commit, merge, push, or branch operations.
- Historical Wave 4.1 record in `README.md` Project Status that says "all ten buttons show hover tooltips" was left as-is because it accurately describes the Wave-4.1 state, not the current post-Wave-6 state.
