# Task 2 — Front-end Technical Specification: egui/eframe App Shell

**Date:** 2026-10-31  
**Wave:** Wave 1 — Project Skeleton and Open Image  
**Scope boundary:** Task 2 only (Cargo project scaffold + empty egui/eframe window shell). Tasks 3/4 receive their own front-end specifications; this document notes their plug-in hooks but does not specify them.  
**Target implementer restriction:** Junior developer under 50% authority — no architectural or scope decisions are left to the implementer.

## 1. Application shell archetype

The application is a **single-window egui/eframe desktop GUI** for Windows. The shell stage renders an empty viewer window only; image loading, dialogs, and navigation arrive in Tasks 3/4.

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 1.1 | Single native window | The program calls `eframe::run_native` exactly once with a single `eframe::App` instance. |
| 1.2 | Window title | The viewport title is the exact string `Images Scroller` (final app name, user-decided 2026-10-29). |
| 1.3 | Regular resizable window | Window creation options set `resizable(true)`. |
| 1.4 | Not maximized at launch | Window creation options set `maximized(false)`. |
| 1.5 | Not a modal dialog | The window is created as the primary application window, not as a modal or child dialog. |
| 1.6 | Default OS placement | No explicit position or size is forced; the window manager uses its default placement behavior. |
| 1.7 | No functional shell-stage input | Keyboard and mouse events are unrestricted at launch; this task claims no bindings. |

## 2. Immediate-mode app-struct pattern

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 2.1 | `eframe::App` implementation | A struct named `ImagesScrollerApp` (or another self-documenting name) implements `eframe::App`. |
| 2.2 | State ownership | The app struct owns all mutable state; no global or `static` mutable variables are used. |
| 2.3 | Constructor | A `new` method returns the app struct, accepting only the parameters required for the empty shell (e.g., `cc: &eframe::CreationContext<'_>` if needed by eframe). |
| 2.4 | `update` responsibility | The `update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)` method performs exactly one clear background paint and nothing else. |
| 2.5 | Hook for Task 4 | The `update` method paints inside `egui::CentralPanel` so that the future image display area is the central surface. No image texture or scaling code is added in Task 2. |
| 2.6 | Hook for Task 3 | The app struct contains a field placeholder (private) for the current image path/context, initialized to `None`; Task 3 will populate it. No CLI parsing or dialog code is added in Task 2. |

## 3. Render / paint contract

| # | Requirement | Decision |
|---|-------------|----------|
| 3.1 | Background clear | **Recommended:** `egui::CentralPanel::default().show(ctx, |_ui| {});` lets egui fill the panel with the current visual style's background color. This is the standard immediate-mode surface and creates the correct anchor for Task 4's image widget. |
| 3.2 | Alternative (acceptable but not recommended) | Calling `frame.clear_color(...)` directly is allowed but must be avoided as the primary pattern because it does not establish the `CentralPanel` surface that later tasks need. |
| 3.3 | No content | No labels, buttons, images, menus, or placeholder text are rendered in Task 2. |

## 4. Cargo / binary contract (binding constraints)

| # | Requirement | Testable statement |
|---|-------------|-------------------|
| 4.1 | Package identity | `Cargo.toml` has `package.name = "img-scroller"` and `package.version = "0.1.0"`. |
| 4.2 | Binary inheritance | `Cargo.toml` contains **no** `[[bin]]` section; the binary name inherits from the package name and produces `img-scroller.exe`. |
| 4.3 | Source location | All Rust source lives under `src/`. The placeholder `src/.gitkeep` is removed once real source files exist. |
| 4.4 | Dependency addition | The GUI stack dependencies are added with `cargo add` during implementation; this spec does not name or pin crate versions. |
| 4.5 | Required capabilities | The chosen crates must provide: `eframe::App`, `eframe::run_native`, viewport/window options (`resizable`, `maximized`, `title`), and `egui::CentralPanel`. |

## 5. Static verification checklist for 4.5a

The following must be verifiable by reading the code only (no build/run):

- [ ] `Cargo.toml` exists at the repository root.
- [ ] `package.name == "img-scroller"` and `package.version == "0.1.0"`.
- [ ] No `[[bin]]` section in `Cargo.toml`.
- [ ] `src/.gitkeep` is removed.
- [ ] At least one source file exists under `src/`.
- [ ] An app struct implements `eframe::App`.
- [ ] Window options include `resizable(true)` and `maximized(false)`.
- [ ] Window title is set to the exact string `"Images Scroller"`.
- [ ] `update()` paints a clear background pass via `CentralPanel` (or equivalent standard egui surface).
- [ ] No image rendering, dialog, CLI parsing, or navigation code exists in Task 2.
- [ ] No global or `static` mutable state is used.
- [ ] No leftover placeholder comments, TODO markers, or commented-out code.

## 6. Project-rule constraints that shape the GUI code

| Rule | How it applies to Task 2 |
|------|--------------------------|
| `max-lines-per-file` (≤200 lines; ideally ≤125 substantive) | The shell code must fit in one small `src/main.rs` or a tiny `src/main.rs` + `src/app.rs`. |
| `max-lines-per-method` (≤50 lines) | `new`, `update`, and `main` must each be ≤50 lines of body. |
| `max-depth` (≤2 nested blocks) | Avoid deep nesting; extract any third-level logic into a private helper. |
| `prefer-private-members` | App struct fields and helpers are `private` by default. |
| `self-documenting-code` | Use descriptive names (`ImagesScrollerApp`, `run_app`, etc.); minimal comments. |
| `single-section-boolean-conditions` | Any multi-section condition must be extracted into a named helper returning `bool`. |
| `no-commented-code` | No commented-out code is left in source files. |

## 7. What this spec does NOT decide (deferred to Tasks 3/4 or architector 4.1b)

- Crate names/versions for egui/eframe (identified as required capabilities only).
- WIC access mechanism or binding approach.
- Native Windows open-dialog mechanism.
- CLI argument parsing.
- Unsupported-file message wording.
- Image texture pipeline, fit-scaling, zoom modes, or `TextureHandle` management.
- Internal module/file layout beyond the single shell-stage entry.
- Fullscreen keybindings or UI control placement.

## 8. Acceptance criteria

1. The project compiles in the container harness once `Cargo.toml` and source exist.
2. Running the produced `img-scroller.exe` on Windows opens a regular, resizable window titled `Images Scroller`.
3. The window is not maximized and is not a modal dialog.
4. The window background is painted (no uninitialized/black artifact region).
5. No UI controls, images, or input handlers are present in Task 2.
6. The code respects file/method size, depth, private-by-default, and self-documenting rules.
