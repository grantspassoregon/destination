//! The `josephine_county` module contains data types for importing addresses from ECSO and
//! Josephine County.
use crate::{
    deserialize_arcgis_data, zero_floor, Address, AddressError, AddressErrorKind, AddressStatus,
    Addresses, IntoBin, IntoCsv, Io, Point, State, StreetNamePostType, StreetNamePreDirectional,
    StreetNamePreModifier, StreetNamePreType, StreetSeparator, SubaddressType, _from_csv,
    _load_bin, _save, _to_csv,
};
use derive_more::{Deref, DerefMut};
use galileo::galileo_types::geo::GeoPoint;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The `JosephineCountyAddress2024` struct represents an address site point for Josephine County,
/// consistent with the schema adopted by the agency in April of 2024.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct JosephineCountyAddress2024 {
    /// The `address_number` field represents the address number component of the complete address
    /// number.
    #[serde(rename = "add_number")]
    pub address_number: i64,
    /// The `address_number_suffix` field represents the address number suffix component of the complete
    /// address number.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "addnum_suf")]
    pub address_number_suffix: Option<String>,
    /// The `street_name_pre_directional` field represents the street name pre directional component of the
    /// complete street name.
    #[serde(
        deserialize_with = "StreetNamePreDirectional::deserialize_mixed",
        rename = "st_predir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    /// The `street_name_pre_modifier` field represents the street name pre modifier component of the complete
    /// street name.
    #[serde(
        deserialize_with = "StreetNamePreModifier::deserialize_mixed",
        rename = "st_premod"
    )]
    pub street_name_pre_modifier: Option<StreetNamePreModifier>,
    /// The `street_name_pre_type` field represents the street name pre type component of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetNamePreType::deserialize_mixed",
        rename = "st_pretyp"
    )]
    pub street_name_pre_type: Option<StreetNamePreType>,
    /// The `street_name_separator` field represents the separator element component of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetSeparator::deserialize_mixed",
        rename = "st_presep"
    )]
    pub street_name_separator: Option<StreetSeparator>,
    /// The `street_name` field represents the street name component of the complete street name.
    #[serde(rename = "st_name")]
    pub street_name: String,
    /// The `street_name_post_type` field represents the street name post type component of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetNamePostType::deserialize_mixed",
        rename = "st_postyp"
    )]
    pub street_name_post_type: Option<StreetNamePostType>,
    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.
    #[serde(
        deserialize_with = "SubaddressType::deserialize_mixed",
        rename = "unittype"
    )]
    pub subaddress_type: Option<SubaddressType>,
    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "unit")]
    pub subaddress_identifier: Option<String>,
    /// The `floor` field represents the floor identifier, corresponding to the `Floor` field from the NENA standard.
    #[serde(deserialize_with = "zero_floor")]
    pub floor: Option<i64>,
    /// The `complete_street_address` field represents the full street address as a string.
    #[serde(rename = "st_fullad")]
    pub complete_street_address: String,
    /// The `postal_community` field represents the postal community component of the address,
    /// being either the unincorporated or incorporated municipality name.
    #[serde(rename = "uninc_comm")]
    pub postal_community: String,
    /// The `zip_code` field represents the postal zip code of the address.
    #[serde(rename = "post_code")]
    pub zip_code: i64,
    /// The `state_name` field represents the state name component of the address.
    #[serde(deserialize_with = "State::deserialize_mixed")]
    #[serde(rename = "state")]
    pub state_name: State,
    /// The `status` field represents the local status of the address as determined by the relevant
    /// addressing authority.
    pub status: AddressStatus,
}

impl Address for JosephineCountyAddress2024 {
    fn number(&self) -> i64 {
        self.address_number
    }

    fn number_mut(&mut self) -> &mut i64 {
        &mut self.address_number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.address_number_suffix
    }

    fn number_suffix_mut(&mut self) -> &mut Option<String> {
        &mut self.address_number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.street_name_pre_directional
    }

    fn directional_mut(&mut self) -> &mut Option<StreetNamePreDirectional> {
        &mut self.street_name_pre_directional
    }

    fn street_name_pre_modifier(&self) -> &Option<StreetNamePreModifier> {
        &self.street_name_pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<StreetNamePreModifier> {
        &mut self.street_name_pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<StreetNamePreType> {
        &self.street_name_pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<StreetNamePreType> {
        &mut self.street_name_pre_type
    }

    fn street_name_separator(&self) -> &Option<StreetSeparator> {
        &self.street_name_separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<StreetSeparator> {
        &mut self.street_name_separator
    }

    fn street_name(&self) -> &String {
        &self.street_name
    }

    fn street_name_mut(&mut self) -> &mut String {
        &mut self.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.street_name_post_type
    }

    fn street_type_mut(&mut self) -> &mut Option<StreetNamePostType> {
        &mut self.street_name_post_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.subaddress_identifier
    }

    fn subaddress_id_mut(&mut self) -> &mut Option<String> {
        &mut self.subaddress_identifier
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.subaddress_type
    }

    fn subaddress_type_mut(&mut self) -> &mut Option<SubaddressType> {
        &mut self.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.floor
    }

    fn floor_mut(&mut self) -> &mut Option<i64> {
        &mut self.floor
    }

    fn building(&self) -> &Option<String> {
        &None
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.address_number_suffix
    }

    fn zip(&self) -> i64 {
        self.zip_code
    }

    fn zip_mut(&mut self) -> &mut i64 {
        &mut self.zip_code
    }

    fn postal_community(&self) -> &String {
        &self.postal_community
    }

    fn postal_community_mut(&mut self) -> &mut String {
        &mut self.postal_community
    }

    fn state(&self) -> &State {
        &self.state_name
    }

    fn state_mut(&mut self) -> &mut State {
        &mut self.state_name
    }

    fn status(&self) -> &AddressStatus {
        &self.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.status
    }
}

/// The `JosephineCountyAddresses2024` struct holds a vector of type
/// ['JosephineCountyAddress2024'].
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Deref, DerefMut)]
pub struct JosephineCountyAddresses2024(Vec<JosephineCountyAddress2024>);

impl Addresses<JosephineCountyAddress2024> for JosephineCountyAddresses2024 {}

impl IntoBin<JosephineCountyAddresses2024> for JosephineCountyAddresses2024 {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, AddressError> {
        match _load_bin(path) {
            Ok(records) => {
                let decode: Self = bincode::deserialize(&records)?;
                Ok(decode)
            }
            Err(source) => Err(AddressErrorKind::from(source).into()),
        }
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), AddressError> {
        _save(self, path)
    }
}

impl IntoCsv<JosephineCountyAddresses2024> for JosephineCountyAddresses2024 {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Io> {
        let records = _from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        _to_csv(&mut self.0, path.as_ref().into())
    }
}

/// The `JosephineCountySpatialAddress2024` struct represents an address site point for Josephine County that includes geographic and projected coordinate information,
/// consistent with the schema adopted by the agency in April of 2024.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct JosephineCountySpatialAddress2024 {
    /// The `address_number` field represents the address number component of the complete address
    /// number.
    #[serde(rename = "add_number")]
    pub address_number: i64,
    /// The `address_number_suffix` field represents the address number suffix component of the complete
    /// address number.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "addnum_suf")]
    pub address_number_suffix: Option<String>,
    /// The `street_name_pre_directional` field represents the street name pre directional component of the
    /// complete street name.
    #[serde(
        deserialize_with = "StreetNamePreDirectional::deserialize_mixed",
        rename = "st_predir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    /// The `street_name_pre_modifier` field represents the street name pre modifier component of the complete
    /// street name.
    #[serde(
        deserialize_with = "StreetNamePreModifier::deserialize_mixed",
        rename = "st_premod"
    )]
    pub street_name_pre_modifier: Option<StreetNamePreModifier>,
    /// The `street_name_pre_type` field represents the street name pre type component of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetNamePreType::deserialize_mixed",
        rename = "st_pretyp"
    )]
    pub street_name_pre_type: Option<StreetNamePreType>,
    /// The `street_name_separator` field represents the separator element component of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetSeparator::deserialize_mixed",
        rename = "st_presep"
    )]
    pub street_name_separator: Option<StreetSeparator>,
    /// The `street_name` field represents the street name component of the complete street name.
    #[serde(rename = "st_name")]
    pub street_name: String,
    /// The `street_name_post_type` field represents the street name post type component of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetNamePostType::deserialize_mixed",
        rename = "st_postyp"
    )]
    pub street_name_post_type: Option<StreetNamePostType>,
    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.
    #[serde(
        deserialize_with = "SubaddressType::deserialize_mixed",
        rename = "unittype"
    )]
    pub subaddress_type: Option<SubaddressType>,
    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "unit")]
    pub subaddress_identifier: Option<String>,
    /// The `floor` field represents the floor identifier, corresponding to the `Floor` field from the NENA standard.
    #[serde(deserialize_with = "zero_floor")]
    pub floor: Option<i64>,
    /// The `complete_street_address` field represents the full street address as a string.
    #[serde(rename = "st_fullad")]
    pub complete_street_address: String,
    /// The `postal_community` field represents the postal community component of the address,
    /// being either the unincorporated or incorporated municipality name.
    #[serde(rename = "uninc_comm")]
    pub postal_community: String,
    /// The `zip_code` field represents the postal zip code of the address.
    #[serde(rename = "post_code")]
    pub zip_code: i64,
    /// The `state_name` field represents the state name component of the address.
    #[serde(deserialize_with = "State::deserialize_mixed")]
    #[serde(rename = "state")]
    pub state_name: State,
    /// The `status` field represents the local status of the address as determined by the relevant
    /// addressing authority.
    pub status: AddressStatus,
    /// The `x` field represents the cartesian X portion of the projected coordinates of the
    /// address.
    pub x: f64,
    /// The `y` field represents the cartesian Y portion of the projected coordinates of the
    /// address.
    pub y: f64,
    /// The `lat` field represents the latitude of the geographic coordinates for the address.
    #[serde(rename = "latitude")]
    pub lat: f64,
    /// The `lon` field represents the longitude of the geographic coordinates for the address.
    #[serde(rename = "longitude")]
    pub lon: f64,
}

impl Address for JosephineCountySpatialAddress2024 {
    fn number(&self) -> i64 {
        self.address_number
    }

    fn number_mut(&mut self) -> &mut i64 {
        &mut self.address_number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.address_number_suffix
    }

    fn number_suffix_mut(&mut self) -> &mut Option<String> {
        &mut self.address_number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.street_name_pre_directional
    }

    fn directional_mut(&mut self) -> &mut Option<StreetNamePreDirectional> {
        &mut self.street_name_pre_directional
    }

    fn street_name_pre_modifier(&self) -> &Option<StreetNamePreModifier> {
        &self.street_name_pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<StreetNamePreModifier> {
        &mut self.street_name_pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<StreetNamePreType> {
        &self.street_name_pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<StreetNamePreType> {
        &mut self.street_name_pre_type
    }

    fn street_name_separator(&self) -> &Option<StreetSeparator> {
        &self.street_name_separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<StreetSeparator> {
        &mut self.street_name_separator
    }

    fn street_name(&self) -> &String {
        &self.street_name
    }

    fn street_name_mut(&mut self) -> &mut String {
        &mut self.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.street_name_post_type
    }

    fn street_type_mut(&mut self) -> &mut Option<StreetNamePostType> {
        &mut self.street_name_post_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.subaddress_identifier
    }

    fn subaddress_id_mut(&mut self) -> &mut Option<String> {
        &mut self.subaddress_identifier
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.subaddress_type
    }

    fn subaddress_type_mut(&mut self) -> &mut Option<SubaddressType> {
        &mut self.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.floor
    }

    fn floor_mut(&mut self) -> &mut Option<i64> {
        &mut self.floor
    }

    fn building(&self) -> &Option<String> {
        &None
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.address_number_suffix
    }

    fn zip(&self) -> i64 {
        self.zip_code
    }

    fn zip_mut(&mut self) -> &mut i64 {
        &mut self.zip_code
    }

    fn postal_community(&self) -> &String {
        &self.postal_community
    }

    fn postal_community_mut(&mut self) -> &mut String {
        &mut self.postal_community
    }

    fn state(&self) -> &State {
        &self.state_name
    }

    fn state_mut(&mut self) -> &mut State {
        &mut self.state_name
    }

    fn status(&self) -> &AddressStatus {
        &self.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.status
    }
}

impl Point for JosephineCountySpatialAddress2024 {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl GeoPoint for JosephineCountySpatialAddress2024 {
    type Num = f64;

    fn lat(&self) -> Self::Num {
        self.lat
    }

    fn lon(&self) -> Self::Num {
        self.lon
    }
}

/// The `JosephineCountySpatialAddresses2024` struct holds a vector of type
/// ['JosephineCountySpatialAddress2024'].
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Deref, DerefMut)]
pub struct JosephineCountySpatialAddresses2024(Vec<JosephineCountySpatialAddress2024>);

impl Addresses<JosephineCountySpatialAddress2024> for JosephineCountySpatialAddresses2024 {}

impl IntoBin<JosephineCountySpatialAddresses2024> for JosephineCountySpatialAddresses2024 {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, AddressError> {
        match _load_bin(path) {
            Ok(records) => {
                let decode: Self = bincode::deserialize(&records)?;
                Ok(decode)
            }
            Err(source) => Err(AddressErrorKind::from(source).into()),
        }
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), AddressError> {
        _save(self, path)
    }
}

impl IntoCsv<JosephineCountySpatialAddresses2024> for JosephineCountySpatialAddresses2024 {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Io> {
        let records = _from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        _to_csv(&mut self.0, path.as_ref().into())
    }
}

/// The `JosephineCountyAddress` struct represents an address site point for Josephine County,
/// prior to the schema adopted by the agency in April of 2024.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct JosephineCountyAddress {
    /// The `taxlot` field represents the map tax lot number of the parcel on which the address
    /// is located.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub taxlot: Option<String>,
    /// The `address_number` field represents the address number component of the complete address
    /// number.
    #[serde(rename = "stnum")]
    pub address_number: i64,
    /// The `address_number_suffix` field represents the address number suffix component of the complete
    /// address number.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "stnumsuf")]
    pub address_number_suffix: Option<String>,
    /// The `street_name_pre_directional` field represents the street name pre directional component of the
    /// complete street name.
    #[serde(
        deserialize_with = "StreetNamePreDirectional::deserialize_abbreviated",
        rename = "predir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    /// The `street_name` field represents the street name component of the complete street name.
    #[serde(rename = "name")]
    pub street_name: String,
    /// The `street_name_post_type` field represents the street name post type component of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetNamePostType::deserialize_mixed",
        // deserialize_with = "deserialize_abbreviated_post_type",
        rename = "type"
    )]
    pub street_name_post_type: Option<StreetNamePostType>,
    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.
    #[serde(
        deserialize_with = "SubaddressType::deserialize_abbreviated",
        rename = "unit_type"
    )]
    pub subaddress_type: Option<SubaddressType>,
    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "unit")]
    pub subaddress_identifier: Option<String>,
    /// The `floor` field represents the floor identifier, corresponding to the `Floor` field from the NENA standard.
    #[serde(deserialize_with = "zero_floor")]
    pub floor: Option<i64>,
    /// The `complete_street_address` field represents the full street address as a string.
    #[serde(rename = "address")]
    pub complete_street_address: String,
    /// The `postal_community` field represents the postal community component of the address,
    /// being either the unincorporated or incorporated municipality name.
    #[serde(rename = "postcomm")]
    pub postal_community: String,
    /// The `zip_code` field represents the postal zip code of the address.
    #[serde(rename = "zip")]
    pub zip_code: i64,
    /// The `state_name` field represents the state name component of the address.
    #[serde(deserialize_with = "State::deserialize_mixed")]
    #[serde(rename = "state")]
    pub state_name: State,
    /// The `status` field represents the local status of the address as determined by the relevant
    /// addressing authority.
    pub status: AddressStatus,
    /// The `street_name_pre_modifier` field holds the pre-modifier element of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetNamePreModifier::deserialize_mixed",
        rename = "premod"
    )]
    pub street_name_pre_modifier: Option<StreetNamePreModifier>,
    /// The `street_name_pre_type` field holds the pre-type element of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetNamePreType::deserialize_mixed",
        rename = "pretype"
    )]
    pub street_name_pre_type: Option<StreetNamePreType>,
    /// The `street_separator` field holds the separator element of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetSeparator::deserialize_mixed",
        rename = "structdesc"
    )]
    pub street_separator: Option<StreetSeparator>,
}

impl Address for JosephineCountyAddress {
    fn number(&self) -> i64 {
        self.address_number
    }

    fn number_mut(&mut self) -> &mut i64 {
        &mut self.address_number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.address_number_suffix
    }

    fn number_suffix_mut(&mut self) -> &mut Option<String> {
        &mut self.address_number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.street_name_pre_directional
    }

    fn directional_mut(&mut self) -> &mut Option<StreetNamePreDirectional> {
        &mut self.street_name_pre_directional
    }

    fn street_name_pre_modifier(&self) -> &Option<StreetNamePreModifier> {
        &self.street_name_pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<StreetNamePreModifier> {
        &mut self.street_name_pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<StreetNamePreType> {
        &self.street_name_pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<StreetNamePreType> {
        &mut self.street_name_pre_type
    }

    fn street_name_separator(&self) -> &Option<StreetSeparator> {
        &self.street_separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<StreetSeparator> {
        &mut self.street_separator
    }

    fn street_name(&self) -> &String {
        &self.street_name
    }

    fn street_name_mut(&mut self) -> &mut String {
        &mut self.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.street_name_post_type
    }

    fn street_type_mut(&mut self) -> &mut Option<StreetNamePostType> {
        &mut self.street_name_post_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.subaddress_identifier
    }

    fn subaddress_id_mut(&mut self) -> &mut Option<String> {
        &mut self.subaddress_identifier
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.subaddress_type
    }

    fn subaddress_type_mut(&mut self) -> &mut Option<SubaddressType> {
        &mut self.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.floor
    }

    fn floor_mut(&mut self) -> &mut Option<i64> {
        &mut self.floor
    }

    fn building(&self) -> &Option<String> {
        &None
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.address_number_suffix
    }

    fn zip(&self) -> i64 {
        self.zip_code
    }

    fn zip_mut(&mut self) -> &mut i64 {
        &mut self.zip_code
    }

    fn postal_community(&self) -> &String {
        &self.postal_community
    }

    fn postal_community_mut(&mut self) -> &mut String {
        &mut self.postal_community
    }

    fn state(&self) -> &State {
        &self.state_name
    }

    fn state_mut(&mut self) -> &mut State {
        &mut self.state_name
    }

    fn status(&self) -> &AddressStatus {
        &self.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.status
    }
}

/// The `JosephineCountyAddresses` struct holds a vector of type
/// ['JosephineCountyAddress'].
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Deref, DerefMut)]
pub struct JosephineCountyAddresses(Vec<JosephineCountyAddress>);

impl Addresses<JosephineCountyAddress> for JosephineCountyAddresses {}

impl IntoBin<JosephineCountyAddresses> for JosephineCountyAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, AddressError> {
        match _load_bin(path) {
            Ok(records) => {
                let decode: Self = bincode::deserialize(&records)?;
                Ok(decode)
            }
            Err(source) => Err(AddressErrorKind::from(source).into()),
        }
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), AddressError> {
        _save(self, path)
    }
}

impl IntoCsv<JosephineCountyAddresses> for JosephineCountyAddresses {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Io> {
        let records = _from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        _to_csv(&mut self.0, path.as_ref().into())
    }
}

/// The `JosephineCountySpatialAddress` struct represents an address site point for Josephine County that includes geographic and projected coordinate information,
/// prior to the schema adopted by the agency in April of 2024.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct JosephineCountySpatialAddress {
    /// The `taxlot` field represents the map tax lot number of the parcel on which the address
    /// is located.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub taxlot: Option<String>,
    /// The `address_number` field represents the address number component of the complete address
    /// number.
    #[serde(rename = "stnum")]
    pub address_number: i64,
    /// The `address_number_suffix` field represents the address number suffix component of the complete
    /// address number.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "stnumsuf")]
    pub address_number_suffix: Option<String>,
    /// The `street_name_pre_directional` field represents the street name pre directional component of the
    /// complete street name.
    #[serde(
        deserialize_with = "StreetNamePreDirectional::deserialize_abbreviated",
        rename = "predir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    /// The `street_name` field represents the street name component of the complete street name.
    #[serde(rename = "name")]
    pub street_name: String,
    /// The `street_name_post_type` field represents the street name post type component of the complete street
    /// name.
    #[serde(
        // from abbreviated to mixed
        deserialize_with = "StreetNamePostType::deserialize_mixed",
        rename = "type"
    )]
    pub street_name_post_type: Option<StreetNamePostType>,
    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.
    #[serde(
        deserialize_with = "SubaddressType::deserialize_abbreviated",
        rename = "unit_type"
    )]
    pub subaddress_type: Option<SubaddressType>,
    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "unit")]
    pub subaddress_identifier: Option<String>,
    /// The `floor` field represents the floor identifier, corresponding to the `Floor` field from the NENA standard.
    #[serde(deserialize_with = "zero_floor")]
    pub floor: Option<i64>,
    /// The `complete_street_address` field represents the full street address as a string.
    #[serde(rename = "address")]
    pub complete_street_address: String,
    /// The `postal_community` field represents the postal community component of the address,
    /// being either the unincorporated or incorporated municipality name.
    #[serde(rename = "postcomm")]
    pub postal_community: String,
    /// The `zip_code` field represents the postal zip code of the address.
    #[serde(rename = "zip")]
    pub zip_code: i64,
    /// The `state_name` field represents the state name component of the address.
    #[serde(deserialize_with = "State::deserialize_mixed")]
    #[serde(rename = "state")]
    pub state_name: State,
    /// The `status` field represents the local status of the address as determined by the relevant
    /// addressing authority.
    pub status: AddressStatus,
    /// The `x` field represents the cartesian X portion of the projected coordinates of the
    /// address.
    #[serde(rename = "point_x")]
    pub x: f64,
    /// The `y` field represents the cartesian Y portion of the projected coordinates of the
    /// address.
    #[serde(rename = "point_y")]
    pub y: f64,
    /// The `lat` field represents the latitude of the geographic coordinates for the address.
    #[serde(rename = "latitude")]
    pub lat: f64,
    /// The `lon` field represents the longitude of the geographic coordinates for the address.
    #[serde(rename = "longitude")]
    pub lon: f64,
    /// The `street_name_pre_modifier` field holds the pre-modifier element of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetNamePreModifier::deserialize_mixed",
        rename = "premod"
    )]
    pub street_name_pre_modifier: Option<StreetNamePreModifier>,
    /// The `street_name_pre_type` field holds the pre-type element of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetNamePreType::deserialize_mixed",
        rename = "pretype"
    )]
    pub street_name_pre_type: Option<StreetNamePreType>,
    /// The `street_separator` field holds the separator element of the complete street
    /// name.
    #[serde(
        deserialize_with = "StreetSeparator::deserialize_mixed",
        rename = "structdesc"
    )]
    pub street_separator: Option<StreetSeparator>,
}

impl Address for JosephineCountySpatialAddress {
    fn number(&self) -> i64 {
        self.address_number
    }

    fn number_mut(&mut self) -> &mut i64 {
        &mut self.address_number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.address_number_suffix
    }

    fn number_suffix_mut(&mut self) -> &mut Option<String> {
        &mut self.address_number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.street_name_pre_directional
    }

    fn directional_mut(&mut self) -> &mut Option<StreetNamePreDirectional> {
        &mut self.street_name_pre_directional
    }

    fn street_name_pre_modifier(&self) -> &Option<StreetNamePreModifier> {
        &self.street_name_pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<StreetNamePreModifier> {
        &mut self.street_name_pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<StreetNamePreType> {
        &self.street_name_pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<StreetNamePreType> {
        &mut self.street_name_pre_type
    }

    fn street_name_separator(&self) -> &Option<StreetSeparator> {
        &self.street_separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<StreetSeparator> {
        &mut self.street_separator
    }

    fn street_name(&self) -> &String {
        &self.street_name
    }

    fn street_name_mut(&mut self) -> &mut String {
        &mut self.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.street_name_post_type
    }

    fn street_type_mut(&mut self) -> &mut Option<StreetNamePostType> {
        &mut self.street_name_post_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.subaddress_identifier
    }

    fn subaddress_id_mut(&mut self) -> &mut Option<String> {
        &mut self.subaddress_identifier
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.subaddress_type
    }

    fn subaddress_type_mut(&mut self) -> &mut Option<SubaddressType> {
        &mut self.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.floor
    }

    fn floor_mut(&mut self) -> &mut Option<i64> {
        &mut self.floor
    }

    fn building(&self) -> &Option<String> {
        &None
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.address_number_suffix
    }

    fn zip(&self) -> i64 {
        self.zip_code
    }

    fn zip_mut(&mut self) -> &mut i64 {
        &mut self.zip_code
    }

    fn postal_community(&self) -> &String {
        &self.postal_community
    }

    fn postal_community_mut(&mut self) -> &mut String {
        &mut self.postal_community
    }

    fn state(&self) -> &State {
        &self.state_name
    }

    fn state_mut(&mut self) -> &mut State {
        &mut self.state_name
    }

    fn status(&self) -> &AddressStatus {
        &self.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.status
    }
}

impl Point for JosephineCountySpatialAddress {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl GeoPoint for JosephineCountySpatialAddress {
    type Num = f64;

    fn lat(&self) -> Self::Num {
        self.lat
    }

    fn lon(&self) -> Self::Num {
        self.lon
    }
}

/// The `JosephineCountySpatialAddresses` struct holds a vector of type
/// ['JosephineCountySpatialAddress'].
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Deref, DerefMut)]
pub struct JosephineCountySpatialAddresses(Vec<JosephineCountySpatialAddress>);

impl Addresses<JosephineCountySpatialAddress> for JosephineCountySpatialAddresses {}

impl IntoBin<JosephineCountySpatialAddresses> for JosephineCountySpatialAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, AddressError> {
        match _load_bin(path) {
            Ok(records) => {
                let decode: Self = bincode::deserialize(&records)?;
                Ok(decode)
            }
            Err(source) => Err(AddressErrorKind::from(source).into()),
        }
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), AddressError> {
        _save(self, path)
    }
}

impl IntoCsv<JosephineCountySpatialAddresses> for JosephineCountySpatialAddresses {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Io> {
        let records = _from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        _to_csv(&mut self.0, path.as_ref().into())
    }
}
