// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// *Implementation adapted from `/sys/redox/condvar.rs`

use cell::UnsafeCell;
use intrinsics::atomic_cxchg;
use ptr;
use time::Duration;

use sys::mutex::{self, Mutex};

use libctru::{__sync_get_arbiter, LightLock};
use libctru::{svcArbitrateAddress, ArbitrationType};

pub struct Condvar {
    lock: UnsafeCell<*mut LightLock>,
}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar {
            lock: UnsafeCell::new(ptr::null_mut()),
        }
    }

    #[inline]
    pub unsafe fn init(&self) {
        *self.lock.get() = ptr::null_mut();
    }

    #[inline]
    pub fn notify_one(&self) {
        unsafe {
            let arbiter = __sync_get_arbiter();

            svcArbitrateAddress(arbiter,
                                *self.lock.get() as u32,
                                ArbitrationType::ARBITRATION_SIGNAL,
                                1,
                                0);
        }
    }

    #[inline]
    pub fn notify_all(&self) {
        unsafe {
            let lock = self.lock.get();

            if *lock == ptr::null_mut() {
                return;
            }

            let arbiter = __sync_get_arbiter();

            svcArbitrateAddress(arbiter,
                                *self.lock.get() as u32,
                                ArbitrationType::ARBITRATION_SIGNAL,
                                -1,
                                0);
        }
    }

    #[inline]
    pub fn wait(&self, mutex: &Mutex) {
        unsafe {
            let lock = self.lock.get();

            if *lock != mutex::raw(mutex) {
                if *lock != ptr::null_mut() {
                    panic!("Condvar used with more than one Mutex");
                }

                atomic_cxchg(lock as *mut usize, 0, mutex::raw(mutex) as usize);
            }

            mutex.unlock();

            let arbiter = __sync_get_arbiter();

            svcArbitrateAddress(arbiter,
                                *self.lock.get() as u32,
                                ArbitrationType::ARBITRATION_WAIT_IF_LESS_THAN,
                                2,
                                0);

            mutex.lock();
        }
    }

    #[inline]
    pub fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        use time::Instant;

        unsafe {
            let lock = self.lock.get();

            if *lock != mutex::raw(mutex) {
                if *lock != ptr::null_mut() {
                    panic!("Condvar used with more than one Mutex");
                }

                atomic_cxchg(lock as *mut usize, 0, mutex::raw(mutex) as usize);
            }

            let now = Instant::now();

            let nanos = dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64;

            mutex.unlock();

            let arbiter = __sync_get_arbiter();

            svcArbitrateAddress(arbiter,
                                *self.lock.get() as u32,
                                ArbitrationType::ARBITRATION_WAIT_IF_LESS_THAN_TIMEOUT,
                                2,
                                nanos as i64);

            mutex.lock();

            now.elapsed() < dur
        }
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        *self.lock.get() = ptr::null_mut();
    }
}
