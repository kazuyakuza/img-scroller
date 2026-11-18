# Wave 8 — Task 1 (Executable Wizard Installer) — Implementation Plan (4.1b)

- Source TODO: [`.agent/todos/20261117/20261117-todo-1.md`](../../.agent/todos/20261117/20261117-todo-1.md) → `### Task 1: Executable wizard installer` + the `## Deferred decisions` section (ALL settled here — no open choices remain).
- Global plan: [`.kilo/plans/20261118-wave8-wizard-installer-global-plan.md`](20261118-wave8-wizard-installer-global-plan.md) (NOT edited; this is a NEW file).
- Executor of the implementation: the 4.2 implementer (JUNIOR, 50% restriction). This plan encodes every structural/architectural/scope decision; nothing below requires judgment.
- Git state (verified 2026-11-18): branch `feat/wave-8-wizard-installer` exists and is checked out, working tree clean, version bump `chore: bump version to 1.1.0` landed at `4fb6b94`. This step commits NOTHING and pushes NOTHING (commits = 4.2; push = workflow Step 5 ONLY, `main` → `origin`).

[Project Info: Active] — context files read independently (brief.md, tech.md, architecture.md, context.md via the log, project-structure.md, WORKFLOWS.md, critical-workflow.md, packaging/, scripts/, docs/, Cargo.toml, LICENSE, .gitattributes, .gitignore).

---

## 0. Feasibility Verification (MANDATED before deciding — DONE 2026-11-18)

All probes ran through the `alpine-vm` MCP (`vm_status` FIRST — `vmState: running | sshReachable: true`; exit codes authoritative).

| # | Probe | Command (shape) | Evidence |
| --- | --- | --- | --- |
| V1 | VM/share reachable | `vm_status` | running, SSH-reachable — proceed |
| V2 | Debian `nsis` package exists in the PINNED image's base | disposable `img-scroller-rust:latest` container: `apt-get update -qq && apt-cache policy nsis` | exit 0; `Candidate: 3.08-3+deb12u1` from bookworm main |
| V3 | Package contents | `apt-get install -y --no-install-recommends nsis` in a disposable container, then `which makensis`, `makensis -VERSION`, header/plugin listings | exit 0; `/usr/bin/makensis` v3.08; `MUI2.nsh`, `LogicLib.nsh`, `nsDialogs.dll`, `System.dll`, and the full standard header set under `/usr/share/nsis/` |
| V4 | makensis cross-compiles a Windows installer on Linux | minimal MUI2 script → `makensis -V2 probe.nsi` | exit 0; non-empty `probe-installer.exe` (106,726 B) |
| V5 | The EXACT instruction surface compiles | second probe with nsDialogs checkbox + `WriteRegStr HKCU` (keys containing spaces), `DeleteRegValue`, `DeleteRegKey`, `ExecShell "" "ms-settings:defaultapps"`, `System::Call "shell32::SHChangeNotify(...)"`, uninstall section | exit 0; non-empty installer (128,643 B); sole warning was the probe's missing `WriteUninstaller` (the real .nsi calls it — warning disappears) |

Probe residue cleaned (`rm -rf /tmp/nsisprobe2` on the VM, exit 0).

**Verdict: NSIS 3.08 in-container cross-build is FEASIBLE and verified.** Fallbacks eliminated, not merely deprioritized: Inno Setup requires a Windows-hosted compiler (would make the installer build operator-assisted — defeats the agent-side verification model); WiX is .NET-heavy (no toolchain in the pinned image; heavier than verified NSIS). The choice is settled by the evidence, with zero remaining alternatives to adjudicate.

---

## 1. Settled Decisions (each with rationale — NO open choices)

| # | Decision | Settled value | Rationale |
| --- | --- | --- | --- |
| D1 | Installer technology | **NSIS 3.08** (`makensis`, MUI2 wizard pages, nsDialogs checkboxes) | Probes V2–V5: container-friendly cross-compiler; MUI2 destination page + custom association page + HKCU registry writes + uninstall section all verified compilable; headers/plugins ship inside the single Debian package |
| D2 | Build mechanism | **Container cross-build.** `nsis` appended to the Dockerfile's existing apt line; image rebuild; new tracked `scripts/build-installer.sh` run via docker compose on the VM | Keeps agent-side verification possible via the `alpine-vm` MCP (the whole wave's build model); Wave-7 Dockerfile-change precedent (apt line + rebuild) |
| D3 | Installer file name | **`ImagesScroller-<version>-windows-installer.exe`** → `ImagesScroller-1.1.0-windows-installer.exe` (version read DYNAMICALLY from `Cargo.toml` at build time — never hardcoded) | Same dynamic-version + naming family as the Wave-7 zips (`ImagesScroller-<v>-windows-{x64,x86}.zip`); distinguishes installer vs zip artifacts |
| D4 | Installer count / arch | **ONE installer embedding the x64 exe** (`dist/windows-x64/img-scroller.exe`). NO x86 installer | The installer is one .exe for one arch; x64 is the primary Windows audience; a 32-bit user keeps the retained x86 zip (portable). Two installers would double the wizard verification surface for a diminishing x86 audience |
| D5 | Install scope | **Per-user, HKCU-only.** `RequestExecutionLevel user` (no elevation, no UAC); default dir `InstallDir "$LOCALAPPDATA\Programs\Images Scroller"`; `SetShellVarContext current`; `InstallDirRegKey HKCU "Software\Images Scroller" "InstallDir"` (reinstall pre-fills the previous destination) | The TODO recommends per-user HKCU; consistent with the Wave-7 helper; no admin rights ever |
| D6 | Association page | **Custom nsDialogs page** between Directory and InstFiles: `Page custom ShowAssociationPage LeaveAssociationPage` AFTER `MUI_PAGE_DIRECTORY`. **Per-type checkboxes over EXACTLY the 17 supported extensions** (avif bmp dib gif heic heif ico jfif jpe jpeg jpg jxr png tif tiff webp wdp — lowercase dotted `.ext` labels), **3 columns × 6 rows (6/6/5)** at fixed nsDialogs units, **NONE preselected** | The user's binding request ("the user can select that only png files are default-opened"); none-preselected = the user explicitly picks (the TODO's recommended lean); the 17 list is `src/folder_listing.rs`'s whitelist — the single source |
| D7 | Registry surface | **REUSE the Wave-7 helper surface verbatim (HKCU-only):** ProgID `ImagesScroller.Image` (default value `Images Scroller Image`, `FriendlyTypeName`, `DefaultIcon` = `"<exe>",0`, `shell\open\command` = `"<exe>" "%1"`); per-SELECTED-type only: `HKCU\Software\Classes\.ext\OpenWithProgids` value `ImagesScroller.Image` (REG_SZ, empty data) + `HKCU\Software\Images Scroller\Capabilities\FileAssociations` value `.ext` = `ImagesScroller.Image`; Capabilities `ApplicationName`/`ApplicationIcon`/`ApplicationDescription` (same strings as the helper); `HKCU\Software\RegisteredApplications` value `Images Scroller` = `Software\Images Scroller\Capabilities`. **NEVER writes `UserChoice`** (FORBIDDEN — hash-protected, unsupported programmatically; the Wave-7 precedent governs). After writes: `System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, i 0, i 0)'` (SHCNE_ASSOCCHANGED — refreshes Explorer's Open-with cache; `System.dll` ships in the package, probe V5) | Exact reuse = zero registry-semantics drift from the tested helper; the OS constraint in the TODO preamble governs verbatim |
| D8 | Selected-vs-unselected + reinstall semantics | **Pre-clean-then-write:** before the conditional writes, the install section DELETES (value-only, absence-tolerant) the `OpenWithProgids` value + the `FileAssociations` value for ALL 17 types unconditionally; then writes entries ONLY for checkbox-selected types. Unselected types get nothing new; a previous run's selections are removed → a reinstall with only `.png` checked converges to exactly `.png` | Idempotent like the helper's `/f`-overwrite converge; enables deselect-between-runs without an extra "remove associations" mode |
| D9 | End-of-install Settings page | **Open `ms-settings:defaultapps` ONLY when at least one type was selected**: the install section sets a `$InstallHadSelections` flag if any checkbox was checked; `.onInstSuccess` → `${If} $InstallHadSelections = 1` → `ExecShell "" "ms-settings:defaultapps"`. NO selection → NO Settings page (nothing to confirm) | Mirrors the helper's hand-off (final per-type choice is user-mediated in Settings — the OS constraint); avoids a pointless Settings jump for a no-associations install |
| D10 | Installed-file set | **`img-scroller.exe` + `img-scroller.toml` + `LICENSE`** — nothing else. **NOT installed:** `register-default-viewer.cmd` and `DEFAULT-VIEWER-GUIDE.txt` (the association page replaces the helper; helper-specific guidance would be misleading next to the wizard) | MIT requires distributing the license text with the binary; the helper's fate is explicit: retired from the INSTALLER payload but RETAINED in the zips (D12) |
| D11 | Shortcut set | **Per-user Start Menu ONLY:** `CreateShortcut "$SMPROGRAMS\Images Scroller.lnk" "$INSTDIR\img-scroller.exe"` (with `SetShellVarContext current`). **NO desktop shortcut. NO run-after-install checkbox** (MUI finish page shows only Finish) | Minimal, per-user-consistent; the finish flow's action is the Settings hand-off (D9), not app launch |
| D12 | Wave-7 zips' fate | **KEPT — produced and published alongside.** `scripts/package-windows.sh`, its manifest, the helper, and the guide are UNTOUCHED (4-entry zips incl. helper + guide unchanged) | The zips remain the portable alternative AND the only x86 distribution (D4) |
| D13 | Uninstaller behavior | **`WriteUninstaller "$INSTDIR\uninstall.exe"` + an HKCU Add/Remove-Programs entry** (`HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\Images Scroller`: `DisplayName`=`Images Scroller`, `DisplayVersion`=`${PRODUCT_VERSION}`, `DisplayIcon`=`"<exe>",0`, `UninstallString`=`"<dir>\uninstall.exe"`, `NoModify`=`1`, `NoRepair`=`1`). The uninstall section: deletes the 3 installed files + the Start Menu shortcut; performs D8's unconditional value-only deletes for all 17 types; `DeleteRegKey HKCU "Software\Classes\ImagesScroller.Image"`; `DeleteRegKey HKCU "Software\Images Scroller"` (whole tree incl. Capabilities + the persisted InstallDir value); `DeleteRegValue HKCU "Software\RegisteredApplications" "Images Scroller"` (value-only — the key may host other apps); `DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Images Scroller"`; final SHChangeNotify. **NEVER deletes an extension key itself** (`.ext` keys may be Windows-owned/shared — the helper precedent deletes only the OpenWithProgids VALUE); **never touches `UserChoice`** | Removes EXACTLY what the install created; mirrors the helper's `--unregister` deletions 1:1 |
| D14 | New tracked files | **`packaging/nsis/installer.nsi`** (NSIS source — inside `packaging/` = "distribution-shipped artifacts", no new top-level folder), **`scripts/build-installer.sh`**, one **`.gitattributes` line** `packaging/nsis/*.nsi text eol=lf` (container `makensis` reads LF reliably; same rationale as `*.sh`) | Fits `.agent/project-structure.md` (updated at 4.2, step 5 below) |
| D15 | Docs guide home | **Extend `docs/build-windows.md`** with an "Windows installer packaging (NSIS)" section (the page already owns Windows build + zip packaging). NO separate `docs/installer.md` | Docs-split rule: build mechanics live in docs/build-windows.md; user-facing install prose is Task 6's README job |
| D16 | Build pipeline ordering | **`build-windows.sh x86_64-pc-windows-gnu` (x64 only feeds the installer) → `build-installer.sh` (makensis embeds `dist/windows-x64/img-scroller.exe`).** The x86 build + `package-windows.sh` feed the retained zips unchanged | D4; the installer compile is a consumer of the x64 artifact exactly like package-windows.sh consumes both |
| D17 | Version | **1.1.0 — CONFIRMED (bump already DONE at `4fb6b94`; Cargo.toml/Cargo.lock are NOT touched by Task 1).** CHANGELOG entry wording is **Task 6's job**; recorded decision only: a `## [Unreleased]` entry describing the wizard installer is released as `## [1.1.0] - <date>` | TODO's Step-3 bump note + Task-6 ownership |
| D18 | Census / tests | **Census unchanged: 195 unit tests / 28 test-bearing files. NO Rust logic exists in this task** (the wizard UI is NSIS-owned; the registry instructions are installer-runtime, not Rust). NO new tests; dev-checks still run once as a drift gate (step 7) | Global plan Task-1 pre-analysis: "Unit tests: none expected" |
| D19 | `makensis` invocation style | `makensis -V2 -DPRODUCT_VERSION=… -DPRODUCT_VERSION_4=… -DINSTALLER_OUTFILE=… -DEXE_PAYLOAD=… -DCONFIG_PAYLOAD=… -DLICENSE_PAYLOAD=… packaging/nsis/installer.nsi` — ALL paths/versions injected as **absolute container-path defines** by the build script (no cwd-relative ambiguity inside the .nsi); exit code authoritative; warnings are non-fatal (exit 0 + non-empty artifact = success) | Deterministic; the script owns the dynamic data, the .nsi owns the static structure |

---

## 2. High-Level Approach

A three-piece change, all committed by the 4.2 implementer on the existing branch:

1. **Container image** — append `nsis` to the Dockerfile's existing apt line (Wave-7 precedent: one apt line, `--no-install-recommends`, cache purge) + one `.gitattributes` line; rebuild the image; verify `makensis -VERSION` inside the image.
2. **Installer source** — `packaging/nsis/installer.nsi`: MUI2 wizard (Welcome → Directory → custom association page → InstFiles → Finish), per-user HKCU install of 3 files, Start Menu shortcut, uninstaller + Add/Remove entry, the exact Wave-7 helper registry surface written ONLY for the user-selected types, pre-clean-then-write reinstall semantics, SHChangeNotify refresh, conditional `ms-settings:defaultapps` hand-off.
3. **Build script** — `scripts/build-installer.sh`: POSIX sh, container-run, extracts the version from `Cargo.toml` (same CR-tolerant sed as `package-windows.sh`), injects defines, runs `makensis`, installs the artifact part-then-rename under gitignored `dist/`, typed `build-installer: error:` failures, never touches git.

Verification (this task, 4.2): image rebuild exit 0 → `makensis -VERSION` prints `v3.08…` → x64 build exit 0 → installer build exit 0 with non-empty `dist/ImagesScroller-1.1.0-windows-installer.exe` → dev-checks `ALL CHECKS PASSED` (census 195/28 unchanged). Windows wizard UX = Task 5's OPERATOR-RUN (hooks in §9).

---

## 3. Step-by-Step Plan (4.2 implementer)

> All console commands run through the `alpine-vm` MCP: call `vm_status` FIRST, then `vm_run_command`; the command EXIT CODE is the authoritative success evidence; absent cargo stderr is NOT a failure signal (docs/build.md). Every `sh -c` one-liner composes steps explicitly; do not invent alternates.

### Step 0 — Preconditions + Gitignore compliance

1. Confirm branch: `git status --short --branch` → expect `## feat/wave-8-wizard-installer` and a clean tree (clean tree verified 2026-11-18; if anything is unstaged, commit it FIRST per the Critical Workflow Step-2 discipline with a meaningful message).
2. Read `.gitignore` and apply the Gitignore Compliance Rule to every commit below: `dist/`, `logs/`, `target/`, `*.log` are ignored — the installer/zip/exe artifacts are NEVER staged. Before each commit run `git status` and verify no ignored path appears staged; never `git add -f`.
3. VM gate: call `vm_status`; if the VM or the `/img-scroller` share is unavailable → STOP and return a user question.

### Step 1 — Commit the plan file (untracked artifact of this 4.1b step)

- `git add .kilo/plans/20261118-wave8-task1-wizard-installer-implementation-plan.md`
- Commit: **`docs: add wave8 task1 installer implementation plan`**
- (Matches every prior wave's chain shape: the plan file lands in a docs commit at the start of 4.2.)

### Step 2 — Dockerfile + .gitattributes edits

Edit `Dockerfile` — ONE change only. Modify the existing apt line (line 16) by appending `nsis` to the package list; do NOT reorganize anything else, do NOT add a new RUN line:

```dockerfile
RUN apt-get update && apt-get install -y --no-install-recommends gcc-mingw-w64-x86-64 gcc-mingw-w64-i686 zip unzip nsis && rm -rf /var/lib/apt/lists/*
```

Update the comment block above it by appending one comment line:

```dockerfile
# nsis (makensis) cross-builds the Windows wizard installer from Linux (Wave 8).
```

Edit `.gitattributes` — append EXACTLY one line (EOF, LF endings):

```
packaging/nsis/*.nsi text eol=lf
```

### Step 3 — Image rebuild + makensis verification (commands, exit-code evidence)

1. Rebuild the image (required after the Dockerfile change; Wave-7 precedent):
   `sh -c "docker compose -f /img-scroller/docker-compose.yml build rust"` — expect exit 0.
2. Verify makensis inside the image:
   `sh -c "docker compose -f /img-scroller/docker-compose.yml run --rm rust makensis -VERSION"` — expect exit 0 and stdout starting `v3.08`.
   On failure `build-installer: error: makensis not found…` semantics apply at script level; at this step the typed expectation is: rebuild exit 0 then version probe exit 0 — any nonzero → STOP, return the failure detail to the caller.
3. Commit:
   - `git add Dockerfile .gitattributes`
   - Commit message: **`feat: add nsis to the container image for installer builds`**

### Step 4 — Create `packaging/nsis/installer.nsi`

New file `packaging/nsis/installer.nsi` (LF endings — enforced by the new .gitattributes rule). Structure pinned below: the EXACT instruction blocks for every section; the implementer composes the file in this order with NSIS-standard whitespace/comments (self-documenting; no commented-out code). Lines starting `#` are commentary here, not in the file.

**Header block:**

```nsis
OutFile "${INSTALLER_OUTFILE}"
Name "Images Scroller ${PRODUCT_VERSION}"
InstallDir "$LOCALAPPDATA\Programs\Images Scroller"
InstallDirRegKey HKCU "Software\Images Scroller" "InstallDir"
RequestExecutionLevel user
VIProductVersion "${PRODUCT_VERSION_4}"
VIAddVersionKey "FileDescription" "Images Scroller Installer"
VIAddVersionKey "ProductName" "Images Scroller"
VIAddVersionKey "FileVersion" "${PRODUCT_VERSION_4}"
VIAddVersionKey "ProductVersion" "${PRODUCT_VERSION}"
!include MUI2.nsh
!include nsDialogs.nsh
!include LogicLib.nsh
```

**Static defines (single sources of truth — no magic strings repeated):**

```nsis
!define PRODUCT_NAME "Images Scroller"
!define PROGID "ImagesScroller.Image"
!define APP_KEY "Software\Images Scroller"
!define REG_APP_NAME "Images Scroller"
!define UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\Images Scroller"
!define EXTENSION_LIST "avif bmp dib gif heic heif ico jfif jpe jpeg jpg jxr png tif tiff webp wdp"
```

(Note: `!define PRODUCT_NAME` may shadow nothing in NSIS — `Name` above uses `${PRODUCT_VERSION}` injected by the script; keep both consistent. `${PROGID}`/`${APP_KEY}`/`${REG_APP_NAME}`/`${UNINST_KEY}`/`${PRODUCT_NAME}` are used in every registry/shortcut instruction below instead of repeating literals.)

**Per-extension macro set** (declared once, invoked 17× per consumer in the fixed order above — no loops anywhere):

```nsis
!macro ASSOC_DECLARE ext
  Var CheckboxHwnd${ext}
  Var CheckboxSelected${ext}
!macroend

!macro ASSOC_CREATE ext col row
  ${NSD_CreateCheckBox} ${col}u ${row}u 42u 12u ".${ext}"
  Pop $CheckboxHwnd${ext}
!macroend

!macro ASSOC_READ ext
  ${NSD_GetState} $CheckboxHwnd${ext} $0
  ${If} $0 = ${BST_CHECKED}
    StrCpy $CheckboxSelected${ext} "1"
    StrCpy $InstallHadSelections "1"
  ${Else}
    StrCpy $CheckboxSelected${ext} "0"
  ${EndIf}
!macroend

!macro ASSOC_CLEAN ext
  DeleteRegValue HKCU "Software\Classes\.${ext}\OpenWithProgids" "${PROGID}"
  DeleteRegValue HKCU "${APP_KEY}\Capabilities\FileAssociations" ".${ext}"
!macroend

!macro ASSOC_WRITE ext
  ${If} $CheckboxSelected${ext} = "1"
    WriteRegStr HKCU "Software\Classes\.${ext}\OpenWithProgids" "${PROGID}" ""
    WriteRegStr HKCU "${APP_KEY}\Capabilities\FileAssociations" ".${ext}" "${PROGID}"
  ${EndIf}
!macroend
```

Plus the install-section registry writes for the Capabilities tree itself:

```nsis
!macro WRITE_CAPABILITIES
  WriteRegStr HKCU "${APP_KEY}\Capabilities" "ApplicationName" "${REG_APP_NAME}"
  WriteRegStr HKCU "${APP_KEY}\Capabilities" "ApplicationIcon" '"$INSTDIR\img-scroller.exe",0'
  WriteRegStr HKCU "${APP_KEY}\Capabilities" "ApplicationDescription" "Classic gallery-style image viewer: open one image and scroll through the sibling images and sibling folders with the arrow keys."
  WriteRegStr HKCU "Software\RegisteredApplications" "${REG_APP_NAME}" "Software\Images Scroller\Capabilities"
!macroend
```

The ProgID writes (plain instructions in the install section, NOT macro'd — written exactly once):

```nsis
WriteRegStr HKCU "Software\Classes\${PROGID}" "" "Images Scroller Image"
WriteRegStr HKCU "Software\Classes\${PROGID}" "FriendlyTypeName" "Images Scroller Image"
WriteRegStr HKCU "Software\Classes\${PROGID}\DefaultIcon" "" '"$INSTDIR\img-scroller.exe",0'
WriteRegStr HKCU "Software\Classes\${PROGID}\shell\open\command" "" '"$INSTDIR\img-scroller.exe" "%1"'
```

**Global vars** (NSIS `Var` directives at top level, after the includes):

```nsis
Var Dialog
Var InstallHadSelections
```

(the per-extension `CheckboxHwnd*`/`CheckboxSelected*` vars come from `ASSOC_DECLARE`).

**Pages** (exact order):

```nsis
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
Page custom ShowAssociationPage LeaveAssociationPage
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_LANGUAGE English
```

(MUI_LANGUAGE LAST — probe V4's benign warning came from page directives mixed around it; with MUI_PAGE_* macros + MUI_LANGUAGE last the tree is warning-clean.)

**Association page functions** (17 `ASSOC_CREATE` invocations with EXPLICIT coordinates; column pitch 50u starting at x 0u, row pitch 13u starting at y 26u — 26u leaves room for the intro label; 6 rows per column for columns 1–2, 5 for column 3):

```nsis
Function ShowAssociationPage
  !insertmacro MUI_HEADER_TEXT "File associations" "Choose which image file types Images Scroller will be offered for."
  nsDialogs::Create 1018
  Pop $Dialog
  ${NSD_CreateLabel} 0u 0u 250u 16u "Select the image file types to offer Images Scroller for. You confirm the actual defaults in Windows Settings, which opens after the install if any type is selected."
  Pop $0
  !insertmacro ASSOC_CREATE avif 0 26
  !insertmacro ASSOC_CREATE bmp 0 39
  !insertmacro ASSOC_CREATE dib 0 52
  !insertmacro ASSOC_CREATE gif 0 65
  !insertmacro ASSOC_CREATE heic 0 78
  !insertmacro ASSOC_CREATE heif 0 91
  !insertmacro ASSOC_CREATE ico 50 26
  !insertmacro ASSOC_CREATE jfif 50 39
  !insertmacro ASSOC_CREATE jpe 50 52
  !insertmacro ASSOC_CREATE jpeg 50 65
  !insertmacro ASSOC_CREATE jpg 50 78
  !insertmacro ASSOC_CREATE jxr 50 91
  !insertmacro ASSOC_CREATE png 100 26
  !insertmacro ASSOC_CREATE tif 100 39
  !insertmacro ASSOC_CREATE tiff 100 52
  !insertmacro ASSOC_CREATE webp 100 65
  !insertmacro ASSOC_CREATE wdp 100 78
  nsDialogs::Show
FunctionEnd

Function LeaveAssociationPage
  StrCpy $InstallHadSelections "0"
  !insertmacro ASSOC_READ avif
  !insertmacro ASSOC_READ bmp
  !insertmacro ASSOC_READ dib
  !insertmacro ASSOC_READ gif
  !insertmacro ASSOC_READ heic
  !insertmacro ASSOC_READ heif
  !insertmacro ASSOC_READ ico
  !insertmacro ASSOC_READ jfif
  !insertmacro ASSOC_READ jpe
  !insertmacro ASSOC_READ jpeg
  !insertmacro ASSOC_READ jpg
  !insertmacro ASSOC_READ jxr
  !insertmacro ASSOC_READ png
  !insertmacro ASSOC_READ tif
  !insertmacro ASSOC_READ tiff
  !insertmacro ASSOC_READ webp
  !insertmacro ASSOC_READ wdp
FunctionEnd
```

(Do NOT add `!insertmacro MUI_HEADER_TEXT` before the MUI_PAGE macros — it lives inside the show function only, per probe V5.)

**`.onInit`** (per-user shell context; keep short — ≤50-line rule does not bind .nsi, but keep it minimal anyway):

```nsis
Function .onInit
  SetShellVarContext current
FunctionEnd
```

**Install section** (exact order: payload → shortcut → persisted dir → registry pre-clean → ProgID → conditional per-type writes → Capabilities → SHChangeNotify → uninstaller + Add/Remove entry):

```nsis
Section "Install" SEC_INSTALL
  SetOutPath "$INSTDIR"
  File /oname=img-scroller.exe "${EXE_PAYLOAD}"
  File /oname=img-scroller.toml "${CONFIG_PAYLOAD}"
  File /oname=LICENSE "${LICENSE_PAYLOAD}"
  CreateShortcut "$SMPROGRAMS\Images Scroller.lnk" "$INSTDIR\img-scroller.exe"
  WriteRegStr HKCU "${APP_KEY}" "InstallDir" "$INSTDIR"

  ; Reinstall convergence: drop every previously-created per-type entry first (D8)
  !insertmacro ASSOC_CLEAN avif
  !insertmacro ASSOC_CLEAN bmp
  !insertmacro ASSOC_CLEAN dib
  !insertmacro ASSOC_CLEAN gif
  !insertmacro ASSOC_CLEAN heic
  !insertmacro ASSOC_CLEAN heif
  !insertmacro ASSOC_CLEAN ico
  !insertmacro ASSOC_CLEAN jfif
  !insertmacro ASSOC_CLEAN jpe
  !insertmacro ASSOC_CLEAN jpeg
  !insertmacro ASSOC_CLEAN jpg
  !insertmacro ASSOC_CLEAN jxr
  !insertmacro ASSOC_CLEAN png
  !insertmacro ASSOC_CLEAN tif
  !insertmacro ASSOC_CLEAN tiff
  !insertmacro ASSOC_CLEAN webp
  !insertmacro ASSOC_CLEAN wdp

  <the four ProgID WriteRegStr lines from above, verbatim>
  !insertmacro ASSOC_WRITE avif   ; … then all 17, fixed order
  …
  !insertmacro WRITE_CAPABILITIES
  System::Call "shell32::SHChangeNotify(i 0x08000000, i 0, i 0, i 0)"
  WriteUninstaller "$INSTDIR\uninstall.exe"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayName" "${PRODUCT_NAME}"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayIcon" '"$INSTDIR\img-scroller.exe",0'
  WriteRegStr HKCU "${UNINST_KEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoModify" 1
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoRepair" 1
SectionEnd
```

**Uninstall section** (exact deletions — D13 verbatim):

```nsis
Section "un.Uninstall" SEC_UNINSTALL
  Delete "$INSTDIR\img-scroller.exe"
  Delete "$INSTDIR\img-scroller.toml"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"
  Delete "$SMPROGRAMS\Images Scroller.lnk"
  !insertmacro ASSOC_CLEAN avif   ; … all 17, fixed order (value-only deletes; extension keys NEVER deleted)
  …
  DeleteRegKey HKCU "Software\Classes\${PROGID}"
  DeleteRegKey HKCU "${APP_KEY}"
  DeleteRegValue HKCU "Software\RegisteredApplications" "${REG_APP_NAME}"
  DeleteRegKey HKCU "${UNINST_KEY}"
  System::Call "shell32::SHChangeNotify(i 0x08000000, i 0, i 0, i 0)"
SectionEnd
```

**`.onInstSuccess`** (D9):

```nsis
Function .onInstSuccess
  ${If} $InstallHadSelections = "1"
    ExecShell "" "ms-settings:defaultapps"
  ${EndIf}
FunctionEnd
```

Hard requirements for this file: LF endings; UTF-8/ASCII only (all strings above are ASCII); NO `UserChoice` string anywhere; no commented-out code; the 17-macro invocation order matches `EXTENSION_LIST` exactly in every block (declare, create, read, clean, write); the fixed 17-order is also the page's visual order.

### Step 5 — Create `scripts/build-installer.sh` + update `.agent/project-structure.md`

New file `scripts/build-installer.sh` (LF; `*.sh text eol=lf` already covers it). Structure — mirror `scripts/package-windows.sh`'s discipline exactly (`set -u`, readonly constants, typed `report_failure`, EXIT-trap staging cleanup where applicable, never touches git):

```sh
#!/bin/sh
# Windows wizard-installer cross-build for Images Scroller (NSIS, x64 payload).
# Runs inside the container:
#   docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh
# Consumes the x64 release exe produced by scripts/build-windows.sh plus the tracked
# installer source (packaging/nsis/installer.nsi), config file and LICENSE, and
# compiles ONE installer executable into the gitignored dist/ tree:
#   dist/ImagesScroller-<version>-windows-installer.exe   (version read from Cargo.toml)
# Typed failures: every failure line starts with the literal prefix
# "build-installer: error:" and the script exits 1. A failed run never deletes or
# modifies a previously installed installer artifact; the transient .part file is
# removed on failure. This script never touches git.
```

Required behavior (pinned):

1. `set -u`; `readonly PROJECT_DIR="/img-scroller"`; `assert_current_directory` (same body as package-windows.sh, prefix `build-installer:`).
2. `report_failure()` → `printf 'build-installer: error: %s\n' "$1" >&2; exit 1`.
3. `command -v makensis >/dev/null 2>&1` → else fail `"makensis not found (rebuild the container image after the Dockerfile change)"`.
4. Assert sources present (`-s`): `packaging/nsis/installer.nsi`, `dist/windows-x64/img-scroller.exe` (fail text: `"expected payload not found or empty: <path> (produce the x64 exe with scripts/build-windows.sh first)"`), `packaging/img-scroller.toml`, `LICENSE`.
5. `extract_package_version` — copy package-windows.sh's version function VERBATIM (same CR-strip + anchored sed + empty-check; do not refactor the other script).
6. Compute `version4="${package_version}.0"`, `out_part="dist/ImagesScroller-${package_version}-windows-installer.exe.part"`, `out_final="dist/ImagesScroller-${package_version}-windows-installer.exe"`.
7. `makensis -V2 -DPRODUCT_VERSION="${package_version}" -DPRODUCT_VERSION_4="${version4}" -DINSTALLER_OUTFILE="/img-scroller/${out_part}" -DEXE_PAYLOAD="/img-scroller/dist/windows-x64/img-scroller.exe" -DCONFIG_PAYLOAD="/img-scroller/packaging/img-scroller.toml" -DLICENSE_PAYLOAD="/img-scroller/LICENSE" packaging/nsis/installer.nsi || { rm -f "$out_part"; report_failure "makensis compile failed (exit $?); see the makensis diagnostics above"; }`
8. `[ -s "$out_part" ] || { rm -f "$out_part"; report_failure "installer artifact missing or empty after compile: $out_part"; }`
9. `mv "$out_part" "$out_final" || { rm -f "$out_part"; report_failure "install of installer artifact failed: $out_part -> $out_final"; }`
10. `printf 'build-installer: artifact: %s\n' "$out_final"`; `printf 'build-installer: OK\n'`; `exit 0`.

`.agent/project-structure.md` — TWO edits (preserve all other content byte-identical):
- In `# Other folders`, extend the `packaging/` bullet by appending after the existing helper/guide text: `; nsis/installer.nsi (the Wave-8 NSIS wizard-installer source: MUI2 destination page + a custom nsDialogs association-selection page over the 17 whitelisted extensions writing the Wave-7 HKCU helper registry surface for the user-selected types only, Start Menu shortcut, uninstaller + HKCU Add/Remove entry; compiled container-side by scripts/build-installer.sh into dist/ImagesScroller-<version>-windows-installer.exe)`.
- In `# Other folders`, extend the `scripts/` bullet: append `and the Windows wizard-installer NSIS cross-build (Wave 8)` to the existing description so it reads `…, portable zip packaging, and the Windows wizard-installer NSIS cross-build (POSIX sh)`.
- Commit both files + the .nsi: `git add packaging/nsis/installer.nsi scripts/build-installer.sh .agent/project-structure.md` → commit message: **`feat: nsis wizard installer source and build script`**

### Step 6 — Build + verification run (this task's formal evidence)

In this order, each through the `alpine-vm` MCP (`vm_status` first; exit code authoritative):

1. x64 release build (feeds BOTH the installer and the x64 zip; the x86 zip keeps its own target):
   `sh -c "docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu"` — expect exit 0 + `build-windows: artifact: dist/windows-x64/img-scroller.exe`.
2. Installer build:
   `sh -c "docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh"` — expect exit 0, output line `build-installer: artifact: dist/ImagesScroller-1.1.0-windows-installer.exe`, `build-installer: OK`.
3. Artifact corroboration:
   `sh -c "ls -la /img-scroller/dist/ImagesScroller-1.1.0-windows-installer.exe"` — expect exit 0 and a non-empty file (an installer embedding a ~19.6 MB exe is expected in the ~20 MB range; record the byte size in the completion summary — never committed).
4. Drift gate (no Rust changes happened — census must hold):
   `sh -c "docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh"` — expect `ALL CHECKS PASSED`, exit 0, `195 passed; 0 failed`, 28 test-bearing files (census unchanged per D18).
5. Spot checks of the typed failure path (EXPECTED nonzero exits — run to prove the discipline, per the Wave-7 precedent):
   - `sh -c "docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh bogus"` — the script ignores/never validates extra args by design (step 7 above has NO argument handling) — SKIP this probe; instead probe the missing-artifact path by temporarily renaming the exe? NO — do not touch dist/ files. The pinned failure probes are the WRONG-WORKING-DIRECTORY and MISSING-SOURCE paths run WITHOUT the share trick... DECIDED: run exactly ONE typed probe, the missing-payload path, by pointing the probe at a harmless empty state: `sh -c "docker compose -f /img-scroller/docker-compose.yml run --rm -w /img-scroller rust sh -c 'mv dist/windows-x64/img-scroller.exe dist/windows-x64/img-scroller.exe.hold; sh scripts/build-installer.sh; rc=\$?; mv dist/windows-x64/img-scroller.exe.hold dist/windows-x64/img-scroller.exe; exit \$rc'"` — expect exit 1, stderr line `build-installer: error: expected payload not found or empty: … (produce the x64 exe with scripts/build-windows.sh first)`, and the exe RESTORED afterward (the probe moves it back; the final `exit $rc` preserves the evidence). If the restore fails, STOP and report immediately.
6. Zip pipeline UNTOUCHED proof (D12): run `sh -c "docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh"` — expect exit 0 and BOTH `ImagesScroller-1.1.0-windows-*.zip` lines (the 4-entry manifest unchanged). Record both zip names in the summary.

### Step 7 — Commits + hand-off summary

- Commits already made: Step 1 (plan), Step 3 (Dockerfile+.gitattributes), Step 5 (.nsi + script + structure map). NOTHING else is committed (dist/ artifacts are gitignored and never committed; verification results files belong to Task 5).
- Final `git status` must show a clean tree (except untracked gitignored dist/ content) and `git log --oneline -3` must show the three commits in order.
- Return to the caller: the summary (what was done / NOT done), the artifact byte sizes, and the confirmation that all verification commands exited 0 (or the typed-probe exited 1 as designed).

### Step 8 — What the 4.2 implementer must NOT do

- Do NOT touch `Cargo.toml`, `Cargo.lock`, any `src/` file, `scripts/build-windows.sh`, `scripts/package-windows.sh`, `packaging/register-default-viewer.cmd`, `packaging/DEFAULT-VIEWER-GUIDE.txt`, `packaging/img-scroller.toml`, `README.md`, `docs/*`, `CHANGELOG.md` (Task 6), or any existing `.kilo/plans/*` file.
- Do NOT run `cargo` commands outside the container harness; do NOT install anything on the host or globally (`never-global-installs.md`).
- Do NOT push (Step 5 only); do NOT create branches (Step 2 only — the branch already exists).
- Do NOT write `UserChoice` anywhere in the .nsi; do NOT delete any `.ext` registry KEY (value-only deletes under extension keys); do NOT register types the user did not select.
- Do NOT stage or commit anything under `dist/` or any `.log` file.

---

## 4. Failure Modes (typed)

| Failure | Signal | Disposition |
| --- | --- | --- |
| VM or `/img-scroller` share unavailable | `vm_status` not running/SSH-unreachable | STOP; return a user question (standing rule) |
| Image rebuild fails | nonzero `docker compose build` exit | STOP; return the build error to the caller (do not improvise Dockerfile fixes) |
| makensis missing in image | `build-installer: error: makensis not found (rebuild the container image after the Dockerfile change)` | Rebuild the image (Step 3 was skipped/failed); retry once, then STOP |
| Payload missing | `build-installer: error: expected payload not found or empty: …` | Run the x64 build first (Step 6.1), then retry |
| makensis compile failure | `build-installer: error: makensis compile failed (exit N)` + makensis diagnostics | Fix the .nsi at the reported line; common causes: define name mismatch, `${ext}` macro-name concatenation typos, a missing `!insertmacro` in one of the five 17-blocks; retry; STOP after 3 attempts |
| Artifact empty after compile | `build-installer: error: installer artifact missing or empty after compile: …` | Treat as compile failure (probe V4 showed a partial file only on hard abort) |
| Version extraction fails | `build-installer: error: cannot extract the package version from Cargo.toml` | STOP (indicates a Cargo.toml drift — versions are Step-3-owned, not Task-1's) |
| Drift gate census mismatch | dev-checks fails or census ≠ 195/28 | STOP; an unexpected Rust change happened — report to the caller |
| Zero-exit-but-empty-stdout probes | MCP capture quirk (absent cargo stderr) | Exit code is authoritative per docs/build.md — do not treat absent stderr as failure |

---

## 5. Documentation Ownership Map (for later steps — recorded, not executed here)

- **4.4 (docs-specialist)**: extend `docs/build-windows.md` with the NSIS installer section (invocation command, artifact name `ImagesScroller-<version>-windows-installer.exe`, the define-injection mechanism, the x64-payload-only note + x86-zip rationale, typed-failure table rows for the `build-installer:` classes, the image now shipping `nsis`, the association-page/registry surface summary, the pre-clean-then-write + uninstall semantics). NO README prose (Task 6 owns the user-facing install instructions).
- **Task 6**: README installer-first install instructions; tech.md/architecture.md/context.md records; `brief.md` addendum; `CHANGELOG.md` `## [Unreleased]` entry (wording per D17).
- **Task 5 verification hooks** (4.5b + Task-5 plan consumers): (a) installer build exit 0 + non-empty artifact via the alpine-vm MCP; (b) `package-windows.sh` exit 0 + both zips (retained); (c) dev-checks `ALL CHECKS PASSED` with census 195/28 UNCHANGED; (d) OPERATOR-RUN (agent-side impossible from the container): run `ImagesScroller-1.1.0-windows-installer.exe` on a real Windows 10/11 machine → wizard pages appear in order; destination default `%LOCALAPPDATA%\Programs\Images Scroller`; the 17-type association page with NONE preselected; checking only `.png` then installing lands EXACTLY the Wave-7 helper surface for `.png` only (Open With list + Capabilities + RegisteredApplications; NO UserChoice write); Settings opens automatically; app launches from the install location; uninstaller removes the 3 files + shortcut + exactly the created associations; GitHub release (Task 7) attaches the installer + both zips.

## 6. Plan-Verification Against the Original Task

- Wizard asks destination ✓ (MUI_PAGE_DIRECTORY, D5). Installs exe + config (+ LICENSE per architector decision) ✓ (D10). Association-selection page with per-type checkboxes over EXACTLY the 17 supported types, e.g. only `.png` ✓ (D6/D7). Shortcut set decided ✓ (D11). Uninstaller removes files, shortcuts, and the created associations ✓ (D13). Technology + build mechanism + file name/versioning decided with VERIFIED feasibility ✓ (§0, D1–D4, D19). Install scope + defaults + registry surface + uninstall scope decided ✓ (D5–D9, D13). Helper/zip fates explicit ✓ (D10, D12). CHANGELOG decision recorded (Task 6 executes) ✓ (D17). Census/versions untouched; no commits/pushes in this step; branch exists ✓. No open choices remain.
