// in tests/basic_boot.rs

// 集成测试

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(lin_os::test_runner)]

use core::panic::PanicInfo;
use lin_os::println;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lin_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}