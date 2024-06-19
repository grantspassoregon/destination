#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://www.grantspassoregon.gov/DocumentCenter/View/31368/GPLogo_450W-PNG"
)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! The `address` library provides types and methods for managing physical addresses in a
//! municipality.
pub mod address;
pub mod address_components;
pub mod business;
pub mod compare;
pub mod geo;
pub mod import;
pub mod lexisnexis;
pub mod parser;
pub mod utils;

/// The `prelude` module exposes the user-facing data structures and functions that make up the
/// library.
pub mod prelude {
    pub use crate::address::{
        Address, AddressDelta, AddressDeltas, Addresses, CommonAddress, CommonAddresses,
        PartialAddress, PartialAddresses,
    };
    pub use crate::address_components::{
        AddressStatus, PostalCommunity, State, StreetNamePostType, StreetNamePreDirectional,
        StreetNamePreModifier, StreetNamePreType, StreetSeparator, SubaddressType,
    };
    pub use crate::business::{BusinessLicenses, BusinessMatchRecord, BusinessMatchRecords};
    pub use crate::compare::{
        AddressMatch, FireInspectionMatchRecords, FireInspectionMatches, MatchPartialRecord,
        MatchPartialRecords, MatchRecord, MatchRecords, MatchStatus, Mismatch,
    };
    pub use crate::geo::{GeoAddress, GeoAddresses, Point, SpatialAddress, SpatialAddresses};
    pub use crate::import::{
        Business, Businesses, FireInspection, FireInspections, GrantsPassAddress,
        GrantsPassAddresses, GrantsPassSpatialAddress, GrantsPassSpatialAddresses,
        JosephineCountyAddress, JosephineCountyAddress2024, JosephineCountyAddresses,
        JosephineCountyAddresses2024, JosephineCountySpatialAddress,
        JosephineCountySpatialAddress2024, JosephineCountySpatialAddresses,
        JosephineCountySpatialAddresses2024,
    };
    pub use crate::lexisnexis::{LexisNexis, LexisNexisItem};
    pub use crate::parser::{
        deserialize_phone_number, multi_word, parse_address, parse_address_number,
        parse_address_number_suffix, parse_complete_street_name, parse_phone_number,
        parse_post_type, parse_pre_directional, parse_subaddress_element,
        parse_subaddress_elements, parse_subaddress_identifiers, parse_subaddress_type,
        recursive_post_type, Parser,
    };
    pub use crate::utils::{from_csv, load_bin, save, to_csv, Portable};
}
