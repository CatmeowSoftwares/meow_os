use crate::{
    kprintln,
    mem::allocators::{self, Allocator, bitmap::Bitmap},
    requests::MEMMAP_REQUEST,
};
use core::ptr::null_mut;
use limine::memmap::*;
use spin::Mutex;
pub const PAGE_SIZE: u64 = 0x1000;
static HEAD: Mutex<Node> = Mutex::new(Node { next: null_mut() });
struct Node {
    next: *mut Node,
}
unsafe impl Send for Node {}

pub fn init() {
    let mut bitmap = allocators::bitmap::BITMAP.lock();
    let rand_num = 67;
    bitmap.set_bit(rand_num, true);
    let bit = bitmap.get_bit(rand_num);
    kprintln!("{bit}");
    if let Some(memmap_response) = MEMMAP_REQUEST.response() {
        let mut arr = [0; 16];
        let mut i = 0;
        for entry in memmap_response.entries() {
            if entry.type_ == MEMMAP_USABLE {
                arr[i] = entry.length;
                bitmap.init(entry.base, entry.length as usize);
                i += 1;
            }
        }
        let mut sum = 0;
        for i in arr {
            sum += i;
        }
        kprintln!("mem sum: {sum}");
        let mut count = 0;
        for i in &bitmap.array {
            if *i == u64::MAX {
                count += 1;
            }
        }
        kprintln!("{}", bitmap.array.len() - count);
    }
}

pub fn allocate() -> *mut u8 {
    let mut allocator = allocators::bitmap::BITMAP.lock();
    let ptr = allocator.allocate();

    ptr
}
