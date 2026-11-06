# Wave 4.1 Task 2 — Button Hotkey References + Hover Tooltips: 4.5b Plan-Adherence Report

**Date:** 2026-11-05
**Workflow step:** 4.5b (architector, Plan Adherence) — Wave-4.1 Task 2, branch `fix/wave-4-operator-fixes`
**Plan audited:** [`.kilo/plans/20261102-wave4-1-task2-references-tooltips-plan.md`](20261102-wave4-1-task2-references-tooltips-plan.md)
**Spec (binding input):** [`.kilo/plans/20261102-wave4-1-task2-references-tooltips-frontend-spec.md`](20261102-wave4-1-task2-references-tooltips-frontend-spec.md)
**Simplification plan (4.3):** [`.kilo/plans/20261102-wave4-1-task2-references-tooltips-simplification.md`](20261102-wave4-1-task2-references-tooltips-simplification.md)
**Frontend verification report (NEW, incorporated):** [`.kilo/plans/20261102-wave4-1-task2-references-tooltips-frontend-verification.md`](20261102-wave4-1-task2-references-tooltips-frontend-verification.md)
**Baseline chain audited:** `86a1c2c` (Task-1 mark, plan-verified HEAD) → `fa8ec7e` → `4a2e7f0` → `95247dc` → `19ff32c` → `020c22f` (audited HEAD)
**Method:** `git log --oneline`, `git show --stat` / `git diff <base> <commit>` per commit (whole chain `86a1c2c..020c22f` in total), full-source reads of `src/app.rs` and `src/app/button_hint.rs`, line-count verification by read-tool line numbers, working-tree `git status`. No code touched; this report file is the only output. (Uncommitted working tree besides HEAD: the two `.kilo/plans/*` report/simplification files — the caller's to commit; nothing else.)

---

## 1. Commit chain exactness — VERDICT: CONFORMS (one recorded sequencing deviation, LEDGER-D1)

| Commit | Message (actual) | Files staged | Plan expectation | Verdict |
|---|---|---|---|---|
| `fa8ec7e` | `docs: add wave4.1 task2 spec and plan` | spec `.md` + plan `.md` (2 files) | §2 Commit 1: spec + plan + `context.md` (the already-present 4.1a diff) | **deviation D1** — `context.md` NOT staged here (deferred to `19ff32c`); message + 2-of-3 file set exact |
| `4a2e7f0` | `feat: button hotkey references and hover tooltips` | exactly `src/app.rs` (+2), `src/app/button_hint.rs` (new, 188), `src/app/fit_mode_button.rs`, `src/app/floating_controls.rs`, `src/app/fullscreen_button.rs`, `src/app/zoom_button.rs` (stat: 261+/32−) | §13.2: the six `src/` files only; message verbatim §13.2 | **CONFORMS** — file set exact, nothing else |
| `95247dc` | `fix: button hint doc trims and app module whitespace` | `src/app.rs` + `src/app/button_hint.rs` only (9+/30−) | Simplification plan: S1+S2+S3 in one fix commit; caller-lane app.rs decision | **CONFORMS as caller-authorized amendment** (see §3.1) |
| `19ff32c` | `docs: record task2 button-hint spec status in context log` | `.agent/project-info/context.md` only (3+/1−) | NOT a planned commit | **deviation D1 carrier** — the 4.1a context-log content plan §2 assigned to Commit 1, plus a Current Work Focus rewrite and an Immediate-Next-Steps 4.1b line; all Task-2-scope docs only |
| `020c22f` | `docs: record wave4.1 task2 button-hint state` | `.agent/project-info/context.md` (+1) + `.agent/project-structure.md` (1 line replaced) | §13.3 Commit 3: both files, message verbatim | **CONFORMS** — file set + message exact; content gaps recorded (LEDGER-GAP-1, §3.2) |

Nothing drifted beyond the six `src/` files + the two Task-2 `.kilo/plans/*` creations + the two project-info doc files. No TODO-file change, no `CHANGELOG.md`, no existing-plan edit anywhere in the chain (`git diff 86a1c2c..020c22f --stat` = exactly the 10 files itemized above). Message texts match the plan verbatim wherever the plan prescribed one (§2.51, §13.2, §13.3); `95247dc`'s message differs from the simplification plan's *suggested* form (`fix: button-hint review fixes (trim docs duplicated from the module header)`) — cosmetic variance on a non-binding suggestion, actual message is more accurate.

## 2. Plan §§ materialized — VERDICT: CONFORMS

| Plan item | Landing evidence (final HEAD) | Verdict |
|---|---|---|
| §3 closed table: 10-variant `PillButton` + conv. constructors `from_arrow_control`/`from_zoom_action`/`from_fit_mode_action` + `PillButtonPlacement`/`ButtonColumnSite`/`show_button_column` | `src/app/button_hint.rs:15–148`; constructors 30–52; content byte-matches the §3 snippet modulo the allowed rustfmt reflow (test import reordered to `{PillButton, hotkey_reference, tooltip_text}`; tooltip-chain wrap split into two lines) | CONFORMS |
| Captions `F11/I/O/0/W/H` on the six keyboard-distinct buttons; arrows `None` | `button_hint.rs:57–70` (`hotkey_reference`), floated verbatim vs spec §2; arrows uncaptioned by `None` 59–62 | CONFORMS |
| Tooltips all ten, verbatim vs spec §3 | `button_hint.rs:73–86`; all ten strings byte-identical to the spec table | CONFORMS |
| `Align::TOP` pill row | `src/app/floating_controls.rs:76` — `egui::Layout::left_to_right(egui::Align::TOP)` (only that line changed) | CONFORMS |
| Per-button column wrap: placement → site → `show_button_column` → existing click dispatch at ≤2 args | arrows `floating_controls.rs:128–140`; fullscreen `src/app/fullscreen_button.rs:17–24` (extracted fn-passthrough `allocate_and_paint` 26–34); zoom `src/app/zoom_button.rs:59–71` (extracted `allocate_and_paint` 73–80); fit `src/app/fit_mode_button.rs:60–72` (extracted `allocate_and_paint` 74–83) — extract bodies are the old inline code with `row_ui`→`column_ui`, exactly per §6.2/§7.3/§8.3 | CONFORMS |
| §5.1 two `//!` header lines + §5.2 `button_hint` import in alphabetical position | `floating_controls.rs:6–7` (plan-verbatim wording), import at 12 | CONFORMS |
| §7.3 enum upgrades in place (`#[derive(Clone, Copy)] pub(super) enum`) | `zoom_button.rs:23–27`, `fit_mode_button.rs:24–27` — bodies/uses otherwise untouched | CONFORMS |
| §7.4 zoom glyph merge (`zoom_glyph_shapes` folded into `paint_glyph`) | `zoom_button.rs:117–129` — merged match-based geometry, mechanical, identical shapes | CONFORMS |
| §4 app.rs mod line (blank + `mod button_hint;` between `arrow_direction` and `central_surface`) | Added exactly so in `4a2e7f0` (diff: +blank, +`mod button_hint;`); declaration still sits in that alphabetical slot after the §3.1 reorder; net result 183 lines | CONFORMS |
| §17 reviewer notes (no extra doc comments beyond specified; caption `if let` NOT extracted) | `grep`-level review: header pair only in `floating_controls`; no commented-out code in any touched file | CONFORMS |

## 3. Deviations ledger

### 3.1 (a) `app.rs` 200 → 183 via caller-authorized collapse — VERDICT: **ACCEPTABLE**

- **Facts.** `95247dc` collapsed the blank lines between `mod` declarations AND alphabetized the whole module block (including the `#[cfg(test)] mod tests;` test trio), taking `app.rs` from 200 → 183. The simplification plan (Considered-rejected #10, Informational #1) had REJECTED exactly this trim, deferring it to the caller with option (a): "the caller decides between (a) explicitly authorizing the whitespace collapse in a later step, (b) …, (c) accepting the cap risk". Two prior REJECTED items (Wave-4 Task-2 REJECTED #7, Task-3 REJECTED #3 — "Kept unless the caller explicitly decides otherwise") were overridden by this route.
- **Judgment.** The plan's own §12 STOP gate fires ONLY on "any touched file lands >200 after the fmt round" — 183 is under, so no STOP. The override went through the sanctioned caller lane, not through implementer or simplifier discretion; it is recorded in the log (context.md: "caller-authorized, review-fix commit") and the 4.5a frontend verification reviewed it behavior-neutral (Rust mod-block order/whitespace has zero semantics; the declaration still occupies its alphabetical slot). Project benefit is real: Wave-4.1 Tasks 5–6 will edit zoom_button.rs (4 lines of headroom recorded by the simplification plan) and Tasks 3–7 will likely each need `app.rs` lines. Front-end spec §9's affected checklist item ("no new fields are added to `ImagesScrollerApp`") is unaffected — the collapse added none. **Acceptable; encode in 4.6's reference that the caller authorized it.**

### 3.2 (b) Context/structure record landed at `020c22f` — VERDICT: **ACCEPTABLE (one absorbable content gap)**

- The plan's Commit 3 (§13.3/§15) IS the context/structure record; `020c22f` delivers it: Recent Changes entry (dated 2026-11-05) naming the chain `fa8ec7e/4a2e7f0/95247dc`, the caption/tooltip set, `button_hint.rs`, +2 tests 93→95, and the collapse attribution; `project-structure.md` `app/` bullet reworded with the `button_hint.rs (...)` phrase in the §15.2 style. Both files, exact planned message.
- **Gap (LEDGER-GAP-1, docs-only, low severity):** §15.1's remaining sub-pieces were NOT done — `Current Work Focus` still reads "Task 2 … step 4.1a is complete … await their 4.1b/4.2 cycles" (not "Task 2 4.1b complete — implementation (4.2) pending"), the HEAD note is stale at `86a1c2c`, and "Immediate Next Steps" still lists Task 2 4.1b first although 4.1b/4.2/4.3/simplification/4.5a are done. The entry's chain also omits `19ff32c/020c22f` (unavoidable at landing). This is coherent-by-absence (the 4.1a wording in Current Work Focus was written at `19ff32c` and not refreshed at `020c22f`). **Substitute accepted; the refresh belongs to the 4.4 docs pass / 4.6 close of this wave, not a new fix commit.**

### 3.3 (c) Doc entry date 2026-11-05 — VERDICT: **ACCEPTED (recorded)**

Both 4.1a/4.1b/implementation context entries carry 2026-11-05 (the actual work date, matching all Task-2 plan/spec/report headers). Caller already accepted; recorded here for the audit trail.

### 3.4 (d) Simplification trims S1–S3 — VERDICT: **ACCEPTABLE, sanctioned classes, byte-verified**

`95247dc`'s `button_hint.rs` hunks delete exactly the four planned doc lines (S1: `CAPTION_TEXT_SIZE` doc, line 12 of source; S2: `CAPTION_TEXT_GAP` doc; S3: the two-line `PillButton` enum doc) — 188 → 184, zero code tokens touched; both consts, the enum, the variants, the tables and both tests are byte-identical to `4a2e7f0` (verified by reading the commit diff end-to-end). Mandated `float` values `9.0`/`2.0` preserved as named constants; `hotkey_reference`/`tooltip_text` keep their §3 doc comments and privacy (module-private per the recorded deliberate decision, §3 design notes; the 4.5a report §2 confirms, not a deviation). The simplification plan's spec-level 4.5a-site neutrality claim checked out: `button_hint.rs` 184 (not the plan §12 estimate 150–170 — the estimate included the now-deleted docs; final is 14 above the upper estimate but 16 under cap; per §20 the ±2 tolerance excludes `button_hint.rs`'s estimated range).

## 4. Test surface — VERDICT: CONFORMS

- Exactly +2 inline tests in `src/app/button_hint.rs` (`§14` names verbatim): `hotkey_reference_matches_the_spec` (155–168, 10 assertions), `tooltip_text_matches_the_spec` (170–183, 10 assertions) = 20 assertion-level expectations; no `egui::Response` mocking (pure enum → `&'static str`).
- Suite census 93 → 95: the 4.5a harness line reads `running 95 tests` (frontend verification §1 item 5), consistent with Wave-4 Task-4's pinned 93.
- No existing test file edited (`86a1c2c..020c22f` stat: no `*_tests.rs`, no `src/zoom.rs`, `src/scaling.rs` touched).

## 5. TODO Task 2 sub-item mapping (for 4.6's `[ ]` → `[x]` reference) — VERDICT: READY

- **Sub-item 1** ("each UI button displays beneath it a small hotkey reference — keyboard-distinct buttons only; the set decided in this wave's architector step"): the architector decision = plan §0.1 (user-approved set `F11/I/O/0/W/H`, arrows uncaptioned). Reference: the table `src/app/button_hint.rs:57–70` rendered beneath each glyph inside `show_button_column` (`button_hint.rs:138–144`, 9 pt `CAPTION_TEXT_SIZE`, 2.0-pt gap `CAPTION_TEXT_GAP`); per-button wrap sites — `floating_controls.rs:128–140` (arrows: tooltips, no caption), `fullscreen_button.rs:17–24` (`F11`), `zoom_button.rs:59–71` (`I`,`O`,`0`), `fit_mode_button.rs:60–72` (`W`,`H`).
- **Sub-item 2** ("hovering shows a tooltip with a short description"): `button_hint.rs:73–86` (all ten verbatim strings) attached on the SAME click Response by chain `.on_hover_text(tooltip).on_disabled_hover_text(tooltip)` — `button_hint.rs:135–137` — covering all ten buttons including painted-disabled ones; covered by the pinned test.
- Both sub-items are complete agent-side; the TODO "exact architector clause" was consumed by plan §0 (single shared helper, exact strings locked) and is materialized byte-exact in code + tests.

## 6. Out-of-scope guard — VERDICT: CONFORMS

- `keyboard_intake.rs` (locks and everything else), `arrow_direction.rs` (66 lines), `src/zoom.rs`, `src/scaling.rs`, `central_surface.rs`, `main.rs`, `Cargo.toml/lock`: absent from every diff in the chain.
- No new `ImagesScrollerApp` field (feat `app.rs` diff = 2 mod lines only; collapse adds none); no enablement/click/nav/zoom/fit seams touched (behavior-neutral check 4.5a item 3 — `accepts_click`/`clicked`/`apply_action`/`navigation_button_enabled` untouched beyond the mechanical wraps).
- No parked-idea implementation (no drag&drop / auto-hide / bottom-hover reveal code anywhere in the chain), no `CHANGELOG.md` edit, no edit to any existing `.kilo/plans/*` file (the two Task-2 plan files are new creations only), no TODO-file edit.
- Fullscreen contract intact: pill path stays behind `full_ui_chrome_is_visible()`; no extra tooltip gating added (4.5a item 6).

## 7. Harness evidence sufficiency for 4.6 — VERDICT: SUFFICIENT (one record-only caveat)

- `4a2e7f0→95247dc` is **comment-only** (4 deleted doc lines in `button_hint.rs` + whitespace/order in `app.rs`); `95247dc→020c22f` is docs-only (both commits touch `.md` files exclusively — confirmed by commit stats). The final CODE state is therefore identical at `95247dc` and `020c22f`.
- The full dev-checks harness at 4.5a (verified HEAD `020c22f`) recorded: `fmt-check`/`check`/`test`/`clippy` all PASS, `running 95 tests` with the two new Task-2 tests passing, final `ALL CHECKS PASSED`, alpine-vm MCP with `vm_status` first and exit code authoritative. This proves 95/95 green **at the final code state** (= `95247dc` code state), which is what 4.6 needs for the `[DONE]` mark. Evidence pointer for the mark: the 4.5a report §1 item 5.
- **Caveat (record-only):** the harness output exists as recorded evidence in the 4.5a report, not as a persisted log artifact — the established wave pattern (same as Wave-4 Tasks 1–3); nothing further required.
- Exactness nuance: the plan Steps §10/§11 ran at 4.2 and the harness green claim was re-established at 4.3→4.5a; the byte-level identity across `95247dc..020c22f` makes the chain of custody sound.

---

## Final ledger summary

| # | Item | Verdict |
|---|---|---|
| D1 | Plan Commit-1 `context.md` content deferred to an extra commit `19ff32c` instead of staging inside `fa8ec7e` | ACCEPTABLE — sequencing only; content stayed Task-2-scope docs; chain order docs→feat→docs preserved semantically |
| A | `app.rs` 200 → 183 collapse + alphabetization in `95247dc`, overriding Wave-4 Task-2 REJECTED #7 / Task-3 REJECTED #3 | ACCEPTABLE — caller-lane authorization (simplification plan option (a)), under the §12 STOP gate, logged in context.md, review-verified behavior-neutral |
| B | Context/structure record executed at `020c22f` | ACCEPTABLE — matches planned Commit 3 exactly; residual docs-drift (Current Work Focus / HEAD note / Immediate-Next-Steps staleness) = **GAP-1**, to be absorbed by this wave's 4.4/4.6 |
| C | Entries dated 2026-11-05 | ACCEPTED (caller), recorded |
| D | S1–S3 comment-only trims | ACCEPTABLE — sanctioned classes, zero code-token change, verified byte-level |

**VERDICT: CONFORMS** — the Task-2 commit chain `fa8ec7e → 4a2e7f0 → 95247dc → 19ff32c → 020c22f` materializes the plan's §§ exactly (wrapper tables, `Align::TOP`, verbatim captions/tooltips, `mod button_hint;`, the §7.4 glyph merge), with all five deviations caller-sanctioned or acceptable; one docs-only staleness gap (Current Work Focus/HEAD note) is flagged for wave-level absorption; the 95/95 harness evidence is sufficient for 4.6.
