#![crate_type = "rlib"]
#![crate_name = "ctru"]

#[macro_use]
extern crate bitflags;

extern crate ctru_sys as libctru;

pub mod console;
pub mod srv;
pub mod gfx;
pub mod services;
pub mod sdmc;

pub use srv::Srv;
pub use gfx::Gfx;
pub use sdmc::Sdmc;
