# Amendment 4 — Task 3 Implementation Plan: `frame_from_path` error-type fix + edition-2024 `unsafe_op_in_unsafe_fn` hygiene

- Date: 2026-10-31
- Amends: [`fix3.md`](20261031-task3-wave1-open-image-fix3.md) **§3 ONLY** (internal type inconsistency of the fix3 draft: `frame_from_path` signature vs. its own body), plus edition-2024 `unsafe_op_in_unsafe_fn` hygiene in the same file. Not windows-rs drift; NOT a re-open of any other target, file, or plan section.
- Workflow step: Critical Workflow 4.1b MINI AMENDMENT #4 (Task 3), resume 4.2 at the round-3 S6 re-check.
- Plan approval: AUTO-APPROVED ("Approve Global and Tasks Plans", 2026-10-30). Save + return path; no user approval round.
- Committer (4.2) resumes ONLY at this amendment's resume instructions (§4). No edits to existing `.kilo/plans/*`. No unrelated file changes.

## 1. Verified round-3 evidence (implementer capture 2026-10-31, exit 101)

`docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu`: **exit 101 — 2 errors + 8 warnings, ALL in `src/platform/windows.rs`**; all other targets clean; fix3 §3 content confirmed byte-landed (tree read 144 lines, 2026-10-31).

| # | Finding | Location (current tree) | Root cause |
| --- | --- | --- | --- |
| E1 | E0277: `?` yields `windows_core::Error`, expected `ImageLoadFailure` | L84 `decode_with_factory`: `let frame = frame_from_path(&factory, path)?;` | fix3 §3 declared `frame_from_path -> Result<IWICBitmapFrameDecode, Error>` while its own body computes `ImageLoadFailure` via `.map_err(classify_failure)?` — internal inconsistency of the fix3 draft |
| E2 | E0277: same root | L101 `frame_from_path`: body ends `decoder.GetFrame(0)` (Result<_, Error>) while the fn must return `ImageLoadFailure`-classified results | same as E1 (body already `.map_err(classify_failure)?` → `ImageLoadFailure` inside an `Error`-returning signature) |
| W1 | 8× warning E0133 — `unsafe` call inside `unsafe fn` body without inner `unsafe {}` | `selected_dialog_path` (GetResult, GetDisplayName, PWSTR::to_string, CoTaskMemFree), `frame_from_path` (CreateDecoderFromFilename, GetSize → CreateDecoderFromFilename/GetFrame), `decoded_image` (GetSize, CreateFormatConverter, Initialize, CopyPixels) | edition-2024 `unsafe_op_in_unsafe_fn` lint — WARN-ONLY for `cargo check`, but the tree must land edition-2024-clean |

## 2. Sanctioned fix — TWO items, ONE file (`src/platform/windows.rs`)

1. `frame_from_path` returns `Result<IWICBitmapFrameDecode, ImageLoadFailure>`; the `GetFrame(0)` tail gains `.map_err(classify_failure)`. The existing `.map_err(classify_failure)?` on `CreateDecoderFromFilename` stays. Ripple: `decode_with_factory`'s `frame_from_path(&factory, path)?` now propagates `ImageLoadFailure` correctly; its other `map_err(classify_failure)` sites and its signature are UNCHANGED; no import changes.
2. Edition-2024 hygiene: the bodies of ALL THREE `unsafe fn`s (`selected_dialog_path`, `frame_from_path`, `decoded_image`) are wrapped in an inner `unsafe { … }` block (body indented +4). Every unsafe call site is now explicitly inside an `unsafe {}` → all 8 E0133 silenced, ZERO behavior change. `classify_failure` (safe fn) is UNCHANGED.

## 3. EXACT final content — the three changed functions ONLY

Replace the three functions in the tree exactly as below (LF endings, trailing newline, rustfmt-compatible layout; body indented one level inside the inner `unsafe` block). The REST of the file stands AS THE TREE HOLDS IT (fix3 §3, verified 2026-10-31): module doc comment, all `use` blocks, `show_open_image_dialog`, `decode_image`, `ensure_com_initialized`, `owner_window`, `run_open_dialog`, `decode_with_factory`, `classify_failure` — byte-unchanged.

```rust
unsafe fn selected_dialog_path(dialog: IFileOpenDialog) -> Result<PathBuf, Error> {
    unsafe {
        let item: IShellItem = dialog.GetResult()?;
        let wide_path = item.GetDisplayName(SIGDN_FILESYSPATH)?;
        let file_name = wide_path
            .to_string()
            .map_err(|_| Error::from_hresult(windows::Win32::Foundation::E_FAIL))?;
        CoTaskMemFree(Some(wide_path.as_ptr().cast::<core::ffi::c_void>()));
        Ok(PathBuf::from(file_name))
    }
}

unsafe fn frame_from_path(
    factory: &IWICImagingFactory,
    path: &Path,
) -> Result<IWICBitmapFrameDecode, ImageLoadFailure> {
    unsafe {
        let file_path: Vec<u16> = path.as_os_str().encode_wide().chain([0]).collect();
        let decoder: IWICBitmapDecoder = factory
            .CreateDecoderFromFilename(
                PCWSTR(file_path.as_ptr()),
                None,
                GENERIC_READ,
                WICDecodeMetadataCacheOnDemand,
            )
            .map_err(classify_failure)?;
        decoder.GetFrame(0).map_err(classify_failure)
    }
}

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
        frame
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

## 3a. Unchanged-semantics confirmation (verify during 4.3/4.5b)

| Item | Status |
| --- | --- |
| Pipeline order (`CoCreateInstance` → `frame_from_path` → `decoded_image`) | UNTOUCHED |
| Failure mapping (`WINCODEC_ERR_COMPONENTNOTFOUND → Unsupported`, else `Unloadable` — `classify_failure` by value) | UNTOUCHED |
| `DecodedImage` payload (straight RGBA8 via `GUID_WICPixelFormat32bppRGBA`; NO byte-swap) | UNTOUCHED |
| Dialog + COM patterns (no `SetFileTypes`; same FOS options; null-owner fallback; `S_OK | S_FALSE | RPC_E_CHANGED_MODE`; `CoTaskMemFree` after materialization) | UNTOUCHED |
| Facade signatures (`show_open_image_dialog`, `decode_image`) and every other file (`Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/app.rs`, `src/image_loading.rs`, `src/platform/mod.rs`, `src/platform/stub.rs`, `.agent/project-structure.md`) | UNTOUCHED |
| Risk G1-style deviation (rewrite beyond the two items) | FORBIDDEN — deviation = amend/review failure |

## 4. Resume instructions for 4.2 (single verification round, then commit)

1. Replace the three functions in `src/platform/windows.rs` (host file tools) with §3 byte-exact; read the file back and verify: `frame_from_path` returns `ImageLoadFailure` with the `.map_err(classify_failure)` tail; all three `unsafe fn` bodies wrapped in an inner `unsafe { … }`; everything else byte-identical to the tree. ONLY `windows.rs` changes.
2. DF-GATE: `alpine-vm_vm_status`, then `df -h` via `alpine-vm_vm_run_command` (single command, prefix `df`); if `/` available < 3 GiB → STOP + report; do NOT free disk space yourself.
3. ONE re-run (`timeoutMs: 1800000`; EBUSY → identical retry ONCE): `docker compose -f /img-scroller/docker-compose.yml run --rm rust cargo check --locked --target x86_64-pc-windows-gnu`. Expected: exit 0, zero errors AND zero warnings. **ANY residual `windows.rs` failure (or failure anywhere else) = HARD ESCALATE to the caller with the FULL captured output — no more amendment rounds.**
4. On exit 0 → base-plan S7 commit VERBATIM (gitignore-compliance procedure: read `.gitignore` before staging; NO `git add .`; NO staging of `.gitignore`-matching paths (logs/target/dist); NO context.md/TODO/plan-file staging). EXACT 9-file list: `Cargo.toml Cargo.lock src/main.rs src/app.rs src/image_loading.rs src/platform/mod.rs src/platform/windows.rs src/platform/stub.rs .agent/project-structure.md`. Message EXACT: `Open images via CLI argument or native dialog with WIC decode`. `git status` clean after. NO push, NO branch ops.
5. → base-plan S8 summary VERBATIM, plus record: amendment 4 applied (`frame_from_path` error type + edition-2024 unsafe hygiene); fallback history = base draft → S5 sanctioned fixes → fix1 (target-scoped manifest) → fix2 (`mod windows;`/`Panel` call sites) → fix3 (six-category compiler patch, partial) → fix4 (final restoration round); four amendments total; NO further plan-file edits; operator-run UX stays OPERATOR-PENDING (Task 5).

DECISION: change ONLY the `frame_from_path` error type/tail and wrap the three `unsafe fn` bodies in inner `unsafe {}` blocks; single S6 re-run under the 3 GiB df-gate; HARD ESCALATE on any residual failure; proceed to the unchanged S7 commit + S8 summary on exit 0.
