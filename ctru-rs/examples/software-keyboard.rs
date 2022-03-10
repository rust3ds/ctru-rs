use ctru::applets::swkbd::{Button, Swkbd};
use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

fn main() {
    ctru::init();
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();
    let _console = Console::init(gfx.top_screen.borrow_mut());

    println!("Press A to enter some text or press Start to quit");

    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();

        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_A) {
            // Prepares a software keyboard with two buttons: One to cancel input and one
            // to accept it. You can also use `Swkbd::init()` to launch the keyboard in different
            // configurations.
            let mut keyboard = Swkbd::default();

            // String used to store text received from the keyboard
            let mut text = String::new();

            // Raise the software keyboard. You can perform different actions depending on which
            // software button the user pressed
            match keyboard.get_utf8(&mut text) {
                Ok(Button::Right) => println!("You entered: {}", text),
                Ok(Button::Left) => println!("Cancelled"),
                Ok(Button::Middle) => println!("How did you even press this?"),
                Err(_) => println!("Oh noes, an error happened!"),
            }
        }

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
    }
}
