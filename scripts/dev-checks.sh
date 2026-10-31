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
