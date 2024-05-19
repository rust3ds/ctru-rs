//! This is a stub for the generated layout test. We use bindgen callbacks along
//! with the [`cpp`] crate to compile actual `sizeof` and `alignof` calls in C,
//! as opposed to bindgen's generated layout tests which use hardcoded size literals.
//!
//! This should help ensure that the generated bindings are correct for the actual
//! ABI used by libctru and the devkitARM toolchain, instead of just what libclang
//! thinks they should be at bindgen time.

#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_console)]

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod generated_tests {
    use bindgen_tests::{align_of, offset_of, size_of};

    use cpp::cpp;
    use ctru_sys::*;

    include!(concat!(env!("OUT_DIR"), "/generated_layout_test.rs"));
}
