#![crate_type = "rlib"]
#![crate_name = "ctru"]
#![feature(test)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run)]

extern "C" fn services_deinit() {
    unsafe {
        ctru_sys::psExit();
    }
}

/// Call this somewhere to force Rust to link some required crates
/// This is also a setup for some crate integration only available at runtime
///
/// See https://github.com/rust-lang/rust/issues/47384
pub fn init() {
    linker_fix_3ds::init();
    pthread_3ds::init();

    // Initialize the PS service for random data generation
    unsafe {
        ctru_sys::psInit();

        // Setup the deconstruction at the program's end
        libc::atexit(services_deinit);
    }

    use std::panic::PanicInfo;

    let main_thread = thread::current().id();

    // Panic Hook setup
    let default_hook = std::panic::take_hook();
    let new_hook = Box::new(move |info: &PanicInfo| {
        default_hook(info);

        // Only for panics in the main thread
        if main_thread == thread::current().id() && console::Console::exists() {
            println!("\nPress SELECT to exit the software");

            // The use of unsafe functions here is basically obligatory.
            // To have memory safety when using the `Hid` struct, we must not make more
            // than one available at the same time, so no drop/service ownership issues arise.
            // The problem here is that the `panic_hook` is run _before_ the app cleanup,
            // so an `Hid` stuct may still be alive and thus make the `panic_hook` panic.
            // If that were to happen, the system would have to reboot to properly close the app.
            //
            // Using `hidInit` is safe when another instance is open, and we can do safe operations afterwards.
            // The only (probably) unsafe part of this system is the `hidExit`, since in a multithreaded
            // environment some other threads may still be doing operations on the service
            // before the cleanup, though the time window would be almost nonexistent, and it would only
            // really be a problem in preemptive threads.
            //
            // TL;DR : This code is bad.
            unsafe {
                ctru_sys::hidInit();

                loop {
                    ctru_sys::hidScanInput();
                    let keys = services::hid::KeyPad::from_bits_truncate(ctru_sys::hidKeysDown());
                    if keys.contains(services::hid::KeyPad::KEY_SELECT) {
                        ctru_sys::hidExit();
                        break;
                    }
                }
            }
        }
    });
    std::panic::set_hook(new_hook);
}

pub mod applets;
pub mod console;
pub mod error;
pub mod gfx;
pub mod services;
pub mod srv;
pub mod thread;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "romfs", romfs_exists))] {
        pub mod romfs;
    } else {
        pub mod romfs {
            //! The RomFS folder has not been detected and/or the `romfs` feature has not been enabled.
            //!
            //! Configure the path in Cargo.toml (the default path is "romfs"). Paths are relative to the
            //! `CARGO_MANIFEST_DIR` environment variable, which is the directory containing the manifest of
            //! your package.
            //!
            //! ```toml
            //! [package.metadata.cargo-3ds]
            //! romfs_dir = "romfs"
            //! ```
        }
    }
}

#[cfg(test)]
mod test_runner;

pub use crate::error::{Error, Result};

pub use crate::gfx::Gfx;
pub use crate::srv::Srv;
