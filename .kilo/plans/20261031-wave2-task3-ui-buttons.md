# Task 3 Implementation Plan — Wave 2: On-Screen Navigation UI Buttons (Approved Layout K3 — Corner Rails)

Date: 2026-10-31
Workflow step: Wave-2 Task 3 → Step 4.1b (architector)
Source TODO: [`.agent/todos/20261029/20261029-todo-4.md`](../../.agent/todos/20261029/20261029-todo-4.md) → `### Task 3: Previous/next image navigation — on-screen UI buttons`
Global plan: [`.kilo/plans/20261031-wave2-horizontal-scroll.md`](../../.kilo/plans/20261031-wave2-horizontal-scroll.md) (§Task 3, §2.2 constraints)
Front-end input: [`.kilo/plans/20261031-wave2-task3-ui-buttons-frontend-spec.md`](../../.kilo/plans/20261031-wave2-task3-ui-buttons-frontend-spec.md) (4.1a spec; §K3 is the APPROVED layout)
Predecessor: [`.kilo/plans/20261031-wave2-task2-keyboard-nav.md`](../../.kilo/plans/20261031-wave2-task2-keyboard-nav.md) (Task 2 landed: commits `d1a85d5`, `957cb17`; `navigate(NavigationDirection::Previous/Next)` is private to `src/app.rs`)
Layout approval: **USER-APPROVED K3 — Corner-level rails, question gate 2026-10-31.** This plan encodes K3 VERBATIM; zero layout judgment remains.
Status: APPROVED-AUTO (user auto-approves per-task plans; the Task-3 layout gate itself was passed explicitly above).

---

## 0. Scope Guard

Task 3 ships EXACTLY: the two corner navigation rails (right rail + bottom rail), the removal of the old conditional bottom message bar, and the `←`/`→` button wiring through the Task-2 navigation seam. NOTHING else:

- NO live `↑`/`↓` wiring (Wave 3 — the up/down buttons are rendered DISABLED and call nothing).
- NO menu contents / "Open with…" (later waves), NO zoom/fullscreen (Wave 4+), NO pan-then-navigate (Wave 5).
- NO documentation edits (Task 5 owns `README.md`/`tech.md`/`architecture.md`/`context.md` and `.agent/project-structure.md`). NO `CHANGELOG.md` (wave binding). NO edits to existing `.kilo/plans/*` and no TODO `[DONE]` marks (steps 4.6/5 own those).
- **NO cargo commands in Task 3** (Task-2 precedent, caller instruction): the host has no Rust/Cargo; container dev-checks belong to Task 4. The implementer self-verifies statically and REPORTS compile/rustfmt/clippy hazards in the step report (§8 hazard format) instead of running cargo.
- `Cargo.toml` / `Cargo.lock` need NOTHING (std + eframe/egui already present; version already 0.2.0 from step 3).

## 1. Binding decision — layout K3 (VERBATIM, from the approved 4.1a spec §K3)

The landed `ui()` renders FOUR egui surfaces, in THIS order:

1. `egui::Panel::top("open_toolbar")` — UNCHANGED (Wave-1 `Open…` button).
2. `egui::Panel::right("folder_rail")`, default size 40 points (panel-axis width), vertical layout, holding `↑` and `↓` — **always disabled in Wave 2** (Wave 3 reserved; deactivated via `ui.add_enabled(false, ...)`; they call NO handler).
3. `egui::Panel::bottom("image_rail")`, default size 40 points (panel-axis height), horizontal layout, holding `←`, `→` and the message text area — **ABSORBING and REPLACING the old conditional bottom message bar**. The old block
   ```rust
   if let Some(message) = self.message_text {
       egui::Panel::bottom("message_bar").show(ui, |ui| {
           ui.label(message);
       });
   }
   ```
   is DELETED from `src/app.rs` (the `message_bar` panel id ceases to exist).
4. `egui::CentralPanel::default()` — LAST, filling the remaining space (unchanged body).

- Panel ordering is binding: top → right → bottom → CentralPanel (spec §K3 "panel ordering must be respected").
- **Rails visibility gate**: BOTH rails are shown ONLY when an image is open (`self.opened_image_path.is_some()`, via the new private `image_is_open` helper — single-section boolean). In the empty state ONLY the top `Open…` panel is visible; no rail is ever created. (Spec §K3: "The rails are shown only when an image is open; in the empty state only the top Open… panel is visible, leaving the central area clean.")
- **Within a shown rail**: the `←` button is disabled at the first sibling, `→` disabled at the last sibling (STAY semantics — §4); the message area inside the bottom rail renders `message_text` when `Some`, an empty label otherwise. `message_text` STATE semantics are untouched (Wave-1 open-flow rule).
- Button glyphs EXACTLY: `←`, `→`, `↑`, `↓`. Every arrow button: `egui::Button::new(glyph).min_size(egui::vec2(28.0, 24.0))` (spec §3 K1/K3 "same arrow glyphs and 28×24 minimum size"; spec §1/§2 API facts).

## 2. Binding decision — button → navigation seam (one shared path)

- Left button click → EXACTLY `self.navigate(NavigationDirection::Previous)`; right button click → EXACTLY `self.navigate(NavigationDirection::Next)`. No second handler, no inline decode logic, no direct mutation of `opened_image_path`/`decoded_image`/`message_text`/`current_display` from button code. Behavior must be identical to the keys (Task-2 `navigate → neighbor_image_path → open_image → accept_open_outcome` chain untouched).
- `NavigationDirection` STAYS file-private in `src/app.rs`; `navigate` and `neighbor_image_path` STAYS private. Making them `pub`/`pub(crate)` is FORBIDDEN (Task-2 binding "Task 3's buttons live in the same file and call the same private method; Wave 3 moves it only if a real cross-module need ever appears" — the child-module choice of §3 preserves exactly that privacy).

## 3. Binding decision — file structure (architector-decided)

### 3.1 Choice: NEW child module `src/app/rails.rs` (NOT a `src/main.rs`-registered sibling)

- The rail code CANNOT be a top-level module registered in `src/main.rs`: it must call the **file-private** `NavigationDirection`, `navigate`, `neighbor_image_path` and read the **private** `message_text` field. A sibling module would force `pub(crate)` markers on them — violating Task-2's privacy binding and the prefer-private rule.
- A CHILD module of `app` resolves this with zero privacy changes: private items of `app` are visible to all its descendants (established precedent: `src/app/tests.rs` already does `use super::{ImagesScrollerApp, NavigationDirection};`).
- So: registration line **in `src/app.rs`** (not `src/main.rs`) — this is the caller-anticipated structure, with the registration point changed from `main.rs` to the parent module for the privacy reason stated. Nothing is registered in `src/main.rs`; `src/main.rs` is UNTOUCHED.
- `src/app.rs` stays small (arithmetic §3.3), so the inline option ("stay inline in src/app.rs") was REJECTED: rails code inline pushes app.rs to ~210–220 lines > 200 (max-lines-per-file rule).

### 3.2 EXACT signatures bound (all ≤ 2 non-self params)

| Item | File | Signature | Visibility |
| --- | --- | --- | --- |
| consts | `src/app/rails.rs` | `RAIL_SIZE: f32 = 40.0`, `ARROW_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(28.0, 24.0)`, `ARROW_BUTTON_SPACING: f32 = 8.0` | private |
| `arrow_button` | `src/app/rails.rs` | `fn arrow_button(glyph: &str) -> egui::Button` | private |
| `navigation_button_enabled` | `src/app/rails.rs` | `pub(super) fn navigation_button_enabled(app: &ImagesScrollerApp, direction: NavigationDirection) -> bool` | pub(super) = minimal widening, REQUIRED by the new unit test in `src/app/tests.rs` |
| `show_folder_rail` | `src/app/rails.rs` | `pub(super) fn show_folder_rail(parent_ui: &mut egui::Ui)` (1 param — needs no app state: up/down are statically disabled) | pub(super) |
| `sheet` helper | `src/app/rails.rs` | `fn folder_rail_buttons(rail_ui: &mut egui::Ui)` | private |
| `show_image_rail` | `src/app/rails.rs` | `pub(super) fn show_image_rail(app: &mut ImagesScrollerApp, parent_ui: &mut egui::Ui)` (2 params ✓) | pub(super) |
| helper | `src/app/rails.rs` | `fn image_rail_controls(app: &mut ImagesScrollerApp, rail_ui: &mut egui::Ui)` (2 params ✓) | private |
| `image_is_open` | `src/app.rs` | `fn image_is_open(&self) -> bool` | private |

### 3.3 Line-count arithmetic (verification of the ≤200 budget)

**`src/app.rs`** (current 160 lines):
- −5: delete the conditional `message_bar` block (lines 135–139).
- +4: the rail-call block replacing it in `ui()` (4 lines, §5.2).
- +5: `image_is_open` method (+2-line doc) + 1 blank separator, appended at the end of `impl ImagesScrollerApp` (after `neighbor_image_path`).
- +2: `mod rails;` + 1 blank separator, inserted directly above `#[cfg(test)] mod tests;`.
- **Total: 160 − 5 + 4 + 5 + 2 = 166 lines ≤ 200 ✓** (substantive ≈ 140 ≤ 125-line ideal is met at the guideline level).

**`src/app/rails.rs`** (new): ≈ 70 lines ≤ 200 ✓.

**`src/app/tests.rs`** (current 181 lines): + 17 lines (one new test, §6) = **198 ≤ 200** ✓ (tight — do NOT reflow existing lines; keep the new test on the plain single-line assert style so it lands under 200; if rustfmt later wraps an assert in a 4.3/Task-4 fix cycle, move the whole new test module-wise then — handle via fix cycle, never pre-emptively).

Untouched files: `src/main.rs`, `src/folder_listing.rs`, `src/folder_listing/tests.rs`, `src/app_display.rs`, `src/image_loading.rs`, `src/platform/*`, `src/scaling.rs`, `Cargo.toml`, `Cargo.lock`.

## 4. Binding decision — disabled-state rules (exact)

| Button | Disabled when | Enabled when |
| --- | --- | --- |
| `←` (bottom rail) | ALWAYS when the rail is hidden (empty state); when shown: no `previous_neighbor` (`ListBeginningReached`, no listing, unresolvable/unlisted index, empty state legacy) | `navigation_button_enabled(app, NavigationDirection::Previous)` == true, i.e. `app.neighbor_image_path(NavigationDirection::Previous).is_some()` |
| `→` (bottom rail) | Same rule mirrored | `app.neighbor_image_path(NavigationDirection::Next).is_some()` |
| `↑`, `↓` (right rail) | ALWAYS in Wave 2 (static `add_enabled(false, ...)`; Wave 3 reserved; call NO handler) | never in Wave 2 |

- Enablement is computed through the Task-2 seam ONLY (`neighbor_image_path` → Task-1 `previous_neighbor`/`next_neighbor` → `ListBeginningReached`/`ListEndReached` map to `None` = disabled). This is exactly the STAY rule: at list edges the button is greyed out; a click can only be produced while enabled, and the click path is the keys' path.
- The enable predicate is a single-section condition everywhere (`.is_some()` / `false`) — no compound booleans.
- No new state fields are added solely for button UI (spec checklist); enablement is derived per frame like the keyboard index.

## 5. EXACT code

### 5.1 `src/app.rs` — three edits only

Edit A — DELETE the message-bar block inside `ui()` (the `if let Some(message) = self.message_text { egui::Panel::bottom("message_bar")... }` block of lines 135–139 AND one of its adjacent blank lines is re-consumed by Edit B below).

Edit B — insert the rail block in its place (between the top-panel show and the CentralPanel show, keeping exactly one blank line on each side):

```rust
        if self.image_is_open() {
            rails::show_folder_rail(ui);
            rails::show_image_rail(self, ui);
        }
```

Result — the FULL final `ui()` shape (everything else byte-identical to the landed file):

```rust
impl eframe::App for ImagesScrollerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let left_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowLeft));
        let right_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowRight));
        if left_pressed {
            self.navigate(NavigationDirection::Previous);
        }
        if right_pressed {
            self.navigate(NavigationDirection::Next);
        }

        egui::Panel::top("open_toolbar").show(ui, |ui| {
            self.open_toolbar_button(ui, frame);
        });

        if self.image_is_open() {
            rails::show_folder_rail(ui);
            rails::show_image_rail(self, ui);
        }

        egui::CentralPanel::default().show(ui, |panel_ui| {
            if let Some(decoded_image) = self.decoded_image.as_ref() {
                self.current_display.paint(panel_ui, decoded_image);
            }
        });
    }
}
```

Edit C — append after `neighbor_image_path` inside `impl ImagesScrollerApp`, and add the module declaration at the file bottom:

```rust
    /// True while an image remains successfully opened (the corner rails' gate).
    fn image_is_open(&self) -> bool {
        self.opened_image_path.is_some()
    }
```

```rust
mod rails;

#[cfg(test)]
mod tests;
```

(`mod rails;` goes immediately above the existing `#[cfg(test)] mod tests;`. Do NOT touch any other line of `src/app.rs`: imports, struct, `NavigationDirection`, `empty_state`, `accept_open_outcome`, `open_*`, `navigate`, `neighbor_image_path`, `window_options` stay byte-identical.)

### 5.2 CREATE `src/app/rails.rs` — full file content

```rust
//! On-screen corner navigation rails: the right rail holds the Wave-3-reserved
//! folder arrows (disabled in Wave 2); the bottom rail holds the image arrows
//! and the open-failure message area (it replaced the Wave-1 message bar).

use eframe::egui;

use super::{ImagesScrollerApp, NavigationDirection};

const RAIL_SIZE: f32 = 40.0;
const ARROW_BUTTON_SIZE: egui::Vec2 = egui::Vec2::new(28.0, 24.0);
const ARROW_BUTTON_SPACING: f32 = 8.0;

fn arrow_button(glyph: &str) -> egui::Button {
    egui::Button::new(glyph).min_size(ARROW_BUTTON_SIZE)
}

/// A direction button is active exactly when the navigation seam resolves a
/// neighbor: no image, no listing, unresolvable index, and list edges disable.
pub(super) fn navigation_button_enabled(app: &ImagesScrollerApp, direction: NavigationDirection) -> bool {
    app.neighbor_image_path(direction).is_some()
}

/// Shows the right rail with the up/down folder arrows, disabled in Wave 2.
pub(super) fn show_folder_rail(parent_ui: &mut egui::Ui) {
    egui::Panel::right("folder_rail")
        .default_size(RAIL_SIZE)
        .show(parent_ui, |rail_ui| {
            folder_rail_buttons(rail_ui);
        });
}

fn folder_rail_buttons(rail_ui: &mut egui::Ui) {
    rail_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |rail_ui| {
        rail_ui.add_enabled(false, arrow_button("↑"));
        rail_ui.add_space(ARROW_BUTTON_SPACING);
        rail_ui.add_enabled(false, arrow_button("↓"));
    });
}

/// Shows the bottom rail with the previous/next image arrows and the message area.
pub(super) fn show_image_rail(app: &mut ImagesScrollerApp, parent_ui: &mut egui::Ui) {
    egui::Panel::bottom("image_rail")
        .default_size(RAIL_SIZE)
        .show(parent_ui, |rail_ui| {
            image_rail_controls(app, rail_ui);
        });
}

fn image_rail_controls(app: &mut ImagesScrollerApp, rail_ui: &mut egui::Ui) {
    let previous_enabled = navigation_button_enabled(app, NavigationDirection::Previous);
    let next_enabled = navigation_button_enabled(app, NavigationDirection::Next);
    let message = app.message_text;
    rail_ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Center),
        |rail_ui| {
            if rail_ui.add_enabled(previous_enabled, arrow_button("←")).clicked() {
                app.navigate(NavigationDirection::Previous);
            }
            if rail_ui.add_enabled(next_enabled, arrow_button("→")).clicked() {
                app.navigate(NavigationDirection::Next);
            }
            rail_ui.label(message.unwrap_or(""));
        },
    );
}
```

Binding notes baked into this file (do not deviate):

1. `previous_enabled`, `next_enabled`, and `message` are computed BEFORE the panel closures (Copy/`Option<&'static str>` values moved in) — this keeps the closure borrow chain trivial (the standard egui sequential-capture pattern) and avoids any second simultaneous borrow of `app`.
2. `message.unwrap_or("")` renders `message_text` when `Some` and an empty label otherwise — the caller-mandated message-area semantics; `message_text` state logic is NOT touched anywhere.
3. `↑`/`↓` are `ui.add_enabled(false, ...)` ONLY — no condition, no handler call, no `navigate`.
4. The button flags feed `add_enabled`; the CLICK arms call `app.navigate(NavigationDirection::Previous/Next)` — nothing else. The `navigate` call re-derives the neighbor (Task-2 derivation-per-event design; cheap and self-consistent).
5. Nesting audit: `image_rail_controls` body depth 0 → `with_layout` closure depth 1 → `if` blocks depth 2 ✓. `ui()` rail block depth 1 ✓. Every method ≤ 50 lines ✓. Every param count ≤ 2 ✓. Only `pub(super)` items are widened, each with a stated consumer.

## 6. Tests — decision and EXACT content

### 6.1 Decision (binded)

- The extracted enablement helper `navigation_button_enabled` is pure and container-runnable (takes `(&ImagesScrollerApp, NavigationDirection)`), so ONE new unit test lands in the existing file-based app test module — it pins the button-disable semantics at the same edges Task 2 pinned for the keys.
- NO other new tests: panel rendering, glyph shape, and click wiring are UI-frame behavior → covered by the 4.5a static verification report (spec §5 conformance checklist) + the Task-4 OPERATOR-RUN.
- **Message-bar removal does NOT threaten the 9 existing app tests** (verified by inspection): all of them assert `message_text`/`opened_image_path`/`decoded_image`/`current_folder_listing` STATE and call `navigate`/`neighbor_image_path`/`accept_open_outcome` directly — none references the `message_bar` panel, the `ui()` method, or any renderer. Their premises are untouched by Task 3.

### 6.2 EXACT new test — APPEND to `src/app/tests.rs` (after the last test, one blank line before it)

```rust
#[test]
fn arrow_button_enablement_follows_the_navigation_edges() {
    let folder = TempImageFolder::create("button_edges");
    folder.create_file("a.png");
    folder.create_file("b.png");
    folder.create_file("c.png");
    let mut app = ImagesScrollerApp::empty_state();
    assert!(!super::rails::navigation_button_enabled(&app, NavigationDirection::Previous));
    assert!(!super::rails::navigation_button_enabled(&app, NavigationDirection::Next));
    app.accept_open_outcome(folder.image_path("a.png"), Ok(sample_decoded_image()));
    assert!(!super::rails::navigation_button_enabled(&app, NavigationDirection::Previous));
    assert!(super::rails::navigation_button_enabled(&app, NavigationDirection::Next));
    app.accept_open_outcome(folder.image_path("c.png"), Ok(sample_decoded_image()));
    assert!(super::rails::navigation_button_enabled(&app, NavigationDirection::Previous));
    assert!(!super::rails::navigation_button_enabled(&app, NavigationDirection::Next));
}
```

- New imports: NONE (the test reuses `TempImageFolder`, `sample_decoded_image`, `NavigationDirection` already in scope; `super::rails::...` qualifies the pub(super) fn).
- asserts: empty state → both disabled; first sibling → `←` disabled, `→` enabled; last sibling → `←` enabled, `→` disabled (STAY-edge parity with the keys' `neighbor_image_path` tests).
- Expected project test count after Task 3: caller-reported 20/20 currently green → 21 after this task (count is authoritative only from Task-4 container dev-checks; do not reconcile mid-step).

## 7. Implementation steps (in order)

1. Verify branch: `git branch --show-current` must print `feat/wave-2-horizontal-scroll`; on mismatch → STOP, return a question to the caller.
2. CREATE `src/app/rails.rs` with EXACTLY the §5.2 content (4-space indent, ≤ 100 columns — the two long `pub(super) fn` lines fit at 100 cols; verify while typing).
3. EDIT `src/app.rs` per §5.1: delete the message-bar block, insert the rail block, append `image_is_open`, add `mod rails;`. Touch NOTHING else.
4. APPEND the §6.2 test to `src/app/tests.rs` (one blank line before `#[test]`; no import edits).
5. Self-verify statically (§8 checklist, no cargo), fix found drift by re-editing files only.
6. Commit per §7.1–7.3.

## 7.1 Terminal commands (git only — NO cargo anywhere in this step)

- `git branch --show-current` (step 1 gate).
- `git status` (before staging — gitignore compliance; also read `.gitignore` first per rule).
- `git add src/app.rs src/app/rails.rs src/app/tests.rs .kilo/plans/20261031-wave2-task3-ui-buttons.md .kilo/plans/20261031-wave2-task3-ui-buttons-frontend-spec.md` — staging list EXACT (§7.2).
- `git commit -m "feat: corner-level navigation rails with enabled/disabled button states (Wave 2 Task 3)"`.
- `git status` (after commit — verify clean except pre-existing `.agent/project-info/context.md` upkeep which must NOT be staged).

## 7.2 Staging list (EXACT — nothing else)

| Staged | Why |
| --- | --- |
| `src/app.rs` | Edit A/B/C |
| `src/app/rails.rs` | new rails module |
| `src/app/tests.rs` | the one new test |
| `.kilo/plans/20261031-wave2-task3-ui-buttons.md` | this plan (4.1b artifact) |
| `.kilo/plans/20261031-wave2-task3-ui-buttons-frontend-spec.md` | the task's untracked 4.1a spec (committed alongside — Task-1/Task-2 precedent) |

NOT staged: `.agent/project-info/context.md` (planner/architector upkeep, committed at wave closing), `dist/`, `logs/`, `target/`, any TODO file, any docs file, `Cargo.toml`/`Cargo.lock` (unchanged). Never stage gitignore-matching paths; never force-add.

## 7.3 Git rules

- ONE commit, message EXACTLY: `feat: corner-level navigation rails with enabled/disabled button states (Wave 2 Task 3)`.
- NO amend, NO force, NO push (push is restricted to workflow step 5, `main` → `origin` ONLY). Branch already exists from step 2 — never re-create.
- TODO `[DONE]` marks are NOT made here (step 4.6). `.agent/project-structure.md` stays untouched (stale for `app/tests.rs`/`rails.rs` already — planner/docs territory, Task 5).

## 8. Implementer self-verification checklist (no cargo — read/inspect only)

1. `src/app.rs` = 166 lines ±2, ≤ 200; `src/app/rails.rs` ≤ 200 (≈70); `src/app/tests.rs` ≤ 200 (≈198); all other files byte-identical to pre-edit state (`src/main.rs`, `src/folder_listing.rs`, `src/folder_listing/tests.rs`, `src/app_display.rs`, `Cargo.toml` untouched — confirm via `git diff --stat`).
2. The strings `message_bar` and `Panel::bottom("message_bar")` appear NOWHERE in `src/` (search); `image_rail` and `folder_rail` appear exactly in `rails.rs`.
3. Every method ≤ 50 lines; max nesting depth 2; every `if`/condition single-section; `navigation_button_enabled`, `show_image_rail`, `image_rail_controls` each carry ≤ 2 non-self params.
4. Privacy: `NavigationDirection`, `navigate`, `neighbor_image_path`, `message_text`, all fields, all consts, `arrow_button`, `folder_rail_buttons`, `image_rail_controls`, `image_is_open` remain PRIVATE; only the three rail surface fns are `pub(super)` (+ the enablement helper for test access). No `pub(crate)` or `pub` anywhere new.
5. Button click arms call ONLY `app.navigate(NavigationDirection::Previous)` / `app.navigate(NavigationDirection::Next)`; there is no second navigation handler and no decode/state logic outside the Task-2 seam (search for `open_image(` — only at its definition and `navigate`).
6. `↑`/`↓` use `add_enabled(false, ...)` only and no `navigate` call exists in `show_folder_rail`/`folder_rail_buttons`.
7. Panel order in `ui()`: top → `image_is_open` rail block → CentralPanel; rail ids `"folder_rail"` (right) and `"image_rail"` (bottom); `default_size(RAIL_SIZE)` present on both.
8. Keyboard-intake code at the top of `ui()` byte-identical to the landed file; `key_pressed` only.
9. No commented-out code; new comments only the short module doc + `///` docs as written in §5.2/§5.1; no `// TODO`.
10. `git status` after edits: only the 5 §7.2 files modified/created/untracked-expected; nothing gitignored present.

## 8.1 Hazard report format (attach to the step report — NO cargo allowed, so REPORT, do not fix)

The implementer reports these known compile/fmt/clippy candidate hazards for Task 4 (each is either spec-cited verified or standard-pattern): (a) `egui::Panel::{right,bottom}(...)` + `.default_size(f32)` signatures (spec §2 cites the panel source — verify in container); (b) `egui::Vec2::new` in a `const` initializer (const fn in emath — verify); (c) the nested-closure `&mut app` capture in `show_image_rail → image_rail_controls` (standard egui/eframe sequential pattern — verify); (d) unused `Response` values from the `↑`/`↓` `add_enabled(false, ...)` statements (egui Response is not must-use — verify clippy); (e) rustfmt wrapping of the two ~100-col `pub(super) fn` signature lines and the §6.2 assert lines. Any OTHER suspected hazard found during the edit goes into the report the same way. If `cargo fmt --check` fails in Task 4, remediation is a 4.3/Task-4 `cargo fmt` run — never a manual reflow here.

## 9. Risk notes

- **[Flagged to caller/Task 4 — deterministic K3 consequence] Empty-state failure message visibility.** Under K3 as approved, BOTH rails render only when `opened_image_path.is_some()`. A failed open from the EMPTY state sets `message_text` but no rail exists → the unsupported-file message has NO visible surface in that situation (Wave-1 showed it in the bottom bar). This is NOT a bug in the encoding — it is the literal approved layout ("empty state shows only the top Open… panel") — and `message_text` state semantics remain untouched. Task-4 OPERATOR-RUN item must include this case; if the user deems it unacceptable, the fix is a Planner-level layout decision (e.g. rail also gated on message presence — a two-section condition would then need an extracted named helper), NOT an implementer improvisation.
- Borrow-safety (§8-1a/c) and const-context items are listed as hazards; they are the standard patterns and spec-cited APIs, not inventions.
- Test-count parity is resolved authoritatively in Task 4 (dev-checks), not here.
- The 100-column/rustfmt risk is bounded by §8-1e.

## 10. Out of scope (explicit)

- `↑`/`↓` live wiring, sibling-folder navigation, last-viewed memory (Wave 3 — `NavigationDirection` grows/is superseded only there).
- Menu contents / "Open with…" (later waves; NO menu placeholder exists in K3 — K3 defines no menu control, and none is invented).
- Zoom, fit modes, fullscreen (Wave 4+); pan-then-navigate and drag/wheel interactions (Wave 5).
- Window-title changes, tooltips, accessibility announcements (deferred; K3 defines none).
- All documentation files incl. `.agent/project-structure.md`, `CHANGELOG.md`, existing plan files, TODO files.
- Cargo/fmt/clippy/test runs (Task 4).

## 11. Verification mapping (TODO Task-3 sub-items → plan elements)

| TODO Task-3 sub-item | Covered by |
| --- | --- |
| Left and right UI buttons perform the same navigation as the keys | §2 + §5.2 click arms (`navigate` only); §6.2 test pins enablement; TASK-4 dev-checks/OPERATOR-RUN |
| UI layout proposed, approved, NOT invented silently | §1 = K3 VERBATIM from the approved 4.1a spec, user approval dated 2026-10-31; §10 forbids invention |

4.1a-spec §5 conformance checklist: every item is certifiable from this plan — panels/order exact (§1, §5.1), click→exact seam (§2, §5.2), no separate handler/direct mutation (§2, §5.2), `add_enabled` + `neighbor_image_path` disable rule (§4), up/down disabled + handler-free (§5.2-3), no menu in K3 (§10), texture path untouched (§0 — no display change), no new state fields (§4), rule compliance (§3.2, §8).

---

## Addendum — Decisions Summary for the caller (briefing later steps)

1. LAYOUT: user-approved K3 encoded verbatim — `Panel::right("folder_rail")` (40pt, vertical, `↑`/`↓` statically disabled, Wave 3 reserved) + `Panel::bottom("image_rail")` (40pt, horizontal, `←`/`→` + message area) shown ONLY while `opened_image_path.is_some()`; old conditional `message_bar` panel DELETED; order top → right → bottom → CentralPanel; glyphs `← → ↑ ↓` at `min_size(vec2(28.0, 24.0))`.
2. MESSAGE BAR ABSORBED: `message_text` state untouched; bottom-rail message area renders `Some` → label, `None` → empty label (`unwrap_or("")`). FLAGGED: a failed open from the EMPTY state now has no visible message surface (literal K3 consequence) — recorded in §9 for Task-4 OPERATOR-RUN.
3. FILE STRUCTURE: NEW child module `src/app/rails.rs` (`mod rails;` declared in `src/app.rs`, NOT registered in `src/main.rs`) so the file-private `NavigationDirection`/`navigate`/`neighbor_image_path`/`message_text` stay private to `app`'s descendants (prefer-private + Task-2 binding preserved). Arithmetic: app.rs 160 − 5 + 4 + 5 + 2 = 166 ≤ 200; rails.rs ≈ 70; app/tests.rs 198.
4. ENABLEMENT HELPER: `pub(super) fn navigation_button_enabled(app: &ImagesScrollerApp, direction: NavigationDirection) -> bool { app.neighbor_image_path(direction).is_some() }` — STAY-edge parity with the keys; two params; single-section predicate.
5. TESTS: exactly ONE new container-runnable test, `arrow_button_enablement_follows_the_navigation_edges` in `src/app/tests.rs` (empty state + first-sibling + last-sibling enablement); everything else UI-bound → 4.5a static checklist + Task-4 OPERATOR-RUN; the 9 existing app tests are structurally unaffected (state-asserting only).
6. GIT: one commit `feat: corner-level navigation rails with enabled/disabled button states (Wave 2 Task 3)` staging exactly `src/app.rs`, `src/app/rails.rs`, `src/app/tests.rs`, this plan, the 4.1a spec; no cargo, no push, no TODO/docs edits; hazards reported per §8.1.
