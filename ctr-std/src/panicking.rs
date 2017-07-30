// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of various bits and pieces of the `panic!` macro and
//! associated runtime pieces.

use io::prelude::*;

use any::Any;
use cell::RefCell;
use fmt::{self, Display};
use mem;
use ptr;
use raw;
use sys_common::thread_info;

thread_local! {
    pub static LOCAL_STDERR: RefCell<Option<Box<Write + Send>>> = {
        RefCell::new(None)
    }
}

///The compiler wants this to be here. Otherwise it won't be happy. And we like happy compilers.
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

/// Determines whether the current thread is unwinding because of panic.
// Which it won't be, since we still don't have stack unwinding.
pub fn panicking() -> bool {
    false
}

/// Entry point of panic from the libcore crate.
#[lang = "panic_fmt"]
pub extern fn rust_begin_panic(msg: fmt::Arguments,
                               file: &'static str,
                               line: u32,
                               col: u32) -> ! {
    begin_panic_fmt(&msg, &(file, line, col))
}

/// The entry point for panicking with a formatted message.
///
/// This is designed to reduce the amount of code required at the call
/// site as much as possible (so that `panic!()` has as low an impact
/// on (e.g.) the inlining of other functions as possible), by moving
/// the actual formatting into this shared place.
#[unstable(feature = "libstd_sys_internals",
           reason = "used by the panic! macro",
           issue = "0")]
#[inline(never)] #[cold]
pub fn begin_panic_fmt(msg: &fmt::Arguments, file_line_col: &(&'static str, u32, u32)) -> ! {
    use fmt::Write;

    let mut s = String::new();
    let _ = s.write_fmt(*msg);
    begin_panic(s, file_line_col);
}

// Citra doesn't support the Error applet yet
#[cfg(feature = "citra")]
#[unstable(feature = "libstd_sys_internals",
           reason = "used by the panic! macro",
           issue = "0")]
#[inline(never)] #[cold]
pub fn begin_panic<M: Any + Send + Display>(msg: M, file_line_col: &(&'static str, u32, u32)) -> ! {
    let msg = Box::new(msg);
    let (file, line, col) = *file_line_col;

    // 3DS-specific code begins here
    use libctru::{consoleInit, consoleClear, gfxScreen_t, threadGetCurrent};

    unsafe {
        // Set up a new console, overwriting whatever was on the top screen
        // before we started panicking
        let _console = consoleInit(gfxScreen_t::GFX_TOP, ptr::null_mut());
        consoleClear();

        // Determine thread name
        let thread = thread_info::current_thread();
        let name = thread.as_ref()
            .and_then(|t| t.name())
            .unwrap_or(if threadGetCurrent() == ptr::null_mut() {"main"} else {"<unnamed>"});

        println!("thread '{}' panicked at '{}', {}:{}:{}",
                 name, msg, file, line, col);

        // Citra seems to ignore calls to svcExitProcess, and libc::abort()
        // causes it to lock up and fill the console with endless debug statements.
        // So instead of terminating the program, we just let it spin in the following
        // loop. This means that any background threads might continue to run after
        // this thread panics, but there's not a lot we can do about that currently.
        loop { }
    }
}

#[cfg(not(feature = "citra"))]
#[unstable(feature = "libstd_sys_internals",
           reason = "used by the panic! macro",
           issue = "0")]
#[inline(never)] #[cold]
pub fn begin_panic<M: Any + Send + Display>(msg: M, file_line_col: &(&'static str, u32, u32)) -> ! {
    let msg = Box::new(msg);
    let (file, line, col) = *file_line_col;

    // 3DS-specific code begins here
    use libctru::{errorInit, errorText, errorDisp,
                  APT_HardwareResetAsync, svcExitProcess, threadGetCurrent,
                  errorConf, errorType, CFG_Language};
    use libc;

    unsafe {
        // Determine thread name
        let thread = thread_info::current_thread();
        let name = thread.as_ref()
            .and_then(|t| t.name())
            .unwrap_or(if threadGetCurrent() == ptr::null_mut() {"main"} else {"<unnamed>"});

        // Setup error payload
        let error_text = format!("thread '{}' panicked at '{}', {}:{}:{}",
                                 name, msg, file, line, col);
        let mut error_conf: errorConf = mem::uninitialized();
        errorInit(&mut error_conf,
                  errorType::ERROR_TEXT_WORD_WRAP,
                  CFG_Language::CFG_LANGUAGE_EN);
        errorText(&mut error_conf, error_text.as_ptr() as *const libc::c_char);

        // Display error
        errorDisp(&mut error_conf);

        // Now that we're all done printing, it's time to exit the program.
        // We don't have stack unwinding yet, so let's just trigger a reboot
        APT_HardwareResetAsync();

        // If rebooting fails for some reason, we extra-forcibly end the program
        svcExitProcess()
    }
}

/// Invoke a closure, capturing the cause of an unwinding panic if one occurs.
pub unsafe fn try<R, F: FnOnce() -> R>(f: F) -> Result<R, Box<Any + Send>> {
    #[allow(unions_with_drop_fields)]
    union Data<F, R> {
        f: F,
        r: R,
    }

    // We do some sketchy operations with ownership here for the sake of
    // performance. We can only  pass pointers down to
    // `__rust_maybe_catch_panic` (can't pass objects by value), so we do all
    // the ownership tracking here manually using a union.
    //
    // We go through a transition where:
    //
    // * First, we set the data to be the closure that we're going to call.
    // * When we make the function call, the `do_call` function below, we take
    //   ownership of the function pointer. At this point the `Data` union is
    //   entirely uninitialized.
    // * If the closure successfully returns, we write the return value into the
    //   data's return slot. Note that `ptr::write` is used as it's overwriting
    //   uninitialized data.
    // * Finally, when we come back out of the `__rust_maybe_catch_panic` we're
    //   in one of two states:
    //
    //      1. The closure didn't panic, in which case the return value was
    //         filled in. We move it out of `data` and return it.
    //      2. The closure panicked, in which case the return value wasn't
    //         filled in. In this case the entire `data` union is invalid, so
    //         there is no need to drop anything.
    //
    // Once we stack all that together we should have the "most efficient'
    // method of calling a catch panic whilst juggling ownership.
    let mut any_data = 0;
    let mut any_vtable = 0;
    let mut data = Data {
        f: f,
    };

    let r = __rust_maybe_catch_panic(do_call::<F, R>,
                                     &mut data as *mut _ as *mut u8,
                                     &mut any_data,
                                     &mut any_vtable);

    return if r == 0 {
        debug_assert!(update_panic_count(0) == 0);
        Ok(data.r)
    } else {
        update_panic_count(-1);
        debug_assert!(update_panic_count(0) == 0);
        Err(mem::transmute(raw::TraitObject {
            data: any_data as *mut _,
            vtable: any_vtable as *mut _,
        }))
    };

    fn do_call<F: FnOnce() -> R, R>(data: *mut u8) {
        unsafe {
            let data = data as *mut Data<F, R>;
            let f = ptr::read(&mut (*data).f);
            ptr::write(&mut (*data).r, f());
        }
    }
}

#[cfg(not(test))]
#[doc(hidden)]
#[unstable(feature = "update_panic_count", issue = "0")]
pub fn update_panic_count(amt: isize) -> usize {
    use cell::Cell;
    thread_local! { static PANIC_COUNT: Cell<usize> = Cell::new(0) }

    PANIC_COUNT.with(|c| {
        let next = (c.get() as isize + amt) as usize;
        c.set(next);
        return next
    })
}

// *Implementation borrowed from the libpanic_abort crate*
//
// Rust's "try" function, but if we're aborting on panics we just call the
// function as there's nothing else we need to do here.
#[allow(improper_ctypes)]
extern fn __rust_maybe_catch_panic(f: fn(*mut u8),
                                data: *mut u8,
                                _data_ptr: *mut usize,
								_vtable_ptr: *mut usize) -> u32 {
    f(data);
    0
}
