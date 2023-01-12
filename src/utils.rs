use serde::de::{Deserialize, Deserializer};

pub fn deserialize_arcgis_data<'de, D: Deserializer<'de>>(de: D) -> Result<Option<String>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;

    match intermediate {
        None => Ok(None),
        Some("<Null>") => Ok(None),
        Some(other_value) => Ok(Some(other_value.to_string())),
    }
}
