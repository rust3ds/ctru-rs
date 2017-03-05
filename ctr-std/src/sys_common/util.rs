// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use fmt;
use io::prelude::*;
use sync::atomic::{self, Ordering};
use sys::stdio::Stderr;

pub fn min_stack() -> usize {
    static MIN: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
    match MIN.load(Ordering::SeqCst) {
        0 => {}
        n => return n - 1,
    }

	// NOTE: We don't have env variable support on the 3DS so let's just use the
	// default minimum

    // let amt = env::var("RUST_MIN_STACK").ok().and_then(|s| s.parse().ok());
    // let amt = amt.unwrap_or(2 * 1024 * 1024);

	let amt = 2 * 1024 * 1024;

    // 0 is our sentinel value, so ensure that we'll never see 0 after
    // initialization has run
    MIN.store(amt + 1, Ordering::SeqCst);
    amt
}

pub fn dumb_print(args: fmt::Arguments) {
    let _ = Stderr::new().map(|mut stderr| stderr.write_fmt(args));
}

// Other platforms should use the appropriate platform-specific mechanism for
// aborting the process.  If no platform-specific mechanism is available,
// ::intrinsics::abort() may be used instead.  The above implementations cover
// all targets currently supported by libstd.

pub fn abort(args: fmt::Arguments) -> ! {
    dumb_print(format_args!("fatal runtime error: {}\n", args));
    unsafe { ::sys::abort_internal(); }
}
