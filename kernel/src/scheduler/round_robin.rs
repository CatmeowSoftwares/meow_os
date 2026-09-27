use core::ptr::null_mut;

use crate::{
    arch::x86_64::tss::TSS,
    kprintln,
    scheduler::{
        Scheduler,
        thread::{Thread, ThreadState, pop_registers, save_registers},
    },
};
use alloc::{boxed::Box, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub(crate) static ref SCHEDULER: Mutex<RoundRobin> = Mutex::new(RoundRobin::new());
}
pub(crate) struct Node {
    pub(crate) id: u32,
    pub(crate) thread: Thread,
    pub(crate) next: *mut Node,
}
unsafe impl Send for RoundRobin {}
unsafe impl Sync for RoundRobin {}
pub(crate) struct RoundRobin {
    pub(crate) head: *mut Node,
    pub(crate) current: *mut Node,
    pub(crate) last: *mut Node,
}
impl RoundRobin {
    fn new() -> Self {
        Self {
            head: null_mut(),
            current: null_mut(),
            last: null_mut(),
        }
    }
    pub fn add(&mut self, id: u32) {
        let temp = Box::into_raw(Box::new(Node {
            id,
            next: self.head,
            thread: Thread::new(),
        }));
        let head = self.head;
        unsafe {
            (*self.last).next = temp;
        }
        self.head = temp;
        unsafe {
            (*self.last).next = self.head;
        }
    }
}
impl Scheduler for RoundRobin {
    fn poll(&mut self) {
        if self.head.is_null() {
            return;
        }
        unsafe {
            save_registers(&mut (*self.current).thread.registers.rsp as *mut u64);
        }
        let mut tss = TSS.lock();
        unsafe {
            core::arch::asm!("mov {}, cr3" , in(reg) (*self.current).thread.cr3);
        }
        unsafe { (*self.current).thread.state = ThreadState::Ready };
        while let ThreadState::Ready = unsafe { &(*self.current).thread.state } {
            let current = self.current;
            unsafe {
                self.current = (*current).next;
                (*self.current).thread.state = ThreadState::Running;
            }
        }
        tss.rsp0 = unsafe { (*self.current).thread.rsp0 };
        unsafe {
            pop_registers((*self.current).thread.registers.rsp);
        }
    }
}
