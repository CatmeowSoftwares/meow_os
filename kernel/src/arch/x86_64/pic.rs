use crate::arch::x86_64::{inb, outb};

const PIC1: u8 = 0x20;
const PIC2: u8 = 0xa0;
const PIC1_COMMAND: u16 = PIC1 as u16;
const PIC1_DATA: u16 = PIC1 as u16 + 1;
const PIC2_COMMAND: u16 = PIC2 as u16;
const PIC2_DATA: u16 = PIC2 as u16 + 1;

const PIC_EOI: u8 = 0x20;

const ICW1_ICW4: u8 = 0x01;
const ICW1_SINGLE: u8 = 0x02;
const ICW1_INTERVAL4: u8 = 0x04;
const ICW1_LEVEL: u8 = 0x08;
const ICW1_INIT: u8 = 0x10;

const ICW4_8086: u8 = 0x01;
const ICW4_AUTO: u8 = 0x02;
const ICW4_BUF_SLAVE: u8 = 0x08;
const ICW4_BUF_MASTER: u8 = 0x0C;
const ICW4_SFNM: u8 = 0x10;

const CASCADE_IRQ: u8 = 2;

fn io_wait() {
    unsafe {
        outb(0x80, 0);
    }
}

fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_COMMAND, PIC_EOI);
        }
        outb(PIC1_COMMAND, PIC_EOI);
    }
}

pub fn remap(offset1: i32, offset2: i32) {
    unsafe {
        outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_wait();
        outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_wait();
        outb(PIC1_DATA, offset1 as u8);
        io_wait();
        outb(PIC2_DATA, offset2 as u8);
        io_wait();
        outb(PIC1_DATA, 1 << CASCADE_IRQ);
        io_wait();
        outb(PIC2_DATA, CASCADE_IRQ);
        io_wait();

        outb(PIC1_DATA, ICW4_8086);
        io_wait();
        outb(PIC2_DATA, ICW4_8086);
        io_wait();

        outb(PIC1_DATA, 0);
        outb(PIC2_DATA, 0);
    }
}
fn disable() {
    unsafe {
        outb(PIC1_DATA, 0xff);
        outb(PIC2_DATA, 0xff);
    }
}

pub fn irq_set_mask(irq_line: u8) {
    let port: u16;
    let value: u8;
    let mut irq_line = irq_line;
    if irq_line < 8 {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq_line -= 8;
    }
    unsafe {
        value = inb(port) | (1 << irq_line);
        outb(port, value);
    }
}

pub fn irq_clear_mask(irq_line: u8) {
    let port: u16;
    let value: u8;
    let mut irq_line = irq_line;
    if irq_line < 8 {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq_line -= 8;
    }
    unsafe {
        value = inb(port) & !(1 << irq_line);
        outb(port, value);
    }
}
