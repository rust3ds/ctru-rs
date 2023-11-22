//! Time Clock example.
//!
//! This example showcases how to retrieve the local time set in the console's configuration
//! using the implementations of the standard library.

use ctru::prelude::*;

fn main() {
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

        let hours = cur_time.hour();
        let minutes = cur_time.minute();
        let seconds = cur_time.second();

        let weekday = cur_time.weekday().to_string();
        let month = cur_time.month().to_string();
        let day = cur_time.day();
        let year = cur_time.year();

        println!("\x1b[1;1H{hours:0>2}:{minutes:0>2}:{seconds:0>2}");
        println!("{weekday} {month} {day} {year}");

        gfx.wait_for_vblank();
    }
}
