extern crate ctru;
use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

extern crate ferris_says;

use std::io::BufWriter;

fn main() {
    ctru::init();
    let gfx = Gfx::default();
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    let out = b"Hello fellow Rustaceans, I'm on the Nintendo 3DS!";
    let width = 24;

    let mut writer = BufWriter::new(Vec::new());
    ferris_says::say(out, width, &mut writer).unwrap();

    println!(
        "\x1b[0;0H{}",
        String::from_utf8_lossy(&writer.into_inner().unwrap())
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