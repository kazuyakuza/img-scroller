# Amendment — Task 3 Implementation Plan: target-scope the `windows` dependency

- Date: 2026-10-31
- Parent plan (read together with this file; do NOT edit it): [`.kilo/plans/20261031-task3-wave1-open-image-implementation.md`](20261031-task3-wave1-open-image-implementation.md)
- Workflow step: Critical Workflow 4.1b MINI AMENDMENT (Task 3), resumed 4.2 for the Task-3 implementation.
- Plan approval: AUTO-APPROVED ("Approve Global and Tasks Plans", 2026-10-30). Save + return path; no user approval round.
- Trigger (implementer evidence, deterministic): parent-plan fact V2 is FALSE as applied — with `windows@=0.62.2` in unconditional `[dependencies]`, `cargo check --locked` on the container's Linux target FAILS exit 101: `windows-future 0.3.2` + `windows-threading 0.2.1` (transitives) reference `windows_core::imp::IMarshal` / `marshaler` / `submit`, which exist only on Windows targets — 16 × E0425, ALL in registry crates, none in `src/`. Separately, the per-plan `cargo add` retry hit the known vboxsf `Text file busy (os error 26)` ×2 (deterministic on this share, same as Task 2); the authorized fallback already wrote the §2.1.0 dependency entries host-side and materialized the lock (tree currently holds `M Cargo.toml` + `M Cargo.lock`; lockfile sanity OK per R2: additions only, eframe family untouched, `raw-window-handle` stays 0.6.2).
- Scope: ONLY the manifest structural fix + its consequences. No unrelated changes.

## 0. Corrected fact (V2)

- CORRECTED: the windows-family crates (including transitives `windows-future`, `windows-threading`) DO compile-fail on the Linux host target when `windows` is declared unconditionally — the crate-root cfg does not gate out all transitives' item references. Target-scoping the dependency is the standard `windows-rs` pattern (its own documentation recommends target-specific dependencies via `[target.'cfg(windows)'.dependencies]`).
- All other parent-plan facts (V1, V3–V8) stand unchanged.

## 1. Exact final manifest state (mandatory)

The `windows` dependency moves into a TARGET-SCOPED table; `raw-window-handle` stays unconditional. The complete final `Cargo.toml` must read EXACTLY (full file, byte-for-byte the target state; LF endings, trailing newline):

```toml
[package]
name = "img-scroller"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "=0.37.0", default-features = false, features = [
    "glow",
    "default_fonts",
    "clipboard",
    "links",
    "wayland",
    "x11",
] }
raw-window-handle = "=0.6.2"

[target.'cfg(windows)'.dependencies]
windows = { version = "=0.62.2", features = [
    "Win32_Foundation",
    "Win32_Graphics_Imaging",
    "Win32_System_Com",
    "Win32_UI_Shell",
    "Win32_UI_Shell_Common",
    "Win32_UI_WindowsAndMessaging",
] }
```

SHAPE check is value-exact: pin `=0.62.2`, the EXACT six features, `windows` in `[target.'cfg(windows)'.dependencies]`, `raw-window-handle = "=0.6.2"` unconditional, eframe entry byte-identical to today. Cargo-add's persisted formatting (whitespace/feature order) may differ — whitespace does not matter.

## 2. Explicit consequences

1. `src/platform/windows.rs` keeps its verbatim draft (§2.1.5) UNCHANGED — it is inside the `#[cfg(target_os = "windows")] mod windows_impl;` gating at `platform/mod.rs` (§2.1.4); the facade pattern and both signatures are untouched; the stub side (§2.1.6) is untouched.
2. On the container Linux target, the windows family is never built anymore: `cargo check --locked` / `cargo test --locked` / `cargo clippy --locked -- -D warnings` resolve without `windows-future`/`windows-threading` item errors → dev-checks (S5) is attainable.
3. S6 keeps its first-validation role unchanged: `cargo check --locked --target x86_64-pc-windows-gnu` now actually builds the windows family for GNU and compiles `windows.rs` — the §3-S6 micro-fix list applies exactly as written.
4. Lockfile R2 analysis unchanged: `Cargo.lock` covers all targets — the windows-family entries remain locked (no stage deletes them); eframe-family entries untouched; `raw-window-handle = 0.6.2` holds; eframe's `^0.6.2` is still satisfied. NO R2 re-analysis is needed.
5. The dependency-add procedure in parent-plan §3-S2 is SUPERSEDED by §3 of this file (C-S2). The `--locked` policy afterward is unchanged: from S3 onward every harness cargo invocation uses `--locked`; a manifest/lock mismatch after this point remains a STOP condition.

## 3. Resume instructions for 4.2 (starts AT the corrected manifest state)

Current observed tree state (2026-10-31): `Cargo.toml` ALREADY holds all three dependency entries — but `windows` sits in unconditional `[dependencies]` (the form proven to fail) — and `Cargo.lock` is already materialized (additions only). Therefore:

- C-S1 Preconditions (unchanged, re-run): `alpine-vm_vm_status` first; host `git status` → expect exactly `M Cargo.toml` + `M Cargo.lock` from the pre-authorized fallback; any OTHER drift → STOP + report. Parent-plan S1 file checks stand (`src/` still Task-2 shape: no `src/platform/`, no `src/image_loading.rs`).
- C-S2 Manifest target-scoping (host file tools; NO `cargo add` runs on this resume):
  1. Edit `Cargo.toml` host-side to the EXACT §1 content (remove the `windows` block from `[dependencies]`; append the target-scoped table).
  2. Run ONE lock-consistency run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check` — `timeoutMs: 180000` — NO `--locked`, ONE command only, exit code authoritative. Target-scoping does not change lock resolution (the lock covers all targets), so expected: exit 0 AND `git diff Cargo.lock` shows NO change. A Cargo.lock diff → STOP + report with full output (unexpected drift). If the check fails otherwise → STOP + report. (If the VM/share is unavailable for this re-run, STOP + user question — this single run is required to certify the final manifest state; a re-run with an unchanged lock is harmless by design.)
  3. Record in the 4.2 summary that the pre-authorized host-side fallback path was used for BOTH the original §2.1.0 entries AND this target-scoping rewrite (with captured output lines); `cargo add` is NOT retried — EBUSY is deterministic on this share (×2 here + ×3 in Task 2 evidence).
  4. Superseded S2 primary path (kept ONLY for the hypothetical case the manifest regresses to MISSING entries — e.g. a reset): `vm_status` → ONE `cargo add windows@=0.62.2 --features Win32_Foundation,Win32_Graphics_Imaging,Win32_System_Com,Win32_UI_Shell,Win32_UI_Shell_Common,Win32_UI_WindowsAndMessaging --target cfg(windows)` + ONE `cargo add raw-window-handle@=0.6.2`, each `timeoutMs: 180000`; on EBUSY-class failure do NOT retry → proceed DIRECTLY to the fallback: host-side write the EXACT §1 TOML + the single `cargo check` of step C-S2.2.
- C-S3 onward: parent-plan steps S3–S4 run VERBATIM, then S5/S6 with the consequence adjustments of §2 (S5 expected outcome now explicitly "windows family not built" alongside the parent-plan expectations), S7 commit per §4 (file list unchanged), S8 summary.
- Verification of this amendment (for 4.5b): `Cargo.toml` matches §1 value-exact; dev-checks S5 exit 0 on the Linux target; S6 exit 0 on `x86_64-pc-windows-gnu`; `git diff Cargo.lock` shows windows-family additions only, eframe family untouched, `raw-window-handle` 0.6.2.

## 4. No other plan deliverable changes (explicit confirmation)

- Drafts §2.1.1–§2.1.7 stand VERBATIM (including `windows.rs` §2.1.5 — its imports resolve only on the windows side, which compiles exclusively under S6).
- Steps S3–S4 unchanged. S6 unchanged (unchanged command, timeout, micro-fix list, STOP rule). Step S8 unchanged.
- Commit §4 file list unchanged: `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/app.rs`, `src/image_loading.rs`, `src/platform/mod.rs`, `src/platform/windows.rs`, `src/platform/stub.rs`, `.agent/project-structure.md`.
- §5 frontend-spec mapping rows: all unchanged.
- §6 verification mapping rows: checked — NO row references the old S2; no §6 edit is required.
- §7 risks: R1/R2 narratives remain as written (the fallback illustrated there is the same one this resume already used; R2 verification stands). No new risk rows.
- §8 cross-task bindings unchanged (Task 4/5 consume `DecodedImage`, test counts, and the warmed GNU target volume exactly as written).

## 5. Risks unique to this amendment

| # | Risk | Handling |
| --- | --- | --- |
| A1 | Implementer "helpfully" removes windows-family entries from `Cargo.lock` after target-scoping | FORBIDDEN: never hand-edit `Cargo.lock`. The lock holds all-target entries; S5's `--locked` uses resolution across targets as-is. Any manual lock edit = STOP. |
| A2 | EBUSY strikes the host-side-edit-on-share flow again (edit appears not to persist) | Host-side writes via host file tools do NOT go through the container share lock path used by container-side writers (Tasks 2/3 evidence: host writes persisted fine; container writes hit EBUSY). If verification (step C-S2.2's `cat Cargo.toml`-equivalent read inside the container) shows stale content → STOP + report, do not loop retries. |
| A3 | Feature name typo in the target-scoped table breaks S6 | The six features are value-checked verbatim in §1; S6 validates them by compiling for GNU. Do not add/remove/alter features under any circumstance beyond STOP+report. |
