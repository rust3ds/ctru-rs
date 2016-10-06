pub const STDOUT_FILENO: c_int = 1;

pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const EINTR: c_int = 4;
pub const EAGAIN: c_int = 11;
pub const EACCES: c_int = 13;
pub const EEXIST: c_int = 17;
pub const EINVAL: c_int = 22;
pub const EPIPE: c_int = 32;
pub const EWOULDBLOCK: c_int = EAGAIN;

pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ECONNABORTED: c_int = 103;
pub const ECONNRESET: c_int = 104;
pub const ENOTCONN: c_int = 107;
pub const ETIMEDOUT: c_int = 110;
pub const ECONNREFUSED: c_int = 111;
pub const EHOSTDOWN: c_int = 112;

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

pub type size_t = usize;
pub type ssize_t = isize;

extern "C" {
    pub fn __errno() -> *const c_int;
    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
}
