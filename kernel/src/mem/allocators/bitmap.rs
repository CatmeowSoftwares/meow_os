use core::ptr::null_mut;

use spin::Mutex;

use crate::{kprint, kprintln};

pub static BITMAP: Mutex<Bitmap> = Mutex::new(Bitmap {
    array: [u64::MAX; u16::MAX as usize],
});
pub struct Bitmap {
    pub array: [u64; u16::MAX as usize],
}

impl Bitmap {
    pub fn set_bit(&mut self, idx: usize, value: bool) {
        let arr_idx = idx / 64;
        let bit_idx = idx % 64;
        let flag = 1 << bit_idx;
        if value {
            self.array[arr_idx] |= flag;
        } else {
            self.array[arr_idx] &= !flag;
        }
    }
    pub fn get_bit(&self, idx: usize) -> bool {
        let arr_idx = idx / 64;
        let bit_idx = idx % 64;
        let flag = 1 << bit_idx;
        (self.array[arr_idx] & flag) != 0
    }
}

impl super::Allocator for Bitmap {
    fn init(&mut self, base: u64, size: usize) {
        let start = base as usize / Self::PAGE_SIZE;
        let count = size / Self::PAGE_SIZE;
        for i in start..start + count {
            self.set_bit(i, false);
        }
    }
    fn allocate(&mut self) -> *mut u8 {
        let mut idx = 0;
        let max_idx = self.array.len() * 64;
        loop {
            if idx >= max_idx {
                kprintln!("out of memory");
                return null_mut();
            }
            if !self.get_bit(idx) {
                break;
            }
            idx += 1;
        }
        self.set_bit(idx, true);
        (idx * Self::PAGE_SIZE) as *mut u8
    }
    fn free(&mut self, ptr: *mut u8) {
        self.set_bit(ptr as usize / Self::PAGE_SIZE, false);
    }
}
