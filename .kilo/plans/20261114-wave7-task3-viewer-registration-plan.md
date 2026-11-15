# Wave 7 — Task 3 Implementation Plan: Default-Viewer Registration Helper/Guidance

- Source TODO: `.agent/todos/20261029/20261029-todo-9.md` → `### Task 3: Default-viewer registration helper/guidance`. Verbatim sub-items: (1) "The package includes a helper and/or guidance that helps the user register the app as the default image viewer (file associations) in Windows."; (2) "The helper mechanism (script vs executable, exact mechanism) is decided in this wave's architector step — this wave file must not pre-select it."
- Global plan (approved): `.kilo/plans/20261114-wave7-distribution-mvp.md` → `## Task 3 — Default-viewer registration helper/guidance` + Wave-level decision #3 (STANDING CONSTRAINT: the helper must NOT add a new GUI surface to the app or the package — a GUI helper is out of v1 scope) and decision #5 (packaging-manifest single source in `scripts/package-windows.sh`'s `package_manifest_entries` — Task 3 appends its entries as additional lines).
- Task 1 plan (landed): `.kilo/plans/20261114-wave7-task1-config-plan.md` — `packaging/img-scroller.toml` exists; Task 3's files join `packaging/`.
- Task 2 plan (landed): `.kilo/plans/20261114-wave7-task2-installer-packaging-plan.md` — manifest format `<arch>|<source path>|<zip entry name>`; flat zip layout; one zip per arch; zips gitignored; build matrix D4.
- Date: 2026-11-14. Status: FOR PLANNER PRESENTATION (pending user approval before 4.2 runs).
- Branch: `feat/wave-7-distribution` (created at workflow Step 2 — NOT this task's job). Version at execution: 1.0.0 (bumped at Step 3 — NOT this task's job; this task NEVER edits version fields).
- Front-end related: NO (Wave-level decision #3 — no 4.1a, no 4.5a; this is the 4.1b architector settlement).
- High-level approach: a SINGLE packaged Windows batch helper (`register-default-viewer.cmd`) that registers the app HKCU-only into the system's choice lists (ProgID + per-extension OpenWithProgids + Capabilities/RegisteredApplications — the modern way an app appears in Settings > Apps > Default apps) and then OPENS the Settings Default apps page where the user makes the final per-type choice (Windows 10/11 protect the programmatically-set default, so the helper never claims a default itself); plus a packaged plain-text guidance file and the manifest extension into both zips.

## 0. Deferred-Decision Settlement (BINDING — this section settles the wave's deferred decision "Default-viewer registration helper mechanism")

Every decision below is final for Task 3's implementation. The implementer must not re-choose.

### D1 — Mechanism: ONE Windows batch script (.cmd), no .ps1, no console executable

- **Decision: option (a) — a single `.cmd` batch script** (`packaging/register-default-viewer.cmd`) using the built-in `reg.exe` (`reg add` / `reg delete`) plus `start` of the Settings page. NO `.ps1` + launcher, NO console executable.
- Why not a PowerShell script: a `.ps1` double-click opens the editor by default (not runnable by double-click), so a thin `.cmd` launcher would be needed anyway; and `-ExecutionPolicy Bypass` handling adds machinery with ZERO functional gain — `reg add`/`reg delete`/`start` cover everything a .ps1 could do here.
- Why not a console executable: a second build target (x64+x86 exe pair), build-script/Dockerfile scope, and the census would stay unchanged anyway — a script expresses this with zero build surface. Discouraged per the task framing; rejected.
- No-GUI constraint: honored — the script prints plain console text (its own console window when double-clicked) and opens the Settings page; it adds NO GUI surface to the app or the package (Wave-level decision #3).
- HKCU-only: every registry path below is under `HKCU` — the script never requires administrator rights and never touches `HKLM`.
- Protected-key constraint: the script NEVER writes or deletes `UserChoice` (the per-user default choice under `...\FileExts\<ext>\UserChoice` is hash-protected on Windows 10/11; programmatic default setting is unsupported). The helper registers the app INTO the system's choice lists (it appears in the per-type "Open with" lists and in Settings > Apps > Default apps) and then OPENS `ms-settings:defaultapps` where the user makes the final per-type choice. A pure `OpenWithProgids`-style registration is INCLUDED (it adds the app to the "Open with" lists) but is combined with the Capabilities registration (the modern mechanism) — see the exact keys below.
- Portable: the script derives the installed exe path from its OWN location at run time (`%~dp0img-scroller.exe` — `%~dp0` is the script's folder with a trailing backslash). The helper lives in the install folder next to `img-scroller.exe` (flat zip root, D4); no hardcoded install path anywhere. A `--unregister` run does not need the exe present (it deletes keys by name, not by path check).
- Idempotent: `reg add ... /f` overwrites with the SAME values, so re-running register is safe and converges to the same state; `reg delete ... /f` (with stderr suppressed) tolerates already-absent keys/values, so re-running unregister is safe.
- Minimal-blast-radius: the script creates ONLY the keys/values listed in D1b and deletes ONLY those in D5; it never deletes or modifies an extension's default value, another program's ProgID, or anything else. Known property (recorded, accepted): `reg add` failures surface via reg.exe's own stderr messages; the script does not aggregate them (each `reg add` is independent; a partial registration still shows whatever registered, and re-running retries).
- No `SHChangeNotify` (DECIDED — not included): a pure batch script cannot call it reliably; the registration does NOT modify any extension's default value or icon (only additions to OpenWithProgids/Capabilities/RegisteredApplications), and the Settings Default apps page and the Explorer "Open with" menu are computed on demand, so a cache-refresh call is not needed for the registered surface. Recorded as a considered-not-needed item.
- Deep link to the app's own Settings page (`ms-settings:defaultapps?registeredAppUser=Images%20Scroller`) — CONSIDERED, NOT CHOSEN: the generic `ms-settings:defaultapps` page is version-robust across ALL Windows 10/11 builds (no parameter-support variance); the guidance text (D6) covers the two extra clicks to find the app.

### D1b — EXACT registry surface (the complete key/value contract, both modes)

REGISTER mode (no argument or double-click) — `APP_EXE` = `%~dp0img-scroller.exe`:

1. The application ProgID (ONE ProgID for all 17 extensions — allowed: one ProgID may map many extensions; the extensions are the same file class):

```
HKCU\Software\Classes\ImagesScroller.Image
  (default)            REG_SZ   "Images Scroller Image"
  FriendlyTypeName     REG_SZ   "Images Scroller Image"
HKCU\Software\Classes\ImagesScroller.Image\DefaultIcon
  (default)            REG_SZ   "<APP_EXE>,0"          (quoted: "C:\...\img-scroller.exe",0)
HKCU\Software\Classes\ImagesScroller.Image\shell\open\command
  (default)            REG_SZ   "<APP_EXE> %1"         (quoted: "C:\...\img-scroller.exe" "%1")
```

2. Per-extension "Open with" entries — 17 values, one per extension (value name = the ProgID; value type REG_SZ with EMPTY data — the documented OpenWithProgids convention "empty string or null value"):

```
HKCU\Software\Classes\<ext>\OpenWithProgids
  ImagesScroller.Image   REG_SZ   (empty)
```

   for `<ext>` = each of `.avif`, `.bmp`, `.dib`, `.gif`, `.heic`, `.heif`, `.ico`, `.jfif`, `.jpe`, `.jpeg`, `.jpg`, `.jxr`, `.png`, `.tif`, `.tiff`, `.webp`, `.wdp` (exactly the 17-extension whitelist of `src/folder_listing.rs` `IMAGE_FILE_EXTENSIONS` — D2; keys are case-insensitive in the registry, written lowercase with the leading dot).

3. The Capabilities registration (the modern way the app appears in Settings > Apps > Default apps and in the per-type choosers):

```
HKCU\Software\Images Scroller\Capabilities
  ApplicationName           REG_SZ   "Images Scroller"
  ApplicationIcon           REG_SZ   "<APP_EXE>,0"            (quoted, as in 1)
  ApplicationDescription    REG_SZ   "Classic gallery-style image viewer: open one image and scroll through the sibling images and sibling folders with the arrow keys."
HKCU\Software\Images Scroller\Capabilities\FileAssociations
  .avif ... .wdp            REG_SZ   "ImagesScroller.Image"   (17 values, name = the dotted extension, data = the ProgID)
```

   Note (Microsoft contract): `ApplicationName` must match the name registered under `RegisteredApplications` — both are `Images Scroller`.

4. The RegisteredApplications pointer (the ONE key outside `Software\Classes` and the Capabilities key):

```
HKCU\Software\RegisteredApplications
  Images Scroller           REG_SZ   "Software\Images Scroller\Capabilities"
```

5. After ALL `reg add` calls: print the success text, then `start "" "ms-settings:defaultapps"` (the generic Settings Default apps page — D1's deep-link decision), then `exit /b 0`.

UNREGISTER mode (`--unregister`) — deletes EXACTLY what register created, tolerant of absence (`>nul 2>&1` on every delete), NO other writes:

```
reg delete "HKCU\Software\Classes\ImagesScroller.Image" /f
reg delete "HKCU\Software\Classes\<ext>\OpenWithProgids" /v "ImagesScroller.Image" /f      (17 deletes, value-only)
reg delete "HKCU\Software\Images Scroller" /f                                              (the Capabilities tree)
reg delete "HKCU\Software\RegisteredApplications" /v "Images Scroller" /f                  (value-only)
```

Then print the removal text and `exit /b 0`. The extension keys themselves and every OTHER program's registration are NEVER touched (value-only deletes under the extension keys).

ARGUMENT handling (single-section conditions — no chained `if`s): empty argument → register; exactly `--unregister` → unregister; ANY other argument → print `Unknown argument: <arg>` + the usage line and `exit /b 1`. REGISTER mode additionally fails with `exit /b 1` when `%~dp0img-scroller.exe` does not exist (the double-clicked-from-inside-the-zip case — the script is not extracted yet).

### D2 — Extension coverage: EXACTLY the fixed 17-extension whitelist

- Read from `src/folder_listing.rs` line 11–14, `IMAGE_FILE_EXTENSIONS`: `avif, bmp, dib, gif, heic, heif, ico, jfif, jpe, jpeg, jpg, jxr, png, tif, tiff, webp, wdp`.
- The registry writes use lowercase dotted forms (`.avif` ... `.wdp`) in BOTH the OpenWithProgids value keys and the Capabilities FileAssociations value names — registry keys are case-insensitive; lowercase is the convention. The order in the script's `for` lists is EXACTLY the whitelist order above (same string as the Rust constant, dotted in the registry writes).
- No other extension is ever registered.

### D3 — File set + names + locations + line-ending pinning

- **Tracked source files (both in `packaging/`, joining the Task-1 config):**
  1. `packaging/register-default-viewer.cmd` — the helper (CRLF, ASCII-only, UTF-8 WITHOUT BOM — a BOM would break `@echo off`; 64 lines + trailing newline, exact content in §3 Step 1).
  2. `packaging/DEFAULT-VIEWER-GUIDE.txt` — the packaged guidance (LF, ASCII-only, 69 lines + trailing newline, exact content in §3 Step 2).
- NO `.ps1` file exists anywhere; NO console executable; NO repo-root script; NO Rust file.
- Runtime names == repo names (copied verbatim by the packaging step, same names at the zip root — no rename anywhere).
- **Format choice for the guidance: `.txt`, NOT `.md`** — a `.txt` opens in Notepad by double-click on every Windows 10/11 machine; a `.md` may open in nothing or an editor the user does not have. The guide is plain text formatted for Notepad reading.
- **`.gitattributes` — DECIDED and PINNED (2 new lines appended at the end, after the existing 2):**

```
packaging/*.txt text eol=lf
packaging/*.cmd text eol=crlf
```

- Line-ending rationale (verified reasoning, pinned): (a) the `.cmd` MUST be CRLF in the packaged zip — cmd.exe batch files are traditionally CRLF and LF-only batch files have known `goto`/label-parsing quirks on some cmd versions, so CRLF is the safe choice; `text eol=crlf` forces a CRLF WORKING TREE on every platform checkout, and the packaging script copies the working-tree file into the zip, so the packaged copy is CRLF by construction (the repo-normalized form stays LF — irrelevant to the zip). (b) The `.txt` guide ships LF — Notepad on Windows 10/11 renders LF fine (since 2018) and this matches the Task-1 TOML precedent. No generic `*.cmd`/`*.txt` rules (scope-escape prevention — same reasoning as Task 1's no-generic-`*.toml` rule).

### D4 — Packaging integration (Wave-level decision #5 — manifest lines ONLY, plus two comment refreshes)

- **The EXACT 4 manifest lines appended** to `package_manifest_entries` (after the existing 4 lines, grouped by architecture mirroring the existing pattern — x64 pair first, then x86 pair):

```sh
  printf '%s\n' "$X64_ARCH|packaging/register-default-viewer.cmd|register-default-viewer.cmd"
  printf '%s\n' "$X64_ARCH|packaging/DEFAULT-VIEWER-GUIDE.txt|DEFAULT-VIEWER-GUIDE.txt"
  printf '%s\n' "$X86_ARCH|packaging/register-default-viewer.cmd|register-default-viewer.cmd"
  printf '%s\n' "$X86_ARCH|packaging/DEFAULT-VIEWER-GUIDE.txt|DEFAULT-VIEWER-GUIDE.txt"
```

- The final function has EXACTLY 8 `printf` lines. Zip entry names sit at the FLAT zip root (no subfolders — the staging + `zip -q -X -j` mechanism already enforces this); the helper works from the extracted destination because it locates the exe relative to ITSELF (`%~dp0img-scroller.exe` — D1). Each zip now contains EXACTLY 4 entries: `img-scroller.exe`, `img-scroller.toml`, `register-default-viewer.cmd`, `DEFAULT-VIEWER-GUIDE.txt`.
- **Two comment-only edits** to `scripts/package-windows.sh` (the ONLY non-manifest script change; justified: comments must not lie — self-documenting-code rule; NO logic, name, path, or behavior change):
  1. Header lines 5–10 — replace:

```
# Consumes the release exes produced by scripts/build-windows.sh plus the tracked
# config file, and writes ONE zip per architecture into the gitignored dist/
# tree. Zip layout is FLAT: the exe and the config file sit at the zip root, no
# subfolders (the user extracts the zip into a destination folder of their
# choice — that folder is the install folder). Zip artifacts are gitignored and
# must NEVER be committed; this script never touches git.
```

     with:

```
# Consumes the release exes produced by scripts/build-windows.sh plus the tracked
# packaged files (the config file, the default-viewer registration helper and
# the viewer guidance text), and writes ONE zip per architecture into the
# gitignored dist/ tree. Zip layout is FLAT: the exe, the config file, the
# helper script and the guidance file sit at the zip root, no subfolders (the
# user extracts the zip into a destination folder of their choice — that
# folder is the install folder). Zip artifacts are gitignored and must NEVER
# be committed; this script never touches git.
```

  2. Manifest-function comment lines 25–28 — replace the 4-line block:

```
# The packaging manifest (the single source of packaged content): one line per
# architecture and packaged file, format "<arch>|<source path>|<zip entry name>".
# A later task (default-viewer helper/guidance) appends its entries here; no
# other part of the script changes for that.
```

     with the 2-line block:

```
# The packaging manifest (the single source of packaged content): one line per
# architecture and packaged file, format "<arch>|<source path>|<zip entry name>".
```

- Nothing else in the script changes (no `unzip`/`zip` flag, no version extraction, no staging logic).

### D5 — Uninstall/rollback: --unregister mode IN the same script (DECIDED — implemented in v1)

- **Decision: an `--unregister` mode in the SAME `.cmd`** (not a second script, not a separate uninstaller). It deletes EXACTLY the entries register created (D1b item list) — value-only deletes under the extension keys, key deletes for the ProgID tree and the `HKCU\Software\Images Scroller` tree, and the value-only delete under `RegisteredApplications`.
- Justification: "no rollback" would leave ghost entries in the user's Open With lists and Settings after they delete the install folder — a one-mode deletion loop of 20 lines is cheap and is the minimal-but-sane rollback. Registry cleanup "via the user's control panel otherwise" is NOT a real path for a portable app with no uninstaller entry.
- Boundaries (pinned): it does NOT attempt to reset any file-type DEFAULT — if the user had set Images Scroller as default for a type (their `UserChoice`), Windows keeps that hash-protected choice; the script's message tells the user to pick a different default in Settings for such types. It never deletes anything it did not create.

### D6 — Guidance content: `packaging/DEFAULT-VIEWER-GUIDE.txt` (FULL literal text)

- The exact 69-line text is in §3 Step 2. It covers: what the helper does (HKCU-only, no admin, Open With lists + Settings visibility, opens the Settings page); what the user does in Settings (Windows 11 and Windows 10 variants); the 17 file types; the WIC codec dependency note (same constraint as the app itself); idempotency + re-run-after-moving-the-folder; the `--unregister` command line and its boundaries; the Windows-owns-the-final-choice fact. README wording remains Task 5 (the guide is the PACKAGED self-contained text; the README gets the distribution/installation/registration prose later).

### D7 — Unit tests: NONE; census stays pinned; NO dev-checks run in Task 3

- **Decision: NO pure Rust logic lands** (the helper is a script; the app's behavior is unchanged; nothing in `src/` or the Cargo manifests changes). NO unit tests are added. The census stays **195 unit tests / 28 test-bearing files** (breakdown: 19 `*_tests.rs` + 4 `tests.rs` + 5 in-file modules). NO dev-checks (`scripts/dev-checks.sh`) run is required for Task 3 — the formal dev-checks run belongs to Task 4 at wave HEAD (pin: census unchanged at 195/28 unless Tasks 4–6 add pure logic, which none do).
- The helper's correctness evidence in THIS task is static (container-side file checks) + the packaging re-run listing the new zip entries; the REAL registration behavior (the script actually running under cmd.exe on Windows 10/11) is Task 4's OPERATOR-RUN gate — agent-side runtime execution of a Windows batch script is impossible from the Linux container and is explicitly out of this task's scope.

### D8 — Docs touchpoints decision

- Task 3 touches **NO prose documentation**: `README.md`, `.agent/project-info/{brief,product,tech,architecture,context}.md`, `docs/*` are all Task-5 scope (the global plan's Task-5 pre-analysis already assigns them the helper-mechanism record; the guidance text INSIDE the package is this task's deliverable — no duplicate maintenance: Task 5 references it, not rewrites it). The ONE exception, per the structure rule: **`.agent/project-structure.md` IS updated by Task 3** (the `packaging/` bullet gains the two new files; the stale `.gitattributes` bullet gains the full four-rule list — Task 1's addition left it stale). The 4.4 docs-specialist expected outcome: "nothing to document; deferred to Task 5" as a clear message. Creating any documentation file in Task 3 is a plan deviation.
- The 4.1b context-log (`context.md`) closing touch of THIS architector step is recorded by the architector (process bookkeeping, not prose docs); it stays UNCOMMITTED (same ride-along treatment as the Task-1/Task-2 4.1b entries — the implementer never stages `context.md`).

## 1. Environment Facts (verified 2026-11-14, at planning time)

- Git: on branch `feat/wave-7-distribution` (Tasks 1–2 landed: commits `7384a24`…`f38dece`; version 1.0.0 at `e7089d0`). Working tree: ONE unstaged modification — `.agent/project-info/context.md` (the Task-1/Task-2 4.1b architector entries — ride-along, NOT part of Task 3's commits).
- `scripts/package-windows.sh` EXISTS with the pinned Task-2 shape (103 lines), including the post-plan CR-tolerant `extract_package_version` (the version-extraction fix commit `2d0bcab`/`3405b80`); `package_manifest_entries` currently has EXACTLY the 4 `printf` lines; the two comment blocks to refresh are at lines 5–10 and 25–28.
- `packaging/` exists with `img-scroller.toml` only (Task 1); `.gitattributes` = exactly 2 lines (`*.sh text eol=lf`, `packaging/*.toml text eol=lf`); `.gitignore` covers `dist/`, `logs/`, `target/`, `*.log` (no change needed — the helper and guide are tracked, the zips stay gitignored under `dist/`).
- Release artifacts from Task 2 exist at the gitignored `dist/windows-x64/img-scroller.exe` and `dist/windows-i686/img-scroller.exe` (1.0.0 smoke builds) — the packaging re-run in Step 7 consumes them; if either is missing, the packaging script's typed failure (`packaged file missing or empty: ...`) tells the implementer to run the no-argument build script first, then re-run packaging.
- Zip names at packaging time: `dist/ImagesScroller-1.0.0-windows-x64.zip` and `dist/ImagesScroller-1.0.0-windows-x86.zip` (version read dynamically from `Cargo.toml` — 1.0.0 at execution).
- Alpine VM (expected RUNNING — verified at wave planning): all container-side evidence runs through the `alpine-vm` MCP (`vm_status` FIRST, then commands; the command exit code is the authoritative evidence). If the VM or the `/img-scroller` share is unavailable: STOP and return a user question (wave execution note).
- Mechanism research (2026-11-14, verified against Microsoft's Default Programs documentation and the SumatraPDF per-user registry installer pattern): the Capabilities + RegisteredApplications mechanism (ApplicationName matching the RegisteredApplications value name; FileAssociations values named by dotted extension pointing at the ProgID; ApplicationIcon `"path",0`) is the documented way an app appears in Settings > Apps > Default apps on Windows 10/11; OpenWithProgids values are "empty string or null value" type; the per-user `UserChoice` default is hash-protected and MUST NOT be written programmatically; per-user (HKCU) registration requires no elevation.

## 2. Binding Constraints (apply to every step)

- NO Rust/Cargo command at all in Task 3 (D7). NO dev-checks run. NO `Cargo.toml`/`Cargo.lock` edit. NO version edit (restricted to Step 3 — already done at 1.0.0).
- NO `git push` (restricted to Step 5). NO branch creation/switch (restricted to Step 2).
- Gitignore Compliance Rule before EVERY commit: read `.gitignore`, run `git status`, ensure NO `dist/`, `logs/`, `target/`, or `*.log` path is staged. NEVER stage `.agent/project-info/context.md` (the uncommitted 4.1b ride-along).
- Do NOT edit any existing `.kilo/plans/*` file. This plan file is the only NEW plan file of this task.
- Do NOT edit `README.md`, `docs/*`, `.agent/project-info/{brief,product,tech,architecture}.md`, `CHANGELOG.md`, `Dockerfile`, `docker-compose.yml`, `scripts/build-windows.sh`, `scripts/dev-checks.sh`, or anything under `src/` — Task 5 owns the wave's prose; no build change belongs to Task 3.
- The helper runs ONLY on Windows (operator territory): agent-side verification is static review + container-side file/zip checks ONLY; real registration = Task 4's OPERATOR-RUN. NEVER attempt to execute the `.cmd` anywhere.
- File writes use real newline characters (never literal `\n` escape sequences) — newline-prevention rule. The `.cmd` file MUST be created with CRLF line endings (D3) and no BOM; the `.txt` with LF and no BOM; `scripts/package-windows.sh` edits keep LF.
- Max-lines rules: `src/` rules not applicable (no `src/` change); the `.cmd` is 64 lines, the `.txt` 69 lines — both compact.

## 3. Step-by-Step Implementation (workflow step 4.2 — implementer, JUNIOR under 50%)

Execute the steps in order. Check the plan between steps. Stop and ask the caller on any ambiguity — never guess. Every `alpine-vm_vm_run_command` call below is ONE command; the exit code is authoritative.

### Step 0 — Commit the plan file (caller/implementer, BEFORE implementation)

- Verify: `git status` → on branch `feat/wave-7-distribution`; the tree contains (at least) the new file `.kilo/plans/20261114-wave7-task3-viewer-registration-plan.md` plus the pre-existing unstaged `context.md` modification (NOT this commit).
- Stage ONLY the plan file: `git add .kilo/plans/20261114-wave7-task3-viewer-registration-plan.md`
- Commit (verbatim message): `docs: add wave7 task3 viewer registration plan`

### Step 1 — Create the helper script

- Create `C:\repo\img-scroller\packaging\register-default-viewer.cmd` (new file in the existing `packaging/` directory).
- Content: EXACTLY the following 64 lines, in this order, with CRLF line endings, a trailing newline after line 64, ASCII-only, NO BOM (nothing more, nothing less — note `%%1`/`%%E` are the batch-file percent escapes):

```bat
@echo off
setlocal
rem Images Scroller - default image viewer registration helper (Windows 10/11).
rem
rem Run this script from INSIDE the extracted Images Scroller folder, next to
rem img-scroller.exe. It registers Images Scroller for YOUR Windows user only
rem (HKCU registry keys; no administrator rights) and then opens the Windows
rem Settings "Default apps" page, where you pick Images Scroller for the file
rem types you want. Windows never lets a script set a default silently - the
rem final per-type choice is always yours in Settings.
rem
rem Usage:
rem   register-default-viewer.cmd                 register (or re-register)
rem   register-default-viewer.cmd --unregister    remove the registration
rem Running it again is safe: it writes the same registration values again.

set REGISTRY_PROGID=ImagesScroller.Image
set REGISTRY_APP_KEY=HKCU\Software\Images Scroller
set REGISTRY_APP_NAME=Images Scroller

if "%~1"=="" goto register
if "%~1"=="--unregister" goto unregister
echo Unknown argument: %~1
echo Usage: register-default-viewer.cmd [--unregister]
exit /b 1

:register
set APP_EXE=%~dp0img-scroller.exe
if not exist "%APP_EXE%" goto missing_exe
rem The application's own ProgID: how Windows describes and launches the app.
reg add "HKCU\Software\Classes\%REGISTRY_PROGID%" /ve /d "Images Scroller Image" /f >nul
reg add "HKCU\Software\Classes\%REGISTRY_PROGID%" /v "FriendlyTypeName" /t REG_SZ /d "Images Scroller Image" /f >nul
reg add "HKCU\Software\Classes\%REGISTRY_PROGID%\DefaultIcon" /ve /d "\"%APP_EXE%\",0" /f >nul
reg add "HKCU\Software\Classes\%REGISTRY_PROGID%\shell\open\command" /ve /d "\"%APP_EXE%\" \"%%1\"" /f >nul
rem Per-extension "Open with" entries (right-click an image, "Open with") and
rem Capabilities file associations (the Settings "Default apps" page).
for %%E in (avif bmp dib gif heic heif ico jfif jpe jpeg jpg jxr png tif tiff webp wdp) do (
  reg add "HKCU\Software\Classes\.%%E\OpenWithProgids" /v "%REGISTRY_PROGID%" /t REG_SZ /d "" /f >nul
  reg add "%REGISTRY_APP_KEY%\Capabilities\FileAssociations" /v ".%%E" /t REG_SZ /d "%REGISTRY_PROGID%" /f >nul
)
reg add "%REGISTRY_APP_KEY%\Capabilities" /v "ApplicationName" /t REG_SZ /d "%REGISTRY_APP_NAME%" /f >nul
reg add "%REGISTRY_APP_KEY%\Capabilities" /v "ApplicationIcon" /t REG_SZ /d "\"%APP_EXE%\",0" /f >nul
reg add "%REGISTRY_APP_KEY%\Capabilities" /v "ApplicationDescription" /t REG_SZ /d "Classic gallery-style image viewer: open one image and scroll through the sibling images and sibling folders with the arrow keys." /f >nul
reg add "HKCU\Software\RegisteredApplications" /v "%REGISTRY_APP_NAME%" /t REG_SZ /d "Software\Images Scroller\Capabilities" /f >nul
echo Images Scroller is registered for this Windows user.
echo Windows opened the "Default apps" Settings page: pick Images Scroller
echo there for the file types you want it to open.
start "" "ms-settings:defaultapps"
exit /b 0

:unregister
reg delete "HKCU\Software\Classes\%REGISTRY_PROGID%" /f >nul 2>&1
for %%E in (avif bmp dib gif heic heif ico jfif jpe jpeg jpg jxr png tif tiff webp wdp) do reg delete "HKCU\Software\Classes\.%%E\OpenWithProgids" /v "%REGISTRY_PROGID%" /f >nul 2>&1
reg delete "%REGISTRY_APP_KEY%" /f >nul 2>&1
reg delete "HKCU\Software\RegisteredApplications" /v "%REGISTRY_APP_NAME%" /f >nul 2>&1
echo Images Scroller registration removed for this Windows user.
echo If it was set as the default for a file type, pick a different
echo default for that type in Windows Settings (Apps, Default apps).
exit /b 0

:missing_exe
echo img-scroller.exe was not found next to this script.
echo Run this script from inside the extracted Images Scroller folder.
exit /b 1
```

- Mapping to D1b: lines 31–34 = the ProgID block; line 37–40 = the 17-extension `for` loop (OpenWithProgids + FileAssociations); lines 41–44 = Capabilities values + RegisteredApplications; line 48 = the Settings page launch; lines 52–55 = the exact unregister deletes; lines 21–25 = the argument gate; lines 61–64 = the missing-exe failure. NO echo line contains a `>` or `|` or `<` character (cmd redirection safety — the ONE place a `>` appeared was reworded to `(Apps, Default apps)`).

### Step 2 — Create the guidance file

- Create `C:\repo\img-scroller\packaging\DEFAULT-VIEWER-GUIDE.txt` (new file).
- Content: EXACTLY the following 69 lines, in this order, with LF line endings, a trailing newline after line 69, ASCII-only, NO BOM:

```text
IMAGES SCROLLER - DEFAULT IMAGE VIEWER REGISTRATION
===================================================

This folder contains the Images Scroller application plus a small helper
script that prepares Windows so you can choose Images Scroller as the
default viewer for your image files.

WHAT THE HELPER DOES
--------------------

Run "register-default-viewer.cmd" (double-click it) from inside this
folder. The helper:

1. Registers Images Scroller in YOUR Windows user profile only (HKCU
   registry keys). It never needs administrator rights and never writes
   outside your own user profile.
2. Adds Images Scroller to the "Open with" list of the image file types
   it supports (right-click an image file, choose "Open with").
3. Makes Images Scroller visible in Windows Settings under
   Apps > Default apps.
4. Opens the Windows Settings "Default apps" page for you.

Windows does not let any program set a default viewer silently - the
final choice is always made by you in Settings. That is by design in
Windows 10 and 11, so the helper registers the application and then
hands you the page where you confirm the choice.

WHAT YOU DO IN SETTINGS
-----------------------

On the "Default apps" page:

- Windows 11: scroll the app list (or use the search box) for
  "Images Scroller", click it, and click the button next to each file
  type you want to open with Images Scroller.
- Windows 10: click "Set defaults by app" (or "Default apps by file
  type"), choose "Images Scroller", and set it as the default for each
  file type you want.

FILE TYPES COVERED
------------------

The helper registers Images Scroller for these 17 image file types:

  .avif  .bmp  .dib  .gif  .heic  .heif  .ico  .jfif  .jpe  .jpeg
  .jpg   .jxr  .png  .tif  .tiff  .webp  .wdp

Whether a given file type can actually be decoded depends on the image
codecs installed in your Windows system (Windows Imaging Component) -
the same constraint the application itself has.

NOTES
-----

- The helper is safe to run again at any time: it writes the same
  registration values again. If you MOVE this folder somewhere else,
  run the helper once more so the registration points at the new
  location.
- To undo the registration, run the helper with the --unregister
  option: open a command prompt in this folder and run

      register-default-viewer.cmd --unregister

  It removes only the registry entries Images Scroller created for this
  user; it never deletes your files, your configuration, or another
  program's registration. If Images Scroller is currently the default
  for a file type, change that file type's default in Settings first -
  Windows keeps the final say on defaults.
- No administrator rights are needed for registering or unregistering.
```

- D6 coverage check (reviewer-facing): helper behavior ✔ (WHAT THE HELPER DOES), Settings steps for both Windows versions ✔, the 17 types ✔ (the two-line grid = 10 + 7), HKCU-only/no-admin ✔, unregister ✔, WIC dependency ✔, Windows-owns-the-choice ✔.

### Step 3 — Pin line endings in `.gitattributes`

- Append TWO lines at the end of `C:\repo\img-scroller\.gitattributes` (after the existing `packaging/*.toml text eol=lf` line, separated by a newline). Final file is EXACTLY 4 lines + trailing newline:

```
*.sh text eol=lf
packaging/*.toml text eol=lf
packaging/*.txt text eol=lf
packaging/*.cmd text eol=crlf
```

- Nothing else in the file changes. NO generic `*.cmd`/`*.txt` rule (D3 rationale).

### Step 4 — Extend `scripts/package-windows.sh` (manifest + two comment refreshes ONLY)

- Edit `C:\repo\img-scroller\scripts\package-windows.sh`:
  1. Append the 4 exact `printf` lines from D4 inside `package_manifest_entries`, after the existing 4 (final function: EXACTLY 8 `printf` lines, x64 pair then x86 pair).
  2. Replace the header comment block (lines 5–10) with the 9-line block from D4 item 1.
  3. Replace the 4-line manifest comment block (lines 25–28) with the 2-line block from D4 item 2.
- NOTHING else in the file changes (no logic, no names, no flags). The file stays LF.

### Step 5 — Update the structure map

- Edit `C:\repo\img-scroller\.agent\project-structure.md` — ONLY two bullets:
  1. In `# Other folders`, replace the `packaging/` bullet with:

```
- packaging/ - distribution-shipped artifacts: img-scroller.toml (the application config file, TOML-style, header comments only in v1 — the v1 binary does not read it), register-default-viewer.cmd (the default-viewer registration helper — a Windows batch script that registers the app HKCU-only: the ProgID, per-extension OpenWithProgids, Capabilities FileAssociations and RegisteredApplications entries for the 17 whitelisted extensions, then opens the Settings Default apps page; --unregister removes exactly those entries) and DEFAULT-VIEWER-GUIDE.txt (the packaged user guidance for the helper); the Wave-7 packaging step copies all three verbatim next to the exe
```

  2. In `# Other files (repository root)`, replace the `.gitattributes` bullet with:

```
- .gitattributes - line-ending policy: `*.sh text eol=lf` (shell scripts, after a CRLF Windows checkout broke the container dev-checks), `packaging/*.toml text eol=lf` (config file), `packaging/*.txt text eol=lf` (guidance text) and `packaging/*.cmd text eol=crlf` (the batch helper — checked out CRLF so the packaged copy reaches the user with CRLF)
```

- Do NOT touch any other line of the file (this also repairs the pre-existing staleness of the `.gitattributes` bullet — Task 1's rule was never recorded there).

### Step 6 — Line-ending + content checks (container-side, alpine-vm MCP)

- Call `vm_status` FIRST. If not running/SSH-unreachable: STOP, return a user question.
- Run each command below as its OWN `alpine-vm_vm_run_command` call; the exit code is authoritative:
  1. `ls -la /img-scroller/packaging` — exit 0; output lists EXACTLY `DEFAULT-VIEWER-GUIDE.txt`, `img-scroller.toml`, `register-default-viewer.cmd` (plus `.`/`..`).
  2. `cat /img-scroller/packaging/register-default-viewer.cmd` — exit 0; output matches the Step-1 literal line-for-line.
  3. `wc -l /img-scroller/packaging/register-default-viewer.cmd` — exit 0; expect `64`.
  4. `sh -c "grep -q \"$(printf '\r')\" /img-scroller/packaging/register-default-viewer.cmd"` — exit 0 = CR present (CRLF file; exit 1 = no CR → fix the file's line endings and re-run).
  5. `sh -c "crs=$(tr -dc '\r' < /img-scroller/packaging/register-default-viewer.cmd | wc -c); lines=$(wc -l < /img-scroller/packaging/register-default-viewer.cmd); [ \"$crs\" -eq \"$lines\" ]"` — exit 0 = one CR per line (expect 64 = 64; exit 1 = a line lacks CR → fix and re-run).
  6. `cat /img-scroller/packaging/DEFAULT-VIEWER-GUIDE.txt` — exit 0; output matches the Step-2 literal line-for-line.
  7. `wc -l /img-scroller/packaging/DEFAULT-VIEWER-GUIDE.txt` — exit 0; expect `69`.
  8. `sh -c "! grep -q \"$(printf '\r')\" /img-scroller/packaging/DEFAULT-VIEWER-GUIDE.txt"` — exit 0 = LF-only (exit 1 = a CR was found → fix and re-run).
  9. `sh -c "! grep -q \"$(printf '\r')\" /img-scroller/scripts/package-windows.sh"` — exit 0 = the edited script stays LF-only.
- NO dev-checks run, NO Rust/Cargo command, NO execution of the `.cmd` anywhere (D7; the helper runs only on Windows — Task 4's OPERATOR-RUN).

### Step 7 — Packaging re-run + zip listing checks (container-side evidence of the manifest integration)

- Prerequisite: the Task-2 artifacts exist (`dist/windows-x64/img-scroller.exe`, `dist/windows-i686/img-scroller.exe`). If Step 7.1 fails with a typed `package-windows: error: packaged file missing or empty: ...` line: run `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` (exit 0 expected), then re-run Step 7.1.
- Run each command below as its OWN `alpine-vm_vm_run_command` call; the exit code is authoritative:
  1. `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh` — expected exit 0; output contains `package-windows: zip: dist/ImagesScroller-1.0.0-windows-x64.zip`, `package-windows: zip: dist/ImagesScroller-1.0.0-windows-x86.zip`, `package-windows: OK`.
  2. `unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip` — exit 0; EXACTLY 4 entries: `img-scroller.exe`, `img-scroller.toml`, `register-default-viewer.cmd`, `DEFAULT-VIEWER-GUIDE.txt` (no directory entries).
  3. `unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip` — exit 0; EXACTLY the same 4 entries.
  4. `sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""` — exit 0 = the PACKAGED helper copy carries CRLF (the eol=crlf working-tree pin did its job).
  5. `sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""` — exit 0; same evidence for the x86 zip.
- A typed `package-windows: error:` line or a nonzero exit (other than the handled prerequisite case) → STOP and ask the caller.

### Step 8 — Gitignore compliance + commit

- `git status` → confirm NO gitignored path is staged or listed as untracked-to-stage (`dist/` zips/staging stay unstaged; `context.md` stays unstaged).
- Stage EXACTLY: `git add packaging/register-default-viewer.cmd packaging/DEFAULT-VIEWER-GUIDE.txt .gitattributes scripts/package-windows.sh .agent/project-structure.md`
- Commit (verbatim message): `feat: add default viewer registration helper and guidance`
- Post-commit `git status`: clean apart from the pre-existing unstaged `context.md` ride-along.

### Explicitly NOT done in Task 3 (for the implementer)

- NO Rust/source changes; NO `Cargo.toml`/`Cargo.lock` edits; NO dev-checks run (Task 4); NO tests added (D7 — census stays 195/28); NO README/`docs/`/project-info prose edits (Task 5; the 4.1b context-log entry is the architector's, already recorded); NO `CHANGELOG.md` edit; NO `.ps1`, no console executable, no repo-root script; NO execution of the helper anywhere (Task 4's OPERATOR-RUN); NO `git push`; NO branch/version actions; NO TODO marking (that is 4.6).

## 4. Workflow step 4.3 — Code Review & Simplification guidance

- code-reviewer checklist (deviation from plan = finding): the `.cmd` matches the Step-1 literal byte-for-byte semantics (64 lines, ASCII, no BOM, CRLF, `%%1`/`%%E` escapes intact, no `>`/`|`/`<` in any echo line, the for-loop extension list matches `src/folder_listing.rs` `IMAGE_FILE_EXTENSIONS` order exactly, all registry paths under `HKCU`, register creates ONLY the D1b surface, unregister deletes ONLY the D5 surface); the `.txt` matches the Step-2 literal (69 lines, ASCII, LF); `.gitattributes` gains exactly the 2 pinned lines; `package-windows.sh` gains exactly the 4 manifest lines + the 2 pinned comment blocks and NOTHING else; the structure-map bullets match Step 5 verbatim in substance; the commit contains ONLY the five files; NO file under `src/`, `Cargo.toml`, `Cargo.lock`, `dist/`, `docs/`, `.agent/project-info/*` (except NOTHING — `context.md` is not Task 3's), `README.md`, `Dockerfile`, or `CHANGELOG.md` changed vs the Step-0 baseline; no gitignored path committed; no `UserChoice` write or delete anywhere in the helper.
- code-simplifier: review the `.cmd` for dead duplication (the 17-extension list appears twice — in the register for-loop and the unregister for-loop; BOTH are KEPT: batch has no shared-constant mechanism without a second file, and duplicating a literal 17-token list inline in each mode is the standard batch idiom). Expected outcome: none or trivial — any change must NOT alter the pinned registry surface, argument contract, message text, or line count pins without a plan amendment.
- Max 3 review cycles, then escalate (workflow rule).

## 5. Workflow step 4.4 — Documentation guidance

- The docs-specialist adds NOTHING in Task 3 (D8): the packaged guide IS this task's user-facing text (self-contained by design), the helper's header comments are its developer-facing text, and ALL wave prose documentation (README distribution/installation/registration section, tech.md helper-mechanism record, architecture.md packaging-pipeline update, context.md wave entry) is Task 5's deliverable per the TODO's Task 5. Expected outcome: "nothing to document; deferred to Task 5" as a clear message. Creating any documentation file in Task 3 is a plan deviation.

## 6. Workflow step 4.5b — Plan-adherence guidance

- Verify sections 0–3 against the landed diff (`git diff <Step-0-commit>..<Task-3 commit>`): the exact file set (`packaging/register-default-viewer.cmd`, `packaging/DEFAULT-VIEWER-GUIDE.txt`, `.gitattributes`, `scripts/package-windows.sh`, `.agent/project-structure.md`) with the exact contents/decisions of D1–D6 + Steps 3–5, and the verification evidence points (Steps 6–7: the 9 file checks exit 0 with the pinned counts 64/69 and CR evidence; the packaging re-run exit 0; the two `unzip -l` listings with exactly 4 flat entries each; the two packaged-CR checks exit 0).
- If every element conforms: return a clear "no diffs, VERIFIED" message (no report file needed). If a deviation exists: record it in a NEW plan file `.kilo/plans/20261114-wave7-task3-adherence.md` and propose the disposition — never edit existing plans.

## 7. Workflow step 4.6 — Task completion (implementer)

- Edit `.agent/todos/20261029/20261029-todo-9.md` (preserve ALL other content byte-identical):
  - `### Task 3: Default-viewer registration helper/guidance` → `### Task 3: Default-viewer registration helper/guidance [DONE]`
  - Its TWO sub-items, each `- [ ]` → `- [x]` keeping the rest of each line verbatim: the "The package includes a helper and/or guidance..." item and the "The helper mechanism ... decided in this wave's architector step..." item.
- Commit message (verbatim): `task: mark wave7 task3 done`
- Gitignore compliance before the commit (`git status`; the TODO file is tracked — safe).

## 8. Risks / stop conditions

- VM or `/img-scroller` share unavailable at any step → STOP, return a user question (wave execution note).
- The packaging re-run fails for a MISSING artifact (the Task-2 exes were cleaned) → run the no-argument build script once (Step 7 prerequisite), then re-run packaging; any OTHER typed `package-windows: error:` → STOP and ask the caller.
- Line-ending drift (the `.cmd` lands LF on disk, or the `.txt` gains CRs) → the Step-6 checks catch it; fix the file's endings and re-run; NEVER "fix" by editing the `.gitattributes` rules.
- The helper's real runtime behavior (registry effect, Settings appearance, the double-click flow, the unregister flow) is NOT verifiable agent-side — it is Task 4's OPERATOR-RUN gate; do not improvise an emulation.
- Any plan ambiguity encountered by a sub-agent → STOP and return the question to the Planner; never guess.
- Review-cycle cap: max 3 (4.3) cycles, then escalate to the user.

## 9. Plan-vs-task compliance check

- TODO Task 3 requires: (1) "The package includes a helper and/or guidance that helps the user register the app as the default image viewer (file associations) in Windows" ✔ (both: the helper script AND the guidance text are packaged into BOTH zips at the flat root — D4; they reach the extracted install folder next to the exe); (2) "The helper mechanism (script vs executable, exact mechanism) is decided in this wave's architector step — this wave file must not pre-select it" ✔ (D1: ONE `.cmd` batch script using built-in reg.exe; D1b pins the EXACT registry keys/values/types/data for the chosen mechanism — the modern RegisteredApplications + Capabilities registration combined with per-extension OpenWithProgids; D2 the 17 extensions; D3 the file set/names/locations/line endings; D5 the rollback; D6 the guidance text). Wave-level decision #3 honored (no GUI surface — console text + the Settings page only); decision #5 honored (manifest lines only + two pinned comment refreshes, no other script change). Nothing from Tasks 4–6 is executed (verification/dev-checks/manual-UX = Task 4; prose docs = Task 5; CHANGELOG = Task 6). The plan is complete and correct against the task.
