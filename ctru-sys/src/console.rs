use c_void;

use super::gfx::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ConsoleFont {
    pub gfx: *mut u8,
    pub asciiOffset: u16,
    pub numChars: u16,
}

pub type ConsolePrint = extern "C" fn(con: *mut c_void, c: i32) -> u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PrintConsole {
    pub font: ConsoleFont,
    pub frameBuffer: *mut u16,
    pub cursorX: i32,
    pub cursorY: i32,
    pub prevCursorX: i32,
    pub prevCursorY: i32,
    pub consoleWidth: i32,
    pub consoleHeight: i32,
    pub windowX: i32,
    pub windowY: i32,
    pub windowWidth: i32,
    pub windowHeight: i32,
    pub tabSize: i32,
    pub fg: i32,
    pub bg: i32,
    pub flags: i32,
    pub PrintChar: ConsolePrint,
    pub consoleInitialised: u8,
}

pub const CONSOLE_COLOR_BOLD: i32 = 1;
pub const CONSOLE_COLOR_FAINT: i32 = 2;
pub const CONSOLE_ITALIC: i32 = 4;
pub const CONSOLE_UNDERLINE: i32 = 8;
pub const CONSOLE_BLINK_SLOW: i32 = 16;
pub const CONSOLE_BLINK_FAST: i32 = 32;
pub const CONSOLE_COLOR_REVERSE: i32 = 64;
pub const CONSOLE_CONCEAL: i32 = 128;

#[repr(C)]
pub enum debugDevice {
    NULL = 0,
    _3DMOO = 1,
    CONSOLE = 2,
}


extern "C" {
    pub fn consoleSetFont(console: *mut PrintConsole, font: *mut ConsoleFont) -> ();
    pub fn consoleSetWindow(console: *mut PrintConsole,
                            x: i32,
                            y: i32,
                            width: i32,
                            height: i32)
                            -> ();
    pub fn consoleGetDefault() -> *mut PrintConsole;
    pub fn consoleSelect(console: *mut PrintConsole) -> *mut PrintConsole;
    pub fn consoleInit(screen: gfxScreen_t, console: *mut PrintConsole) -> *mut PrintConsole;
    pub fn consoleDebugInit(device: debugDevice) -> ();
    pub fn consoleClear() -> ();
}
