#![no_main]
#![no_std]
#![feature(exclusive_range_pattern)]

use uefi_spec::efi::{self, Status};
use uefi_spec::{global_data::GlobalData, protocols::simple_text_input};

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static GLOBAL_SYSTEM_TABLE: GlobalData<efi::SystemTable> = GlobalData::new();
// Size in bytes
const MAX_BUFFER_SIZE: usize = 200;

#[export_name = "efi_main"]
pub extern "C" fn main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let r = GLOBAL_SYSTEM_TABLE.init(st);
    if r.is_err() {
        return efi::Status::ABORTED;
    }

    efi::Status::SUCCESS
}

struct Stdin;

impl Stdin {
    fn read(buf: &mut [u8]) -> Result<usize, Status> {
        let buf_len = buf.len();
        let mut buf_count = 0;

        let st = match GLOBAL_SYSTEM_TABLE.load() {
            Ok(x) => x,
            Err(_) => return Err(efi::Status::ABORTED),
        };

        // Max 3 bytes can be required to store a ucs2 character as utf8
        while buf_len - buf_count >= 3 {
            let ch = match simple_text_input::read_key_stroke(st) {
                // Need to add check for non-printable keys
                Ok(x) => x.unicode_char,
                Err(_) => return Err(efi::Status::ABORTED),
            };

            let l = utf16_to_utf8_char(ch, &mut buf[buf_count..]);
            buf_count += l;
        }

        Ok(buf_count)
    }
}

fn utf16_to_utf8_char(ch: u16, buf: &mut [u8]) -> usize {
    match ch {
        0b0000_0000_0000_0000..0b0000_0000_0111_1111 => {
            // 1-byte
            buf[0] = ch as u8;
            1
        }
        0b0000_0000_0111_1111..0b0000_0111_1111_1111 => {
            // 2-byte
            let a = ((ch & 0b0000_0111_1100_0000) >> 6) as u8;
            let b = (ch & 0b0000_0000_0011_1111) as u8;
            buf[0] = a | 0b1100_0000;
            buf[1] = b | 0b1000_0000;
            2
        }
        _ => {
            // 3-byte
            let a = ((ch & 0b1111_0000_0000_0000) >> 12) as u8;
            let b = ((ch & 0b0000_1111_1100_0000) >> 6) as u8;
            let c = (ch & 0b0000_0000_0011_1111) as u8;
            buf[0] = a | 0b1110_0000;
            buf[1] = b | 0b1000_0000;
            buf[2] = c | 0b1000_0000;
            3
        }
    }
}
