#![feature(alloc)]
#![feature(allow_internal_unstable)]
#![feature(collections)]
#![feature(core_intrinsics)]
#![feature(char_escape_debug)]
#![feature(int_error_internals)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(prelude_import)]
#![feature(slice_concat_ext)]
#![feature(slice_patterns)]
#![feature(str_internals)]
#![feature(try_from)]
#![feature(unicode)]

#![no_std]

#![crate_type = "rlib"]
#![crate_name = "ctru"]

extern crate alloc;
extern crate alloc_system;
#[macro_reexport(format, vec)]
#[macro_use]
extern crate collections;
extern crate rustc_unicode;

extern crate ctru_sys as libctru;

#[prelude_import]
#[allow(unused)]
use prelude::*;

pub mod std {
    pub use core::{any, cell, clone, cmp, convert, default, hash, i16, i32, i64, i8, isize, iter,
                   marker, mem, ops, option, ptr, result, u16, u32, u64, u8, usize, intrinsics};
    pub use rustc_unicode::char;
    pub use alloc::{arc, rc};
    pub use collections::{borrow, boxed, fmt, slice, str, string, vec};
    pub use system::{error, io, memchr, ascii, ffi, path};

    pub mod collections {
        pub use collections::{binary_heap, btree_map, btree_set, linked_list, vec_deque,
                              BinaryHeap, LinkedList, VecDeque, String, Vec, BTreeMap, BTreeSet};
    }
}

pub mod prelude {
    pub use std;
    pub use std::marker::{Copy, Send, Sized, Sync};
    pub use std::ops::{Drop, Fn, FnMut, FnOnce};
    pub use std::mem::drop;
    pub use std::boxed::Box;
    pub use std::borrow::ToOwned;
    pub use std::clone::Clone;
    pub use std::cmp::{PartialEq, PartialOrd, Eq, Ord};
    pub use std::convert::{AsRef, AsMut, Into, From};
    pub use std::default::Default;
    pub use std::iter::{Iterator, Extend, IntoIterator};
    pub use std::iter::{DoubleEndedIterator, ExactSizeIterator};
    pub use std::option::Option::{self, Some, None};
    pub use std::result::Result::{self, Ok, Err};
    pub use std::slice::SliceConcatExt;
    pub use std::string::{String, ToString};
    pub use std::vec::Vec;
    pub use std::fmt::Write;
}

pub use std::{fmt, boxed, vec};

pub mod console;
pub mod srv;
pub mod gfx;
pub mod services;
pub mod sdmc;
pub mod system;

pub use srv::Srv;
pub use gfx::Gfx;
pub use sdmc::Sdmc;
