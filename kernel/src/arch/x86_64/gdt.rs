use super::tss::{TSS, Tss};
use crate::kprintln;
use bitfields::bitfield;
use bitflags::{Flag, bitflags};
use spin::Mutex;
#[repr(C)]
struct Gdt([u64; 7]);

#[bitfield(u64)]
struct GdtEntry {
    limit1: u16,
    base1: u16,
    base2: u8,
    access: u8,
    #[bits(4)]
    limit2: u8,
    #[bits(4)]
    flags: u8,
    base3: u8,
}
impl GdtEntry {
    fn encode(base: u32, limit: u32, access: Access, flags: Flags) -> u64 {
        GdtEntryBuilder::new()
            .with_base1(base as u16 & 0xffff)
            .with_base2((base >> 16) as u8 & 0xff)
            .with_base3((base >> 24) as u8 & 0xff)
            .with_limit1(limit as u16 & 0xffff)
            .with_limit2((limit >> 16) as u8 & 0xff)
            .with_access(access.bits())
            .with_flags(flags.bits())
            .build()
            .into_bits()
    }
    fn encode_tss(base: u64, limit: u32, access: SystemAccess, flags: Flags) -> (u64, u64) {
        let tss1 = Self::encode(
            (base & 0xffffffff) as u32,
            limit,
            Access::from_bits(access.bits()).unwrap(),
            flags,
        );
        let tss2 = base >> 32 & 0xffffffff;
        (tss1, tss2)
    }
}

bitflags! {
    struct Flags: u8 {
        const L = 0b0010;
        const DB = 0b0100;
        const G = 0b1000;
    }
}
bitflags! {
    struct Access: u8 {
        const A  = 0b00000001;
        const RW = 0b00000010;
        const DC = 0b00000100;
        const E  = 0b00001000;
        const S  = 0b00010000;
        const R0 = 0b00000000;
        const R1 = 0b00100000;
        const R2 = 0b01000000;
        const R3 = 0b01100000;
        const P  = 0b10000000;
    }
}
bitflags! {
    struct SystemAccess: u8 {
        const LDT = 0x2;

        /// 32 bit
        const BIT16_AVAILABLE = 0b00000001;
        const BIT16_BUSY = 0x3;
        const BIT32_AVAILABLE = 0x9;
        const BIT32_BUSY = 0xb;

        // 64 bit
        const BIT64_AVAILABLE = 0x9;
        const BIT64_BUSY = 0xb;



        const S  = 0b00010000;
        const R0 = 0b00000000;
        const R1 = 0b00100000;
        const R2 = 0b01000000;
        const R3 = 0b01100000;
        const P  = 0b10000000;
    }
}

#[repr(C, packed)]
struct Gdtr {
    size: u16,
    offset: u64,
}
static GDTR: Mutex<Gdtr> = Mutex::new(Gdtr { size: 0, offset: 0 });
static GDT: Mutex<Gdt> = Mutex::new(Gdt([0; 7]));
pub fn init() {
    let mut gdt = GDT.lock();
    let tss = TSS.lock();

    let kernel_code_flags = Flags::L | Flags::G;
    let kernel_data_flags = Flags::G | Flags::DB;
    let user_code_flags = Flags::L | Flags::G;
    let user_data_flags = Flags::L | Flags::G;
    let tss_flags = Flags::empty();

    //0b10011010
    let kernel_code_access = Access::P | Access::S | Access::E | Access::RW;
    //0b10010010
    let kernel_data_access = Access::P | Access::S | Access::RW;
    let user_code_access = Access::P | Access::R3 | Access::S | Access::RW;
    let user_data_access = Access::P | Access::R3 | Access::S | Access::E | Access::RW;
    let tss_access = SystemAccess::P | SystemAccess::BIT64_AVAILABLE;

    let null_desc = 0;
    let kernel_code = GdtEntry::encode(0, 0xfffff, kernel_code_access, kernel_code_flags);
    let kernel_data = GdtEntry::encode(0, 0xfffff, kernel_data_access, kernel_data_flags);
    let user_code = GdtEntry::encode(0, 0xfffff, user_code_access, user_code_flags);
    let user_data = GdtEntry::encode(0, 0xfffff, user_data_access, user_data_flags);
    let (tss1, tss2) =
        GdtEntry::encode_tss(&*tss as *const Tss as u64, 0xfffff, tss_access, tss_flags);

    gdt.0 = [
        null_desc,
        kernel_code,
        kernel_data,
        user_code,
        user_data,
        tss1,
        tss2,
    ];
    let mut gdtr = GDTR.lock();
    gdtr.offset = &gdt.0 as *const _ as u64;
    gdtr.size = size_of::<Gdt>() as u16 - 1;
    kprintln!("starting to load gdt");
    unsafe {
        core::arch::asm!("
        lgdt [{}]
        
    ", in(reg) &*gdtr)
    }
    kprintln!("reloading");
    unsafe {
        core::arch::asm!(
            "
            push 0x08
            lea rax, [rip + 2f]
            push rax
            retfq
            2:
            mov ax, 0x10
            mov ds, ax
            mov es, ax
            mov fs, ax
            mov gs, ax
            mov ss, ax
        "
        );
    }
    kprintln!("GDT INIT");
}
