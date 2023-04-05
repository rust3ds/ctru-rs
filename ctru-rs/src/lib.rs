#![crate_type = "rlib"]
#![crate_name = "ctru"]
#![feature(test)]
#![feature(custom_test_frameworks)]
#![feature(try_trait_v2)]
#![feature(allocator_api)]
#![feature(nonnull_slice_from_raw_parts)]
#![test_runner(test_runner::run)]

// Nothing is imported from these crates but their inclusion here assures correct linking of the missing implementations.
extern crate shim_3ds;
extern crate pthread_3ds;

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

/// Activate the default panic handler.
///
/// With this implementation, the main thread will stop and try to print debug info to an available [console::Console].
/// In case it fails to find an active [console::Console] the program will just exit.
///
/// # Notes
///
/// When ´test´ is enabled, this function won't do anything, as it should be overridden by the ´test´ environment.
pub fn use_panic_handler() {
    #[cfg(not(test))]
    panic_hook_setup();
}

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

            match Hid::init() {
                Ok(hid) => loop {
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
