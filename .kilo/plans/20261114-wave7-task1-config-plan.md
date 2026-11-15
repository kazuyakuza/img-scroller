# Wave 7 — Task 1 Implementation Plan: Application Config File (existence + format decision)

- Source TODO: `.agent/todos/20261029/20261029-todo-9.md` → `### Task 1: Config file (existence + format decision)`. Verbatim: "Introduce the application config file shipped alongside the binary; its contents/format are decided in this wave's architector step (contents must remain minimal and consistent with architecture.md's 'On disk' note). No last-viewed persistence in the config (v1 constraint — session memory stays RAM-only)."
- Global plan (approved): `.kilo/plans/20261114-wave7-distribution-mvp.md` → `## Task 1 — Config file (existence + format decision)`.
- Date: 2026-11-14. Status: FOR PLANNER PRESENTATION (pending user approval before 4.2 runs).
- Branch: `feat/wave-7-distribution` (created at workflow Step 2 — NOT this task's job). Version at execution: 1.0.0 (bumped at Step 3 — NOT this task's job).
- Front-end related: NO (Wave-level decision #3 — no 4.1a, no 4.5a; this is the 4.1b architector settlement).

## 0. Deferred-Decision Settlement (BINDING — this section settles the wave's deferred decision "Config file contents/format")

Every decision below is final for Task 1's implementation. The implementer must not re-choose.

### D1 — Format: TOML-style (file extension `.toml`)

- The config file is a **TOML-style** file: UTF-8, no BOM, LF line endings, `#` comment lines only in v1.
- Trade-off, stated explicitly: **NO new dependency in v1.** The v1 binary does not parse the file (D4), so no `toml`/`serde` crate is added. When the FIRST real setting lands (a future version), that wave's architector decides the parser (e.g. the `toml` crate) — adding it now would be an unused dependency against tech.md's minimal-dependency-surface policy.
- Why not the alternatives:
  - INI-style: an informally standardized subset of TOML's key-value syntax with weaker tooling and ecosystem conventions; TOML dominates it for future-proofing at identical v1 cost (zero parse cost — nothing is parsed).
  - Plain text with header comments only: satisfies v1 but is NOT defensible as the application's future structured config location (no key syntax convention, no ecosystem tooling, no migration-free path).
- Precedent: the project already lives in a TOML world (`Cargo.toml`); TOML is the Rust ecosystem's canonical config format. A v1 header-only TOML file is valid TOML and needs NO format migration when keys arrive.

### D2 — Contents: header comments ONLY, ZERO keys

- The file carries NO keys — not even commented-out placeholder keys. A visible or commented key would pre-commit a future key's name before any settings decision exists (the parked Idea 8 checkbox naming is explicitly OPEN in `future-ideas.md`), and a present key a user could un-comment would silently no-op — a support hazard. Future key names are documented in the file when the settings that read them arrive (the header says exactly that).
- The header records: (a) the file's purpose and install location, (b) that settings arrive in a future version and the current version does not read the file, (c) the v1 constraint that the per-folder last-viewed memory is session RAM-only and is intentionally never stored here (satisfies the TODO's "No last-viewed persistence" clause verbatim).
- **Exact literal file content (the implementer writes EXACTLY these 11 lines, nothing more):**

```toml
# Images Scroller - application configuration file.
#
# This file is installed next to the Images Scroller executable and is
# the application's settings location. Settings arrive in a future
# version; the current version does not read this file, so changes made
# here have no effect yet.
#
# Future setting keys will be documented here when they are introduced.
#
# Note: the per-folder last-viewed image memory is session-only (kept in
# RAM) and is intentionally never stored in this file.
```

- Content properties: ASCII-only, UTF-8 without BOM, LF line endings, a trailing newline after the last line (`wc -l` = 11). No keys, no `[sections]`, no other bytes.

### D3 — Repo location + file name: `packaging/img-scroller.toml`

- New TRACKED directory `packaging/` at the repository root; the file is `packaging/img-scroller.toml`. NOT under `dist/` (gitignored build-artifact location — prohibited).
- Why `packaging/` (and not `config/`): Task 2's packaging script consumes it (`packaging/` groups distribution-shipped artifacts); Task 3's helper/guidance files likely join the same directory (their mechanism is Task 3's architector decision — NOT pre-decided here).
- Runtime name: the packaging step (Task 2) copies the file VERBATIM into the zip root, flat next to `img-scroller.exe`, keeping the same name `img-scroller.toml` (repo path == runtime name; no rename anywhere).
- `.gitattributes` gains ONE line at the end: `packaging/*.toml text eol=lf` (text file, LF everywhere, shipped to Windows users as LF — fine for TOML/Notepad). Do NOT add a generic `*.toml` rule (that would touch `Cargo.toml`/`Cargo.lock` renormalization — out of scope). `.gitignore` needs NO change (`packaging/` is tracked; `dist/` stays ignored).

### D4 — Runtime behavior of the v1 binary: the binary does NOT read the config

- **Decision: NO.** The v1 binary performs ZERO reads, parsing, existence checks, or references of the config file. No Rust source changes at all in Task 1.
- One-sentence justification: reading a file that contains zero consumable settings adds dead code, an I/O failure mode, and startup cost with no behavioral gain — the config's v1 role is distribution presence (brief: the package unzips the binary AND its config file), and parsing arrives with the first real setting in a future version.
- Consequence: the app's behavior must not change in any observable way — and with no source change, it cannot. (The fail-safe read contract is therefore moot in v1; if a future wave adds a read, "missing/unreadable config never blocks the viewer" is that wave's contract to define.)

### D5 — Tests: NONE in Task 1; census stays pinned

- No pure logic is added (no parsing, no Rust change), so **no unit tests are added**. The census remains **195 unit tests / 28 test-bearing files** (breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file modules). Task 4's plan must pin the census unchanged unless Tasks 2–3 add pure logic.
- No dev-checks (`scripts/dev-checks.sh`) run is required for Task 1: nothing cargo builds or lints changes (`src/` and `Cargo.toml` untouched). The formal dev-checks run belongs to Task 4 at wave HEAD.

## 1. Environment Facts (verified 2026-11-14)

- Git: `main` == `origin/main` == `0ec23e6`; version 0.6.2 at `main`; Task 1 executes on `feat/wave-7-distribution` AFTER Steps 2–3 (branch + 1.0.0 bump) per the global plan's step map.
- Repo root layout: no `packaging/` or `config/` directory exists; `.gitignore` covers `dist/`, `logs/`, `target/`, `*.log`; `.gitattributes` currently contains only `*.sh text eol=lf`.
- Alpine VM (expected RUNNING — verified at wave planning): all container-side evidence runs through the `alpine-vm` MCP (`vm_status` FIRST, then commands; exit code is the authoritative evidence). If the VM or the `/img-scroller` share is unavailable: STOP and return a user question (wave execution note).
- Census pinned: 195 unit tests / 28 test-bearing files (D5 — unchanged by Task 1).

## 2. Binding Constraints (apply to every step)

- All Rust/Cargo runs ONLY via the container harness on the Alpine VM (`alpine-vm` MCP; `vm_status` first; exit code authoritative). Task 1 needs NO Rust/Cargo command (D4/D5).
- NO `git push` (restricted to Step 5). NO branch creation/switch (restricted to Step 2). NO version edit (restricted to Step 3).
- Gitignore Compliance Rule before every commit: run `git status`, ensure NO `.gitignore`-matching file (`dist/`, `logs/`, `target/`, `*.log`, etc.) is staged.
- Do NOT edit any existing `.kilo/plans/*` file. This plan file is the only NEW plan file of this task.
- Do NOT edit `README.md`, `.agent/project-info/{brief,product,tech,architecture,context}.md`, `docs/*` — Task 5 owns the wave's prose documentation. Exception: `.agent/project-structure.md` (the structure map) IS edited by Task 1 (structure rule: new tracked files/folders must be reflected) and the 4.1b context-log closing touch already recorded by the architector (process bookkeeping, not prose docs).
- File writes use real newline characters (never literal `\n` escape sequences) — newline-prevention rule.
- New `src/` lines: none (D4). Max-lines rules: not applicable (no code).

## 3. Step-by-Step Implementation (workflow step 4.2 — implementer, JUNIOR under 50%)

Execute the steps in order. Check the plan between steps. Stop and ask the caller on any ambiguity — never guess.

### Step 0 — Commit the plan file (caller/implementer, BEFORE implementation)

- Verify: `git status` → on branch `feat/wave-7-distribution`; working tree contains (at least) the new file `.kilo/plans/20261114-wave7-task1-config-plan.md` plus the 3 pre-existing unstaged files recorded in the global plan (`.agent/project-info/context.md`, `.agent/project-info/future-ideas.md`, `.agent/todos/20261113/20261113-todo-2-DONE.md` — those ride Step 2's commit, NOT this one; leave them unstaged if Step 2 already committed them, in which case the tree is clean apart from the plan file).
- Stage ONLY the plan file: `git add .kilo/plans/20261114-wave7-task1-config-plan.md`
- Commit: `docs: add wave7 task1 config plan`

### Step 1 — Create the config file

- Create `C:\repo\img-scroller\packaging\img-scroller.toml` (new directory `packaging/` + new file).
- Content: EXACTLY the 11 lines from section D2 (nothing more, nothing less). LF line endings, UTF-8 no BOM, trailing newline.

### Step 2 — Pin LF in `.gitattributes`

- Append ONE line at the end of `C:\repo\img-scroller\.gitattributes` (after the existing `*.sh text eol=lf` line, separated by a newline):

```
packaging/*.toml text eol=lf
```

- Final file is 2 lines + trailing newline. Nothing else in the file changes.

### Step 3 — Update the structure map

- Edit `C:\repo\img-scroller\.agent\project-structure.md`:
  - In `# Other folders`, add ONE new bullet (alphabetical position: after `.opencode/`, before `docs/`):

```
- packaging/ - distribution-shipped artifacts: img-scroller.toml (the application config file, TOML-style, header comments only in v1 — the v1 binary does not read it; the Wave-7 packaging step copies it verbatim next to the exe)
```

  - Do NOT touch any other line of the file.

### Step 4 — Gitignore compliance pre-commit check

- `git status` → confirm the staged/untracked set is EXACTLY: `packaging/img-scroller.toml`, `.gitattributes`, `.agent/project-structure.md` (+ the Step-0-committed plan already in history). NO `dist/`/`logs/`/`target/`/`*.log` path appears. If a gitignored path appears: unstage it and re-check.

### Step 5 — Container-side verification (alpine-vm MCP — the task's acceptable evidence)

- Call `vm_status` FIRST. If not running/SSH-unreachable: STOP, return a user question.
- Run each command below as its OWN `alpine-vm_vm_run_command` call; the exit code is authoritative:
  1. `ls -la /img-scroller/packaging` — exit 0; output lists exactly `img-scroller.toml` (plus `.`/`..`).
  2. `cat /img-scroller/packaging/img-scroller.toml` — exit 0; output matches the D2 literal content line-for-line.
  3. `wc -l /img-scroller/packaging/img-scroller.toml` — exit 0; expect `11` as the line count.
  4. `sh -c "! grep -q \"\$(printf '\r')\" /img-scroller/packaging/img-scroller.toml"` — exit 0 = LF-only (exit 1 = a CR was found → fix the file's line endings and re-run).
- NO dev-checks run (D5). NO Rust/Cargo command (D4).

### Step 6 — Commit

- Stage EXACTLY: `git add packaging/img-scroller.toml .gitattributes .agent/project-structure.md`
- Commit message (verbatim): `feat: add application config file placeholder`
- Post-commit `git status`: clean apart from the 3 pre-existing ride-along files if Step 2 has not run yet (they are NOT Task 1's commit).

### Explicitly NOT done in Task 1 (for the implementer)

- NO Rust/Cargo/source changes; NO dev-checks; NO dependency changes (`Cargo.toml`/`Cargo.lock` untouched); NO version edits; NO README/tech/architecture/brief/product edits; NO `CHANGELOG.md` edit; NO packaging script (Task 2); NO helper (Task 3); NO `git push`; NO TODO marking (that is 4.6).

## 4. Workflow step 4.3 — Code Review & Simplification guidance

- code-reviewer checklist (deviation from plan = finding): the file at `packaging/img-scroller.toml` matches the D2 literal content byte-for-byte semantics (11 lines, ASCII, no BOM); `.gitattributes` has exactly the one added line; the structure-map bullet matches section Step 3 verbatim in substance; the commit contains ONLY the three files; NO file under `src/`, `Cargo.toml`, `Cargo.lock`, `dist/`, `docs/`, `.agent/project-info/*` (except the structure map), or `CHANGELOG.md` changed vs the Step-0 baseline; no gitignored path committed.
- code-simplifier: no code exists to simplify — expected outcome "not required" (a clear message is the deliverable). Do NOT create a simplification plan for a comments-only file.
- Max 3 review cycles, then escalate (workflow rule).

## 5. Workflow step 4.4 — Documentation guidance

- The docs-specialist adds NOTHING in Task 1: the config file is self-documenting by design (D2 header), no code exists to comment, and ALL wave prose documentation (README, tech.md `## Distribution and Packaging`, architecture.md "On disk" note, context.md wave entry) is Task 5's deliverable per the TODO's Task 5. Expected outcome: "nothing to document; deferred to Task 5" as a clear message. Creating any documentation file in Task 1 is a plan deviation.

## 6. Workflow step 4.5b — Plan-adherence guidance

- Verify sections 0–3 against the landed diff (`git diff <Step-0-commit>..<Task-1 commit>`): the three files and their exact changes, plus the two verification evidence points (container check exit 0; no dev-checks needed).
- If every element conforms: return a clear "no diffs, VERIFIED" message (no report file needed). If a deviation exists: record it in a NEW plan file `.kilo/plans/20261114-wave7-task1-adherence.md` and propose the disposition — never edit existing plans.

## 7. Workflow step 4.6 — Task completion (implementer)

- Edit `.agent/todos/20261029/20261029-todo-9.md` (preserve ALL other content byte-identical):
  - `### Task 1: Config file (existence + format decision)` → `### Task 1: Config file (existence + format decision) [DONE]`
  - Its single sub-item `- [ ] Introduce the application config file` → `- [x] Introduce the application config file` (keep the rest of that line verbatim).
- Commit message (verbatim): `task: mark wave7 task1 done`
- Gitignore compliance before the commit (`git status`; the TODO file is tracked — safe).

## 8. Risks / stop conditions

- VM or `/img-scroller` share unavailable at Step 5 → STOP, return a user question (wave execution note).
- Any plan ambiguity encountered → STOP and return the question to the Planner; never guess.
- The `.gitattributes` addition makes Git report the file as renormalized only on future checkouts — no renormalization action is required now (the file is created LF and stays LF).
- If the caller's environment already created `packaging/` for another artifact before this task (not the case at planning time), STOP and ask rather than overwrite.

## 9. Plan-vs-task compliance check

- TODO Task 1 requires: config file introduced ✔ (D3), shipped alongside the binary ✔ (Task 2 consumes `packaging/img-scroller.toml` verbatim — coordination note), contents/format decided in the architector step ✔ (D1/D2), minimal ✔ (11 comment lines, zero keys), consistent with architecture.md's "On disk" note ✔ (existence recorded; contents were an implementation decision — now decided here), NO last-viewed persistence ✔ (recorded in the file header AND absent from any mechanism). Nothing else from later tasks is executed. The plan is complete and correct against the task.
