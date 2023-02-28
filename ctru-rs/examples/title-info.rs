use ctru::prelude::*;
use ctru::services::am::Am;
use ctru::services::fs::FsMediaType;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let am = Am::init().expect("Couldn't obtain AM controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    let title_count = am
        .get_title_count(FsMediaType::Sd)
        .expect("Failed to get title count");
    println!("This 3DS has {title_count} titles on its SD Card:");

    let title_list = am
        .get_title_list(FsMediaType::Sd)
        .expect("Failed to get title list");
    for id in title_list {
        println!("{id:x}");
    }

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
