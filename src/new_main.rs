use crate::UartWriter;
use crate::asm;
use crate::{hwinfo::HWInfo, uprintln};
use core::fmt::Write;
pub fn vmem_switch(hw: impl HWInfo) {
    let mem = hw.get_memory();
}

pub fn after_vmem() {
    uprintln!("Virtual memory initialized!");
    unsafe { asm!("mv a0, pc") };
    uprintln!("Current PC: {}", 4)
}
