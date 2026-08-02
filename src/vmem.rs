use crate::hwinfo::{HWInfo, MemoryInfo};
mod entry;

fn make_pte(ppn: u64, flags: u64) -> u64 {
    (ppn << 10) | flags
}
