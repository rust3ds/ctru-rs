#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

pub mod result;
pub use result::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// In lieu of a proper errno function exposed by libc
/// (<https://github.com/rust-lang/libc/issues/1995>).
pub unsafe fn errno() -> s32 {
    *__errno()
}
