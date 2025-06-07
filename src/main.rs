#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

extern crate alloc;

use alloc::format;
use drivers::uart::putchar;

mod drivers;
mod std;

fn uart_print(message: &str) {
    for c in message.as_bytes() {
        putchar(*c);
    }
}

#[unsafe(link_section = ".text.start")]
#[unsafe(no_mangle)]
pub fn _start() -> ! {
    unsafe { asm!("la sp, __stack_top") }
    kernel_main();
    wfi_loop()
}

pub fn wfi_loop() -> ! {
    uart_print("Falling into infinite loop...");
    loop {
        unsafe { asm!("wfi") }
    }
}

pub fn kernel_main() {
    let mut cnt = 0;
    loop {
        let s = format!("test: {}\n", cnt);
        cnt += 1;
        uart_print(&s);
    }
}

#[panic_handler]
fn panic(_p: &PanicInfo) -> ! {
    uart_print(
        "\
You fucked up so badly that kernel went to cry in the other room.
Are you proud of yourself? Please be better next time.
Ah and she left you a note, that reads:
\"I can't believe you have done this to me!
Did you really thought it was okay to:
\t",
    );
    uart_print(_p.message().as_str().unwrap_or("take a wild guess"));
    uart_print(
        "
? We are DONE.\"
",
    );
    wfi_loop()
}
