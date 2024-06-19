use derive_more::Display;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

/// The `StreetNamePreModifier` is the pre-modifier element of a complete street name.
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
pub enum StreetNamePreModifier {
    #[default]
    Old,
    Upper,
    Lower,
    Right,
    Left,
    Northbound,
}

impl StreetNamePreModifier {
    /// The `label` method returns the street name pre-modifier in all caps, for printing
    /// labels.
    pub fn label(&self) -> String {
        let label = match self {
            Self::Old => "OLD",
            Self::Upper => "UPPER",
            Self::Lower => "LOWER",
            Self::Right => "RIGHT",
            Self::Left => "LEFT",
            Self::Northbound => "NORTHBOUND",
        };
        label.to_string()
    }

    /// The `match_mixed` method attempts to match the string `input` against a variant of
    /// `StreetNamePreModifier`.
    pub fn match_mixed(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "old" => Some(Self::Old),
            "upper" => Some(Self::Upper),
            "lower" => Some(Self::Lower),
            "right" => Some(Self::Right),
            "left" => Some(Self::Left),
            "northbound" => Some(Self::Northbound),
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
