/// The `AddressError` struct serves as the main error type for the `address` library.
#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
#[from(AddressErrorKind)]
pub struct AddressError {
    kind: Box<AddressErrorKind>,
}

macro_rules! impl_address_error {
    ( $( $name:ident),* ) => {
        $(
            impl From<$name> for AddressError {
                fn from(value: $name) -> Self {
                    let kind = Box::new(AddressErrorKind::from(value));
                    Self { kind }
                }
            }
        )*
    };
    ( $( $name:ident),+ ,) => {
       impl_address_err![ $( $name ),* ];
    };
}

impl_address_error!(Decode, Encode, Io, Nom);

/// The `AddressErrorKind` enum contains the individual error type associated with the library operation.
#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
pub enum AddressErrorKind {
    /// The `Decode` variant contains a [`Decode`] error.
    #[from(Decode)]
    Decode(Decode),
    /// The `Encode` variant contains a [`Encode`] error.
    #[from(Encode)]
    Encode(Encode),
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
#[display("io error at path {path:?} in line {line} of {file}")]
pub struct Io {
    path: std::path::PathBuf,
    source: std::io::Error,
    line: u32,
    file: String,
}

/// The `Csv` struct contains error information associated with the `csv` crate.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_new::new)]
#[display("csv error at path {path:?} in line {line} of {file}")]
pub struct Csv {
    path: std::path::PathBuf,
    source: csv::Error,
    line: u32,
    file: String,
}

/// The `Decode` struct contains decode error information associated with the `bincode` crate.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_new::new)]
#[display("bincode decode error: {source:?} at line {line} in {file}")]
pub struct Decode {
    source: bincode::error::DecodeError,
    line: u32,
    file: String,
}

/// The `Encode` struct contains encoding error information associated with the `bincode` crate.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_new::new)]
#[display("bincode encode error: {source:?} at line {line} in {file}")]
pub struct Encode {
    source: bincode::error::EncodeError,
    line: u32,
    file: String,
}

/// The `Builder` struct contains error information about failure to construct a type from a builder.
#[derive(Debug, derive_more::Display, derive_new::new)]
#[display("Error constructing {target}: {issue} in line {line} of {file}")]
pub struct Builder {
    issue: String,
    target: String,
    line: u32,
    file: String,
}

impl std::error::Error for Builder {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// The `Nom` struct contains error information associated with the `nom` crate.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("nom parsing error: {description} in line {line} of {file}")]
pub struct Nom {
    description: String,
    source: nom::Err<nom::error::Error<String>>,
    line: u32,
    file: String,
}

impl Nom {
    /// The `new` method creates a new instance of the error type.
    pub fn new(
        description: String,
        source: nom::Err<nom::error::Error<&str>>,
        line: u32,
        file: String,
    ) -> Self {
        let source = source.to_owned();
        Self {
            description,
            source,
            line,
            file,
        }
    }
}
