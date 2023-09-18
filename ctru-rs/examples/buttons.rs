//! Buttons example.
//!
//! This example showcases how to retrieve button inputs from the console's HID.

use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let console = Console::new(gfx.top_screen.borrow_mut());

    println!("Hi there! Try pressing a button");
    println!("\x1b[29;16HPress Start to exit");

    // This struct will contain the keys that we held on the previous frame
    let mut old_keys = KeyPad::empty();

    while apt.main_loop() {
        // Scan for user input on the current frame.
        hid.scan_input();

        // Get information about which keys were held down on this frame.
        let keys = hid.keys_held();

        // Print the status of the 2 sliders.
        println!(
            "\x1b[20;0HVolume slider: {}              ",
            hid.slider_volume()
        );
        println!("\x1b[21;0H3D slider: {}              ", hid.slider_3d());

        // We only want to print when the keys we're holding now are different
        // from what they were on the previous frame.
        if keys != old_keys {
            // Clear the screen.
            console.clear();

            // We print these again because we just cleared the screen above.
            println!("Hi there! Try pressing a button");
            println!("\x1b[29;16HPress Start to exit");

            // Move the cursor back to the top of the screen.
            println!("\x1b[3;0H");

            // Print to the screen depending on which keys were held.
            //
            // The `.contains()` method checks for all of the provided keys,
            // and the `.intersects()` method checks for any of the provided keys.
            //
            // You can also use the `.bits()` method to do direct comparisons on
            // the underlying bits.

            if keys.contains(KeyPad::A) {
                println!("You held A!");
            }
            if keys.bits() & KeyPad::B.bits() != 0 {
                println!("You held B!");
            }
            if keys.contains(KeyPad::X | KeyPad::Y) {
                println!("You held X and Y!");
            }
            if keys.intersects(KeyPad::L | KeyPad::R | KeyPad::ZL | KeyPad::ZR) {
                println!("You held a shoulder button!");
            }
            if keys.contains(KeyPad::START) {
                println!("See ya!");
                break;
            }
        }

        // Save our current key presses for the next frame.
        old_keys = keys;

        gfx.wait_for_vblank();
    }
}
