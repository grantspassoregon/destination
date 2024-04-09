//! The `address` module defines the library data standard for a valid address, and provides
//! implementation blocks to convert data from import types to the valid address format.
use crate::address_components::*;
use crate::compare::*;
use crate::import::*;
use crate::utils;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::{error, info, trace};

pub trait Addres {
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
    fn coincident<T: Addres>(&self, other: &T) -> AddressMatch {
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
    fn filter_field<T: Addres + Clone + Send + Sync>(values: &[T], filter: &str, field: &str) -> Vec<T> {
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
    fn to_csv<T: Addres + Clone + Serialize>(
        values: &mut [T],
        title: std::path::PathBuf,
    ) -> Result<(), std::io::Error> {
        utils::to_csv(&mut values.to_vec(), title)?;
        Ok(())
    }

    // /// Filter elements of the `records` vector according to the argument specified in `filter`.
    // /// Currently only the value "duplicate" is supported as an argument to `filter`, which only
    // /// retains duplicate addresses in the vector.  Duplicate addresses are defined as having
    // /// identical address labels using the [`Address::label()`] method.
    // fn filter<T: Addres + Clone>(values: &[T], filter: &str) -> Vec<T> {
    //     let mut records = Vec::new();
    //     match filter {
    //         "duplicate" => {
    //             let style = indicatif::ProgressStyle::with_template(
    //         "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Checking for duplicate addresses.'}",
    //     )
    //     .unwrap();
    //             let mut seen = HashSet::new();
    //             let bar = ProgressBar::new(values.len() as u64);
    //             bar.set_style(style);
    //             for address in values {
    //                 let label = address.label();
    //                 if !seen.contains(&label) {
    //                     seen.insert(label.clone());
    //                     let mut same = Addres::filter_field(values, "label", &label);
    //                     if same.len() > 1 {
    //                         records.append(&mut same);
    //                     }
    //                 }
    //                 bar.inc(1);
    //             }
    //         }
    //         _ => error!("Invalid filter provided."),
    //     }
    //     records
    // }
}

pub trait Point {
    fn lat(&self) -> f64;
    fn lon(&self) -> f64;

    /// The `distance` function returns the distance between a point `self` and another point
    /// `other` in the same unit as `self`.
    fn distance<T: Point + ?Sized>(&self, other: &T) -> f64 {
        ((self.lat() - other.lat()).powi(2) + (self.lon() - other.lon()).powi(2)).sqrt()
    }

    /// Distance between address and other addresses with matching label.
    /// Iterates through records of `others`, calculates the distance from self
    /// to matching addresses in others, collects the results into a vector and
    /// returns the results in the records field of a new `AddressDeltas` struct.
    //     pub fn deltas<'a, V: Point + rayon::iter::IntoParallelIterator + rayon::iter::IntoParallelRefIterator<'a>, U: Points<V> + ParallelProgressIterator + rayon::iter::IntoParallelIterator + rayon::iter::IntoParallelRefIterator<'a> + Addres>(&self, others: &U, min: f64) -> AddressDeltas {
    fn delta<T: Addres + Clone + Point + Sync + Send>(
        &self,
        others: &[T],
        min: f64,
    ) -> AddressDeltas
    where
        Self: Addres + Point + Sized + Clone + Send + Sync,
    {
        let records = others
            .par_iter()
            .filter(|v| v.label() == self.label())
            .map(|v| AddressDelta::new(v, v.distance(self)))
            .filter(|d| d.delta > min)
            .collect::<Vec<AddressDelta>>();
        AddressDeltas { records }
    }

    /// Distance between addresses and other addresses with matching label.
    /// Iterates through records of `others`, calculates the distance from self
    /// to matching addresses in others, collects the results into a vector and
    /// returns the results in the records field of a new `AddressDeltas` struct. Calls
    /// [`Address::deltas()`].
    fn deltas<
        'a,
        T: Point + Addres + Clone + Sync + Send,
        U: Point + Addres + Clone + Sync + Send,
    >(
        values: &[T],
        other: &[U],
        min: f64,
    ) -> AddressDeltas {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Calculating deltas...'}",
        )
        .unwrap();
        let records_raw = values
            .par_iter()
            .progress_with_style(style)
            .map(|v| Point::delta(v, other, min))
            .collect::<Vec<AddressDeltas>>();
        let mut records = Vec::new();
        records_raw
            .iter()
            .map(|v| records.append(&mut v.records()))
            .for_each(drop);
        AddressDeltas { records }
    }
}

/// The `Address` struct defines the fields of a valid address.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    address_latitude: f64,
    address_longitude: f64,
}

impl Point for Address {
    fn lat(&self) -> f64 {
        self.address_latitude
    }

    fn lon(&self) -> f64 {
        self.address_longitude
    }
}

impl Address {
    /// An address is coincident when the `other` address refers to the same assignment or
    /// location.  If the addresses are coincident, but details (such as the floor number or
    /// address status) differ, then the differences are recorded as a vector of type [`Mismatch`].
    /// The results are converted to type [`AddressMatch`].
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

    /// Returns a String representing the address label, consisting of the complete address number,
    /// complete street name and complete subaddress, used to produce map or mailing labels.
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
            None => None,
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

    /// The `complete_street_name` method returns the complete street name of the address.
    pub fn complete_street_name(&self) -> String {
        match self.street_name_pre_directional {
            Some(pre_directional) => format!(
                "{:?} {} {:?}",
                pre_directional, self.street_name, self.street_name_post_type
            ),
            None => format!("{} {:?}", self.street_name, self.street_name_post_type),
        }
    }

    /// Distance between address and other addresses with matching label.
    /// Iterates through records of `others`, calculates the distance from self
    /// to matching addresses in others, collects the results into a vector and
    /// returns the results in the records field of a new `AddressDeltas` struct.
    // pub fn deltas(&self, others: &Addresses, min: f64) -> AddressDeltas {
    //     let records = others
    //         .records_ref()
    //         .par_iter()
    //         .filter(|v| v.label() == self.label())
    //         .map(|v| AddressDelta::new(v, v.distance(self)))
    //         .filter(|d| d.delta > min)
    //         .collect::<Vec<AddressDelta>>();
    //     AddressDeltas { records }
    // }

    /// The `distance` function returns the distance between a point `self` and another point
    /// `other` in the same unit as `self`.
    pub fn distance(&self, other: &Address) -> f64 {
        ((self.address_latitude() - other.address_latitude()).powi(2)
            + (self.address_longitude() - other.address_longitude()).powi(2))
        .sqrt()
    }

    /// The `address_number` field represents the address number component of the complete address
    /// number.  This function returns the value of the field.
    pub fn address_number(&self) -> i64 {
        self.address_number
    }

    /// The `street_name` field represents the street name component of the complete street name.
    /// This function returns the cloned value of the field.
    pub fn street_name(&self) -> String {
        self.street_name.to_owned()
    }

    /// The `set_street_name` field sets the street name component of the complete street name.
    pub fn set_street_name(&mut self, name: &str) {
        self.street_name = name.to_owned();
    }

    /// The `pre_directional` field represents the street name predirectional component of the
    /// complete street name.  This function returns the cloned value of the field.
    pub fn pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    /// The `pre_directional` field represents the street name predirectional component of the
    /// complete street name.  This function returns the cloned value of the field.
    pub fn pre_directional_abbreviated(&self) -> Option<String> {
        match self.street_name_pre_directional {
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

    /// The `post_type` field represents the street name posttype component of the complete street
    /// name.  This function returns the cloned value of the field.
    pub fn post_type(&self) -> StreetNamePostType {
        self.street_name_post_type
    }

    /// The `set post_type` field sets the street name posttype component of the complete street
    /// name.
    pub fn set_post_type(&mut self, value: &StreetNamePostType) {
        self.street_name_post_type = value.to_owned();
    }

    /// The `subaddress_identifier` field represents the subaddress identifier component of the complete
    /// subaddress.  This function returns the cloned value of the field.
    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.to_owned()
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
        self.zip_code
    }

    /// The `status` field represents the status of an address.  This function returns the cloned
    /// value of the field.
    pub fn status(&self) -> AddressStatus {
        self.status
    }

    /// The `state_name` field contains the postal code for the State in an address.  This
    /// function returns the cloned value of the field.
    pub fn state_name(&self) -> String {
        self.state_name.to_owned()
    }

    /// The `postal_community` field represents the incorporated municipality or unincorporated
    /// community that serves as the postal community (the "City" field in an address).  This
    /// function returns the cloned value of the field.
    pub fn postal_community(&self) -> String {
        self.postal_community.to_owned()
    }

    /// The `address_latitude` field represents the latitude of the address location.  The spatial
    /// representation of the values depend upon how the data was exported from GIS.  This function
    /// returns the value of the field.
    pub fn address_latitude(&self) -> f64 {
        self.address_latitude
    }

    /// The `address_longitude` field represents the longitude of the address location.  The spatial
    /// representation of the values depend upon how the data was exported from GIS.  This function
    /// returns the value of the field.
    pub fn address_longitude(&self) -> f64 {
        self.address_longitude
    }
}

// impl From<CityAddress> for Address {
//     fn from(item: CityAddress) -> Self {
//         Address {
//             address_number: item.address_number(),
//             address_number_suffix: item.address_number_suffix(),
//             street_name_pre_directional: item.street_name_pre_directional(),
//             street_name: item.street_name(),
//             street_name_post_type: item.street_name_post_type(),
//             subaddress_type: item.subaddress_type(),
//             subaddress_identifier: item.subaddress_identifier(),
//             floor: item.floor(),
//             building: item.building(),
//             zip_code: item.zip_code(),
//             postal_community: item.postal_community(),
//             state_name: item.state_name(),
//             status: item.status(),
//             address_latitude: item.address_latitude(),
//             address_longitude: item.address_longitude(),
//         }
//     }
// }
//
// impl From<&CityAddress> for Address {
//     fn from(item: &CityAddress) -> Self {
//         Address {
//             address_number: item.address_number(),
//             address_number_suffix: item.address_number_suffix(),
//             street_name_pre_directional: item.street_name_pre_directional(),
//             street_name: item.street_name(),
//             street_name_post_type: item.street_name_post_type(),
//             subaddress_type: item.subaddress_type(),
//             subaddress_identifier: item.subaddress_identifier(),
//             floor: item.floor(),
//             building: item.building(),
//             zip_code: item.zip_code(),
//             postal_community: item.postal_community(),
//             state_name: item.state_name(),
//             status: item.status(),
//             address_latitude: item.address_latitude(),
//             address_longitude: item.address_longitude(),
//         }
//     }
// }

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
                address_latitude: item.address_latitude(),
                address_longitude: item.address_longitude(),
            }),
            None => Err(()),
        }
    }
}

pub trait Addreses<T: Addres + Clone + Serialize + IntoParallelIterator> {
    fn records(&self) -> &Vec<T>;

    /// The `filter_field` method returns the subset of addresses where the field `filter` is equal
    /// to the value in `field`.
    fn filter_field(&self, filter: &str, field: &str) -> Vec<T> {
        let mut records = Vec::new();
        match filter {
            "label" => records.append(
                &mut self
                    .records()
                    .iter()
                    .cloned()
                    .filter(|record| field == record.label())
                    .collect(),
            ),
            "street_name" => records.append(
                &mut self
                    .records()
                    .iter()
                    .cloned()
                    .filter(|record| field == record.street_name())
                    .collect(),
            ),
            "pre_directional" => records.append(
                &mut self
                    .records()
                    .iter()
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
                &mut self
                    .records()
                    .iter()
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
    fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        utils::to_csv(&mut self.records().clone(), title)?;
        Ok(())
    }

    /// Filter elements of the `records` vector according to the argument specified in `filter`.
    /// Currently only the value "duplicate" is supported as an argument to `filter`, which only
    /// retains duplicate addresses in the vector.  Duplicate addresses are defined as having
    /// identical address labels using the [`Address::label()`] method.
    fn filter(&self, filter: &str) -> Vec<T> {
        let mut records = Vec::new();
        match filter {
            "duplicate" => {
                let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Checking for duplicate addresses.'}",
        )
        .unwrap();
                let mut seen = HashSet::new();
                let bar = ProgressBar::new(self.records().len() as u64);
                bar.set_style(style);
                for address in self.records() {
                    let label = address.label();
                    if !seen.contains(&label) {
                        seen.insert(label.clone());
                        let mut same = self.filter_field("label", &label);
                        if same.len() > 1 {
                            records.append(&mut same);
                        }
                    }
                    bar.inc(1);
                }
            }
            _ => error!("Invalid filter provided."),
        }
        records
    }
}

pub trait Points<T: Point + Addres + Clone + Serialize> {
    fn records(&self) -> &Vec<T>;

    // /// Distance between addresses and other addresses with matching label.
    // /// Iterates through records of `others`, calculates the distance from self
    // /// to matching addresses in others, collects the results into a vector and
    // /// returns the results in the records field of a new `AddressDeltas` struct. Calls
    // /// [`Address::deltas()`].
    // fn deltas<U: Points<T> + Addreses<T>>(&self, other: &U, min: f64) -> AddressDeltas {
    //     let style = indicatif::ProgressStyle::with_template(
    //         "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Calculating deltas...'}",
    //     )
    //     .unwrap();
    //     let records_raw = self
    //         .records()
    //         .iter()
    //         // .progress_with_style(style)
    //         .map(|v| v.deltas(other, min))
    //         .collect::<Vec<AddressDeltas>>();
    //     let mut records = Vec::new();
    //     records_raw
    //         .iter()
    //         .map(|v| records.append(&mut v.records()))
    //         .for_each(drop);
    //     AddressDeltas { records }
    // }
}

/// The `Addresses` struct holds a vector of type [`Address`].
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Addresses {
    records: Vec<Address>,
}

impl Addresses {
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
        Addresses { records }
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
                    .filter(|record| field == record.street_name())
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
                    .filter(|record| field == format!("{:?}", record.post_type()))
                    .collect(),
            ),

            _ => info!("Invalid filter provided."),
        }
        Addresses { records }
    }

    /// Writes the contents of `Addresses` to a CSV file output to path `title`.  Each element
    /// of the vector in `records` writes to a row on the CSV file.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        utils::to_csv(&mut self.records(), title)?;
        Ok(())
    }

    // /// Distance between addresses and other addresses with matching label.
    // /// Iterates through records of `others`, calculates the distance from self
    // /// to matching addresses in others, collects the results into a vector and
    // /// returns the results in the records field of a new `AddressDeltas` struct. Calls
    // /// [`Address::deltas()`].
    // pub fn deltas(&self, other: &Addresses, min: f64) -> AddressDeltas {
    //     let style = indicatif::ProgressStyle::with_template(
    //         "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Calculating deltas...'}",
    //     )
    //     .unwrap();
    //     let records_raw = self
    //         .records_ref()
    //         .par_iter()
    //         .progress_with_style(style)
    //         .map(|v| v.deltas(other, min))
    //         .collect::<Vec<AddressDeltas>>();
    //     let mut records = Vec::new();
    //     records_raw
    //         .iter()
    //         .map(|v| records.append(&mut v.records()))
    //         .for_each(drop);
    //     AddressDeltas { records }
    // }

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
    pub fn orphan_streets(&self, other: &Addresses) -> Vec<String> {
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
        Addresses { records }
    }

    /// The `records` field hold a vector of type [`Address`].  This function returns the cloned
    /// value of the field.
    pub fn records(&self) -> Vec<Address> {
        self.records.to_owned()
    }

    /// This function returns a reference to the vector in the `records` field.
    pub fn records_ref(&self) -> &Vec<Address> {
        &self.records
    }
}

// impl From<CityAddresses> for Addresses {
//     fn from(item: CityAddresses) -> Self {
//         let mut records = Vec::new();
//         for address in item.records {
//             if let Ok(record) = Address::try_from(address) {
//                 records.push(record);
//             }
//         }
//         Addresses { records }
//     }
// }

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

/// The `PartialAddress` struct contains optional fields so that incomplete or missing data can be
/// compared against [`Addresses`] or [`PartialAddresses`] for potential matches.  Used to help
/// match address information that does not parse into a full valid address.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
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
    label: String,
    /// Distance between points representing the same address.
    delta: f64,
    latitude: f64,
    longitude: f64,
}

impl AddressDelta {
    /// Initiates a new `AddressDelta` struct from the provided input values.
    pub fn new<T: Addres + Point>(address: &T, delta: f64) -> Self {
        AddressDelta {
            label: address.label(),
            delta,
            latitude: address.lat(),
            longitude: address.lon(),
        }
    }
}

/// The `AddressDeltas` struct holds a `records` field that contains a vector of type
/// [`AddressDelta`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddressDeltas {
    records: Vec<AddressDelta>,
}

impl AddressDeltas {
    /// Writes the contents of `AddressDeltas` to a CSV file output to path `title`.  Each element
    /// of the vector in `records` writes to a row on the CSV file.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        utils::to_csv(&mut self.records(), title)?;
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
