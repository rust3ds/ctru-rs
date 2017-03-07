// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
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

use libctru::svc::{svcSleepThread, svcGetThreadPriority};
use libctru::thread::{threadCreate, threadJoin, threadFree};
use libctru::thread::Thread as ThreadHandle;

pub struct Thread {
    handle: ThreadHandle,
}

// Some platforms may have pthread_t as a pointer in which case we still want
// a thread to be Send/Sync
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub unsafe fn new<'a>(stack: usize, p: Box<FnBox() + 'a>) -> io::Result<Thread> {
        let p = box p;
        let stack_size = cmp::max(stack, 0x10000);

        // this retrieves the main thread's priority value. child threads need
        // to be spawned with a greater priority (smaller priority value) than
        // the main thread
        let mut priority = 0;
        svcGetThreadPriority(&mut priority, 0xFFFF8000);
        priority -= 1;

        let handle = threadCreate(Some(thread_func), &*p as *const _ as *mut _,
                                  stack_size, priority, -2, 0);

        return if handle == ptr::null_mut() {
            Err(io::Error::from_raw_os_error(libc::EAGAIN))
        } else {
            mem::forget(p); // ownership passed to the new thread
            Ok(Thread { handle: handle })
        };

        extern "C" fn thread_func(start: *mut libc::c_void) {
            unsafe { start_thread(start) }
        }
    }

    pub fn yield_now() {
        unimplemented!()
    }

    pub fn set_name(_name: &CStr) {
        // can't set thread names on the 3DS
    }

    pub fn sleep(dur: Duration) {
        unsafe {
            let nanos = dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64;
            svcSleepThread(nanos as i64)
        }
    }

    pub fn join(self) {
        unsafe {
            let ret = threadJoin(self.handle, u64::max_value());
            threadFree(self.handle);
            mem::forget(self);
            debug_assert_eq!(ret, 0);
        }
    }

    pub fn id(&self) -> usize {
        unimplemented!()
    }

    pub fn into_id(self) -> usize {
        unimplemented!()
    }
}

pub mod guard {
    pub unsafe fn current() -> Option<usize> { None }
    pub unsafe fn init() -> Option<usize> { None }
}
