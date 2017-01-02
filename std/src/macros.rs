// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The entry point for panic of Rust threads.
///
/// This macro is used to inject panic into a Rust thread, causing the thread to
/// panic entirely. Each thread's panic can be reaped as the `Box<Any>` type,
/// and the single-argument form of the `panic!` macro will be the value which
/// is transmitted.
///
/// The multi-argument form of this macro panics with a string and has the
/// `format!` syntax for building a string.
///
/// # Examples
///
/// ```should_panic
/// # #![allow(unreachable_code)]
/// panic!();
/// panic!("this is a terrible mistake!");
/// panic!(4); // panic with the value of 4 to be collected elsewhere
/// panic!("this is a {} {message}", "fancy", message = "message");
/// ```
#[macro_export]
macro_rules! panic {
    () => ({
        panic!("explicit panic")
    });
    ($msg:expr) => ({
        $crate::rt::begin_panic($msg, {
            // static requires less code at runtime, more constant data
            static _FILE_LINE: (&'static str, u32) = (file!(), line!());
            &_FILE_LINE
        })
    });
    ($fmt:expr, $($arg:tt)+) => ({
        $crate::rt::begin_panic_fmt(&format_args!($fmt, $($arg)+), {
            // The leading _'s are to avoid dead code warnings if this is
            // used inside a dead function. Just `#[allow(dead_code)]` is
            // insufficient, since the user may have
            // `#[forbid(dead_code)]` and which cannot be overridden.
            static _FILE_LINE: (&'static str, u32) = (file!(), line!());
            &_FILE_LINE
        })
    });
}

/// Helper macro for unwrapping `Result` values while returning early with an
/// error if the value of the expression is `Err`. Can only be used in
/// functions that return `Result` because of the early return of `Err` that
/// it provides.
///
/// # Examples
///
/// ```
/// use std::io;
/// use std::fs::File;
/// use std::io::prelude::*;
///
/// fn write_to_file_using_try() -> Result<(), io::Error> {
///     let mut file = try!(File::create("my_best_friends.txt"));
///     try!(file.write_all(b"This is a list of my best friends."));
///     println!("I wrote to the file");
///     Ok(())
/// }
/// // This is equivalent to:
/// fn write_to_file_using_match() -> Result<(), io::Error> {
///     let mut file = try!(File::create("my_best_friends.txt"));
///     match file.write_all(b"This is a list of my best friends.") {
///         Ok(v) => v,
///         Err(e) => return Err(e),
///     }
///     println!("I wrote to the file");
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! try {
    ($expr:expr) => (match $expr {
        $crate::result::Result::Ok(val) => val,
        $crate::result::Result::Err(err) => {
            return $crate::result::Result::Err($crate::convert::From::from(err))
        }
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        $crate::io::_print(format_args!($($arg)*));
    );
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
