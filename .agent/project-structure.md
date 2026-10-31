# Project Structure

# Files in src/

- app.rs - eframe application shell: viewer window options, open-flow state, top Open… button, bottom message bar, window-title constant
- image_loading.rs - open-image seam: load_image entry, DecodedImage payload shape, failure classification, unsupported-file message constant
- main.rs - entry point: CLI image-path intake, starts the eframe native app

# Folders in src/

- platform/ - OS-bound facade for the native open dialog and WIC decode: mod.rs (facade re-exports), windows.rs (Windows implementation), stub.rs (non-Windows fallback)

# Other folders

- .agent/ - agent context: project-info/, todos/, rules/workflow indexes and the structure map
- .kilo/ - Kilo Code integration: agents/, rules/, commands/ and plans/
- .opencode/ - opencode integration: agents/, commands/ and opencode.json
- docs/ - Documentation files
- scripts/ - container harness scripts: dev-checks and Windows x64 cross-build (POSIX sh)
