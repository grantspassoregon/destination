use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
    Unit,
    Upper,
    Rec,
    Laundry,
}

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
