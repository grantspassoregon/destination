use serde::{Deserialize, Serialize};
use serde::de::{Deserializer};

#[derive(Debug, Deserialize, Serialize)]
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

pub fn deserialize_abbreviated_pre_directional<'de, D: Deserializer<'de>>(de: D) -> Result<Option<StreetNamePreDirectional>, D::Error> {
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
