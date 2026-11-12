use super::SiblingFolderListing;
use super::tests::{TempScope, folder_neighbor, scope_child_paths};
use crate::name_collation::{EntryNameOrder, compare_entry_names};
use std::cmp::Ordering;

fn reversed_entry_names(left_name: &str, right_name: &str) -> Ordering {
    compare_entry_names(right_name, left_name)
}

#[test]
fn sibling_folders_sort_through_the_injected_order_seam() {
    let scope = TempScope::create("japanese_reversed_order");
    for folder_name in ["a", "あ", "ア", "漢"] {
        scope.create_scope_child_folder(folder_name);
    }
    let anchor = scope.current_image_path("shot.png");
    let listing = SiblingFolderListing::from_opened_image_with_order(
        anchor.as_path(),
        EntryNameOrder::with_comparator(reversed_entry_names),
    )
    .expect("listing built");
    assert_eq!(
        listing.sibling_folder_paths,
        scope_child_paths(&scope, &["漢", "ア", "あ", "a"])
    );
}

#[test]
fn anchor_position_follows_the_injected_order() {
    let scope = TempScope::create("japanese_anchor_order");
    scope.create_scope_child_folder("aaa");
    scope.create_scope_child_folder("zzz");
    let anchor = scope.current_image_path("shot.png");
    let listing = SiblingFolderListing::from_opened_image_with_order(
        anchor.as_path(),
        EntryNameOrder::with_comparator(reversed_entry_names),
    )
    .expect("listing built");
    assert_eq!(
        listing.sibling_folder_paths,
        scope_child_paths(&scope, &["zzz", "aaa"])
    );
    assert_eq!(
        listing.previous_neighbor_of(anchor.as_path()),
        Some(folder_neighbor(0, scope.scope_child_path("zzz")))
    );
    assert_eq!(
        listing.next_neighbor_of(anchor.as_path()),
        Some(folder_neighbor(1, scope.scope_child_path("aaa")))
    );
}
