# Review Fix Plan — Task 1 Wave 1 Build Harness: `scripts/build-windows.sh` command-substitution failure-path bug

## Status

CONFIRMED. The defect is present in the committed file at `scripts/build-windows.sh`.

## POSIX sh verification

In `main()`:

```sh
final_artifact_path=$(install_artifact_into_dist "$source_artifact_path")
```

`install_artifact_into_dist` invokes `report_failure` for four failure classes:

1. `dist/` directory-create failure (`mkdir -p`)
2. copy failure (`cp`)
3. install-of-copy failure (`mv`)
4. artifact missing/empty after copy (`[ -s ]`)

Per IEEE Std 1003.1, command substitution (`$()`) executes the enclosed compound command in a **subshell environment** (2.6.3). An `exit` performed inside that subshell terminates **only the subshell**, not the parent shell (2.12 Shell Execution Environment).

Consequences in the current code:

- `report_failure`'s `printf` writes to the subshell's **stdout**, which is captured into `final_artifact_path`; the operator never sees the error line.
- `report_failure`'s `exit 1` terminates only the subshell.
- The assignment statement receives exit status 1 from the subshell but, because it is unguarded, `main` continues.
- `main` then prints `build-windows: OK`, prints `build-windows: artifact: build-windows: error: <message>`, and exits 0.

Result: 4 of the 7 typed failure classes violate the contract (deterministic exit 1 + visible `build-windows: error:` line). The other 3 classes (`wrong working directory`, `unset/empty CARGO_TARGET_DIR`, `cargo build failure`, and `expected release executable not found or empty`) are called directly in `main`'s shell and work correctly.

## Chosen fix

**Option (b)** from the review instructions: keep the command substitution, redirect `report_failure` output to **stderr**, and guard the substitution with `|| exit 1`.

### Justification

- **Fewest changed lines**: only 2 lines differ from the committed file.
- **Bulletproof typed-failure semantics**:
  - stderr is **not** captured by the command substitution, so the operator sees every `build-windows: error:` line.
  - The subshell's `exit 1` becomes the exit status of the assignment statement, so `|| exit 1` terminates the parent shell with exit 1 before any success note is printed.
- Preserves every settled decision: names, paths, `.part`-then-rename safety, no-deletion-of-pre-existing-artifacts guarantee, success-note lines, and OBSERVED DLL placeholder.
- Remains POSIX dash-safe and within all project line-count/depth rules.

Alternative (a) (drop command substitution, use a global result variable) would also work but touches 6+ lines and introduces a cross-function global dependency; it is unnecessary because option (b) fixes the subshell semantics directly.

## Exact before/after

### Change 1 — `report_failure` writes to stderr

Before:

```sh
report_failure() {
  printf 'build-windows: error: %s\n' "$1"
  exit 1
}
```

After:

```sh
report_failure() {
  printf 'build-windows: error: %s\n' "$1" >&2
  exit 1
}
```

### Change 2 — guard the command substitution in `main`

Before:

```sh
  final_artifact_path=$(install_artifact_into_dist "$source_artifact_path")
```

After:

```sh
  final_artifact_path=$(install_artifact_into_dist "$source_artifact_path") || exit 1
```

## Full corrected `scripts/build-windows.sh` (verbatim replacement)

```sh
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
  final_artifact_path=$(install_artifact_into_dist "$source_artifact_path") || exit 1
  printf 'build-windows: OK\n'
  printf 'build-windows: artifact: %s\n' "$final_artifact_path"
  printf 'build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list: %s\n' "$OBSERVED_DLL_IMPORT_LIST"
  exit 0
}

main "$@"
```

## Why this does not change any settled decision

- Names, paths, and the decision chain (`dist/windows-x64/img-scroller.exe`, `CARGO_TARGET_DIR=/cargo-target`, etc.) are unchanged.
- The `.part`-then-rename copy semantics and the guarantee that a failed run never deletes/modifies a pre-existing artifact are unchanged.
- Success note lines (`build-windows: OK`, artifact line, OBSERVED DLL placeholder line) are unchanged.
- Cargo diagnostics remain untouched.
- The fix is purely a local robustness correction to the failure-reporting path.

## Post-fix verification (implementer)

1. **Syntax check only** (do not run the harness; no Cargo project exists yet):
   ```sh
   docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -n scripts/build-windows.sh
   ```
   Expected: exit 0.

2. **Manual trace checklist** (reason through each path; no execution required):

   | Failure class | Code path | Expected behavior after fix |
   | --- | --- | --- |
   | wrong working directory | `assert_current_directory` in `main` shell | `report_failure` writes to stderr, exits parent shell with 1; no OK line |
   | unset/empty `CARGO_TARGET_DIR` | `assert_cargo_target_dir` in `main` shell | same |
   | cargo build failure | `cargo build ... \|\| report_failure` in `main` shell | same |
   | expected release executable not found/empty | `assert_source_artifact_present` in `main` shell | same |
   | `dist/` directory-create failure | `mkdir -p ... \|\| report_failure` inside command substitution | error line goes to stderr (visible); subshell exits 1; `\|\| exit 1` in `main` exits process 1; OK line not printed |
   | copy failure | `cp ... \|\| { rm -f .part; report_failure; }` inside command substitution | same |
   | install-of-copy failure (rename) | `mv ... \|\| { rm -f .part; report_failure; }` inside command substitution | same |
   | artifact missing/empty after copy | `[ -s ... ] \|\| report_failure` inside command substitution | same |

3. Confirm that on every failure path the full process exits `1` and no `build-windows: OK` line is emitted.
