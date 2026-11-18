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

set -u

readonly PROJECT_DIR="/img-scroller"
readonly CARGO_MANIFEST="Cargo.toml"
readonly INSTALLER_SOURCE="packaging/nsis/installer.nsi"
readonly X64_PAYLOAD="dist/windows-x64/img-scroller.exe"
readonly CONFIG_PAYLOAD="packaging/img-scroller.toml"
readonly LICENSE_PAYLOAD="LICENSE"

report_failure() {
  printf 'build-installer: error: %s\n' "$1" >&2
  exit 1
}

assert_current_directory() {
  if [ "$(pwd)" != "$PROJECT_DIR" ]; then
    report_failure "wrong working directory: $(pwd); expected: $PROJECT_DIR (run through docker compose)"
  fi
}

assert_source_present() {
  if [ ! -s "$1" ]; then
    report_failure "expected source not found or empty: $1"
  fi
}

assert_payload_present() {
  if [ ! -s "$X64_PAYLOAD" ]; then
    report_failure "expected payload not found or empty: $X64_PAYLOAD (produce the x64 exe with scripts/build-windows.sh first)"
  fi
}

# Strip CRs first: the Windows working tree checks Cargo.toml out with CRLF
# line endings (.gitattributes pins LF only for *.sh and packaging/*.toml),
# and the end-of-line anchor below must match both LF and CRLF checkouts.
extract_package_version() {
  package_version=$(tr -d '\r' < "$CARGO_MANIFEST" | sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' | head -n 1)
  if [ -z "$package_version" ]; then
    report_failure "cannot extract the package version from $CARGO_MANIFEST"
  fi
  printf '%s' "$package_version"
}

compile_installer() {
  package_version="$1"
  version4="${package_version}.0"
  out_part="dist/ImagesScroller-${package_version}-windows-installer.exe.part"
  out_final="dist/ImagesScroller-${package_version}-windows-installer.exe"
  makensis -V2 -DPRODUCT_VERSION="${package_version}" -DPRODUCT_VERSION_4="${version4}" -DINSTALLER_OUTFILE="/img-scroller/${out_part}" -DEXE_PAYLOAD="/img-scroller/${X64_PAYLOAD}" -DCONFIG_PAYLOAD="/img-scroller/${CONFIG_PAYLOAD}" -DLICENSE_PAYLOAD="/img-scroller/${LICENSE_PAYLOAD}" "$INSTALLER_SOURCE"
  makensis_status=$?
  if [ "$makensis_status" -ne 0 ]; then
    rm -f "$out_part"
    report_failure "makensis compile failed (exit $makensis_status); see the makensis diagnostics above"
  fi
  if [ ! -s "$out_part" ]; then
    rm -f "$out_part"
    report_failure "installer artifact missing or empty after compile: $out_part"
  fi
  mv "$out_part" "$out_final"
  move_status=$?
  if [ "$move_status" -ne 0 ]; then
    rm -f "$out_part"
    report_failure "install of installer artifact failed: $out_part -> $out_final"
  fi
  printf 'build-installer: artifact: %s\n' "$out_final"
}

main() {
  assert_current_directory
  if ! command -v makensis >/dev/null 2>&1; then
    report_failure "makensis not found (rebuild the container image after the Dockerfile change)"
  fi
  assert_source_present "$INSTALLER_SOURCE"
  assert_payload_present
  assert_source_present "$CONFIG_PAYLOAD"
  assert_source_present "$LICENSE_PAYLOAD"
  package_version=$(extract_package_version) || exit 1
  compile_installer "$package_version"
  printf 'build-installer: OK\n'
  exit 0
}

main "$@"
