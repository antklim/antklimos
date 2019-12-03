#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(antklimos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use antklimos::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("AntklimOS, version {}", "0.0.1");

    antklimos::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

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
    antklimos::test_panic_handler(info);
}
