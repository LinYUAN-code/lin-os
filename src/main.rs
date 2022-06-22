#![no_std]  // 不适用std标准库
#![no_main] // 禁用入口特性
use core::panic::PanicInfo;
mod vga_buffer;


// extern "C" 以c函数调用格式生成这个函数
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{}","hello world from lrj");
    loop {}
}



// panic处理函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}