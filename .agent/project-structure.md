# Project Structure

# Files in src/

- app.rs - eframe application shell: viewer window options, app struct, window-title constant
- main.rs - entry point: starts the eframe native app

(no folders in src/)

# Other folders

- .agent/ - agent context: project-info/, todos/, rules/workflow indexes and the structure map
- .kilo/ - Kilo Code integration: agents/, rules/, commands/ and plans/
- .opencode/ - opencode integration: agents/, commands/ and opencode.json
- docs/ - Documentation files
- scripts/ - container harness scripts: dev-checks and Windows x64 cross-build (POSIX sh)
