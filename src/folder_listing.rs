//! Folder listing for horizontal image navigation: the file-sibling images of
//! one folder only -- non-recursive, whitelisted-extension-filtered, and
//! sorted by the fixed v1 alphanumeric collation (folded name + exact tie-break).

use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

const IMAGE_FILE_EXTENSIONS: [&str; 17] = [
    "avif", "bmp", "dib", "gif", "heic", "heif", "ico", "jfif", "jpe", "jpeg", "jpg", "jxr", "png",
    "tif", "tiff", "webp", "wdp",
];

pub(crate) struct FolderListing {
    image_paths: Vec<PathBuf>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ListedNeighbor {
    At {
        #[allow(dead_code)]
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
        let image_paths = Self::collect_folder_images(&resolved_folder)?;
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

    #[allow(dead_code)]
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

#[cfg(test)]
mod tests;
