//! Module holding a custom Error in order to handle the various errors we may be facing with ease
//!
//! This is very much inspired by `https://github.com/BurntSushi/rust-csv/blob/master/src/error.rs`
//! and the `error::Error` module (not to say copy pasted :grin:)

use std::{fmt, io, result};

use reqwest;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Error(Box::new(kind))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

/// The implementation of the `fmt::Display` trait, basically passing through the implementations
/// of the underlying Errors
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::VarError(ref err) => err.fmt(f),
            ErrorKind::Reqwest(ref err) => err.fmt(f),
            ErrorKind::Other(ref err) => err.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    VarError(std::env::VarError),
    Reqwest(reqwest::Error),
    Other(String),
}

/// Convert an `ErrorKind` to an `Error`
impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::new(kind)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::Io(ref err) => err.description(),
            ErrorKind::VarError(ref err) => err.description(),
            ErrorKind::Reqwest(ref err) => err.description(),
            ErrorKind::Other(ref err) => &err,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        // Generic error, underlying cause isn't tracked yet
        None
    }
}

/// Convert a `std::env:VarError` to an `error::Error`
impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Error {
        Error::new(ErrorKind::VarError(err))
    }
}

/// Convert a `std::io:Error` to an `error::Error`
impl From<std::io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(ErrorKind::Io(err))
    }
}

/// Convert a `reqwest::error:Error` to an `error::Error`
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::new(ErrorKind::Reqwest(err))
    }
}

/// Convert an `crate::error::Error` to an `std::io::Error`
impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

/// Convert a `String` to an `error::Error`
impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::new(ErrorKind::Other(err))
    }
}
