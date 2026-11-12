//! Windows collation helper behind the platform facade's system entry-name
//! order: the system file manager's logical name comparison (`StrCmpLogicalW`,
//! shlwapi) -- digit-aware natural ordering (1, 2, ..., 9, 10, 11 ...) and
//! case-insensitive; the comparison never fails.

use std::cmp::Ordering;

use windows::Win32::UI::Shell::StrCmpLogicalW;
use windows::core::PCWSTR;

pub(super) fn system_collation_order(left_name: &str, right_name: &str) -> Ordering {
    let left_units: Vec<u16> = left_name.encode_utf16().chain([0]).collect();
    let right_units: Vec<u16> = right_name.encode_utf16().chain([0]).collect();
    let result =
        unsafe { StrCmpLogicalW(PCWSTR(left_units.as_ptr()), PCWSTR(right_units.as_ptr())) };
    result.cmp(&0)
}
