#![no_std]
#![no_main]

use core::panic::PanicInfo;

static GREET: &[u8] = b"CRUCIFIX";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xB8000 as *mut u8;

    GREET
        .iter()
        .flat_map(|bt| [*bt, 0xF as u8])
        .enumerate()
        .for_each(|(i, byte)| unsafe {
            *vga_buffer.offset(i as isize) = byte;
        });

    loop {}
}
