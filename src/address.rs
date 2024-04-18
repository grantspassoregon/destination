//! The `address` module defines the library data standard for a valid address, and provides
//! implementation blocks to convert data from import types to the valid address format.
use crate::prelude::*;
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::{error, info, trace};

pub trait Address {
    fn number(&self) -> i64;
    fn number_suffix(&self) -> &Option<String>;
    fn directional(&self) -> &Option<StreetNamePreDirectional>;
    fn street_name(&self) -> &String;
    fn street_type(&self) -> &Option<StreetNamePostType>;
    fn subaddress_id(&self) -> &Option<String>;
    fn subaddress_type(&self) -> &Option<SubaddressType>;
    fn floor(&self) -> &Option<i64>;
    fn building(&self) -> &Option<String>;
    fn zip(&self) -> i64;
    fn postal_community(&self) -> &String;
    fn state(&self) -> &String;
    fn status(&self) -> &AddressStatus;

    /// An address is coincident when the `other` address refers to the same assignment or
    /// location.  If the addresses are coincident, but details (such as the floor number or
    /// address status) differ, then the differences are recorded as a vector of type [`Mismatch`].
    /// The results are converted to type [`AddressMatch`].
    fn coincident<T: Address>(&self, other: &T) -> AddressMatch {
        let mut coincident = false;
        let mut mismatches = Vec::new();
        if self.number() == other.number()
            && self.number_suffix() == other.number_suffix()
            && self.directional() == other.directional()
            && self.street_name() == other.street_name()
            && self.street_type() == other.street_type()
            && self.subaddress_id() == other.subaddress_id()
            && self.zip() == other.zip()
            && self.postal_community() == other.postal_community()
            && self.state() == other.state()
        {
            coincident = true;
            if self.subaddress_type() != other.subaddress_type() {
                mismatches.push(Mismatch::subaddress_type(
                    self.subaddress_type().clone(),
                    other.subaddress_type().clone(),
                ));
            }
            if self.floor() != other.floor() {
                mismatches.push(Mismatch::floor(self.floor().clone(), other.floor().clone()));
            }
            if self.building() != other.building() {
                mismatches.push(Mismatch::building(
                    self.building().clone(),
                    other.building().clone(),
                ));
            }
            if self.status() != other.status() {
                mismatches.push(Mismatch::status(
                    self.status().clone(),
                    other.status().clone(),
                ));
            }
        }
        AddressMatch::new(coincident, mismatches)
    }

    /// Returns a String representing the address label, consisting of the complete address number,
    /// complete street name and complete subaddress, used to produce map or mailing labels.
    fn label(&self) -> String {
        let complete_address_number = match &self.number_suffix() {
            Some(suffix) => format!("{} {}", self.number(), suffix),
            None => self.number().to_string(),
        };

        let complete_street_name = match self.directional() {
            Some(pre_directional) => match self.street_type() {
                Some(post_type) => format!(
                    "{} {} {}",
                    pre_directional,
                    self.street_name(),
                    post_type.abbreviate()
                ),
                None => format!("{} {}", pre_directional, self.street_name()),
            },
            None => match self.street_type() {
                Some(post_type) => format!("{} {}", self.street_name(), post_type.abbreviate()),
                None => format!("{}", self.street_name()),
            },
        };

        let accessory = match self.building() {
            Some(value) => Some(format!("BLDG {}", value)),
            None => None,
        };

        let complete_subaddress = match &self.subaddress_id() {
            Some(identifier) => match self.subaddress_type() {
                Some(subaddress_type) => Some(format!("{} {}", subaddress_type, identifier)),
                None => Some(format!("#{}", identifier)),
            },
            None => self
                .subaddress_type()
                .map(|subaddress_type| format!("{}", subaddress_type)),
        };

        match complete_subaddress {
            Some(subaddress) => format!(
                "{} {} {}",
                complete_address_number, complete_street_name, subaddress
            ),
            None => match accessory {
                Some(value) => format!(
                    "{} {} {}",
                    complete_address_number, complete_street_name, value
                ),
                None => format!("{} {}", complete_address_number, complete_street_name),
            },
        }
    }

    /// The `complete_street_name` method returns the complete street name of the address.
    fn complete_street_name(&self) -> String {
        match self.directional() {
            Some(pre_directional) => match self.street_type() {
                Some(post_type) => {
                    format!("{} {} {:?}", pre_directional, self.street_name(), post_type)
                }
                None => format!("{} {}", pre_directional, self.street_name()),
            },
            None => format!("{} {:?}", self.street_name(), self.street_type()),
        }
    }

    /// The `pre_directional` field represents the street name predirectional component of the
    /// complete street name.  This function returns the cloned value of the field.
    fn directional_abbreviated(&self) -> Option<String> {
        match self.directional() {
            Some(StreetNamePreDirectional::NORTH) => Some("N".to_string()),
            Some(StreetNamePreDirectional::EAST) => Some("E".to_string()),
            Some(StreetNamePreDirectional::SOUTH) => Some("S".to_string()),
            Some(StreetNamePreDirectional::WEST) => Some("W".to_string()),
            Some(StreetNamePreDirectional::NORTHEAST) => Some("NE".to_string()),
            Some(StreetNamePreDirectional::NORTHWEST) => Some("NW".to_string()),
            Some(StreetNamePreDirectional::SOUTHEAST) => Some("SE".to_string()),
            Some(StreetNamePreDirectional::SOUTHWEST) => Some("SW".to_string()),
            None => None,
        }
    }

    /// The `filter_field` method returns the subset of addresses where the field `filter` is equal
    /// to the value in `field`.
    fn filter_field<T: Address + Clone + Send + Sync>(values: &[T], filter: &str, field: &str) -> Vec<T> {
        let mut records = Vec::new();
        match filter {
            "label" => records.append(
                &mut values
                    // .iter()
                    .par_iter()
                    .cloned()
                    .filter(|record| field == record.label())
                    .collect(),
            ),
            "street_name" => records.append(
                &mut values
                    .par_iter()
                    .cloned()
                    .filter(|record| field == record.street_name())
                    .collect(),
            ),
            "pre_directional" => records.append(
                &mut values
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        if let Some(dir) = record.directional() {
                            field == &format!("{}", dir)
                        } else {
                            false
                        }
                    })
                    .collect(),
            ),
            "post_type" => records.append(
                &mut values
                    .par_iter()
                    .cloned()
                    .filter(|record| field == &format!("{:?}", record.street_type()))
                    .collect(),
            ),

            _ => info!("Invalid filter provided."),
        }
        records
    }

    /// Writes the contents of `Addresses` to a CSV file output to path `title`.  Each element
    /// of the vector in `records` writes to a row on the CSV file.
    fn to_csv<T: Address + Clone + Serialize>(
        values: &mut [T],
        title: std::path::PathBuf,
    ) -> Result<(), std::io::Error> {
        to_csv(&mut values.to_vec(), title)?;
        Ok(())
    }
}


/// The `Address` struct defines the fields of a valid address.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommonAddress {
    /// The `number` field represents the address number component of the complete address
    /// number.
    pub number: i64,
    pub number_suffix: Option<String>,
    pub directional: Option<StreetNamePreDirectional>,
    /// The `street_name` field represents the street name component of the complete street name.
    pub street_name: String,
    /// The `street_type` field represents the street name post type component of the complete street
    /// name.
    pub street_type: Option<StreetNamePostType>,
    pub subaddress_type: Option<SubaddressType>,
    pub subaddress_id: Option<String>,
    pub floor: Option<i64>,
    pub building: Option<String>,
    pub zip: i64,
    pub postal_community: String,
    pub state: String,
    pub status: AddressStatus,
}


impl CommonAddress {
    /// An address is coincident when the `other` address refers to the same assignment or
    /// location.  If the addresses are coincident, but details (such as the floor number or
    /// address status) differ, then the differences are recorded as a vector of type [`Mismatch`].
    /// The results are converted to type [`AddressMatch`].
    pub fn coincident(&self, other: &CommonAddress) -> AddressMatch {
        let mut coincident = false;
        let mut mismatches = Vec::new();
        if self.number == other.number
            && self.number_suffix == other.number_suffix
            && self.directional == other.directional
            && self.street_name == other.street_name
            && self.street_type == other.street_type
            && self.subaddress_id == other.subaddress_id
            && self.zip == other.zip
            && self.postal_community == other.postal_community
            && self.state == other.state
        {
            coincident = true;
            if self.subaddress_type != other.subaddress_type {
                mismatches.push(Mismatch::subaddress_type(
                    self.subaddress_type,
                    other.subaddress_type,
                ));
            }
            if self.floor != other.floor {
                mismatches.push(Mismatch::floor(self.floor, other.floor));
            }
            if self.building != other.building {
                mismatches.push(Mismatch::building(
                    self.building.clone(),
                    other.building.clone(),
                ));
            }
            if self.status != other.status {
                mismatches.push(Mismatch::status(self.status, other.status));
            }
        }
        AddressMatch::new(coincident, mismatches)
    }

    /// Returns a String representing the address label, consisting of the complete address number,
    /// complete street name and complete subaddress, used to produce map or mailing labels.
    pub fn label(&self) -> String {
        let complete_address_number = match &self.number_suffix {
            Some(suffix) => format!("{} {}", self.number, suffix),
            None => self.number.to_string(),
        };

        let complete_street_name = match self.directional {
            Some(pre_directional) => format!(
                "{:?} {} {:?}",
                pre_directional, self.street_name, self.street_type
            ),
            None => format!("{} {:?}", self.street_name, self.street_type),
        };

        let accessory = match self.building() {
            Some(value) => Some(format!("BLDG {}", value)),
            None => None,
        };

        let complete_subaddress = match &self.subaddress_id {
            Some(identifier) => match self.subaddress_type {
                Some(subaddress_type) => Some(format!("{:?} {}", subaddress_type, identifier)),
                None => Some(format!("#{}", identifier)),
            },
            None => self
                .subaddress_type
                .map(|subaddress_type| format!("{:?}", subaddress_type)),
        };

        match complete_subaddress {
            Some(subaddress) => format!(
                "{} {} {}",
                complete_address_number, complete_street_name, subaddress
            ),
            None => match accessory {
                Some(value) => format!(
                    "{} {} {}",
                    complete_address_number, complete_street_name, value
                ),
                None => format!("{} {}", complete_address_number, complete_street_name),
            },
        }
    }

    /// The `complete_street_name` method returns the complete street name of the address.
    pub fn complete_street_name(&self) -> String {
        match self.directional {
            Some(pre_directional) => format!(
                "{:?} {} {:?}",
                pre_directional, self.street_name, self.street_type
            ),
            None => format!("{} {:?}", self.street_name, self.street_type),
        }
    }

    /// The `set_street_name` field sets the street name component of the complete street name.
    pub fn set_street_name(&mut self, name: &str) {
        self.street_name = name.to_owned();
    }

    /// The `pre_directional` field represents the street name predirectional component of the
    /// complete street name.  This function returns the cloned value of the field.
    pub fn pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.directional
    }

    /// The `pre_directional` field represents the street name predirectional component of the
    /// complete street name.  This function returns the cloned value of the field.
    pub fn pre_directional_abbreviated(&self) -> Option<String> {
        match self.directional {
            Some(StreetNamePreDirectional::NORTH) => Some("N".to_string()),
            Some(StreetNamePreDirectional::EAST) => Some("E".to_string()),
            Some(StreetNamePreDirectional::SOUTH) => Some("S".to_string()),
            Some(StreetNamePreDirectional::WEST) => Some("W".to_string()),
            Some(StreetNamePreDirectional::NORTHEAST) => Some("NE".to_string()),
            Some(StreetNamePreDirectional::NORTHWEST) => Some("NW".to_string()),
            Some(StreetNamePreDirectional::SOUTHEAST) => Some("SE".to_string()),
            Some(StreetNamePreDirectional::SOUTHWEST) => Some("SW".to_string()),
            None => None,
        }
    }

    /// The `set post_type` field sets the street name posttype component of the complete street
    /// name.
    pub fn set_post_type(&mut self, value: &StreetNamePostType) {
        self.street_type = Some(value.to_owned());
    }

    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.  This function returns the cloned value of the field.
    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_id.to_owned()
    }

    /// The `floor` field represents the floor of the building on which the address point is located.  This function returns the value of the field.
    pub fn floor(&self) -> Option<i64> {
        self.floor
    }

    /// The `building` field represents the unique identifier for a building.  This function
    /// returns the cloned value of the field.
    pub fn building(&self) -> Option<String> {
        self.building.to_owned()
    }

    /// The `zip_code` field represents the zip code.  This function returns the value of the
    /// field.
    pub fn zip_code(&self) -> i64 {
        self.zip
    }

    /// The `status` field represents the status of an address.  This function returns the cloned
    /// value of the field.
    pub fn status(&self) -> AddressStatus {
        self.status
    }

    /// The `state_name` field contains the postal code for the State in an address.  This
    /// function returns the cloned value of the field.
    pub fn state_name(&self) -> String {
        self.state.to_owned()
    }

    /// The `postal_community` field represents the incorporated municipality or unincorporated
    /// community that serves as the postal community (the "City" field in an address).  This
    /// function returns the cloned value of the field.
    pub fn postal_community(&self) -> String {
        self.postal_community.to_owned()
    }
}

impl Address for CommonAddress {
    fn number(&self) -> i64 {
        self.number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.directional
    }

    fn street_name(&self) -> &String {
        &self.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.street_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.subaddress_id
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
        self.zip
    }

    fn postal_community(&self) -> &String {
        &self.postal_community
    }

    fn state(&self) -> &String {
        &self.state
    }

    fn status(&self) -> &AddressStatus {
        &self.status
    }
}

impl<T: Address> From<&T> for CommonAddress {
    fn from(address: &T) -> Self {
        let number = address.number();
        let number_suffix = address.number_suffix().clone();
        let directional = address.directional().clone();
        let street_name = address.street_name().clone();
        let street_type = address.street_type().clone();
        let subaddress_type = address.subaddress_type().clone();
        let subaddress_id = address.subaddress_id().clone();
        let floor = address.floor().clone();
        let building = address.building().clone();
        let zip = address.zip();
        let postal_community = address.postal_community().clone();
        let state = address.state().clone();
        let status = address.status().clone();
        Self {
            number,
            number_suffix,
            directional,
            street_name,
            street_type,
            subaddress_type,
            subaddress_id,
            floor,
            building,
            zip,
            postal_community,
            state,
            status
        }
    }
}

/// The `CommonAddresses` struct holds a vector of type [`CommonAddress`].
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CommonAddresses {
    pub records: Vec<CommonAddress>,
}

impl CommonAddresses {
    /// Filter elements of the `records` vector according to the argument specified in `filter`.
    /// Currently only the value "duplicate" is supported as an argument to `filter`, which only
    /// retains duplicate addresses in the vector.  Duplicate addresses are defined as having
    /// identical address labels using the [`Address::label()`] method.
    pub fn filter(&self, filter: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "duplicate" => {
                let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Checking for duplicate addresses.'}",
        )
        .unwrap();
                let mut seen = HashSet::new();
                let bar = ProgressBar::new(self.records.len() as u64);
                bar.set_style(style);
                for address in &self.records {
                    let label = address.label();
                    if !seen.contains(&label) {
                        seen.insert(label.clone());
                        let mut same = self.filter_field("label", &label);
                        if same.records.len() > 1 {
                            records.append(&mut same.records);
                        }
                    }
                    bar.inc(1);
                }
            }
            _ => error!("Invalid filter provided."),
        }
        CommonAddresses { records }
    }

    /// The `filter_field` method returns the subset of addresses where the field `filter` is equal
    /// to the value in `field`.
    pub fn filter_field(&self, filter: &str, field: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "label" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| field == record.label())
                    .collect(),
            ),
            "street_name" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| field == record.street_name)
                    .collect(),
            ),
            "pre_directional" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| field == format!("{:?}", record.pre_directional()))
                    .collect(),
            ),
            "post_type" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| field == format!("{:?}", record.street_type))
                    .collect(),
            ),

            _ => info!("Invalid filter provided."),
        }
        CommonAddresses { records }
    }

    /// Writes the contents of `CommonAddresses` to a CSV file output to path `title`.  Each element
    /// of the vector in `records` writes to a row on the CSV file.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(&mut self.records(), title)?;
        Ok(())
    }

    /// Compares the complete street name of an address to the value in `street`, returning true if
    /// equal.
    pub fn contains_street(&self, street: &String) -> bool {
        let mut contains = false;
        for address in self.records_ref() {
            let comp_street = address.complete_street_name();
            if &comp_street == street {
                contains = true;
            }
        }
        contains
    }

    /// The `orphan_streets` method returns the list of complete street names that are contained in
    /// self but are not present in `other`.
    pub fn orphan_streets(&self, other: &CommonAddresses) -> Vec<String> {
        let mut seen = HashSet::new();
        let mut orphans = Vec::new();
        for address in self.records_ref() {
            let street = address.complete_street_name();
            if !seen.contains(&street) {
                seen.insert(street.clone());
                if !other.contains_street(&street) {
                    orphans.push(street);
                }
            }
        }
        orphans
    }

    /// The `citify` method takes county address naming conventions and converts them to city
    /// naming conventions.
    pub fn citify(&self) -> Self {
        trace!("Running Citify");
        let mut records = Vec::new();
        for mut address in self.records() {
            let comp_street = address.complete_street_name();
            if comp_street == "NE BEAVILLA VIEW" {
                trace!("Fixing Beavilla View");
                address.set_street_name("BEAVILLA");
                address.set_post_type(&StreetNamePostType::VIEW);
            }
            if comp_street == "COLUMBIA CREST" {
                trace!("Fixing Beavilla View");
                address.set_street_name("COLUMBIA");
                address.set_post_type(&StreetNamePostType::CREST);
            }
            if comp_street == "SE FORMOSA GARDENS" {
                trace!("Fixing Formosa Gardens");
                address.set_street_name("FORMOSA");
                address.set_post_type(&StreetNamePostType::GARDENS);
            }
            if comp_street == "SE HILLTOP VIEW" {
                trace!("Fixing Hilltop View");
                address.set_street_name("HILLTOP");
                address.set_post_type(&StreetNamePostType::VIEW);
            }
            if comp_street == "MARILEE ROW" {
                trace!("Fixing Marilee Row");
                address.set_street_name("MARILEE");
                address.set_post_type(&StreetNamePostType::ROW);
            }
            if comp_street == "MEADOW GLEN" {
                trace!("Fixing Meadow Glen");
                address.set_street_name("MEADOW");
                address.set_post_type(&StreetNamePostType::GLEN);
            }
            if comp_street == "ROBERTSON CREST" {
                trace!("Fixing Robertson Crest");
                address.set_street_name("ROBERTSON");
                address.set_post_type(&StreetNamePostType::CREST);
            }
            if comp_street == "NE QUAIL CROSSING" {
                trace!("Fixing Quail Crossing");
                address.set_street_name("QUAIL");
                address.set_post_type(&StreetNamePostType::CROSSING);
            }
            records.push(address)
        }
        CommonAddresses { records }
    }

    /// The `records` field hold a vector of type [`Address`].  This function returns the cloned
    /// value of the field.
    pub fn records(&self) -> Vec<CommonAddress> {
        self.records.to_owned()
    }

    /// This function returns a reference to the vector in the `records` field.
    pub fn records_ref(&self) -> &Vec<CommonAddress> {
        &self.records
    }
}

impl<T: Address + Clone> From<&[T]> for CommonAddresses {
    fn from(addresses: &[T]) -> Self {
        let records = addresses.iter().map(|v| CommonAddress::from(v)).collect::<Vec<CommonAddress>>();
        Self { records }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct SpatialAddress {
    address: CommonAddress,
    x: f64,
    y: f64,
    lat: f64,
    lon: f64,
}

impl Point for SpatialAddress {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl GeoPoint for SpatialAddress {
    fn lat(&self) -> f64 {
        self.lat
    }

    fn lon(&self) -> f64 {
        self.lon
    }
}


impl Address for SpatialAddress {
    fn number(&self) -> i64 {
        self.address.number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.address.number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.address.directional
    }

    fn street_name(&self) -> &String {
        &self.address.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.address.street_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.address.subaddress_id
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.address.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.address.floor
    }

    fn building(&self) -> &Option<String> {
        &self.address.building
    }

    fn zip(&self) -> i64 {
        self.address.zip
    }

    fn postal_community(&self) -> &String {
        &self.address.postal_community
    }

    fn state(&self) -> &String {
        &self.address.state
    }

    fn status(&self) -> &AddressStatus {
        &self.address.status
    }
}

/// The `SpatialAddresses` struct holds a vector of type [`SpatialAddress`].
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct SpatialAddresses {
    pub records: Vec<SpatialAddress>,
}


/// The `PartialAddress` struct contains optional fields so that incomplete or missing data can be
/// compared against [`Addresses`] or [`PartialAddresses`] for potential matches.  Used to help
/// match address information that does not parse into a full valid address.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartialAddress {
    /// The `address_number` field represents the address number component of the complete address
    /// number.
    pub address_number: Option<i64>,
    pub address_number_suffix: Option<String>,
    pub street_name_pre_directional: Option<StreetNamePreDirectional>,
    pub street_name: Option<String>,
    pub street_name_post_type: Option<StreetNamePostType>,
    pub subaddress_type: Option<SubaddressType>,
    pub subaddress_identifier: Option<String>,
    pub floor: Option<i64>,
    pub building: Option<String>,
    pub zip_code: Option<i64>,
    pub postal_community: Option<String>,
    pub state_name: Option<String>,
    pub status: Option<AddressStatus>,
}

impl PartialAddress {
    /// Creates an empty new `PartialAddress` with all fields set to None.
    pub fn new() -> Self {
        PartialAddress::default()
    }

    /// The `address_number` field represents the address number component of the complete address
    /// number.  This function returns the value of the field.
    pub fn address_number(&self) -> Option<i64> {
        self.address_number
    }

    /// The `address_number_suffix` field represents the address number suffix component of the
    /// complete address number.  This function returns the cloned value of the field.
    pub fn address_number_suffix(&self) -> Option<String> {
        self.address_number_suffix.clone()
    }

    /// The `street_name_pre_directional` field represents the street name predirectional component of the
    /// complete street name.  This function returns the cloned value of the field.
    pub fn street_name_pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    /// The `street_name` field represents the street name component of the complete street name.
    /// This function returns the cloned value of the field.
    pub fn street_name(&self) -> Option<String> {
        self.street_name.clone()
    }

    /// The `street_name_post_type` field represents the street name posttype component of the complete street
    /// name.  This function returns the cloned value of the field.
    pub fn street_name_post_type(&self) -> Option<StreetNamePostType> {
        self.street_name_post_type
    }

    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.  This function returns the cloned value of the field.
    pub fn subaddress_type(&self) -> Option<SubaddressType> {
        self.subaddress_type
    }

    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.  This function returns the cloned value of the field.
    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.clone()
    }

    /// The `building` field represents the unique identifier for a building.  This function
    /// returns the cloned value of the field.
    pub fn building(&self) -> Option<String> {
        self.building.clone()
    }

    /// The `floor` field represents the floor of the building on which the address point is located.  This function returns the value of the field.
    pub fn floor(&self) -> Option<i64> {
        self.floor.clone()
    }

    /// Sets the value of the `address_number` field to Some(`value`).
    pub fn set_address_number(&mut self, value: i64) {
        self.address_number = Some(value);
    }

    /// Sets the value of the `address_number_suffix` field to Some(`value`).
    pub fn set_address_number_suffix(&mut self, value: Option<&str>) {
        if let Some(suffix) = value {
            self.address_number_suffix = Some(suffix.to_owned());
        } else {
            self.address_number_suffix = None;
        }
    }

    /// Sets the value of the `street_name_pre_directional` field to Some(`value`).
    pub fn set_pre_directional(&mut self, value: &StreetNamePreDirectional) {
        self.street_name_pre_directional = Some(value.to_owned());
    }

    /// Sets the value of the `street_name` field to Some(`value`).
    pub fn set_street_name(&mut self, value: &str) {
        self.street_name = Some(value.to_owned());
    }

    /// Sets the value of the `street_name_post_type` field to Some(`value`).
    pub fn set_post_type(&mut self, value: &StreetNamePostType) {
        self.street_name_post_type = Some(value.to_owned());
    }

    /// Sets the value of the `subaddress_type` field to Some(`value`).
    pub fn set_subaddress_type(&mut self, value: &SubaddressType) {
        self.subaddress_type = Some(value.to_owned());
    }

    /// Sets the value of the `subaddress_identifier` field to Some(`value`).
    pub fn set_subaddress_identifier(&mut self, value: &str) {
        self.subaddress_identifier = Some(value.to_owned());
    }

    /// Returns a String representing the address label, consisting of the complete address number,
    /// complete street name and complete subaddress, used to produce map or mailing labels.
    pub fn label(&self) -> String {
        let mut address = "".to_owned();
        if let Some(address_number) = self.address_number() {
            address.push_str(&format!("{}", address_number));
        }
        if let Some(address_number_suffix) = self.address_number_suffix() {
            address.push_str(" ");
            address.push_str(&address_number_suffix);
        }
        if let Some(pre_directional) = self.street_name_pre_directional() {
            address.push_str(" ");
            address.push_str(&format!("{:?}", pre_directional));
        }
        if let Some(street_name) = self.street_name() {
            address.push_str(" ");
            address.push_str(&street_name);
        }
        if let Some(post_type) = self.street_name_post_type() {
            address.push_str(" ");
            address.push_str(&format!("{:?}", post_type));
        }
        if let Some(subtype) = self.subaddress_type() {
            address.push_str(" ");
            address.push_str(&format!("{:?}", subtype));
        }
        if let Some(subaddress_identifier) = self.subaddress_identifier() {
            address.push_str(" ");
            address.push_str(&subaddress_identifier);
        }
        address
    }
}

/// The `PartialAddresses` struct holds a `records` field that contains a vector of type
/// [`PartialAddress`].
pub struct PartialAddresses {
    records: Vec<PartialAddress>,
}

impl PartialAddresses {
    /// Creates a new `PartialAddresses` struct from the provided `records`, a vector of
    /// [`PartialAddress`] objects.
    pub fn new(records: Vec<PartialAddress>) -> Self {
        Self { records }
    }
    /// The `records` field holds a vector of type [`PartialAddress`].  This function returns the
    /// cloned value of the field.
    pub fn records(&self) -> Vec<PartialAddress> {
        self.records.clone()
    }
}

impl From<Vec<PartialAddress>> for PartialAddresses {
    fn from(records: Vec<PartialAddress>) -> Self {
        PartialAddresses { records }
    }
}

impl From<&FireInspections> for PartialAddresses {
    fn from(fire_inspections: &FireInspections) -> Self {
        PartialAddresses::from(
            fire_inspections
                .records()
                .iter()
                .map(|r| r.address())
                .collect::<Vec<PartialAddress>>(),
        )
    }
}

/// Deltas - Measuring the distance between points based upon matching values.
/// The `label` field of `AddressDelta` holds the matching value and the `delta`
/// field holds the distance between matching points.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddressDelta {
    /// Addresses match by address label.
    pub label: String,
    /// Distance between points representing the same address.
    pub delta: f64,
    pub latitude: f64,
    pub longitude: f64,
}

impl AddressDelta {
    /// Initiates a new `AddressDelta` struct from the provided input values.
    pub fn new<T: Address + Point>(address: &T, delta: f64) -> Self {
        AddressDelta {
            label: address.label(),
            delta,
            latitude: address.y(),
            longitude: address.x(),
        }
    }
}

/// The `AddressDeltas` struct holds a `records` field that contains a vector of type
/// [`AddressDelta`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddressDeltas {
    pub records: Vec<AddressDelta>,
}

impl AddressDeltas {
    /// Writes the contents of `AddressDeltas` to a CSV file output to path `title`.  Each element
    /// of the vector in `records` writes to a row on the CSV file.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(&mut self.records(), title)?;
        Ok(())
    }

    /// The `records` field hold a vector of type [`AddressDelta`].  This function returns the cloned
    /// value of the field.
    pub fn records(&self) -> Vec<AddressDelta> {
        self.records.clone()
    }

    /// This functions returns a reference to the vector contained in the `records` field.
    pub fn records_ref(&self) -> &Vec<AddressDelta> {
        &self.records
    }

    /// This functions returns a mutable reference to the vector contained in the `records` field.
    pub fn records_mut(&mut self) -> &mut Vec<AddressDelta> {
        &mut self.records
    }
}
