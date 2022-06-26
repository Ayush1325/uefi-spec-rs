//! This module contains functions related to SimpleTextOutput Protocol

use crate::efi::{Boolean, SystemTable};
use crate::{errors, helpers};
use r_efi::protocols::simple_text_output;

pub type Result<T> = core::result::Result<T, errors::StatusNullError>;

/// Call `Reset` function from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn reset(st: *mut SystemTable, extended_verification: bool) -> Result<()> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;

    let reset_ptr = unsafe { (*conn_out_protocol).reset };

    let status = (reset_ptr)(conn_out_protocol, Boolean::from(extended_verification));

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call `OutputString` function from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn output_string(st: *mut SystemTable, string: &mut [u16]) -> Result<()> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;

    let output_string_ptr = unsafe { (*conn_out_protocol).output_string };

    let status = (output_string_ptr)(conn_out_protocol, string.as_mut_ptr());

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call `TestString` function from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn test_string(st: *mut SystemTable, string: &mut [u16]) -> Result<()> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;

    let test_string_ptr = unsafe { (*conn_out_protocol).test_string };

    let status = (test_string_ptr)(conn_out_protocol, string.as_mut_ptr());

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call `QueryMode` function from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn query_mode(
    st: *mut SystemTable,
    mode_number: usize,
    columns: &mut usize,
    rows: &mut usize,
) -> Result<()> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;

    let query_mode_ptr = unsafe { (*conn_out_protocol).query_mode };

    let status = (query_mode_ptr)(conn_out_protocol, mode_number, columns, rows);

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call `SetMode` function from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn set_mode(st: *mut SystemTable, mode_number: usize) -> Result<()> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;

    let set_mode_ptr = unsafe { (*conn_out_protocol).set_mode };

    let status = (set_mode_ptr)(conn_out_protocol, mode_number);

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call `SetAttribute` function from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn set_attribute(st: *mut SystemTable, attribute: usize) -> Result<()> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;

    let set_attribute_ptr = unsafe { (*conn_out_protocol).set_attribute };

    let status = (set_attribute_ptr)(conn_out_protocol, attribute);

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call `ClearScreen` function from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn clear_screen(st: *mut SystemTable) -> Result<()> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;

    let clear_screen_ptr = unsafe { (*conn_out_protocol).clear_screen };

    let status = (clear_screen_ptr)(conn_out_protocol);

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call `SetCursorPostion` function from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn set_cursor_position(st: *mut SystemTable, column: usize, row: usize) -> Result<()> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;

    let set_cursor_position_ptr = unsafe { (*conn_out_protocol).set_cursor_position };

    let status = (set_cursor_position_ptr)(conn_out_protocol, column, row);

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call `EnableCursor` function from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn enable_cursor(st: *mut SystemTable, visible: bool) -> Result<()> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;

    let enable_cursor_ptr = unsafe { (*conn_out_protocol).enable_cursor };

    let status = (enable_cursor_ptr)(conn_out_protocol, Boolean::from(visible));

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Get `Mode` from `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
/// The returned Mode has the same lifetime as the `st` argument.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub unsafe fn get_mode(
    st: *mut SystemTable,
) -> core::result::Result<*mut simple_text_output::Mode, errors::NullPtrError> {
    let conn_out_protocol = unsafe { get_protocol(st) }?;
    Ok(unsafe { (*conn_out_protocol).mode })
}

/// Get the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` from SystemTable.
/// The returned Protocol has the same lifetime as the `st` argument.
/// Protocol caching should generally be avoided since the Protocol pointer can becoming invalid.
/// Thus this function is unsafe to call.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub unsafe fn get_protocol(
    st: *mut SystemTable,
) -> core::result::Result<*mut simple_text_output::Protocol, errors::NullPtrError> {
    let conn_out_protocol = unsafe { (*st).con_out };
    helpers::null_check_mut(conn_out_protocol, "Conn Out")?;
    Ok(conn_out_protocol)
}
