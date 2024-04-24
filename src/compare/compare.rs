use crate::prelude::*;
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

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Mismatches {
    pub fields: Vec<Mismatch>,
}

impl Mismatches {
    pub fn new(fields: Vec<Mismatch>) -> Self {
        Mismatches { fields }
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct AddressMatch {
    pub coincident: bool,
    pub mismatches: Option<Mismatches>,
}

impl AddressMatch {
    pub fn new(coincident: bool, fields: Vec<Mismatch>) -> Self {
        let mismatches = match fields.len() {
            0 => None,
            _ => Some(Mismatches::new(fields)),
        };
        AddressMatch {
            coincident,
            mismatches,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum MatchStatus {
    Matching,
    Divergent,
    #[default]
    Missing,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MatchRecord {
    pub match_status: MatchStatus,
    pub address_label: String,
    pub subaddress_type: Option<String>,
    pub floor: Option<String>,
    pub building: Option<String>,
    pub status: Option<String>,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MatchRecords {
    pub records: Vec<MatchRecord>,
}

impl MatchRecords {
    pub fn new<T: Address + GeoPoint, U: Address + GeoPoint>(
        self_address: &T,
        other_addresses: &[U],
    ) -> Self {
        let address_label = self_address.label();
        let latitude = self_address.lat();
        let longitude = self_address.lon();

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
                    }),
                    Some(mismatches) => {
                        for mismatch in mismatches.fields {
                            match mismatch {
                                Mismatch::SubaddressType(message) => {
                                    subaddress_type = Some(message)
                                }
                                Mismatch::Floor(message) => floor = Some(message),
                                Mismatch::Building(message) => building = Some(message),
                                Mismatch::Status(message) => status = Some(message),
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
            })
        }
        MatchRecords {
            records: match_record,
        }
    }

    pub fn compare<T: Address + GeoPoint + Send + Sync, U: Address + GeoPoint + Send + Sync>(
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
            records.append(&mut item.records);
        }
        MatchRecords { records }
    }

    pub fn filter(self, filter: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "matching" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Matching)
                    .collect(),
            ),
            "missing" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Missing)
                    .collect(),
            ),
            "divergent" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Divergent)
                    .collect(),
            ),
            "subaddress" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        record.match_status == MatchStatus::Divergent
                            && record.subaddress_type.is_some()
                    })
                    .collect(),
            ),
            "floor" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        record.match_status == MatchStatus::Divergent && record.floor.is_some()
                    })
                    .collect(),
            ),
            "building" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        record.match_status == MatchStatus::Divergent && record.building.is_some()
                    })
                    .collect(),
            ),
            "status" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| {
                        record.match_status == MatchStatus::Divergent && record.status.is_some()
                    })
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        MatchRecords { records }
    }

    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(self.records_mut(), title)?;
        Ok(())
    }

    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(MatchRecords { records })
    }

    pub fn records_ref(&self) -> &Vec<MatchRecord> {
        &self.records
    }

    pub fn records_mut(&mut self) -> &mut Vec<MatchRecord> {
        &mut self.records
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MatchPartialRecord {
    match_status: MatchStatus,
    address_label: String,
    other_label: Option<String>,
    longitude: Option<f64>,
    latitude: Option<f64>,
}

impl MatchPartialRecord {
    pub fn coincident<T: Address + GeoPoint>(
        partial: &PartialAddress,
        address: &T,
    ) -> Option<MatchPartialRecord> {
        let mut match_status = MatchStatus::Missing;

        if let Some(value) = partial.address_number {
            if value == address.number() {
                match_status = MatchStatus::Matching;
            }
        }

        if &partial.street_name_pre_directional != address.directional()
            && match_status == MatchStatus::Matching
        {
            match_status = MatchStatus::Missing;
        }

        if let Some(value) = &partial.street_name {
            if value != address.street_name() && match_status == MatchStatus::Matching {
                match_status = MatchStatus::Missing;
            }
        }

        if let Some(value) = partial.street_name_post_type() {
            if let &Some(street_type) = address.street_type() {
                if value != street_type && match_status == MatchStatus::Matching {
                    match_status = MatchStatus::Missing;
                }
            }
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
                longitude: Some(address.lon()),
                latitude: Some(address.lat()),
            })
        } else {
            None
        }
    }

    pub fn compare<T: Address + GeoPoint>(
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
        let compared = MatchPartialRecords { records };
        let matching = compared.clone().filter("matching");
        if matching.records().is_empty() {
            compared
        } else {
            matching
        }
    }

    pub fn match_status(&self) -> MatchStatus {
        self.match_status.to_owned()
    }

    pub fn address_label(&self) -> String {
        self.address_label.to_owned()
    }

    pub fn other_label(&self) -> Option<String> {
        self.other_label.clone()
    }

    pub fn longitude(&self) -> Option<f64> {
        self.longitude
    }

    pub fn latitude(&self) -> Option<f64> {
        self.latitude
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MatchPartialRecords {
    records: Vec<MatchPartialRecord>,
}

impl MatchPartialRecords {
    pub fn compare<T: Address + GeoPoint + Send + Sync>(
        self_addresses: &PartialAddresses,
        other_addresses: &[T],
    ) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = self_addresses
            .records()
            .par_iter()
            .map(|address| MatchPartialRecord::compare(address, other_addresses))
            .progress_with_style(style)
            .collect::<Vec<MatchPartialRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item.records);
        }
        MatchPartialRecords { records }
    }

    pub fn filter(self, filter: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "missing" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Missing)
                    .collect(),
            ),
            "divergent" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Divergent)
                    .collect(),
            ),
            "matching" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Matching)
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        MatchPartialRecords { records }
    }

    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(&mut self.records(), title)?;
        Ok(())
    }

    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut records = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: MatchPartialRecord = result?;
            records.push(record);
        }

        Ok(MatchPartialRecords { records })
    }

    pub fn records(&self) -> Vec<MatchPartialRecord> {
        self.records.clone()
    }
}
