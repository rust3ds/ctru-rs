//! Wi-Fi info example
//!
//! This example prints out all the info about the console's network, like SSID, security, proxy info...

use ctru::{prelude::*, services::ac::Ac};

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let _console = Console::new(gfx.top_screen.borrow_mut());

    let mut ac = Ac::new().expect("Couldn't get an AC handle");

    print_network_info(&ac).expect("Error while gathering network info");
    println!("Press START to exit.");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        gfx.wait_for_vblank();
    }
}

fn print_network_info(ac: &Ac) -> ctru::Result<()> {
    let connected = ac.get_wifi_status()?;
    println!("Wi-Fi connected: {}", connected);

    // Some methods error out if the console isn't connected
    if connected {
        println!("Wi-Fi SSID: {}", ac.get_wifi_ssid()?);
        println!("Wi-Fi security: {:?}", ac.get_wifi_security()?);
    }

    Ok(())
}
