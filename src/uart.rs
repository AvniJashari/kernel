const UART0_BASE: usize = 0x1000_0000;

const UART_RBR: usize = UART0_BASE + 0x00;
const UART_THR: usize = UART0_BASE + 0x00;
const UART_LSR: usize = UART0_BASE + 0x05;

const LSR_TX_IDLE: u8 = 0x20;

pub(crate) struct Uart{
//    index: usize,
}


impl Uart {
//    fn new(index: usize) -> Self{
//        Self{ index: index }
//    }

    pub fn putc(c: u8) -> () {
        use core::ptr::{read_volatile, write_volatile};
        unsafe {
            loop {
                if (read_volatile(UART_LSR as *const u8) & LSR_TX_IDLE) == 0 {
                    break;
                }
            }
            write_volatile(UART_THR as *mut u8, c);
        }
    }
}
