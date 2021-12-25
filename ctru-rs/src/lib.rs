#![crate_type = "rlib"]
#![crate_name = "ctru"]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate widestring;

extern crate ctru_sys as libctru;

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
