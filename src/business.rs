//! The `business` module matches addresses associated with business licenses against a set of known [`Addresses`], producing a record of
//! matching, divergent and missing addresses.
use crate::prelude::*;
use galileo_types::geo::GeoPoint;
use indicatif::ParallelProgressIterator;
use num_traits::cast::FromPrimitive;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::info;

/// The `BusinessMatchRecord` struct holds match data for a licensed business.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMatchRecord {
    match_status: MatchStatus,
    business_address_label: String,
    company_name: Option<String>,
    contact_name: Option<String>,
    business_type: String,
    dba: Option<String>,
    license: String,
    expires: String,
    other_address_label: Option<String>,
    address_latitude: Option<f64>,
    address_longitude: Option<f64>,
}

impl BusinessMatchRecord {
    /// The `company_name` field represents the registered business name associated with the
    /// active business license.
    pub fn company_name(&self) -> Option<String> {
        self.company_name.clone()
    }
}

/// The `BusinessMatchRecords` struct holds a vector of [`BusinessMatchRecord`] objects.
#[derive(Clone)]
pub struct BusinessMatchRecords {
    /// The `records` field holds a vector of [`BusinessMatchRecord`] objects.
    pub records: Vec<BusinessMatchRecord>,
}

impl BusinessMatchRecords {
    /// Matches the provided address associated with a business license against the addresses in
    /// `addresses`, creating a new `BusinessMatchRecords` struct containing the results.
    pub fn new<T: Address + GeoPoint<Num = f64>>(business: &BusinessLicense, addresses: &[T]) -> Self {
        let mut records = Vec::new();
        for address in addresses {
            let business_match = business.coincident(address);
            if let Some(record) = business_match {
                records.push(record);
            }
        }
        if records.is_empty() {
            records.push(BusinessMatchRecord {
                match_status: MatchStatus::Missing,
                business_address_label: business.label(),
                company_name: business.company_name(),
                contact_name: business.contact_name(),
                business_type: business.business_type(),
                dba: business.dba(),
                license: business.license(),
                expires: business.expires(),
                other_address_label: None,
                address_latitude: None,
                address_longitude: None,
            });
        }
        let business_record = BusinessMatchRecords { records };
        let matched = business_record.filter("matching");
        let divergent = business_record.filter("divergent");
        if !matched.records.is_empty() {
            let trim_match = matched.records[0].clone();
            BusinessMatchRecords {
                records: vec![trim_match],
            }
        } else if !divergent.records.is_empty() {
            divergent
        } else {
            business_record
        }
    }

    /// This method compares the address of `business` against the addresses in `address_list`,
    /// returning a `BusinessMatchRecords` struct containing an exact match if found, otherwise a
    /// list of partial (divergent) matches if found, otherwise a missing record.  Since divergent
    /// addresses are unnecessary to inspect if an exact match is found, this is a more efficient
    /// matching method compared to [`BusinessMatchRecords::compare()`].
    pub fn chain<T: Address + GeoPoint<Num = f64>>(business: &BusinessLicense, address_list: &[&[T]]) -> Self {
        let mut matching = Vec::new();
        let mut divergent = Vec::new();
        let mut missing = Vec::new();
        for addresses in address_list {
            let record = BusinessMatchRecords::new(business, addresses);
            let matched = record.filter("matching");
            let diverged = record.filter("divergent");
            let missed = record.filter("missing");
            if !matched.records.is_empty() && matching.is_empty() {
                matching = matched.records;
            } else if !diverged.records.is_empty() && divergent.is_empty() {
                divergent = diverged.records;
            } else if !missed.records.is_empty() && missing.is_empty() {
                missing = missed.records;
            }
        }
        if !matching.is_empty() {
            BusinessMatchRecords { records: matching }
        } else if !divergent.is_empty() {
            BusinessMatchRecords { records: divergent }
        } else {
            BusinessMatchRecords { records: missing }
        }
    }

    /// For each [`BusinessLicense`] object in `businesses`, this method creates a
    /// `BusinessMatchRecords` using the [`BusinessMatchRecords::new()`] method.  Match records
    /// will include matching, divergent and missing records.
    pub fn compare<T: Address + GeoPoint<Num = f64> + Send + Sync>(businesses: &BusinessLicenses, addresses: &[T]) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = businesses
            .records
            .par_iter()
            .map(|address| BusinessMatchRecords::new(address, addresses))
            .progress_with_style(style)
            .collect::<Vec<BusinessMatchRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item.records);
        }
        BusinessMatchRecords { records }
    }

    /// Compares each address in `businesses` against the addresses in `addresses` using the
    /// [`BusinessMatchRecords::chain()`] method, which returns only an exact match if available,
    /// otherwise returning a list of partial matches or a missing record.
    pub fn compare_chain<T: Address + GeoPoint<Num = f64> + Send + Sync>(businesses: &BusinessLicenses, addresses: &[&[T]]) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = businesses
            .records
            .par_iter()
            .map(|address| BusinessMatchRecords::chain(address, addresses))
            .progress_with_style(style)
            .collect::<Vec<BusinessMatchRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item.records);
        }
        BusinessMatchRecords { records }
    }

    /// The `filter` method filters the [`BusinessMatchRecord`] objects in the `records` field
    /// based upon the match status of the record.  The `filter` field accepts the values
    /// "missing", "nonmissing", "divergent", "matching", "unique" and "multiple". The "unique"
    /// option returns records where the business name is unique.  The "multiple" options returns
    /// records where multiple licenses exist registered under the same business name.
    pub fn filter(&self, filter: &str) -> Self {
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
            "nonmissing" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status != MatchStatus::Missing)
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
            "unique" => {
                let mut names = HashSet::new();
                for record in self.records.clone() {
                    if let Some(name) = record.company_name() {
                        if !names.contains(&name) {
                            names.insert(name.clone());
                            let subset = self.filter_field("name", &name);
                            if subset.records.len() == 1 {
                                records.push(subset.records[0].clone());
                            }
                        }
                    }
                }
            }
            "multiple" => {
                let mut names = HashSet::new();
                for record in self.records.clone() {
                    if let Some(name) = record.company_name() {
                        if !names.contains(&name) {
                            names.insert(name.clone());
                            let mut subset = self.filter_field("name", &name);
                            if subset.records.len() > 1 {
                                records.append(&mut subset.records);
                            }
                        }
                    }
                }
            }
            _ => info!("Invalid filter provided."),
        }
        BusinessMatchRecords { records }
    }

    /// The `filter_field` method filters [`BusinessMatchRecord`] objects in the `records` field
    /// by comparing the value of the field specified in `filter` to the value of `field`.  The
    /// `filter` field accepts the value "name", and matches the value of `field` against the company
    /// name associated with the record.
    pub fn filter_field(&self, filter: &str, field: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "name" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.company_name() == Some(field.to_string()))
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        BusinessMatchRecords { records }
    }

    /// Writes the contents of `BusinessMatchRecords` to a CSV file at location `title`.  Each element in
    /// the vector of type [`BusinessMatchRecord`] maps to a row of data on the CSV.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(self.records_mut(), title)?;
        Ok(())
    }

    /// Creates a new `BusinessMatchRecords` struct from a CSV file located at `path`.  This method
    /// does not parse raw business license data, and should only be used to read files output from
    /// [`BusinessMatchRecords::to_csv()`].
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(BusinessMatchRecords { records })
    }

    /// The `records` field holds a vector of type [`BusinessMatchRecord`].  This method returns a
    /// mutable reference to the vector.
    pub fn records_mut(&mut self) -> &mut Vec<BusinessMatchRecord> {
        &mut self.records
    }
}

/// The `BusinessLicense` struct is designed to deserialize CSV data produced by querying the
/// EnerGov SQL database for active business licenses.  If the structure of the SQL query changes,
/// this function will need to change to match the resulting fields in the CSV.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusinessLicense {
    company_name: Option<String>,
    contact_name: Option<String>,
    business_type: String,
    #[serde(rename(deserialize = "dba"))]
    dba: Option<String>,
    business_phone: Option<i64>,
    #[serde(rename(deserialize = "LICENSENUMBER"))]
    license: String,
    #[serde(rename(deserialize = "EXPIRATIONDATE"))]
    expires: String,
    #[serde(rename(deserialize = "ADDRESSLINE1"))]
    address_number: i64,
    #[serde(rename(deserialize = "ADDRESSLINE2"))]
    street_name: String,
    #[serde(
        rename(deserialize = "PREDIRECTION"),
        deserialize_with = "deserialize_mixed_pre_directional"
    )]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(
        rename(deserialize = "STREETTYPE"),
        deserialize_with = "deserialize_mixed_post_type"
    )]
    street_name_post_type: Option<StreetNamePostType>,
    #[serde(
        rename(deserialize = "UNITORSUITE"),
        deserialize_with = "csv::invalid_option"
    )]
    subaddress_identifier: Option<String>,
    #[serde(rename(deserialize = "CITY"))]
    postal_community: String,
    #[serde(rename(deserialize = "STATE"))]
    state_name: String,
    #[serde(rename(deserialize = "POSTALCODE"))]
    zip_code: i64,
}

impl BusinessLicense {
    /// Compares the address of `BusinessLicense` to `address`, producing either a matching
    /// [`BusinessMatchRecord`], any divergent [`BusinessMatchRecord`], or `None` if missing.
    pub fn coincident<T: Address + GeoPoint<Num = f64>>(&self, address: &T) -> Option<BusinessMatchRecord> {
        let mut match_status = MatchStatus::Missing;
        let mut business_match = None;
        let street_name = self.street_name.trim().to_string();
        if self.address_number == address.number()
            && self.street_name_pre_directional == *address.directional()
            && street_name == *address.street_name()
            && self.street_name_post_type == *address.street_type()
        // && self.postal_community == address.postal_community()
        // && self.state_name == address.state_name()
        {
            if self.subaddress_identifier != *address.subaddress_id() {
                match_status = MatchStatus::Divergent;
            }
            if self.zip_code != address.zip() {
                match_status = MatchStatus::Divergent;
            }
            if match_status != MatchStatus::Divergent {
                match_status = MatchStatus::Matching;
            }
            let lat = address.lat();
            let lat = lat;
            business_match = Some(BusinessMatchRecord {
                match_status,
                business_address_label: self.label(),
                company_name: self.company_name(),
                contact_name: self.contact_name(),
                business_type: self.business_type(),
                dba: self.dba(),
                license: self.license(),
                expires: self.expires(),
                other_address_label: Some(address.label()),
                address_latitude: Some(address.lat()),
                address_longitude: Some(address.lon()),
            });
        }
        business_match
    }

    /// The `company_name` field represents the registered name of the business.  This method
    /// returns the cloned value of the field.
    pub fn company_name(&self) -> Option<String> {
        match self.company_name.clone() {
            Some(name) => Some(name.trim().to_string()),
            None => None.to_owned(),
        }
    }

    /// The `contact_name` field represents the business owner name.  This method clones the
    /// value of the field.
    pub fn contact_name(&self) -> Option<String> {
        self.contact_name.to_owned()
    }

    /// The `business_type` field represents the tax classification associated with a business
    /// license.  This method clones the value of the field.
    pub fn business_type(&self) -> String {
        self.business_type.to_owned()
    }

    /// The `dba` field represents the alias name associated with a business license.  This method
    /// clones the value of the field.
    pub fn dba(&self) -> Option<String> {
        self.dba.to_owned()
    }

    /// The `license` field represents the license ID associated with the business.  This method
    /// clones the value of the field.
    pub fn license(&self) -> String {
        self.license.to_owned()
    }

    /// The `expires` field represents the time of expiration for the active business license.
    /// This method clones the value of the field.
    pub fn expires(&self) -> String {
        self.expires.to_owned()
    }

    /// The `pre_directional` field represents the street pre-directional designation associated
    /// with a business license.  This method clones the value of the field.
    pub fn pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    /// The `post_type` field represents the street post type designation of the business address.
    /// This method returns the cloned value of the field.
    pub fn post_type(&self) -> Option<StreetNamePostType> {
        self.street_name_post_type
    }

    /// The `subaddress_identifier` field represents the subaddress unit identifier associated with
    /// a business address.  This method clones the value of the field.
    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.to_owned()
    }

    /// The `label` method creates a string representation of the complete street address
    /// associated with a business license.
    fn label(&self) -> String {
        let street_name = match self.post_type() {
            Some(post_type) => format!("{} {:?}", self.street_name, post_type),
            None => self.street_name.to_string(),
        };
        let complete_street_name = match self.street_name_pre_directional {
            Some(pre_directional) => format!("{:?} {}", pre_directional, street_name),
            None => street_name,
        };
        match self.subaddress_identifier() {
            Some(subaddress) => format!(
                "{} {} {}",
                self.address_number, complete_street_name, subaddress
            ),
            None => format!("{} {}", self.address_number, complete_street_name),
        }
    }
}

/// The `BusinessLicenses` struct holds a `records` field containing a vector of type
/// [`BusinessLicense`].
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BusinessLicenses {
    /// The `records` field contains a vector of type [`BusinessLicense`].
    pub records: Vec<BusinessLicense>,
}

impl BusinessLicenses {
    /// Creates a new `BusinessLicenses` struct from a CSV file located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(BusinessLicenses { records })
    }

    /// Returns the subset of `BusinessLicenses` where the value of the `filter` field is equal to
    /// the test value in `field`.  Currently `filter` can take the value `name`, referring to the
    /// company name.
    pub fn filter(&self, filter: &str, field: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "name" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.company_name() == Some(field.to_string()))
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        BusinessLicenses { records }
    }

    /// Retains one record from each license in `BusinessLicenses`, keeping the first encountered,
    /// intended to remove duplicate licenses from a record.
    pub fn deduplicate(&self) -> Self {
        let mut records = Vec::new();
        let mut licenses = HashSet::new();
        for record in self.records.clone() {
            let license = record.license();
            if !licenses.contains(&license) {
                licenses.insert(license);
                records.push(record);
            }
        }
        BusinessLicenses { records }
    }
}
