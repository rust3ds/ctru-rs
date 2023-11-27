//! Software Keyboard example.
//!
//! This example showcases the use of the Software Keyboard applet to receive text input from the user.

use ctru::applets::swkbd::{Button, SoftwareKeyboard};
use ctru::prelude::*;

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let _console = Console::new(gfx.top_screen.borrow_mut());

    // Prepares a software keyboard with two buttons: one to cancel input and one
    // to accept it. You can also use `SoftwareKeyboard::new()` to launch the keyboard
    // with different configurations.
    let mut keyboard = SoftwareKeyboard::default();

    println!("Press A to enter some text or press Start to exit.");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Check if the user request to write some input.
        if hid.keys_down().contains(KeyPad::A) {
            // Raise the software keyboard. You can perform different actions depending on which
            // software button the user pressed.
            match keyboard.get_string(2048, &apt, &gfx) {
                Ok((text, Button::Right)) => println!("You entered: {text}"),
                Ok((_, Button::Left)) => println!("Cancelled"),
                Ok((_, Button::Middle)) => println!("How did you even press this?"),
                Err(_) => println!("Oh noes, an error happened!"),
            }
        }

        gfx.wait_for_vblank();
    }
}
