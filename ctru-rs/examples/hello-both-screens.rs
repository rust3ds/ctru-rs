use ctru::console::Console;
use ctru::gfx::{Gfx, Screen};
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

fn main() {
    // Initialize services
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();

    // Start a console on the top screen
    let top_screen = Console::init(Screen::Top);

    // Start a console on the bottom screen.
    // The most recently initialized console will be active by default
    let bottom_screen = Console::init(Screen::Bottom);

    // Let's print on the top screen first
    top_screen.select();
    println!("This is the top screen! We have a lot of space up here!");

    // Now let's print something on the bottom screen
    bottom_screen.select();
    println!("\x1b[14;00HThis is the bottom screen.");
    println!("There's not as much space down here, but that's okay.");

    top_screen.select();
    println!("\x1b[29;16HPress Start to exit");

    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();

        hid.scan_input();
        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
    }
}
