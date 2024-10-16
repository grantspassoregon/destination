use derive_more::Display;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

/// The `SubaddressType` enum represents the subaddress type of an address.  Valid type
/// designations include the list of secondary unit designators in Appendix C2 of the United States
/// Postal Service (USPS) Publication 28 - Postal Addressing Standards.
#[allow(missing_docs)]
#[derive(
    Copy,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Hash,
    Display,
)]
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

impl SubaddressType {
    /// The `abbreviate` method returns a String with the postal abbreviation of the subaddress
    /// type.
    pub fn abbreviate(&self) -> String {
        let str = match self {
            SubaddressType::Apartment => "apt",
            SubaddressType::Basement => "bsmt",
            SubaddressType::Building => "bldg",
            SubaddressType::Department => "dept",
            SubaddressType::Floor => "fl",
            SubaddressType::Front => "frnt",
            SubaddressType::Hanger => "hngr",
            SubaddressType::Key => "key",
            SubaddressType::Lobby => "lbby",
            SubaddressType::Lot => "lot",
            SubaddressType::Lower => "lowr",
            SubaddressType::Office => "ofc",
            SubaddressType::Penthouse => "ph",
            SubaddressType::Pier => "pier",
            SubaddressType::Rear => "rear",
            SubaddressType::Room => "rm",
            SubaddressType::Side => "side",
            SubaddressType::Slip => "slip",
            SubaddressType::Space => "spc",
            SubaddressType::Stop => "stop",
            SubaddressType::Suite => "ste",
            SubaddressType::Trailer => "trlr",
            SubaddressType::Unit => "unit",
            SubaddressType::Upper => "uppr",
            SubaddressType::Rec => "rec",
            SubaddressType::Laundry => "laun",
        };
        str.to_uppercase()
    }

    /// Matches subaddress types in the
    /// data that observe the official postal contraction.  For subaddress types with a mix of abbreviations and
    /// alternative spellings, the `match_mixed` method will work better.
    pub fn match_abbreviated(input: &str) -> Option<Self> {
        match input.to_uppercase().as_ref() {
            "APT" => Some(SubaddressType::Apartment),
            "BSMT" => Some(SubaddressType::Basement),
            "BLDG" => Some(SubaddressType::Building),
            "DEPT" => Some(SubaddressType::Department),
            "FL" => Some(SubaddressType::Floor),
            "FRNT" => Some(SubaddressType::Front),
            "HNGR" => Some(SubaddressType::Hanger),
            "KEY" => Some(SubaddressType::Key),
            "LBBY" => Some(SubaddressType::Lobby),
            "LOT" => Some(SubaddressType::Lot),
            "LOWR" => Some(SubaddressType::Lower),
            "OFC" => Some(SubaddressType::Office),
            "PH" => Some(SubaddressType::Penthouse),
            "PIER" => Some(SubaddressType::Pier),
            "REAR" => Some(SubaddressType::Rear),
            "RM" => Some(SubaddressType::Room),
            "SIDE" => Some(SubaddressType::Side),
            "SLIP" => Some(SubaddressType::Slip),
            "SPC" => Some(SubaddressType::Space),
            "STOP" => Some(SubaddressType::Stop),
            "STE" => Some(SubaddressType::Suite),
            "TRLR" => Some(SubaddressType::Trailer),
            "UNIT" => Some(SubaddressType::Unit),
            "UPPR" => Some(SubaddressType::Upper),
            "REC" => Some(SubaddressType::Rec),
            "LAUN" => Some(SubaddressType::Laundry),
            _ => None,
        }
    }

    /// Deserialization function for subaddress types.  This works if all the subaddress types in the
    /// data observe the official postal contraction.  For subaddress types with a mix of abbreviations and
    /// alternative spellings, [`Self::match_mixed`] will work better.
    pub fn deserialize_abbreviated<'de, D: Deserializer<'de>>(
        de: D,
    ) -> Result<Option<Self>, D::Error> {
        let intermediate = Deserialize::deserialize(de)?;
        Ok(Self::match_abbreviated(intermediate))
    }

    /// Matches the target data against novel spellings of valid subaddress types.  Add any missing spelling
    /// variants to the match statement.  Called by [`crate::Parser::subaddress_type`].
    /// Add additional variants to accommodate alternative abbreviations as needed.
    pub fn match_mixed(input: &str) -> Option<Self> {
        match input.to_uppercase().as_str() {
            "APT" => Some(Self::Apartment),
            "APARTMENT" => Some(Self::Apartment),
            "BSMT" => Some(Self::Basement),
            "BASEMENT" => Some(Self::Basement),
            "BLDG" => Some(Self::Building),
            "BUILDING" => Some(Self::Building),
            "DEPT" => Some(Self::Department),
            "DEPARTMENT" => Some(Self::Department),
            "FL" => Some(Self::Floor),
            "FLOOR" => Some(Self::Floor),
            "FRNT" => Some(Self::Front),
            "FRONT" => Some(Self::Front),
            "HNGR" => Some(Self::Hanger),
            "HANGER" => Some(Self::Hanger),
            "KEY" => Some(Self::Key),
            "LBBY" => Some(Self::Lobby),
            "LOBBY" => Some(Self::Lobby),
            "LOT" => Some(Self::Lot),
            "LOWR" => Some(Self::Lower),
            "LOWER" => Some(Self::Lower),
            "OFC" => Some(Self::Office),
            "OFFICE" => Some(Self::Office),
            "PH" => Some(Self::Penthouse),
            "PENTHOUSE" => Some(Self::Penthouse),
            "PIER" => Some(Self::Pier),
            "REAR" => Some(Self::Rear),
            "RM" => Some(Self::Room),
            "ROOM" => Some(Self::Room),
            "SIDE" => Some(Self::Side),
            "SLIP" => Some(Self::Slip),
            "SPC" => Some(Self::Space),
            "SPACE" => Some(Self::Space),
            "STOP" => Some(Self::Stop),
            "STE" => Some(Self::Suite),
            "SUITE" => Some(Self::Suite),
            "TRLR" => Some(Self::Trailer),
            "TRAILER" => Some(Self::Trailer),
            "UNIT" => Some(Self::Unit),
            "UPPR" => Some(Self::Upper),
            "UPPER" => Some(Self::Upper),
            "REC" => Some(Self::Rec),
            "LAUN" => Some(Self::Laundry),
            "LAUNDRY" => Some(Self::Laundry),
            _ => None,
        }
    }

    /// The `deserialize_mixed_subaddress_type` function attempts to deserialize the input data into a
    /// `SubaddressType`.
    pub fn deserialize_mixed<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Self>, D::Error> {
        let intermediate = Deserialize::deserialize(de)?;
        Ok(Self::match_mixed(intermediate))
    }
}
