#![no_std]
#![no_main]

use core::{
    arch::asm,
    fmt::Write,
    panic::PanicInfo,
    ptr::{slice_from_raw_parts, write_volatile},
};

// extern crate alloc;

// use alloc::{borrow::ToOwned, vec::Splice};
use fdt::node::FdtNode;

use crate::{drivers::uart::UartWriter, hwinfo::HWInfo};

mod drivers;
mod fdtboot;
mod hwinfo;
mod new_main;
mod std;
mod vmem;

#[unsafe(link_section = ".text.start")]
#[unsafe(no_mangle)]
pub fn _start(hart_id: usize, dev_tree: *const u8) -> ! {
    unsafe { asm!("la sp, __stack_top") }
    kernel_main(hart_id, dev_tree);
    wfi_loop()
}

pub fn wfi_loop() -> ! {
    uprintln!("Falling into infinite loop...");
    loop {
        riscv::asm::wfi();
    }
}

pub fn kernel_main(hart_id: usize, dev_tree: *const u8) {
    uprintln!("FDT at: {:#?}", dev_tree);

    // let hw = fdtboot::get_hwinfo();
    // hw.get_memory();
    let fdt = unsafe { fdt::Fdt::from_ptr(dev_tree).unwrap() };
    print_node(fdt.find_node("/").unwrap(), 0);
    uprintln!("Available memory regions:");
    for region in fdt.memory().regions() {
        uprintln!(
            "Memory at: {:#?}, amount: {:#x}",
            region.starting_address,
            region.size.unwrap()
        )
    }
    for reserved in fdt.memory_reservations() {
        uprintln!(
            "Memory reserved at: {:#?}, amount: {:#x}",
            reserved.address(),
            reserved.size()
        )
    }
    let reserved_mem = fdt
        .find_node("/reserved-memory")
        .expect("expected reserved memory");
    for mem in reserved_mem.children() {
        for reg in mem.reg().unwrap() {
            uprintln!(
                "Memory reserved at: {:#?}, amount: {:#x}",
                reg.starting_address,
                reg.size.unwrap()
            )
        }
    }
    // unsafe { write_volatile(0x80000000 as *mut u64, 42) };
    let mmu_on: i32;
    unsafe { asm!("csrr {}, satp", out(reg) mmu_on) };
    uprintln!("MMU On? = {}", mmu_on);
    // uprintln!("MMU On? = {}", mmu_on);
}

fn print_node(node: FdtNode<'_, '_>, n_spaces: usize) {
    (0..n_spaces).for_each(|_| {
        uprint!(" ");
        ()
    });
    uprintln!("{}/", node.name);
    for prop in node.properties() {
        (0..n_spaces + 2).for_each(|_| {
            uprint!(" ");
            ()
        });
        uprintln!("{}/", prop.name);
    }

    for child in node.children() {
        print_node(child, n_spaces + 4);
    }
}

#[panic_handler]
fn panic(p: &PanicInfo) -> ! {
    uprintln!(
        "\
You fucked up so badly that kernel went to cry in the other room.
Are you proud of yourself? Please be better next time.
Ah and she left you a note, that reads:
\"I can't believe you have done this to me!
Did you really thought it was okay to:
\t {}
? We are DONE.\"
",
        p
    );
    wfi_loop()
}
