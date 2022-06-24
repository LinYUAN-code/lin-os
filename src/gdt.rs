
use lazy_static::lazy_static;
use x86_64::{structures::{tss::TaskStateSegment, gdt::{GlobalDescriptorTable, Descriptor}}, VirtAddr, registers::segmentation::SegmentSelector};


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // 设置栈的大小
            const STACK_SIZE: usize = 4096 * 5;
            // 获取一块大小为4096*5 byte的存储空间作为栈空间
            // 由于还没有实现堆分配--现在只能使用data段以及栈内存空间--只能在编译的时候申请空间--无语😶
            // 注意到我们并没有设置guard page 很容易就会访问到非法空间--所以不要爆栈
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe {
                & STACK
            });
            let stack_end = stack_start + STACK_SIZE;
            // lol--栈是向下增长的
            stack_end
        };
        tss
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    
    GDT.0.load();
    unsafe {
        // 设置段寄存器--意义不明--不是有分页了吗
        // 对GDT TSS switch stack 的工作原理 没理解
        CS::set_reg(GDT.1.code_selector);
        // 设置tss寄存器
        load_tss(GDT.1.tss_selector);
    }
}