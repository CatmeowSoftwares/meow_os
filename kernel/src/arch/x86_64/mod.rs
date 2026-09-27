use crate::{acpi, kprintln, scheduler};

pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod pic;
pub mod pit;
pub mod tss;
pub fn init() {
    gdt::init();
    tss::init();
    //pit::init();
    idt::init();
    crate::mem::init();
    scheduler::init();
    acpi::init();
}

pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    value
}

pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

pub unsafe fn inl(port: u16) -> u32 {
    let value: u32;
    unsafe {
        core::arch::asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    value
}
pub unsafe fn outl(port: u16, value: u32) {
    unsafe {
        core::arch::asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

pub fn cpuid(code: i32) -> (u32, u32, u32, u32) {
    let mut eax;
    let mut ebx;
    let mut ecx;
    let mut edx;
    unsafe {
        core::arch::asm!(
            "
            push rbx
            cpuid
            mov {tmp:e}, ebx
            pop rbx
            ", 
            inout("eax") code => eax, 
            tmp = out(reg) ebx, 
            out("ecx") ecx,
            out("edx") edx);
    }
    (eax, ebx, ecx, edx)
}
