#![crate_type = "rlib"]
#![crate_name = "ctru"]

#![feature(fnbox)]
#![feature(thread_local_state)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate widestring;

extern crate ctru_sys as libctru;

pub mod applet;
pub mod console;
pub mod error;
pub mod srv;
pub mod gfx;
pub mod services;
pub mod sdmc;
pub mod thread;

pub use error::{Result, Error};

pub use srv::Srv;
pub use gfx::Gfx;
pub use sdmc::Sdmc;
