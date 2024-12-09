use convert_case::Casing;
use serde::de::Deserializer;

/// The `StreetNamePreModifier` is the pre-modifier element of a complete street name.
#[allow(missing_docs)]
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
    /// The `upper` method converts the variant name to `UPPERCASE` case using
    /// [`convert_case::Case::Upper`].
    #[tracing::instrument]
    pub fn upper(&self) -> String {
        self.to_string().to_case(convert_case::Case::Upper)
    }

    /// The `label` method returns the street name pre-modifier in all caps, for printing
    /// labels.
    #[tracing::instrument]
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
    #[tracing::instrument]
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
    #[tracing::instrument(skip_all)]
    pub fn deserialize_mixed<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Self>, D::Error> {
        let intermediate = serde::Deserialize::deserialize(de)?;
        Ok(Self::match_mixed(intermediate))
    }
}

#[test]
/// Establishes equality between the output of the two methods, before replacing the old with the
/// new.
fn premodifier_labels() -> Result<(), String> {
    use strum::IntoEnumIterator;
    for item in StreetNamePreModifier::iter() {
        let label = item.label();
        let upper = item.upper();
        assert_eq!(label, upper);
    }

    Ok(())
}
