use super::{ListedFolderNeighbor, ScopeEntriesReader, ScopeEntry, SiblingFolderListing};
use std::fs;
use std::path::{Path, PathBuf};

const CURRENT_FOLDER_NAME: &str = "mid";

struct TempScope {
    scope_path: PathBuf,
}

impl TempScope {
    fn create(test_case: &str) -> Self {
        let scope_path = std::env::temp_dir().join(format!(
            "img_scroller_sibling_scope_{}_{}",
            test_case,
            std::process::id()
        ));
        let mid_folder_path = scope_path.join(CURRENT_FOLDER_NAME);
        fs::create_dir_all(&mid_folder_path).expect("scope fixture created");
        fs::write(mid_folder_path.join("shot.png"), b"x").expect("marker written");
        Self { scope_path }
    }
    fn current_image_path(&self, file_name: &str) -> PathBuf {
        self.scope_path.join(CURRENT_FOLDER_NAME).join(file_name)
    }
    fn scope_child_path(&self, name: &str) -> PathBuf {
        self.scope_path.join(name)
    }
    fn create_scope_child_folder(&self, name: &str) {
        fs::create_dir(self.scope_child_path(name)).expect("scope child folder created");
    }
    fn create_scope_child_file(&self, name: &str) {
        fs::write(self.scope_child_path(name), b"x").expect("scope child file written");
    }
    fn synthetic_entry(&self, name: &str, is_directory: bool) -> ScopeEntry {
        ScopeEntry {
            path: self.scope_child_path(name),
            is_directory,
        }
    }
    fn external_target_path(&self) -> PathBuf {
        let scope_name = self.scope_path.file_name().unwrap_or_default().to_string_lossy();
        self.scope_path.with_file_name(format!("{scope_name}_target"))
    }
}

impl Drop for TempScope {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.scope_path);
        let _ = fs::remove_dir_all(self.external_target_path());
    }
}

fn listing_for(image_path: &Path) -> SiblingFolderListing {
    SiblingFolderListing::from_opened_image(image_path).expect("listing built")
}
fn listing_with_entries(
    image_path: &Path,
    read_scope_entries: &ScopeEntriesReader,
) -> SiblingFolderListing {
    SiblingFolderListing::from_opened_image_with_scope_entries(image_path, read_scope_entries)
        .expect("listing built")
}
fn folder_neighbor(listed_index: usize, folder_path: PathBuf) -> ListedFolderNeighbor {
    ListedFolderNeighbor::At {
        listed_index,
        folder_path,
    }
}
fn scope_child_paths(scope: &TempScope, names: &[&str]) -> Vec<PathBuf> {
    names.iter().map(|name| scope.scope_child_path(name)).collect()
}

#[test]
fn sibling_folders_sort_by_lowercase_form_then_exact_name_tie_break() {
    let scope = TempScope::create("sort_collation");
    for folder_name in ["ALPHA", "Alpha", "alpha", "Bombay", "ZULU"] {
        scope.create_scope_child_folder(folder_name);
    }
    let listing = listing_for(scope.current_image_path("shot.png").as_path());
    assert_eq!(listing.listed_folder_count(), 5);
    assert_eq!(
        listing.sibling_folder_paths,
        scope_child_paths(&scope, &["ALPHA", "Alpha", "alpha", "Bombay", "ZULU"])
    );
}
#[test]
fn listing_enumerates_only_scope_child_directories() {
    let scope = TempScope::create("directories_only");
    scope.create_scope_child_folder("folder1");
    scope.create_scope_child_folder(".concealed_dir");
    scope.create_scope_child_file("stray.txt");
    scope.create_scope_child_file(".concealed_file.png");
    let listing = listing_for(scope.current_image_path("shot.png").as_path());
    assert_eq!(listing.listed_folder_count(), 2);
    assert_eq!(
        listing.sibling_folder_paths,
        scope_child_paths(&scope, &[".concealed_dir", "folder1"])
    );
}
#[test]
fn current_folder_is_excluded_from_the_enumerated_siblings() {
    let scope = TempScope::create("self_exclusion");
    let read_entries = |_: &Path| {
        Some(vec![
            scope.synthetic_entry("first", true),
            scope.synthetic_entry(CURRENT_FOLDER_NAME, true),
            scope.synthetic_entry("second", true),
        ])
    };
    let anchor = scope.current_image_path("shot.png");
    let listing = listing_with_entries(anchor.as_path(), &read_entries);
    assert_eq!(listing.listed_folder_count(), 2);
    assert_eq!(
        listing.sibling_folder_paths,
        scope_child_paths(&scope, &["first", "second"])
    );
}
#[test]
fn listing_covers_only_immediate_scope_child_folders() {
    let scope = TempScope::create("non_recursive");
    scope.create_scope_child_folder("folder_a");
    fs::create_dir(scope.scope_child_path("folder_a").join("inner")).expect("grandchild created");
    fs::write(scope.scope_child_path("folder_a").join("leaf.txt"), b"x").expect("leaf written");
    scope.create_scope_child_folder("folder_b");
    scope.create_scope_child_folder("folder_c");
    fs::write(scope.scope_child_path("folder_c").join("pic.png"), b"x").expect("pic written");
    let listing = listing_for(scope.current_image_path("shot.png").as_path());
    assert_eq!(
        listing.sibling_folder_paths,
        scope_child_paths(&scope, &["folder_a", "folder_b", "folder_c"])
    );
}
#[test]
fn folder_neighbors_step_by_one_and_report_scope_edges() {
    let scope = TempScope::create("neighbor_stepping");
    for folder_name in ["alpha", "beta", "gamma"] {
        scope.create_scope_child_folder(folder_name);
    }
    let read_entries = |_: &Path| {
        Some(vec![
            scope.synthetic_entry("alpha", true),
            scope.synthetic_entry("beta", true),
            scope.synthetic_entry("gamma", true),
        ])
    };
    let alpha_at_zero = || Some(folder_neighbor(0, scope.scope_child_path("alpha")));
    let beta_at_zero = || Some(folder_neighbor(0, scope.scope_child_path("beta")));
    let beta_at_one = || Some(folder_neighbor(1, scope.scope_child_path("beta")));
    let gamma_at_one = || Some(folder_neighbor(1, scope.scope_child_path("gamma")));
    let beta_anchor = scope.scope_child_path("beta").join("img.png");
    let beta_listing = listing_with_entries(beta_anchor.as_path(), &read_entries);
    assert_eq!(beta_listing.previous_neighbor_of(beta_anchor.as_path()), alpha_at_zero());
    assert_eq!(beta_listing.next_neighbor_of(beta_anchor.as_path()), gamma_at_one());
    let alpha_anchor = scope.scope_child_path("alpha").join("img.png");
    let alpha_listing = listing_with_entries(alpha_anchor.as_path(), &read_entries);
    assert_eq!(
        alpha_listing.previous_neighbor_of(alpha_anchor.as_path()),
        Some(ListedFolderNeighbor::ListBeginningReached)
    );
    assert_eq!(alpha_listing.next_neighbor_of(alpha_anchor.as_path()), beta_at_zero());
    let gamma_anchor = scope.scope_child_path("gamma").join("img.png");
    let gamma_listing = listing_with_entries(gamma_anchor.as_path(), &read_entries);
    assert_eq!(gamma_listing.previous_neighbor_of(gamma_anchor.as_path()), beta_at_one());
    assert_eq!(
        gamma_listing.next_neighbor_of(gamma_anchor.as_path()),
        Some(ListedFolderNeighbor::ListEndReached)
    );
}
#[test]
fn scope_without_siblings_yields_empty_listing() {
    let scope = TempScope::create("no_siblings");
    scope.create_scope_child_file("stray.txt");
    let listing = listing_for(scope.current_image_path("shot.png").as_path());
    assert_eq!(listing.listed_folder_count(), 0);
}
#[test]
fn missing_or_unreadable_scope_folder_yields_empty_listing_and_none_when_unanchorable() {
    let scope = TempScope::create("unreadable_scope");
    let anchor = scope.current_image_path("shot.png");
    let unreadable_listing = listing_with_entries(anchor.as_path(), &|_: &Path| None);
    assert_eq!(unreadable_listing.listed_folder_count(), 0);
    assert!(SiblingFolderListing::from_opened_image(Path::new("single_name.png")).is_none());
    let missing_current_image = scope.scope_child_path("missing_dir").join("img.png");
    assert!(SiblingFolderListing::from_opened_image(missing_current_image.as_path()).is_none());
}
#[test]
fn symlinked_scope_entries_are_not_listed_as_directories() {
    let scope = TempScope::create("symlink_policy");
    scope.create_scope_child_folder("folder1");
    let target_path = scope.external_target_path();
    fs::create_dir_all(&target_path).expect("symlink target created");
    std::os::unix::fs::symlink(&target_path, scope.scope_child_path("linked_dir"))
        .expect("symlink created");
    let listing = listing_for(scope.current_image_path("shot.png").as_path());
    assert_eq!(
        listing.sibling_folder_paths,
        scope_child_paths(&scope, &["folder1"])
    );
}
