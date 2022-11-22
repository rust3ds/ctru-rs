use ctru::applets::mii_selector::MiiSelector;
use ctru::prelude::*;

fn main() {
    ctru::init();

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    let mut mii_selector = MiiSelector::init();
    mii_selector.set_initial_index(3);
    mii_selector.blacklist_user_mii(0.into());
    mii_selector.set_title("Great Mii Selector!");

    let result = mii_selector.launch().unwrap();

    println!("Is Mii selected?: {:?}", result.is_mii_selected);
    println!("Mii type: {:?}", result.mii_type);
    println!("Name: {:?}", result.mii_data.name);
    println!("Author: {:?}", result.mii_data.author_name);
    println!(
        "Does the Mii have moles?: {:?}",
        result.mii_data.mole_details.is_enabled
    );

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
