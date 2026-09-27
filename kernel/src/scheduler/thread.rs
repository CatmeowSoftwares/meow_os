use crate::{
    kprintln,
    mem::{
        pmm,
        vmm::{Flags, get_hhdm, map_address},
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
    pub(crate) registers: Registers,
    pub(crate) cr3: u64,
    pub(crate) state: ThreadState,
}
impl Thread {
    pub(crate) fn new() -> Self {
        let addr = pmm::allocate();
        map_address(
            (addr as u64 + get_hhdm()) as _,
            addr as _,
            Flags::WRITE | Flags::PRESENT,
        );
        let mut rsp = addr as u64 + get_hhdm() + 0x1000;
        rsp -= 8;
        unsafe {
            *(rsp as *mut u64) = thread_1 as *const u64 as u64;
        }
        for _ in 0..15 {
            rsp -= 8;
            unsafe {
                *(rsp as *mut u64) = 0;
            }
        }

        //let mut rsp;
        unsafe { core::arch::asm!("mov {}, rsp", in(reg) rsp) };

        //let addr = 0x6666_7777_0000;
        //let virt_addr = addr + get_hhdm();
        //let cr3 = addr;
        let mut cr3;
        unsafe { core::arch::asm!("mov {}, cr3", out(reg) cr3) };
        //map_address(virt_addr as _, addr as _, Flags::WRITE | Flags::PRESENT);
        Self {
            registers: Registers {
                rsp,
                ..Default::default()
            },
            cr3,
            ..Default::default()
        }
    }
    fn switch(&mut self, old: Self) {}
}

fn idle() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

fn create_thread(ptr: *const u8) -> Thread {
    Thread::default()
}

fn idle_thread() {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
fn thread_1() {
    loop {
        //kprintln!("from 1");
    }
}
fn thread_2() {
    loop {
        kprintln!("from 2");
    }
}
#[unsafe(naked)]
pub(crate) extern "C" fn save_registers(old_rsp: *mut u64) {
    core::arch::naked_asm!(
        "
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

            mov [rdi], rsp
            ret
        "
    );
}

#[unsafe(naked)]
pub(crate) extern "C" fn pop_registers(rsp: u64) {
    core::arch::naked_asm!(
        "
            mov rsp, rdi
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
            ret
        "
    );
}
