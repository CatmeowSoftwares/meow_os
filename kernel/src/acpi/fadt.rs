use spin::Once;

use crate::{acpi::AcpiSdtHeader, arch::x86_64::inl, kprintln};

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct GenericAddressStructure {
    address_space: u8,
    bit_width: u8,
    bit_offset: u8,
    access_size: u8,
    address: u64,
}
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Fadt {
    h: AcpiSdtHeader,
    firmware_ctrl: u32,
    dsdt: u32,
    reserved: u8,
    preferred_power_management_profile: u8,
    sci_interrupt: u16,
    smi_command_port: u32,
    acpi_enable: u8,
    acpi_disable: u8,
    s4bios_req: u8,
    pstate_cnt: u8,
    pm1a_event_block: u32,
    pm1b_event_block: u32,
    pm1a_control_block: u32,
    pm1b_control_block: u32,
    pm2_control_block: u32,
    pm_timer_block: u32,
    gpe0_block: u32,
    gpe1_block: u32,
    pm1_event_length: u8,
    pm1_contol_length: u8,
    pm2_contol_length: u8,
    pm_timer_length: u8,
    gpe0_length: u8,
    gpe1_length: u8,
    gpe1_base: u8,
    cstate_control: u8,
    worst_c2_latency: u16,
    worst_c3_latency: u16,
    flush_size: u16,
    flush_stride: u16,
    duty_offset: u8,
    duty_width: u8,
    day_alarm: u8,
    month_alarm: u8,
    century: u8,

    boot_architecture_flags: u16,
    reserved2: u8,
    flags: u32,
    reset_reg: GenericAddressStructure,
    reset_value: u8,
    reserved3: [u8; 3],
    x_firmware_control: u64,
    x_dsdt: u64,
    x_pm1a_event_block: GenericAddressStructure,
    x_pm1b_event_block: GenericAddressStructure,
    x_pm1a_control_block: GenericAddressStructure,
    x_pm1b_control_block: GenericAddressStructure,
    x_pm2_control_block: GenericAddressStructure,
    x_pmt_timer_block: GenericAddressStructure,
    x_gpe0_block: GenericAddressStructure,
    x_gpe1_block: GenericAddressStructure,
}
pub static FADT: Once<Fadt> = Once::new();
impl Fadt {
    fn is_timer_32_bit(&self) -> bool {
        self.flags & (1 << 8) != 0
    }
    fn timer_mask(&self) -> u32 {
        if self.is_timer_32_bit() {
            0xffffffff
        } else {
            0x00ffffff
        }
    }
    pub fn read_timer(&self) -> u32 {
        unsafe { inl(self.pm_timer_block as _) & self.timer_mask() }
    }
    pub fn sleep(&self, ms: u32) {
        let mask = self.timer_mask();
        let ticks_needed = (ms as u64 * 3579545) / 1000;

        let mut last = self.read_timer();
        let mut elapsed: u64 = 0;
        let mut prev = last;
        while elapsed < ticks_needed {
            let now = self.read_timer();
            let delta = now.wrapping_sub(prev) & mask;
            elapsed += delta as u64;
            prev = now;
        }
    }
}
pub(crate) fn init(fadt: &AcpiSdtHeader) {
    unsafe {
        kprintln!("checking apic: {}", check_apic());
        let fadt = &*(fadt as *const AcpiSdtHeader as *const Fadt);
        FADT.call_once(|| *fadt);
        let length = fadt.pm_timer_length;
        kprintln!("pm timer length: {length}");
        kprintln!("{}", fadt.h.revision);
        kprintln!("is 32 bit?: {}", fadt.is_timer_32_bit());
        kprintln!("sleeping for 1s");
        fadt.sleep(1000);
        kprintln!("slept for 1s");
    }
}

fn check_apic() -> bool {
    let mut edx: u32;
    unsafe {
        core::arch::asm!("
        mov eax, 1
        cpuid"
    ,out("edx") edx);
    }
    edx & (1 << 9) != 0
}

const IA32_APIC_BASE_MSR: u32 = 0x1b;
const IA32_APIC_BASE_MSR_BSP: u16 = 0x100;
const IA32_APIC_BASE_MSR_ENABLE: u32 = 0x800;
fn cpu_get_apic_base() -> u64 {
    let mut msr_lower: u32;
    let mut msr_upper: u32;
    unsafe {
        core::arch::asm!("
            rdmsr
        ", in("ecx") IA32_APIC_BASE_MSR, out("edx") msr_upper, out("eax") msr_lower, options(nomem, nostack, preserves_flags));
    }
    kprintln!("msr_upper: {msr_upper:#x}");
    kprintln!("msr_lower: {msr_lower:#x}");
    let apic_timer = ((msr_upper as u64) << 32) | msr_lower as u64;
    apic_timer
}
fn cpu_set_apic_base(apic: u64) {
    let edx: u32;
    let eax: u32 = (apic & 0xfffff0000) as u32 | IA32_APIC_BASE_MSR_ENABLE;
    edx = (apic >> 32) as u32 & 0x0f;
    unsafe {
        core::arch::asm!("wrmsr", in("ecx")IA32_APIC_BASE_MSR, in("edx") edx, in("eax") eax);
    }
}
fn enable_apic() {
    let apic = cpu_get_apic_base();
    cpu_set_apic_base(apic);
}
