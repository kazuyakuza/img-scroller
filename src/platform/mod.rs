//! OS-bound facade for the native open dialog and image decoding.
//! Both target sides expose the same signatures; see the Task-2 plan §1.1-h.

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
mod stub;

#[cfg(target_os = "windows")]
pub(crate) use windows::{decode_image, show_open_image_dialog};

#[cfg(not(target_os = "windows"))]
pub(crate) use stub::{decode_image, show_open_image_dialog};
