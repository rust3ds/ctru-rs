#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(test_runner::run_gdb))]
#![doc(
    html_favicon_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(html_root_url = "https://rust3ds.github.io/ctru-rs/crates")]

pub mod result;
pub use result::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// In lieu of a proper errno function exposed by libc
/// (<https://github.com/rust-lang/libc/issues/1995>).
pub unsafe fn errno() -> s32 {
    *__errno()
}

// Prevent linking errors from the standard `test` library when running `cargo 3ds test --lib`.
#[cfg(test)]
extern crate shim_3ds;
