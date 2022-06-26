use r_efi::{
    efi::Boolean,
    protocols::simple_text_input::{self, InputKey},
    system::SystemTable,
};

use crate::{errors, helpers};

/// Call `Reset` function from `EFI_SIMPLE_TEXT_INPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn reset(
    st: *mut SystemTable,
    extended_verification: bool,
) -> Result<(), errors::StatusNullError> {
    let protocol = get_protocol(st)?;
    let reset_ptr = unsafe { (*protocol).reset };

    let r = (reset_ptr)(protocol, Boolean::from(extended_verification));

    helpers::status_to_result(r)?;

    Ok(())
}

/// Call `ReadKeyStroke` function from `EFI_SIMPLE_TEXT_INPUT_PROTOCOL`.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn read_key_stroke(st: *mut SystemTable) -> Result<InputKey, errors::StatusNullError> {
    let protocol = get_protocol(st)?;
    let read_key_stroke_ptr = unsafe { (*protocol).read_key_stroke };

    let mut input_key = InputKey::default();

    let r = (read_key_stroke_ptr)(protocol, &mut input_key);
    helpers::status_to_result(r)?;

    Ok(input_key)
}

pub fn get_protocol(
    st: *mut SystemTable,
) -> Result<*mut simple_text_input::Protocol, errors::NullPtrError> {
    let r = unsafe { (*st).con_in };
    helpers::null_check_mut(r, "Console In")?;
    Ok(r)
}
