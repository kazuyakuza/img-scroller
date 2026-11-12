//! Sibling-folder listing for vertical navigation: the DIRECTORY entries of the
//! parent of the opened image's folder only, from one non-recursive read -- an
//! unreadable scope parent yields an empty listing -- with the current folder
//! excluded from the destinations and sorted by the fixed v1 collation shared
//! with the image listing.

use crate::name_collation::EntryNameOrder;
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

type ScopeEntriesReader<'r> = dyn Fn(&Path) -> Option<Vec<ScopeEntry>> + 'r;

struct ScopeEntry {
    path: PathBuf,
    is_directory: bool,
}

/// Resolved sibling destination folder, or the list bound reached from the anchor.
#[derive(Debug, PartialEq)]
pub(crate) enum ListedFolderNeighbor {
    At {
        #[allow(dead_code)]
        listed_index: usize,
        folder_path: PathBuf,
    },
    ListBeginningReached,
    ListEndReached,
}

pub(crate) struct SiblingFolderListing {
    sibling_folder_paths: Vec<PathBuf>,
    entry_order: EntryNameOrder,
}

struct SortScope<'a> {
    current_folder: &'a Path,
    entry_order: EntryNameOrder,
}

impl SiblingFolderListing {
    /// Build the listing from the opened image path; `None` if folder or parent is unresolvable.
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
    /// Number of listed sibling destination folders (the current folder is excluded).
    #[allow(dead_code)]
    pub(crate) fn listed_folder_count(&self) -> usize {
        self.sibling_folder_paths.len()
    }
    /// Previous sibling from the anchor; `ListBeginningReached` at the start of the list.
    pub(crate) fn previous_neighbor_of(
        &self,
        opened_image_path: &Path,
    ) -> Option<ListedFolderNeighbor> {
        let position = self.position_of_current_folder(opened_image_path)?;
        if position == 0 {
            return Some(ListedFolderNeighbor::ListBeginningReached);
        }
        Some(self.neighbor_at(position - 1))
    }
    /// Next sibling from the anchor; `ListEndReached` at the end of the list.
    pub(crate) fn next_neighbor_of(
        &self,
        opened_image_path: &Path,
    ) -> Option<ListedFolderNeighbor> {
        let position = self.position_of_current_folder(opened_image_path)?;
        if position == self.sibling_folder_paths.len() {
            return Some(ListedFolderNeighbor::ListEndReached);
        }
        Some(self.neighbor_at(position))
    }
    #[allow(dead_code)]
    fn from_opened_image_with_scope_entries(
        opened_image_path: &Path,
        read_scope_entries: &ScopeEntriesReader<'_>,
    ) -> Option<Self> {
        let resolved_current_folder = resolve_current_folder(opened_image_path)?;
        let scope_parent_path = resolved_current_folder.parent()?;
        let scope_entries = read_scope_entries(scope_parent_path).unwrap_or_default();
        let sort_scope = SortScope {
            current_folder: &resolved_current_folder,
            entry_order: EntryNameOrder::system(),
        };
        Some(Self::sorted_scope_folders(scope_entries, sort_scope))
    }
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
        let listed_name = listed_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        self.entry_order.compare(&listed_name, current_name) == Ordering::Less
    }
    fn neighbor_at(&self, listed_index: usize) -> ListedFolderNeighbor {
        ListedFolderNeighbor::At {
            listed_index,
            folder_path: self.sibling_folder_paths[listed_index].clone(),
        }
    }
}

fn resolve_current_folder(opened_image_path: &Path) -> Option<PathBuf> {
    let current_folder_path = opened_image_path.parent()?;
    fs::canonicalize(current_folder_path).ok()
}

fn read_scope_entries(folder_path: &Path) -> Option<Vec<ScopeEntry>> {
    let dir_entries = fs::read_dir(folder_path).ok()?;
    Some(
        dir_entries
            .filter_map(|entry| scope_entry_of(entry.ok()?))
            .collect(),
    )
}

fn scope_entry_of(dir_entry: fs::DirEntry) -> Option<ScopeEntry> {
    let entry_type = dir_entry.file_type().ok()?;
    Some(ScopeEntry {
        path: dir_entry.path(),
        is_directory: entry_type.is_dir(),
    })
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod neighbor_tests;

#[cfg(test)]
mod collation_tests;
