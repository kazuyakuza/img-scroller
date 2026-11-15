#!/bin/sh
# Portable zip packaging for Images Scroller (one flat zip per architecture).
# Runs inside the container:
#   docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh
# Consumes the release exes produced by scripts/build-windows.sh plus the tracked
# packaged files (the config file, the default-viewer registration helper and
# the viewer guidance text), and writes ONE zip per architecture into the
# gitignored dist/ tree. Zip layout is FLAT: the exe, the config file, the
# helper script and the guidance file sit at the zip root, no subfolders (the
# user extracts the zip into a destination folder of their choice — that
# folder is the install folder). Zip artifacts are gitignored and must NEVER
# be committed; this script never touches git.
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
package_manifest_entries() {
  printf '%s\n' "$X64_ARCH|dist/windows-x64/img-scroller.exe|img-scroller.exe"
  printf '%s\n' "$X64_ARCH|packaging/img-scroller.toml|img-scroller.toml"
  printf '%s\n' "$X64_ARCH|packaging/register-default-viewer.cmd|register-default-viewer.cmd"
  printf '%s\n' "$X64_ARCH|packaging/DEFAULT-VIEWER-GUIDE.txt|DEFAULT-VIEWER-GUIDE.txt"
  printf '%s\n' "$X86_ARCH|dist/windows-i686/img-scroller.exe|img-scroller.exe"
  printf '%s\n' "$X86_ARCH|packaging/img-scroller.toml|img-scroller.toml"
  printf '%s\n' "$X86_ARCH|packaging/register-default-viewer.cmd|register-default-viewer.cmd"
  printf '%s\n' "$X86_ARCH|packaging/DEFAULT-VIEWER-GUIDE.txt|DEFAULT-VIEWER-GUIDE.txt"
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
  # Strip CRs first: the Windows working tree checks Cargo.toml out with CRLF
  # line endings (.gitattributes pins LF only for *.sh and packaging/*.toml),
  # and the end-of-line anchor below must match both LF and CRLF checkouts.
  package_version=$(tr -d '\r' < "$CARGO_MANIFEST" | sed -n 's/^version = "\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)"$/\1/p' | head -n 1)
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
