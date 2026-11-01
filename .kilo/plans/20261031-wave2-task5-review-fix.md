# Wave 2 Task 5 — Documentation Review Fix Plan

Date: 2026-11-01
Source review: Step 4.3 code-reviewer verification of commit `0b9702f`.
Status: Fix required — one genuine doc defect found.

## Defect summary

`.agent/project-info/context.md` §"Current Project State" contains stale git refs:

- Line 35 claims Wave 1 is on `main` at `92dc22c`, but `origin/main` is now at `5e5a91b` (and `92dc22c` is the older TODO-done merge commit).
- Line 42 claims `main == origin/main == 92dc22c`, but the current refs are:
  - `main` → `24ad0b6` (`docs: add Wave 2 horizontal scroll global plan`, local, not pushed)
  - `origin/main` → `5e5a91b` (`docs: record Wave 1 completion in context log`)
  - feature branch HEAD → `0b9702f` (the docs commit being reviewed)

All other landed doc statements were verified against the code, tests, exe size, DLL list, and commit graph and are accurate.

## Fix edits (docs-specialist only)

Apply only these two replacements in `.agent/project-info/context.md`.

### F1 — line 35: correct Wave-1 `main` hash

Current:

```
- **Wave 1 landed on `main`** (`92dc22c`, pushed to `origin`): container harness + application skeleton (open flow, WIC decode, fit display). **The user confirmed the Wave-1 exe OPERATOR-RUN worked as expected (reported 2026-11-01).**
```

Replacement:

```
- **Wave 1 landed on `main`** (pushed to `origin` through `5e5a91b`): container harness + application skeleton (open flow, WIC decode, fit display). **The user confirmed the Wave-1 exe OPERATOR-RUN worked as expected (reported 2026-11-01).**
```

### F2 — line 42: correct current git refs

Current:

```
- Git: branch `feat/wave-2-horizontal-scroll` (Tasks 1–4 through `85007ce`, plus this step's docs commit); `main` == `origin/main` == `92dc22c`; merge/push happen at Step 5.
```

Replacement:

```
- Git: branch `feat/wave-2-horizontal-scroll` (Tasks 1–4 through `85007ce`, plus this step's docs commit `0b9702f`); `main` is at `24ad0b6` (Wave-2 global plan, not yet pushed); `origin/main` is at `5e5a91b`; merge/push happen at Step 5.
```

## Scope notes

- NO other files change.
- NO wording re-litigation — the only changes are factual git hashes/refs.
- The historical `92dc22c` mention in the "Wave 1 COMPLETE and merged" Recent Changes entry (line 23) stays; it correctly describes that past commit.
- After applying, implementer commits with message: `docs: address Task 5 review findings`.
- Re-verify: `git -C C:\repo\img-scroller rev-parse main` → `24ad0b6...`; `git rev-parse origin/main` → `5e5a91b...`; context.md matches.
