//! Software Keyboard example.
//!
//! This example showcases the use of the Software Keyboard applet to receive text input from the user.

use ctru::applets::swkbd::{Button, CallbackResult, SoftwareKeyboard};
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

    // Custom filter callback to handle the given input.
    // Using this callback it's possible to integrate the applet
    // with custom error messages when the input is incorrect.
    let mut did_it_again = false;

    keyboard.set_filter_callback(Some(Box::new(move |input| {
        if input.contains("boo") && !did_it_again {
            did_it_again = true;
            CallbackResult::Retry("Aaaah, you scared me! Don't use that word again!".into())
        } else if input.contains("boo") && did_it_again {
            CallbackResult::Close("Hey, I told you to stop that!".into())
        } else {
            did_it_again = false;
            CallbackResult::Ok
        }
    })));

    println!("Press A to enter some text or press Start to exit.");

    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // Check if the user request to write some input.
        if hid.keys_down().contains(KeyPad::A) {
            // Launch the software keyboard. You can perform different actions depending on which
            // software button the user pressed.
            match keyboard.launch(&apt, &gfx) {
                Ok((text, Button::Right)) => println!("You entered: {text}"),
                Ok((_, Button::Left)) => println!("Cancelled"),
                Ok((_, Button::Middle)) => println!("How did you even press this?"),
                Err(_) => println!("Oh noes, an error happened!"),
            }
        }

        gfx.wait_for_vblank();
    }
}
