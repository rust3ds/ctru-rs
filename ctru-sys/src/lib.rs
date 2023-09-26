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

// TODO: not sure if there's a better way to do this, but I have gotten myself
// with this a couple times so having the hint seems nice to have.
#[cfg(test)]
compile_error!(concat!(
    "ctru-sys doesn't have tests and its lib test will fail to build at link time. ",
    "Try specifying `--package ctru-rs` to build those tests.",
));
