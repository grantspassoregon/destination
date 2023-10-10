use crate::address_components::*;
use crate::business::*;
use crate::compare::*;
use crate::import::*;
use crate::utils;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashSet;
use tracing::{error, info};

#[derive(Debug, Clone, Default, Serialize)]
pub struct Address {
    address_number: i64,
    address_number_suffix: Option<String>,
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    street_name: String,
    street_name_post_type: StreetNamePostType,
    subaddress_type: Option<SubaddressType>,
    subaddress_identifier: Option<String>,
    floor: Option<i64>,
    building: Option<String>,
    zip_code: i64,
    postal_community: String,
    state_name: String,
    status: AddressStatus,
    object_id: i64,
    address_latitude: f64,
    address_longitude: f64,
}

impl Address {
    pub fn coincident(&self, other: &Address) -> AddressMatch {
        let mut coincident = false;
        let mut mismatches = Vec::new();
        if self.address_number == other.address_number
            && self.address_number_suffix == other.address_number_suffix
            && self.street_name_pre_directional == other.street_name_pre_directional
            && self.street_name == other.street_name
            && self.street_name_post_type == other.street_name_post_type
            && self.subaddress_identifier == other.subaddress_identifier
            && self.zip_code == other.zip_code
            && self.postal_community == other.postal_community
            && self.state_name == other.state_name
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

    pub fn label(&self) -> String {
        let complete_address_number = match &self.address_number_suffix {
            Some(suffix) => format!("{} {}", self.address_number, suffix),
            None => self.address_number.to_string(),
        };
        let complete_street_name = match self.street_name_pre_directional {
            Some(pre_directional) => format!(
                "{:?} {} {:?}",
                pre_directional, self.street_name, self.street_name_post_type
            ),
            None => format!("{} {:?}", self.street_name, self.street_name_post_type),
        };

        let accessory = match self.building() {
            Some(value) => Some(format!("BLDG {}", value)),
            None => None // match self.floor() {
                // Some(value) => Some(format!("FLR {}", value)),
                // None => None,
            // },
        };
        let complete_subaddress = match &self.subaddress_identifier {
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

    /// Distance between address and other addresses with matching label.
    /// Iterates through records of `others`, calculates the distance from self
    /// to matching addresses in others, collects the results into a vector and
    /// returns the results in the records field of a new AddressDeltas struct.
    pub fn deltas(&self, others: &Addresses, min: f64) -> AddressDeltas {
        let records = others
            .records_ref()
            .par_iter()
            .filter(|v| v.label() == self.label())
            .map(|v| AddressDelta::new(v, v.distance(self)))
            .filter(|d| d.delta > min)
            .collect::<Vec<AddressDelta>>();
        AddressDeltas { records }
    }

    pub fn distance(&self, other: &Address) -> f64 {
        ((self.address_latitude() - other.address_latitude()).powi(2)
            + (self.address_longitude() - other.address_longitude()).powi(2))
        .sqrt()
    }

    pub fn address_number(&self) -> i64 {
        self.address_number
    }

    pub fn street_name(&self) -> String {
        self.street_name.to_owned()
    }

    pub fn pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    pub fn post_type(&self) -> StreetNamePostType {
        self.street_name_post_type
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

impl From<CityAddress> for Address {
    fn from(item: CityAddress) -> Self {
        Address {
            address_number: item.address_number(),
            address_number_suffix: item.address_number_suffix(),
            street_name_pre_directional: item.street_name_pre_directional(),
            street_name: item.street_name(),
            street_name_post_type: item.street_name_post_type(),
            subaddress_type: item.subaddress_type(),
            subaddress_identifier: item.subaddress_identifier(),
            floor: item.floor(),
            building: item.building(),
            zip_code: item.zip_code(),
            postal_community: item.postal_community(),
            state_name: item.state_name(),
            status: item.status(),
            object_id: item.object_id(),
            address_latitude: item.address_latitude(),
            address_longitude: item.address_longitude(),
        }
    }
}

impl From<&CityAddress> for Address {
    fn from(item: &CityAddress) -> Self {
        Address {
            address_number: item.address_number(),
            address_number_suffix: item.address_number_suffix(),
            street_name_pre_directional: item.street_name_pre_directional(),
            street_name: item.street_name(),
            street_name_post_type: item.street_name_post_type(),
            subaddress_type: item.subaddress_type(),
            subaddress_identifier: item.subaddress_identifier(),
            floor: item.floor(),
            building: item.building(),
            zip_code: item.zip_code(),
            postal_community: item.postal_community(),
            state_name: item.state_name(),
            status: item.status(),
            object_id: item.object_id(),
            address_latitude: item.address_latitude(),
            address_longitude: item.address_longitude(),
        }
    }
}

impl TryFrom<CountyAddress> for Address {
    type Error = ();

    fn try_from(item: CountyAddress) -> Result<Self, Self::Error> {
        match item.street_name_post_type() {
            Some(post_type) => Ok(Address {
                address_number: item.address_number(),
                address_number_suffix: item.address_number_suffix(),
                street_name_pre_directional: item.street_name_pre_directional(),
                street_name: item.street_name(),
                street_name_post_type: post_type,
                subaddress_type: item.subaddress_type(),
                subaddress_identifier: item.subaddress_identifier(),
                floor: item.floor(),
                building: None,
                zip_code: item.zip_code(),
                postal_community: item.postal_community(),
                state_name: item.state_name(),
                status: item.status(),
                object_id: item.object_id(),
                address_latitude: item.address_latitude(),
                address_longitude: item.address_longitude(),
            }),
            None => Err(()),
        }
    }
}

impl TryFrom<&CountyAddress> for Address {
    type Error = ();

    fn try_from(item: &CountyAddress) -> Result<Self, Self::Error> {
        match item.street_name_post_type() {
            Some(post_type) => Ok(Address {
                address_number: item.address_number(),
                address_number_suffix: item.address_number_suffix(),
                street_name_pre_directional: item.street_name_pre_directional(),
                street_name: item.street_name(),
                street_name_post_type: post_type,
                subaddress_type: item.subaddress_type(),
                subaddress_identifier: item.subaddress_identifier(),
                floor: item.floor(),
                building: None,
                zip_code: item.zip_code(),
                postal_community: item.postal_community(),
                state_name: item.state_name(),
                status: item.status(),
                object_id: item.object_id(),
                address_latitude: item.address_latitude(),
                address_longitude: item.address_longitude(),
            }),
            None => Err(()),
        }
    }
}

impl TryFrom<GrantsPass2022Address> for Address {
    type Error = ();

    fn try_from(item: GrantsPass2022Address) -> Result<Self, Self::Error> {
        match item.post_type() {
            Some(post_type) => Ok(Address {
                address_number: item.address_number(),
                address_number_suffix: None,
                street_name_pre_directional: item.pre_directional(),
                street_name: item.street_name(),
                street_name_post_type: post_type,
                subaddress_type: None,
                subaddress_identifier: item.subaddress_identifier(),
                floor: item.floor(),
                building: None,
                zip_code: item.zip_code(),
                postal_community: item.postal_community(),
                state_name: item.state_name(),
                status: item.status(),
                object_id: item.object_id(),
                address_latitude: item.address_latitude(),
                address_longitude: item.address_longitude(),
            }),
            None => Err(()),
        }
    }
}

impl TryFrom<&GrantsPass2022Address> for Address {
    type Error = ();

    fn try_from(item: &GrantsPass2022Address) -> Result<Self, Self::Error> {
        match item.post_type() {
            Some(post_type) => Ok(Address {
                address_number: item.address_number(),
                address_number_suffix: None,
                street_name_pre_directional: item.pre_directional(),
                street_name: item.street_name(),
                street_name_post_type: post_type,
                subaddress_type: None,
                subaddress_identifier: item.subaddress_identifier(),
                floor: item.floor(),
                building: None,
                zip_code: item.zip_code(),
                postal_community: item.postal_community(),
                state_name: item.state_name(),
                status: item.status(),
                object_id: item.object_id(),
                address_latitude: item.address_latitude(),
                address_longitude: item.address_longitude(),
            }),
            None => Err(()),
        }
    }
}

#[derive(Default, Serialize, Clone)]
pub struct Addresses {
    pub records: Vec<Address>,
}

impl Addresses {
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
        Addresses { records }
    }

    fn filter_field(&self, filter: &str, field: &str) -> Self {
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
            _ => info!("Invalid filter provided."),
        }
        Addresses { records }
    }

    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        utils::to_csv(&mut self.records(), title)?;
        Ok(())
    }

    pub fn deltas(&self, other: &Addresses, min: f64) -> AddressDeltas {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Calculating deltas...'}",
        )
        .unwrap();
        let records_raw = self
            .records_ref()
            .par_iter()
            .progress_with_style(style)
            .map(|v| v.deltas(other, min))
            .collect::<Vec<AddressDeltas>>();
        let mut records = Vec::new();
        records_raw
            .iter()
            .map(|v| records.append(&mut v.records()))
            .for_each(drop);
        AddressDeltas { records }
    }

    pub fn records(&self) -> Vec<Address> {
        self.records.to_owned()
    }

    pub fn records_ref(&self) -> &Vec<Address> {
        &self.records
    }
}

impl From<CityAddresses> for Addresses {
    fn from(item: CityAddresses) -> Self {
        let mut records = Vec::new();
        for address in item.records {
            if let Ok(record) = Address::try_from(address) {
                records.push(record);
            }
        }
        Addresses { records }
    }
}

impl From<CountyAddresses> for Addresses {
    fn from(item: CountyAddresses) -> Self {
        let mut records = Vec::new();
        for address in item.records {
            if let Ok(record) = Address::try_from(address) {
                records.push(record);
            }
        }
        Addresses { records }
    }
}

impl From<GrantsPass2022Addresses> for Addresses {
    fn from(item: GrantsPass2022Addresses) -> Self {
        let mut records = Vec::new();
        for address in item.records {
            if let Ok(record) = Address::try_from(address) {
                records.push(record);
            }
        }
        Addresses { records }
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct PartialAddress {
    address_number: Option<i64>,
    address_number_suffix: Option<String>,
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    street_name: Option<String>,
    street_name_post_type: Option<StreetNamePostType>,
    subaddress_type: Option<SubaddressType>,
    subaddress_identifier: Option<String>,
    floor: Option<i64>,
    building: Option<String>,
    zip_code: Option<i64>,
    postal_community: Option<String>,
    state_name: Option<String>,
    status: Option<AddressStatus>,
}

impl PartialAddress {
    pub fn new() -> Self {
        PartialAddress::default()
    }

    pub fn address_number(&self) -> Option<i64> {
        self.address_number
    }

    pub fn address_number_suffix(&self) -> Option<String> {
        self.address_number_suffix.clone()
    }

    pub fn street_name_pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    pub fn street_name(&self) -> Option<String> {
        self.street_name.clone()
    }

    pub fn street_name_post_type(&self) -> Option<StreetNamePostType> {
        self.street_name_post_type
    }

    pub fn subaddress_type(&self) -> Option<SubaddressType> {
        self.subaddress_type
    }

    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.clone()
    }

    pub fn building(&self) -> Option<String> {
        self.building.clone()
    }

    pub fn floor(&self) -> Option<i64> {
        self.floor.clone()
    }

    pub fn set_address_number(&mut self, value: i64) {
        self.address_number = Some(value);
    }

    pub fn set_address_number_suffix(&mut self, value: Option<&str>) {
        if let Some(suffix) = value {
            self.address_number_suffix = Some(suffix.to_owned());
        } else {
            self.address_number_suffix = None;
        }
    }

    pub fn set_pre_directional(&mut self, value: &StreetNamePreDirectional) {
        self.street_name_pre_directional = Some(value.to_owned());
    }

    pub fn set_street_name(&mut self, value: &str) {
        self.street_name = Some(value.to_owned());
    }

    pub fn set_post_type(&mut self, value: &StreetNamePostType) {
        self.street_name_post_type = Some(value.to_owned());
    }

    pub fn set_subaddress_type(&mut self, value: &SubaddressType) {
        self.subaddress_type = Some(value.to_owned());
    }

    pub fn set_subaddress_identifier(&mut self, value: &str) {
        self.subaddress_identifier = Some(value.to_owned());
    }

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

pub struct PartialAddresses {
    records: Vec<PartialAddress>,
}

impl PartialAddresses {
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
/// The label field of AddressDelta holds the matching value and the delta
/// field holds the distance between matching points.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddressDelta {
    /// Addresses match by address label.
    label: String,
    /// Distance between points representing the same address.
    delta: f64,
    latitude: f64,
    longitude: f64,
}

impl AddressDelta {
    /// Initiates a new AddressDelta struct from the provided input values.
    pub fn new(address: &Address, delta: f64) -> Self {
        AddressDelta {
            label: address.label(),
            delta,
            latitude: address.address_latitude(),
            longitude: address.address_longitude(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AddressDeltas {
    records: Vec<AddressDelta>,
}

impl AddressDeltas {
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        utils::to_csv(&mut self.records(), title)?;
        Ok(())
    }

    pub fn records(&self) -> Vec<AddressDelta> {
        self.records.clone()
    }

    pub fn records_ref(&self) -> &Vec<AddressDelta> {
        &self.records
    }

    pub fn records_mut(&mut self) -> &mut Vec<AddressDelta> {
        &mut self.records
    }
}
