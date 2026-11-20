#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";
// question: what is &[u8]?
// answer:
// b"..." 产生的不是 &str 而是 &'static [u8; 12] 定长数组的引用
// &[u8; 12] 被转换为了 -> &[u8] 长度被从类型移动到了 fat pointer 中的 len 字段中
//  &[u8] = ┌──────────────┬────────┐
//          │ 地址 (8字节)  │ 长度   │
//          └──────────────┴────────┘
// 在 64 位上占用 16 字节

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    // question: what is as *mut u8?
    // Answer:
    // *mut T 是 raw pointer（裸指针），可变的、指向 T 的指针。
    // 和引用 &mut T 是对应关系, 但完全不受借用检查器管辖
    //
    // 在地址 0xb8000 处，我声称有一块可写的 u8 数组”。编译器不验证这件事

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            // question: what is offset?
            // answer:
            // offset 是裸指针上的一个方法, 从这个指针往前/后移动 count 个 T，返回新指针。
            //
            // question: what is unsafe?
            // answer:
            // 允许调用编译器无法验证的操作
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
