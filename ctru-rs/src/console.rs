use std::cell::RefMut;
use std::default::Default;

use ctru_sys::{consoleClear, consoleInit, consoleSelect, consoleSetWindow, PrintConsole};

use crate::gfx::Screen;

pub struct Console<'screen> {
    context: Box<PrintConsole>,
    screen: RefMut<'screen, dyn Screen>,
}

impl<'screen> Console<'screen> {
    /// Initialize a console on the chosen screen, overwriting whatever was on the screen
    /// previously (including other consoles). The new console is automatically selected for
    /// printing.
    pub fn init(screen: RefMut<'screen, dyn Screen>) -> Self {
        let mut context = Box::new(PrintConsole::default());

        unsafe { consoleInit(screen.as_raw(), context.as_mut()) };

        Console {
            context,
            screen,
        }
    }

    /// Select this console as the current target for stdout
    pub fn select(&self) {
        unsafe {
            consoleSelect(self.context.as_ref() as *const _ as *mut _);
        }
    }

    /// Clears all text from the console
    pub fn clear(&self) {
        unsafe { consoleClear() }
    }

    /// Resizes the active console to fit in a smaller portion of the screen.
    ///
    /// The first two arguments are the desired coordinates of the top-left corner
    /// of the console, and the second pair is the new width and height
    ///
    /// # Safety
    /// This function is unsafe because it does not validate that the input will produce
    /// a console that actually fits on the screen
    pub unsafe fn set_window(&mut self, x: i32, y: i32, width: i32, height: i32) {
        consoleSetWindow(self.context.as_mut(), x, y, width, height);
    }
}

impl Drop for Console<'_> {
    fn drop(&mut self) {
        static mut EMPTY_CONSOLE: PrintConsole = unsafe { const_zero::const_zero!(PrintConsole) };

        unsafe {
            // Safety: We are about to deallocate the PrintConsole data pointed
            // to by libctru. Without this drop code libctru would have a
            // dangling pointer that it writes to on every print. To prevent
            // this we replace the console with an empty one if it was selected.
            // This is the same state that libctru starts up in, before
            // initializing a console. Writes to the console will not show up on
            // the screen, but it won't crash either.

            // Get the current console by replacing it with an empty one.
            let current_console = ctru_sys::consoleSelect(&mut EMPTY_CONSOLE);

            if std::ptr::eq(current_console, &*self.context) {
                // Console dropped while selected. We just replaced it with the
                // empty console so nothing more to do.
            } else {
                // Console dropped while a different console was selected. Put back
                // the console that was selected.
                ctru_sys::consoleSelect(current_console);
            }
        }
    }
}
