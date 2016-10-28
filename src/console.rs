use std::default::Default;
use std::ptr;

use gfx::Screen;

use libctru::console::*;

pub struct Console {
    context: PrintConsole,
}

impl Console {
    pub fn init(screen: Screen) -> Self {
        unsafe {
            let ret = *(consoleInit(screen.into(), ptr::null_mut()));
            Console { context: ret }
        }
    }

    pub fn select(&mut self) {
        unsafe { consoleSelect(&mut self.context); }
    }

    pub fn set_window(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { consoleSetWindow(&mut self.context, x, y, width, height) }
    } 

    pub fn clear(&mut self) {
        unsafe { consoleClear() }
    }
}

impl Default for Console {
    fn default() -> Self {
        Console::init(Screen::Top)
    }
}
