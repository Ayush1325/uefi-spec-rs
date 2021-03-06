#![cfg_attr(not(test), no_std)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod boot_services;
pub mod errors;
pub mod global_data;
mod helpers;
pub mod protocols;

pub mod efi {
    pub use r_efi::efi::{
        AllocateType, Boolean, BootServices, Handle, MemoryDescriptor, MemoryType, PhysicalAddress,
        Status, SystemTable, LOADER_DATA,
    };
}
