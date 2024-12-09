use serde::de::Deserializer;

/// The `StreetNamePreDirectional` enum represents the street name predirectional component of the
/// complete street name.  Predirectionals in the City consist of NW, NE, SW and SE, but County
/// roads annexed by the City can contain N, E, S and W.
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
pub enum StreetNamePreDirectional {
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST,
    #[default]
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl StreetNamePreDirectional {
    /// The `abbreviate` method converts the variant to an abbreviated string for labeling.
    #[tracing::instrument]
    pub fn abbreviate(&self) -> String {
        let abbr = match self {
            StreetNamePreDirectional::NORTH => "N",
            StreetNamePreDirectional::SOUTH => "S",
            StreetNamePreDirectional::EAST => "E",
            StreetNamePreDirectional::WEST => "W",
            StreetNamePreDirectional::NORTHEAST => "NE",
            StreetNamePreDirectional::NORTHWEST => "NW",
            StreetNamePreDirectional::SOUTHEAST => "SE",
            StreetNamePreDirectional::SOUTHWEST => "SW",
        };
        abbr.to_string()
    }

    /// Matches the target data against the official postal abbreviation for street name
    /// prediretionals.
    #[tracing::instrument]
    pub fn match_abbreviated(input: &str) -> Option<Self> {
        match input.to_uppercase().as_str() {
            "NE" => Some(Self::NORTHEAST),
            "NW" => Some(Self::NORTHWEST),
            "SE" => Some(Self::SOUTHEAST),
            "SW" => Some(Self::SOUTHWEST),
            "N" => Some(Self::NORTH),
            "S" => Some(Self::SOUTH),
            "E" => Some(Self::EAST),
            "W" => Some(Self::WEST),
            _ => None,
        }
    }

    /// Deserialization function for street name predirectionals.  This works if all the predirectionals in the
    /// data observe the official postal contraction.  For predirectionals with a mix of abbreviations and
    /// alternative spellings, [`Self::deserialize_mixed`] will work better.
    #[tracing::instrument(skip_all)]
    pub fn deserialize_abbreviated<'de, D: Deserializer<'de>>(
        de: D,
    ) -> Result<Option<Self>, D::Error> {
        let intermediate = serde::Deserialize::deserialize(de)?;
        Ok(Self::match_abbreviated(intermediate))
    }

    /// Maps the string representation of a street pre-directional designation to the appropriate
    /// [`StreetNamePreDirectional`] enum variant.
    #[tracing::instrument]
    pub fn match_mixed(input: &str) -> Option<Self> {
        if let Some(dir) = Self::match_abbreviated(input) {
            Some(dir)
        } else {
            match input.to_uppercase().as_str() {
                "N.E." => Some(Self::NORTHEAST),
                "NE." => Some(Self::NORTHEAST),
                "NORTHEAST" => Some(Self::NORTHEAST),
                "N.W." => Some(Self::NORTHWEST),
                "NW." => Some(Self::NORTHWEST),
                "NORTHWEST" => Some(Self::NORTHWEST),
                "S.E." => Some(Self::SOUTHEAST),
                "SE." => Some(Self::SOUTHEAST),
                "SOUTHEAST" => Some(Self::SOUTHEAST),
                "S.W." => Some(Self::SOUTHWEST),
                "SW." => Some(Self::SOUTHWEST),
                "SOUTHWEST" => Some(Self::SOUTHWEST),
                "N." => Some(Self::NORTH),
                "NORTH" => Some(Self::NORTH),
                "S." => Some(Self::SOUTH),
                "SOUTH" => Some(Self::SOUTH),
                "EAST" => Some(Self::EAST),
                "W." => Some(Self::WEST),
                "WEST" => Some(Self::WEST),
                _ => None,
            }
        }
    }

    /// Deserialization function for street name predirectionals.
    /// Matches the target data against novel spellings of valid predirectionals.  Add any missing spelling
    /// variants to the match statement.
    #[tracing::instrument(skip_all)]
    pub fn deserialize_mixed<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Self>, D::Error> {
        let intermediate = serde::Deserialize::deserialize(de)?;
        Ok(Self::match_mixed(intermediate))
    }
}
