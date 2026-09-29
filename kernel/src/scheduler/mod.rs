use core::ptr::null_mut;

use alloc::boxed::Box;

use crate::{
    kprint, kprintln,
    scheduler::thread::{Thread, thread_1, thread_2},
};

pub mod round_robin;
pub mod thread;
pub trait Scheduler {
    fn poll(&mut self) {}
}

pub fn schedule() {
    let mut scheduler = round_robin::SCHEDULER.lock();
    scheduler.poll();
}

pub(crate) fn init() {
    let mut scheduler = round_robin::SCHEDULER.lock();
    scheduler.head = Box::into_raw(Box::new(Thread {
        next: null_mut(),
        ..Thread::new()
    }));
    unsafe {
        (*scheduler.head).next = scheduler.head;
        scheduler.current = scheduler.head;
        scheduler.last = scheduler.current;
        (*scheduler.last).next = scheduler.head;
    }
    scheduler.add(thread_1 as *const u64 as _);
    scheduler.add(thread_2 as *const u64 as _);
}
