// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use mem;
use slice;

pub fn hashmap_random_keys() -> (u64, u64) {
    let mut v = (0, 0);
    unsafe {
        let view = slice::from_raw_parts_mut(&mut v as *mut _ as *mut u8,
                                             mem::size_of_val(&v));
        imp::fill_bytes(view);
    }
    return v
}

mod imp {
    use libctru;

    pub fn fill_bytes(v: &mut [u8]) {
        unsafe {
            // Initializing and de-initializing the sslC subsystem every time
            // we initialize a hashmap is pretty dumb, but I can't think of a
            // better method at the moment.
            //
            // lazy_static won't work because
            // destructors (for closing the subsystem on exit) won't run.
            //
            // Perhaps overriding __appInit() and __appExit() will work,
            // but that's an experiment for another time.
            libctru::sslcInit(0);
            libctru::sslcGenerateRandomData(v.as_ptr() as _, v.len() as u32);
            libctru::sslcExit();
        }
    }
}
