# Wave 7 — Task 2 Implementation Plan: Zip-Style Portable Installer (x64 + x86 build extension + packaging)

- Source TODO: `.agent/todos/20261029/20261029-todo-9.md` → `### Task 2: Zip-style portable installer`. Verbatim sub-items: (1) "The user picks a destination folder; the package unzips the application binary AND the config file into that destination."; (2) "Release builds for x64 AND x86: this wave consumes AND extends Wave 1's `scripts/build-windows.sh` — the x86 build is an extension of Wave 1's harness: the `i686-pc-windows-gnu` target plus the `gcc-mingw-w64-i686` toolchain addition to the tracked Dockerfile are added when the x86 target is realized; the exact extension mechanism/details are decided in this wave's architector step."; (3) "The packaging mechanism (script vs program, archive tooling) is decided in this wave's architector step; the packaging consumes the produced x64/x86 artifacts (the produced `.exe` files) from the gitignored `dist/` locations."
- Global plan (approved): `.kilo/plans/20261114-wave7-distribution-mvp.md` → `## Task 2 — Zip-style portable installer` + Wave-level decisions (#2 version 1.0.0, #3 no-GUI/no-frontend, #5 packaging-manifest single source, #7 house rules).
- Task 1 plan (landed): `.kilo/plans/20261114-wave7-task1-config-plan.md` — the config file `packaging/img-scroller.toml` exists (11 header-comment lines, LF, tracked; D3: copied verbatim flat next to the exe, same name).
- Date: 2026-11-14. Status: FOR PLANNER PRESENTATION (pending user approval before 4.2 runs).
- Branch: `feat/wave-7-distribution` (created at workflow Step 2 — NOT this task's job). Version at execution: 1.0.0 (bumped at Step 3 — NOT this task's job; this task NEVER edits version fields).
- Front-end related: NO (Wave-level decision #3 — no 4.1a, no 4.5a; this is the 4.1b architector settlement).

## 0. Deferred-Decision Settlement (BINDING — settles every Task-2 deferred decision of the wave file)

Every decision below is final for Task 2's implementation. The implementer must not re-choose.

### D1 — Dockerfile extension (verified against the pinned base image)

- Verified 2026-11-14 inside a throwaway `rust:1.99.1-bookworm` container on the Alpine VM (exit 0): the Debian bookworm apt package **`gcc-mingw-w64-i686`** exists (candidate `12.2.0-14+25.2`) and installs the i686 cross binaries `i686-w64-mingw32-gcc`, `i686-w64-mingw32-ld`, and `i686-w64-mingw32-objdump` (the last backs the x86 artifact's import audit — the i686 binutils dependency ships it). The archive tooling packages `zip` (candidate `3.0-13+deb12u1`) and `unzip` (candidate `6.0-28+deb12u1`) also exist.
- **Decision: the packaging runs CONTAINER-SIDE** (the wave requires container runs for builds; host-side packaging is NOT acceptable — the host has no toolchain and no Docker), so the archive tooling is installed in the image: `zip` to build the zips, `unzip` to verify their listings (`unzip -l`) without extracting.
- **Exact new Dockerfile content (the implementer writes EXACTLY this whole file — 21 lines, replacing the current 15):**

```dockerfile
# Images Scroller — container build harness (dev checks + Windows x64/x86 cross-builds).
# Pinned official Rust base image; a movable tag (latest/1) is deliberately not used.
FROM rust:1.99.1-bookworm

# Lint/format components on the default stable toolchain shipped in the base image.
RUN rustup component add rustfmt clippy

# MinGW-w64 x86-64 GNU cross toolchain:
# x86_64-w64-mingw32-gcc is the linker backend for x86_64-pc-windows-gnu;
# x86_64-w64-mingw32-objdump (via the package's binutils dependency) backs the
# Windows import audit performed during Wave 1 verification (Task 5).
# MinGW-w64 i686 GNU cross toolchain (Wave 7):
# i686-w64-mingw32-gcc is the linker backend for i686-pc-windows-gnu;
# i686-w64-mingw32-objdump backs the x86 artifact's import audit.
# zip + unzip back the Wave-7 portable packaging script and its zip listing checks.
RUN apt-get update && apt-get install -y --no-install-recommends gcc-mingw-w64-x86-64 gcc-mingw-w64-i686 zip unzip && rm -rf /var/lib/apt/lists/*

# Windows x64 cross-compilation target on the same default toolchain.
RUN rustup target add x86_64-pc-windows-gnu

# Windows x86 cross-compilation target on the same default toolchain (Wave 7).
RUN rustup target add i686-pc-windows-gnu
```

- Placement rules: the i686 package is appended to the EXISTING single `apt-get install` line (one apt transaction; the existing x86-64 package name is unchanged and stays first); `zip unzip` appended after it on the same line; the new `rustup target add i686-pc-windows-gnu` RUN line goes AFTER the existing `x86_64-pc-windows-gnu` line. Nothing else in the file changes; the base-image pin stays `rust:1.99.1-bookworm`.
- **Image rebuild is REQUIRED after this change. Exact rebuild command (per `docs/build.md`):**
  `docker compose -f /img-scroller/docker-compose.yml build rust` — expected exit 0. (Compose layer cache reuses unchanged layers; the apt layer and the two rustup layers re-run.)

### D2 — `scripts/build-windows.sh` extension mechanism (full new content pinned)

- **Mechanism decision: the script is extended IN PLACE to a target-parameterized loop.** Invocation contract (pinned): `sh scripts/build-windows.sh` with NO argument builds BOTH targets in one run (x64 first, then x86); ONE optional argument builds exactly that target (`x86_64-pc-windows-gnu` | `i686-pc-windows-gnu`); more than one argument or an unknown target is a typed failure. This gives Task 4 its per-target "each command exits 0" evidence via two single-target invocations, while keeping a one-command full rebuild.
- Preserved disciplines (unchanged semantics): the typed-failure prefix `build-windows: error:` with exit 1; the atomic `.part` → rename install; a failed run never deletes or modifies a previous artifact; `--locked` release builds; the OBSERVED-DLL audit note printed by the script (the script still does NOT run the audit itself — same as Wave 1); exit 0 on full success, exit 1 on any failure.
- **x64 artifact path UNCHANGED: `dist/windows-x64/img-scroller.exe`** (never relocated).
- **x86 artifact path PINNED: `dist/windows-i686/img-scroller.exe`** — mirrors the target triple's `i686` (as `windows-x64` mirrors `x86_64`); internal artifact naming follows the triple, while user-facing zip names (D3) use the conventional `x64`/`x86`.
- **Per-target DLL-audit handling decision:** the existing constant keeps the name `OBSERVED_DLL_IMPORT_LIST` (it IS the observed x64 22-DLL list — no rename, no churn) and the x64 success note keeps printing it; the x86 artifact gets a typed note pointing at the audit tool `i686-w64-mingw32-objdump` (verified present in the image) and stating the OBSERVED x86 list is recorded by the Wave-7 verification task (Task 4 runs the audit and records it — the same fill-at-verification precedent as Wave 1). The script NEVER prints an unobserved list.
- **POSIX-sh subshell discipline (encoded because the script must keep the exit-0-or-1 contract):** `report_failure` must never be relied on from inside a `$(...)` command substitution UNLESS the assignment carries `|| exit 1` (a subshell exit does not end the parent shell). Every command substitution in the new script that can fail carries `|| exit 1` exactly as written below.
- **Exact new `scripts/build-windows.sh` content (the implementer writes EXACTLY this whole file):**

```sh
#!/bin/sh
# Windows release cross-builds for Images Scroller (x64 and x86).
# Runs inside the container:
#   docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh
# Typed failures: every failure line starts with the literal prefix "build-windows: error:"
# and the script exits 1. A failed run never deletes or modifies a previous artifact.
# Invocation: no argument builds BOTH targets in one run (x64 first, then x86);
# one optional argument builds exactly that target:
#   x86_64-pc-windows-gnu | i686-pc-windows-gnu

set -u

readonly PROJECT_DIR="/img-scroller"
readonly RELEASE_SUBDIR="release"
readonly BINARY_NAME="img-scroller.exe"
readonly PART_FILE_SUFFIX=".part"
readonly X64_TARGET="x86_64-pc-windows-gnu"
readonly X86_TARGET="i686-pc-windows-gnu"
readonly X64_DIST_DIR="dist/windows-x64"
readonly X86_DIST_DIR="dist/windows-i686"
readonly OBSERVED_DLL_IMPORT_LIST="advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll"

report_failure() {
  printf 'build-windows: error: %s\n' "$1" >&2
  exit 1
}

assert_current_directory() {
  if [ "$(pwd)" != "$PROJECT_DIR" ]; then
    report_failure "wrong working directory: $(pwd); expected: $PROJECT_DIR (run through docker compose)"
  fi
}

assert_cargo_target_dir() {
  if [ -z "${CARGO_TARGET_DIR:-}" ]; then
    report_failure "CARGO_TARGET_DIR is unset or empty (the rust compose service sets it to the named volume)"
  fi
}

resolve_target_list() {
  if [ "$#" -eq 0 ]; then
    printf '%s\n' "$X64_TARGET" "$X86_TARGET"
  elif [ "$#" -eq 1 ]; then
    case "$1" in
      "$X64_TARGET"|"$X86_TARGET") printf '%s\n' "$1" ;;
      *) report_failure "unknown target: $1 (expected $X64_TARGET, $X86_TARGET, or no argument for both)" ;;
    esac
  else
    report_failure "expected at most 1 argument, got $#"
  fi
}

dist_dir_for_target() {
  case "$1" in
    "$X64_TARGET") printf '%s' "$X64_DIST_DIR" ;;
    "$X86_TARGET") printf '%s' "$X86_DIST_DIR" ;;
    *) report_failure "no dist directory known for target: $1" ;;
  esac
}

assert_source_artifact_present() {
  if [ ! -s "$1" ]; then
    report_failure "expected release executable not found or empty: $1"
  fi
}

install_artifact_into_dist() {
  source_artifact_path="$1"
  dist_dir="$2"
  final_artifact_path="$dist_dir/$BINARY_NAME"
  part_artifact_path="$dist_dir/$BINARY_NAME$PART_FILE_SUFFIX"
  mkdir -p "$dist_dir" || report_failure "cannot create dist directory: $dist_dir"
  cp "$source_artifact_path" "$part_artifact_path" || {
    rm -f "$part_artifact_path"
    report_failure "copy failed: $source_artifact_path -> $dist_dir"
  }
  mv "$part_artifact_path" "$final_artifact_path" || {
    rm -f "$part_artifact_path"
    report_failure "install of copied artifact failed: $part_artifact_path -> $final_artifact_path"
  }
  [ -s "$final_artifact_path" ] || report_failure "artifact missing or empty after copy: $final_artifact_path"
  printf '%s\n' "$final_artifact_path"
}

print_audit_note_for_target() {
  windows_target="$1"
  case "$windows_target" in
    "$X64_TARGET")
      printf 'build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (%s): %s\n' "$X64_DIST_DIR" "$OBSERVED_DLL_IMPORT_LIST"
      ;;
    "$X86_TARGET")
      printf 'build-windows: %s DLL import audit: the OBSERVED list is recorded by the Wave-7 verification task (audit tool: i686-w64-mingw32-objdump)\n' "$X86_DIST_DIR"
      ;;
  esac
}

build_target() {
  windows_target="$1"
  dist_dir=$(dist_dir_for_target "$windows_target") || exit 1
  source_artifact_path="$CARGO_TARGET_DIR/$windows_target/$RELEASE_SUBDIR/$BINARY_NAME"
  cargo build --release --locked --target "$windows_target" || report_failure "cargo build failed for $windows_target (exit $?); check Cargo.toml / Cargo.lock consistency via the dev checks"
  assert_source_artifact_present "$source_artifact_path"
  final_artifact_path=$(install_artifact_into_dist "$source_artifact_path" "$dist_dir") || exit 1
  printf 'build-windows: artifact: %s\n' "$final_artifact_path"
  print_audit_note_for_target "$windows_target"
}

main() {
  assert_current_directory
  assert_cargo_target_dir
  target_list=$(resolve_target_list "$@") || exit 1
  for windows_target in $target_list; do
    printf 'build-windows: building target: %s\n' "$windows_target"
    build_target "$windows_target" || exit 1
  done
  printf 'build-windows: OK\n'
  exit 0
}

main "$@"
```

- Note on the ONE wording delta vs the old x64 success line: the audit note now names the artifact directory in parentheses (`OBSERVED DLL import list (dist/windows-x64): ...`) — docs reconciliation belongs to Task 5.

### D3 — Packaging mechanism: tracked script, zip, container-side (full new content pinned)

- **Decision: a NEW tracked POSIX-sh script `scripts/package-windows.sh`** (consistent with the harness script family; `.gitattributes` already pins `*.sh text eol=lf`), run through the compose harness, using the **`zip`** CLI installed by D1. NOT a Rust program (a Rust packager would need a new build target and add packaging code to the app's binary surface — a shell script consumes the produced artifacts with zero dependency change; the wave's leaning confirmed).
- **One zip per architecture — justification (one sentence):** a 32-bit Windows user cannot run the 64-bit exe and vice versa, so one zip per architecture gives each user a minimal, directly-usable download.
- **Zip contents — FLAT layout, PINNED:** exactly `img-scroller.exe` + `img-scroller.toml` at the zip ROOT — NO subfolders, NO directory entries. The config file is `packaging/img-scroller.toml` (landed in Task 1), copied VERBATIM (same name, LF, byte-identical — Task 1 D3). The zip is built by staging each manifest entry under its exact zip entry name and `zip -q -X -j` (junk paths) per entry — the flat layout is enforced by construction, not by convention.
- **Zip file names — PINNED, version read DYNAMICALLY from `Cargo.toml`:** `ImagesScroller-<version>-windows-x64.zip` and `ImagesScroller-<version>-windows-x86.zip`. The version is the `[package] version` extracted at packaging time (single source of truth — no double maintenance with the Step-3 bump; at execution this yields `ImagesScroller-1.0.0-windows-x64.zip` / `ImagesScroller-1.0.0-windows-x86.zip`). The exact extraction (inside the script): `sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' Cargo.toml | head -n 1` — verified safe: `Cargo.toml` line 3 is the ONLY column-0 `version = "..."` line (dependency versions are inline after crate names); an empty extraction is a typed failure. Zip names use the user-facing `x64`/`x86` tokens; the internal artifact dir uses the triple-ish `windows-i686` (D2) — both PINNED as stated.
- **Output location — PINNED: directly under the gitignored `dist/` tree** (`dist/ImagesScroller-1.0.0-windows-x64.zip`, `dist/ImagesScroller-1.0.0-windows-x86.zip`) — the zips are release deliverables, not per-arch build artifacts. Zip artifacts are gitignored (`dist/` in `.gitignore`) and must NEVER be committed; the script NEVER invokes git (it cannot stage or commit anything); the implementer's Gitignore Compliance check before every commit enforces the rest.
- **Atomic-overwrite + cleanup discipline (mirrors the build script):** every zip is built at a transient `dist/package-staging/<name>.zip.part` and renamed over the final name ONLY after a non-empty check; a failed run never deletes or modifies a previously installed zip; ALL transient files (staging copies, manifest dump, part zips) live under `dist/package-staging/` and an EXIT trap removes that directory on every exit. Per-arch failure semantics (pinned, mirrors the build script's per-target semantics): if the x86 zip fails after the x64 zip installed, the new x64 zip stays and the previous x86 zip stays untouched.
- **Packaging-manifest SINGLE SOURCE (Wave-level decision #5 — the Task-3 extension point):** the manifest is ONE function, `package_manifest_entries`, emitting one line per (architecture, file) pair in the format `<arch>|<source path>|<zip entry name>`. Task 3 (helper/guidance) appends its entries as additional lines — NO other script change. The x86 build's zip consumes the `x86` lines; the x64 zip the `x64` lines; a duplicate zip entry name is a typed failure.
- **Exact new `scripts/package-windows.sh` content (the implementer writes EXACTLY this whole file):**

```sh
#!/bin/sh
# Portable zip packaging for Images Scroller (one flat zip per architecture).
# Runs inside the container:
#   docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh
# Consumes the release exes produced by scripts/build-windows.sh plus the tracked
# config file, and writes ONE zip per architecture into the gitignored dist/
# tree. Zip layout is FLAT: the exe and the config file sit at the zip root, no
# subfolders (the user extracts the zip into a destination folder of their
# choice — that folder is the install folder). Zip artifacts are gitignored and
# must NEVER be committed; this script never touches git.
# Typed failures: every failure line starts with the literal prefix
# "package-windows: error:" and the script exits 1. A failed run never deletes
# or modifies a previously installed zip; all transient files live under the
# staging directory, which an EXIT trap removes.

set -u

readonly PROJECT_DIR="/img-scroller"
readonly CARGO_MANIFEST="Cargo.toml"
readonly STAGING_DIR="dist/package-staging"
readonly MANIFEST_FILE="dist/package-staging/manifest.txt"
readonly X64_ARCH="x64"
readonly X86_ARCH="x86"

# The packaging manifest (the single source of packaged content): one line per
# architecture and packaged file, format "<arch>|<source path>|<zip entry name>".
# A later task (default-viewer helper/guidance) appends its entries here; no
# other part of the script changes for that.
package_manifest_entries() {
  printf '%s\n' "$X64_ARCH|dist/windows-x64/img-scroller.exe|img-scroller.exe"
  printf '%s\n' "$X64_ARCH|packaging/img-scroller.toml|img-scroller.toml"
  printf '%s\n' "$X86_ARCH|dist/windows-i686/img-scroller.exe|img-scroller.exe"
  printf '%s\n' "$X86_ARCH|packaging/img-scroller.toml|img-scroller.toml"
}

report_failure() {
  printf 'package-windows: error: %s\n' "$1" >&2
  exit 1
}

assert_current_directory() {
  if [ "$(pwd)" != "$PROJECT_DIR" ]; then
    report_failure "wrong working directory: $(pwd); expected: $PROJECT_DIR (run through docker compose)"
  fi
}

assert_source_present() {
  if [ ! -s "$1" ]; then
    report_failure "packaged file missing or empty: $1 (produce it with scripts/build-windows.sh first)"
  fi
}

cleanup_staging() {
  rm -rf "$STAGING_DIR"
}

extract_package_version() {
  package_version=$(sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' "$CARGO_MANIFEST" | head -n 1)
  if [ -z "$package_version" ]; then
    report_failure "cannot extract the package version from $CARGO_MANIFEST"
  fi
  printf '%s' "$package_version"
}

build_zip_for_arch() {
  arch="$1"
  zip_base_name="$2"
  part_zip_path="$STAGING_DIR/$zip_base_name.zip.part"
  staging_dir="$STAGING_DIR/$zip_base_name"
  mkdir -p "$staging_dir" || report_failure "cannot create staging directory: $staging_dir"
  while IFS='|' read -r entry_arch source_path entry_name; do
    if [ "$entry_arch" != "$arch" ]; then
      continue
    fi
    assert_source_present "$source_path"
    if [ -e "$staging_dir/$entry_name" ]; then
      report_failure "duplicate zip entry name in the manifest: $entry_name"
    fi
    cp "$source_path" "$staging_dir/$entry_name" || report_failure "copy failed: $source_path -> $staging_dir/$entry_name"
    zip -q -X -j "$part_zip_path" "$staging_dir/$entry_name" || report_failure "zip failed adding: $entry_name -> $part_zip_path"
  done < "$MANIFEST_FILE"
  [ -s "$part_zip_path" ] || report_failure "zip is missing or empty after packaging: $part_zip_path"
  mv "$part_zip_path" "dist/$zip_base_name.zip" || report_failure "install of packaged zip failed: $part_zip_path -> dist/$zip_base_name.zip"
  printf 'package-windows: zip: dist/%s.zip\n' "$zip_base_name"
}

main() {
  assert_current_directory
  cleanup_staging
  mkdir -p "$STAGING_DIR" || report_failure "cannot create staging directory: $STAGING_DIR"
  package_manifest_entries > "$MANIFEST_FILE"
  package_version=$(extract_package_version) || exit 1
  build_zip_for_arch "$X64_ARCH" "ImagesScroller-$package_version-windows-x64"
  build_zip_for_arch "$X86_ARCH" "ImagesScroller-$package_version-windows-x86"
  printf 'package-windows: OK\n'
  exit 0
}

trap cleanup_staging EXIT
main "$@"
```

- Shell-function parameter discipline: no script function takes more than 2 parameters (functions read the `readonly` constants instead — the project's max-arguments rule honored in shell).

### D4 — Build matrix (PINNED — one table)

| Item | x64 | x86 |
| --- | --- | --- |
| Release target triple | `x86_64-pc-windows-gnu` | `i686-pc-windows-gnu` |
| rustup target line | existing (Wave 1) | new (this task) |
| MinGW apt package | `gcc-mingw-w64-x86-64` (existing) | `gcc-mingw-w64-i686` (new, D1) |
| Audit tool (objdump) | `x86_64-w64-mingw32-objdump` | `i686-w64-mingw32-objdump` |
| Build invocation (single-target) | `sh scripts/build-windows.sh x86_64-pc-windows-gnu` | `sh scripts/build-windows.sh i686-pc-windows-gnu` |
| Build invocation (both) | `sh scripts/build-windows.sh` (no argument — builds x64 then x86) | same command |
| Artifact path (gitignored) | `dist/windows-x64/img-scroller.exe` (UNCHANGED) | `dist/windows-i686/img-scroller.exe` (NEW) |
| Zip name | `ImagesScroller-<version>-windows-x64.zip` | `ImagesScroller-<version>-windows-x86.zip` |
| Zip contents (flat, at zip root) | `img-scroller.exe`, `img-scroller.toml` | `img-scroller.exe`, `img-scroller.toml` |
| DLL import audit status | OBSERVED 22-DLL list (script constant + `docs/build-windows.md`) | OBSERVED list recorded by Task 4 (expect the same 22 system DLLs; any delta adjudicated there) |

### D5 — "The user picks a destination" semantics (recorded behavior)

- The zip is a PLAIN ZIP archive. The user extracts it into a destination folder of their own choice using the standard Windows Explorer "Extract All..." flow (no self-extracting exe, no installer UI in v1 — the brief's "user picks a destination" is the extract-destination choice). The chosen destination IS the install folder: `img-scroller.exe` + `img-scroller.toml` land flat at its root. Implementation impact: NONE beyond the flat zip layout (D3) — this decision is recorded behavior. README/installation wording belongs to Task 5.

### D6 — Smoke-build expectation for step 4.2 (this task's verification evidence)

- The implementer runs, through the `alpine-vm` MCP (`vm_status` FIRST; exit code authoritative), in this order: (1) the Dockerfile rebuild (exit 0); (2) the x64 single-target build (exit 0, non-empty `dist/windows-x64/img-scroller.exe` — the pre-existing 0.6.2-era artifact is atomically overwritten by the 1.0.0 rebuild); (3) the x86 single-target build (exit 0, non-empty `dist/windows-i686/img-scroller.exe` — first x86 build compiles the whole dependency tree for the new target: allow a long timeout); (4) the no-argument build run (exit 0 — validates the both-targets default path); (5) one typed-failure spot check with a bogus target (exit 1 EXPECTED — the authoritative evidence of the preserved typed-failure contract); (6) one packaging run (exit 0 — produces BOTH zips in one invocation); (7) zip listing checks (`unzip -l`, exit 0, exactly 2 entries each). Task 4 re-runs the formal dual-architecture verification (dev-checks, artifacts, x86 DLL audit).

### D7 — Unit tests: NONE; census stays pinned; no dev-checks run in Task 2

- **Decision: NO Rust-side tests and NO Rust-side helper.** The packaging logic is shell (a zip script); the build extension changes no Rust code; no pure function lands in `src/`. A Rust-side packaging helper would add dead code to the binary surface for shell-expressible logic — not warranted. The census stays **195 unit tests / 28 test-bearing files** (19 `*_tests.rs` + 4 `tests.rs` + 5 in-file modules). NO dev-checks run is required for Task 2: `src/` and the Cargo manifests are untouched (the formal dev-checks run belongs to Task 4 at wave HEAD). The scripts' correctness evidence IS the smoke run (D6): exit codes, typed failures, artifact/zip checks.

### D8 — Docs touchpoints decision

- Task 2 touches **NO prose documentation**: `README.md`, `.agent/project-info/{tech,architecture,context-entries,brief,product}.md`, `docs/build.md`, and `docs/build-windows.md` are all Task-5 scope (the global plan's Task-5 pre-analysis already assigns them the x86/packaging/build-matrix records; the `docs/build-windows.md` typed-failure table gains its new rows there). The ONE non-docs exception, per the structure rule: **`.agent/project-structure.md` IS updated by Task 2** (the `scripts/` bullet now covers three harness scripts). The 4.4 docs-specialist expected outcome: "nothing to document; deferred to Task 5" (same pattern as Task 1). The context-log (`context.md`) closing touch of THIS 4.1b architector step is already recorded by the architector (process bookkeeping, not prose docs).

## 1. Environment Facts (verified 2026-11-14)

- Git state at planning: `main` == `origin/main` == `0ec23e6`; Task 2 executes on `feat/wave-7-distribution` AFTER Steps 2–3 (branch + 1.0.0 bump) per the global plan's step map; the Task-1 commits (config file + `.gitattributes` line) precede this task.
- Repo files relevant here: `Dockerfile` (15 lines), `docker-compose.yml` (unchanged this task), `scripts/build-windows.sh` (74 lines, single-target), `packaging/img-scroller.toml` (Task 1, 11 lines), `.gitignore` covers `dist/`, `logs/`, `target/`, `*.log`; `.gitattributes` = `*.sh text eol=lf` + `packaging/*.toml text eol=lf`.
- Alpine VM: RUNNING and SSH-reachable (verified via `vm_status` 2026-11-14). Docker + Compose plugin exist ONLY on the VM; all commands go through the `alpine-vm` MCP (`vm_status` FIRST; exit code authoritative; single commands, no `&&` chains on the VM shell). If the VM or the `/img-scroller` share is unavailable: STOP and return a user question (wave execution note).
- Base-image verification (exit 0, throwaway `rust:1.99.1-bookworm` container): `gcc-mingw-w64-i686` candidate `12.2.0-14+25.2` providing `i686-w64-mingw32-gcc`/`-ld`/`-objdump`; `zip` candidate `3.0-13+deb12u1`; `unzip` candidate `6.0-28+deb12u1`.
- `Cargo.toml` version line: line 3 `version = "1.0.0"` — the ONLY column-0 `version = "..."` line (D3's sed extraction is safe).
- Census pinned: 195 unit tests / 28 test-bearing files (D7 — unchanged by Task 2).
- Existing `dist/windows-x64/` also holds the gitignored diagnostic exe + log (kept per the Planner's Wave-6.2 decision) — the packaging manifest is explicit, so those files are never packaged; nothing touches them.

## 2. Binding Constraints (apply to every step)

- All build/run verification ONLY via the container harness on the Alpine VM (`alpine-vm` MCP; `vm_status` FIRST; exit code authoritative; compose invocation pattern per `docs/build.md`).
- NO `git push` (restricted to Step 5). NO branch creation/switch (restricted to Step 2). NO version edit (restricted to Step 3 — the packaging script READS `Cargo.toml`, never writes it).
- Gitignore Compliance Rule before EVERY commit: read `.gitignore`, run `git status`, ensure NO `dist/`, `logs/`, `target/`, or `*.log` path is staged (the zips and staging tree live under `dist/` — never staged).
- Do NOT edit any existing `.kilo/plans/*` file. This plan file is the only NEW plan file of this task.
- Do NOT edit `README.md`, `docs/*` (`build.md`, `build-windows.md` included), `.agent/project-info/{brief,product,tech,architecture}.md`, `CHANGELOG.md` — Task 5 owns the wave's prose. Exception: `.agent/project-structure.md` (structure rule) IS edited by Task 2.
- NO helper/guidance files (Task 3); NO manifest extension beyond the four pinned lines (Task 3).
- File writes use real newline characters (never literal `\n` escape sequences) — newline-prevention rule. Shell scripts MUST reach the container with LF endings (the Wave-2 CRLF incident) — the plan encodes CR checks for every edited/new script AND the Dockerfile.
- Max-lines rules: not applicable to `scripts/`/`Dockerfile` (src/ rule), but the scripts stay under the same spirit (~110 lines each).

## 3. Step-by-Step Implementation (workflow step 4.2 — implementer, JUNIOR under 50%)

Execute the steps in order. Check the plan between steps. Stop and ask the caller on any ambiguity — never guess. Every `alpine-vm_vm_run_command` call below is ONE command; the exit code is authoritative.

### Step 0 — Commit the plan file (caller/implementer, BEFORE implementation)

- Verify: `git status` → on branch `feat/wave-7-distribution`; the tree contains (at least) the new file `.kilo/plans/20261114-wave7-task2-installer-packaging-plan.md` (the Task-1 ride-along files, if still unstaged, are NOT this commit).
- Stage ONLY the plan file: `git add .kilo/plans/20261114-wave7-task2-installer-packaging-plan.md`
- Commit (verbatim message): `docs: add wave7 task2 packaging plan`

### Step 1 — Extend the Dockerfile

- Replace the WHOLE content of `C:\repo\img-scroller\Dockerfile` with the exact 21-line content from D1 (real newlines; NO BOM).
- (The old file is in version control history — do not preserve anything beyond what D1 shows.)

### Step 2 — Extend `scripts/build-windows.sh`

- Replace the WHOLE content of `C:\repo\img-scroller\scripts\build-windows.sh` with the exact content from D2 (real newlines, LF endings).

### Step 3 — Line-ending checks (CR-free evidence) for both edited files

- Call `vm_status` FIRST. Then run, each as its own command (exit 0 = LF-only; exit 1 = a CR was found → fix the file's line endings and re-run):
  1. `sh -c "! grep -q \"$(printf '\r')\" /img-scroller/Dockerfile"`
  2. `sh -c "! grep -q \"$(printf '\r')\" /img-scroller/scripts/build-windows.sh"`

### Step 4 — Rebuild the container image (REQUIRED after the Dockerfile change)

- `docker compose -f /img-scroller/docker-compose.yml build rust` — expected exit 0. (Timeout allowance: up to 30 min.)

### Step 5 — x64 single-target release build

- `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu`
- Expected: exit 0; output contains `build-windows: building target: x86_64-pc-windows-gnu`, `build-windows: artifact: dist/windows-x64/img-scroller.exe`, the OBSERVED 22-DLL audit line, `build-windows: OK`. The 1.0.0 rebuild atomically overwrites the pre-existing artifact (timeout allowance: up to 60 min).

### Step 6 — x86 single-target release build

- `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu`
- Expected: exit 0; output contains `build-windows: building target: i686-pc-windows-gnu`, `build-windows: artifact: dist/windows-i686/img-scroller.exe`, the x86 audit-note line (`... recorded by the Wave-7 verification task ...`), `build-windows: OK`. First x86 build compiles the full dependency tree for the new target (timeout allowance: up to 60 min). A typed `build-windows: error:` line or a nonzero exit → STOP and ask the caller (risk §8).

### Step 7 — No-argument build run + typed-failure spot check

- `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` — expected exit 0 (both targets rebuild-noop + reinstall; validates the default both-targets path).
- `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh not-a-target` — expected **exit 1** (THIS IS THE EXPECTED OUTCOME; the captured output must contain `build-windows: error: unknown target: not-a-target ...`). Do not treat this exit 1 as a failure of the task — it is the evidence.

### Step 8 — Gitignore compliance + commit A (build extension)

- `git status` → the staged/untracked set must contain NO `dist/`, `logs/`, `target/`, or `*.log` path (the zips do not exist yet; `dist/windows-i686/` is gitignored and must NOT appear as staged).
- Stage EXACTLY: `git add Dockerfile scripts/build-windows.sh`
- Commit (verbatim message): `feat: extend windows release build to x86`
- Post-commit `git status`: only this task's remaining files (Step 9–10) unstaged.

### Step 9 — Create the packaging script

- Create `C:\repo\img-scroller\scripts\package-windows.sh` with the EXACT content from D3 (real newlines, LF endings).

### Step 10 — Update the structure map

- Edit `C:\repo\img-scroller\.agent\project-structure.md`, ONLY the `scripts/` bullet in `# Other folders` — replace it with:

```
- scripts/ - container harness scripts: dev-checks, Windows x64/x86 release cross-build, and portable zip packaging (POSIX sh)
```

- Do NOT touch any other line of the file (the `packaging/` bullet from Task 1 stays as-is).

### Step 11 — Container verification of the packaging (the task's acceptable evidence)

- Call `vm_status` FIRST. Then run, each as its own command:
  1. CR check: `sh -c "! grep -q \"$(printf '\r')\" /img-scroller/scripts/package-windows.sh"` — exit 0.
  2. Packaging run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh` — expected exit 0; output contains `package-windows: zip: dist/ImagesScroller-1.0.0-windows-x64.zip`, `package-windows: zip: dist/ImagesScroller-1.0.0-windows-x86.zip`, `package-windows: OK`.
  3. `ls -l /img-scroller/dist/windows-x64/img-scroller.exe /img-scroller/dist/windows-i686/img-scroller.exe /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip` — exit 0; all four paths non-empty (sizes > 0).
  4. `unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip` — exit 0; EXACTLY two entries: `img-scroller.exe` and `img-scroller.toml` (no directory entries).
  5. `unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip` — exit 0; EXACTLY the same two entries.
- A typed `package-windows: error:` line or a nonzero exit → STOP and ask the caller.

### Step 12 — Gitignore compliance + commit B (packaging)

- `git status` → confirm NO gitignored path is staged (the zips + `dist/package-staging/` are under `dist/` — the EXIT trap already removed the staging tree; the zips remain on disk, gitignored, never staged).
- Stage EXACTLY: `git add scripts/package-windows.sh .agent/project-structure.md`
- Commit (verbatim message): `feat: add portable windows zip packaging`

### Explicitly NOT done in Task 2 (for the implementer)

- NO Rust/source changes; NO `Cargo.toml`/`Cargo.lock` edits (version was bumped at Step 3 by the caller's flow — this task only reads it); NO dev-checks run (Task 4); NO tests added (D7 — census stays 195/28); NO `docs/`/`README.md`/project-info prose edits (Task 5; the context-log 4.1b entry is the architector's, already recorded); NO `CHANGELOG.md` edit; NO helper/guidance files or manifest extension (Task 3); NO `git push`; NO branch/version actions; NO TODO marking (that is 4.6).

## 4. Workflow step 4.3 — Code Review & Simplification guidance

- code-reviewer checklist (deviation from plan = finding): the Dockerfile matches D1 exactly (21 lines; i686 + zip + unzip appended to the ONE apt line; i686 rustup line after the x64 one); `build-windows.sh` matches D2 exactly — typed-failure prefix preserved, `|| exit 1` on every fallible command substitution, atomic `.part` install preserved, x64 path unchanged, x86 path `dist/windows-i686/img-scroller.exe`, audit note per-target (x86 line carries NO list), exit-0-or-1 contract; `package-windows.sh` matches D3 exactly — manifest single-source function, flat layout via staging + `-j`, dynamic version extraction with typed empty-failure, part-then-rename install, EXIT-trap staging cleanup, script never touches git; structure-map bullet matches Step 10; the two commits contain ONLY their pinned files; NO file under `src/`, `Cargo.toml`, `Cargo.lock`, `docs/`, `.agent/project-info/*` (except nothing), `README.md`, or `CHANGELOG.md` changed vs the Step-0 baseline; no gitignored path committed.
- code-simplifier: review the two shell scripts for simplification (dead duplication, needless branches). Expected outcome: minor or none — the scripts are deliberately structured per the plan's decisions; any change must NOT alter the pinned invocation contract, paths, names, or typed-failure text without a plan amendment. If simplification is too extensive, a NEW plan file under `.kilo/plans/` (never edit this one).
- Max 3 review cycles, then escalate (workflow rule).

## 5. Workflow step 4.4 — Documentation guidance

- The docs-specialist adds NOTHING in Task 2 (D8): all wave prose documentation — README distribution/installation section, `tech.md` packaging/build-matrix records, `architecture.md` pipeline updates, `docs/build-windows.md` x86 guide rows + the x86 OBSERVED list once Task 4 observes it, `docs/build.md` if harness semantics changed — is Task 5's deliverable. Expected outcome: "nothing to document; deferred to Task 5" as a clear message. Creating any documentation file in Task 2 is a plan deviation.

## 6. Workflow step 4.5b — Plan-adherence guidance

- Verify sections 0–3 against the landed diff (`git diff <Step-0-commit>..<Task-2 last commit>`): the exact file set (Dockerfile, `scripts/build-windows.sh`, `scripts/package-windows.sh`, `.agent/project-structure.md`) with the exact contents/decisions of D1–D3 + Step 10, and the smoke-evidence points of D6/Steps 3–7 + 11 (rebuild exit 0; x64/x86/no-arg builds exit 0; bogus-target exit 1 with the typed line; packaging exit 0; the two `unzip -l` listings with exactly 2 flat entries each; CR checks exit 0).
- If every element conforms: return a clear "no diffs, VERIFIED" message (no report file needed). If a deviation exists: record it in a NEW plan file `.kilo/plans/20261114-wave7-task2-adherence.md` and propose the disposition — never edit existing plans.

## 7. Workflow step 4.6 — Task completion (implementer)

- Edit `.agent/todos/20261029/20261029-todo-9.md` (preserve ALL other content byte-identical):
  - `### Task 2: Zip-style portable installer` → `### Task 2: Zip-style portable installer [DONE]`
  - Its THREE sub-items, each `- [ ]` → `- [x]` keeping the rest of each line verbatim: the destination-unzip item, the x64 AND x86 release-builds item, and the packaging-mechanism item.
- Commit message (verbatim): `task: mark wave7 task2 done`
- Gitignore compliance before the commit (`git status`; the TODO file is tracked — safe).

## 8. Risks / stop conditions

- VM or `/img-scroller` share unavailable at any step → STOP, return a user question (wave execution note).
- x86 toolchain/image-rebuild failures → typed failures via the script's `build-windows: error:` discipline; pause and ask the user before improvising.
- x86 cargo build failure (new target, full dependency recompile) → the typed cargo-failure line appears; STOP and ask the caller; do not modify `Cargo.toml`/`Cargo.lock` to "fix" it.
- x86 DLL-audit delta on the produced artifact → adjudicated in Task 4 (expect the same 22 system DLLs; any new MinGW runtime DLL means the build is not portable — fix before packaging there).
- Any plan ambiguity encountered by a sub-agent → STOP and return the question to the Planner; never guess.
- Review-cycle cap: max 3 (4.3) cycles, then escalate to the user.

## 9. Plan-vs-task compliance check

- TODO Task 2 requires: (1) "The user picks a destination folder; the package unzips the application binary AND the config file into that destination" ✔ (D3 flat zips with exe + config at the root; D5 records the extract-destination semantics; README wording = Task 5); (2) "Release builds for x64 AND x86 ... extension of Wave 1's harness ... exact extension mechanism/details decided in this wave's architector step" ✔ (D1 Dockerfile `gcc-mingw-w64-i686` + `i686-pc-windows-gnu` — package name and provided binaries verified against the pinned image; D2 script extension mechanism, paths, audit handling, all pinned); (3) "The packaging mechanism (script vs program, archive tooling) is decided ... consumes the produced x64/x86 artifacts from the gitignored dist/ locations" ✔ (D3: tracked `scripts/package-windows.sh`, zip tooling in-image, container-side, consuming both dist exes + the Task-1 config). Also settled from the wave's deferred list for this task: build matrix (D4), one-zip-vs-two (D3 — two zips, justified), zip layout/naming (D3), x86 realization details (D1/D2). Nothing from Tasks 3–6 is executed (helper mechanism untouched; verification/dev-checks/manual-UX = Task 4; prose docs = Task 5; CHANGELOG = Task 6). The plan is complete and correct against the task.
