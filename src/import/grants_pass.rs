use crate::address_components::*;
use crate::prelude::{from_csv, load_bin, save, to_csv, Address, Point, Portable};
use crate::utils;
use crate::utils::deserialize_arcgis_data;
use aid::prelude::*;
use aid::prelude::*;
use galileo::galileo_types::geo::GeoPoint;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct GrantsPassAddress {
    #[serde(rename = "Add_Number")]
    pub address_number: i64,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "AddNum_Suf")]
    pub address_number_suffix: Option<String>,
    #[serde(
        deserialize_with = "deserialize_mixed_pre_directional",
        rename = "St_PreDir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename = "St_Name")]
    pub street_name: String,
    #[serde(rename = "St_PosTyp")]
    pub street_name_post_type: Option<StreetNamePostType>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub subaddress_type: Option<SubaddressType>,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub subaddress_identifier: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub floor: Option<i64>,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub building: Option<String>,
    #[serde(rename = "Post_Code")]
    pub zip_code: i64,
    #[serde(rename = "STATUS")]
    pub status: AddressStatus,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "NOTIFICATION")]
    pub notification: Option<String>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "NOTES")]
    pub notes: Option<String>,
    #[serde(rename(serialize = "GlobalID", deserialize = "GlobalID"))]
    pub global_id: String,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "created_user")]
    pub created_user: Option<String>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "created_date")]
    pub created_date: Option<String>,
    #[serde(rename = "last_edited_user")]
    pub last_edited_user: String,
    #[serde(rename = "last_edited_date")]
    pub last_edited_date: String,
    pub complete_address_number: String,
    pub complete_street_name: String,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub complete_subaddress: Option<String>,
    pub complete_street_address: String,
    #[serde(rename = "FULLADDRESS")]
    pub street_address_label: String,
    pub place_state_zip: String,
    #[serde(rename = "Post_Comm")]
    pub postal_community: String,
    pub state_name: String,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "Inc_Muni")]
    pub incorporated_municipality: Option<String>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "Uninc_Comm")]
    pub unincorporated_community: Option<String>,
}

impl Address for GrantsPassAddress {
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
        &self.building
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct GrantsPassAddresses {
    pub records: Vec<GrantsPassAddress>,
}

impl GrantsPassAddresses {
    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Portable<GrantsPassAddresses> for GrantsPassAddresses {
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct GrantsPassSpatialAddress {
    #[serde(rename = "Add_Number")]
    pub address_number: i64,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "AddNum_Suf")]
    pub address_number_suffix: Option<String>,
    #[serde(
        deserialize_with = "deserialize_mixed_pre_directional",
        rename = "St_PreDir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename = "St_Name")]
    pub street_name: String,
    #[serde(rename = "St_PosTyp")]
    pub street_name_post_type: Option<StreetNamePostType>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub subaddress_type: Option<SubaddressType>,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub subaddress_identifier: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub floor: Option<i64>,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub building: Option<String>,
    #[serde(rename = "Post_Code")]
    pub zip_code: i64,
    #[serde(rename = "STATUS")]
    pub status: AddressStatus,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "NOTIFICATION")]
    pub notification: Option<String>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "NOTES")]
    pub notes: Option<String>,
    #[serde(rename(serialize = "GlobalID", deserialize = "GlobalID"))]
    pub global_id: String,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "created_user")]
    pub created_user: Option<String>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "created_date")]
    pub created_date: Option<String>,
    #[serde(rename = "last_edited_user")]
    pub last_edited_user: String,
    #[serde(rename = "last_edited_date")]
    pub last_edited_date: String,
    pub complete_address_number: String,
    pub complete_street_name: String,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub complete_subaddress: Option<String>,
    pub complete_street_address: String,
    #[serde(rename = "FULLADDRESS")]
    pub street_address_label: String,
    pub place_state_zip: String,
    #[serde(rename = "Post_Comm")]
    pub postal_community: String,
    pub state_name: String,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "Inc_Muni")]
    pub incorporated_municipality: Option<String>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "Uninc_Comm")]
    pub unincorporated_community: Option<String>,
    #[serde(rename = "AddressYCoordinate")]
    pub x: f64,
    #[serde(rename = "AddressXCoordinate")]
    pub y: f64,
    #[serde(rename = "latitude")]
    pub latitude: f64,
    #[serde(rename = "longitude")]
    pub longitude: f64,
}

impl Address for GrantsPassSpatialAddress {
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
        &self.building
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

impl Point for GrantsPassSpatialAddress {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl GeoPoint for GrantsPassSpatialAddress {
    type Num = f64;

    fn lat(&self) -> Self::Num {
        self.latitude
    }

    fn lon(&self) -> Self::Num {
        self.longitude
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct GrantsPassSpatialAddresses {
    pub records: Vec<GrantsPassSpatialAddress>,
}

impl GrantsPassSpatialAddresses {
    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Portable<GrantsPassSpatialAddresses> for GrantsPassSpatialAddresses {
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct CityAddress {
    #[serde(rename(deserialize = "Add_Number"))]
    address_number: i64,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "AddNum_Suf")
    )]
    address_number_suffix: Option<String>,
    #[serde(
        deserialize_with = "deserialize_mixed_pre_directional",
        // deserialize_with = "csv::invalid_option",
        rename(deserialize = "St_PreDir")
    )]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename(deserialize = "St_Name"))]
    street_name: String,
    #[serde(rename(deserialize = "St_PosTyp"))]
    // #[serde(deserialize_with = "deserialize_mixed_post_type",
    //         rename(deserialize = "St_PosTyp"))]
    street_name_post_type: Option<StreetNamePostType>,
    #[serde(deserialize_with = "csv::invalid_option")]
    subaddress_type: Option<SubaddressType>,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    subaddress_identifier: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    floor: Option<i64>,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    building: Option<String>,
    #[serde(rename(deserialize = "Post_Code"))]
    zip_code: i64,
    #[serde(rename(deserialize = "STATUS"))]
    status: AddressStatus,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "NOTIFICATION")
    )]
    notification: Option<String>,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "NOTES")
    )]
    notes: Option<String>,
    #[serde(rename(deserialize = "GlobalID"))]
    global_id: String,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "created_user")
    )]
    created_user: Option<String>,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "created_date")
    )]
    created_date: Option<String>,
    #[serde(rename(deserialize = "last_edited_user"))]
    last_edited_user: String,
    #[serde(rename(deserialize = "last_edited_date"))]
    last_edited_date: String,
    complete_address_number: String,
    complete_street_name: String,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    complete_subaddress: Option<String>,
    complete_street_address: String,
    #[serde(rename(deserialize = "FULLADDRESS"))]
    street_address_label: String,
    place_state_zip: String,
    #[serde(rename(deserialize = "Post_Comm"))]
    postal_community: String,
    state_name: String,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "Inc_Muni")
    )]
    incorporated_municipality: Option<String>,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "Uninc_Comm")
    )]
    unincorporated_community: Option<String>,
    #[serde(rename(deserialize = "AddressYCoordinate"))]
    address_x_coordinate: f64,
    #[serde(rename(deserialize = "AddressXCoordinate"))]
    address_y_coordinate: f64,
    #[serde(rename(deserialize = "latitude"))]
    address_latitude: f64,
    #[serde(rename(deserialize = "longitude"))]
    address_longitude: f64,
}

impl CityAddress {
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

    pub fn building(&self) -> Option<String> {
        self.building.to_owned()
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

    pub fn address_x_coordinate(&self) -> f64 {
        self.address_x_coordinate
    }

    pub fn address_y_coordinate(&self) -> f64 {
        self.address_y_coordinate
    }

    pub fn address_latitude(&self) -> f64 {
        self.address_latitude
    }

    pub fn address_longitude(&self) -> f64 {
        self.address_longitude
    }
}

impl Address for CityAddress {
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
        &self.building
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

impl Point for CityAddress {
    fn x(&self) -> f64 {
        self.address_x_coordinate
    }

    fn y(&self) -> f64 {
        self.address_y_coordinate
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CityAddresses {
    pub records: Vec<CityAddress>,
}

impl CityAddresses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = utils::from_csv(path)?;
        Ok(CityAddresses { records })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OldCityAddress {
    #[serde(rename(deserialize = "OID_"))]
    object_id: i64,
    #[serde(rename(deserialize = "ADDRNUM"))]
    address_number: i64,
    #[serde(
        deserialize_with = "deserialize_arcgis_data",
        rename(deserialize = "APARTMENT")
    )]
    subaddress_identifier: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    floor: Option<i64>,
    #[serde(
        deserialize_with = "csv::invalid_option",
        rename(deserialize = "ROADPREDIR")
    )]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename(deserialize = "ROADNAME"))]
    street_name: String,
    #[serde(rename(deserialize = "ROADTYPE"))]
    street_name_post_type: StreetNamePostType,
    #[serde(rename(deserialize = "GlobalID"))]
    global_id: String,
    #[serde(rename(deserialize = "AddressYCoordinate"))]
    address_latitude: f64,
    #[serde(rename(deserialize = "AddressXCoordinate"))]
    address_longitude: f64,
}

impl OldCityAddress {
    pub fn address_number(&self) -> i64 {
        self.address_number
    }

    pub fn street_name(&self) -> String {
        self.street_name.to_owned()
    }

    pub fn street_name_pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    pub fn street_name_post_type(&self) -> StreetNamePostType {
        self.street_name_post_type
    }

    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.to_owned()
    }

    pub fn floor(&self) -> Option<i64> {
        self.floor
    }

    pub fn object_id(&self) -> i64 {
        self.object_id
    }

    pub fn global_id(&self) -> String {
        self.global_id.clone()
    }

    pub fn global_id_ref(&self) -> &String {
        &self.global_id
    }

    pub fn address_latitude(&self) -> f64 {
        self.address_latitude
    }

    pub fn address_longitude(&self) -> f64 {
        self.address_longitude
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OldCityAddresses {
    pub records: Vec<OldCityAddress>,
}

impl OldCityAddresses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = utils::from_csv(path)?;
        Ok(OldCityAddresses { records })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GrantsPass2022Address {
    #[serde(rename(deserialize = "OID_"))]
    object_id: i64,
    #[serde(rename(deserialize = "ADDRNUM"))]
    address_number: i64,
    #[serde(
        deserialize_with = "csv::invalid_option",
        rename(deserialize = "ROADPREDIR")
    )]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename(deserialize = "ROADNAME"))]
    street_name: String,
    #[serde(
        rename(deserialize = "ROADTYPE"),
        deserialize_with = "deserialize_mixed_post_type"
    )]
    street_name_post_type: Option<StreetNamePostType>,
    #[serde(
        rename(deserialize = "APARTMENT"),
        deserialize_with = "deserialize_arcgis_data"
    )]
    subaddress_identifier: Option<String>,
    #[serde(
        rename(deserialize = "FLOOR"),
        deserialize_with = "csv::invalid_option"
    )]
    floor: Option<i64>,
    #[serde(rename(deserialize = "ZIP"))]
    zip_code: i64,
    #[serde(rename(deserialize = "STATUS"))]
    status: AddressStatus,
    #[serde(rename(deserialize = "CITY"))]
    postal_community: String,
    #[serde(rename(deserialize = "STATE"))]
    state_name: String,
    #[serde(rename(deserialize = "AddressXCoordinate"))]
    address_latitude: f64,
    #[serde(rename(deserialize = "AddressYCoordinate"))]
    address_longitude: f64,
    // #[serde(rename(deserialize = "latitude"))]
    // address_latitude: f64,
    // #[serde(rename(deserialize = "longitude"))]
    // address_longitude: f64,
}

impl GrantsPass2022Address {
    pub fn address_number(&self) -> i64 {
        self.address_number
    }

    pub fn street_name(&self) -> String {
        self.street_name.to_owned()
    }

    pub fn pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    pub fn post_type(&self) -> Option<StreetNamePostType> {
        self.street_name_post_type
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

    pub fn object_id(&self) -> i64 {
        self.object_id
    }

    pub fn address_latitude(&self) -> f64 {
        self.address_latitude
    }

    pub fn address_longitude(&self) -> f64 {
        self.address_longitude
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrantsPass2022Addresses {
    pub records: Vec<GrantsPass2022Address>,
}

impl GrantsPass2022Addresses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = utils::from_csv(path)?;
        Ok(GrantsPass2022Addresses { records })
    }
}
