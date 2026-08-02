pub struct MemoryInfo {
    ranges: usize,
}

pub trait HWInfo {
    fn get_memory(&self) -> MemoryInfo;
}
