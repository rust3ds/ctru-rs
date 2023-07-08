//! Safe wrapper around `libctru`.
//!
//! # About
//!
//! This crate behaves as the main tool to access system-specific functionality on the Nintendo 3DS when developing homebrew software in Rust.
//! Thanks to it, developers can access the underlying system services and the console's hardware to develop userland applications
//! (such as HID devices, network capabilities, graphics, built-in cameras, etc.).
//!
//! Among these features, `ctru` also automatically includes functionality to properly integrate the Rust `std` with the console, which the developer would otherwise need to implement manually.
//!
//! # Usage
//!
//! Read thoroughly the official [`ctru` wiki](https://github.com/rust3ds/ctru-rs/wiki) which guides you through the setup needed to install the required toolchain and helpful tools.
//! After following the guide and understanding the many quirks of the Nintendo 3DS homebrew development environment, you can create a new project by including this crate as a dependency
//! of your project in your `Cargo.toml` manifest and build your binaries either manually (for the `armv6k-nintendo-3ds` target) or via [`cargo-3ds`](https://github.com/rust3ds/cargo-3ds).
//!
//! # Examples
//!
//! You can check out the examples provided with this crate which dive into most of the implemented functionality.

#![crate_type = "rlib"]
#![crate_name = "ctru"]
#![warn(missing_docs)]
#![feature(test)]
#![feature(custom_test_frameworks)]
#![feature(try_trait_v2)]
#![feature(once_cell_try)]
#![feature(allocator_api)]
#![test_runner(test_runner::run)]
#![doc(
    html_favicon_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]

// Nothing is imported from these crates but their inclusion here assures correct linking of the missing implementations.
extern crate pthread_3ds;
extern crate shim_3ds;

/// Expanded stack size used to spawn the main thread by `libctru`.
///
/// It takes effect only if the `big-stack` feature is active. Otherwise, the default stack size should be ~32kB.
///
/// This value was chosen to support crate dependencies which expected more stack than provided. It's suggested to use less stack if possible.
#[no_mangle]
#[cfg(feature = "big-stack")]
static __stacksize__: usize = 2 * 1024 * 1024; // 2MB

macro_rules! from_impl {
    ($from_type:ty, $into_type:ty) => {
        impl From<$from_type> for $into_type {
            fn from(v: $from_type) -> Self {
                v as $into_type
            }
        }
    };
}

/// Activate the custom `ctru` panic handler.
///
/// With this implementation, the main thread will stop and try to print debug info to an available [`Console`](console::Console).
/// In case it fails to find an active [`Console`](console::Console) the program will just exit.
///
/// # Notes
///
/// When ´test´ is enabled, this function will not do anything, as it should be overridden by the ´test´ environment.
pub fn use_panic_handler() {
    #[cfg(not(test))]
    panic_hook_setup();
}

/// Internal protocol to activate the custom panic handler hook.
///
/// # Notes
///
/// When `test` is enabled, this function will be ignored.
#[cfg(not(test))]
fn panic_hook_setup() {
    use crate::services::hid::{Hid, KeyPad};
    use std::panic::PanicInfo;

    let main_thread = std::thread::current().id();

    // Panic Hook setup
    let default_hook = std::panic::take_hook();
    let new_hook = Box::new(move |info: &PanicInfo| {
        default_hook(info);

        // Only for panics in the main thread
        if main_thread == std::thread::current().id() && console::Console::exists() {
            println!("\nPress SELECT to exit the software");

            match Hid::new() {
                Ok(mut hid) => loop {
                    hid.scan_input();
                    let keys = hid.keys_down();
                    if keys.contains(KeyPad::SELECT) {
                        break;
                    }
                },
                Err(e) => println!("Error while intializing Hid controller during panic: {e}"),
            }
        }
    });
    std::panic::set_hook(new_hook);
}

pub mod applets;
pub mod console;
pub mod error;
pub mod linear;
pub mod mii;
pub mod prelude;
pub mod services;

#[cfg(test)]
mod test_runner;

pub use crate::error::{Error, Result};
