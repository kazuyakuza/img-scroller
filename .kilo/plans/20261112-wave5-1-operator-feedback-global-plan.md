# Global Plan — Session 2026-11-12: Wave 5.1 (Operator Feedback Round 2)

Session request: process the COMPLETED 2026-11-12 operator report as TODO file `.agent/todos/20261112/20261112-todo-1.md` — Wave 5.1, version 0.5.0 → 0.5.1 (patch), branch `fix/wave-5-1-operator-feedback`. THREE defects (D1 Shift+'+' alias round 2; D2 popup expiry; D3 Japanese name order), one passed gate closed (Wave-5 pan), one parked future idea recorded (pan enhancements — record only).

Pre-analysis: base = `main` == `origin/main` == `b1435e7` (clean). Binding facts at HEAD: `src/app/zoom_alias.rs` accept-set (keys `Equals`/`Plus` in, `Minus` out; typed `'+'`/`'='` in, `'-'`/`U+2212`/`'_'` out — all shift-gated); `src/app/esc_close_confirm.rs` (`POPUP_VISIBLE_SECONDS = 2.0`; `current_time` = `input.time`; `update_esc_close_popup` runs per frame in the intake between reads and dispatch); `src/name_collation.rs` `compare_entry_names` (lowercase fold + exact-character tie-break — shared by `src/folder_listing.rs` + `src/sibling_folder_listing.rs`, which feed BOTH navigation axes); `src/platform/` cfg-gated facade precedent for OS-bound seams with container-testable stubs; census 150 across 23 files; harness green; exe 19,606,124 bytes (0.5.0). The two keyboard/popup defects ship BECAUSE their real-World behaviors were never covered by the unit harness (alias: synthetic-key tests only; expiry: pure `_at` cores only).

## Workflow steps

### Step 2 — Git Feature Branch Setup → implementer
- Clean-tree check; verify `main` @ `b1435e7`; create + switch to `fix/wave-5-1-operator-feedback`; commit the untracked Wave-5.1 todo + global plan (two `docs:` commits per the established precedent).

### Step 3 — Version Update → implementer
- `Cargo.toml` + `Cargo.lock` `0.5.0` → `0.5.1` (hand-aligned lock per precedent); commit `chore: bump version to 0.5.1`.

### Task 1 — Record the completed report (NOT front-end; light cycle)
- 4.1b architector plan → 4.2 implementer (mark `20261029-todo-7-DONE.md` Wave-5 manual-UX sub-item `[x]`; context.md dated entry with the full results incl. the D3 assignment and the closed Wave-5 gate; `future-ideas.md` gains the parked pan-enhancements idea — record only) → 4.3 reviewers (no-change expected) → 4.4 N/A → 4.5b → 4.6 mark.

### Task 2 — Fix the Shift+'+' zoom-in alias, round 2 (NOT front-end → 4.1b only)
- 4.1b architector: root-cause WITH THE NEW EVIDENCE (Shift+'−' fires; Shift+'+' delivers nothing the round-1 accept-set catches) against the RESOLVED egui-winit 0.37 source inside the container — pin the real Windows translation of the shifted '=' row key (logical Character('+') path, physical member, label/char mapping coverage) with cited source lines; decide the accept path (likely the key event's `physical_key` member at the '=' position, shift-gated — evidence decides); keep/justify the unproven text-member entries; plan synthetic tests reproducing each accepted/rejected resolution (existing `zoom_alias_tests.rs` Context-run pattern); budgets; commit segmentation; harness deferred to Task 5. DIAGNOSTIC-EXE FALLBACK only on caller authorization (STOP + question).
- 4.2 implementer → 4.3 reviewers (+fix) → 4.4 docs-specialist → 4.5b → 4.6 mark.

### Task 3 — Fix the popup 2-second expiry (NOT front-end; intake/state logic → 4.1b only)
- 4.1b architector: root-cause the never-passing deadline on real Windows (candidates: non-advancing `input.time`; a gated/skipped per-frame update; a wrong time base — evidence with file/line citations); fix the failing link WITHOUT regressing the proven contracts (close-on-second-distinct-press, re-arm, disarm-on-fullscreen-ON, fullscreen-ON ESC exit); ADD the missing ctx-level test driving `Context::run_pass` with advancing `RawInput.time` through the REAL intake wiring; budgets; commit segmentation.
- 4.2 implementer → 4.3 reviewers (+fix) → 4.4 docs-specialist → 4.5b → 4.6 mark.

### Task 4 — Fix the navigation name order for Unicode/Japanese names (NOT front-end; pure listing/collation logic → 4.1b only)
- 4.1b architector: root-cause what `compare_entry_names` produces for Japanese-mixed names and why it diverges from the expected order (code-point order vs locale-aware collation; cite mechanics); DECIDE the target order — research the mechanism (leading candidate: the Windows collation API behind the cfg-gated `platform` facade with a container-testable stub seam, so both listing modules stay container-testable; the exact API/rules are the architector's decision, the TODO does not name them) — then encode: the seam shape, the collation consumer changes (BOTH axes through the same seam), the test plan (Japanese-mixed fixtures at the collation seam + both listing consumers), the ASCII-behavior check (pure-ASCII names keep working sensibly), and the first-image/last-viewed rule verification (they read the same sorted listings — no separate change expected).
- 4.2 implementer → 4.3 reviewers (+fix) → 4.4 docs-specialist → 4.5b → 4.6 mark.

### Task 5 — Verification (NOT front-end; light cycle)
- 4.1b architector verification plan (house format): fresh dev-checks at HEAD (discovery run — conditional fmt/compile-fix branches; census re-derived with the three fix tasks' deltas), wave-pin sweeps (round-2 accept path; expiry wiring; collation seam; no-touch list: `message_text` 3 sites, ESC semantics, pan controller, styling), release rebuild (0.5.1 exe; record bytes; no `.part`), DLL audit set-equal 22, gitignore sweep, commit segmentation, and the §-manual UX checklist copied verbatim from the todo's Task 5 (the SINGLE 0.5.1 re-test: alias + expiry re-checks; the NEW name-order check on BOTH axes with Japanese-mixed names; standing regressions incl. the passed Wave-5 pan behaviors); manual-UX sub-item stays `[ ]` until the user reports.
- 4.2 implementer → 4.3 reviewers (light) → 4.4 N/A → 4.5b → 4.6 mark.

### Task 6 — Documentation updates (NOT front-end; light cycle)
- 4.1b architector docs plan → 4.2 docs-specialist writes: README/tech/architecture/context (three fixes + report results + 0.5.1 state; trim the flagged stale Wave-4.1 historical state bullet) + brief.md addendum item 8 (navigation order follows the names' expected Unicode/system order — refining `## Sorting Order` for non-ASCII names) → 4.3 reviewers (light) → 4.5b → 4.6 mark.

### Step 5 — TODO File Completion → implementer
- Rename `20261112-todo-1.md` → `-DONE`; tmp sweep; checkout `main`; fast-forward merge `fix/wave-5-1-operator-feedback`; verify + delete branch; push `main` → `origin` ONLY; notify on failure.

## Session closing
- context.md final update (docs-specialist in Task 6's write covers it; one closing touch after Step 5 if the final state shifts) → Workflow Step 6 finish message: resume + the next-session prompt (Wave 6 = `.agent/todos/20261029/20261029-todo-8.md`, "Open with..." menu).

## Error handling
- Sub-agent failure/empty response → resume the same step; repeated failure → split smaller; endless failure → escalate to the user.
- Harness/VM/share unavailability → STOP + user question.
- The Task-2 diagnostic-exe fallback → STOP + question tool (caller decision).
- Max 3 review cycles per task, then escalate.
