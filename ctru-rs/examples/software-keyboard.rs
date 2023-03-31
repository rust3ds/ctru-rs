use ctru::applets::swkbd::{Button, Swkbd};
use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

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

        if hid.keys_down().contains(KeyPad::A) {
            // Prepares a software keyboard with two buttons: One to cancel input and one
            // to accept it. You can also use `Swkbd::init()` to launch the keyboard in different
            // configurations.
            let mut keyboard = Swkbd::default();

            // Raise the software keyboard. You can perform different actions depending on which
            // software button the user pressed
            match keyboard.write_to_string() {
                Ok((text, Button::Right)) => println!("You entered: {text}"),
                Ok((_, Button::Left)) => println!("Cancelled"),
                Ok((_, Button::Middle)) => println!("How did you even press this?"),
                Err(_) => println!("Oh noes, an error happened!"),
            }
        }

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}
