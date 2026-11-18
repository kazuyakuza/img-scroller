//! Window-title composition (Wave-8 Task 4): pure, container-testable fitting
//! of the visible image's path into the window title -- full path,
//! name+extension, last-chars truncation, the containing-folder/drive/parent
//! chain with the `...` gap marker -- plus the shared name extraction and the
//! task's pinned constants (segment separator, ellipsis, usable-width
//! fraction, popup seconds). Text width is supplied by an injected measure
//! closure; no egui context is touched here.

use std::path::Path;

use crate::app::APP_WINDOW_TITLE;

use segments::{PathSegments, path_segments};

pub(crate) const PATH_SEGMENT_SEPARATOR: &str = "\\";
pub(crate) const TITLE_ELLIPSIS: &str = "...";
pub(crate) const TITLE_BAR_USABLE_WIDTH_FRACTION: f32 = 0.85;
pub(crate) const FULLSCREEN_NAME_POPUP_VISIBLE_SECONDS: f64 = 3.0;
pub(crate) const FULLSCREEN_NAME_POPUP_EXPIRY_WAKE_MARGIN_SECONDS: f64 = 0.05;

/// Composition request (param object -- the project max-2-args rule).
pub(crate) struct TitleComposition<'m> {
    pub(crate) path: Option<&'m Path>,
    pub(crate) available_width: f32,
    pub(crate) is_maximized: bool,
    pub(crate) measure: &'m mut dyn FnMut(&str) -> f32,
}

/// Compose the title by the Task-4 priority ladder. Maximized: the whole full
/// path if it fits, else the name+ext (last-chars-truncated to fit) grown by
/// FIRST the containing folder, SECOND the drive with a `...` gap between the
/// drive and the visible folders, then parent folders walked up while each still
/// fits. Not-maximized: the name+ext alone. Width is measured through the
/// injected `measure` closure; `path == None` yields the base app title.
pub(crate) fn compose_window_title(request: TitleComposition<'_>) -> String {
    let Some(path) = request.path else {
        return APP_WINDOW_TITLE.to_owned();
    };
    let segments = path_segments(path);
    let mut fitter = WidthFitter {
        max_width: request.available_width,
        measure: request.measure,
    };
    if !request.is_maximized {
        return fitter.fit_or_truncate_last(&segments.file_name);
    }
    let mut composer = ChainComposer::new(&segments, &mut fitter);
    let full_chain = composer.full_chain();
    if composer.fitter.fits(&full_chain) {
        return full_chain;
    }
    composer.base = composer.fitter.fit_or_truncate_last(&segments.file_name);
    if composer.containing_folder_fits() {
        composer.visible_folder_count = 1;
    }
    if composer.drive_step_fits() {
        composer.drive_visible = true;
    }
    while composer.should_add_next_parent() {
        composer.visible_folder_count += 1;
    }
    composer.candidate()
}

/// The name+extension form (the last path segment) -- the single source shared
/// by the title composition, the not-maximized rule, and the fullscreen popup.
pub(crate) fn file_name_with_extension(path: &Path) -> String {
    path_segments(path).file_name
}

struct WidthFitter<'m> {
    max_width: f32,
    measure: &'m mut dyn FnMut(&str) -> f32,
}

impl WidthFitter<'_> {
    fn fits(&mut self, text: &str) -> bool {
        (self.measure)(text) <= self.max_width
    }

    fn fit_or_truncate_last(&mut self, text: &str) -> String {
        if self.fits(text) {
            return text.to_owned();
        }
        for (start_index, _) in text.char_indices() {
            let candidate = format!("{TITLE_ELLIPSIS}{}", &text[start_index..]);
            if self.fits(&candidate) {
                return candidate;
            }
        }
        TITLE_ELLIPSIS.to_owned()
    }
}

struct ChainComposer<'s, 'f, 'm> {
    segments: &'s PathSegments,
    base: String,
    visible_folder_count: usize,
    drive_visible: bool,
    fitter: &'f mut WidthFitter<'m>,
}

impl<'s, 'f, 'm> ChainComposer<'s, 'f, 'm> {
    fn new(segments: &'s PathSegments, fitter: &'f mut WidthFitter<'m>) -> Self {
        Self {
            segments,
            base: segments.file_name.clone(),
            visible_folder_count: 0,
            drive_visible: false,
            fitter,
        }
    }

    fn full_chain(&self) -> String {
        self.candidate_with(self.segments.folders.len(), true)
    }

    fn candidate(&self) -> String {
        self.candidate_with(self.visible_folder_count, self.drive_visible)
    }

    fn candidate_with(&self, visible_folder_count: usize, drive_visible: bool) -> String {
        let mut parts: Vec<String> = Vec::new();
        let drive_is_included = drive_visible && self.segments.drive.is_some();
        if drive_is_included {
            parts.extend(self.segments.drive.clone());
        }
        let omitted_folder_count = self
            .segments
            .folders
            .len()
            .saturating_sub(visible_folder_count);
        if ellipsis_marker_is_needed(drive_is_included, omitted_folder_count) {
            parts.push(TITLE_ELLIPSIS.to_owned());
        }
        let visible_folder_count = visible_folder_count.min(self.segments.folders.len());
        let visible_start = self.segments.folders.len() - visible_folder_count;
        parts.extend(self.segments.folders[visible_start..].iter().cloned());
        parts.push(self.base.to_owned());
        parts.join(PATH_SEGMENT_SEPARATOR)
    }

    fn containing_folder_fits(&mut self) -> bool {
        if self.segments.folders.is_empty() {
            return false;
        }
        let candidate = self.candidate_with(1, false);
        self.fitter.fits(&candidate)
    }

    fn drive_step_fits(&mut self) -> bool {
        if self.segments.drive.is_none() {
            return false;
        }
        let candidate = self.candidate_with(self.visible_folder_count, true);
        self.fitter.fits(&candidate)
    }

    fn next_parent_fits(&mut self) -> bool {
        let candidate = self.candidate_with(self.visible_folder_count + 1, self.drive_visible);
        self.fitter.fits(&candidate)
    }

    fn parent_walk_is_bounded(&self) -> bool {
        self.visible_folder_count < self.segments.folders.len()
    }

    fn should_add_next_parent(&mut self) -> bool {
        if !self.parent_walk_is_bounded() {
            return false;
        }
        self.next_parent_fits()
    }
}

fn ellipsis_marker_is_needed(drive_is_included: bool, omitted_folder_count: usize) -> bool {
    drive_is_included && omitted_folder_count > 0
}

mod segments;

#[cfg(test)]
mod tests;
