// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unix-specific primitives available on all unix platforms

#![stable(feature = "raw_ext", since = "1.1.0")]
#![rustc_deprecated(since = "1.8.0",
                    reason = "these type aliases are no longer supported by \
                              the standard library, the `libc` crate on \
                              crates.io should be used instead for the correct \
                              definitions")]
#![allow(deprecated, warnings)]

#[allow(missing_docs)]
#[stable(feature = "raw_ext", since = "1.1.0")] pub type uid_t = u32;

#[allow(missing_docs)]
#[stable(feature = "raw_ext", since = "1.1.0")] pub type gid_t = u32;

#[allow(missing_docs)]
#[stable(feature = "raw_ext", since = "1.1.0")] pub type pid_t = i32;
