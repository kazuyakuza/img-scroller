//! Windows collation helpers behind the platform facade's system entry-name
//! order: the user-default locale's `CompareStringEx` result compared
//! case-insensitively (`NORM_IGNORECASE`) and mapped to a standard order,
//! with any system-call failure surfaced as `None`.

use std::cmp::Ordering;

use windows::Win32::Globalization::{
    COMPARESTRING_RESULT, CSTR_EQUAL, CSTR_GREATER_THAN, CSTR_LESS_THAN, CompareStringEx,
    GetUserDefaultLocaleName, NORM_IGNORECASE,
};
use windows::Win32::System::SystemServices::LOCALE_NAME_MAX_LENGTH;
use windows::core::PCWSTR;

pub(super) fn system_collation_order(left_name: &str, right_name: &str) -> Option<Ordering> {
    let locale_name = user_locale_name()?;
    let left_units: Vec<u16> = left_name.encode_utf16().collect();
    let right_units: Vec<u16> = right_name.encode_utf16().collect();
    let result = unsafe {
        CompareStringEx(
            PCWSTR(locale_name.as_ptr()),
            NORM_IGNORECASE,
            &left_units,
            &right_units,
            None,
            None,
            None,
        )
    };
    collation_result_order(result)
}

fn user_locale_name() -> Option<Vec<u16>> {
    let mut buffer = [0u16; LOCALE_NAME_MAX_LENGTH as usize];
    let written = unsafe { GetUserDefaultLocaleName(&mut buffer) };
    if written <= 0 {
        return None;
    }
    Some(buffer[..written as usize].to_vec())
}

fn collation_result_order(result: COMPARESTRING_RESULT) -> Option<Ordering> {
    match result {
        CSTR_LESS_THAN => Some(Ordering::Less),
        CSTR_EQUAL => Some(Ordering::Equal),
        CSTR_GREATER_THAN => Some(Ordering::Greater),
        _ => None,
    }
}
