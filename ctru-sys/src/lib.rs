#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

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
