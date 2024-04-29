use crate::address_components::*;
use crate::prelude::{from_csv, load_bin, save, to_csv, Address, Point, Portable, Vectorized, Addresses};
use crate::utils::deserialize_arcgis_data;
use aid::prelude::*;
use galileo::galileo_types::geo::GeoPoint;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct JosephineCountyAddress {
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub taxlot: Option<String>,
    #[serde(rename = "stnum")]
    pub address_number: i64,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "stnumsuf")]
    pub address_number_suffix: Option<String>,
    #[serde(
        deserialize_with = "deserialize_abbreviated_pre_directional",
        rename = "predir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename = "name")]
    pub street_name: String,
    #[serde(
        deserialize_with = "deserialize_abbreviated_post_type",
        rename = "type"
    )]
    pub street_name_post_type: Option<StreetNamePostType>,
    #[serde(
        deserialize_with = "deserialize_abbreviated_subaddress_type",
        rename = "unit_type"
    )]
    pub subaddress_type: Option<SubaddressType>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "unit")]
    pub subaddress_identifier: Option<String>,
    #[serde(deserialize_with = "zero_floor")]
    pub floor: Option<i64>,
    #[serde(rename = "address")]
    pub complete_street_address: String,
    #[serde(rename = "postcomm")]
    pub postal_community: String,
    #[serde(rename = "zip")]
    pub zip_code: i64,
    #[serde(rename = "state")]
    pub state_name: String,
    pub status: AddressStatus,
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

    fn state(&self) -> &String {
        &self.state_name
    }

    fn state_mut(&mut self) -> &mut String {
        &mut self.state_name
    }

    fn status(&self) -> &AddressStatus {
        &self.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.status
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct JosephineCountyAddresses {
    pub records: Vec<JosephineCountyAddress>,
}

impl Addresses<JosephineCountyAddress> for JosephineCountyAddresses {}

impl Vectorized<JosephineCountyAddress> for JosephineCountyAddresses {
    fn values(&self) -> &Vec<JosephineCountyAddress> {
        &self.records
    }

    fn values_mut(&mut self) -> &mut Vec<JosephineCountyAddress> {
        &mut self.records
    }

    fn into_values(self) -> Vec<JosephineCountyAddress> {
        self.records
    }
}

impl Portable<JosephineCountyAddresses> for JosephineCountyAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        save(self, path)
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = from_csv(path)?;
        Ok(Self { records })
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.records, path.as_ref().into())?)
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct JosephineCountySpatialAddress {
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub taxlot: Option<String>,
    #[serde(rename = "stnum")]
    pub address_number: i64,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "stnumsuf")]
    pub address_number_suffix: Option<String>,
    #[serde(
        deserialize_with = "deserialize_abbreviated_pre_directional",
        rename = "predir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename = "name")]
    pub street_name: String,
    #[serde(
        deserialize_with = "deserialize_abbreviated_post_type",
        rename = "type"
    )]
    pub street_name_post_type: Option<StreetNamePostType>,
    #[serde(
        deserialize_with = "deserialize_abbreviated_subaddress_type",
        rename = "unit_type"
    )]
    pub subaddress_type: Option<SubaddressType>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "unit")]
    pub subaddress_identifier: Option<String>,
    #[serde(deserialize_with = "zero_floor")]
    pub floor: Option<i64>,
    #[serde(rename = "address")]
    pub complete_street_address: String,
    #[serde(rename = "postcomm")]
    pub postal_community: String,
    #[serde(rename = "zip")]
    pub zip_code: i64,
    #[serde(rename = "state")]
    pub state_name: String,
    pub status: AddressStatus,
    #[serde(rename = "point_y")]
    pub x: f64,
    #[serde(rename = "point_x")]
    pub y: f64,
    #[serde(rename = "latitude")]
    pub lat: f64,
    #[serde(rename = "longitude")]
    pub lon: f64,
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

    fn state(&self) -> &String {
        &self.state_name
    }

    fn state_mut(&mut self) -> &mut String {
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

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct JosephineCountySpatialAddresses {
    pub records: Vec<JosephineCountySpatialAddress>,
}

impl Addresses<JosephineCountySpatialAddress> for JosephineCountySpatialAddresses {}

impl Vectorized<JosephineCountySpatialAddress> for JosephineCountySpatialAddresses {
    fn values(&self) -> &Vec<JosephineCountySpatialAddress> {
        &self.records
    }

    fn values_mut(&mut self) -> &mut Vec<JosephineCountySpatialAddress> {
        &mut self.records
    }

    fn into_values(self) -> Vec<JosephineCountySpatialAddress> {
        self.records
    }
}

impl Portable<JosephineCountySpatialAddresses> for JosephineCountySpatialAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        save(self, path)
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = from_csv(path)?;
        Ok(Self { records })
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.records, path.as_ref().into())?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CountyAddress {
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    taxlot: Option<String>,
    #[serde(rename(deserialize = "stnum"))]
    address_number: i64,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "stnumsuf")
    )]
    address_number_suffix: Option<String>,
    #[serde(
        deserialize_with = "deserialize_abbreviated_pre_directional",
        rename(deserialize = "predir")
    )]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename(deserialize = "name"))]
    street_name: String,
    #[serde(
        deserialize_with = "deserialize_abbreviated_post_type",
        rename(deserialize = "type")
    )]
    street_name_post_type: Option<StreetNamePostType>,
    #[serde(
        deserialize_with = "deserialize_abbreviated_subaddress_type",
        rename(deserialize = "unit_type")
    )]
    subaddress_type: Option<SubaddressType>,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "unit")
    )]
    subaddress_identifier: Option<String>,
    #[serde(deserialize_with = "zero_floor")]
    floor: Option<i64>,
    #[serde(rename(deserialize = "address"))]
    complete_street_address: String,
    #[serde(rename(deserialize = "postcomm"))]
    postal_community: String,
    #[serde(rename(deserialize = "zip"))]
    zip_code: i64,
    #[serde(rename(deserialize = "state"))]
    state_name: String,
    status: AddressStatus,
    #[serde(rename(deserialize = "point_y"))]
    address_latitude: f64,
    #[serde(rename(deserialize = "point_x"))]
    address_longitude: f64,
    // #[serde(rename(deserialize = "latitude"))]
    // address_latitude: f64,
    // #[serde(rename(deserialize = "longitude"))]
    // address_longitude: f64,
}

impl Address for CountyAddress {
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

    fn state(&self) -> &String {
        &self.state_name
    }

    fn state_mut(&mut self) -> &mut String {
        &mut self.state_name
    }

    fn status(&self) -> &AddressStatus {
        &self.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.status
    }
}

impl GeoPoint for CountyAddress {
    type Num = f64;

    fn lat(&self) -> Self::Num {
        self.address_latitude
    }

    fn lon(&self) -> Self::Num {
        self.address_longitude
    }
}

impl CountyAddress {
    pub fn address_number(&self) -> i64 {
        self.address_number
    }

    pub fn address_number_suffix(&self) -> Option<String> {
        self.address_number_suffix.to_owned()
    }

    pub fn street_name(&self) -> String {
        self.street_name.to_owned()
    }

    pub fn street_name_pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    pub fn street_name_post_type(&self) -> Option<StreetNamePostType> {
        self.street_name_post_type
    }

    pub fn subaddress_type(&self) -> Option<SubaddressType> {
        self.subaddress_type.to_owned()
    }

    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.to_owned()
    }

    pub fn floor(&self) -> Option<i64> {
        self.floor
    }

    pub fn zip_code(&self) -> i64 {
        self.zip_code
    }

    pub fn status(&self) -> AddressStatus {
        self.status
    }

    pub fn state_name(&self) -> String {
        self.state_name.to_owned()
    }

    pub fn postal_community(&self) -> String {
        self.postal_community.to_owned()
    }

    pub fn address_latitude(&self) -> f64 {
        self.address_latitude
    }

    pub fn address_longitude(&self) -> f64 {
        self.address_longitude
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CountyAddresses {
    pub records: Vec<CountyAddress>,
}

impl Portable<CountyAddresses> for CountyAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        save(self, path)
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = from_csv(path)?;
        Ok(Self { records })
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.records, path.as_ref().into())?)
    }
}
