use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

/// The `StreetNamePreDirectional` enum represents the street name predirectional component of the
/// complete street name.  Predirectionals in the City consist of NW, NE, SW and SE, but County
/// roads annexed by the City can contain N, E, S and W.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Default)]
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

/// Matches the target data against the official postal abbreviation for street name
/// prediretionals.
pub fn match_abbreviated_pre_directional(input: &str) -> Option<StreetNamePreDirectional> {
    match input {
        "NE" => Some(StreetNamePreDirectional::NORTHEAST),
        "NW" => Some(StreetNamePreDirectional::NORTHWEST),
        "SE" => Some(StreetNamePreDirectional::SOUTHEAST),
        "SW" => Some(StreetNamePreDirectional::SOUTHWEST),
        "N" => Some(StreetNamePreDirectional::NORTH),
        "S" => Some(StreetNamePreDirectional::SOUTH),
        "E" => Some(StreetNamePreDirectional::EAST),
        "W" => Some(StreetNamePreDirectional::WEST),
        _ => None,
    }
}

/// Deserialization function for street name predirectionals.  This works if all the predirectionals in the
/// data observe the official postal contraction.  For predirectionals with a mix of abbreviations and
/// alternative spellings, [`deserialize_mixed_pre_directional()`] will work better.
pub fn deserialize_abbreviated_pre_directional<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<StreetNamePreDirectional>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    let result = match_abbreviated_pre_directional(intermediate);
    Ok(result)
}

/// Maps the string representation of a street pre-directional designation to the appropriate
/// [`StreetNamePreDirectional`] enum variant.
pub fn match_mixed_pre_directional(input: &str) -> Option<StreetNamePreDirectional> {
    match input {
        "NE" => Some(StreetNamePreDirectional::NORTHEAST),
        "NORTHEAST" => Some(StreetNamePreDirectional::NORTHEAST),
        "NW" => Some(StreetNamePreDirectional::NORTHWEST),
        "NORTHWEST" => Some(StreetNamePreDirectional::NORTHWEST),
        "SE" => Some(StreetNamePreDirectional::SOUTHEAST),
        "SOUTHEAST" => Some(StreetNamePreDirectional::SOUTHEAST),
        "SW" => Some(StreetNamePreDirectional::SOUTHWEST),
        "SOUTHWEST" => Some(StreetNamePreDirectional::SOUTHWEST),
        "N" => Some(StreetNamePreDirectional::NORTH),
        "NORTH" => Some(StreetNamePreDirectional::NORTH),
        "S" => Some(StreetNamePreDirectional::SOUTH),
        "SOUTH" => Some(StreetNamePreDirectional::SOUTH),
        "E" => Some(StreetNamePreDirectional::EAST),
        "EAST" => Some(StreetNamePreDirectional::EAST),
        "W" => Some(StreetNamePreDirectional::WEST),
        "WEST" => Some(StreetNamePreDirectional::WEST),
        _ => None,
    }
}

/// Deserialization function for street name predirectionals.
/// Matches the target data against novel spellings of valid predirectionals.  Add any missing spelling
/// variants to the match statement.
pub fn deserialize_mixed_pre_directional<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<StreetNamePreDirectional>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    let result = match_mixed_pre_directional(intermediate);
    Ok(result)
}

impl std::fmt::Display for StreetNamePreDirectional {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = "".to_owned();
        match self {
            StreetNamePreDirectional::NORTH => out.push_str("N"),
            StreetNamePreDirectional::SOUTH => out.push_str("S"),
            StreetNamePreDirectional::EAST => out.push_str("E"),
            StreetNamePreDirectional::WEST => out.push_str("W"),
            StreetNamePreDirectional::NORTHEAST => out.push_str("NE"),
            StreetNamePreDirectional::NORTHWEST => out.push_str("NW"),
            StreetNamePreDirectional::SOUTHEAST => out.push_str("SE"),
            StreetNamePreDirectional::SOUTHWEST => out.push_str("SW"),
        }
        write!(f, "{}", out)
    }
}
