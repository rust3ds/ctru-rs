use std::error;
use std::ffi::CStr;
use std::fmt;
use std::ops::{ControlFlow, FromResidual, Try};

use ctru_sys::result::{R_DESCRIPTION, R_LEVEL, R_MODULE, R_SUMMARY};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(crate) struct LibCtruError(pub i32);

impl Try for LibCtruError {
    type Output = ();
    type Residual = crate::Result<core::convert::Infallible>;

    fn from_output(_: Self::Output) -> Self {
        Self(0)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            0 => ControlFlow::Continue(()),
            _ => ControlFlow::Break(Err(self.into())),
        }
    }
}

impl FromResidual for LibCtruError {
    fn from_residual(_: <Self as Try>::Residual) -> Self {
        Self(1)
    }
}

/// The error type returned by all libctru functions.
#[non_exhaustive]
pub enum Error {
    Os(ctru_sys::Result),
    Libc(String),
    ServiceAlreadyActive,
    OutputAlreadyRedirected,
}

impl Error {
    /// Create an [`Error`] out of the last set value in `errno`. This can be used
    /// to get a human-readable error string from calls to `libc` functions.
    pub(crate) fn from_errno() -> Self {
        let error_str = unsafe {
            let errno = ctru_sys::errno();
            let str_ptr = libc::strerror(errno);

            // Safety: strerror should always return a valid string,
            // even if the error number is unknown
            CStr::from_ptr(str_ptr)
        };

        // Copy out of the error string, since it may be changed by other libc calls later
        Self::Libc(error_str.to_string_lossy().into())
    }
}

impl From<ctru_sys::Result> for Error {
    fn from(err: ctru_sys::Result) -> Self {
        Error::Os(err)
    }
}

impl From<LibCtruError> for Error {
    fn from(err: LibCtruError) -> Self {
        Self::Os(err.0)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Self::Os(err) => f
                .debug_struct("Error")
                .field("raw", &format_args!("{:#08X}", err))
                .field("description", &R_DESCRIPTION(err))
                .field("module", &R_MODULE(err))
                .field("summary", &R_SUMMARY(err))
                .field("level", &R_LEVEL(err))
                .finish(),
            Self::Libc(err) => f.debug_tuple("Libc").field(err).finish(),
            Self::ServiceAlreadyActive => f.debug_tuple("ServiceAlreadyActive").finish(),
            Self::OutputAlreadyRedirected => f.debug_tuple("OutputAlreadyRedirected").finish(),
        }
    }
}

// TODO: Expand libctru result code into human-readable error message. These should be useful:
// https://www.3dbrew.org/wiki/Error_codes
// https://github.com/devkitPro/libctru/blob/master/libctru/include/3ds/result.h
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Self::Os(err) => write!(f, "libctru result code: 0x{:08X}", err),
            Self::Libc(err) => write!(f, "{}", err),
            Self::ServiceAlreadyActive => write!(f, "Service already active"),
            Self::OutputAlreadyRedirected => {
                write!(f, "output streams are already redirected to 3dslink")
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "error originating from a libctru function"
    }
}
