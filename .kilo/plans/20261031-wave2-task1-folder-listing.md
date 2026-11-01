# Task 1 Implementation Plan — Wave 2: Current-Folder Image Listing

Date: 2026-10-31
Workflow step: Wave-2 Task 1 → Step 4.1b (architector)
Source TODO: [`.agent/todos/20261029/20261029-todo-4.md`](../../.agent/todos/20261029/20261029-todo-4.md) → `### Task 1: Current-folder image listing`
Global plan: [`.kilo/plans/20261031-wave2-horizontal-scroll.md`](../../.kilo/plans/20261031-wave2-horizontal-scroll.md) (§Task 1, §2.2 constraints, §2.3 decision inventory)
Status: APPROVED-AUTO (user auto-approves per-task plans, decision dated 2026-10-31)

---

## 0. Scope Guard

Task 1 ships EXACTLY: one new module `src/folder_listing.rs` + its unit tests + a one-line module registration in `src/main.rs`. NOTHING else:

- NO keyboard wiring, NO UI buttons, NO app-state changes (Tasks 2/3 own those).
- NO cargo runs in Task 1 (the host has no Rust/Cargo; checks run later in Task 4 via the Alpine-VM container harness per `docs/build.md`). Task 1 code must be written so `cargo fmt --check` accepts it — match the formatting style of the existing `src/*.rs` files exactly (4-space indent, rustfmt-ish line breaks, ≤ 100 columns).
- NO documentation file edits (Task 5 owns docs). NO `CHANGELOG.md` touches (wave binding). NO edits to existing `.kilo/plans/*` files (new plan files only).
- NO TODO-file `[DONE]` marks in this step (workflow step 4.6 handles them).

## 1. Decision (a): Image-file detection filter — FIXED EXTENSION WHITELIST

### 1.1 Options analyzed

| Option | Analysis |
| --- | --- |
| (1) Fixed extension whitelist | Pure logic on file names; runs and is unit-testable on Linux in the container (global-plan constraint §2.2-9: container-runnable tests). Deterministic; no COM; no WIC dependency in this module; keeps the Wave-1 pure-logic module style (`scaling.rs` exemplar). Weakness: a codec the user installed for an exotic suffix is not reflected (that suffix is not offered in navigation). |
| (2) WIC codec-availability probe | Requires the `windows` crate + COM behind the `platform` facade. Windows-only execution — the Linux container CANNOT run it, so Task 4 unit tests could only cover a stub, leaving the real detection logic UNTESTED agent-side. Also couples the listing module to OS-bound code, breaking the pure-module pattern. |

**PICK: Option (1) — fixed extension whitelist.** Decisive reasons:

1. Container unit tests must run on Linux; option (2) is untestable there.
2. Over-listing degrades gracefully: the Wave-1 open-flow already shows the unsupported-file message when a listed-but-undecodable file is navigated to; under-listing silently breaks navigation continuity (a sibling disappears from prev/next).
3. The brief's scope is "any image whose format is registered as a codec in the Windows system, e.g. jpg, png, bmp, gif, webp, heic" — the whitelist covers every WIC built-in codec plus near-universal vendor codecs.

### 1.2 EXACT final extension set (17 entries, lowercase canonical form)

`avif, bmp, dib, gif, heic, heif, ico, jfif, jpe, jpeg, jpg, jxr, png, tif, tiff, webp, wdp`

Per-entry justification: `jpg/jpeg/jpe/jfif` — Windows JPEG codec aliases; `png` — brief example; `bmp` + `dib` — WIC built-in BMP codec and its registered alias; `gif` — brief example; `tif/tiff` — TIFF codec aliases; `wdp/jxr` — HD Photo / JPEG XR (WIC built-in); `ico` — WIC built-in icon codec; `webp` — brief example (universal vendor codec); `heic/heif` — brief's HEIC-system-codec example; `avif` — Windows 10/11 AVIF image decode via the AV1 extension. Deliberately NOT included: camera-raw suffixes (`cr2/nef/arw/dng/…`) — decode support varies by installed codec pack; excluded to keep v1 navigation to common consumer formats (an unlisted sibling does not break the viewer — the user can still open that file directly).

### 1.3 Exact matching semantics

- Match on the file NAME suffix: `Path::extension()` of the entry's file name, compared ASCII-case-insensitively via `OsStr::eq_ignore_ascii_case` against the whitelist (the TODO's requirement "extension matching must be case-insensitive on filename suffix" is fulfilled exactly here).
- Files without an extension are NEVER listed (a literal file named `.png` has no extension by `Path::extension()` semantics and is excluded; `photo.png.` likewise resolves to a non-whitelisted empty extension).
- Hidden-attribute files are NOT filtered: every file with a listed extension is listed regardless of hidden status (`read_dir` exposes hidden files on both Windows and Linux; a Linux dotfile like `.concealed.png` matches through its `png` extension — the same behavior class as a hidden file on Windows).

## 2. Decision (b): v1 alphanumeric collation — CASE-INSENSITIVE FOLD + EXACT-NAME TIE-BREAK

### 2.1 Options analyzed (the two inventoried candidates from global plan §Task 1)

| Option | Behavior on `{"Zebra.png", "apple.png", "beta.jpg"}` |
| --- | --- |
| (1) Case-sensitive lexicographic (raw UTF-8 byte / Unicode-codepoint order, "RuneToInt"-style) | `Zebra.png` FIRST ('Z'=90 < 'a'=97): every uppercase letter sorts before every lowercase letter, so the order is scattered across the whole list — unlike any folder-viewer convention. |
| (2) Case-insensitive fold with exact-name tie-break | Folded order: `apple.png`, `beta.jpg`, `Zebra.png` — case variants land adjacent; total and deterministic. |

**PICK: Option (2) — case-insensitive fold, exact-name tie-break.** Fixed for v1, no settings UI. Reasons: (i) the brief's "alphanumeric" must read naturally to a gallery user — raw byte order makes `Zebra` precede `apple`; (ii) the deferred-decision inventory contains exactly these two candidates — a numeric-aware "natural"/Explorer order is NOT an inventoried candidate and is therefore NOT implemented anywhere in v1 (if ever wanted, it belongs to the future sort-settings wave); (iii) the tie-break makes the operator total and deterministic for folded-equal names (coexistence is only possible on case-sensitive filesystems such as the container's, but the operator is defined for all pairs regardless).

### 2.2 EXACT collation operator (final semantics, binding)

For two file names `left_name: &str`, `right_name: &str` (the FULL file name INCLUDING dot+extension):

1. PRIMARY KEY: compare `left_name.to_lowercase()` vs `right_name.to_lowercase()` lexicographically — Rust `str::cmp` compares UTF-8 bytes; for valid UTF-8 (always, on both OS sides) byte order equals Unicode codepoint order. `str::to_lowercase` (full Unicode folding) is the case normalization (a few characters fold into multi-char sequences — the result is still a total, deterministic order).
2. TIE-BREAK: when the folded comparison is `Ordering::Equal`, compare the ORIGINAL unfolded names the same way (`left_name.cmp(right_name)` — UTF-8 byte / codepoint order; `A`(65) < `a`(97), so `ALPHA.jpg` < `Alpha.jpg` < `alpha.jpg`).
3. The extension is an ordinary part of the compared name (no separate extension key): folded `beta.jpg` < `beta.png` ⟹ `beta.jpg` < `Beta.png`; folded `alpha.gif` < `alpha.jpg`.
4. NO numeric-aware chunking: folded `img10.png` sorts BEFORE `img2.png` because `'1'` < `'2'` — the fixed v1 order, recorded here so operator-runs validate against it.
5. Sorting runs over the file name only, never over the folder path (single-folder listing).

This one order is the v1 navigation order for horizontal sibling images; the later folder-navigation waves reuse the same definition verbatim for folder names and the "first image" rule (noted for later waves, not implemented here).

## 3. Decision (c): module design (exact API)

New file: **`src/folder_listing.rs`** (name EXACTLY this). Constraints encoded: file ≤ 200 lines; every method ≤ 50 lines; max nesting depth 2; members private by default — `pub(crate)` only where Tasks 2/3 consume; no comments beyond a ≤ 3-line module doc comment; self-documenting names; single-section boolean conditions everywhere.

### 3.1 Type: `FolderListing` (listing type name and fields — final)

```rust
pub(crate) struct FolderListing {
    image_paths: Vec<PathBuf>,
}
```

- One PRIVATE field only; deliberately NO folder-path field — the folder identity is always recoverable via `opened_image_path.parent()` by the consumer (Task 2). Minimal field surface.
- Invariant after construction: one entry per qualifying sibling file, each `PathBuf` = canonicalized-folder + file name, in the v1 collation order of §2.2.

### 3.2 Constructor — from the currently opened image's parent folder

```rust
pub(crate) fn from_opened_image(opened_image_path: &Path) -> Option<Self>
```

Exact behavior, step by step:

1. `opened_image_path.parent()?` — a bare name with no parent folder yields `None`.
2. `fs::canonicalize(folder_path).ok()?` — a nonexistent/unresolvable folder (including the empty `""` parent) yields `None`. Canonicalizing the folder FIRST means every entry path of the listing is built in canonical form.
3. `fs::read_dir(resolved_folder)` — STRICTLY NON-RECURSIVE (`read_dir` never descends; no grandparent/child traversal by construction).
4. Per entry: `dir_entry.file_type().ok()?` then SKIP any child directory (`entry_type.is_dir()`); a file-type error deterministically skips the entry.
5. Per surviving entry: keep ONLY names with a whitelisted extension (decision (a)); `dir_entry.ok()` on the iterator's `Result` makes an entry unreadable at read time a deterministic skip.
6. Sort the kept `PathBuf`s by decision (b).
7. Return `Some(Self { image_paths })` — possibly EMPTY: a real folder with zero qualifying images is a valid listing of count 0.

Designated `None` conditions (for Task 2): no parent, folder fails `canonicalize`, `read_dir` fails. A folder that exists but holds no image files is NOT `None` — it is an empty listing.

### 3.3 Index lookup — by resolved canonical path (PICKED, with rationale)

```rust
pub(crate) fn listed_index_of(&self, opened_image_path: &Path) -> Option<usize>
```

Exact behavior: `fs::canonicalize(opened_image_path).ok()?` → the first `position` in `image_paths` whose stored `PathBuf` EQUALS the resolved path (plain `PathBuf ==` comparison; NO second canonicalization pass — entries are already canonical via §3.2 step 2). Fail to canonicalize the request ⇒ `None`.

WHY exact-equality-on-canonical-forms suffices and how it interacts with path differences:

- `std::fs::canonicalize` normalizes BOTH sides identically: absolute path, resolved `.`/`..`/symlinks, OS-canonical separators; on Windows it additionally resolves 8.3 short names and drive-letter casing and returns the `\\?\`-prefixed absolute form — so a relative, `.`-mixed, or differently-spelled form of the same file resolves to the same path and then matches EXACTLY once, case-sensitively at the byte level of the canonical form.
- The app only ever looks up files that ARE openable (Wave-1 decode requires a readable file), so the request side succeeds for every real consumer; a failed `canonicalize` (file deleted between open and listing) is legitimately "no longer listed" ⇒ `None`.

### 3.4 Stepping helpers — ±1 index computation for Task 2 to consume

```rust
#[derive(Debug, PartialEq)]
pub(crate) enum ListedNeighbor {
    At {
        listed_index: usize,
        image_path: PathBuf,
    },
    ListBeginningReached,
    ListEndReached,
}

pub(crate) fn previous_neighbor(&self, listed_index: usize) -> ListedNeighbor
pub(crate) fn next_neighbor(&self, listed_index: usize) -> ListedNeighbor
```

Exact behavior:

- CONTRACT: the argument is ALWAYS a value previously returned by `listed_index_of` (an out-of-range index is a consumer programming error). Both methods open with `assert!(self.contains_listed_index(listed_index));` where `contains_listed_index` is a private predicate `listed_index < self.image_paths.len()` (this keeps every `if` a single-section condition). An EMPTY listing would hit the assert instead of returning a wrong silent answer — by contract it is unreachable there, because `listed_index_of` on an empty listing returns `None` first.
- `previous_neighbor(i)`: `i == 0` ⇒ `ListBeginningReached`; otherwise `At { listed_index: i - 1, image_path }`.
- `next_neighbor(i)`: `i == len - 1` ⇒ `ListEndReached`; otherwise `At { listed_index: i + 1, image_path }`.
- `At` carries BOTH the neighbor's index and its path (`image_path` cloned out of the listing), so Task 2 can open it through the Wave-1 seam (`open_image`) and reason about the resulting position.
- DESIGNATED RESULT for empty listing / image-not-in-listing / unresolvable-path: ALL THREE collapse to `listed_index_of(...) == None`. The consumer (Task 2) must call `listed_index_of` FIRST: `None` ⇒ no navigation attempt at all (no `previous/next_neighbor` call; the current image stays). This is the double-check behavior Task 2 encodes; Task 1 only guarantees the `None`.
- STAY-vs-WRAP at the edges is explicitly NOT decided here (Task 2's architector owns it): `ListBeginningReached` / `ListEndReached` are exactly the ±1-boundary signals that let Task 2 choose stay or wrap later WITHOUT touching this module.

Minimal accessors — nothing else public:

```rust
pub(crate) fn listed_image_count(&self) -> usize
```

Private helpers (all in this one file): `from_folder`, `collect_folder_images`, `neighbor_at`, `contains_listed_index`, and the free functions `listed_image_entry_path`, `has_listed_image_extension`, `is_listed_image_extension`, `compare_image_file_names`.

### 3.5 Dead-code suppression until Task 2 wires the module

Nothing in the crate consumes `FolderListing` yet, so the whole module is dead code in non-test builds and `cargo clippy -D warnings` (Task 4) would fail on it. Wave-1 precedent uses per-item `#[allow(dead_code)]` (`src/image_loading.rs`); here EVERY item is currently unused, so the plan lands ONE module-level inner attribute immediately after the module doc comment:

```rust
#![allow(dead_code)]
```

(one attribute for the whole module — cleaner than ~8 per-item attributes for a fully-unwired module). BINDING NOTE FOR TASK 2'S PLAN: Task 2 removes this attribute line when the first consumer (app-state wiring) lands.

## 4. Files to create / modify (exact paths)

| Action | Path |
| --- | --- |
| CREATE | `C:\repo\img-scroller\src\folder_listing.rs` |
| MODIFY (one line only) | `C:\repo\img-scroller\src\main.rs` — add `mod folder_listing;` to the alphabetical mod list (after `mod app_display;`, before `mod image_loading;`) |

No other file changes. `Cargo.toml` needs NOTHING (std-only module; version already 0.2.0 — workflow step 3 already done).

## 5. Implementation steps (in order)

### Step A — register the module in `src/main.rs`

Insert exactly one line so the mod list reads (alphabetically):

```rust
mod app;
mod app_display;
mod folder_listing;
mod image_loading;
mod platform;
mod scaling;
```

### Step B — create `src/folder_listing.rs` core (exact shape; write it fmt-style as shown, no comments except the module doc)

```rust
//! Folder listing for horizontal image navigation: the file-sibling images of
//! one folder only -- non-recursive, whitelisted-extension-filtered, and
//! sorted by the fixed v1 alphanumeric collation (folded name + exact tie-break).

#![allow(dead_code)]

use std::cmp::Ordering;
use std::fs;
use std::path::{OsStr, Path, PathBuf};

const IMAGE_FILE_EXTENSIONS: [&str; 17] = [
    "avif", "bmp", "dib", "gif", "heic", "heif", "ico", "jfif", "jpe", "jpeg", "jpg", "jxr",
    "png", "tif", "tiff", "webp", "wdp",
];

pub(crate) struct FolderListing {
    image_paths: Vec<PathBuf>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ListedNeighbor {
    At {
        listed_index: usize,
        image_path: PathBuf,
    },
    ListBeginningReached,
    ListEndReached,
}

impl FolderListing {
    pub(crate) fn from_opened_image(opened_image_path: &Path) -> Option<Self> {
        let folder_path = opened_image_path.parent()?;
        Self::from_folder(folder_path)
    }

    fn from_folder(folder_path: &Path) -> Option<Self> {
        let resolved_folder = fs::canonicalize(folder_path).ok()?;
        let Some(image_paths) = Self::collect_folder_images(&resolved_folder) else {
            return None;
        };
        Some(Self { image_paths })
    }

    fn collect_folder_images(resolved_folder: &Path) -> Option<Vec<PathBuf>> {
        let dir_entries = fs::read_dir(resolved_folder).ok()?;
        let mut image_paths: Vec<PathBuf> = dir_entries
            .filter_map(|dir_entry| listed_image_entry_path(dir_entry.ok()?))
            .collect();
        image_paths.sort_by(|left_path, right_path| {
            compare_image_file_names(&left_path.to_string_lossy(), &right_path.to_string_lossy())
        });
        Some(image_paths)
    }

    pub(crate) fn listed_image_count(&self) -> usize {
        self.image_paths.len()
    }

    pub(crate) fn listed_index_of(&self, opened_image_path: &Path) -> Option<usize> {
        let resolved_image_path = fs::canonicalize(opened_image_path).ok()?;
        self.image_paths
            .iter()
            .position(|listed_path| resolved_image_path == *listed_path)
    }

    pub(crate) fn previous_neighbor(&self, listed_index: usize) -> ListedNeighbor {
        assert!(self.contains_listed_index(listed_index));
        if listed_index == 0 {
            return ListedNeighbor::ListBeginningReached;
        }
        self.neighbor_at(listed_index - 1)
    }

    pub(crate) fn next_neighbor(&self, listed_index: usize) -> ListedNeighbor {
        assert!(self.contains_listed_index(listed_index));
        if listed_index == self.image_paths.len() - 1 {
            return ListedNeighbor::ListEndReached;
        }
        self.neighbor_at(listed_index + 1)
    }

    fn neighbor_at(&self, listed_index: usize) -> ListedNeighbor {
        ListedNeighbor::At {
            listed_index,
            image_path: self.image_paths[listed_index].clone(),
        }
    }

    fn contains_listed_index(&self, listed_index: usize) -> bool {
        listed_index < self.image_paths.len()
    }
}

fn listed_image_entry_path(dir_entry: fs::DirEntry) -> Option<PathBuf> {
    let entry_type = dir_entry.file_type().ok()?;
    if entry_type.is_dir() {
        return None;
    }
    if !has_listed_image_extension(&dir_entry.file_name()) {
        return None;
    }
    Some(dir_entry.path())
}

fn has_listed_image_extension(file_name: &OsStr) -> bool {
    Path::new(file_name)
        .extension()
        .is_some_and(is_listed_image_extension)
}

fn is_listed_image_extension(extension: &OsStr) -> bool {
    IMAGE_FILE_EXTENSIONS
        .iter()
        .any(|listed| extension.eq_ignore_ascii_case(*listed))
}

fn compare_image_file_names(left_name: &str, right_name: &str) -> Ordering {
    let case_folded_order = left_name.to_lowercase().cmp(&right_name.to_lowercase());
    if case_folded_order != Ordering::Equal {
        return case_folded_order;
    }
    left_name.cmp(right_name)
}
```

Transcription notes for the implementer (mechanical facts, not design comments): `*listed` deref is REQUIRED — `eq_ignore_ascii_case` takes `impl AsRef<OsStr>` (i.e. `&str`) and `IMAGE_FILE_EXTENSIONS.iter()` yields `&&str`; the sort closure transforms its arguments (`to_string_lossy`), so it is NOT flagged by clippy's redundant-closure lint, and the comparator stays `fn(&str, &str)`.

### Step C — append the `#[cfg(test)] mod tests` block to `src/folder_listing.rs` (exact shape)

Fixture helper — cleanup is automatic via `Drop` and runs after EVERY test, even on assertion panic; temp dirs live under `std::env::temp_dir()` (container `/tmp`) and NEVER inside the repo:

```rust
#[cfg(test)]
mod tests {
    use super::{FolderListing, ListedNeighbor};
    use std::fs;
    use std::path::Path;

    struct TempFolder {
        folder_path: std::path::PathBuf,
    }

    impl TempFolder {
        fn create(test_case: &str) -> Self {
            let folder_path = std::env::temp_dir().join(format!(
                "img_scroller_folder_listing_{}_{}",
                test_case,
                std::process::id()
            ));
            fs::create_dir_all(&folder_path).expect("temp folder created");
            Self { folder_path }
        }

        fn create_file(&self, file_name: &str) {
            fs::write(self.folder_path.join(file_name), b"x").expect("fixture file written");
        }

        fn image_path(&self, file_name: &str) -> std::path::PathBuf {
            self.folder_path.join(file_name)
        }
    }

    impl Drop for TempFolder {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.folder_path);
        }
    }
```

TEST 1 — enumeration + collation (fold + exact tie-break; deterministic file set named exactly):

```rust
    #[test]
    fn listed_images_sort_case_insensitively_then_by_exact_name_tie_break() {
        let folder = TempFolder::create("sort_collation");
        folder.create_file("beta.jpg");
        folder.create_file("Beta.png");
        folder.create_file("ALPHA.jpg");
        folder.create_file("Alpha.jpg");
        folder.create_file("alpha.jpg");
        folder.create_file("alpha.gif");
        folder.create_file("zeta.webp");
        let listing =
            FolderListing::from_opened_image(folder.image_path("beta.jpg").as_path())
                .expect("listing built");
        assert_eq!(listing.listed_image_count(), 7);
        assert_eq!(listing.listed_index_of(&folder.image_path("alpha.gif")), Some(0));
        assert_eq!(listing.listed_index_of(&folder.image_path("ALPHA.jpg")), Some(1));
        assert_eq!(listing.listed_index_of(&folder.image_path("Alpha.jpg")), Some(2));
        assert_eq!(listing.listed_index_of(&folder.image_path("alpha.jpg")), Some(3));
        assert_eq!(listing.listed_index_of(&folder.image_path("beta.jpg")), Some(4));
        assert_eq!(listing.listed_index_of(&folder.image_path("Beta.png")), Some(5));
        assert_eq!(listing.listed_index_of(&folder.image_path("zeta.webp")), Some(6));
    }
```

Why index round-trips prove the order (no list export needed): `listed_index_of` returns EACH file's position in the constructed sort — if the fold wrongly placed `alpha.jpg` before `ALPHA.jpg`, the `Some(1)`/`Some(2)` assertions would swap and fail. Expected-order derivation: folded `alpha.gif` < `alpha.jpg` (=`ALPHA.jpg`=`Alpha.jpg` folded) < `beta.jpg` < `beta.png` (=`Beta.png` folded) < `zeta.webp`; tie-break inside the alpha trio: `ALPHA.jpg` < `Alpha.jpg` < `alpha.jpg` (`L`(76) vs `h`(104), then `A`(65) vs `a`(97)).

TEST 2 — folder-only scope + extension filter (case-insensitive suffix + hidden-style dotfile + no-extension exclusion + nested-dir exclusion):

```rust
    #[test]
    fn listing_excludes_directories_and_files_without_listed_extensions() {
        let folder = TempFolder::create("folder_only_scope");
        folder.create_file("notes.txt");
        folder.create_file("extensionless_image");
        folder.create_file(".concealed.png");
        folder.create_file("LOWER.PNG");
        folder.create_file("upper.jpg");
        fs::create_dir(folder.folder_path.join("sibling_subfolder")).expect("subfolder");
        fs::write(
            folder.folder_path.join("sibling_subfolder").join("inner.png"),
            b"x",
        )
        .expect("nested file");
        let listing =
            FolderListing::from_opened_image(folder.image_path("upper.jpg").as_path())
                .expect("listing built");
        assert_eq!(listing.listed_image_count(), 3);
        assert_eq!(listing.listed_index_of(&folder.image_path(".concealed.png")), Some(0));
        assert_eq!(listing.listed_index_of(&folder.image_path("LOWER.PNG")), Some(1));
        assert_eq!(listing.listed_index_of(&folder.image_path("upper.jpg")), Some(2));
    }
```

(`inner.png` inside `sibling_subfolder` does NOT appear — proves strict one-level scope; `notes.txt`/`extensionless_image` cannot appear at any index; `LOWER.PNG` inclusion proves the case-insensitive suffix match; `.concealed.png` inclusion proves hidden-file neutrality; folded order `".concealed.png"` < `"lower.png"` < `"upper.jpg"` because `.`(46) < `l` < `u`.)

TEST 3 — index resolution (exact canonical match, `.`-variant match, unlisted → `None`, missing → `None`):

```rust
    #[test]
    fn unlisted_and_missing_images_resolve_to_no_listing_index() {
        let folder = TempFolder::create("index_resolution");
        folder.create_file("only.jpg");
        folder.create_file("other.txt");
        let listing =
            FolderListing::from_opened_image(folder.image_path("only.jpg").as_path())
                .expect("listing built");
        assert_eq!(listing.listed_index_of(&folder.image_path("only.jpg")), Some(0));
        assert_eq!(
            listing.listed_index_of(&folder.folder_path.join(".").join("only.jpg")),
            Some(0)
        );
        assert_eq!(listing.listed_index_of(&folder.image_path("other.txt")), None);
        assert_eq!(listing.listed_index_of(&folder.image_path("missing.png")), None);
    }
```

(the `join(".")` assertion proves canonicalization absorbs redundant components — the path-differences case of §3.3.)

TEST 4 — prev/next stepping + both list edges:

```rust
    #[test]
    fn neighbors_step_by_one_and_report_list_edges() {
        let folder = TempFolder::create("neighbor_stepping");
        folder.create_file("a.png");
        folder.create_file("b.png");
        folder.create_file("c.png");
        let listing =
            FolderListing::from_opened_image(folder.image_path("b.png").as_path())
                .expect("listing built");
        assert_eq!(
            listing.previous_neighbor(1),
            ListedNeighbor::At {
                listed_index: 0,
                image_path: folder.image_path("a.png"),
            }
        );
        assert_eq!(
            listing.next_neighbor(1),
            ListedNeighbor::At {
                listed_index: 2,
                image_path: folder.image_path("c.png"),
            }
        );
        assert_eq!(
            listing.previous_neighbor(0),
            ListedNeighbor::ListBeginningReached
        );
        assert_eq!(listing.next_neighbor(2), ListedNeighbor::ListEndReached);
    }
```

TEST 5 — empty folder (valid zero-count listing) + unresolvable parent (`None`):

```rust
    #[test]
    fn empty_folder_yields_empty_listing_and_missing_folder_yields_none() {
        let folder = TempFolder::create("empty_folder");
        let listing =
            FolderListing::from_opened_image(folder.image_path("unopened.png").as_path())
                .expect("listing built for an existing folder");
        assert_eq!(listing.listed_image_count(), 0);
        let listing_without_folder =
            FolderListing::from_opened_image(Path::new("single_name.png"));
        assert!(listing_without_folder.is_none());
    }
```

(`single_name.png` has parent `Some("")`, whose `canonicalize` fails ⇒ `None` — the §3.2 designated `None` set.)

### Step D — line-budget check (binding)

Read the finished `src/folder_listing.rs` and confirm ≤ 200 total lines (the shapes above land ≈ 195–205). PRE-DECIDED remedy if the written file exceeds 200 total lines: move the `TempFolder` helper (whole struct + `impl TempFolder` + `impl Drop`, ≈ 22 lines) into a NEW file `src/folder_listing_test_support.rs` containing the same helper with `pub(crate)` items, register it in `src/main.rs` with `#[cfg(test)] mod folder_listing_test_support;`, and change the tests to `use crate::folder_listing_test_support::TempFolder;`. Do NOT weaken any test to save lines.

### Step E — implementer verification WITHOUT cargo (host has no Rust)

1. Re-read both finished files with the Read tool and confirm: mod list alphabetical; `folder_listing.rs` ≤ 200 lines; no commented-out code (the `#![allow(dead_code)]` attribute is NOT a comment and stays); no comments besides the 3-line module doc; every `if` single-section; nesting depth ≤ 2; every method ≤ 50 lines; every function/method ≤ 2 non-self params.
2. Confirm the `ListedNeighbor` enum derives `Debug, PartialEq` (the tests `assert_eq!` on it directly).
3. Confirm NOTHING else changed (`git status` shows only `src/folder_listing.rs` (new), `src/main.rs` (modified), and the plan file).

## 6. Terminal commands (Task 1)

NONE. No cargo runs, no fmt/clippy/test invocations in Task 1 — the host has no toolchain, and the TODO/§0 scope reserves container checks for Task 4 (dev-checks harness: `vm_status` first → `sh scripts/dev-checks.sh` via the `alpine-vm` MCP; exit code authoritative). The Task-4 dev-checks cover this module's fmt/check/test/clippy.

## 7. Git actions

- Branch: `feat/wave-2-horizontal-scroll` (already created at workflow step 2; implementer verifies `git branch --show-current` first — if not on it, STOP and return a question to the caller).
- Before staging: follow `.kilo/rules/gitignore-compliance.md` — read `.gitignore`, run `git status`; ensure nothing matching `dist/`, `logs/`, `target/`, `*.log` is staged. Expected staged set: `src/folder_listing.rs`, `src/main.rs`, `.kilo/plans/20261031-wave2-task1-folder-listing.md`.
- One commit AFTER implementation: `git add src/folder_listing.rs src/main.rs .kilo/plans/20261031-wave2-task1-folder-listing.md` → commit message: `feat: current-folder image listing for horizontal scroll (Wave 2 Task 1)`.
- NO push in this step (push is restricted to workflow step 5, `main` → `origin` only).
- TODO-file `[DONE]` marks + `[x]` sub-items happen at workflow step 4.6 (separate implementer commit) — NOT here.

## 8. Integration seam to Wave-1 code (recording only — NOT implemented in Task 1)

- Who calls the constructor: Task 2's navigation code inside `ImagesScrollerApp` — it reads `self.opened_image_path` (the Wave-1 state field) and calls `FolderListing::from_opened_image(path.as_path())` after each successful open (dialog or CLI); Task 2's plan owns the in-session listing cache, rebuild triggers, and invalidation.
- Task 3's buttons reuse Task 2's shared navigation handler — no separate listing consumption.
- Consumption contract handed to Task 2 (verbatim summary): (1) build/rebuild the listing from the opened image's parent; (2) resolve the current index with `listed_index_of` — `None` (empty listing / not listed / unresolvable) ⇒ NO navigation, current image stays; (3) feed the resolved index into `previous_neighbor` / `next_neighbor`; (4) `ListBeginningReached`/`ListEndReached` ⇒ Task 2's architector decides stay or wrap; (5) `At { image_path, listed_index }` ⇒ open via the Wave-1 `open_image` seam (failure keeps the current image and shows the message — existing behavior).

## 9. Notes for later waves / steps (do NOT execute now)

- Task 2: remove the module-level `#![allow(dead_code)]` once a consumer exists; wire state; decide stay-vs-wrap using `ListBeginningReached`/`ListEndReached`.
- Task 4: add any first/last-edge tests the Task-2 decision requires at the app level; this module's five tests already cover enumeration scope, sort, stepping, empty, not-in-listing.
- Task 5 (docs): record in `tech.md` the landed decisions — 17-entry extension whitelist (list it there, once), collation = Unicode-folded name compare + exact-name UTF-8/codepoint tie-break, index resolution = `fs::canonicalize` both sides + exact `PathBuf` equality; record in `architecture.md` the new module in "Module Layout"; `context.md` closing update.
- Risk (Wave-1 precedent): agent-written code can drift from `cargo fmt`'s exact output; if `cargo fmt --check` fails in Task 4's dev-checks, run container-side `cargo fmt` (or the fix cycle) — same pattern as Wave 1's fmt-restoration of `windows.rs`.
- Risk: fixture ordering relies on Linux `/tmp` canonicalization identity (container `tmp` has no symlink on Debian — verified behavior of the pinned `rust:1.99.1-bookworm` image); tests are container-runnable by design.

## 10. Verification mapping (TODO Task 1 sub-items → plan elements)

| TODO Task-1 sub-item | Covered by |
| --- | --- |
| Enumerate SAME-folder images only (non-recursive) | §3.2 steps 3–5 (read_dir + `is_dir` skip); TEST 2 (nested `inner.png` excluded; subfolder entry excluded) |
| "Image" filter decided in architector step | §1 — 17-entry whitelist, ASCII-case-insensitive suffix match; TEST 2 assertions |
| Sort alphanumeric (collation decided here; v1 fixed, no settings UI) | §2 — fold + exact tie-break; TEST 1 + the `Beta.png` pair inside it |

---
## Addendum — Decisions Summary for the caller (briefing later steps)

1. FILTER: fixed extension whitelist (NOT a WIC probe — container-untestable); exact set (17, matched ASCII-case-insensitively on the filename suffix): `avif, bmp, dib, gif, heic, heif, ico, jfif, jpe, jpeg, jpg, jxr, png, tif, tiff, webp, wdp`; extension-less files excluded; hidden files included.
2. COLLATION: case-INSENSITIVE primary compare via `str::to_lowercase` fold, compared as UTF-8 bytes (≡ Unicode codepoint order); tie-break = exact-name UTF-8-byte compare; extension is an ordinary name part; NO numeric/natural chunking (`img10` < `img2` is the fixed v1 order).
3. MODULE: `src/folder_listing.rs`; `pub(crate) struct FolderListing { image_paths: Vec<PathBuf> }`; `from_opened_image(&Path) -> Option<Self>` (canonicalize parent first, read_dir one level, skip dirs, filter, sort); `listed_index_of(&Path) -> Option<usize>` (canonicalize request, exact canonical `PathBuf` equality); `previous_neighbor`/`next_neighbor(usize) -> ListedNeighbor` with `At { listed_index, image_path }` / `ListBeginningReached` / `ListEndReached`; `listed_image_count()`.
4. DESIGNATED `None` (single result for empty listing / image-not-in-listing / unresolvable path) = `listed_index_of(...) == None` ⇒ Task 2 performs NO navigation; stay-vs-wrap NOT decided here (Task 2's architector uses the two edge variants).
5. FILES: create `src/folder_listing.rs` (~200 lines incl. 5 tests + `TempFolder` fixture under `std::env::temp_dir()` with `Drop` cleanup); modify `src/main.rs` (one `mod folder_listing;` line, alphabetical). Module-level `#![allow(dead_code)]` until Task 2 wires (removal is Task 2's binding).
6. COMMANDS: none in Task 1; verification = Task 4 container dev-checks. GIT: one implementer commit `feat: current-folder image listing for horizontal scroll (Wave 2 Task 1)` (src files + this plan file; gitignore check first; no push).

