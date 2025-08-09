//! Wi-Fi info example
//!
//! This example prints out all the info about the console's network, like SSID, security, proxy info...

use ctru::{
    prelude::*,
    services::ac::{Ac, NetworkStatus},
};
use std::error::Error;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    ctru::set_panic_hook(true);

    let _console = Console::new(gfx.top_screen.borrow_mut());

    let ac = Ac::new().expect("Couldn't get an AC handle");

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

fn print_network_info(ac: &Ac) -> Result<(), Box<dyn Error>> {
    let status = ac.wifi_status()?;
    println!("Wi-Fi status: {:?}", status);

    // Some methods error out if the console isn't connected
    if matches!(
        status,
        NetworkStatus::WANConnected | NetworkStatus::LANConnected
    ) {
        println!("Wi-Fi SSID: {}", String::from_utf8(ac.wifi_ssid()?)?);
        println!("Wi-Fi security: {:?}", ac.wifi_security()?);
        let proxied = ac.proxy_enabled()?;
        println!("Proxy enabled: {}", proxied);
        if proxied {
            println!("Proxy port: {}", ac.proxy_port()?);
            println!(
                "Proxy username: {}",
                String::from_utf8(ac.proxy_username()?)?
            );
            println!(
                "Proxy password: {}",
                String::from_utf8(ac.proxy_password()?)?
            );
        }
    } else {
        println!("Not connected to any network.")
    }

    Ok(())
}
