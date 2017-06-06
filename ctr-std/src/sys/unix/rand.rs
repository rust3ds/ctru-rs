// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use io::{self, Error, ErrorKind};
use mem;
use rand::Rng;

use libctru::{sslcInit, sslcExit, sslcGenerateRandomData};

pub struct OsRng(());

impl OsRng {
    pub fn new() -> io::Result<OsRng> {
        unsafe { 
            let r = sslcInit(0); 
            if r < 0 {
                Err(Error::new(ErrorKind::Other, "Unable to initialize the RNG"))
            } else {
                Ok(OsRng(()))
            }
        }
    }
}

impl Rng for OsRng {
    fn next_u32(&mut self) -> u32 {
        let mut v = [0; 4];
        self.fill_bytes(&mut v);
        unsafe { mem::transmute(v) }
    }

    fn next_u64(&mut self) -> u64 {
        let mut v = [0; 8];
        self.fill_bytes(&mut v);
        unsafe { mem::transmute(v) }
    }

    fn fill_bytes(&mut self, v: &mut [u8]) {
        unsafe { sslcGenerateRandomData(v.as_ptr() as _, v.len() as u32); }
    }
}

impl Drop for OsRng {
    fn drop(&mut self) {
        unsafe { sslcExit() }
    }
}
