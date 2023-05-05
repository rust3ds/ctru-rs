use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let console = Console::new(gfx.top_screen.borrow_mut());

    // We'll hold the previous touch position for comparison.
    let mut old_touch: (u16, u16) = (0, 0);

    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Get X and Y coordinates of the touch point.
        // The touch screen is 320x240.
        let touch: (u16, u16) = hid.touch_position();

        // We only want to print the position when it's different
        // from what it was on the previous frame
        if touch != old_touch {
            // Special case for when the user lifts the stylus/finger from the screen.
            // This is done to avoid some screen tearing.
            if touch == (0, 0) {
                console.clear();

                // Print again because we just cleared the screen
                println!("\x1b[29;16HPress Start to exit");
            }

            // Move the cursor back to the top of the screen and print the coordinates
            print!("\x1b[1;1HTouch Screen position: {:#?}", touch);
        }

        // Save our current touch position for the next frame
        old_touch = touch;

        gfx.wait_for_vblank();
    }
}
