# Wave 7 — Task 2 Fix Plan: CR-Tolerant Package-Version Extraction

- Context: Task 2 step 4.2 STOPPED at plan §3 Step 11.2 (typed failure `package-windows: error: cannot extract the package version from Cargo.toml`, exit 1). The implementer's diagnosis is CONFIRMED by the Planner: the plan's pinned `extract_package_version` sed uses a `"$` end-of-line anchor, but the working-tree `Cargo.toml` is CRLF (diagnostic: `od -c` of line 3 shows `v e r s i o n   =   " 1 . 0 . 0 " \r \n`). `.gitattributes` pins LF only for `*.sh` and `packaging/*.toml` — `Cargo.toml` follows the Windows checkout's CRLF (the repo's KNOWN, deferred renormalization item; fixing THAT is explicitly out of scope here).
- Ruling (Planner, 2026-11-14): make the extraction CR-tolerant in the script. Rejected alternative: adding `Cargo.toml text eol=lf` to `.gitattributes` — it would touch the deferred renormalization surface and potentially churn other tracked files. The script must work regardless of the working tree's line endings.
- This is a plan AMENDMENT recorded in a NEW file (existing plans are never edited). All other Task-2 plan decisions (D1–D8) remain binding.

## The fix (exactly ONE function body change in `scripts/package-windows.sh`)

Replace in `extract_package_version()`:

```sh
  package_version=$(sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' "$CARGO_MANIFEST" | head -n 1)
```

with:

```sh
  # Strip CRs first: the Windows working tree checks Cargo.toml out with CRLF
  # line endings (.gitattributes pins LF only for *.sh and packaging/*.toml),
  # and the end-of-line anchor below must match both LF and CRLF checkouts.
  package_version=$(tr -d '\r' < "$CARGO_MANIFEST" | sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' | head -n 1)
```

- `tr -d '\r'` is POSIX-portable and strips every CR before the anchored sed match; an LF checkout is unaffected (no CRs to strip). Nothing else in the function changes (the empty-extraction typed failure stays).

## Execution order (implementer resumes Task 2 §3 at Step 11)

1. Commit THIS fix plan file: stage exactly `.kilo/plans/20261114-wave7-task2-version-extraction-fix.md` → commit verbatim `docs: add wave7 task2 version extraction fix plan`.
2. Apply the one-function-body change to `C:\repo\img-scroller\scripts\package-windows.sh` (real newlines, LF endings — the file already carries the `*.sh` LF pin; keep the file byte-identical otherwise).
3. Re-run plan §3 Step 11 IN FULL, one command per call via the `alpine-vm` MCP (`vm_status` first; exit code authoritative):
   - CR check of `scripts/package-windows.sh` → exit 0.
   - Packaging run → exit 0 with `package-windows: zip: dist/ImagesScroller-1.0.0-windows-x64.zip` + `...windows-x86.zip` + `package-windows: OK`.
   - `ls -l` of the four artifacts/zips → all non-empty (record byte sizes).
   - `unzip -l` both zips → EXACTLY 2 flat entries each (`img-scroller.exe`, `img-scroller.toml`).
4. Gitignore compliance check → stage EXACTLY `scripts/package-windows.sh .agent/project-structure.md` → commit verbatim `feat: add portable windows zip packaging` (this is plan §3 Step 12 — the script + structure map were left uncommitted by the stop).
5. Explicitly NOT done: no other file edits (context.md bookkeeping stays unstaged), no push, no TODO marking.
