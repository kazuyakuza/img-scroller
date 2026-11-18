# Wave 8 — Task 6 (Documentation Updates) — Implementation Plan (4.1b)

- Source TODO: [`.agent/todos/20261117/20261117-todo-1.md`](../../.agent/todos/20261117/20261117-todo-1.md) → `### Task 6: Documentation updates` (verbatim, binding):
  - "`README.md`: the installer-first installation instructions (replacing/adjusting the zip-extract flow), the GitHub Releases pointer, the hotkeys record (Task 3), and the window-title behavior. `tech.md`, `architecture.md`, `context.md`: the installer mechanism, the releases flow, the association-selection registry surface, the title mechanism, the build matrix."
  - "`brief.md`: the distribution model gains the user's 2026-11-17 wizard-installer decision as a BINDING addendum item (the wave's documentation step)."
  - "`CHANGELOG.md`: the new version's entry (post-MVP entries are allowed — the Wave-7-only restriction ended at the MVP; new entries go under `## [Unreleased]` and are released with the version bump)."
- Carried item (Task-5 4.5b adjudication, BINDING): `docs/build-windows.md` line 107 — replace the stale parenthetical `(at execution: 1.0.0)` with `(at execution: 1.1.0)` (the REST of the line stays verbatim).
- Executor: the 4.2 implementer (JUNIOR, 50% restriction). Every structural decision below is PINNED — nothing requires judgment.
- Front-end related: NO (no 4.1a, no 4.5a). Docs-only task: NO `src/` change, NO `Cargo.toml`/`Cargo.lock` change, NO packaging/scripts change, NO container/VM command, NO harness run (the Task-2 D7 precedent applies verbatim).
- Git state (verified 2026-11-18): branch `feat/wave-8-wizard-installer`, HEAD `e82e061` (`task: mark wave8 task5 done`), Tasks 1–5 landed and marked `[DONE]` (the Task-5 manual-UX sub-item stays `[ ]`), version 1.1.0 (bumped at `4fb6b94`), census 213 unit tests / 29 test-bearing files. This 4.1b step commits NOTHING and pushes NOTHING (commits = 4.2; push = workflow Step 5 ONLY, `main` → `origin`).

[Project Info: Active] — context files read independently (brief.md, tech.md, architecture.md, context.md, product.md, project-structure.md, WORKFLOWS.md, critical-workflow.md, README.md, docs/build-windows.md, CHANGELOG.md, Cargo.toml, src/window_title.rs, src/app/window_title.rs, the Task-1/Task-4 implementation plans, the Task-3 hotkeys results, the Task-5 verification results, the Task-2 doc-indexes plan, the Wave-7 Task-5 documentation plan).

---

## 0. Verified Fact Sources (the documentation-fact discipline)

Every number, path, name, and behavior written by this task comes VERBATIM from these landed records — never invented:

| Fact | Source (verified 2026-11-18) |
| --- | --- |
| Version 1.1.0 | `Cargo.toml` line 3 (`version = "1.1.0"`); bump commit `4fb6b94` |
| Installer mechanism (NSIS 3.08, container cross-build, artifact name, pages, HKCU scope, association page, registry surface, uninstaller) | Task-1 plan D1–D19 (`.kilo/plans/20261118-wave8-task1-wizard-installer-implementation-plan.md`) + `docs/build-windows.md` → `## Windows installer packaging (NSIS)` (Task 1's 4.4 wrote it) |
| Zips KEPT (4-entry manifest unchanged; the x86 zip stays the only 32-bit distribution) | Task-1 plan D4/D10/D12 |
| Census 213 unit tests / 29 test-bearing files (delta = `src/window_title/tests.rs`, 18 tests; 29 = 19 `*_tests.rs` + 5 `tests.rs` + 5 in-file) | `.kilo/plans/20261118-wave8-task5-verification-results.md` §1–§2; corroborated by `rg -c "#\[test\]" src/window_title/tests.rs` → 18 |
| Fresh artifact chain (x64 → x86 → installer → both zips, all exit 0) + byte sizes (installer 6,846,665 B; x64 zip 6,720,409 B; x86 zip 7,172,404 B; x64 exe 19,675,523 B; x86 exe 19,828,521 B) | Task-5 results §3, §5.3 |
| Both DLL import audits SET-EQUAL 22 (audited 2026-11-18 on the fresh artifacts) | Task-5 results §4 |
| Window-title mechanism (pixel measurement via egui fonts; the 0.85 fraction; the shell-parseable display form; the `...` literal; the composition ladder; the not-maximized rule; the popup constants) | Task-4 plan D1–D16 + `src/window_title.rs` + `src/app/window_title.rs` |
| Popup-refresh behavior (RE-ARMS on every navigation while fullscreen ON, showing the CURRENT image's name+ext; the plan's D10 "stable for the full 3 s" clause adjudicated SUPERSEDED by the TODO's "on every visible-image change" rule) | The doc comment at `src/app/window_title.rs:8–15` (the adjudicated record; the docs document the TODO-governed behavior) |
| Hotkeys verdict: COMPLETE, zero gaps, ZERO README edits to make (the Controls table + per-feature text stand as the record) | `.kilo/plans/20261118-wave8-task3-hotkeys-verification-results.md` §5 |
| Task-7 release mechanism (tagged `v1.1.0`; installer + zips attached; notes from the CHANGELOG entry; `gh` availability/auth verified FIRST; STOP-and-ask if agent-side publishing impossible; NOT YET EXECUTED) | TODO Task 7 + `.kilo/plans/20261118-wave8-wizard-installer-global-plan.md` |
| Task-2 TOC rolling trigger (any later task adding/removing/retitling a `##`/`###` heading in a file carrying `## Contents` MUST update that TOC in the SAME edit) | `.kilo/plans/20261118-wave8-task2-doc-indexes-implementation-plan.md` D5 |

---

## 1. Settled Decisions (each with rationale — NO open choices)

| # | Decision | Settled value | Rationale |
| --- | --- | --- | --- |
| D1 | Files edited (EXACTLY seven) | `README.md`, `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`, `.agent/project-info/context.md`, `.agent/project-info/brief.md`, `CHANGELOG.md`, `docs/build-windows.md` | The TODO's enumeration + the carried Task-5 4.5b fix. Nothing else is touched. |
| D2 | Verify-only files (NO edits, verdicts recorded) | `.agent/project-structure.md` (CURRENT — its `nsis/installer.nsi` / `build-installer.sh` / `window_title.rs` entries landed at Tasks 1+4; verified by read), `.agent/project-info/product.md` (historical greenfield overview — the standing precedent leaves it untouched), `docs/build.md` (NOT in the Task-6 enumeration), `docs/build-windows.md` `## Contents` (no heading changes there), `architecture.md` `## Contents` (no `##` heading changes there), `context.md` `## Contents` (the four section headings are unchanged — verified) | Scope fence + Task-2 D5. |
| D3 | README hotkeys disposition (the Task-3 item) | NOTHING added. Task 3's verdict (§5 discharge note) is BINDING: the README's Controls table + per-feature text already list every hotkey with its use; Task 6 adds NO hotkeys table and NO hotkeys note. The disposition is recorded HERE and in the context.md entry (not in the README) | Task 3 §5: "Task 6 adds NO hotkeys table and NO hotkeys note to README." |
| D4 | Window-title README placement | A NEW `## Window Title and Fullscreen Name Popup (Wave 8)` section inserted AFTER the `## v1 Features` section's final "Additional scope notes" block and BEFORE `## Technology Stack` | A post-v1 Wave-8 feature must NOT join the numbered seven-feature v1 enumeration (the README's "The v1 scope is exactly the seven features below" would be contradicted); its own `##` section keeps the v1 list intact and gives the TOC one clean item |
| D5 | Distribution section shape | `## Distribution and Installation` REPLACED in place, installer-first: intro (installer recommended + Releases pointer + all three artifacts) → `### Install with the wizard installer (recommended)` → `### Install from the portable zip (portable alternative)` → `### Make Images Scroller the default image viewer (optional, zip installs)` (the existing helper subsection RETAINED but re-scoped to zip installs) | The TODO: "installer-first installation instructions (replacing/adjusting the zip-extract flow)"; the helper lives only in the zips now (Task-1 D10), so the default-viewer subsection must say so |
| D6 | Releases-flow wording approach | User-facing README: describes the standing mechanism ("Official packages are published in the GitHub Releases section …; each release carries the wizard installer plus the zips, with the release notes summarizing the version's changes") with NO "not yet published" caveat. Internal docs (tech.md + context.md + the architecture decisions row): the SAME mechanism described AND explicitly marked as the wave's release flow EXECUTED BY TASK 7, pending at this documentation's date | The caller's pin: docs describe the flow, not the execution; README is user-facing and stays evergreen; the pending status belongs to the internal log |
| D7 | brief.md addendum form | A NEW `## Addendum — Operator Additions (2026-11-17, BINDING)` section (mirroring the 2026-11-02/07/12 addenda pattern) carrying item **9** — the wizard-installer distribution decision. NO body edit anywhere in brief.md (the addendum supersedes contradicting sentences by its own terms) | The TODO: "the distribution model gains the user's 2026-11-17 wizard-installer decision as a BINDING addendum item"; the established addendum mechanics |
| D8 | CHANGELOG conversion | The current empty `## [Unreleased]` block becomes the `## [1.1.0] - 2026-11-18` entry (Added + Changed filled, no empty subsections — the 1.0.0 entry's precedent) and a FRESH empty `## [Unreleased]` skeleton (the same six-subsection shape) is re-inserted ABOVE it | Keep-a-Changelog convention; the TODO: "new entries go under `## [Unreleased]` and are released with the version bump" (the bump already landed at Step 3 — this task writes the release) |
| D9 | Commit strategy (PINNED — one choice) | Commit A: `docs: add wave8 task6 documentation plan` (this plan file only). Commit B: `docs: wave8 task6 wizard installer and window title documentation` — ALL SEVEN edited files in ONE commit (the Wave-7 Task-5 precedent: one plan commit + one docs commit carrying every edited file) | Prior-wave commit-chain shape; the caller's "one docs commit or split per file group — pin ONE" |
| D10 | Census record | 195 → **213** unit tests; 28 → **29** test-bearing files; the single delta is `src/window_title/tests.rs` (18 tests). Breakdown: 29 = 19 `*_tests.rs` + 5 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`) | Task-5 results §2 (verbatim substance) |
| D11 | The popup-refresh record | All docs state the TODO-governed behavior: while fullscreen is ON the popup RE-ARMS on EVERY visible-image change and shows the CURRENTLY visible image's name+ext (each re-arm rewrites the text and resets the 3-s deadline). The superseded D10 "stable for the full 3 s" clause is mentioned ONLY in the tech.md mechanism bullet as the adjudication note (the `src/app/window_title.rs` doc-comment fact), never as current behavior | The adjudicated record in `src/app/window_title.rs:8–15`; the caller's "document the TODO-governed behavior" |
| D12 | NO harness run | Zero `alpine-vm` calls, zero docker commands, zero dev-checks runs in this task. Verification is pure read/anchor checking (§5). The census is NOT re-run — it is RECORDED from the Task-5 results | Task-2 D7 precedent; docs-only |

---

## 2. High-Level Approach

Seven documentation files are updated to the wave's landed facts: the README becomes installer-first (with the Releases pointer, the new window-title section, the hotkeys disposition = nothing added, and the Task-2 rolling TOC update), tech.md gains one new mechanism section + the census/build-matrix/releases-flow records, architecture.md gains the component/pipeline/decisions records, context.md gets the closing-step rewrite (Current Work Focus + Recent Changes + Current Project State + Immediate Next Steps, including the pending OPERATOR-RUN gate and the pending Task 7), brief.md gains the BINDING addendum item 9, CHANGELOG.md gains the `## [1.1.0] - 2026-11-18` entry, and docs/build-windows.md receives the one carried line fix. Commit A (plan) then Commit B (all seven files). 4.3 reviews, 4.4 runs the coherence/TOC-anchor pass, 4.5b checks adherence, 4.6 marks the TODO.

---

## 3. Exact Per-File Edits (4.2 implementer)

> All edits use real newline characters (newline-prevention rule); the files' existing line endings are kept (no `.gitattributes` pin on markdown). Every anchor below was derived with the Task-2 D1 slug rules (lowercase; spaces → hyphens; punctuation stripped) — re-derive each during verification (§5).

### 3.1 `README.md` (six edits)

**E1 — `## Contents` REPLACED in place** (the whole block, current lines 4–19). New block (11 numbered items + 7 sub-bullets):

```markdown
## Contents

1. [Overview](#overview)
2. [Problem Statement](#problem-statement)
3. [v1 Features](#v1-features)
4. [Window Title and Fullscreen Name Popup (Wave 8)](#window-title-and-fullscreen-name-popup-wave-8)
5. [Technology Stack](#technology-stack)
6. [Distribution and Installation](#distribution-and-installation)
   - [Install with the wizard installer (recommended)](#install-with-the-wizard-installer-recommended)
   - [Install from the portable zip (portable alternative)](#install-from-the-portable-zip-portable-alternative)
   - [Make Images Scroller the default image viewer (optional, zip installs)](#make-images-scroller-the-default-image-viewer-optional-zip-installs)
7. [Project Status](#project-status)
8. [Build and Run](#build-and-run)
   - [Container dev checks (no host Rust/Cargo)](#container-dev-checks-no-host-rustcargo)
   - [Windows release cross-builds (x64 and x86)](#windows-release-cross-builds-x64-and-x86)
   - [Portable zip packaging](#portable-zip-packaging)
   - [Windows wizard-installer build (NSIS)](#windows-wizard-installer-build-nsis)
9. [Note for AI Agents](#note-for-ai-agents)
10. [License](#license)
```

**E2 — NEW `## Window Title and Fullscreen Name Popup (Wave 8)` section** inserted between the `## v1 Features` section's last line (the `- Per the brief, the on-screen UI layout remains subject to later definition by the user; ...` bullet) and the `## Technology Stack` heading (one blank line on each side):

```markdown
## Window Title and Fullscreen Name Popup (Wave 8)

The window title always shows the path of the currently visible image and updates on every navigation:

- Maximized (the launch state): the FULL path is shown when it fits completely in the title. Otherwise the file name with its extension is shown completely — or, when the name itself is too long, only its LAST characters are shown, prefixed with `...`. With the remaining space the containing folder is added FIRST, then the drive, with `...` between the drive and the visible folder chain, and more parent folders keep being added (walking up) while they still fit.
- While the window is NOT maximized: ONLY the file name (+ extension) is shown — last-chars-truncated with the same `...` prefix when the name itself is too long.
- With no image open (including while an empty sibling folder is browsed) the title stays `Images Scroller`.
- While fullscreen is ON, on every visible-image change (navigation) a popup at the TOP CENTER of the screen shows the currently visible image's name + extension for 3 seconds.

(Mechanism detail for developers: [.agent/project-info/tech.md](.agent/project-info/tech.md) → `## Wave-8 Wizard-Installer and Window-Title Mechanisms (Implemented)`.)
```

**E3 — `## Distribution and Installation` REPLACED in place** (the whole section from the `##` heading through the last helper-notes line, i.e. current lines 78–106, same position). New content:

```markdown
## Distribution and Installation

Images Scroller is distributed primarily as an executable **wizard installer** for 64-bit Windows, published in the GitHub **Releases** section of the project (https://github.com/kazuyakuza/img-scroller/releases) together with the portable zip packages; packages can also be built from source with the container harness (see [Build and Run](#build-and-run) below and [docs/build-windows.md](docs/build-windows.md)). Each release carries the wizard installer plus the zips, with the release notes summarizing the version's changes. Release artifacts (the version is read from `Cargo.toml`; the current release is 1.1.0):

- 64-bit Windows: the wizard installer `ImagesScroller-1.1.0-windows-installer.exe` (recommended), plus the portable zip `ImagesScroller-1.1.0-windows-x64.zip`
- 32-bit Windows: the portable zip `ImagesScroller-1.1.0-windows-x86.zip` (there is NO 32-bit installer — a 32-bit Windows user cannot run the 64-bit installer's payload, so the x86 zip stays the only 32-bit distribution)

### Install with the wizard installer (recommended)

1. Download `ImagesScroller-1.1.0-windows-installer.exe` from the Releases section and run it.
2. The wizard asks for the destination folder — the default is your per-user `Programs\Images Scroller` folder under `%LOCALAPPDATA%` (remembered across reinstalls). Pick any folder you like: no administrator rights are needed, and everything installs for YOUR Windows account only.
3. The **file-association page** shows one checkbox per supported image type — the 17 types listed below — with NONE preselected: pick exactly the types you want to offer Images Scroller for (for example, only `.png`). The install registers only the types you pick (the "Open with" lists and the Windows Settings "Default apps" visibility). Windows never lets a program set defaults silently, so when at least one type was selected the installer opens the Settings "Default apps" page for you to make the final per-type choice.
4. Finish. The install places `img-scroller.exe`, the config file `img-scroller.toml`, and `LICENSE` in the destination folder and creates a Start Menu shortcut. Double-click the shortcut (or the exe) — the application opens (the window opens maximized; no console window appears).

To remove the app, use the **Images Scroller** entry in Windows' "Installed apps" / Add-or-remove-programs list: the uninstaller removes the installed files, the Start Menu shortcut, and EXACTLY the associations the install created (nothing else is touched).

### Install from the portable zip (portable alternative)

1. Download the zip that matches your Windows architecture (64-bit → `...-x64.zip`, 32-bit → `...-x86.zip`).
2. Right-click the zip → "Extract All..." → pick a NEW, EMPTY destination folder of your choice. That folder IS the install folder: the four files land at its root.
3. Double-click `img-scroller.exe` — the application opens (the window opens maximized; no console window appears).

The config file `img-scroller.toml` ships next to the executable; the current version does not read it (settings arrive in a future version), so there is nothing to configure.

### Make Images Scroller the default image viewer (optional, zip installs)

The wizard installer's file-association page replaces this flow for installer installs; this helper flow applies to ZIP installs, where the helper ships inside the extracted folder. Windows 10/11 do not let any program set a default viewer silently — the final choice is always made by you in Settings. The packaged helper prepares that choice:

1. Double-click `register-default-viewer.cmd` INSIDE the extracted install folder. It registers Images Scroller for YOUR Windows user only (HKCU registry keys; no administrator rights), adds it to the "Open with" lists, makes it visible in Settings → Apps → Default apps, and opens that Settings page for you.
2. On the "Default apps" page, find **Images Scroller** and set it as the default for the file types you want (Windows 11: click the app in the list; Windows 10: use "Set defaults by app").

The 17 image file types (identical for the installer's association page and the helper): `.avif`, `.bmp`, `.dib`, `.gif`, `.heic`, `.heif`, `.ico`, `.jfif`, `.jpe`, `.jpeg`, `.jpg`, `.jxr`, `.png`, `.tif`, `.tiff`, `.webp`, `.wdp`. Whether a given type can actually be decoded depends on the image codecs installed in your Windows system (Windows Imaging Component) — the same constraint the application itself has.

Zip-install helper notes: re-running the helper is safe (it writes the same registration values again); if you MOVE the install folder, run the helper once more so the registration points at the new location; to undo the registration, open a command prompt in the install folder and run `register-default-viewer.cmd --unregister` (it removes only the registry entries Images Scroller created — your files and other programs' registrations are untouched). The full packaged guide is `DEFAULT-VIEWER-GUIDE.txt` in the install folder.
```

**E4 — Technology Stack.** Two micro-edits:
(a) The intro sentence (line 66): insert `, and the Wave-8 wizard-installer and window-title slice` immediately after `and the Wave-7 distribution/packaging slice` and before ` — all seven v1 features are landed`. Nothing else in the sentence changes.
(b) The Language row: `Cargo package `img-scroller` 1.0.0` → `Cargo package `img-scroller` 1.1.0` (the ONLY version change in the row).

**E5 — Project Status paragraph.** APPEND at the very END of the paragraph (after its last sentence, before the newline; no earlier sentence modified):

```markdown
Wave 8 landed (branch `feat/wave-8-wizard-installer`, version 1.1.0): the NSIS wizard installer (a per-user HKCU-only install with a destination page and a file-association selection page over the 17 supported types — none preselected — a Start Menu shortcut, and an uninstaller removing exactly the created surface), the portable zips retained as the alternative (the x86 zip stays the only 32-bit distribution), and the window title showing the visible image's path (the full-path/name/truncation ladder, the not-maximized name-only rule, and the fullscreen 3-second top-center name popup). Verification: dev checks `ALL CHECKS PASSED` (213 unit tests across 29 test-bearing files), the fresh artifact chain exited 0, and both DLL import audits are set-equal to the recorded 22-DLL constant. Pending: the Wave-8 OPERATOR-RUN (the installer/association flow on real Windows — the user's gate) and the GitHub Releases publish (Task 7).
```

**E6 — `## Build and Run`.** Append the NEW subsection AFTER `### Portable zip packaging` (after its paragraph, before `## Note for AI Agents`):

```markdown
### Windows wizard-installer build (NSIS)

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh

Builds ONE installer embedding the x64 release exe — build that target FIRST (`sh scripts/build-windows.sh x86_64-pc-windows-gnu`), because the installer consumes `dist/windows-x64/img-scroller.exe`. Artifact: `dist/ImagesScroller-<version>-windows-installer.exe` (the version is read from `Cargo.toml`). The `makensis` (NSIS 3.08) cross-build, the define-injection mechanism, the installer behavior, and the typed failures are documented in [docs/build-windows.md](docs/build-windows.md) → "Windows installer packaging (NSIS)".
```

(README edits DONE: E1–E6. The hotkeys record is NOT touched — D3. The Controls table, v1 Features, Overview, Problem Statement, Note for AI Agents, License stay byte-identical.)

### 3.2 `.agent/project-info/tech.md` (nine edits)

**T1 — `## Contents` REPLACED in place** (the whole numbered list, current lines 9–27). New list (20 items — the new item 18; old 18→19, old 19→20):

```markdown
1. [Technology Stack](#technology-stack)
2. [Image Format Support](#image-format-support)
3. [Dependencies (Landed, Wave 1)](#dependencies-landed-wave-1)
4. [WIC Decode and Open-Dialog Mechanism (Implemented)](#wic-decode-and-open-dialog-mechanism-implemented)
5. [Horizontal Navigation Mechanism (Implemented, Wave 2)](#horizontal-navigation-mechanism-implemented-wave-2)
6. [Vertical Navigation and Session Last-Viewed Memory (Implemented, Wave 3)](#vertical-navigation-and-session-last-viewed-memory-implemented-wave-3)
7. [Fullscreen, Zoom and Fit Modes (Implemented, Wave 4)](#fullscreen-zoom-and-fit-modes-implemented-wave-4)
8. [Wave-4.2 Operator-Feedback Mechanisms (Implemented)](#wave-42-operator-feedback-mechanisms-implemented)
9. [Wave-5 Pan-then-Navigate Mechanism (Implemented)](#wave-5-pan-then-navigate-mechanism-implemented)
10. [Entry-Name System Order (Implemented, Wave 5.1; Mechanism Wave 5.2)](#entry-name-system-order-implemented-wave-51-mechanism-wave-52)
11. [Open-with Enumeration and Launcher Mechanism (Implemented, Wave 6)](#open-with-enumeration-and-launcher-mechanism-implemented-wave-6)
12. [Development Environment and Setup](#development-environment-and-setup)
13. [Build Environment (Container on the Alpine VM)](#build-environment-container-on-the-alpine-vm)
14. [Build and Run](#build-and-run)
15. [Testing](#testing)
16. [Technical Constraints](#technical-constraints)
17. [Distribution and Packaging (Implemented, Wave 7)](#distribution-and-packaging-implemented-wave-7)
18. [Wave-8 Wizard-Installer and Window-Title Mechanisms (Implemented)](#wave-8-wizard-installer-and-window-title-mechanisms-implemented)
19. [Tool Usage Patterns for AI Agents](#tool-usage-patterns-for-ai-agents)
20. [References](#references)
```

**T2 — `## Status` paragraph.** APPEND at the very end of the paragraph (no earlier sentence modified):

```markdown
Landed for Wave 8 (branch `feat/wave-8-wizard-installer`, version 1.1.0, 2026-11-18): the NSIS wizard installer, the window-title path feature, and the census growth to 213 unit tests / 29 test-bearing files (mechanisms: `## Wave-8 Wizard-Installer and Window-Title Mechanisms (Implemented)` below).
```

**T3 — Technology Stack, Language row.** `version 1.0.0` → `version 1.1.0` (the ONLY change in the row).

**T4 — `## Build Environment (Container on the Alpine VM)`, the harness-landing bullet.** APPEND at the bullet's end (nothing else in the bullet changes):

```markdown
 Extended in Wave 8 (Task 1): the Dockerfile's apt line also gained `nsis` (`makensis`, backing the installer build — an image rebuild was required after the change).
```

**T5 — `## Build and Run`.** Append ONE new bullet at the END of the section (after the `- Portable zip packaging:` bullet):

```markdown
- Windows wizard-installer build: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh` — ONE installer embedding the x64 exe; the x64 target must be built FIRST (build order: `sh scripts/build-windows.sh x86_64-pc-windows-gnu` → `sh scripts/build-installer.sh`); artifact `dist/ImagesScroller-<version>-windows-installer.exe` under gitignored `dist/`; full guide: [docs/build-windows.md](../../docs/build-windows.md) → "Windows installer packaging (NSIS)".
```

**T6 — NEW `## Wave-8 Wizard-Installer and Window-Title Mechanisms (Implemented)` section** inserted AFTER the `## Distribution and Packaging (Implemented, Wave 7)` section and BEFORE `## Tool Usage Patterns for AI Agents`:

```markdown
## Wave-8 Wizard-Installer and Window-Title Mechanisms (Implemented)

Landed in Wave 8 (branch `feat/wave-8-wizard-installer`, version 1.1.0; sources: `.kilo/plans/20261118-wave8-task1-wizard-installer-implementation-plan.md` D1–D19, `.kilo/plans/20261118-wave8-task4-window-title-implementation-plan.md` D1–D16, and `.kilo/plans/20261118-wave8-task5-verification-results.md`):

- Installer technology + artifact: NSIS 3.08 (`makensis`) cross-compiled in the container (`nsis` appended to the Dockerfile's apt line at Task 1 — image rebuilt); the tracked source is `packaging/nsis/installer.nsi` (MUI2 + nsDialogs + LogicLib), compiled by the tracked POSIX-sh `scripts/build-installer.sh`, which injects every dynamic value as an absolute container-path `makensis` define (`-DPRODUCT_VERSION=…`, `-DPRODUCT_VERSION_4=…`, `-DINSTALLER_OUTFILE=…`, `-DEXE_PAYLOAD=…`, `-DCONFIG_PAYLOAD=…`, `-DLICENSE_PAYLOAD=…`) and installs the artifact part-then-rename at `dist/ImagesScroller-<version>-windows-installer.exe` (the version read dynamically from `Cargo.toml`); `makensis` warnings are NON-FATAL (exit 0 + a non-empty artifact is the success evidence; the observed warning was 9100, no `LegalCopyright` key); typed failures use the `build-installer: error:` prefix; the script never touches git. ONE installer embedding ONLY the x64 exe (`dist/windows-x64/img-scroller.exe` — build order: `build-windows.sh x86_64-pc-windows-gnu` FIRST, then `build-installer.sh`); NO x86 installer — 32-bit users keep the retained x86 zip.
- Wizard pages + install scope: Welcome → destination folder → the custom file-association page → install → finish. Per-user HKCU-ONLY (`RequestExecutionLevel user`, `SetShellVarContext current`; no elevation, no UAC); default destination `%LOCALAPPDATA%\Programs\Images Scroller`; `InstallDirRegKey` HKCU `Software\Images Scroller` / `InstallDir` persists the destination across reinstalls. Installed set: `img-scroller.exe` + `img-scroller.toml` + `LICENSE` — NOTHING else (`register-default-viewer.cmd` and `DEFAULT-VIEWER-GUIDE.txt` are NOT installed — the association page replaces them; they stay in the zips). Shortcut: a per-user Start Menu shortcut ONLY (no desktop shortcut, no run-after-install checkbox).
- Association-selection page: a custom nsDialogs page with per-type checkboxes over EXACTLY the 17 supported extensions of `src/folder_listing.rs`'s whitelist (`avif bmp dib gif heic heif ico jfif jpe jpeg jpg jxr png tif tiff webp wdp`, lowercase dotted `.ext` labels), a fixed 3-columns × 6-rows grid (6/6/5), **NONE preselected** — the user explicitly picks (e.g. only `.png`).
- Registry surface (the Wave-7 helper surface REUSED verbatim, HKCU-only): the ProgID `HKCU\Software\Classes\ImagesScroller.Image` (default `Images Scroller Image`, `FriendlyTypeName`, `DefaultIcon` `"<exe>",0`, `shell\open\command` `"<exe>" "%1"`); for the SELECTED types only: the per-extension `OpenWithProgids` value + the `HKCU\Software\Images Scroller\Capabilities\FileAssociations` value; the Capabilities tree (`ApplicationName`/`ApplicationIcon`/`ApplicationDescription`) and the `HKCU\Software\RegisteredApplications` pointer. **`UserChoice` is NEVER written** (FORBIDDEN — hash-protected, unsupported programmatically; the Wave-7 precedent governs; the final per-type default stays user-mediated in Windows Settings). Reinstall semantics: pre-clean-then-write — every per-type entry the installer could have created is deleted first (value-only, absence-tolerant), then only the newly selected types are written, so a reinstall with a smaller selection converges to exactly that selection. After the writes: `System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, i 0, i 0)'` (SHCNE_ASSOCCHANGED — refreshes Explorer's Open-with cache).
- Finish hand-off: `ms-settings:defaultapps` is opened at install success ONLY when at least one type was selected (`$InstallHadSelections`); with no selection no Settings page opens.
- Uninstaller: `WriteUninstaller "$INSTDIR\uninstall.exe"` + an HKCU Add/Remove-Programs entry (`HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\Images Scroller`: `DisplayName`/`DisplayVersion`/`DisplayIcon`/`UninstallString`/`NoModify`/`NoRepair`). The uninstall section deletes the 3 installed files + the Start Menu shortcut + the per-type entries (value-only deletes under the extension keys — the `.ext` keys themselves are NEVER deleted), the ProgID key, the whole `HKCU\Software\Images Scroller` tree, the `RegisteredApplications` value, and the uninstall key; a final SHChangeNotify. NEVER touches `UserChoice`.
- Releases flow (the wave's release mechanism — EXECUTED BY TASK 7, pending at this documentation's date): a GitHub Release on `origin` tagged `v1.1.0` with the installer executable AND both zips attached, with release notes drawn from the `CHANGELOG.md` `## [1.1.0]` entry; `gh` CLI availability/auth is VERIFIED FIRST (host or VM) — if agent-side publishing is impossible the wave says STOP-and-ask (the user executes the publish or provides authorization). Publishing is a GitHub-API action, NOT a git push (pushing Git refs stays restricted to workflow Step 5, `main` → `origin` ONLY).
- Window-title mechanism (`src/window_title.rs` pure + `src/window_title/segments.rs` + `src/app/window_title.rs` wiring; the TODO-governed behavior): the title shows the VISIBLE image's path, recomposed only on a path change or a maximize/restore change (a plain window resize does NOT recompose until the next trigger), sent via `egui::ViewportCommand::Title` ONLY when the composed string changed. Measurement: pixel-accurate via the REAL egui font layout (`fonts_mut(|fonts| fonts.layout_no_wrap(...))` → the galley width) against 85 % of the viewport outer width (`TITLE_BAR_USABLE_WIDTH_FRACTION = 0.85`, reserving ~15 % for the OS caption/icon/buttons). Segmentation: the stored canonical path is converted READ-ONLY through `open_with_listing::shell_parseable_path` (the Wave-6.2 display-form conversion — `\\?\UNC\` → `\\`, `\\?\` stripped, pass-through) and split on `\` and `/` (a UNC's first two segments form ONE drive segment; a 2-char `X:` segment is the local drive; the last segment is the file name). Composition ladder (maximized): the FULL path if it fits verbatim → else the name+ext base, last-chars-truncated with the literal `...` prefix if the name alone is too long (a truncated base consumes the whole budget — nothing else is added) → the containing folder FIRST → the drive SECOND with the `...` gap between the drive and the visible chain EXACTLY when at least one folder is omitted (a drive that does not fit is SKIPPED, not terminal) → parent folders walked up NEAREST-FIRST while each still fits (the first failure stops the walk). Not-maximized: name+ext only, same budget and fitter. `path == None` (fresh state; also while an empty sibling folder is browsed) → the base title `Images Scroller`.
- Fullscreen name popup: while fullscreen is ON, on EVERY visible-image change the popup RE-ARMS at the top center (`Align2::CENTER_TOP`, 16-pt offset; `Order::Foreground`; non-interactable; dark filled frame — the ESC-close popup visual precedent) showing the CURRENTLY visible image's name+ext (`file_name_with_extension` — the shared source) for `FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS = 3.0`, with the `0.05`-s expiry wake margin (the `esc_close_confirm::request_expiry_wake` precedent; `input.time` the single clock); toggling fullscreen OFF disarms. ADJUDICATION NOTE (recorded in `src/app/window_title.rs`'s doc comment): the implementation plan's D10 "popup text stable for the full 3 s" clause was SUPERSEDED by the TODO's "on every visible-image change" rule — the re-arm behavior above is the shipped behavior.
- Census + verification (Task 5, VERIFIED): dev checks `ALL CHECKS PASSED` exit 0 — census 195 → **213** unit tests, 28 → **29** test-bearing files (the single delta: NEW `src/window_title/tests.rs`, 18 tests; no installer pure logic exists in Rust — the wizard is NSIS-owned, zero Rust lines, zero tests); the fresh artifact chain (x64 → x86 → installer → both zips) exited 0 with non-empty correctly named artifacts (installer 6,846,665 B; x64 zip 6,720,409 B; x86 zip 7,172,404 B); BOTH DLL import audits SET-EQUAL 22 (no delta, no script fill); NO dependency change. Evidence: `.kilo/plans/20261118-wave8-task5-verification-results.md`.
```

**T7 — `## Testing`, the census bullet (line 184, ONE long line).** THREE surgical edits inside it:
(a) The opening `195 unit tests` → `213 unit tests`.
(b) Insert the new per-file entry immediately BEFORE the trailing `Grand total` clause (after the `src/zoom.rs` — 8 entry, keeping the list's ordering): `` `src/window_title/tests.rs` — 18 (Wave-8 Task 4: the composition matrix — the full-path/name/last-chars fitting, the containing-folder/drive/parent chain with the `...` marker, the constants, and the shared name extraction)``.
(c) The trailing `Grand total 195 across 28 test-bearing files — **28 test-bearing files = 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`)**` → `Grand total 213 across 29 test-bearing files — **29 test-bearing files = 19 `*_tests.rs` + 5 `tests.rs` + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`)** — the Wave-8 delta is `src/window_title/tests.rs` (18 tests)`.

**T8 — `## Technical Constraints`.** Append ONE new bullet at the END of the section:

```markdown
- Installer-registered file-type defaults stay USER-MEDIATED: neither the Wave-7 helper nor the Wave-8 wizard installer EVER writes the hash-protected `UserChoice` key — both register the user-selected types HKCU-only (the Open-with lists + the Capabilities/RegisteredApplications surface) and may open `ms-settings:defaultapps` for the final Settings confirmation; the per-type default is always chosen by the user in Windows Settings.
```

**T9 — `## References`, the build-windows line.** Replace:

```markdown
- [`docs/build-windows.md`](../../docs/build-windows.md) — Windows x64/x86 cross-build + zip packaging guide + OBSERVED DLL import lists (both architectures).
```

with:

```markdown
- [`docs/build-windows.md`](../../docs/build-windows.md) — Windows x64/x86 cross-build + zip packaging + NSIS installer guide + OBSERVED DLL import lists (both architectures).
```

### 3.3 `.agent/project-info/architecture.md` (six edits)

**A1 — `## Status` paragraph.** APPEND at the very end of the paragraph (no earlier sentence modified):

```markdown
Wave 8 (branch `feat/wave-8-wizard-installer`, version 1.1.0) landed the NSIS wizard-installer distribution and the window-title path feature (see `## Logical Components`, `## Module Layout (Implemented)`, and `## Build and Release Pipeline`).
```

**A2 — `## Logical Components`.** Two edits:
(a) The `- **Distribution Packaging — LANDED (Wave 7)**:` bullet is REPLACED in place with:

```markdown
- **Distribution Packaging — LANDED (Wave 7; the Wave-8 wizard installer supersedes the install flow)**: portable zip packaging (one flat zip per architecture from `scripts/package-windows.sh`: the exe + config + registration helper + guidance at the zip root; the version read from `Cargo.toml`; output under gitignored `dist/`; RETAINED in Wave 8 as the portable alternative and the only x86 distribution), the shipped config file `img-scroller.toml` (header-comments-only TOML the v1 binary does not read), and the default-viewer registration helper `register-default-viewer.cmd` + packaged guidance `DEFAULT-VIEWER-GUIDE.txt` (both retained in the zips; NOT installed by the installer — the wizard's association page replaces them). The Wave-8 **wizard installer** (`scripts/build-installer.sh` compiling `packaging/nsis/installer.nsi` with NSIS 3.08) is the PRIMARY distribution: MUI2 wizard (Welcome → destination → the custom file-association page → install → finish), per-user HKCU-only install at `%LOCALAPPDATA%\Programs\Images Scroller` (remembered across reinstalls), per-type association checkboxes over the 17 supported extensions with NONE preselected, the Wave-7 helper registry surface written ONLY for the selected types (the ProgID `ImagesScroller.Image`, the per-type OpenWithProgids, the Capabilities FileAssociations, the RegisteredApplications pointer — NEVER `UserChoice`; the final default stays user-mediated in Settings, which the installer opens at finish when ≥1 type was selected), installed files exe + config + LICENSE, a per-user Start Menu shortcut, pre-clean-then-write reinstall convergence + `SHChangeNotify`, and an uninstaller removing exactly the created files/shortcut/associations plus the Add/Remove-Programs entry.
```

(b) A NEW bullet inserted immediately BEFORE the Distribution bullet (after the `- **Fast "Open with..." Service — LANDED (Wave 6)**:` bullet):

```markdown
- **Window Title + Fullscreen Name Popup — LANDED (Wave 8)**: the title always reflects the VISIBLE image's path — pure composition in `src/window_title.rs` (+ `src/window_title/segments.rs`: the read-only `shell_parseable_path` display form split on `\`/`/` into drive/chain/name) pixel-measured against 85 % of the viewport outer width (`TITLE_BAR_USABLE_WIDTH_FRACTION`) via an injected egui-font measure closure (`src/app/window_title.rs` owns the per-frame read: viewport maximized state + outer width → recomposition ONLY on a path change or a maximize/restore change → `ViewportCommand::Title` sent ONLY when the string changed). Ladder (maximized): full path if it fits → name+ext complete (last-chars truncation with the literal `...` prefix if the name alone is too long; a truncated base consumes the whole budget) → containing folder FIRST → drive SECOND with the `...` gap between the drive and the visible chain when folders are omitted → parent folders walked up nearest-first while each fits (the first failure stops the walk; a skipped drive does not). Not-maximized: name+ext only (same budget/fitter). `path == None` (fresh state; also while an empty folder is browsed) → `Images Scroller`. Fullscreen ON: on EVERY visible-image change the top-center (`Align2::CENTER_TOP`, 16-pt offset) non-interactable dark-frame popup shows the CURRENT image's name+ext for 3.0 s (re-armed on every navigation; the 0.05-s expiry wake margin — the `esc_close_confirm` precedent); toggling fullscreen OFF disarms; the implementation plan's D10 "stable for the full 3 s" clause was adjudicated superseded by the TODO's every-navigation rule (recorded in `src/app/window_title.rs`'s doc comment).
```

**A3 — `## Module Layout (Implemented)`.** Two edits:
(a) In the `- \`src/app.rs\` —` bullet: (i) the state list gains the window-title state — after `...refreshed at the three decided points))` insert `, and the session \`window_title_state\` (Wave 8: the five RAM-only window-title/fullscreen-popup fields grouped in \`WindowTitleState\`, managed by the \`window_title\` child module)` so the list reads `... open_with_programs (…), and the session window_title_state (…)`. (ii) In the bullet's child-modules enumeration, insert `src/app/window_title.rs` between `src/app/pill_style.rs` and `src/app/zoom.rs` (alphabetical).
(b) A NEW bullet inserted between the `- \`src/pan.rs\` —` bullet and the `- \`src/app_display.rs\` —` bullet:

```markdown
- `src/window_title.rs` — pure window-title composition (Wave 8 Task 4; child modules `src/window_title/segments.rs` — the read-only display-path conversion, the `\`/`/` split, and the drive/folder/file-name partition — and `src/window_title/tests.rs`, the 18-test composition matrix): `compose_window_title` implements the Task-4 ladder (full path → base name with last-chars `...` truncation → containing folder → drive with the `...` gap → parent folders; not-maximized = name only; `path == None` → `APP_WINDOW_TITLE`); `file_name_with_extension` is the shared name source; the constants `PATH_SEGMENT_SEPARATOR`, `TITLE_ELLIPSIS`, `TITLE_BAR_USABLE_WIDTH_FRACTION = 0.85`, `FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS = 3.0`, `FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS = 0.05`. The wiring child `src/app/window_title.rs` owns `WindowTitleState` (five RAM-only fields), the per-frame `update_window_title_and_popups` (change-gated `ViewportCommand::Title` sends + the popup arm/expire/wake), and the popup render; `src/app/fullscreen.rs` gained the OFF-branch disarm and the end-of-fullscreen-layout render call.
```

**A4 — `## Data and State`, the per-session in-memory bullet (the Wave-3–6 one).** APPEND at the bullet's end (nothing else changes):

```markdown
, and the session `window_title_state` (Wave 8: five RAM-only fields grouped in `WindowTitleState` — the last-title-image path, the last maximized state, the last sent title, and the fullscreen name-popup expiry/text; never persisted)
```

**A5 — `## Build and Release Pipeline`.** Three edits:
(a) The intro paragraph: after `...the packaging run produces both flat zips.` insert ` Extended in Wave 8 with the NSIS installer build: the x64 → installer chain exits 0 and produces \`dist/ImagesScroller-1.1.0-windows-installer.exe\`.`
(b) A NEW bullet appended AFTER the `- Portable zip packaging (Wave 7):` bullet:

```markdown
- Wizard-installer build (Wave 8): `scripts/build-installer.sh` compiles `packaging/nsis/installer.nsi` with `makensis` (NSIS 3.08, baked into the image since Wave 8), injecting every dynamic value as an absolute container-path define, and installs the artifact part-then-rename at `dist/ImagesScroller-<version>-windows-installer.exe` (the version read dynamically from `Cargo.toml`); it embeds ONLY the x64 exe — build order: `build-windows.sh x86_64-pc-windows-gnu` FIRST, then `build-installer.sh`; `makensis` warnings are non-fatal (exit 0 + a non-empty artifact is the success evidence); typed failures use the `build-installer: error:` prefix; the script never touches git.
```

(c) The container-image bullet: APPEND at its end (nothing else changes):

```markdown
 Since Wave 8 the image also carries `nsis` (`makensis`).
```

**A6 — `## Project Paths` + `## Technical Decisions Summary`.** Three edits:
(a) The Cargo bullet: `package \`img-scroller\` 1.0.0` → `package \`img-scroller\` 1.1.0` (the ONLY version change).
(b) The repository-layout bullet: the packaging clause `(the config file + the registration helper + the guidance text)` → `(the config file + the registration helper + the guidance text + the NSIS installer source \`nsis/installer.nsi\`)`.
(c) TWO new rows appended at the END of the decisions table (after the Distribution row):

```markdown
| Distribution: NSIS wizard installer (per-user HKCU; destination page; association-selection page over the 17 supported types with NONE preselected; the Wave-7 helper registry surface for the selected types only — NEVER `UserChoice`; installed exe + config + LICENSE; Start-Menu shortcut; uninstaller; the zips retained alongside — the x86 zip stays the only 32-bit distribution); GitHub Releases as the official channel (a release tagged `v<version>` with the installer + zips attached and notes from the CHANGELOG entry) | landed (Wave 8; the Release publish itself = Task 7, pending) | the user's 2026-11-17 request (the brief addendum item 9) |
| Window title: the visible image's path composed by the pixel-measured ladder (full path → name+ext → containing folder → drive with the `...` gap → parents; not-maximized = name only; 85 % of the viewport outer width via egui font measurement) + the fullscreen top-center name popup (3 s, re-armed on every navigation) | landed (Wave 8) | the user's 2026-11-17 request |
```

(architecture.md `## Contents` needs NO edit: no `##` heading is added, removed, or retitled — verify-only per D2.)

### 3.4 `.agent/project-info/context.md` (five edits)

**C1 — `## Current Work Focus`, the FIRST dated entry REPLACED in place** (the whole `Dated 2026-11-17. ...` block; the `Dated 2026-11-15.` and older entries below stay verbatim). Replacement:

```markdown
Dated 2026-11-18. **Wave 8 is agent-side IN PROGRESS on the branch `feat/wave-8-wizard-installer` (version 1.1.0, bumped `4fb6b94`; NOT merged): Tasks 1–5 are `[DONE]` (the Task-5 manual-UX OPERATOR-RUN sub-item stays `[ ]`), Task 6 (documentation) is in its 4.x cycle, and Task 7 (GitHub Releases publishing — the wave's FINAL task) is pending.** What landed: (Task 1) the NSIS 3.08 wizard installer — `packaging/nsis/installer.nsi` (MUI2: Welcome → destination page → the custom nsDialogs association-selection page → install → finish), per-user HKCU-only install at `%LOCALAPPDATA%\Programs\Images Scroller` (no elevation), per-type checkboxes over EXACTLY the 17 supported extensions (NONE preselected), the Wave-7 helper registry surface verbatim for the SELECTED types only (the ProgID + per-type OpenWithProgids + Capabilities FileAssociations + the Capabilities tree + the RegisteredApplications pointer; NEVER `UserChoice`), pre-clean-then-write reinstall convergence, `SHChangeNotify`, `ms-settings:defaultapps` opened at finish ONLY when ≥1 type was selected, installed set = exe + config + LICENSE, a Start-Menu shortcut ONLY, and an uninstaller removing exactly the created surface; built container-side by `scripts/build-installer.sh` (`nsis` in the Dockerfile; defines injected; warnings non-fatal) → `dist/ImagesScroller-1.1.0-windows-installer.exe` (x64-only; the x64 build feeds it); the Wave-7 zips KEPT. (Task 2) README TOC + long-doc indexes (`## Contents` in README.md, docs/build-windows.md, context.md). (Task 3) the hotkeys verified COMPLETE — a 16-row inventory, zero gaps, ZERO edits (`.kilo/plans/20261118-wave8-task3-hotkeys-verification-results.md`). (Task 4) the window-title path feature — pure `src/window_title.rs` (+ `segments.rs`, the 18-test `tests.rs`) + wiring `src/app/window_title.rs`: the pixel-measured ladder (full path → name+ext with last-chars truncation → containing folder → drive with `...` → parents; not-maximized name-only; 0.85 of the viewport outer width via egui font measurement; the read-only `shell_parseable_path` display form) + the fullscreen 3-s CENTER_TOP name popup RE-ARMED on every navigation (the plan's D10 3-s-stable clause adjudicated superseded; recorded in the module doc). (Task 5, VERIFIED) dev-checks `ALL CHECKS PASSED` — census 213 unit tests / 29 test-bearing files; the fresh artifact chain (x64 → x86 → installer → both zips) exit 0; BOTH DLL audits set-equal 22 (`.kilo/plans/20261118-wave8-task5-verification-results.md`). (Task 6, this step) the wave documentation landed (README/tech/architecture/context/brief/CHANGELOG/build-windows). Pending: Task 7 (the `v1.1.0` GitHub Release; `gh` availability/auth verified FIRST — STOP-and-ask if agent-side publishing is impossible), then Step 5 (rename/merge/push), and the Wave-8 OPERATOR-RUN gate (the PLAIN checklist at `.kilo/plans/20261118-wave8-task5-verification-plan.md` §8; its release item is gated on Task 7).
```

**C2 — `## Recent Changes`: ONE new entry PREPENDED at the top** (above the existing `1. 2026-11-17:` entries, which stay verbatim as history):

```markdown
1. 2026-11-18: **Wave 8 Task 6 documentation landed (this step) — the wave's records are in README.md, tech.md, architecture.md, this file, brief.md, CHANGELOG.md, and docs/build-windows.md.** Branch `feat/wave-8-wizard-installer`, version 1.1.0. Commits (verbatim): `docs: add wave8 task6 documentation plan` (the plan file), then `docs: wave8 task6 wizard installer and window title documentation` (the seven edited files). What landed: (README.md) `## Distribution and Installation` REPLACED installer-first (the wizard-installer download/run flow — the destination page, the association page over the 17 types with none preselected, the Settings hand-off, the uninstaller note — the portable-zip alternative condensed, the default-viewer helper subsection re-scoped to zip installs, the GitHub Releases pointer kept in the intro); the NEW `## Window Title and Fullscreen Name Popup (Wave 8)` section; the Build-and-Run section gained the `### Windows wizard-installer build (NSIS)` subsection; the TOC updated per Task 2's D5 rolling trigger; the Technology-Stack Language row 1.0.0 → 1.1.0; the Project-Status paragraph gained the Wave-8 sentence; the hotkeys record UNTOUCHED (Task 3's zero-gap verdict discharges it). (tech.md) the NEW `## Wave-8 Wizard-Installer and Window-Title Mechanisms (Implemented)` section (the installer mechanism incl. the registry surface + uninstaller, the releases flow marked pending-Task-7, the title mechanism incl. the 0.85 fraction and the popup re-arm adjudication note, the census); the Testing census updated to 213/29 with the `src/window_title/tests.rs` — 18 entry; the Build-Environment harness bullet + the Build-and-Run installer bullet; the Status paragraph + Language row; the Technical-Constraints user-mediated-defaults bullet; the References build-windows line; the TOC item added. (architecture.md) the Distribution bullet extended to the installer; the NEW Window-Title component bullet; the module-layout window_title entries; the Data-and-State window_title_state mention; the pipeline installer bullet + the image `nsis` note; the Project-Paths version 1.1.0 + the packaging clause; TWO decisions-table rows; the Status paragraph. (this file) the Current-Work-Focus rewrite, this entry, the Current-Project-State Wave-8 bullet + the TODO-state and release-artifact and Git updates, and the Immediate-Next-Steps rewrite (the OPERATOR-RUN gate + Task 7 + Step 5). (brief.md) the 2026-11-17 addendum item 9 (the wizard-installer distribution decision, BINDING). (CHANGELOG.md) `## [Unreleased]` became `## [1.1.0] - 2026-11-18` (the wave's Added/Changed bullets; a fresh empty Unreleased skeleton re-added). (docs/build-windows.md) the carried Task-5 4.5b fix: the zip-packaging version parenthetical `(at execution: 1.0.0)` → `(at execution: 1.1.0)`. Verify-only with NO edits: `.agent/project-structure.md` (current — the nsis/build-installer/window_title entries landed at Tasks 1+4), `product.md` (historical overview), `docs/build.md`. NEXT: the 4.3–4.6 cycle, then Task 7.
```

**C3 — `## Current Project State`.** Four edits:
(a) The FIRST bullet `- **Wave 8 SCHEDULED (NOT processed ...)` is REPLACED in place with:

```markdown
- **Wave 8 IN PROGRESS on the branch `feat/wave-8-wizard-installer` (Tasks 1–5 `[DONE]`; Task 6 docs in its 4.x cycle; Task 7 + Step 5 pending; NOT merged)**: version 1.1.0 (bumped `4fb6b94`); the NSIS 3.08 wizard installer (`packaging/nsis/installer.nsi` + `scripts/build-installer.sh`; the Dockerfile gained `nsis`; artifact `dist/ImagesScroller-1.1.0-windows-installer.exe` 6,846,665 B, x64-only; per-user HKCU install at `%LOCALAPPDATA%\Programs\Images Scroller`; the association page over the 17 types, NONE preselected; the Wave-7 helper registry surface for the selected types only; NEVER `UserChoice`; pre-clean-then-write; a Start-Menu shortcut only; installed exe + config + LICENSE; the uninstaller; the zips KEPT — the x64 zip 6,720,409 B / the x86 zip 7,172,404 B, each 4 flat entries; x64 exe 19,675,523 B / x86 exe 19,828,521 B); the window-title path feature (`src/window_title.rs` + `segments.rs` + `src/app/window_title.rs`; 18 tests; the ladder + the 0.85 fraction + the fullscreen 3-s re-armed popup); the README TOC + long-doc indexes (Task 2); the hotkeys verified COMPLETE (Task 3); census 213 unit tests / 29 test-bearing files; dev-checks `ALL CHECKS PASSED`; BOTH DLL audits set-equal 22 (`.kilo/plans/20261118-wave8-task5-verification-results.md`); the pending gates: the Wave-8 OPERATOR-RUN (the user's run; the checklist at the task-5 plan §8) and Task 7 (the `v1.1.0` GitHub Release).
```

(b) The `- Release artifacts (Wave 7, 1.0.0):` bullet is REPLACED in place with:

```markdown
- Release artifacts (Wave 8, 1.1.0): gitignored `dist/windows-x64/img-scroller.exe` (19,675,523 bytes), `dist/windows-i686/img-scroller.exe` (19,828,521 bytes), `dist/ImagesScroller-1.1.0-windows-installer.exe` (6,846,665 bytes, x64-only), `dist/ImagesScroller-1.1.0-windows-x64.zip` (6,720,409 bytes), and `dist/ImagesScroller-1.1.0-windows-x86.zip` (7,172,404 bytes) — the exes/zips/installer are NEVER committed (`dist/` is gitignored); both zips carry EXACTLY the 4 flat entries with CRLF helper copies; import audit: BOTH exes import the SAME 22 system DLLs (set-equal; audited 2026-11-18 on the fresh artifacts — recorded in the `scripts/build-windows.sh` constants + `docs/build-windows.md`); the gitignored diagnostic exe + log are untracked artifacts (possibly still present in `dist/windows-x64/` — never committed).
```

(c) The `- **TODO state**:` bullet: replace its opening `**Wave 8 (SCHEDULED)**: \`.agent/todos/20261117/20261117-todo-1.md\` — NOT processed (Next Steps #2: a fresh session).` with `**Wave 8 (IN PROGRESS)**: \`.agent/todos/20261117/20261117-todo-1.md\` — Tasks 1–5 marked \`[DONE]\` (the Task-5 manual-UX OPERATOR-RUN sub-item STAYS \`[ ]\` — the user's gate); Task 6 (documentation) is in its 4.x cycle at this step; Task 7 (GitHub Releases publishing — the wave's FINAL task) pending its own 4.x cycle.` (the rest of the bullet — the Wave 7 and older clauses — stays verbatim).

(d) The `- Git:` bullet's content is REPLACED with:

```markdown
- Git: the wave branch `feat/wave-8-wizard-installer` carries the whole Wave-8 chain on top of `ea9dbee` (the planning-session base; version bump `4fb6b94`; HEAD `e82e061` at this step's start — `git log --oneline` is the authoritative record); NOTHING is pushed (push = Step 5, `main` → `origin` ONLY); `origin/main` == `eca641c`; local `main` still carries the four 2026-11-17 planning/session commits UNPUSHED, riding this wave's Step-5 push (the Wave-6 push published pre-existing unpushed ride-alongs — expected precedent). All prior-wave branches were deleted after their verified merges.
```

**C4 — `## Immediate Next Steps`, the TWO numbered items REPLACED** with:

```markdown
1. **Pending Wave-8 OPERATOR-RUN gate — the user's run on a real Windows 10/11 machine** — the PLAIN self-contained checklist, verbatim at `.kilo/plans/20261118-wave8-task5-verification-plan.md` §8: run the wizard installer → install to a chosen destination; the association page lets the user pick types (e.g. only `.png`) and the registration lands correctly (Open With lists; the Settings confirmation the installer opens); the app launches from the install location and the Waves-1–7 behaviors still work from there; the window title behaves per Task 4 (full path / name / truncation / not-maximized rule / the fullscreen 3-s popup); the uninstaller removes files, shortcuts, and associations; the GitHub release is visible and its artifacts download (item 16 — GATED on Task 7; report it "pending Task 7" until the release exists). STANDING NOTE: report the result back (any FAIL means the wave is NOT closed); the TODO's Task-5 manual-UX sub-item stays `[ ]` until the report closes it.

2. **Task 7 — GitHub Releases publishing (the wave's FINAL task)** — runs ONLY after Tasks 1–6 are `[DONE]`; the 4.1b architector step VERIFIES `gh` CLI availability/auth FIRST (host or VM); if agent-side publishing is impossible → STOP-and-ask the user (the user executes the publish or provides authorization); the release is tagged `v1.1.0` with the installer + both zips attached and notes drawn from the `CHANGELOG.md` `## [1.1.0]` entry; publishing is a GitHub-API action, NOT a push (pushing Git refs stays restricted to workflow Step 5, `main` → `origin` ONLY). Then Step 5: the TODO renamed `-DONE` (preserving content), the tmp sweep, checkout `main`, fast-forward merge `feat/wave-8-wizard-installer`, delete the branch after the verified merge, push `main` → `origin` ONLY (the push publishes the unpushed planning-session ride-alongs — expected precedent).
```

**C5 — `## Contents`: NO edit.** The file's four `##` section headings are unchanged by this task (only entries inside them change) — the TOC still resolves. Verify-only.

### 3.5 `.agent/project-info/brief.md` (one edit)

**B1 — NEW addendum section** inserted AFTER the `## Addendum — Operator Corrections (2026-11-12, BINDING)` section (after its item-8 paragraph) and BEFORE the `<!-- DO NOT DELETE NEXT SECTION -->` marker (one blank line on each side):

```markdown
## Addendum — Operator Additions (2026-11-17, BINDING)

The 2026-11-17 operator chat additions supersede any contradicting sentence of this brief wherever it appears; everything not contradicted below stays byte-identical. Source: [`.agent/todos/20261117/20261117-todo-1.md`](../todos/20261117/20261117-todo-1.md) (Wave 8).

9. **Distribution model gains the executable wizard installer** (supersedes/extends `## Distribution and Default App Registration`'s zip-style model; the exact mechanism was decided in the Wave-8 architector step; source: the user's 2026-11-17 request): the primary distribution artifact is an executable **wizard installer** — the user picks the install destination folder in the wizard, and a file-association selection page lets the user choose WHICH supported image types the app is offered for (e.g. only `.png`; none preselected); the installer registers only the user-selected types for the user's Windows account (the Wave-7 HKCU helper registry surface) and NEVER writes the hash-protected `UserChoice` key — the final per-type default stays user-mediated in Windows Settings, which the installer may open for confirmation; an uninstaller removes what the install created. The releases are published in the GitHub **Releases** section of the project. The target platform stays Windows 10/11 (the installer embeds the x64 build; the zip model stays retained as the portable alternative, including the x86 zip).
```

(NO other brief.md change: the `## Distribution and Default App Registration` body, `## Purpose and Goals`, and every other section stay byte-identical — the addendum governs by its own terms.)

### 3.6 `CHANGELOG.md` (one edit)

**CH1 — The block from `## [Unreleased]` (line 8) through the empty `### Security` line (line 20) is REPLACED in place** with:

```markdown
## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [1.1.0] - 2026-11-18

### Added

- Installs Images Scroller with an executable Windows wizard installer (ImagesScroller-1.1.0-windows-installer.exe, published in the GitHub Releases section): the wizard asks for the destination folder — defaulting to your per-user Programs folder under `%LOCALAPPDATA%` — installs the executable, the config file, and the license, and creates a Start Menu shortcut, all without administrator rights because everything installs for your Windows account only.
- Lets you choose which image file types to offer Images Scroller for on a dedicated file-association wizard page — one checkbox for each of the 17 supported types, none preselected — registering only the types you pick for your Windows account (the "Open with" lists and the Windows Settings "Default apps" visibility, which opens automatically after the install when at least one type was selected); Windows itself always makes the final per-type default choice, and the installer never writes it for you.
- Ships an uninstaller (reachable from Windows' "Installed apps" / Add-or-remove-programs list) that removes the installed files, the Start Menu shortcut, and exactly the associations the install created.
- Shows the path of the currently visible image in the window title, updated on every navigation: the full path when it fits, otherwise the file name with its extension (its last characters only when the name itself is too long) grown by the containing folder, then the drive — with `...` between the drive and the visible folder chain — and more parent folders while space allows; while the window is not maximized the title shows only the file name, and while fullscreen is on a popup at the top center shows the visible image's name for 3 seconds on every image change.
- Publishes the releases in the GitHub Releases section: this 1.1.0 release carries the wizard installer for 64-bit Windows plus the portable zip packages for 64-bit and 32-bit Windows.

### Changed

- Keeps the portable zip packages alongside the wizard installer: each zip still contains exactly the four flat files (the executable, the config file, the register-default-viewer.cmd helper, and DEFAULT-VIEWER-GUIDE.txt), and the 32-bit zip remains the only distribution for 32-bit Windows (the installer embeds the 64-bit executable only).
```

(The existing `## [1.0.0] - 2026-11-15` entry below stays byte-identical.)

### 3.7 `docs/build-windows.md` (one edit — the carried Task-5 4.5b item)

**DW1 — Line 107.** Replace:

```markdown
- The version is read dynamically from `Cargo.toml` (at execution: 1.0.0).
```

with:

```markdown
- The version is read dynamically from `Cargo.toml` (at execution: 1.1.0).
```

(the rest of the line and the file stay verbatim; no other build-windows.md edit — Task 1's 4.4 already wrote the NSIS section, and no heading changes there).

---

## 4. Step-by-Step Implementation (workflow step 4.2 — implementer, JUNIOR under 50%)

Execute in order; check the plan between steps; STOP and ask the caller on ANY ambiguity — never guess. All edits are host-side file edits (read → edit tools); NO container command exists in this task (D12).

### Step 0 — Preconditions + commit the plan file

1. `git status --short --branch` → expect `## feat/wave-8-wizard-installer` with a clean tree except the untracked plan file. Read `.gitignore`; confirm the markdown/plan paths are not ignored.
2. Stage and commit ONLY the plan file: `git add .kilo/plans/20261118-wave8-task6-docs-implementation-plan.md` → commit **`docs: add wave8 task6 documentation plan`** (Commit A, D9).
3. Gitignore Compliance Rule before EVERY commit in this task: run `git status`, verify NO `dist/`, `logs/`, `target/`, or `*.log` path is staged; never `git add -f`.

### Step 1 — Apply §3.1 (README.md)

Apply E1–E6 exactly (read the file first; the current README is 150 lines). After the edits, re-read the file and confirm: the new TOC block (11 items + 7 sub-bullets), the new window-title section sitting between v1 Features and Technology Stack, the replaced Distribution section, the appended installer-build subsection, the 1.1.0 Language row, and the appended status sentence.

### Step 2 — Apply §3.2 (tech.md)

Apply T1–T9 exactly (the current tech.md is 222 lines). After the edits, re-read and confirm: the 20-item Contents, the appended Status sentence, the 1.1.0 Language row, the harness-bullet extension, the Build-and-Run installer bullet, the new mechanism section (10 bullets), the updated census line (opening 213; the inserted `src/window_title/tests.rs` — 18 entry; the 213/29 grand total with the pinned breakdown), the new Technical-Constraints bullet, and the updated References line.

### Step 3 — Apply §3.3 (architecture.md)

Apply A1–A6 exactly (the current architecture.md is 150 lines). After the edits, re-read and confirm: the appended Status sentence, the replaced Distribution bullet, the new Window-Title bullet, the module-layout entries (the app.rs state/module-list additions + the new `src/window_title.rs` bullet), the Data-and-State append, the pipeline intro append + installer bullet + image `nsis` append, the 1.1.0 Project-Paths version + the packaging clause, and the two new decisions-table rows.

### Step 4 — Apply §3.4 (context.md)

Apply C1–C4 exactly (the current context.md is 123 lines; its `## Contents` needs NO edit — C5 verify-only). After the edits, re-read and confirm: the replaced first Current-Work-Focus entry (the older dated entries untouched), the prepended Recent-Changes entry (numbering: the new entry is `1.` at the top and the pre-existing entries keep their list style — the file's convention already repeats `1.` for all entries), the Current-Project-State updates (a–d), and the replaced Immediate-Next-Steps.

### Step 5 — Apply §3.5–§3.7 (brief.md, CHANGELOG.md, docs/build-windows.md)

Apply B1, CH1, and DW1 exactly. After the edits, re-read and confirm: the new addendum section placed before the DO-NOT-DELETE marker; the CHANGELOG's fresh empty `## [Unreleased]` skeleton followed by the filled `## [1.1.0] - 2026-11-18` entry and the byte-identical 1.0.0 entry; the single changed parenthetical on build-windows.md line 107.

### Step 6 — Verification (§5) + commit

1. Run §5's checks; fix nothing silently — any mismatch found here is either a transcription slip (fix the edit to match the plan and re-verify) or a plan defect (STOP and report).
2. Stage EXACTLY the seven files: `git add README.md .agent/project-info/tech.md .agent/project-info/architecture.md .agent/project-info/context.md .agent/project-info/brief.md CHANGELOG.md docs/build-windows.md` → commit **`docs: wave8 task6 wizard installer and window title documentation`** (Commit B, D9).
3. Final `git status`: clean tree (except gitignored/untracked artifacts). `git log --oneline -3` shows Commit B, Commit A, and `e82e061` in order.

---

## 5. Verification (4.2 runs; 4.4 re-checks)

1. **Cross-file coherence** (grep-verified — every fact identical wherever it appears):
   - Census `213` tests / `29` test-bearing files: tech.md Testing bullet; tech.md mechanism section census bullet; README status sentence; context.md Current-Work-Focus + Current-Project-State; (the task-5 results file is the source).
   - Version `1.1.0`: README Language row + distribution artifact names + status; tech.md Language row; architecture.md Project Paths; CHANGELOG heading; docs/build-windows.md line 107; context.md.
   - Artifact names: `ImagesScroller-1.1.0-windows-installer.exe` (README, tech, architecture, context), `ImagesScroller-1.1.0-windows-{x64,x86}.zip` (README, context).
   - The 17-type list (identical member order): README (the installer page + helper subsection), tech.md mechanism section.
   - Registry surface wording (`UserChoice` NEVER; per-user HKCU; pre-clean-then-write): README, tech.md, architecture.md, context.md, brief.md item 9 — all consistent.
   - Task-7 pending status: README has NO pending caveat (D6); tech.md mechanism section says "EXECUTED BY TASK 7, pending"; architecture decisions row says "the Release publish itself = Task 7, pending"; context.md Next-Steps #2; the README status sentence says "Pending: ... the GitHub Releases publish (Task 7)" (internal-status acceptable there because the sentence's Pending tail is the project-status paragraph, not the user-facing install steps).
2. **TOC anchors resolve** (re-derive each slug with the Task-2 D1 rules; every anchor must hit a real same-file heading):
   - README: 11 numbered items + 7 sub-bullets (E1) — incl. the new `#window-title-and-fullscreen-name-popup-wave-8`, `#install-with-the-wizard-installer-recommended`, `#install-from-the-portable-zip-portable-alternative`, `#make-images-scroller-the-default-image-viewer-optional-zip-installs`, `#windows-wizard-installer-build-nsis`.
   - tech.md: 20 items (T1) — incl. the new `#wave-8-wizard-installer-and-window-title-mechanisms-implemented`.
   - architecture.md: 11 items unchanged (no heading changes — verify).
   - context.md: 4 items unchanged (C5 — verify).
   - docs/build-windows.md: 11 items unchanged (verify).
3. **Heading-coverage sweep** (the Task-2 D5 closing sweep): grep each modified file for `^#{1,3} ` and confirm every content heading appears in its `## Contents` (README: 10 `##` + 7 `###`; tech: 20 `##`; build-windows: 11 `##`; context: 4 `##`; architecture: 11 `##`).
4. **Git hygiene**: `git status` shows ONLY the seven intended files as modifications (plus the committed plan file); NO gitignored path staged; no `.log` staged.
5. **Byte-identity confirmations**: `git diff --stat` for Commit B shows exactly 7 files changed; `git diff CHANGELOG.md` shows the 1.0.0 entry untouched; `git diff docs/build-windows.md` shows exactly one changed line; `git diff brief.md` shows only the inserted addendum block (no body modifications).

---

## 6. Sub-Step Mapping for Task 6

- **4.2 implementer**: Steps 0–6 of §4 (Commit A + all edits + §5 verification + Commit B). Report: what was done / what was NOT done (including the verify-only verdicts and the D3 hotkeys disposition).
- **4.3 code-reviewer + code-simplifier**: review the seven edits against THIS plan (insertion points, pinned text, anchors); simplification is NOT expected for markdown (record "not required" if so); any fix goes through the Planner (max 3 review cycles).
- **4.4 docs-specialist**: the coherence-fix pass — the FINAL TOC-coverage + cross-file-facts sweep (Task-2 D5's closing sweep). Re-run §5.1–§5.3. FIX AUTHORITY (pinned, bounded): ONLY (a) broken/mismatched TOC anchor slugs and (b) word-level factual corrections that do not change any decision pinned in this plan — applied as a single-purpose commit `docs: wave8 task6 coherence fixes`. ANY other defect (a decision-level mismatch, a missing section, a stale fact needing a rewrite) → STOP and report to the caller (no improvisation).
- **4.5b architector**: adherence check of the implementation against THIS plan; report diffs (if any) and acceptability (new report file only if diffs are found).
- **4.6 implementer**: mark `### Task 6: Documentation updates` `[DONE]` in the TODO file with BOTH its sub-items `[]` → `[x]` (preserve all other content byte-identical; do NOT touch other tasks' marks — Task 7 stays `[ ]`), commit `task: mark wave8 task6 done`.

---

## 7. NOT-to-Do List (hard prohibitions for every sub-step)

- NO changes to anything under `src/`; NO `Cargo.toml`/`Cargo.lock` change; NO `Dockerfile`/`docker-compose.yml`/`scripts/*`/`packaging/*` change (their records are DOCUMENTED, not edited).
- NO edits to `.agent/project-structure.md`, `product.md`, `docs/build.md`, `future-ideas.md`, `AGENTS.md`, or any existing `.kilo/plans/*` file (this plan is the only NEW plan file of this task).
- NO `git push` (Step 5 only); NO branch creation/switch (Step 2 only — the branch exists); NO version edit (the docs RECORD 1.1.0 as text, which is not a version edit).
- NO TODO `[DONE]` mark before 4.6; NEVER mark Task 7 as done.
- NO harness/VM/container command (D12); NO global installs.
- Do NOT describe Task 7's release as executed anywhere — it is PENDING at doc-writing time (tech.md/context.md mark it; README stays evergreen per D6).
- Do NOT remove or rewrite any historical context.md entry (prepend only).
- Gitignore Compliance Rule before every commit; real newline characters only (newline-prevention rule); keep the files' existing line endings.

---

## 8. Flags for the Caller (recorded, NOT acted on)

1. `docs/build.md`'s image-contents sentence (its build-environment paragraph) does not yet mention `nsis` — the same stale-fact class the Wave-7 Task-5 plan fixed for the i686/zip additions. It is OUT of Task 6's enumerated scope (D1/D2), so NO edit is made here; the caller may schedule the one-sentence accuracy fix in a later step (e.g. ride Task 7's docs or a maintenance sweep).
2. The Wave-7-era zips of 1.0.0 remain on disk in `dist/` alongside the fresh 1.1.0 artifacts (gitignored, never committed); the Task-7 release attaches the 1.1.0 set. No doc records the 1.0.0 zips as current — the README/context now speak only of the 1.1.0 names.

---

## 9. Plan-Verification Against the Original Task

- README installer-first install instructions ✓ (E3); the GitHub Releases pointer ✓ (kept in the Distribution intro + the mechanism wording, D6); the hotkeys record (Task 3) ✓ (the Task-3 discharge disposition pinned, D3 — nothing added); the window-title behavior ✓ (E2 — placement pinned per D4); the TOC rolling trigger ✓ (E1).
- tech.md: the installer mechanism ✓ (T6), the releases flow ✓ (T6 — pending-Task-7 wording), the association-selection registry surface ✓ (T6), the title mechanism ✓ (T6 — incl. the 0.85 fraction + the census), the build matrix incl. the installer build command ✓ (T5/T6 + the build order), the census 213/29 ✓ (T7).
- architecture.md: the Logical-Components Distribution bullet ✓ (A2a), the decisions-table rows ✓ (A6c), the Status ✓ (A1), the Data/Build-pipeline updates ✓ (A4/A5), the window-title component ✓ (A2b), the module layout ✓ (A3).
- context.md: the Current-Work-Focus rewrite ✓ (C1), the Recent-Changes entry ✓ (C2), the Current-Project-State ✓ (C3, incl. the pending OPERATOR-RUN gate), the Immediate-Next-Steps ✓ (C4), the TOC verified unchanged ✓ (C5).
- brief.md: the BINDING addendum item 9 ✓ (B1 — the user's 2026-11-17 decision pinned verbatim in substance).
- CHANGELOG.md: `## [Unreleased]` → `## [1.1.0] - 2026-11-18` with the wave's Added/Changed bullets enumerated ✓ (CH1); a fresh Unreleased skeleton kept ✓.
- The carried docs/build-windows.md fix ✓ (DW1, one line, rest verbatim).
- Commit strategy pinned ✓ (D9); sub-step mapping ✓ (§6); NOT-to-do ✓ (§7); verification incl. cross-file coherence + TOC anchors ✓ (§5). No open choices remain.
