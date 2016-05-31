use libctru::console::{PrintConsole, consoleInit, consoleClear};
use libctru::gfx;

use core::ptr;

extern "C" {
    fn putchar(ch: u8) -> i32;
}

pub fn console_default_init() -> *mut PrintConsole {
    unsafe { consoleInit(gfx::gfxScreen_t::GFX_TOP, ptr::null_mut()) }
}

pub fn console_write<'a>(s: &'a str) {
    unsafe {
        for c in s.as_bytes().iter() {
            putchar(*c);
        }
    }
}

pub fn console_writeln<'a>(s: &'a str) {
    unsafe {
        console_write(s);
        putchar('\n' as u8);
    }
}

pub fn console_clear() {
    unsafe { consoleClear() }
}
