//! Hello World example.
//!
//! Simple "Hello World" application to showcase the basic setup needed for any user-oriented app to work.

use std::io::BufWriter;

use ctru::prelude::*;

fn main() {
    // Setup the custom panic handler in case any errors arise.
    // Thanks to it the user will get promptly notified of any panics.
    ctru::use_panic_handler();

    // Setup Graphics, Controller Inputs, Application runtime.
    // These is standard setup any app would need.
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    // Create a Console to print our "Hello, World!" to.
    let _console = Console::new(gfx.top_screen.borrow_mut());

    // Snazzy message created via `ferris_says`.
    let out = b"Hello fellow Rustaceans, I'm on the Nintendo 3DS!";
    let width = 24;

    let mut writer = BufWriter::new(Vec::new());
    ferris_says::say(out, width, &mut writer).unwrap();

    println!(
        "\x1b[0;0H{}",
        String::from_utf8_lossy(&writer.into_inner().unwrap())
    );

    dbg!(std::env::args());

    println!("\x1b[29;16HPress Start to exit");

    // Main application loop. This checks whether the app is normally running in the foreground.
    while apt.main_loop() {
        // Scan all the controller inputs.
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Use VSync to cap the framerate at the same speed as the LCD screen's refresh rate (60 fps).
        gfx.wait_for_vblank();
    }
}
