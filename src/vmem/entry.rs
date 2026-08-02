/// A RISC-V Sv39 virtual memory page entry.
///
/// Layout:
/// ```txt
///  63  62 61  60      54 53  28 27  19 18  10  9 8   7 6 5 4 3 2 1 0
/// | N | PBMT | Reserved | PPN2 | PPN1 | PPN0 | RSW | D A G U X W R V |
/// ```
///
/// Flags:
/// Dirty
/// Accessed
/// Global
/// User
/// Executable
/// Writable
/// Readable
/// Valid
pub struct Sv39Entry {
    data: u64,
}

const PTE_V: u64 = 1 << 0; // Valid
const PTE_R: u64 = 1 << 1; // Read
const PTE_W: u64 = 1 << 2; // Write
const PTE_X: u64 = 1 << 3; // Execute
const PTE_U: u64 = 1 << 4; // User
const PTE_G: u64 = 1 << 5; // Global
const PTE_A: u64 = 1 << 6; // Accessed
const PTE_D: u64 = 1 << 7; // Dirty

impl Sv39Entry {
    fn is_valid(&self) -> bool {
        self.data & 0x1 == 1
    }
    fn is_leaf(&self) -> bool {
        self.data & 0xE != 0
    }

    // fn new()
}

#[repr(align(4096))]
pub struct PageTable {
    entries: [Sv39Entry; 512],
}
