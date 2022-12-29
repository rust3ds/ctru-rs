#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::arch::asm;

pub mod result;

mod bindings;

pub use bindings::*;
pub use result::*;

/// In lieu of a proper errno function exposed by libc
/// (<https://github.com/rust-lang/libc/issues/1995>), this will retrieve the
/// last error set in the global reentrancy struct.
pub unsafe fn errno() -> s32 {
    (*__getreent())._errno
}

pub unsafe fn getThreadLocalStorage() -> *mut libc::c_void {
    let return_value: *mut libc::c_void;
    asm!("mrc p15, 0, {}, c13, c0, 3", out(reg) return_value);
    return_value
}

pub unsafe fn getThreadCommandBuffer() -> *mut u32 {
    (getThreadLocalStorage() as *mut u8).add(0x80) as *mut u32
}
