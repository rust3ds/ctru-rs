//! Safe and idiomatic Rust wrapper around [`libctru`](https://github.com/devkitPro/libctru).
//!
//! # About
//!
//! This crate behaves as the main tool to access system-specific functionality on the Nintendo 3DS when developing homebrew software in Rust.
//! Thanks to it, developers can develop userland applications by accessing access the underlying system services and the console's hardware
//! (such as [HID devices](crate::services::hid), [network capabilities](crate::services::soc), [graphics](crate::services::gfx), [built-in cameras](crate::services::cam), etc.).
//!
//! Among these features, [`ctru-rs`](crate) also automatically includes functionality to properly integrate the Rust `std` with the console's operating system,
//! which the developer would otherwise need to implement manually.
//!
//! # Usage
//!
//! Thoroughly read the official [`ctru-rs` wiki](https://github.com/rust3ds/ctru-rs/wiki) which guides you through the setup needed to install the required toolchain and helpful tools.
//! After following the guide and understanding the many quirks of the Nintendo 3DS homebrew development environment, you can create a new project by including this crate as a dependency
//! in your `Cargo.toml` manifest and build your binaries either manually (for the `armv6k-nintendo-3ds` target) or via [`cargo-3ds`](https://github.com/rust3ds/cargo-3ds).

#![crate_type = "rlib"]
#![crate_name = "ctru"]
#![warn(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(custom_test_frameworks)]
#![feature(try_trait_v2)]
#![feature(allocator_api)]
#![feature(new_uninit)]
#![test_runner(test_runner::run_gdb)] // TODO: does this make sense to have configurable?
#![doc(
    html_favicon_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(html_root_url = "https://rust3ds.github.io/ctru-rs/crates")]

// Nothing is imported from these crates but their inclusion here assures correct linking of the missing implementations.
extern crate pthread_3ds;
extern crate shim_3ds;

/// Expanded stack size used to spawn the main thread by `libctru`.
///
/// It takes effect only if the `big-stack` feature is active. Otherwise, the default stack size should be ~32kB.
///
/// This value was chosen to support crate dependencies which expected more stack than provided. It's suggested to use less stack if possible.
#[no_mangle]
// When building lib tests, we don't want to redefine the same symbol twice,
// since ctru-rs is both the crate under test and a dev-dependency (non-test).
// We might also be able to use #[linkage] for similar effect, but this way
// works without depending on another unstable feature.
#[cfg(all(feature = "big-stack", not(test)))]
static __stacksize__: usize = 2 * 1024 * 1024; // 2MB

macro_rules! from_impl {
    ($from_type:ty, $into_type:ty) => {
        impl From<$from_type> for $into_type {
            fn from(v: $from_type) -> Self {
                v as $into_type
            }
        }
    };
}

pub mod applets;
pub mod console;
pub mod error;
pub mod linear;
pub mod mii;
pub mod os;
pub mod prelude;
mod sealed;
pub mod services;

pub use crate::error::{Error, Result};
