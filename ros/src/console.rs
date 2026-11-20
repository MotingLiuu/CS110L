use core::fmt;

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

#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    use core::fmt::Write;
    // question: why use core::fmt::Write here? use in a function?
    // not in the beginning of the file?

    let mut console = Console;
    console.write_fmt(args).unwrap();
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
