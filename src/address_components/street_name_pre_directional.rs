use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum StreetNamePreDirectional {
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST,
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

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

pub fn deserialize_abbreviated_pre_directional<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<StreetNamePreDirectional>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    let result = match_abbreviated_pre_directional(intermediate);
    Ok(result)
}

//     match intermediate {
//         "NE" => Ok(Some(StreetNamePreDirectional::NORTHEAST)),
//         "NW" => Ok(Some(StreetNamePreDirectional::NORTHWEST)),
//         "SE" => Ok(Some(StreetNamePreDirectional::SOUTHEAST)),
//         "SW" => Ok(Some(StreetNamePreDirectional::SOUTHWEST)),
//         "N" => Ok(Some(StreetNamePreDirectional::NORTH)),
//         "S" => Ok(Some(StreetNamePreDirectional::SOUTH)),
//         "E" => Ok(Some(StreetNamePreDirectional::EAST)),
//         "W" => Ok(Some(StreetNamePreDirectional::WEST)),
//         _ => Ok(None),
//     }
// }

pub fn deserialize_mixed_pre_directional<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<StreetNamePreDirectional>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;

    match intermediate {
        "NE" => Ok(Some(StreetNamePreDirectional::NORTHEAST)),
        "NORTHEAST" => Ok(Some(StreetNamePreDirectional::NORTHEAST)),
        "NW" => Ok(Some(StreetNamePreDirectional::NORTHWEST)),
        "NORTHWEST" => Ok(Some(StreetNamePreDirectional::NORTHWEST)),
        "SE" => Ok(Some(StreetNamePreDirectional::SOUTHEAST)),
        "SOUTHEAST" => Ok(Some(StreetNamePreDirectional::SOUTHEAST)),
        "SW" => Ok(Some(StreetNamePreDirectional::SOUTHWEST)),
        "SOUTHWEST" => Ok(Some(StreetNamePreDirectional::SOUTHWEST)),
        "N" => Ok(Some(StreetNamePreDirectional::NORTH)),
        "NORTH" => Ok(Some(StreetNamePreDirectional::NORTH)),
        "S" => Ok(Some(StreetNamePreDirectional::SOUTH)),
        "SOUTH" => Ok(Some(StreetNamePreDirectional::SOUTH)),
        "E" => Ok(Some(StreetNamePreDirectional::EAST)),
        "EAST" => Ok(Some(StreetNamePreDirectional::EAST)),
        "W" => Ok(Some(StreetNamePreDirectional::WEST)),
        "WEST" => Ok(Some(StreetNamePreDirectional::WEST)),
        _ => Ok(None),
    }
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
