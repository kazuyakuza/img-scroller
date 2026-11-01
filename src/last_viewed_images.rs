//! Session last-viewed memory: the image last successfully displayed for
//! every folder visited this session. RAM-only -- created empty with the app,
//! updated at the open-success transition, and destroyed with the application
//! struct; v1 has no disk persistence.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) struct LastViewedImages {
    folder_to_image_paths: HashMap<PathBuf, PathBuf>,
}

impl LastViewedImages {
    pub(crate) fn empty() -> Self {
        Self {
            folder_to_image_paths: HashMap::new(),
        }
    }

    /// Records `image_path` as its folder's last displayed image. The key is
    /// the opened path's parent folder (plain `parent()`, never canonicalized
    /// -- Task 3 looks up with the same folder-path derivation).
    pub(crate) fn record_displayed_image(&mut self, image_path: &Path) {
        let folder_path = image_path.parent().unwrap_or(Path::new(""));
        self.folder_to_image_paths
            .insert(folder_path.to_path_buf(), image_path.to_path_buf());
    }

    /// The session's last displayed image of `folder_path`; `None` before the
    /// folder was visited. Task 3's target-priority lookup calls this.
    #[allow(dead_code)]
    pub(crate) fn last_displayed_image(&self, folder_path: &Path) -> Option<&Path> {
        self.folder_to_image_paths.get(folder_path).map(PathBuf::as_path)
    }

    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.folder_to_image_paths.is_empty()
    }
}
