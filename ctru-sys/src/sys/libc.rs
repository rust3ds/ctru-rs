#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

pub const STDOUT_FILENO: c_int = 1;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_char = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;
pub type size_t = usize;
pub type ssize_t = isize;

pub type u_char = c_uchar;
pub type u_short = c_ushort;
pub type u_int = c_uint;
pub type u_long = c_ulong;
pub type ushort = c_ushort;
pub type uint_ = c_uint;
pub type ulong = c_ulong;
pub type clock_t = c_ulong;
pub type time_t = c_long;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}
impl ::core::default::Default for timespec {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
impl ::core::default::Default for itimerspec {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type daddr_t = c_long;
pub type caddr_t = *mut c_char;
pub type ino_t = c_uint;
pub type off_t = c_long;
pub type dev_t = c_int;
pub type uid_t = c_ushort;
pub type gid_t = c_ushort;
pub type pid_t = c_int;
pub type key_t = c_long;
pub type mode_t = c_uint;
pub type nlink_t = c_ushort;
pub type fd_mask = c_long;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _types_fd_set {
    pub fds_bits: [fd_mask; 1usize],
}
impl ::core::default::Default for _types_fd_set {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
pub type clockid_t = c_ulong;
pub type timer_t = c_ulong;
pub type useconds_t = c_ulong;
pub type suseconds_t = c_long;
pub type fsblkcnt_t = c_uint;
pub type fsfilcnt_t = c_uint;

extern "C" {
    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
}
