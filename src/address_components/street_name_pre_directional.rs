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

pub fn deserialize_abbreviated_pre_directional<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<StreetNamePreDirectional>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;

    match intermediate {
        "NE" => Ok(Some(StreetNamePreDirectional::NORTHEAST)),
        "NW" => Ok(Some(StreetNamePreDirectional::NORTHWEST)),
        "SE" => Ok(Some(StreetNamePreDirectional::SOUTHEAST)),
        "SW" => Ok(Some(StreetNamePreDirectional::SOUTHWEST)),
        "N" => Ok(Some(StreetNamePreDirectional::NORTH)),
        "S" => Ok(Some(StreetNamePreDirectional::SOUTH)),
        "E" => Ok(Some(StreetNamePreDirectional::EAST)),
        "W" => Ok(Some(StreetNamePreDirectional::WEST)),
        _ => Ok(None),
    }
}

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
