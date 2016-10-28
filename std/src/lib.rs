#![feature(alloc)]
#![feature(allow_internal_unstable)]
#![feature(collections)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(char_escape_debug)]
#![feature(float_extras)]
#![feature(int_error_internals)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(optin_builtin_traits)]
#![feature(prelude_import)]
#![feature(raw)]
#![feature(slice_concat_ext)]
#![feature(slice_patterns)]
#![feature(str_internals)]
#![feature(try_from)]
#![feature(unicode)]
#![feature(zero_one)]
#![allow(non_camel_case_types)]
#![no_std]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*; 
#[macro_reexport(assert, assert_eq, debug_assert, debug_assert_eq,
                 unreachable, unimplemented, write, writeln)]
extern crate core as __core;
#[macro_use]
#[macro_reexport(vec, format)]
extern crate collections as core_collections;
extern crate alloc;
extern crate rustc_unicode;

extern crate alloc_system;

extern crate ctru_sys as libctru;
extern crate spin;

pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::intrinsics;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::ptr;
pub use core::raw;
pub use core::result;
pub use core::option;

pub use alloc::arc;
pub use alloc::boxed;
pub use alloc::rc;

pub use core_collections::borrow;
pub use core_collections::fmt;
pub use core_collections::slice;
pub use core_collections::str;
pub use core_collections::string;
pub use core_collections::vec;

pub use rustc_unicode::char;

#[macro_use]
pub mod macros;

pub mod prelude;

pub use core::isize;
pub use core::i8;
pub use core::i16;
pub use core::i32;
pub use core::i64;

pub use core::usize;
pub use core::u8;
pub use core::u16;
pub use core::u32;
pub use core::u64;

#[path = "num/f32.rs"] pub mod f32;
#[path = "num/f64.rs"] pub mod f64;

pub mod ascii;
pub mod error;
pub mod ffi;
pub mod io;
pub mod num;
pub mod path;
pub mod rt;
pub mod sync;
mod memchr;
mod panicking;
mod sys;
