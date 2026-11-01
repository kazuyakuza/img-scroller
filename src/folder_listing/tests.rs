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
