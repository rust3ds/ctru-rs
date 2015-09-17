#![feature(no_std)]
#![feature(lang_items)]
#![no_std]
#![crate_type = "rlib"]
#![crate_name = "ctru"]

pub mod raw;

pub type Result = i32;
pub type Handle = u32;

pub mod srv;
pub mod gfx;
pub mod services;

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
