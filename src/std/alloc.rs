use core::alloc::GlobalAlloc;

unsafe extern "C" {
    static __heap_start: u8;
    static __heap_end: u8;
}

static mut OFFSET: usize = 0;

#[global_allocator]
static ALLOC: AllocS = AllocS;

struct AllocS;

unsafe impl GlobalAlloc for AllocS {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe {
            let heap_start = &__heap_start as *const u8 as usize;
            let heap_end = &__heap_end as *const u8 as usize;
            let heap_size = heap_end - heap_start;
            let layout = layout.pad_to_align();
            if heap_size - OFFSET < layout.size() {
                panic!("no mem");
                // return 0x0 as *mut u8;
            }
            let ptr = (heap_start + OFFSET) as *mut u8;
            OFFSET += layout.size() + 16;
            return ptr.into();
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {}
}
