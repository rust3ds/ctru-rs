//! Mii Selector example.
//!
//! This example showcases the use of the MiiSelector applet to obtain Mii data from the user's input.

use ctru::applets::mii_selector::{Error, MiiSelector, Options};
use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    // Setup the Mii Selector configuration.
    let mut mii_selector = MiiSelector::new();
    // The Mii Selector window can be closed without selecting a Mii.
    mii_selector.set_options(Options::ENABLE_CANCEL);
    mii_selector.set_initial_index(3);
    // The first user-made Mii cannot be used.
    mii_selector.blocklist_user_mii(0.into());
    mii_selector.set_title("Great Mii Selector!");

    // Launch the Mii Selector and use its result to print the selected Mii's information.
    match mii_selector.launch() {
        Ok(result) => {
            println!("Mii type: {:?}", result.mii_type);
            println!("Name: {:?}", result.mii_data.name);
            println!("Author: {:?}", result.mii_data.author_name);
            println!(
                "Does the Mii have moles?: {:?}",
                result.mii_data.mole_details.is_enabled
            );
        }
        Err(Error::InvalidChecksum) => println!("Corrupt Mii selected"),
        Err(Error::NoMiiSelected) => println!("No Mii selected"),
    }

    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        gfx.wait_for_vblank();
    }
}
