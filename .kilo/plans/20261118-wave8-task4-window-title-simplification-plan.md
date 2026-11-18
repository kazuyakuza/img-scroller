# Wave 8 Task 4 — Window Title Simplification Plan (4.3)

Status: review verdict = **4 local simplifications warranted** (all small, atomic, behavior-preserving). No structural change is proposed; the D15 module split stays.

Baseline (verified 2026-11-18): branch `feat/wave-8-wizard-installer`, HEAD `1234b45`, dev-checks green — exit `0`, `ALL CHECKS PASSED`, `running 213 tests` / all passed, 29 test-bearing files (`grep -rl '#[test]' src | wc -l` = 29). The post-plan §7 "212" is the plan's own arithmetic slip (195 + 18 = 213); **213/29 is the census every verification below must reproduce**.

Binding constraints (do NOT touch): the implementation plan's decisions D1–D16, the 18-test matrix in `src/window_title/tests.rs` (pinned — no test is edited, added, or removed), all five task constants and their values, the D7 step order and Interpretation A, the D10 arm/disarm/wake behavior, and every hard prohibition in §8 of the implementation plan.

Companion edit: the code-review fix plan `20261118-wave8-task4-window-title-review-fix.md` Issue 1 rewrites the BODY of `arm_fullscreen_name_popup` only. None of the items below touch that function's body, so the two tracks compose in either application order. If both are executed, run the review fix first, then S1–S4, then the ONE final verification.

Rustfmt caveat for the implementer: after applying S1–S4 run the container `cargo fmt` step (step 5 in Execution order) BEFORE the verification run — it normalizes any wrapping difference, so a `fmt-check` failure can never be a trap.

---

## S1 — Drop the redundant ellipsis pre-check in `fit_or_truncate_last`

File: `src/window_title.rs` (WidthFitter impl). Also deletes the one-call-site helper `ellipsis_does_not_fit` (an implementer addition — the pinned §4.1 WidthFitter shape lists ONLY `fits` + `fit_or_truncate_last`; restoring that shape is spec-aligned).

Why: both the pre-check's branch (line 84) and the loop-exhaustion fallback (line 92) return the SAME value, `TITLE_ELLIPSIS`. The pre-check only short-circuits iterations that provably end in the same fallback (every candidate starts with the ellipsis, so none is narrower under the char-count stub and egui text measurement). One fallback, one source of the minimum-output guarantee.

Before (exact):

```rust
    fn fit_or_truncate_last(&mut self, text: &str) -> String {
        if self.fits(text) {
            return text.to_owned();
        }
        if self.ellipsis_does_not_fit() {
            return TITLE_ELLIPSIS.to_owned();
        }
        for (start_index, _) in text.char_indices() {
            let candidate = format!("{TITLE_ELLIPSIS}{}", &text[start_index..]);
            if self.fits(&candidate) {
                return candidate;
            }
        }
        TITLE_ELLIPSIS.to_owned()
    }

    fn ellipsis_does_not_fit(&mut self) -> bool {
        (self.measure)(TITLE_ELLIPSIS) > self.max_width
    }
```

After (exact — the helper method is deleted entirely with its preceding blank line):

```rust
    fn fit_or_truncate_last(&mut self, text: &str) -> String {
        if self.fits(text) {
            return text.to_owned();
        }
        for (start_index, _) in text.char_indices() {
            let candidate = format!("{TITLE_ELLIPSIS}{}", &text[start_index..]);
            if self.fits(&candidate) {
                return candidate;
            }
        }
        TITLE_ELLIPSIS.to_owned()
    }
```

Behavior-preservation argument (D6 wording): text fits → returned complete (unchanged); nothing fits → bare `TITLE_ELLIPSIS`, never empty, never a panic (unchanged); otherwise → `...` + longest fitting tail (unchanged). Pinned test 5 (budget 2 chars: ellipsis 3 chars never fits, every candidate never fits → `...`) and tests 3/4/12 walk identically. Empty-name edge: `file_name` `...` fallback comes from `path_segments`, not this fitter; `fits("")` short-circuits at the first branch (unchanged).

Risk: none (pure local control-flow fold; single caller visible in-file plus tests via `compose_window_title`/`file_name_with_extension`).

Verification: dev-checks command below; expect exit 0, 213 passed / 0 failed, 29 files, no new fmt/clippy findings.

## S2 — Compose: drop the `..` destructuring and build `ChainComposer` only on the maximized path

File: `src/window_title.rs`, `compose_window_title`.

Why: (a) the struct destructure lists ALL fields yet adds a redundant `..` — that pattern suppresses the compile error if a field is ever added (exhaustiveness is the point of a param-object); (b) the `!is_maximized` early return currently runs AFTER `ChainComposer::new` and reaches THROUGH the composer for the fitter (`composer.fitter.fit_or_truncate_last`) — constructing the composer (incl. its load-bearing `base` clone) is wasted work on the name-only path. Moving construction below the early return removes the reach-through on that path. This does NOT reorder the pinned step checks: the non-maximized check staying before the step-0 full-chain check is REQUIRED by D7's "Step order (maximized = true only)" heading and pinned tests 2/3/5 (already recorded as the §4.1-sketch supersession in the review-fix observations; do not change it).

Before (exact):

```rust
    let TitleComposition {
        available_width,
        is_maximized,
        measure,
        ..
    } = request;
    let segments = path_segments(path);
    let mut fitter = WidthFitter {
        max_width: available_width,
        measure,
    };
    let mut composer = ChainComposer::new(&segments, &mut fitter);
    if !is_maximized {
        return composer.fitter.fit_or_truncate_last(&segments.file_name);
    }
    let full_chain = composer.full_chain();
```

After (exact; everything from `let full_chain` downward stays as in HEAD):

```rust
    let segments = path_segments(path);
    let mut fitter = WidthFitter {
        max_width: request.available_width,
        measure: request.measure,
    };
    if !request.is_maximized {
        return fitter.fit_or_truncate_last(&segments.file_name);
    }
    let mut composer = ChainComposer::new(&segments, &mut fitter);
    let full_chain = composer.full_chain();
```

(`let Some(path) = request.path else { ... };` above stays first — `Option<&Path>` is `Copy`, so the later partial moves of `request` remain legal; `request.measure` is moved into `fitter`, `request.available_width`/`request.is_maximized` are `Copy`.)

Risk: none (identical call sequence per branch; maximized path still constructs the composer with `base` = full name BEFORE the step-0 `full_chain` check, which is what makes step 0 verbatim-per-D7).

Verification: dev-checks command below; expect the same green census. Tests 6–15 pin the maximized path; 1–5 pin the name-only path.

## S3 — Fold the popup expire + wake pair into one expiry pass

File: `src/app/window_title.rs`.

Why: `update_window_title_and_popups` calls `expire_fullscreen_name_popup_if_due(now)` and then `request_fullscreen_name_popup_wake(screen_context, now)` in that order every frame. Expire clears any deadline with `now >= deadline`, so at wake time a surviving deadline ALWAYS implies `remaining_seconds > 0` — the wake's `remaining_seconds <= 0.0` guard is unreachable dead code (a copy-paste artifact of the `esc_close_confirm` precedent, where wake runs BEFORE expire and the guard is live). The pair also reads the same `expires_at` field twice. One method, one read, no dead branch. Neither method name is pinned by the plan (the pinned §4.3 list covers only the other four members).

Before — call site (exact):

```rust
        self.expire_fullscreen_name_popup_if_due(now);
        self.request_fullscreen_name_popup_wake(screen_context, now);
```

After — call site (exact):

```rust
        self.refresh_fullscreen_name_popup_expiry(screen_context, now);
```

Before — the two methods (exact; they sit adjacent between `arm_fullscreen_name_popup` and the impl close brace):

```rust
    fn expire_fullscreen_name_popup_if_due(&mut self, now: f64) {
        let Some(deadline) = self.window_title_state.fullscreen_name_popup_expires_at else {
            return;
        };
        if now >= deadline {
            self.window_title_state.fullscreen_name_popup_expires_at = None;
        }
    }

    fn request_fullscreen_name_popup_wake(&self, screen_context: &egui::Context, now: f64) {
        let Some(deadline) = self.window_title_state.fullscreen_name_popup_expires_at else {
            return;
        };
        let remaining_seconds = deadline - now;
        if remaining_seconds <= 0.0 {
            return;
        }
        screen_context.request_repaint_after(Duration::from_secs_f64(
            remaining_seconds + FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS,
        ));
    }
```

After — replacing BOTH methods with one (exact; if the single-line signature exceeds rustfmt's taste the step-5 `cargo fmt` normalizes it):

```rust
    fn refresh_fullscreen_name_popup_expiry(&mut self, screen_context: &egui::Context, now: f64) {
        let Some(deadline) = self.window_title_state.fullscreen_name_popup_expires_at else {
            return;
        };
        if now >= deadline {
            self.window_title_state.fullscreen_name_popup_expires_at = None;
            return;
        }
        screen_context.request_repaint_after(Duration::from_secs_f64(
            deadline - now + FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS,
        ));
    }
```

Behavior-preservation argument (D10 per pass): unarmed → no clear, no wake (unchanged); armed and `now >= deadline` → cleared, no wake requested (unchanged — the old wake saw `None` after the old expire); armed and live → wake at `deadline - now + margin` re-requested every pass (unchanged value). The arm-time wake inside `arm_fullscreen_name_popup` is NOT touched. Single-section `if`s, depth 1, 2 args + receiver.

Risk: low. The only semantic surface is the removed guard, proven unreachable above; no pinned name, no test covers the wiring (D15 pin — wiring is manual-verified; the OPERATOR-RUN gate still applies).

Verification: dev-checks command below; expect the same green census (this file has no tests, so "213 passed, 29 files" is the unchanged proof).

## S4 — Build the UNC drive segment from `UNC_PATH_MARKER`

File: `src/window_title/segments.rs`, `extract_drive_segment`.

Why: the `\\` lead is spelled inline by doubling `PATH_SEGMENT_SEPARATOR` in the format string, while the very same marker already has a named constant (`UNC_PATH_MARKER`) used in its detection (`display_path.starts_with(UNC_PATH_MARKER)` at `path_segments`). One concept, one source; shorter, self-describing line.

Before (exact):

```rust
        return Some(format!(
            "{PATH_SEGMENT_SEPARATOR}{PATH_SEGMENT_SEPARATOR}{server}{PATH_SEGMENT_SEPARATOR}{share}"
        ));
```

After (exact — same 3-line shape, so rustfmt-neutral):

```rust
        return Some(format!(
            "{UNC_PATH_MARKER}{server}{PATH_SEGMENT_SEPARATOR}{share}"
        ));
```

Risk: none — byte-identical output (`\` + `\` + server + `\` + share); pinned test 15 asserts the composed UNC title.

Verification: dev-checks command below; expect the same green census.

---

## Execution order (implementer, junior track — one step at a time)

1. Apply S1 (exact after-block above) in `src/window_title.rs`.
2. Apply S2 (exact after-block above) in `src/window_title.rs`.
3. Apply S3 (two exact edits) in `src/app/window_title.rs`.
4. Apply S4 (exact after-block) in `src/window_title/segments.rs`.
5. Format pass (container, single command):
   `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt`
6. Verification (container, single command — alpine-vm MCP, `vm_status` first; exit code is the authoritative evidence):
   `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`
   Expected: exit `0`, final `ALL CHECKS PASSED`, `213 passed; 0 failed`, 29 test-bearing files, no new fmt/clippy findings beyond the pre-existing benign `compare_entry_names` dead-code warning. ANY deviation → STOP and ask the caller; never ship a red run.
7. Commit (after the review-fix Issue-1 commit, if it runs first — implementation plan §5 item 4 governs fix commits):
   `refactor: simplify wave8 task4 window-title modules` — staging ONLY the four files above (`src/window_title.rs`, `src/app/window_title.rs`, `src/window_title/segments.rs`; plus the review-fix file if bundled by the caller decision — the caller decides bundling, default: separate commits). Before committing run `git status` and verify no `.gitignore`-matched paths (`logs/` contains the dev-checks log — never stage it).
8. Line-cap re-check after the edits (informational, should pass by construction): `src/window_title.rs` ≈175 (from 188), `src/app/window_title.rs` ≈171 (from 177; ≈174 if the review-fix +3 lines landed first) — all well under the 200 cap; no structure-map line-count text exists to update, so `.agent/project-structure.md` stays UNTOUCHED by this plan.

## Reviewed and deliberately KEPT (no action — do not "helpfully" change these)

- `ChainComposer` parent-walk trio (`should_add_next_parent` → `parent_walk_is_bounded` + `next_parent_fits`): `next_parent_fits` is a D16-pinned name; the bound check is REQUIRED to terminate the while-loop (without it, `candidate_with`'s clamp makes a saturated candidate fit forever → infinite walk). Merging would break the pinned name or the loop invariant.
- `candidate_with`'s `saturating_sub` + `.min(folders.len())` double defense: the clamp is what makes step 2's `candidate_with(1, false)` safe on a folder-less path; keep per the consider-edge-cases guideline.
- The three `screen_context.input(...)` reads in `update_window_title_and_popups` (maximized / outer_rect / time): matches D2's pinned two-read shape and the single-clock precedent; consolidating into a tuple closure is stylistic churn (already logged by the review-fix observations).
- Popup visual constants duplicated from `esc_close_confirm.rs`: D11-mandated ("do NOT re-export or widen its visibility") — deliberate spec duplication, not dead logic.
- `title_recomposition_is_due` / `fullscreen_name_popup_rearm_is_due` / the `TitleComposition` param-object / the `WidthFitter` and `ChainComposer` split: all D16/§4.1-pinned shapes.
- `show_navigation_layout` in `src/app/central_surface.rs` + its single call site (`src/app.rs` show_chrome_layout): D14/§4.5 verbatim relocation, consumed coherently; `src/app.rs` has no dead residue (no orphaned imports — `esc_close_confirm` is still used at the chrome-layout popup line; `floating_controls`/panel calls were never imported there).
- `composer.base = ...` reassignment after step 0 and `ChainComposer::new`'s `base: segments.file_name.clone()`: the clone is load-bearing (step 0's `full_chain` must carry the COMPLETE name per D7), not a wasted copy.
- `parts.extend(self.segments.drive.clone())` on the guaranteed-`Some` option: readable, one line, spec-sketch territory; ref-vector rewrite would trade clones for lifetime noise — not worth it.
- `mod segments;` declared at the BOTTOM of `src/window_title.rs`: matches the existing in-crate convention in this file family (child-decl after content); moving it is pure style.
- Rule audit results, all PASS: no commented-out code; no unused imports/fields (every `WindowTitleState` field and every pinned constant has a live consumer incl. tests); all method bodies ≤50 lines; nesting ≤2; all ≤2 args; every `if`/`while` condition single-section; file caps ≤200 (188/177/197/70/138/52/65/51); `.agent/project-structure.md` matches the on-disk layout.

## Caller notes (NOT planned here)

1. Census arithmetic: the implementation plan §7 pins "212" but 195 + 18 = 213 — the repo truth is 213 tests / 29 files. No code change possible; the 4.5a adherence report must record the slip. (Already observed in the review-fix file, note 2.)
2. §4.1 pseudocode vs D7/tests: the pinned-body sketch puts the step-0 full-chain check BEFORE the not-maximized return, which would break pinned tests 2/3/5; the implementation correctly follows D7's heading ("maximized = true only"). 4.5a must record this supersession explicitly.
3. `fonts_mut` substitution (D1 sanctioned fallback) is in use in `measure_text_width`; 4.5a must record the substitution (review-fix observation 1 already evidences it).
4. S3 removes a guard whose only protection was call-order; if a future task ever calls the wake pass BEFORE the expiry pass, re-introduce the remaining-clause (the `esc_close_confirm` ordering shows why). Not actionable now.
