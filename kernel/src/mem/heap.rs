pub fn init() {
    init_heap();
}

use linked_list_allocator::LockedHeap;

use crate::{
    kprintln,
    mem::{
        pmm,
        vmm::{self, Flags},
    },
};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

static HEAP_ADDRESS: usize = 0x_4444_4444_0000;

fn init_heap() {
    kprintln!("heap start");
    kprintln!("allocating ptr");
    let ptr = pmm::allocate();
    kprintln!("mapping addr");
    vmm::map_pages(HEAP_ADDRESS as *mut u8, 256, Flags::PRESENT | Flags::WRITE);
    kprintln!("success mapping addr");
    let heap_start = HEAP_ADDRESS;
    let heap_end = HEAP_ADDRESS + (256 * 0x1000);
    let heap_size = heap_end - heap_start;
    kprintln!("start init allocator");
    unsafe {
        ALLOCATOR.lock().init(heap_start as *mut u8, heap_size);
    }
    kprintln!("success init allocator");

    kprintln!("heap init");
}
