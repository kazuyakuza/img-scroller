# Global Plan — Wave-2 UI Fixes: Centered Floating Controls + Empty-State Center Open

Date: 2026-11-01
Source TODO: [`.agent/todos/20261101/20261101-todo-1.md`](../../.agent/todos/20261101/20261101-todo-1.md) (operator-run feedback; Pattern C, 5 task sections)
Status: DRAFT — awaiting user approval (workflow step 1.3)

## 1. Task Origin

- User report on the Wave-2 exe (chat, 2026-11-01, screenshot evidence): (1) arrow-button icons invisible (tofu boxes — default egui font lacks ←→↑↓); (2) toolbars/rails rejected → centered bottom floating buttons; (3) Open visible ONLY in the empty state, big and centered.
- Confirmed working: open/decode/fit display. The user's directives ARE the specification (no proposal gate needed; any NEW layout choice beyond them → ask user before implementing).
- Wave binding updates to keep: 17-extension filter, collation, edge-stay-silently, open-flow failure semantics, literal-K3 message visibility (only-while-image-open), pan-then-navigate → Wave 5 (arrows direct).

## 2. Global Pre-Analysis

### 2.1 Session-bound facts

- Git: `main` == `origin/main` == `de332fc` (Wave 2 closed, pushed); tree clean. Branch creation restricted to Step 2: `fix/centered-floating-controls`.
- Alpine VM running/SSH-reachable (verified this session); container harness rules unchanged (`vm_status` first; exit code authoritative; `--locked`; docs/build.md).
- Code state (Wave 2 landed, 26 tests): `src/app.rs` (163 L; key intake, `navigate`, `neighbor_image_path`, `image_is_open`, `accept_open_outcome` rebuild), `src/app/rails.rs` (74 L — becomes obsolete), `src/app/tests.rs` (187 L), `src/app/rails_tests.rs` (40 L), `src/folder_listing.rs` + tests, `image_loading/scaling/app_display/platform` untouched by this TODO (expected).
- Version: `0.2.0` → Step 3 bumps patch → `0.2.1` (operator-run defect fix).

### 2.2 Task grouping (workflow-authorized join)

| TODO task | Cycle | Front-end? |
| --- | --- | --- |
| Tasks 1–3 (icon mechanism + floating controls + center Open — one atomic UI restyle) | Cycle A: 4.1a → 4.1b → 4.2 → 4.3(+fix) → 4.4 → 4.5a → 4.5b → 4.6 | YES (4.1a + 4.5a run) |
| Task 4 (verification + exe refresh) | Cycle B (no 4.1a; 4.5b only) | NO |
| Task 5 (documentation) | Cycle C (no 4.1a; 4.5b only) | NO |

Grouping rationale: Tasks 1–3 are one inseparable code change (icons exist only in the replacing controls; intermediate states would not compile/present sensibly).

### 2.3 Technical pre-analysis (Cycle A decision inventory for 4.1a/4.1b — architector decides with context7-verified egui 0.37 API facts)

1. **Icon mechanism** (TODO Task 1): candidates — (a) egui-painted vector triangles via `egui::Painter`/`Shape` inside `ui.allocate_response`/`Button::raw`/custom widget (NO font dependency; hiring constraint: click sense + hover/disabled visual states); (b) ASCII labels "<" ">" "^" "v" (zero risk, less polished); (c) bundled font file (new binary asset — heavier, redistribution concern unused so far; note app must stay portable). Architector PICKS ONE decisively; encode exact draw/signature detail.
2. **Floating overlay** (TODO Task 2): replace `src/app/rails.rs` with a new module (name exactly, e.g. `src/app/floating_controls.rs`): `egui::Area::new(...).anchor(egui::Align2::CENTER_BOTTOM, offset)` + frame/pivot, no reserving panel; contains painted ←/→ buttons wired to `app.navigate(Previous/Next)` + ↑/↓ disabled-in-Wave-3 buttons; enablement still derived from `neighbor_image_path(...).is_some()` (STAY parity); message label placement while image open (decide: inside the pill's row or a small floating line above it); `image_is_open()` gate stays.
3. **Empty-state center Open** (TODO Task 3): CentralPanel empty-state child (or Area anchored CENTER): one BIG button calling `open_via_dialog(frame)`; no top toolbar (`open_toolbar` panel removed; `open_toolbar_button` method retired); with an image open, NO Open control anywhere.
4. **State semantics unchanged**: `navigation`/listing/message logic untouched by design; only UI surface changes. Tests: `rails_tests` test semantics probably unchanged (enablement helper survives); `app/tests.rs` untouched unless the empty-state change alters testable state paths (no new state expected); file-line arithmetic re-derived (≤200 each).
5. **Obsolete-code removal rule**: rails.rs deleted (no commented-out code); git history retains it.
6. **Windows-visible icon guarantee**: the chosen mechanism must not depend on system font availability; 4.5a + operator re-test verify visually.

### 2.4 Binding constraints (all sub-agent prompts)

Identical to `.kilo/plans/20261031-wave2-horizontal-scroll.md` §2.2 (container-only toolchain via alpine-vm MCP; `--locked`; branch/push/version step restrictions; no `CHANGELOG.md`; no edits to existing `.kilo/plans/*` (new files only); code rules (≤200 lines/file, ≤50-line methods, ≤2 nesting, ≤2 params, private-by-default, single-section booleans, no commented code); docs edited only by Planner/docs-specialist; TODO marks only `[DONE]`/`[x]` preserving content; gitignore compliance each commit).

### 2.5 Deliverable naming

- Plans: `.kilo/plans/20261101-floating-controls-<slug>.md` (4.1b outputs), frontend spec: `20261101-floating-controls-frontend-spec.md`, verification report: `...-frontend-verification.md`, review-fix: `...-review-fix.md`, adherence: `...-adherence.md` (new files only).

## 3. Steps Checklist

### Step 2 — Git Feature Branch Setup → implementer
- On `main` (verify) → create + switch `fix/centered-floating-controls`. Branch ops ONLY here.

### Step 3 — Version Update → implementer
- `Cargo.toml`/`Cargo.lock` `0.2.0` → `0.2.1`; commit `chore: bump version to 0.2.1`.

### Cycle A — TODO Tasks 1–3 grouped (front-end related)

- **4.1a** → frontend-specialist: front-end spec encoding the USER-DIRECTED layout (no proposal menu): icon mechanism comparison feed-in, floating pill anatomy (container/type, anchor, spacing, sizes, disabled states incl. STAY-derived enablement, message placement implication), empty-state center Open anatomy, out-of-scope list (menu placeholder → later waves; Wave-5 pan; Wave-3 up/down wiring), 4.5a conformance checklist. Save to `.kilo/plans/20261101-floating-controls-frontend-spec.md`.
- **4.1b** → architector: full implementation plan (junior-level, zero open decisions): module shape (`src/app/floating_controls.rs` replacing `rails.rs`), exact egui 0.37 API usage (context7-verified), app.rs edits (toolbar removal, empty-state branch in `ui()`, `open_via_dialog` reuse), tests impact analysis + any test moves (≤200-line files), git commit plan. Save to `.kilo/plans/20261101-floating-controls.md`.
- **4.2** → implementer: implement per plan; one feature commit; no cargo (Task-4 runs checks; report compile-hazard notes).
- **4.3** → code-reviewer + code-simplifier concurrent → fix plan if required → 4.3-fix implementer (container verification like Wave-2 practice: fmt/check/test/clippy).
- **4.4** → docs-specialist: doc-comment conformance only (no user-facing docs — Cycle C owns those).
- **4.5a** → frontend-specialist: verify implementation vs the 4.1a spec → report file.
- **4.5b** → architector: adherence (incl. 4.5a report); READY/NOT READY.
- **4.6** → implementer: `[DONE]` marks for TODO Tasks 1–3 (+ `[x]` sub-items); include the 4.5a report in the commit.

### Cycle B — TODO Task 4 (verification)

- **4.1b** → architector: verification plan (test-census expectations; harness run; exe rebuild; operator re-test handoff text). Save to `.kilo/plans/20261101-floating-controls-verification.md`.
- **4.2** → implementer: run container dev-checks (`ALL CHECKS PASSED` expected; 26 tests unless legitimately changed) + rebuild exe via `scripts/build-windows.sh` + gitignore sweep.
- **4.3** → code-reviewer (+ code-simplifier if warranted) → fix cycle only if a genuine defect.
- **4.5b** → architector adherence; **4.6** → implementer `[DONE]` marks (Task 4's OPERATOR-RUN sub-item stays `[ ]` until user reports).

### Cycle C — TODO Task 5 (documentation)

- **4.1b** → architector: docs-change plan (README features wording: floating controls + center Open; tech.md/architecture.md Viewer-UI + module wording rails→floating; message-surface wording; project-structure.md map: rails.rs → floating_controls.rs; context.md closing update incl. operator report + re-test pending). Save to `.kilo/plans/20261101-floating-controls-docs.md`.
- **4.2** → docs-specialist applies; **4.3** → code-reviewer (docs); fix cycle if needed; **4.5b** → architector; **4.6** → implementer `[DONE]` + commit.

### Step 5 — TODO File Completion → implementer
- Rename `20261101-todo-1.md` → `20261101-todo-1-DONE.md` (content unchanged); tmp cleanup; ensure committed; merge `fix/centered-floating-controls` to `main` (fast-forward expected; delete branch after verified); push `main` to `origin` ONLY.

### Step 6 — Finish (Planner)
- Summary + OPERATOR-RUN re-test handoff to user + next-session prompt for the next undone TODO (Wave 3: `.agent/todos/20261029/20261029-todo-5.md`); record merge/push hashes at closing in `context.md`.

## 4. Risk Notes

- Icon painter mechanics in egui 0.37 must be API-verified (context7) — no memory-trust (Wave-1 lesson).
- `rails.rs` deletion → module-registration edits in `src/app.rs` (`mod rails;` line retires); `rails_tests.rs` re-homes its enablement test to the new module's test file (or retiring if the helper shape changes — architector encodes exactly).
- Floating overlay must not intercept arrow-key intake (input intake stays frame-first; Area defaults w/o keyboard focus).
- Dock/anchored Area anti-overlap with the CentralPanel image: paint order (CentralPanel last keeps overlay above — verify; egui Areas render above panels by default).
