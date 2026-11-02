//! Neighbor-stepping and symlink-policy tests of sibling_folder_listing,
//! reusing the TempScope/listing helpers published by the sibling
//! `tests` module over the same fixture.

use super::ListedFolderNeighbor;
use super::tests::{
    TempScope, folder_neighbor, listing_for, listing_with_entries, scope_child_paths,
};
use std::fs;
use std::path::Path;

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
    assert_eq!(
        beta_listing.previous_neighbor_of(beta_anchor.as_path()),
        alpha_at_zero()
    );
    assert_eq!(
        beta_listing.next_neighbor_of(beta_anchor.as_path()),
        gamma_at_one()
    );
    let alpha_anchor = scope.scope_child_path("alpha").join("img.png");
    let alpha_listing = listing_with_entries(alpha_anchor.as_path(), &read_entries);
    assert_eq!(
        alpha_listing.previous_neighbor_of(alpha_anchor.as_path()),
        Some(ListedFolderNeighbor::ListBeginningReached)
    );
    assert_eq!(
        alpha_listing.next_neighbor_of(alpha_anchor.as_path()),
        beta_at_zero()
    );
    let gamma_anchor = scope.scope_child_path("gamma").join("img.png");
    let gamma_listing = listing_with_entries(gamma_anchor.as_path(), &read_entries);
    assert_eq!(
        gamma_listing.previous_neighbor_of(gamma_anchor.as_path()),
        beta_at_one()
    );
    assert_eq!(
        gamma_listing.next_neighbor_of(gamma_anchor.as_path()),
        Some(ListedFolderNeighbor::ListEndReached)
    );
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
