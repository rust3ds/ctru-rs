use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

use regex::Regex;

fn main() {
    ctru::init();
    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let _console = Console::init(gfx.bottom_screen.borrow_mut());

    let re = Regex::new(r"(?P<key>.+)=(?P<value>.+)").unwrap();

    for m in ["K1=V1", "no match", "X=Y"] {
        if let Some(m) = re.captures(m) {
            println!(
                "Key = {key:?}, value = {value:?}",
                key = m.name("key"),
                value = m.name("value")
            );
        }
    }

    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        // Wait for VBlank
        gfx.wait_for_vblank();
    }
}
