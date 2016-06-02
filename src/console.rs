use libctru::console::{consoleInit, consoleClear};
use libctru::gfx;

use core::default::Default;
use core::marker::PhantomData;
use core::ptr;

extern "C" {
    fn putchar(ch: u8) -> i32;
}

pub struct Console {
    pd: PhantomData<()>,
}

impl Console {
    pub fn print<'a>(&mut self, s: &'a str) {
        unsafe {
            for ch in s.as_bytes().iter() {
                putchar(*ch);
            }
        }
    }

    pub fn println<'a>(&mut self, s: &'a str) {
        unsafe {
            self.print(s);
            putchar('\n' as u8);
        }
    }

    pub fn clear(&mut self) {
        unsafe { consoleClear() }
    }
}

impl Default for Console {
    fn default() -> Self {
        unsafe { consoleInit(gfx::gfxScreen_t::GFX_TOP, ptr::null_mut()); }
        Console { pd: PhantomData }
    }
}
