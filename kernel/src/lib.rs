#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(str_from_raw_parts)]
pub mod acpi;
pub mod arch;
pub mod console;
pub mod elf;
pub mod ipc;
pub mod mem;
pub mod process;
pub mod requests;
pub mod scheduler;
pub mod timers;
extern crate alloc;
