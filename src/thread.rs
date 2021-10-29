// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! 3DS-specific threading API
//!
//! While it is possible to create threads on the 3DS using functions found in
//! `std::thread`, the standard API does not expose the ability to set a thread's
//! priority level and to pin a thread to a specific CPU core. This module exists
//! to address those and other shortcomings.
//!
//! All 3DS models have at least two CPU cores available to spawn threads on:
//! The application core (appcore) and the system core (syscore). The New 3DS
//! has an additional two cores, the first of which can also run user-created
//! threads.
//!
//! Threads spawned on the appcore are cooperative rather than preemptive. This
//! means that threads must explicitly yield control to other threads (whether
//! via synchronization primitives or explicit calls to `yield_now`) when they
//! are not actively performing work. Failure to do so may result in control
//! flow being stuck in an inactive thread while the other threads are powerless
//! to continue their work.  
//!
//! However, it is possible to spawn one fully preemptive thread on the syscore
//! by using `apt::set_app_cpu_time_limit` to reserve a slice of time for a
//! thread to run. Attempting to run more than one thread at a time on the syscore
//! will result in an error.

use std::any::Any;
use std::cell::UnsafeCell;
use std::fmt;
use std::io;
use std::panic;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::thread as std_thread;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////
// Builder
////////////////////////////////////////////////////////////////////////////////

/// Thread factory, which can be used in order to configure the properties of
/// a new thread.
#[derive(Debug)]
pub struct Builder {
    // The size of the stack for the spawned thread in bytes
    stack_size: Option<usize>,
    // The spawned thread's priority value
    priority: Option<i32>,
    // The spawned thread's CPU affinity value
    affinity: Option<i32>,
}

impl Builder {
    /// Generates the base configuration for spawning a thread, from which
    /// configuration methods can be chained.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctru::thread;
    ///
    /// let builder = thread::Builder::new()
    ///                               .stack_size(10);
    ///
    /// let handler = builder.spawn(|| {
    ///     // thread code
    /// }).unwrap();
    ///
    /// handler.join().unwrap();
    /// ```
    pub fn new() -> Builder {
        Builder {
            stack_size: None,
            priority: None,
            affinity: None,
        }
    }

    /// Sets the size of the stack (in bytes) for the new thread.
    ///
    /// The actual stack size may be greater than this value if
    /// the platform specifies minimal stack size.
    ///
    /// For more information about the stack size for threads, see
    /// [this module-level documentation][stack-size].
    ///
    /// # Examples
    ///
    /// ```
    /// use ctru::thread;
    ///
    /// let builder = thread::Builder::new().stack_size(32 * 1024);
    /// ```
    ///
    /// [stack-size]: ./index.html#stack-size
    pub fn stack_size(mut self, size: usize) -> Builder {
        self.stack_size = Some(size);
        self
    }

    /// Sets the priority level for the new thread
    ///
    /// Low values gives the thread higher priority. For userland apps, this has
    /// to be within the range of 0x18 to 0x3F inclusive. The main thread usually
    /// has a priority of 0x30, but not always.
    pub fn priority(mut self, priority: i32) -> Builder {
        self.priority = Some(priority);
        self
    }

    /// Sets the ID of the processor the thread should be ran on.
    ///
    /// Processor IDs are labeled starting from 0. On Old3DS it must be <2, and
    /// on New3DS it must be <4. Pass -1 to execute the thread on all CPUs and
    /// -2 to execute the thread on the default CPU (set in the application's Exheader).
    ///
    /// *Processor #0 is the application core. It is always possible to create a thread on this
    /// core.
    /// *Processor #1 is the system core. If APT_SetAppCpuTimeLimit is used, it is possible
    /// to create a single thread on this core.
    /// *Processor #2 is New3DS exclusive. Normal applications can create threads on
    /// this core if the exheader kernel flags bitmask has 0x2000 set.
    /// *Processor #3 is New3DS exclusive. Normal applications cannot create threads
    /// on this core.
    ///
    /// Processes in the BASE memory region can always create threads on
    /// processors #2 and #3.
    pub fn affinity(mut self, affinity: i32) -> Builder {
        self.affinity = Some(affinity);
        self
    }

    /// Spawns a new thread by taking ownership of the `Builder`, and returns an
    /// [`io::Result`] to its [`JoinHandle`].
    ///
    /// The spawned thread may outlive the caller (unless the caller thread
    /// is the main thread; the whole process is terminated when the main
    /// thread finishes). The join handle can be used to block on
    /// termination of the child thread, including recovering its panics.
    ///
    /// For a more complete documentation see [`thread::spawn`][`spawn`].
    ///
    /// # Errors
    ///
    /// Unlike the [`spawn`] free function, this method yields an
    /// [`io::Result`] to capture any failure to create the thread at
    /// the OS level.
    ///
    /// [`spawn`]: ../../std/thread/fn.spawn.html
    /// [`io::Result`]: ../../std/io/type.Result.html
    /// [`JoinHandle`]: ../../std/thread/struct.JoinHandle.html
    ///
    /// # Examples
    ///
    /// ```
    /// use ctru::thread;
    ///
    /// let builder = thread::Builder::new();
    ///
    /// let handler = builder.spawn(|| {
    ///     // thread code
    /// }).unwrap();
    ///
    /// handler.join().unwrap();
    /// ```
    pub fn spawn<F, T>(self, f: F) -> io::Result<JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let Builder {
            stack_size,
            priority,
            affinity,
        } = self;

        let stack_size = stack_size.unwrap_or(imp::DEFAULT_MIN_STACK_SIZE);

        // If no priority value is specified, spawn with the same
        // priority as the parent thread
        let priority = priority.unwrap_or_else(|| imp::Thread::priority());

        // If no affinity is specified, spawn on the default core (determined by
        // the application's Exheader)
        let affinity = affinity.unwrap_or(-2);

        let my_thread = Thread::new();
        let their_thread = my_thread.clone();

        let my_packet: Arc<UnsafeCell<Option<Result<T>>>> = Arc::new(UnsafeCell::new(None));
        let their_packet = my_packet.clone();

        let main = move || {
            unsafe {
                thread_info::set(their_thread);
                let try_result = panic::catch_unwind(panic::AssertUnwindSafe(f));
                *their_packet.get() = Some(try_result);
            }
        };

        Ok(JoinHandle(JoinInner {
            native: unsafe {
                Some(imp::Thread::new(
                    stack_size,
                    priority,
                    affinity,
                    Box::new(main),
                )?)
            },
            thread: my_thread,
            packet: Packet(my_packet),
        }))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Free functions
////////////////////////////////////////////////////////////////////////////////

/// Spawns a new thread, returning a [`JoinHandle`] for it.
///
/// The join handle will implicitly *detach* the child thread upon being
/// dropped. In this case, the child thread may outlive the parent (unless
/// the parent thread is the main thread; the whole process is terminated when
/// the main thread finishes). Additionally, the join handle provides a [`join`]
/// method that can be used to join the child thread. If the child thread
/// panics, [`join`] will return an [`Err`] containing the argument given to
/// [`panic`].
///
/// This will create a thread using default parameters of [`Builder`], if you
/// want to specify the stack size or the name of the thread, use this API
/// instead.
///
/// As you can see in the signature of `spawn` there are two constraints on
/// both the closure given to `spawn` and its return value, let's explain them:
///
/// - The `'static` constraint means that the closure and its return value
///   must have a lifetime of the whole program execution. The reason for this
///   is that threads can `detach` and outlive the lifetime they have been
///   created in.
///   Indeed if the thread, and by extension its return value, can outlive their
///   caller, we need to make sure that they will be valid afterwards, and since
///   we *can't* know when it will return we need to have them valid as long as
///   possible, that is until the end of the program, hence the `'static`
///   lifetime.
/// - The [`Send`] constraint is because the closure will need to be passed
///   *by value* from the thread where it is spawned to the new thread. Its
///   return value will need to be passed from the new thread to the thread
///   where it is `join`ed.
///   As a reminder, the [`Send`] marker trait expresses that it is safe to be
///   passed from thread to thread. [`Sync`] expresses that it is safe to have a
///   reference be passed from thread to thread.
///
/// # Panics
///
/// Panics if the OS fails to create a thread; use [`Builder::spawn`]
/// to recover from such errors.
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    Builder::new().spawn(f).unwrap()
}

/// Gets a handle to the thread that invokes it.
///
/// # Examples
///
/// Getting a handle to the current thread with `thread::current()`:
///
/// ```
/// use ctru::thread;
///
/// let handler = thread::Builder::new()
///     .name("named thread".into())
///     .spawn(|| {
///         let handle = thread::current();
///         assert_eq!(handle.name(), Some("named thread"));
///     })
///     .unwrap();
///
/// handler.join().unwrap();
/// ```
pub fn current() -> Thread {
    thread_info::current_thread().expect(
        "use of ctru::thread::current() is not \
         possible after the thread's local \
         data has been destroyed",
    )
}

/// Cooperatively gives up a timeslice to the OS scheduler.
pub fn yield_now() {
    imp::Thread::yield_now()
}

/// Determines whether the current thread is unwinding because of panic.
///
/// A common use of this feature is to poison shared resources when writing
/// unsafe code, by checking `panicking` when the `drop` is called.
///
/// This is usually not needed when writing safe code, as [`Mutex`es][Mutex]
/// already poison themselves when a thread panics while holding the lock.
///
/// This can also be used in multithreaded applications, in order to send a
/// message to other threads warning that a thread has panicked (e.g. for
/// monitoring purposes).
#[inline]
pub fn panicking() -> bool {
    std_thread::panicking()
}

/// Puts the current thread to sleep for the specified amount of time.
///
/// The thread may sleep longer than the duration specified due to scheduling
/// specifics or platform-dependent functionality.
pub fn sleep(dur: Duration) {
    imp::Thread::sleep(dur)
}

// constants for park/unpark
const EMPTY: usize = 0;
const PARKED: usize = 1;
const NOTIFIED: usize = 2;

/// Blocks unless or until the current thread's token is made available.
///
/// A call to `park` does not guarantee that the thread will remain parked
/// forever, and callers should be prepared for this possibility.
///
/// # park and unpark
///
/// Every thread is equipped with some basic low-level blocking support, via the
/// [`thread::park`][`park`] function and [`thread::Thread::unpark`][`unpark`]
/// method. [`park`] blocks the current thread, which can then be resumed from
/// another thread by calling the [`unpark`] method on the blocked thread's
/// handle.
///
/// Conceptually, each [`Thread`] handle has an associated token, which is
/// initially not present:
///
/// * The [`thread::park`][`park`] function blocks the current thread unless or
///   until the token is available for its thread handle, at which point it
///   atomically consumes the token. It may also return *spuriously*, without
///   consuming the token. [`thread::park_timeout`] does the same, but allows
///   specifying a maximum time to block the thread for.
///
/// * The [`unpark`] method on a [`Thread`] atomically makes the token available
///   if it wasn't already.
///
/// In other words, each [`Thread`] acts a bit like a spinlock that can be
/// locked and unlocked using `park` and `unpark`.
///
/// The API is typically used by acquiring a handle to the current thread,
/// placing that handle in a shared data structure so that other threads can
/// find it, and then `park`ing. When some desired condition is met, another
/// thread calls [`unpark`] on the handle.
///
/// The motivation for this design is twofold:
///
/// * It avoids the need to allocate mutexes and condvars when building new
///   synchronization primitives; the threads already provide basic
///   blocking/signaling.
///
/// * It can be implemented very efficiently on many platforms.
///
/// # Examples
///
/// ```
/// use ctru::thread;
/// use std::time::Duration;
///
/// let parked_thread = thread::Builder::new()
///     .spawn(|| {
///         println!("Parking thread");
///         thread::park();
///         println!("Thread unparked");
///     })
///     .unwrap();
///
/// // Let some time pass for the thread to be spawned.
/// thread::sleep(Duration::from_millis(10));
///
/// println!("Unpark the thread");
/// parked_thread.thread().unpark();
///
/// parked_thread.join().unwrap();
/// ```
///
/// [`Thread`]: ../../std/thread/struct.Thread.html
/// [`park`]: ../../std/thread/fn.park.html
/// [`unpark`]: ../../std/thread/struct.Thread.html#method.unpark
/// [`thread::park_timeout`]: ../../std/thread/fn.park_timeout.html
//
// The implementation currently uses the trivial strategy of a Mutex+Condvar
// with wakeup flag, which does not actually allow spurious wakeups. In the
// future, this will be implemented in a more efficient way, perhaps along the lines of
//   http://cr.openjdk.java.net/~stefank/6989984.1/raw_files/new/src/os/linux/vm/os_linux.cpp
// or futuxes, and in either case may allow spurious wakeups.
pub fn park() {
    let thread = current();

    // If we were previously notified then we consume this notification and
    // return quickly.
    if thread
        .inner
        .state
        .compare_exchange(NOTIFIED, EMPTY, SeqCst, SeqCst)
        .is_ok()
    {
        return;
    }

    // Otherwise we need to coordinate going to sleep
    let mut m = thread.inner.lock.lock().unwrap();
    match thread
        .inner
        .state
        .compare_exchange(EMPTY, PARKED, SeqCst, SeqCst)
    {
        Ok(_) => {}
        Err(NOTIFIED) => return, // notified after we locked
        Err(_) => panic!("inconsistent park state"),
    }
    loop {
        m = thread.inner.cvar.wait(m).unwrap();
        match thread
            .inner
            .state
            .compare_exchange(NOTIFIED, EMPTY, SeqCst, SeqCst)
        {
            Ok(_) => return, // got a notification
            Err(_) => {}     // spurious wakeup, go back to sleep
        }
    }
}

/// Blocks unless or until the current thread's token is made available or
/// the specified duration has been reached (may wake spuriously).
///
/// The semantics of this function are equivalent to [`park`][park] except
/// that the thread will be blocked for roughly no longer than `dur`. This
/// method should not be used for precise timing due to anomalies such as
/// preemption or platform differences that may not cause the maximum
/// amount of time waited to be precisely `dur` long.
///
/// See the [park documentation][park] for more details.
///
/// # Platform-specific behavior
///
/// Platforms which do not support nanosecond precision for sleeping will have
/// `dur` rounded up to the nearest granularity of time they can sleep for.
///
/// # Examples
///
/// Waiting for the complete expiration of the timeout:
///
/// ```rust,no_run
/// use ctru::thread::park_timeout;
/// use std::time::{Instant, Duration};
///
/// let timeout = Duration::from_secs(2);
/// let beginning_park = Instant::now();
///
/// let mut timeout_remaining = timeout;
/// loop {
///     park_timeout(timeout_remaining);
///     let elapsed = beginning_park.elapsed();
///     if elapsed >= timeout {
///         break;
///     }
///     println!("restarting park_timeout after {:?}", elapsed);
///     timeout_remaining = timeout - elapsed;
/// }
/// ```
///
/// [park]: fn.park.html
pub fn park_timeout(dur: Duration) {
    let thread = current();

    // Like `park` above we have a fast path for an already-notified thread, and
    // afterwards we start coordinating for a sleep.
    // return quickly.
    if thread
        .inner
        .state
        .compare_exchange(NOTIFIED, EMPTY, SeqCst, SeqCst)
        .is_ok()
    {
        return;
    }
    let m = thread.inner.lock.lock().unwrap();
    match thread
        .inner
        .state
        .compare_exchange(EMPTY, PARKED, SeqCst, SeqCst)
    {
        Ok(_) => {}
        Err(NOTIFIED) => return, // notified after we locked
        Err(_) => panic!("inconsistent park_timeout state"),
    }

    // Wait with a timeout, and if we spuriously wake up or otherwise wake up
    // from a notification we just want to unconditionally set the state back to
    // empty, either consuming a notification or un-flagging ourselves as
    // parked.
    let (_m, _result) = thread.inner.cvar.wait_timeout(m, dur).unwrap();
    match thread.inner.state.swap(EMPTY, SeqCst) {
        NOTIFIED => {} // got a notification, hurray!
        PARKED => {}   // no notification, alas
        n => panic!("inconsistent park_timeout state: {}", n),
    }
}

////////////////////////////////////////////////////////////////////////////////
// ThreadId
////////////////////////////////////////////////////////////////////////////////

/// A unique identifier for a running thread.
///
/// # Examples
///
/// ```
/// use ctru::thread;
///
/// let other_thread = thread::spawn(|| {
///     thread::current().id()
/// });
///
/// let other_thread_id = other_thread.join().unwrap();
/// assert!(thread::current().id() != other_thread_id);
/// ```
///
/// [`id`]: ../../std/thread/struct.Thread.html#method.id
/// [`Thread`]: ../../std/thread/struct.Thread.html
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct ThreadId(u32);

////////////////////////////////////////////////////////////////////////////////
// Thread
////////////////////////////////////////////////////////////////////////////////

/// The internal representation of a `Thread` handle
struct Inner {
    // state for thread park/unpark
    state: AtomicUsize,
    lock: Mutex<()>,
    cvar: Condvar,
}

#[derive(Clone)]
/// A handle to a thread.
///
/// Threads are represented via the `Thread` type, which you can get in one of
/// two ways:
///
/// * By spawning a new thread, e.g. using the [`thread::spawn`][`spawn`]
///   function, and calling [`thread`][`JoinHandle::thread`] on the
///   [`JoinHandle`].
/// * By requesting the current thread, using the [`thread::current`] function.
///
/// The [`thread::current`] function is available even for threads not spawned
/// by the APIs of this module.
///
/// There is usually no need to create a `Thread` struct yourself, one
/// should instead use a function like `spawn` to create new threads, see the
/// docs of [`Builder`] and [`spawn`] for more details.
///
/// [`Builder`]: ../../std/thread/struct.Builder.html
/// [`JoinHandle::thread`]: ../../std/thread/struct.JoinHandle.html#method.thread
/// [`JoinHandle`]: ../../std/thread/struct.JoinHandle.html
/// [`thread::current`]: ../../std/thread/fn.current.html
/// [`spawn`]: ../../std/thread/fn.spawn.html

pub struct Thread {
    inner: Arc<Inner>,
}

impl Thread {
    // Used only internally to construct a thread object without spawning
    // Panics if the name contains nuls.
    pub(crate) fn new() -> Thread {
        Thread {
            inner: Arc::new(Inner {
                state: AtomicUsize::new(EMPTY),
                lock: Mutex::new(()),
                cvar: Condvar::new(),
            }),
        }
    }

    /// Atomically makes the handle's token available if it is not already.
    ///
    /// Every thread is equipped with some basic low-level blocking support, via
    /// the [`park`][park] function and the `unpark()` method. These can be
    /// used as a more CPU-efficient implementation of a spinlock.
    ///
    /// See the [park documentation][park] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctru::thread;
    /// use std::time::Duration;
    ///
    /// let parked_thread = thread::Builder::new()
    ///     .spawn(|| {
    ///         println!("Parking thread");
    ///         thread::park();
    ///         println!("Thread unparked");
    ///     })
    ///     .unwrap();
    ///
    /// // Let some time pass for the thread to be spawned.
    /// thread::sleep(Duration::from_millis(10));
    ///
    /// println!("Unpark the thread");
    /// parked_thread.thread().unpark();
    ///
    /// parked_thread.join().unwrap();
    /// ```
    ///
    /// [park]: fn.park.html
    pub fn unpark(&self) {
        loop {
            match self.inner
                .state
                .compare_exchange(EMPTY, NOTIFIED, SeqCst, SeqCst)
            {
                Ok(_) => return,         // no one was waiting
                Err(NOTIFIED) => return, // already unparked
                Err(PARKED) => {}        // gotta go wake someone up
                _ => panic!("inconsistent state in unpark"),
            }

            // Coordinate wakeup through the mutex and a condvar notification
            let _lock = self.inner.lock.lock().unwrap();
            match self.inner
                .state
                .compare_exchange(PARKED, NOTIFIED, SeqCst, SeqCst)
            {
                Ok(_) => return self.inner.cvar.notify_one(),
                Err(NOTIFIED) => return, // a different thread unparked
                Err(EMPTY) => {}         // parked thread went away, try again
                _ => panic!("inconsistent state in unpark"),
            }
        }
    }

    /// Gets the thread's unique identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctru::thread;
    ///
    /// let other_thread = thread::spawn(|| {
    ///     thread::current().id()
    /// });
    ///
    /// let other_thread_id = other_thread.join().unwrap();
    /// assert!(thread::current().id() != other_thread_id);
    /// ```
    pub fn id(&self) -> ThreadId {
        ThreadId(imp::Thread::id())
    }

    /// Get the current thread's priority level. Lower values correspond to higher
    /// priority levels. The main thread's priority is typically 0x30, but not always.
    pub fn priority(&self) -> i32 {
        imp::Thread::priority()
    }

    /// Returns the ID of the processor the current thread is running on.
    pub fn affinity(&self) -> i32 {
        imp::Thread::affinity()
    }
}

impl fmt::Debug for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.id(), f)
    }
}

////////////////////////////////////////////////////////////////////////////////
// JoinHandle
////////////////////////////////////////////////////////////////////////////////

/// A specialized [`Result`] type for threads.
///
/// Indicates the manner in which a thread exited.
///
/// A thread that completes without panicking is considered to exit successfully.
///
/// # Examples
///
/// ```no_run
/// use ctru::thread;
/// use std::fs;
///
/// fn copy_in_thread() -> thread::Result<()> {
///     thread::spawn(move || { fs::copy("foo.txt", "bar.txt").unwrap(); }).join()
/// }
///
/// fn main() {
///     match copy_in_thread() {
///         Ok(_) => println!("this is fine"),
///         Err(_) => println!("thread panicked"),
///     }
/// }
/// ```
///
/// [`Result`]: ../../std/result/enum.Result.html
pub type Result<T> = ::std::result::Result<T, Box<dyn Any + Send + 'static>>;

// This packet is used to communicate the return value between the child thread
// and the parent thread. Memory is shared through the `Arc` within and there's
// no need for a mutex here because synchronization happens with `join()` (the
// parent thread never reads this packet until the child has exited).
//
// This packet itself is then stored into a `JoinInner` which in turns is placed
// in `JoinHandle` and `JoinGuard`. Due to the usage of `UnsafeCell` we need to
// manually worry about impls like Send and Sync. The type `T` should
// already always be Send (otherwise the thread could not have been created) and
// this type is inherently Sync because no methods take &self. Regardless,
// however, we add inheriting impls for Send/Sync to this type to ensure it's
// Send/Sync and that future modifications will still appropriately classify it.
struct Packet<T>(Arc<UnsafeCell<Option<Result<T>>>>);

unsafe impl<T: Send> Send for Packet<T> {}
unsafe impl<T: Sync> Sync for Packet<T> {}

/// Inner representation for JoinHandle
struct JoinInner<T> {
    native: Option<imp::Thread>,
    thread: Thread,
    packet: Packet<T>,
}

impl<T> JoinInner<T> {
    fn join(&mut self) -> Result<T> {
        self.native.take().unwrap().join();
        unsafe { (*self.packet.0.get()).take().unwrap() }
    }
}

/// An owned permission to join on a thread (block on its termination).
///
/// A `JoinHandle` *detaches* the associated thread when it is dropped, which
/// means that there is no longer any handle to thread and no way to `join`
/// on it.
///
/// Due to platform restrictions, it is not possible to [`Clone`] this
/// handle: the ability to join a thread is a uniquely-owned permission.
///
/// This `struct` is created by the [`thread::spawn`] function and the
/// [`thread::Builder::spawn`] method.
///
/// # Examples
///
/// Creation from [`thread::spawn`]:
///
/// ```
/// use ctru::thread;
///
/// let join_handle: thread::JoinHandle<_> = thread::spawn(|| {
///     // some work here
/// });
/// ```
///
/// Creation from [`thread::Builder::spawn`]:
///
/// ```
/// use ctru::thread;
///
/// let builder = thread::Builder::new();
///
/// let join_handle: thread::JoinHandle<_> = builder.spawn(|| {
///     // some work here
/// }).unwrap();
/// ```
///
/// Child being detached and outliving its parent:
///
/// ```no_run
/// use ctru::thread;
/// use std::time::Duration;
///
/// let original_thread = thread::spawn(|| {
///     let _detached_thread = thread::spawn(|| {
///         // Here we sleep to make sure that the first thread returns before.
///         thread::sleep(Duration::from_millis(10));
///         // This will be called, even though the JoinHandle is dropped.
///         println!("♫ Still alive ♫");
///     });
/// });
///
/// original_thread.join().expect("The thread being joined has panicked");
/// println!("Original thread is joined.");
///
/// // We make sure that the new thread has time to run, before the main
/// // thread returns.
///
/// thread::sleep(Duration::from_millis(1000));
/// ```
///
/// [`Clone`]: ../../std/clone/trait.Clone.html
/// [`thread::spawn`]: fn.spawn.html
/// [`thread::Builder::spawn`]: struct.Builder.html#method.spawn
pub struct JoinHandle<T>(JoinInner<T>);

impl<T> JoinHandle<T> {
    /// Extracts a handle to the underlying thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctru::thread;
    ///
    /// let builder = thread::Builder::new();
    ///
    /// let join_handle: thread::JoinHandle<_> = builder.spawn(|| {
    ///     // some work here
    /// }).unwrap();
    ///
    /// let thread = join_handle.thread();
    /// println!("thread id: {:?}", thread.id());
    /// ```
    pub fn thread(&self) -> &Thread {
        &self.0.thread
    }

    /// Waits for the associated thread to finish.
    ///
    /// If the child thread panics, [`Err`] is returned with the parameter given
    /// to [`panic`].
    ///
    /// [`Err`]: ../../std/result/enum.Result.html#variant.Err
    /// [`panic`]: ../../std/macro.panic.html
    ///
    /// # Panics
    ///
    /// This function may panic on some platforms if a thread attempts to join
    /// itself or otherwise may create a deadlock with joining threads.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctru::thread;
    ///
    /// let builder = thread::Builder::new();
    ///
    /// let join_handle: thread::JoinHandle<_> = builder.spawn(|| {
    ///     // some work here
    /// }).unwrap();
    /// join_handle.join().expect("Couldn't join on the associated thread");
    /// ```
    pub fn join(mut self) -> Result<T> {
        self.0.join()
    }
}

impl<T> fmt::Debug for JoinHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("JoinHandle { .. }")
    }
}

fn _assert_sync_and_send() {
    fn _assert_both<T: Send + Sync>() {}
    _assert_both::<JoinHandle<()>>();
    _assert_both::<Thread>();
}

mod imp {
    use std::boxed::Box;
    use std::cmp;
    use std::io;
    use std::mem;
    use std::ptr;
    use std::time::Duration;
    use std::convert::TryInto;

    use libc;

    use crate::raw::{Thread as ThreadHandle, svcSleepThread, svcGetThreadId, svcGetThreadPriority,
                  svcGetProcessorID, threadCreate, threadDetach, threadFree, threadJoin};

    pub struct Thread {
        handle: ThreadHandle,
    }

    unsafe impl Send for Thread {}
    unsafe impl Sync for Thread {}

    pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

    impl Thread {
        pub unsafe fn new<'a>(
            stack: usize,
            priority: i32,
            affinity: i32,
            p: Box<dyn FnOnce() + 'a>,
        ) -> io::Result<Thread> {
            let p = Box::new(p);
            let stack_size = cmp::max(stack, DEFAULT_MIN_STACK_SIZE);

            let handle = threadCreate(
                Some(thread_func),
                &*p as *const _ as *mut _,
                stack_size.try_into().unwrap(),
                priority,
                affinity,
                false,
            );

            return if handle == ptr::null_mut() {
                Err(io::Error::from_raw_os_error(libc::EAGAIN))
            } else {
                mem::forget(p); // ownership passed to the new thread
                Ok(Thread { handle: handle })
            };

            extern "C" fn thread_func(start: *mut libc::c_void) {
                unsafe { Thread::_start_thread(start as *mut u8) }
            }
        }

        pub fn id() -> u32 {
            unsafe {
                let mut id = 0;
                svcGetThreadId(&mut id, 0xFFFF8000);
                id
            }
        }

        pub fn priority() -> i32 {
            unsafe {
                let mut priority = 0;
                svcGetThreadPriority(&mut priority, 0xFFFF8000);
                priority
            }
        }

        pub fn affinity() -> i32 {
            unsafe {
                svcGetProcessorID()
            }
        }

        unsafe fn _start_thread(main: *mut u8) {
            Box::from_raw(main as *mut Box<dyn FnOnce()>)()
        }

        pub fn yield_now() {
            unsafe { svcSleepThread(0) }
        }

        pub fn sleep(dur: Duration) {
            unsafe {
                let nanos = dur.as_secs()
                    .saturating_mul(1_000_000_000)
                    .saturating_add(dur.subsec_nanos() as u64);
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


        #[allow(dead_code)]
        pub fn handle(&self) -> ThreadHandle {
            self.handle
        }

        #[allow(dead_code)]
        pub fn into_handle(self) -> ThreadHandle {
            let handle = self.handle;
            mem::forget(self);
            handle
        }
    }

    impl Drop for Thread {
        fn drop(&mut self) {
            unsafe { threadDetach(self.handle) }
        }
    }
}

mod thread_info {
    use std::cell::RefCell;
    use crate::thread::Thread;

    struct ThreadInfo {
        thread: Thread,
    }

    thread_local! { static CTRU_THREAD_INFO: RefCell<Option<ThreadInfo>> = RefCell::new(None) }

    impl ThreadInfo {
        fn with<R, F>(f: F) -> Option<R>
        where
            F: FnOnce(&mut ThreadInfo) -> R,
        {
            CTRU_THREAD_INFO
                .try_with(move |c| {
                    if c.borrow().is_none() {
                        *c.borrow_mut() = Some(ThreadInfo {
                            thread: Thread::new(),
                        })
                    }
                    f(c.borrow_mut().as_mut().unwrap())
                })
                .ok()
        }
    }

    pub fn current_thread() -> Option<Thread> {
        ThreadInfo::with(|info| info.thread.clone())
    }

    pub fn set(thread: Thread) {
        CTRU_THREAD_INFO.with(|c| assert!(c.borrow().is_none()));
        CTRU_THREAD_INFO.with(move |c| *c.borrow_mut() = Some(ThreadInfo { thread }));
    }
}
