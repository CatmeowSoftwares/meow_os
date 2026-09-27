use crate::{
    acpi::madt::{APIC, EOI_REG},
    arch::x86_64::outb,
    kprint, kprintln,
    mem::vmm::PhysicalAddress,
};

#[derive(Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    ip: usize,
    cs: usize,
    flags: usize,
    sp: usize,
    ss: usize,
}
pub unsafe extern "x86-interrupt" fn pit(_stack_frame: InterruptStackFrame) {
    kprint!(".");

    crate::scheduler::schedule();
    let apic = APIC.get().unwrap();
    let eoi = EOI_REG;
    let ptr = PhysicalAddress::map_raw(*apic + eoi);
    unsafe {
        *ptr = 0;
    }
}
pub unsafe extern "x86-interrupt" fn divide_error(stack_frame: InterruptStackFrame) {
    panic!("#DE: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn debug_exception(stack_frame: InterruptStackFrame) {
    panic!("#DB: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn nmi_interrupt(stack_frame: InterruptStackFrame) {
    panic!("NMI: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame) {
    panic!("#BP: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn overflow(stack_frame: InterruptStackFrame) {
    panic!("#OF: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn bound_range_exeeded(stack_frame: InterruptStackFrame) {
    panic!("#BR: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn invalid_opcode(stack_frame: InterruptStackFrame) {
    panic!("#UD: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn device_not_available(stack_frame: InterruptStackFrame) {
    panic!("#NM: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn double_fault(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("#DF: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn invalid_tss(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("#TS: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn segment_not_present(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("#NP: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn stack_segment_fault(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("#SS: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn general_protection(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("#GP: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn page_fault(stack_frame: InterruptStackFrame, error_code: u64) {
    let mut cr2 = 0u64;
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) cr2);
    }

    panic!("#PF: {:#x?}\n cr2: {cr2:#x}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn x87_floating_point_error(stack_frame: InterruptStackFrame) {
    panic!("#MF: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn alignment_check(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("#AC: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn machine_check(stack_frame: InterruptStackFrame) {
    panic!("#MC: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn simd_floating_point_exception(
    stack_frame: InterruptStackFrame,
) {
    panic!("#XM: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn virtualization_exception(stack_frame: InterruptStackFrame) {
    panic!("#VE: {:#x?}", stack_frame);
}
pub unsafe extern "x86-interrupt" fn control_protection_exception(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("#CP: {:#x?}", stack_frame);
}
