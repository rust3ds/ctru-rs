// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(bad_style, overflowing_literals, improper_ctypes, non_camel_case_types)]

// Attributes needed when building as part of the standard library
#![cfg_attr(stdbuild, feature(no_std, core, core_slice_ext, staged_api, custom_attribute, cfg_target_vendor))]
#![cfg_attr(stdbuild, feature(link_cfg))]
#![cfg_attr(stdbuild, no_std)]
#![cfg_attr(stdbuild, staged_api)]
#![cfg_attr(stdbuild, allow(warnings))]
#![cfg_attr(stdbuild, unstable(feature = "libc",
                               reason = "use `libc` from crates.io",
                               issue = "27783"))]

#![cfg_attr(not(feature = "use_std"), no_std)]

#[cfg(all(not(stdbuild), not(dox), feature = "use_std"))]
extern crate std as core;

mod constants;
mod functions;
pub use constants::*;
pub use functions::*;

#[link(name = "c")]
#[link(name = "m")]
extern {}

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

// char is u8 on ARM
pub type c_char = u8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;

// 4 bytes on ARM
pub type c_long = i32;
pub type c_ulong = u32;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

// devkitARM says wchar_t is 4 bytes. Nintendo's API says it's 2 bytes.
// hope you never have to interact between the two...
pub type wchar_t = c_int;

pub type int8_t = i8;
pub type uint8_t = u8;
pub type int16_t = i16;
pub type uint16_t = u16;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type int64_t = i64;
pub type uint64_t = u64;

pub type time_t = i32;
pub type clockid_t = c_int;
pub type mode_t = u32;
pub type sighandler_t = size_t;
pub type dev_t = u32;
pub type nlink_t = u32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type off_t = i64;
pub type blksize_t = i32;
pub type blkcnt_t = c_ulong;
pub type fsblkcnt_t = uint64_t;
pub type fsfilcnt_t = uint32_t;
pub type ino_t = u32;
pub type suseconds_t = i32;
pub type error_t = c_int;

pub enum timezone {}

pub enum _reent {}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
}

pub enum DIR {}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_atime: time_t,
    pub st_spare1: c_long,
    pub st_mtime: time_t,
    pub st_spare2: c_long,
    pub st_ctime: time_t,
    pub st_spare3: c_long,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_spare4: [c_long; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct statvfs {
    pub f_bsize: c_ulong,
    pub f_frsize: c_ulong,
    pub f_blocks: fsblkcnt_t,
    pub f_bfree: fsblkcnt_t,
    pub f_bavail: fsblkcnt_t,
    pub f_files: fsfilcnt_t,
    pub f_ffree: fsfilcnt_t,
    pub f_favail: fsfilcnt_t,
    pub f_fsid: c_ulong,
    pub f_flag: c_ulong,
    pub f_namemax: c_ulong,
}

#[repr(C)]
#[derive(Copy)]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256usize],
}
impl Clone for dirent {
    fn clone(&self) -> Self { *self }
}
