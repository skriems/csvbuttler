//! Module holding a custom Error in order to handle the various errors we may be facing with ease
//!
//! This is very much inspired by `https://github.com/BurntSushi/rust-csv/blob/master/src/error.rs`
//! and `https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html`
//!
//! (not to say copy pasted :grin:)

use std::{fmt, io, result};

use reqwest;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    VarError(std::env::VarError),
    Reqwest(reqwest::Error),
    Other(String),
}

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
            ErrorKind::Io(ref e) => e.fmt(f),
            ErrorKind::VarError(ref e) => e.fmt(f),
            ErrorKind::Reqwest(ref e) => e.fmt(f),
            ErrorKind::Other(ref e) => e.fmt(f),
        }
    }
}

/// Convert an `ErrorKind` to an `Error`
impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::new(kind)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self.0 {
            ErrorKind::Io(ref e) => Some(e),
            ErrorKind::VarError(ref e) => Some(e),
            ErrorKind::Reqwest(ref e) => Some(e),
            ErrorKind::Other(ref _str) => None,
        }
    }

    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::Io(ref e) => e.description(),
            ErrorKind::VarError(ref e) => e.description(),
            ErrorKind::Reqwest(ref e) => e.description(),
            ErrorKind::Other(ref e) => &e,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        // Generic error, underlying cause isn't tracked yet
        None
    }
}

/// Convert a `std::env:VarError` to an `Error`
impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Error {
        Error::new(ErrorKind::VarError(e))
    }
}

/// Convert a `std::io:Error` to an `Error`
impl From<std::io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::new(ErrorKind::Io(e))
    }
}

/// Convert a `reqwest::Error` to an `Error`
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::new(ErrorKind::Reqwest(e))
    }
}

/// Convert a `String` to an `Error`
impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::new(ErrorKind::Other(e))
    }
}

/// Convert an `Error` to an `std::io::Error`
impl From<Error> for io::Error {
    fn from(e: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, e)
    }
}

