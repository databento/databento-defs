use std::{ffi::NulError, fmt::Display, num::TryFromIntError};

#[derive(Debug, Clone)]
pub enum Error {
    /// Received an unexpected `NULL` back from an FFI function.
    NullPointer,
    /// Failed type conversion or casting.
    TypeConversion(&'static str),
    /// A file that was expected to exist does not.
    FileDoesNotExist(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NullPointer => write!(f, "Received unexpected NULL from the FFI"),
            Error::TypeConversion(msg) => write!(f, "Type conversion error: {msg}"),
            Error::FileDoesNotExist(path) => write!(f, "Path doesn't exist: {path}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<NulError> for Error {
    fn from(_: NulError) -> Self {
        Self::TypeConversion("Missing null byte in CString conversion")
    }
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::TypeConversion("Out of range int conversion")
    }
}
