# Project Structure

# Files in src/

- app.rs - eframe application shell: viewer window options, open-flow state, top Open… button, Left/Right keyboard navigation and the shared navigate seam, in-session folder-listing state, window-title constant
- app_display.rs - central image display: texture lifecycle wrapper (one upload per decode), fit-scaled paint, pending-upload flag
- folder_listing.rs - current-folder image listing for horizontal navigation: one non-recursive parent-folder read, 17-extension image whitelist, lowercase-folded sort with exact-name tie-break, ±1 neighbor stepping with list-edge reports
- image_loading.rs - open-image seam: load_image entry, DecodedImage payload shape, failure classification, unsupported-file message constant
- main.rs - entry point: CLI image-path intake, module registration, starts the eframe native app
- scaling.rs - pure fit-scaling math: largest uniform factor for the available area (no egui painting)

# Folders in src/

- app/ - child modules of the application shell: rails.rs (corner navigation rails: right rail with disabled up/down folder arrows, bottom rail with image arrows + message area), tests.rs (shell/navigation unit tests), rails_tests.rs (rail button enablement tests)
- folder_listing/ - child module: tests.rs (listing/collation/neighbor unit tests)
- platform/ - OS-bound facade for the native open dialog and WIC decode: mod.rs (facade re-exports), windows.rs (Windows implementation), stub.rs (non-Windows fallback)

# Other folders

- .agent/ - agent context: project-info/, todos/, rules/workflow indexes and the structure map
- .kilo/ - Kilo Code integration: agents/, rules/, commands/ and plans/
- .opencode/ - opencode integration: agents/, commands/ and opencode.json
- docs/ - Documentation files
- scripts/ - container harness scripts: dev-checks and Windows x64 cross-build (POSIX sh)

# Other files (repository root)

- .gitattributes - line-ending policy: `*.sh text eol=lf` forces LF working-tree checkouts for the shell scripts (added after a CRLF Windows checkout broke the container dev-checks)
