use spin::Mutex;

pub(crate) static TSS: Mutex<Tss> = Mutex::new(Tss::new());

#[repr(C)]
pub(crate) struct Tss {
    reserved1: u32,
    pub(crate) rsp0: u64,
    pub(crate) rsp1: u64,
    pub(crate) rsp2: u64,
    reserved2: u64,
    pub(crate) ist1: u64,
    pub(crate) ist2: u64,
    pub(crate) ist3: u64,
    pub(crate) ist4: u64,
    pub(crate) ist5: u64,
    pub(crate) ist6: u64,
    pub(crate) ist7: u64,
    reserved3: u64,
    reserved4: u16,
    pub(crate) iopb: u16,
}

impl Tss {
    const fn new() -> Self {
        Self {
            reserved1: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            reserved2: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            reserved3: 0,
            reserved4: 0,
            iopb: 0,
        }
    }
}
pub(crate) fn init() {
    unsafe {
        core::arch::asm!(
            "
            mov ax, 0x28
            ltr ax
        "
        )
    }
}
