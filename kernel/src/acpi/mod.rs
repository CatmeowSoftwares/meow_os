use bitfields::bitfield;

use crate::{kprintln, requests::RSDP_REQUEST};

pub mod fadt;
pub mod madt;
#[repr(C, packed)]
struct Xsdp {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt_address: u32,

    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(crate) struct AcpiSdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oemid: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}
#[repr(C)]
struct Xsdt {
    h: AcpiSdtHeader,
}
pub fn init() {
    if let Some(rsdp) = RSDP_REQUEST.response() {
        let addr = rsdp.address;
        let hhdm = crate::mem::vmm::get_hhdm();
        kprintln!("addr: {addr:#p}");
        let xsdp = unsafe { &*(addr as *const Xsdp) };
        kprintln!("{}", str::from_utf8(&xsdp.signature).unwrap());
        kprintln!("{}", xsdp.revision);
        let xsdt_phys = xsdp.xsdt_address;
        let xsdt_virt = xsdt_phys + hhdm;
        kprintln!("{:#x}", xsdt_virt);
        let xsdt = unsafe { &*((xsdt_virt) as *const Xsdt) };
        kprintln!("{}", str::from_utf8(&xsdt.h.signature).unwrap());
        let size = (xsdt.h.length as usize - size_of::<AcpiSdtHeader>()) / size_of::<u64>();
        kprintln!("{size}");
        let entries_ptr =
            unsafe { (xsdt_virt as *const u8).add(size_of::<AcpiSdtHeader>()) as *const u64 };
        for i in 0..size {
            unsafe {
                let entry_phys = core::ptr::read_unaligned(entries_ptr.add(i));
                let entry_virt = entry_phys + hhdm;
                let pptr = &*(entry_virt as *const AcpiSdtHeader);
                kprintln!("{:?}", str::from_utf8(&pptr.signature).unwrap());
                if &pptr.signature == b"FACP" {
                    kprintln!("facp found?");
                    fadt::init(pptr);
                }
                if &pptr.signature == b"APIC" {
                    kprintln!("apic found?");
                    madt::init(pptr);
                }
            }
        }
    } else {
        kprintln!("saddddddddddddd");
    }
}
