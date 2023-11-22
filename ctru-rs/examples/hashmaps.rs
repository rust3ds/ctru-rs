//! Hashmap example.
//!
//! This example showcases using Hashmaps on the 3DS console using the functionality implemented by the standard library.
//! While it may seem inappropriate for such a simple (and somewhat out-of-scope) example to be included here, it's important to note how
//! normally Hashmaps wouldn't work on the console, and are only capable to because of the internal implementations made by `ctru-rs`.
//!
//! As such, this example functions more closely to a test than a demonstration.

use ctru::prelude::*;

fn main() {
    // HashMaps generate hashes thanks to the 3DS' cryptografically secure generator.
    // This generator is only active when activating the `PS` service.
    // This service is automatically initialized.
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());

    let mut map = std::collections::HashMap::new();
    map.insert("A Key!", 102);
    map.insert("Another key?", 543);
    map.remove("A Key!");

    println!("{map:#?}");
    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}
