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

pub(crate) type EntryNameCompare = fn(&str, &str) -> Ordering;

/// The active entry-name order carried as data: `system()` resolves through
/// the platform facade (the system collation on Windows, the pure fallback
/// elsewhere); `with_comparator` lets container tests inject synthetic
/// comparators to prove the listing modules sort through the seam.
#[derive(Clone, Copy)]
pub(crate) struct EntryNameOrder {
    entry_name_compare: EntryNameCompare,
}

impl EntryNameOrder {
    pub(crate) fn system() -> Self {
        Self {
            entry_name_compare: crate::platform::compare_entry_names_system,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn with_comparator(entry_name_compare: EntryNameCompare) -> Self {
        Self { entry_name_compare }
    }
    pub(crate) fn compare(&self, left_name: &str, right_name: &str) -> Ordering {
        (self.entry_name_compare)(left_name, right_name)
    }
}

#[cfg(test)]
mod tests {
    use super::{EntryNameOrder, compare_entry_names};
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

    #[test]
    fn system_order_matches_the_pure_fallback_for_ascii_names() {
        let entry_order = EntryNameOrder::system();
        assert_eq!(entry_order.compare("alpha.gif", "ALPHA.jpg"), Ordering::Less);
        assert_eq!(
            entry_order.compare("zeta.jpg", "alpha.gif"),
            Ordering::Greater
        );
        assert_eq!(entry_order.compare("ALPHA", "Alpha"), Ordering::Less);
    }

    #[test]
    fn injected_comparator_decides_the_entry_order() {
        fn reversed_entry_names(left_name: &str, right_name: &str) -> Ordering {
            compare_entry_names(right_name, left_name)
        }
        let entry_order = EntryNameOrder::with_comparator(reversed_entry_names);
        assert_eq!(entry_order.compare("a", "b"), Ordering::Greater);
        assert_eq!(entry_order.compare("b", "a"), Ordering::Less);
    }
}
