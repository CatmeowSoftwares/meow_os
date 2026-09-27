use spin::Once;

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
pub static APIC: Once<u64> = Once::new();
static APIC_TIMER_FREQ: Once<u64> = Once::new();
pub(crate) fn init(madt: &AcpiSdtHeader) {
    let madt = unsafe { &*(madt as *const AcpiSdtHeader as *const Madt) };
    kprintln!("{:#x}", madt.lapic_address);
    let mut msr_lower: u32;
    let mut msr_upper: u32;
    unsafe {
        core::arch::asm!("
            rdmsr
        ", in("ecx") 0x1b, out("edx") msr_upper, out("eax") msr_lower, options(nomem, nostack, preserves_flags));
    }
    let apic_timer = (((msr_upper as u64) << 32) | msr_lower as u64) & 0x000f_ffff_ffff_f000; //only 12-51 bits

    kprintln!("apic_timer: {apic_timer:#x}");
    APIC.call_once(|| apic_timer);
    let fadt = crate::acpi::fadt::FADT.get().unwrap();
    map_address(
        apic_timer as _,
        apic_timer as _,
        Flags::PRESENT | Flags::WRITE,
    );
    unsafe {
        let div_config_ptr = PhysicalAddress::<u32>::map_raw(apic_timer + DIVIDE_CONFIGURATION_REG);
        *div_config_ptr = 0x03;
        let init_count_reg_ptr = PhysicalAddress::<u32>::map_raw(apic_timer + INITIAL_COUNT_REG);

        let current_count_reg_ptr = PhysicalAddress::<u32>::map_raw(apic_timer + CURRENT_COUNT_REG);
        *init_count_reg_ptr = u32::MAX;
        const SAMPLES: usize = 10;
        let mut arr: [u64; SAMPLES] = [0; SAMPLES];
        kprintln!("calibrating apic timer");
        for i in 0..SAMPLES {
            *init_count_reg_ptr = u32::MAX;
            fadt.sleep(1000);
            let val = *init_count_reg_ptr - *current_count_reg_ptr;
            arr[i] = val as u64;
        }

        let mut sum = 0;
        for n in arr {
            sum += n;
        }
        sum /= SAMPLES as u64;
        *init_count_reg_ptr = sum as u32;
        APIC_TIMER_FREQ.call_once(|| sum);
        kprintln!("sum: {sum}");

        let lvt_timer_reg = PhysicalAddress::<u32>::map_raw(apic_timer + LVT_TIMER_REG);
        *lvt_timer_reg = (*lvt_timer_reg & !0xff) | 0x20 | (1 << 17);
        *lvt_timer_reg &= !(1 << 16);
        let siv_reg = PhysicalAddress::<u32>::map_raw(apic_timer + SPURIOUS_INTERRUPT_VECTOR_REG);
        *siv_reg |= 0x1ff;
    }
    kprintln!("fadt sleep 1s");
    fadt.sleep(1000);
    kprintln!("fadt slep 1s");
}
#[unsafe(naked)]
extern "C" fn isr(apic: u64) {
    core::arch::naked_asm!("
        mov [rdi + {}], 0
        iret
    ", const EOI_REG);
}
const LAPIC_ID_REG: u64 = 0x20;
const LAPIC_VERSION_REG: u64 = 0x30;
const TASK_PRIORITY_REG: u64 = 0x80;
const ARBITRATION_PRIORITY_REG: u64 = 0x90;
const PROCESSOR_PRIORITY_REG: u64 = 0xa0;
pub const EOI_REG: u64 = 0xb0;
const REMOTE_READ_REG: u64 = 0xc0;
const LOGICAL_DESTIONATION_REG: u64 = 0xd0;
const DESTINATION_FORMAT_REG: u64 = 0xe0;
const SPURIOUS_INTERRUPT_VECTOR_REG: u64 = 0xf0;
const IN_SERVICE_REG: u64 = 0x100;
const TRIGGER_MODE_REG: u64 = 0x180;
const INTERRUPT_REQUEST_REG: u64 = 0x200;
const ERROR_STATUS_REG: u64 = 0x280;
const LVT_CORRECTED_MACHINE_CHECK_INTERRUPT_REG: u64 = 0x2f0;
const INTERRUPT_COMMAND_REG: u64 = 0x300;
const LVT_TIMER_REG: u64 = 0x320;
const LVT_THERMAL_SENSOR_REG: u64 = 0x330;
const LVT_PERFORMANCE_MONITORING_REG: u64 = 0x340;
const LVT_LINT0_REG: u64 = 0x350;
const LVT_LINT1_REG: u64 = 0x360;
const LVT_ERROR_REG: u64 = 0x370;
const INITIAL_COUNT_REG: u64 = 0x380; // for timer
const CURRENT_COUNT_REG: u64 = 0x390; // for timer
const DIVIDE_CONFIGURATION_REG: u64 = 0x3e0; // for timer
