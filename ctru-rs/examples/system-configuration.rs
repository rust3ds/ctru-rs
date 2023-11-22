//! System Configuration example.
//!
//! This example showcases the CFGU service to retrieve information about the console that the application is running on,
//! such as the model, region and used language.

use ctru::prelude::*;
use ctru::services::cfgu::Cfgu;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    // Initialize the CFGU service to retrieve all wanted information.
    let cfgu = Cfgu::new().expect("Couldn't obtain CFGU controller");

    println!("\x1b[0;0HRegion: {:?}", cfgu.region().unwrap());
    println!("\x1b[10;0HLanguage: {:?}", cfgu.language().unwrap());
    println!("\x1b[20;0HModel: {:?}", cfgu.model().unwrap());

    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        gfx.wait_for_vblank();
    }
}
