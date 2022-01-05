#![crate_type = "rlib"]
#![crate_name = "ctru"]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate widestring;

extern crate ctru_sys as libctru;

/// Call this somewhere to force Rust to link some required crates
/// (ex. pthread-3ds). The call doesn't need to execute, just exist.
///
/// See https://github.com/rust-lang/rust/issues/47384
pub fn init() {
    linker_fix_3ds::init();
    pthread_3ds::init();
}

pub mod applets;
pub mod console;
pub mod error;
pub mod gfx;
pub mod sdmc;
pub mod services;
pub mod srv;
pub mod thread;

pub use error::{Error, Result};

pub use gfx::Gfx;
pub use sdmc::Sdmc;
pub use srv::Srv;
