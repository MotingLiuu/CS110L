use core::arch::asm;
use core::fmt::{self, Write};
use spin::Mutex;

static CONSOLE: Mutex<Console> = Mutex::new(Console);

pub struct Console;

impl Console {
    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            crate::uart::putc(b'\r');
        }
        crate::uart::putc(byte);
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// 保存旧的 sstatus，同时清除 SIE
fn irq_save() -> usize {
    let old: usize;

    unsafe {
        asm!(
            "csrrci {old}, sstatus, 2",
            old = out(reg) old,
        );
    }

    old
}

// 恢复进入临界区之前的中断开关状态
fn irq_restore(old: usize) {
    const SSTATUS_SIE: usize = 1 << 1;

    if old & SSTATUS_SIE != 0 {
        unsafe {
            asm!("csrsi sstatus, 2");
        }
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    // 1. 保存当前中断状态并关闭本 hart SIE
    let old = irq_save();

    // 2. 获取锁、格式化、输出
    let result = {
        let mut writer = CONSOLE.lock();
        writer.write_fmt(args)
    }; // MutexGuard 在这里 drop，锁被释放

    // 3. 恢复原本的中断状态
    irq_restore(old);

    // 4. 在锁释放后处理格式化错误
    result.unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::console::_print(
            format_args!($($arg)*)
        );
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };

    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}
