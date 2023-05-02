use ctru::{prelude::*, services::hid::TouchPosition};

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");

    let console = Console::init(gfx.top_screen.borrow_mut());

    // This struct will hold the touch position
    let mut touch_position = TouchPosition::default();

    // This will hold the previous touch position.
    let mut old_touch: (u16, u16) = (0, 0);

    println!("Press Start to exit.");

    // Main loop
    while apt.main_loop() {
        // Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Get X and Y coordinates of the touch point.
        // The touch screen is 320x240.
        let touch: (u16, u16) = touch_position.get();

        // We only want to print the position when it's different
        // from what it was on the previous frame
        if (touch != old_touch) {
            // Clear the console so we can print the new touch position
            console.clear();

            // Print again because we just cleared the screen
            println!("Press Start to exit.");

            // Move the cursor back to the top of the screen and print the coordinates
            println!("Touch Screen position: ({}; {})", touch.0, touch.1);
        }

        // Save our current touch position for the next frame
        old_touch = touch;

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
