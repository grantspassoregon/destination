//! The `error` module creates a library-specific Error type `AddressError`, and an alias for Result using the
//! `AddressError`, called `AddressResult`.
use thiserror::Error;

/// The `AddressError` enum represents the library-specific Error type.
#[derive(Error, Debug)]
pub enum AddressError {
    /// A `ParseError` indicates the Nom library returned an error.
    #[error("Parse error.")]
    ParseError,
    /// A `DeserializeError` indicates an error occurred during deserialization.
    #[error("Deserialize error.")]
    DeserializeError(#[from] serde::de::value::Error),
    /// Error conversion type for [`std::io::Error`].
    #[error("Input/output error from std.")]
    Io(#[from] std::io::Error),
    /// Error conversion type for [`std::env::VarError`].
    #[error("Could not read environmental variables from .env.")]
    EnvError(#[from] std::env::VarError),
    /// Error conversion type for [`std::ffi::OsString`].
    #[error("Bad file name {0:?}.")]
    FileNameError(std::ffi::OsString),
}

/// Alias for the Result type using the local Error type.
pub type AddressResult<T> = Result<T, AddressError>;
