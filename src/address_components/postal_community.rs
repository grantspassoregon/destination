use convert_case::Casing;
use serde::de::{Deserialize, Deserializer};

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
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
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
    /// The `upper` method converts the variant name to `UPPERCASE` case using
    /// [`convert_case::Case::Upper`].
    #[tracing::instrument]
    pub fn upper(&self) -> String {
        self.to_string().to_case(convert_case::Case::Upper)
    }

    /// The `label` method returns the name of the community in all caps with spaces, for printing
    /// labels.
    #[tracing::instrument]
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
    #[tracing::instrument]
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
    #[tracing::instrument(skip_all)]
    pub fn deserialize_mixed<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Self>, D::Error> {
        let intermediate = Deserialize::deserialize(de)?;
        Ok(Self::match_mixed(intermediate))
    }
}

#[test]
/// Establishes equality between the output of the two methods, before replacing the old with the
/// new.
#[tracing::instrument]
fn community_labels() -> Result<(), String> {
    use strum::IntoEnumIterator;
    for comm in PostalCommunity::iter() {
        let label = comm.label();
        let upper = comm.upper();
        assert_eq!(label, upper);
    }

    Ok(())
}
