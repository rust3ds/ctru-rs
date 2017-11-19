use std::default::Default;
use std::ptr;

use gfx::Screen;

pub struct Console {
    context: ::libctru::PrintConsole,
}

impl Console {
    pub fn init(screen: Screen) -> Self {
        unsafe {
            let context = ptr::read(::libctru::consoleInit(screen.into(), ptr::null_mut()));
            Console { context, }
        }
    }

    pub fn select(&mut self) {
        unsafe { ::libctru::consoleSelect(&mut self.context); }
    }

    pub fn set_window(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { ::libctru::consoleSetWindow(&mut self.context, x, y, width, height) }
    } 

    pub fn clear(&mut self) {
        unsafe { ::libctru::consoleClear() }
    }
}

impl Default for Console {
    fn default() -> Self {
        Console::init(Screen::Top)
    }
}
