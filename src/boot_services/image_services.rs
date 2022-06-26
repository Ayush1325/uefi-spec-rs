use core::ffi::c_void;

use crate::efi::{Handle, Status, SystemTable};
use crate::{errors, helpers};
use r_efi::efi::Boolean;
use r_efi::protocols::device_path;

type Result<T> = core::result::Result<T, errors::StatusNullError>;

/// SAFETY: The caller must ensure that `st` is not null.
pub fn exit(
    st: *mut SystemTable,
    image_handle: Handle,
    exit_status: Status,
    exit_data: &mut [u16],
) -> Result<()> {
    let exit_data_size: usize = core::mem::size_of_val(exit_data);
    let boot_services = super::get_boot_services(st)?;
    let exit_ptr = unsafe { (*boot_services).exit };

    let r = (exit_ptr)(
        image_handle,
        exit_status,
        exit_data_size,
        exit_data.as_mut_ptr(),
    );

    helpers::status_to_result(r)?;

    Ok(())
}
