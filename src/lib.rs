#![feature(core)]
#![feature(no_std)]
#![no_std]

extern crate core;

pub mod raw;

pub type Result = i32;
pub type Handle = u32;

pub mod srv {
    use super::Result;
    use super::raw::srv;
    pub fn init() -> Result {
        unsafe {
            return srv::srvInit();
        }
    }
    pub fn exit() -> Result {
        unsafe {
            return srv::srvExit();
        }
    }

    pub fn awesome() -> i32 {
        0
    }
}
