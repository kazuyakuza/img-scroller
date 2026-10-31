# Global Plan — frontend-specialist Definition Alignment (egui/eframe)

- Date: 2026-10-30
- TODO (source): [`.agent/todos/20261030/20261030-todo-2.md`](../../.agent/todos/20261030/20261030-todo-2.md) — Pattern B: the single `##` section below the title is ONE task.
- Origin: user directive during Wave-1 global-plan approval ("Modify the frontend-specialist definition so it is aligned for current project front-end tech. Start with this, and request my confirmation of the changes.").
- Role in the overall session: prerequisite for marking Wave-1 GUI tasks (2–4 of [`.agent/todos/20261029/20261029-todo-3.md`](../../.agent/todos/20261029/20261029-todo-3.md)) as front-end related (4.1a spec + 4.5a verification with this agent); Wave-1 per-task plans auto-approved, per-task global plan approval already granted.

## Task: Align the frontend-specialist agent definition with egui/eframe

Single task; one 4.x cycle. Branch handling: change is committed on `main` BEFORE Wave-1 Step 2 branch creation (user-confirmable; see approval question) so the new definition is durable and effective in any session after the user restarts opencode.

### Files to modify (BOTH copies kept in sync; identical body edits)

1. `C:\repo\img-scroller\.opencode\agents\frontend-specialist.md` (opencode integration)
2. `C:\repo\img-scroller\.kilo\agents\frontend-specialist.md` (Kilo Code integration)

Do NOT touch frontmatter permission blocks beyond background-neutral cleanup — both permission blocks are environment-shaped already (`mcp: allow` exists only in the Kilo copy; leave that asymmetry intact).

### Body edits (exact, junior-under-50% guided)

1. Opening identity line:
   - OLD: "You are a frontend developer expert in Angular, VueJS, TypeScript, modern CSS (vanilla and related libs/frameworks). You handle frontend development tasks."
   - NEW: "You are a desktop UI expert in egui/eframe immediate-mode GUI development in Rust (window shell, panel/widget structure, image texture rendering, keyboard input handling, fonts/theming). You handle desktop front-end tasks."
2. `## Role` section:
   - OLD: "Build responsive user interfaces, manage state, integrate with APIs, and optimize performance."
   - NEW: "Specify and verify the desktop GUI surface: window shell options, panel/widget structure and state contracts, image texture pipeline, keyboard interaction handling, and theming — for the project's egui/eframe stack."
3. `### 2. Front-end Analysis (for 4.1a)` — replace the nine web bullets with egui/eframe equivalents:
   - Window shell: eframe/egui window options (viewport title `Images Scroller`, initial size range, `resizable(true)`; NEVER maximized; not a modal).
   - App state structure: `eframe::App` implementation struct — fields for image texture handle, source path/context, unsupported-file message state; ownership/lifetime of decoded image data.
   - Component structure: `CentralPanel` (image surface) vs `TopBottomPanel`/`SidePanel` (UI controls) boundaries, hierarchy, and reuse.
   - Contracts: widget/model state (Store/State fields), display construction/teardown, and effect/event flow for keyboard input (`InputState` arrow keys) — desktop equivalents of props/events.
   - Image texture pipeline: decoded pixels → `egui::ColorImage` → `TextureHandle`/`Image` source; fit-scaling integration point (`Image` sizing; zoom modes arrive Wave 4 — not in scope).
   - Theming/tokens: egui `Visuals`/`Style` customization choices and naming conventions (desktop analog of the CSS/styling bullet).
   - Responsive behavior: window-resize sanity (keep display sane on resize; DPI/scale factor handling) — replaces responsive breakpoints.
   - Accessibility: keyboard-only navigation defaults, clear/focused input state, high-contrast-friendly default visuals (desktop analog of web a11y).
   - Performance: texture re-upload discipline (only on image change, not per-frame), avoiding per-frame allocations, frame-time budget.
   - Keep the closing instruction that dispatch is unchanged (no web routing analog required for this project).
4. `### 3. Produce Front-end Technical Specification (for 4.1a)`:
   - OLD bullets: component boundaries/contracts, design tokens & styling, API integration contract, a11y/responsive/perf acceptance criteria.
   - NEW bullets: window-shell/state boundaries and contracts; texture pipeline decisions; keyboard interaction contract; theming decisions; acceptance criteria (resizable sanity, title, not-maximized, correct fit display; keyboard nav; texture upload discipline).
5. `### 4. Front-end Verification (for 4.5a)`:
   - OLD bullets: confirm structure/contracts, CSS/styling, responsive, a11y, state/API; run npm/npx build/test/lint/typecheck.
   - NEW bullets: confirm window options and app-state structure match spec; confirm texture pipeline and fit-scaling match; confirm keyboard handling matches; confirm theming matches; confirm no per-frame texture re-upload/allocation churn.
   - Replace the npm/npx run-command bullet with: "This agent does NOT run cargo/docker commands (not in its allowed tooling). Run-level build/test/check evidence comes from the implementer/architector (4.5b) via the Planner; your verification is static code review against the spec, using READ-only tooling (read/grep/glob/cat/git-read) per permission block."
   - Keep the closing "Report diffs ... (4.5b)" paragraph verbatim.
6. Keep verbatim: frontmatter keys/permission shapes, `## Tools Preference`, `## Workflow Integration`, `## Context Loading`, `### 1. Intake`, `## Target Implementer: JUNIOR Developer (50% Restriction)` section, `## Boundaries` section. Do NOT add entries about other frameworks; do NOT generalize beyond the current stack (egui/eframe named explicitly).
7. Self-check: files ≤ 200 effective-content lines is NOT enforced for agent/config md (Max-Lines rule excludes config/docs); keep real newlines (Newline Prevention rule); no commented-out code blocks (No-Commented-Code rule — replace, never re-comment).

### Execution steps (sub-agent instructions)

- STEP-A (implementer): apply ALL body edits above to BOTH files with the structured `edit` tool (no shell heredocs); preserve real newlines.
- STEP-B (implementer): show `git diff --` for the two files in the summary (read-only evidence); do NOT commit (commit decision is user-approved in the confirmation question; possible values recorded in the question options).
- STEP-C (Planner): present actual diff summary to the user for confirmation; only then commit per the chosen option (commit restricted to a single follow-up implementer task), then rename this TODO with `-DONE` suffix; afterwards continue Wave-1 execution order (global plan steps).

### Constraints

- The definition takes effect only after the user restarts opencode (config is loaded at startup); remind the user at the completion signal.
- `.agent/WORKFLOWS.md`-listed workflows respected: agent definitions are opencode/Kilo integration files, NOT docs — markdown-generation rule attribution does not forbid implementer edits to existing non-plan agent md files when the Planner plans them.
- Git safety: commit only with user-confirmed option; push stays exclusive to Wave-1 Step 5 (this mini task pushes nothing).
