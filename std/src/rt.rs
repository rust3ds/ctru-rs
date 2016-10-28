// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Runtime services
//!
//! The `rt` module provides a narrow set of runtime services,
//! including the global heap (exported in `heap`) and unwinding and
//! backtrace support. The APIs in this module are highly unstable,
//! and should be considered as private implementation details for the
//! time being.

use mem;

// Reexport some of our utilities which are expected by other crates.
pub use panicking::{begin_panic, begin_panic_fmt};

//TODO: Handle argc/argv arguments
#[lang = "start"]
#[allow(unused_variables)]
fn lang_start(main: *const u8, argc: isize, argv: *const *const u8) -> isize {
    unsafe { mem::transmute::<_, fn()>(main)(); }
    0
}
