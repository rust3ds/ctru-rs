// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of various bits and pieces of the `panic!` macro and
//! associated runtime pieces.

use any::Any;
use fmt;
use __core::fmt::Display;

///The compiler wants this to be here. Otherwise it won't be happy. And we like happy compilers.
#[lang = "eh_personality"]
extern fn eh_personality() {}

/// Entry point of panic from the libcore crate.
#[lang = "panic_fmt"]
extern fn rust_begin_panic(msg: fmt::Arguments, file: &'static str, line: u32) -> ! {
    begin_panic_fmt(&msg, &(file, line))
}

/// The entry point for panicking with a formatted message.
///
/// This is designed to reduce the amount of code required at the call
/// site as much as possible (so that `panic!()` has as low an impact
/// on (e.g.) the inlining of other functions as possible), by moving
/// the actual formatting into this shared place.
#[unstable(feature = "libstd_sys_internals",
           reason = "used by the panic! macro",
           issue = "0")]
#[inline(never)] #[cold]
pub fn begin_panic_fmt(msg: &fmt::Arguments, file_line: &(&'static str, u32)) -> ! {
    use fmt::Write;

    let mut s = String::new();
    let _ = s.write_fmt(*msg);
    begin_panic(s, file_line);
}

/// This is where the main panic logic happens.
#[inline(never)]
#[cold]
pub fn begin_panic<M: Any + Send + Display>(msg: M, file_line: &(&'static str, u32)) -> ! {
    let msg = Box::new(msg);
    let (file, line) = *file_line;

    println!("--------------------------------------------------");
    println!("PANIC in {} at line {}:", file, line);
    println!("    {}", msg);
    println!("\x1b[29;00H--------------------------------------------------");

    loop {}
}
