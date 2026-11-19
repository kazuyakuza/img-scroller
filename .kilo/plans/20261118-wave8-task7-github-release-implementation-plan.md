# Wave 8 — Task 7: GitHub Releases Publishing — Implementation Plan (REVISED — GitHub Actions mechanism)

- TODO: [`.agent/todos/20261117/20261117-todo-1.md`](../../.agent/todos/20261117/20261117-todo-1.md) — Task 7 (the wave's FINAL task). Runs ONLY after Tasks 1–6 are `[DONE]` (verified: `1fc3b51 task: mark wave8 task6 done`).
- **REVISION NOTE (binding):** this plan REPLACES the original 4.1b mechanism (gh-on-host/VM, verified missing → STOP-and-ask). The CALLER-USER's 2026-11-18 binding instruction supersedes it: **"Just use a github action, that should run when branch main is updated."** The publishing mechanism is a **GitHub Actions workflow** triggered on push to `main`. NO `gh` is installed agent-side; NO STOP-and-ask is needed — the mechanism requires no local gh at all. The user's instruction is the mechanism source (recorded in §2 D1 and in `context.md`).
- Critical Workflow step: **4.1b (Analysis and Planning — architector)**, redo round 1. Implementation plan for a **JUNIOR developer under 50% restriction** — every structural decision is pinned; no judgment calls remain.

## 0. Pinned repo-state facts (verified 2026-11-18)

| Fact | Value | Evidence |
|---|---|---|
| Branch / HEAD | `feat/wave-8-wizard-installer` / `1fc3b51` (`task: mark wave8 task6 done`); tree clean except the uncommitted `context.md` 4.1b entry this step fixes | `git branch --show-current`, `git log --oneline -8`, `git status` |
| Version | `1.1.0` | `Cargo.toml` line 3 (`version = "1.1.0"`) |
| `origin` | `https://kazuyakuza@github.com/kazuyakuza/img-scroller.git` → owner/repo `kazuyakuza/img-scroller` (HTTPS) | `.git/config` |
| `origin/main` | `eca641c` (pre-wave; NOT re-verified live — VM git cannot reach the remote) | `context.md` |
| CHANGELOG entry | `## [1.1.0] - 2026-11-18` — 5 `### Added` + 1 `### Changed` bullets, between `## [Unreleased]` (above) and `## [1.0.0] - 2026-11-15` (below) | `CHANGELOG.md` |
| Compose harness | service `rust`; image `img-scroller-rust:latest` built from the root `Dockerfile`; **`working_dir: /img-scroller`**; volumes: **`- /img-scroller:/img-scroller`** (an ABSOLUTE bind — VM-specific; see §1 R1) + the named volume `img-scroller-cargo-target` with `CARGO_TARGET_DIR: /cargo-target` | `docker-compose.yml` lines 1–16 |
| Dockerfile | `rust:1.99.1-bookworm` + `rustfmt`/`clippy` + `gcc-mingw-w64-x86-64` + `gcc-mingw-w64-i686` + `zip unzip nsis` + `rustup target add x86_64-pc-windows-gnu` + `i686-pc-windows-gnu` | `Dockerfile` lines 3–22 |
| `scripts/dev-checks.sh` | runs fmt-check/check/test/clippy, EVERY check runs even if earlier ones fail, final verdict `ALL CHECKS PASSED` exit 0 or `<N> CHECK(S) FAILED` exit 1 — the exit code IS the gate | `scripts/dev-checks.sh:39–55` |
| `scripts/build-windows.sh` | no argument = BOTH targets (x64 first, then x86) → `dist/windows-x64/img-scroller.exe` + `dist/windows-i686/img-scroller.exe`; typed `build-windows: error:` failures exit 1 | header lines 1–13 |
| `scripts/build-installer.sh` | consumes the x64 exe; `makensis` compile → `dist/ImagesScroller-<version>-windows-installer.exe`; version read via `tr -d '\r' < Cargo.toml \| sed -n 's/^version = "\([0-9]*\.[0-9]*\.[0-9]*\)"$/\1/p' \| head -n 1` | lines 1–12, 54–60 |
| `scripts/package-windows.sh` | consumes BOTH exes + 3 tracked packaged files → `dist/ImagesScroller-<version>-windows-{x64,x86}.zip`; same version extraction | lines 1–12, 61–70 |
| All three container scripts | `assert_current_directory` requires `pwd == /img-scroller` — satisfied by the compose `working_dir` | each script |
| Stale dist files | `dist/ImagesScroller-1.0.0-windows-{x64,x86}.zip` exist — NEVER attached to the release | `ls dist` |
| gh agent-side | MISSING on host, VM, and container (verified in the original 4.1b round — now IRRELEVANT as the mechanism; recorded only as the decision's context) | original plan §1, `context.md` line 30 |

## 1. Mechanism verification (done this step; the numbered open items R1–R4 are the 4.5b verification points)

- **R1 — compose portability (verified by file analysis):** `docker-compose.yml` binds the absolute host path `/img-scroller`. On a GitHub-hosted ubuntu runner the checkout lands at `$GITHUB_WORKSPACE` (`/home/runner/work/img-scroller/img-scroller`) and `/img-scroller` does not exist → a plain compose run would bind an empty auto-created directory and every script's `assert_current_directory`/file reads would fail. **Adaptation pinned (workflow step, runner-only): `sudo ln -sfn "$GITHUB_WORKSPACE" /img-scroller`** — Docker canonicalizes the bind path through the symlink, the container sees the checkout, artifacts land in the checkout, and the compose file's literal volume line stays UNTOUCHED (the VM harness is unaffected). ubuntu-latest runners have passwordless `sudo`. **4.5b point: confirm the symlink step runs before every compose invocation and that `docker-compose.yml` itself is not modified.**
- **R2 — runner capabilities (known facts, marked for 4.5b confirmation):** `ubuntu-latest` ships Docker + the Compose v2 plugin (`docker compose` subcommand), preinstalled `gh`, and passwordless `sudo`. **4.5b point: re-verify against the current GitHub-hosted-runners image docs before approving; if `docker compose` is unavailable the fallback (only with caller approval) is `docker-compose v2` install in a workflow step.**
- **R3 — release creation facts (known facts, marked for 4.5b confirmation):** `permissions: contents: write` is required for `gh release create` to create tags/releases via the default `GITHUB_TOKEN`; `github.sha` on a push-to-main trigger IS the pushed main HEAD, so `--target $GITHUB_SHA` points the tag at a commit that exists on the remote BY CONSTRUCTION (the workflow runs on GitHub's side — no local push, no push-restriction conflict: the tag is a GitHub-API creation, the same ruling the TODO already made for publishing); `gh` on the runner is authenticated through `GH_TOKEN: ${{ github.token }}`. **4.5b point: confirm the permissions block + env token wiring against the current Actions docs.**
- **R4 — YAML validation timing:** YAML/action mistakes surface ONLY after the Step-5 push (in the Actions tab). The 4.5b static-review checklist (§9) is the pre-push defense.

## 2. Settled decisions (all binding)

- **D1 — Mechanism (the user's binding instruction):** a GitHub Actions workflow `.github/workflows/release.yml`, trigger = push to `main`. It auto-publishes the `v<version>` GitHub Release with the installer + both zips attached and CHANGELOG-derived notes. No gh-on-host, no VM involvement, no STOP-and-ask.
- **D2 — Dev-checks gate: YES** (architect decision). The workflow runs the container dev-checks (`scripts/dev-checks.sh`, exit code authoritative) BEFORE any build/release — a broken `main` cannot release.
- **D3 — Build chain order (pinned, exactly the local harness):** `docker compose build` → `docker compose run --rm rust sh scripts/dev-checks.sh` → `docker compose run --rm rust sh scripts/build-windows.sh` (NO argument — BOTH targets, so both exes exist) → `docker compose run --rm rust sh scripts/build-installer.sh` (x64 payload) → `docker compose run --rm rust sh scripts/package-windows.sh` (both zips). The named `cargo-target` volume is reused across the steps of the one job (fresh on the runner; NO Actions cache is added — cold-build cost accepted, cache would be a future-ideas item, NOT this task).
- **D4 — Version + notes extraction (pinned to the scripts' own discipline):** the version is extracted with the SAME line-anchored sed the three scripts use (`tr -d '\r' < Cargo.toml | sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' | head -n 1`, empty → typed failure exit 1). The notes are extracted from `CHANGELOG.md` with the pinned awk (§3 step "Extract the release notes") keyed on the version string, into `dist/release-notes-v<version>.md`, with the `[ -s … ]` non-empty guard.
- **D5 — Idempotency gate (pinned):** before building, `gh release view v<version>` (exit-0 = exists). If the release EXISTS the job skips every build/release step and succeeds with a skip notice — repeated pushes to `main` never duplicate a release. If the check errors in a way that is NOT "not found", the job fails visibly (no silent skip).
- **D6 — Tag semantics:** tag name `v<version>`; created BY the workflow's `gh release create` via the GitHub API with `--target $GITHUB_SHA` (the pushed main HEAD). NO local `git push`/`git tag` anywhere; the workflow is NOT a push.
- **D7 — Release title:** `Images Scroller <version>` (e.g. `Images Scroller 1.1.0`).
- **D8 — Attachments (exactly 3, from the checkout's `dist/`):** `ImagesScroller-<version>-windows-installer.exe`, `ImagesScroller-<version>-windows-x64.zip`, `ImagesScroller-<version>-windows-x86.zip`. The 1.0.0 zips are NEVER attached (the fresh runner checkout contains only the `<version>` artifacts anyway — the gate is the version-stamped names).
- **D9 — upload-artifact debugging aid: NO** (decided). The release itself is the artifact channel; a failing run leaves its diagnostics in the Actions logs. Keep the workflow lean.
- **D10 — Failure semantics:** ANY step failure → the job fails, NO release is created, the failure is visible in the Actions tab. The scripts' own typed-failure discipline (`build-windows: error:` etc.) passes through unchanged.
- **D11 — Permissions/concurrency (pinned):** `permissions: contents: write` (least privilege for releases only); `concurrency.group: release`, `cancel-in-progress: false` (never cancel a release mid-build; one release job at a time).
- **D12 — Runner:** `ubuntu-latest`.

## 3. The new tracked file `.github/workflows/release.yml` — FULL literal content (the implementer creates it verbatim)

```yaml
# Images Scroller release automation (Wave 8 Task 7).
# On every push to main: runs the container dev checks, cross-builds the Windows
# artifacts through the same compose harness used locally, and publishes a
# GitHub Release tagged v<version> (version read from Cargo.toml) with the
# wizard installer and both portable zips attached and the matching CHANGELOG
# section as the release notes. The release is idempotent per version: if
# v<version> already exists the job skips building and publishing.
# Publishing is a GitHub-API action (gh + the workflow token) — NOT a git push.

name: Release

on:
  push:
    branches:
      - main

permissions:
  contents: write

concurrency:
  group: release
  cancel-in-progress: false

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - name: Check out the repository
        uses: actions/checkout@v4

      - name: Read the package version
        id: version
        run: |
          VERSION="$(tr -d '\r' < Cargo.toml | sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' | head -n 1)"
          if [ -z "$VERSION" ]; then
            echo "release: error: cannot extract the package version from Cargo.toml" >&2
            exit 1
          fi
          echo "version=$VERSION" >> "$GITHUB_OUTPUT"

      - name: Check whether the release already exists
        id: existing
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          VERSION="${{ steps.version.outputs.version }}"
          if gh release view "v$VERSION" --repo "$GITHUB_REPOSITORY" >/dev/null 2>&1; then
            echo "exists=true" >> "$GITHUB_OUTPUT"
          else
            echo "exists=false" >> "$GITHUB_OUTPUT"
          fi

      - name: Point the compose mount at the checkout
        if: steps.existing.outputs.exists == 'false'
        run: sudo ln -sfn "$GITHUB_WORKSPACE" /img-scroller

      - name: Build the container image
        if: steps.existing.outputs.exists == 'false'
        run: docker compose build

      - name: Run the dev checks in the container
        if: steps.existing.outputs.exists == 'false'
        run: docker compose run --rm rust sh scripts/dev-checks.sh

      - name: Build the Windows release artifacts in the container
        if: steps.existing.outputs.exists == 'false'
        run: |
          docker compose run --rm rust sh scripts/build-windows.sh
          docker compose run --rm rust sh scripts/build-installer.sh
          docker compose run --rm rust sh scripts/package-windows.sh

      - name: Extract the release notes from the changelog
        if: steps.existing.outputs.exists == 'false'
        run: |
          VERSION="${{ steps.version.outputs.version }}"
          awk -v heading="## [$VERSION]" '
            index($0, heading) == 1 { printing = 1; print; next }
            printing && /^## / { exit }
            printing { print }
          ' CHANGELOG.md > "dist/release-notes-v$VERSION.md"
          if [ ! -s "dist/release-notes-v$VERSION.md" ]; then
            echo "release: error: no CHANGELOG entry found for version $VERSION" >&2
            exit 1
          fi

      - name: Create the release
        if: steps.existing.outputs.exists == 'false'
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          VERSION="${{ steps.version.outputs.version }}"
          gh release create "v$VERSION" \
            --title "Images Scroller $VERSION" \
            --notes-file "dist/release-notes-v$VERSION.md" \
            --target "$GITHUB_SHA" \
            "dist/ImagesScroller-$VERSION-windows-installer.exe" \
            "dist/ImagesScroller-$VERSION-windows-x64.zip" \
            "dist/ImagesScroller-$VERSION-windows-x86.zip"

      - name: Notice the skipped release
        if: steps.existing.outputs.exists == 'true'
        run: echo "release: skip: v${{ steps.version.outputs.version }} already exists; no new release"
```

Content notes for the implementer: real newlines, LF endings (add nothing to `.gitattributes` — YAML has no rule; the repo blob is LF by default); the `if:` conditions use the exact string comparisons shown; the awk `index($0, heading) == 1` prefix match handles the full `## [1.1.0] - 2026-11-18` heading line; the notes file lives under gitignored `dist/`; the CHANGELOG's section order (Unreleased above `[1.1.0]`, `[1.0.0]` below) makes the awk stop at `## [1.0.0] - 2026-11-15`.

## 4. Sub-step mapping for the 4.x cycle (binding)

1. **4.2 (implementer):** create `.github/workflows/release.yml` verbatim per §3; apply the docs ride-alongs (§5); fix the uncommitted `context.md` 4.1b entry (§5 item 3 — already done in the working tree by this 4.1b step; the implementer VERIFIES it, does not redo it). Commit per §7. **NO push. A local commit does NOT trigger the workflow — the workflow activates only when Step 5 pushes `main` to `origin`. State this in the summary.**
2. **4.3 (code-reviewer + code-simplifier):** review the YAML + docs edits against this plan (static review; no local run possible — §9 checklist is the rubric).
3. **4.4 (docs-specialist):** confirm/complete the §5 ride-alongs; no new prose beyond §5.
4. **4.5b (architector):** adherence check with the §9 static-review checklist; the R1–R4 verification points adjudicated.
5. **4.6 (implementer):** mark semantics per §8.
6. **Step 5 (the caller sequences AFTER 4.6):** the normal TODO rename/merge/push of `main` → `origin` ONLY. The push triggers the workflow on GitHub.
7. **Post-push verification (the caller/user performs; recorded by the implementer when the report returns):** open `https://github.com/kazuyakuza/img-scroller/actions` (the `Release` run must go green) and `https://github.com/kazuyakuza/img-scroller/releases` (the `v1.1.0` release with exactly 3 assets + the CHANGELOG notes). Read-only web checks. A red run or a failed asset upload → STOP and ask the caller (fix plan in a NEW plan file; existing plans are never edited).

## 5. Docs ride-alongs (EXACT enumerated edits; only mechanism statements change — all other Task-6 facts stay)

1. **tech.md** — the `Releases flow` bullet (line ~220, in `## Wave-8 Wizard-Installer and Window-Title Mechanisms (Implemented)`) currently reads "EXECUTED BY TASK 7, pending at this documentation's date … `gh` CLI availability/auth is VERIFIED FIRST … STOP-and-ask …". REPLACE the bullet with: `- Releases flow (implemented, Task 7): a GitHub Actions workflow .github/workflows/release.yml (committed on the wave branch) triggers on EVERY push to main, runs the container dev checks and the build-windows (both targets) → build-installer → package-windows chain in the SAME compose harness on the ubuntu-latest runner (the checkout is symlinked to /img-scroller so the compose volume literal holds unchanged), extracts the version from Cargo.toml (the scripts' line-anchored sed) and the notes from the matching CHANGELOG ## [<version>] section, and publishes the GitHub Release tagged v<version> titled "Images Scroller <version>" with the installer + both zips attached — idempotent per version (a gh release view skip gate: repeated main pushes never duplicate a release), tag created via the GitHub API with --target of the pushed main HEAD, permissions: contents: write, any step failure → no release (visible in the Actions tab).`
2. **architecture.md** — the Distribution decisions row (line ~148) currently ends "the Release publish itself = Task 7, pending". REPLACE that parenthetical with `the Release publish = the GitHub Actions release workflow (push-to-main trigger; idempotent per version)`.
3. **context.md** — the UNCOMMITTED 4.1b entry at line ~30 (this step's artifact): **already UPDATED in the working tree by this 4.1b step** (mechanism = GitHub Actions per the user's 2026-11-18 instruction; the gh absence now records as decision context only). The 4.2 implementer VERIFIES the entry text matches this plan's §2 D1/D6 facts — verify-only, no re-edit. **Also** (ride-along edit for the implementer): `Immediate Next Steps` item 2 (line ~127) still describes the gh STOP-and-ask flow — REPLACE the middle sentences with: `the publishing mechanism is the GitHub Actions release workflow .github/workflows/release.yml (the user's 2026-11-18 binding instruction): it triggers on push to main, runs the container dev-checks + build chain, and publishes the v<version> release (installer + both zips, CHANGELOG notes) idempotently; the local commit CANNOT trigger it — it activates at Step 5's push, and the post-push verification is the read-only check of the Actions run and the Releases page.`
4. **README.md** — **NO edit (decided, pinned):** verified that the README's Distribution-and-Installation intro describes the user-facing Releases CHANNEL, not the publishing mechanism; no gh/publish-mechanism statement exists to fix.
5. **CHANGELOG.md** — **LEAVE the `[1.1.0]` entry's release bullet (line 30) unchanged (decided, pinned):** it describes what the release carries (user-facing), not the internal mechanism; the mechanism lives in tech/architecture/context.
6. **`.agent/project-structure.md`** — add to the `# Other folders` list, as the FIRST bullet (before `.agent/`), exactly: `- .github/ - CI: the GitHub Actions release workflow (workflows/release.yml) — on every push to main it runs the container dev checks and the build chain through the same compose harness on a GitHub-hosted runner and publishes the tagged v<version> GitHub Release (Wave 8 Task 7)`

## 6. Results-record file `.kilo/plans/20261118-wave8-task7-release-results.md`

Created at 4.2 with the implementation evidence (the workflow file created, the docs ride-alongs applied, the commits, the note that the mechanism activates at Step 5's push); COMPLETED post-push with the release verification (the Actions run verdict, the release URL, the tag target = the pushed main HEAD, the 3 asset names + observed sizes, the notes rendering). Both updates pinned (creation commit + the post-push completion edit/commit when the report returns — the latter rides the next wave's push if Step 5 already ran, per the established precedent).

## 7. Commits (pinned messages, executed at 4.2; Gitignore Compliance Rule before each)

| Order | Files | Message |
|---|---|---|
| 1 | the revised plan file (untracked → tracked) + `context.md` (the corrected 4.1b entry) | `docs: revise wave8 task7 release plan to the actions mechanism` |
| 2 | `.github/workflows/release.yml` + tech.md + architecture.md + context.md (Next-Steps item) + `.agent/project-structure.md` | `ci: add github actions release workflow` |

No `dist/` files are committed (gitignored).

## 8. 4.6 mark semantics (pinned)

- The Task-7 heading gets `[DONE]` at 4.6, and BOTH sub-items are marked `[x]` ONLY when the mechanism is implemented (the workflow file committed) AND — after Step 5's push — the release is verified to exist with the 3 artifacts (the caller's read-only check per §4 step 7; the user's confirmation or the caller's web check is the evidence recorded in the results file).
- If the wave closes before the post-push verification returns (or the run is red), the sub-items stay `[ ]` with the dated annotation `(post-push verification pending — see the task-7 results file)`, mirroring the manual-UX sub-item precedent; a later dated touch marks them `[x]`.
- Sub-item 2 additionally gets the dated decision note: `— decided: GitHub Actions workflow on push to main (the user's 2026-11-18 instruction); idempotent per version; tag created via the GitHub API — no local push`.

## 9. 4.5b static-review checklist (the pre-push defense; every item checked before approval)

1. YAML parses (syntax, indentation, block scalars, the `${{ }}` expressions, the `if:` conditions referencing `steps.existing.outputs.exists`).
2. `on.push.branches: [main]`; `permissions: contents: write`; `concurrency.group: release`, `cancel-in-progress: false`; `runs-on: ubuntu-latest`.
3. The compose commands match the local harness VERBATIM (`docker compose build`; `run --rm rust sh scripts/dev-checks.sh` / `build-windows.sh` / `build-installer.sh` / `package-windows.sh` — no invented flags, no `-f` path changes, `docker-compose.yml` untouched).
4. The `/img-scroller` symlink step precedes every compose step (R1).
5. The version sed is byte-identical to the scripts' extraction (R: §0).
6. The awk extraction boundaries (prefix match on `## [<version>]`, stop at the next `## `) and the non-empty guard.
7. `gh release create` flags: `--title`, `--notes-file`, `--target $GITHUB_SHA`, EXACTLY the 3 attachment paths; no `--draft`, no force flags; the idempotency gate precedes everything build-related.
8. Action versions pinned (`actions/checkout@v4`); `GH_TOKEN: ${{ github.token }}` env on both gh steps (R3).
9. R2 runner-capability facts re-confirmed against the current GitHub docs.

## 10. What NOT to do (binding prohibitions)

- NO `gh` installation anywhere (host/VM/container) — the mechanism needs no local gh.
- NO `git push` outside workflow Step 5 (`main` → `origin` ONLY); NO local `git tag`; NO local workflow runs — **a local commit does NOT trigger the workflow; it activates only on the Step-5 push of `main`**.
- NO edits to existing `.kilo/plans/*` files (new plan files only; this revision is the mandated rewrite of the still-untracked Task-7 plan).
- NO edits beyond §5's enumerated lines in the docs files; NO CHANGELOG/README mechanism edits (§5 items 4–5 pin the NO-edit decisions).
- NO attaching the 1.0.0 zips; NO `--draft`/force release flags; NO release deletion.
- NO Actions cache, NO upload-artifact step, NO additional jobs (lean single job — D9/D3).
- NO Step-5 actions inside Task 7 (the caller sequences Step 5 after 4.6; the release verification happens after the Step-5 push).
