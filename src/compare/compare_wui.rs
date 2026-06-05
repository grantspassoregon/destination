//!  The `compare_wui` module implements address matching and comparison for fire suppression
//!  applications in the WUI.
use crate::{
    Address, AddressErrorKind, Geographic, IntoCsv, Io, MatchPartialRecord, MatchPartialRecords,
    MatchStatus, Wui, Wuis, from_csv, to_csv,
};
use derive_getters::Getters;
use derive_more::{Deref, DerefMut};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

/// The `WuiMatch` struct holds a [`Wui`] in the `wui` field, and a
/// [`MatchPartialRecord`] in the `record` field.  The `record` matches the partial business
/// address against a set of fully-specified addresses.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Getters)]
pub struct WuiMatch {
    // The fire inspection record.
    wui: Wui,
    // The address match record for the provided business address.
    record: MatchPartialRecords,
}

impl WuiMatch {
    /// The `compare` method wraps [`MatchPartialRecord::compare`], taking the address
    /// from the ['Wui'] and comparing it against a set of fully-specified *addresses*.
    #[tracing::instrument(skip_all)]
    pub fn compare<T: Address + Geographic>(wui: &Wui, addresses: &[T]) -> Self {
        let record = MatchPartialRecord::compare(wui.address(), addresses);
        Self {
            wui: wui.clone(),
            record,
        }
    }
}

/// The `FireInspectionMatches` struct is a wrapper for a vector of type [`FireInspectionMatch`].
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Deref, DerefMut)]
pub struct WuiMatches(Vec<WuiMatch>);

impl WuiMatches {
    /// The `compare` method creates a [`WuiMatch`] for each applicant record.  
    /// Used to convert [`Wuis`] into a new instance of
    /// `WuisMatches`.
    pub fn compare<T: Address + Geographic + Send + Sync>(wuis: &Wuis, addresses: &[T]) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing WUI addresses.'}",
        )
        .unwrap();
        let records = wuis
            .par_iter()
            .map(|r| WuiMatch::compare(r, addresses))
            .progress_with_style(style)
            .collect::<Vec<WuiMatch>>();
        Self(records)
    }

    /// The `filter` method filters records from Self.  Currently accepts values "missing",
    /// "divergent" and "matching", which filter based on the match status [`MatchStatus`].
    pub fn filter(&mut self, filter: &str) {
        match filter {
            "missing" => self.retain(|r| r.record()[0].match_status() == MatchStatus::Missing),
            "divergent" => self.retain(|r| r.record()[0].match_status() == MatchStatus::Divergent),
            "matching" => self.retain(|r| r.record()[0].match_status() == MatchStatus::Matching),
            _ => info!("Invalid filter provided."),
        }
    }
}

/// The `WuiMatchRecord` struct holds a selection of fields from the fire suppression application
/// and the partial address match, designed to export to csv for visualization in GIS.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Getters)]
pub struct WuiMatchRecord {
    // The applicant name
    name: Option<String>,
    // The applicant phone number
    phone: Option<String>,
    // The applicant email
    email: Option<String>,
    // Date of application
    date: String,
    // Time of application
    time: i64,
    // The match status of the partial address
    status: MatchStatus,
    // The provided business address
    address_label: String,
    // The comparison address
    other_label: Option<String>,
    // Longitude of comparison address
    longitude: Option<f64>,
    // Latitude of comparison address
    latitude: Option<f64>,
}

/// The `WuiMatchRecords` struct is wrapper for a vector of type
/// [`FireInspectionMatchRecord`].
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Deref, DerefMut)]
pub struct WuiMatchRecords(Vec<WuiMatchRecord>);

impl WuiMatchRecords {
    /// The `filter` method returns the subset of records that match the filter.  Current values
    /// for the `filter` field include "missing", "divergent", "matching", which filter by address
    /// match status.
    pub fn filter(&mut self, filter: &str) {
        match filter {
            "missing" => self.retain(|r| *r.status() == MatchStatus::Missing),
            "divergent" => self.retain(|r| *r.status() == MatchStatus::Divergent),
            "matching" => self.retain(|r| *r.status() == MatchStatus::Matching),
            _ => info!("Invalid filter provided."),
        }
    }
}

impl IntoCsv<WuiMatchRecords> for WuiMatchRecords {
    fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        to_csv(&mut self.0, path.as_ref().into())
    }
}

impl From<&WuiMatch> for WuiMatchRecords {
    fn from(wui: &WuiMatch) -> Self {
        let mut records = Vec::new();
        let name = wui.wui().name();
        let phone = wui.wui().phone();
        let email = wui.wui().email();
        let date = wui.wui().date();
        let time = *wui.wui().time();
        let address_label = wui.wui().address().label();
        for record in wui.record().iter() {
            records.push(WuiMatchRecord {
                name: name.clone(),
                phone: phone.clone(),
                email: email.clone(),
                date: date.clone(),
                time,
                status: record.match_status(),
                address_label: address_label.to_owned(),
                other_label: record.other_label(),
                longitude: record.longitude(),
                latitude: record.latitude(),
            });
        }

        Self(records)
    }
}

impl From<&WuiMatches> for WuiMatchRecords {
    fn from(inspections: &WuiMatches) -> Self {
        let mut records = Vec::new();
        for record in inspections.iter() {
            let matches = WuiMatchRecords::from(record);
            for item in matches.iter() {
                records.push(item.clone());
            }
        }
        Self(records)
    }
}
