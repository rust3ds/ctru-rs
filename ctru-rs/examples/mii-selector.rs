use ctru::applets::mii_selector::{LaunchError, MiiSelector, Options};
use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    let mut mii_selector = MiiSelector::new();
    mii_selector.set_options(Options::MII_SELECTOR_CANCEL);
    mii_selector.set_initial_index(3);
    mii_selector.blacklist_user_mii(0.into());
    mii_selector.set_title("Great Mii Selector!");

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
        Err(LaunchError::InvalidChecksum) => println!("Corrupt Mii selected"),
        Err(LaunchError::NoMiiSelected) => println!("No Mii selected"),
    }

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
