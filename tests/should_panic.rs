#![no_std]
#![no_main]

use crucifix::{exit_qemu, serial_print, serial_println, QemuExitCode};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[TEST DID NOT PANIC]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_print!("crucifix::should_fail...\t");
    assert_eq!(0, 1);    
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[OK]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}