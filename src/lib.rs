#![feature(lang_items)]
#![no_std]
#![crate_type = "rlib"]
#![crate_name = "ctru"]

extern crate ctru_sys as libctru;

pub mod console;
pub mod srv;
pub mod gfx;
pub mod sdmc;

pub mod services;

pub use srv::Srv;
pub use gfx::Gfx;
pub use sdmc::Sdmc;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    unsafe { libctru::libc::abort() }
}
