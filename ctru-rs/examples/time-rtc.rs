//! Time Clock example.
//!
//! This example showcases how to retrieve the local time set in the console's configuration
//! using the implementations of the standard library.

use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let _console = Console::new(gfx.top_screen.borrow_mut());

    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Technically, this actually just gets local time and assumes it's UTC,
        // since the 3DS doesn't seem to support timezones.
        let cur_time = time::OffsetDateTime::now_utc();

        // Display the retrieved information.
        println!("\x1b[1;1H{cur_time}");

        gfx.wait_for_vblank();
    }
}
