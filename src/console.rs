use libctru::console::{PrintConsole, consoleInit};
use libctru::gfx;
use rcstring::CString;

use core::ptr;

extern "C" {
    fn puts(cstr: *const u8) -> u8;
}

pub fn console_default_init() -> *mut PrintConsole {
    unsafe { consoleInit(gfx::gfxScreen_t::GFX_TOP, ptr::null_mut()) }
}

pub fn console_write<'a>(s: &'a str) -> u8 {
    unsafe { puts(CString::new(s).unwrap().into_raw()) }
}
