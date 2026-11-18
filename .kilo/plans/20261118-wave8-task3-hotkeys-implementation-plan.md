# Wave 8 — Task 3 (Hotkeys list: verify, then complete) — Implementation Plan

Step: Critical Workflow **4.1b** (Analysis and Planning) for Wave 8, Task 3 — architector step.
TODO: [.agent/todos/20261117/20261117-todo-1.md](../../.agent/todos/20261117/20261117-todo-1.md) — Task 3 verbatim:

> - [ ] VERIFY FIRST whether the README/docs already list ALL program hotkeys and their uses (the README's Controls table + per-feature text cover F11/ESC/I/O/0/W/H/arrows/Q as of 2026-11-17); if the list is complete, record the verification verdict only; if gaps are found, add a consolidated hotkeys table (key → use) so every hotkey is listed in one place.

Implementer profile: JUNIOR developer under 50% restriction — every structural/scope decision is pinned below; no judgment calls are left open.
Not front-end related → NO 4.1a front-end spec and NO 4.5a front-end verification in this task's cycle (only 4.1b → 4.2 → 4.3 → 4.4 → 4.5b → 4.6).

---

## §0 Verification evidence (produced NOW, in this 4.1b step, 2026-11-18)

### §0.1 Authoritative hotkey inventory (source of truth: the implemented intake)

Built from an exhaustive sweep of `egui::Key::` and `egui::Event::Text` across `src/` (production code; test files excluded — test-only constants pin the production ones and add no new keys). The COMPLETE set of keyboard chords the app implements:

| # | Key / chord | Use | Source reference (file:line) | README location | Verdict |
|---|---|---|---|---|---|
| 1 | `F11` | Toggle fullscreen ON/OFF (with the pill's fullscreen-square button; one shared action) | `src/app/fullscreen.rs:14` (`FULLSCREEN_TOGGLE_KEY = egui::Key::F11`) | Controls table line 49 ("Toggle fullscreen … F11 (ESC also exits while fullscreen is ON)") + feature 4 text line 40 | LISTED |
| 2 | `ESC` (while fullscreen ON) | Exits fullscreen (never toggles it ON; no-op otherwise) | `src/app/fullscreen.rs:17` (`FULLSCREEN_EXIT_KEY = egui::Key::Escape`) + the pure predicate `escape_exits_fullscreen` at `:21–23`, wired at `src/app/keyboard_intake.rs:41–44` | Controls table line 49 + feature 4 text line 40 ("F11 — or ESC — exits fullscreen while ON") | LISTED |
| 3 | `ESC` ×2 (while fullscreen OFF) | Close the application: FIRST distinct press arms the centered "Press ESC again to close" popup for 2 s; a SECOND DISTINCT press closes; held/autorepeat never counts; expiry re-arms; every fullscreen toggle disarms | `src/app/esc_close_confirm.rs:29` (`POPUP_MESSAGE_TEXT`), `:32` (`POPUP_VISIBLE_SECONDS = 2.0`), `:93–95` (`non_repeat_escape_press_occurred`), `:63–72` (`esc_close_action`); dispatch at `src/app/keyboard_intake.rs:148–165` | Controls table line 50 ("ESC pressed twice — … second distinct press closes (holding ESC never counts)") + feature 4 text line 40 | LISTED |
| 4 | `ESC` (while the Open-with menu is open) | Closes ONLY the open-with menu popup (never arms the ESC-close popup, never exits fullscreen) | `src/app/keyboard_intake.rs:184–189` (`open_with_menu_close_pressed`) + the modal guard `:136–144` | feature 7 text line 43 ("`ESC` or `Q` again closes the menu (`ESC` closes ONLY the menu …)") | LISTED |
| 5 | `I` | Zoom in one step (×1.25; silent at the ×8 ceiling) | `src/app/zoom.rs:46` (`ZOOM_IN_KEY = egui::Key::I`) | Controls table line 51 ("Zoom in / zoom out … I / O") + feature 5 text line 41 ("`I` zooms in") | LISTED |
| 6 | `O` | Zoom out one step (×1.25; silent at the ÷8 floor) | `src/app/zoom.rs:48` (`ZOOM_OUT_KEY = egui::Key::O`) | Controls table line 51 + feature 5 text line 41 ("`O` zooms out") | LISTED |
| 7 | `0` | Restore whole-image fit (zoom factor back to ×1.0; pan reset) | `src/app/zoom.rs:50` (`RESTORE_FIT_KEY = egui::Key::Num0`) + `restore_zoom_to_fit` `:75–78` | Controls table line 52 ("Restore whole-image fit … 0") + feature 5 text line 41 ("The `0` key … return to whole-image fit at ×1.0") | LISTED |
| 8 | `W` | One-shot fit-width size set (width fills the window; zoom keeps working from the set level; silent) | `src/app/zoom.rs:53` (`FIT_WIDTH_KEY = egui::Key::W`) + `set_zoom_to_fit_width` `:86–98` | Controls table line 53 ("Fit width / fit height (one-shot size set) … W / H") + feature 5 text line 41 | LISTED |
| 9 | `H` | One-shot fit-height size set (the W twin) | `src/app/zoom.rs:55` (`FIT_HEIGHT_KEY = egui::Key::H`) + `set_zoom_to_fit_height` `:101–113` | Controls table line 53 + feature 5 text line 41 | LISTED |
| 10 | `←` Left arrow | Pan-then-navigate: pans left while the zoomed image overflows horizontally; navigates to the PREVIOUS sibling image only at the image's left edge; direct when it fits | `src/app/keyboard_intake.rs:52` + the non-repeat read `:56–58`, routed through `handle_arrow_key` (`src/app/pan.rs`) | Controls table line 47 ("Navigate previous/next image … pans first …") + feature 6 text line 42 | LISTED |
| 11 | `→` Right arrow | Same rule, NEXT sibling image | `src/app/keyboard_intake.rs:53` + `:59–61` | Controls table line 47 + feature 6 text line 42 | LISTED |
| 12 | `↑` Up arrow | Pan-then-navigate on the vertical axis: PREVIOUS sibling folder | `src/app/keyboard_intake.rs:54` + `:62–63` | Controls table line 48 + feature 6 text line 42 | LISTED |
| 13 | `↓` Down arrow | Same rule, NEXT sibling folder | `src/app/keyboard_intake.rs:55` + `:64–66` | Controls table line 48 + feature 6 text line 42 | LISTED |
| 14 | `Q` | Open-with menu toggle: opens the menu exactly like the button click (fresh list refresh); pressing Q again closes it; non-repeat reads only | `src/app/open_with_button.rs:33` (`OPEN_WITH_MENU_KEY = egui::Key::Q`); read at `src/app/keyboard_intake.rs:175–182` | Controls table line 54 ("Q (opens the menu exactly like the button click; pressing Q again closes the open menu)") + feature 7 text line 43 | LISTED |
| 15 | Shift+`+` family (zoom-IN aliases) | Zoom in one step: the shift-gated key members `Equals`/`Plus` AND the shift-gated typed characters `'+'`, `'='`, `'*'` (the `'*'` member is the Wave-5.1 Spanish-layout shifted-plus variant; on US-like layouts Shift+8 also zooms in — documented side effect); plain unshifted keys inert | `src/app/zoom_alias.rs:23` (`ZOOM_IN_ALIAS_KEYS = [Equals, Plus]`), `:25` (`ZOOM_IN_ALIAS_CHARACTERS = ['+', '=', '*']`), per-member shift gate `:28–30` | Controls table line 51 ("strict shifted aliases: Shift+'+' — including its shifted-plus variants, e.g. the typed * glyph — or Shift+'=' in … unshifted +/=/*/− inert") + feature 5 text line 41 (full round-2 explanation incl. the typed-character channel) | LISTED |
| 16 | Shift+`−` family (zoom-OUT aliases) | Zoom out one step: the shift-gated key member `Minus` AND the shift-gated typed characters `'-'`, the true minus sign `U+2212`, and `'_'`; plain unshifted keys inert | `src/app/zoom_alias.rs:24` (`ZOOM_OUT_ALIAS_KEYS = [Minus]`), `:26` (`ZOOM_OUT_ALIAS_CHARACTERS = ['-', TRUE_MINUS_SIGN, '_']`) | Controls table line 51 ("Shift+'−' out; unshifted +/=/*/− inert") + feature 5 text line 41 ("recognized on BOTH the key and the typed-character channels … Shift+'−' zooms out") | LISTED |

Completeness proof of the sweep: the grep `egui::Key::` over `src/**.rs` yields, in production files, EXACTLY the constants in the table — `F11` and `Escape` (`fullscreen.rs:14/:17`; `Escape` again in `esc_close_confirm.rs:94` and the intake's menu-close read), `ArrowLeft/Right/Up/Down` (`keyboard_intake.rs:52–55` + the pan non-repeat reads `:56–66`), `Equals`/`Plus`/`Minus` (`zoom_alias.rs:23–24`), `I`/`O`/`Num0`/`W`/`H` (`zoom.rs:46/:48/:50/:53/:55`), `Q` (`open_with_button.rs:33`). The ONLY `egui::Event::Text` intake in production code is `src/app/zoom_alias.rs:70–76` (the typed alias channel). NO Ctrl/Alt/Meta chord exists anywhere in `src/`; opening images is click/CLI-argument only; there are no other keyboard bindings. ⇒ the table above is the COMPLETE authoritative hotkey inventory.

### §0.2 The zoom-alias representation decision (pinned)

If a hotkeys table were ever needed, the alias layer appears as the TWO user-facing chords — `Shift+'+'` (including `Shift+'='` and the typed `*` shifted-plus variant) for zoom-in and `Shift+'−'` for zoom-out — NOT one row per internal member. This is exactly how the README already represents them (Controls table row line 51; feature 5 text line 41). The typed `'_'` member (`zoom_alias.rs:26`) is NOT a separate user chord: on US-like layouts Shift+`-` types `_` — it IS what the Shift+`−` chord physically produces — so the README sentence "Shift+'−' zooms out", together with the explicit declaration that the aliases are "recognized on BOTH the key and the typed-character channels", covers it. The full internal member set (`'-'`/`U+2212`/`'_'`) is documented for the technical audience in `.agent/project-info/tech.md:97` and `tech.md:107`. ⇒ COVERED, not a gap.

### §0.3 Mouse-equivalents (recorded separately, no action)

The floating pill's 11 buttons are click-only (no hotkeys): 4 painted arrows, fullscreen square, `[+]`/`[−]`/restore-fit, fit-width/fit-height, Open-with. The README Controls table's "On-screen (bottom pill)" column (README lines 47–54) plus the caption/tooltip paragraph (line 56: the seven hotkey captions `F11`/`I`/`O`/`0`/`W`/`H`/`Q` under their buttons and all 11 tooltips) cover them. They are NOT hotkeys and stay OUT of the hotkey inventory; no documentation action results from them.

### §0.4 VERDICT

**COMPLETE — zero gaps.** The README already IS the consolidated hotkeys record the user asked for: the Controls table (README lines 45–54) + the per-feature text (lines 38–43) list every one of the 16 inventory rows WITH its use. The gap branch of the TODO ("add a consolidated hotkeys table") is NOT taken.

---

## §1 Settled decisions (all pinned — none left open)

- **D1 — Verdict: COMPLETE (§0.4).** Action = "record the verification verdict only". NO consolidated hotkeys table is added anywhere.
- **D2 — NO file-content edit of any documentation or source file in Task 3.** Rationale: (a) the TODO's edit branch is conditional on gaps, and none exist; (b) the README's existing Controls table + per-feature text already is the single consolidated record; (c) per project precedent, process/verification metadata stays OUT of user-facing README (dated audit notes live only in technical docs like `docs/build-windows.md`, and none is needed here — `tech.md` already documents the alias member set); (d) the wave's Task 6 owns all README content changes (installer-first instructions, releases pointer, window-title behavior) — a Task-3 README edit would collide with Task 6's rewrite of the same document.
- **D3 — Where the verdict is recorded (exactly three places):** (a) this plan file §0 (committed with the wave); (b) the NEW results file `.kilo/plans/20261118-wave8-task3-hotkeys-verification-results.md` written by the 4.2 implementer (the durable, dated verification record — the Wave-6.x/7 results-file precedent); (c) the 4.6 `[DONE]` mark on the TODO heading. NOT in README; NOT in `context.md` (the wave's Task 6 owns the context-log entry; the Planner copies the verdict there at that step if desired — out of this task's scope).
- **D4 — Task-6 carry-forward (binding note for the caller/Task-6 planner):** TODO Task 6's README item "the hotkeys record (Task 3)" is DISCHARGED by this verdict — Task 6 adds NO hotkeys table and NO hotkeys note to README; the verified Controls table + per-feature text stand as the record. Task 6's README work covers only its other content (installer-first instructions, GitHub Releases pointer, window-title behavior).
- **D5 — Sub-step flow:** 4.1a NOT applicable (not front-end) → this 4.1b only; 4.5a NOT applicable; 4.5b applies. This task adds NO code, NO tests, NO dependencies → NO dev-checks/harness run and NO `alpine-vm` MCP usage in Task 3 (no build/test command exists for it; the wave's Task 5 runs the harness). Census stays 195 unit tests / 28 test-bearing files — unchanged.
- **D6 — Git discipline:** all work on the wave branch `feat/wave-8-wizard-installer` (created at workflow Step 2). If the branch is absent/misnamed at 4.2 time → STOP and ask the caller. NO branch creation, NO switch, NO push (push is Step 5, `main` → `origin` ONLY), NO version bump (Step 3; Task 3 changes nothing versioned). Commit messages pinned in §2.
- **D7 — Verification re-run venue:** host-side file reads/greps on the working tree (`C:\repo\img-scroller`) using the built-in Read/Grep tools. NO container, NO VM — nothing here compiles or runs.

---

## §2 Sub-steps 4.2–4.6 (each is ONE task-tool invocation by the Planner)

### §2.1 Step 4.2 — Implementation (implementer; the verification-evidence run)

1. **Preconditions:** `git status` → working tree has only this untracked plan file (plus nothing unexpected); `git branch --show-current` → `feat/wave-8-wizard-installer`. Anything else → STOP and ask.
2. **Commit the plan file:** `git add .kilo/plans/20261118-wave8-task3-hotkeys-implementation-plan.md` → commit `docs: add wave8 task3 hotkeys verification plan`.
3. **Re-run the verification procedure as EVIDENCE** (all host-side; every check must match §0 exactly):
   - **Sweep A (inventory):** Grep pattern `egui::Key::` over `src` (`*.rs`) → the production-file matches must be EXACTLY the pinned lines of §0.1 col. 4 (fullscreen.rs:14/:17; esc_close_confirm.rs:94; keyboard_intake.rs:52–55/:56–66; zoom_alias.rs:23–24; zoom.rs:46/:48/:50/:53/:55; open_with_button.rs:33) plus their test-file pins. Grep pattern `Event::Text` over `src` → production matches ONLY in `src/app/zoom_alias.rs` (module `:70–76` area) plus `zoom_alias_tests.rs`. ANY additional production key constant or text intake that is NOT in the §0.1 table → STOP and ask (the inventory changed).
   - **Sweep B (README coverage):** Grep `README.md` for each row's pinned evidence and confirm the expected line: `F11` (lines 40, 49), `Press ESC again to close` (40, 50), `ESC\` or \`Q\` again closes the menu` (43), `I\` zooms in` (41), `O\` zooms out` (41), `The \`0\` key` (41) + `Restore whole-image fit | 0` (52), `Fit width / fit height` (53), `Navigate previous/next image` (47), `Navigate previous/next sibling folder` (48), `hotkey is \`Q\`` (43, 54), `Shift+'+'` (41, 51), `Shift+'−'` (41, 51), `typed-character channels` (41), `unshifted +/=/*/− inert` (41, 51). All 16 rows read LISTED.
   - **Sweep C (gap probe):** Grep `README.md` for the `'_'` alias member (pattern `'_'\|Shift\+_\|underscore`) → expect NO README match (the member is documented in `tech.md:97`/`:107` and covered by the typed-channel declaration per §0.2). Record the probe output in the results file as the COVERED note. If the implementer believes this is a genuine gap → STOP and ask; NEVER flip the verdict unilaterally.
4. **Write the results file** `.kilo/plans/20261118-wave8-task3-hotkeys-verification-results.md` (NEW file — never edit an existing plan file). Content skeleton (pinned): title "Wave 8 Task 3 — hotkeys verification results"; date 2026-11-18; branch + HEAD SHA; verdict **COMPLETE (zero gaps)**; the 16-row evidence table (copy §0.1's table, updating any line numbers if the tree moved); Sweeps A–C outputs summarized; the §0.2 alias-representation note; the §0.3 mouse-equivalents note; the D4 Task-6 discharge note; the explicit "no file was edited" statement (D2).
5. **Commit the results:** `git add .kilo/plans/20261118-wave8-task3-hotkeys-verification-results.md` → commit `docs: wave8 task3 hotkeys verification results`.
6. **Return the summary:** verdict + what was done + what was NOT done (no README/src/docs edit, no harness run).

### §2.2 Step 4.3 — Code Review & Simplification

- NO code changed → code-reviewer and code-simplifier each return a clear "not required" message. NO fix plan, NO simplification plan. (If either insists on producing a plan, the Planner adjudicates: Task 3's scope is docs-verification-only — there is nothing to review/simplify.)

### §2.3 Step 4.4 — Documentation

- Verify-only: README + tech.md already carry the full hotkeys record (§0.1 is the proof). The docs-specialist confirms against the results file and returns a clear no-change message. NO edits, NO commit.

### §2.4 Step 4.5b — Overall Plan Adherence (no 4.5a)

- The architector checks 4.2's evidence + the results file against this plan: expected verdict **CONFORMS** (verdict-only branch taken; zero file edits beyond the two `.kilo/plans/` artifacts; commits exactly as pinned). Any deviation → a new plan file per the workflow.

### §2.5 Step 4.6 — Task Completion (implementer)

- Edit `.agent/todos/20261117/20261117-todo-1.md` line 25: `### Task 3: Hotkeys list (verify, then complete)` → `### Task 3: Hotkeys list (verify, then complete) [DONE]`. Preserve EVERY other byte of the file (the task has no `[ ]` sub-items — nothing else to mark). Commit: `task: mark wave8 task3 done`.

---

## §3 What NOT to do (hard prohibitions)

- NO changes to `src/**` of any kind; NO `Cargo.toml`/`Cargo.lock` change; NO tests.
- NO edit to `README.md`, `tech.md`, `architecture.md`, `context.md`, `brief.md`, `CHANGELOG.md`, or `docs/*` — the verdict-only branch means ZERO documentation edits (D2/D3).
- NO consolidated hotkeys table anywhere (the gap branch is NOT taken).
- NO TODO edits beyond the single Task-3 heading `[DONE]` mark (Overwrite-TODO-Prevention Rule); NO TODO rename (that is wave Step 5).
- NO push, NO branch create/switch, NO version bump, NO dev-checks/build commands, NO `alpine-vm` MCP calls (D5/D6/D7).
- NO scope expansion into Tasks 1/2/4–7 (installer, TOC, window title, verification, docs, releases) or into `future-ideas.md` items.

## §4 Final cross-check (the Task-3 verification, run at 4.6)

1. `git log --oneline -5` on the branch shows, in order, `task: mark wave8 task3 done` ← `docs: wave8 task3 hotkeys verification results` ← `docs: add wave8 task3 hotkeys verification plan` atop the wave base — exactly three new commits, nothing else.
2. `git diff <wave-base>..HEAD -- README.md src` → EMPTY (proves the zero-edit record: the README still lists every inventory item because it was never touched — §0.1's verdict stands as of the verified tree).
3. `git diff HEAD~1 -- .agent/todos/20261117/20261117-todo-1.md` → EXACTLY one changed line (the Task-3 heading `[DONE]` mark).
4. Re-read the results file → all 16 rows LISTED; verdict COMPLETE; `git status` clean.
5. Gitignore Compliance Rule before every commit: read `.gitignore`, run `git status`, stage only the named files.

## §5 STOP conditions (return a question to the caller, never guess)

- The wave branch is missing/misnamed; unexpected staged/unstaged content at 4.2 start.
- Sweep A finds a hotkey constant or text intake not in §0.1 (the implemented intake changed — the inventory and verdict must be re-derived first).
- Sweep B finds any inventory row NOT listed in the then-current README (e.g. another task edited it between 4.1b and 4.2 — re-verify against that tree; if a genuine gap appears, STOP: the gap branch needs caller approval).
- Sweep C seems like a real gap to the implementer (§0.2 adjudicates it as COVERED; disagreement → ask).
- Any git/commit failure beyond the trivial retry.

---

## §6 Plan-vs-task self-check

- VERIFY FIRST: done — §0 is the verification, run in this 4.1b step, with per-item source+README evidence.
- Complete → "record the verification verdict only": encoded as D1/D2/D3 (verdict recorded in plan §0 + the results file + the 4.6 mark; zero edits).
- Gap branch: fully specified as the STOP condition in §5 (not taken; if it ever triggers, the caller approves before any table is added).
- Sub-steps, commits, prohibitions, cross-check: §2–§4. Task-6 interaction: D4.
