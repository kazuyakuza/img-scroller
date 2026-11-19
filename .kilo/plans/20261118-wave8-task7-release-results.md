# Wave 8 — Task 7 Release Publishing Results

- Source TODO: `.agent/todos/20261117/20261117-todo-1.md` → `### Task 7: GitHub Releases publishing` (the wave's FINAL task).
- Implementation plan: `.kilo/plans/20261118-wave8-task7-github-release-implementation-plan.md` (REVISED to the GitHub Actions mechanism; approved).
- Date: 2026-11-18.
- Branch: `feat/wave-8-wizard-installer`; HEAD at the 4.2 run start: `1fc3b51` (`task: mark wave8 task6 done`); the implementation commits are `2ef84c0` and `2e603fb`, and this results file is committed after them.
- Version: 1.1.0 (`Cargo.toml` line 3).
- Mechanism: a GitHub Actions workflow `.github/workflows/release.yml`, triggered on EVERY push to `main` — the caller-user's 2026-11-18 binding instruction ("Just use a github action, that should run when branch main is updated."), per the plan §2 D1.
- Decision context (the agent-side `gh` absence): the FIRST 4.1b round verified `gh` availability/auth as the original TODO mandated and found `gh` MISSING everywhere — the host (absent from every standard install location; direct invocation blocked by the host tool-permission allowlist, so no auth), the Alpine VM (not on `PATH`; `vm_status` FIRST, running and SSH-reachable), and the pinned container image `rust:1.99.1-bookworm` (`NO_GH_IN_CONTAINER`, exit 0); the VM share's git cannot reach the remote (`dubious ownership`/no remote access). That finding made an agent-side `gh` publish impossible and is the reason the caller-user chose the Actions mechanism. It is recorded here as decision context only: the shipped mechanism needs NO local `gh` at all (the runner ships `gh` preinstalled; see §1 item 9).
- Execution channel: the workflow YAML was authored host-side with structured write tools; NO Docker/VM command was run for this task and NO workflow was executed locally (a local commit does NOT trigger the workflow — it activates only at workflow Step 5's push of `main`).

---

## 1. Implementation Evidence (workflow Step 4.2)

### 1.1 Commits (verbatim, in order)

| Order | Hash | Message | Files |
|---|---|---|---|
| 1 | `2ef84c0` | `docs: revise wave8 task7 release plan to the actions mechanism` | the revised plan file (untracked → tracked, 215 lines) + `.agent/project-info/context.md` (the corrected 4.1b entry, +2 lines) |
| 2 | `2e603fb` | `ci: add github actions release workflow` | `.github/workflows/release.yml` (new, 102 lines) + `.agent/project-info/tech.md` (the Releases-flow bullet) + `.agent/project-info/architecture.md` (the Distribution decisions-row parenthetical) + `.agent/project-info/context.md` (Immediate-Next-Steps item 2) + `.agent/project-structure.md` (the `.github/` bullet) |
| 3 | (this commit) | `docs: add wave8 task7 release results (implementation evidence)` | this file only |

### 1.2 Files created / edited

- Created: `.github/workflows/release.yml` — the FULL literal content pinned by the plan §3, verified byte-identical to the plan's §3 block by a programmatic diff (only the plan's closing code fence excluded).
- Edited (plan §5 ride-alongs only): `tech.md` releases-flow bullet → the Actions mechanism; `architecture.md` decisions-row parenthetical → `the Release publish = the GitHub Actions release workflow (push-to-main trigger; idempotent per version)`; `context.md` Immediate-Next-Steps item 2 → the Actions mechanism (and the 4.1b entry verified coherent with §2 D1/D6 — mechanism, `--target` of the pushed main HEAD, no local push); `.agent/project-structure.md` → the `.github/` bullet added as the FIRST `# Other folders` entry.
- Verified NOT touched (pinned): `README.md`, `CHANGELOG.md` (plan §5 items 4–5), `docker-compose.yml` (byte-identical — the `/img-scroller` symlink strategy in the workflow handles the runner path), `src/`, `packaging/`, `scripts/`, `Cargo.toml`, `Cargo.lock`.

### 1.3 Static-review checklist (plan §9 — all PASS before commit)

1. **YAML parses** — PASS (structural): no YAML parser exists locally (PyYAML absent on the host and the VM; no `yq`), so the file was validated by the byte-identical diff against the approved plan §3 plus a manual review of indentation, block scalars, the `${{ }}` expressions, and the `if:` conditions referencing `steps.existing.outputs.exists`; `git diff --check` reported no whitespace/CRLF defects (the committed blob is LF).
2. **Trigger / permissions / concurrency / runner** — PASS: `on.push.branches: [main]` (L12–15); `permissions: contents: write` (L17–18); `concurrency.group: release`, `cancel-in-progress: false` (L20–22); `runs-on: ubuntu-latest` (L26).
3. **Compose commands match the local harness verbatim** — PASS: `docker compose build` (L59); `docker compose run --rm rust sh scripts/dev-checks.sh` (L63); `build-windows.sh` (NO argument = both targets), `build-installer.sh`, `package-windows.sh` (L68–70); no invented flags, no `-f` path changes, `docker-compose.yml` untouched.
4. **The `/img-scroller` symlink step precedes every compose step (R1)** — PASS: `sudo ln -sfn "$GITHUB_WORKSPACE" /img-scroller` (L53–55) precedes all compose invocations (L57+); the compose file's absolute volume literal stays unchanged.
5. **The version sed is byte-identical to the scripts' extraction** — PASS (L34 matches the plan §0/§3 line-anchored sed exactly, with the same empty-version typed failure).
6. **The awk extraction boundaries and the non-empty guard** — PASS: the `index($0, heading) == 1` prefix match on `## [<version>]`, the stop at the next `/^## /` (L76–80), and the `[ ! -s ]` guard (L81–84).
7. **`gh release create` flags** — PASS: `--title`, `--notes-file`, `--target "$GITHUB_SHA"`, and EXACTLY the 3 attachment paths (installer + x64 zip + x86 zip); no `--draft`, no force flags; the idempotency gate (L41–51) precedes every build/release step.
8. **Action versions pinned + token env (R3)** — PASS: `actions/checkout@v4` (L29); `GH_TOKEN: ${{ github.token }}` on both `gh` steps (L43–44 and L88–89), with `permissions: contents: write` supplying the release-creation authority.
9. **R2 runner-capability facts re-confirmed against the current GitHub docs** — PASS: the current runner-images readmes for BOTH Ubuntu 24.04 and Ubuntu 26.04 list Docker (28.0.4 / 29.4.2), the Docker Compose v2 plugin (2.38.2 / 5.1.3 — so the `docker compose` subcommand is available), GitHub CLI 2.103.3, and `sudo`. NOTE for the 4.5b check: the announcement states the `ubuntu-latest` label moves to Ubuntu 26.04 in November 2026; both images carry the required toolset, so the workflow needs no fallback.

### 1.4 Idempotency and failure semantics (as implemented)

- The gate `gh release view "v$VERSION"` sets `exists=true|false`; when `true`, every build/release step is skipped and the job succeeds with the `release: skip:` notice — repeated pushes to `main` never duplicate a release.
- ANY step failure fails the job and creates NO release; the failure is visible in the Actions tab, and the scripts' own typed-failure discipline (`build-windows: error:` etc.) passes through unchanged.

---

## 2. Post-Push Release Verification (PENDING — recorded after workflow Step 5)

Status: PENDING. A local commit does NOT trigger the workflow; it activates when workflow Step 5 pushes `main` to `origin`.

Procedure (read-only, performed by the caller after the Step-5 push):

1. Open `https://github.com/kazuyakuza/img-scroller/actions` — the `Release` run for the pushed `main` HEAD must complete green. A red run → STOP and ask the caller (a fix plan goes into a NEW plan file; existing plans are never edited).
2. Open `https://github.com/kazuyakuza/img-scroller/releases` — confirm the `v1.1.0` release exists with:
   - tag `v1.1.0`, targeting the pushed `main` HEAD (created via the GitHub API with `--target $GITHUB_SHA`);
   - title `Images Scroller 1.1.0`;
   - notes rendered from the `CHANGELOG.md` `## [1.1.0] - 2026-11-18` section (up to the next `## ` heading);
   - EXACTLY 3 assets: `ImagesScroller-1.1.0-windows-installer.exe`, `ImagesScroller-1.1.0-windows-x64.zip`, `ImagesScroller-1.1.0-windows-x86.zip` (the 1.0.0 zips are never attached).
3. Record the verdict, the release URL, the tag target, the 3 observed asset names + sizes, and the notes rendering in this section (a later small commit — not part of the 4.2 step).

---

## 3. What Was NOT Done

- NO `git push`, NO tag creation, NO workflow run locally, NO `gh` install, NO Docker/VM execution — a local commit does NOT trigger the workflow; it activates only at workflow Step 5's push of `main`.
- NO workflow Step-5 actions inside this task (the TODO rename/merge/push are sequenced by the caller after 4.6).
- NO edits to `README.md` / `CHANGELOG.md`, `src/`, `packaging/`, `scripts/`, `Cargo.toml`/`Cargo.lock`, or `docker-compose.yml`.
