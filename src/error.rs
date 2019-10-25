//! Module holding a custom Error in order to handle the various errors we may be facing with ease
//!
//! This is very much inspired by `https://github.com/BurntSushi/rust-csv/blob/master/src/error.rs`
//! and `https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html`
//!
//! (not to say copy pasted :grin:)

use std::{io, result};

use derive_more::Display;
use reqwest;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Display)]
pub enum Error {
    Io(io::Error),

    #[display(fmt = "Missing Environment Variable: {}", _0)]
    VarError(std::env::VarError),

    #[display(fmt = "Reqwest Error: {}", _0)]
    Reqwest(reqwest::Error),

    #[display(fmt = "Error: {}", _0)]
    Other(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::VarError(ref e) => Some(e),
            Error::Reqwest(ref e) => Some(e),
            Error::Other(ref _str) => None,
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::VarError(ref e) => e.description(),
            Error::Reqwest(ref e) => e.description(),
            Error::Other(ref e) => &e,
        }
    }

    fn cause(&self) -> Option<&(dyn std::error::Error)> {
        // Generic error, underlying cause isn't tracked yet
        None
    }
}

// From `std::env:VarError` to our `Error`
impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Error {
        Error::VarError(e)
    }
}

// From `std::io:Error` to our `Error`
impl From<std::io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

// From `reqwest::Error` to an `Error`
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

// Convert an `Error` to an `std::io::Error`
impl From<Error> for io::Error {
    fn from(e: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, e)
    }
}
