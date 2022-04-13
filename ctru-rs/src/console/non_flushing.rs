//! This is a bit of a hack, but allows us to reuse most of the libctru console
//! implementation without writing one from scratch. This lets the user control
//! when the console characters are flushed to the screen, which can help prevent
//! deadlocks while using the console and graphics simultaneously.

use ctru_sys::PrintConsole;

/// Print a character to the console, without flushing the graphics buffers.
/// Falls back to the default implementation for any characters other than
/// `\n` or `\r`, which are the only ones that normally result in a flush.
pub extern "C" fn print_char(console: *mut libc::c_void, c: i32) -> bool {
    let console = unsafe { &mut *(console.cast::<PrintConsole>()) };

    let ret = match c {
        // '\r'
        10 => {
            new_row(console);
            console.cursorX = 0;
            true
        }
        // '\n'
        13 => {
            console.cursorX = 0;
            true
        }
        _ => false, // use default console printer
    };

    if console.cursorX >= console.windowWidth {
        console.cursorX = 0;
        new_row(console);
        true
    } else {
        ret
    }
}

/// Direct port of
/// <https://github.com/devkitPro/libctru/blob/master/libctru/source/console.c#L724>
fn new_row(console: &mut PrintConsole) {
    console.cursorY += 1;

    if console.cursorY >= console.windowHeight {
        console.cursorY -= 1;
        let idx = (console.windowX * 8 * 240) + (239 - (console.windowY * 8));

        unsafe {
            let mut dst: *mut u16 = console.frameBuffer.offset(idx as _);
            let mut src: *mut u16 = dst.offset(-8);

            for _ in 0..console.windowWidth * 8 {
                let mut from: *mut u32 = (src as i32 & !3) as _;
                let mut to: *mut u32 = (dst as i32 & !3) as _;
                for _ in 0..(((console.windowHeight - 1) * 8) / 2) {
                    *to = *from;
                    to = to.offset(-1);
                    from = from.offset(-1);
                }

                dst = dst.offset(240);
                src = src.offset(240);
            }
        }

        clear_line(console);
    }
}

// HACK: this is technically an implementation detail of libctru's console,
//  but it has a that we can link to. We use this so we don't need to re-implement
// the entire call stack down from clear_line just to print some spaces characters.
extern "C" {
    fn consolePrintChar(c: i32);
}

/// Direct port of `consoleClearLine('2')`, minus the flush at the end.
/// <https://github.com/devkitPro/libctru/blob/master/libctru/source/console.c#L231>
fn clear_line(console: &mut PrintConsole) {
    let col_tmp = console.cursorX;

    console.cursorX = 0;

    for _ in 0..console.windowWidth {
        unsafe {
            consolePrintChar(b' ' as i32);
        }
    }

    console.cursorX = col_tmp;
}
