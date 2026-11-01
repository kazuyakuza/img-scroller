//! Sibling-folder listing for vertical navigation: the DIRECTORY entries of the
//! parent of the opened image's folder only, from one non-recursive read, with
//! the current folder excluded from the destinations and sorted by the fixed
//! v1 collation shared with the image listing.

use crate::name_collation::compare_entry_names;
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

type ScopeEntriesReader = dyn Fn(&Path) -> Option<Vec<ScopeEntry>>;

struct ScopeEntry {
    path: PathBuf,
    is_directory: bool,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
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
}

impl SiblingFolderListing {
    #[allow(dead_code)]
    pub(crate) fn from_opened_image(opened_image_path: &Path) -> Option<Self> {
        Self::from_opened_image_with_scope_entries(opened_image_path, &read_scope_entries)
    }
    #[allow(dead_code)]
    pub(crate) fn listed_folder_count(&self) -> usize {
        self.sibling_folder_paths.len()
    }
    #[allow(dead_code)]
    pub(crate) fn previous_neighbor_of(&self, folder_path: &Path) -> Option<ListedFolderNeighbor> {
        let position = self.position_of_current_folder(folder_path)?;
        if position == 0 {
            return Some(ListedFolderNeighbor::ListBeginningReached);
        }
        Some(self.neighbor_at(position - 1))
    }
    #[allow(dead_code)]
    pub(crate) fn next_neighbor_of(&self, folder_path: &Path) -> Option<ListedFolderNeighbor> {
        let position = self.position_of_current_folder(folder_path)?;
        if position == self.sibling_folder_paths.len() {
            return Some(ListedFolderNeighbor::ListEndReached);
        }
        Some(self.neighbor_at(position))
    }
    fn from_opened_image_with_scope_entries(
        opened_image_path: &Path,
        read_scope_entries: &ScopeEntriesReader,
    ) -> Option<Self> {
        let current_folder_path = opened_image_path.parent()?;
        let resolved_current_folder = fs::canonicalize(current_folder_path).ok()?;
        let scope_parent_path = resolved_current_folder.parent()?;
        let scope_entries = read_scope_entries(scope_parent_path).unwrap_or_default();
        Some(Self::sorted_scope_folders(scope_entries, &resolved_current_folder))
    }
    fn sorted_scope_folders(entries: Vec<ScopeEntry>, current_folder: &Path) -> Self {
        let mut sibling_folder_paths: Vec<PathBuf> = entries
            .into_iter()
            .filter(|entry| entry.is_directory)
            .map(|entry| entry.path)
            .filter(|folder_path| folder_path != current_folder)
            .collect();
        sibling_folder_paths.sort_by(|left_path, right_path| {
            compare_entry_names(&left_path.to_string_lossy(), &right_path.to_string_lossy())
        });
        Self {
            sibling_folder_paths,
        }
    }
    fn position_of_current_folder(&self, current_folder_path: &Path) -> Option<usize> {
        let resolved_folder = fs::canonicalize(current_folder_path).ok()?;
        let current_name = resolved_folder.file_name()?.to_string_lossy();
        let preceding_folder_count = self
            .sibling_folder_paths
            .iter()
            .take_while(|listed_path| preceding_folder_name(listed_path.as_path(), &current_name))
            .count();
        Some(preceding_folder_count)
    }
    fn neighbor_at(&self, listed_index: usize) -> ListedFolderNeighbor {
        ListedFolderNeighbor::At {
            listed_index,
            folder_path: self.sibling_folder_paths[listed_index].clone(),
        }
    }
}

fn preceding_folder_name(listed_path: &Path, current_name: &str) -> bool {
    let listed_name = listed_path.file_name().unwrap_or_default().to_string_lossy();
    compare_entry_names(&listed_name, current_name) == Ordering::Less
}

fn read_scope_entries(folder_path: &Path) -> Option<Vec<ScopeEntry>> {
    let dir_entries = fs::read_dir(folder_path).ok()?;
    Some(dir_entries.filter_map(|entry| scope_entry_of(entry.ok()?)).collect())
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
