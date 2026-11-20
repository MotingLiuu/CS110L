#![no_std]
#![no_main]

use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;

use crate::console::Console;

mod console;
mod uart;

global_asm!(
    ".section .text.entry",
    ".globl _start",
    "_start:",
    "    la sp, boot_stack_top",
    "    call kernel_main",
    ".section .bss.stack",
    ".globl boot_stack_lower_bound",
    "boot_stack_lower_bound:",
    "    .space 4096 * 4",
    ".globl boot_stack_top",
    "boot_stack_top:",
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";
// // question: what is &[u8]?
// // answer:
// // b"..." 产生的不是 &str 而是 &'static [u8; 12] 定长数组的引用
// // &[u8; 12] 被转换为了 -> &[u8] 长度被从类型移动到了 fat pointer 中的 len 字段中
// //  &[u8] = ┌──────────────┬────────┐
// //          │ 地址 (8字节)  │ 长度   │
// //          └──────────────┴────────┘
// // 在 64 位上占用 16 字节
//
// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     let vga_buffer = 0xb8000 as *mut u8;
//     // question: what is as *mut u8?
//     // Answer:
//     // *mut T 是 raw pointer（裸指针），可变的、指向 T 的指针。
//     // 和引用 &mut T 是对应关系, 但完全不受借用检查器管辖
//     //
//     // 在地址 0xb8000 处，我声称有一块可写的 u8 数组”。编译器不验证这件事
//
//     for (i, &byte) in HELLO.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(i as isize * 2) = byte;
//             // question: what is offset?
//             // answer:
//             // offset 是裸指针上的一个方法, 从这个指针往前/后移动 count 个 T，返回新指针。
//             //
//             // question: what is unsafe?
//             // answerhttps://os.phil-opp.com/zh-CN/vga-text-mode/:
//             // 允许调用编译器无法验证的操作
//             *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//         }
//     }
//
//     loop {}
// }

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    // let uart = 0x1000_0000 as *mut u8;
    // for &byte in b"Hello RISC-V!\n" {
    //     unsafe {
    //         uart.write_volatile(byte);
    //     }
    // }
    let mut console = Console;
    write!(console, "hart={}, address={:#x}", 0, 0x8000000).unwrap();
    loop {}
}
