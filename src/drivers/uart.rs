const UART: *mut u8 = 0x10000000 as *mut u8;
const UART_LSR: *mut u8 = 0x10000005 as *mut u8;

pub fn putchar(c: u8) {
    unsafe {
        core::ptr::write_volatile(UART, c as u8);
    }
}

pub fn getchar() -> u8 {
    unsafe {
        while *UART_LSR & 0x01 == 0 {}
        return *UART;
    }
}
