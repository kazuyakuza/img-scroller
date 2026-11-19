# Wave 8 — Task 7 Release Workflow Review Fix Plan

Review date: 2026-11-18  
Reviewer: code-reviewer (Wave 8 Task 7 step 4.3)  
Scope: `.github/workflows/release.yml` + the Task 7 docs ride-alongs (plan §5) + chain discipline.

## Verdict

The implementation matches the **literal YAML block in plan §3** byte-for-byte and the docs ride-alongs match plan §5. Commit messages, file scope, and gitignore compliance are correct. However, the YAML as written (and as pinned in §3) does **not** satisfy plan §2 D5's requirement that non-"not found" errors from the idempotency gate fail visibly. One targeted fix is required before push.

---

## Issue 1: Idempotency gate swallows non-"not found" `gh release view` errors

**Location:** `.github/workflows/release.yml`, step `Check whether the release already exists`.

**Plan requirement (§2 D5):**

> before building, `gh release view v<version>` (exit-0 = exists). If the release EXISTS the job skips every build/release step and succeeds with a skip notice … If the check errors in a way that is NOT "not found", the job fails visibly (no silent skip).

**Current code (matches plan §3 literally):**

```yaml
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
```

**Deviation:** any non-zero exit from `gh release view` — including auth failures, network errors, permission errors, or repository-not-found — is recorded as `exists=false`. The job then proceeds to build and only fails later at `gh release create`. This masks the root cause and violates the "no silent skip" rule in D5.

**Root cause:** plan-internal conflict. §3's literal YAML implements only the "exists / not exists" binary, while §2 D5 requires distinguishing "not found" from other errors. The fix resolves the conflict in favor of D5.

**Fix:** replace the step `run` block with stderr-aware logic. `gh release view` writes "release not found" / "Not Found" / HTTP 404 to stderr on the expected miss; any other stderr indicates a real failure that must abort the job.

```yaml
      - name: Check whether the release already exists
        id: existing
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          VERSION="${{ steps.version.outputs.version }}"
          set +e
          stderr="$(gh release view "v$VERSION" --repo "$GITHUB_REPOSITORY" 2>&1 >/dev/null)"
          exit_code=$?
          set -e
          if [ "$exit_code" -eq 0 ]; then
            echo "exists=true" >> "$GITHUB_OUTPUT"
          elif printf '%s\n' "$stderr" | grep -qiE 'release not found|not found|HTTP 404|404 Not Found'; then
            echo "exists=false" >> "$GITHUB_OUTPUT"
          else
            printf '%s\n' "$stderr" >&2
            exit 1
          fi
```

**Verification after fix:**

1. `npx yaml valid .github/workflows/release.yml` must return 0 with no output.
2. `git diff --check` must report no whitespace/CRLF defects.
3. Re-verify the remaining §9 checklist items (trigger, permissions, concurrency, compose commands, symlink ordering, version sed, awk boundaries, release-create flags, action pinning, token env).
4. Update `.kilo/plans/20261118-wave8-task7-release-results.md` §1.3 to note that item 7 (gate semantics) initially passed the structural check but was later corrected to satisfy D5's non-"not found" failure requirement; do not edit the existing plan file (create new plan files only).

---

## What was verified as correct (no fix needed)

- **YAML byte identity with plan §3:** the committed `.github/workflows/release.yml` is byte-identical to the plan's fenced block (verified by line-by-line comparison; the only deviation is the issue above, which is a plan-internal D5/§3 conflict).
- **Syntax:** `npx yaml valid .github/workflows/release.yml` passes.
- **Trigger / permissions / concurrency / runner:** `on.push.branches: [main]`, `permissions: contents: write`, `concurrency: group release / cancel-in-progress: false`, `runs-on: ubuntu-latest`.
- **Harness commands:** `docker compose build` and `docker compose run --rm rust sh scripts/{dev-checks,build-windows,build-installer,package-windows}.sh` are verbatim; `build-windows.sh` receives no argument (both targets); `docker-compose.yml` is untouched.
- **Symlink strategy:** `sudo ln -sfn "$GITHUB_WORKSPACE" /img-scroller` precedes every compose invocation.
- **Version extraction:** byte-identical to the scripts' `tr -d '\r' < Cargo.toml | sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' | head -n 1` with the empty-version failure.
- **Release notes extraction:** `awk` prefix-matches `## [<version>]`, stops at the next `## `, and the `[ ! -s ]` non-empty guard is present.
- **`gh release create` flags:** `--title "Images Scroller $VERSION"`, `--notes-file`, `--target "$GITHUB_SHA"`, exactly 3 attachment paths, no `--draft`, no force flags; idempotency gate precedes builds.
- **Token wiring:** `GH_TOKEN: ${{ github.token }}` on both `gh` steps; `actions/checkout@v4` pinned as specified.
- **Docs ride-alongs:** `tech.md`, `architecture.md`, `context.md` (4.1b entry + Immediate Next Steps item 2), and `.agent/project-structure.md` match plan §5.
- **Untouched files:** `README.md`, `CHANGELOG.md`, `docker-compose.yml`, `src/`, `packaging/`, `scripts/`, `Cargo.toml`, `Cargo.lock` show no changes between `1fc3b51` and `04a8c5e`.
- **Commit messages and chain:** `2ef84c0 docs: revise wave8 task7 release plan to the actions mechanism`, `2e603fb ci: add github actions release workflow`, `04a8c5e docs: add wave8 task7 release results (implementation evidence)` (the third commit is the Planner-adjudicated results file, per caller context).
- **Gitignore compliance:** `git status --short` is clean; `dist/` artifacts and `opencode/node_modules` are ignored and not staged.

---

## Implementation instructions for the fix

1. Apply the Issue 1 replacement to `.github/workflows/release.yml`.
2. Run `npx yaml valid .github/workflows/release.yml` and `git diff --check`.
3. Amend commit `2e603fb` or create a new fix commit (per caller preference) with message `ci: make release idempotency gate fail on unexpected gh errors`.
4. Update the results file in a follow-up commit noting the correction.
5. Do **not** push; Step 5 remains the caller's responsibility.
