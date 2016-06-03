use libctru::console::{consoleInit, consoleClear};
use libctru::gfx;
use libctru::libc;

use core::fmt::{self, Write};
use core::default::Default;
use core::marker::PhantomData;
use core::ptr;

pub struct Console {
    pd: PhantomData<()>,
}

impl Console {
    pub fn clear(&mut self) {
        unsafe { consoleClear() }
    }
}

impl Default for Console {
    fn default() -> Self {
        unsafe {
            consoleInit(gfx::gfxScreen_t::GFX_TOP, ptr::null_mut());
        }
        Console { pd: PhantomData }
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ret = unsafe { libc::write(libc::STDOUT_FILENO, s.as_ptr() as *const _, s.len()) };
        if ret == s.len() as isize {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}
