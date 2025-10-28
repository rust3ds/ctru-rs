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
#[unsafe(no_mangle)]
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

/// Sets a custom [panic hook](https://doc.rust-lang.org/std/panic/fn.set_hook.html) that uses the error applet to display panic messages.
///
/// You can also choose to have the previously registered panic hook called along with the error applet popup, which can be useful
/// if you want to use output redirection to display panic messages over `3dslink` or `GDB`.
///
/// You can use [`std::panic::take_hook`](https://doc.rust-lang.org/std/panic/fn.take_hook.html) to unregister the panic hook
/// set by this function.
///
/// # Notes
///
/// * If the [`Gfx`] service is not initialized during a panic, the error applet will not be displayed and the old panic hook will be called.
pub fn set_panic_hook(call_old_hook: bool) {
    crate::applets::error::set_panic_hook(call_old_hook);
}

/// A helper type for writing string data into `[u16]` buffers.
///
/// This type is mainly useful for interop with `libctru` APIs that expect UTF-16 text as input. The writer implements the
/// [`std::fmt::Write`](https://doc.rust-lang.org/std/fmt/trait.Write.html) trait and ensures that the text is written in-bounds and properly nul-terminated.
///
/// # Notes
///
/// Subsequent writes to the same `Utf16Writer` will append to the buffer instead of overwriting the existing contents. If you want to start over from the
/// beginning of the buffer, simply create a new `Utf16Writer`.
///
/// If a write causes the buffer to reach the end of its capacity, `std::fmt::Error` will be returned, but all string data up until the end of the capacity will
/// still be written.
pub struct Utf16Writer<'a> {
    buf: &'a mut [u16],
    index: usize,
}

impl Utf16Writer<'_> {
    /// Creates a new [Utf16Writer] that writes its output into the provided buffer.
    pub fn new(buf: &mut [u16]) -> Utf16Writer<'_> {
        Utf16Writer { buf, index: 0 }
    }
}

impl std::fmt::Write for Utf16Writer<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let max = self.buf.len() - 1;

        for code_unit in s.encode_utf16() {
            if self.index == max {
                self.buf[self.index] = 0;
                return Err(std::fmt::Error);
            } else {
                self.buf[self.index] = code_unit;
                self.index += 1;
            }
        }

        self.buf[self.index] = 0;

        Ok(())
    }
}
