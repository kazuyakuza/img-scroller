//! Windows implementation of the open-with enumeration and launcher (Tasks 1+2):
//! SHAssocEnumHandlers queries the system's registered handlers for the current
//! file's extension; IAssocHandler::GetUIName yields display names (fed to the
//! pure open_with_listing seam) and IAssocHandler::Invoke launches the selected
//! handler with the current file's data object. Compiled only on Windows
//! targets; first-compile validation is the cross-target check step.

use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use windows::Win32::System::Com::{CoTaskMemFree, IDataObject};
use windows::Win32::UI::Shell::{
    ASSOC_FILTER_NONE, IAssocHandler, IEnumAssocHandlers, IShellItem, SHAssocEnumHandlers,
    SHCreateItemFromParsingName, SHCreateShellItemArrayFromShellItem,
};
use windows::core::PCWSTR;

use crate::open_with_listing::{
    OpenWithLaunchFailure, RegisteredProgram, programs_from_handler_names,
};

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn enumerate_open_with_programs(file_path: &Path) -> Vec<RegisteredProgram> {
    raw_handlers(file_path)
        .map(|handlers| programs_from_handler_names(&handler_ui_names(&handlers)))
        .unwrap_or_default()
}

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn open_file_with_selected_program(
    file_path: &Path,
    program_index: usize,
) -> Result<(), OpenWithLaunchFailure> {
    let handlers = raw_handlers(file_path).ok_or(OpenWithLaunchFailure::LaunchFailed)?;
    let handler = handlers
        .get(program_index)
        .ok_or(OpenWithLaunchFailure::LaunchFailed)?;
    let launch_outcome =
        unsafe { file_data_object(file_path).and_then(|data_object| handler.Invoke(&data_object)) };
    launch_outcome.map_err(|_| OpenWithLaunchFailure::LaunchFailed)
}

fn raw_handlers(file_path: &Path) -> Option<Vec<IAssocHandler>> {
    if !super::windows::ensure_com_initialized() {
        return None;
    }
    let query = extension_query_string(file_path)?;
    let enum_handlers =
        unsafe { SHAssocEnumHandlers(PCWSTR(query.as_ptr()), ASSOC_FILTER_NONE) }.ok()?;
    Some(unsafe { drained_handlers(&enum_handlers) })
}

unsafe fn drained_handlers(enum_handlers: &IEnumAssocHandlers) -> Vec<IAssocHandler> {
    unsafe {
        let mut handlers = Vec::new();
        loop {
            let Some(handler) = next_handler(enum_handlers) else {
                break;
            };
            handlers.push(handler);
        }
        handlers
    }
}

unsafe fn next_handler(enum_handlers: &IEnumAssocHandlers) -> Option<IAssocHandler> {
    unsafe {
        let mut buffer = [None::<IAssocHandler>];
        enum_handlers.Next(&mut buffer, None).ok()?; // pceltfetched None is legal when celt == 1
        buffer[0].take()
    }
}

fn handler_ui_names(handlers: &[IAssocHandler]) -> Vec<String> {
    handlers
        .iter()
        .filter_map(|handler| unsafe { handler_ui_name(handler) })
        .collect()
}

unsafe fn handler_ui_name(handler: &IAssocHandler) -> Option<String> {
    unsafe {
        let ui_name = handler.GetUIName().ok()?;
        let text = ui_name.to_string().ok();
        CoTaskMemFree(Some(ui_name.as_ptr().cast::<core::ffi::c_void>()));
        text
    }
}

fn extension_query_string(file_path: &Path) -> Option<Vec<u16>> {
    let extension = file_path.extension()?;
    let mut query: Vec<u16> = Vec::with_capacity(extension.len() + 2);
    query.push(u16::from(b'.'));
    query.extend(extension.encode_wide());
    query.push(0);
    Some(query)
}

unsafe fn file_data_object(file_path: &Path) -> windows::core::Result<IDataObject> {
    unsafe {
        let wide_path: Vec<u16> = file_path.as_os_str().encode_wide().chain([0]).collect();
        let shell_item: IShellItem = SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None)?;
        let data_object: IDataObject = SHCreateShellItemArrayFromShellItem(&shell_item)?;
        Ok(data_object)
    }
}
