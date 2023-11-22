//! Hello World example using both screens.
//!
//! This is similar to the `hello-world` example, with the main difference of using 2 virtual `Console`s that can be alternated to print on both screens.

use ctru::prelude::*;

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();

    // Start a console on the top screen
    let top_screen = Console::new(gfx.top_screen.borrow_mut());

    // Start a console on the bottom screen.
    // The most recently initialized console will be active by default.
    let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());

    // Let's print on the top screen first.
    // Since the bottom screen is currently selected (being created afterwards), it is required to select the top screen console first.
    top_screen.select();
    println!("This is the top screen! We have a lot of space up here!");

    // Now let's print something on the bottom screen.
    bottom_screen.select();
    println!("\x1b[14;00HThis is the bottom screen.");
    println!("There's not as much space down here, but that's okay.");

    top_screen.select();
    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}
