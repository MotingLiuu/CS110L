use core::hint::spin_loop;
// question: what is core::hint::spin_loop?
use core::ptr::{read_volatile, write_volatile};

const UART_BASE: usize = 0x1000_0000;

// NS16550A 寄存器偏移，单位为字节
const THR: usize = 0; // Transmit Holding Register
const LSR: usize = 5; // Line Status Register

const LSR_THRE: u8 = 1 << 5;

// 读取一个 8-bit UART 寄存器
fn read_reg(offset: usize) -> u8 {
    let ptr = (UART_BASE + offset) as *const u8;
    // question: what is *const? 这代表指针不能变 还是 指针所指向的数据不能变

    // SAFETY:
    // 假设 QEMU virt 的 UART MMIO 地址已映射且可读。
    unsafe { read_volatile(ptr) }
}

// 写入一个 8-bit UART 寄存器
fn write_reg(offset: usize, value: u8) {
    let ptr = (UART_BASE + offset) as *mut u8;

    // SAFETY:
    // 假设 UART MMIO 地址已映射且寄存器可写。
    unsafe { write_volatile(ptr, value) }
}

// 阻塞式发送一个字节
pub fn putc(byte: u8) {
    // 等待 THR 可以接收下一个字节
    while read_reg(LSR) & LSR_THRE == 0 {
        spin_loop();
    }

    write_reg(THR, byte);
}

// 发送一个字符串
pub fn puts(s: &str) {
    for byte in s.bytes() {
        putc(byte);
    }
}
