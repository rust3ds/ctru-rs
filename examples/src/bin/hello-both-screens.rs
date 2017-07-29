extern crate ctru;

use ctru::gfx::{Gfx, Screen};
use ctru::console::Console;
use ctru::services::apt::Apt;
use ctru::services::hid::{self, Hid};

fn main() {
    // Initialize services
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let mut gfx = Gfx::default();

    // Start a console on the top screen
    let mut top_screen = Console::init(Screen::Top);

    // Start a console on the bottom screen.
    // The most recently initialized console will be active by default
    let mut bottom_screen = Console::init(Screen::Bottom);

    // Let's print on the top screen first
    top_screen.select();
    println!("This is the top screen! We have a lot of space up here!");

    // Now let's print something on the bottom screen
    bottom_screen.select();
    println!("\x1b[14;00HThis is the bottom screen.");
    println!("There's not as much space down here, but that's okay.");

    top_screen.select();
    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();

        hid.scan_input();
        if hid.keys_down().contains(hid::KEY_START) {
            break;
        }
    }
}
