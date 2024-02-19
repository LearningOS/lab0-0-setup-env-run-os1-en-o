// 告诉 Rust 编译器不使用 Rust 标准库 std 转而使用核心库 core（core库不需要操作系统的支持）
#![no_std]
// 告诉编译器我们没有一般意义上的 main 函数
#![no_main]
mod lang_items;

use core::arch::global_asm;
// 通过 include_str! 宏将同目录下的汇编代码 entry.asm 转化为字符串并通过 global_asm! 宏嵌入到代码中
global_asm!(include_str!("entry.asm"));


// 避免编译器对名字[rust_main]进行混淆 "entry.asm -> call rust_main"
#[no_mangle]
pub fn rust_main() -> ! {
    // 先完成对 .bss 段的清零
    clear_bss();
    loop {}
}

// 对 .bss 段的清零

fn clear_bss() {

    // 引用一个外部的 C 函数接口(这意味着调用它的时候要遵从目标平台的 C 语言调用规范)
    extern "C" {
        // 由链接脚本 linker.ld 给出，并分别指出需要被清零的 .bss 段的起始和终止地址
        fn sbss();
        fn ebss();
    }
    // 等价 for 循环
    (sbss as usize..ebss as usize).for_each(|a| {
        // 将 .bss 段内的一个地址转化为一个 裸指针 (Raw Pointer)，并将它指向的值修改为 0
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
