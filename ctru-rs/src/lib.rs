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

    let main_thread = thread::current().id();

    // Panic Hook setup
    let default_hook = std::panic::take_hook();
    let new_hook = Box::new(move |info: &PanicInfo| {
        default_hook(info);

        // Only for panics in the main thread
        if main_thread == thread::current().id() && console::Console::exists() {
            println!("\nPress SELECT to exit the software");
            let hid = services::hid::Hid::init().unwrap();

            loop {
                hid.scan_input();
                if hid.keys_down().contains(services::hid::KeyPad::KEY_SELECT) {
                    break;
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

pub use crate::error::{Error, Result};

pub use crate::gfx::Gfx;
pub use crate::srv::Srv;
