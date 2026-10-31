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
