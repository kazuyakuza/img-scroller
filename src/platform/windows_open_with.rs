//! Windows implementation of the open-with enumeration and launcher (Tasks 1+2):
//! SHAssocEnumHandlers queries the system's registered handlers for the current
//! file's extension with ASSOC_FILTER_NONE — no app-side filter, so UWP/Store
//! apps are included by design; IEnumAssocHandlers is drained into the raw
//! handler list and IAssocHandler::GetUIName yields display names (fed to the
//! pure open_with_listing seam). The launcher re-enumerates FRESH per call —
//! launch_index addresses that fresh list; nothing is cached or persisted
//! across calls — and launches the selected handler via IAssocHandler::Invoke
//! with an IDataObject built from the current canonical file — converted first
//! to its shell-parseable form (`fs::canonicalize`'s \?\ verbatim prefix is
//! not a shell parsing name; `SHCreateItemFromParsingName` rejects it with
//! E_INVALIDARG — the Wave 6.2 D1 root cause) via
//! IShellItem::BindToHandler(BHID_DataObject) — the documented way to obtain
//! an item's data object; the removed SHCreateShellItemArrayFromShellItem
//! route requested IID_IDataObject off-contract from a shell item array and
//! failed on real Windows (Wave 6.1 D1 root cause — see
//! .kilo/plans/20261113-wave6-1-task1-launcher-fix-implementation-plan.md
//! §0/§1). COM init reuses
//! the shared ensure_com_initialized discipline from windows.rs; every failure
//! path returns the empty/default value, never panics. Compiled only on
//! Windows targets; first-compile validation is the cross-target check step.
//! Each launch path also carries cfg-gated Wave-6.2 diagnostic hooks calling
//! `super::launch_log`, compiled only when the `diagnostic-launch-log` feature
//! is enabled — OFF by default, so normal builds and the release artifact see
//! none of them.
//! For future AI agents: mechanism decisions are recorded in
//! .kilo/plans/20261112-wave6-tasks1-2-enumeration-launcher-implementation-plan.md
//! (§1, Decisions 1–3); the user-facing wave documentation lands in tech.md
//! (Wave 6, Task 5).

use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use windows::Win32::System::Com::{CoTaskMemFree, IBindCtx, IDataObject};
use windows::Win32::UI::Shell::{
    ASSOC_FILTER_NONE, BHID_DataObject, IAssocHandler, IEnumAssocHandlers, IShellItem,
    SHAssocEnumHandlers, SHCreateItemFromParsingName,
};
use windows::core::PCWSTR;

use crate::open_with_listing::{
    OpenWithLaunchFailure, RegisteredProgram, programs_from_handler_names, shell_parseable_path,
};

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn enumerate_open_with_programs(file_path: &Path) -> Vec<RegisteredProgram> {
    let programs = raw_handlers(file_path)
        .map(|handlers| programs_from_handler_names(&handler_ui_names(&handlers)))
        .unwrap_or_default();
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::enumeration_stored(&programs);
    programs
}

#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) fn open_file_with_selected_program(
    file_path: &Path,
    program_index: usize,
) -> Result<(), OpenWithLaunchFailure> {
    let handlers = match raw_handlers(file_path) {
        Some(handlers) => handlers,
        None => {
            #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
            super::launch_log::fresh_enumeration_failed();
            return Err(OpenWithLaunchFailure::LaunchFailed);
        }
    };
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::attempt_started(file_path, program_index);
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::fresh_enumeration_drained(handlers.len(), &handler_ui_names(&handlers));
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::launch_index_resolved(program_index, handlers.get(program_index).is_some());
    let handler = handlers
        .get(program_index)
        .ok_or(OpenWithLaunchFailure::LaunchFailed)?;
    let launch_outcome =
        unsafe { file_data_object(file_path).and_then(|data_object| handler.Invoke(&data_object)) };
    #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
    super::launch_log::launch_finished(launch_outcome.is_ok(), error_code_of(&launch_outcome));
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
        let shell_path = shell_parseable_path(file_path);
        let wide_path: Vec<u16> = shell_path.as_os_str().encode_wide().chain([0]).collect();
        let shell_item: windows::core::Result<IShellItem> =
            SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None);
        #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
        super::launch_log::shell_item_created(shell_item.is_ok(), error_code_of(&shell_item));
        let shell_item = shell_item?;
        let bind_outcome: windows::core::Result<IDataObject> =
            shell_item.BindToHandler(None::<&IBindCtx>, &BHID_DataObject);
        #[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
        super::launch_log::data_object_bound(bind_outcome.is_ok(), error_code_of(&bind_outcome));
        Ok(bind_outcome?)
    }
}

#[cfg(all(target_os = "windows", feature = "diagnostic-launch-log"))]
fn error_code_of<T>(outcome: &windows::core::Result<T>) -> Option<i32> {
    outcome.as_ref().err().map(|error| error.code().0)
}
