use core::{
    fmt::{Arguments, Write},
    ptr::null_mut,
};

use flanterm::sys::*;
use spin::*;

static TERMINAL: Mutex<Terminal> = Mutex::new(Terminal { ctx: null_mut() });
#[doc(hidden)]
pub fn _print(args: Arguments) {
    interrupts::without(|| {
        let mut terminal = TERMINAL.lock();
        terminal
            .write_fmt(args)
            .expect("failed to write in console");
    });
}

struct Terminal {
    ctx: *mut flanterm_context,
}
unsafe impl Send for Terminal {}

impl Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            flanterm_write(self.ctx, s.as_ptr() as *const i8, s.len());
        }
        Ok(())
    }
}
#[macro_export]
macro_rules! kprintln {
    () => {
        $crate::console::_print(format_args!("\n"));
    };
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!("{}\n", format_args!($($arg)*)));
    };
}

#[macro_export]
macro_rules! kprint {
    () => {};
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!($($arg)*))
    };
}

pub fn init(fb: &limine::framebuffer::Framebuffer) {
    let mut terminal = TERMINAL.lock();
    terminal.ctx = unsafe {
        flanterm_fb_init(
            None,
            None,
            fb.address() as *mut u32,
            fb.width as usize,
            fb.height as usize,
            fb.pitch as usize,
            fb.red_mask_size,
            fb.red_mask_shift,
            fb.green_mask_size,
            fb.green_mask_shift,
            fb.blue_mask_size,
            fb.blue_mask_shift,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            0,
            1,
            0,
            0,
            0,
        )
    };
}
