//! 3DS-specific threading APIs
//!
//! The standard library's thread API has been minimally extended for the 3DS to
//! expose the ability to set a thread's priority level and to pin a thread to a
//! specific CPU core. This module exists to provide more libctru-specific
//! functionality not normally found in the standard library.
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

use ctru_sys::svcGetProcessorID;

/// Get the current thread's priority level. Lower values correspond to higher
/// priority levels. The main thread's priority is typically 0x30, but not always.
pub fn priority() -> i32 {
    let thread_id = unsafe { libc::pthread_self() };
    let mut policy = 0;
    let mut sched_param = libc::sched_param { sched_priority: 0 };

    unsafe { libc::pthread_getschedparam(thread_id, &mut policy, &mut sched_param) };

    sched_param.sched_priority
}

/// Returns the ID of the processor the current thread is running on.
pub fn affinity() -> i32 {
    unsafe { svcGetProcessorID() }
}
