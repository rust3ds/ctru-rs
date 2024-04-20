use std::process::Termination;

use ctru::error::ResultCode;

use super::TestRunner;

// We use a little trick with cfg(doctest) to make code fences appear in
// rustdoc output, but compile without them when doctesting. This raises warnings
// for invalid code, though, so silence that lint here.
#[cfg_attr(not(doctest), allow(rustdoc::invalid_rust_codeblocks))]
/// Show test output in GDB, using the [File I/O Protocol] (called HIO in some 3DS
/// homebrew resources). Both stdout and stderr will be printed to the GDB console.
///
/// Creating this runner at the beginning of a doctest enables output from failing
/// tests. Without `GdbRunner`, tests will still fail on panic, but they won't display
/// anything written to `stdout` or `stderr`.
///
/// The runner should remain in scope for the remainder of the test.
///
/// [File I/O Protocol]: https://sourceware.org/gdb/onlinedocs/gdb/File_002dI_002fO-Overview.html#File_002dI_002fO-Overview
///
/// # Examples
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```
/// let _runner = test_runner::GdbRunner::default();
/// assert_eq!(2 + 2, 4);
/// ```
#[cfg_attr(not(doctest), doc = "````")]
///
#[cfg_attr(not(doctest), doc = "````")]
/// ```should_panic
/// let _runner = test_runner::GdbRunner::default();
/// assert_eq!(2 + 2, 5);
/// ```
#[cfg_attr(not(doctest), doc = "````")]
pub struct GdbRunner(());

impl Default for GdbRunner {
    fn default() -> Self {
        || -> ctru::Result<()> {
            // TODO: `ctru` expose safe API to do this and call that instead
            unsafe {
                ResultCode(ctru_sys::gdbHioDevInit())?;
                // TODO: should we actually redirect stdin or nah?
                ResultCode(ctru_sys::gdbHioDevRedirectStdStreams(true, true, true))?;
            }
            Ok(())
        }()
        .expect("failed to redirect I/O streams to GDB");

        Self(())
    }
}

impl Drop for GdbRunner {
    fn drop(&mut self) {
        unsafe { ctru_sys::gdbHioDevExit() }
    }
}

impl TestRunner for GdbRunner {
    type Context<'this> = ();

    fn new() -> Self {
        Self::default()
    }

    fn setup(&mut self) -> Self::Context<'_> {}

    fn cleanup<T: Termination>(self, test_result: T) -> T {
        // GDB actually has the opportunity to inspect the exit code,
        // unlike other runners, so let's follow the default behavior of the
        // stdlib test runner.
        test_result.report().exit_process()
    }
}
