#![no_std]
#![feature(lang_items)]
#![crate_type = "rlib"]
#![crate_name = "ctru"]

extern crate ctru_sys as libctru;

pub mod srv;
pub mod gfx;
pub mod sdmc;

pub mod services;

pub use srv::Srv;
pub use gfx::Gfx;
pub use sdmc::Sdmc;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
