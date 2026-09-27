use crate::kprint;
use crate::kprintln;
use bitfields::bitfield;
use lazy_static::lazy_static;
use spin::Mutex;

use super::interrupts::*;
#[repr(align(4096))]
struct Idt([IdtEntry; 256]);
#[repr(C, packed)]
struct Idtr {
    size: u16,
    offset: u64,
}

#[derive(Default, Clone, Copy)]
#[repr(C, packed)]
struct IdtEntry {
    offset1: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset2: u16,
    offset3: u32,
    reserved: u32,
}

impl IdtEntry {
    fn new(offset: u64, flags: u8) -> Self {
        Self {
            offset1: (offset & 0xffff) as u16,
            offset2: ((offset >> 16) & 0xffff) as u16,
            offset3: ((offset >> 32) & 0xffffffff) as u32,
            ist: 0,
            selector: 0x08,
            type_attr: flags,
            reserved: 0,
        }
    }
}

static IDT: Mutex<Idt> = Mutex::new(Idt([IdtEntry {
    offset1: 0,
    offset2: 0,
    offset3: 0,
    ist: 0,
    selector: 0,
    type_attr: 0,
    reserved: 0,
}; 256]));
static IDTR: Mutex<Idtr> = Mutex::new(Idtr { size: 0, offset: 0 });

pub fn init() {
    let mut idt = IDT.lock();
    let mut idtr = IDTR.lock();

    idt.0[0] = IdtEntry::new(divide_error as *const u64 as u64, 0x8e);
    idt.0[1] = IdtEntry::new(debug_exception as *const u64 as u64, 0x8e);
    idt.0[2] = IdtEntry::new(nmi_interrupt as *const u64 as u64, 0x8e);
    idt.0[3] = IdtEntry::new(breakpoint as *const u64 as u64, 0x8e);
    idt.0[4] = IdtEntry::new(overflow as *const u64 as u64, 0x8e);
    idt.0[5] = IdtEntry::new(bound_range_exeeded as *const u64 as u64, 0x8e);
    idt.0[6] = IdtEntry::new(invalid_opcode as *const u64 as u64, 0x8e);
    idt.0[7] = IdtEntry::new(device_not_available as *const u64 as u64, 0x8e);
    idt.0[8] = IdtEntry::new(double_fault as *const u64 as u64, 0x8e);
    idt.0[10] = IdtEntry::new(invalid_tss as *const u64 as u64, 0x8e);
    idt.0[11] = IdtEntry::new(segment_not_present as *const u64 as u64, 0x8e);
    idt.0[12] = IdtEntry::new(stack_segment_fault as *const u64 as u64, 0x8e);
    idt.0[13] = IdtEntry::new(general_protection as *const u64 as u64, 0x8e);
    idt.0[14] = IdtEntry::new(page_fault as *const u64 as u64, 0x8e);
    idt.0[16] = IdtEntry::new(x87_floating_point_error as *const u64 as u64, 0x8e);
    idt.0[17] = IdtEntry::new(alignment_check as *const u64 as u64, 0x8e);
    idt.0[18] = IdtEntry::new(machine_check as *const u64 as u64, 0x8e);
    idt.0[19] = IdtEntry::new(simd_floating_point_exception as *const u64 as u64, 0x8e);
    idt.0[20] = IdtEntry::new(virtualization_exception as *const u64 as u64, 0x8e);
    idt.0[21] = IdtEntry::new(control_protection_exception as *const u64 as u64, 0x8e);
    idt.0[32] = IdtEntry::new(pit as *const u64 as u64, 0x8e);

    *idtr = Idtr {
        offset: &idt.0 as *const _ as u64,
        size: (size_of::<Idt>() - 1) as u16,
    };
    unsafe {
        core::arch::asm!("
        lidt [{}]
        ", 
        in(reg) &*idtr as *const Idtr);

        core::arch::asm!("sti");
    }
    kprintln!("IDT INIT");
}
