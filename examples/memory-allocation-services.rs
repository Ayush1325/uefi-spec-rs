#![no_main]
#![no_std]
#![feature(alloc_error_handler, strict_provenance)]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::alloc::GlobalAlloc;

use uefi_spec::boot_services::memory_allocation_services;
use uefi_spec::efi;
use uefi_spec::global_data::GlobalData;
use uefi_spec::protocols::simple_text_output;

#[alloc_error_handler]
fn rust_oom_handler(_layout: core::alloc::Layout) -> ! {
    panic!();
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const POOL_ALIGNMENT: usize = 8;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

static GLOBAL_SYSTEM_TABLE: GlobalData<efi::SystemTable> = GlobalData::new();

pub fn efi_run() -> efi::Status {
    let st = match GLOBAL_SYSTEM_TABLE.load() {
        Ok(x) => x,
        Err(_) => return efi::Status::ABORTED,
    };

    let s: String;
    let mut v: Vec<u16>;

    // Create string and convert to UTF-16. We need a terminating NUL, since UEFI uses C-String
    // style wide-strings.
    s = String::from("Hello World!\n");
    v = s.encode_utf16().collect();
    v.push(0);

    // Print the string on console-out.
    let r = simple_text_output::output_string(st, v.as_mut_slice());
    if r.is_err() {
        efi::Status::ABORTED
    } else {
        efi::Status::SUCCESS
    }
}

#[export_name = "efi_main"]
pub extern "C" fn main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let r = GLOBAL_SYSTEM_TABLE.init(st);
    if r.is_err() {
        return efi::Status::ABORTED;
    }

    efi_run()
}

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let st = match GLOBAL_SYSTEM_TABLE.load() {
            Ok(x) => x,
            Err(_) => return core::ptr::null_mut(),
        };

        let align = layout.align();
        let size = layout.size();

        if size == 0 {
            return core::ptr::null_mut();
        }

        let mut ptr: *mut core::ffi::c_void = core::ptr::null_mut();
        let aligned_size = align_size(size, align);

        let r =
            memory_allocation_services::allocate_pool(st, efi::LOADER_DATA, aligned_size, &mut ptr);

        if r.is_err() || ptr.is_null() {
            return core::ptr::null_mut();
        }

        unsafe { align_ptr(ptr.cast(), align) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let st = match GLOBAL_SYSTEM_TABLE.load() {
            Ok(x) => x,
            Err(_) => return,
        };

        if layout.size() != 0 {
            let ptr = unsafe { unalign_ptr(ptr, layout.align()) };
            let r = memory_allocation_services::free_pool(st, ptr.cast());
            assert!(r.is_ok());
        }
    }
}

#[inline]
fn align_size(size: usize, align: usize) -> usize {
    if align > POOL_ALIGNMENT {
        // Allocate extra padding in order to be able to satisfy the alignment.
        size + align
    } else {
        size
    }
}

#[repr(C)]
struct Header(*mut u8);

#[inline]
unsafe fn align_ptr(ptr: *mut u8, align: usize) -> *mut u8 {
    if align > POOL_ALIGNMENT {
        let offset = align - (ptr.addr() & (align - 1));

        // SAFETY: `MIN_ALIGN` <= `offset` <= `layout.align()` and the size of the allocated
        // block is `layout.align() + layout.size()`. `aligned` will thus be a correctly aligned
        // pointer inside the allocated block with at least `layout.size()` bytes after it and at
        // least `MIN_ALIGN` bytes of padding before it.
        let aligned = unsafe { ptr.add(offset) };

        // SAFETY: Because the size and alignment of a header is <= `MIN_ALIGN` and `aligned`
        // is aligned to at least `MIN_ALIGN` and has at least `MIN_ALIGN` bytes of padding before
        // it, it is safe to write a header directly before it.
        unsafe { core::ptr::write((aligned as *mut Header).offset(-1), Header(ptr)) };

        aligned
    } else {
        ptr
    }
}

#[inline]
unsafe fn unalign_ptr(ptr: *mut u8, align: usize) -> *mut u8 {
    if align > POOL_ALIGNMENT {
        // SAFETY: Because of the contract of `System`, `ptr` is guaranteed to be non-null
        // and have a header readable directly before it.
        unsafe { core::ptr::read((ptr as *mut Header).offset(-1)).0 }
    } else {
        ptr
    }
}
