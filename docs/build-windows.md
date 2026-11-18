# Windows x64/x86 Release Cross-Build and Zip Packaging (Container)

The release `.exe` for Windows x64 AND x86 is cross-compiled **inside the container**: targets
`x86_64-pc-windows-gnu` (MinGW-w64 x64 toolchain) and `i686-pc-windows-gnu` (MinGW-w64 i686 toolchain,
Wave 7). This page covers both architectures, the portable zip packaging, and the
NSIS wizard-installer build (Wave 8).

## Contents

1. [Prerequisites](#prerequisites)
2. [Invocation](#invocation)
3. [Expected output path](#expected-output-path)
4. [Overwrite / rebuild behavior](#overwrite--rebuild-behavior)
5. [Typed failure table](#typed-failure-table)
6. [Toolchain / target notes](#toolchain--target-notes)
7. [Static-linking / import audit (retained scope)](#static-linking--import-audit-retained-scope)
8. [Portable zip packaging](#portable-zip-packaging)
9. [Windows installer packaging (NSIS)](#windows-installer-packaging-nsis)
10. [Validation stance](#validation-stance)
11. [Lockfile policy](#lockfile-policy)

## Prerequisites

- The container image is built once (includes both MinGW-w64 linkers — x64 and i686 — both windows-gnu targets, `zip`/`unzip` for the packaging, and `nsis` (`makensis`) for the installer build) — see [`build.md`](build.md).
- The Alpine VM shares the project at `/img-scroller`; Docker + Compose exist only on the VM.
- Agents use the `alpine-vm` MCP: `vm_status` first; the command exit code is the authoritative success evidence; absent cargo stderr is not a failure signal.

## Invocation

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh

With NO argument the script builds BOTH targets (x64 first, then x86). ONE optional argument builds
exactly that target:

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu
    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu

More than one argument, or an unknown target, is a typed failure (see the table below).

## Expected output path

    dist/windows-x64/img-scroller.exe   (x64)
    dist/windows-i686/img-scroller.exe  (x86, Wave 7)

The repo root `dist/` tree is gitignored; on the Windows host the paths appear at the repo root under
`dist\windows-x64\` and `dist\windows-i686\`.

## Overwrite / rebuild behavior

- Per target, a successful rebuild atomically overwrites the previous exe (the new file is copied to a transient
  `img-scroller.exe.part` and then renamed over the final name).
- A failed run leaves the previous artifact byte-for-byte intact; the transient part file is removed
  on failure and nothing pre-existing is ever deleted.

## Typed failure table

Every failure line starts with the literal prefix `build-windows: error:` and the script exits `1`.

`<dist dir>` is `dist/windows-x64` for the x64 target and `dist/windows-i686` for the x86 target.

| Failure class | Behavior |
| --- | --- |
| wrong working directory | `build-windows: error: wrong working directory: <cwd>; expected: /img-scroller (run through docker compose)` |
| unset/empty CARGO_TARGET_DIR | `build-windows: error: CARGO_TARGET_DIR is unset or empty (the rust compose service sets it to the named volume)` |
| cargo build failure (incl. lockfile/manifest mismatch) | build diagnostics pass through unmodified, then `build-windows: error: cargo build failed (exit <N>); check Cargo.toml / Cargo.lock consistency via the dev checks` — reconcile manifest and `Cargo.lock` before retrying |
| expected release executable not found or empty | `build-windows: error: expected release executable not found or empty: <path>` |
| `dist/` directory-create failure | `build-windows: error: cannot create dist directory: <dist dir>` |
| copy failure | `build-windows: error: copy failed: <source> -> <dist dir>` |
| install-of-copy failure (rename) | `build-windows: error: install of copied artifact failed: <part> -> <final>` |
| artifact missing/empty after copy | `build-windows: error: artifact missing or empty after copy: <dist dir>/img-scroller.exe` |
| unknown target argument | `build-windows: error: unknown target: <arg> (expected x86_64-pc-windows-gnu, i686-pc-windows-gnu, or no argument for both)` |
| more than one argument | `build-windows: error: expected at most 1 argument, got <N>` |

## Toolchain / target notes

- Both GNU cross targets and their linkers are baked into the pinned image: `x86_64-pc-windows-gnu`
  (linker `x86_64-w64-mingw32-gcc`) and `i686-pc-windows-gnu` (linker `i686-w64-mingw32-gcc`, Wave 7);
  no local toolchain setup exists on any host.
- The build runs `cargo build --release --locked --target <triple>` per target; the GNU runtime
  components are statically linked into each produced exe (no extra runtime DLLs distributed with the
  app — corroborated by the import audit below for BOTH artifacts).

## Static-linking / import audit (retained scope)

The build script does NOT run the import audit itself. Once an exe is produced, the verification
session runs ONE import audit per artifact with the matching objdump tool (`x86_64-w64-mingw32-objdump`
for x64, `i686-w64-mingw32-objdump` for x86) and records the OBSERVED DLL list here and in the
script's per-target constants (`OBSERVED_DLL_IMPORT_LIST` / `OBSERVED_DLL_IMPORT_LIST_I686`). The
lists are observed evidence — never copied from any reference project.

Observed DLL import lists — BOTH artifacts import the SAME 22 system DLLs (set-equal):

    advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll

    x64: audited 2026-10-31 with `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` inside the container (Wave 1 verification; first produced exe). Re-audited 2026-11-15 on the 1.0.0 artifact (Wave 7 Task 4) — set-equal.
    x86: audited 2026-11-15 with `i686-w64-mingw32-objdump -x dist/windows-i686/img-scroller.exe` inside the container (Wave 7 Task 4) — set-equal to the x64 list; NO MinGW runtime DLL (`libgcc_s_dw2.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`) is imported, so the x86 build is portable. Lists are deduplicated and lowercase.

## Portable zip packaging

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh

Produces ONE flat zip per architecture from the built exes plus the tracked `packaging/` files:

    dist/ImagesScroller-<version>-windows-x64.zip
    dist/ImagesScroller-<version>-windows-x86.zip

- The version is read dynamically from `Cargo.toml` (at execution: 1.1.0).
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

## Windows installer packaging (NSIS)

Wave 8 adds an executable **wizard installer** as the primary distribution artifact; the zip
packaging above is RETAINED unchanged and stays the only x86 distribution. `makensis` (NSIS 3.08)
is baked into the pinned image; the installer cross-builds in the same container:

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh

PAYLOAD PREREQUISITE: the installer embeds the x64 release exe, so produce it FIRST (build order):

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu
    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh

A missing x64 exe is a typed failure (table below). ONE installer embedding the **x64 exe only** —
there is NO x86 installer; 32-bit Windows users keep the retained x86 zip (portable). The installed
payload is `img-scroller.exe`, `img-scroller.toml`, and `LICENSE`; the
`register-default-viewer.cmd` helper and `DEFAULT-VIEWER-GUIDE.txt` are NOT installed (the wizard's
association page replaces them) and stay in the zips.

Artifact (version read dynamically from `Cargo.toml`, same extraction as the zip packaging):

    dist/ImagesScroller-<version>-windows-installer.exe

Success prints `build-installer: artifact: <path>` then `build-installer: OK` and exits `0`. The
script compiles into a transient `.part` file and renames it over the final name; a failed run
leaves any previous installer byte-for-byte intact and removes the transient file. Output lives
under the gitignored `dist/` tree and is NEVER committed; the script never touches git.

The tracked source is `packaging/nsis/installer.nsi` (MUI2 + nsDialogs + LogicLib). The build
script injects every dynamic value as an absolute container-path `makensis` define
(`-DPRODUCT_VERSION=…`, `-DPRODUCT_VERSION_4=…`, `-DINSTALLER_OUTFILE=…`, `-DEXE_PAYLOAD=…`,
`-DCONFIG_PAYLOAD=…`, `-DLICENSE_PAYLOAD=…`); the `.nsi` owns only the static structure.
`makensis` warnings are non-fatal — exit `0` plus a non-empty artifact is the success evidence.
Full semantics live in `scripts/build-installer.sh` and `packaging/nsis/installer.nsi` themselves;
this guide does not duplicate them.

Installer behavior:

- Pages: Welcome → destination folder → **file-association page** → install → finish.
- Association page: one checkbox per type over EXACTLY the 17 supported extensions (`.avif`,
  `.bmp`, `.dib`, `.gif`, `.heic`, `.heif`, `.ico`, `.jfif`, `.jpe`, `.jpeg`, `.jpg`, `.jxr`,
  `.png`, `.tif`, `.tiff`, `.webp`, `.wdp`) in a fixed grid, **NONE preselected** — the user
  explicitly picks, e.g. only `.png`.
- Scope: per-user **HKCU only**, no elevation; default destination
  `%LOCALAPPDATA%\Programs\Images Scroller`, remembered across reinstalls. A per-user Start Menu
  shortcut is created; no desktop shortcut and no run-after-install checkbox.
- Registry surface: the Wave-7 helper surface verbatim, written ONLY for the selected types —
  ProgID `ImagesScroller.Image`, per-type `OpenWithProgids` values, the `Images Scroller`
  Capabilities tree, and the `RegisteredApplications` registration. **`UserChoice` is NEVER
  written** — the final per-type default stays user-mediated in Windows Settings.
- Reinstall convergence: every per-type entry the installer could have created is deleted first,
  then only the newly selected types are written, so a reinstall with a smaller selection
  converges to exactly that selection; the Explorer Open-with cache is refreshed via
  `SHChangeNotify`.
- Finish hand-off: `ms-settings:defaultapps` opens automatically ONLY when at least one type was
  selected; with no selection no Settings page is shown.
- Uninstall: removes the installed files, the Start Menu shortcut, and EXACTLY what the install
  created — value-only deletes under the extension keys (the `.ext` keys themselves are NEVER
  deleted), the ProgID / Capabilities / Add-Remove-Programs keys, and the
  `RegisteredApplications` value.

Every installer-build failure line starts with the literal prefix `build-installer: error:` and the
script exits `1`.

| Failure class | Behavior |
| --- | --- |
| wrong working directory | `build-installer: error: wrong working directory: <cwd>; expected: /img-scroller (run through docker compose)` |
| `makensis` missing from the image | `build-installer: error: makensis not found (rebuild the container image after the Dockerfile change)` |
| installer source file missing or empty | `build-installer: error: expected source not found or empty: <path>` (`packaging/nsis/installer.nsi`, `packaging/img-scroller.toml`, or `LICENSE`) |
| x64 payload missing or empty | `build-installer: error: expected payload not found or empty: dist/windows-x64/img-scroller.exe (produce the x64 exe with scripts/build-windows.sh first)` |
| version extraction failure | `build-installer: error: cannot extract the package version from Cargo.toml` |
| makensis compile failure | makensis diagnostics pass through, then `build-installer: error: makensis compile failed (exit <N>); see the makensis diagnostics above` |
| artifact missing/empty after compile | `build-installer: error: installer artifact missing or empty after compile: <part>` |
| install-of-artifact failure (rename) | `build-installer: error: install of installer artifact failed: <part> -> <final>` |

## Validation stance

Building is container-only. Running the produced `.exe`, GUI behavior, and image-format handling are
validated by the operator on a real Windows 10/11 machine — agent-side execution from the Linux
container is impossible.

## Lockfile policy

`Cargo.lock` is tracked; the build uses `--locked`; changed dependencies must be committed together
with the regenerated `Cargo.lock`.
