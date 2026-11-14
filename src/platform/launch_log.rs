//! OPTIONAL Wave-6.2 Task-1 diagnostic (caller-authorized 2026-11-13): records
//! the runtime evidence of each "Open with..." launch into a plain `.log` file
//! so the operator can report it back. Compiled ONLY when BOTH the `windows`
//! target AND the `diagnostic-launch-log` Cargo feature are enabled; the release
//! build never enables the feature, so the shipped artifact contains none of
//! this code. The log file is created lazily on the first diagnostic write and
//! appended to on every later write; any write failure (or an unresolvable
//! target) is silently skipped — this module must never alter app behavior or
//! panic. Task 2's root-cause consumes the resulting log.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::open_with_listing::RegisteredProgram;

const LOG_FILE_NAME: &str = "img-scroller-diagnostic.log";

static HEADER_WRITTEN: AtomicBool = AtomicBool::new(false);

pub(crate) fn enumeration_stored(programs: &[RegisteredProgram]) {
    let entries = programs
        .iter()
        .enumerate()
        .map(|(position, program)| {
            format!(
                "pos={position} launch_index={} \"{}\"",
                program.launch_index, program.display_name
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    emit(&format!(
        "stored-enumeration: count={} entries=[{entries}]",
        programs.len()
    ));
}

pub(crate) fn attempt_started(file_path: &Path, menu_position: usize) {
    emit(&format!(
        "--- attempt: file=\"{}\" menu_position={menu_position} unix={}",
        file_path.display(),
        unix_epoch_seconds()
    ));
}

pub(crate) fn fresh_enumeration_failed() {
    emit("fresh-enumeration: FAILED (raw_handlers returned None)");
}

pub(crate) fn fresh_enumeration_drained(handler_count: usize, handler_names: &[String]) {
    emit(&format!(
        "fresh-enumeration: handler_count={handler_count} names=[{}]",
        quoted_names(handler_names)
    ));
}

pub(crate) fn launch_index_resolved(menu_position: usize, resolved: bool) {
    emit(&format!(
        "launch-index: menu_position={menu_position} resolved={resolved}"
    ));
}

pub(crate) fn shell_item_created(created: bool, error_code: Option<i32>) {
    emit(&format!(
        "shcreateitemfromparsingname: created={created} code={}",
        error_code_text(error_code)
    ));
}

pub(crate) fn data_object_bound(bound: bool, error_code: Option<i32>) {
    emit(&format!(
        "bindtohandler-bhid_dataobject: bound={bound} code={}",
        error_code_text(error_code)
    ));
}

pub(crate) fn launch_finished(succeeded: bool, error_code: Option<i32>) {
    let classified_name = if succeeded { "Success" } else { "LaunchFailed" };
    emit(&format!(
        "launch-final: succeeded={succeeded} code={} classified={classified_name}",
        error_code_text(error_code)
    ));
}

fn emit(line: &str) {
    if let Some(header) = header_line_if_first_write() {
        append_line(&header);
    }
    append_line(line);
}

fn append_line(line: &str) {
    if append_in_directory(current_exe_directory(), line) {
        return;
    }
    append_in_directory(Some(std::env::temp_dir()), line);
}

fn append_in_directory(directory: Option<PathBuf>, line: &str) -> bool {
    let Some(directory) = directory else {
        return false;
    };
    let Ok(mut log_file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(directory.join(LOG_FILE_NAME))
    else {
        return false;
    };
    writeln!(log_file, "{line}").is_ok()
}

fn current_exe_directory() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()?
        .parent()
        .map(Path::to_path_buf)
}

fn header_line_if_first_write() -> Option<String> {
    if HEADER_WRITTEN
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return None;
    }
    Some(format!(
        "=== img-scroller-diagnostic v{} profile={} started_unix={} ===",
        env!("CARGO_PKG_VERSION"),
        profile_name(),
        unix_epoch_seconds()
    ))
}

fn profile_name() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

fn unix_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0)
}

fn error_code_text(error_code: Option<i32>) -> String {
    match error_code {
        Some(code) => format!("0x{:08X}", code as u32),
        None => "none".to_string(),
    }
}

fn quoted_names(names: &[String]) -> String {
    names
        .iter()
        .map(|name| format!("\"{name}\""))
        .collect::<Vec<_>>()
        .join(", ")
}
