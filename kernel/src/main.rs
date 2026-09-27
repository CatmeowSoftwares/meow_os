#![no_std]
#![no_main]

use core::{arch, panic::PanicInfo};
use kernel::{console, kprintln, requests::*};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("{info}");
    loop {
        #[cfg(target_arch = "x86_64")]
        kernel::arch::x86_64::pit::play_sound(1000);
        unsafe { core::arch::asm!("hlt") };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    if let Some(fb_response) = FRAMEBUFFER_REQUEST.response() {
        if let Some(fb) = fb_response.framebuffers().first() {
            console::init(fb);
        }
    }
    kprintln!("hi");
    kprintln!("hello");
    kernel::arch::init();
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
