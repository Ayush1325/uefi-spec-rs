#![no_main]
#![no_std]
#![feature(exclusive_range_pattern)]

use uefi_spec::efi;
use uefi_spec::{global_data::GlobalData, protocols::simple_text_output};

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static GLOBAL_SYSTEM_TABLE: GlobalData<efi::SystemTable> = GlobalData::new();

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

    let r = GLOBAL_SYSTEM_TABLE.init(st);
    if r.is_err() {
        return efi::Status::ABORTED;
    }

    let st_ref = match GLOBAL_SYSTEM_TABLE.load() {
        Ok(x) => x,
        Err(_) => return efi::Status::ABORTED,
    };

    // Print "Hello World!".
    let r = simple_text_output::output_string(st_ref, &mut s);

    match Stdout::write("Ayush\n".as_bytes()) {
        Ok(_) => {
            simple_text_output::output_string(st_ref, &mut s);
        }
        Err(_) => {
            simple_text_output::output_string(st_ref, &mut s);
        }
    }

    match r {
        Ok(_) => efi::Status::SUCCESS,
        Err(x) => match x {
            uefi_spec::errors::StatusNullError::NullPtrError(_) => efi::Status::ABORTED,
            uefi_spec::errors::StatusNullError::UefiWarning(y) => efi::Status::from_usize(y),
            uefi_spec::errors::StatusNullError::UefiError(y) => efi::Status::from_usize(y),
        },
    }
}

struct Stdout;

impl Stdout {
    fn write(buf: &[u8]) -> Result<usize, &'static str> {
        let st = match GLOBAL_SYSTEM_TABLE.load() {
            Ok(x) => x,
            Err(_) => return Err("global_data"),
        };

        let mut output_string = [0u16; 100];

        let count = match utf8_to_utf16(buf, &mut output_string) {
            Ok(x) => x,
            Err(_) => return Err("Conversion Error"),
        };

        output_string[count] = 0;

        match simple_text_output::output_string(st, &mut output_string) {
            Ok(_) => Ok(count),
            Err(_) => Err("Output String"),
        }
    }
}

fn utf8_to_utf16(utf8_buf: &[u8], utf16_buf: &mut [u16]) -> Result<usize, ConversionError> {
    let utf8_buf_len = utf8_buf.len();
    let utf16_buf_len = utf16_buf.len();
    let mut utf8_buf_count = 0;
    let mut utf16_buf_count = 0;

    while utf8_buf_count < utf8_buf_len {
        match utf8_buf[utf8_buf_count] {
            0b0000_0000..0b0111_1111 => {
                // 1-byte
                utf16_buf[utf16_buf_count] = u16::from(utf8_buf[utf8_buf_count]);
                utf8_buf_count += 1;
            }
            0b1100_0000..0b1101_1111 => {
                // 2-byte
                if utf16_buf_count + 1 >= utf8_buf_len {
                    return Err(ConversionError::InvalidUtf8);
                }
                let a = u16::from(utf8_buf[utf8_buf_count] & 0b0001_1111);
                let b = u16::from(utf8_buf[utf8_buf_count + 1] & 0b0011_1111);
                utf16_buf[utf16_buf_count] = a << 6 | b;

                utf8_buf_count += 2;
            }
            0b1110_0000..0b1110_1111 => {
                // 3-byte
                if utf16_buf_count + 2 >= utf8_buf_len {
                    return Err(ConversionError::InvalidUtf8);
                }
                let a = u16::from(utf8_buf[utf8_buf_count] & 0b0000_1111);
                let b = u16::from(utf8_buf[utf8_buf_count + 1] & 0b0011_1111);
                let c = u16::from(utf8_buf[utf8_buf_count + 2] & 0b0011_1111);
                utf16_buf[utf16_buf_count] = a << 12 | b << 6 | c;
                utf8_buf_count += 3;
            }
            0b1111_0000..0b1111_0111 => {
                // 4-byte
                return Err(ConversionError::UnsupportedUtf8);
            }
            _ => {
                // Invalid stuff
                return Err(ConversionError::InvalidUtf8);
            }
        }

        utf16_buf_count += 1;

        if utf16_buf_count >= utf16_buf_len {
            return Err(ConversionError::BufferOverflow);
        }
    }
    Ok(utf16_buf_count)
}

enum ConversionError {
    InvalidUtf8,
    BufferOverflow,
    UnsupportedUtf8,
}
