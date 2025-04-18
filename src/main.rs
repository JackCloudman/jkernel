#![no_std] // no stdlib
#![no_main] // no main
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from JKernel!");
    
    println!("**************************************************");
    println!("{:^50}", "JKernel Started");
    println!("{:^50}", "Welcome to JKernel!");
    println!("**************************************************");
    
    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called when a panic occurs.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("aserción trivial... ");
    assert_eq!(1, 1);
    println!("[ok]");
}