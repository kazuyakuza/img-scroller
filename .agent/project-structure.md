# Project Structure

# Files in src/

- app.rs - eframe application shell: viewer window options, open-flow state, the empty-state centered Open button, the navigation-context gate for the floating pill (image open or browsed empty folder), four-direction arrow-key navigation and the shared navigate seam, in-session folder/sibling-folder listings plus last-viewed memory and empty-folder-browse state, window-title constant; the shared central-surface paint seam and the fullscreen/zoom/fit state routed through the child modules below
- app_display.rs - central image display: texture lifecycle wrapper (one upload per decode), fit-scaled paint, pending-upload flag
- folder_listing.rs - current-folder image listing for horizontal navigation: one non-recursive parent-folder read, 17-extension image whitelist, ±1 neighbor stepping with list-edge reports, plus first_listed_image (the vertical first-image priority probe); sorts with the shared name_collation
- image_loading.rs - open-image seam: load_image entry, DecodedImage payload shape, failure classification, unsupported-file message constant
- main.rs - entry point: CLI image-path intake, module registration, starts the eframe native app
- last_viewed_images.rs - session RAM map of the last displayed image per visited folder: created empty with the app, updated at every open-success, read for the visited-folder target priority, destroyed with the app (no persistence)
- name_collation.rs - fixed v1 alphanumeric entry-name collation shared by the image and folder listings (lowercase form first, exact-character tie-break) with its inline unit tests
- sibling_folder_listing.rs - sibling-folder listing for vertical navigation: one non-recursive read of the current folder's parent, immediate child directories only (current folder excluded; symlinked entries unlisted), shared-collation sort, ±1 folder stepping with scope-edge reports
- scaling.rs - pure fit-scaling math: largest uniform factor for the available area (no egui painting)
- zoom.rs - pure zoom math: the ×1.25 step clamped to the [1.0, 8.0] bounds, at-bound predicates, the restore-to-fit constant, and the fit-size × factor draw-size composition (with its inline unit tests)

# Folders in src/

- app/ - child modules of the application shell: arrow_direction.rs (painted-arrow control types for the floating pill), button_hint.rs (the pill-button hotkey captions + hover-tooltip strings keyed by button, plus the shared top-aligned vertical button-column wrapper), floating_controls.rs (centered floating bottom pill: message line + the 10-button row of painted vector arrows, the fullscreen square, the zoom [+][−][restore-fit] and fit-width/fit-height groups), folder_navigation.rs (shared navigate seam: the four directions' ±1 neighbor resolution on both axes, the vertical target-image priority, and empty-folder mode), empty_folder_view.rs (the empty-folder browsing message), empty_state_view.rs (the fresh empty-state centered Open button), fullscreen.rs (session fullscreen flag: F11/button toggle action sending the viewport command, the chrome visibility gate, the fullscreen surface), fullscreen_button.rs (the pill's always-enabled painted-square fullscreen button), keyboard_intake.rs (the one per-frame keyboard intake: F11, zoom I/O/0 chords, fit W/H, the four arrows), zoom.rs (zoom/fit-mode app seam: factor steps and bounds, set_fit_mode with mode-relative reset and activation messages), zoom_button.rs (the three zoom pill buttons with at-bound dimming), fit_mode_button.rs (the two fit pill buttons gated on an open image), central_surface.rs (the single ImagePainting paint seam shared by chrome and fullscreen), plus the test modules tests.rs, floating_controls_tests.rs, folder_navigation_tests.rs, folder_navigation_continuation_tests.rs, last_viewed_tests.rs, fullscreen_tests.rs, zoom_tests.rs, fit_mode_tests.rs
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
