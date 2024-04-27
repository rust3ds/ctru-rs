pub mod gen;

use std::{fmt, io};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(io::Error),
    #[cfg(feature = "rustfmt")]
    Format(rust_format::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(io) => write!(f, "I/O error: {io}"),
            #[cfg(feature = "rustfmt")]
            Error::Format(fmt) => write!(f, "Format error: {fmt}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[cfg(feature = "rustfmt")]
impl From<rust_format::Error> for Error {
    fn from(e: rust_format::Error) -> Self {
        Self::Format(e)
    }
}
