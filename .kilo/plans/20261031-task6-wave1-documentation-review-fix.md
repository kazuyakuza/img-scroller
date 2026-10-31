# Wave 1, Task 6 — Documentation Review Fix Plan

- Date: 2026-10-31
- Source review: commit `11d68f5` on branch `feat/wave-1-skeleton-open-image`
- Files to change: `.agent/project-info/tech.md`, `.agent/project-info/architecture.md`
- Files NOT to change: everything else (this is a doc-accuracy fix only)

## Defect

`tech.md` §WIC Decode and Open-Dialog Mechanism (Implemented) and `architecture.md` §Open-Image and Display Flow (Implemented) describe the decode chain as converting to `GUID_WICPixelFormat32bppRGBA` and then calling `CopyPixels` on the converted result.

The landed implementation in `src/platform/windows.rs` creates and initializes the format converter but then calls `CopyPixels` on the original `IWICBitmapFrameDecode` (`frame.CopyPixels(...)`). The documentation therefore overstates the conversion step and does not match the current source.

## Fix — verbatim edits

### Edit 1: `.agent/project-info/tech.md`

Replace the WIC decode-chain bullet with wording that reflects the current implementation.

oldString:

```markdown
- WIC decode chain: `CoCreateInstance(&CLSID_WICImagingFactory)` → `IWICImagingFactory::CreateDecoderFromFilename` (`GENERIC_READ`, `WICDecodeMetadataCacheOnDemand`) → `GetFrame(0)` → `GetSize` → `CreateFormatConverter` → `Initialize` to `GUID_WICPixelFormat32bppRGBA` (dither `WICBitmapDitherTypeNone`, palette `WICBitmapPaletteTypeCustom`) → `CopyPixels` into an RGBA8 byte buffer.
```

newString:

```markdown
- WIC decode chain: `CoCreateInstance(&CLSID_WICImagingFactory)` → `IWICImagingFactory::CreateDecoderFromFilename` (`GENERIC_READ`, `WICDecodeMetadataCacheOnDemand`) → `GetFrame(0)` → `GetSize` → `CreateFormatConverter` → `Initialize` to `GUID_WICPixelFormat32bppRGBA` (dither `WICBitmapDitherTypeNone`, palette `WICBitmapPaletteTypeCustom`). The current implementation then calls `CopyPixels` on the original decoded frame into an RGBA8 byte buffer; the converter is created but is not yet used for the pixel copy.
```

### Edit 2: `.agent/project-info/architecture.md`

Replace step 2 of the Open-Image and Display Flow with wording that reflects the current implementation.

oldString:

```markdown
2. Decode: `platform::decode_image` → WIC factory → decoder from filename → frame → format-convert to 32bppRGBA → `CopyPixels` (mechanism detail: [tech.md](tech.md)).
```

newString:

```markdown
2. Decode: `platform::decode_image` → WIC factory → decoder from filename → frame; a format converter to 32bppRGBA is created and initialized, and `CopyPixels` reads the frame pixels into the buffer (mechanism detail: [tech.md](tech.md)).
```

## Verification after applying

1. Re-read `src/platform/windows.rs` and confirm both doc files describe the actual call chain (converter created/initialized, then `frame.CopyPixels`).
2. Confirm line counts remain valid: `tech.md` 121 lines, `architecture.md` 118 lines (each edit replaces one line with one line).
3. Confirm all Contents anchors still resolve.
4. Run `git diff --check` and ensure it reports no whitespace errors.

## Note

This fix brings the documentation into line with the currently landed code. If a parallel code change later switches the implementation to actually copy from the converter, these doc lines should be updated again to match that code.
