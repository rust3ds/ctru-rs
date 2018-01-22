// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use alloc::boxed::FnBox;
use libc;
use cmp;
use ffi::CStr;
use io;
use mem;
use ptr;
use sys_common::thread::start_thread;
use time::Duration;

use libctru::Thread as ThreadHandle;

pub struct Thread {
    handle: ThreadHandle,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;


impl Thread {
    pub unsafe fn new<'a>(stack: usize, p: Box<FnBox() + 'a>) -> io::Result<Thread> {
        let p = box p;
        let stack_size = cmp::max(stack, DEFAULT_MIN_STACK_SIZE);

        let mut priority = 0;
        ::libctru::svcGetThreadPriority(&mut priority, 0xFFFF8000);

        let handle = ::libctru::threadCreate(Some(thread_func), &*p as *const _ as *mut _,
                                             stack_size, priority, -2, false);

        return if handle == ptr::null_mut() {
            Err(io::Error::from_raw_os_error(libc::EAGAIN))
        } else {
            mem::forget(p); // ownership passed to the new thread
            Ok(Thread { handle: handle })
        };

        extern "C" fn thread_func(start: *mut libc::c_void) {
            unsafe { start_thread(start as *mut u8) }
        }
    }
    
    pub fn yield_now() {
        unsafe {
        ::libctru::svcSleepThread(0)
        }
    }

    pub fn set_name(_name: &CStr) {
        // threads aren't named in libctru
    }

    pub fn sleep(dur: Duration) {
        unsafe {
            let nanos = dur.as_secs()
                .saturating_mul(1_000_000_000)
                .saturating_add(dur.subsec_nanos() as u64);
            ::libctru::svcSleepThread(nanos as i64)
        }
    }

    pub fn join(self) {
        unsafe {
            let ret = ::libctru::threadJoin(self.handle, u64::max_value());
            ::libctru::threadFree(self.handle);
            mem::forget(self);
            debug_assert_eq!(ret, 0);
        }
    }

    #[allow(dead_code)]    
    pub fn id(&self) -> ThreadHandle {
        self.handle
    }

    #[allow(dead_code)]
    pub fn into_id(self) -> ThreadHandle {
        let handle = self.handle;
        mem::forget(self);
        handle
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe { ::libctru::threadDetach(self.handle) }
    }
}

pub mod guard {
    pub unsafe fn current() -> Option<usize> { None }
    pub unsafe fn init() -> Option<usize> { None }
}
