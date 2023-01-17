use serde::de::{Deserialize, Deserializer};

pub fn zero_floor<'de, D: Deserializer<'de>>(de: D) -> Result<Option<i64>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;

    match intermediate {
        0 => Ok(None),
        _ => Ok(Some(intermediate)),
    }
}
