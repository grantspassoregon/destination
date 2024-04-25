use crate::prelude::*;
use galileo_types::geo::GeoPoint;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize)]
pub struct FireInspectionMatch {
    inspection: FireInspection,
    record: MatchPartialRecords,
}

impl FireInspectionMatch {
    pub fn compare<T: Address + GeoPoint<Num = f64>>(inspection: &FireInspection, addresses: &[T]) -> Self {
        let record = MatchPartialRecord::compare(&inspection.address(), addresses);
        FireInspectionMatch {
            inspection: inspection.clone(),
            record,
        }
    }

    pub fn inspection(&self) -> FireInspection {
        self.inspection.to_owned()
    }

    pub fn record(&self) -> MatchPartialRecords {
        self.record.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct FireInspectionMatches {
    records: Vec<FireInspectionMatch>,
}

impl FireInspectionMatches {
    pub fn compare<T: Address + GeoPoint<Num = f64> + Send + Sync>(inspections: &FireInspections, addresses: &[T]) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let records = inspections
            .records()
            .par_iter()
            .map(|r| FireInspectionMatch::compare(r, addresses))
            .progress_with_style(style)
            .collect::<Vec<FireInspectionMatch>>();
        FireInspectionMatches { records }
    }

    pub fn filter(self, filter: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "missing" => records.append(
                &mut self
                    .records()
                    .par_iter()
                    .cloned()
                    .filter(|r| r.record().records()[0].match_status() == MatchStatus::Missing)
                    .collect(),
            ),
            "divergent" => records.append(
                &mut self
                    .records()
                    .par_iter()
                    .cloned()
                    .filter(|r| r.record().records()[0].match_status() == MatchStatus::Divergent)
                    .collect(),
            ),
            "matching" => records.append(
                &mut self
                    .records()
                    .par_iter()
                    .cloned()
                    .filter(|r| r.record().records()[0].match_status() == MatchStatus::Matching)
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        FireInspectionMatches { records }
    }

    pub fn records(&self) -> Vec<FireInspectionMatch> {
        self.records.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireInspectionMatchRecord {
    status: MatchStatus,
    name: String,
    address_label: String,
    other_label: Option<String>,
    longitude: Option<f64>,
    latitude: Option<f64>,
}

impl FireInspectionMatchRecord {
    pub fn status(&self) -> MatchStatus {
        self.status.to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireInspectionMatchRecords {
    records: Vec<FireInspectionMatchRecord>,
}

impl FireInspectionMatchRecords {
    pub fn records(&self) -> Vec<FireInspectionMatchRecord> {
        self.records.to_owned()
    }

    pub fn records_mut(&mut self) -> &mut Vec<FireInspectionMatchRecord> {
        &mut self.records
    }

    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(self.records_mut(), title)?;
        Ok(())
    }

    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(FireInspectionMatchRecords { records })
    }

    pub fn filter(&self, filter: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "missing" => records.append(
                &mut self
                    .records()
                    .par_iter()
                    .cloned()
                    .filter(|r| r.status() == MatchStatus::Missing)
                    .collect(),
            ),
            "divergent" => records.append(
                &mut self
                    .records()
                    .par_iter()
                    .cloned()
                    .filter(|r| r.status() == MatchStatus::Divergent)
                    .collect(),
            ),
            "matching" => records.append(
                &mut self
                    .records()
                    .par_iter()
                    .cloned()
                    .filter(|r| r.status() == MatchStatus::Matching)
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        FireInspectionMatchRecords { records }
    }
}

impl From<&FireInspectionMatch> for FireInspectionMatchRecords {
    fn from(inspection: &FireInspectionMatch) -> Self {
        let mut records = Vec::new();
        let name = inspection.inspection().name();
        let address_label = inspection.inspection().address().label();
        for record in inspection.record().records() {
            records.push(FireInspectionMatchRecord {
                status: record.match_status(),
                name: name.to_owned(),
                address_label: address_label.to_owned(),
                other_label: record.other_label(),
                longitude: record.longitude(),
                latitude: record.latitude(),
            });
        }

        FireInspectionMatchRecords { records }
    }
}

impl From<&FireInspectionMatches> for FireInspectionMatchRecords {
    fn from(inspections: &FireInspectionMatches) -> Self {
        let mut records = Vec::new();
        for record in inspections.records() {
            let matches = FireInspectionMatchRecords::from(&record);
            for item in matches.records() {
                records.push(item);
            }
        }
        FireInspectionMatchRecords { records }
    }
}
