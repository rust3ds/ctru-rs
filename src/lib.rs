#![feature(alloc)]
#![feature(collections)]
#![feature(char_escape_debug)]
#![feature(int_error_internals)]
#![feature(lang_items)]
#![feature(question_mark)]
#![feature(reflect_marker)]
#![feature(slice_patterns)]
#![feature(str_internals)]
#![feature(try_from)]
#![feature(unicode)]

#![no_std]

#![crate_type = "rlib"]
#![crate_name = "ctru"]

extern crate alloc;
extern crate alloc_system;
extern crate collections;
extern crate rustc_unicode;

extern crate ctru_sys as libctru;

pub mod console;
pub mod srv;
pub mod gfx;
pub mod services;
pub mod sdmc;

pub mod ascii;
pub mod error;
pub mod ffi;
pub mod panic;
pub mod path;

mod sys;

pub use srv::Srv;
pub use gfx::Gfx;
pub use sdmc::Sdmc;
