// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Native threads.
//!
//! ## The threading model
//!
//! An executing Rust program consists of a collection of native OS threads,
//! each with their own stack and local state. Threads can be named, and
//! provide some built-in support for low-level synchronization.
//!
//! Communication between threads can be done through
//! [channels], Rust's message-passing types, along with [other forms of thread
//! synchronization](../../std/sync/index.html) and shared-memory data
//! structures. In particular, types that are guaranteed to be
//! threadsafe are easily shared between threads using the
//! atomically-reference-counted container, [`Arc`].
//!
//! Fatal logic errors in Rust cause *thread panic*, during which
//! a thread will unwind the stack, running destructors and freeing
//! owned resources. Thread panic is unrecoverable from within
//! the panicking thread (i.e. there is no 'try/catch' in Rust), but
//! the panic may optionally be detected from a different thread. If
//! the main thread panics, the application will exit with a non-zero
//! exit code.
//!
//! When the main thread of a Rust program terminates, the entire program shuts
//! down, even if other threads are still running. However, this module provides
//! convenient facilities for automatically waiting for the termination of a
//! child thread (i.e., join).
//!
//! ## Spawning a thread
//!
//! A new thread can be spawned using the [`thread::spawn`][`spawn`] function:
//!
//! ```rust
//! use std::thread;
//!
//! thread::spawn(move || {
//!     // some work here
//! });
//! ```
//!
//! In this example, the spawned thread is "detached" from the current
//! thread. This means that it can outlive its parent (the thread that spawned
//! it), unless this parent is the main thread.
//!
//! The parent thread can also wait on the completion of the child
//! thread; a call to [`spawn`] produces a [`JoinHandle`], which provides
//! a `join` method for waiting:
//!
//! ```rust
//! use std::thread;
//!
//! let child = thread::spawn(move || {
//!     // some work here
//! });
//! // some work here
//! let res = child.join();
//! ```
//!
//! The [`join`] method returns a [`Result`] containing [`Ok`] of the final
//! value produced by the child thread, or [`Err`] of the value given to
//! a call to [`panic!`] if the child panicked.
//!
//! ## Configuring threads
//!
//! A new thread can be configured before it is spawned via the [`Builder`] type,
//! which currently allows you to set the name and stack size for the child thread:
//!
//! ```rust
//! # #![allow(unused_must_use)]
//! use std::thread;
//!
//! thread::Builder::new().name("child1".to_string()).spawn(move || {
//!     println!("Hello, world!");
//! });
//! ```
//!
//! ## The `Thread` type
//!
//! Threads are represented via the [`Thread`] type, which you can get in one of
//! two ways:
//!
//! * By spawning a new thread, e.g. using the [`thread::spawn`][`spawn`]
//!   function, and calling [`thread()`] on the [`JoinHandle`].
//! * By requesting the current thread, using the [`thread::current()`] function.
//!
//! The [`thread::current()`] function is available even for threads not spawned
//! by the APIs of this module.
//!
//! ## Blocking support: park and unpark
//!
//! Every thread is equipped with some basic low-level blocking support, via the
//! [`thread::park()`][`park()`] function and [`thread::Thread::unpark()`][`unpark()`]
//! method. [`park()`] blocks the current thread, which can then be resumed from
//! another thread by calling the [`unpark()`] method on the blocked thread's handle.
//!
//! Conceptually, each [`Thread`] handle has an associated token, which is
//! initially not present:
//!
//! * The [`thread::park()`][`park()`] function blocks the current thread unless or until
//!   the token is available for its thread handle, at which point it atomically
//!   consumes the token. It may also return *spuriously*, without consuming the
//!   token. [`thread::park_timeout()`] does the same, but allows specifying a
//!   maximum time to block the thread for.
//!
//! * The [`unpark()`] method on a [`Thread`] atomically makes the token available
//!   if it wasn't already.
//!
//! In other words, each [`Thread`] acts a bit like a semaphore with initial count
//! 0, except that the semaphore is *saturating* (the count cannot go above 1),
//! and can return spuriously.
//!
//! The API is typically used by acquiring a handle to the current thread,
//! placing that handle in a shared data structure so that other threads can
//! find it, and then `park`ing. When some desired condition is met, another
//! thread calls [`unpark()`] on the handle.
//!
//! The motivation for this design is twofold:
//!
//! * It avoids the need to allocate mutexes and condvars when building new
//!   synchronization primitives; the threads already provide basic blocking/signaling.
//!
//! * It can be implemented very efficiently on many platforms.
//!
//! ## Thread-local storage
//!
//! This module also provides an implementation of thread-local storage for Rust
//! programs. Thread-local storage is a method of storing data into a global
//! variable that each thread in the program will have its own copy of.
//! Threads do not share this data, so accesses do not need to be synchronized.
//!
//! A thread-local key owns the value it contains and will destroy the value when the
//! thread exits. It is created with the [`thread_local!`] macro and can contain any
//! value that is `'static` (no borrowed pointers). It provides an accessor function,
//! [`with`], that yields a shared reference to the value to the specified
//! closure. Thread-local keys allow only shared access to values, as there would be no
//! way to guarantee uniqueness if mutable borrows were allowed. Most values
//! will want to make use of some form of **interior mutability** through the
//! [`Cell`] or [`RefCell`] types.
//!
//! [channels]: ../../std/sync/mpsc/index.html
//! [`Arc`]: ../../std/sync/struct.Arc.html
//! [`spawn`]: ../../std/thread/fn.spawn.html
//! [`JoinHandle`]: ../../std/thread/struct.JoinHandle.html
//! [`thread()`]: ../../std/thread/struct.JoinHandle.html#method.thread
//! [`join`]: ../../std/thread/struct.JoinHandle.html#method.join
//! [`Result`]: ../../std/result/enum.Result.html
//! [`Ok`]: ../../std/result/enum.Result.html#variant.Ok
//! [`Err`]: ../../std/result/enum.Result.html#variant.Err
//! [`panic!`]: ../../std/macro.panic.html
//! [`Builder`]: ../../std/thread/struct.Builder.html
//! [`thread::current()`]: ../../std/thread/fn.spawn.html
//! [`Thread`]: ../../std/thread/struct.Thread.html
//! [`park()`]: ../../std/thread/fn.park.html
//! [`unpark()`]: ../../std/thread/struct.Thread.html#method.unpark
//! [`thread::park_timeout()`]: ../../std/thread/fn.park_timeout.html
//! [`Cell`]: ../cell/struct.Cell.html
//! [`RefCell`]: ../cell/struct.RefCell.html
//! [`thread_local!`]: ../macro.thread_local.html
//! [`with`]: struct.LocalKey.html#method.with

#![stable(feature = "rust1", since = "1.0.0")]

////////////////////////////////////////////////////////////////////////////////
// Thread-local storage
////////////////////////////////////////////////////////////////////////////////

#[macro_use] mod local;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::local::{LocalKey, LocalKeyState};

// The types used by the thread_local! macro to access TLS keys. Note that there
// are two types, the "OS" type and the "fast" type. The OS thread local key
// type is accessed via platform-specific API calls and is slow, while the fast
// key type is accessed via code generated via LLVM, where TLS keys are set up
// by the elf linker. Note that the OS TLS type is always available: on macOS
// the standard library is compiled with support for older platform versions
// where fast TLS was not available; end-user code is compiled with fast TLS
// where available, but both are needed.

#[unstable(feature = "libstd_thread_internals", issue = "0")]
#[cfg(target_thread_local)]
#[doc(hidden)] pub use sys::fast_thread_local::Key as __FastLocalKeyInner;
#[unstable(feature = "libstd_thread_internals", issue = "0")]
#[doc(hidden)] pub use self::local::os::Key as __OsLocalKeyInner;
