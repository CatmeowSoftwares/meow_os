use spin::{Mutex, Once};

use crate::{
    acpi::AcpiSdtHeader,
    arch::x86_64::cpuid,
    kprintln,
    mem::vmm::{Flags, PhysicalAddress, map_address},
};

#[repr(C)]
struct Madt {
    h: AcpiSdtHeader,
    lapic_address: u32,
    flags: u32,
}

struct MadtEntry {
    entry_type: u8,
    record_length: u8,
}
pub(crate) fn init(madt: &AcpiSdtHeader) {
    let madt = unsafe { &*(madt as *const AcpiSdtHeader as *const Madt) };
    kprintln!("{:#x}", madt.lapic_address);
}
