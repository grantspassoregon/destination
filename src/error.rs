/// The `AddressError` struct serves as the main error type for the `address` library.
#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
#[from(AddressErrorKind)]
pub struct AddressError {
    kind: Box<AddressErrorKind>,
}

impl From<Box<bincode::ErrorKind>> for AddressError {
    fn from(value: Box<bincode::ErrorKind>) -> Self {
        let error = Bincode::from(value);
        AddressErrorKind::from(error).into()
    }
}

/// The `AddressErrorKind` enum contains the individual error type associated with the library operation.
#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
pub enum AddressErrorKind {
    /// The `Bincode` variant contains a [`Bincode`] error.
    #[from(Bincode)]
    Bincode(Bincode),
    /// The `Builder` variant contains a [`Builder`] error.
    #[from(Builder)]
    Builder,
    /// The `Csv` variant contains a [`Csv`] error.
    #[from(Csv)]
    Csv(Csv),
    /// The `Io` variant contains an [`Io`] error.
    #[from(Io)]
    Io(Io),
    /// The `Nom` variant contains an [`Nom`] error.
    #[from(Nom)]
    Nom(Nom),
}

/// The `Io` struct contains error information associated with input/output calls.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_new::new)]
#[display("io error at path {:?}", self.path)]
pub struct Io {
    path: std::path::PathBuf,
    source: std::io::Error,
}

/// The `Csv` struct contains error information associated with the `csv` crate.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_new::new)]
#[display("csv error at path {:?}", self.path)]
pub struct Csv {
    path: std::path::PathBuf,
    source: csv::Error,
}

/// The `Bincode` struct contains error information associated with the `bincode` crate.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("bincode error")]
#[from(Box<bincode::ErrorKind>)]
pub struct Bincode {
    source: Box<bincode::ErrorKind>,
}

/// The `Builder` struct contains error information about failure to construct a type from a builder.
#[derive(Debug, derive_more::Display, derive_new::new)]
#[display("Error constructing {}: {}", self.target, self.issue)]
pub struct Builder {
    issue: String,
    target: String,
}

impl std::error::Error for Builder {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// The `Nom` struct contains error information associated with the `nom` crate.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("nom parsing error")]
pub struct Nom {
    description: String,
    source: nom::Err<nom::error::Error<String>>,
}

impl Nom {
    /// The `new` method creates a new instance of the error type.
    pub fn new(description: String, source: nom::Err<nom::error::Error<&str>>) -> Self {
        let source = source.to_owned();
        Self {
            description,
            source,
        }
    }
}
