use core::ptr::null_mut;

use alloc::boxed::Box;

use crate::{
    kprint, kprintln,
    scheduler::{round_robin::Node, thread::Thread},
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
    scheduler.head = Box::into_raw(Box::new(Node {
        id: 1,
        next: null_mut(),
        thread: Thread::new(),
    }));
    unsafe {
        (*scheduler.head).next = scheduler.head;
        scheduler.current = scheduler.head;
        scheduler.last = scheduler.current;
        (*scheduler.last).next = scheduler.head;
    }
    scheduler.add(2);
    scheduler.add(3);
}
