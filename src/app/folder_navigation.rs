//! Shared navigation seam for the arrow keys and the pill buttons: the ±1
//! neighbor path per `NavigationDirection` -- the sibling-image listing of the
//! current folder on the horizontal axis, the ADJACENT sibling folder under
//! the same parent on the vertical axis. Vertical navigation ALWAYS moves to
//! the adjacent folder; its resolvable target image opens through the normal
//! open flow, otherwise empty-folder browsing mode is entered. No neighbor is
//! skipped, no wrap. The image-versus-empty decision happens ONLY in
//! `navigate`: the enablement path sees folder adjacency alone.

use crate::folder_listing::{FolderListing, ListedNeighbor};
use crate::sibling_folder_listing::{ListedFolderNeighbor, SiblingFolderListing};
use std::path::{Path, PathBuf};

use super::ImagesScrollerApp;

/// Placeholder file name a listing anchor always carries: the anchor's PARENT
/// folder is the real target; the name itself is never read (the Task-1 plan
/// § 7 trick, reused for the first-image probe and the empty-mode re-anchor).
const LISTING_ANCHOR_FILE_PLACEHOLDER: &str = "listing_anchor.bin";

/// Arrow directions of the shared navigation seam: the image axis steps the
/// current folder's sibling images; the folder axis steps the sibling folders
/// under the same parent.
pub(super) enum NavigationDirection {
    Previous,
    Next,
    PreviousSiblingFolder,
    NextSiblingFolder,
}

impl ImagesScrollerApp {
    /// Navigates one step per direction: horizontal neighbors open their image
    /// path; vertical neighbors open the adjacent folder's target image or,
    /// when the folder yields none, enter empty-folder mode (D2-final).
    pub(super) fn navigate(&mut self, direction: NavigationDirection) {
        let is_folder_axis = matches!(
            direction,
            NavigationDirection::PreviousSiblingFolder | NavigationDirection::NextSiblingFolder
        );
        let Some(neighbor_path) = self.neighbor_image_path(direction) else {
            return;
        };
        if is_folder_axis {
            self.enter_sibling_folder(neighbor_path);
        } else {
            self.open_image(neighbor_path);
        }
    }

    /// The ±1 neighbor path per direction: the sibling IMAGE path on the
    /// horizontal axis; the adjacent sibling FOLDER path on the vertical axis
    /// (the priority-chain and empty-folder decisions apply only in
    /// `navigate`). Every "nothing in this direction" case (no image or
    /// browsed folder, no listing, unresolvable anchor, reached list edge)
    /// resolves to None. Also drives the per-frame pill enablement.
    pub(super) fn neighbor_image_path(&self, direction: NavigationDirection) -> Option<PathBuf> {
        match direction {
            NavigationDirection::Previous | NavigationDirection::Next => {
                listed_image_neighbor(self, direction)
            }
            NavigationDirection::PreviousSiblingFolder | NavigationDirection::NextSiblingFolder => {
                sibling_folder_neighbor_path(self, direction)
            }
        }
    }

    /// The floating pill renders while an image is open, or while an
    /// empty-folder context is being browsed (D2-final); only the fresh empty
    /// state (neither) falls back to the lone center Open button.
    pub(super) fn navigation_context_is_active(&self) -> bool {
        self.image_is_open() || self.browsed_folder_context.is_some()
    }

    /// Target-image priority (TODO Task 3, binding): visited this session -> its
    /// recorded last-viewed image; otherwise ALWAYS the folder's FIRST whitelisted
    /// image in the fixed collation. NO same-filename attempt, NO other fallback;
    /// an image-less folder resolves to None (the caller enters empty-folder mode).
    /// (pub(super) for the locked priority tests; internal caller: enter_sibling_folder.)
    pub(super) fn sibling_folder_target_image_path(&self, folder_path: &Path) -> Option<PathBuf> {
        if let Some(last_viewed) = self.last_viewed_images.last_displayed_image(folder_path) {
            return Some(last_viewed.to_path_buf());
        }
        first_image_path_of_folder(folder_path)
    }

    /// The vertical open decision: the folder's target image opens via the
    /// normal open flow (case i); anything else enters empty-folder mode for
    /// the folder (case ii). Never skips past a folder, never wraps.
    fn enter_sibling_folder(&mut self, folder_path: PathBuf) {
        match self.sibling_folder_target_image_path(&folder_path) {
            Some(target_image) => self.open_image(target_image),
            None => self.enter_empty_folder_mode(folder_path),
        }
    }

    /// Empty-folder mode (D2-final): the browsed folder replaces the open
    /// image -- opened path, decoded payload, horizontal listing and message
    /// released; the vertical sibling list re-anchors ON the browsed folder by
    /// passing `folder.join(placeholder)` verbatim to the Task-1 API; no
    /// last-viewed entry is recorded. (A vanished browsed folder yields an
    /// empty listing: both vertical arrows disable, no message.)
    fn enter_empty_folder_mode(&mut self, folder_path: PathBuf) {
        let anchor = folder_path.join(LISTING_ANCHOR_FILE_PLACEHOLDER);
        self.browsed_folder_context = Some(folder_path);
        self.opened_image_path = None;
        self.decoded_image = None;
        self.message_text = None;
        self.current_folder_listing = None;
        self.current_sibling_folders = SiblingFolderListing::from_opened_image(anchor.as_path());
    }
}

fn listed_image_neighbor(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> Option<PathBuf> {
    let opened_path = app.opened_image_path.as_ref()?;
    let listing = app.current_folder_listing.as_ref()?;
    let listed_index = listing.listed_index_of(opened_path)?;
    let neighbor = match direction {
        NavigationDirection::Previous => listing.previous_neighbor(listed_index),
        NavigationDirection::Next => listing.next_neighbor(listed_index),
        NavigationDirection::PreviousSiblingFolder | NavigationDirection::NextSiblingFolder => {
            return None;
        }
    };
    let ListedNeighbor::At { image_path, .. } = neighbor else {
        return None;
    };
    Some(image_path)
}

/// The ADJACENT sibling FOLDER per direction from the vertical listing; the
/// anchor is passed VERBATIM to the Task-1 API (the open image in image mode;
/// the browsed empty folder plus the placeholder name in empty-folder mode --
/// the API canonicalizes internally, no caller canonicalization). List edges
/// resolve to None (D3); a missing listing resolves to None.
fn sibling_folder_neighbor_path(
    app: &ImagesScrollerApp,
    direction: NavigationDirection,
) -> Option<PathBuf> {
    let listing = app.current_sibling_folders.as_ref()?;
    let anchor_path = vertical_anchor_path(app)?;
    let folder_neighbor = match direction {
        NavigationDirection::PreviousSiblingFolder => listing.previous_neighbor_of(&anchor_path),
        NavigationDirection::NextSiblingFolder => listing.next_neighbor_of(&anchor_path),
        NavigationDirection::Previous | NavigationDirection::Next => return None,
    }?;
    let ListedFolderNeighbor::At { folder_path, .. } = folder_neighbor else {
        return None;
    };
    Some(folder_path)
}

/// The vertical listing anchor: the open image verbatim (image mode) or the
/// browsed empty folder with the placeholder name (empty-folder mode). Image
/// mode wins if both were set (kept impossible by the atomic transitions).
fn vertical_anchor_path(app: &ImagesScrollerApp) -> Option<PathBuf> {
    if let Some(opened_path) = app.opened_image_path.as_ref() {
        return Some(opened_path.clone());
    }
    let browsed_folder = app.browsed_folder_context.as_ref()?;
    Some(browsed_folder.join(LISTING_ANCHOR_FILE_PLACEHOLDER))
}

fn first_image_path_of_folder(target_folder_path: &Path) -> Option<PathBuf> {
    let placeholder_path = target_folder_path.join(LISTING_ANCHOR_FILE_PLACEHOLDER);
    let folder_listing = FolderListing::from_opened_image(&placeholder_path)?;
    folder_listing.first_listed_image()
}
