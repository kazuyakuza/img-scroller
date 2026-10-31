# Wave 1 — Corrective Amendment: WIC CopyPixels must copy FROM THE FORMAT CONVERTER (code fix + re-verification + doc alignment)

- Date: 2026-10-31
- Type: CORRECTIVE AMENDMENT for Wave 1 (Task-6 code-review finding — SINGLE DISCRETE STEP of the Critical Workflow)
- Supersedes: the copy-source line of [`.kilo/plans/20261031-task3-wave1-open-image-fix3.md`](20261031-task3-wave1-open-image-fix3.md) §3/§6 item 5 (`frame.CopyPixels` → `converter.CopyPixels`) and the two `newString`s of [`.kilo/plans/20261031-task6-wave1-documentation-review-fix.md`](20261031-task6-wave1-documentation-review-fix.md). DO NOT edit either referenced plan file — supersession is recorded HERE only (deviations ledger seed, §6).
- Scope (TOP PRIORITY, hard boundary): ONLY the verified WIC CopyPixels-source defect + its re-verification + doc alignment. Nothing else reopens; NO other file/class/behavior change; NO edits to any existing `.kilo/plans/*`.
- Plan approval: AUTO-APPROVED ("Approve Global and Tasks Plans"). Save + return path; no user approval round.
- Committer executes ONLY what this file lists. NO cargo/docker on the HOST (all cargo/docker runs go through the Alpine-VM MCP per `## Build Environment` in tech.md).
- `CopyPixels` appears in EXACTLY ONE source location: `src/platform/windows.rs` line 135 (verified 2026-10-31 by content search of `src/`). The defect exists in NO other file.

## 1. Defect record (verified evidence)

- Tree state: `src/platform/windows.rs` `decoded_image` creates and initializes `IWICFormatConverter` to `GUID_WICPixelFormat32bppRGBA` (straight RGBA8) but then calls `CopyPixels` on the ORIGINAL `IWICBitmapFrameDecode` (`frame.CopyPixels(...)`), reading pixels in the frame's NATIVE format and discarding the conversion.
- Consequence: the `DecodedImage` payload contract (straight RGBA8 `pixels`, Task-3 §1.2-d + Task-4 assumption) is violated at runtime for any codec whose native format is not already 32bppRGBA-straight (e.g. 24bpp JPEG, premultiplied-BGRA sources). Container tests cannot catch this (OS-bound WIC code, decision g: untested by design); the first real runtime validation is the Task-5 operator-run item.
- WIC semantics (verified): `IWICFormatConverter` itself implements `IWICBitmapSource` in windows-rs 0.62.2; the conversion materializes at `CopyPixels` CALLED ON THE CONVERTER. Citation (re-confirmed 2026-10-31 from microsoft.github.io/windows-docs-rs, windows 0.62.2, `struct.IWICFormatConverter`): the converter publishes `Methods from Deref<Target = IWICBitmapSource>` including `pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> Result<()>` — the same 3-arg signature already verified for the frame in fix3 (V4). The `Deref` impl is additionally confirmed on the same page (`type Target = IWICBitmapSource`) and `CanInto<IWICBitmapSource>`.
- Root cause (for the record; no action): the defect entered via the Task-3 fix3 amendment stage — the remedy for S6 authored the copy call against `frame`, and the six-category micro-fix verification compared API signatures, not the conversion source object.

## 2. CODE FIX — byte-exact, single localized change

File: `src/platform/windows.rs` (ONLY file changed by the code fix; everything else in the file stays byte-identical as the tree holds it, read 2026-10-31: 150 lines).

Replace EXACTLY the three-line statement at lines 134–136:

old (current tree):

```rust
        frame
            .CopyPixels(std::ptr::null(), row_stride, &mut pixels)
            .map_err(classify_failure)?;
```

new (this amendment):

```rust
        converter
            .CopyPixels(std::ptr::null(), row_stride, &mut pixels)
            .map_err(classify_failure)?;
```

Rules:
- ONLY the receiver token `frame` becomes `converter`. `row_stride` stays the `u32` value of `width.checked_mul(4)`; the allocation stays `vec![0u8; byte_length]` (sized by `pixel_buffer_size`); `classify_failure` and every `map_err(classify_failure)` site unchanged; `std::ptr::null()` rect unchanged.
- Do NOT rename the `frame` parameter (it is still consumed by `GetSize` and `Initialize`).
- Do NOT touch imports, `decode_with_factory`, `frame_from_path`, `classify_failure`, dialog code, or any other line of `windows.rs`.
- Byte-exact final `decoded_image` (AFTER the fix; this is the required end-state function — verify by read-back):

```rust
unsafe fn decoded_image(
    factory: &IWICImagingFactory,
    frame: &IWICBitmapFrameDecode,
) -> Result<DecodedImage, ImageLoadFailure> {
    unsafe {
        let mut width = 0u32;
        let mut height = 0u32;
        frame
            .GetSize(&mut width, &mut height)
            .map_err(classify_failure)?;
        let byte_length = pixel_buffer_size(width, height).ok_or(ImageLoadFailure::Unloadable)?;
        let converter: IWICFormatConverter =
            factory.CreateFormatConverter().map_err(classify_failure)?;
        converter
            .Initialize(
                frame,
                &GUID_WICPixelFormat32bppRGBA,
                WICBitmapDitherTypeNone,
                None::<&IWICPalette>,
                0.0,
                WICBitmapPaletteTypeCustom,
            )
            .map_err(classify_failure)?;
        let mut pixels = vec![0u8; byte_length];
        let row_stride = width.checked_mul(4).ok_or(ImageLoadFailure::Unloadable)?;
        converter
            .CopyPixels(std::ptr::null(), row_stride, &mut pixels)
            .map_err(classify_failure)?;
        Ok(DecodedImage {
            width,
            height,
            pixels,
        })
    }
}
```

Read-back verification after the edit: the function above matches the tree EXACTLY; the only diff `git diff` shows for `windows.rs` is the receiver token swap in that one statement; `grep CopyPixels src/` still yields exactly one match (line ~135).

## 3. Re-verification chain (ordered; EACH step gated and evidenced)

Protocol for every step: call `alpine-vm_vm_status` first (running + SSH reachable, else STOP + user question); run via `alpine-vm_vm_run_command`; the command's exit code is the authoritative evidence; cargo stderr may be absent in captured output (absent stderr is NOT a failure signal). DF-GATE: run `df -h` via MCP (single `df`-prefixed command) and confirm free space on `/` meets the gate BEFORE the step; if short → STOP + report, never free disk yourself.

The exe currently in `dist/` was built from the DEFECTIVE code and MUST be replaced (steps d–e).

- a. **fmt (normalize)**: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo fmt` — `timeoutMs: 600000`; df-gate ≥ 1 GiB; EXPECT exit 0 with NO content change to tracked files (the §2 edit is formatted exactly like the tree's existing chain style). EBUSY-class error → retry the identical command ONCE; second failure → STOP + report. Host-side afterwards: `git diff --check` (expect clean). If `cargo fmt` DOES rewrite `src/platform/windows.rs` → STOP + report (formatting assumption violated; caller decides).
- b. **dev-checks**: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` — `timeoutMs: 1200000`; df-gate ≥ 1.5 GiB; EXPECT exit 0 with `test result: ok. 15 passed` (semantics unchanged; still 15 tests) and the final `ALL CHECKS PASSED` line. Non-zero → capture output, STOP + report.
- c. **cross-target check**: `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu` — `timeoutMs: 1800000`; df-gate ≥ 3 GiB; EXPECT exit 0 — proves the converter-copy call compiles for the GNU target. Errors in `windows.rs` → capture, STOP + report.
- d. **REBUILD the release artifact**: `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/build-windows.sh` — `timeoutMs: 3600000`; df-gate ≥ 2.5 GiB; EXPECT exit 0 with `build-windows: OK` and `build-windows: artifact: dist/windows-x64/img-scroller.exe`; the script overwrites atomically (`.part` copy → rename). Afterwards confirm host- or MCP-side (`ls -l dist/windows-x64/`): the exe is non-empty, NO `img-scroller.exe.part` remains, and RECORD the new byte size (the previous recorded size was 19,428,809 bytes; it WILL change).
- e. **RE-AUDIT the NEW exe ONCE**: `docker compose -f /img-scroller/docker-compose.yml run --rm rust x86_64-w64-mingw32-objdump -x dist/windows-x64/img-scroller.exe` — `timeoutMs: 300000`; df-gate ≥ 1 GiB. Parse the `DLL Name` lines from the captured output; deduplicate, lowercase, compare to the recorded 22-entry OBSERVED list (verbatim below):

```
advapi32.dll, api-ms-win-core-path-l1-1-0.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll, bcryptprimitives.dll, combase.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, msvcrt.dll, ntdll.dll, ole32.dll, oleaut32.dll, opengl32.dll, rpcrt4.dll, shell32.dll, shlwapi.dll, user32.dll, userenv.dll, uxtheme.dll, ws2_32.dll
```

  - IF IDENTICAL (expected — the change stays within the same WIC API surface; no new DLL dependency): NO fill change; record the comparison evidence (both lists identical) as an amendment note.
  - IF DIFFERENT: apply the Task-5 OBSERVED-DLL fill procedure, caller-authorized NOW by this plan (documented as a deviation: a second-audit/superseding event):
    1. `scripts/build-windows.sh` line 15: replace the value of the `OBSERVED_DLL_IMPORT_LIST=` quoted constant with the new deduplicated lowercase comma-space-joined list (exactly one quoted line, same format rules).
    2. `docs/build-windows.md` "Observed DLL import list" code block (line ~58–60): replace the list line with the new one (same format).
    3. `docs/build-windows.md` evidence paragraph (line ~62): update the phrase to say re-audited 2026-10-31 after the Wave-1 WIC converter-copypixels corrective amendment (same deduplicated-lowercase wording style; keep the "observed evidence — never copied from any reference" rule).
    4. These two files then join commit 2 (§4).
  - IF the objdump capture is truncated by MCP output limits: re-run with `timeoutMs: 300000` is NOT enough to fix truncation — instead rerun the single command with output redirected to a file inside the container and `grep 'DLL Name'` afterwards via separate MCP calls (e.g. `sh -c` wrapped through the compose `rust` service); never chain on the host. If still unusable → STOP + report.

## 4. DOC ALIGNMENT — byte-exact verbatim edits (supersedes the docs-review-fix newStrings)

The docs-review-fix plan has NOT been applied to the tree (verified 2026-10-31: `tech.md` line 53 and `architecture.md` line 52 still hold the original chain wording), so the reviewer's `oldString`s remain valid anchors; the correction is in the `newString`s. The reviewer plan itself anticipated this switch ("If a parallel code change later switches the implementation to actually copy from the converter, these doc lines should be updated again to match that code") — that note is the basis for THIS plan's superseding edits. One line replaced by one line in each file → line counts stay exactly `tech.md` 121 / `architecture.md` 118.

### Edit 1: `.agent/project-info/tech.md` (line 53)

oldString (unchanged from the reviewer plan):

```markdown
- WIC decode chain: `CoCreateInstance(&CLSID_WICImagingFactory)` → `IWICImagingFactory::CreateDecoderFromFilename` (`GENERIC_READ`, `WICDecodeMetadataCacheOnDemand`) → `GetFrame(0)` → `GetSize` → `CreateFormatConverter` → `Initialize` to `GUID_WICPixelFormat32bppRGBA` (dither `WICBitmapDitherTypeNone`, palette `WICBitmapPaletteTypeCustom`) → `CopyPixels` into an RGBA8 byte buffer.
```

newString (byte-exact):

```markdown
- WIC decode chain: `CoCreateInstance(&CLSID_WICImagingFactory)` → `IWICImagingFactory::CreateDecoderFromFilename` (`GENERIC_READ`, `WICDecodeMetadataCacheOnDemand`) → `GetFrame(0)` → `GetSize` → `CreateFormatConverter` → `Initialize` to `GUID_WICPixelFormat32bppRGBA` (dither `WICBitmapDitherTypeNone`, palette `WICBitmapPaletteTypeCustom`) → `CopyPixels` read from the initialized converter (the conversion to straight RGBA materializes during this copy) into an RGBA8 byte buffer.
```

### Edit 2: `.agent/project-info/architecture.md` (line 52)

oldString (unchanged from the reviewer plan):

```markdown
2. Decode: `platform::decode_image` → WIC factory → decoder from filename → frame → format-convert to 32bppRGBA → `CopyPixels` (mechanism detail: [tech.md](tech.md)).
```

newString (byte-exact):

```markdown
2. Decode: `platform::decode_image` → WIC factory → decoder from filename → frame → format converter initialized to 32bppRGBA → `CopyPixels` copies from the converter (mechanism detail: [tech.md](tech.md)).
```

NOT in scope (explicit): the historical byte-size mentions of `19,428,809` in `tech.md`/`architecture.md` describe the Task-5 artifact and stay untouched; the NEW size is recorded only in this amendment's execution evidence. If the caller later wants those refreshed, that is a separate decision.

## 5. Commits (TWO; in this order, after §3 AND §4 are complete)

Procedure per commit (both commits): read `.gitignore` first; stage ONLY the listed files (NO `git add .`); never stage `.gitignore`-matching paths (`dist/`, `logs/`, `*.log`, `target/`) or any `.md` outside the named list; `git status` clean after each commit; NO push (push = separate caller step, `origin` only); NO branch creation/deletion; NO edits to existing plan files.

1. Commit 1 — message EXACT: `Fix WIC decode to copy converted RGBA pixels from the format converter`
   - Files: `src/platform/windows.rs` — nothing else.
2. Commit 2 — message EXACT: `Align docs with the WIC converter-copied decode path`
   - Files: `.agent/project-info/tech.md`, `.agent/project-info/architecture.md` — PLUS `scripts/build-windows.sh` and `docs/build-windows.md` ONLY IF step e found a different import list (fill applied). If the list was identical, commit 2 is exactly the two doc files.

## 6. Deviations ledger seed (record here; do NOT edit fix3 or the docs-review-fix plan)

1. Converter-copy code fix: this amendment supersedes fix3 §3's copy-source line (§6 item 5 of fix3's shorthand patch map) — `frame.CopyPixels` → `converter.CopyPixels` in `decoded_image`; supersession note recorded here; fix3 file untouched.
2. Doc alignment: this plan's two newStrings supersede the docs-review-fix plan's newStrings (same oldString anchors); that plan file untouched.
3. Optional second audit / OBSERVED-DLL fill applied to BOTH files (script constant + docs list + evidence phrase), only if step e's list differs — authorized now by this corrective plan as a superseding second-audit event; documented as a deviation with the comparison evidence.

## 7. Risks and handling

| # | Risk | Handling |
| --- | --- | --- |
| R1 | Converter `CopyPixels` behaves unexpectedly on exotic codecs (runtime) | Container-untestable by design (decision g); validated by the Task-5 OPERATOR-RUN item — stays OPERATOR-PENDING; never agent-failed. |
| R2 | Import-set drift after rebuild | Step e handles it (identical → evidence only; different → authorized fill on both files). |
| R3 | Disk exhaustion during checks/build | Every step carries a df-gate; short → STOP + report before running. |
| R4 | EBUSY-class error on a compose invocation | Retry the identical command ONCE; second failure → STOP + report. |
| R5 | `cargo fmt` rewrites the §2 edit | STOP + report (the edit is fmt-clean by construction); caller decides. |
| R6 | Implementer drifts beyond scope (extra files, plan edits, premature push) | FORBIDDEN: §2 single-statement change, §4 two single-line edits, §5 two exact commits, no existing-plan edits, no push/branch ops. |

## 8. Verification checklist (delta for the review step)

1. `windows.rs` matches §2's byte-exact final `decoded_image`; `grep CopyPixels src/` → exactly one match on `converter`.
2. Step b evidence: exit 0, `test result: ok. 15 passed`, `ALL CHECKS PASSED`.
3. Step c evidence: exit 0 on `x86_64-pc-windows-gnu`.
4. Step d evidence: exit 0, `build-windows: OK`, non-empty exe, no `.part`, new size recorded.
5. Step e evidence: list comparison verdict recorded (identical, or fill applied to both files).
6. Doc read-back: tech.md/architecture.md hold the §4 newStrings; line counts 121/118; Contents anchors resolve; `git diff --check` clean.
7. Git: two commits with the exact §5 messages and exact file lists; `git status` clean; no push.

DECISION: apply the single receiver swap (`frame` → `converter` in the `CopyPixels` call of `decoded_image`), re-verify through the gated chain (fmt → dev-checks → GNU cross-check → rebuild → one re-audit), align the two doc lines with the corrected chain wording, land as the two exact commits; STOP + report on any gate/truncation/scope drift.
