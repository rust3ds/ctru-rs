/*
 * C bindings generation: 
 * bindgen --sysroot=$DEVKITARM/arm-none-eabi -I$CTRULIB/include $CTRULIB/include/3ds.h
 */

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(overflowing_literals)]

pub mod console;
pub mod env;
pub mod gfx;
pub mod gpu;
pub mod ipc;
pub mod lock;
pub mod os;
pub mod sdmc;
pub mod srv;
pub mod svc;
pub mod synchronization;
pub mod thread;
pub mod types;

pub mod services;

pub use self::types::*;

pub type Result = i32;
pub type Handle = u32;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

pub type ThreadFunc = Option<extern "C" fn(arg1: *mut c_void) -> ()>;
