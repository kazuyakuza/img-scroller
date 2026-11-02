//! Painted-arrow types of the floating navigation pill: the triangle paint
//! direction in the `egui::Shape::rotated_triangle` rotation space, and the
//! navigation control each pill button represents.

use super::NavigationDirection;

/// Painted triangle direction. `Shape::rotated_triangle` paints 0.0 down,
/// a quarter turn left, half a turn up, minus a quarter turn right.
pub(super) enum ArrowDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ArrowDirection {
    pub(super) fn paint_rotation(self) -> f32 {
        match self {
            Self::Up => std::f32::consts::TAU / 2.0,
            Self::Down => 0.0,
            Self::Left => std::f32::consts::TAU / 4.0,
            Self::Right => -std::f32::consts::TAU / 4.0,
        }
    }
}

/// Which navigation control a floating-pill button represents. The image
/// arrows step sibling images and the folder arrows step the sibling folders
/// under the same parent; both axes share the keyboard navigation seam.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum ArrowControl {
    ImagePrevious,
    ImageNext,
    FolderUp,
    FolderDown,
}

impl ArrowControl {
    pub(super) fn painted_direction(self) -> ArrowDirection {
        match self {
            Self::ImagePrevious => ArrowDirection::Left,
            Self::ImageNext => ArrowDirection::Right,
            Self::FolderUp => ArrowDirection::Up,
            Self::FolderDown => ArrowDirection::Down,
        }
    }

    pub(super) fn accessible_label(self) -> &'static str {
        match self {
            Self::ImagePrevious => "Previous image",
            Self::ImageNext => "Next image",
            Self::FolderUp => "Previous sibling folder",
            Self::FolderDown => "Next sibling folder",
        }
    }

    /// Concrete direction per button; all four arrows navigate.
    pub(super) fn navigation_direction(self) -> Option<NavigationDirection> {
        match self {
            Self::ImagePrevious => Some(NavigationDirection::Previous),
            Self::ImageNext => Some(NavigationDirection::Next),
            Self::FolderUp => Some(NavigationDirection::PreviousSiblingFolder),
            Self::FolderDown => Some(NavigationDirection::NextSiblingFolder),
        }
    }
}
