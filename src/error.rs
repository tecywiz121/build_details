use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result::Result as StdResult;

#[derive(Debug)]
pub enum Error {
    Fmt(fmt::Error),
    Io(io::Error),
    Missing,
    MissingDetail(String),
    MissingEnv(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Fmt(e) => write!(f, "Unable to format: {}", e),
            Error::Io(e) => write!(f, "Input/output error: {}", e),
            Error::Missing => write!(f, "Missing value"),
            Error::MissingDetail(x) => write!(f, "Missing value: {}", x),
            Error::MissingEnv(x) => write!(f, "A required environment variable is missing: {}", x),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &'static str {
        match self {
            Error::Fmt(_) => "formatting error",
            Error::Io(_) => "input/output error",
            Error::Missing => "missing detail",
            Error::MissingDetail(_) => "missing detail",
            Error::MissingEnv(_) => "missing environment variable",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self {
            Error::Fmt(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            Error::Missing => None,
            Error::MissingDetail(_) => None,
            Error::MissingEnv(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(o: io::Error) -> Error {
        Error::Io(o)
    }
}

impl From<fmt::Error> for Error {
    fn from(o: fmt::Error) -> Error {
        Error::Fmt(o)
    }
}

pub type Result<T> = StdResult<T, Error>;
