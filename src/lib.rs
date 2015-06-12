#![feature(core)]
#![feature(no_std)]
#![no_std]
#![crate_type = "rlib"]
#![crate_name = "ctru"]

extern crate core;

pub mod raw;

pub type Result = i32;
pub type Handle = u32;

pub mod srv;
pub mod gfx;
pub mod services;
