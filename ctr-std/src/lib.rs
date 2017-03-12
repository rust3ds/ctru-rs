#![feature(alloc)]
#![feature(allow_internal_unstable)]
#![feature(box_syntax)]
#![feature(cfg_target_has_atomic)]
#![feature(cfg_target_thread_local)]
#![feature(collections)]
#![feature(collections_bound)]
#![feature(collections_range)]
#![feature(const_fn)]
#![feature(compiler_builtins_lib)]
#![feature(core_intrinsics)]
#![feature(char_escape_debug)]
#![feature(dropck_eyepatch)]
#![feature(float_extras)]
#![feature(fn_traits)]
#![feature(fnbox)]
#![feature(fused)]
#![feature(generic_param_attrs)]
#![feature(int_error_internals)]
#![feature(integer_atomics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(oom)]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(prelude_import)]
#![feature(raw)]
#![feature(shared)]
#![feature(slice_concat_ext)]
#![feature(slice_patterns)]
#![feature(staged_api)]
#![feature(str_internals)]
#![feature(thread_local)]
#![feature(try_from)]
#![feature(unboxed_closures)]
#![feature(unicode)]
#![feature(unique)]
#![feature(untagged_unions)]
#![feature(zero_one)]
#![allow(non_camel_case_types, dead_code, unused_features)]
#![no_std]

#![stable(feature = "rust1", since = "1.0.0")]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

#[macro_reexport(assert, assert_eq, debug_assert, debug_assert_eq,
                 unreachable, unimplemented, write, writeln, try)]
extern crate core as __core;

#[macro_use]
#[macro_reexport(vec, format)]
extern crate collections as core_collections;

extern crate alloc;
extern crate std_unicode;
extern crate alloc_system;

// compiler-rt intrinsics
extern crate compiler_builtins;

// 3ds-specific dependencies
extern crate ctr_libc as libc;
extern crate ctru_sys as libctru;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// The Rust prelude
pub mod prelude;

// Public module declarations and reexports
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::any;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cell;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::clone;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cmp;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::convert;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::default;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::hash;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::intrinsics;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::iter;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::marker;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::mem;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ops;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ptr;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::raw;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::result;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::option;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::isize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i64;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::usize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u64;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::boxed;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::rc;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::borrow;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::fmt;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::slice;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::str;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::string;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::vec;
#[stable(feature = "rust1", since = "1.0.0")]
pub use std_unicode::char;

pub mod f32;
pub mod f64;

#[macro_use]
pub mod thread;
pub mod ascii;
pub mod fs;
pub mod collections;
pub mod error;
pub mod ffi;
pub mod io;
pub mod num;
pub mod os;
pub mod panic;
pub mod path;
pub mod sync;
pub mod time;

// Platform-abstraction modules
#[macro_use]
mod sys_common;
mod sys;

// Private support modules
mod panicking;
mod memchr;

// The runtime entry point and a few unstable public functions used by the
// compiler
pub mod rt;

// NOTE: These two are "undefined" symbols that LLVM emits but that
// we never actually use
#[doc(hidden)]

#[stable(feature = "3ds", since = "1.0.0")]
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr0() {
    intrinsics::unreachable()
}

#[stable(feature = "3ds", since = "1.0.0")]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr1() {
    intrinsics::unreachable()
}
