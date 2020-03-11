//! Module holding a custom Error in order to handle the various errors we may be facing with ease
//!
//! This is very much inspired by `https://github.com/BurntSushi/rust-csv/blob/master/src/error.rs`
//! and `https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html`
//!
//! (not to say copy pasted :grin:)
use actix_web::{error::ResponseError, HttpResponse};
use config;
use derive_more::Display;
use reqwest;
use std::{io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    Io(io::Error),

    #[display(fmt = "Error: {}", _0)]
    Other(String),

    #[display(fmt = "Reqwest Error: {}", _0)]
    Reqwest(reqwest::Error),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "ConfigError: {}", _0)]
    ConfigError(config::ConfigError),

    #[display(fmt = "Missing Environment Variable: {}", _0)]
    VarError(std::env::VarError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::BadRequest(ref _str) => None,
            Error::ConfigError(ref e) => Some(e),
            Error::InternalServerError => None,
            Error::Io(ref e) => Some(e),
            Error::Other(ref _str) => None,
            Error::Reqwest(ref e) => Some(e),
            Error::Unauthorized => None,
            Error::VarError(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::BadRequest(ref e) => &e,
            Error::ConfigError(ref e) => e.description(),
            Error::InternalServerError => "InternalServerError",
            Error::Io(ref e) => e.description(),
            Error::Other(ref e) => &e,
            Error::Reqwest(ref e) => e.description(),
            Error::Unauthorized => "Unauthorized",
            Error::VarError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&(dyn std::error::Error)> {
        // Generic error, underlying cause isn't tracked yet
        None
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            Error::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            _ => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
        }
    }
}

// From `std::env:VarError` to our `Error`
impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Error {
        Error::VarError(e)
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Error {
        Error::ConfigError(e)
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
