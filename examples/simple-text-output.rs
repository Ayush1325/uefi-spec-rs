#![no_main]
#![no_std]

use uefi_spec::efi;
use uefi_spec::{global_data::GlobalData, protocols::simple_text_output};

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "C" fn main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let mut s = [
        0x0048u16, 0x0065u16, 0x006cu16, 0x006cu16, 0x006fu16, // "Hello"
        0x0020u16, //                                             " "
        0x0057u16, 0x006fu16, 0x0072u16, 0x006cu16, 0x0064u16, // "World"
        0x0021u16, //                                             "!"
        0x000au16, //                                             "\n"
        0x0000u16, //                                             NUL
    ];

    let system_table = GlobalData::new();
    let r = system_table.init(st);
    if r.is_err() {
        return efi::Status::ABORTED;
    }

    let st_ref = match system_table.load() {
        Ok(x) => x,
        Err(_) => return efi::Status::ABORTED,
    };

    // Print "Hello World!".
    let r = simple_text_output::output_string(st_ref, &mut s);

    match r {
        Ok(_) => efi::Status::SUCCESS,
        Err(x) => match x {
            uefi_spec::errors::StatusNullError::NullPtrError(_) => efi::Status::ABORTED,
            uefi_spec::errors::StatusNullError::UefiWarning(y) => efi::Status::from_usize(y),
            uefi_spec::errors::StatusNullError::UefiError(y) => efi::Status::from_usize(y),
        },
    }
}
