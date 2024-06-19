//! The `street_separator` module provides the `StreetSeparator` struct, which holds variants
//! of the street name separator currently in use by ECSO.  We do
//! not issue addresses with street separators.
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
pub enum StreetSeparator {
    #[default]
    OfThe,
}

impl StreetSeparator {
    /// The `label` method returns the street name separator in all caps with spaces, for printing
    /// labels.
    pub fn label(&self) -> String {
        let label = match self {
            Self::OfThe => "OF THE",
        };
        label.to_string()
    }

    /// The `match_mixed` method attempts to match the string `input` against a variant of
    /// `StreetNamePreType`.
    pub fn match_mixed(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "of the" => Some(Self::OfThe),
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
