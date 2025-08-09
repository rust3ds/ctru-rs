# test-runner

A helper crate for running automated Rust tests on a 3DS. Since the builtin
Rust test framework expects a more traditional OS with subprocesses etc.,
it's necessary to use `custom_test_frameworks` when running tests on 3DS
hardware or in an emulator to get a similar experience as a usual `cargo test`.

## Usage

First the test runner to your crate:

```sh
cargo add --dev test-runner --git https://github.com/rust3ds/ctru-rs
```

In `lib.rs` and any integration test files:

```rs
#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]
```

<!-- TODO document the different runners -->

## Caveats

* GDB doesn't seem to support separate output streams for `stdout` and `stderr`,
  so all test output to `stderr` will end up combined with `stdout` and both will be
  printed to the runner's `stdout`. If you know a workaround for this that doesn't
  require patching + building GDB itself please open an issue about it!

* Doctests require a bit of extra setup to work with the runner, since they don't
  use the crate's `#![test_runner]`. To write doctests, add the following to the
  beginning of the doctest (or `fn main()` if the test defines it):

  ```rust
  let _runner = test_runner::GdbRunner::default();
  ```

  The runner must remain in scope for the duration of the test in order for
  the test output to be printed.
