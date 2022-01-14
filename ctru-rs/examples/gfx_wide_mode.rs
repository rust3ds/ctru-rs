extern crate ctru;

use ctru::console::Console;
use ctru::gfx::Screen;
use ctru::services::hid::KeyPad;
use ctru::services::{Apt, Hid};
use ctru::Gfx;

fn main() {
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let _console = Console::init(&gfx, ctru::gfx::Screen::Top);

    println!("Press A to enable/disable wide screen mode.");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        if hid.keys_down().contains(KeyPad::KEY_A) {
            gfx.set_wide_mode(!gfx.get_wide_mode());
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
