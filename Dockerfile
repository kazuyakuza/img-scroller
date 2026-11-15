# Images Scroller — container build harness (dev checks + Windows x64/x86 cross-builds).
# Pinned official Rust base image; a movable tag (latest/1) is deliberately not used.
FROM rust:1.99.1-bookworm

# Lint/format components on the default stable toolchain shipped in the base image.
RUN rustup component add rustfmt clippy

# MinGW-w64 x86-64 GNU cross toolchain:
# x86_64-w64-mingw32-gcc is the linker backend for x86_64-pc-windows-gnu;
# x86_64-w64-mingw32-objdump (via the package's binutils dependency) backs the
# Windows import audit performed during Wave 1 verification (Task 5).
# MinGW-w64 i686 GNU cross toolchain (Wave 7):
# i686-w64-mingw32-gcc is the linker backend for i686-pc-windows-gnu;
# i686-w64-mingw32-objdump backs the x86 artifact's import audit.
# zip + unzip back the Wave-7 portable packaging script and its zip listing checks.
RUN apt-get update && apt-get install -y --no-install-recommends gcc-mingw-w64-x86-64 gcc-mingw-w64-i686 zip unzip && rm -rf /var/lib/apt/lists/*

# Windows x64 cross-compilation target on the same default toolchain.
RUN rustup target add x86_64-pc-windows-gnu

# Windows x86 cross-compilation target on the same default toolchain (Wave 7).
RUN rustup target add i686-pc-windows-gnu
