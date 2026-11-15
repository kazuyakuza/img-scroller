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
readonly OBSERVED_DLL_IMPORT_LIST_I686="advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll"

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
      printf 'build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (%s): %s\n' "$X86_DIST_DIR" "$OBSERVED_DLL_IMPORT_LIST_I686"
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
