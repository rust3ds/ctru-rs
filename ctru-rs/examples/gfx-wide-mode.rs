//! Wide-Mode Graphics example.
//!
//! This example demonstrates the wide-mode capability of the top screen
//! which doubles the horizontal resolution of the screen by merging the 2 stereoscopic 3D sides.
//!
//! Beware, wide-mode doesn't work on Old 2DS consoles.

use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let mut console = Console::new(gfx.top_screen.borrow_mut());

    println!("Press A to enable/disable wide screen mode.");
    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Since we can't set wide-mode while running the console (since that would break the currently displayed text),
        // we need to rebuild the console each time we want to make the switch.
        if hid.keys_down().contains(KeyPad::A) {
            drop(console);

            // Switch the state of the wide-mode.
            let wide_mode = gfx.top_screen.borrow().is_wide();
            gfx.top_screen.borrow_mut().set_wide_mode(!wide_mode);

            console = Console::new(gfx.top_screen.borrow_mut());
            println!("Press A to enable/disable wide screen mode.");
            println!("\x1b[29;16HPress Start to exit");
        }

        gfx.wait_for_vblank();
    }
}
