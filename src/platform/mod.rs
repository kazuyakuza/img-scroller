//! OS-bound facade for the native open dialog, image decoding and the system
//! entry-name order. It additionally re-exports the open-with enumeration and
//! launcher pair. Both target sides expose the same signatures; see the
//! Task-2 plan §1.1-h.

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
mod windows_collation;

#[cfg(target_os = "windows")]
mod windows_open_with;

#[cfg(not(target_os = "windows"))]
mod stub;

#[cfg(target_os = "windows")]
pub(crate) use windows::{decode_image, show_open_image_dialog};

#[cfg(not(target_os = "windows"))]
pub(crate) use stub::{decode_image, show_open_image_dialog};

#[cfg(target_os = "windows")]
pub(crate) use windows::compare_entry_names_system;

#[cfg(not(target_os = "windows"))]
pub(crate) use stub::compare_entry_names_system;

// Consumed by the Task-3 menu UI (src/app/open_with_button.rs).
#[cfg(target_os = "windows")]
pub(crate) use windows_open_with::{enumerate_open_with_programs, open_file_with_selected_program};

#[cfg(not(target_os = "windows"))]
pub(crate) use stub::{enumerate_open_with_programs, open_file_with_selected_program};
