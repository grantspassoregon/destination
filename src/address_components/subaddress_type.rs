use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The `SubaddressType` enum represents the subaddress type of an address.  Valid type
/// designations include the list of secondary unit designators in Appendix C2 of the United States
/// Postal Service (USPS) Publication 28 - Postal Addressing Standards.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SubaddressType {
    Apartment,
    Basement,
    Building,
    Department,
    Floor,
    Front,
    Hanger,
    Key,
    Lobby,
    Lot,
    Lower,
    Office,
    Penthouse,
    Pier,
    Rear,
    Room,
    Side,
    Slip,
    Space,
    Stop,
    Suite,
    Trailer,
    #[default]
    Unit,
    Upper,
    /// Recreation room.  A shared space common to apartment complexes.
    Rec,
    /// Laundry room.  A shared space common to apartment complexes.
    Laundry,
}

impl fmt::Display for SubaddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Apartment => write!(f, "Apartment"),
            Self::Basement => write!(f, "Basement"),
            Self::Building => write!(f, "Building"),
            Self::Department => write!(f, "Department"),
            Self::Floor => write!(f, "Floor"),
            Self::Front => write!(f, "Front"),
            Self::Hanger => write!(f, "Hanger"),
            Self::Key => write!(f, "Key"),
            Self::Lobby => write!(f, "Lobby"),
            Self::Lot => write!(f, "Lot"),
            Self::Lower => write!(f, "Lower"),
            Self::Office => write!(f, "Office"),
            Self::Penthouse => write!(f, "Penthouse"),
            Self::Pier => write!(f, "Pier"),
            Self::Rear => write!(f, "Rear"),
            Self::Room => write!(f, "Room"),
            Self::Side => write!(f, "Side"),
            Self::Slip => write!(f, "Slip"),
            Self::Space => write!(f, "Space"),
            Self::Stop => write!(f, "Stop"),
            Self::Suite => write!(f, "Suite"),
            Self::Trailer => write!(f, "Trailer"),
            Self::Unit => write!(f, "Unit"),
            Self::Upper => write!(f, "Upper"),
            Self::Rec => write!(f, "Rec"),
            Self::Laundry => write!(f, "Laundry"),
        }
    }
}

/// Deserialization function for subaddress types.  This works if all the subaddress types in the
/// data observe the official postal contraction.  For subaddress types with a mix of abbreviations and
/// alternative spellings, [`match_mixed_subaddress_type()`] will work better.
pub fn deserialize_abbreviated_subaddress_type<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<SubaddressType>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;

    match intermediate {
        "APT" => Ok(Some(SubaddressType::Apartment)),
        "BSMT" => Ok(Some(SubaddressType::Basement)),
        "BLDG" => Ok(Some(SubaddressType::Building)),
        "DEPT" => Ok(Some(SubaddressType::Department)),
        "FL" => Ok(Some(SubaddressType::Floor)),
        "FRNT" => Ok(Some(SubaddressType::Front)),
        "HNGR" => Ok(Some(SubaddressType::Hanger)),
        "KEY" => Ok(Some(SubaddressType::Key)),
        "LBBY" => Ok(Some(SubaddressType::Lobby)),
        "LOT" => Ok(Some(SubaddressType::Lot)),
        "LOWR" => Ok(Some(SubaddressType::Lower)),
        "OFC" => Ok(Some(SubaddressType::Office)),
        "PH" => Ok(Some(SubaddressType::Penthouse)),
        "PIER" => Ok(Some(SubaddressType::Pier)),
        "REAR" => Ok(Some(SubaddressType::Rear)),
        "RM" => Ok(Some(SubaddressType::Room)),
        "SIDE" => Ok(Some(SubaddressType::Side)),
        "SLIP" => Ok(Some(SubaddressType::Slip)),
        "SPC" => Ok(Some(SubaddressType::Space)),
        "STOP" => Ok(Some(SubaddressType::Stop)),
        "STE" => Ok(Some(SubaddressType::Suite)),
        "TRLR" => Ok(Some(SubaddressType::Trailer)),
        "UNIT" => Ok(Some(SubaddressType::Unit)),
        "UPPR" => Ok(Some(SubaddressType::Upper)),
        "REC" => Ok(Some(SubaddressType::Rec)),
        "LAUN" => Ok(Some(SubaddressType::Laundry)),
        _ => Ok(None),
    }
}

pub fn deserialize_mixed_subaddress_type<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<SubaddressType>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    let result = match_mixed_subaddress_type(intermediate);
    Ok(result)
}

/// Matches the target data against novel spellings of valid subaddress types.  Add any missing spelling
/// variants to the match statement.  Called by [`crate::parser::parse_subaddress_type()`].
pub fn match_mixed_subaddress_type(input: &str) -> Option<SubaddressType> {
    match input {
        "APT" => Some(SubaddressType::Apartment),
        "APARTMENT" => Some(SubaddressType::Apartment),
        "BSMT" => Some(SubaddressType::Basement),
        "BASEMENT" => Some(SubaddressType::Basement),
        "BLDG" => Some(SubaddressType::Building),
        "BUILDING" => Some(SubaddressType::Building),
        "DEPT" => Some(SubaddressType::Department),
        "DEPARTMENT" => Some(SubaddressType::Department),
        "FL" => Some(SubaddressType::Floor),
        "FLOOR" => Some(SubaddressType::Floor),
        "FRNT" => Some(SubaddressType::Front),
        "FRONT" => Some(SubaddressType::Front),
        "HNGR" => Some(SubaddressType::Hanger),
        "HANGER" => Some(SubaddressType::Hanger),
        "KEY" => Some(SubaddressType::Key),
        "LBBY" => Some(SubaddressType::Lobby),
        "LOBBY" => Some(SubaddressType::Lobby),
        "LOT" => Some(SubaddressType::Lot),
        "LOWR" => Some(SubaddressType::Lower),
        "LOWER" => Some(SubaddressType::Lower),
        "OFC" => Some(SubaddressType::Office),
        "OFFICE" => Some(SubaddressType::Office),
        "PH" => Some(SubaddressType::Penthouse),
        "PENTHOUSE" => Some(SubaddressType::Penthouse),
        "PIER" => Some(SubaddressType::Pier),
        "REAR" => Some(SubaddressType::Rear),
        "RM" => Some(SubaddressType::Room),
        "ROOM" => Some(SubaddressType::Room),
        "SIDE" => Some(SubaddressType::Side),
        "SLIP" => Some(SubaddressType::Slip),
        "SPC" => Some(SubaddressType::Space),
        "SPACE" => Some(SubaddressType::Space),
        "STOP" => Some(SubaddressType::Stop),
        "STE" => Some(SubaddressType::Suite),
        "SUITE" => Some(SubaddressType::Suite),
        "TRLR" => Some(SubaddressType::Trailer),
        "TRAILER" => Some(SubaddressType::Trailer),
        "UNIT" => Some(SubaddressType::Unit),
        "UPPR" => Some(SubaddressType::Upper),
        "UPPER" => Some(SubaddressType::Upper),
        "REC" => Some(SubaddressType::Rec),
        "LAUN" => Some(SubaddressType::Laundry),
        "LAUNDRY" => Some(SubaddressType::Laundry),
        _ => None,
    }
}
