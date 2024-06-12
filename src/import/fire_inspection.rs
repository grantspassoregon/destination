//! The `fire_inspections` module imports data from fire inspections into the library to facilitate
//! address matching.
use crate::prelude::*;
use aid::prelude::*;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

/// The `FireInspectionRaw` struct functions as a builder for a [`FireInspection`] struct.
/// The fields correspond to the csv of fire inspection data from the fire department.
/// A raw inspection represents the address a String, as opposed to a [`PartialAddress`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
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
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Deref, DerefMut,
)]
pub struct FireInspectionsRaw(Vec<FireInspectionRaw>);

impl FireInspectionsRaw {
    /// Used to read fire inspection data in from the csv source file.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(FireInspectionsRaw(records))
    }
}

/// The `FireInspection` struct contains fields from a fire inspection record, with the business
/// address mapped to a [`PartialAddress`].  Built from a [`FireInspectionRaw`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FireInspection {
    // Business name.
    name: String,
    // Business address (partial).
    address: PartialAddress,
    // Field used by fire dept.
    class: Option<String>,
    // Field used by fire dept.
    subclass: Option<String>,
}

impl FireInspection {
    /// The `name` method returns the cloned value of the `name` field containing the business name.
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    /// The `address` method returns the cloned value of the `address` field.
    pub fn address(&self) -> PartialAddress {
        self.address.clone()
    }

    /// The `class` method returns the cloned value of the `class` field.
    pub fn class(&self) -> Option<String> {
        self.class.clone()
    }

    /// The `subclass` method returns the cloned value of the `subclass` field.
    pub fn subclass(&self) -> Option<String> {
        self.subclass.clone()
    }
}

impl TryFrom<FireInspectionRaw> for FireInspection {
    type Error = Bandage;

    fn try_from(raw: FireInspectionRaw) -> Result<Self, Self::Error> {
        match parse_address(&raw.address) {
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
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Deref, DerefMut,
)]
pub struct FireInspections(Vec<FireInspection>);

impl FireInspections {
    /// Reads in the data as a raw fire inspections, attempts to parse each address, returning a
    /// `FireInspections` if successful.
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
