use derive_more::Display;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

/// The `PostalCommunity` enum holds valid variants for the postal community field of an address.
/// The list of valid postal communities is limited to the set of communities encountered locally,
/// and we add new variants as needed.
///
/// This enum exists to facilitate parsing addresses.  Because street names and types can parse
/// ambiguously, it can be unclear during a parse whether a word should be parsed as a street name,
/// street type or postal community.  By ensuring the postal community maps to a valid value, we
/// can reduce the risk of parsing a word to the wrong address element.
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
pub enum PostalCommunity {
    /// The City of Grants Pass, an incorporated municipality.
    #[default]
    GrantsPass,
    /// The City of Medford, an incorporated municipality.
    Medford,
    /// The City of Merlin, an unincorporated community.
    Merlin,
}

impl PostalCommunity {
    /// The `label` method returns the name of the community in all caps with spaces, for printing
    /// labels.
    pub fn label(&self) -> String {
        let label = match self {
            Self::GrantsPass => "GRANTS PASS",
            Self::Medford => "MEDFORD",
            Self::Merlin => "MERLIN",
        };
        label.to_string()
    }

    /// The `match_mixed` method attempts to match the string `input` against a variant of
    /// `PostalCommunity`.
    pub fn match_mixed(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "grants pass" => Some(Self::GrantsPass),
            "gp" => Some(Self::GrantsPass),
            "medford" => Some(Self::Medford),
            "merlin" => Some(Self::Merlin),
            _ => None,
        }
    }

    /// The `deserialize_mixed` method attempts to match the input to a valid postal community
    /// variant.
    pub fn deserialize_mixed<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Self>, D::Error> {
        let intermediate = Deserialize::deserialize(de)?;
        Ok(Self::match_mixed(intermediate))
    }
}
