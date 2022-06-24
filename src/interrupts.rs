
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{println, gdt, print};
use lazy_static::lazy_static;
use spin::{self, Mutex};

// 第一个外部中断index从32开始---前面的供cpu内部使用
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;


// 设置pics的映射范围
pub static  PICS: spin::Mutex<ChainedPics> = 
    spin::Mutex::new(unsafe {
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) 
    });

#[derive(Debug,Clone, Copy)]
#[repr(u8)]
pub enum InterrupIndex {
    Timer = PIC_1_OFFSET,
    KeyBoadr
}

impl InterrupIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}


lazy_static! {
    static ref IDT: InterruptDescriptorTable =  {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }
        idt[InterrupIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        idt[InterrupIndex::KeyBoadr.as_usize()]
            .set_handler_fn(key_boadr_interrupt_handler);
        idt
    };
}
// 初始化 interrupter Descriptor Table
pub fn init_idt() { 
    IDT.load();
}

extern "x86-interrupt" fn key_boadr_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    use x86_64::instructions::port::Port;
    use pc_keyboard::{layouts, ScancodeSet1, HandleControl, DecodedKey, Keyboard};

    // 扫描码转化成真正的字符
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key,ScancodeSet1>> = 
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
    }

    let mut keyboard = KEYBOARD.lock();
    // 键盘输入口
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe {
        port.read()
    };

    
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        // process_keyevent 逻辑只有在可打印字符按下的时候才会打印出来
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                // unicode字符类型的直接打印
                DecodedKey::Unicode(character) => print!("{}", character),
                // 打印出Enum的类型名字
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterrupIndex::KeyBoadr.as_u8());
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterrupIndex::Timer.as_u8());
    }
}

extern  "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}",stack_frame);
}


extern  "x86-interrupt" fn double_fault_handler (
    stack_frame: InterruptStackFrame,
    _err_code: u64
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n${:#?}",stack_frame);
}


#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
