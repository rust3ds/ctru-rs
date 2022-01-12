#![crate_type = "rlib"]
#![crate_name = "ctru"]

/// Call this somewhere to force Rust to link some required crates
/// This is also a setup for some crate integration only available at runtime
///
/// See https://github.com/rust-lang/rust/issues/47384
pub fn init() {
    linker_fix_3ds::init();
    pthread_3ds::init();

    use std::panic::PanicInfo;

    // Panic Hook setup
    let default_hook = std::panic::take_hook();
    let new_hook = Box::new(move |info: &PanicInfo| {
        println!("\x1b[1;31m\n--------------------------------------------------");
        default_hook(info);
        println!("\nPress SELECT to exit the software");
        let hid = services::hid::Hid::init().unwrap();

        loop {
            hid.scan_input();
            if hid.keys_down().contains(services::hid::KeyPad::KEY_SELECT) {
                break;
            }
        }
    });
    std::panic::set_hook(new_hook);
}

pub mod applets;
pub mod console;
pub mod error;
pub mod gfx;
pub mod sdmc;
pub mod services;
pub mod srv;
pub mod thread;

pub use crate::error::{Error, Result};

pub use crate::gfx::Gfx;
pub use crate::sdmc::Sdmc;
pub use crate::srv::Srv;
