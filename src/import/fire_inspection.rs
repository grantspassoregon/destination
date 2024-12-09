//! The `fire_inspections` module imports data from fire inspections into the library to facilitate
//! address matching.
use crate::{from_csv, Io, Parser, PartialAddress};
use aid::prelude::*;

/// The `FireInspectionRaw` struct functions as a builder for a [`FireInspection`] struct.
/// The fields correspond to the csv of fire inspection data from the fire department.
/// A raw inspection represents the address a String, as opposed to a [`PartialAddress`].
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "PascalCase")]
pub struct FireInspectionRaw {
    // Business name.
    name: String,
    // Address blob.
    address: String,
    // Field used by fire dept.
    class: Option<String>,
    // Field used by fire dept.
    subclass: Option<String>,
}

/// The `FireInspectionsRaw` struct is a wrapper around a vector of type [`FireInspectionRaw`]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct FireInspectionsRaw(Vec<FireInspectionRaw>);

impl FireInspectionsRaw {
    /// Used to read fire inspection data in from the csv source file.
    #[tracing::instrument(skip_all)]
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(FireInspectionsRaw(records))
    }

    /// Used to read fire inspection data in from the csv source file.
    #[tracing::instrument(skip_all)]
    pub fn _from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = crate::_from_csv(path)?;
        Ok(FireInspectionsRaw(records))
    }
}

/// The `FireInspection` struct contains fields from a fire inspection record, with the business
/// address mapped to a [`PartialAddress`].  Built from a [`FireInspectionRaw`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_", strip_option, borrow_self)]
pub struct FireInspection {
    #[setters(doc = "Sets the value of the `name` field representing the business name.")]
    name: String,
    #[setters(doc = "Sets the value of the `address` field representing the business address.")]
    address: PartialAddress,
    // Field used by fire dept.
    class: Option<String>,
    // Field used by fire dept.
    subclass: Option<String>,
}

impl TryFrom<FireInspectionRaw> for FireInspection {
    type Error = Bandage;

    fn try_from(raw: FireInspectionRaw) -> Result<Self, Self::Error> {
        match Parser::address(&raw.address) {
            Ok((_, address)) => {
                let mut upper_address = address.clone();
                if let Some(identifier) = address.subaddress_identifier() {
                    upper_address.set_subaddress_identifier(&identifier.to_uppercase())
                };
                Ok(FireInspection {
                    name: raw.name,
                    address: upper_address,
                    class: raw.class,
                    subclass: raw.subclass,
                })
            }
            Err(_) => Err(Bandage::Parse),
        }
    }
}

/// The `FireInspections` struct is a wrapper around a vector of type [`FireInspection`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct FireInspections(Vec<FireInspection>);

impl FireInspections {
    /// Reads in the data as a raw fire inspections, attempts to parse each address, returning a
    /// `FireInspections` if successful.
    #[tracing::instrument(skip_all)]
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Clean<Self> {
        // Try to read in as raw.
        let raw = FireInspectionsRaw::from_csv(path)?;
        let mut records = Vec::new();
        for record in raw.iter() {
            // Parse the raw address.
            records.push(FireInspection::try_from(record.clone())?);
        }
        Ok(FireInspections(records))
    }
}
