# Wave 7 — Task 4 Verification Results

- Source TODO: `.agent/todos/20261029/20261029-todo-9.md` → `### Task 4: Verification`
- Executed plan: `.kilo/plans/20261114-wave7-task4-verification-plan.md`
- Date: 2026-11-15
- Branch: `feat/wave-7-distribution`; plan commit `cdb8326`; version 1.0.0
- Environment: Alpine VM via the `alpine-vm` MCP (`vm_status`: running, SSH reachable); all container commands run as `docker compose -f /img-scroller/docker-compose.yml run --rm rust ...`; exit codes authoritative.

## 1. Dev-checks evidence (TODO sub-item 1)

Command:

```text
docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh
```

Exit code: `0`. Log file: `logs/checks/dev-checks-20261115T033421Z.log` (gitignored).

```text
PASS fmt-check exit=0 (2 s)
PASS check exit=0 (21 s)
PASS test exit=0 (30 s)
PASS clippy exit=0 (8 s)
ALL CHECKS PASSED
```

Census corroboration:

- Harness `test` section summary: `test result: ok. 195 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s` (`running 195 tests`).
- Test-bearing file count: `grep -rl '#\[test\]' src | wc -l` → exit `0`, output `28`.

Census statement: the census is pinned at **195 unit tests / 28 test-bearing files**. Task 4 adds NO tests — the only trigger would be a pure packaging/config function landing in `src/`, and none exists; Tasks 1–3 changed no file under `src/` and no Cargo manifest.

## 2. Build evidence (TODO sub-item 2)

### x64 build

Command: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh x86_64-pc-windows-gnu` → exit `0`.

```text
build-windows: building target: x86_64-pc-windows-gnu
build-windows: artifact: dist/windows-x64/img-scroller.exe
build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (dist/windows-x64): advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll
build-windows: OK
```

Non-empty artifact: `dist/windows-x64/img-scroller.exe` = 19,649,092 bytes.

### x86 build (pre-fill script state)

Command: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu` → exit `0`.

```text
build-windows: building target: i686-pc-windows-gnu
build-windows: artifact: dist/windows-i686/img-scroller.exe
build-windows: dist/windows-i686 DLL import audit: the OBSERVED list is recorded by the Wave-7 verification task (audit tool: i686-w64-mingw32-objdump)
build-windows: OK
```

Non-empty artifact: `dist/windows-i686/img-scroller.exe` = 19,802,972 bytes.

## 3. Audit evidence (D3)

### x64 re-audit (tool: `x86_64-w64-mingw32-objdump`)

Pipeline: `x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u` → exit `0`; count pipeline (`| wc -l`) → exit `0`, output `22`.

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

Verdict: set-equal to the recorded 22-DLL constant in `scripts/build-windows.sh` (`OBSERVED_DLL_IMPORT_LIST`). No delta.

### x86 first audit (tool: `i686-w64-mingw32-objdump`)

Pipeline: `i686-w64-mingw32-objdump -x dist/windows-i686/img-scroller.exe | grep -i 'DLL Name:' | sed 's/.*DLL Name: //' | tr 'A-Z' 'a-z' | sort -u` → exit `0`; count pipeline (`| wc -l`) → exit `0`, output `22`.

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

Adjudication: the observed x86 set is **set-equal** to the recorded 22-DLL constant (D3 rule (a)). No NEW name; in particular NO MinGW runtime DLL (`libgcc_s_dw2.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`) is imported — the x86 build is portable. Proceed to the script fill (D4b).

## 4. Packaging evidence (D5)

Command: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/package-windows.sh` → exit `0`.

```text
package-windows: zip: dist/ImagesScroller-1.0.0-windows-x64.zip
package-windows: zip: dist/ImagesScroller-1.0.0-windows-x86.zip
package-windows: OK
```

`unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip` → exit `0`:

```text
Archive:  /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
 19649092  11-15-2026 03:38   img-scroller.exe
      495  11-15-2026 03:38   img-scroller.toml
     3727  11-15-2026 03:38   register-default-viewer.cmd
     2747  11-15-2026 03:38   DEFAULT-VIEWER-GUIDE.txt
 --------                     -------
 19656061                     4 files
```

`unzip -l /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip` → exit `0`:

```text
Archive:  /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
 19802972  11-15-2026 03:38   img-scroller.exe
      495  11-15-2026 03:38   img-scroller.toml
     3727  11-15-2026 03:38   register-default-viewer.cmd
     2747  11-15-2026 03:38   DEFAULT-VIEWER-GUIDE.txt
 --------                     -------
 19809941                     4 files
```

Both archives carry EXACTLY the 4 flat files (no directory entries).

Packaged-CRLF evidence (both exit `0`):

```text
sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.0.0-windows-x64.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""  → exit 0
sh -c "unzip -p /img-scroller/dist/ImagesScroller-1.0.0-windows-x86.zip register-default-viewer.cmd | grep -q \"$(printf '\r')\""  → exit 0
```

Artifact byte-size record (`ls -l`, exit `0`):

| Artifact | Bytes |
| --- | --- |
| `dist/windows-x64/img-scroller.exe` | 19,649,092 |
| `dist/windows-i686/img-scroller.exe` | 19,802,972 |
| `dist/ImagesScroller-1.0.0-windows-x64.zip` | 6,711,458 |
| `dist/ImagesScroller-1.0.0-windows-x86.zip` | 7,160,311 |

The pre-existing gitignored `dist/windows-x64/img-scroller-diagnostic.exe` and its `.log` were not touched and are not packaged.

## 5. Script-fill record (D4b)

`scripts/build-windows.sh` received EXACTLY the two planned edits:

1. New constant after line 21: `OBSERVED_DLL_IMPORT_LIST_I686="advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll"` — identical to the x64 constant's value (set-equal adjudication).
2. The x86 branch of `print_audit_note_for_target` now prints the observed list instead of the Wave-7 verification-task pointer.

CR check: `sh -c "! grep -q \"$(printf '\r')\" /img-scroller/scripts/build-windows.sh"` → exit `0` (LF-only preserved); the file is 121 lines.

Confirmation re-run: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh i686-pc-windows-gnu` → exit `0`:

```text
build-windows: building target: i686-pc-windows-gnu
build-windows: artifact: dist/windows-i686/img-scroller.exe
build-windows: GNU runtime static-linking evidence — OBSERVED DLL import list (dist/windows-i686): advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll
build-windows: OK
```

Commit (verbatim message): `feat: record observed x86 dll import list` — stages ONLY `scripts/build-windows.sh`.

## 6. Pending OPERATOR-RUN gate (TODO sub-item 3)

The manual-UX check runs on the operator's real Windows 10/11 machine (agent-side GUI/runtime execution is impossible from the Linux container). The TODO's manual-UX sub-item stays `[ ]` until the user reports the run. The self-contained checklist is `.kilo/plans/20261114-wave7-task4-verification-plan.md` §8.

## 7. Task-5 carry-forward

- (a) The OBSERVED x86 DLL import list recorded above lands in `docs/build-windows.md` (the Task-5 docs step).
- (b) `docs/build-windows.md` also gains the x86/build-matrix rows for its typed-failure and invocation sections.
- (c) `docs/build.md` needs NO change (harness semantics untouched; dev-checks ran unchanged).

## 8. Verdict

VERIFIED — agent-side verification evidence complete: dev-checks `ALL CHECKS PASSED` (census 195/28), both release builds exit `0` with non-empty artifacts, both DLL audits match the recorded 22-DLL constant (x86 portable — no MinGW runtime DLL), packaging re-run produces both 4-file flat zips with CRLF helper copies, and the OBSERVED x86 list is filled into the build script. The OPERATOR-RUN manual-UX gate remains open pending the user's report.
