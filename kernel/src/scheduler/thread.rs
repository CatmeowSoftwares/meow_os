
use crate::{
    kprintln, mem::{
        pmm::{self, PAGE_SIZE}, vmm::get_hhdm,
    },
};

#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum ThreadState {
    #[default]
    Ready,
    Running,
    Blocked,
    Finished,
}
#[derive(Default)]
pub(crate) struct Registers {
    pub(crate) rax: u64,
    pub(crate) rbx: u64,
    pub(crate) rcx: u64,
    pub(crate) rdx: u64,
    pub(crate) rsi: u64,
    pub(crate) rdi: u64,
    pub(crate) rsp: u64,
    pub(crate) rbp: u64,
    pub(crate) r8: u64,
    pub(crate) r9: u64,
    pub(crate) r10: u64,
    pub(crate) r11: u64,
    pub(crate) r12: u64,
    pub(crate) r13: u64,
    pub(crate) r14: u64,
    pub(crate) r15: u64,

    pub(crate) rip: u64,

    pub(crate) cs: u16,
    pub(crate) ds: u16,
    pub(crate) ss: u16,
    pub(crate) es: u16,
    pub(crate) fs: u16,
    pub(crate) gs: u16,
}

#[derive(Default)]
pub(crate) struct Thread {
    pub(crate) rsp0: u64,
    pub(crate) rsp: u64,
    pub(crate) cr3: u64,
    pub(crate) state: ThreadState,
    pub(crate) next: *mut Self,
}
impl Thread {
    pub(crate) fn new() -> Self {
        let phys_addr = pmm::allocate();
        let virt_addr = (phys_addr as u64) + get_hhdm();
        let stack_top = virt_addr + PAGE_SIZE;
        let rsp = stack_top;
        Self {
            rsp,
            ..Default::default()
        }
    }
}

fn idle() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}


fn idle_thread() {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
fn thread_0() {
    loop {
        kprintln!("e");
    }
}
pub(crate) fn thread_1() {
    loop {
        kprintln!("from 1");
    }
}
pub(crate) fn thread_2() {
    loop {
        kprintln!("from 2");
    }
}