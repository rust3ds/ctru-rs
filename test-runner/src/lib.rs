//! Custom test runner for building/running tests on the 3DS.
//!
//! This library can be used with
//! [`custom_test_frameworks`](https://doc.rust-lang.org/unstable-book/language-features/custom-test-frameworks.html)
//! to enable normal Rust testing workflows for 3DS homebrew.

#![feature(test)]
#![feature(custom_test_frameworks)]
#![feature(exitcode_exit_method)]
#![test_runner(run_gdb)]

extern crate test;

mod console;
mod gdb;
mod socket;

use std::process::{ExitCode, Termination};

pub use console::ConsoleRunner;
pub use gdb::GdbRunner;
pub use socket::SocketRunner;
use test::{ColorConfig, OutputFormat, TestDescAndFn, TestFn, TestOpts};

/// Run tests using the [`GdbRunner`].
/// This function can be used with the `#[test_runner]` attribute.
pub fn run_gdb(tests: &[&TestDescAndFn]) {
    run::<GdbRunner>(tests);
}

/// Run tests using the [`ConsoleRunner`].
/// This function can be used with the `#[test_runner]` attribute.
pub fn run_console(tests: &[&TestDescAndFn]) {
    run::<ConsoleRunner>(tests);
}

/// Run tests using the [`SocketRunner`].
/// This function can be used with the `#[test_runner]` attribute.
pub fn run_socket(tests: &[&TestDescAndFn]) {
    run::<SocketRunner>(tests);
}

fn run<Runner: TestRunner>(tests: &[&TestDescAndFn]) {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut runner = Runner::new();
    let ctx = runner.setup();

    let opts = TestOpts {
        force_run_in_process: true,
        run_tests: true,
        // TODO: color doesn't work because of TERM/TERMINFO.
        // With RomFS we might be able to fake this out nicely...
        color: ColorConfig::AlwaysColor,
        format: OutputFormat::Pretty,
        test_threads: Some(1),
        // Hopefully this interface is more stable vs specifying individual options,
        // and parsing the empty list of args should always work, I think.
        // TODO Ideally we could pass actual std::env::args() here too
        ..test::test::parse_opts(&[]).unwrap().unwrap()
    };

    let tests = tests.iter().map(|t| make_owned_test(t)).collect();
    let result = test::run_tests_console(&opts, tests);

    drop(ctx);

    let reportable_result = match result {
        Ok(true) => Ok(()),
        // Try to match stdlib console test runner behavior as best we can
        _ => Err(ExitCode::from(101)),
    };

    let _ = runner.cleanup(reportable_result);
}

/// Adapted from [`test::make_owned_test`].
/// Clones static values for putting into a dynamic vector, which `test_main()`
/// needs to hand out ownership of tests to parallel test runners.
///
/// This will panic when fed any dynamic tests, because they cannot be cloned.
fn make_owned_test(test: &TestDescAndFn) -> TestDescAndFn {
    let testfn = match test.testfn {
        TestFn::StaticTestFn(f) => TestFn::StaticTestFn(f),
        TestFn::StaticBenchFn(f) => TestFn::StaticBenchFn(f),
        _ => panic!("non-static tests passed to test::test_main_static"),
    };

    TestDescAndFn {
        testfn,
        desc: test.desc.clone(),
    }
}

/// A helper trait to make the behavior of test runners consistent.
trait TestRunner: Sized {
    /// Any context the test runner needs to remain alive for the duration of
    /// the test. This can be used for things that need to borrow the test runner
    /// itself.
    // TODO: with associated type defaults this could be `= ();`
    type Context<'this>
    where
        Self: 'this;

    /// Initialize the test runner.
    fn new() -> Self;

    /// Create the [`Context`](Self::Context), if any.
    fn setup(&mut self) -> Self::Context<'_>;

    /// Handle the results of the test and perform any necessary cleanup.
    /// The [`Context`](Self::Context) will be dropped just before this is called.
    ///
    /// This returns `T` so that the result can be used in doctests.
    fn cleanup<T: Termination>(self, test_result: T) -> T {
        test_result
    }
}

/// This module has stubs needed to link the test library, but they do nothing
/// because we don't actually need them for the runner to work.
mod link_fix {
    #[no_mangle]
    extern "C" fn execvp(
        _argc: *const libc::c_char,
        _argv: *mut *const libc::c_char,
    ) -> libc::c_int {
        -1
    }

    #[no_mangle]
    extern "C" fn pipe(_fildes: *mut libc::c_int) -> libc::c_int {
        -1
    }

    #[no_mangle]
    extern "C" fn sigemptyset(_arg1: *mut libc::sigset_t) -> ::libc::c_int {
        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        assert_eq!(2 + 2, 5);
    }
}
