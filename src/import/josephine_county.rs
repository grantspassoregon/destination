use crate::address_components::*;
use crate::prelude::{Address, GeoPoint};
use crate::utils;
use crate::utils::deserialize_arcgis_data;
use serde::{Deserialize, Serialize};

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

    fn number_suffix(&self) -> &Option<String> {
        &self.address_number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.street_name_pre_directional
    }

    fn street_name(&self) -> &String {
        &self.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.street_name_post_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.subaddress_identifier
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.floor
    }

    fn building(&self) -> &Option<String> {
        &None
    }

    fn zip(&self) -> i64 {
        self.zip_code
    }

    fn postal_community(&self) -> &String {
        &self.postal_community
    }

    fn state(&self) -> &String {
        &self.state_name
    }

    fn status(&self) -> &AddressStatus {
        &self.status
    }
}

impl GeoPoint for CountyAddress {
    fn lat(&self) -> f64 {
        self.address_latitude
    }

    fn lon(&self) -> f64 {
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
//
// impl Addreses<CountyAddress> for CountyAddresses {
//     fn records(&self) -> &Vec<CountyAddress> {
//         &self.records
//     }
// }
//
// impl Points<CountyAddress> for CountyAddresses {
//     fn records(&self) -> &Vec<CountyAddress> {
//         &self.records
//     }
//
// }

impl CountyAddresses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = utils::from_csv(path)?;
        Ok(CountyAddresses { records })
    }
}
