//! Output redirection example.
//!
//! This example uses the `3dslink --server` option for redirecting output from the 3DS back
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
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    // We need to use network sockets to send the data stream back.
    let mut soc = Soc::new().expect("Couldn't obtain SOC controller");

    // Set the output to be redirected to the `3dslink` server.
    soc.redirect_to_3dslink(true, true)
        .expect("unable to redirect stdout/err to 3dslink server");

    println!("Hello 3dslink!");
    eprintln!("Press Start on the device to disconnect and exit.");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        gfx.wait_for_vblank();
    }
}
