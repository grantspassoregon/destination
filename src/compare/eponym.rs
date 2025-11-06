//! The `eponym` module is the eponymous module for `compare`.  Contains types and methods for
//! comparing addresses.
use crate::{
    Address, AddressErrorKind, AddressStatus, Geographic, IntoCsv, Io, PartialAddress,
    PartialAddresses, SubaddressType, from_csv, to_csv,
};
use derive_more::{Deref, DerefMut};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

/// The `Mismatch` enum tracks the fields of an address that can diverge while still potentially
/// referring to the same location.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum Mismatch {
    /// Represents a mismatch in the subaddress type.
    SubaddressType(String),
    /// Represents a mismatch in the floor number.
    Floor(String),
    /// Represents a mismatch in the building identifier.
    Building(String),
    /// Represents a mismatch in the address status.
    Status(String),
}

impl Mismatch {
    /// The `subaddress_type` method captures information about the mismatch between subaddress
    /// type fields as a message contained in the enum variant.
    pub fn subaddress_type(from: Option<SubaddressType>, to: Option<SubaddressType>) -> Self {
        let message = format!("{:?} not equal to {:?}", from, to);
        Self::SubaddressType(message)
    }

    /// The `floor` method captures information about the mismatch between the `floor` fields as a message contained in the enum variant.
    pub fn floor(from: Option<i64>, to: Option<i64>) -> Self {
        let message = format!("{:?} not equal to {:?}", from, to);
        Self::Floor(message)
    }

    /// The `building` method captures information about the mismatch between the `building` fields as a message contained in the enum variant.
    pub fn building(from: Option<String>, to: Option<String>) -> Self {
        let message = format!("{:?} not equal to {:?}", from, to);
        Self::Building(message)
    }

    /// The `status` method captures information about the mismatch between the `status` fields as a message contained in the enum variant.
    pub fn status(from: AddressStatus, to: AddressStatus) -> Self {
        let message = format!("{} not equal to {}", from, to);
        Self::Status(message)
    }
}

/// The `Mismatches` struct holds a vector of type [`Mismatch`].
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Deref,
    DerefMut,
)]
pub struct Mismatches(Vec<Mismatch>);

impl Mismatches {
    /// Creates a new 'Mismatches' from a vector of type ['Mismatch'].
    pub fn new(fields: Vec<Mismatch>) -> Self {
        Mismatches(fields)
    }
}

/// The `AddressMatch` is an intermediary data structure used internally to aggregate match information from
/// comparing types that implement [`crate::Addresses`], for the purpose of producing [`MatchRecords`].
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct AddressMatch {
    /// The `coincident` field indicates the compared addresses refer to the same location, or are
    /// coincidental.
    pub coincident: bool,
    /// The `mismatches` field holds [`Mismatch`] information for each field that differs between
    /// the compared addresses.  If no coincident address is present, this field is `None`.
    pub mismatches: Option<Mismatches>,
}

impl AddressMatch {
    /// Constructor for creating an `AddressMatch` from its constituent fields.
    pub fn new(coincident: bool, fields: Vec<Mismatch>) -> Self {
        let mismatches = if fields.is_empty() {
            None
        } else {
            Some(Mismatches::new(fields))
        };
        AddressMatch {
            coincident,
            mismatches,
        }
    }
}

/// The `MatchStatus` enum delineates whether a given address has a match (the `Matching` variant),
/// has a match but differs in some descriptive fields (the `Divergent` variant), or does not have
/// a match in the comparison set (the `Missing` variant).
///
/// We have derived Default using the Missing variant, mostly so structs that take a `MatchStatus`
/// as a field can also derive default.  Properly speaking, there is no meaningful default for this
/// struct, but if you need to create one first and fill it in later, you can.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum MatchStatus {
    /// The `Matching` variant indicates an address has an exact match in the comparison set.
    Matching,
    /// The `Divergent` variant indicates an address has a match in the comparison set, but
    /// the address contains fields with different values than in the comparison (e.g. the
    /// address has status 'Retired' compared to 'Current').
    Divergent,
    #[default]
    /// The `Missing` variant indicates the address does not have a match in the comparison set.
    Missing,
}

/// A `MatchRecord` reports the match results for a single address compared against a set of
/// addresses.  Designed to plot and diagnose missing and divergent addresses.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MatchRecord {
    /// The `match_status` field represents the match status of the address.
    pub match_status: MatchStatus,
    /// The `address_label` field is the text representation of the subject address.
    pub address_label: String,
    /// The `subaddress_type` field indicates a difference in subaddress type between a subject
    /// address and its match, if present.  E.g. "SUITE" does not match "APARTMENT".
    pub subaddress_type: Option<String>,
    /// The `floor` field indicates the subject address and its match, if present, have different floor numbers.
    pub floor: Option<String>,
    /// The `building` field indicates the subject address and its match, if present, have
    /// different building identifiers.
    pub building: Option<String>,
    /// The `status` field indicates the subject address and its match, if present, have different
    /// values for the address status. E.g. "Current" does not match "Other".
    pub status: Option<String>,
    /// The `longitude` field represents the 'x' value of the address point.  Depending on the
    /// input from the caller, the value may be in decimal degrees, meters or feet.
    pub longitude: f64,
    /// The `latitude` field represents the 'y' value of the address point.  Depending on the
    /// input from the caller, the value may be in decimal degrees, meters or feet.
    pub latitude: f64,
    /// The `id` field is an internal unique id.
    pub id: uuid::Uuid,
}

impl Geographic for MatchRecord {
    fn latitude(&self) -> f64 {
        self.latitude
    }

    fn longitude(&self) -> f64 {
        self.longitude
    }
}

/// The `MatchRecords` struct holds a vector of type [`MatchRecord`].
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Deref, DerefMut)]
pub struct MatchRecords(Vec<MatchRecord>);

impl MatchRecords {
    /// The constructor for `MatchRecords` compares a single subject address against a set of
    /// addresses, and returns the `MatchRecords` for the subject address.  A subject address can
    /// match against multiple candidates (e.g. a parent address will match against all
    /// subaddresses associated with the parent), so the result type must potentially accommodate
    /// multiple records.
    pub fn new<T: Address + Geographic, U: Address + Geographic>(
        self_address: &T,
        other_addresses: &[U],
    ) -> Self {
        let address_label = self_address.label();
        let latitude = self_address.latitude();
        let longitude = self_address.longitude();
        let id = uuid::Uuid::new_v4();

        let mut match_record = Vec::new();

        for address in other_addresses {
            let address_match = self_address.coincident(address);
            if address_match.coincident {
                let mut subaddress_type = None;
                let mut floor = None;
                let mut building = None;
                let mut status = None;
                match address_match.mismatches {
                    None => match_record.push(MatchRecord {
                        match_status: MatchStatus::Matching,
                        address_label: address_label.clone(),
                        subaddress_type,
                        floor,
                        building,
                        status,
                        longitude,
                        latitude,
                        id,
                    }),
                    Some(mismatches) => {
                        for mismatch in mismatches.iter() {
                            match mismatch {
                                Mismatch::SubaddressType(message) => {
                                    subaddress_type = Some(message.to_owned())
                                }
                                Mismatch::Floor(message) => floor = Some(message.to_owned()),
                                Mismatch::Building(message) => building = Some(message.to_owned()),
                                Mismatch::Status(message) => status = Some(message.to_owned()),
                            }
                        }
                        match_record.push(MatchRecord {
                            match_status: MatchStatus::Divergent,
                            address_label: address_label.clone(),
                            subaddress_type,
                            floor,
                            building,
                            status,
                            longitude,
                            latitude,
                            id,
                        })
                    }
                }
            }
        }
        if match_record.is_empty() {
            match_record.push(MatchRecord {
                match_status: MatchStatus::Missing,
                address_label,
                subaddress_type: None,
                floor: None,
                building: None,
                status: None,
                longitude,
                latitude,
                id,
            })
        }
        MatchRecords(match_record)
    }

    /// For each address in `self_addresses`, the `compare` method calculates the match record for
    /// the subject address compared against the addresses in `other_addresses`, and returns the
    /// results in a [`MatchRecords`] struct.
    pub fn compare<T: Address + Geographic + Send + Sync, U: Address + Geographic + Send + Sync>(
        self_addresses: &[T],
        other_addresses: &[U],
    ) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = self_addresses
            .par_iter()
            .map(|address| MatchRecords::new(address, other_addresses))
            .progress_with_style(style)
            .collect::<Vec<MatchRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item);
        }
        MatchRecords(records)
    }

    /// The `filter` method returns the subset of `MatchRecords` that meet the filter requirement.
    /// The `filter` parameter takes a string reference that can take the values "matching",
    /// "missing", "divergent", "subaddress", "floor", "building" and "status".  When filtering by
    /// match status, the return records contain those records where the match status equals the
    /// filter value.  For the mismatch fields, the return records contain values where a mismatch
    /// is present in the provided field.
    pub fn filter(mut self, filter: &str) -> Self {
        match filter {
            "matching" => self.retain(|r| r.match_status == MatchStatus::Matching),
            "missing" => self.retain(|r| r.match_status == MatchStatus::Missing),
            "divergent" => self.retain(|r| r.match_status == MatchStatus::Divergent),
            "subaddress" => self.retain(|r| {
                r.match_status == MatchStatus::Divergent && r.subaddress_type.is_some()
            }),
            "floor" => {
                self.retain(|r| r.match_status == MatchStatus::Divergent && r.floor.is_some())
            }
            "building" => {
                self.retain(|r| r.match_status == MatchStatus::Divergent && r.building.is_some())
            }
            "status" => {
                self.retain(|r| r.match_status == MatchStatus::Divergent && r.status.is_some())
            }
            _ => info!("Invalid filter provided."),
        }
        self
    }
}

impl IntoCsv<MatchRecords> for MatchRecords {
    fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        to_csv(&mut self.0, path.as_ref().into())
    }
}

/// The `MatchPartialRecord` struct contains match data for a [`PartialAddress`].
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MatchPartialRecord {
    /// The `match_status` field represents the match status of the partial address.
    match_status: MatchStatus,
    /// The `address_label` field is the text representation of the partial address.
    address_label: String,
    /// The `other_label` field is the text representation of the matching address.
    other_label: Option<String>,
    /// The `longitude` field represents the 'x' value of the matching address, if present.
    longitude: Option<f64>,
    /// The `latitude` field represents the 'y' value of the matching address, if present.
    latitude: Option<f64>,
}

impl MatchPartialRecord {
    /// The `coincident` method attempts to match fields present in the partial address against the
    /// comparison address, returning a `MatchPartialRecord` if successful.  Returns `None` if
    /// the match status is "missing".
    pub fn coincident<T: Address + Geographic>(
        partial: &PartialAddress,
        address: &T,
    ) -> Option<MatchPartialRecord> {
        let mut match_status = MatchStatus::Missing;

        if let Some(value) = partial.address_number
            && value == address.number()
        {
            match_status = MatchStatus::Matching;
        }

        if &partial.street_name_pre_directional != address.directional()
            && match_status == MatchStatus::Matching
        {
            match_status = MatchStatus::Missing;
        }

        if let Some(value) = &partial.street_name
            && value != address.street_name()
            && match_status == MatchStatus::Matching
        {
            match_status = MatchStatus::Missing;
        }

        if let Some(value) = partial.street_name_post_type()
            && let &Some(street_type) = address.street_type()
            && value != street_type
            && match_status == MatchStatus::Matching
        {
            match_status = MatchStatus::Missing;
        }

        if &partial.subaddress_identifier() != address.subaddress_id()
            && match_status == MatchStatus::Matching
        {
            match_status = MatchStatus::Divergent;
        }

        if address.subaddress_id().is_none()
            && &partial.building() != address.building()
            && match_status == MatchStatus::Matching
        {
            match_status = MatchStatus::Divergent;
        }

        if address.subaddress_id().is_none()
            && address.building().is_none()
            && &partial.floor() != address.floor()
            && match_status == MatchStatus::Matching
        {
            match_status = MatchStatus::Divergent;
        }

        if match_status != MatchStatus::Missing {
            Some(MatchPartialRecord {
                match_status,
                address_label: partial.label(),
                other_label: Some(address.label()),
                longitude: Some(address.longitude()),
                latitude: Some(address.latitude()),
            })
        } else {
            None
        }
    }

    /// The `compare` method attempts to match fields present in the partial address against a set
    /// of comparison addresses, returning a [`MatchPartialRecords`].
    pub fn compare<T: Address + Geographic>(
        partial: &PartialAddress,
        addresses: &[T],
    ) -> MatchPartialRecords {
        let mut records = Vec::new();
        for address in addresses {
            let coincident = MatchPartialRecord::coincident(partial, address);
            if let Some(record) = coincident {
                records.push(record);
            }
        }
        if records.is_empty() {
            records.push(MatchPartialRecord {
                match_status: MatchStatus::Missing,
                address_label: partial.label(),
                other_label: None,
                longitude: None,
                latitude: None,
            })
        }
        let compared = MatchPartialRecords(records);
        let matching = compared.clone().filter("matching");
        if matching.is_empty() {
            compared
        } else {
            matching
        }
    }

    /// The `match_status` method returns the cloned value of the `match_status` field.
    pub fn match_status(&self) -> MatchStatus {
        self.match_status.to_owned()
    }

    /// The `address_label` method returns the cloned value of the `address_label` field.
    pub fn address_label(&self) -> String {
        self.address_label.to_owned()
    }

    /// The `other_label` method returns the cloned value of the `other_label` field.
    pub fn other_label(&self) -> Option<String> {
        self.other_label.clone()
    }

    /// The `longitude` method returns the value of the `longitude` field.
    pub fn longitude(&self) -> Option<f64> {
        self.longitude
    }

    /// The `latitude` method returns the value of the `latitude` field.
    pub fn latitude(&self) -> Option<f64> {
        self.latitude
    }
}

/// The `MatchPartialRecords` struct holds a vector of type [`MatchPartialRecord`].
#[derive(
    Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Deref, DerefMut, derive_new::new,
)]
pub struct MatchPartialRecords(Vec<MatchPartialRecord>);

impl MatchPartialRecords {
    /// For each partial address in `self_addresses`, the `compare` method attempts to match the
    /// fields present in the partial address against the addresses in `other_addresses`, returning
    /// a `MatchPartialRecords`.
    pub fn compare<T: Address + Geographic + Send + Sync>(
        self_addresses: &PartialAddresses,
        other_addresses: &[T],
    ) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = self_addresses
            .par_iter()
            .map(|address| MatchPartialRecord::compare(address, other_addresses))
            .progress_with_style(style)
            .collect::<Vec<MatchPartialRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item);
        }
        MatchPartialRecords(records)
    }

    /// The `filter` method returns the subset of `PartialMatchRecords` that meet the filter requirement.
    /// The `filter` parameter takes a string reference that can take the values "matching",
    /// "missing", or "divergent".  The return records contain those records where the match status equals the
    /// filter value.
    pub fn filter(mut self, filter: &str) -> Self {
        match filter {
            "missing" => self.retain(|r| r.match_status == MatchStatus::Missing),
            "divergent" => self.retain(|r| r.match_status == MatchStatus::Divergent),
            "matching" => self.retain(|r| r.match_status == MatchStatus::Matching),
            _ => info!("Invalid filter provided."),
        }
        self
    }
}

impl IntoCsv<MatchPartialRecords> for MatchPartialRecords {
    fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        to_csv(&mut self.0, path.as_ref().into())
    }
}
