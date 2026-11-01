# Global Plan — Wave 2: Horizontal Scroll (Sibling Images)

Date: 2026-10-31
Source TODO: [`.agent/todos/20261029/20261029-todo-4.md`](../../.agent/todos/20261029/20261029-todo-4.md) (Pattern C: `# Title` → `## Tasks` → five `###` tasks)
Status: DRAFT — awaiting user approval (workflow step 1.3)

## 1. Task Origin

- User request: full read `AGENTS.md` & follow `/critical-workflow`; do the TODO file above.
- User note (session start): the Wave-1 exe OPERATOR-RUN check was tested by the user and **worked as expected** — this closes Wave 1's pending manual-UX item; it is recorded in `context.md` at this wave's closing update (Task 5).
- Workflow: `.kilo/commands/critical-workflow.md` — this file is ONLY the global plan; per-task plans are separate new files (never overwrite this one; never edit pre-existing `.kilo/plans/*`).

## 2. Global Pre-Analysis

### 2.1 Session-bound facts (verified 2026-10-31 at plan time)

- Git: on `main`, working tree clean, `main` == `origin/main` == `92dc22c`. No other branches.
- Alpine VM: `vmState: running`, SSH reachable (`127.0.0.1:3022`) — container harness available. Agent-side Rust/Cargo runs ONLY inside the container on the VM (`alpine-vm` MCP: call `vm_status` first; exit code authoritative; `docs/build.md` full semantics).
- Dev-checks command (per `docs/build.md`): `sh scripts/dev-checks.sh` via `docker compose -f /img-scroller/docker-compose.yml run --rm rust` → `ALL CHECKS PASSED` exit 0 expected.
- Wave-1 code shape (the navigation foundation): `ImagesScrollerApp` holds `opened_image_path` + `decoded_image` + `message_text` + `current_display`; the open-flow seam is `open_image(path)` (decode → state transition; failure shows message only). Wave 2 reuses this seam for every navigation step.
- `Cargo.toml` version exists: `0.1.0` → step 3 applies semver.

### 2.2 Binding constraints (baked into every sub-agent prompt)

1. All Rust/Cargo commands run inside the container on the Alpine VM via `alpine-vm` MCP (`vm_status` first; exit code authoritative). Host needs no Rust/Cargo; never global installs.
2. Every harness cargo invocation uses `--locked`; `Cargo.lock` stays tracked.
3. Version update restricted to step 3; branch create/switch restricted to step 2; `git push` restricted to step 5, `main` → `origin` ONLY.
4. Do NOT touch `CHANGELOG.md` in this wave (TODO binding).
5. Do NOT edit existing `.kilo/plans/*` files — new plan files only (TODO binding).
6. All source in `src/`; files ≤200 lines; methods ≤50 lines; max-2 nesting; max-2 params (param-object in a new file when more); private by default; single-section boolean conditions (extract named method when more); self-documenting names, minimal comments.
7. Docs/markdown edits: docs files are modified ONLY by Planner or docs-specialist (markdown-generation rule) — implemented in Task 5's step assignment (see 4.5). Plans/reports also Planner/architector/frontend-specialist per their 4.1/4.5 roles.
8. TODO file edits: only `[DONE]` marks + `[]`→`[x]` on sub-items, preserving original content (per critical-workflow 4.6/5).
9. Container tests must remain runnable on Linux (pure logic testable; OS-bound paths stay behind the `platform` facade pattern).
10. Rules: single-section boolean conditions, no commented-out code, gitignore compliance before every commit (no `dist/`, `logs/`, `target/` staged).

### 2.3 Deferred-decision inventory (TODO §"Deferred decisions" → owner step)

| Deferred decision | Owner (this wave) | Handling |
| --- | --- | --- |
| Image-file detection filter (extensions vs codec availability) | Task 1 → 4.1b architector | Decided in per-task plan; options analyzed there (extension whitelist container-testable; WIC probe Windows-only/untunnable in container). User approves per-task plan (or auto-ables per user's global approval choice). |
| Exact alphanumeric collation (case handling method) | Task 1 → 4.1b architector | Decided in per-task plan; v1 fixed, no settings UI. |
| Edge-of-list behavior (first/last: stay vs wrap) | Task 2 → 4.1b architector | Decided in per-task plan. |
| Pan-then-navigate interplay | Wave 5 (NOT this wave) | Task 2 records "arrows navigate directly" explicitly; no pan logic implemented. |
| UI layout (left/right/up/down buttons + menu area) | Task 3 → 4.1a frontend-specialist | MANDATORY propose-and-ask: concrete proposal presented to user via `question` tool BEFORE any layout code; applies even if the user auto-approves per-task plans (TODO Task 3 binding). |

### 2.4 Front-end related flags (recorded for 4.1a & 4.5a)

| Task | Front-end related? | 4.1a / 4.5a |
| --- | --- | --- |
| Task 1 (listing) | NO | skip both |
| Task 2 (keyboard nav) | YES (egui input → display interplay) | run 4.1a then 4.1b; 4.5a then 4.5b |
| Task 3 (UI buttons) | YES (UI controls + layout proposal gate) | run 4.1a then 4.1b; 4.5a then 4.5b |
| Task 4 (verification) | NO | skip both |
| Task 5 (documentation) | NO | skip both |

### 2.5 Deliverable naming conventions

- Per-task plans (4.1b): `.kilo/plans/20261031-wave2-task<N>-<slug>.md` (architector output).
- Front-end specs (4.1a): `.kilo/plans/20261031-wave2-task<N>-<slug>-frontend-spec.md`.
- Front-end verification reports (4.5a): `.kilo/plans/20261031-wave2-task<N>-<slug>-frontend-verification.md`.
- Review/fix plans (4.3): `.kilo/plans/20261031-wave2-task<N>-<slug>-fix.md` (or shared review-fix names used by Wave 1 when both reviewers align).
- Adherence reports (4.5b): architector returns report path or clear "no diffs" message; when a new plan is needed: `...-adherence.md`.

## 3. Steps Checklist (workflow steps 2–6; one `task` invocation per step)

### Step 2 — Git Feature Branch Setup → implementer

- `git status` (clean today) → commit any strays per gitignore rule if found.
- On `main` (verified) → create + switch to new branch `feat/wave-2-horizontal-scroll` (allowed name from TODO §"Future session execution notes"). Branch creation happens ONLY here.

### Step 3 — Version Update → implementer

- `Cargo.toml` `0.1.0` → `0.2.0` (minor: Wave 2 adds the horizontal-scroll feature; semver feature = minor). Commit: `chore: bump version to 0.2.0`.
- Note: `Cargo.lock` package version entry updates with the bump (`Cargo.lock` is tracked; commit along).

### Task 1 — Current-folder image listing (NOT front-end)

- Step 4.1b → architector: decide (a) the image-file detection filter (recommendation to analyze: fixed extension whitelist matching brief's WIC-system-codec examples vs WIC codec-availability probe — probe is Windows-only and container-untestable), (b) the alphanumeric collation (recommendation candidates: case-insensitive lexicographic with stable tie-break vs byte-order; v1 fixed, no settings UI), (c) module shape: a new pure-logic module in `src/` (e.g. `folder_listing.rs`) exposing a listing type built from the currently opened image's parent folder: non-recursive `std::fs::read_dir`, file-entries-only (no dirs), filter + alphanumeric sort, plus helpers to locate the current image's index and step prev/next; unit-testable in the container with temp dirs created under `std::env::temp_dir()` (never inside the repo). Plan file per 2.5. Present per-task plan to user (unless auto-approved per user's choice).
- Step 4.2 → implementer: implement per plan; complexity rules respected; commit meaningful.
- Step 4.3 → code-reviewer + code-simplifier (concurrent): fix/simplification plan if required → 4.3-fix implementer. Max 3 cycles.
- Step 4.4 → docs-specialist: minimal doc touches for the new module only (module-level doc comments inline; docs files only if a user-facing fact changed).
- Step 4.5b → architector: plan adherence report.
- Step 4.6 → implementer: mark Task 1 `[DONE]` + `[x]` sub-items in TODO; commit.

### Task 2 — Previous/next image navigation — keyboard (FRONT-END related)

- Step 4.1a → frontend-specialist: front-end spec (egui input intake for Left/Right arrows bound to the central panel/window input state; arrows must navigate DIRECTLY — no pan interplay; spec states Wave 5 owns pan-then-navigate). Spec file per 2.5.
- Step 4.1b → architector: decide edge-of-list behavior (stay vs wrap; candidates analyzed: stay (classic viewer behavior: begin/end silently) vs wrap (string-adjacent navigation never stops scanning)); decide failure handling when the ±1 neighbor fails to decode (recommendation: show message, keep current image unchanged — consistent with open-flow rule); decide state shape for the in-session listing cache (rebuild triggers: successful open from dialog/CLI, navigation; invalidation when folder changes; RAM-only). Incorporate 4.1a spec. Plan file per 2.5; present per-task plan to user (unless auto-approved).
- Step 4.2 → implementer: implement per plan (input wiring in `app.rs` using the seam `open_image`; listing consumed from Task 1's module); commit.
- Steps 4.3 / 4.3-fix as in Task 1.
- Step 4.4 → docs-specialist: doc comments only unless user-facing change.
- Step 4.5a → frontend-specialist: verify implementation against the 4.1a spec → report file.
- Step 4.5b → architector: adherence using the 4.5a report.
- Step 4.6 → implementer: `[DONE]` mark + commit.

### Task 3 — Previous/next image navigation — on-screen UI buttons (FRONT-END related)

- Step 4.1a → frontend-specialist: front-end spec + the MANDATORY UI-layout PROPOSAL: concrete placement/design for left/right (and, for future-wave reuse, up/down) navigation buttons and the menu area (Wave 1 has top toolbar + bottom message bar; propose a consistent concrete layout). NO layout is invented silently.
- [GATE — Planner/user] Planner presents the proposed UI layout via `question` tool BEFORE 4.1b finalizes layout code. This gate applies regardless of the user's global auto-approval choice (TODO Task 3 binding). Options presented = the concrete proposals from the spec (recommended first); user may also type their own.
- Step 4.1b → architector: final per-task plan with the USER-APPROVED layout encoded (no judgment left); button clicks reuse the same navigation path as Task 2 (one shared handler); plan file per 2.5.
- Step 4.2 → implementer: implement ONLY the approved layout + button wiring; commit.
- Steps 4.3 / 4.3-fix, 4.5a (spec vs implementation), 4.6 as in Task 2.

### Task 4 — Verification (NOT front-end)

- Step 4.1b → architector: verification plan — unit tests ∅ (listing enumeration incl. folder-only scope incl. nested-subfolder exclusion, alphanumeric sort correctness incl. collation case-handling cases, prev/next index stepping, first/last edge behavior per Task 2's decision), all container-runnable (`cargo test --locked`); dev-checks harness plan; exe refresh plan (OPERATOR-RUN below needs a current exe). Plan file per 2.5.
- Step 4.2 → implementer: add the plan's tests where they belong (module tests + app-level tests); then run via `alpine-vm` MCP: `vm_status` first → `sh scripts/dev-checks.sh` in the container → authoritative `ALL CHECKS PASSED` exit 0; then refresh the Windows exe: `sh scripts/build-windows.sh` in the container → non-empty exit-0 exe (dist/ gitignored — never staged). Commit tests. If the VM or `/img-scroller` share is unavailable → STOP and return a user question.
- Step 4.3 → code-reviewer + code-simplifier: test-quality review; 4.3-fix if required.
- Step 4.4 → docs-specialist: only if test guidance must appear in `docs/build.md` (avoid duplication otherwise).
- Step 4.5b → architector: adherence report.
- Step 4.6 → implementer: mark agent-verifiable sub-items `[x]`; the OPERATOR-RUN manual-UX sub-item stays unchecked until the user reports (Wave-1 precedent: TODO can close with the operator item recorded pending in `context.md`); commit.
- [USER] At close, Planner asks the user to OPERATOR-RUN `dist\windows-x64\img-scroller.exe`: keys/buttons move prev/next; nested subfolders NOT entered; order matches the decided collation; results recorded into `context.md`.

### Task 5 — Documentation updates (NOT front-end)

- Step 4.1b → architector: docs-change plan — exactly which sections/files change: `README.md` (features: horizontal scroll behavior), `tech.md` (navigation-engine decisions landed: filter + collation + edge behavior + listing state shape), `architecture.md` (Navigation Engine component: planned → landed slice; module layout gains new modules), `context.md` (closing update incl. the user's Wave-1 exe confirmation from §1 and Wave-2 state). `CHANGELOG.md` NOT touched (binding 4).
- Step 4.2 → **docs-specialist** (NOT implementer — markdown-generation rule: docs files are edited only by Planner/docs-specialist; implementer only commits afterward). Applies the plan's exact doc changes; docs-split rule respected (no duplication between tech.md/build docs); new/updated content with TOC/index preserved where files exceed 100 lines.
- Step 4.3 → code-reviewer alone enough (docs change review: plan adherence); code-simplifier not applicable to markdown — returns clear "not required" (workflow permits).
- Step 4.5b → architector adherence; 4.6 → implementer `[DONE]` + commit.
- Planner closing duty (after step 6): final `context.md` update per project-info instructions.

### Step 5 — TODO File Completion → implementer

- Rename `.agent/todos/20261029/20261029-todo-4.md` → `20261029-todo-4-DONE.md` (content unchanged at rename; commits the rename).
- Review and remove any tmp files/folders created during the process (none expected; `dist/`+`logs/` are gitignored build outputs, stay).
- Ensure all changes committed on the feature branch; gitignore compliance check.
- Merge to `main`: switch to `main` → merge `feat/wave-2-horizontal-scroll` → on success delete the feature branch (verify first) → on failure notify user.
- Push `main` to `origin` ONLY. Never push to other remotes. Notify user on push failure.

### Step 6 — Finish (Planner)

- Short summary of realized work.
- Provide the next-session prompt text (next undone TODO = `.agent/todos/20261029/20261029-todo-5.md`).
- Record context.md closing facts (Planner): Wave-2 merged hash, test count, decisions landed, user's Wave-1 exe confirmation, OPERATOR-RUN status for Wave 2.

## 4. Risk Notes

- vboxsf EBUSY on container manifest writes (known Wave-1 pattern): if `cargo check/test` commands mutate `Cargo.lock`/image manifests inside the container, use the authorized host-side write fallback documented in Wave-1 plan files; `--locked` minimizes occurrences.
- `egui::Panel` 0.37 API facts (Wave-1 learning): frontend-specialist/architector must re-verify exact egui 0.37 API names against the landed code before writing specs (context7 docs, not memory).
- Test temp-dirs policy: tests create temp fixtures under `std::env::temp_dir()` (container `/tmp`), never inside the repo; clean up fixture dirs after each test.
- Max-2-params rule impacts the shared navigation handler signature — architector encodes the param-object choice in per-task plans.
