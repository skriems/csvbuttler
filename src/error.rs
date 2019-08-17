use std::error;
use std::{fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error happened!")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Error happened"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked yet
        None
    }
}

/// convert a `std::env:VarError` to an `error::Error`
impl From<std::env::VarError> for Error {
    fn from(_: std::env::VarError) -> Error {
        Error
    }
}

/// convert an `crate::error::Error` to an `std::io::Error`
impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}
