use super::{RegisteredProgram, programs_from_handler_names};

fn names_and_indices(programs: &[RegisteredProgram]) -> Vec<(&str, usize)> {
    programs
        .iter()
        .map(|program| (program.display_name.as_str(), program.launch_index))
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
    let display_names: Vec<&str> = programs
        .iter()
        .map(|program| program.display_name.as_str())
        .collect();
    assert_eq!(display_names, ["Zed Viewer", "Alpha Paint", "Photos"]);
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
    let display_names: Vec<&str> = programs
        .iter()
        .map(|program| program.display_name.as_str())
        .collect();
    assert_eq!(display_names, ["写真ビューア", "Photos"]);
}
