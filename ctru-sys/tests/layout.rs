//! This is a stub for the generated layout test. We use bindgen callbacks along
//! with the [`cpp`] crate to compile actual `sizeof` and `alignof` calls in C,
//! as opposed to bindgen's generated layout tests which use hardcoded size literals.
//!
//! This should help ensure that the generated bindings are correct for the actual
//! ABI used by libctru and the devkitARM toolchain, instead of just what libclang
//! thinks they should be at bindgen time.

#![allow(non_snake_case)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]

extern crate shim_3ds;

use binding_helpers::{align_of, size_of};

// TODO: might want to move this into a test crate so we can avoid compiling it
// for non-test builds? Idk if there's a reasonable way to do it though.

use cpp::cpp;
use std::mem;

include!(concat!(env!("OUT_DIR"), "/layout_test.rs"));
