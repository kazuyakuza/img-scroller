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
