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

use cpp::cpp;
use ctru_sys::*;

fn size_of_ret<T, U>(_f: impl Fn(U) -> T) -> usize {
    ::std::mem::size_of::<T>()
}

macro_rules! size_of {
    ($ty:ident::$field:ident) => {{
        size_of_ret(|x: $ty| x.$field)
    }};
    ($ty:ty) => {
        ::std::mem::size_of::<$ty>()
    };
    ($expr:expr) => {
        ::std::mem::size_of_val(&$expr)
    };
}

fn align_of_ret<T, U>(_f: impl Fn(U) -> T) -> usize {
    ::std::mem::align_of::<T>()
}

macro_rules! align_of {
    ($ty:ident::$field:ident) => {{
        align_of_ret(|x: $ty| x.$field)
    }};
    ($ty:ty) => {
        ::std::mem::align_of::<$ty>()
    };
    ($expr:expr) => {
        ::std::mem::align_of_val(&$expr)
    };
}

include!(concat!(env!("OUT_DIR"), "/generated_layout_test.rs"));
