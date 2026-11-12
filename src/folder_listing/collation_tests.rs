use super::FolderListing;
use super::tests::TempFolder;
use crate::name_collation::{EntryNameOrder, compare_entry_names};
use std::cmp::Ordering;

fn reversed_entry_names(left_name: &str, right_name: &str) -> Ordering {
    compare_entry_names(right_name, left_name)
}

#[test]
fn listed_images_fallback_order_places_ascii_before_japanese() {
    let folder = TempFolder::create("japanese_fallback_order");
    let file_names = ["a.png", "あ.png", "ア.png", "漢.png"];
    for file_name in file_names {
        folder.create_file(file_name);
    }
    let listing = FolderListing::from_opened_image_with_order(
        folder.image_path("a.png").as_path(),
        EntryNameOrder::with_comparator(compare_entry_names),
    )
    .expect("listing built");
    let listed_indices: Vec<Option<usize>> = file_names
        .iter()
        .map(|file_name| listing.listed_index_of(&folder.image_path(file_name)))
        .collect();
    assert_eq!(listed_indices, [Some(0), Some(1), Some(2), Some(3)]);
}

#[test]
fn listed_images_sort_through_the_injected_order_seam() {
    let folder = TempFolder::create("japanese_reversed_order");
    let file_names = ["a.png", "あ.png", "ア.png", "漢.png"];
    for file_name in file_names {
        folder.create_file(file_name);
    }
    let listing = FolderListing::from_opened_image_with_order(
        folder.image_path("a.png").as_path(),
        EntryNameOrder::with_comparator(reversed_entry_names),
    )
    .expect("listing built");
    let listed_indices: Vec<Option<usize>> = ["漢.png", "ア.png", "あ.png", "a.png"]
        .iter()
        .map(|file_name| listing.listed_index_of(&folder.image_path(file_name)))
        .collect();
    assert_eq!(listed_indices, [Some(0), Some(1), Some(2), Some(3)]);
}
