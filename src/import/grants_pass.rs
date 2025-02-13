//! The `grants_pass` module contains data types for importing addresses from the City of Grants
//! Pass.
use crate::{
    deserialize_arcgis_data, from_bin, from_csv, to_bin, to_csv, Address, AddressError,
    AddressErrorKind, AddressStatus, Addresses, Bincode, Cartesian, Geographic, IntoBin, IntoCsv,
    Io, State, StreetNamePostType, StreetNamePreDirectional, StreetNamePreModifier,
    StreetNamePreType, StreetSeparator, SubaddressType,
};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The `GrantsPassSpatialAddress` struct represents an address site point for the City of Grants Pass.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct GrantsPassAddress {
    /// The `address_number` field represents the address number component of the complete address
    /// number.
    #[serde(rename = "Add_Number")]
    pub address_number: i64,
    /// The `address_number_suffix` field represents the address number suffix component of the complete
    /// address number.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "AddNum_Suf")]
    pub address_number_suffix: Option<String>,
    /// The `street_name_pre_directional` field represents the street name pre directional component of the
    /// complete street name.
    #[serde(
        deserialize_with = "StreetNamePreDirectional::deserialize_mixed",
        rename = "St_PreDir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    /// The `street_name_pre_modifier` field represents the street name pre modifier component of the complete
    /// street name.
    #[serde(deserialize_with = "StreetNamePreModifier::deserialize_mixed")]
    pub street_name_pre_modifier: Option<StreetNamePreModifier>,
    /// The `street_name_pre_type` field represents the street name pre type component of the complete street
    /// name.
    #[serde(deserialize_with = "StreetNamePreType::deserialize_mixed")]
    pub street_name_pre_type: Option<StreetNamePreType>,
    /// The `street_name_separator` field represents the separator element component of the complete street
    /// name.
    #[serde(deserialize_with = "StreetSeparator::deserialize_mixed")]
    pub street_name_separator: Option<StreetSeparator>,
    /// The `street_name` field represents the street name component of the complete street name.
    #[serde(rename = "St_Name")]
    pub street_name: String,
    /// The `street_name_post_type` field represents the street name post type component of the complete street
    /// name.
    #[serde(rename = "St_PosTyp")]
    pub street_name_post_type: Option<StreetNamePostType>,
    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.
    #[serde(deserialize_with = "csv::invalid_option")]
    pub subaddress_type: Option<SubaddressType>,
    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub subaddress_identifier: Option<String>,
    /// The `floor` field represents the floor identifier, corresponding to the `Floor` field from the NENA standard.
    #[serde(deserialize_with = "csv::invalid_option")]
    pub floor: Option<i64>,
    /// The `building` field represents the building identifier, corresponding to the `Building`
    /// field from the NENA standard.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub building: Option<String>,
    /// The `zip_code` field represents the postal zip code of the address.
    #[serde(rename = "Post_Code")]
    pub zip_code: i64,
    /// The `status` field represents the local status of the address as determined by the relevant
    /// addressing authority.
    #[serde(rename = "STATUS")]
    pub status: AddressStatus,
    /// The `notification` field holds a web link to the final address notification issued by the
    /// City of Grants Pass.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "NOTIFICATION")]
    pub notification: Option<String>,
    /// The `notes` field holds any text note associated with the address.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "NOTES")]
    pub notes: Option<String>,
    /// The `global_id` field holds the ESRI Global ID associated with the feature.
    #[serde(rename(serialize = "GlobalID", deserialize = "GlobalID"))]
    pub global_id: String,
    /// The `created_user` field contains the user ID associated with the original creator of the
    /// feature.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "created_user")]
    pub created_user: Option<String>,
    /// The `created_date` field contains the original date of creation for the feature.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "created_date")]
    pub created_date: Option<String>,
    /// The `last_edited_user` field contains the user ID associated with the last edit of the
    /// feature.
    #[serde(rename = "last_edited_user")]
    pub last_edited_user: String,
    /// The `last_edited_date` field contains the date-time stamp associated with the last edit
    /// made to the feature.
    #[serde(rename = "last_edited_date")]
    pub last_edited_date: String,
    /// The `complete_address_number` field contains the complete address number component of the
    /// address, which is the space-delimited concatenation of the address number and address number suffix
    /// components.
    pub complete_address_number: String,
    /// The `complete_street_name` field contains the complete street name of the address, which is
    /// the space-delimited concatenation of those elements present among the street name pre
    /// directional, street name pre modifier, street name pre type, separator element, street
    /// name, street name post type and street name post directional, in that order.
    pub complete_street_name: String,
    /// The `complete_subaddress` field contains the complete subaddress component of the address,
    /// which consists of one or more subaddress elements, where each element includes an optional
    /// subaddress type and a required subaddress identifier.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub complete_subaddress: Option<String>,
    /// The `complete_street_address` is the string representation of the complete street address,
    /// where each component is fully spelled out according to FGDC specifications.
    pub complete_street_address: String,
    /// The `street_address_label` field contains the string representation of the street address
    /// using standard postal abbreviations for the street name pre directional and the street name
    /// post type.
    #[serde(rename = "FULLADDRESS")]
    pub street_address_label: String,
    /// The `place_state_zip` field contains the postal community, state name abbreviation and zip
    /// code formatted for printing mailing labels.
    pub place_state_zip: String,
    /// The `postal_community` field represents the postal community component of the address,
    /// being either the unincorporated or incorporated municipality name.
    #[serde(rename = "Post_Comm")]
    pub postal_community: String,
    /// The `state_name` field represents the state name component of the address.
    #[serde(deserialize_with = "State::deserialize_mixed")]
    pub state_name: State,
    /// The `incorporated_municipality` field contains the name of the incorporated municipality
    /// associated with the address (e.g. City of Grants Pass).
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "Inc_Muni")]
    pub incorporated_municipality: Option<String>,
    /// The `unincorporated_community` field contains the name of the unincorporated community
    /// associated with the address (e.g. Merlin).
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "Uninc_Comm")]
    pub unincorporated_community: Option<String>,
}

impl Address for GrantsPassAddress {
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
        &self.building
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.building
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

/// The `GrantsPassAddresses` struct holds a vector of type
/// ['GrantsPassAddress'].
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct GrantsPassAddresses(Vec<GrantsPassAddress>);

impl Addresses<GrantsPassAddress> for GrantsPassAddresses {}

impl IntoBin<GrantsPassAddresses> for GrantsPassAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, AddressError> {
        match from_bin(path) {
            Ok(records) => bincode::deserialize::<Self>(&records)
                .map_err(|source| Bincode::new(source, line!(), file!().into()).into()),
            Err(source) => Err(AddressErrorKind::from(source).into()),
        }
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), AddressError> {
        to_bin(self, path)
    }
}

impl IntoCsv<GrantsPassAddresses> for GrantsPassAddresses {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        to_csv(&mut self.0, path.as_ref().into())
    }
}

/// The `GrantsPassSpatialAddress` struct represents an address site point for the City of Grants Pass that includes geographic and projected coordinate information.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct GrantsPassSpatialAddress {
    /// The `address_number` field represents the address number component of the complete address
    /// number.
    #[serde(rename = "Add_Number")]
    pub address_number: i64,
    /// The `address_number_suffix` field represents the address number suffix component of the complete
    /// address number.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "AddNum_Suf")]
    pub address_number_suffix: Option<String>,
    /// The `street_name_pre_directional` field represents the street name pre directional component of the
    /// complete street name.
    #[serde(
        deserialize_with = "StreetNamePreDirectional::deserialize_mixed",
        rename = "St_PreDir"
    )]
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    /// The `street_name_pre_modifier` field represents the street name pre modifier component of the complete
    /// street name.
    #[serde(deserialize_with = "StreetNamePreModifier::deserialize_mixed")]
    pub street_name_pre_modifier: Option<StreetNamePreModifier>,
    /// The `street_name_pre_type` field represents the street name pre type component of the complete street
    /// name.
    #[serde(deserialize_with = "StreetNamePreType::deserialize_mixed")]
    pub street_name_pre_type: Option<StreetNamePreType>,
    /// The `street_name_separator` field represents the separator element component of the complete street
    /// name.
    #[serde(deserialize_with = "StreetSeparator::deserialize_mixed")]
    pub street_name_separator: Option<StreetSeparator>,
    /// The `street_name` field represents the street name component of the complete street name.
    #[serde(rename = "St_Name")]
    pub street_name: String,
    /// The `street_name_post_type` field represents the street name post type component of the complete street
    /// name.
    #[serde(deserialize_with = "StreetNamePostType::deserialize_mixed")]
    #[serde(rename = "St_PosTyp")]
    pub street_name_post_type: Option<StreetNamePostType>,
    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.
    #[serde(deserialize_with = "csv::invalid_option")]
    pub subaddress_type: Option<SubaddressType>,
    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub subaddress_identifier: Option<String>,
    /// The `floor` field represents the floor identifier, corresponding to the `Floor` field from the NENA standard.
    #[serde(deserialize_with = "csv::invalid_option")]
    pub floor: Option<i64>,
    /// The `building` field represents the building identifier, corresponding to the `Building`
    /// field from the NENA standard.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub building: Option<String>,
    /// The `zip_code` field represents the postal zip code of the address.
    #[serde(rename = "Post_Code")]
    pub zip_code: i64,
    /// The `status` field represents the local status of the address as determined by the relevant
    /// addressing authority.
    #[serde(rename = "STATUS")]
    pub status: AddressStatus,
    /// The `notification` field holds a web link to the final address notification issued by the
    /// City of Grants Pass.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "NOTIFICATION")]
    pub notification: Option<String>,
    /// The `notes` field holds any text note associated with the address.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "NOTES")]
    pub notes: Option<String>,
    /// The `global_id` field holds the ESRI Global ID associated with the feature.
    #[serde(rename(serialize = "GlobalID", deserialize = "GlobalID"))]
    pub global_id: String,
    /// The `created_user` field contains the user ID associated with the original creator of the
    /// feature.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "created_user")]
    pub created_user: Option<String>,
    /// The `created_date` field contains the original date of creation for the feature.
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "created_date")]
    pub created_date: Option<String>,
    /// The `last_edited_user` field contains the user ID associated with the last edit of the
    /// feature.
    #[serde(rename = "last_edited_user")]
    pub last_edited_user: String,
    /// The `last_edited_date` field contains the date-time stamp associated with the last edit
    /// made to the feature.
    #[serde(rename = "last_edited_date")]
    pub last_edited_date: String,
    /// The `complete_address_number` field contains the complete address number component of the
    /// address, which is the space-delimited concatenation of the address number and address number suffix
    /// components.
    pub complete_address_number: String,
    /// The `complete_street_name` field contains the complete street name of the address, which is
    /// the space-delimited concatenation of those elements present among the street name pre
    /// directional, street name pre modifier, street name pre type, separator element, street
    /// name, street name post type and street name post directional, in that order.
    pub complete_street_name: String,
    /// The `complete_subaddress` field contains the complete subaddress component of the address,
    /// which consists of one or more subaddress elements, where each element includes an optional
    /// subaddress type and a required subaddress identifier.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub complete_subaddress: Option<String>,
    /// The `complete_street_address` is the string representation of the complete street address,
    /// where each component is fully spelled out according to FGDC specifications.
    pub complete_street_address: String,
    /// The `street_address_label` field contains the string representation of the street address
    /// using standard postal abbreviations for the street name pre directional and the street name
    /// post type.
    #[serde(rename = "FULLADDRESS")]
    pub street_address_label: String,
    /// The `place_state_zip` field contains the postal community, state name abbreviation and zip
    /// code formatted for printing mailing labels.
    pub place_state_zip: String,
    /// The `postal_community` field represents the postal community component of the address,
    /// being either the unincorporated community or incorporated municipality name.
    #[serde(rename = "Post_Comm")]
    pub postal_community: String,
    /// The `state_name` field represents the state name component of the address.
    #[serde(deserialize_with = "State::deserialize_mixed")]
    pub state_name: State,
    /// The `incorporated_municipality` field contains the name of the incorporated municipality
    /// associated with the address (e.g. City of Grants Pass).
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "Inc_Muni")]
    pub incorporated_municipality: Option<String>,
    /// The `unincorporated_community` field contains the name of the unincorporated community
    /// associated with the address (e.g. Merlin).
    #[serde(deserialize_with = "deserialize_arcgis_data", rename = "Uninc_Comm")]
    pub unincorporated_community: Option<String>,
    /// The `x` field represents the cartesian X portion of the projected coordinates of the
    /// address.
    #[serde(rename = "x")]
    pub x: f64,
    /// The `y` field represents the cartesian Y portion of the projected coordinates of the
    /// address.
    #[serde(rename = "y")]
    pub y: f64,
    /// The `latitude` field represents the latitude of the geographic coordinates for the address.
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// The `longitude` field represents the longitude of the geographic coordinates for the address.
    #[serde(rename = "longitude")]
    pub longitude: f64,
}

impl Address for GrantsPassSpatialAddress {
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
        &self.building
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.building
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

impl Cartesian for GrantsPassSpatialAddress {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl Geographic for GrantsPassSpatialAddress {
    fn latitude(&self) -> f64 {
        self.latitude
    }

    fn longitude(&self) -> f64 {
        self.longitude
    }
}

/// The `GrantsPassSpatialAddresses` struct holds a vector of type
/// ['GrantsPassSpatialAddress'].
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct GrantsPassSpatialAddresses(Vec<GrantsPassSpatialAddress>);

impl Addresses<GrantsPassSpatialAddress> for GrantsPassSpatialAddresses {}

impl IntoBin<GrantsPassSpatialAddresses> for GrantsPassSpatialAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, AddressError> {
        match from_bin(path) {
            Ok(records) => bincode::deserialize::<Self>(&records)
                .map_err(|source| Bincode::new(source, line!(), file!().into()).into()),
            Err(source) => Err(AddressErrorKind::from(source).into()),
        }
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), AddressError> {
        to_bin(self, path)
    }
}

impl IntoCsv<GrantsPassSpatialAddresses> for GrantsPassSpatialAddresses {
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        to_csv(&mut self.0, path.as_ref().into())
    }
}
