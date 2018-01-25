// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cell::UnsafeCell;
use super::mutex::Mutex;
use super::condvar::Condvar;

// A simple read-preferring RWLock implementation that I found on wikipedia <.<
pub struct RWLock {
    mutex: Mutex,
    cvar: Condvar,
    reader_count: UnsafeCell<u32>, 
    writer_active: UnsafeCell<bool>,
}

unsafe impl Send for RWLock {}
unsafe impl Sync for RWLock {}

impl RWLock {
    pub const fn new() -> RWLock {
        RWLock {
            mutex: Mutex::new(),
            cvar: Condvar::new(),
            reader_count: UnsafeCell::new(0),
            writer_active: UnsafeCell::new(false),
        }
    }

    #[inline]
    pub unsafe fn read(&self) {
        self.mutex.lock();

        while *self.writer_active.get() {
            self.cvar.wait(&self.mutex);
        }

        assert!(*self.reader_count.get() != u32::max_value());
        *self.reader_count.get() += 1;

        self.mutex.unlock();
    }

    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        if !self.mutex.try_lock() {
            return false
        }

        while *self.writer_active.get() {
            self.cvar.wait(&self.mutex);
        }

        assert!(*self.reader_count.get() != u32::max_value());
        *self.reader_count.get() += 1;

        self.mutex.unlock();
        true
    }

    #[inline]
    pub unsafe fn write(&self) {
        self.mutex.lock();

        while *self.writer_active.get() || *self.reader_count.get() > 0 {
            self.cvar.wait(&self.mutex);
        }

        *self.writer_active.get() = true;

        self.mutex.unlock();
    }

    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        if !self.mutex.try_lock() {
            return false;
        }

        while *self.writer_active.get() || *self.reader_count.get() > 0 {
            self.cvar.wait(&self.mutex);
        }

        *self.writer_active.get() = true;

        self.mutex.unlock();
        true
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        self.mutex.lock();

        *self.reader_count.get() -= 1;

        if *self.reader_count.get() == 0 {
            self.cvar.notify_one()
        }

        self.mutex.unlock();
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        self.mutex.lock();

        *self.writer_active.get() = false;

        self.cvar.notify_all();

        self.mutex.unlock();
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        self.mutex.destroy();
        self.cvar.destroy();
        *self.reader_count.get() = 0;
        *self.writer_active.get() = false;
    }
}
