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
