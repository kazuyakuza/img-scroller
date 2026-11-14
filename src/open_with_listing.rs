//! Pure open-with program listing: turns raw system handler names into the
//! menu-list data. The OS-bound enumeration (platform facade) feeds it the
//! handlers' display names in the system's enumeration order; container tests
//! feed synthetic fixtures through the same function.
//!
//! Pinned rules (enforced by the tests module): blank names are dropped, the
//! system enumeration order is preserved verbatim (no re-sort), and display
//! names dedup case-insensitively keeping the FIRST occurrence — its raw
//! input position survives as launch_index. Launch-index contract: the index
//! addresses a FRESH re-enumeration of the same system query, performed by
//! the launcher at hand-off time; handler objects are never persisted and no
//! internal program database exists anywhere. Every launch hand-off failure
//! collapses into OPEN_WITH_LAUNCH_FAILURE_MESSAGE, the single static
//! message (the app's only message surface takes &'static str).
//!
//! The launcher's shell hand-off additionally converts the app's canonical
//! opened-image path to its shell-parseable form (shell_parseable_path):
//! SHCreateItemFromParsingName requires a shell parsing name and rejects the
//! fs::canonicalize `\\?\` verbatim prefix with E_INVALIDARG 0x80070057 (the
//! diagnostic log's Wave 6.2 D1 root-cause evidence); the conversion happens
//! ONLY at that hand-off — the stored canonical path is never rewritten.
//!
//! For future AI agents: mechanism decisions are recorded in
//! .kilo/plans/20261112-wave6-tasks1-2-enumeration-launcher-implementation-plan.md
//! (§1, Decisions 2 and 4); the Wave 6.2 shell_parseable_path is pinned in
//! .kilo/plans/20261113-wave6-2-task2-launcher-fix-implementation-plan.md
//! (§1 D1–D5); the user-facing wave documentation lands in
//! tech.md (Wave 6, Task 5).

use std::path::{Path, PathBuf};

pub(crate) const OPEN_WITH_LAUNCH_FAILURE_MESSAGE: &str =
    "Windows could not open this file with the selected program.";

/// One system-registered program for the current file's type: the name the
/// menu shows plus the launch payload (the handler's position in the raw
/// system enumeration, consumed by the launcher's fresh re-enumeration).
#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) struct RegisteredProgram {
    pub display_name: String,
    pub launch_index: usize,
}

/// Launcher-hand-off failure (Task 2): collapses every launch error into the
/// single static message (the app's only message surface takes &'static str).
#[allow(dead_code)] // consumed by the Task-3 menu UI
pub(crate) enum OpenWithLaunchFailure {
    LaunchFailed,
}

impl OpenWithLaunchFailure {
    #[allow(dead_code)]
    pub(crate) fn user_message(self) -> &'static str {
        OPEN_WITH_LAUNCH_FAILURE_MESSAGE
    }
}

/// Filters blank handler names, dedups display names case-insensitively
/// (first occurrence kept, its raw index survives as launch_index), and
/// preserves the system enumeration order verbatim.
#[allow(dead_code)] // consumed by the Windows-side enumeration (this target: stub path)
pub(crate) fn programs_from_handler_names(handler_names: &[String]) -> Vec<RegisteredProgram> {
    let mut folded_names_seen: Vec<String> = Vec::new();
    let mut programs = Vec::new();
    for (raw_index, raw_name) in handler_names.iter().enumerate() {
        if handler_name_is_blank(raw_name) {
            continue;
        }
        let folded_name = raw_name.to_lowercase();
        if folded_names_seen.contains(&folded_name) {
            continue;
        }
        folded_names_seen.push(folded_name);
        programs.push(RegisteredProgram {
            display_name: raw_name.clone(),
            launch_index: raw_index,
        });
    }
    programs
}

fn handler_name_is_blank(raw_name: &str) -> bool {
    raw_name.trim().is_empty()
}

const VERBATIM_PATH_PREFIX: &str = r"\\?\";
const VERBATIM_UNC_PATH_PREFIX: &str = r"\\?\UNC\";

/// Shell hand-off form of a canonical path: strips the `\\?\` verbatim
/// prefix (the `\\?\UNC\` device form maps to the `\\server\share\...`
/// network form); every other path passes through unchanged.
#[allow(dead_code)] // consumed by the Windows-side launcher (this target: stub path)
pub(crate) fn shell_parseable_path(canonical_path: &Path) -> PathBuf {
    let path_text = canonical_path.as_os_str().to_string_lossy();
    if let Some(network_path) = path_text.strip_prefix(VERBATIM_UNC_PATH_PREFIX) {
        return PathBuf::from(format!(r"\\{network_path}"));
    }
    if let Some(local_path) = path_text.strip_prefix(VERBATIM_PATH_PREFIX) {
        return PathBuf::from(local_path);
    }
    canonical_path.to_path_buf()
}

#[cfg(test)]
mod tests;
