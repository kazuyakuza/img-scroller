# Wave 5 — Plan Adherence Report (Overall Plan Adherence, 4.5b)

**Report path:** `.kilo/plans/20261107-wave5-adherence.md`
**Workflow step:** B-Group Step 4.5b of `.kilo/plans/20261107-wave42-then-wave5-global-plan.md`
**Adherence target:** `.kilo/plans/20261107-wave5-pan-navigate-implementation-plan.md`
**Executed chain:** `ca8b65f` (plan docs) → `272b98f` (feature) → `8058be8` (4.3-fix) → `1bfd0cc` (module docs); HEAD = `1bfd0cc`
**Branch:** `feat/wave-5-pan-then-navigate`
**Incorporates:** `.kilo/plans/20261107-wave5-pan-navigate-verification.md` (4.5a report) + the 4.3 reviewer-fix plan `.kilo/plans/20261107-wave5-review-fix.md` + the simplifier's "not applicable" caller notes
**Verdict:** CONFORMS with ONE routed one-line doc correction (§7). No functional deviations.

---

## 1. Plan-required steps → landed evidence (HEAD `1bfd0cc`)

| Plan item (§) | Required | Landed evidence | State |
|---|---|---|---|
| D1 pan step (§0) | `PAN_STEP_FRACTION = 0.125` × available axis; final step clamps | `src/pan.rs:12`; `axis_pan_step` :43–45; pure test `arrow_outcome_pans_by_the_pinned_step_when_not_at_the_edge` pins `Pan(+100.0)` = 0.125 × 800; overshoot test pins the exact clamp | PASS |
| D2 repeat policy (§0) | pan accepts OS auto-repeat; at-edge navigation needs a distinct non-repeat press; held key at edge absorbed as `Pan(0.0)` | `src/pan.rs:137–142` (`at_edge_outcome`); `src/app/pan.rs:24–39` (`matches!` over `Key { pressed: true, repeat: false }`); `was_pressed` arrow reads unchanged at `src/app/keyboard_intake.rs:38–41`; no timing logic added | PASS |
| D3 mouse pan (§0) | OUT-OF-SCOPE | No wheel/drag pan handlers in `src/`; `src/app/pan.rs:5` doc pins it | PASS |
| D4 fit suppression (§0) | MOOT (`FitMode` gone) | No `FitMode` reference anywhere; no suppression logic | PASS |
| D5 available size (§0) | controller uses `viewport_rect().size()` at intake; paint re-clamps with the central-panel rect | `src/app/keyboard_intake.rs:80/89/98/107`; `src/app_display.rs:53,59–60` (`available_rect_before_wrap()` + `clamped_pan_offset`) | PASS |
| D6 reset semantics (§0) | zero init; reset on every successful open; restore resets; W/H, zoom steps, empty-folder do NOT | `src/app.rs:44` (field) / :67 (`empty_state`) / :95 (open reset); `src/app/zoom.rs:74` (`restore_zoom_to_fit`); tests 2–5 of `src/app/pan_tests.rs` pin each clause | PASS |
| Module API (§1/§3) | `ArrowPanInputs`/`ArrowPanOutcome`/`PanAxis`/`PanSign`/`arrow_outcome`; `ArrowKeyPress`/`non_repeat_arrow_press_occurred`/`reset_pan_offset`/`handle_arrow_key`/axis+sign mapping | `src/pan.rs:64–149`; `src/app/pan.rs:16–128` — signatures, field order, `let-else` shape, and the three private helpers match the plan sketches | PASS |
| window_options relocation (§2 Step 2a) | move verbatim to `main.rs` as PRIVATE fn; call updated; `use eframe::egui`; `APP_WINDOW_TITLE` stays in `app.rs` | `src/main.rs:22–30` (byte-identical builder chain: `with_title`/`with_resizable(true)`/`with_maximized(true)`), call at :36, import at :20; `src/app.rs:15` keeps the constant | PASS |
| app state lines (§2 Step 2b) | 7 net lines; `mod pan;` between `keyboard_intake`/`pill_style`; `mod pan_tests;` after `last_viewed_tests` | `src/app.rs:42–44,67,95,164,187` — exactly as pinned | PASS |
| intake wiring (§2 Step 4) | four non-repeat reads after the four `was_pressed` reads; dispatch blocks replaced with `handle_arrow_key(...)` in Left→Right→Up→Down order; reads-before-dispatch preserved; nothing else changes; doc chord list updated | `src/app/keyboard_intake.rs:42–53` (reads), :78–113 (dispatch), all reads above the chain from :55; F11/ESC/zoom/fit chords byte-identical; doc line 3 gained "the four pan-or-navigate arrows" | PASS |
| paint composition (§2 Step 5) | 3-field `ImagePainting`; explicit `panel_ui.put(image_rect, ...)` with clamped offset; centered layout gone; doc contract updated; texture code untouched; `central_surface.rs` passes `pan_offset` | `src/app_display.rs:17–21,43–63,85–117` (texture code byte-stable); `src/app/central_surface.rs:31–43` (field added after `zoom_factor`); both doc texts updated | PASS |
| restore seam (§2 Step 6) | one line `self.reset_pan_offset();` in `restore_zoom_to_fit`; `src/zoom.rs` doc line updated to present tense | `src/app/zoom.rs:72–75`; `src/zoom.rs:63` "Wave-5 pan composes its clamped offsets on this size" | PASS |
| pinned-untouched verify (§2 Step 7) | floating_controls/esc/fullscreen/folder_navigation unchanged | `git diff` over the chain confirms zero changes to those modules (except the review-fix derive below, §4) | PASS |
| 23 tests (§2 Steps 1/3, §5) | exactly 11 pure + 12 app tests, pinned names/order; census 127 → 150 | `src/pan/pan_tests.rs` (11) and `src/app/pan_tests.rs` (12) — every pinned name present; static census count at HEAD = 150 (52 non-app + 98 app = 127 pre-wave + 23); the plan's optional `pan_navigation_tests.rs` split did NOT fire (app file = 193 ≤ 200; plan sanctions the single-file shape) | PASS |
| commit discipline (§2 Step 9) | message `feat: pan then navigate at image edges while zoomed`; staged set = planned files | `272b98f` stat matches the §2 Step 9 list exactly | PASS |

## 2. Judged corrections within plan scope

1. **Copy derives (`8058be8` Fix 1)** — WITHIN-SCOPE. The plan's own §2 Step 3 helper signatures pass `ArrowKeyPress` (and reuse `press.direction`) by value more than once; the plan's sketch omitted the derives, and the reviewer's fix plan (`20261107-wave5-review-fix.md` Edit 1a/1b) supplies the minimal compile-correcting derive on two fieldless/data-only types. `git show 8058be8` confirms the diffs are exactly the two `#[derive(Clone, Copy)]` lines — no runtime behavior change. Note: `src/app/folder_navigation.rs` is on the plan §4 no-touch list; this derive-only touch is adjudicated as the sanctioned exception because the plan's own code requires it (root cause is a plan-sketch omission, not scope expansion).
2. **Tests 8/10/11 attempt-assertions (`8058be8` Fix 2)** — WITHIN-SCOPE TEST-SEMANTICS CORRECTION. The plan's Step 3 test list pins test NAMES and BEHAVIOR CLAIMS, not assertion mechanics; its §5 census pin explicitly says the harness's actual count is authoritative. The rewrite (assert pre-press target resolution via `neighbor_image_path`/`sibling_folder_target_image_path`, then `message_text == UNSUPPORTED_FILE_MESSAGE` + kept image) preserves every claim: the edge press routes through `navigate` → `open_image` (the failure message is the attempt evidence on the Linux stub) and resolves the correct target. The dropped sub-claim of test 8 ("new image's pan offset is zero") remains covered by test 2 (`every_successful_open_resets_the_pan_offset`). Census unchanged: still 12 app tests, 150 total.
3. **Simplifier "not applicable"** — CLOSED. The two caller notes (app.rs effective-143 over the soft ideal — flagged architect-level, deferred; two `pub(crate)` edge predicates that could be private — zero-value, left) require no action in this wave; nothing landed as a simplification commit and none is owed.

## 3. 4.5a report incorporation

- Its 15-item conformance checklist was independently re-verified against HEAD reads in §1 above — all 15 PASS confirmations hold (its line references match the landed code).
- Its §2 D5-signature confirmation matches the landed `handle_arrow_key`/intake/paint split.
- Its §4 six quality observations (pan-step feel at 0.125, the 0.5 px edge tolerance, landing-press-vs-next-press feel, ×8/floor extremes, empty-state/empty-folder arrow routing, fullscreen-ON parity) are all operator-feel/visual items — manual-gate-class, correctly deferred to B-Task 4's OPERATOR-RUN; none is agent-failable here.
- Its §5 administrative follow-up ("structure doc stale") is RESOLVED: `1bfd0cc` updated `.agent/project-structure.md` to list `src/pan.rs`, `src/pan/pan_tests.rs`, the `app/pan.rs` child module + `pan_tests.rs`, and the `main.rs` window-options relocation.

## 4. Minor, recorded-not-blocking items

1. **Plan-text arithmetic slip (test 6):** the plan pinned app test 6's expectation as "+200.0 (0.125 × 1600)" while its own D1 and decision-table row 6 define the step as 0.125 × AVAILABLE axis (= 0.125 × 800 = 100) and its own pure test 7 pins `Pan(+100.0)`. The landed test asserts +100.0 — conformant to the governing definition (D1). No action; the plan text, not the code, was internally inconsistent.
2. **4.3-fix commits touching `folder_navigation.rs`** — see §2.1; adjudicated in-scope.

## 5. Budgets and no-touch verification at HEAD

- Line counts (all ≤ 200): `app.rs` 195, `main.rs` 49, `pan.rs` 152, `pan/pan_tests.rs` 140, `app/pan.rs` 129, `app/pan_tests.rs` 193, `app/keyboard_intake.rs` 141, `app_display.rs` 118, `app/central_surface.rs` 44, `app/zoom.rs` 149, `zoom.rs` 129, `app/folder_navigation.rs` 171.
- Census: 150 (static `#[test]` count; 52 outside `app/` + 98 inside). B-Task 4's harness run remains the authoritative confirmation.
- `git diff --name-only ca8b65f 1bfd0cc` = exactly the plan's Step 9 file list + `folder_navigation.rs` (review-fix derive) + `20261107-wave5-review-fix.md` (4.3 output) + `.agent/project-structure.md` (module-docs step). The plan §4 no-touch set (`Cargo.toml`/`Cargo.lock`, `CHANGELOG.md`, `src/platform/**`, listing/collation/last-viewed/image-loading/scaling modules, all pill/button/ESC/fullscreen modules, existing plan files) is untouched. No `.gitignore`-matching paths appear.

## 6. Global-plan cross-check

The B-Group chain of `.kilo/plans/20261107-wave42-then-wave5-global-plan.md` (4.1a spec → 4.1b plan → 4.2 feature → 4.3 review → 4.4 docs → 4.5a → 4.5b → 4.6 pending) is intact: all artifacts exist at the pinned paths, the branch matches, and the remaining steps are exactly B-Group 4.6 (marks Tasks 1–3), B-Task 4 (verification: harness + 0.5.0 exe rebuild + 22-DLL audit + OPERATOR-RUN checklist), B-Task 5 (docs), and Step B.5 (rename/merge/push).

## 7. Routed correction (owed, one line — caller routes; NOT edited here)

**Deviation:** `src/app.rs:1` module header still reads `//! Application shell: the resizable viewer window of Images Scroller, maximized at every launch.` — after `window_options()` moved to `src/main.rs` (Wave 5, Step 2a), the resizable/maximized facts are constructed in `main.rs`, and the corrected `.agent/project-structure.md` entries (1bfd0cc) attribute them there. Two module-level docs at HEAD now disagree; judged **(b) a one-line `//!` correction owed**, consistent with this codebase's established stale-header-fix precedent (1bfd0cc itself fixed `app_display.rs`'s stale header line the same way).

**Exact edit (single line replacement at `src/app.rs` line 1):**

```
OLD: //! Application shell: the resizable viewer window of Images Scroller, maximized at every launch.
NEW: //! Application shell of the Images Scroller viewer (the resizable, always-maximized viewer-window options moved to `src/main.rs` in Wave 5).
```

Suggested routing: fold into B-Task 5's documentation work (or a one-file docs commit), message e.g. `docs: wave5 app.rs module header window-options relocation note`.

## 8. Verdict

**CONFORMS with the §7 routed one-line doc correction.** All functional, structural, API, test, census, and budget requirements of `.kilo/plans/20261107-wave5-pan-navigate-implementation-plan.md` are met at HEAD `1bfd0cc`; the 4.3-fix and 4.5a inputs are fully incorporated. No code changes owed; no commit/push performed by this step.
