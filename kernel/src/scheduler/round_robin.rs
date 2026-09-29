use core::{mem::offset_of, ptr::null_mut};

use crate::{
    arch::x86_64::tss::{TSS, Tss}, kprintln, mem::{
        pmm::{self, PAGE_SIZE}, vmm::{Flags, get_hhdm, map_address},
    }, scheduler::{
        Scheduler,
        thread::Thread,
    },
};
use alloc::boxed::Box;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub(crate) static ref SCHEDULER: Mutex<RoundRobin> = Mutex::new(RoundRobin::new());
}
unsafe impl Send for RoundRobin {}
unsafe impl Sync for RoundRobin {}
pub(crate) struct RoundRobin {
    pub(crate) head: *mut Thread,
    pub(crate) current: *mut Thread,
    pub(crate) last: *mut Thread,
}
impl RoundRobin {
    fn new() -> Self {
        Self {
            head: null_mut(),
            current: null_mut(),
            last: null_mut(),
        }
    }
    pub fn add(&mut self, entry_point: u64) {
        let addr = pmm::allocate();
        map_address(
            (addr as u64 + get_hhdm()) as _,
            addr as _,
            Flags::WRITE | Flags::PRESENT,
        );
        let mut rsp = addr as u64 + get_hhdm() + PAGE_SIZE;
        rsp -= 8;
        unsafe {
            *(rsp as *mut u64) = entry_point;
        }
        for _ in 0..15 {
            rsp -= 8;
            unsafe {
                *(rsp as *mut u64) = 0;
            }
        }
        unsafe { core::arch::asm!("mov {}, rsp", in(reg) rsp) };

        let temp = Box::into_raw(Box::new(Thread {
            next: self.head,
            rsp,
            ..Thread::new()
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

        let old_rsp = unsafe { (*self.current).rsp as *mut u64 };
        kprintln!("old_rsp: {old_rsp:p}");
        let current = self.current;

        let next = unsafe { (*self.current).next };
        let mut tss = TSS.lock();
        let tss = &mut tss;
        let a = 1;
        let b = a + a;
        let c = a + b;
        switch(current, next, tss);
    }
}

#[unsafe(naked)]
extern "C" fn switch(current: *mut Thread, next: *mut Thread, tss: &mut Tss) {
    // rdi = current
    // rsi = next
    core::arch::naked_asm!("
            push rax
            push rbx
            push rcx
            push rdx
            push rdi
            push rsi
            push rbp
            
            push r8
            push r9
            push r10
            push r11
            push r12
            push r13
            push r14
            push r15


            mov r8, [rdi]
            mov [rdx + {rsp}], rsp 
            mov rax, [rsi]
            mov [rdi], rax
            
            mov rsp, [r8 + {rsp}]
            mov rax, [r8 + {cr3}]
            mov rbx, [r8 + {rsp0}]
            mov [rdx + {tss_rsp0}], rbx
            mov rcx, cr3

            cmp rax, rcx
            je 2f
            mov cr3, rax
        2:

            pop r15
            pop r14
            pop r13
            pop r12
            pop r11
            pop r10
            pop r9
            pop r8
            pop rbp
            pop rsi
            pop rdi
            pop rdx
            pop rcx
            pop rbx
            pop rax
    ",
            rsp = const offset_of!(Thread, rsp),
            cr3 = const offset_of!(Thread, cr3),
            rsp0 = const offset_of!(Thread, rsp0),
            tss_rsp0 = const offset_of!(Tss, rsp0),

    );
}
