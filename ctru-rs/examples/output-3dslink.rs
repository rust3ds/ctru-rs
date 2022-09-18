//! Use the `3dslink --server` option for redirecting output from the 3DS back
//! to the device that sent the executable.
//!
//! For now, `cargo 3ds run` does not support this flag, so to run this example
//! it must be sent manually, like this:
//! ```sh
//! cargo 3ds build --example output-3dslink
//! 3dslink --server target/armv6k-nintendo-3ds/debug/examples/output-3dslink.3dsx
//! ```

use ctru::prelude::*;

fn main() {
    ctru::init();
    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");

    let mut soc = Soc::init().expect("Couldn't obtain SOC controller");

    soc.redirect_to_3dslink(true, true)
        .expect("unable to redirect stdout/err to 3dslink server");

    println!("Hello 3dslink!");
    eprintln!("Press Start on the device to disconnect and exit.");

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
