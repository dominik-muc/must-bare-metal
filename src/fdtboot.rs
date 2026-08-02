use crate::hwinfo;

struct FDTInfo;

impl hwinfo::HWInfo for FDTInfo {
    fn get_memory(&self) -> hwinfo::MemoryInfo {
        todo!()
    }
}

pub fn get_hwinfo() -> impl hwinfo::HWInfo {
    FDTInfo
}
