#![no_std] // no stdlib
#![no_main] // no main

use core::panic::PanicInfo;

#[no_mangle] // no mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker searches for a function
    // called `_start` by default
    loop {}
}

/// This function is called when a panic occurs.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}