//! The `street_name_pre_type` module provides the `StreetNamePreType` struct, which holds variants
//! of the street name pre type currently in use by ECSO.  We do not cover all the valid post
//! types, because some valid post types are in use as street names, and we want "Park" and "Fall"
//! to map to street names, while catching those existing cases in our area using pre types.  We do
//! not issue addresses with pre types.
use derive_more::Display;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

/// The `StreetNamePreType` is the pre-type element of a complete street name.
#[allow(missing_docs)]
#[derive(
    Copy,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    Display,
)]
pub enum StreetNamePreType {
    Avenue,
    Fork,
    #[default]
    Highway,
    Interstate,
    Mount,
}

impl StreetNamePreType {
    /// The `label` method returns the street name pre-type in all caps, for printing
    /// labels.
    pub fn label(&self) -> String {
        let label = match self {
            Self::Avenue => "AVENUE",
            Self::Fork => "FORK",
            Self::Highway => "HIGHWAY",
            Self::Interstate => "INTERSTATE",
            Self::Mount => "MOUNT",
        };
        label.to_string()
    }

    /// The `match_mixed` method attempts to match the string `input` against a variant of
    /// `StreetNamePreType`.
    pub fn match_mixed(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "avenue" => Some(Self::Avenue),
            "ave" => Some(Self::Avenue),
            "fork" => Some(Self::Fork),
            "highway" => Some(Self::Highway),
            "hwy" => Some(Self::Highway),
            "interstate" => Some(Self::Interstate),
            "mount" => Some(Self::Mount),
            "mt" => Some(Self::Mount),
            _ => None,
        }
    }

    /// The `deserialize_mixed` method attempts to match the input to a valid street name
    /// pre-modifier variant.
    pub fn deserialize_mixed<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Self>, D::Error> {
        let intermediate = Deserialize::deserialize(de)?;
        Ok(Self::match_mixed(intermediate))
    }
}
