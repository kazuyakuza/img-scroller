use super::{RegisteredProgram, programs_from_handler_names, shell_parseable_path};
use std::path::{Path, PathBuf};

fn names_and_indices(programs: &[RegisteredProgram]) -> Vec<(&str, usize)> {
    programs
        .iter()
        .map(|program| (program.display_name.as_str(), program.launch_index))
        .collect()
}

fn display_names(programs: &[RegisteredProgram]) -> Vec<&str> {
    programs
        .iter()
        .map(|program| program.display_name.as_str())
        .collect()
}

#[test]
fn empty_input_yields_no_programs() {
    let programs = programs_from_handler_names(&Vec::new());
    assert!(programs.is_empty());
}

#[test]
fn handler_names_map_to_programs_in_system_order() {
    let handler_names = vec![
        "Zed Viewer".to_string(),
        "Alpha Paint".to_string(),
        "Photos".to_string(),
    ];
    let programs = programs_from_handler_names(&handler_names);
    assert_eq!(
        display_names(&programs),
        ["Zed Viewer", "Alpha Paint", "Photos"]
    );
}

#[test]
fn launch_index_records_the_raw_enumeration_position() {
    let handler_names = vec!["B".to_string(), String::new(), "C".to_string()];
    let programs = programs_from_handler_names(&handler_names);
    assert_eq!(names_and_indices(&programs), [("B", 0), ("C", 2)]);
}

#[test]
fn blank_handler_names_are_dropped() {
    let handler_names = vec![
        "Real".to_string(),
        String::new(),
        "   ".to_string(),
        "\t".to_string(),
    ];
    let programs = programs_from_handler_names(&handler_names);
    assert_eq!(names_and_indices(&programs), [("Real", 0)]);
}

#[test]
fn duplicate_display_names_dedup_case_insensitively_keeping_first() {
    let handler_names = vec![
        "Paint".to_string(),
        "Viewer".to_string(),
        "PAINT".to_string(),
        "paint".to_string(),
    ];
    let programs = programs_from_handler_names(&handler_names);
    assert_eq!(names_and_indices(&programs), [("Paint", 0), ("Viewer", 1)]);
}

#[test]
fn non_ascii_display_names_are_preserved_verbatim() {
    let handler_names = vec!["写真ビューア".to_string(), "Photos".to_string()];
    let programs = programs_from_handler_names(&handler_names);
    assert_eq!(display_names(&programs), ["写真ビューア", "Photos"]);
}

#[test]
fn verbatim_prefixed_local_path_loses_the_prefix() {
    let canonical_path = Path::new(r"\\?\C:\_kazuya\imgs\Chino_Kafuu__BOOTH_\Chino Kafuu 1.png");
    assert_eq!(
        shell_parseable_path(canonical_path),
        PathBuf::from(r"C:\_kazuya\imgs\Chino_Kafuu__BOOTH_\Chino Kafuu 1.png")
    );
}

#[test]
fn verbatim_unc_path_maps_to_the_network_share_form() {
    let canonical_path = Path::new(r"\\?\UNC\server\share\folder\f.png");
    assert_eq!(
        shell_parseable_path(canonical_path),
        PathBuf::from(r"\\server\share\folder\f.png")
    );
}

#[test]
fn unprefixed_local_path_passes_through_unchanged() {
    let canonical_path = Path::new(r"C:\x.png");
    assert_eq!(
        shell_parseable_path(canonical_path),
        PathBuf::from(r"C:\x.png")
    );
}

#[test]
fn plain_unc_path_passes_through_unchanged() {
    let canonical_path = Path::new(r"\\server\share\f.png");
    assert_eq!(
        shell_parseable_path(canonical_path),
        PathBuf::from(r"\\server\share\f.png")
    );
}

#[test]
fn relative_path_passes_through_unchanged() {
    let canonical_path = Path::new("photo.png");
    assert_eq!(
        shell_parseable_path(canonical_path),
        PathBuf::from("photo.png")
    );
}

#[test]
fn verbatim_prefix_conversion_preserves_case_and_non_ascii_names() {
    let canonical_path = Path::new(r"\\?\C:\智乃\Chino Kafuu 1.PNG");
    assert_eq!(
        shell_parseable_path(canonical_path),
        PathBuf::from(r"C:\智乃\Chino Kafuu 1.PNG")
    );
}
