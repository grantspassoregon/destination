#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://www.grantspassoregon.gov/DocumentCenter/View/31368/GPLogo_450W-PNG"
)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![doc = include_str!("../README.md")]
mod address;
mod address_components;
mod business;
mod cli;
mod compare;
mod error;
mod geo;
mod import;
mod lexisnexis;
mod parser;
mod utils;

pub use address::{
    Address, AddressDelta, AddressDeltas, Addresses, CommonAddress, CommonAddresses,
    PartialAddress, PartialAddresses,
};
pub use address_components::{
    AddressStatus, PostalCommunity, State, StreetNamePostType, StreetNamePreDirectional,
    StreetNamePreModifier, StreetNamePreType, StreetSeparator, SubaddressType, zero_floor,
};
pub use business::{BusinessLicense, BusinessLicenses, BusinessMatchRecord, BusinessMatchRecords};
pub use cli::Cli;
pub use compare::{
    AddressMatch, FireInspectionMatch, FireInspectionMatchRecord, FireInspectionMatchRecords,
    FireInspectionMatches, MatchPartialRecord, MatchPartialRecords, MatchRecord, MatchRecords,
    MatchStatus, Mismatch,
};
pub use error::{AddressError, AddressErrorKind, Builder, Csv, Decode, Encode, Io, Nom};
pub use geo::{
    AddressPoints, Cartesian, GeoAddress, GeoAddresses, Geographic, SpatialAddress,
    SpatialAddresses,
};
pub use import::{
    Business, Businesses, FireInspection, FireInspectionRaw, FireInspections, GrantsPassAddress,
    GrantsPassAddresses, GrantsPassSpatialAddress, GrantsPassSpatialAddresses,
    JosephineCountyAddress, JosephineCountyAddress2024, JosephineCountyAddresses,
    JosephineCountyAddresses2024, JosephineCountySpatialAddress, JosephineCountySpatialAddress2024,
    JosephineCountySpatialAddresses, JosephineCountySpatialAddresses2024, SpatialAddressesRaw,
};
pub use lexisnexis::{
    LexisNexis, LexisNexisItem, LexisNexisItemBuilder, LexisNexisRange, LexisNexisRangeItem,
};
pub use parser::{Parse, deserialize_phone_number, parse_phone_number};
pub use utils::{
    IntoBin, IntoCsv, deserialize_arcgis_data, from_bin, from_csv, to_bin, to_csv, trace_init,
};
