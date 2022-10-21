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

    let result = mii_selector.launch();

    println!("\x1b[0;0HIs Mii selected?: {:?}", result.is_mii_selected);
    println!("\x1b[2;0HValid checksum?: {:?}", result.valid_checksum());
    println!("\x1b[4;0HMii type: {:?}", result.mii_type);
    println!("\x1b[6;0HMii checksum: {:?}", result.checksum);
    println!("\x1b[8;0HName: {:?}", result.name());
    println!("\x1b[12;0HAuthor: {:?}", result.author());

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
