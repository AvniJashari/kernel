use crate::sbi;
use crate::uart;

pub fn putc(c: u8) -> (){
    sbi::putc(c);
    //uart::Uart::putc(c);
}

pub struct Printer;

impl core::fmt::Write for Printer{
    fn write_str(&mut self, s: &str) -> core::fmt::Result{
        for byte in s.bytes() {
            putc(byte);
        }

        Ok(())
    }
}


macro_rules! println {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = writeln!($crate::print::Printer, $($arg)*);
    }};
}

macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::print::Printer, $($arg)*);
    }};
}
