use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use bitfields::bitfield;

use crate::{
    arch::x86_64::{inb, outb, pic},
    kprint, kprintln,
};

pub fn init() {
    //init_pit();
    kprintln!("pit");
    write_pit((1193182 / 1000) as u16);
}
fn irq0_handler() {
    unsafe {
        outb(0x20, 0x20);
    }
}
fn init_pit() {
    let desired_frq = 1000;
    let slowest_frq = 0x10000;
    let pit_reload_value = 0;
    let irq0_freq = 0;
    let time_in_ms = pit_reload_value / (3579545 / 3) * 1000;
    if desired_frq <= 18 {}
    if desired_frq >= 1193181 {}

    unsafe {
        outb(0x43, 0x36);
        outb(0x40, pit_reload_value as u8);
        outb(0x40, (pit_reload_value >> 8) as u8);
    }
}
#[bitfield(u8)]
struct ControlWordFormat {
    bcd: bool,
    m0: bool,
    m1: bool,
    m2: bool,
    rl0: bool,
    rl1: bool,
    sc0: bool,
    sc1: bool,
}

const HZ: u32 = 1193182;
const CHANNEL_0: u8 = 0x40;
const CHANNEL_1: u8 = 0x41;
const CHANNEL_2: u8 = 0x42;
const MODE: u8 = 0x43; // command register

pub fn play_sound(freq: u32) {
    let mut div: u32;
    let mut tmp: u8;
    div = 1193180 / freq;
    unsafe {
        outb(0x43, 0xb6);
        outb(0x42, div as u8);
        outb(0x42, (div >> 8) as u8);
        tmp = inb(0x61);
        if tmp != (tmp | 3) {
            outb(0x61, tmp | 3);
        }
    }
}
fn write_pit(reload_value: u16) {
    unsafe {
        outb(0x43, 0b00110100);
        outb(0x40, reload_value as u8);
        outb(0x40, (reload_value >> 8) as u8);
    }
}
fn read_pit_count() -> u32 {
    unsafe {
        core::arch::asm!("cli");

        outb(0x43, 0b0000000);

        let low = inb(0x40) as u32;
        let high = (inb(0x40) as u32) << 8;
        core::arch::asm!("sti");

        high | low
    }
}

pub fn sleep(ms: u32) {
    COUNTDOWN.store(ms, Ordering::Relaxed);
    while COUNTDOWN.load(Ordering::Relaxed) > 0 {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)) };
    }
}

static TICKS: AtomicU32 = AtomicU32::new(0);
static COUNTDOWN: AtomicU32 = AtomicU32::new(0);

pub fn timer_irq() {
    TICKS.fetch_add(1, Ordering::Relaxed);
    let count = COUNTDOWN.load(Ordering::Relaxed);
    if count > 0 {
        COUNTDOWN.store(count - 1, Ordering::Relaxed);
    }
}
pub fn get_ticks() -> u32 {
    TICKS.load(Ordering::Relaxed)
}
fn timer_done() {}
