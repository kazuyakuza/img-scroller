//! Fixed v1 alphanumeric entry-name collation shared by the image and folder
//! listings: compare by lowercase form first, tie-break by exact characters
//! (tech.md -> Horizontal Navigation Mechanism, Wave-2 decision; brief
//! `## Sorting Order`: one fixed order for files AND folders).

use std::cmp::Ordering;

pub(crate) fn compare_entry_names(left_name: &str, right_name: &str) -> Ordering {
    let case_folded_order = left_name.to_lowercase().cmp(&right_name.to_lowercase());
    if case_folded_order != Ordering::Equal {
        return case_folded_order;
    }
    left_name.cmp(right_name)
}

#[cfg(test)]
mod tests {
    use super::compare_entry_names;
    use std::cmp::Ordering;

    #[test]
    fn lowercase_form_decides_order_before_exact_characters() {
        assert_eq!(
            compare_entry_names("alpha.gif", "ALPHA.jpg"),
            Ordering::Less
        );
        assert_eq!(compare_entry_names("alpha.gif", "zeta.jpg"), Ordering::Less);
    }

    #[test]
    fn exact_characters_tie_break_when_lowercase_forms_match() {
        assert_eq!(
            compare_entry_names("Alpha.jpg", "alpha.jpg"),
            Ordering::Less
        );
        assert_eq!(compare_entry_names("ALPHA", "Alpha"), Ordering::Less);
    }
}
