# Wave 8 — Task 1 (Wizard Installer) — Simplification Plan (4.3)

- Binding spec (UNCHANGED authority): [`.kilo/plans/20261118-wave8-task1-wizard-installer-implementation-plan.md`](20261118-wave8-task1-wizard-installer-implementation-plan.md) — decisions D1–D19 and the Step-4 pinned blocks. NOTHING below changes observable installer/build behavior.
- Reviewed artifacts (HEAD `2958249` on `feat/wave-8-wizard-installer`): `packaging/nsis/installer.nsi` (236 L), `scripts/build-installer.sh` (96 L), `Dockerfile` (nsis apt line — no simplification surface), `.gitattributes` (one line — none), `.agent/project-structure.md` (prose — none).
- Executor: the 4.3 implementer (JUNIOR, 50% restriction). Apply EXACTLY the edits below, in order, verbatim. No judgment calls, no additional refactors, NO commits (the caller commits after this step).
- Verification protocol for the implementer (after ALL edits, once): via `alpine-vm` MCP — `vm_status` first; then
  `sh -c "docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh"` — expect exit 0, line `build-installer: artifact: dist/ImagesScroller-1.1.0-windows-installer.exe`, line `build-installer: OK`.
  Any nonzero exit or missing typed line → STOP, revert the edits made, report to the caller. No Rust files changed → dev-checks NOT re-run (census 195/28 untouched per D18).

## Compliance findings (no action needed — recorded for audit)

- 17-macro invocation order matches `EXTENSION_LIST` and `src/folder_listing.rs` verbatim in ALL five blocks (declare/create/read/clean-install/clean-uninstall/write). ✓
- No `UserChoice` string anywhere. ✓ LF endings enforced (`attr/text eol=lf`). ✓ Pure ASCII. ✓
- No commented-out code: the single `;` line (install section, pre-clean block) is explanatory (plan-pinned). ✓
- No unreachable LogicLib branches: the `ASSOC_READ` `Else` (sets `"0"`) is REQUIRED — without it a Back→Next revisit would keep a stale `"1"` and re-register a deselected type. ✓
- Nesting: NSIS LogicLib depth 1 inside macros (inherent to the language, not forced-flat); shell functions depth ≤2; every sh function body ≤22 L. ✓
- `makensis_status=$?` capture pattern: CORRECT as implemented — do NOT "simplify" it back to the plan's literal `|| { rm -f; report_failure "…(exit $?)…" }` shape, whose `$?` would capture `rm`'s status (0), printing a wrong exit code. The implementation's deviation from the plan's one-liner is a correctness improvement; it stays.

---

## S1 — `packaging/nsis/installer.nsi`: reuse `${APP_KEY}` for the RegisteredApplications value data

- File: `packaging/nsis/installer.nsi`, line 58 (inside `WRITE_CAPABILITIES`).
- Exact change — from:

  ```nsis
    WriteRegStr HKCU "Software\RegisteredApplications" "${REG_APP_NAME}" "Software\Images Scroller\Capabilities"
  ```

  to:

  ```nsis
    WriteRegStr HKCU "Software\RegisteredApplications" "${REG_APP_NAME}" "${APP_KEY}\Capabilities"
  ```

- Why: the literal value data duplicates the `!define APP_KEY` string — the plan's own define-block mandate is "single sources of truth — no magic strings repeated" (its line-148 note lists the five defines used "instead of repeating literals"; this was the one missed usage). The macro expands to the byte-identical string `Software\Images Scroller\Capabilities`, so the D7 registry surface is unchanged.
- Risk: NONE at runtime (compile-time textual substitution; expanded string identical). Compile verification (protocol above) proves macro resolution.
- NOT part of this item: do NOT move the define block above the header and do NOT touch lines 2–10 (pinned header block) — see Rejected R6.

## S2 — `scripts/build-installer.sh`: collapse the triple `rm -f "$out_part"` cleanup into one helper

- File: `scripts/build-installer.sh`.
- Exact change 1 — insert this function immediately AFTER the `report_failure` function (after current line 26, keeping one blank line between functions):

  ```sh
  discard_part_and_fail() {
    rm -f "$out_part"
    report_failure "$1"
  }
  ```

- Exact change 2 — inside `compile_installer`, replace the three failure branches:

  ```sh
    if [ "$makensis_status" -ne 0 ]; then
      rm -f "$out_part"
      report_failure "makensis compile failed (exit $makensis_status); see the makensis diagnostics above"
    fi
    if [ ! -s "$out_part" ]; then
      rm -f "$out_part"
      report_failure "installer artifact missing or empty after compile: $out_part"
    fi
  ```

  and

  ```sh
    if [ "$move_status" -ne 0 ]; then
      rm -f "$out_part"
      report_failure "install of installer artifact failed: $out_part -> $out_final"
    fi
  ```

  with (message strings byte-identical, cleanup call replaced):

  ```sh
    if [ "$makensis_status" -ne 0 ]; then
      discard_part_and_fail "makensis compile failed (exit $makensis_status); see the makensis diagnostics above"
    fi
    if [ ! -s "$out_part" ]; then
      discard_part_and_fail "installer artifact missing or empty after compile: $out_part"
    fi
  ```

  and

  ```sh
    if [ "$move_status" -ne 0 ]; then
      discard_part_and_fail "install of installer artifact failed: $out_part -> $out_final"
    fi
  ```

- Why: the plan header pins "the transient .part file is removed on failure" as a single discipline, yet it is re-implemented at three sites — a future fourth failure path could silently forget the cleanup. One helper makes the invariant single-source. Behavior is an identical command sequence (`rm -f` then the typed `printf`/`exit 1`; `exit` inside the nested function still terminates the script; `$out_part`/`$makensis_status` are global in POSIX sh, so the interpolated messages expand identically).
- Risk: NONE — same execution order, same typed `build-installer: error:` texts, same exit codes. Final file stays ≤100 L, depth ≤2.
- Do NOT touch: `report_failure` itself, the message texts, any other function, or `package-windows.sh` (untouched per plan §8).

## S3 — `scripts/build-installer.sh`: use `${PROJECT_DIR}` for the four container-root literals in the `makensis` line

- File: `scripts/build-installer.sh`, line 62 (the single `makensis` command inside `compile_installer`).
- Exact change — from:

  ```sh
    makensis -V2 -DPRODUCT_VERSION="${package_version}" -DPRODUCT_VERSION_4="${version4}" -DINSTALLER_OUTFILE="/img-scroller/${out_part}" -DEXE_PAYLOAD="/img-scroller/${X64_PAYLOAD}" -DCONFIG_PAYLOAD="/img-scroller/${CONFIG_PAYLOAD}" -DLICENSE_PAYLOAD="/img-scroller/${LICENSE_PAYLOAD}" "$INSTALLER_SOURCE"
  ```

  to:

  ```sh
    makensis -V2 -DPRODUCT_VERSION="${package_version}" -DPRODUCT_VERSION_4="${version4}" -DINSTALLER_OUTFILE="${PROJECT_DIR}/${out_part}" -DEXE_PAYLOAD="${PROJECT_DIR}/${X64_PAYLOAD}" -DCONFIG_PAYLOAD="${PROJECT_DIR}/${CONFIG_PAYLOAD}" -DLICENSE_PAYLOAD="${PROJECT_DIR}/${LICENSE_PAYLOAD}" "$INSTALLER_SOURCE"
  ```

- Why: `/img-scroller` is already a named constant (`readonly PROJECT_DIR`) used by `assert_current_directory`; repeating it as four magic literals violates "avoid magic numbers/values". `${PROJECT_DIR}` expands to `/img-scroller`, so D19's requirement (absolute container-path defines) still holds exactly.
- Risk: NONE (constant equals the literal). If `PROJECT_DIR` ever changes, the defines follow it — strictly better consistency; no other behavior.
- Do NOT change: `-DEXE_PAYLOAD` still resolves to `/img-scroller/dist/windows-x64/img-scroller.exe`, etc. — verify by reading, do not restructure the line further (single command, one line, as now).

---

## Considered & REJECTED (do NOT implement — bound by D1–D19 / the pinned Step-4 structure)

- **R1 — remove the unused `!define EXTENSION_LIST`** (referenced exactly once in the file — dead at compile time): the plan's Step-4 define block pins it and the Hard-requirements paragraph uses it as the named anchor for the fixed 17-order audit. Removing it would weaken the documented order contract with zero behavioral gain. KEEP.
- **R2 — `!for`/`!foreach` loop conversion of the five 17-`!insertmacro` blocks** (~90 flat lines → 5 loops): explicitly forbidden by the pinned spec ("invoked 17× per consumer … no loops anywhere"); probes V4/V5 validated the flat form only; a version-gated preprocessor construct expands the compile-failure surface the plan's §4 failure table enumerates by per-macro name. TOO EXTENSIVE — recorded as a TODO-file candidate for the caller's decision only (would need an architector plan amendment), NOT scheduled here.
- **R3 — drop the `FileAssociations` half of the uninstall-section `ASSOC_CLEAN`** (made moot by the following whole-tree `DeleteRegKey HKCU "${APP_KEY}"`): D13 mandates the uninstall deletions 1:1 with the helper's `--unregister` (value deletes THEN tree deletes). The redundancy is a pinned decision. KEEP.
- **R4 — remove `Var Dialog`** (written by `Pop $Dialog`, never read): plan-pinned global-var block; harmless NSIS convention; removing it touches the pinned blocks for zero behavioral or readability gain. KEEP.
- **R5 — merge `assert_source_present`/`assert_payload_present`**: the two carry DISTINCT pinned failure texts with distinct recovery hints (rebuild-vs-produce-exe); separate self-documenting names beat one message-parameterized helper. KEEP.
- **R6 — reorder defines above the header** to dedupe the `"Software\Images Scroller"` literal on line 4 (`InstallDirRegKey`): the Step-4 header block is pinned verbatim (defines come after the includes); one string instance is not worth restructuring the pinned top of the file. KEEP.
- **R7 — merge `!define PRODUCT_NAME`/`!define REG_APP_NAME`** (same value `"Images Scroller"`): semantically distinct roles (UI display name vs registry key-name); both pinned. KEEP.

## Implementer prohibited list (in addition to the plan §8 list carried over)

- Apply ONLY S1/S2/S3. No renames, no reflows, no comment edits, no whitespace normalization beyond the exact blocks above.
- No changes to `Dockerfile`, `.gitattributes`, `.agent/project-structure.md`, `src/`, `Cargo.toml`, `scripts/package-windows.sh`, `scripts/build-windows.sh`, `packaging/register-default-viewer.cmd`, `packaging/DEFAULT-VIEWER-GUIDE.txt`, or any `.kilo/plans/*` file except this plan being followed.
- No `git` write commands (no add/commit/push) — the caller commits after the 4.3 verification passes.
