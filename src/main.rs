#![no_std]  // 不适用std标准库
#![no_main] // 禁用入口特性

#![feature(custom_test_frameworks)]
#![test_runner(lin_os::test_runner)]


// 将测试的入口函数改名为test_main--方便我们在_start中调用
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lin_os::println;


// extern "C" 以c函数调用格式生成这个函数
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}","hello world from lrj");

    lin_os::init();

    #[cfg(test)]
    test_main();

    lin_os::hlt_loop();
}

// 非测试环境panic处理函数
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lin_os::test_panic_handler(info)
}