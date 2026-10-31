#!/bin/sh
# Windows x64 release cross-build for Images Scroller.
# Runs inside the container: docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh
# Typed failures: every failure line starts with the literal prefix "build-windows: error:"
# and the script exits 1. A failed run never deletes or modifies a previous artifact.

set -u

readonly PROJECT_DIR="/img-scroller"
readonly WINDOWS_TARGET="x86_64-pc-windows-gnu"
readonly RELEASE_SUBDIR="release"
readonly BINARY_NAME="img-scroller.exe"
readonly DIST_DIR="dist/windows-x64"
readonly PART_FILE_SUFFIX=".part"
readonly OBSERVED_DLL_IMPORT_LIST="PENDING — filled once in Wave 1 verification (Task 5) from the x86_64-w64-mingw32-objdump import audit of our produced exe"

report_failure() {
  printf 'build-windows: error: %s\n' "$1"
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

compose_source_artifact_path() {
  printf '%s/%s/%s/%s' "$CARGO_TARGET_DIR" "$WINDOWS_TARGET" "$RELEASE_SUBDIR" "$BINARY_NAME"
}

assert_source_artifact_present() {
  if [ ! -s "$1" ]; then
    report_failure "expected release executable not found or empty: $1"
  fi
}

install_artifact_into_dist() {
  source_artifact_path="$1"
  final_artifact_path="$DIST_DIR/$BINARY_NAME"
  part_artifact_path="$DIST_DIR/$BINARY_NAME$PART_FILE_SUFFIX"
  mkdir -p "$DIST_DIR" || report_failure "cannot create dist directory: $DIST_DIR"
  cp "$source_artifact_path" "$part_artifact_path" || {
    rm -f "$part_artifact_path"
    report_failure "copy failed: $source_artifact_path -> $DIST_DIR"
  }
  mv "$part_artifact_path" "$final_artifact_path" || {
    rm -f "$part_artifact_path"
    report_failure "install of copied artifact failed: $part_artifact_path -> $final_artifact_path"
  }
  [ -s "$final_artifact_path" ] || report_failure "artifact missing or empty after copy: $final_artifact_path"
  printf '%s' "$final_artifact_path"
}

main() {
  assert_current_directory
  assert_cargo_target_dir
  source_artifact_path=$(compose_source_artifact_path)
  cargo build --release --locked --target "$WINDOWS_TARGET" || report_failure "cargo build failed (exit $?); check Cargo.toml / Cargo.lock consistency via the dev checks"
  assert_source_artifact_present "$source_artifact_path"
  final_artifact_path=$(install_artifact_into_dist "$source_artifact_path")
  printf 'build-windows: OK\n'
  printf 'build-windows: artifact: %s\n' "$final_artifact_path"
  printf 'build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list: %s\n' "$OBSERVED_DLL_IMPORT_LIST"
  exit 0
}

main "$@"
