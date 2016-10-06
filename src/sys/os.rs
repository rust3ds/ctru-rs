use libctru::libc;
use io::ErrorKind;
use collections::{str, String};
use collections::borrow::ToOwned;
use ffi::CStr;

const TMPBUF_SZ: usize = 128;

pub fn decode_error_kind(errno: i32) -> ErrorKind {
    match errno as libc::c_int {
        libc::ECONNREFUSED => ErrorKind::ConnectionRefused,
        libc::ECONNRESET => ErrorKind::ConnectionReset,
        libc::EPERM | libc::EACCES => ErrorKind::PermissionDenied,
        libc::EPIPE => ErrorKind::BrokenPipe,
        libc::ENOTCONN => ErrorKind::NotConnected,
        libc::ECONNABORTED => ErrorKind::ConnectionAborted,
        libc::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        libc::EADDRINUSE => ErrorKind::AddrInUse,
        libc::ENOENT => ErrorKind::NotFound,
        libc::EINTR => ErrorKind::Interrupted,
        libc::EINVAL => ErrorKind::InvalidInput,
        libc::ETIMEDOUT => ErrorKind::TimedOut,
        libc::EEXIST => ErrorKind::AlreadyExists,

        // These two constants can have the same value on some systems,
        // but different values on others, so we can't use a match
        // clause
        x if x == libc::EAGAIN || x == libc::EWOULDBLOCK => ErrorKind::WouldBlock,

        _ => ErrorKind::Other,
    }
}

extern "C" {
        #[cfg(not(target_os = "dragonfly"))]
        #[cfg_attr(any(target_os = "linux", target_os = "emscripten"),
                   link_name = "__errno_location")]
        #[cfg_attr(any(target_os = "bitrig",
                       target_os = "netbsd",
                       target_os = "openbsd",
                       target_os = "android",
                       target_env = "newlib"),
                   link_name = "__errno")]
        #[cfg_attr(target_os = "solaris", link_name = "___errno")]
        #[cfg_attr(any(target_os = "macos",
                       target_os = "ios",
                       target_os = "freebsd"),
                       link_name = "__error")]
    fn errno_location() -> *mut libc::c_int;
}

pub fn errno() -> i32 {
    unsafe { (*errno_location()) as i32 }
}

/// Gets a detailed string description for the given error number.
pub fn error_string(errno: i32) -> String {
    extern "C" {
            #[cfg_attr(any(target_os = "linux", target_env = "newlib"),
                       link_name = "__xpg_strerror_r")]
        fn strerror_r(errnum: libc::c_int,
                      buf: *mut libc::c_char,
                      buflen: libc::size_t)
                      -> libc::c_int;
    }

    let mut buf = [0 as libc::c_char; TMPBUF_SZ];

    let p = buf.as_mut_ptr();
    unsafe {
        if strerror_r(errno as libc::c_int, p, buf.len() as libc::size_t) < 0 {
            panic!("strerror_r failure");
        }

        let p = p as *const _;
        str::from_utf8(CStr::from_ptr(p).to_bytes()).unwrap().to_owned()
    }
}
