//!  The `compare_fire` module implements address matching and comparison for Fire Inspections.
use crate::{
    from_csv, to_csv, Address, FireInspection, FireInspections, MatchPartialRecord,
    MatchPartialRecords, MatchStatus,
};
use derive_more::{Deref, DerefMut};
use galileo::galileo_types::geo::GeoPoint;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

/// The `FireInspectionmatch` struct holds a [`FireInspection`] in the `inspection` field, and a
/// [`MatchPartialRecord`] in the `record` field.  The `record` matches the partial business
/// address against a set of fully-specified addresses.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FireInspectionMatch {
    // The fire inspection record.
    inspection: FireInspection,
    // The address match record for the provided business address.
    record: MatchPartialRecords,
}

impl FireInspectionMatch {
    /// The `compare` method wraps [`MatchPartialRecord::compare`], taking the business address
    /// from the fire inspection and comparing it against a set of fully-specified addresses.
    pub fn compare<T: Address + GeoPoint<Num = f64>>(
        inspection: &FireInspection,
        addresses: &[T],
    ) -> Self {
        let record = MatchPartialRecord::compare(&inspection.address(), addresses);
        FireInspectionMatch {
            inspection: inspection.clone(),
            record,
        }
    }

    /// The `inspection` method returns the cloned value of the `inspection` field, which contains
    /// the fire inspection record.
    pub fn inspection(&self) -> FireInspection {
        self.inspection.to_owned()
    }

    /// The `record` method returns the cloned value of the `record` field, which contains the
    /// address match record for the provided business address.
    pub fn record(&self) -> MatchPartialRecords {
        self.record.to_owned()
    }
}

/// The `FireInspectionMatches` struct is a wrapper for a vector of type [`FireInspectionMatch`].
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Deref, DerefMut)]
pub struct FireInspectionMatches(Vec<FireInspectionMatch>);

impl FireInspectionMatches {
    /// The `compare` method creates a [`FireInspectionMatch`] for each business address in the
    /// inspection record.  Used to convert [`FireInspections`] into a new instance of
    /// `FireInspectionMatches`.
    pub fn compare<T: Address + GeoPoint<Num = f64> + Send + Sync>(
        inspections: &FireInspections,
        addresses: &[T],
    ) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let records = inspections
            .par_iter()
            .map(|r| FireInspectionMatch::compare(r, addresses))
            .progress_with_style(style)
            .collect::<Vec<FireInspectionMatch>>();
        FireInspectionMatches(records)
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

/// The `FireInspectionMatchRecord` struct holds a selection of fields from the fire inspection
/// record and the partial address match, designed to export to csv for visualization in GIS.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FireInspectionMatchRecord {
    // The match status of the partial address.
    status: MatchStatus,
    // The business name.
    name: String,
    // The provided business address.
    address_label: String,
    // The comparison address.
    other_label: Option<String>,
    // Longitude of comparison address.
    longitude: Option<f64>,
    // Latitude of comparison address.
    latitude: Option<f64>,
}

impl FireInspectionMatchRecord {
    /// The `status` method returns the cloned value of the `status` field, containing the match
    /// status of the address.
    pub fn status(&self) -> MatchStatus {
        self.status.to_owned()
    }
}

/// The `FireInspectionMatchRecords` struct is wrapper for a vector of type
/// [`FireInspectionMatchRecord`].
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Deref, DerefMut)]
pub struct FireInspectionMatchRecords(Vec<FireInspectionMatchRecord>);

impl FireInspectionMatchRecords {
    /// Writes the contents of the struct to a csv file at location `title`.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(self, title)?;
        Ok(())
    }

    /// Reads the contents of the struct from a csv file at location `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(FireInspectionMatchRecords(records))
    }

    /// The `filter` method returns the subset of records that match the filter.  Current values
    /// for the `filter` field include "missing", "divergent", "matching", which filter by address
    /// match status.
    pub fn filter(&mut self, filter: &str) {
        match filter {
            "missing" => self.retain(|r| r.status() == MatchStatus::Missing),
            "divergent" => self.retain(|r| r.status() == MatchStatus::Divergent),
            "matching" => self.retain(|r| r.status() == MatchStatus::Matching),
            _ => info!("Invalid filter provided."),
        }
    }
}

impl From<&FireInspectionMatch> for FireInspectionMatchRecords {
    fn from(inspection: &FireInspectionMatch) -> Self {
        let mut records = Vec::new();
        let name = inspection.inspection().name();
        let address_label = inspection.inspection().address().label();
        for record in inspection.record().iter() {
            records.push(FireInspectionMatchRecord {
                status: record.match_status(),
                name: name.to_owned(),
                address_label: address_label.to_owned(),
                other_label: record.other_label(),
                longitude: record.longitude(),
                latitude: record.latitude(),
            });
        }

        FireInspectionMatchRecords(records)
    }
}

impl From<&FireInspectionMatches> for FireInspectionMatchRecords {
    fn from(inspections: &FireInspectionMatches) -> Self {
        let mut records = Vec::new();
        for record in inspections.iter() {
            let matches = FireInspectionMatchRecords::from(record);
            for item in matches.iter() {
                records.push(item.clone());
            }
        }
        FireInspectionMatchRecords(records)
    }
}
