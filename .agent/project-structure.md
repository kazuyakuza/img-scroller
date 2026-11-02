# Project Structure

# Files in src/

- app.rs - eframe application shell: viewer window options, open-flow state, the empty-state centered Open button, the image-open gate for the floating pill, Left/Right keyboard navigation and the shared navigate seam, in-session folder-listing state, window-title constant
- app_display.rs - central image display: texture lifecycle wrapper (one upload per decode), fit-scaled paint, pending-upload flag
- folder_listing.rs - current-folder image listing for horizontal navigation: one non-recursive parent-folder read, 17-extension image whitelist, ±1 neighbor stepping with list-edge reports, plus first_listed_image (the vertical first-image priority probe); sorts with the shared name_collation
- image_loading.rs - open-image seam: load_image entry, DecodedImage payload shape, failure classification, unsupported-file message constant
- main.rs - entry point: CLI image-path intake, module registration, starts the eframe native app
- last_viewed_images.rs - session RAM map of the last displayed image per visited folder: created empty with the app, updated at every open-success, read for the visited-folder target priority, destroyed with the app (no persistence)
- name_collation.rs - fixed v1 alphanumeric entry-name collation shared by the image and folder listings (lowercase form first, exact-character tie-break) with its inline unit tests
- sibling_folder_listing.rs - sibling-folder listing for vertical navigation: one non-recursive read of the current folder's parent, immediate child directories only (current folder excluded; symlinked entries unlisted), shared-collation sort, ±1 folder stepping with scope-edge reports
- scaling.rs - pure fit-scaling math: largest uniform factor for the available area (no egui painting)

# Folders in src/

- app/ - child modules of the application shell: arrow_direction.rs (painted-arrow control types for the floating pill), floating_controls.rs (centered floating bottom pill: message line + painted vector-triangle arrow buttons), folder_navigation.rs (shared navigate seam: the four directions' ±1 neighbor resolution on both axes, the vertical target-image priority, and empty-folder mode), empty_folder_view.rs (the empty-folder browsing message), tests.rs (shell/navigation unit tests), floating_controls_tests.rs, folder_navigation_tests.rs, folder_navigation_continuation_tests.rs, last_viewed_tests.rs (pill enablement and vertical navigation / last-viewed / empty-folder unit tests)
- folder_listing/ - child module: tests.rs (listing/collation/neighbor unit tests)
- sibling_folder_listing/ - child modules: tests.rs (listing/collation/self-exclusion/non-recursive/empty-scope unit tests), neighbor_tests.rs (neighbor-stepping unit tests + the symlink policy tests, reusing the sibling tests module's TempScope fixture)
- platform/ - OS-bound facade for the native open dialog and WIC decode: mod.rs (facade re-exports), windows.rs (Windows implementation), stub.rs (non-Windows fallback)

# Other folders

- .agent/ - agent context: project-info/, todos/, rules/workflow indexes and the structure map
- .kilo/ - Kilo Code integration: agents/, rules/, commands/ and plans/
- .opencode/ - opencode integration: agents/, commands/ and opencode.json
- docs/ - Documentation files
- scripts/ - container harness scripts: dev-checks and Windows x64 cross-build (POSIX sh)

# Other files (repository root)

- .gitattributes - line-ending policy: `*.sh text eol=lf` forces LF working-tree checkouts for the shell scripts (added after a CRLF Windows checkout broke the container dev-checks)
