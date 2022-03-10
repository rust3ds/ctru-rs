use std::error;
use std::fmt;

use ctru_sys::result::{R_DESCRIPTION, R_LEVEL, R_MODULE, R_SUMMARY};

pub type Result<T> = ::std::result::Result<T, Error>;

/// The error type returned by all libctru functions.
#[non_exhaustive]
pub enum Error {
    Os(ctru_sys::Result),
    ServiceAlreadyActive(&'static str),
}

impl From<ctru_sys::Result> for Error {
    fn from(err: ctru_sys::Result) -> Self {
        Error::Os(err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Os(err) => f
                .debug_struct("Error")
                .field("raw", &format_args!("{:#08X}", err))
                .field("description", &R_DESCRIPTION(err))
                .field("module", &R_MODULE(err))
                .field("summary", &R_SUMMARY(err))
                .field("level", &R_LEVEL(err))
                .finish(),
            Error::ServiceAlreadyActive(service) => f
                .debug_tuple("ServiceAlreadyActive")
                .field(&String::from(service))
                .finish(),
        }
    }
}

// TODO: Expand libctru result code into human-readable error message. These should be useful:
// https://www.3dbrew.org/wiki/Error_codes
// https://github.com/devkitPro/libctru/blob/master/libctru/include/3ds/result.h
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Os(err) => write!(f, "libctru result code: 0x{:08X}", err),
            Error::ServiceAlreadyActive(service) => write!(f, "Service {service} already active"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "error originating from a libctru function"
    }
}
