# Wave 5.1 Task 6 — Documentation Review Fix Plan

Review step: Wave 5.1, Task 6, Step 4.3 (code-reviewer / documentation review arm).
Branch: `fix/wave-5-1-operator-feedback`.
Commits reviewed: `acb21ba` (C1 plan) + `d85dcd4` (C2 docs).

## Verdict

The landed documentation in the five committed files is **factually accurate and conforms to the docs plan** `.kilo/plans/20261112-wave5-1-task6-documentation-plan.md`. No content edits are required to README.md, tech.md, architecture.md, context.md, or brief.md.

One cleanup item is required: two gitignored scratch leftovers remain in `logs/` from the docs-specialist's execution.

## What was verified (no change needed)

- Commit range `8ed2b28..d85dcd4` contains exactly the expected files: the plan file plus the five doc files (`README.md`, `tech.md`, `architecture.md`, `context.md`, `brief.md`).
- Test census: 164 `#[test]` annotations counted across 26 Rust test files, matching the plan.
- Release executable size at `dist/windows-x64/img-scroller.exe`: 19,636,215 bytes.
- DLL import audit list in `scripts/build-windows.sh`: 22 entries, unchanged.
- Code/doc consistency checks:
  - `ZOOM_IN_ALIAS_CHARACTERS` is `['+', '=', '*']` at `src/app/zoom_alias.rs:25`; `shift_alias_is_pressed` remains at `:28`; out-family untouched.
  - `request_expiry_wake` is at `src/app/esc_close_confirm.rs:141`, called per frame at `:116`; `POPUP_EXPIRY_WAKE_MARGIN_SECONDS = 0.05` at `:37`; `input.time` is the single clock via `current_time`.
  - `EntryNameOrder` seam at `src/name_collation.rs:24`; `CompareStringEx`/`GetUserDefaultLocaleName`/`NORM_IGNORECASE` in `src/platform/windows_collation.rs`; fallback in `src/name_collation.rs` and `src/platform/stub.rs`; both listings consume the seam and the anchor position reads the stored order.
  - `Cargo.toml` has exactly eight Windows-target features including `Win32_Globalization` and `Win32_System_SystemServices`.
- `brief.md` addendum item 8 appears exactly once and follows the pattern of items 5–7.
- The Task-5 §11 OPERATOR-RUN block is reproduced byte-for-byte in `context.md` Immediate Next Steps.
- The stale Wave-4.1 bullet text (`fix/wave-4-operator-fixes` still described as live/pending) is gone from `context.md` Current Project State.
- No doc sentence claims behavior absent from the code.

## Adjudication of docs-specialist deviations

1. **T10a self-correction** — Acceptable. The final tech.md census bullet matches the plan's pinned text exactly.
2. **Whole-line .NET ops for oversized anchors** — Acceptable. The PowerShell script used `[System.IO.File]::ReadAllLines`/`ReadAllText`/`WriteAllText` to apply the large context.md replacements; the resulting `context.md` content matches the plan.
3. **Two gitignored scratch leftovers in `logs/`** — Fix-worthy. The files are not part of the intended deliverable and should be removed.
4. **Console glyph note** — Acceptable. No such note appears in the committed documentation; it was a working note only.

## Required fix (junior-safe)

Delete the two scratch files left in the gitignored `logs/` directory. They are not tracked, so no `git` staging is involved.

```powershell
Remove-Item -LiteralPath "C:\repo\img-scroller\logs\perm-test.txt"
Remove-Item -LiteralPath "C:\repo\img-scroller\logs\w51t6_ctx.ps1"
```

Then verify the working tree remains clean (the removed files were gitignored, so `git status --short` should still report nothing):

```powershell
git status --short
```

Expected result: empty output.

No other action is required for this review.
