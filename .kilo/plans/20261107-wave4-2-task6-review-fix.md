# Wave 4.2 — A-Task 6, Step 4.3: Documentation Review Fix Plan

Date: 2026-11-10
Reviewer: code-reviewer
Source plan: `.kilo/plans/20261107-wave4-2-task6-documentation-plan.md`
Docs commit reviewed: `df58a25` (`docs: wave4.2 task6 documentation`)

## Review verdict

REQUIRES ONE JUNIOR-SAFE FIX.

The five-file documentation set (README.md, tech.md, architecture.md, context.md, brief.md) correctly lands every planned edit group from the docs plan (R-1..R-6, T-1..T-7, A-1..A-8, C-1..C-4, brief addendum item 7). Verification-only files (`future-ideas.md`, `.agent/project-structure.md`) were correctly left untouched.

### Adjudication of the docs-specialist's reported deviations #1–#6

- **#1 paren quote rebuild** — ACCEPTABLE. README.md R-2 replaced the backtick-quoted aliases with single-quote characters; the semantics byte-match the plan's TO string.
- **#2 `<SHA>` self-reference handling** — ACCEPTABLE. context.md C-2 first entry uses the wording `this documentation commit (`docs: wave4.2 task6 documentation`)` instead of a literal self-referential SHA, consistent with the wave-3 precedent of not embedding a commit's own hash inside itself.
- **#3 execution-time SHA fills** — ACCEPTABLE. The Task-1 mark SHAs (`7929220`/`0de6812`) and Task-5 mark SHAs (`e28f33d`/`b848ca4`) in context.md C-2 are real git evidence.
- **#4 G-2 census bullet internal order** — COSMETIC, ACCEPTABLE. The `tech.md` Testing bullet lists the 21 files in the same order and with the same per-file counts as the plan's composed text; the hand-sum equals 127.
- **#5 `architecture.md` Project-Paths line still reads `0.4.1`** — **REQUIRES CORRECTION**. This is the only surviving current-state `0.4.1` fact in an edited file. `Cargo.toml` is `0.4.2`; the docs must match.
- **#6 plan-internal pointer text** — COSMETIC, ACCEPTABLE.

### Factual-accuracy sweep (source of truth = landed code)

- Census 127 in 21 files: verified by `rg "#\[test\]" src --count-matches`; hand-sum = 127.
- `dist/windows-x64/img-scroller.exe` = 19,603,539 bytes: verified by `Get-ChildItem`.
- DLL import audit = 22 OBSERVED DLLs: the recorded constant in `scripts/build-windows.sh` lists 22 entries; docs state it is unchanged.
- Named constants/line-pins verified:
  - `POPUP_MESSAGE_TEXT = "Press ESC again to close"` — `src/app/esc_close_confirm.rs:21`.
  - `POPUP_VISIBLE_SECONDS: f64 = 2.0` — `src/app/esc_close_confirm.rs:24`.
  - `PILL_GLYPH_COLOR = Color32::WHITE` — `src/app/pill_style.rs:12`.
  - `OUTLINE_COLOR = Color32::BLACK` — `src/app/pill_style.rs:15`.
  - `OUTLINE_STROKE_WIDTH: f32 = 1.0` — `src/app/pill_style.rs:18`.
  - `CAPTION_OUTLINE_OFFSETS` 8 offsets — `src/app/pill_style.rs:24-33`.
  - `ZOOM_IN_ALIAS_KEYS`, `ZOOM_OUT_ALIAS_KEYS`, `ZOOM_IN_ALIAS_CHARACTERS`, `ZOOM_OUT_ALIAS_CHARACTERS`, `TRUE_MINUS_SIGN` — `src/app/zoom_alias.rs:12-17`.
  - `ViewportCommand::Close` is sent exactly once in `src/` — `src/app/keyboard_intake.rs:91`.
  - `disarm_esc_close_popup()` is called inside the shared fullscreen toggle — `src/app/fullscreen.rs:31`.
- No doc sentence claims behavior absent from the code.

## Required fix (one bounded edit)

### File: `.agent/project-info/architecture.md`

**Issue:** The Project-Paths paragraph still states the Cargo package version as `0.4.1` in a current-state sentence. `Cargo.toml` is `0.4.2`.

**Plan A-6 already updated the module enumeration in this paragraph; the version clause was missed.** Apply the one-line correction exactly as follows:

- FROM: `- Cargo project at the repository root: \`Cargo.toml\` (package \`img-scroller\` 0.4.1, edition 2024) plus the tracked \`Cargo.lock\` — exists since Wave 1.`
- TO:   `- Cargo project at the repository root: \`Cargo.toml\` (package \`img-scroller\` 0.4.2, edition 2024) plus the tracked \`Cargo.lock\` — exists since Wave 1.`

Only the version number changes; everything else on the line stays byte-identical.

## Verification after fix

1. Run `Select-String -Path .agent/project-info/architecture.md -Pattern "0\.4\.1"` and confirm the only hit in this file is inside dated-history sentences (Wave 4.1 paragraph), not in the Project-Paths current-state line.
2. Confirm the edited line reads `package \`img-scroller\` 0.4.2`.
3. Run `git status --short` and confirm only `.agent/project-info/architecture.md` is modified.
4. Do NOT commit, push, or merge. Report the fix-plan path back to the caller.
