# Wave 8 — Task 5 Verification Results

- Source TODO: `.agent/todos/20261117/20261117-todo-1.md` → `### Task 5: Verification`.
- Verification plan: `.kilo/plans/20261118-wave8-task5-verification-plan.md` (approved; §0 V1–V8 binding).
- Date: 2026-11-18.
- Branch: `feat/wave-8-wizard-installer`; HEAD at run start: `8df8894` (the results file is committed after the plan commit `7a14f23`).
- Version: 1.1.0.
- Execution channel: `alpine-vm` MCP, `vm_status` first before every VM command; the exit code is authoritative. Workflow step 4.2 (implementer). No source, script, packaging, or documentation file changed by Task 5.

---

## 1. Dev-Checks Evidence (V1)

Command (exit code authoritative):

`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh`

- Exit code: `0`.
- Log path printed by the harness: `logs/checks/dev-checks-20261118T231035Z.log`.
- PASS lines (from the harness log):

```text
PASS fmt-check exit=0 (1 s)
PASS check exit=0 (8 s)
PASS test exit=0 (22 s)
PASS clippy exit=0 (5 s)
ALL CHECKS PASSED
```

- Cargo test summary: the harness's `cargo test --locked` step (the harness runs exactly `cargo test --locked`) reports:

```text
test result: ok. 213 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s
```

The same `cargo test --locked` invocation was re-captured directly to read its final summary line because the MCP display truncated the harness's combined output at 10,000 characters; the harness itself exited `0` with `ALL CHECKS PASSED`.

- Census file-count corroboration (pinned method):

`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh -c "grep -rl '#\[test\]' src | wc -l"`

- Exit code: `0`; output: `29`.

- The pre-existing benign clippy finding (`compare_entry_names` dead-code warning in `src/name_collation.rs`) appeared during the release builds; the harness clippy step passed (`clippy -- -D warnings` exit 0). No NEW fmt/clippy finding.

---

## 2. Census Statement (V1 verbatim substance)

Task 5 adds NO tests and NO `src/` change. The TODO's "unit tests for the new pure logic" sub-item is discharged by the formal wave-HEAD harness run exercising the tests Task 4 already landed: the 18-test composition matrix in `src/window_title/tests.rs` (full-path/name/last-chars fitting, the containing-folder/drive/parent chain, the `...` marker, the constants). Census: `195 → 213` tests and `28 → 29` test-bearing files (the Wave-8 delta is `src/window_title/tests.rs`).

The installer/association-selection logic produced NO Rust pure logic: the wizard UI, the association page, and the registry writes are the NSIS script `packaging/nsis/installer.nsi` — installer-runtime NSIS instructions executed on Windows, not Rust, not compilable or testable by the dev-checks harness (Task-1 D18 pinned this; zero Rust lines exist for it and zero tests are added).

Breakdown (informational): 29 = 19 `*_tests.rs` + 5 `tests.rs` child files (the Wave-8 delta is `src/window_title/tests.rs`) + 5 in-file test modules (`app_display.rs`, `image_loading.rs`, `name_collation.rs`, `scaling.rs`, `zoom.rs`).

---

## 3. Build Evidence (V2/V3) — Fresh Chain at Wave HEAD

Pinned order executed: x64 build → x86 build → installer build → zip packaging; each its own command, exit code authoritative.

### 3.1 x64 release build

`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu`

- Exit code: `0`.
- Pinned output lines:

```text
build-windows: building target: x86_64-pc-windows-gnu
build-windows: artifact: dist/windows-x64/img-scroller.exe
build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (dist/windows-x64): advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll
build-windows: OK
```

### 3.2 x86 release build

`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu`

- Exit code: `0`.
- Pinned output lines:

```text
build-windows: building target: i686-pc-windows-gnu
build-windows: artifact: dist/windows-i686/img-scroller.exe
build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (dist/windows-i686): advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll
build-windows: OK
```

### 3.3 Installer build

`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-installer.sh`

- Exit code: `0`.
- Pinned output lines:

```text
build-installer: artifact: dist/ImagesScroller-1.1.0-windows-installer.exe
build-installer: OK
```

- `makensis` warning `9100` (no standard key `LegalCopyright`) appeared; NON-FATAL by design (Task-1 D19) — exit 0 + a non-empty artifact is the success evidence.

### 3.4 Zip packaging (RETAINED per Task-1 D12)

`docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh`

- Exit code: `0`.
- Pinned output lines:

```text
package-windows: zip: dist/ImagesScroller-1.1.0-windows-x64.zip
package-windows: zip: dist/ImagesScroller-1.1.0-windows-x86.zip
package-windows: OK
```

### 3.5 Freshness decisions recorded (V3)

- Fresh x64 build REQUIRED: the installer embeds the x64 exe (`-DEXE_PAYLOAD`), and both the installer and the x64 zip are release artifacts; the payload is now compiled from wave-HEAD source.
- Fresh x86 build REQUIRED: the x86 exe exists ONLY inside the x86 zip; Task 1 built only x64, so the Task-1-era x86 payload had unverified provenance. The fresh build removes all doubt.
- The Task-1-era artifacts (installer at 6,836,150 B and both 1.1.0 zips) are SUPERSEDED as the FORMAL record by this fresh chain. A byte-size difference vs the planning-time value is NOT a failure (non-empty is the gate; sizes are evidence only).

---

## 4. Audit Evidence (V4) — Both DLL Import Audits on the Fresh Artifacts

### 4.1 x64 audit

LIST command: `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u`

- Exit code: `0`; observed list verbatim (22 names):

```text
advapi32.dll
api-ms-win-core-path-l1-1-0.dll
api-ms-win-core-synch-l1-2-0.dll
api-ms-win-core-winrt-error-l1-1-0.dll
bcryptprimitives.dll
combase.dll
dwmapi.dll
gdi32.dll
imm32.dll
kernel32.dll
msvcrt.dll
ntdll.dll
ole32.dll
oleaut32.dll
opengl32.dll
rpcrt4.dll
shell32.dll
shlwapi.dll
user32.dll
userenv.dll
uxtheme.dll
ws2_32.dll
```

COUNT command (same pipeline + `| wc -l`): exit code `0`; output `22`.

### 4.2 x86 audit

LIST command: `docker compose -f /img-scroller/docker-compose.yml run --rm rust i686-w64-mingw32-objdump -x dist/windows-i686/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u`

- Exit code: `0`; observed list verbatim (22 names):

```text
advapi32.dll
api-ms-win-core-path-l1-1-0.dll
api-ms-win-core-synch-l1-2-0.dll
api-ms-win-core-winrt-error-l1-1-0.dll
bcryptprimitives.dll
combase.dll
dwmapi.dll
gdi32.dll
imm32.dll
kernel32.dll
msvcrt.dll
ntdll.dll
ole32.dll
oleaut32.dll
opengl32.dll
rpcrt4.dll
shell32.dll
shlwapi.dll
user32.dll
userenv.dll
uxtheme.dll
ws2_32.dll
```

COUNT command (same pipeline + `| wc -l`): exit code `0`; output `22`.

### 4.3 Adjudication

- BOTH observed lists are SET-EQUAL to the recorded 22-DLL constant (`OBSERVED_DLL_IMPORT_LIST` and `OBSERVED_DLL_IMPORT_LIST_I686` in `scripts/build-windows.sh`), verified item-by-item, order-independent.
- No NEW name and no MISSING name; no MinGW runtime DLL (`libgcc_s_dw2.dll` / `libstdc++-6.dll` / `libwinpthread-1.dll`).
- NO script fill was performed (the constants pre-exist; a delta would have been an adjudication stop, and none occurred).

---

## 5. Packaging Evidence (V2.4 + Wave-7 D5 check set)

### 5.1 Zip listings (4 flat entries each)

`unzip -l /img-scroller/dist/ImagesScroller-1.1.0-windows-x64.zip` — exit code `0`:

```text
  Length      Date    Time    Name
---------  ---------- -----   ----
 19675523  11-18-2026 23:14   img-scroller.exe
      495  11-18-2026 23:14   img-scroller.toml
     3727  11-18-2026 23:14   register-default-viewer.cmd
     2747  11-18-2026 23:14   DEFAULT-VIEWER-GUIDE.txt
 --------                     -------
 19682492                     4 files
```

`unzip -l /img-scroller/dist/ImagesScroller-1.1.0-windows-x86.zip` — exit code `0`:

```text
  Length      Date    Time    Name
---------  ---------- -----   ----
 19828521  11-18-2026 23:14   img-scroller.exe
      495  11-18-2026 23:14   img-scroller.toml
     3727  11-18-2026 23:14   register-default-viewer.cmd
     2747  11-18-2026 23:14   DEFAULT-VIEWER-GUIDE.txt
 --------                     -------
 19835490                     4 files
```

Both archives contain EXACTLY the four flat entries (no directory entries): `img-scroller.exe`, `img-scroller.toml`, `register-default-viewer.cmd`, `DEFAULT-VIEWER-GUIDE.txt`.

### 5.2 Packaged-.cmd CRLF evidence (both zips)

- `sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.1.0-windows-x64.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""` — exit code `0`.
- `sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.1.0-windows-x86.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""` — exit code `0`.

The packaged helper copy carries CRLF in both archives.

### 5.3 Artifact byte sizes (verbatim from `ls -l`, exit code `0`)

```text
 6846665  /img-scroller/dist/ImagesScroller-1.1.0-windows-installer.exe
 6720409  /img-scroller/dist/ImagesScroller-1.1.0-windows-x64.zip
 7172404  /img-scroller/dist/ImagesScroller-1.1.0-windows-x86.zip
19828521  /img-scroller/dist/windows-i686/img-scroller.exe
19675523  /img-scroller/dist/windows-x64/img-scroller.exe
```

All five artifacts are non-empty; the installer name is exactly `dist/ImagesScroller-1.1.0-windows-installer.exe` and both zips are exactly the 1.1.0 names (no version drift).

---

## 6. Pending OPERATOR-RUN Gate (V6) — the User's Run

- The TODO's manual-UX sub-item STAYS `[ ]` until the user reports the run.
- The PLAIN, self-contained OPERATOR-RUN checklist is `.kilo/plans/20261118-wave8-task5-verification-plan.md` §8 (handed verbatim to the user).
- The checklist's GitHub-release item (item 16) is GATED on Task 7 (GitHub Releases publishing): the user reports it "pending Task 7" until the release exists; the rest of the checklist can run now.
- Any §8 item FAIL means the wave is NOT closed; the caller decides the fix path.

---

## 7. Verdict

`VERIFIED` — agent-side evidence complete: dev-checks `ALL CHECKS PASSED` exit 0 with census `213/29`; the fresh artifact chain (x64 → x86 → installer → zips) exited 0 at every step with non-empty, correctly named artifacts; BOTH DLL import audits are SET-EQUAL 22 with no delta and no script fill; both zips carry exactly four flat entries with CRLF helper copies. The OPERATOR-RUN gate remains open for the user.
