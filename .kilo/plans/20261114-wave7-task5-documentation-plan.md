# Wave 7 — Task 5 Implementation Plan: Documentation Updates (Distribution / Packaging / Helper / Build Matrix)

- Source TODO: `.agent/todos/20261029/20261029-todo-9.md` → `### Task 5: Documentation updates`. Verbatim: "`README.md`: distribution/installation/registration instructions. `tech.md`, `architecture.md`, `context.md`: record config format, packaging mechanism, helper mechanism, build matrix."
- Global plan (approved): `.kilo/plans/20261114-wave7-distribution-mvp.md` → `## Task 5 — Documentation updates` (README gains the distribution/install/registration section; tech.md gains the packaging/build-matrix + config-format records — the "Distribution and Packaging (Planned)" section becomes implemented; architecture.md updates the pipeline section + decisions table + "On disk" note; context.md gets the wave entry + current-state/next-steps refresh; coherence sweep: `.agent/project-structure.md`, `docs/build-windows.md`, `docs/build.md` only if harness semantics changed).
- Wave plans (landed — the fact sources this plan carries verbatim): `.kilo/plans/20261114-wave7-task1-config-plan.md` (config), `20261114-wave7-task2-installer-packaging-plan.md` (build extension + packaging), `20261114-wave7-task3-viewer-registration-plan.md` (helper), `20261114-wave7-task4-verification-plan.md` + `20261114-wave7-task4-verification-results.md` (verification evidence + the Task-5 carry-forward).
- Date: 2026-11-15. Status: FOR PLANNER PRESENTATION (pending user approval before 4.2 runs).
- Branch: `feat/wave-7-distribution` (created at workflow Step 2 — NOT this task's job). Version at execution: 1.0.0 (bumped at Step 3; this task NEVER edits version fields — it only updates documentation text that RECORDS the version).
- Front-end related: NO (Wave-level decision #3 — no 4.1a, no 4.5a).
- High-level approach: docs-only task. Six files edited (README.md, tech.md, architecture.md, context.md, docs/build-windows.md, docs/build.md), three verified with NO edits (.agent/project-structure.md, brief.md, product.md). ONE documentation commit that also sweeps the four pending uncommitted 4.1b architector ride-along entries in context.md. NO container runs, NO source changes, NO CHANGELOG.md (Task 6's exclusive file).

## 0. Settled Decisions (BINDING — the implementer must not re-choose)

Every decision below is final for Task 5. The documentation-fact discipline applies to every sentence: every number, path, name, and behavior comes from the landed plans/results VERBATIM — never invented, never rounded.

### D1 — README.md changes (exact)

**D1.1 — Technology Stack table, Language row (line 53).** Replace the whole row:

```
| Language | Rust | Implemented: Cargo package `img-scroller` 0.6.2, edition 2024. |
```

with:

```
| Language | Rust | Implemented: Cargo package `img-scroller` 1.0.0, edition 2024. |
```

**D1.2 — Technology Stack table, Target architectures row (line 57).** Replace the whole row:

```
| Target architectures | x64 and x86 builds | x64 cross-build landed (Wave 1); x86 belongs to Wave 7. |
```

with:

```
| Target architectures | x64 and x86 builds | Both landed: x64 (Wave 1, `dist/windows-x64/`) and x86 (Wave 7, `i686-pc-windows-gnu`, `dist/windows-i686/`). |
```

**D1.3 — Technology Stack intro sentence (line 49).** The sentence lists the landed slices and ends `...and the Wave-6.2 shell-parseable launch-path fix (the diagnostic-decided launcher correction) — all seven v1 features are landed;`. Insert `, and the Wave-7 distribution/packaging slice` immediately after `(the diagnostic-decided launcher correction)` and before ` — all seven v1 features are landed`. Nothing else in the sentence changes.

**D1.4 — `## Distribution (Planned)` section (lines 61–65) REPLACED in place** by the following section (same position; the heading changes; keep one blank line before and after):

```markdown
## Distribution and Installation

Images Scroller is distributed as a portable zip package — no installer, no self-extracting exe. ONE zip per architecture is produced by the container packaging script (the version is read from `Cargo.toml`; the current release is 1.0.0):

- 64-bit Windows: `ImagesScroller-1.0.0-windows-x64.zip`
- 32-bit Windows: `ImagesScroller-1.0.0-windows-x86.zip`

Each zip contains EXACTLY four files at its root (flat, no subfolders): the application executable `img-scroller.exe`, the application config file `img-scroller.toml`, the default-viewer registration helper `register-default-viewer.cmd`, and the user guidance `DEFAULT-VIEWER-GUIDE.txt`.

Official packages are published in the GitHub **Releases** section of the project (https://github.com/kazuyakuza/img-scroller/releases); packages can also be built from source with the container harness (see [Build and Run](#build-and-run) below and [docs/build-windows.md](docs/build-windows.md)).

### Install (the destination folder is your choice)

1. Download the zip that matches your Windows architecture (64-bit → `...-x64.zip`, 32-bit → `...-x86.zip`).
2. Right-click the zip → "Extract All..." → pick a NEW, EMPTY destination folder of your choice. That folder IS the install folder: the four files land at its root.
3. Double-click `img-scroller.exe` — the application opens (the window opens maximized; no console window appears).

The config file `img-scroller.toml` ships next to the executable; the current version does not read it (settings arrive in a future version), so there is nothing to configure in v1.

### Make Images Scroller the default image viewer (optional)

Windows 10/11 do not let any program set a default viewer silently — the final choice is always made by you in Settings. The packaged helper prepares that choice:

1. Double-click `register-default-viewer.cmd` INSIDE the extracted install folder. It registers Images Scroller for YOUR Windows user only (HKCU registry keys; no administrator rights), adds it to the "Open with" lists, makes it visible in Settings → Apps → Default apps, and opens that Settings page for you.
2. On the "Default apps" page, find **Images Scroller** and set it as the default for the file types you want (Windows 11: click the app in the list; Windows 10: use "Set defaults by app").

The helper covers the 17 image file types the app supports: `.avif`, `.bmp`, `.dib`, `.gif`, `.heic`, `.heif`, `.ico`, `.jfif`, `.jpe`, `.jpeg`, `.jpg`, `.jxr`, `.png`, `.tif`, `.tiff`, `.webp`, `.wdp`. Whether a given type can actually be decoded depends on the image codecs installed in your Windows system (Windows Imaging Component) — the same constraint the application itself has.

Notes: re-running the helper is safe (it writes the same registration values again); if you MOVE the install folder, run the helper once more so the registration points at the new location; to undo the registration, open a command prompt in the install folder and run `register-default-viewer.cmd --unregister` (it removes only the registry entries Images Scroller created — your files and other programs' registrations are untouched). The full packaged guide is `DEFAULT-VIEWER-GUIDE.txt` in the install folder.
```

**D1.5 — Project Status paragraph (line 69).** APPEND the following sentence at the very END of the paragraph (after its last sentence, before the newline; do not modify any earlier sentence):

```
Wave 7 landed (branch `feat/wave-7-distribution`, version 1.0.0 — the MVP checkpoint wave): the distribution landed — the application config file `packaging/img-scroller.toml` (header-comments-only TOML; the v1 binary does not read it), the x86 release target (`i686-pc-windows-gnu`; artifact `dist/windows-i686/img-scroller.exe`), the portable zip packaging (one flat zip per architecture: exe + config + the `register-default-viewer.cmd` helper + `DEFAULT-VIEWER-GUIDE.txt`), and the HKCU-only default-viewer registration helper that opens the Windows "Default apps" Settings page (the user makes the final per-type choice). Verification: dev checks `ALL CHECKS PASSED` (195 unit tests across 28 test-bearing files), both release builds exit 0, and both DLL import audits are set-equal to the recorded 22-DLL constant. Pending: the Wave-7 OPERATOR-RUN (the installer/registration flow on real Windows — the user's gate) and the first `CHANGELOG.md` MVP entry (Task 6).
```

**D1.6 — `## Build and Run` section.** Two edits:

(a) Replace the whole subsection from the heading `### Windows x64 release cross-build` (line 86) through the end of its paragraph (line 90) with:

```markdown
### Windows release cross-builds (x64 and x86)

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh

With NO argument the script builds BOTH release targets (x64 first, then x86); ONE optional argument builds exactly that target (`x86_64-pc-windows-gnu` or `i686-pc-windows-gnu`). Artifacts land at `dist/windows-x64/img-scroller.exe` (x64) and `dist/windows-i686/img-scroller.exe` (x86) — the `dist/` tree is gitignored. Typed failures, the per-target build matrix, and the static-linking import audit are documented in [docs/build-windows.md](docs/build-windows.md).

### Portable zip packaging

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh

Produces ONE flat zip per architecture from the built exes, the config file, the registration helper, and the guidance text: `dist/ImagesScroller-<version>-windows-x64.zip` and `dist/ImagesScroller-<version>-windows-x86.zip` (the version is read from `Cargo.toml`). Running the produced `.exe` — GUI behavior and image-format handling — is validated by the operator on a real Windows 10/11 machine; the Linux container cannot run the Windows GUI.
```

(b) The `### Container dev checks (no host Rust/Cargo)` subsection ABOVE it stays UNCHANGED (its semantics are untouched).

**D1.7 — README controls text.** NO other README change: the v1 Features, the controls table, the pill description, Overview, Problem Statement, Note for AI Agents, and License sections are untouched by Wave 7 (the GUI did not change).

### D2 — tech.md changes (exact)

**D2.1 — Contents list, item 17 (line 25).** Replace:

```
17. [Distribution and Packaging (Planned)](#distribution-and-packaging-planned)
```

with:

```
17. [Distribution and Packaging (Implemented, Wave 7)](#distribution-and-packaging-implemented-wave-7)
```

**D2.2 — `## Status` paragraph (line 5).** APPEND at the very end of the paragraph (do not modify any earlier sentence):

```
Landed for Wave 7 (branch `feat/wave-7-distribution`, version 1.0.0, 2026-11-14/15): the distribution — the application config file `packaging/img-scroller.toml` (header-comments-only TOML the v1 binary does not read), the x86 release target + the portable zip packaging, and the HKCU-only default-viewer registration helper (mechanisms: `## Distribution and Packaging (Implemented, Wave 7)` below).
```

**D2.3 — Technology Stack table.** Two row replacements:

(a) Language row (line 33): replace `Cargo package \`img-scroller\` version 0.6.2 (single binary; tracked \`Cargo.lock\`).` with `Cargo package \`img-scroller\` version 1.0.0 (single binary; tracked \`Cargo.lock\`).` — the ONLY version change in the row.

(b) Target architectures row (line 37): replace the whole row

```
| Target architectures | x64 and x86 builds | x64 (`x86_64-pc-windows-gnu`) cross-build landed in Wave 1; x86 belongs to Wave 7. |
```

with:

```
| Target architectures | x64 and x86 builds | Both landed: x64 (`x86_64-pc-windows-gnu`, Wave 1) and x86 (`i686-pc-windows-gnu`, Wave 7); artifacts under gitignored `dist/windows-x64/` and `dist/windows-i686/`. |
```

**D2.4 — `## Development Environment and Setup`, toolchain bullet (line 154).** Replace the bullet:

```
- The image also carries the MinGW-w64 x86-64 GNU cross toolchain (linker `x86_64-w64-mingw32-gcc`; `x86_64-w64-mingw32-objdump` backs the Windows import audit) and the rustup target `x86_64-pc-windows-gnu`.
```

with:

```
- The image also carries the MinGW-w64 x86-64 GNU cross toolchain (linker `x86_64-w64-mingw32-gcc`; `x86_64-w64-mingw32-objdump` backs the Windows import audit) and the rustup target `x86_64-pc-windows-gnu`. Since Wave 7 it also carries the MinGW-w64 i686 GNU cross toolchain (apt package `gcc-mingw-w64-i686` — linker `i686-w64-mingw32-gcc`; `i686-w64-mingw32-objdump` backs the x86 import audit), the rustup target `i686-pc-windows-gnu`, and the `zip`/`unzip` archive tooling that backs the packaging script.
```

**D2.5 — `## Build Environment (Container on the Alpine VM)` section.** Three edits:

(a) Harness-landing bullet (line 170): append at its end (nothing else in the bullet changes):

```
 Extended in Wave 7 (Task 2): the Dockerfile gained `gcc-mingw-w64-i686`, `zip`, `unzip`, and the `i686-pc-windows-gnu` target (an image rebuild was required after the change).
```

(b) Import-audit bullet (line 171): append at its end (nothing else in the bullet changes):

```
 Since Wave 7 the x86 artifact's OBSERVED list is recorded the same way — the `OBSERVED_DLL_IMPORT_LIST_I686` constant in `scripts/build-windows.sh` and in [docs/build-windows.md](../../docs/build-windows.md) — audited 2026-11-15 with `i686-w64-mingw32-objdump`, set-equal to the 22-name x64 list.
```

(c) NEW bullet appended at the END of the section (after the Reference-pattern bullet, line 172):

```
- Packaging (DONE, Wave 7 Task 2): `scripts/package-windows.sh` produces ONE flat zip per architecture (`ImagesScroller-<version>-windows-x64.zip` / `-x86.zip`, the version read dynamically from `Cargo.toml`) from the built exes plus the tracked `packaging/` files (the config file, the registration helper, the guidance text); output under gitignored `dist/`; the single-source content manifest is the script's `package_manifest_entries` function; the script never touches git.
```

**D2.6 — `## Build and Run` section (lines 174–179).** Replace the TWO bullets `- Windows x64 release: ...` (line 177) and `- x86 release: NOT built in Wave 1 — belongs to Wave 7 (extend \`scripts/build-windows.sh\` + add the i686 MinGW toolchain when realized).` (line 179) with the TWO bullets:

```
- Windows release builds: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` — NO argument builds BOTH targets (x64 first, then x86); ONE optional argument builds exactly `x86_64-pc-windows-gnu` or `i686-pc-windows-gnu`; artifacts at `dist/windows-x64/img-scroller.exe` (unchanged since Wave 1) and `dist/windows-i686/img-scroller.exe` (Wave 7); full guide (typed failures, the build matrix, the audit): [docs/build-windows.md](../../docs/build-windows.md).
- Portable zip packaging: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh` — ONE flat zip per architecture (`ImagesScroller-<version>-windows-x64.zip` / `-x86.zip`; the version read from `Cargo.toml`) under gitignored `dist/`.
```

(the `- Dev launch:` bullet between them stays byte-identical).

**D2.7 — `## Distribution and Packaging (Planned)` section (lines 197–200) REPLACED in place** by (keep the same position; heading changes):

```markdown
## Distribution and Packaging (Implemented, Wave 7)

- Application config file (Task 1): `packaging/img-scroller.toml` — TOML-style, UTF-8 without BOM, LF line endings, header comments ONLY (11 lines, ZERO keys); the v1 binary does NOT read it (no parser, no dependency — parsing arrives with the first real setting in a future version); the packaging copies it verbatim flat next to the exe; the per-folder last-viewed memory stays session-RAM-only and is intentionally never stored in this file.
- Build matrix (Task 2): x64 = `x86_64-pc-windows-gnu` (Wave 1) / x86 = `i686-pc-windows-gnu` (Wave 7); MinGW apt packages `gcc-mingw-w64-x86-64` / `gcc-mingw-w64-i686`; audit tools `x86_64-w64-mingw32-objdump` / `i686-w64-mingw32-objdump`; invocation `sh scripts/build-windows.sh` (NO argument = both targets, ONE argument = exactly that target); artifacts `dist/windows-x64/img-scroller.exe` / `dist/windows-i686/img-scroller.exe` (both gitignored); BOTH artifacts' OBSERVED DLL import lists are the SAME 22 system DLLs (set-equal; the x86 build is portable — NO MinGW runtime DLL; x86 audited 2026-11-15).
- Packaging mechanism (Task 2): the tracked POSIX-sh script `scripts/package-windows.sh`, run container-side with the in-image `zip`/`unzip` — ONE zip per architecture (a 32-bit Windows user cannot run the 64-bit exe and vice versa), FLAT layout (exactly 4 entries at the zip root: exe, config, helper, guidance), names `ImagesScroller-<version>-windows-x64.zip` / `ImagesScroller-<version>-windows-x86.zip` (the version read dynamically from `Cargo.toml`), output directly under gitignored `dist/`; the single-source content manifest is the script's `package_manifest_entries` function; typed failures use the `package-windows: error:` prefix; the script never touches git.
- Helper mechanism (Task 3): `packaging/register-default-viewer.cmd` — a Windows batch script (built-in reg.exe; NO `.ps1`, NO console executable) that registers the app HKCU-ONLY (no administrator rights; never touches `HKLM`; NEVER writes the hash-protected `UserChoice` key): the ProgID `ImagesScroller.Image`, per-extension OpenWithProgids + Capabilities FileAssociations for EXACTLY the app's 17 whitelisted extensions, the `HKCU\Software\Images Scroller\Capabilities` tree, and the `HKCU\Software\RegisteredApplications` pointer; it then opens `ms-settings:defaultapps` where the user makes the final per-type choice (Windows 10/11 protect defaults programmatically); `--unregister` deletes exactly the created entries; the portable script derives the exe from its own location (`%~dp0img-scroller.exe`); packaged guidance: `packaging/DEFAULT-VIEWER-GUIDE.txt` (plain text, Notepad-openable).
- Destination semantics (Task 2 D5): the zip is a PLAIN zip — the standard Windows "Extract All..." flow picks the destination folder, and that folder IS the install folder (no self-extracting exe, no installer UI in v1).
- Verification (Task 4): dev checks `ALL CHECKS PASSED` (195 unit tests / 28 test-bearing files — unchanged); both single-target builds exit 0; both DLL audits set-equal 22; one packaging re-run produced both 4-entry flat zips with CRLF helper copies (evidence: `.kilo/plans/20261114-wave7-task4-verification-results.md`).
```

**D2.8 — `## Testing` section.** Verify-only: the census statement (line 183, `195 unit tests ...`) is UNCHANGED by Wave 7 (no `src/` change — pinned by Task 4). NO edit.

**D2.9 — `## References`, build-windows line (line 216).** Replace:

```
- [`docs/build-windows.md`](../../docs/build-windows.md) — Windows x64 cross-build guide + OBSERVED DLL import list.
```

with:

```
- [`docs/build-windows.md`](../../docs/build-windows.md) — Windows x64/x86 cross-build + zip packaging guide + OBSERVED DLL import lists (both architectures).
```

### D3 — architecture.md changes (exact)

**D3.1 — `## Status` paragraph (line 5).** APPEND at the very end of the paragraph (do not modify any earlier sentence):

```
Wave 7 (branch `feat/wave-7-distribution`, version 1.0.0) landed the distribution slice: the shipped application config file (`packaging/img-scroller.toml`), the x86 release target, the portable zip packaging, and the default-viewer registration helper (see `## Build and Release Pipeline` and `## Data and State`).
```

**D3.2 — `## Logical Components`, Distribution bullet (line 40).** Replace the bullet:

```
- **Distribution Packaging — planned**: zip-style installer (destination-folder selection; unzips the app binary and config file) plus default-viewer registration helper/guidance — the helper mechanism is deferred.
```

with:

```
- **Distribution Packaging — LANDED (Wave 7)**: portable zip packaging (one flat zip per architecture from `scripts/package-windows.sh`: the exe + config + registration helper + guidance at the zip root; the version read from `Cargo.toml`; output under gitignored `dist/`), the shipped config file `img-scroller.toml` (header-comments-only TOML the v1 binary does not read), and the default-viewer registration helper `register-default-viewer.cmd` (HKCU-only registry registration for the app's 17 whitelisted extensions + the Capabilities/RegisteredApplications surface, then it opens the Windows "Default apps" Settings page — the user makes the final choice; `--unregister` removes exactly the created entries; packaged guidance `DEFAULT-VIEWER-GUIDE.txt`). The user picks the install destination via the standard zip-extract flow — that folder is the install folder.
```

**D3.3 — `## Data and State`, On-disk bullet (line 97).** Replace the bullet:

```
- **On disk (planned)**: an application **config file** shipped by the installer alongside the binary (brief distribution flow). Its contents/format are **not defined anywhere yet** — only its existence is recorded here; contents are an implementation decision.
```

with:

```
- **On disk (LANDED, Wave 7)**: the application **config file** `img-scroller.toml`, shipped flat next to the binary in every package (the brief's distribution flow). It is TOML-style, header comments ONLY (11 lines, zero keys), and the v1 binary does NOT read it — its v1 role is distribution presence; parsing arrives with the first real setting in a future version. The per-folder last-viewed memory is intentionally NEVER stored here (session RAM-only).
```

(the following bullet `- No other persistent state in v1; explicitly no persistence of the last-viewed memory.` stays byte-identical).

**D3.4 — `## Build and Release Pipeline` section.** Four edits:

(a) Intro paragraph (line 102): replace

```
Landed in Wave 1 (Task 1) and exercised in Task 5: dev checks ended `ALL CHECKS PASSED`; the Windows x64 cross-build exited 0 → a non-empty `dist/windows-x64/img-scroller.exe`. Guides: [docs/build.md](../../docs/build.md) and [docs/build-windows.md](../../docs/build-windows.md).
```

with:

```
Landed in Wave 1 (Task 1) and exercised in Task 5; extended in Wave 7 with the x86 target and the zip packaging. Dev checks ended `ALL CHECKS PASSED`; the x64 and x86 cross-builds both exit 0 → non-empty `dist/windows-x64/img-scroller.exe` and `dist/windows-i686/img-scroller.exe`; the packaging run produces both flat zips. Guides: [docs/build.md](../../docs/build.md) and [docs/build-windows.md](../../docs/build-windows.md).
```

(b) Windows-release bullet (line 105): replace the single bullet with TWO bullets:

```
- Windows release cross-builds: `scripts/build-windows.sh` runs `cargo build --release --locked --target <triple>` inside the container — NO argument builds BOTH targets (`x86_64-pc-windows-gnu` first, then `i686-pc-windows-gnu`, Wave 7); ONE optional argument builds exactly that target — and installs each final exe atomically (`.part` copy → rename) at `dist/windows-x64/img-scroller.exe` (x64, unchanged since Wave 1) / `dist/windows-i686/img-scroller.exe` (x86, Wave 7); a failed run leaves the previous artifact untouched.
- Portable zip packaging (Wave 7): `scripts/package-windows.sh` stages each manifest entry under its exact zip entry name and builds ONE flat zip per architecture — `ImagesScroller-<version>-windows-x64.zip` / `ImagesScroller-<version>-windows-x86.zip` (the version read dynamically from `Cargo.toml`), each with EXACTLY 4 entries at the zip root (exe, config, `register-default-viewer.cmd`, `DEFAULT-VIEWER-GUIDE.txt`), installed part-then-rename under gitignored `dist/`; the single-source manifest is the script's `package_manifest_entries` function; typed failures use the `package-windows: error:` prefix; the script never touches git.
```

(c) Container-image bullet (line 106): append at its end (nothing else changes):

```
 Since Wave 7 the image also carries the MinGW-w64 i686 toolchain (`gcc-mingw-w64-i686`), the `i686-pc-windows-gnu` target, and `zip`/`unzip`.
```

(d) Import-audit bullet (line 107): replace

```
- Import-audit evidence: the OBSERVED 22-DLL import list of the produced exe is recorded once in `scripts/build-windows.sh` (`OBSERVED_DLL_IMPORT_LIST`) and in [docs/build-windows.md](../../docs/build-windows.md); not duplicated here.
```

with:

```
- Import-audit evidence: the OBSERVED 22-DLL import lists of BOTH produced exes are recorded once each in `scripts/build-windows.sh` (`OBSERVED_DLL_IMPORT_LIST` for x64, audited 2026-10-31; `OBSERVED_DLL_IMPORT_LIST_I686` for x86, audited 2026-11-15 — the two lists are SET-EQUAL) and in [docs/build-windows.md](../../docs/build-windows.md); not duplicated here.
```

**D3.5 — `## Project Paths`.** Two edits:

(a) Cargo bullet (line 113): replace `package \`img-scroller\` 0.6.2, edition 2024` with `package \`img-scroller\` 1.0.0, edition 2024` — the ONLY version change.

(b) Repository-layout bullet (line 114): after `guides in \`docs/\`,` insert `distribution artifacts in \`packaging/\` (the config file + the registration helper + the guidance text),` — so the clause reads `guides in \`docs/\`, distribution artifacts in \`packaging/\` (the config file + the registration helper + the guidance text), agent scaffolding in ...`. Nothing else in the bullet changes.

**D3.6 — `## Technical Decisions Summary` table.** Two row replacements:

(a) Target-architectures row (line 131): replace

```
| Target architectures: x64 + x86 | x64 landed (Wave 1); x86 planned (Wave 7) | `## Technology Stack` |
```

with:

```
| Target architectures: x64 + x86 | both landed (x64 Wave 1; x86 Wave 7) | `## Technology Stack` |
```

(b) Distribution row (line 143): replace

```
| Distribution: zip-style installer | planned | `## Distribution and Default App Registration` |
```

with:

```
| Distribution: portable zip per architecture (flat: exe + config + registration helper + guidance; the version read from `Cargo.toml`), the shipped config file (header-only TOML unread by v1), the HKCU-only default-viewer registration helper + packaged guidance | landed (Wave 7) | `## Distribution and Default App Registration` |
```

**D3.7 — `## Open Architectural Decisions` (lines 145–151).** The TWO decided items are REMOVED; exactly ONE bullet remains. Replace the list body with:

```
- Design patterns beyond immediate-mode.
```

(the intro line `Each item below is to be decided during implementation, NOT now:` stays). The removed items: `- Distribution-helper mechanism (script vs. executable).` (decided in Task 3: the `.cmd` batch script) and `- Config file contents/format.` (decided in Task 1: header-only TOML) — both are now recorded as landed facts in `## Technical Decisions Summary` and in tech.md's implemented Distribution-and-Packaging section.

### D4 — context.md changes (exact; includes the ride-along sweep)

The working tree currently carries EXACTLY ONE unstaged modification: `.agent/project-info/context.md` (+4 inserted lines = the four Wave-7 4.1b architector Recent-Changes entries, dated 2026-11-14/15, verified via `git diff` 2026-11-15). DECISION (the caller's directive): those four ride-along entries are KEPT VERBATIM as the historical log and are COMMITTED by this task's documentation commit — they are NOT duplicated or rewritten. The Task-5 edits ADD on top:

**D4.1 — `## Current Work Focus` first entry REPLACED.** The current first entry begins `Dated 2026-11-13. **The 0.6.2 gate is CLOSED ...` and runs to the end of that dated block (the `Dated 2026-11-12.` Wave-5.2 entry below it stays). Replace the ENTIRE 2026-11-13 first entry with:

```
Dated 2026-11-15. **Wave 7 (`.agent/todos/20261029/20261029-todo-9.md`) is agent-side COMPLETE through Task 5 documentation, PENDING the user's Task-4 OPERATOR-RUN gate, Task 6 (the first `CHANGELOG.md` MVP entry — the wave's FINAL task), and global-plan Step 5; branch `feat/wave-7-distribution`, version 1.0.0, NOT yet merged.** What landed: (Task 1) the application config file `packaging/img-scroller.toml` (TOML-style, 11 header-comment lines, ZERO keys; the v1 binary does NOT read it; `.gitattributes` pin `packaging/*.toml text eol=lf`). (Task 2) the x86 release target — the Dockerfile gained `gcc-mingw-w64-i686` + `zip`/`unzip` + the `i686-pc-windows-gnu` target; `scripts/build-windows.sh` extended to a target-parameterized loop (NO argument = both targets, ONE argument = exactly that target; typed `build-windows: error:` discipline; x64 artifact path UNCHANGED `dist/windows-x64/img-scroller.exe`, x86 `dist/windows-i686/img-scroller.exe`); the NEW `scripts/package-windows.sh` produces ONE flat zip per architecture (`ImagesScroller-<version>-windows-{x64,x86}.zip`, the version read dynamically from `Cargo.toml`) with the single-source `package_manifest_entries` manifest. (Task 3) the default-viewer registration helper `packaging/register-default-viewer.cmd` (HKCU-only batch script: the ProgID, 17 per-extension OpenWithProgids, Capabilities FileAssociations, RegisteredApplications pointer; opens `ms-settings:defaultapps`; NEVER writes `UserChoice`; `--unregister` mode) + the packaged guidance `packaging/DEFAULT-VIEWER-GUIDE.txt` (+ the `.gitattributes` rules `packaging/*.txt text eol=lf`, `packaging/*.cmd text eol=crlf`). (Task 4, VERIFIED) dev-checks `ALL CHECKS PASSED` (195/28 census unchanged); both single-target builds exit 0 (x64 exe 19,649,092 B; x86 exe 19,802,972 B); BOTH DLL import audits are the SAME 22-name set (set-equal; the x86 build is portable — NO MinGW runtime DLL; the OBSERVED x86 list filled into `scripts/build-windows.sh` as `OBSERVED_DLL_IMPORT_LIST_I686`, the Wave-1 fill-at-verification precedent); one packaging re-run produced both 4-entry flat zips with CRLF helper copies (byte sizes recorded in `.kilo/plans/20261114-wave7-task4-verification-results.md`). (Task 5, this step) the wave documentation landed: README (the Distribution-and-Installation section + status/stack/build-and-run updates), tech.md (the implemented Distribution-and-Packaging section + the build-matrix/config/helper records + the Build-Environment/Build-and-Run x86+packaging commands), architecture.md (the pipeline/decisions-table/Data-and-State updates; the two decided Open-Architectural-Decisions items removed), this file, docs/build-windows.md (the x64+x86 guide + the x86 OBSERVED list + the packaging section), docs/build.md (the image-contents accuracy fix). The pending gate: the Task-4 OPERATOR-RUN (the PLAIN checklist at `.kilo/plans/20261114-wave7-task4-verification-plan.md` §8) — the user's run; the TODO's manual-UX sub-item stays `[ ]`. NEXT: Task 6 (the first `CHANGELOG.md` entry — the wave's FINAL task; its own 4.1b architector plan comes after this task's cycle closes), then global-plan Step 5.
```

**D4.2 — `## Recent Changes`: ONE new entry PREPENDED at the top** (above the existing `1. 2026-11-15: Wave 7 Task 4 step 4.1b...` entry, which stays verbatim as history):

```
1. 2026-11-15: **Wave 7 Task 5 documentation landed (this step) — the distribution/config/packaging/helper/build-matrix records are in README.md, tech.md, architecture.md, docs/build-windows.md, and docs/build.md; the four pending 4.1b architector ride-along entries below are swept and committed with this task's docs commit.** Branch `feat/wave-7-distribution`, version 1.0.0. Commits (verbatim): `docs: add wave7 task5 documentation plan` (the plan file), then `docs: wave7 distribution packaging documentation` (the six edited files). What landed: (README.md) the `## Distribution (Planned)` section REPLACED by `## Distribution and Installation` (the portable zip per architecture, the 4-file flat content, the Releases/build-from-source pointer, the extract-destination install steps, the optional default-viewer registration flow with the helper + Settings steps + the 17 types + the WIC codec constraint + the `DEFAULT-VIEWER-GUIDE.txt` reference); the Technology Stack Language row 0.6.2 → 1.0.0 and the Target-architectures row updated (both landed); the Build-and-Run section gained the both/single-target build contract and the packaging subsection; the Project Status paragraph gained the Wave-7 landed sentence. (tech.md) `## Distribution and Packaging (Planned)` became `## Distribution and Packaging (Implemented, Wave 7)` with the config/build-matrix/packaging/helper/destination-semantics/verification records; the Build-Environment section gained the i686 toolchain/target + zip/unzip, the second audit constant, and the packaging bullet; Build and Run gained the x86 + packaging commands (the stale "NOT built in Wave 1" bullet removed); the Technology Stack rows updated; the References build-windows line updated; the Testing census UNCHANGED (195/28 — verify-only). (architecture.md) the Build-and-Release-Pipeline section extended (x86 realized + the packaging bullet + zip names); the Data-and-State "On disk (planned)" bullet became the LANDED config-file record; the Distribution decisions row → landed (Wave 7) with the mechanism summary; the Target-architectures row → both landed; the TWO decided Open-Architectural-Decisions items (distribution-helper mechanism; config file contents/format) REMOVED (only "Design patterns beyond immediate-mode." remains); the Logical-Components Distribution bullet → LANDED; the Project-Paths version line 0.6.2 → 1.0.0 + the packaging/ clause; the Status paragraph gained the Wave-7 sentence. (docs/build-windows.md) the guide retitled to x64/x86 + zip packaging: the invocation contract (no-arg = both; one arg = exactly that target), both artifact paths, the extended typed-failure table (the unknown-target and argument-count rows), BOTH OBSERVED DLL import lists (x64 audited 2026-10-31; x86 audited 2026-11-15 with `i686-w64-mingw32-objdump` — set-equal 22), and the NEW portable-zip-packaging section (the Task-4 carry-forward items (a)+(b) landed). (docs/build.md) the one-accuracy fix: the image-contents sentence now also names the i686 toolchain/target and zip/unzip. (context.md) this entry + the Current-Work-Focus/Current-Project-State/Immediate-Next-Steps rewrite; the four 4.1b ride-along entries committed verbatim. Verify-only with NO edits: `.agent/project-structure.md` (CURRENT — the packaging/, scripts/, and .gitattributes bullets already reflect the wave), `brief.md` (the distribution section governs; no binding correction needed), `product.md` (historical greenfield overview; the standing precedent leaves it untouched). NEXT: the 4.3–4.6 cycle, then Task 6 (CHANGELOG).
```

**D4.3 — `## Current Project State`: ONE new Wave-7 bullet PREPENDED** at the top of the bullet list (above the existing `- **Wave 6.2 LANDED on \`main\`...` bullet, which stays):

```
- **Wave 7 IN PROGRESS on the branch `feat/wave-7-distribution` (Tasks 1–5 landed; Task 6 + Step 5 pending; NOT merged)**: version 1.0.0; the config file `packaging/img-scroller.toml` (TOML-style, 11 header-comment lines, zero keys; the v1 binary does not read it); the x86 release target (artifact `dist/windows-i686/img-scroller.exe`, 19,802,972 bytes) alongside the x64 artifact `dist/windows-x64/img-scroller.exe` (19,649,092 bytes, the 0.6.2-era artifact overwritten by the 1.0.0 rebuild); the packaging script `scripts/package-windows.sh` producing `dist/ImagesScroller-1.0.0-windows-x64.zip` (6,711,458 bytes) and `dist/ImagesScroller-1.0.0-windows-x86.zip` (7,160,311 bytes) — each zip EXACTLY 4 flat entries (exe, config, `register-default-viewer.cmd`, `DEFAULT-VIEWER-GUIDE.txt`); the HKCU-only registration helper + packaged guidance; census 195 unit tests / 28 test-bearing files (UNCHANGED — no pure packaging/config logic in `src/`); dev-checks `ALL CHECKS PASSED`; BOTH DLL import audits set-equal to the SAME 22-DLL list (the x86 OBSERVED list filled into the script constant `OBSERVED_DLL_IMPORT_LIST_I686` and recorded in docs/build-windows.md); Task-4 verdict VERIFIED (`.kilo/plans/20261114-wave7-task4-verification-results.md`); the pending gate = the Task-4 OPERATOR-RUN (the PLAIN checklist at the task-4 plan §8 — the user's run); then Task 6 (the first CHANGELOG entry, the wave's FINAL task) and Step 5.
```

**D4.4 — `## Current Project State`, Release-artifact bullet (the line beginning `- Release artifact: gitignored \`dist/windows-x64/img-scroller.exe\` (19,649,107 bytes, 0.6.2 Wave-6.2 rebuild...`).** Replace the WHOLE bullet with:

```
- Release artifacts (Wave 7, 1.0.0): gitignored `dist/windows-x64/img-scroller.exe` (19,649,092 bytes) and `dist/windows-i686/img-scroller.exe` (19,802,972 bytes), plus the zips `dist/ImagesScroller-1.0.0-windows-x64.zip` (6,711,458 bytes) and `dist/ImagesScroller-1.0.0-windows-x86.zip` (7,160,311 bytes) — the exes and zips are NEVER committed (`dist/` is gitignored); import audit: BOTH artifacts import the SAME 22 system DLLs (set-equal; the x64 list audited 2026-10-31, the x86 list audited 2026-11-15 — recorded in the `scripts/build-windows.sh` constants + `docs/build-windows.md`); the gitignored diagnostic exe + log remain in `dist/windows-x64/` until the wave's Step-5 tmp sweep.
```

**D4.5 — `## Current Project State`, TODO-state bullet (the line beginning `- **TODO state**: **Wave 6.2**: ...`).** Insert the Wave-7 sentence between `**TODO state**:` and `**Wave 6.2**:` (nothing else in the bullet changes):

```
**Wave 7 (IN PROGRESS)**: `.agent/todos/20261029/20261029-todo-9.md` — Tasks 1–4 marked `[DONE]` (the Task-4 manual-UX OPERATOR-RUN sub-item STAYS `[ ]` — the user's gate); Task 5 marked `[DONE]` at this step's 4.6; Task 6 (the first CHANGELOG entry — the wave's FINAL task) is pending its own 4.x cycle.
```

**D4.6 — `## Immediate Next Steps`: items 1–3 REPLACED** with:

```
1. **Pending Wave-7 OPERATOR-RUN gate — the user's run on a real Windows 10/11 machine** — the PLAIN self-contained checklist, verbatim at `.kilo/plans/20261114-wave7-task4-verification-plan.md` §8: extract `ImagesScroller-1.0.0-windows-x64.zip` (or the x86 zip) into a NEW EMPTY destination folder of your choice; confirm the 4 flat files; double-click `img-scroller.exe` (launches maximized, no console); run the Waves-1–6 navigation regression from the installed location; try the "Open with..." menu; double-click `register-default-viewer.cmd` (console message + the Settings "Default apps" page opens); set Images Scroller as the default for one type (e.g. `.png`) and open a file of that type from Explorer; check the "Open with" context-menu list; open `DEFAULT-VIEWER-GUIDE.txt`; optionally run `register-default-viewer.cmd --unregister`. STANDING NOTE: report the result back (any FAIL means the wave is NOT done); the TODO's Task-4 manual-UX sub-item stays `[ ]` until your report closes it.
2. **Task 6 — MVP checkpoint: the FIRST `CHANGELOG.md` entry (the wave's FINAL task)** — per user decision 2026-10-29, MVP = end of Wave 7 = full v1; this is the ONLY wave allowed to write the first `CHANGELOG.md` entry, and it does so only after Tasks 1–5 are ALL `[DONE]` in the TODO file (the gate is the `[DONE]` task HEADINGS, which stand once Task 5's 4.6 lands — the user-gated manual-UX sub-item is not the gate); the entry summarizes v1.0.0 content (the 7 features + distribution) in the existing Keep-a-Changelog structure; the entry text is written at execution by Task 6's own 4.1b architector plan (the execution order used in this wave: one 4.1b → 4.2 → 4.3 → 4.4 → 4.5b → 4.6 cycle per task, in file order; Task 6's plan file is NEW, never an edit of an existing plan).
3. **Then global-plan Step 5** — the TODO renamed `20261029-todo-9-DONE.md` (preserving content), the tmp sweep (the gitignored diagnostic exe/log in `dist/windows-x64/` — the wave closes, cleanup per the planner; the zips/exes are gitignored artifacts, never committed), checkout `main`, fast-forward merge `feat/wave-7-distribution`, delete the branch after the verified merge, push `main` → `origin` ONLY.
```

### D5 — docs/build-windows.md changes (exact)

**D5.1 — Title (line 1).** Replace:

```
# Windows x64 Release Cross-Build (Container)
```

with:

```
# Windows x64/x86 Release Cross-Build and Zip Packaging (Container)
```

**D5.2 — Intro paragraph (lines 3–5).** Replace:

```
The release `.exe` for Windows x64 is cross-compiled **inside the container**: target
`x86_64-pc-windows-gnu` with the MinGW-w64 x64 GNU toolchain. ONLY 64-bit Windows is covered here —
32-bit (x86) belongs to Wave 7 if realized.
```

with:

```
The release `.exe` for Windows x64 AND x86 is cross-compiled **inside the container**: targets
`x86_64-pc-windows-gnu` (MinGW-w64 x64 toolchain) and `i686-pc-windows-gnu` (MinGW-w64 i686 toolchain,
Wave 7). This page covers both architectures and the portable zip packaging.
```

**D5.3 — Prerequisites, first bullet (line 9).** Replace:

```
- The container image is built once (includes the MinGW-w64 x64 linker and the windows-gnu target) — see [`build.md`](build.md).
```

with:

```
- The container image is built once (includes both MinGW-w64 linkers — x64 and i686 — both windows-gnu targets, and `zip`/`unzip` for the packaging) — see [`build.md`](build.md).
```

**D5.4 — `## Invocation` section (lines 13–15).** Replace the single command block with:

```
    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh

With NO argument the script builds BOTH targets (x64 first, then x86). ONE optional argument builds
exactly that target:

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu
    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu

More than one argument, or an unknown target, is a typed failure (see the table below).
```

**D5.5 — `## Expected output path` section (lines 17–22).** Replace with:

```
    dist/windows-x64/img-scroller.exe   (x64)
    dist/windows-i686/img-scroller.exe  (x86, Wave 7)

The repo root `dist/` tree is gitignored; on the Windows host the paths appear at the repo root under
`dist\windows-x64\` and `dist\windows-i686\`.
```

**D5.6 — `## Overwrite / rebuild behavior` (lines 24–29).** Unchanged in substance; make ONE wording fix so it reads per-target: replace the first bullet's opening `- A successful rebuild atomically overwrites the previous exe` with `- Per target, a successful rebuild atomically overwrites the previous exe`. The remaining bullets stay.

**D5.7 — `## Typed failure table` (lines 31–44).** Two edits:

(a) After the intro sentence (`Every failure line starts with the literal prefix ... exits 1.`), insert ONE clarifying line and generalize the artifact-path examples: replace the literal occurrences of `dist/windows-x64` inside the last four rows' message examples with `<dist dir>`:

```
`<dist dir>` is `dist/windows-x64` for the x64 target and `dist/windows-i686` for the x86 target.
```

(b) APPEND these two rows at the end of the table:

```
| unknown target argument | `build-windows: error: unknown target: <arg> (expected x86_64-pc-windows-gnu, i686-pc-windows-gnu, or no argument for both)` |
| more than one argument | `build-windows: error: expected at most 1 argument, got <N>` |
```

**D5.8 — `## Toolchain / target notes` (lines 46–49).** Replace the section body with:

```
- Both GNU cross targets and their linkers are baked into the pinned image: `x86_64-pc-windows-gnu`
  (linker `x86_64-w64-mingw32-gcc`) and `i686-pc-windows-gnu` (linker `i686-w64-mingw32-gcc`, Wave 7);
  no local toolchain setup exists on any host.
- The build runs `cargo build --release --locked --target <triple>` per target; the GNU runtime
  components are statically linked into each produced exe (no extra runtime DLLs distributed with the
  app — corroborated by the import audit below for BOTH artifacts).
```

**D5.9 — `## Static-linking / import audit (retained scope)` (lines 51–62).** Replace the intro paragraph and the observed list with the BOTH-architecture record (this is the Task-4 carry-forward item (a)):

```
The build script does NOT run the import audit itself. Once an exe is produced, the verification
session runs ONE import audit per artifact with the matching objdump tool (`x86_64-w64-mingw32-objdump`
for x64, `i686-w64-mingw32-objdump` for x86) and records the OBSERVED DLL list here and in the
script's per-target constants (`OBSERVED_DLL_IMPORT_LIST` / `OBSERVED_DLL_IMPORT_LIST_I686`). The
lists are observed evidence — never copied from any reference project.

Observed DLL import lists — BOTH artifacts import the SAME 22 system DLLs (set-equal):

    advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll

    x64: audited 2026-10-31 with `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` inside the container (Wave 1 verification; first produced exe). Re-audited 2026-11-15 on the 1.0.0 artifact (Wave 7 Task 4) — set-equal.
    x86: audited 2026-11-15 with `i686-w64-mingw32-objdump -x dist/windows-i686/img-scroller.exe` inside the container (Wave 7 Task 4) — set-equal to the x64 list; NO MinGW runtime DLL (`libgcc_s_dw2.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`) is imported, so the x86 build is portable. Lists are deduplicated and lowercase.
```

**D5.10 — NEW `## Portable zip packaging` section** inserted AFTER the import-audit section and BEFORE `## Validation stance`:

```
## Portable zip packaging

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh

Produces ONE flat zip per architecture from the built exes plus the tracked `packaging/` files:

    dist/ImagesScroller-<version>-windows-x64.zip
    dist/ImagesScroller-<version>-windows-x86.zip

- The version is read dynamically from `Cargo.toml` (at execution: 1.0.0).
- FLAT layout: each zip contains EXACTLY 4 entries at its root — `img-scroller.exe`,
  `img-scroller.toml`, `register-default-viewer.cmd`, `DEFAULT-VIEWER-GUIDE.txt` — no subfolders,
  no directory entries.
- ONE zip per architecture: a 32-bit Windows user cannot run the 64-bit exe and vice versa.
- The content manifest is the script's single-source `package_manifest_entries` function; the helper
  and guidance files join it as manifest lines (the Task-3 extension point).
- Output lives under the gitignored `dist/` tree and is NEVER committed; the script never touches git.
- Typed failures: every failure line starts with the literal prefix `package-windows: error:` and the
  script exits 1 (e.g. `package-windows: error: packaged file missing or empty: <path> (produce it
  with scripts/build-windows.sh first)`). A failed run never deletes or modifies a previously
  installed zip; all transient files live under `dist/package-staging/`, removed by the script's
  EXIT trap on every exit.
- Full semantics live in `scripts/package-windows.sh` itself (self-documenting script); this guide
  does not duplicate them.
```

**D5.11 — `## Validation stance` and `## Lockfile policy`.** Unchanged.

### D6 — docs/build.md: the ONE pinned accuracy edit

The harness SEMANTICS are unchanged (the Task-4 results file's carry-forward item (c): `docs/build.md` needs NO change) — but the image-contents sentence is now factually stale. ONE edit, the image-build paragraph (lines 17–20): after `...and the \`x86_64-pc-windows-gnu\` target.` insert (same paragraph, before `Rebuild only after a Dockerfile change;`):

```
Since Wave 7 the image also contains the MinGW-w64 i686 cross toolchain (linker + `i686-w64-mingw32-objdump`), the `i686-pc-windows-gnu` target, and the `zip`/`unzip` packaging tooling.
```

NO other build.md change (the packaging command does NOT belong here — this page explicitly documents the dev checks, not release builds; the packaging guide is docs/build-windows.md per D5.10).

### D7 — Coherence-sweep verdicts (verify-only; NO edits)

1. `.agent/project-structure.md` — VERIFIED CURRENT (2026-11-15 read): the `packaging/` bullet lists all three files with the helper's mechanism summary; the `scripts/` bullet covers dev-checks + x64/x86 cross-build + zip packaging; the `.gitattributes` bullet carries the full four-rule list. NO edit.
2. `brief.md` — VERIFIED: `## Distribution and Default App Registration` already governs the landed flow (its "helper implementation details deferred to implementation" clause describes the brief's deferral role, not a stale project claim — the decision was made where the brief pointed: the architector steps); `## Technology Stack` build-environment line ("x86 realized in Wave 7 packaging — decision date 2026-10-30") is ACCURATE. NO edit, NO addendum.
3. `product.md` — VERIFIED: it is the historical greenfield-era overview (it still says "no application code exists yet (as of 2026-10-29)"); the standing precedent across all waves leaves it untouched; nothing in it BINDINGLY contradicts the landed state for requirements purposes (brief.md is the source of truth). NO edit. (Flagged as a standing note, not a defect.)

### D8 — Commit plan

- ONE documentation commit carries ALL six edited files (and thereby sweeps the four context.md ride-along entries): message (verbatim) `docs: wave7 distribution packaging documentation`.
- The plan file is committed FIRST (Step 0 below): `docs: add wave7 task5 documentation plan`.
- 4.6 marks the TODO (§7): `task: mark wave7 task5 done`.
- NO container runs are needed (docs-only task — the 4.5b architector check verifies; this recommendation is binding: no `alpine-vm` call is part of 4.2).

## 1. Environment Facts (verified 2026-11-15)

- Git: on branch `feat/wave-7-distribution`; HEAD `e61d664` (`task: mark wave7 task4 done`); Tasks 1–4 all landed (chain `1050785`…`e61d664`); version 1.0.0. Working tree: EXACTLY ONE unstaged modification — `.agent/project-info/context.md` (+4 lines, the Wave-7 4.1b ride-along entries; verified via `git diff` 2026-11-15). NO untracked files.
- Version: `Cargo.toml` = 1.0.0 (never edited by this task).
- Docs state at plan time: README `## Distribution (Planned)` + `0.6.2` Language row + x64-only Build-and-Run; tech.md `## Distribution and Packaging (Planned)` + `0.6.2` + x64-only Build-and-Run + the stale "NOT built in Wave 1" x86 bullet; architecture.md `planned` Distribution row + `0.6.2` Project-Paths line + the two open decisions + "On disk (planned)"; docs/build-windows.md x64-only title/intro/invocation/path/table/toolchain/audit; docs/build.md image-contents sentence x64-only; context.md Current-Work-Focus/Current-Project-State/Immediate-Next-Steps still Wave-6.2-era.
- Verification evidence available for the docs (verbatim from `.kilo/plans/20261114-wave7-task4-verification-results.md`): dev-checks `ALL CHECKS PASSED` exit 0, census 195/28; x64 exe 19,649,092 B; x86 exe 19,802,972 B; zips 6,711,458 B / 7,160,311 B; both audits set-equal 22 (x86 portable); script fill `OBSERVED_DLL_IMPORT_LIST_I686` landed; packaging re-run exit 0 with the three pinned output lines; the four 4-entry flat zip listings; both packaged-.cmd CRLF checks exit 0.
- Alpine VM: NOT needed for this task (docs-only). If any container verification is ever requested by the caller, use the `alpine-vm` MCP (`vm_status` first; exit code authoritative) — but the binding plan has none.
- Census pinned: 195 unit tests / 28 test-bearing files (breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file modules) — UNCHANGED; the docs record it unchanged.

## 2. Binding Constraints (apply to every step)

- NO `git push` (restricted to Step 5). NO branch creation/switch (restricted to Step 2). NO version edit (`Cargo.toml`/`Cargo.lock` NEVER touched — the docs RECORD 1.0.0 as text, which is not a version edit).
- NO edit to `CHANGELOG.md` (Task 6's exclusive file). NO edit to anything under `src/`. NO edit to any existing `.kilo/plans/*` file — this plan is the only NEW plan file of this task.
- NO edit to `Dockerfile`, `docker-compose.yml`, `scripts/*`, `packaging/*` (their records are DOCUMENTED here, not changed).
- Gitignore Compliance Rule before EVERY commit: read `.gitignore`, run `git status`, ensure NO `dist/`, `logs/`, `target/`, or `*.log` path is staged.
- File edits use real newline characters (never literal `\n` escape sequences) — newline-prevention rule. Markdown files have no `.gitattributes` line-ending pin — keep the files' existing line endings (a CRLF working tree on this Windows checkout is expected and correct for un-pinned files).
- The documentation-fact discipline: every number/path/name/behavior in the new text comes from the landed plans/results VERBATIM (byte sizes 19,649,092 / 19,802,972 / 6,711,458 / 7,160,311; census 195/28; 22-DLL set-equal; the 17-type list; the exact zip names; the exact registry-surface summary). NEVER invent or "improve" a number.
- Max-lines rules: not applicable (docs files, not `src/`).

## 3. Step-by-Step Implementation (workflow step 4.2 — implementer, JUNIOR under 50%)

Execute the steps in order. Check the plan between steps. Stop and ask the caller on ANY ambiguity — never guess. All edits are host-side file edits (read → edit tools); NO container command exists in this task.

### Step 0 — Commit the plan file (BEFORE implementation)

- `git status` → on branch `feat/wave-7-distribution`; the tree contains (at least) the new file `.kilo/plans/20261114-wave7-task5-documentation-plan.md` plus the pre-existing unstaged `context.md` ride-along (NOT this commit).
- Gitignore compliance: read `.gitignore`; run `git status`; confirm nothing gitignored is about to be staged.
- Stage ONLY the plan file: `git add .kilo/plans/20261114-wave7-task5-documentation-plan.md`
- Commit (verbatim message): `docs: add wave7 task5 documentation plan`

### Step 1 — README.md edits (D1)

Apply D1.1–D1.6 in order, each as an anchored edit (exact old string → exact new string). After D1.5, re-read the Project Status paragraph to confirm the appended sentence sits at the paragraph's end as ONE paragraph (a Wave-7 sentence inside the same paragraph, matching the file's existing style). After D1.6, confirm the dev-checks subsection above is untouched.

### Step 2 — tech.md edits (D2)

Apply D2.1–D2.7 and D2.9 in order. D2.8 is verify-only. The Contents anchor (D2.1) must match the NEW heading text of D2.7 exactly (`distribution-and-packaging-implemented-wave-7`).

### Step 3 — architecture.md edits (D3)

Apply D3.1–D3.7 in order. After D3.7, confirm the Open-Architectural-Decisions list has EXACTLY one bullet (`- Design patterns beyond immediate-mode.`).

### Step 4 — docs/build-windows.md edits (D5)

Apply D5.1–D5.10 in order. The new `## Portable zip packaging` section (D5.10) sits between the import-audit section and `## Validation stance`.

### Step 5 — docs/build.md edit (D6)

Apply the ONE pinned insertion (D6). Nothing else in the file changes.

### Step 6 — context.md edits (D4)

Apply D4.1–D4.6 in order. The four existing 4.1b Recent-Changes entries (Tasks 1–4) stay VERBATIM below the new D4.2 entry. After D4.6, confirm `## Immediate Next Steps` has exactly 3 items and no 0.6.2-era checklist remains at the top.

### Step 7 — Verify-only sweep (D7)

Re-read `.agent/project-structure.md`, `brief.md`, `product.md`. Confirm the D7 verdicts. NO edits. If a verdict cannot be confirmed (e.g. the structure map lost a bullet), STOP and return the finding to the caller.

### Step 8 — Gitignore compliance + the documentation commit

- `git status` → the staged/untracked set must contain NO `dist/`, `logs/`, `target/`, or `*.log` path. The staged set for this commit is EXACTLY: `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`, `docs/build-windows.md`, `docs/build.md`.
- Stage EXACTLY: `git add README.md .agent/project-info/tech.md .agent/project-info/architecture.md .agent/project-info/context.md docs/build-windows.md docs/build.md`
- Commit (verbatim message): `docs: wave7 distribution packaging documentation`
- Post-commit `git status`: clean (the ride-along is now committed as part of this commit; no unstaged files remain).

### Explicitly NOT done in Task 5 (for the implementer)

- NO `CHANGELOG.md` edit (Task 6 — even though the CHANGELOG is documentation, it is EXPLICITLY out of this task's scope). NO `src/` change, NO `Cargo.toml`/`Cargo.lock` edit, NO version-bump action. NO script/Dockerfile/packaging-file edit. NO `.agent/project-structure.md` / `brief.md` / `product.md` edit (verify-only). NO edits to existing `.kilo/plans/*`. NO container run, NO dev-checks run, NO build, NO packaging run. NO `git push`; NO branch/version actions. NO TODO-file content change beyond the 4.6 mark (the overwrite-TODO-prevention rule: 4.6 toggles ONLY Task 5's heading + its single sub-item checkbox).

## 4. Workflow step 4.3 — Code Review & Simplification guidance

- code-reviewer checklist (deviation from plan = a finding). Verify against the landed diff (`git diff <Step-0-commit>..<docs commit>`):
  1. File set EXACTLY the six files of §3 Step 8 — nothing else changed vs the Step-0 baseline (in particular: NOT `CHANGELOG.md`, NOT `src/`, NOT `Cargo.toml`/`Cargo.lock`, NOT `scripts/`, NOT `packaging/`, NOT any existing `.kilo/plans/*`, NOT `.agent/project-structure.md`/`brief.md`/`product.md`).
  2. README: the D1.4 section replaced `## Distribution (Planned)` in place with the pinned heading/content; no other README section touched except D1.1–D1.3/D1.5/D1.6; the 17-type list matches `src/folder_listing.rs` `IMAGE_FILE_EXTENSIONS` (avif bmp dib gif heic heif ico jfif jpe jpeg jpg jxr png tif tiff webp wdp); the byte sizes and zip names match the results file; NO invented fact.
  3. tech.md: the D2.7 section heading + Contents anchor match; the census text unchanged; the stale "NOT built in Wave 1" bullet GONE; the D2.6 two-bullet replacement exact.
  4. architecture.md: the Open-Architectural-Decisions list = exactly ONE bullet; the Distribution decisions row says landed (Wave 7); the Data-and-State bullet says LANDED with the unread-by-v1 fact; the D3.4(b) packaging bullet present.
  5. docs/build-windows.md: both OBSERVED lists recorded with BOTH audit dates and tools; the two new typed-failure rows match the script's actual failure strings (`build-windows: error: unknown target: ...` / `build-windows: error: expected at most 1 argument, got N`); the packaging section present.
  6. docs/build.md: EXACTLY the one insertion; no semantics text changed.
  7. context.md: the four 4.1b entries present VERBATIM (untouched text); the new D4 entries in place; the 0.6.2-era Next-Steps checklist REMOVED (replaced by D4.6).
  8. Fact discipline: grep the diff for invented numbers — every byte size, census number, and audit count matches the landed results file.
- code-simplifier: documentation prose — expected outcome "not required" (a clear message is the deliverable). If wording tightening is proposed, it must NOT alter any pinned fact, number, path, or name without a plan amendment.
- Max 3 review cycles, then escalate (workflow rule).

## 5. Workflow step 4.4 — Documentation guidance

- This task IS the wave's documentation task: the docs-specialist's expected outcome is "nothing further to document" — the four required record classes (config format, packaging mechanism, helper mechanism, build matrix) are covered by D1/D2/D3, and the user-facing guide text is the PACKAGED `DEFAULT-VIEWER-GUIDE.txt` (referenced, not rewritten — no duplicate maintenance per the Task-3 D8 decision). Creating any NEW documentation file is a plan deviation (the caller's recommendation stands: NO new `docs/package-windows.md`; one build/packaging guide).
- The Critical Closing Step (context.md recent changes → current state → next steps) is D4 itself — done inside this task, not deferred.

## 6. Workflow step 4.5b — Plan-adherence guidance

- Verify §0–§3 against the landed commits: the six-file diff matches D1–D6 section-by-section; the Step-0/Step-8 commits carry exactly their pinned files with the verbatim messages; gitignore compliance held; the ride-along sweep landed inside the docs commit (the +4 context.md lines are part of the committed file); NO prohibited file appears in either commit.
- NO container evidence is required (the plan prescribes none); the check is a diff/inspect check.
- If every element conforms: return a clear "no diffs, VERIFIED" message (no report file needed). If a deviation exists: record it in a NEW plan file `.kilo/plans/20261115-wave7-task5-adherence.md` and propose the disposition — never edit existing plans.

## 7. Workflow step 4.6 — Task completion (implementer)

- Edit `.agent/todos/20261029/20261029-todo-9.md` (preserve ALL other content byte-identical):
  - `### Task 5: Documentation updates` → `### Task 5: Documentation updates [DONE]`
  - Its single sub-item `- [ ]` → `- [x]` (keep the rest of the line verbatim — the README/tech/architecture/context instruction line).
- Gitignore compliance before the commit (`git status`; the TODO file is tracked — safe).
- Commit (verbatim message): `task: mark wave7 task5 done`

## 8. Risks / stop conditions

- Any plan ambiguity encountered by a sub-agent → STOP and return the question to the Planner; never guess.
- A verify-only file found stale beyond the D7 verdicts (e.g. the structure map missing a wave file) → STOP and report the finding; do not improvise an edit.
- Accidental touch of `CHANGELOG.md` or any existing `.kilo/plans/*` → revert immediately and report.
- The context.md ride-along must NOT be lost: the Step-8 commit stages `context.md` and thereby commits the four 4.1b entries; do NOT stage `context.md` in the Step-0 plan commit.
- Status-paragraph appends (D1.5/D2.2/D3.1) must not modify earlier sentences — a truncated or duplicated historical sentence is a review finding.
- Historical pending-gate narrations (e.g. the 0.6.1/0.6.2 OPERATOR-RUN mentions inside wave narrations) are NEVER edited by this task — they are dated history; only context.md's LIVE sections (Current Work Focus / Current Project State / Immediate Next Steps) carry the current gate.
- Review-cycle cap: max 3 (4.3) cycles, then escalate to the user.

## 9. Plan-vs-task compliance check

- TODO Task 5 requires: "`README.md`: distribution/installation/registration instructions" ✔ (D1.4: the Distribution-and-Installation section — download/extract/run, the optional registration flow, the 17 types + WIC constraint, the packaged-guide reference, the Releases/build-from-source pointer; D1.1–D1.3/D1.5/D1.6: the stale version/status/build rows fixed). "`tech.md` ... record config format, packaging mechanism, helper mechanism, build matrix" ✔ (D2.7: all four records; D2.4–D2.6: the build-env/build-and-run commands; D2.8: census verified unchanged). "`architecture.md`" ✔ (D3.2–D3.7: the pipeline, the on-disk config record, the decisions table, the two decided open items removed, the logical-components bullet landed). "`context.md`" ✔ (D4: the wave entry, the state bullet, the next-steps rewrite, the ride-along sweep decision). Plus the coherence sweep ✔ (D5 build-windows.md with the Task-4 carry-forward; D6 build.md judged-and-fixed minimally; D7 the three verify-only verdicts). Binding constraints honored: NO push, NO version edit, NO CHANGELOG (Task 6), NO `src/` changes, no existing-plan edits, facts carried verbatim from the landed plans/results. Nothing from Task 6 is executed (the CHANGELOG is untouched; Task 6 runs its own cycle after this task's 4.6). The plan is complete and correct against the task.



