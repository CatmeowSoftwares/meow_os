pub trait Allocator {
    const PAGE_SIZE: usize = 4096;
    fn allocate(&mut self) -> *mut u8;
    fn free(&mut self, ptr: *mut u8);
    fn init(&mut self, base: u64, size: usize);
}

pub mod bitmap;
