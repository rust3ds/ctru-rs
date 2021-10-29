use std::fmt;

pub type Result<T> = ::std::result::Result<T, Error>;

/// The error type returned by all libctru functions.
pub enum Error {
    Os(i32),
}

impl From<i32> for Error {
    fn from(err: i32) -> Self {
        Error::Os(err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Os(err) => write!(f, "libctru result code: {:08X}", err),
        }
    }
}

// TODO: Expand libctru result code into human-readable error message
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Os(err) => write!(f, "libctru result code: 0x{:08X}", err),
        }
    }
}
