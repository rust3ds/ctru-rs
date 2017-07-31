extern crate ctru;

use ctru::gfx::Gfx;
use ctru::console::Console;
use ctru::services::apt::Apt;
use ctru::services::hid::{self, Hid, KeyPad};

fn main() {
    // Setup services
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let mut gfx = Gfx::default();
    let mut console = Console::default();

    println!("Hi there! Try pressing a button");
    println!("\x1b[29;16HPress Start to exit");

    // This struct will contain the keys that we held on the previous frame
    let mut old_keys = KeyPad::empty();

    while apt.main_loop() {
        // Scan for user input on the current frame.
        hid.scan_input();

        // Get information about which keys were held down on this frame
        let keys = hid.keys_held();

        // We only want to print when the keys we're holding now are different
        // from what they were on the previous frame
        if keys != old_keys {

            // Clear the screen
            console.clear();

            // We print these again because we just cleared the screen above
            println!("Hi there! Try pressing a button");
            println!("\x1b[29;16HPress Start to exit");

            // Move the cursor back to the top of the screen
            println!("\x1b[3;0H");

            // Print to the screen depending on which keys were held.
            //
            // The .contains() method checks for all of the provided keys,
            // and the .intersects() method checks for any of the provided keys.
            //
            // You can also use the .bits() method to do direct comparisons on
            // the underlying bits

            if keys.contains(hid::KEY_A) {
                println!("You held A!");
            }
            if keys.bits() & hid::KEY_B.bits() != 0 {
                println!("You held B!");
            }
            if keys.contains(hid::KEY_X | hid::KEY_Y) {
                println!("You held X and Y!");
            }
            if keys.intersects(hid::KEY_L | hid::KEY_R | hid::KEY_ZL | hid::KEY_ZR) {
                println!("You held a shoulder button!");
            }
            if keys.intersects(hid::KEY_START) {
                println!("See ya!");
                break
            }
        }

        // Save our current key presses for the next frame
        old_keys = keys;

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();
    }
}
