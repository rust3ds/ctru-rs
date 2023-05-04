use ctru::applets::swkbd::{Button, Swkbd};
use ctru::prelude::*;

fn main() {
    ctru::use_panic_handler();

    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());

    println!("Press A to enter some text or press Start to quit");

    while apt.main_loop() {
        gfx.wait_for_vblank();

        hid.scan_input();

        if hid.keys_down().contains(KeyPad::A) {
            // Prepares a software keyboard with two buttons: One to cancel input and one
            // to accept it. You can also use `Swkbd::new()` to launch the keyboard in different
            // configurations.
            let mut keyboard = Swkbd::default();

            // Raise the software keyboard. You can perform different actions depending on which
            // software button the user pressed
            match keyboard.get_string(2048) {
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
