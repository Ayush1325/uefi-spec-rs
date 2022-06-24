//! This module provides APIS for various boot services.

pub mod memory_allocation_services;

use crate::{efi, errors, helpers};

pub fn get_boot_services<'a>(
    st: *mut efi::SystemTable,
) -> Result<*mut efi::BootServices, errors::NullPtrError> {
    let boot_services = unsafe { (*st).boot_services };
    helpers::null_check_mut(boot_services, "Boot Services")?;
    Ok(unsafe { &mut *boot_services })
}
