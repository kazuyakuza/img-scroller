# Wave 6.1 — Task 6 Documentation Adherence Verdict + Corrective Plan (Critical Workflow step 4.5b, architector)

- Source: adherence check of the applied Task-6 docs (plan `.kilo/plans/20261113-wave6-1-task6-documentation-plan.md`, applied at `63ad5f4` + coherence fixes at `3d4e0d9`) against the TODO Task-6 requirements and the wave facts.
- Verdict: **FIX REQUIRED (small)** — every plan-prescribed edit is applied correctly and TODO Task 6 is substantively satisfied, but the coherence sweep found FOUR stale current-state statements in `.agent/project-info/architecture.md` (Module Layout section) that the docs plan's edit list did not cover. They contradict the 0.6.1 census recorded in the same file and in tech.md.
- Scope: docs-only Markdown edits to `.agent/project-info/architecture.md` ONLY. NO application code, NO Cargo, NO TODO marks, NO merge/push, NO `CHANGELOG.md`, NO edits to existing plan files.
- Executor: a JUNIOR agent under 50% restriction. Every edit below is FINAL (anchor + exact replacement); do NOT redesign or reword beyond the pinned fragments. Minor latitude: mechanical markdown line-wrapping inside the prescribed sentences only.

---

## §0 WHAT WAS VERIFIED (adherence record — no action)

1. **All plan prescriptions applied**: README R1–R6, tech T1–T8 (incl. the two review-fix items: the Testing lead `189` and the "the Wave-6.1 bullet below" pointer), architecture A1–A9, context C1–C4 — all verified present at HEAD `3d4e0d9` (branch `fix/wave-6-1-open-with-feedback`, clean tree).
2. **TODO Task-6 requirements all satisfied**: launcher root cause + fix (README R2b/R6b; tech T3 + the module-doc pointer; arch A2a/A8; context C1/C2/C4a), `Q` hotkey (all four docs), modal menu behavior (all four docs), warning color (all four docs), 0.6.1 state (all four docs), remaining open items — the pending 0.6.1 OPERATOR-RUN gate, the DIAGNOSTIC-fallback note, and the Wave-7-next pointer (context Next-Steps #1/#2/#3; README R6b; tech T5-bullet-4).
3. **Cross-doc consistency (all PASS)**: version 0.6.1 (zero current-state `0.6.0` version cells); census 189/28 with the corrected breakdown 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file (tech lead + Grand total, README R6b, arch A1, context C1/C4a); exe 19,645,219 B; DLL audit set-equal 22; the `BindToHandler` route recorded with ZERO stale array-route current-state claims (remaining `SHCreateShellItemArrayFromShellItem` mentions are dated Wave-6/0.6.0 root-cause narrations — acceptable); the PLAIN 10-item checklist + STANDING NOTE carried VERBATIM in context.md Next-Steps #1 (compared item-by-item against `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9 — exact match).
4. **Verify-only verdicts re-confirmed**: brief.md binding intact (NO-EDIT correct); docs/build-windows.md OBSERVED list unchanged (NO-EDIT correct); project-structure.md carries the planner-adjudicated corrections ("three-point", the `Q` hotkey, `open_with_button_tests.rs` 16) — acceptable-as-adjudicated.
5. **Sweep checks §7 #1–#5, #7, #8 of the docs plan: PASS.** Two recorded waivers (no action): (a) README line 69's lowercase "no hotkey in v1" is dated 0.6.0 narration — planner-waived in the review-fix adjudicated non-fixes; (b) the plan's §7 check 6 expected `WARNING_MESSAGE_COLOR|255, 165, 0` matches in README, but the plan's own prescribed README text (R2b/R6b) says "WARNING ORANGE"/"warning orange" — a plan-internal expectation inconsistency, NOT an executor deviation; the warning color IS recorded in README. No README edit.

---

## §1 FINDINGS — THE FOUR STALE CURRENT-STATE STATEMENTS (all in `.agent/project-info/architecture.md`, Module Layout section)

| # | Line | Current (stale) | Correct current state (source: tech.md `## Testing` + T5 bullets; the landed code) |
| --- | --- | --- | --- |
| S1 | 55 | `the per-button hotkey captions (six keyboard-distinct buttons ONLY — the four arrows are uncaptioned)` | SEVEN keyboard-distinct captioned buttons since Wave 6.1 (the Open-with `Q`); the four arrows stay uncaptioned. |
| S2 | 58 | `...enabled/disabled palette; 3 tests in `src/app/pill_style_tests.rs`.` AND the bullet omits the Wave-6.1 `WARNING_MESSAGE_COLOR` constant + `message_text_color` selector | 5 tests (Wave-6.1: 3 → 5); the module also carries `WARNING_MESSAGE_COLOR` + `message_text_color`. |
| S3 | 59 | `the pure non-repeat ESC press filter, ...; 11 tests in `src/app/esc_close_confirm_tests.rs`` AND the bullet omits the generalized `non_repeat_key_press_occurred` filter | 12 tests (Wave-6.1: 11 → 12); the ESC filter delegates to the generic `non_repeat_key_press_occurred` filter. |
| S4 | 63 | the `read_and_dispatch_keystrokes` fixed-order description omits the Wave-6.1 first-line `update_esc_close_popup` expiry update, the early-return modal guard `handle_modal_open_with_menu_keys`, and the distinct-`Q` open-with read/dispatch | tech.md T5-bullet-2 and architecture's own Fast Open-with bullet record the guard; the Module-Layout bullet must match. |

These contradict the 189 census recorded in architecture.md line 5 and tech.md line 180 — the same defect class the 4.3 review fixed (the 177 lead).

---

## §2 PRESCRIBED FIXES (apply in order; each is an exact fragment replacement)

### F1 — architecture.md line 55 (`src/app/button_hint.rs` bullet)

- FIND: `the per-button hotkey captions (six keyboard-distinct buttons ONLY — the four arrows are uncaptioned)`
- REPLACE WITH: `the per-button hotkey captions (seven keyboard-distinct buttons since Wave 6.1 — the Open-with button gained `Q`; the four arrows stay uncaptioned)`

### F2 — architecture.md line 58 (`src/app/pill_style.rs` bullet, two fragments)

- F2a — FIND: `and the `glyph_state_for`/`glyph_state_color` enabled/disabled palette; 3 tests in `src/app/pill_style_tests.rs`.`
  REPLACE WITH: `the `glyph_state_for`/`glyph_state_color` enabled/disabled palette, and the Wave-6.1 warning-message pair (`WARNING_MESSAGE_COLOR` = orange, applied ONLY to the open-with launch-failure message via `message_text_color`); 5 tests in `src/app/pill_style_tests.rs` (3 Wave-4.2 + 2 Wave-6.1).`
- F2b — FIND (the same bullet's parenthetical): `(Wave 4.2 Task 2): the `PILL_GLYPH_COLOR``
  REPLACE WITH: `(Wave 4.2 Task 2; the warning-message pair Wave 6.1): the `PILL_GLYPH_COLOR``

### F3 — architecture.md line 59 (`src/app/esc_close_confirm.rs` bullet, two fragments)

- F3a — FIND: `the pure non-repeat ESC press filter, the pure `esc_close_action` arm/close decision`
  REPLACE WITH: `the pure non-repeat ESC press filter (delegating to the generic `non_repeat_key_press_occurred` filter since Wave 6.1), the pure `esc_close_action` arm/close decision`
- F3b — FIND: `and the self-gated centered popup render; 11 tests in `src/app/esc_close_confirm_tests.rs``
  REPLACE WITH: `and the self-gated centered popup render; 12 tests in `src/app/esc_close_confirm_tests.rs` (11 Wave-4.2 + 1 Wave-6.1 the generic-filter test)`

### F4 — architecture.md line 63 (`src/app/keyboard_intake.rs` bullet, one append)

- FIND (the bullet's final sentence): `running in EVERY mode (fullscreen included) before the layout dispatch.`
- REPLACE WITH: `running in EVERY mode (fullscreen included) before the layout dispatch. Wave 6.1 added: the per-frame ESC-close popup-expiry update (`update_esc_close_popup`) moved to the FIRST line, the early-return modal guard `handle_modal_open_with_menu_keys` immediately after it (while the open-with menu is open it closes the menu on a distinct `Q`/`ESC` via `Popup::close_id` and returns — every other read/dispatch is skipped), and the distinct-`Q` open-with read/dispatch that opens the menu (refresh + `Popup::open_id`; a further `Q` closes it).`

---

## §3 POST-FIX SWEEP (host bash, single `rg` commands — all must pass)

1. `rg -n 'six keyboard-distinct buttons ONLY' .agent/project-info/architecture.md` → ZERO matches.
2. `rg -n '3 tests in .src/app/pill_style_tests.rs' .agent/project-info/architecture.md` → ZERO matches.
3. `rg -n '11 tests in .src/app/esc_close_confirm_tests.rs' .agent/project-info/architecture.md` → ZERO matches.
4. `rg -n 'five keyboard|177 unit tests, 0 failed' .agent/project-info/architecture.md` → ZERO matches (no regression).
5. `rg -c 'handle_modal_open_with_menu_keys' .agent/project-info/architecture.md` → ≥ 2 (the Fast Open-with bullet + the keyboard_intake bullet).

---

## §4 COMMIT PROTOCOL (before the commit: read `.gitignore` + run `git status`; stage ONLY `.agent/project-info/architecture.md`; NO gitignored path staged)

- Commit message: `docs: wave6.1 task6 adherence fixes`
- After the commit, re-run `git status` → clean tree. NO push (Step-5-restricted), NO merge, NO tag, NO CHANGELOG, NO TODO marks.

---

## §5 SCOPE BOUNDARIES (HARD)

- Edit target: EXACTLY `.agent/project-info/architecture.md`, EXACTLY the four sites in §2. NO other file is created or modified.
- Ambiguity or anchor-not-found at any edit → STOP and return the exact anchor/discrepancy to the caller. NEVER improvise replacement text.
- Follow-through (NOT this plan): after the §2–§4 fixes land, the Task-6 4.6 step marks TODO Task 6 `[DONE]` (separate step).

---

## §6 SOURCES

- `.agent/todos/20261113/20261113-todo-1.md` (Task 6 verbatim).
- `.kilo/plans/20261113-wave6-1-task6-documentation-plan.md` (the applied plan; §7 sweep) and `.kilo/plans/20261113-wave6-1-task6-review-fix.md` (the adjudicated review fixes).
- Landed docs read at HEAD `3d4e0d9`: `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`, `.agent/project-structure.md`, `.agent/project-info/brief.md`, `docs/build-windows.md`.
- `.kilo/plans/20261113-wave6-1-task5-verification-plan.md` §9 (the PLAIN checklist compared verbatim).
