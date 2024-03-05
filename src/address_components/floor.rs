use serde::de::{Deserialize, Deserializer};

/// Deserialization function for the `floor` field of County addresses.  The County records single
/// floor buildings as floor zero, whereas the City records floor numbers for multistory buildings
/// and leaves the floor field empty for single story structures.
pub fn zero_floor<'de, D: Deserializer<'de>>(de: D) -> Result<Option<i64>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;

    match intermediate {
        0 => Ok(None),
        _ => Ok(Some(intermediate)),
    }
}
