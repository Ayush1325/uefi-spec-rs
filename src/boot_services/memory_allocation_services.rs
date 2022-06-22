//! This module contains functions related to Memory Allocation.

use crate::{errors, helpers};
use core::ffi::c_void;
use r_efi::{
    efi::{PhysicalAddress, SystemTable},
    system::{AllocateType, MemoryDescriptor, MemoryType},
};

pub type Result<T> = core::result::Result<T, errors::StatusNullError>;

/// Call EFI_ALLOCATE_POOL boot service function.
/// This function treats Warnings as error right now. This might
/// change in the future though.
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn allocate_pool(
    st: &mut *mut SystemTable,
    memtype: MemoryType,
    allocate_size: usize,
    ptr: &mut *mut c_void,
) -> Result<()> {
    let boot_services = unsafe { (*(*st)).boot_services };
    helpers::null_check_mut(boot_services, errors::NullPtrError::new("Boot Services"))?;

    // TODO: Check if the assumption that allocate_pool_ptr will be valid as long as boot_services
    // ptr is valid. Else this might need change upstream in r-efi
    let allocate_pool_ptr = unsafe { (*boot_services).allocate_pool };

    let status = (allocate_pool_ptr)(memtype, allocate_size, ptr);

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call EFI_FREE_POOL boot service function
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn free_pool(st: &mut *mut SystemTable, ptr: *mut c_void) -> Result<()> {
    let boot_services = unsafe { (*(*st)).boot_services };
    helpers::null_check_mut(boot_services, errors::NullPtrError::new("Boot Services"))?;

    // TODO: Check if the assumption that free_pool_ptr will be valid as long as boot_services
    // ptr is valid. Else this might need change upstream in r-efi
    let free_pool_ptr = unsafe { (*boot_services).free_pool };

    let status = (free_pool_ptr)(ptr);

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call EFI_ALLOCATE_PAGES boot service function
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn allocate_pages(
    st: &mut *mut SystemTable,
    alloc_type: AllocateType,
    memtype: MemoryType,
    pages: usize,
    memory_address: *mut PhysicalAddress,
) -> Result<()> {
    let boot_services = unsafe { (*(*st)).boot_services };
    helpers::null_check_mut(boot_services, errors::NullPtrError::new("Boot Services"))?;

    // TODO: Check if the assumption that allocate_pages_ptr will be valid as long as boot_services
    // ptr is valid. Else this might need change upstream in r-efi
    let allocate_pages_ptr = unsafe { (*boot_services).allocate_pages };

    let status = (allocate_pages_ptr)(alloc_type, memtype, pages, memory_address);

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call EFI_FREE_PAGES boot service function
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn free_pages(
    st: &mut *mut SystemTable,
    memory_address: PhysicalAddress,
    pages: usize,
) -> Result<()> {
    let boot_services = unsafe { (*(*st)).boot_services };
    helpers::null_check_mut(boot_services, errors::NullPtrError::new("Boot Services"))?;

    // TODO: Check if the assumption that free_pages_ptr will be valid as long as boot_services
    // ptr is valid. Else this might need change upstream in r-efi
    let free_pages_ptr = unsafe { (*boot_services).free_pages };

    let status = (free_pages_ptr)(memory_address, pages);

    helpers::status_to_result(status).map_err(|x| x.into())
}

/// Call EFI_GET_MEMORY_MAP boot service function
/// SAFETY : The `st` pointer must be valid. This is gaurenteed if `GlobalData` is used to store
/// the pointer.
pub fn get_memory_map(
    st: &mut *mut SystemTable,
    memory_map_address: &mut usize,
    memory_map: &mut MemoryDescriptor,
    map_key: &mut usize,
    descriptor_size: &mut usize,
    descriptor_version: &mut u32,
) -> Result<()> {
    let boot_services = unsafe { (*(*st)).boot_services };
    helpers::null_check_mut(boot_services, errors::NullPtrError::new("Boot Services"))?;

    // TODO: Check if the assumption that get_memory_map_ptr will be valid as long as boot_services
    // ptr is valid. Else this might need change upstream in r-efi
    let get_memory_map_ptr = unsafe { (*boot_services).get_memory_map };

    let status = (get_memory_map_ptr)(
        memory_map_address,
        memory_map,
        map_key,
        descriptor_size,
        descriptor_version,
    );

    helpers::status_to_result(status).map_err(|x| x.into())
}
