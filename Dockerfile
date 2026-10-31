# Images Scroller — container build harness (dev checks + Windows x64 cross-build).
# Pinned official Rust base image; a movable tag (latest/1) is deliberately not used.
FROM rust:1.99.1-bookworm

# Lint/format components on the default stable toolchain shipped in the base image.
RUN rustup component add rustfmt clippy

# MinGW-w64 x86-64 GNU cross toolchain:
# x86_64-w64-mingw32-gcc is the linker backend for x86_64-pc-windows-gnu;
# x86_64-w64-mingw32-objdump (via the package's binutils dependency) backs the
# Windows import audit performed during Wave 1 verification (Task 5).
RUN apt-get update && apt-get install -y --no-install-recommends gcc-mingw-w64-x86-64 && rm -rf /var/lib/apt/lists/*

# Windows x64 cross-compilation target on the same default toolchain.
RUN rustup target add x86_64-pc-windows-gnu
