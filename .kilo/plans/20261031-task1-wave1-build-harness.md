# Implementation Plan — Wave 1, Task 1: Container Build Harness (Docker on the Alpine VM)

- Date: 2026-10-31
- Workflow step: Critical Workflow §4.1b (Analysis & Planning) — Task 1 of TODO
  [`.agent/todos/20261029/20261029-todo-3.md`](../../.agent/todos/20261029/20261029-todo-3.md) ("### Task 1: Container build harness (Docker on the Alpine VM)")
- Role: Architector. Plan ONLY — the file below is the deliverable of this step.
- Executing branch (already created in Step 2): `feat/wave-1-skeleton-open-image` — no branch creation in this task.
- Plan approval: per-task plans are AUTO-APPROVED (user chose "Approve Global and Tasks Plans", 2026-10-30). Return the plan path after saving; do NOT ask for approval.
- Front-end marking: Task 1 is NOT front-end related → 4.1b only (no 4.1a, no 4.5a).
- Global plan in force (do NOT modify): [`.kilo/plans/20261030-wave1-skeleton-open-image.md`](20261030-wave1-skeleton-open-image.md) — §0.2 (binding build environment), §0.3 (deferred decisions), §1 (sequence), §2 Task 1, §3 (risks).
- Target implementer: JUNIOR developer, 50% restriction — every structural/naming/scope decision is encoded here; no "pick one" is left behind.

## 0. Scope

### 0.1 In scope (TODO Task 1 sub-items, verbatim mapping)

| # | TODO sub-item (abbreviated) | Delivered by this plan |
| --- | --- | --- |
| 1 | `Dockerfile`: pinned official Rust base image; `rustup component add rustfmt clippy`; `gcc-mingw-w64-x86-64`; `rustup target add x86_64-pc-windows-gnu`; NO FFmpeg; `objdump` available for the import audit | §2.1 |
| 2 | `docker-compose.yml`: single `rust` service; bind mount `/img-scroller:/img-scroller`; `working_dir: /img-scroller`; named volume + `CARGO_TARGET_DIR` into it; no `target/` leak into repo root; only root artifact = tracked `Cargo.lock` | §2.2 |
| 3 | `scripts/dev-checks.sh` (POSIX sh): ordered checks, PASS/FAIL lines, summary, UTC log under `logs/checks/`, exit 0/1, FORCE_FAIL hook (synthetic exit 7), never modifies sources / never applies fmt | §2.3 |
| 4 | `scripts/build-windows.sh` (POSIX sh): `cargo build --release --locked --target x86_64-pc-windows-gnu`; cwd + `CARGO_TARGET_DIR` assertions; copy ONLY the final `.exe` into `dist/` subfolder; typed `build-windows: error:` failures → exit 1; passthrough diagnostics; failed run leaves `dist/` untouched | §2.4 |
| 5 | Import-audit retained scope: script does NOT run the audit; success note + docs carry the OBSERVED DLL placeholder; filled once in Task 5 via `x86_64-w64-mingw32-objdump` on OUR exe; reference DLL list NEVER copied | §2.4 + §2.6 + §8 |
| 6 | `docs/build.md`: dev checks + usage through the Alpine VM; per-check fallbacks; FORCE_FAIL; UTC log; expected output semantics; `alpine-vm` MCP notes; container-only validation stance; explicitly NOT a Linux-release-build guide | §2.5 |
| 7 | `docs/build-windows.md`: cross-build guide; prerequisites; invocation; output path; overwrite/rebuild; typed failure table; toolchain/target notes; static-link/import-audit OBSERVED-list placeholder; container-only stance + operator-run Windows validation; x64 ONLY (x86 = Wave 7) | §2.6 |
| 8 | `.gitignore`: verify `dist/`, `logs/`, `*.log` remain (verified 2026-10-31 — all three present); add EXACTLY one new entry: `target/` | §2.7 |
| 9 | Policy: `Cargo.lock` tracked; every harness cargo invocation uses `--locked` | §2.3, §2.4, §2.5 |
| 10 | Alpine-VM/MCP operational notes (landed in `docs/build.md`) | §2.5 |
| 11 | Ordering note: files/docs land AND image built ONCE on the VM; harness commands are first EXERCISED in Task 5 (per TODO) | §3 |
| 12 | (workflow) `.agent/project-structure.md` accuracy: new `scripts/` folder must be reflected | §2.8 |
| 13 | (workflow) Commits per step with meaningful messages; gitignore compliance before every commit | §4 |

### 0.2 Out of scope (explicit)

- No `Cargo.toml`, no `Cargo.lock`, no `src/` content — those land in Task 2.
- Do NOT run `scripts/dev-checks.sh` or `scripts/build-windows.sh` in this task (no Cargo project exists yet; harness runs are first EXERCISED in Task 5 per TODO Task 1 ordering note; whether Task 2's implementation also exercises them is decided in Task 2's own 4.1b — TODO Task 1's binding statement here is: THIS task does not exercise them).
- Do NOT run the import audit (`x86_64-w64-mingw32-objdump`) — no exe exists; Task 5 fills the OBSERVED list.
- No Linux release build of any kind; no Linux artifact is ever produced.
- No Windows GUI/runtime validation (operator-run on real Windows 10/11, later tasks).
- No changes to `README.md`, `CHANGELOG.md`, `.agent/project-info/*` (Task 6 / Planner), no edits to existing `.kilo/plans/*` files.
- No digest pinning of the Docker image; no x86/i686 work (Wave 7).

### 0.3 Binding constraints (global plan §0.2 + tech.md §Build Environment)

- ALL Rust/Cargo runs inside a Linux Docker container on the Alpine VM; host needs NO Rust/Cargo.
- VM access only via `alpine-vm` MCP: call `vm_status` BEFORE any command; the command EXIT CODE is the authoritative success signal; cargo stderr may be ABSENT in captured MCP output — an absent stderr is NOT a failure signal.
- Project shared on the VM at `/img-scroller`; Docker + Compose plugin exist ONLY on the VM.
- If the VM or the `/img-scroller` share is unavailable → STOP and return a user question.
- Reference pattern (user-approved, adapt — never copy names/pins): `kazuyakuza/rust-youtube-streamer`. Its `rust:1.82` pin and its FFmpeg install are explicitly NOT adopted. Its volume name, dist/exe names, and DLL list are NEVER copied.
- Validation stance: container-only; Windows GUI validation is OPERATOR-RUN on real Windows.

## 1. Architector decisions (settled NOW; bind later tasks)

Verified 2026-10-31 against Docker Hub (`rust` official image tags) and Rust release data: current stable Rust line is **1.99.x** (1.99.0 released 2026-10-01, 1.99.1 released 2026-10-15); official tags `1`, `1.99`, `1.99.1`, plus distro variants `1.99.1-bookworm`, `1.99.1-trixie` exist on the official `rust` repository.

### 1.1 Decision table

| # | Decision | Value | Rationale |
| --- | --- | --- | --- |
| a | Pinned official Rust base image | **`rust:1.99.1-bookworm`** (full variant, NOT slim) | Exact major.minor.patch + Debian codename tag = reproducible without digest pinning; NOT the floating `latest`/`1` tags; NOT the reference's `1.82`. Full variant ships build tools (gcc, make, git, pkg-config) so only the MinGW cross package needs an apt install. `bookworm` (Debian 12) chosen over `trixie`: the MinGW-w64 x64 package set is long-stable there and it sidesteps Debian-13 transition churn; Debian 12 has a long support tail. Updating the pin later is a deliberate, plan-gated change (do not auto-bump). |
| b | Named volume for build artifacts | Volume name **`img-scroller-cargo-target`**; container mount path **`/cargo-target`**; env value **`CARGO_TARGET_DIR=/cargo-target`** | Volume name is project-derived (NOT the reference's); mount path is short, outside the bind-mounted tree, and cannot collide with `/img-scroller` paths. Cargo then writes ALL build-state (all target subdirs for every target triple) into the named volume, so nothing lands in the repo root except the tracked `Cargo.lock`. |
| c | `dist/` subfolder + `.exe` naming | Subfolder **`dist/windows-x64/`**; final exe name **`img-scroller.exe`** → full path `dist/windows-x64/img-scroller.exe` | Arch-suffixed subfolder leaves room for the Wave 7 `windows-x86` artifact without renaming; exe name comes directly from the binary name (decision d). `dist/` is already gitignored (verified). NOT the reference's names. |
| d | Cargo package/binary naming chain | Package name **`img-scroller`** (valid Cargo name: lowercase, alnum + hyphen); binary inherits the package name (NO `[[bin]]` section, NO bin-name override in Task 2's `Cargo.toml`); produced Windows exe = binary name + `.exe` = **`img-scroller.exe`** | Consistent with the repo (`img-scroller`) and the app final title (`Images Scroller`); single inherited chain makes the exe filename deterministic for `build-windows.sh` (§2.4) with zero coupling to manifest internals. BINDS Task 2: `Cargo.toml` `package.name = "img-scroller"`, no `[[bin]]`, no `bin`.rename of any kind. |
| e | Compose-managed image name | **`img-scroller-rust:latest`** (explicit tag in compose `image:`) | Deterministic, doc-friendly name for the locally built harness image; project-derived, never the reference's. |
| f | Dev-checks behavior on early failure | ALL FOUR checks always run (no early stop); failures counted; summary at end | A complete `fmt/check/test/clippy` report per run beats fail-fast for an agent harness; matches the per-check line format's purpose. |
| g | FORCE_FAIL mechanics | Match is exact case-sensitive equality with the check name; when matched, the REAL command still runs (output passes through) but the recorded exit code becomes `7`; unknown FORCE_FAIL values are silently ignored (no validation) | Exactly the TODO semantics ("exactly that check fails with synthetic exit 7"); no additional validation beyond spec. |
| h | `build-windows.sh` assertion order | 1) cwd == `/img-scroller`, 2) `CARGO_TARGET_DIR` set and non-empty | cwd is the more fundamental precondition (plugin compose always sets both; a wrong cwd means everything else is suspect). Encoded so the implementer does not re-order. |
| i | Safe copy into `dist/` | Copy goes to a transient `img-scroller.exe.part` file inside `dist/windows-x64/`, then atomic rename to the final name; on cp/mv failure the PART file (only the part file — a file created by THIS run, never a pre-existing artifact) is removed | Guarantees the byte-for-byte "failed run leaves `dist/` untouched (previous artifact stays in place)" promise even against a partial copy; the transient file cannot collide with the final name. |
| j | Log-file naming + collision | `logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log` (UTC); same-second re-run truncates (`: >`) the same file | Simple, sortable, gitignored via `logs/`; truncation removes any race need for suffix loops. |
| k | sh dialect | Strict POSIX sh (no `local`, no arrays, no `[[ ]]`); the container base is Debian `bookworm` whose `sh` is `dash` | The Dockerfile base's shell is dash; POSIX-clean scripts run identically. Scripts are invoked as `sh scripts/<name>.sh` (never `./scripts/...` — no exec bit is guaranteed by the Windows→VM share). |
| l | `.gitignore` new entry placement | Single line `target/` added inside the existing `# Build artifacts (generic)` block, immediately after `dist/` | EXACTLY one new entry per TODO; no new comment lines. |

### 1.2 Naming chain carried forward (single decision chain)

- `Cargo.toml` (Task 2): `package.name = "img-scroller"`, `package.version = "0.1.0"` (exact value from global plan §0.5), edition = decided in Task 2's 4.1b, NO `[[bin]]` section.
- Binary name: `img-scroller` → source exe at `$CARGO_TARGET_DIR/x86_64-pc-windows-gnu/release/img-scroller.exe`.
- Deliverable exe: `dist/windows-x64/img-scroller.exe`.
- Later-wave artifacts follow the same convention: `dist/windows-<arch>/img-scroller.exe` (Wave 7 note; recorded here so the Consume+extend pattern has a stable contract).
- Task 5 4.1b MUST fill the OBSERVED DLL placeholder in `scripts/build-windows.sh` (constant) and `docs/build-windows.md` once the first exe is audited with `x86_64-w64-mingw32-objdump`.

## 2. Deliverables — full drafts (implementer materializes verbatim)

Nothing here may be re-interpreted; the drafts are authoritative. All scripts: POSIX sh, LF line endings, trailing newline, no commented-out code, shallow nesting, ≤2 conditions per boolean statement (project single-section boolean rule applies).

### 2.1 `Dockerfile` (new, repository root)

```dockerfile
# Images Scroller — container build harness (dev checks + Windows x64 cross-build).
# Pinned official Rust base image; a movable tag (latest/1) is deliberately not used.
FROM rust:1.99.1-bookworm

# Lint/format components on the default stable toolchain shipped in the base image.
RUN rustup component add rustfmt clippy

# MinGW-w64 x86-64 GNU cross toolchain:
# x86_64-w64-mingw32-gcc is the linker backend for x86_64-pc-windows-gnu;
# x86_64-w64-mingw32-objdump (via the package's binutils dependency) backs the
# Windows import audit performed during Wave 1 verification (Task 5).
RUN apt-get update && apt-get install -y --no-install-recommends gcc-mingw-w64-x86-64 && rm -rf /var/lib/apt/lists/*

# Windows x64 cross-compilation target on the same default toolchain.
RUN rustup target add x86_64-pc-windows-gnu
```

Notes:
- Layer order follows the TODO bullet order (rustfmt/clippy → mingw packages → target) so 4.3 review maps line-by-line.
- The single apt RUN chain minimizes image size (no stale apt lists).
- NO FFmpeg anywhere (explicitly dropped reference adaptation).
- No `CMD`/`ENTRYPOINT` — compose `run` always passes an explicit command.

### 2.2 `docker-compose.yml` (new, repository root)

```yaml
services:
  rust:
    image: img-scroller-rust:latest
    build:
      context: .
      dockerfile: Dockerfile
    working_dir: /img-scroller
    volumes:
      - /img-scroller:/img-scroller
      - img-scroller-cargo-target:/cargo-target
    environment:
      CARGO_TARGET_DIR: /cargo-target

volumes:
  cargo-target:
    name: img-scroller-cargo-target
```

Semantics the implementer must be able to state back:
- Bind mount `/img-scroller:/img-scroller`: the repo is visible inside the container at the same path; `working_dir: /img-scroller` makes every command start at the repo root.
- Named volume `img-scroller-cargo-target` (long-form declaration) is mounted at `/cargo-target`; `CARGO_TARGET_DIR=/cargo-target` redirects ALL cargo build state there. Consequence: container builds leave NO `target/` in the repo root; the only repo-root artifact cargo generates is the tracked `Cargo.lock`.
- `image:` + `build:` together tag the built image `img-scroller-rust:latest`.
- No `version:` key: the Compose v2 spec ignores it (the VM has the Compose plugin).
- Relative `context: .` resolves to the compose file's directory (`/img-scroller`) because commands always use `-f /img-scroller/docker-compose.yml`.

### 2.3 `scripts/dev-checks.sh` (new)

```sh
#!/bin/sh
# Dev checks harness for Images Scroller.
# Runs inside the container: docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
# Checks run in a fixed order and EVERY check runs even if earlier ones fail.
# This script never modifies source files and never applies cargo fmt fixes.

set -u

readonly CHECK_LOG_DIR="logs/checks"
readonly FORCE_FAIL_EXIT_CODE=7

utc_log_suffix() {
  date -u +"%Y%m%dT%H%M%SZ"
}

log_line() {
  printf '%s\n' "$1" | tee -a "$dev_checks_log_file"
}

run_check() {
  check_name="$1"
  shift
  log_line "==> $check_name"
  check_start_seconds=$(date +%s)
  "$@" 2>&1
  check_exit_code=$?
  check_duration_seconds=$(( $(date +%s) - check_start_seconds ))
  if [ "${FORCE_FAIL:-}" = "$check_name" ]; then
    check_exit_code=$FORCE_FAIL_EXIT_CODE
  fi
  if [ "$check_exit_code" -eq 0 ]; then
    log_line "PASS $check_name exit=$check_exit_code ($check_duration_seconds s)"
  else
    dev_checks_failed=$((dev_checks_failed + 1))
    log_line "FAIL $check_name exit=$check_exit_code ($check_duration_seconds s)"
  fi
}

main() {
  mkdir -p "$CHECK_LOG_DIR"
  dev_checks_log_file="$CHECK_LOG_DIR/dev-checks-$(utc_log_suffix).log"
  : > "$dev_checks_log_file"
  log_line "dev-checks starting; log file: $dev_checks_log_file"
  dev_checks_failed=0
  run_check fmt-check cargo fmt --check
  run_check check cargo check --locked
  run_check test cargo test --locked
  run_check clippy cargo clippy --locked -- -D warnings
  if [ "$dev_checks_failed" -eq 0 ]; then
    log_line "ALL CHECKS PASSED"
    exit 0
  fi
  log_line "$dev_checks_failed CHECK(S) FAILED"
  exit 1
}

main "$@"
```

Behavior contract (state these back in review):
1. Check names and commands, in order: `fmt-check` → `cargo fmt --check`; `check` → `cargo check --locked`; `test` → `cargo test --locked`; `clippy` → `cargo clippy --locked -- -D warnings`. Every cargo invocation carries `--locked` (fmt has no lock requirement and uses none).
2. Per-check result line format is exactly `PASS|FAIL <name> exit=<code> (<n> s)`; final summary is exactly `ALL CHECKS PASSED` (exit 0) or `N CHECK(S) FAILED` (exit 1).
3. `"$@" 2>&1` runs the real command with stderr folded into stdout, so error interleaving matches what the operator sees; cargo diagnostics are never suppressed.
4. FORCE_FAIL: `-e FORCE_FAIL=<name>` on `docker compose run` (values: `fmt-check|check|test|clippy`). When the name matches, the real command still runs (its output passes through) but the recorded exit becomes `7` and the check counts as failed. Unknown values are ignored. The hook never modifies sources.
5. Log: created at `$CHECK_LOG_DIR/dev-checks-<UTC suffix>.log` under the gitignored `logs/` tree (relative to cwd, which the compose service fixes at `/img-scroller`); the path is printed at script start; every `==>` header, result line, and summary line is appended to the log as well as shown on the console (tee). Duration is wall-clock whole seconds.
6. No early stop: all four checks run in one invocation; the failure count drives the summary and exit code.
7. POSIX sh discipline: no `local` (function locals are uniquely named globals: `check_*`, `dev_checks_*`), no `[[ ]]`, no arrays; `set -u` only (no `set -e` — per-check codes are handled explicitly).

### 2.4 `scripts/build-windows.sh` (new)

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
```

Behavior contract (state these back in review):
1. Assertion order is cwd FIRST, then `CARGO_TARGET_DIR` (decision h). Build command: `cargo build --release --locked --target x86_64-pc-windows-gnu`.
2. Expected exe path is composed from the decision chain: `$CARGO_TARGET_DIR/x86_64-pc-windows-gnu/release/img-scroller.exe`.
3. Typed failure classes → all lines start literally with `build-windows: error:` and exit is deterministically `1`: wrong cwd; unset/empty `CARGO_TARGET_DIR`; cargo build failure (the `(exit N)` message covers lockfile/manifest mismatch — the message tells the user to reconcile via the dev checks); expected release executable not found or empty (`-s` covers both); `dist/` mkdir failure; copy failure (cp) and install-of-copy failure (mv) as separate lines under the same class; artifact missing/empty after copy.
4. Cargo diagnostics pass through untouched (no redirection away from the container output).
5. `dist/` safety: `mkdir -p` never deletes anything; the copy uses the `.part`-then-rename pattern so a partially copied file can never clobber a previous good exe; on copy/install failure ONLY the transient part file created by THIS run is removed (decision i — the "nothing is deleted" promise protects pre-existing artifacts, not our own temp file that same run created).
6. Success note lines: `build-windows: OK`, `build-windows: artifact: dist/windows-x64/img-scroller.exe` (actual path from the copy step), and the GNU-runtime static-linking evidence line WITH the OBSERVED DLL import-list placeholder (constant `OBSERVED_DLL_IMPORT_LIST`). The script does NOT run the import audit itself (retained scope: Task 5 runs it ONCE and replaces the placeholder HERE and in `docs/build-windows.md`).
7. POSIX sh discipline identical to §2.3; unique global names (`*_artifact_path`, `part_artifact_path`); single-section boolean conditions only.

### 2.5 `docs/build.md` (new — full draft, implementer writes verbatim)

```markdown
# Building and Checking the Project in the Container (Alpine VM)

All Rust/Cargo commands for this project run **inside a Linux Docker container on the Alpine VM**.
The host machine needs **no Rust/Cargo**. This page documents the container dev checks.
It is **NOT** a Linux release-build guide: the project ships **no Linux artifact** and never produces one.

## Prerequisites

- The Alpine VM is running and the project is shared at `/img-scroller`.
- Docker with the Compose plugin is available ONLY on the VM (never on the host).
- Agents reach the VM through the `alpine-vm` MCP (see usage notes below).

## One-time image build

    docker compose -f /img-scroller/docker-compose.yml build rust

Builds the pinned Rust image (`img-scroller-rust:latest`) that contains: rustfmt, clippy, the
MinGW-w64 x86-64 GNU cross toolchain (linker + `x86_64-w64-mingw32-objdump` for the Windows import
audit), and the `x86_64-pc-windows-gnu` target. Rebuild only after a Dockerfile change; toolchain
upgrades are plan-gated.

## Running the dev-check harness

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh

Checks run in this fixed order (every check runs even if earlier ones fail):

1. `cargo fmt --check`
2. `cargo check --locked`
3. `cargo test --locked`
4. `cargo clippy --locked -- -D warnings`

### Expected output semantics

- One line per check: `PASS|FAIL <name> exit=<code> (<n> s)` with names `fmt-check`, `check`, `test`, `clippy`.
- Final line: `ALL CHECKS PASSED` (script exit `0`) or `N CHECK(S) FAILED` (script exit `1`).
- The script never modifies source files and never applies `cargo fmt` fixes.

### Checks log

Every harness run appends its headers, result lines, and summary to a UTC-timestamped log under the
gitignored `logs/checks/` tree: `logs/checks/dev-checks-<YYYYMMDDTHHMMSSZ>.log`. The exact path is
printed at the start of the run and is authoritative.

## FORCE_FAIL (fault-injection hook)

    docker compose -f /img-scroller/docker-compose.yml run --rm -e FORCE_FAIL=fmt-check rust sh scripts/dev-checks.sh

`FORCE_FAIL` accepts exactly one of `fmt-check|check|test|clippy`; the named check then fails with a
synthetic exit code `7` (its real command still runs and its output still passes through). This hook
exists to prove the failure-reporting path works.

## Per-check fallback commands

    docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt --check
    docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked
    docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo test --locked
    docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo clippy --locked -- -D warnings

## alpine-vm MCP usage notes

- Call `vm_status` BEFORE any command; only proceed when the VM is running and SSH-reachable.
- The command EXIT CODE is the authoritative success evidence.
- Captured MCP output may lack cargo stderr — an absent stderr is NOT a failure signal.
- If the VM or the `/img-scroller` share is unavailable, STOP and return a user question.

## Validation stance

Validation is container-only. The compiler, linter, formatter, and tests run in the container; the
Windows GUI/runtime can never run there. Runtime/GUI validation of a produced `.exe` is an
**operator-run** step on a real Windows 10/11 machine.

## Lockfile policy

`Cargo.lock` is tracked in git. Every cargo invocation in this harness uses `--locked`; dependency
changes must land together with the committed `Cargo.lock`.
```

### 2.6 `docs/build-windows.md` (new — full draft, implementer writes verbatim)

```markdown
# Windows x64 Release Cross-Build (Container)

The release `.exe` for Windows x64 is cross-compiled **inside the container**: target
`x86_64-pc-windows-gnu` with the MinGW-w64 x64 GNU toolchain. ONLY 64-bit Windows is covered here —
32-bit (x86) belongs to Wave 7 if realized.

## Prerequisites

- The container image is built once (includes the MinGW-w64 x64 linker and the windows-gnu target) — see [`build.md`](build.md).
- The Alpine VM shares the project at `/img-scroller`; Docker + Compose exist only on the VM.
- Agents use the `alpine-vm` MCP: `vm_status` first; the command exit code is the authoritative success evidence; absent cargo stderr is not a failure signal.

## Invocation

    docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh

## Expected output path

    dist/windows-x64/img-scroller.exe

The repo root `dist/` tree is gitignored; on the Windows host it appears at the repo root under
`dist\windows-x64\`.

## Overwrite / rebuild behavior

- A successful rebuild atomically overwrites the previous exe (the new file is copied to a transient
  `img-scroller.exe.part` and then renamed over the final name).
- A failed run leaves the previous artifact byte-for-byte intact; the transient part file is removed
  on failure and nothing pre-existing is ever deleted.

## Typed failure table

Every failure line starts with the literal prefix `build-windows: error:` and the script exits `1`.

| Failure class | Behavior |
| --- | --- |
| wrong working directory | `build-windows: error: wrong working directory: <cwd>; expected: /img-scroller (run through docker compose)` |
| unset/empty CARGO_TARGET_DIR | `build-windows: error: CARGO_TARGET_DIR is unset or empty (the rust compose service sets it to the named volume)` |
| cargo build failure (incl. lockfile/manifest mismatch) | build diagnostics pass through unmodified, then `build-windows: error: cargo build failed (exit <N>); check Cargo.toml / Cargo.lock consistency via the dev checks` — reconcile manifest and `Cargo.lock` before retrying |
| expected release executable not found or empty | `build-windows: error: expected release executable not found or empty: <path>` |
| `dist/` directory-create failure | `build-windows: error: cannot create dist directory: dist/windows-x64` |
| copy failure | `build-windows: error: copy failed: <source> -> dist/windows-x64` |
| install-of-copy failure (rename) | `build-windows: error: install of copied artifact failed: <part> -> <final>` |
| artifact missing/empty after copy | `build-windows: error: artifact missing or empty after copy: dist/windows-x64/img-scroller.exe` |

## Toolchain / target notes

- The GNU cross target (`x86_64-pc-windows-gnu`) and its linker (`x86_64-w64-mingw32-gcc`) are baked into the pinned image; no local toolchain setup exists on any host.
- The build runs `cargo build --release --locked --target x86_64-pc-windows-gnu`; the GNU runtime components are statically linked into the produced exe (no extra runtime DLLs distributed with the app).

## Static-linking / import audit (retained scope)

The build script does NOT run the import audit itself. Once an exe is produced, the Wave-1
verification session runs ONE import audit with `x86_64-w64-mingw32-objdump` on OUR produced exe and
records the OBSERVED DLL list here and in the script's success note. The list is observed evidence —
never copied from any reference project.

Observed DLL import list:

    PENDING — recorded during Wave 1 verification (first produced exe, audited with x86_64-w64-mingw32-objdump).

## Validation stance

Building is container-only. Running the produced `.exe`, GUI behavior, and image-format handling are
validated by the operator on a real Windows 10/11 machine — agent-side execution from the Linux
container is impossible.

## Lockfile policy

`Cargo.lock` is tracked; the build uses `--locked`; changed dependencies must be committed together
with the regenerated `Cargo.lock`.
```

Both doc drafts stay under ~100 lines — if a 4.4 pass expands any file beyond 100 lines, the docs-specialist adds a TOC per the docs workflow.

### 2.7 `.gitignore` — EXACT edit

Verified 2026-10-31: current root `.gitignore` already contains `logs/`, `*.log` (under `# Logs`) and `dist/` (under `# Build artifacts (generic)`). Do NOT duplicate any of them. Add exactly ONE line:

```gitignore
target/
```

Exact placement: inside the `# Build artifacts (generic)` block, immediately after the `dist/` line:

```gitignore
# Build artifacts (generic)
build/
dist/
target/
```

No new comment lines, no other changes anywhere in the file.

### 2.8 `.agent/project-structure.md` — EXACT edit

A new project-level folder `scripts/` appears; the file lists other folders and must stay accurate. Add ONE bullet under `# Other folders`, keeping the existing alphabetical order (after the `docs/` line):

```markdown
- scripts/ - container harness scripts: dev-checks and Windows x64 cross-build (POSIX sh)
```

No other changes. (`docs/` is already listed; the doc files added here do not change the folder list. `src/` is untouched — Task 2 updates that section.)

## 3. Implementation steps (for 4.2 — ordered, atomic, verifiable)

Run through the `alpine-vm` MCP only for VM-side commands; file writes happen on the host via the normal file tools (the repo IS the `/img-scroller` share).

- S1. Preconditions: call `vm_status`. Expect `running` + SSH reachable. If unavailable → STOP, return a user question (binding rule; do not retry VM startup yourself).
- S2. Write files verbatim from §2.1 → §2.6, in listed order, using the write tool: `Dockerfile`, `docker-compose.yml`, `scripts/dev-checks.sh`, `scripts/build-windows.sh`, `docs/build.md`, `docs/build-windows.md`. Requirement: LF line endings, trailing newline, no BOM (CRLF breaks the container `sh` shebang/execution; see risk §7-R6).
- S3. Apply the exact edits of §2.7 (`.gitignore`) and §2.8 (`.agent/project-structure.md`) with the edit tool.
- S4. Share check on the VM: `vm_status`, then `ls /img-scroller/docker-compose.yml` (command prefix `ls` is allowlisted). Expect exit 0 and the file listed. If absent → the share has not synced; re-check `vm_status` and stop after re-listing once; persistent absence → STOP + user question.
- S5. Image build (the task's single build-once requirement): `docker compose -f /img-scroller/docker-compose.yml build rust`, `timeoutMs: 900000`. Exit code 0 = authoritative success; capture the combined output as evidence.
  - On failure: capture output, then retry the SAME command ONCE (transient apt/registry/network failures).
  - Persistent failure: leave files uncommitted, log everything so far, STOP and escalate to the caller with the captured output.
- S6. Toolchain sanity (no project needed — safe before any `Cargo.toml` exists): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "rustc --version && cargo --version && rustup target list --installed && x86_64-w64-mingw32-gcc --version && x86_64-w64-mingw32-objdump --version"`, `timeoutMs: 120000`. Expect exit 0, output containing the rustc/cargo versions, `x86_64-pc-windows-gnu` in the installed target list, and mingw gcc/objdump version lines.
- S7. Script syntax checks (no execution): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -n scripts/dev-checks.sh` and `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -n scripts/build-windows.sh` (two separate VM calls, both expect exit 0). A failure here after a verbatim write means encoding/line-ending corruption → re-check S2 (LF, no BOM) and re-verify once; persistent failure → STOP.
- S8. Git hygiene + commits per §4.
- S9. Do NOT run `dev-checks.sh` or `build-windows.sh`, do NOT create Cargo files, do NOT create `logs/`/`dist/` contents, do NOT run any cargo command in this task.
- S10. Return a structured summary: files created (level: full verbatim per §2), VM commands run with exit codes, commit hashes. Evidence lives in the MCP captured outputs + summary — do NOT create evidence log files in the repo.

## 4. Commit points and messages (with gitignore compliance before EACH commit)

Committer: implementer (4.2), after S6+S7 succeed so the committed Dockerfile is the image-verified one. Nothing is committed if the image build persists in failing (escalation path S5).

Commit 1 — harness:
- Files: `Dockerfile`, `docker-compose.yml`, `scripts/dev-checks.sh`, `scripts/build-windows.sh`, `.gitignore`, `.agent/project-structure.md`
- Message: `Wire container build harness: pinned Rust image, dev-checks, Windows x64 cross-build`

Commit 2 — docs:
- Files: `docs/build.md`, `docs/build-windows.md`
- Message: `Document container dev-checks and Windows x64 cross-build flows`

Procedure before each commit (Gitignore Compliance Rule):
1. Read `.gitignore`.
2. `git status` — confirm only the intended files are modified/new; confirm NO `dist/`, `logs/`, `*.log`, or `target/` paths exist or are staged (this task must not create any of them).
3. Stage ONLY the listed files (plain `git add <file>` list; no `git add .`).
4. Re-check `git status` — verify the staged list matches the intended files exactly and contains no gitignored patterns.
5. Commit with the exact message above (plain descriptive message; no scope prefixes the repo has not established).
6. `git status` confirms a clean tree afterward.
- Do NOT push (push is restricted to Step 5, `origin` only). No branch operations in this task.

## 5. Alpine-VM operational intent (encoded for docs + implementer)

- The image is built ONCE in this task; its existence means the toolchain is in place for Task 2+.
- Harness runs (`dev-checks.sh`/`build-windows.sh`) are first EXERCISED end-to-end in Task 5 (TODO Task 1 ordering note — binding for THIS task; any earlier exercise is a decision belonging to Task 2's own 4.1b).
- Every VM command in later tasks follows S1/S4's pattern: `vm_status` → command → exit code authoritative.

## 6. Verification mapping (for 4.5b — Overall Plan Adherence)

| TODO Task 1 sub-item | Verify in 4.5b by | Pass state |
| --- | --- | --- |
| Dockerfile (pin, rustfmt/clippy, mingw, target, no FFmpeg) | File matches §2.1 exactly; `docker compose build rust` exit 0 (evidence from 4.2 summary) | Content match + build success |
| docker-compose.yml (service, mounts, volume, CARGO_TARGET_DIR) | File matches §2.2 | Content match |
| scripts/dev-checks.sh (order, lines, summary, log, exits, FORCE_FAIL) | File matches §2.3; `sh -n` exit 0; no cargo run in this task | Content match + syntax check |
| scripts/build-windows.sh (command, assertions, errors, dist behavior, DLL placeholder) | File matches §2.4; `sh -n` exit 0 | Content match + syntax check |
| Import-audit retained scope | Placeholder constant present in script (§2.4) + docs placeholder (§2.6); script does NOT invoke objdump | Placeholder present, no audit executed |
| docs/build.md (all mandated elements incl. MCP notes, not-a-Linux-release-guide) | File matches §2.5 | Content match |
| docs/build-windows.md (all mandated elements incl. failure table, x64-only) | File matches §2.6 | Content match |
| .gitignore (verify dist/, logs/, *.log present; add exactly `target/`) | `git show HEAD:.gitignore` diff shows exactly the one added line | Diff = 1 added line |
| Policy: Cargo.lock tracked + `--locked` everywhere | Msg in docs; `--locked` present in all cargo invocations in both scripts (fmt exempt — no lock interplay) | Grep-verified |
| Alpine-VM/MCP notes | Present in docs/build.md (§2.5) | Content match |
| Ordering note (image built once; harness NOT exercised) | 4.2 summary shows S5/S6/S7 ran and NO dev-checks/build-windows invocation | Summary check |
| Commit hygiene | 2 commits exist on `feat/wave-1-skeleton-open-image` with §4 messages; no ignored files staged; nothing pushed | Git log/status check |
| project-structure.md | `scripts/` bullet added per §2.8 | Content match |

## 7. Risks and mitigations

| # | Risk | Mitigation / handling |
| --- | --- | --- |
| R1 | `gcc-mingw-w64-x86-64` apt resolution fails on the pinned base | The package is a standard Debian `bookworm` component (its binutils dependency ships the objdump needed for the audit). `apt` success is the authoritative check; retry policy §3-S5; persistent → STOP + user. |
| R2 | Image build duration / network (MinGW packages + rustup target download) | Day-one cost; `timeoutMs 900000`; one retry; layer caching makes later rebuilds cheap. |
| R3 | Volume mount / `CARGO_TARGET_DIR` correctness not yet provable | No Cargo project exists, so nothing can be exercised here; the manifest (§2.2) is reviewed as-is, and Task 5 verifies artifacts appear under `/cargo-target` — NOT the repo root. Recorded as a Task 5 verification duty. |
| R4 | FORCE_FAIL semantics drift | Decision g pins exact mechanics (real run + synthetic exit 7 + silent ignore of unknown values); encoded in docs. |
| R5 | `--locked` used before a manifest exists | Harmless by design: scripts are never run in this task; a pre-Task-2 run would produce a plain cargo error (no source modification). Enforced from Task 2 onward. |
| R6 | Line endings/encoding corrupting sh scripts (Windows host → VM share) | S2 mandates LF + no BOM; S7 `sh -n` catches corruption deterministically. |
| R7 | vboxsf-style shared-folder semantics in the container | Only reads on the script paths matter now; cargo write hotspots live in the named volume, keeping the share out of the build path. |
| R8 | MCP capture drops cargo stderr | Exit code is the authoritative signal (binding); docs teach the operator not to infer failure from absent stderr. |
| R9 | Same-second log-file collision on `dev-checks` | Truncation (`: >`) at run start (decision j) removes ambiguity. |
| R10 | Partial copy clobbering a previous good exe | `.part` + atomic rename + part-file cleanup (decision i); previous artifacts never deleted/modified on any failure path. |

## 8. Cross-task bindings (carry to later 4.1b cycles)

- Task 2 (4.1b): MUST honor decision d — `package.name = "img-scroller"`, `version = "0.1.0"`, NO `[[bin]]`; exe name `img-scroller.exe` follows automatically. Halo effect: all later `dist/` consumers use `dist/windows-<arch>/img-scroller.exe` (Wave 7 adds `windows-x86`).
- Task 5 (4.1b): runs `x86_64-w64-mingw32-objdump` import audit ONCE on OUR exe; fills the OBSERVED DLL list in `scripts/build-windows.sh` (`OBSERVED_DLL_IMPORT_LIST` constant) and `docs/build-windows.md`; ALSO verifies `CARGO_TARGET_DIR` redirect (R3) and that the repo root contains no `target/`.
- Harness run timing: TODO Task 1 says first EXERCISED in Task 5; global plan's Task-2 pre-analysis mentions Task-2 exercise. THIS task binds only its own scope (no exercise). Task 2's architector owns the earlier-exercise decision; no conflict within this plan.
- Reference (`kazuyakuza/rust-youtube-streamer`) invariant: nothing copied — pin `1.99.1-bookworm` ≠ `1.82`; volume/compose names, dist/exe names, DLL list, and docs wording are project-derived throughout; FFmpeg is absent by design.
