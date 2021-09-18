#![no_std]
#![no_main]

use core::panic::PanicInfo;

const BUFFER: *mut u8 = 0xB8000 as *mut u8;
const COLOR: u8 = 0xB;

static GREET: &[u8] = b"CRUCIFIX";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    GREET
        .iter()
        .flat_map(|bt| [*bt, COLOR])
        .enumerate()
        .for_each(|(i, byte)| unsafe {
            *BUFFER.offset(i as isize) = byte;
        });

    loop {}
}
