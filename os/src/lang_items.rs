// 现panic函数，并通过 #[panic_handler] 属性通知编译器用panic函数来对接 panic! 宏


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}