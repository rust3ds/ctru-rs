#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::arch::asm;

pub mod result;
pub use result::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// In lieu of a proper errno function exposed by libc
/// (<https://github.com/rust-lang/libc/issues/1995>).
pub unsafe fn errno() -> s32 {
    *__errno()
}

pub unsafe fn getThreadLocalStorage() -> *mut libc::c_void {
    let return_value: *mut libc::c_void;
    asm!("mrc p15, 0, {}, c13, c0, 3", out(reg) return_value);
    return_value
}

pub unsafe fn getThreadCommandBuffer() -> *mut u32 {
    (getThreadLocalStorage() as *mut u8).add(0x80) as *mut u32
}

// TODO: not sure if there's a better way to do this, but I have gotten myself
// with this a couple times so having the hint seems nice to have.
#[cfg(test)]
compile_error!(concat!(
    "ctru-sys doesn't have tests and its lib test will fail to build at link time. ",
    "Try specifying `--package ctru-rs` to build those tests.",
));
