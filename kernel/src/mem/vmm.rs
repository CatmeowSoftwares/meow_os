use core::{
    arch::global_asm,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

use bitfields::bitfield;
use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use lazy_static::lazy_static;
use spin::{Mutex, Once};
static HHDM: Once<u64> = Once::new();
use crate::{
    kprintln,
    mem::{
        self,
        allocators::{Allocator, bitmap::Bitmap},
        pmm,
    },
    requests::{EXECUTABLE_ADDRESS_REQUEST, HHDM_REQUEST},
};
static PML4: Mutex<Pml4> = Mutex::new(Pml4([PageDirectory::new(); 512]));
pub fn get_hhdm() -> u64 {
    let hhdm = HHDM.get().unwrap();
    *hhdm
}
pub fn init() {
    let mut pml4 = PML4.lock();
    pml4.0[0].set_p(true);
    if let Some(hhdm) = HHDM_REQUEST.response() {
        HHDM.call_once(|| hhdm.offset);
    }
    if let Some(executable_addr) = EXECUTABLE_ADDRESS_REQUEST.response() {
        let hhdm = *HHDM.get().unwrap();
        let cr3 = get_cr3();
        let cr3_pml1r = (cr3 + hhdm) as *const Pml4;

        let cr3_arr = unsafe { (*cr3_pml1r).0 };

        for i in 0..cr3_arr.len() {
            if bytemuck::cast::<_, u64>(cr3_arr[i]) != 0 {
                pml4.0[i] = cr3_arr[i];
            }
        }
        set_cr3(
            (pml4.0.as_ptr() as u64)
                - (executable_addr.virtual_base - executable_addr.physical_base),
        );
        kprintln!("mrow");
    }
}
#[bitfield(u64, order = msb)]
#[derive(Zeroable, Pod)]
struct VirtualAddress {
    sign_ext: u16,
    #[bits(9)]
    pml4: u16,
    #[bits(9)]
    pml3: u16,
    #[bits(9)]
    pml2: u16,
    #[bits(9)]
    pml1: u16,
    #[bits(12)]
    offset: u32,
}
#[derive(Zeroable, Pod, Clone, Copy)]
#[repr(transparent)]
pub struct PhysicalAddress<T>(VirtualAddress, PhantomData<T>);
impl<T> PhysicalAddress<T> {
    pub fn new(address: u64) -> Self {
        let aligned_addr = address & !0xfff;
        let virtual_addr = aligned_addr + get_hhdm();
        map_address(
            virtual_addr as _,
            aligned_addr as _,
            Flags::WRITE | Flags::PRESENT,
        );
        let virtual_address = bytemuck::cast::<_, VirtualAddress>(address + get_hhdm());
        Self(virtual_address, PhantomData)
    }
    pub fn map_raw(address: u64) -> *mut T {
        let aligned_addr = address & !0xfff;
        let virtual_addr = aligned_addr + get_hhdm();
        map_address(
            virtual_addr as _,
            aligned_addr as _,
            Flags::WRITE | Flags::PRESENT,
        );
        //let virtual_address = bytemuck::cast::<_, VirtualAddress>(address + get_hhdm());
        (address + get_hhdm()) as _
    }
}
impl<T> Deref for PhysicalAddress<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0.0 as *const T) }
    }
}
impl<T> DerefMut for PhysicalAddress<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.0.0 as *mut T) }
    }
}
struct Page {}
pub fn unmap(virtual_address: *mut u8) {
    //let virtual_address = virtual_address as u64 & !0xfff;
    let virtual_addr = bytemuck::cast::<_, VirtualAddress>(virtual_address as u64);
    let pml4_idx = virtual_addr.pml4() as usize;
    let pml3_idx = virtual_addr.pml3() as usize;
    let pml2_idx = virtual_addr.pml2() as usize;
    let pml1_idx = virtual_addr.pml1() as usize;
    let _offset = virtual_addr.offset() as usize;

    let mut pml4 = PML4.lock();
    let pml4 = &mut pml4.0;
    let hhdm = HHDM.get().unwrap();

    unsafe {
        if !pml4[pml4_idx].p() {
            kprintln!("not present");
            return;
        }
        let pml3 =
            &mut *((((pml4[pml4_idx].addr() as u64) << 12) + hhdm) as *mut [PageDirectory; 512]); //or pdpt
        if !pml3[pml3_idx].p() {
            kprintln!("not present");
            return;
        }
        let pml2 =
            &mut *(((((*pml3)[pml3_idx].addr() as u64) << 12) + hhdm) as *mut [PageDirectory; 512]); //or pd

        if !pml2[pml2_idx].p() {
            kprintln!("not present");
            return;
        }
        let pml1 =
            &mut *(((((*pml2)[pml2_idx].addr() as u64) << 12) + hhdm) as *mut [PageTable; 512]); // or pt\

        if !pml1[pml1_idx].p() {
            kprintln!("not present");
            return;
        }
        let page = &mut pml1[pml1_idx];
        page.clear_bits();
        core::arch::asm!("invlpg [{}]", in(reg) virtual_address);
    }
}
pub fn get_physical_addresss(virtual_address: *mut u8) -> *mut u8 {
    let virtual_addr = bytemuck::cast::<_, VirtualAddress>(virtual_address as u64);
    let pml4_idx = virtual_addr.pml4() as usize;
    let pml3_idx = virtual_addr.pml3() as usize;
    let pml2_idx = virtual_addr.pml2() as usize;
    let pml1_idx = virtual_addr.pml1() as usize;
    let offset = virtual_addr.offset() as usize;
    let mut pml4 = PML4.lock();
    let mut pml4 = &mut pml4.0;
    let hhdm = HHDM.get().unwrap();

    unsafe {
        let pml3 =
            &mut *((((pml4[pml4_idx].addr() as u64) << 12) + hhdm) as *mut [PageDirectory; 512]); //or pdpt
        let pml2 =
            &mut *(((((*pml3)[pml3_idx].addr() as u64) << 12) + hhdm) as *mut [PageDirectory; 512]); //or pd
        let pml1 = &mut *(((((*pml2)[pml2_idx].addr() as u64) << 12) + hhdm)
            as *mut [PageTableEntry; 512]); // or pt
        ((pml1[pml1_idx].addr() << 12) as usize | offset) as *mut u8
    }
}
pub fn map_address(virtual_address: *mut u8, physical_address: *mut u8, flags: Flags) {
    let virtual_address = virtual_address as u64 & !0xfff;
    let physical_address = physical_address as u64 & !0xfff;
    let virtual_addr = bytemuck::cast::<_, VirtualAddress>(virtual_address as u64);
    let pml4_idx = virtual_addr.pml4() as usize;
    let pml3_idx = virtual_addr.pml3() as usize;
    let pml2_idx = virtual_addr.pml2() as usize;
    let pml1_idx = virtual_addr.pml1() as usize;
    let _offset = virtual_addr.offset() as usize;

    let mut pml4 = PML4.lock();
    let pml4 = &mut pml4.0;
    let hhdm = HHDM.get().unwrap();
    unsafe {
        if !pml4[pml4_idx].p() {
            let ptr = pmm::allocate();
            pml4[pml4_idx].set_addr(((ptr as u64) >> 12) as u32);
            pml4[pml4_idx].0 |= flags.bits();
        }
        let pml3 =
            &mut *((((pml4[pml4_idx].addr() as u64) << 12) + hhdm) as *mut [PageDirectory; 512]); //or pdpt

        if !pml3[pml3_idx].p() {
            let ptr = pmm::allocate();
            pml3[pml3_idx].set_addr(((ptr as u64) >> 12) as u32);
            pml3[pml3_idx].0 |= flags.bits();
        }

        let pml2 =
            &mut *(((((*pml3)[pml3_idx].addr() as u64) << 12) + hhdm) as *mut [PageDirectory; 512]); //or pd
        if !pml2[pml2_idx].p() {
            let ptr = pmm::allocate();
            pml2[pml2_idx].set_addr(((ptr as u64) >> 12) as u32);
            pml2[pml2_idx].0 |= flags.bits();
        }
        let pml1 = &mut *(((((*pml2)[pml2_idx].addr() as u64) << 12) + hhdm)
            as *mut [PageTableEntry; 512]); // or pt
        let page = &mut pml1[pml1_idx];
        page.set_addr(physical_address >> 12);
        page.0 |= flags.bits();
    }
}
pub fn map_pages(virtual_address: *mut u8, count: usize, flags: Flags) {
    for i in 0..count as u64 {
        let virt = (virtual_address as u64 + (i * 0x1000)) as *mut u8;
        let ptr = (pmm::allocate() as u64 + (i * 0x1000)) as *mut u8;
        map_address(virt, ptr, flags);
    }
}
#[repr(align(4096))]
struct Pml4([PageDirectory; 512]);

#[bitfield(u64, order = msb)]
#[derive(Zeroable, Pod)]
struct PageDirectory {
    xd: bool,
    #[bits(4)]
    pk: u8,
    #[bits(7)]
    avl2: u16,
    #[bits(12)]
    _reserverd: u32,
    #[bits(28)]
    addr: u32,
    #[bits(3)]
    avl1: u8,
    #[bits(3)]
    _reserved: u8,
    a: bool,
    pcd: bool,
    pwt: bool,
    us: bool,
    rw: bool,
    p: bool,
}

#[bitfield(u64, order = msb)]
#[derive(Zeroable, Pod)]
struct PageTable {
    xd: bool,
    #[bits(4)]
    pk: u8,
    #[bits(7)]
    avl2: u8,
    #[bits(12)]
    _reserverd: u32,
    #[bits(28)]
    addr: u32,
    #[bits(3)]
    avl1: u8,
    g: bool,
    pat: bool,
    d: bool,
    a: bool,
    pcd: bool,
    pwt: bool,
    us: bool,
    rw: bool,
    p: bool,
}
#[bitfield(u64, order = msb)]
struct PageTableEntry {
    xd: bool,
    #[bits(4)]
    mem_prot_key: u8,
    #[bits(7)]
    _ignored: u8,
    _reserved: bool,
    #[bits(39)]
    addr: u64,
    #[bits(3)]
    avl1: u8,
    g: bool,
    pat: bool,
    d: bool,
    a: bool,
    pcd: bool,
    pwt: bool,
    us: bool,
    rw: bool,
    p: bool,
}
bitflags! {
    #[derive(Clone, Copy)]
    pub struct Flags: u64 {
        const PRESENT =        0b00000000000000000000000000000001;
        const READ =           0b00000000000000000000000000000000;
        const WRITE=           0b00000000000000000000000000000010;
        const USER =           0b00000000000000000000000000000100;
        const SUPERVISOR =     0b00000000000000000000000000000000;
        const PWT =            0b00000000000000000000000000001000;
        const PCD =            0b00000000000000000000000000010000;
        const ACCESSED =       0b00000000000000000000000000100000;
        const DIRTY =          0b00000000000000000000000001000000;
        const PS =             0b00000000000000000000000010000000;
        const PAT =            0b00000000000000000000000100000000;
        const GLOBAL =         0b00000000000000000000001000000000;
        const XD =             0b10000000000000000000000000000000;
    }
}
fn get_cr3() -> u64 {
    let mut cr3;
    unsafe { core::arch::asm!("mov {}, cr3", out(reg)cr3) }
    cr3
}
fn set_cr3(cr3: u64) {
    unsafe { core::arch::asm!("mov cr3, {}", in(reg)cr3) }
}

unsafe extern "C" {
    fn replace_pml4(pml4: u64);
    fn enable_pae();
    fn reload_segments();
    fn reload_cs();
}

global_asm! {"
.globl enable_pae
.globl replace_pml4
.globl reload_segments

enable_pae:
    mov rdx, cr4
    mov rax, (1 << 5)
    mov rdx, rax
    mov cr4, rdx
replace_pml4:
    mov rax, rdi
    mov cr3, rax

reload_segments:
    mov ax, 0x10
    mov cs, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
reload_cs:
    push 0x08
    lea rax, [rip + reload_segments]
"}
