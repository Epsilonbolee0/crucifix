#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crucifix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use crucifix::println;
 
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, crucifix!");
    crucifix::init();

    #[cfg(test)]
    test_main();

    println!("It fucking worked!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crucifix::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}