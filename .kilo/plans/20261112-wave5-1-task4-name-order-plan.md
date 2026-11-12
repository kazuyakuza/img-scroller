# Implementation Plan — Wave 5.1, Task 4: Navigation Name Order for Unicode/Japanese Names (Both Axes)

Branch: `fix/wave-5-1-operator-feedback` · Version: 0.5.1 · Task source: [`.agent/todos/20261112/20261112-todo-1.md`](../../.agent/todos/20261112/20261112-todo-1.md) Task 4 · Global plan: [`.kilo/plans/20261112-wave5-1-operator-feedback-global-plan.md`](20261112-wave5-1-operator-feedback-global-plan.md) Task 4 · This is the 4.1b architector output. NOT front-end; pure listing/collation logic. No 4.1a exists for this task.

Binding constraint recap (from the todo + caller): BOTH navigation axes (sibling images in a folder; sibling folders under the same parent) must order by NAME in the order the system/Unicode collation produces (the operator's expectation, e.g., the system file manager's order) for names containing Japanese characters; pure-ASCII behavior must not regress sensibly; the first-image rule and the last-viewed revisit priority read the same sorted listings; the listing modules stay container-testable.

---

## 1. Root cause — what the current code produces for Japanese-mixed names (PINNED)

Current collation: `src/name_collation.rs:8-14` — `compare_entry_names(left, right)` compares `left.to_lowercase().cmp(&right.to_lowercase())` first, then tie-breaks with the exact `left.cmp(right)` (both are Rust `str::cmp` = **Unicode scalar value (code-point) order**).

Consumers (complete, verified by grep — these are the ONLY users of `compare_entry_names`):
- `src/folder_listing.rs:5,48` — the sibling-IMAGE listing sort (horizontal axis).
- `src/sibling_folder_listing.rs:5,88` — the sibling-FOLDER listing sort (vertical axis).
- `src/sibling_folder_listing.rs:113-119` (`preceding_folder_name`) — the vertical anchor position: counts listed folders that precede the current folder, again via `compare_entry_names`.

What the operator sees for Japanese-mixed names (this is the defect evidence, cite it in the todo tick):
- `str::cmp` orders by Unicode code point: every ASCII name sorts BEFORE every Japanese name regardless of locale expectations; within Japanese, order is code-point order: hiragana (U+3040–U+309F) < katakana (U+30A0–U+30FF) < CJK ideographs (U+4E00+). Example expected by the operator (system order) vs produced (pure order) diverges as soon as a folder mixes ASCII with kana/kanji names — e.g., pure order puts `a.png` before `あ.png`, while the system collation may interleave or reorder kana/kanji relative to ASCII per the user locale's sort weights.
- The lowercase fold uses `to_lowercase()` (Unicode-aware) but the deciding comparison is still code-point order — it is NOT the locale-aware collation Windows applies (the system file manager class of ordering uses `CompareStringW/Ex` with the user locale's sort table, where hiragana/katakana/kanji weights are locale-defined, e.g., in the Japanese locale hiragana precedes katakana as a kana-type difference and kanji sort per the locale's table).
- Second latent defect in the same mechanism (fixed by this plan): the anchor position at `sibling_folder_listing.rs:94-104` counts preceding folders with the SAME comparator that sorts — any comparator change must keep both in sync or the vertical ±1 stepping resolves the wrong folder. The design below stores the active comparator in the listing so sort and anchor position always agree.

Divergence cause in one line: the pure comparator is locale-INDEPENDENT code-point ordering; the expected order is the USER-LOCALE collation Windows applies to names.

## 2. Target mechanism — DECISION (junior-encoded)

**Decision: (a) Windows collation API `CompareStringEx` through the existing cfg-gated `src/platform/` facade, resolved for the USER-default locale, with the current pure logic retained as the fallback.** Reasoning:

- Matches the brief's "leverage the operating system instead of duplicating it" and the operator's expectation ("the system's order"): `CompareStringEx` is the same collation engine class Windows file managers use for name sorting.
- NO new dependency: the `windows` crate is already a pinned dependency (`Cargo.toml:18`, `=0.62.2`); we only enable additional features of the SAME pinned version. The project's no-new-deps-unless-necessary stance and the pin policy are both respected (verified below that the needed features exist in 0.62.2).
- The alternative candidates are rejected: (b) a Rust-side Unicode collation crate (`icu_collator`/`unicase`-class) adds a NEW unpinned dependency and only approximates the OS locale tables; (c) ICU4X is a heavyweight data-carrying dependency (needs collation data bundles) — both worse than delegating to the OS. `CompareStringOrdinal` is also rejected: it compares by code point with invariant case mapping — it would reproduce the CURRENT pure behavior (all ASCII before all kana/kanji) and NOT fix the defect. `StrCmpLogicalW` (Shlwapi natural/numeric sort) is NOT chosen: the defect scope is the Japanese/mixed-script ordering, and `CompareStringEx` with the user locale is the canonical system-collation call; numeric filename sorting remains out of scope (v1 alphanumeric fixed per the brief, refined by addendum item 8 to system/Unicode order for non-ASCII names — docs task 6 adds the brief addendum, not this task).
- Container-testability: the `platform` facade precedent (`src/platform/mod.rs` cfg-gates `windows.rs` / `stub.rs`) keeps non-Windows container builds compiling and container tests running against the pure fallback.

### 2.1 Resolved-source facts (windows crate 0.62.2, container-verified — cite in the todo tick)

Verified against `/usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/windows-0.62.2` after `cargo fetch --locked` inside the compose container:

- Feature exists: `windows-0.62.2/Cargo.toml:418` → `Win32_Globalization = ["Win32"]`; `:648` → `Win32_System_SystemServices = ["Win32_System"]`.
- Binding (`src/Windows/Win32/Globalization/mod.rs:12-18`):
  `pub unsafe fn CompareStringEx<P0: Param<PCWSTR>>(lplocalename: P0, dwcmpflags: COMPARE_STRING_FLAGS, lpstring1: &[u16], lpstring2: &[u16], lpversioninformation: Option<*const NLSVERSIONINFO>, lpreserved: Option<*const c_void>, lparam: Option<LPARAM>) -> COMPARESTRING_RESULT` — the crate passes `slice.len()` as each `cchcount`, so the string slices must be passed WITHOUT a trailing null terminator.
- Result constants (`Globalization/mod.rs:7242-7244`): `CSTR_LESS_THAN = COMPARESTRING_RESULT(1)`, `CSTR_EQUAL = (2)`, `CSTR_GREATER_THAN = (3)`; any other value (0) is an error result.
- Flags (`Globalization/mod.rs:11385`): `NORM_IGNORECASE: COMPARE_STRING_FLAGS = COMPARE_STRING_FLAGS(1u32)`.
- Locale-name source (`Globalization/mod.rs:580-583`): `pub unsafe fn GetUserDefaultLocaleName(lplocalename: &mut [u16]) -> i32` — writes the user-default locale name (null-terminated) and returns the character count INCLUDING the terminator; returns ≤ 0 on failure.
- Buffer size (`src/Windows/Win32/System/SystemServices/mod.rs:2785`): `pub const LOCALE_NAME_MAX_LENGTH: u32 = 85u32`.
- `LOCALE_NAME_USER_DEFAULT` is NOT exported by windows-0.62.2 (grep across the crate: no hit) — hence `GetUserDefaultLocaleName` is the pinned locale-name source (it also yields a real failure path for the fallback).

### 2.2 Target behavior definition

- On **Windows** (real builds): entry names sort by the USER-default locale's collation, case-insensitive (`NORM_IGNORECASE` only — kana type, width, and every other dimension stay SENSITIVE, matching what the system file manager shows), with any `CompareStringEx`/locale-fetch failure falling back to the pure comparator so the total order is always defined. Tie-break: when the system comparison reports `CSTR_EQUAL`, the pure `compare_entry_names` decides (deterministic exact-character order — preserves the current fold-then-exact contract, e.g., `ALPHA` before `alpha`).
- In the **container** (non-Windows stub, and the pure fallback everywhere): the current pure logic is unchanged — all existing tests keep passing byte-identical.
- ASCII-sensible check: for common pure-ASCII names both orders agree (digits before letters; punctuation ordering for the filename-relevant characters is equivalent at primary weight); the existing ASCII container tests (`folder_listing/tests.rs:36-76`, `sibling_folder_listing/tests.rs:83-94`) test the pure fallback and remain UNCHANGED. On Windows the system collation governs; punctuation corner-case differences are acceptable and are OPERATOR-RUN verified (Task 5).

## 3. Seam design (exact shapes)

Principle: the comparator is carried as DATA so the pure listing modules stay container-testable — container tests inject synthetic comparators through the seam; no real Windows call is needed in container tests (the `cfg(windows)` code is never compiled in the container).

### 3.1 `src/name_collation.rs` (the pure fallback stays; the order param-object is added here)

Keep `compare_entry_names` byte-identical (lines 1-14). Add:

```rust
pub(crate) type EntryNameCompare = fn(&str, &str) -> Ordering;

/// The active entry-name order carried as data: `system()` resolves through
/// the platform facade (the system collation on Windows, the pure fallback
/// elsewhere); `with_comparator` lets container tests inject synthetic
/// comparators to prove the listing modules sort through the seam.
#[derive(Clone, Copy)]
pub(crate) struct EntryNameOrder {
    entry_name_compare: EntryNameCompare,
}

impl EntryNameOrder {
    pub(crate) fn system() -> Self {
        Self {
            entry_name_compare: crate::platform::compare_entry_names_system,
        }
    }
    pub(crate) fn with_comparator(entry_name_compare: EntryNameCompare) -> Self {
        Self { entry_name_compare }
    }
    pub(crate) fn compare(&self, left_name: &str, right_name: &str) -> Ordering {
        (self.entry_name_compare)(left_name, right_name)
    }
}
```

File budget: 38 → ~70 lines (≤ 200 ✓). `crate::platform` compiles on every target (the stub side always exists).

### 3.2 Platform facade additions (cfg-gated, matching the existing pattern)

`src/platform/mod.rs` — add the third re-export pair exactly like the existing two:

```rust
#[cfg(target_os = "windows")]
pub(crate) use windows::compare_entry_names_system;

#[cfg(not(target_os = "windows"))]
pub(crate) use stub::compare_entry_names_system;
```
File: 14 → ~16 lines.

`src/platform/stub.rs` — the non-Windows (container) side of the system-order seam IS the pure fallback:

```rust
pub(crate) fn compare_entry_names_system(left_name: &str, right_name: &str) -> Ordering {
    crate::name_collation::compare_entry_names(left_name, right_name)
}
```
plus `use std::cmp::Ordering;`. File: 15 → ~21 lines. This is why every existing container test keeps testing the pure logic through the same seam.

`src/platform/windows.rs` — the real Windows side (~45 lines added → ~195; under the 200-line cap). All new names come from `windows::Win32::Globalization` except the buffer constant:

```rust
use crate::name_collation::compare_entry_names;
use std::cmp::Ordering;
use windows::Win32::Globalization::{
    CompareStringEx, CSTR_EQUAL, CSTR_GREATER_THAN, CSTR_LESS_THAN, COMPARESTRING_RESULT,
    GetUserDefaultLocaleName, NORM_IGNORECASE,
};
use windows::Win32::System::SystemServices::LOCALE_NAME_MAX_LENGTH;
```

Functions (each ≤ 2 params, bodies ≤ 50 lines, max depth 2):

```rust
/// The system entry-name order (brief addendum item 8): the user-default
/// locale's collation, case-insensitive; any system-call failure falls back
/// to the pure collation so the total order stays defined.
pub(crate) fn compare_entry_names_system(left_name: &str, right_name: &str) -> Ordering {
    system_collation_order(left_name, right_name)
        .unwrap_or_else(|| compare_entry_names(left_name, right_name))
}

fn system_collation_order(left_name: &str, right_name: &str) -> Option<Ordering> {
    let locale_name = user_locale_name()?;
    let left_units: Vec<u16> = left_name.encode_utf16().collect();
    let right_units: Vec<u16> = right_name.encode_utf16().collect();
    let result = unsafe {
        CompareStringEx(
            PCWSTR(locale_name.as_ptr()),
            NORM_IGNORECASE,
            &left_units,
            &right_units,
            None,
            None,
            None,
        )
    };
    collation_result_order(result)
}

fn user_locale_name() -> Option<Vec<u16>> {
    let mut buffer = [0u16; LOCALE_NAME_MAX_LENGTH as usize];
    let written = unsafe { GetUserDefaultLocaleName(&mut buffer) };
    if written <= 0 {
        return None;
    }
    Some(buffer[..written as usize].to_vec()) // includes the terminating null
}

fn collation_result_order(result: COMPARESTRING_RESULT) -> Option<Ordering> {
    match result {
        CSTR_LESS_THAN => Some(Ordering::Less),
        CSTR_EQUAL => Some(Ordering::Equal),
        CSTR_GREATER_THAN => Some(Ordering::Greater),
        _ => None, // error result (0) -> the caller's pure fallback decides
    }
}
```

Notes the implementer must honor:
- The UTF-16 slices are passed WITHOUT a trailing null (the binding passes `len()` as the count; a terminator inside the slice would enter the comparison).
- `PCWSTR(locale_name.as_ptr())` mirrors the existing `PCWSTR(file_path.as_ptr())` pattern at `windows.rs:99` (locale buffer is null-terminated by `GetUserDefaultLocaleName`).
- `CSTR_*` match as const patterns (structurally matchable tuple struct of `i32` — derives verified at `mod.rs:7142-7144`).
- CONTINGENCY only: if after implementation `wc -l src/platform/windows.rs` exceeds 200, extract `system_collation_order`, `user_locale_name`, and `collation_result_order` into a new `src/platform/windows_collation.rs` declared as `#[cfg(target_os = "windows")] mod windows_collation;` in `platform/mod.rs` with `windows.rs` calling `super::windows_collation::system_collation_order(...)`. Do NOT pre-create the file.

`Cargo.toml` — extend the existing pinned windows dependency features (no version change; `Cargo.lock` is untouched by feature additions):

```toml
windows = { version = "=0.62.2", features = [
    "Win32_Foundation",
    "Win32_Graphics_Imaging",
    "Win32_Globalization",
    "Win32_System_Com",
    "Win32_System_SystemServices",
    "Win32_UI_Shell",
    "Win32_UI_Shell_Common",
    "Win32_UI_WindowsAndMessaging",
] }
```

### 3.3 `src/folder_listing.rs` (horizontal axis — sorts through the seam)

- Imports: replace `use crate::name_collation::compare_entry_names;` with `use crate::name_collation::EntryNameOrder;`.
- Constructors (call sites at `app.rs:86` and `folder_navigation.rs:169` keep their signatures — ZERO app-layer edits):

```rust
pub(crate) fn from_opened_image(opened_image_path: &Path) -> Option<Self> {
    Self::from_opened_image_with_order(opened_image_path, EntryNameOrder::system())
}

/// Test-visible seam: sorts with the injected order (container tests prove
/// the listing sorts through the EntryNameOrder seam).
pub(crate) fn from_opened_image_with_order(
    opened_image_path: &Path,
    entry_order: EntryNameOrder,
) -> Option<Self> {
    let folder_path = opened_image_path.parent()?;
    let resolved_folder = fs::canonicalize(folder_path).ok()?;
    let image_paths = Self::collect_folder_images(&resolved_folder, entry_order)?;
    Some(Self { image_paths })
}
```

- `collect_folder_images` gains the order: `fn collect_folder_images(resolved_folder: &Path, entry_order: EntryNameOrder) -> Option<Vec<PathBuf>>` (2 params ✓) and the sort closure at line 47-49 becomes:

```rust
image_paths.sort_by(|left_path, right_path| {
    entry_order.compare(&left_path.to_string_lossy(), &right_path.to_string_lossy())
});
```

- No stored order field is needed here (the comparator is used only during the sort; `listed_index_of`, `first_listed_image`, and neighbor stepping are order-independent).
- File budget: 123 → ~133 lines.

### 3.4 `src/sibling_folder_listing.rs` (vertical axis — sorts AND positions through the seam)

- Imports: replace `use crate::name_collation::compare_entry_names;` with `use crate::name_collation::EntryNameOrder;` (keep `use std::cmp::Ordering;`).
- The struct stores the active order (so the anchor position ALWAYS uses the same comparator that sorted):

```rust
pub(crate) struct SiblingFolderListing {
    sibling_folder_paths: Vec<PathBuf>,
    entry_order: EntryNameOrder,
}
```

- Private param object (respects the ≤2-arguments rule for the 3-fold construction input; internal-only, no new file):

```rust
struct SortScope<'a> {
    current_folder: &'a Path,
    entry_order: EntryNameOrder,
}
```

- Constructors — `from_opened_image` and the existing test reader-seam `from_opened_image_with_scope_entries` keep their signatures; the new order seam is `from_opened_image_with_order` (2 params). Body pattern for BOTH construction paths (4 shared lines duplicated across the two constructors instead of a 3-param helper):

```rust
pub(crate) fn from_opened_image(opened_image_path: &Path) -> Option<Self> {
    Self::from_opened_image_with_order(opened_image_path, EntryNameOrder::system())
}

pub(crate) fn from_opened_image_with_order(
    opened_image_path: &Path,
    entry_order: EntryNameOrder,
) -> Option<Self> {
    let resolved_current_folder = resolve_current_folder(opened_image_path)?;
    let scope_parent_path = resolved_current_folder.parent()?;
    let scope_entries = read_scope_entries(scope_parent_path).unwrap_or_default();
    let sort_scope = SortScope {
        current_folder: &resolved_current_folder,
        entry_order,
    };
    Some(Self::sorted_scope_folders(scope_entries, sort_scope))
}
```

`from_opened_image_with_scope_entries` keeps its exact existing signature/body shape but routes through `resolve_current_folder` and builds `SortScope { current_folder: &resolved_current_folder, entry_order: EntryNameOrder::system() }` before `sorted_scope_folders`.

- New private helper and refactor (extracted from `from_opened_image_with_scope_entries`):

```rust
fn resolve_current_folder(opened_image_path: &Path) -> Option<PathBuf> {
    let current_folder_path = opened_image_path.parent()?;
    fs::canonicalize(current_folder_path).ok()
}
```

- `sorted_scope_folders` takes the param object (2 params ✓):

```rust
fn sorted_scope_folders(entries: Vec<ScopeEntry>, sort_scope: SortScope<'_>) -> Self {
    let mut sibling_folder_paths: Vec<PathBuf> = entries
        .into_iter()
        .filter(|entry| entry.is_directory)
        .map(|entry| entry.path)
        .filter(|folder_path| folder_path != sort_scope.current_folder)
        .collect();
    sibling_folder_paths.sort_by(|left_path, right_path| {
        sort_scope
            .entry_order
            .compare(&left_path.to_string_lossy(), &right_path.to_string_lossy())
    });
    Self {
        sibling_folder_paths,
        entry_order: sort_scope.entry_order,
    }
}
```

- Anchor position uses the STORED order (this is the latent second consumer fixed by the seam — sort and position can never disagree again). Replace the free fn `preceding_folder_name` (lines 113-119, DELETED — no commented code) with a method:

```rust
fn position_of_current_folder(&self, anchor_image_path: &Path) -> Option<usize> {
    let current_folder_path = anchor_image_path.parent()?;
    let resolved_folder = fs::canonicalize(current_folder_path).ok()?;
    let current_name = resolved_folder.file_name()?.to_string_lossy();
    let preceding_folder_count = self
        .sibling_folder_paths
        .iter()
        .take_while(|listed_path| {
            self.listed_folder_name_precedes(listed_path.as_path(), &current_name)
        })
        .count();
    Some(preceding_folder_count)
}

fn listed_folder_name_precedes(&self, listed_path: &Path, current_name: &str) -> bool {
    let listed_name = listed_path.file_name().unwrap_or_default().to_string_lossy();
    self.entry_order.compare(&listed_name, current_name) == Ordering::Less
}
```

(Both borrows in the `take_while` closure are immutable — compiles.)

- File budget: 142 → ~165 lines.
- Existing synthetic-reader tests at `tests.rs:62-126` and `neighbor_tests.rs` keep using `from_opened_image_with_scope_entries` (system order) — unchanged.

### 3.5 Call-site verification (zero app-layer edits)

`FolderListing::from_opened_image` / `SiblingFolderListing::from_opened_image` signatures are unchanged, so all five production call sites stay untouched: `app.rs:86`, `app.rs:89`, `folder_navigation.rs:110`, `folder_navigation.rs:169`, plus the first-image rule at `folder_navigation.rs:167-171` (`first_image_path_of_folder` → `FolderListing::from_opened_image` → `first_listed_image`) and the last-viewed priority at `folder_navigation.rs:80-85` (reads `LastViewedImages` directly, order-independent). **First-image rule and last-viewed revisit priority: VERIFIED no separate change needed** — both consume the same sorted listings or recorded paths, and therefore follow the decided order automatically. Pin this verification in the todo tick.

## 4. Test plan (container-testable; harness run deferred to Task 5)

Rules of the plan: existing tests are UNCHANGED (they test the pure fallback through the stub seam and keep passing); new tests cover (i) the pure fallback's Japanese behavior pinned as the documented fallback contract, (ii) the seam dispatch (listing modules sort through `EntryNameOrder`, and the vertical anchor position follows the injected order). The `cfg(windows)` real `CompareStringEx` code is NOT container-testable by design (platform facade precedent) — its correctness is OPERATOR-RUN verified (Task 5, checklist item (c)).

Test-file splits (≤200-line cap): `folder_listing/tests.rs` is 174 lines and `sibling_folder_listing/tests.rs` is 158 — the new tests go into NEW child modules reusing the parent tests module's fixtures (established pattern: `neighbor_tests.rs:1-8` reuses `TempScope`):

1. **`src/name_collation.rs` inline tests** (+2; file ~70 → ~100 lines, no split):
   - `system_order_matches_the_pure_fallback_for_ascii_names` — `EntryNameOrder::system()` behaves like the pure comparator on assertions that hold under BOTH implementations (Windows system collation and container stub): `compare("alpha.gif", "ALPHA.jpg") == Less`, `compare("zeta.jpg", "alpha.gif") == Greater`, `compare("ALPHA", "Alpha") == Less` (Windows: `NORM_IGNORECASE` yields `CSTR_EQUAL` → the exact tie-break decides Less; container: fold-equal → exact Less). Do NOT assert system() Japanese ordering here (target-implementation-dependent); the Japanese fallback contract is pinned via injected pure comparator in the listing tests.
   - `injected_comparator_decides_the_entry_order` — `EntryNameOrder::with_comparator(...)` with a test-local reversed pure comparator: `compare("a", "b") == Greater`, `compare("b", "a") == Less`.
2. **`src/folder_listing/collation_tests.rs`** (NEW child module; `#[cfg(test)] mod collation_tests;` appended to `folder_listing.rs`; reuses `super::tests::TempFolder` — mark the fixture struct and its constructor `pub(super)` in `tests.rs`) — +2 tests, ~55 lines:
   - `listed_images_fallback_order_places_ascii_before_japanese` — fixture files `a.png`, `あ.png` (U+3042), `ア.png` (U+30A2), `漢.png` (U+6F22); build with `FolderListing::from_opened_image_with_order(..., EntryNameOrder::with_comparator(compare_entry_names))` (the pure comparator injected explicitly — target-independent); assert `listed_index_of` order `[a.png, あ.png, ア.png, 漢.png]` — pins the pure fallback's Japanese contract (all ASCII first; within Japanese code-point order hiragana < katakana < kanji) — the exact "what the operator saw" evidence, now a documented fallback behavior.
   - `listed_images_sort_through_the_injected_order_seam` — same fixture; inject the REVERSED pure comparator (`fn reversed_entry_names(left: &str, right: &str) -> Ordering { compare_entry_names(right, left) }`); assert the listing order is the exact reverse `[漢, ア, あ, a]` — proves the listing sorts THROUGH the seam, not around it.
3. **`src/sibling_folder_listing/collation_tests.rs`** (NEW child module; `#[cfg(test)] mod collation_tests;` appended to `sibling_folder_listing.rs`; reuses `super::tests::TempScope`) — +2 tests, ~60 lines:
   - `sibling_folders_sort_through_the_injected_order_seam` — TempScope with Japanese-mixed child folders `a`, `あ`, `ア`, `漢`; `SiblingFolderListing::from_opened_image_with_order(anchor, EntryNameOrder::with_comparator(reversed_entry_names))`; assert `sibling_folder_paths` equals the reversed order — the vertical listing sorts through the seam.
   - `anchor_position_follows_the_injected_order` — TempScope with child folders `aaa`, `mid` (current), `zzz`; inject the reversed comparator: sorted siblings become `[zzz, aaa]` and `mid`'s preceding count under the reversed order is 1, so `previous_neighbor_of` yields `zzz` and `next_neighbor_of` yields `aaa` (under the pure comparator they would be `aaa`/`zzz`); assert both — proves the anchor-position computation uses the SAME injected order as the sort (the drift fix).

New-test delta: **+6** (2 + 2 + 2). Container tests use real temp-dir fixtures with UTF-8 Japanese names (Linux ext4 handles them; the existing TempFolder/TempScope patterns apply). Naming: self-documenting test names; no comments beyond the established doc lines.

Census: current HEAD census is re-derived at the Task-5 discovery run (150 at Wave-5 close + the Task-2/Task-3 deltas); Task 4 contributes exactly +6. The Task-5 plan pins the final per-file census.

## 5. What NOT to change (no-touch list)

- `src/app/keyboard_intake.rs`, `src/app.rs` (navigation ports, keyboard intake — including the Task-2/Task-3 landed fixes), `src/pan.rs`, `src/app/pan.rs`, `src/app/esc_close_confirm.rs`, `src/app/esc_close_expiry_tests.rs`, `src/zoom.rs`, `src/app/zoom_alias.rs`, `src/app/pill_style.rs`, `src/app/floating_controls.rs`, `src/app/fullscreen*.rs`, `src/app/button_hint.rs` (ESC/pan/zoom/styling).
- `src/last_viewed_images.rs`, `src/app/folder_navigation.rs`, `src/app/folder_navigation*.tests` — verified NO change (§3.5).
- `Cargo.lock` (feature additions do not change it), `scripts/`, `docs/`, `CHANGELOG.md` (excluded by the todo), existing `.kilo/plans/*` (new plan files only), all existing tests (§4).
- `compare_entry_names` body itself (the pure fallback stays byte-identical; the fix routes listings through the seam, not by editing the fallback).

## 6. Git segmentation (Task 4 commits, wave precedent)

1. `docs: add wave5.1 task4 plan` — this plan file only (first commit of the 4.2 step).
2. `feat: follow the system name order for entry navigation` — `Cargo.toml` features + all `src/` changes (name_collation, platform ×3, folder_listing ×2, sibling_folder_listing ×2, the two new test modules) in ONE feature commit.
3. Review-fix commit(s) if 4.3 finds issues (`fix: wave5.1 task4 review fixes` style; max 3 review cycles, then escalate).
4. `docs: wave5.1 task4 module docs` — the module `//!` doc updates (name_collation: reword from "fixed v1 alphanumeric collation" to "pure alphanumeric fallback collation behind the EntryNameOrder seam; the system order resolves through the platform facade (brief addendum item 8)"; folder_listing + sibling_folder_listing: "sorted through the EntryNameOrder seam (system collation on Windows, pure fallback elsewhere; the folder listing's anchor position uses the stored order)"; platform files: one doc line each for `compare_entry_names_system`).
5. `task: mark wave5.1 task4 done` — 4.6 todo tick (preserve file content; the Task-5 manual-UX sub-item stays `[ ]`).

Gitignore compliance before every commit (`git status`, no `logs/`, `target/`, `dist/` staging).

## 7. Implementation step order for the 4.2 implementer

1. Commit this plan (`docs: add wave5.1 task4 plan`).
2. `Cargo.toml` feature additions (exact §3.2 block).
3. `name_collation.rs`: add `EntryNameCompare` + `EntryNameOrder` (§3.1).
4. Platform facade: `mod.rs` re-exports, `stub.rs` fallback, `windows.rs` collation functions (§3.2). Verify `wc -l` windows.rs ≤ 200 (contingency if not).
5. `folder_listing.rs`: seam constructors + sort through `entry_order` (§3.3).
6. `sibling_folder_listing.rs`: stored order + `SortScope` + anchor-position method (§3.4).
7. New test modules (§4): make the two fixture structs `pub(super)` where reused; add the two `collation_tests.rs` modules; append the two `mod` declarations.
8. Container sanity run (the implementer MAY run dev-checks once here; the AUTHORITATIVE run is Task 5): `docker compose -f /img-scroller/docker-compose.yml run --rm rust sh scripts/dev-checks.sh` via the `alpine-vm` MCP (`vm_status` first; exit code authoritative). Expected: `ALL CHECKS PASSED` with the +6 delta; `cargo check --locked` must stay lock-clean (feature additions do not touch the lock).
9. 4.3 code review + simplification, 4.4 N/A, 4.5b adherence, 4.6 mark (commits §6).

## 8. Review checklist (4.3)

- `grep -rn compare_entry_names src/` shows exactly: definition + tests in `name_collation.rs`, the stub fallback in `platform/stub.rs`, the Windows fallback in `platform/windows.rs`, and NO direct listing use (both listing modules go through `EntryNameOrder`).
- The vertical anchor position (`listed_folder_name_precedes`) and the sibling sort both read `self.entry_order` — no second comparator source exists.
- No `.gitignore`-matching file staged; no commented-out code; every new fn ≤ 50 lines / ≤ 2 params / depth ≤ 2; touched files ≤ 200 lines (`wc -l`).
- Container tests pass unchanged + 6 new; `ALL CHECKS PASSED`.

## 9. Task 5 / Task 6 handoff notes (NOT this task's work)

- Census expectation for the Task-5 verification plan: current-HEAD census + 6 (Task-4 delta), re-derived at the discovery run; DLL import audit expected UNCHANGED (set-equal 22 — `CompareStringEx`/`GetUserDefaultLocaleName` are kernel32 exports; kernel32 is already in the linked set).
- OPERATOR-RUN (Task 5, checklist item (c)): Japanese-mixed name order on BOTH axes matches the operator's expected (system file manager) order; standing regressions re-checked.
- Task 6 docs: brief.md addendum item 8 (pattern of items 5-7), README/tech/architecture/context updates to the collation revision; `.agent/project-structure.md` gains the two new test-module lines (project-structure maintenance applies at the 4.2 step's structure update, as with every code landing).
